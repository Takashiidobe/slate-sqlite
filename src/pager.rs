unsafe extern "C" {
    static mut sqlite3CtypeMap: [u8; 0];
    static mut sqlite3Config: Sqlite3Config;
    static mut sqlite3PendingByte: i32;
    fn sqlite3_exec(
        __v1166: *mut sqlite3,
        sql: *const i8,
        callback: Option<
            unsafe extern "C-unwind" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32,
        >,
        __v1169: *mut (),
        errmsg: *mut *mut i8,
    ) -> i32;
    fn sqlite3_free(__v1171: *mut ());
    fn sqlite3_randomness(N: i32, P: *mut ());
    fn sqlite3_uri_boolean(z: *const i8, zParam: *const i8, bDefault: i32) -> i32;
    fn sqlite3_log(iErrCode: i32, zFormat: *const i8, ...);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3OsClose(__v1192: *mut sqlite3_file);
    fn sqlite3OsRead(__v1193: *mut sqlite3_file, __v1194: *mut (), amt: i32, offset: i64) -> i32;
    fn sqlite3OsWrite(__v1197: *mut sqlite3_file, __v1198: *const (), amt: i32, offset: i64)
    -> i32;
    fn sqlite3OsTruncate(__v1201: *mut sqlite3_file, size: i64) -> i32;
    fn sqlite3OsSync(__v1203: *mut sqlite3_file, __v1204: i32) -> i32;
    fn sqlite3OsFileSize(__v1205: *mut sqlite3_file, pSize: *mut i64) -> i32;
    fn sqlite3OsLock(__v1207: *mut sqlite3_file, __v1208: i32) -> i32;
    fn sqlite3OsUnlock(__v1209: *mut sqlite3_file, __v1210: i32) -> i32;
    fn sqlite3OsCheckReservedLock(id: *mut sqlite3_file, pResOut: *mut i32) -> i32;
    fn sqlite3OsFileControl(__v1213: *mut sqlite3_file, __v1214: i32, __v1215: *mut ()) -> i32;
    fn sqlite3OsFileControlHint(__v1216: *mut sqlite3_file, __v1217: i32, __v1218: *mut ());
    fn sqlite3OsSectorSize(id: *mut sqlite3_file) -> i32;
    fn sqlite3OsDeviceCharacteristics(id: *mut sqlite3_file) -> i32;
    fn sqlite3OsFetch(
        id: *mut sqlite3_file,
        __v1222: i64,
        __v1223: i32,
        __v1224: *mut *mut (),
    ) -> i32;
    fn sqlite3OsUnfetch(__v1225: *mut sqlite3_file, __v1226: i64, __v1227: *mut ()) -> i32;
    fn sqlite3OsOpen(
        __v1228: *mut sqlite3_vfs,
        __v1229: *const i8,
        __v1230: *mut sqlite3_file,
        __v1231: i32,
        __v1232: *mut i32,
    ) -> i32;
    fn sqlite3OsDelete(__v1233: *mut sqlite3_vfs, __v1234: *const i8, __v1235: i32) -> i32;
    fn sqlite3OsAccess(
        __v1236: *mut sqlite3_vfs,
        __v1237: *const i8,
        __v1238: i32,
        pResOut: *mut i32,
    ) -> i32;
    fn sqlite3OsFullPathname(
        __v1240: *mut sqlite3_vfs,
        __v1241: *const i8,
        __v1242: i32,
        __v1243: *mut i8,
    ) -> i32;
    fn sqlite3PcacheOpen(
        szPage: i32,
        szExtra: i32,
        bPurgeable: i32,
        xStress: Option<unsafe extern "C-unwind" fn(*mut (), *mut PgHdr) -> i32>,
        pStress: *mut (),
        pToInit: *mut PCache,
    ) -> i32;
    fn sqlite3PcacheSetPageSize(__v1362: *mut PCache, __v1363: i32) -> i32;
    fn sqlite3PcacheSize() -> i32;
    fn sqlite3PcacheFetch(
        __v1364: *mut PCache,
        __v1365: u32,
        createFlag: i32,
    ) -> *mut sqlite3_pcache_page;
    fn sqlite3PcacheFetchStress(
        __v1367: *mut PCache,
        __v1368: u32,
        __v1369: *mut *mut sqlite3_pcache_page,
    ) -> i32;
    fn sqlite3PcacheFetchFinish(
        __v1370: *mut PCache,
        __v1371: u32,
        pPage: *mut sqlite3_pcache_page,
    ) -> *mut PgHdr;
    fn sqlite3PcacheRelease(__v1373: *mut PgHdr);
    fn sqlite3PcacheDrop(__v1374: *mut PgHdr);
    fn sqlite3PcacheMakeDirty(__v1375: *mut PgHdr);
    fn sqlite3PcacheMakeClean(__v1376: *mut PgHdr);
    fn sqlite3PcacheCleanAll(__v1377: *mut PCache);
    fn sqlite3PcacheClearWritable(__v1378: *mut PCache);
    fn sqlite3PcacheMove(__v1379: *mut PgHdr, __v1380: u32);
    fn sqlite3PcacheTruncate(__v1381: *mut PCache, x: u32);
    fn sqlite3PcacheDirtyList(__v1383: *mut PCache) -> *mut PgHdr;
    fn sqlite3PcacheClose(__v1384: *mut PCache);
    fn sqlite3PcacheClearSyncFlags(__v1385: *mut PCache);
    fn sqlite3PcacheClear(__v1386: *mut PCache);
    fn sqlite3PcacheRefCount(__v1387: *mut PCache) -> i64;
    fn sqlite3PcacheRef(__v1388: *mut PgHdr);
    fn sqlite3PcachePageRefcount(__v1389: *mut PgHdr) -> i64;
    fn sqlite3PcachePagecount(__v1390: *mut PCache) -> i32;
    fn sqlite3PcacheSetCachesize(__v1391: *mut PCache, __v1392: i32);
    fn sqlite3PcacheSetSpillsize(__v1393: *mut PCache, __v1394: i32) -> i32;
    fn sqlite3PcacheShrink(__v1395: *mut PCache);
    fn sqlite3PCachePercentDirty(__v1396: *mut PCache) -> i32;
    fn sqlite3PCacheIsDirty(pCache: *mut PCache) -> i32;
    fn sqlite3CorruptError(__v1398: i32) -> i32;
    fn sqlite3CantopenError(__v1399: i32) -> i32;
    fn sqlite3Strlen30(__v1400: *const i8) -> i32;
    fn sqlite3Malloc(__v1401: u64) -> *mut ();
    fn sqlite3MallocZero(__v1402: u64) -> *mut ();
    fn sqlite3DbMallocRaw(__v1403: *mut sqlite3, __v1404: u64) -> *mut ();
    fn sqlite3DbStrDup(__v1405: *mut sqlite3, __v1406: *const i8) -> *mut i8;
    fn sqlite3Realloc(__v1407: *mut (), __v1408: u64) -> *mut ();
    fn sqlite3DbFree(__v1409: *mut sqlite3, __v1410: *mut ());
    fn sqlite3MallocSize(__v1411: *const ()) -> i32;
    fn sqlite3PageMalloc(__v1412: i32) -> *mut ();
    fn sqlite3PageFree(__v1413: *mut ());
    fn sqlite3FaultSim(__v1414: i32) -> i32;
    fn sqlite3BitvecCreate(__v1415: u32) -> *mut Bitvec;
    fn sqlite3BitvecTest(__v1416: *mut Bitvec, __v1417: u32) -> i32;
    fn sqlite3BitvecTestNotNull(__v1418: *mut Bitvec, __v1419: u32) -> i32;
    fn sqlite3BitvecSet(__v1420: *mut Bitvec, __v1421: u32) -> i32;
    fn sqlite3BitvecClear(__v1422: *mut Bitvec, __v1423: u32, __v1424: *mut ());
    fn sqlite3BitvecDestroy(__v1425: *mut Bitvec);
    fn sqlite3IsMemdb(__v1426: *const sqlite3_vfs) -> i32;
    fn sqlite3BackupRestart(__v1427: *mut sqlite3_backup);
    fn sqlite3BackupUpdate(__v1428: *mut sqlite3_backup, __v1429: u32, __v1430: *const u8);
    fn sqlite3BeginBenignMalloc();
    fn sqlite3EndBenignMalloc();
    fn sqlite3JournalOpen(
        __v1431: *mut sqlite3_vfs,
        __v1432: *const i8,
        __v1433: *mut sqlite3_file,
        __v1434: i32,
        __v1435: i32,
    ) -> i32;
    fn sqlite3JournalSize(__v1436: *mut sqlite3_vfs) -> i32;
    fn sqlite3JournalIsInMemory(p: *mut sqlite3_file) -> i32;
    fn sqlite3MemJournalOpen(__v1438: *mut sqlite3_file);
    fn sqlite3Get4byte(__v1439: *const u8) -> u32;
    fn sqlite3Put4byte(__v1440: *mut u8, __v1441: u32);
    fn sqlite3WalOpen(
        __v1442: *mut sqlite3_vfs,
        __v1443: *mut sqlite3_file,
        __v1444: *const i8,
        __v1445: i32,
        __v1446: i64,
        __v1447: *mut *mut Wal,
    ) -> i32;
    fn sqlite3WalClose(
        pWal: *mut Wal,
        __v1449: *mut sqlite3,
        sync_flags: i32,
        __v1451: i32,
        __v1452: *mut u8,
    ) -> i32;
    fn sqlite3WalLimit(__v1453: *mut Wal, __v1454: i64);
    fn sqlite3WalBeginReadTransaction(pWal: *mut Wal, __v1456: *mut i32) -> i32;
    fn sqlite3WalEndReadTransaction(pWal: *mut Wal);
    fn sqlite3WalFindFrame(__v1458: *mut Wal, __v1459: u32, __v1460: *mut u32) -> i32;
    fn sqlite3WalReadFrame(__v1461: *mut Wal, __v1462: u32, __v1463: i32, __v1464: *mut u8) -> i32;
    fn sqlite3WalDbsize(pWal: *mut Wal) -> u32;
    fn sqlite3WalBeginWriteTransaction(pWal: *mut Wal) -> i32;
    fn sqlite3WalEndWriteTransaction(pWal: *mut Wal) -> i32;
    fn sqlite3WalUndo(
        pWal: *mut Wal,
        xUndo: Option<unsafe extern "C-unwind" fn(*mut (), u32) -> i32>,
        pUndoCtx: *mut (),
    ) -> i32;
    fn sqlite3WalSavepoint(pWal: *mut Wal, aWalData: *mut u32);
    fn sqlite3WalSavepointUndo(pWal: *mut Wal, aWalData: *mut u32) -> i32;
    fn sqlite3WalFrames(
        pWal: *mut Wal,
        __v1476: i32,
        __v1477: *mut PgHdr,
        __v1478: u32,
        __v1479: i32,
        __v1480: i32,
    ) -> i32;
    fn sqlite3WalCheckpoint(
        pWal: *mut Wal,
        db: *mut sqlite3,
        eMode: i32,
        xBusy: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
        pBusyArg: *mut (),
        sync_flags: i32,
        nBuf: i32,
        zBuf: *mut u8,
        pnLog: *mut i32,
        pnCkpt: *mut i32,
    ) -> i32;
    fn sqlite3WalCallback(pWal: *mut Wal) -> i32;
    fn sqlite3WalExclusiveMode(pWal: *mut Wal, op: i32) -> i32;
    fn sqlite3WalHeapMemory(pWal: *mut Wal) -> i32;
    fn sqlite3WalFile(pWal: *mut Wal) -> *mut sqlite3_file;
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
    trace: __SlateRecord175,
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
    u1: __SlateRecord176,
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
struct Bitvec {}

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
    u: __SlateRecord177,
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
    __slate_bits_0: __slate_bits::__SlateBits99U0,
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
    u: __SlateRecord178,
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
struct Pager {
    pVfs: *mut sqlite3_vfs,
    exclusiveMode: u8,
    journalMode: u8,
    useJournal: u8,
    noSync: u8,
    fullSync: u8,
    extraSync: u8,
    syncFlags: u8,
    walSyncFlags: u8,
    tempFile: u8,
    noLock: u8,
    readOnly: u8,
    memDb: u8,
    memVfs: u8,
    eState: u8,
    eLock: u8,
    changeCountDone: u8,
    setSuper: u8,
    doNotSpill: u8,
    subjInMemory: u8,
    bUseFetch: u8,
    hasHeldSharedLock: u8,
    dbSize: u32,
    dbOrigSize: u32,
    dbFileSize: u32,
    dbHintSize: u32,
    errCode: i32,
    nRec: i32,
    cksumInit: u32,
    nSubRec: u32,
    pInJournal: *mut Bitvec,
    fd: *mut sqlite3_file,
    jfd: *mut sqlite3_file,
    sjfd: *mut sqlite3_file,
    journalOff: i64,
    journalHdr: i64,
    pBackup: *mut sqlite3_backup,
    aSavepoint: *mut PagerSavepoint,
    nSavepoint: i32,
    iDataVersion: u32,
    dbFileVers: [i8; 16],
    nMmapOut: i32,
    szMmap: i64,
    pMmapFreelist: *mut PgHdr,
    nExtra: u16,
    nReserve: i16,
    vfsFlags: u32,
    sectorSize: u32,
    mxPgno: u32,
    lckPgno: u32,
    pageSize: i64,
    journalSizeLimit: i64,
    zFilename: *mut i8,
    zJournal: *mut i8,
    xBusyHandler: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    pBusyHandlerArg: *mut (),
    aStat: [u32; 4],
    xReiniter: Option<unsafe extern "C-unwind" fn(*mut PgHdr)>,
    xGet: Option<unsafe extern "C-unwind" fn(*mut Pager, u32, *mut *mut PgHdr, i32) -> i32>,
    pTmpSpace: *mut i8,
    pPCache: *mut PCache,
    pWal: *mut Wal,
    zWal: *mut i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PgHdr {
    pPage: *mut sqlite3_pcache_page,
    pData: *mut (),
    pExtra: *mut (),
    pCache: *mut PCache,
    pDirty: *mut PgHdr,
    pPager: *mut Pager,
    pgno: u32,
    flags: u16,
    nRef: i64,
    pDirtyNext: *mut PgHdr,
    pDirtyPrev: *mut PgHdr,
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
struct PCache {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3InitInfo {
    newTnum: u32,
    iDb: u8,
    busy: u8,
    __slate_bits_0: __slate_bits::__SlateBits174U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord175 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    tab: __SlateRecord179,
    view: __SlateRecord180,
    vtab: __SlateRecord181,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord179 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord180 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord181 {
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
struct Wal {}

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
// ** This is the implementation of the page cache subsystem or "pager".
// **
// ** The pager is used to access a database disk file.  It implements
// ** atomic commit and rollback through the use of a journal file that
// ** is separate from the database file.  The pager also implements file
// ** locking to prevent two processes from writing the same database
// ** file simultaneously, or one process from reading the database while
// ** another is writing.
// */
// /******************* NOTES ON THE DESIGN OF THE PAGER ************************
// **
// ** This comment block describes invariants that hold when using a rollback
// ** journal.  These invariants do not apply for journal_mode=WAL,
// ** journal_mode=MEMORY, or journal_mode=OFF.
// **
// ** Within this comment block, a page is deemed to have been synced
// ** automatically as soon as it is written when PRAGMA synchronous=OFF.
// ** Otherwise, the page is not synced until the xSync method of the VFS
// ** is called successfully on the file containing the page.
// **
// ** Definition:  A page of the database file is said to be "overwriteable" if
// ** one or more of the following are true about the page:
// **
// **     (a)  The original content of the page as it was at the beginning of
// **          the transaction has been written into the rollback journal and
// **          synced.
// **
// **     (b)  The page was a freelist leaf page at the start of the transaction.
// **
// **     (c)  The page number is greater than the largest page that existed in
// **          the database file at the start of the transaction.
// **
// ** (1) A page of the database file is never overwritten unless one of the
// **     following are true:
// **
// **     (a) The page and all other pages on the same sector are overwriteable.
// **
// **     (b) The atomic page write optimization is enabled, and the entire
// **         transaction other than the update of the transaction sequence
// **         number consists of a single page change.
// **
// ** (2) The content of a page written into the rollback journal exactly matches
// **     both the content in the database when the rollback journal was written
// **     and the content in the database at the beginning of the current
// **     transaction.
// **
// ** (3) Writes to the database file are an integer multiple of the page size
// **     in length and are aligned on a page boundary.
// **
// ** (4) Reads from the database file are either aligned on a page boundary and
// **     an integer multiple of the page size in length or are taken from the
// **     first 100 bytes of the database file.
// **
// ** (5) All writes to the database file are synced prior to the rollback journal
// **     being deleted, truncated, or zeroed.
// **
// ** (6) If a super-journal file is used, then all writes to the database file
// **     are synced prior to the super-journal being deleted.
// **
// ** Definition: Two databases (or the same database at two points it time)
// ** are said to be "logically equivalent" if they give the same answer to
// ** all queries.  Note in particular the content of freelist leaf
// ** pages can be changed arbitrarily without affecting the logical equivalence
// ** of the database.
// **
// ** (7) At any time, if any subset, including the empty set and the total set,
// **     of the unsynced changes to a rollback journal are removed and the
// **     journal is rolled back, the resulting database file will be logically
// **     equivalent to the database file at the beginning of the transaction.
// **
// ** (8) When a transaction is rolled back, the xTruncate method of the VFS
// **     is called to restore the database file to the same size it was at
// **     the beginning of the transaction.  (In some VFSes, the xTruncate
// **     method is a no-op, but that does not change the fact the SQLite will
// **     invoke it.)
// **
// ** (9) Whenever the database file is modified, at least one bit in the range
// **     of bytes from 24 through 39 inclusive will be changed prior to releasing
// **     the EXCLUSIVE lock, thus signaling other connections on the same
// **     database to flush their caches.
// **
// ** (10) The pattern of bits in bytes 24 through 39 shall not repeat in less
// **      than one billion transactions.
// **
// ** (11) A database file is well-formed at the beginning and at the conclusion
// **      of every transaction.
// **
// ** (12) An EXCLUSIVE lock is held on the database file when writing to
// **      the database file.
// **
// ** (13) A SHARED lock is held on the database file while reading any
// **      content out of the database file.
// **
// ******************************************************************************/
// /*
// ** Macros for troubleshooting.  Normally turned off
// */
// /*
// ** The following two macros are used within the PAGERTRACE() macros above
// ** to print out file-descriptors.
// **
// ** PAGERID() takes a pointer to a Pager struct as its argument. The
// ** associated file-descriptor is returned. FILEHANDLEID() takes an sqlite3_file
// ** struct as its argument.
// */
// /*
// ** The Pager.eState variable stores the current 'state' of a pager. A
// ** pager may be in any one of the seven states shown in the following
// ** state diagram.
// **
// **                            OPEN <------+------+
// **                              |         |      |
// **                              V         |      |
// **               +---------> READER-------+      |
// **               |              |                |
// **               |              V                |
// **               |<-------WRITER_LOCKED------> ERROR
// **               |              |                ^
// **               |              V                |
// **               |<------WRITER_CACHEMOD-------->|
// **               |              |                |
// **               |              V                |
// **               |<-------WRITER_DBMOD---------->|
// **               |              |                |
// **               |              V                |
// **               +<------WRITER_FINISHED-------->+
// **
// **
// ** List of state transitions and the C [function] that performs each:
// **
// **   OPEN              -> READER              [sqlite3PagerSharedLock]
// **   READER            -> OPEN                [pager_unlock]
// **
// **   READER            -> WRITER_LOCKED       [sqlite3PagerBegin]
// **   WRITER_LOCKED     -> WRITER_CACHEMOD     [pager_open_journal]
// **   WRITER_CACHEMOD   -> WRITER_DBMOD        [syncJournal]
// **   WRITER_DBMOD      -> WRITER_FINISHED     [sqlite3PagerCommitPhaseOne]
// **   WRITER_***        -> READER              [pager_end_transaction]
// **
// **   WRITER_***        -> ERROR               [pager_error]
// **   ERROR             -> OPEN                [pager_unlock]
// **
// **
// **  OPEN:
// **
// **    The pager starts up in this state. Nothing is guaranteed in this
// **    state - the file may or may not be locked and the database size is
// **    unknown. The database may not be read or written.
// **
// **    * No read or write transaction is active.
// **    * Any lock, or no lock at all, may be held on the database file.
// **    * The dbSize, dbOrigSize and dbFileSize variables may not be trusted.
// **
// **  READER:
// **
// **    In this state all the requirements for reading the database in
// **    rollback (non-WAL) mode are met. Unless the pager is (or recently
// **    was) in exclusive-locking mode, a user-level read transaction is
// **    open. The database size is known in this state.
// **
// **    A connection running with locking_mode=normal enters this state when
// **    it opens a read-transaction on the database and returns to state
// **    OPEN after the read-transaction is completed. However a connection
// **    running in locking_mode=exclusive (including temp databases) remains in
// **    this state even after the read-transaction is closed. The only way
// **    a locking_mode=exclusive connection can transition from READER to OPEN
// **    is via the ERROR state (see below).
// **
// **    * A read transaction may be active (but a write-transaction cannot).
// **    * A SHARED or greater lock is held on the database file.
// **    * The dbSize variable may be trusted (even if a user-level read
// **      transaction is not active). The dbOrigSize and dbFileSize variables
// **      may not be trusted at this point.
// **    * If the database is a WAL database, then the WAL connection is open.
// **    * Even if a read-transaction is not open, it is guaranteed that
// **      there is no hot-journal in the file-system.
// **
// **  WRITER_LOCKED:
// **
// **    The pager moves to this state from READER when a write-transaction
// **    is first opened on the database. In WRITER_LOCKED state, all locks
// **    required to start a write-transaction are held, but no actual
// **    modifications to the cache or database have taken place.
// **
// **    In rollback mode, a RESERVED or (if the transaction was opened with
// **    BEGIN EXCLUSIVE) EXCLUSIVE lock is obtained on the database file when
// **    moving to this state, but the journal file is not written to or opened
// **    to in this state. If the transaction is committed or rolled back while
// **    in WRITER_LOCKED state, all that is required is to unlock the database
// **    file.
// **
// **    IN WAL mode, WalBeginWriteTransaction() is called to lock the log file.
// **    If the connection is running with locking_mode=exclusive, an attempt
// **    is made to obtain an EXCLUSIVE lock on the database file.
// **
// **    * A write transaction is active.
// **    * If the connection is open in rollback-mode, a RESERVED or greater
// **      lock is held on the database file.
// **    * If the connection is open in WAL-mode, a WAL write transaction
// **      is open (i.e. sqlite3WalBeginWriteTransaction() has been successfully
// **      called).
// **    * The dbSize, dbOrigSize and dbFileSize variables are all valid.
// **    * The contents of the pager cache have not been modified.
// **    * The journal file may or may not be open.
// **    * Nothing (not even the first header) has been written to the journal.
// **
// **  WRITER_CACHEMOD:
// **
// **    A pager moves from WRITER_LOCKED state to this state when a page is
// **    first modified by the upper layer. In rollback mode the journal file
// **    is opened (if it is not already open) and a header written to the
// **    start of it. The database file on disk has not been modified.
// **
// **    * A write transaction is active.
// **    * A RESERVED or greater lock is held on the database file.
// **    * The journal file is open and the first header has been written
// **      to it, but the header has not been synced to disk.
// **    * The contents of the page cache have been modified.
// **
// **  WRITER_DBMOD:
// **
// **    The pager transitions from WRITER_CACHEMOD into WRITER_DBMOD state
// **    when it modifies the contents of the database file. WAL connections
// **    never enter this state (since they do not modify the database file,
// **    just the log file).
// **
// **    * A write transaction is active.
// **    * An EXCLUSIVE or greater lock is held on the database file.
// **    * The journal file is open and the first header has been written
// **      and synced to disk.
// **    * The contents of the page cache have been modified (and possibly
// **      written to disk).
// **
// **  WRITER_FINISHED:
// **
// **    It is not possible for a WAL connection to enter this state.
// **
// **    A rollback-mode pager changes to WRITER_FINISHED state from WRITER_DBMOD
// **    state after the entire transaction has been successfully written into the
// **    database file. In this state the transaction may be committed simply
// **    by finalizing the journal file. Once in WRITER_FINISHED state, it is
// **    not possible to modify the database further. At this point, the upper
// **    layer must either commit or rollback the transaction.
// **
// **    * A write transaction is active.
// **    * An EXCLUSIVE or greater lock is held on the database file.
// **    * All writing and syncing of journal and database data has finished.
// **      If no error occurred, all that remains is to finalize the journal to
// **      commit the transaction. If an error did occur, the caller will need
// **      to rollback the transaction.
// **
// **  ERROR:
// **
// **    The ERROR state is entered when an IO or disk-full error (including
// **    SQLITE_IOERR_NOMEM) occurs at a point in the code that makes it
// **    difficult to be sure that the in-memory pager state (cache contents,
// **    db size etc.) are consistent with the contents of the file-system.
// **
// **    Temporary pager files may enter the ERROR state, but in-memory pagers
// **    cannot.
// **
// **    For example, if an IO error occurs while performing a rollback,
// **    the contents of the page-cache may be left in an inconsistent state.
// **    At this point it would be dangerous to change back to READER state
// **    (as usually happens after a rollback). Any subsequent readers might
// **    report database corruption (due to the inconsistent cache), and if
// **    they upgrade to writers, they may inadvertently corrupt the database
// **    file. To avoid this hazard, the pager switches into the ERROR state
// **    instead of READER following such an error.
// **
// **    Once it has entered the ERROR state, any attempt to use the pager
// **    to read or write data returns an error. Eventually, once all
// **    outstanding transactions have been abandoned, the pager is able to
// **    transition back to OPEN state, discarding the contents of the
// **    page-cache and any other in-memory state at the same time. Everything
// **    is reloaded from disk (and, if necessary, hot-journal rollback performed)
// **    when a read-transaction is next opened on the pager (transitioning
// **    the pager into READER state). At that point the system has recovered
// **    from the error.
// **
// **    Specifically, the pager jumps into the ERROR state if:
// **
// **      1. An error occurs while attempting a rollback. This happens in
// **         function sqlite3PagerRollback().
// **
// **      2. An error occurs while attempting to finalize a journal file
// **         following a commit in function sqlite3PagerCommitPhaseTwo().
// **
// **      3. An error occurs while attempting to write to the journal or
// **         database file in function pagerStress() in order to free up
// **         memory.
// **
// **    In other cases, the error is returned to the b-tree layer. The b-tree
// **    layer then attempts a rollback operation. If the error condition
// **    persists, the pager enters the ERROR state via condition (1) above.
// **
// **    Condition (3) is necessary because it can be triggered by a read-only
// **    statement executed within a transaction. In this case, if the error
// **    code were simply returned to the user, the b-tree layer would not
// **    automatically attempt a rollback, as it assumes that an error in a
// **    read-only statement cannot leave the pager in an internally inconsistent
// **    state.
// **
// **    * The Pager.errCode variable is set to something other than SQLITE_OK.
// **    * There are one or more outstanding references to pages (after the
// **      last reference is dropped the pager should move back to OPEN state).
// **    * The pager is not an in-memory pager.
// **
// **
// ** Notes:
// **
// **   * A pager is never in WRITER_DBMOD or WRITER_FINISHED state if the
// **     connection is open in WAL mode. A WAL connection is always in one
// **     of the first four states.
// **
// **   * Normally, a connection open in exclusive mode is never in PAGER_OPEN
// **     state. There are two exceptions: immediately after exclusive-mode has
// **     been turned on (and before any read or write transactions are
// **     executed), and when the pager is leaving the "error state".
// **
// **   * See also: assert_pager_state().
// */
// /*
// ** The Pager.eLock variable is almost always set to one of the
// ** following locking-states, according to the lock currently held on
// ** the database file: NO_LOCK, SHARED_LOCK, RESERVED_LOCK or EXCLUSIVE_LOCK.
// ** This variable is kept up to date as locks are taken and released by
// ** the pagerLockDb() and pagerUnlockDb() wrappers.
// **
// ** If the VFS xLock() or xUnlock() returns an error other than SQLITE_BUSY
// ** (i.e. one of the SQLITE_IOERR subtypes), it is not clear whether or not
// ** the operation was successful. In these circumstances pagerLockDb() and
// ** pagerUnlockDb() take a conservative approach - eLock is always updated
// ** when unlocking the file, and only updated when locking the file if the
// ** VFS call is successful. This way, the Pager.eLock variable may be set
// ** to a less exclusive (lower) value than the lock that is actually held
// ** at the system level, but it is never set to a more exclusive value.
// **
// ** This is usually safe. If an xUnlock fails or appears to fail, there may
// ** be a few redundant xLock() calls or a lock may be held for longer than
// ** required, but nothing really goes wrong.
// **
// ** The exception is when the database file is unlocked as the pager moves
// ** from ERROR to OPEN state. At this point there may be a hot-journal file
// ** in the file-system that needs to be rolled back (as part of an OPEN->SHARED
// ** transition, by the same pager or any other). If the call to xUnlock()
// ** fails at this point and the pager is left holding an EXCLUSIVE lock, this
// ** can confuse the call to xCheckReservedLock() call made later as part
// ** of hot-journal detection.
// **
// ** xCheckReservedLock() is defined as returning true "if there is a RESERVED
// ** lock held by this process or any others". So xCheckReservedLock may
// ** return true because the caller itself is holding an EXCLUSIVE lock (but
// ** doesn't know it because of a previous error in xUnlock). If this happens
// ** a hot-journal may be mistaken for a journal being created by an active
// ** transaction in another process, causing SQLite to read from the database
// ** without rolling it back.
// **
// ** To work around this, if a call to xUnlock() fails when unlocking the
// ** database in the ERROR state, Pager.eLock is set to UNKNOWN_LOCK. It
// ** is only changed back to a real locking state after a successful call
// ** to xLock(EXCLUSIVE). Also, the code to do the OPEN->SHARED state transition
// ** omits the check for a hot-journal if Pager.eLock is set to UNKNOWN_LOCK
// ** lock. Instead, it assumes a hot-journal exists and obtains an EXCLUSIVE
// ** lock on the database file before attempting to roll it back. See function
// ** PagerSharedLock() for more detail.
// **
// ** Pager.eLock may only be set to UNKNOWN_LOCK when the pager is in
// ** PAGER_OPEN state.
// */
// /*
// ** The maximum allowed sector size. 64KiB. If the xSectorsize() method
// ** returns a value larger than this, then MAX_SECTOR_SIZE is used instead.
// ** This could conceivably cause corruption following a power failure on
// ** such a system. This is currently an undocumented limit.
// */
// /*
// ** An instance of the following structure is allocated for each active
// ** savepoint and statement transaction in the system. All such structures
// ** are stored in the Pager.aSavepoint[] array, which is allocated and
// ** resized using sqlite3Realloc().
// **
// ** When a savepoint is created, the PagerSavepoint.iHdrOffset field is
// ** set to 0. If a journal-header is written into the main journal while
// ** the savepoint is active, then iHdrOffset is set to the byte offset
// ** immediately following the last journal record written into the main
// ** journal before the journal-header. This is required during savepoint
// ** rollback (see pagerPlaybackSavepoint()).
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct PagerSavepoint {
    iOffset: i64,
    // /* Starting offset in main journal */
    iHdrOffset: i64,
    // /* See above */
    pInSavepoint: *mut Bitvec,
    // /* Set of pages in this savepoint */
    nOrig: u32,
    // /* Original number of pages in file */
    iSubRec: u32,
    // /* Index of first record in sub-journal */
    bTruncateOnRelease: i32,
    // /* If stmt journal may be truncated on RELEASE */
    aWalData: [u32; 4],
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
    pub struct __SlateBits75U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
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
    pub struct __SlateBits174U0 {
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
}

// /* WAL savepoint context */
// /*
// ** Bits of the Pager.doNotSpill flag.  See further description below.
// */
// /* Never spill cache.  Set via pragma */
// /* Current rolling back, so do not spill */
// /* Spill is ok, but do not sync */
// /*
// ** An open page cache is an instance of struct Pager. A description of
// ** some of the more important member variables follows:
// **
// ** eState
// **
// **   The current 'state' of the pager object. See the comment and state
// **   diagram above for a description of the pager state.
// **
// ** eLock
// **
// **   For a real on-disk database, the current lock held on the database file -
// **   NO_LOCK, SHARED_LOCK, RESERVED_LOCK or EXCLUSIVE_LOCK.
// **
// **   For a temporary or in-memory database (neither of which require any
// **   locks), this variable is always set to EXCLUSIVE_LOCK. Since such
// **   databases always have Pager.exclusiveMode==1, this tricks the pager
// **   logic into thinking that it already has all the locks it will ever
// **   need (and no reason to release them).
// **
// **   In some (obscure) circumstances, this variable may also be set to
// **   UNKNOWN_LOCK. See the comment above the #define of UNKNOWN_LOCK for
// **   details.
// **
// ** changeCountDone
// **
// **   This boolean variable is used to make sure that the change-counter
// **   (the 4-byte header field at byte offset 24 of the database file) is
// **   not updated more often than necessary.
// **
// **   It is set to true when the change-counter field is updated, which
// **   can only happen if an exclusive lock is held on the database file.
// **   It is cleared (set to false) whenever an exclusive lock is
// **   relinquished on the database file. Each time a transaction is committed,
// **   The changeCountDone flag is inspected. If it is true, the work of
// **   updating the change-counter is omitted for the current transaction.
// **
// **   This mechanism means that when running in exclusive mode, a connection
// **   need only update the change-counter once, for the first transaction
// **   committed.
// **
// ** setSuper
// **
// **   When PagerCommitPhaseOne() is called to commit a transaction, it may
// **   (or may not) specify a super-journal name to be written into the
// **   journal file before it is synced to disk.
// **
// **   Whether or not a journal file contains a super-journal pointer affects
// **   the way in which the journal file is finalized after the transaction is
// **   committed or rolled back when running in "journal_mode=PERSIST" mode.
// **   If a journal file does not contain a super-journal pointer, it is
// **   finalized by overwriting the first journal header with zeroes. If
// **   it does contain a super-journal pointer the journal file is finalized
// **   by truncating it to zero bytes, just as if the connection were
// **   running in "journal_mode=truncate" mode.
// **
// **   Journal files that contain super-journal pointers cannot be finalized
// **   simply by overwriting the first journal-header with zeroes, as the
// **   super-journal pointer could interfere with hot-journal rollback of any
// **   subsequently interrupted transaction that reuses the journal file.
// **
// **   The flag is cleared as soon as the journal file is finalized (either
// **   by PagerCommitPhaseTwo or PagerRollback). If an IO error prevents the
// **   journal file from being successfully finalized, the setSuper flag
// **   is cleared anyway (and the pager will move to ERROR state).
// **
// ** doNotSpill
// **
// **   This variables control the behavior of cache-spills  (calls made by
// **   the pcache module to the pagerStress() routine to write cached data
// **   to the file-system in order to free up memory).
// **
// **   When bits SPILLFLAG_OFF or SPILLFLAG_ROLLBACK of doNotSpill are set,
// **   writing to the database from pagerStress() is disabled altogether.
// **   The SPILLFLAG_ROLLBACK case is done in a very obscure case that
// **   comes up during savepoint rollback that requires the pcache module
// **   to allocate a new page to prevent the journal file from being written
// **   while it is being traversed by code in pager_playback().  The SPILLFLAG_OFF
// **   case is a user preference.
// **
// **   If the SPILLFLAG_NOSYNC bit is set, writing to the database from
// **   pagerStress() is permitted, but syncing the journal file is not.
// **   This flag is set by sqlite3PagerWrite() when the file-system sector-size
// **   is larger than the database page-size in order to prevent a journal sync
// **   from happening in between the journalling of two pages on the same sector.
// **
// ** subjInMemory
// **
// **   This is a boolean variable. If true, then any required sub-journal
// **   is opened as an in-memory journal file. If false, then in-memory
// **   sub-journals are only used for in-memory pager files.
// **
// **   This variable is updated by the upper layer each time a new
// **   write-transaction is opened.
// **
// ** dbSize, dbOrigSize, dbFileSize
// **
// **   Variable dbSize is set to the number of pages in the database file.
// **   It is valid in PAGER_READER and higher states (all states except for
// **   OPEN and ERROR).
// **
// **   dbSize is set based on the size of the database file, which may be
// **   larger than the size of the database (the value stored at offset
// **   28 of the database header by the btree). If the size of the file
// **   is not an integer multiple of the page-size, the value stored in
// **   dbSize is rounded down (i.e. a 5KB file with 2K page-size has dbSize==2).
// **   Except, any file that is greater than 0 bytes in size is considered
// **   to have at least one page. (i.e. a 1KB file with 2K page-size leads
// **   to dbSize==1).
// **
// **   During a write-transaction, if pages with page-numbers greater than
// **   dbSize are modified in the cache, dbSize is updated accordingly.
// **   Similarly, if the database is truncated using PagerTruncateImage(),
// **   dbSize is updated.
// **
// **   Variables dbOrigSize and dbFileSize are valid in states
// **   PAGER_WRITER_LOCKED and higher. dbOrigSize is a copy of the dbSize
// **   variable at the start of the transaction. It is used during rollback,
// **   and to determine whether or not pages need to be journalled before
// **   being modified.
// **
// **   Throughout a write-transaction, dbFileSize contains the size of
// **   the file on disk in pages. It is set to a copy of dbSize when the
// **   write-transaction is first opened, and updated when VFS calls are made
// **   to write or truncate the database file on disk.
// **
// **   The only reason the dbFileSize variable is required is to suppress
// **   unnecessary calls to xTruncate() after committing a transaction. If,
// **   when a transaction is committed, the dbFileSize variable indicates
// **   that the database file is larger than the database image (Pager.dbSize),
// **   pager_truncate() is called. The pager_truncate() call uses xFilesize()
// **   to measure the database file on disk, and then truncates it if required.
// **   dbFileSize is not used when rolling back a transaction. In this case
// **   pager_truncate() is called unconditionally (which means there may be
// **   a call to xFilesize() that is not strictly required). In either case,
// **   pager_truncate() may cause the file to become smaller or larger.
// **
// ** dbHintSize
// **
// **   The dbHintSize variable is used to limit the number of calls made to
// **   the VFS xFileControl(FCNTL_SIZE_HINT) method.
// **
// **   dbHintSize is set to a copy of the dbSize variable when a
// **   write-transaction is opened (at the same time as dbFileSize and
// **   dbOrigSize). If the xFileControl(FCNTL_SIZE_HINT) method is called,
// **   dbHintSize is increased to the number of pages that correspond to the
// **   size-hint passed to the method call. See pager_write_pagelist() for
// **   details.
// **
// ** errCode
// **
// **   The Pager.errCode variable is only ever used in PAGER_ERROR state. It
// **   is set to zero in all other states. In PAGER_ERROR state, Pager.errCode
// **   is always set to SQLITE_FULL, SQLITE_IOERR or one of the SQLITE_IOERR_XXX
// **   sub-codes.
// **
// ** syncFlags, walSyncFlags
// **
// **   syncFlags is either SQLITE_SYNC_NORMAL (0x02) or SQLITE_SYNC_FULL (0x03).
// **   syncFlags is used for rollback mode.  walSyncFlags is used for WAL mode
// **   and contains the flags used to sync the checkpoint operations in the
// **   lower two bits, and sync flags used for transaction commits in the WAL
// **   file in bits 0x04 and 0x08.  In other words, to get the correct sync flags
// **   for checkpoint operations, use (walSyncFlags&0x03) and to get the correct
// **   sync flags for transaction commit, use ((walSyncFlags>>2)&0x03).  Note
// **   that with synchronous=NORMAL in WAL mode, transaction commit is not synced
// **   meaning that the 0x04 and 0x08 bits are both zero.
// */
// /* OS functions to use for IO */
// /* Boolean. True if locking_mode==EXCLUSIVE */
// /* One of the PAGER_JOURNALMODE_* values */
// /* Use a rollback journal on this file */
// /* Do not sync the journal if true */
// /* Do extra syncs of the journal for robustness */
// /* sync directory after journal delete */
// /* SYNC_NORMAL or SYNC_FULL otherwise */
// /* See description above */
// /* zFilename is a temporary or immutable file */
// /* Do not lock (except in WAL mode) */
// /* True for a read-only database */
// /* True to inhibit all file I/O */
// /* VFS-implemented memory database */
// /**************************************************************************
//   ** The following block contains those class members that change during
//   ** routine operation.  Class members not in this block are either fixed
//   ** when the pager is first created or else only change when there is a
//   ** significant mode change (such as changing the page_size, locking_mode,
//   ** or the journal_mode).  From another view, these class members describe
//   ** the "state" of the pager, while other class members describe the
//   ** "configuration" of the pager.
//   */
// /* Pager state (OPEN, READER, WRITER_LOCKED..) */
// /* Current lock held on database file */
// /* Set after incrementing the change-counter */
// /* Super-jrnl name is written into jrnl */
// /* Do not spill the cache when non-zero */
// /* True to use in-memory sub-journals */
// /* True to use xFetch() */
// /* True if a shared lock has ever been held */
// /* Number of pages in the database */
// /* dbSize before the current transaction */
// /* Number of pages in the database file */
// /* Value passed to FCNTL_SIZE_HINT call */
// /* One of several kinds of errors */
// /* Pages journalled since last j-header written */
// /* Quasi-random value added to every checksum */
// /* Number of records written to sub-journal */
// /* One bit for each page in the database file */
// /* File descriptor for database */
// /* File descriptor for main journal */
// /* File descriptor for sub-journal */
// /* Current write offset in the journal file */
// /* Byte offset to previous journal header */
// /* Pointer to list of ongoing backup processes */
// /* Array of active savepoints */
// /* Number of elements in aSavepoint[] */
// /* Changes whenever database content changes */
// /* Changes whenever database file changes */
// /* Number of mmap pages currently outstanding */
// /* Desired maximum mmap size */
// /* List of free mmap page headers (pDirty) */
// /*
//   ** End of the routinely-changing class members
//   ***************************************************************************/
// /* Add this many bytes to each in-memory page */
// /* Number of unused bytes at end of each page */
// /* Flags for sqlite3_vfs.xOpen() */
// /* Assumed sector size during rollback */
// /* Maximum allowed size of the database */
// /* Page number for the locking page */
// /* Number of bytes in a page */
// /* Size limit for persistent journal files */
// /* Name of the database file */
// /* Name of the journal file */
// /* Function to call when busy */
// /* Context argument for xBusyHandler */
// /* Total cache hits, misses, writes, spills */
// /* Call this routine when reloading pages */
// /* Routine to fetch a patch */
// /* Pager.pageSize bytes of space for tmp use */
// /* Pointer to page cache object */
// /* Write-ahead log used by "journal_mode=wal" */
// /* File name for write-ahead log */
// /*
// ** Indexes for use with Pager.aStat[]. The Pager.aStat[] array contains
// ** the values accessed by passing SQLITE_DBSTATUS_CACHE_HIT, CACHE_MISS
// ** or CACHE_WRITE to sqlite3_db_status().
// */
// /*
// ** The following global variables hold counters used for
// ** testing purposes only.  These variables do not exist in
// ** a non-testing build.  These variables are not thread-safe.
// */
// /*
// ** Journal files begin with the following magic string.  The data
// ** was obtained from /dev/random.  It is used only as a sanity check.
// **
// ** Since version 2.8.0, the journal format contains additional sanity
// ** checking information.  If the power fails while the journal is being
// ** written, semi-random garbage data might appear in the journal
// ** file after power is restored.  If an attempt is then made
// ** to roll the journal back, the database could be corrupted.  The additional
// ** sanity checking data is an attempt to discover the garbage in the
// ** journal and ignore it.
// **
// ** The sanity checking information for the new journal format consists
// ** of a 32-bit checksum on each page of data.  The checksum covers both
// ** the page number and the pPager->pageSize bytes of data for the page.
// ** This cksum is initialized to a 32-bit random value that appears in the
// ** journal file right after the header.  The random initializer is important,
// ** because garbage data that appears at the end of a journal is likely
// ** data that was once in other files that have now been deleted.  If the
// ** garbage data came from an obsolete journal file, the checksums might
// ** be correct.  But by initializing the checksum to random value which
// ** is different for every journal, we minimize that risk.
// */
static mut aJournalMagic: [u8; 8] = [
    ((217 as i32) as i8) as u8,
    ((213 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((249 as i32) as i8) as u8,
    ((32 as i32) as i8) as u8,
    ((161 as i32) as i8) as u8,
    ((99 as i32) as i8) as u8,
    ((215 as i32) as i8) as u8,
];

static mut zeroHdr: __SlateAlign16<[i8; 28]> = __SlateAlign16({
    let mut __t0: [i8; 28] = unsafe { std::mem::zeroed() };
    __t0[0] = (0 as i32) as i8;
    __t0
});

static mut zerobyte: u8 = ((0 as i32) as i8) as u8;

static mut zFake: [i8; 8] = [
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
];

// /* The virtual file system to use */
// /* OUT: Return the Pager structure here */
// /* Name of the database file to open */
// /* Extra bytes append to each in-memory page */
// /* flags controlling this file */
// /* flags passed through to sqlite3_vfs.xOpen() */
// /* Function to reinitialize pages */
// /*
// ** Return the sqlite3_file for the main database given the name
// ** of the corresponding WAL or Journal name as passed into
// ** xOpen.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.pager.sqlite3_database_file_object")]
extern "C-unwind" fn sqlite3_database_file_object(mut zName: *const i8) -> *mut sqlite3_file {
    let mut pPager: *mut Pager = unsafe { std::mem::zeroed() };
    let mut p: *const i8 = unsafe { std::mem::zeroed() };
    '__slate_break_1543: while ((unsafe { *unsafe { zName.offset(-(1 as i32) as isize) } }) as i32)
        != (0 as i32)
        || ((unsafe { *unsafe { zName.offset(-(2 as i32) as isize) } }) as i32) != (0 as i32)
        || ((unsafe { *unsafe { zName.offset(-(3 as i32) as isize) } }) as i32) != (0 as i32)
        || ((unsafe { *unsafe { zName.offset(-(4 as i32) as isize) } }) as i32) != (0 as i32)
    {
        let __v1549: *const i8 = zName;
        let __v1550: *const i8 = unsafe { __v1549.offset(-((1 as i32) as isize)) };
        zName = __v1550;
    }
    p = unsafe { unsafe { zName.offset(-((4 as i32) as isize)) }.offset(-((8 as u64) as isize)) };
    0 as i32;
    pPager = unsafe { *(p as *mut *mut Pager) };
    return unsafe { (*pPager).fd };
}

// /*
// ** Allocate and initialize a new Pager object and put a pointer to it
// ** in *ppPager. The pager should eventually be freed by passing it
// ** to sqlite3PagerClose().
// **
// ** The zFilename argument is the path to the database file to open.
// ** If zFilename is NULL then a randomly-named temporary file is created
// ** and used as the file to be cached. Temporary files are be deleted
// ** automatically when they are closed. If zFilename is ":memory:" then
// ** all information is held in cache. It is never written to disk.
// ** This can be used to implement an in-memory database.
// **
// ** The nExtra parameter specifies the number of bytes of space allocated
// ** along with each page reference. This space is available to the user
// ** via the sqlite3PagerGetExtra() API.  When a new page is allocated, the
// ** first 8 bytes of this space are zeroed but the remainder is uninitialized.
// ** (The extra space is used by btree as the MemPage object.)
// **
// ** The flags argument is used to specify properties that affect the
// ** operation of the pager. It should be passed some bitwise combination
// ** of the PAGER_* flags.
// **
// ** The vfsFlags parameter is a bitmask to pass to the flags parameter
// ** of the xOpen() method of the supplied VFS when opening files.
// **
// ** If the pager object is allocated and the specified file opened
// ** successfully, SQLITE_OK is returned and *ppPager set to point to
// ** the new pager object. If an error occurs, *ppPager is set to NULL
// ** and error code returned. This function may return SQLITE_NOMEM
// ** (sqlite3Malloc() is used to allocate memory), SQLITE_CANTOPEN or
// ** various SQLITE_IO_XXX errors.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut ppPager: *mut *mut Pager,
    mut zFilename: *const i8,
    mut nExtra: i32,
    mut flags: i32,
    mut vfsFlags: i32,
    mut xReinit: Option<unsafe extern "C-unwind" fn(*mut PgHdr)>,
) -> i32 {
    let mut __slate_storage_1590: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1590: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1590) as *mut i32;
    let mut __slate_storage_1589: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1589: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1589) as *mut i32;
    let mut __slate_storage_958: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_958: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_958) as *mut i32;
    let mut __slate_storage_1588: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1588: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1588) as *mut i32;
    let mut __slate_storage_1587: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1587: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1587) as *mut i32;
    let mut __slate_storage_1586: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1586: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1586) as *mut i32;
    let mut __slate_storage_1585: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1585: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1585) as *mut i32;
    let mut __slate_storage_1584: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1584: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1584) as *mut i32;
    let mut __slate_storage_957: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_957: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_957) as *mut i32;
    let mut __slate_storage_956: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_956: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_956) as *mut i32;
    let mut __slate_storage_1583: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1583: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1583) as *mut *mut u8;
    let mut __slate_storage_1582: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1582: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1582) as *mut *mut u8;
    let mut __slate_storage_1581: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1581: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1581) as *mut *mut u8;
    let mut __slate_storage_1580: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1580: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1580) as *mut *mut u8;
    let mut __slate_storage_1579: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1579: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1579) as *mut *mut u8;
    let mut __slate_storage_1578: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1578: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1578) as *mut *mut u8;
    let mut __slate_storage_1577: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1577: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1577) as *mut *mut u8;
    let mut __slate_storage_1576: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1576: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1576) as *mut *mut u8;
    let mut __slate_storage_1573: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1573: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1573) as *mut *mut u8;
    let mut __slate_storage_1572: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1572: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1572) as *mut *mut u8;
    let mut __slate_storage_1575: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1575: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1575) as *mut *mut u8;
    let mut __slate_storage_1574: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1574: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1574) as *mut *mut u8;
    let mut __slate_storage_1571: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1571: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1571) as *mut *mut u8;
    let mut __slate_storage_1570: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1570: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1570) as *mut *mut u8;
    let mut __slate_storage_1569: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1569: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1569) as *mut *mut u8;
    let mut __slate_storage_1568: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1568: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1568) as *mut *mut u8;
    let mut __slate_storage_1567: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1567: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1567) as *mut *mut u8;
    let mut __slate_storage_1566: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1566: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1566) as *mut *mut u8;
    let mut __slate_storage_1565: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1565: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1565) as *mut *mut u8;
    let mut __slate_storage_1564: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1564: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1564) as *mut *mut u8;
    let mut __slate_storage_1563: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1563: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1563) as *mut *mut u8;
    let mut __slate_storage_1562: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1562: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1562) as *mut *mut u8;
    let mut __slate_storage_1561: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1561: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1561) as *mut *mut u8;
    let mut __slate_storage_1560: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1560: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1560) as *mut *mut u8;
    let mut __slate_storage_1559: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1559: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1559) as *mut *mut u8;
    let mut __slate_storage_1558: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1558: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1558) as *mut *mut u8;
    let mut __slate_storage_1557: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1557: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1557) as *mut *mut u8;
    let mut __slate_storage_1556: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1556: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1556) as *mut *mut u8;
    let mut __slate_storage_1555: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1555: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1555) as *mut *const i8;
    let mut __slate_storage_1554: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1554: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1554) as *mut *const i8;
    let mut __slate_storage_1553: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1553: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1553) as *mut *const i8;
    let mut __slate_storage_1552: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1552: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1552) as *mut *const i8;
    let mut __slate_storage_1551: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1551: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1551) as *mut *const i8;
    let mut __slate_storage_955: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_955: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_955) as *mut *const i8;
    let mut __slate_storage_954: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_954: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_954) as *mut i32;
    let mut __slate_storage_953: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_953: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_953) as *mut *const i8;
    let mut __slate_storage_952: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_952: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_952) as *mut u32;
    let mut __slate_storage_951: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_951: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_951) as *mut i32;
    let mut __slate_storage_950: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_950: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_950) as *mut i32;
    let mut __slate_storage_949: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_949: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_949) as *mut i32;
    let mut __slate_storage_948: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_948: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_948) as *mut *mut i8;
    let mut __slate_storage_947: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_947: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_947) as *mut i32;
    let mut __slate_storage_946: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_946: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_946) as *mut i32;
    let mut __slate_storage_945: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_945: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_945) as *mut i32;
    let mut __slate_storage_944: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_944: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_944) as *mut i32;
    let mut __slate_storage_943: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_943: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_943) as *mut i32;
    let mut __slate_storage_942: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_942: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_942) as *mut i32;
    let mut __slate_storage_941: std::mem::MaybeUninit<*mut Pager> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_941: *mut *mut Pager =
        std::ptr::addr_of_mut!(__slate_storage_941) as *mut *mut Pager;
    let mut __slate_storage_940: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_940: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_940) as *mut *mut u8;
    unsafe {
        '__join_53: {
            // /* Pager object to allocate and return */
            std::ptr::write(__slate_slot_941, std::ptr::null_mut::<Pager>());
            // /* Return code */
            std::ptr::write(__slate_slot_942, 0 as i32);
            // /* True for temp files (incl. in-memory files) */
            std::ptr::write(__slate_slot_943, 0 as i32);
            // /* True if this is an in-memory file */
            std::ptr::write(__slate_slot_944, 0 as i32);
            // /* Memory journal mode */
            std::ptr::write(__slate_slot_945, 0 as i32);
            // /* True if this is a read-only file */
            std::ptr::write(__slate_slot_946, 0 as i32);
            // /* Bytes to allocate for each journal fd */
            // /* Full path to database file */
            std::ptr::write(__slate_slot_948, std::ptr::null_mut::<i8>());
            // /* Number of bytes in zPathname */
            std::ptr::write(__slate_slot_949, 0 as i32);
            // /* False to omit journal */
            std::ptr::write(__slate_slot_950, (flags & (1 as i32) == (0 as i32)) as i32);
            // /* Bytes to allocate for PCache */
            std::ptr::write(__slate_slot_951, unsafe { sqlite3PcacheSize() });
            // /* Default page size */
            std::ptr::write(__slate_slot_952, (4096 as i32) as u32);
            // /* URI args to copy */
            std::ptr::write(__slate_slot_953, std::ptr::null::<i8>());
            // /* Number of bytes of URI args at *zUri */
            std::ptr::write(__slate_slot_954, 1 as i32);
            // /* Figure out how much space is required for each journal file-handle
            //   ** (there are two of them, the main journal and the sub-journal).  */
            *__slate_slot_947 = (unsafe { sqlite3JournalSize(pVfs) }) + (7 as i32) & !(7 as i32);
            // /* Set the output variable to NULL in case an error occurs. */
            unsafe {
                *ppPager = std::ptr::null_mut::<Pager>();
            }
            if flags & (2 as i32) != (0 as i32) {
                *__slate_slot_944 = 1 as i32;
                if zFilename != std::ptr::null::<i8>()
                    && (unsafe { *unsafe { zFilename.offset((0 as i32) as isize) } }) != (0 as i8)
                {
                    *__slate_slot_948 =
                        unsafe { sqlite3DbStrDup(std::ptr::null_mut::<sqlite3>(), zFilename) };
                    if *__slate_slot_948 == std::ptr::null_mut::<i8>() {
                        return 7 as i32;
                    } else {
                        *__slate_slot_949 =
                            unsafe { sqlite3Strlen30(*__slate_slot_948 as *const i8) };
                        zFilename = std::ptr::null::<i8>();
                    }
                }
            }
        }
        // /* Compute and store the full pathname in an allocated buffer pointed
        //   ** to by zPathname, length nPathname. Or, if this is a temporary file,
        //   ** leave both nPathname and zPathname set to 0.
        //   */
        if zFilename != std::ptr::null::<i8>()
            && (unsafe { *unsafe { zFilename.offset((0 as i32) as isize) } }) != (0 as i8)
        {
            *__slate_slot_949 = (unsafe { (*pVfs).mxPathname }) + (1 as i32);
            *__slate_slot_948 = (unsafe {
                sqlite3DbMallocRaw(
                    std::ptr::null_mut::<sqlite3>(),
                    (((2 as i32) as i64) * (*__slate_slot_949 as i64)) as u64,
                )
            }) as *mut i8;
            if *__slate_slot_948 == std::ptr::null_mut::<i8>() {
                return 7 as i32;
            } else {
                // /* Make sure initialized even if FullPathname() fails */
                unsafe {
                    *unsafe { (*__slate_slot_948).offset((0 as i32) as isize) } = (0 as i32) as i8;
                }
                *__slate_slot_942 = unsafe {
                    sqlite3OsFullPathname(pVfs, zFilename, *__slate_slot_949, *__slate_slot_948)
                };
                if *__slate_slot_942 != (0 as i32) {
                    if *__slate_slot_942 == (0 as i32) | (2 as i32) << (8 as i32) {
                        if vfsFlags & (16777216 as i32) != (0 as i32) {
                            *__slate_slot_942 = (14 as i32) | (6 as i32) << (8 as i32);
                        } else {
                            *__slate_slot_942 = 0 as i32;
                        }
                    }
                }
                *__slate_slot_949 = unsafe { sqlite3Strlen30(*__slate_slot_948 as *const i8) };
                std::ptr::write(__slate_slot_1551, unsafe {
                    zFilename
                        .offset(((unsafe { sqlite3Strlen30(zFilename) }) + (1 as i32)) as isize)
                });
                *__slate_slot_953 = *__slate_slot_1551;
                *__slate_slot_955 = *__slate_slot_1551;
                loop {
                    if (unsafe { *(*__slate_slot_955) }) != (0 as i8) {
                        std::ptr::write(__slate_slot_1552, *__slate_slot_955);
                        std::ptr::write(__slate_slot_1553, unsafe {
                            (*__slate_slot_1552).offset(
                                unsafe { strlen(*__slate_slot_955) }
                                    .wrapping_add(((1 as i32) as i64) as u64)
                                    as isize,
                            )
                        });
                        *__slate_slot_955 = *__slate_slot_1553;
                        std::ptr::write(__slate_slot_1554, *__slate_slot_955);
                        std::ptr::write(__slate_slot_1555, unsafe {
                            (*__slate_slot_1554).offset(
                                unsafe { strlen(*__slate_slot_955) }
                                    .wrapping_add(((1 as i32) as i64) as u64)
                                    as isize,
                            )
                        });
                        *__slate_slot_955 = *__slate_slot_1555;
                    } else {
                        break;
                    }
                }
                *__slate_slot_954 = ((unsafe {
                    unsafe { (*__slate_slot_955).offset((1 as i32) as isize) }
                        .offset_from(*__slate_slot_953 as *const i8)
                }) as i64) as i32;
                0 as i32;
                if *__slate_slot_942 == (0 as i32)
                    && *__slate_slot_949 + (8 as i32) > unsafe { (*pVfs).mxPathname }
                {
                    // /* This branch is taken when the journal path required by
                    //       ** the database being opened will be more than pVfs->mxPathname
                    //       ** bytes in length. This means the database cannot be opened,
                    //       ** as it will not be possible to open the journal file or even
                    //       ** check for a hot-journal before reading.
                    //       */
                    *__slate_slot_942 = unsafe { sqlite3CantopenError(4871 as i32) };
                }
                if *__slate_slot_942 != (0 as i32) {
                    unsafe {
                        sqlite3DbFree(
                            std::ptr::null_mut::<sqlite3>(),
                            *__slate_slot_948 as *mut (),
                        )
                    };
                    return *__slate_slot_942;
                }
            }
        }
        // /* Allocate memory for the Pager structure, PCache object, the
        //   ** three file descriptors, the database file name and the journal
        //   ** file name. The layout in memory is as follows:
        //   **
        //   **     Pager object                    (sizeof(Pager) bytes)
        //   **     PCache object                   (sqlite3PcacheSize() bytes)
        //   **     Database file handle            (pVfs->szOsFile bytes)
        //   **     Sub-journal file handle         (journalFileSize bytes)
        //   **     Main journal file handle        (journalFileSize bytes)
        //   **     Ptr back to the Pager           (sizeof(Pager*) bytes)
        //   **     \0\0\0\0 database prefix        (4 bytes)
        //   **     Database file name              (nPathname+1 bytes)
        //   **     URI query parameters            (nUriByte bytes)
        //   **     Journal filename                (nPathname+8+1 bytes)
        //   **     WAL filename                    (nPathname+4+1 bytes)
        //   **     \0\0\0 terminator               (3 bytes)
        //   **
        //   ** Some 3rd-party software, over which we have no control, depends on
        //   ** the specific order of the filenames and the \0 separators between them
        //   ** so that it can (for example) find the database filename given the WAL
        //   ** filename without using the sqlite3_filename_database() API.  This is a
        //   ** misuse of SQLite and a bug in the 3rd-party software, but the 3rd-party
        //   ** software is in widespread use, so we try to avoid changing the filename
        //   ** order and formatting if possible.  In particular, the details of the
        //   ** filename format expected by 3rd-party software should be as follows:
        //   **
        //   **   - Main Database Path
        //   **   - \0
        //   **   - Multiple URI components consisting of:
        //   **     - Key
        //   **     - \0
        //   **     - Value
        //   **     - \0
        //   **   - \0
        //   **   - Journal Path
        //   **   - \0
        //   **   - WAL Path (zWALName)
        //   **   - \0
        //   **
        //   ** The sqlite3_create_filename() interface and the databaseFilename() utility
        //   ** that is used by sqlite3_filename_database() and kin also depend on the
        //   ** specific formatting and order of the various filenames, so if the format
        //   ** changes here, be sure to change it there as well.
        //   */
        0 as i32;
        *__slate_slot_940 = (unsafe {
            sqlite3MallocZero(
                ((312 as u64).wrapping_add(((7 as i32) as i64) as u64)
                    & ((!(7 as i32) as i64) as u64))
                    .wrapping_add(((*__slate_slot_951 + (7 as i32) & !(7 as i32)) as i64) as u64)
                    .wrapping_add(
                        (((unsafe { (*pVfs).szOsFile }) + (7 as i32) & !(7 as i32)) as i64) as u64,
                    )
                    .wrapping_add(
                        ((*__slate_slot_947 as i64) as u64)
                            .wrapping_mul(((2 as i32) as i64) as u64),
                    )
                    .wrapping_add(((8 as i32) as i64) as u64)
                    .wrapping_add(((4 as i32) as i64) as u64)
                    .wrapping_add((*__slate_slot_949 as i64) as u64)
                    .wrapping_add(((1 as i32) as i64) as u64)
                    .wrapping_add((*__slate_slot_954 as i64) as u64)
                    .wrapping_add((*__slate_slot_949 as i64) as u64)
                    .wrapping_add(((8 as i32) as i64) as u64)
                    .wrapping_add(((1 as i32) as i64) as u64)
                    .wrapping_add((*__slate_slot_949 as i64) as u64)
                    .wrapping_add(((4 as i32) as i64) as u64)
                    .wrapping_add(((1 as i32) as i64) as u64)
                    .wrapping_add(((3 as i32) as i64) as u64),
            )
        }) as *mut u8;
        // /* Pager structure */
        // /* PCache object */
        // /* The main db file */
        // /* The two journal files */
        // /* Space to hold a pointer */
        // /* Database prefix */
        // /* database filename */
        // /* query parameters */
        // /* Journal filename */
        // /* WAL filename */
        // /* Terminator */
        0 as i32;
        if !(*__slate_slot_940 != std::ptr::null_mut::<u8>()) {
            unsafe {
                sqlite3DbFree(
                    std::ptr::null_mut::<sqlite3>(),
                    *__slate_slot_948 as *mut (),
                )
            };
            return 7 as i32;
        } else {
            *__slate_slot_941 = *__slate_slot_940 as *mut Pager;
            std::ptr::write(__slate_slot_1556, *__slate_slot_940);
            std::ptr::write(__slate_slot_1557, unsafe {
                (*__slate_slot_1556).offset(
                    ((312 as u64).wrapping_add(((7 as i32) as i64) as u64)
                        & ((!(7 as i32) as i64) as u64)) as isize,
                )
            });
            *__slate_slot_940 = *__slate_slot_1557;
            unsafe {
                (*(*__slate_slot_941)).pPCache = *__slate_slot_940 as *mut PCache;
            }
            std::ptr::write(__slate_slot_1558, *__slate_slot_940);
            std::ptr::write(__slate_slot_1559, unsafe {
                (*__slate_slot_1558).offset((*__slate_slot_951 + (7 as i32) & !(7 as i32)) as isize)
            });
            *__slate_slot_940 = *__slate_slot_1559;
            unsafe {
                (*(*__slate_slot_941)).fd = *__slate_slot_940 as *mut sqlite3_file;
            }
            std::ptr::write(__slate_slot_1560, *__slate_slot_940);
            std::ptr::write(__slate_slot_1561, unsafe {
                (*__slate_slot_1560)
                    .offset(((unsafe { (*pVfs).szOsFile }) + (7 as i32) & !(7 as i32)) as isize)
            });
            *__slate_slot_940 = *__slate_slot_1561;
            unsafe {
                (*(*__slate_slot_941)).sjfd = *__slate_slot_940 as *mut sqlite3_file;
            }
            std::ptr::write(__slate_slot_1562, *__slate_slot_940);
            std::ptr::write(__slate_slot_1563, unsafe {
                (*__slate_slot_1562).offset(*__slate_slot_947 as isize)
            });
            *__slate_slot_940 = *__slate_slot_1563;
            unsafe {
                (*(*__slate_slot_941)).jfd = *__slate_slot_940 as *mut sqlite3_file;
            }
            std::ptr::write(__slate_slot_1564, *__slate_slot_940);
            std::ptr::write(__slate_slot_1565, unsafe {
                (*__slate_slot_1564).offset(*__slate_slot_947 as isize)
            });
            *__slate_slot_940 = *__slate_slot_1565;
            0 as i32;
            unsafe {
                memcpy(
                    *__slate_slot_940 as *mut (),
                    std::ptr::addr_of_mut!(*__slate_slot_941) as *const (),
                    ((8 as i32) as i64) as u64,
                )
            };
            std::ptr::write(__slate_slot_1566, *__slate_slot_940);
            std::ptr::write(__slate_slot_1567, unsafe {
                (*__slate_slot_1566).offset((8 as i32) as isize)
            });
            *__slate_slot_940 = *__slate_slot_1567;
            // /* Fill in the Pager.zFilename and pPager.zQueryParam fields */
            // /* Skip zero prefix */
            std::ptr::write(__slate_slot_1568, *__slate_slot_940);
            std::ptr::write(__slate_slot_1569, unsafe {
                (*__slate_slot_1568).offset((4 as i32) as isize)
            });
            *__slate_slot_940 = *__slate_slot_1569;
            unsafe {
                (*(*__slate_slot_941)).zFilename = *__slate_slot_940 as *mut i8;
            }
            if *__slate_slot_949 > (0 as i32) {
                unsafe {
                    memcpy(
                        *__slate_slot_940 as *mut (),
                        *__slate_slot_948 as *const (),
                        (*__slate_slot_949 as i64) as u64,
                    )
                };
                std::ptr::write(__slate_slot_1570, *__slate_slot_940);
                std::ptr::write(__slate_slot_1571, unsafe {
                    (*__slate_slot_1570).offset((*__slate_slot_949 + (1 as i32)) as isize)
                });
                *__slate_slot_940 = *__slate_slot_1571;
                if *__slate_slot_953 != std::ptr::null::<i8>() {
                    unsafe {
                        memcpy(
                            *__slate_slot_940 as *mut (),
                            *__slate_slot_953 as *const (),
                            (*__slate_slot_954 as i64) as u64,
                        )
                    };
                    std::ptr::write(__slate_slot_1572, *__slate_slot_940);
                    std::ptr::write(__slate_slot_1573, unsafe {
                        (*__slate_slot_1572).offset(*__slate_slot_954 as isize)
                    });
                    *__slate_slot_940 = *__slate_slot_1573;
                } else {
                    std::ptr::write(__slate_slot_1574, *__slate_slot_940);
                    std::ptr::write(__slate_slot_1575, unsafe {
                        (*__slate_slot_1574).offset((1 as i32) as isize)
                    });
                    *__slate_slot_940 = *__slate_slot_1575;
                }
            }
            // /* Fill in Pager.zJournal */
            if *__slate_slot_949 > (0 as i32) {
                unsafe {
                    (*(*__slate_slot_941)).zJournal = *__slate_slot_940 as *mut i8;
                }
                unsafe {
                    memcpy(
                        *__slate_slot_940 as *mut (),
                        *__slate_slot_948 as *const (),
                        (*__slate_slot_949 as i64) as u64,
                    )
                };
                std::ptr::write(__slate_slot_1576, *__slate_slot_940);
                std::ptr::write(__slate_slot_1577, unsafe {
                    (*__slate_slot_1576).offset(*__slate_slot_949 as isize)
                });
                *__slate_slot_940 = *__slate_slot_1577;
                unsafe {
                    memcpy(
                        *__slate_slot_940 as *mut (),
                        (b"-journal\0".as_ptr() as *mut i8) as *const (),
                        ((8 as i32) as i64) as u64,
                    )
                };
                std::ptr::write(__slate_slot_1578, *__slate_slot_940);
                std::ptr::write(__slate_slot_1579, unsafe {
                    (*__slate_slot_1578).offset(((8 as i32) + (1 as i32)) as isize)
                });
                *__slate_slot_940 = *__slate_slot_1579;
            } else {
                unsafe {
                    (*(*__slate_slot_941)).zJournal = std::ptr::null_mut::<i8>();
                }
            }
            // /* Fill in Pager.zWal */
            if *__slate_slot_949 > (0 as i32) {
                unsafe {
                    (*(*__slate_slot_941)).zWal = *__slate_slot_940 as *mut i8;
                }
                unsafe {
                    memcpy(
                        *__slate_slot_940 as *mut (),
                        *__slate_slot_948 as *const (),
                        (*__slate_slot_949 as i64) as u64,
                    )
                };
                std::ptr::write(__slate_slot_1580, *__slate_slot_940);
                std::ptr::write(__slate_slot_1581, unsafe {
                    (*__slate_slot_1580).offset(*__slate_slot_949 as isize)
                });
                *__slate_slot_940 = *__slate_slot_1581;
                unsafe {
                    memcpy(
                        *__slate_slot_940 as *mut (),
                        (b"-wal\0".as_ptr() as *mut i8) as *const (),
                        ((4 as i32) as i64) as u64,
                    )
                };
                std::ptr::write(__slate_slot_1582, *__slate_slot_940);
                std::ptr::write(__slate_slot_1583, unsafe {
                    (*__slate_slot_1582).offset(((4 as i32) + (1 as i32)) as isize)
                });
                *__slate_slot_940 = *__slate_slot_1583;
            } else {
                unsafe {
                    (*(*__slate_slot_941)).zWal = std::ptr::null_mut::<i8>();
                }
            }
            // /* Suppress warning about unused pPtr value */
            *__slate_slot_940;
            if *__slate_slot_949 != (0 as i32) {
                unsafe {
                    sqlite3DbFree(
                        std::ptr::null_mut::<sqlite3>(),
                        *__slate_slot_948 as *mut (),
                    )
                };
            }
            '__join_10: {
                unsafe {
                    (*(*__slate_slot_941)).pVfs = pVfs;
                }
                unsafe {
                    (*(*__slate_slot_941)).vfsFlags = vfsFlags as u32;
                }
                // /* Open the pager file.
                //   */
                if zFilename != std::ptr::null::<i8>()
                    && (unsafe { *unsafe { zFilename.offset((0 as i32) as isize) } }) != (0 as i8)
                {
                    // /* VFS flags returned by xOpen() */
                    std::ptr::write(__slate_slot_956, 0 as i32);
                    std::ptr::write(__slate_slot_957, unsafe {
                        sqlite3_uri_boolean(
                            (unsafe { (*(*__slate_slot_941)).zFilename }) as *const i8,
                            (b"immutable\0".as_ptr() as *mut i8) as *const i8,
                            0 as i32,
                        )
                    });
                    if *__slate_slot_957 != (0 as i32) {
                        std::ptr::write(__slate_slot_1584, vfsFlags);
                        std::ptr::write(__slate_slot_1585, *__slate_slot_1584 | (1 as i32));
                        vfsFlags = *__slate_slot_1585;
                        std::ptr::write(__slate_slot_1586, vfsFlags);
                        std::ptr::write(
                            __slate_slot_1587,
                            *__slate_slot_1586 & !((2 as i32) | (4 as i32)),
                        );
                        vfsFlags = *__slate_slot_1587;
                    }
                    *__slate_slot_942 = unsafe {
                        sqlite3OsOpen(
                            pVfs,
                            (unsafe { (*(*__slate_slot_941)).zFilename }) as *const i8,
                            unsafe { (*(*__slate_slot_941)).fd },
                            vfsFlags,
                            std::ptr::addr_of_mut!(*__slate_slot_956),
                        )
                    };
                    if *__slate_slot_957 != (0 as i32) {
                    } else {
                        0 as i32;
                        std::ptr::write(
                            __slate_slot_1588,
                            (*__slate_slot_956 & (128 as i32) != (0 as i32)) as i32,
                        );
                        *__slate_slot_945 = *__slate_slot_1588;
                        unsafe {
                            (*(*__slate_slot_941)).memVfs = (*__slate_slot_1588 as i8) as u8;
                        }
                        *__slate_slot_946 = (*__slate_slot_956 & (1 as i32) != (0 as i32)) as i32;
                        // /* If the file was successfully opened for read/write access,
                        //     ** choose a default page size in case we have to create the
                        //     ** database file. The default page size is the maximum of:
                        //     **
                        //     **    + SQLITE_DEFAULT_PAGE_SIZE,
                        //     **    + The value returned by sqlite3OsSectorSize()
                        //     **    + The largest page size that can be written atomically.
                        //     */
                        if *__slate_slot_942 == (0 as i32) {
                            std::ptr::write(__slate_slot_958, unsafe {
                                sqlite3OsDeviceCharacteristics(unsafe { (*(*__slate_slot_941)).fd })
                            });
                            if !(*__slate_slot_946 != (0 as i32)) {
                                setSectorSize(*__slate_slot_941);
                                0 as i32;
                                if *__slate_slot_952 < unsafe { (*(*__slate_slot_941)).sectorSize }
                                {
                                    if (unsafe { (*(*__slate_slot_941)).sectorSize })
                                        > ((8192 as i32) as u32)
                                    {
                                        *__slate_slot_952 = (8192 as i32) as u32;
                                    } else {
                                        *__slate_slot_952 =
                                            unsafe { (*(*__slate_slot_941)).sectorSize };
                                    }
                                }
                            }
                            unsafe {
                                (*(*__slate_slot_941)).noLock = ((unsafe {
                                    sqlite3_uri_boolean(
                                        (unsafe { (*(*__slate_slot_941)).zFilename }) as *const i8,
                                        (b"nolock\0".as_ptr() as *mut i8) as *const i8,
                                        0 as i32,
                                    )
                                })
                                    as i8)
                                    as u8;
                            }
                            if *__slate_slot_958 & (8192 as i32) != (0 as i32) {
                                std::ptr::write(__slate_slot_1589, vfsFlags);
                                std::ptr::write(__slate_slot_1590, *__slate_slot_1589 | (1 as i32));
                                vfsFlags = *__slate_slot_1590;
                            } else {
                                break '__join_10;
                            }
                        } else {
                            break '__join_10;
                        }
                    }
                } else {
                    // /* If a temporary file is requested, it is not opened immediately.
                    //     ** In this case we accept the default page size and delay actually
                    //     ** opening the file until the first call to OsWrite().
                    //     **
                    //     ** This branch is also run for an in-memory database. An in-memory
                    //     ** database is the same as a temp-file that is never written out to
                    //     ** disk and uses an in-memory rollback journal.
                    //     **
                    //     ** This branch also runs for files marked as immutable.
                    //     */
                }
                *__slate_slot_943 = 1 as i32;
                // /* Pretend we already have a lock */
                unsafe {
                    (*(*__slate_slot_941)).eState = ((1 as i32) as i8) as u8;
                }
                // /* Pretend we are in EXCLUSIVE mode */
                unsafe {
                    (*(*__slate_slot_941)).eLock = ((4 as i32) as i8) as u8;
                }
                // /* Do no locking */
                unsafe {
                    (*(*__slate_slot_941)).noLock = ((1 as i32) as i8) as u8;
                }
                *__slate_slot_946 = vfsFlags & (1 as i32);
                0 as i32;
            }
            // /* The following call to PagerSetPagesize() serves to set the value of
            //   ** Pager.pageSize and to allocate the Pager.pTmpSpace buffer.
            //   */
            if *__slate_slot_942 == (0 as i32) {
                0 as i32;
                *__slate_slot_942 = sqlite3PagerSetPagesize(
                    *__slate_slot_941,
                    std::ptr::addr_of_mut!(*__slate_slot_952),
                    -(1 as i32),
                );
                {}
            }
            // /* Initialize the PCache object. */
            if *__slate_slot_942 == (0 as i32) {
                nExtra = nExtra + (7 as i32) & !(7 as i32);
                0 as i32;
                *__slate_slot_942 = unsafe {
                    sqlite3PcacheOpen(
                        *__slate_slot_952 as i32,
                        nExtra,
                        !(*__slate_slot_944 != (0 as i32)) as i32,
                        {
                            let __t0: Option<
                                unsafe extern "C-unwind" fn(*mut (), *mut PgHdr) -> i32,
                            > = if !(*__slate_slot_944 != (0 as i32)) {
                                Some(pagerStress)
                            } else {
                                None
                            };
                            __t0
                        },
                        *__slate_slot_941 as *mut (),
                        unsafe { (*(*__slate_slot_941)).pPCache },
                    )
                };
            }
            // /* If an error occurred above, free the  Pager structure and close the file.
            //   */
            if *__slate_slot_942 != (0 as i32) {
                unsafe { sqlite3OsClose(unsafe { (*(*__slate_slot_941)).fd }) };
                unsafe {
                    sqlite3PageFree((unsafe { (*(*__slate_slot_941)).pTmpSpace }) as *mut ())
                };
                unsafe { sqlite3_free(*__slate_slot_941 as *mut ()) };
                return *__slate_slot_942;
            } else {
                {}
                unsafe {
                    (*(*__slate_slot_941)).useJournal = (*__slate_slot_950 as i8) as u8;
                }
                // /* pPager->stmtOpen = 0; */
                // /* pPager->stmtInUse = 0; */
                // /* pPager->nRef = 0; */
                // /* pPager->stmtSize = 0; */
                // /* pPager->stmtJSize = 0; */
                // /* pPager->nPage = 0; */
                unsafe {
                    (*(*__slate_slot_941)).mxPgno = 4294967294 as u32;
                }
                // /* pPager->state = PAGER_UNLOCK; */
                // /* pPager->errMask = 0; */
                unsafe {
                    (*(*__slate_slot_941)).tempFile = (*__slate_slot_943 as i8) as u8;
                }
                0 as i32;
                0 as i32;
                unsafe {
                    (*(*__slate_slot_941)).exclusiveMode = (*__slate_slot_943 as i8) as u8;
                }
                unsafe {
                    (*(*__slate_slot_941)).changeCountDone =
                        unsafe { (*(*__slate_slot_941)).tempFile };
                }
                unsafe {
                    (*(*__slate_slot_941)).memDb = (*__slate_slot_944 as i8) as u8;
                }
                unsafe {
                    (*(*__slate_slot_941)).readOnly = (*__slate_slot_946 as i8) as u8;
                }
                0 as i32;
                sqlite3PagerSetFlags(
                    *__slate_slot_941,
                    ((2 as i32) + (1 as i32) | (32 as i32)) as u32,
                );
                // /* pPager->pFirst = 0; */
                // /* pPager->pFirstSynced = 0; */
                // /* pPager->pLast = 0; */
                unsafe {
                    (*(*__slate_slot_941)).nExtra = (nExtra as i16) as u16;
                }
                unsafe {
                    (*(*__slate_slot_941)).journalSizeLimit = -(1 as i32) as i64;
                }
                0 as i32;
                setSectorSize(*__slate_slot_941);
                if !(*__slate_slot_950 != (0 as i32)) {
                    unsafe {
                        (*(*__slate_slot_941)).journalMode = ((2 as i32) as i8) as u8;
                    }
                } else {
                    if *__slate_slot_944 != (0 as i32) || *__slate_slot_945 != (0 as i32) {
                        unsafe {
                            (*(*__slate_slot_941)).journalMode = ((4 as i32) as i8) as u8;
                        }
                    }
                }
                // /* pPager->xBusyHandler = 0; */
                // /* pPager->pBusyHandlerArg = 0; */
                unsafe {
                    (*(*__slate_slot_941)).xReiniter = xReinit;
                }
                setGetterMethod(*__slate_slot_941);
                // /* memset(pPager->aHash, 0, sizeof(pPager->aHash)); */
                // /* pPager->szMmap = SQLITE_DEFAULT_MMAP_SIZE // will be set by btree.c */
                unsafe {
                    *ppPager = *__slate_slot_941;
                }
                return 0 as i32;
            }
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Shutdown the page cache.  Free all memory and close all files.
// **
// ** If a transaction was in progress when this routine is called, that
// ** transaction is rolled back.  All outstanding pages are invalidated
// ** and their memory is freed.  Any attempt to use a page associated
// ** with this page cache after this function returns will likely
// ** result in a coredump.
// **
// ** This function always succeeds. If a transaction is active an attempt
// ** is made to roll it back. If an error occurs during the rollback
// ** a hot journal may be left in the filesystem but no error is returned
// ** to the caller.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerClose(mut pPager: *mut Pager, mut db: *mut sqlite3) -> i32 {
    let mut pTmp: *mut u8 = (unsafe { (*pPager).pTmpSpace }) as *mut u8;
    0 as i32;
    0 as i32;
    {}
    unsafe { sqlite3BeginBenignMalloc() };
    pagerFreeMapHdrs(pPager);
    // /* pPager->errCode = 0; */
    unsafe {
        (*pPager).exclusiveMode = ((0 as i32) as i8) as u8;
    }
    let mut a: *mut u8 = std::ptr::null_mut::<u8>();
    0 as i32;
    let __v1591: bool;
    if db != std::ptr::null_mut::<sqlite3>()
        && (((0 as i32) as i64) as u64)
            == (unsafe { (*db).flags }) & (((2048 as i32) as i64) as u64)
    {
        __v1591 = (0 as i32) == databaseIsUnmoved(pPager);
    } else {
        __v1591 = false as bool;
    }
    if __v1591 {
        a = pTmp;
    }
    unsafe {
        sqlite3WalClose(
            unsafe { (*pPager).pWal },
            db,
            ((unsafe { (*pPager).walSyncFlags }) as u32) as i32,
            (unsafe { (*pPager).pageSize }) as i32,
            a,
        )
    };
    unsafe {
        (*pPager).pWal = std::ptr::null_mut::<Wal>();
    }
    pager_reset(pPager);
    if (unsafe { (*pPager).memDb }) != (0 as u8) {
        pager_unlock(pPager);
    } else {
        // /* If it is open, sync the journal file before calling UnlockAndRollback.
        //     ** If this is not done, then an unsynced portion of the open journal
        //     ** file may be played back into the database. If a power failure occurs
        //     ** while this is happening, the database could become corrupt.
        //     **
        //     ** If an error occurs while trying to sync the journal, shift the pager
        //     ** into the ERROR state. This causes UnlockAndRollback to unlock the
        //     ** database and close the journal file without attempting to roll it
        //     ** back or finalize it. The next database user will have to do hot-journal
        //     ** rollback before accessing the database file.
        //     */
        if (unsafe { (*unsafe { (*pPager).jfd }).pMethods })
            != std::ptr::null::<sqlite3_io_methods>()
        {
            pager_error(pPager, pagerSyncHotJournal(pPager));
        }
        pagerUnlockAndRollback(pPager);
    }
    unsafe { sqlite3EndBenignMalloc() };
    {}
    {}
    unsafe { sqlite3OsClose(unsafe { (*pPager).jfd }) };
    unsafe { sqlite3OsClose(unsafe { (*pPager).fd }) };
    unsafe { sqlite3PageFree(pTmp as *mut ()) };
    unsafe { sqlite3PcacheClose(unsafe { (*pPager).pPCache }) };
    0 as i32;
    0 as i32;
    unsafe { sqlite3_free(pPager as *mut ()) };
    return 0 as i32;
}

// /*
// ** The following set of routines are used to disable the simulated
// ** I/O error mechanism.  These routines are used to avoid simulated
// ** errors in places where we do not care about errors.
// **
// ** Unless -DSQLITE_TEST=1 is used, these routines are all no-ops
// ** and generate no code.
// */
// /*
// ** Read the first N bytes from the beginning of the file into memory
// ** that pDest points to.
// **
// ** If the pager was opened on a transient file (zFilename==""), or
// ** opened on a file less than N bytes in size, the output buffer is
// ** zeroed and SQLITE_OK returned. The rationale for this is that this
// ** function is used to read database headers, and a new transient or
// ** zero sized database has a header than consists entirely of zeroes.
// **
// ** If any IO error apart from SQLITE_IOERR_SHORT_READ is encountered,
// ** the error code is returned to the caller and the contents of the
// ** output buffer undefined.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerReadFileheader(
    mut pPager: *mut Pager,
    mut N: i32,
    mut pDest: *mut u8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    unsafe { memset(pDest as *mut (), 0 as i32, (N as i64) as u64) };
    0 as i32;
    // /* This routine is only called by btree immediately after creating
    //   ** the Pager object.  There has not been an opportunity to transition
    //   ** to WAL mode yet.
    //   */
    0 as i32;
    if (unsafe { (*unsafe { (*pPager).fd }).pMethods }) != std::ptr::null::<sqlite3_io_methods>() {
        rc = unsafe {
            sqlite3OsRead(
                unsafe { (*pPager).fd },
                pDest as *mut (),
                N,
                (0 as i32) as i64,
            )
        };
        if rc == (10 as i32) | (2 as i32) << (8 as i32) {
            rc = 0 as i32;
        }
    }
    return rc;
}

// /* The pager object */
// /* Write the file descriptor here */
// /* Flags passed through to the VFS */
// /*
// ** Set the busy handler function.
// **
// ** The pager invokes the busy-handler if sqlite3OsLock() returns
// ** SQLITE_BUSY when trying to upgrade from no-lock to a SHARED lock,
// ** or when trying to upgrade from a RESERVED lock to an EXCLUSIVE
// ** lock. It does *not* invoke the busy handler when upgrading from
// ** SHARED to RESERVED, or when upgrading from SHARED to EXCLUSIVE
// ** (which occurs during hot-journal rollback). Summary:
// **
// **   Transition                        | Invokes xBusyHandler
// **   --------------------------------------------------------
// **   NO_LOCK       -> SHARED_LOCK      | Yes
// **   SHARED_LOCK   -> RESERVED_LOCK    | No
// **   SHARED_LOCK   -> EXCLUSIVE_LOCK   | No
// **   RESERVED_LOCK -> EXCLUSIVE_LOCK   | Yes
// **
// ** If the busy-handler callback returns non-zero, the lock is
// ** retried. If it returns zero, then the SQLITE_BUSY error is
// ** returned to the caller of the pager API function.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerSetBusyHandler(
    mut pPager: *mut Pager,
    mut xBusyHandler: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    mut pBusyHandlerArg: *mut (),
) {
    let mut ap: *mut *mut () = unsafe { std::mem::zeroed() };
    unsafe {
        (*pPager).xBusyHandler = xBusyHandler;
    }
    unsafe {
        (*pPager).pBusyHandlerArg = pBusyHandlerArg;
    }
    ap = (unsafe { std::ptr::addr_of_mut!((*pPager).xBusyHandler) }) as *mut *mut ();
    0 as i32;
    0 as i32;
    unsafe { sqlite3OsFileControlHint(unsafe { (*pPager).fd }, 15 as i32, ap as *mut ()) };
}

// /* Pager object */
// /* Pointer to busy-handler function */
// /* Argument to pass to xBusyHandler */
// /*
// ** Change the page size used by the Pager object. The new page size
// ** is passed in *pPageSize.
// **
// ** If the pager is in the error state when this function is called, it
// ** is a no-op. The value returned is the error state error code (i.e.
// ** one of SQLITE_IOERR, an SQLITE_IOERR_xxx sub-code or SQLITE_FULL).
// **
// ** Otherwise, if all of the following are true:
// **
// **   * the new page size (value of *pPageSize) is valid (a power
// **     of two between 512 and SQLITE_MAX_PAGE_SIZE, inclusive), and
// **
// **   * there are no outstanding page references, and
// **
// **   * the database is either not an in-memory database or it is
// **     an in-memory database that currently consists of zero pages.
// **
// ** then the pager object page size is set to *pPageSize.
// **
// ** If the page size is changed, then this function uses sqlite3PageMalloc()
// ** to obtain a new Pager.pTmpSpace buffer. If this allocation attempt
// ** fails, SQLITE_NOMEM is returned and the page size remains unchanged.
// ** In all other cases, SQLITE_OK is returned.
// **
// ** If the page size is not changed, either because one of the enumerated
// ** conditions above is not true, the pager was in error state when this
// ** function was called, or because the memory allocation attempt failed,
// ** then *pPageSize is set to the old, retained page size before returning.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerSetPagesize(
    mut pPager: *mut Pager,
    mut pPageSize: *mut u32,
    mut nReserve: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    // /* It is not possible to do a full assert_pager_state() here, as this
    //   ** function may be called from within PagerOpen(), before the state
    //   ** of the Pager object is internally consistent.
    //   **
    //   ** At one point this function returned an error if the pager was in
    //   ** PAGER_ERROR state. But since PAGER_ERROR state guarantees that
    //   ** there is at least one outstanding page reference, this function
    //   ** is a no-op for that case anyhow.
    //   */
    let mut pageSize: u32 = unsafe { *pPageSize };
    0 as i32;
    let __v1592: bool;
    if (((unsafe { (*pPager).memDb }) as u32) as i32) == (0 as i32)
        || (unsafe { (*pPager).dbSize }) == ((0 as i32) as u32)
    {
        __v1592 =
            (unsafe { sqlite3PcacheRefCount(unsafe { (*pPager).pPCache }) }) == ((0 as i32) as i64);
    } else {
        __v1592 = false as bool;
    }
    if __v1592
        && pageSize != (0 as u32)
        && pageSize != (((unsafe { (*pPager).pageSize }) as i32) as u32)
    {
        // /* New temp space */
        let mut pNew: *mut i8 = std::ptr::null_mut::<i8>();
        let mut nByte: i64 = (0 as i32) as i64;
        if (((unsafe { (*pPager).eState }) as u32) as i32) > (0 as i32)
            && (unsafe { (*unsafe { (*pPager).fd }).pMethods })
                != std::ptr::null::<sqlite3_io_methods>()
        {
            rc = unsafe {
                sqlite3OsFileSize(unsafe { (*pPager).fd }, std::ptr::addr_of_mut!(nByte))
            };
        }
        if rc == (0 as i32) {
            // /* 8 bytes of zeroed overrun space is sufficient so that the b-tree
            //       * cell header parser will never run off the end of the allocation */
            pNew = (unsafe { sqlite3PageMalloc(pageSize.wrapping_add((8 as i32) as u32) as i32) })
                as *mut i8;
            if !(pNew != std::ptr::null_mut::<i8>()) {
                rc = 7 as i32;
            } else {
                unsafe {
                    memset(
                        (unsafe { pNew.offset(pageSize as isize) }) as *mut (),
                        0 as i32,
                        ((8 as i32) as i64) as u64,
                    )
                };
            }
        }
        if rc == (0 as i32) {
            pager_reset(pPager);
            rc = unsafe { sqlite3PcacheSetPageSize(unsafe { (*pPager).pPCache }, pageSize as i32) };
        }
        if rc == (0 as i32) {
            unsafe { sqlite3PageFree((unsafe { (*pPager).pTmpSpace }) as *mut ()) };
            unsafe {
                (*pPager).pTmpSpace = pNew;
            }
            unsafe {
                (*pPager).dbSize = (((nByte + ((pageSize as u64) as i64) - ((1 as i32) as i64))
                    / ((pageSize as u64) as i64)) as i32) as u32;
            }
            unsafe {
                (*pPager).pageSize = (pageSize as u64) as i64;
            }
            unsafe {
                (*pPager).lckPgno = (((unsafe { sqlite3PendingByte }) as u32) / pageSize)
                    .wrapping_add((1 as i32) as u32);
            }
        } else {
            unsafe { sqlite3PageFree(pNew as *mut ()) };
        }
    }
    unsafe {
        *pPageSize = ((unsafe { (*pPager).pageSize }) as i32) as u32;
    }
    if rc == (0 as i32) {
        if nReserve < (0 as i32) {
            nReserve = (unsafe { (*pPager).nReserve }) as i32;
        }
        0 as i32;
        unsafe {
            (*pPager).nReserve = nReserve as i16;
        }
        pagerFixMaplimit(pPager);
    }
    return rc;
}

// /*
// ** Attempt to set the maximum database page count if mxPage is positive.
// ** Make no changes if mxPage is zero or negative.  And never reduce the
// ** maximum page count below the current size of the database.
// **
// ** Regardless of mxPage, return the current maximum page count.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerMaxPageCount(mut pPager: *mut Pager, mut mxPage: u32) -> u32 {
    if mxPage > ((0 as i32) as u32) {
        unsafe {
            (*pPager).mxPgno = mxPage;
        }
    }
    // /* Called only by OP_MaxPgcnt */
    0 as i32;
    // /* assert( pPager->mxPgno>=pPager->dbSize ); */
    // /* OP_MaxPgcnt ensures that the parameter passed to this function is not
    //   ** less than the total number of valid pages in the database. But this
    //   ** may be less than Pager.dbSize, and so the assert() above is not valid */
    return unsafe { (*pPager).mxPgno };
}

// /*
// ** Change the maximum number of in-memory pages that are allowed
// ** before attempting to recycle clean and unused pages.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerSetCachesize(mut pPager: *mut Pager, mut mxPage: i32) {
    unsafe { sqlite3PcacheSetCachesize(unsafe { (*pPager).pPCache }, mxPage) };
}

// /*
// ** Change the maximum number of in-memory pages that are allowed
// ** before attempting to spill pages to journal.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerSetSpillsize(mut pPager: *mut Pager, mut mxPage: i32) -> i32 {
    return unsafe { sqlite3PcacheSetSpillsize(unsafe { (*pPager).pPCache }, mxPage) };
}

// /*
// ** Change the maximum size of any memory mapping made of the database file.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerSetMmapLimit(mut pPager: *mut Pager, mut szMmap: i64) {
    unsafe {
        (*pPager).szMmap = szMmap;
    }
    pagerFixMaplimit(pPager);
}

// /*
// ** Free as much memory as possible from the pager.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerShrink(mut pPager: *mut Pager) {
    unsafe { sqlite3PcacheShrink(unsafe { (*pPager).pPCache }) };
}

// /*
// ** Adjust settings of the pager to those specified in the pgFlags parameter.
// **
// ** The "level" in pgFlags & PAGER_SYNCHRONOUS_MASK sets the robustness
// ** of the database to damage due to OS crashes or power failures by
// ** changing the number of syncs()s when writing the journals.
// ** There are four levels:
// **
// **    OFF       sqlite3OsSync() is never called.  This is the default
// **              for temporary and transient files.
// **
// **    NORMAL    The journal is synced once before writes begin on the
// **              database.  This is normally adequate protection, but
// **              it is theoretically possible, though very unlikely,
// **              that an inopertune power failure could leave the journal
// **              in a state which would cause damage to the database
// **              when it is rolled back.
// **
// **    FULL      The journal is synced twice before writes begin on the
// **              database (with some additional information - the nRec field
// **              of the journal header - being written in between the two
// **              syncs).  If we assume that writing a
// **              single disk sector is atomic, then this mode provides
// **              assurance that the journal will not be corrupted to the
// **              point of causing damage to the database during rollback.
// **
// **    EXTRA     This is like FULL except that is also syncs the directory
// **              that contains the rollback journal after the rollback
// **              journal is unlinked.
// **
// ** The above is for a rollback-journal mode.  For WAL mode, OFF continues
// ** to mean that no syncs ever occur.  NORMAL means that the WAL is synced
// ** prior to the start of checkpoint and that the database file is synced
// ** at the conclusion of the checkpoint if the entire content of the WAL
// ** was written back into the database.  But no sync operations occur for
// ** an ordinary commit in NORMAL mode with WAL.  FULL means that the WAL
// ** file is synced following each commit operation, in addition to the
// ** syncs associated with NORMAL.  There is no difference between FULL
// ** and EXTRA for WAL mode.
// **
// ** Do not confuse synchronous=FULL with SQLITE_SYNC_FULL.  The
// ** SQLITE_SYNC_FULL macro means to use the MacOSX-style full-fsync
// ** using fcntl(F_FULLFSYNC).  SQLITE_SYNC_NORMAL means to do an
// ** ordinary fsync() call.  There is no difference between SQLITE_SYNC_FULL
// ** and SQLITE_SYNC_NORMAL on platforms other than MacOSX.  But the
// ** synchronous=FULL versus synchronous=NORMAL setting determines when
// ** the xSync primitive is called and is relevant to all platforms.
// **
// ** Numeric values associated with these states are OFF==1, NORMAL=2,
// ** and FULL=3.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerSetFlags(mut pPager: *mut Pager, mut pgFlags: u32) {
    let mut level: u32 = pgFlags & ((7 as i32) as u32);
    if (unsafe { (*pPager).tempFile }) != (0 as u8) || level == ((1 as i32) as u32) {
        unsafe {
            (*pPager).noSync = ((1 as i32) as i8) as u8;
        }
        unsafe {
            (*pPager).fullSync = ((0 as i32) as i8) as u8;
        }
        unsafe {
            (*pPager).extraSync = ((0 as i32) as i8) as u8;
        }
    } else {
        unsafe {
            (*pPager).noSync = ((0 as i32) as i8) as u8;
        }
        unsafe {
            (*pPager).fullSync = ((if level >= ((3 as i32) as u32) {
                1 as i32
            } else {
                0 as i32
            }) as i8) as u8;
        }
        // /* Set Pager.extraSync if "PRAGMA synchronous=EXTRA" is requested, or
        //     ** if the file-system supports F2FS style atomic writes. If this flag
        //     ** is set, SQLite syncs the directory to disk immediately after deleting
        //     ** a journal file in "PRAGMA journal_mode=DELETE" mode.  */
        if level == ((4 as i32) as u32) {
            unsafe {
                (*pPager).extraSync = ((1 as i32) as i8) as u8;
            }
        } else {
            unsafe {
                (*pPager).extraSync = ((0 as i32) as i8) as u8;
            }
        }
    }
    if (unsafe { (*pPager).noSync }) != (0 as u8) {
        unsafe {
            (*pPager).syncFlags = ((0 as i32) as i8) as u8;
        }
    } else {
        if pgFlags & ((8 as i32) as u32) != (0 as u32) {
            unsafe {
                (*pPager).syncFlags = ((3 as i32) as i8) as u8;
            }
        } else {
            unsafe {
                (*pPager).syncFlags = ((2 as i32) as i8) as u8;
            }
        }
    }
    unsafe {
        (*pPager).walSyncFlags =
            (((((unsafe { (*pPager).syncFlags }) as u32) as i32) << (2 as i32)) as i8) as u8;
    }
    if (unsafe { (*pPager).fullSync }) != (0 as u8) {
        let __v1593: *mut Pager = pPager;
        let __v1594: u8 = unsafe { (*__v1593).walSyncFlags };
        let __v1595: u8 = ((((__v1594 as u32) as i32)
            | (((unsafe { (*pPager).syncFlags }) as u32) as i32)) as i8)
            as u8;
        unsafe {
            (*__v1593).walSyncFlags = __v1595;
        }
    }
    if pgFlags & ((16 as i32) as u32) != (0 as u32) && !((unsafe { (*pPager).noSync }) != (0 as u8))
    {
        let __v1596: *mut Pager = pPager;
        let __v1597: u8 = unsafe { (*__v1596).walSyncFlags };
        let __v1598: u8 = ((((__v1597 as u32) as i32) | (3 as i32) << (2 as i32)) as i8) as u8;
        unsafe {
            (*__v1596).walSyncFlags = __v1598;
        }
    }
    if pgFlags & ((32 as i32) as u32) != (0 as u32) {
        let __v1599: *mut Pager = pPager;
        let __v1600: u8 = unsafe { (*__v1599).doNotSpill };
        let __v1601: u8 = ((((__v1600 as u32) as i32) & !(1 as i32)) as i8) as u8;
        unsafe {
            (*__v1599).doNotSpill = __v1601;
        }
    } else {
        let __v1602: *mut Pager = pPager;
        let __v1603: u8 = unsafe { (*__v1602).doNotSpill };
        let __v1604: u8 = ((((__v1603 as u32) as i32) | (1 as i32)) as i8) as u8;
        unsafe {
            (*__v1602).doNotSpill = __v1604;
        }
    }
}

// /*
// ** Get/set the locking-mode for this pager. Parameter eMode must be one
// ** of PAGER_LOCKINGMODE_QUERY, PAGER_LOCKINGMODE_NORMAL or
// ** PAGER_LOCKINGMODE_EXCLUSIVE. If the parameter is not _QUERY, then
// ** the locking-mode is set to the value specified.
// **
// ** The returned value is either PAGER_LOCKINGMODE_NORMAL or
// ** PAGER_LOCKINGMODE_EXCLUSIVE, indicating the current (possibly updated)
// ** locking-mode.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerLockingMode(mut pPager: *mut Pager, mut eMode: i32) -> i32 {
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    let __v1605: bool;
    if eMode >= (0 as i32) && !((unsafe { (*pPager).tempFile }) != (0 as u8)) {
        __v1605 = !((unsafe { sqlite3WalHeapMemory(unsafe { (*pPager).pWal }) }) != (0 as i32));
    } else {
        __v1605 = false as bool;
    }
    if __v1605 {
        unsafe {
            (*pPager).exclusiveMode = (eMode as i8) as u8;
        }
    }
    return ((unsafe { (*pPager).exclusiveMode }) as u32) as i32;
}

// /*
// ** Set the journal-mode for this pager. Parameter eMode must be one of:
// **
// **    PAGER_JOURNALMODE_DELETE
// **    PAGER_JOURNALMODE_TRUNCATE
// **    PAGER_JOURNALMODE_PERSIST
// **    PAGER_JOURNALMODE_OFF
// **    PAGER_JOURNALMODE_MEMORY
// **    PAGER_JOURNALMODE_WAL
// **
// ** The journalmode is set to the value specified if the change is allowed.
// ** The change may be disallowed for the following reasons:
// **
// **   *  An in-memory database can only have its journal_mode set to _OFF
// **      or _MEMORY.
// **
// **   *  Temporary databases cannot have _WAL journalmode.
// **
// ** The returned indicate the current (possibly updated) journal-mode.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerSetJournalMode(mut pPager: *mut Pager, mut eMode: i32) -> i32 {
    // /* Prior journalmode */
    let mut eOld: u8 = unsafe { (*pPager).journalMode };
    // /* The eMode parameter is always valid */
    // /* 0 */
    0 as i32;
    // /* 1 */
    // /* 2 */
    // /* 3 */
    // /* 4 */
    // /* 5 */
    // /* This routine is only called from the OP_JournalMode opcode, and
    //   ** the logic there will never allow a temporary file to be changed
    //   ** to WAL mode.
    //   */
    0 as i32;
    // /* Do allow the journalmode of an in-memory database to be set to
    //   ** anything other than MEMORY or OFF
    //   */
    if (unsafe { (*pPager).memDb }) != (0 as u8) {
        0 as i32;
        if eMode != (4 as i32) && eMode != (2 as i32) {
            eMode = (eOld as u32) as i32;
        }
    }
    if eMode != ((eOld as u32) as i32) {
        // /* Change the journal mode. */
        0 as i32;
        unsafe {
            (*pPager).journalMode = (eMode as i8) as u8;
        }
        // /* When transitioning from TRUNCATE or PERSIST to any other journal
        //     ** mode except WAL, unless the pager is in locking_mode=exclusive mode,
        //     ** delete the journal file.
        //     */
        0 as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        if !((unsafe { (*pPager).exclusiveMode }) != (0 as u8))
            && ((eOld as u32) as i32) & (5 as i32) == (1 as i32)
            && eMode & (1 as i32) == (0 as i32)
        {
            // /* In this case we would like to delete the journal file. If it is
            //       ** not possible, then that is not a problem. Deleting the journal file
            //       ** here is an optimization only.
            //       **
            //       ** Before deleting the journal file, obtain a RESERVED lock on the
            //       ** database file. This ensures that the journal file is not deleted
            //       ** while it is in use by some other client.
            //       */
            unsafe { sqlite3OsClose(unsafe { (*pPager).jfd }) };
            if (((unsafe { (*pPager).eLock }) as u32) as i32) >= (2 as i32) {
                unsafe {
                    sqlite3OsDelete(
                        unsafe { (*pPager).pVfs },
                        (unsafe { (*pPager).zJournal }) as *const i8,
                        0 as i32,
                    )
                };
            } else {
                let mut rc: i32 = 0 as i32;
                let mut state: i32 = ((unsafe { (*pPager).eState }) as u32) as i32;
                0 as i32;
                if state == (0 as i32) {
                    rc = sqlite3PagerSharedLock(pPager);
                }
                if (((unsafe { (*pPager).eState }) as u32) as i32) == (1 as i32) {
                    0 as i32;
                    rc = pagerLockDb(pPager, 2 as i32);
                }
                if rc == (0 as i32) {
                    unsafe {
                        sqlite3OsDelete(
                            unsafe { (*pPager).pVfs },
                            (unsafe { (*pPager).zJournal }) as *const i8,
                            0 as i32,
                        )
                    };
                }
                if rc == (0 as i32) && state == (1 as i32) {
                    pagerUnlockDb(pPager, 1 as i32);
                } else {
                    if state == (0 as i32) {
                        pager_unlock(pPager);
                    }
                }
                0 as i32;
            }
        } else {
            if eMode == (2 as i32) || eMode == (4 as i32) {
                unsafe { sqlite3OsClose(unsafe { (*pPager).jfd }) };
            }
        }
    }
    // /* Return the new journal mode */
    return ((unsafe { (*pPager).journalMode }) as u32) as i32;
}

// /*
// ** Return the current journal mode.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerGetJournalMode(mut pPager: *mut Pager) -> i32 {
    return ((unsafe { (*pPager).journalMode }) as u32) as i32;
}

// /*
// ** Return TRUE if the pager is in a state where it is OK to change the
// ** journalmode.  Journalmode changes can only happen when the database
// ** is unmodified.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerOkToChangeJournalMode(mut pPager: *mut Pager) -> i32 {
    0 as i32;
    if (((unsafe { (*pPager).eState }) as u32) as i32) >= (3 as i32) {
        return 0 as i32;
    }
    if (unsafe { (*unsafe { (*pPager).jfd }).pMethods }) != std::ptr::null::<sqlite3_io_methods>()
        && (unsafe { (*pPager).journalOff }) > ((0 as i32) as i64)
    {
        return 0 as i32;
    }
    return 1 as i32;
}

// /*
// ** Get/set the size-limit used for persistent journal files.
// **
// ** Setting the size limit to -1 means no limit is enforced.
// ** An attempt to set a limit smaller than -1 is a no-op.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerJournalSizeLimit(mut pPager: *mut Pager, mut iLimit: i64) -> i64 {
    if iLimit >= (-(1 as i32) as i64) {
        unsafe {
            (*pPager).journalSizeLimit = iLimit;
        }
        unsafe { sqlite3WalLimit(unsafe { (*pPager).pWal }, iLimit) };
    }
    return unsafe { (*pPager).journalSizeLimit };
}

// /*
// ** Return a pointer to the pPager->pBackup variable. The backup module
// ** in backup.c maintains the content of this variable. This module
// ** uses it opaquely as an argument to sqlite3BackupRestart() and
// ** sqlite3BackupUpdate() only.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerBackupPtr(mut pPager: *mut Pager) -> *mut *mut sqlite3_backup {
    return unsafe { std::ptr::addr_of_mut!((*pPager).pBackup) };
}

// /*
// ** Flush all unreferenced dirty pages to disk.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerFlush(mut pPager: *mut Pager) -> i32 {
    let mut rc: i32 = unsafe { (*pPager).errCode };
    if !((unsafe { (*pPager).memDb }) != (0 as u8)) {
        let mut pList: *mut PgHdr = unsafe { sqlite3PcacheDirtyList(unsafe { (*pPager).pPCache }) };
        0 as i32;
        '__slate_break_1537: while rc == (0 as i32) && pList != std::ptr::null_mut::<PgHdr>() {
            let mut pNext: *mut PgHdr = unsafe { (*pList).pDirty };
            if (unsafe { (*pList).nRef }) == ((0 as i32) as i64) {
                rc = pagerStress(pPager as *mut (), pList);
            }
            pList = pNext;
        }
    }
    return rc;
}

// /* The pager open on the database file */
// /* Page number to fetch */
// /* Write a pointer to the page here */
// /* PAGER_GET_XXX flags */
// /* Dispatch all page fetch requests to the appropriate getter method.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerGet(
    mut pPager: *mut Pager,
    mut pgno: u32,
    mut ppPage: *mut *mut PgHdr,
    mut flags: i32,
) -> i32 {
    // /* Trace page fetch by setting to 1 */
    // /* Normal, high-speed version of sqlite3PagerGet() */
    return unsafe { unsafe { (*pPager).xGet }.unwrap()(pPager, pgno, ppPage, flags) };
}

// /* The pager open on the database file */
// /* Page number to fetch */
// /* Write a pointer to the page here */
// /* PAGER_GET_XXX flags */
// /*
// ** Acquire a page if it is already in the in-memory cache.  Do
// ** not read the page from disk.  Return a pointer to the page,
// ** or 0 if the page is not in cache.
// **
// ** See also sqlite3PagerGet().  The difference between this routine
// ** and sqlite3PagerGet() is that _get() will go to the disk and read
// ** in the page if the page is not already in cache.  This routine
// ** returns NULL if the page is not in cache or if a disk I/O error
// ** has ever happened.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerLookup(mut pPager: *mut Pager, mut pgno: u32) -> *mut PgHdr {
    let mut pPage: *mut sqlite3_pcache_page = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    pPage = unsafe { sqlite3PcacheFetch(unsafe { (*pPager).pPCache }, pgno, 0 as i32) };
    0 as i32;
    if pPage == std::ptr::null_mut::<sqlite3_pcache_page>() {
        return std::ptr::null_mut::<PgHdr>();
    }
    return unsafe { sqlite3PcacheFetchFinish(unsafe { (*pPager).pPCache }, pgno, pPage) };
}

// /*
// ** Increment the reference count for page pPg.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerRef(mut pPg: *mut PgHdr) {
    unsafe { sqlite3PcacheRef(pPg) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerUnref(mut pPg: *mut PgHdr) {
    if pPg != std::ptr::null_mut::<PgHdr>() {
        sqlite3PagerUnrefNotNull(pPg);
    }
}

// /*
// ** Release a page reference.
// **
// ** The sqlite3PagerUnref() and sqlite3PagerUnrefNotNull() may only be used
// ** if we know that the page being released is not the last reference to page1.
// ** The btree layer always holds page1 open until the end, so these first
// ** two routines can be used to release any page other than BtShared.pPage1.
// ** The assert() at tag-20230419-2 proves that this constraint is always
// ** honored.
// **
// ** Use sqlite3PagerUnrefPageOne() to release page1.  This latter routine
// ** checks the total number of outstanding pages and if the number of
// ** pages reaches zero it drops the database lock.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerUnrefNotNull(mut pPg: *mut PgHdr) {
    0 as i32;
    if (((unsafe { (*pPg).flags }) as u32) as i32) & (32 as i32) != (0 as i32) {
        // /* Page1 is never memory mapped */
        0 as i32;
        pagerReleaseMapPage(pPg);
    } else {
        unsafe { sqlite3PcacheRelease(pPg) };
    }
    // /* Do not use this routine to release the last reference to page1 */
    // /* tag-20230419-2 */
    0 as i32;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerUnrefPageOne(mut pPg: *mut PgHdr) {
    let mut pPager: *mut Pager = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    // /* Page1 is never memory mapped */
    0 as i32;
    pPager = unsafe { (*pPg).pPager };
    unsafe { sqlite3PcacheRelease(pPg) };
    pagerUnlockIfUnused(pPager);
}

// /*
// ** Mark a data page as writeable. This routine must be called before
// ** making changes to a page. The caller must check the return value
// ** of this function and be careful not to change any page data unless
// ** this routine returns SQLITE_OK.
// **
// ** The difference between this function and pager_write() is that this
// ** function also deals with the special case where 2 or more pages
// ** fit on a single disk sector. In this case all co-resident pages
// ** must have been written to the journal file before returning.
// **
// ** If an error occurs, SQLITE_NOMEM or an IO error code is returned
// ** as appropriate. Otherwise, SQLITE_OK.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerWrite(mut pPg: *mut PgHdr) -> i32 {
    let mut pPager: *mut Pager = unsafe { (*pPg).pPager };
    0 as i32;
    0 as i32;
    0 as i32;
    if (((unsafe { (*pPg).flags }) as u32) as i32) & (4 as i32) != (0 as i32)
        && (unsafe { (*pPager).dbSize }) >= unsafe { (*pPg).pgno }
    {
        if (unsafe { (*pPager).nSavepoint }) != (0 as i32) {
            return subjournalPageIfRequired(pPg);
        }
        return 0 as i32;
    } else {
        if (unsafe { (*pPager).errCode }) != (0 as i32) {
            return unsafe { (*pPager).errCode };
        } else {
            if (unsafe { (*pPager).sectorSize }) > (((unsafe { (*pPager).pageSize }) as i32) as u32)
            {
                0 as i32;
                return pagerWriteLargeSector(pPg);
            } else {
                return pager_write(pPg);
            }
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Return TRUE if the page given in the argument was previously passed
// ** to sqlite3PagerWrite().  In other words, return TRUE if it is ok
// ** to change the content of the page.
// */
// /*
// ** A call to this routine tells the pager that it is not necessary to
// ** write the information on page pPg back to the disk, even though
// ** that page might be marked as dirty.  This happens, for example, when
// ** the page has been added as a leaf of the freelist and so its
// ** content no longer matters.
// **
// ** The overlying software layer calls this routine when all of the data
// ** on the given page is unused. The pager marks the page as clean so
// ** that it does not get written to disk.
// **
// ** Tests show that this optimization can quadruple the speed of large
// ** DELETE operations.
// **
// ** This optimization cannot be used with a temp-file, as the page may
// ** have been dirty at the start of the transaction. In that case, if
// ** memory pressure forces page pPg out of the cache, the data does need
// ** to be written out to disk so that it may be read back in if the
// ** current transaction is rolled back.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerDontWrite(mut pPg: *mut PgHdr) {
    let mut pPager: *mut Pager = unsafe { (*pPg).pPager };
    if !((unsafe { (*pPager).tempFile }) != (0 as u8))
        && (((unsafe { (*pPg).flags }) as u32) as i32) & (2 as i32) != (0 as i32)
        && (unsafe { (*pPager).nSavepoint }) == (0 as i32)
    {
        {}
        let __v1606: *mut PgHdr = pPg;
        let __v1607: u16 = unsafe { (*__v1606).flags };
        let __v1608: u16 = ((((__v1607 as u32) as i32) | (16 as i32)) as i16) as u16;
        unsafe {
            (*__v1606).flags = __v1608;
        }
        let __v1609: *mut PgHdr = pPg;
        let __v1610: u16 = unsafe { (*__v1609).flags };
        let __v1611: u16 = ((((__v1610 as u32) as i32) & !(4 as i32)) as i16) as u16;
        unsafe {
            (*__v1609).flags = __v1611;
        }
        {}
        {}
    }
}

// /*
// ** Move the page pPg to location pgno in the file.
// **
// ** There must be no references to the page previously located at
// ** pgno (which we call pPgOld) though that page is allowed to be
// ** in cache.  If the page previously located at pgno is not already
// ** in the rollback journal, it is not put there by by this routine.
// **
// ** References to the page pPg remain valid. Updating any
// ** meta-data associated with pPg (i.e. data stored in the nExtra bytes
// ** allocated along with the page) is the responsibility of the caller.
// **
// ** A transaction must be active when this routine is called. It used to be
// ** required that a statement transaction was not active, but this restriction
// ** has been removed (CREATE INDEX needs to move a page when a statement
// ** transaction is active).
// **
// ** If the fourth argument, isCommit, is non-zero, then this page is being
// ** moved as part of a database reorganization just before the transaction
// ** is being committed. In this case, it is guaranteed that the database page
// ** pPg refers to will not be written to again within this transaction.
// **
// ** This function may return SQLITE_NOMEM or an IO error code if an error
// ** occurs. Otherwise, it returns SQLITE_OK.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerMovepage(
    mut pPager: *mut Pager,
    mut pPg: *mut PgHdr,
    mut pgno: u32,
    mut isCommit: i32,
) -> i32 {
    // /* The page being overwritten. */
    let mut pPgOld: *mut PgHdr = unsafe { std::mem::zeroed() };
    // /* Old value of pPg->pgno, if sync is required */
    let mut needSyncPgno: u32 = (0 as i32) as u32;
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* The original page number */
    let mut origPgno: u32 = 0 as u32;
    0 as i32;
    0 as i32;
    0 as i32;
    // /* In order to be able to rollback, an in-memory database must journal
    //   ** the page we are moving from.
    //   */
    0 as i32;
    if (unsafe { (*pPager).tempFile }) != (0 as u8) {
        rc = sqlite3PagerWrite(pPg);
        if rc != (0 as i32) {
            return rc;
        }
    }
    // /* If the page being moved is dirty and has not been saved by the latest
    //   ** savepoint, then save the current contents of the page into the
    //   ** sub-journal now. This is required to handle the following scenario:
    //   **
    //   **   BEGIN;
    //   **     <journal page X, then modify it in memory>
    //   **     SAVEPOINT one;
    //   **       <Move page X to location Y>
    //   **     ROLLBACK TO one;
    //   **
    //   ** If page X were not written to the sub-journal here, it would not
    //   ** be possible to restore its contents when the "ROLLBACK TO one"
    //   ** statement were is processed.
    //   **
    //   ** subjournalPage() may need to allocate space to store pPg->pgno into
    //   ** one or more savepoint bitvecs. This is the reason this function
    //   ** may return SQLITE_NOMEM.
    //   */
    let __v1612: bool;
    if (((unsafe { (*pPg).flags }) as u32) as i32) & (2 as i32) != (0 as i32) {
        let __v1613: i32 = subjournalPageIfRequired(pPg);
        rc = __v1613;
        __v1612 = (0 as i32) != __v1613;
    } else {
        __v1612 = false as bool;
    }
    if __v1612 {
        return rc;
    }
    {}
    // /* If the journal needs to be sync()ed before page pPg->pgno can
    //   ** be written to, store pPg->pgno in local variable needSyncPgno.
    //   **
    //   ** If the isCommit flag is set, there is no need to remember that
    //   ** the journal needs to be sync()ed before database page pPg->pgno
    //   ** can be written to. The caller has already promised not to write to it.
    //   */
    if (((unsafe { (*pPg).flags }) as u32) as i32) & (8 as i32) != (0 as i32)
        && !(isCommit != (0 as i32))
    {
        needSyncPgno = unsafe { (*pPg).pgno };
        0 as i32;
        0 as i32;
    }
    // /* If the cache contains a page with page-number pgno, remove it
    //   ** from its hash chain. Also, if the PGHDR_NEED_SYNC flag was set for
    //   ** page pgno before the 'move' operation, it needs to be retained
    //   ** for the page moved there.
    //   */
    let __v1614: *mut PgHdr = pPg;
    let __v1615: u16 = unsafe { (*__v1614).flags };
    let __v1616: u16 = ((((__v1615 as u32) as i32) & !(8 as i32)) as i16) as u16;
    unsafe {
        (*__v1614).flags = __v1616;
    }
    pPgOld = sqlite3PagerLookup(pPager, pgno);
    0 as i32;
    if pPgOld != std::ptr::null_mut::<PgHdr>() {
        if (unsafe { (*pPgOld).nRef }) > ((1 as i32) as i64) {
            sqlite3PagerUnrefNotNull(pPgOld);
            return unsafe { sqlite3CorruptError(7294 as i32) };
        }
        let __v1617: *mut PgHdr = pPg;
        let __v1618: u16 = unsafe { (*__v1617).flags };
        let __v1619: u16 = ((((__v1618 as u32) as i32)
            | (((unsafe { (*pPgOld).flags }) as u32) as i32) & (8 as i32))
            as i16) as u16;
        unsafe {
            (*__v1617).flags = __v1619;
        }
        if (unsafe { (*pPager).tempFile }) != (0 as u8) {
            // /* Do not discard pages from an in-memory database since we might
            //       ** need to rollback later.  Just move the page out of the way. */
            unsafe {
                sqlite3PcacheMove(
                    pPgOld,
                    unsafe { (*pPager).dbSize }.wrapping_add((1 as i32) as u32),
                )
            };
        } else {
            unsafe { sqlite3PcacheDrop(pPgOld) };
        }
    }
    origPgno = unsafe { (*pPg).pgno };
    unsafe { sqlite3PcacheMove(pPg, pgno) };
    unsafe { sqlite3PcacheMakeDirty(pPg) };
    // /* For an in-memory database, make sure the original page continues
    //   ** to exist, in case the transaction needs to roll back.  Use pPgOld
    //   ** as the original page since it has already been allocated.
    //   */
    if (unsafe { (*pPager).tempFile }) != (0 as u8) && pPgOld != std::ptr::null_mut::<PgHdr>() {
        unsafe { sqlite3PcacheMove(pPgOld, origPgno) };
        sqlite3PagerUnrefNotNull(pPgOld);
    }
    if needSyncPgno != (0 as u32) {
        // /* If needSyncPgno is non-zero, then the journal file needs to be
        //     ** sync()ed before any data is written to database file page needSyncPgno.
        //     ** Currently, no such page exists in the page-cache and the
        //     ** "is journaled" bitvec flag has been set. This needs to be remedied by
        //     ** loading the page into the pager-cache and setting the PGHDR_NEED_SYNC
        //     ** flag.
        //     **
        //     ** If the attempt to load the page into the page-cache fails, (due
        //     ** to a malloc() or IO failure), clear the bit in the pInJournal[]
        //     ** array. Otherwise, if the page is loaded and written again in
        //     ** this transaction, it may be written to the database file before
        //     ** it is synced into the journal file. This way, it may end up in
        //     ** the journal file twice, but that is not a problem.
        //     */
        let mut pPgHdr: *mut PgHdr = unsafe { std::mem::zeroed() };
        rc = sqlite3PagerGet(
            pPager,
            needSyncPgno,
            std::ptr::addr_of_mut!(pPgHdr),
            0 as i32,
        );
        if rc != (0 as i32) {
            if needSyncPgno <= unsafe { (*pPager).dbOrigSize } {
                0 as i32;
                unsafe {
                    sqlite3BitvecClear(
                        unsafe { (*pPager).pInJournal },
                        needSyncPgno,
                        (unsafe { (*pPager).pTmpSpace }) as *mut (),
                    )
                };
            }
            return rc;
        }
        let __v1620: *mut PgHdr = pPgHdr;
        let __v1621: u16 = unsafe { (*__v1620).flags };
        let __v1622: u16 = ((((__v1621 as u32) as i32) | (8 as i32)) as i16) as u16;
        unsafe {
            (*__v1620).flags = __v1622;
        }
        unsafe { sqlite3PcacheMakeDirty(pPgHdr) };
        sqlite3PagerUnrefNotNull(pPgHdr);
    }
    return 0 as i32;
}

// /*
// ** Return the number of references to the specified page.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerPageRefcount(mut pPage: *mut PgHdr) -> i32 {
    return (unsafe { sqlite3PcachePageRefcount(pPage) }) as i32;
}

// /*
// ** Return a pointer to the data for the specified page.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerGetData(mut pPg: *mut PgHdr) -> *mut () {
    0 as i32;
    return unsafe { (*pPg).pData };
}

// /*
// ** Return a pointer to the Pager.nExtra bytes of "extra" space
// ** allocated along with the specified page.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerGetExtra(mut pPg: *mut PgHdr) -> *mut () {
    return unsafe { (*pPg).pExtra };
}

// /*
// ** This function may only be called when a read-transaction is open on
// ** the pager. It returns the total number of pages in the database.
// **
// ** However, if the file is between 1 and <page-size> bytes in size, then
// ** this is considered a 1 page file.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerPagecount(mut pPager: *mut Pager, mut pnPage: *mut i32) {
    0 as i32;
    0 as i32;
    unsafe {
        *pnPage = (unsafe { (*pPager).dbSize }) as i32;
    }
}

// /*
// ** Begin a write-transaction on the specified pager object. If a
// ** write-transaction has already been opened, this function is a no-op.
// **
// ** If the exFlag argument is false, then acquire at least a RESERVED
// ** lock on the database file. If exFlag is true, then acquire at least
// ** an EXCLUSIVE lock. If such a lock is already held, no locking
// ** functions need be called.
// **
// ** If the subjInMemory argument is non-zero, then any sub-journal opened
// ** within this transaction will be opened as an in-memory file. This
// ** has no effect if the sub-journal is already opened (as it may be when
// ** running in exclusive mode) or if the transaction does not require a
// ** sub-journal. If the subjInMemory argument is zero, then any required
// ** sub-journal is implemented in-memory if pPager is an in-memory database,
// ** or using a temporary file otherwise.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerBegin(
    mut pPager: *mut Pager,
    mut exFlag: i32,
    mut subjInMemory: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pPager).errCode }) != (0 as i32) {
        return unsafe { (*pPager).errCode };
    }
    0 as i32;
    unsafe {
        (*pPager).subjInMemory = (subjInMemory as i8) as u8;
    }
    if (((unsafe { (*pPager).eState }) as u32) as i32) == (1 as i32) {
        0 as i32;
        if (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
            // /* If the pager is configured to use locking_mode=exclusive, and an
            //       ** exclusive lock on the database is not already held, obtain it now.
            //       */
            let __v1623: bool;
            if (unsafe { (*pPager).exclusiveMode }) != (0 as u8) {
                __v1623 =
                    (unsafe { sqlite3WalExclusiveMode(unsafe { (*pPager).pWal }, -(1 as i32)) })
                        != (0 as i32);
            } else {
                __v1623 = false as bool;
            }
            if __v1623 {
                rc = pagerLockDb(pPager, 4 as i32);
                if rc != (0 as i32) {
                    return rc;
                }
                unsafe { sqlite3WalExclusiveMode(unsafe { (*pPager).pWal }, 1 as i32) };
            }
            // /* Grab the write lock on the log file. If successful, upgrade to
            //       ** PAGER_RESERVED state. Otherwise, return an error code to the caller.
            //       ** The busy-handler is not invoked if another connection already
            //       ** holds the write-lock. If possible, the upper layer will call it.
            //       */
            rc = unsafe { sqlite3WalBeginWriteTransaction(unsafe { (*pPager).pWal }) };
        } else {
            // /* Obtain a RESERVED lock on the database file. If the exFlag parameter
            //       ** is true, then immediately upgrade this to an EXCLUSIVE lock. The
            //       ** busy-handler callback can be used when upgrading to the EXCLUSIVE
            //       ** lock, but not when obtaining the RESERVED lock.
            //       */
            rc = pagerLockDb(pPager, 2 as i32);
            if rc == (0 as i32) && exFlag != (0 as i32) {
                rc = pager_wait_on_lock(pPager, 4 as i32);
            }
        }
        if rc == (0 as i32) {
            // /* Change to WRITER_LOCKED state.
            //       **
            //       ** WAL mode sets Pager.eState to PAGER_WRITER_LOCKED or CACHEMOD
            //       ** when it has an open transaction, but never to DBMOD or FINISHED.
            //       ** This is because in those states the code to roll back savepoint
            //       ** transactions may copy data from the sub-journal into the database
            //       ** file as well as into the page cache. Which would be incorrect in
            //       ** WAL mode.
            //       */
            unsafe {
                (*pPager).eState = ((2 as i32) as i8) as u8;
            }
            unsafe {
                (*pPager).dbHintSize = unsafe { (*pPager).dbSize };
            }
            unsafe {
                (*pPager).dbFileSize = unsafe { (*pPager).dbSize };
            }
            unsafe {
                (*pPager).dbOrigSize = unsafe { (*pPager).dbSize };
            }
            unsafe {
                (*pPager).journalOff = (0 as i32) as i64;
            }
        }
        0 as i32;
        0 as i32;
        0 as i32;
    }
    {}
    return rc;
}

// /*
// ** Sync the database file for the pager pPager. zSuper points to the name
// ** of a super-journal file that should be written into the individual
// ** journal file. zSuper may be NULL, which is interpreted as no
// ** super-journal (a single database transaction).
// **
// ** This routine ensures that:
// **
// **   * The database file change-counter is updated,
// **   * the journal is synced (unless the atomic-write optimization is used),
// **   * all dirty pages are written to the database file,
// **   * the database file is truncated (if required), and
// **   * the database file synced.
// **
// ** The only thing that remains to commit the transaction is to finalize
// ** (delete, truncate or zero the first part of) the journal file (or
// ** delete the super-journal file if specified).
// **
// ** Note that if zSuper==NULL, this does not overwrite a previous value
// ** passed to an sqlite3PagerCommitPhaseOne() call.
// **
// ** If the final parameter - noSync - is true, then the database file itself
// ** is not synced. The caller must call sqlite3PagerSync() directly to
// ** sync the database file before calling CommitPhaseTwo() to delete the
// ** journal file in this case.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerCommitPhaseOne(
    mut pPager: *mut Pager,
    mut zSuper: *const i8,
    mut noSync: i32,
) -> i32 {
    let mut __slate_storage_1074: std::mem::MaybeUninit<*mut PgHdr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1074: *mut *mut PgHdr =
        std::ptr::addr_of_mut!(__slate_storage_1074) as *mut *mut PgHdr;
    let mut __slate_storage_1075: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1075: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1075) as *mut u32;
    let mut __slate_storage_1073: std::mem::MaybeUninit<*mut PgHdr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1073: *mut *mut PgHdr =
        std::ptr::addr_of_mut!(__slate_storage_1073) as *mut *mut PgHdr;
    let mut __slate_storage_1072: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1072: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1072) as *mut i32;
    unsafe {
        // /* Return code */
        std::ptr::write(__slate_slot_1072, 0 as i32);
        0 as i32;
        0 as i32;
        // /* If a prior error occurred, report that error again. */
        if (unsafe { (*pPager).errCode }) != (0 as i32) {
            return unsafe { (*pPager).errCode };
        } else {
            // /* Provide the ability to easily simulate an I/O error during testing */
            if (unsafe { sqlite3FaultSim(400 as i32) }) != (0 as i32) {
                return 10 as i32;
            } else {
                {}
                // /* If no database changes have been made, return early. */
                if (((unsafe { (*pPager).eState }) as u32) as i32) < (3 as i32) {
                    return 0 as i32;
                } else {
                    '__join_2: {
                        0 as i32;
                        0 as i32;
                        if (0 as i32) == pagerFlushOnCommit(pPager, 1 as i32) {
                            // /* If this is an in-memory db, or no pages have been written to, or this
                            //     ** function has already been called, it is mostly a no-op.  However, any
                            //     ** backup in progress needs to be restarted.  */
                            unsafe { sqlite3BackupRestart(unsafe { (*pPager).pBackup }) };
                        } else {
                            if (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
                                std::ptr::write(__slate_slot_1074, std::ptr::null_mut::<PgHdr>());
                                *__slate_slot_1073 =
                                    unsafe { sqlite3PcacheDirtyList(unsafe { (*pPager).pPCache }) };
                                if *__slate_slot_1073 == std::ptr::null_mut::<PgHdr>() {
                                    // /* Must have at least one page for the WAL commit flag.
                                    //         ** Ticket [2d1a5c67dfc2363e44f29d9bbd57f] 2011-05-18 */
                                    *__slate_slot_1072 = sqlite3PagerGet(
                                        pPager,
                                        (1 as i32) as u32,
                                        std::ptr::addr_of_mut!(*__slate_slot_1074),
                                        0 as i32,
                                    );
                                    *__slate_slot_1073 = *__slate_slot_1074;
                                    unsafe {
                                        (*(*__slate_slot_1073)).pDirty =
                                            std::ptr::null_mut::<PgHdr>();
                                    }
                                }
                                0 as i32;
                                if *__slate_slot_1073 != std::ptr::null_mut::<PgHdr>() {
                                    *__slate_slot_1072 = pagerWalFrames(
                                        pPager,
                                        *__slate_slot_1073,
                                        unsafe { (*pPager).dbSize },
                                        1 as i32,
                                    );
                                }
                                sqlite3PagerUnref(*__slate_slot_1074);
                                if *__slate_slot_1072 == (0 as i32) {
                                    unsafe { sqlite3PcacheCleanAll(unsafe { (*pPager).pPCache }) };
                                }
                            } else {
                                // /* The bBatch boolean is true if the batch-atomic-write commit method
                                //       ** should be used.  No rollback journal is created if batch-atomic-write
                                //       ** is enabled.
                                //       */
                                *__slate_slot_1072 = pager_incr_changecounter(pPager, 0 as i32);
                                // /* !SQLITE_ENABLE_ATOMIC_WRITE */
                                if *__slate_slot_1072 != (0 as i32) {
                                } else {
                                    // /* Write the super-journal name into the journal file. If a
                                    //       ** super-journal file name has already been written to the journal file,
                                    //       ** or if zSuper is NULL (no super-journal), then this call is a no-op.
                                    //       */
                                    *__slate_slot_1072 = writeSuperJournal(pPager, zSuper);
                                    if *__slate_slot_1072 != (0 as i32) {
                                    } else {
                                        // /* Sync the journal file and write all dirty pages to the database.
                                        //       ** If the atomic-update optimization is being used, this sync will not
                                        //       ** create the journal file or perform any real IO.
                                        //       **
                                        //       ** Because the change-counter page was just modified, unless the
                                        //       ** atomic-update optimization is used it is almost certain that the
                                        //       ** journal requires a sync here. However, in locking_mode=exclusive
                                        //       ** on a system under memory pressure it is just possible that this is
                                        //       ** not the case. In this case it is likely enough that the redundant
                                        //       ** xSync() call will be changed to a no-op by the OS anyhow.
                                        //       */
                                        *__slate_slot_1072 = syncJournal(pPager, 0 as i32);
                                        if *__slate_slot_1072 != (0 as i32) {
                                        } else {
                                            *__slate_slot_1073 = unsafe {
                                                sqlite3PcacheDirtyList(unsafe { (*pPager).pPCache })
                                            };
                                            if (0 as i32) == (0 as i32) {
                                                *__slate_slot_1072 = pager_write_pagelist(
                                                    pPager,
                                                    *__slate_slot_1073,
                                                );
                                            }
                                            if *__slate_slot_1072 != (0 as i32) {
                                                0 as i32;
                                            } else {
                                                unsafe {
                                                    sqlite3PcacheCleanAll(unsafe {
                                                        (*pPager).pPCache
                                                    })
                                                };
                                                // /* If the file on disk is smaller than the database image, use
                                                //       ** pager_truncate to grow the file here. This can happen if the database
                                                //       ** image was extended as part of the current transaction and then the
                                                //       ** last page in the db image moved to the free-list. In this case the
                                                //       ** last page is never written out to disk, leaving the database file
                                                //       ** undersized. Fix this now if it is the case.  */
                                                if (unsafe { (*pPager).dbSize })
                                                    > unsafe { (*pPager).dbFileSize }
                                                {
                                                    std::ptr::write(
                                                        __slate_slot_1075,
                                                        unsafe { (*pPager).dbSize }.wrapping_sub(
                                                            (((unsafe { (*pPager).dbSize })
                                                                == unsafe { (*pPager).lckPgno })
                                                                as i32)
                                                                as u32,
                                                        ),
                                                    );
                                                    0 as i32;
                                                    *__slate_slot_1072 =
                                                        pager_truncate(pPager, *__slate_slot_1075);
                                                    if *__slate_slot_1072 != (0 as i32) {
                                                        break '__join_2;
                                                    }
                                                }
                                                // /* Finally, sync the database file. */
                                                if !(noSync != (0 as i32)) {
                                                    *__slate_slot_1072 =
                                                        sqlite3PagerSync(pPager, zSuper);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if *__slate_slot_1072 == (0 as i32)
                        && !((unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>())
                    {
                        unsafe {
                            (*pPager).eState = ((5 as i32) as i8) as u8;
                        }
                    }
                    return *__slate_slot_1072;
                }
            }
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** This function may only be called while a write-transaction is active in
// ** rollback. If the connection is in WAL mode, this call is a no-op.
// ** Otherwise, if the connection does not already have an EXCLUSIVE lock on
// ** the database file, an attempt is made to obtain one.
// **
// ** If the EXCLUSIVE lock is already held or the attempt to obtain it is
// ** successful, or the connection is in WAL mode, SQLITE_OK is returned.
// ** Otherwise, either SQLITE_BUSY or an SQLITE_IOERR_XXX error code is
// ** returned.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerExclusiveLock(mut pPager: *mut Pager) -> i32 {
    let mut rc: i32 = unsafe { (*pPager).errCode };
    0 as i32;
    if rc == (0 as i32) {
        0 as i32;
        0 as i32;
        if (0 as i32) == (((unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>()) as i32) {
            rc = pager_wait_on_lock(pPager, 4 as i32);
        }
    }
    return rc;
}

// /*
// ** Sync the database file to disk. This is a no-op for in-memory databases
// ** or pages with the Pager.noSync flag set.
// **
// ** If successful, or if called on a pager for which it is a no-op, this
// ** function returns SQLITE_OK. Otherwise, an IO error code is returned.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerSync(mut pPager: *mut Pager, mut zSuper: *const i8) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pArg: *mut () = zSuper as *mut ();
    rc = unsafe { sqlite3OsFileControl(unsafe { (*pPager).fd }, 21 as i32, pArg) };
    if rc == (12 as i32) {
        rc = 0 as i32;
    }
    if rc == (0 as i32) && !((unsafe { (*pPager).noSync }) != (0 as u8)) {
        0 as i32;
        rc = unsafe {
            sqlite3OsSync(
                unsafe { (*pPager).fd },
                ((unsafe { (*pPager).syncFlags }) as u32) as i32,
            )
        };
    }
    return rc;
}

// /* Pager object */
// /* If not NULL, the super-journal name */
// /* True to omit the xSync on the db file */
// /*
// ** When this function is called, the database file has been completely
// ** updated to reflect the changes made by the current transaction and
// ** synced to disk. The journal file still exists in the file-system
// ** though, and if a failure occurs at this point it will eventually
// ** be used as a hot-journal and the current transaction rolled back.
// **
// ** This function finalizes the journal file, either by deleting,
// ** truncating or partially zeroing it, so that it cannot be used
// ** for hot-journal rollback. Once this is done the transaction is
// ** irrevocably committed.
// **
// ** If an error occurs, an IO error code is returned and the pager
// ** moves into the error state. Otherwise, SQLITE_OK is returned.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerCommitPhaseTwo(mut pPager: *mut Pager) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* This routine should not be called if a prior error has occurred.
    //   ** But if (due to a coding error elsewhere in the system) it does get
    //   ** called, just return the same error code without doing anything. */
    if (unsafe { (*pPager).errCode }) != (0 as i32) {
        return unsafe { (*pPager).errCode };
    }
    let __v1624: *mut Pager = pPager;
    let __v1625: u32 = unsafe { (*__v1624).iDataVersion };
    let __v1626: u32 = __v1625.wrapping_add((1 as i32) as u32);
    unsafe {
        (*__v1624).iDataVersion = __v1626;
    }
    0 as i32;
    0 as i32;
    // /* An optimization. If the database was not actually modified during
    //   ** this transaction, the pager is running in exclusive-mode and is
    //   ** using persistent journals, then this function is a no-op.
    //   **
    //   ** The start of the journal file currently contains a single journal
    //   ** header with the nRec field set to 0. If such a journal is used as
    //   ** a hot-journal during hot-journal rollback, 0 changes will be made
    //   ** to the database file. So there is no need to zero the journal
    //   ** header. Since the pager is in exclusive mode, there is no need
    //   ** to drop any locks either.
    //   */
    if (((unsafe { (*pPager).eState }) as u32) as i32) == (2 as i32)
        && (unsafe { (*pPager).exclusiveMode }) != (0 as u8)
        && (((unsafe { (*pPager).journalMode }) as u32) as i32) == (1 as i32)
    {
        0 as i32;
        unsafe {
            (*pPager).eState = ((1 as i32) as i8) as u8;
        }
        return 0 as i32;
    }
    {}
    rc = pager_end_transaction(
        pPager,
        ((unsafe { (*pPager).setSuper }) as u32) as i32,
        1 as i32,
    );
    return pager_error(pPager, rc);
}

// /*
// ** If a write transaction is open, then all changes made within the
// ** transaction are reverted and the current write-transaction is closed.
// ** The pager falls back to PAGER_READER state if successful, or PAGER_ERROR
// ** state if an error occurs.
// **
// ** If the pager is already in PAGER_ERROR state when this function is called,
// ** it returns Pager.errCode immediately. No work is performed in this case.
// **
// ** Otherwise, in rollback mode, this function performs two functions:
// **
// **   1) It rolls back the journal file, restoring all database file and
// **      in-memory cache pages to the state they were in when the transaction
// **      was opened, and
// **
// **   2) It finalizes the journal file, so that it is not used for hot
// **      rollback at any point in the future.
// **
// ** Finalization of the journal file (task 2) is only performed if the
// ** rollback is successful.
// **
// ** In WAL mode, all cache-entries containing data modified within the
// ** current transaction are either expelled from the cache or reverted to
// ** their pre-transaction state by re-reading data from the database or
// ** WAL files. The WAL transaction is then closed.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerRollback(mut pPager: *mut Pager) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    {}
    // /* PagerRollback() is a no-op if called in READER or OPEN state. If
    //   ** the pager is already in the ERROR state, the rollback is not
    //   ** attempted here. Instead, the error code is returned to the caller.
    //   */
    0 as i32;
    if (((unsafe { (*pPager).eState }) as u32) as i32) == (6 as i32) {
        return unsafe { (*pPager).errCode };
    }
    if (((unsafe { (*pPager).eState }) as u32) as i32) <= (1 as i32) {
        return 0 as i32;
    }
    if (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
        let mut rc2: i32 = 0 as i32;
        rc = sqlite3PagerSavepoint(pPager, 2 as i32, -(1 as i32));
        rc2 = pager_end_transaction(
            pPager,
            ((unsafe { (*pPager).setSuper }) as u32) as i32,
            0 as i32,
        );
        if rc == (0 as i32) {
            rc = rc2;
        }
    } else {
        if !((unsafe { (*unsafe { (*pPager).jfd }).pMethods })
            != std::ptr::null::<sqlite3_io_methods>())
            || (((unsafe { (*pPager).eState }) as u32) as i32) == (2 as i32)
        {
            let mut eState: i32 = ((unsafe { (*pPager).eState }) as u32) as i32;
            rc = pager_end_transaction(pPager, 0 as i32, 0 as i32);
            if !((unsafe { (*pPager).memDb }) != (0 as u8)) && eState > (2 as i32) {
                // /* This can happen using journal_mode=off. Move the pager to the error
                //       ** state to indicate that the contents of the cache may not be trusted.
                //       ** Any active readers will get SQLITE_ABORT.
                //       */
                unsafe {
                    (*pPager).errCode = 4 as i32;
                }
                unsafe {
                    (*pPager).eState = ((6 as i32) as i8) as u8;
                }
                setGetterMethod(pPager);
                return rc;
            }
        } else {
            rc = pager_playback(pPager, 0 as i32);
        }
    }
    0 as i32;
    0 as i32;
    // /* If an error occurs during a ROLLBACK, we can no longer trust the pager
    //   ** cache. So call pager_error() on the way out to make any error persistent.
    //   */
    return pager_error(pPager, rc);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerOpenSavepoint(mut pPager: *mut Pager, mut nSavepoint: i32) -> i32 {
    0 as i32;
    0 as i32;
    if nSavepoint > unsafe { (*pPager).nSavepoint }
        && (unsafe { (*pPager).useJournal }) != (0 as u8)
    {
        return pagerOpenSavepoint(pPager, nSavepoint);
    } else {
        return 0 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** This function is called to rollback or release (commit) a savepoint.
// ** The savepoint to release or rollback need not be the most recently
// ** created savepoint.
// **
// ** Parameter op is always either SAVEPOINT_ROLLBACK or SAVEPOINT_RELEASE.
// ** If it is SAVEPOINT_RELEASE, then release and destroy the savepoint with
// ** index iSavepoint. If it is SAVEPOINT_ROLLBACK, then rollback all changes
// ** that have occurred since the specified savepoint was created.
// **
// ** The savepoint to rollback or release is identified by parameter
// ** iSavepoint. A value of 0 means to operate on the outermost savepoint
// ** (the first created). A value of (Pager.nSavepoint-1) means operate
// ** on the most recently created savepoint. If iSavepoint is greater than
// ** (Pager.nSavepoint-1), then this function is a no-op.
// **
// ** If a negative value is passed to this function, then the current
// ** transaction is rolled back. This is different to calling
// ** sqlite3PagerRollback() because this function does not terminate
// ** the transaction or unlock the database, it just restores the
// ** contents of the database to its original state.
// **
// ** In any case, all savepoints with an index greater than iSavepoint
// ** are destroyed. If this is a release operation (op==SAVEPOINT_RELEASE),
// ** then savepoint iSavepoint is also destroyed.
// **
// ** This function may return SQLITE_NOMEM if a memory allocation fails,
// ** or an IO error code if an IO error occurs while rolling back a
// ** savepoint. If no errors occur, SQLITE_OK is returned.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerSavepoint(
    mut pPager: *mut Pager,
    mut op: i32,
    mut iSavepoint: i32,
) -> i32 {
    let mut rc: i32 = unsafe { (*pPager).errCode };
    0 as i32;
    0 as i32;
    if rc == (0 as i32) && iSavepoint < unsafe { (*pPager).nSavepoint } {
        // /* Iterator variable */
        let mut ii: i32 = 0 as i32;
        // /* Number of remaining savepoints after this op. */
        let mut nNew: i32 = 0 as i32;
        // /* Figure out how many savepoints will still be active after this
        //     ** operation. Store this value in nNew. Then free resources associated
        //     ** with any savepoints that are destroyed by this operation.
        //     */
        nNew = iSavepoint + if op == (1 as i32) { 0 as i32 } else { 1 as i32 };
        ii = nNew;
        '__slate_break_1547: loop {
            if !(ii < unsafe { (*pPager).nSavepoint }) {
                break;
            }
            unsafe {
                sqlite3BitvecDestroy(unsafe {
                    (*unsafe { unsafe { (*pPager).aSavepoint }.offset(ii as isize) }).pInSavepoint
                })
            };
            let __v1627: i32 = ii;
            let __v1628: i32 = __v1627 + (1 as i32);
            ii = __v1628;
        }
        unsafe {
            (*pPager).nSavepoint = nNew;
        }
        // /* Truncate the sub-journal so that it only includes the parts
        //     ** that are still in use. */
        if op == (1 as i32) {
            let mut pRel: *mut PagerSavepoint =
                unsafe { unsafe { (*pPager).aSavepoint }.offset(nNew as isize) };
            if (unsafe { (*pRel).bTruncateOnRelease }) != (0 as i32)
                && (unsafe { (*unsafe { (*pPager).sjfd }).pMethods })
                    != std::ptr::null::<sqlite3_io_methods>()
            {
                // /* Only truncate if it is an in-memory sub-journal. */
                if (unsafe { sqlite3JournalIsInMemory(unsafe { (*pPager).sjfd }) }) != (0 as i32) {
                    let mut sz: i64 = ((unsafe { (*pPager).pageSize }) + ((4 as i32) as i64))
                        * (((unsafe { (*pRel).iSubRec }) as u64) as i64);
                    rc = unsafe { sqlite3OsTruncate(unsafe { (*pPager).sjfd }, sz) };
                    0 as i32;
                }
                unsafe {
                    (*pPager).nSubRec = unsafe { (*pRel).iSubRec };
                }
            }
        } else {
            if (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>()
                || (unsafe { (*unsafe { (*pPager).jfd }).pMethods })
                    != std::ptr::null::<sqlite3_io_methods>()
            {
                let mut pSavepoint: *mut PagerSavepoint = if nNew == (0 as i32) {
                    std::ptr::null_mut::<PagerSavepoint>()
                } else {
                    unsafe { unsafe { (*pPager).aSavepoint }.offset((nNew - (1 as i32)) as isize) }
                };
                rc = pagerPlaybackSavepoint(pPager, pSavepoint);
                0 as i32;
            }
        }
        // /* Else this is a rollback operation, playback the specified savepoint.
        //     ** If this is a temp-file, it is possible that the journal file has
        //     ** not yet been opened. In this case there have been no changes to
        //     ** the database file, so the playback operation can be skipped.
        //     */
    }
    return rc;
}

// /*
// ** This function is called to obtain a shared lock on the database file.
// ** It is illegal to call sqlite3PagerGet() until after this function
// ** has been successfully called. If a shared-lock is already held when
// ** this function is called, it is a no-op.
// **
// ** The following operations are also performed by this function.
// **
// **   1) If the pager is currently in PAGER_OPEN state (no lock held
// **      on the database file), then an attempt is made to obtain a
// **      SHARED lock on the database file. Immediately after obtaining
// **      the SHARED lock, the file-system is checked for a hot-journal,
// **      which is played back if present. Following any hot-journal
// **      rollback, the contents of the cache are validated by checking
// **      the 'change-counter' field of the database file header and
// **      discarded if they are found to be invalid.
// **
// **   2) If the pager is running in exclusive-mode, and there are currently
// **      no outstanding references to any pages, and is in the error state,
// **      then an attempt is made to clear the error state by discarding
// **      the contents of the page cache and rolling back any open journal
// **      file.
// **
// ** If everything is successful, SQLITE_OK is returned. If an IO error
// ** occurs while locking the database, checking for a hot-journal file or
// ** rolling back a journal file, the IO error code is returned.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerSharedLock(mut pPager: *mut Pager) -> i32 {
    let mut __slate_storage_981: std::mem::MaybeUninit<__SlateAlign16<[i8; 16]>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_981: *mut [i8; 16] =
        std::ptr::addr_of_mut!(__slate_storage_981) as *mut [i8; 16];
    let mut __slate_storage_980: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_980: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_980) as *mut i32;
    let mut __slate_storage_979: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_979: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_979) as *mut i32;
    let mut __slate_storage_978: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_978: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_978) as *mut i32;
    let mut __slate_storage_977: std::mem::MaybeUninit<*mut sqlite3_vfs> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_977: *mut *mut sqlite3_vfs =
        std::ptr::addr_of_mut!(__slate_storage_977) as *mut *mut sqlite3_vfs;
    let mut __slate_storage_976: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_976: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_976) as *mut i32;
    let mut __slate_storage_975: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_975: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_975) as *mut i32;
    unsafe {
        '__join_3: {
            // /* Return code */
            std::ptr::write(__slate_slot_975, 0 as i32);
            // /* This routine is only called from b-tree and only when there are no
            //   ** outstanding pages. This implies that the pager state should either
            //   ** be OPEN or READER. READER is only possible if the pager is or was in
            //   ** exclusive access mode.  */
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            if !((unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>())
                && (((unsafe { (*pPager).eState }) as u32) as i32) == (0 as i32)
            {
                // /* True if there exists a hot journal-file */
                std::ptr::write(__slate_slot_976, 1 as i32);
                0 as i32;
                0 as i32;
                *__slate_slot_975 = pager_wait_on_lock(pPager, 1 as i32);
                if *__slate_slot_975 != (0 as i32) {
                    0 as i32;
                    break '__join_3;
                } else {
                    // /* If a journal file exists, and there is no RESERVED lock on the
                    //     ** database file, then it either needs to be played back or deleted.
                    //     */
                    if (((unsafe { (*pPager).eLock }) as u32) as i32) <= (1 as i32) {
                        *__slate_slot_975 =
                            hasHotJournal(pPager, std::ptr::addr_of_mut!(*__slate_slot_976));
                    }
                    if *__slate_slot_975 != (0 as i32) {
                        break '__join_3;
                    } else {
                        if *__slate_slot_976 != (0 as i32) {
                            if (unsafe { (*pPager).readOnly }) != (0 as u8) {
                                *__slate_slot_975 = (8 as i32) | (3 as i32) << (8 as i32);
                                break '__join_3;
                            } else {
                                // /* Get an EXCLUSIVE lock on the database file. At this point it is
                                //       ** important that a RESERVED lock is not obtained on the way to the
                                //       ** EXCLUSIVE lock. If it were, another process might open the
                                //       ** database file, detect the RESERVED lock, and conclude that the
                                //       ** database is safe to read while this process is still rolling the
                                //       ** hot-journal back.
                                //       **
                                //       ** Because the intermediate RESERVED lock is not requested, any
                                //       ** other process attempting to access the database file will get to
                                //       ** this point in the code and fail to obtain its own EXCLUSIVE lock
                                //       ** on the database file.
                                //       **
                                //       ** Unless the pager is in locking_mode=exclusive mode, the lock is
                                //       ** downgraded to SHARED_LOCK before this function returns.
                                //       */
                                *__slate_slot_975 = pagerLockDb(pPager, 4 as i32);
                                if *__slate_slot_975 != (0 as i32) {
                                    break '__join_3;
                                } else {
                                    // /* If it is not already open and the file exists on disk, open the
                                    //       ** journal for read/write access. Write access is required because
                                    //       ** in exclusive-access mode the file descriptor will be kept open
                                    //       ** and possibly used for a transaction later on. Also, write-access
                                    //       ** is usually required to finalize the journal in journal_mode=persist
                                    //       ** mode (and also for journal_mode=truncate on some systems).
                                    //       **
                                    //       ** If the journal does not exist, it usually means that some
                                    //       ** other connection managed to get in and roll it back before
                                    //       ** this connection obtained the exclusive lock above. Or, it
                                    //       ** may mean that the pager was in the error-state when this
                                    //       ** function was called and the journal file does not exist.
                                    //       */
                                    if !((unsafe { (*unsafe { (*pPager).jfd }).pMethods })
                                        != std::ptr::null::<sqlite3_io_methods>())
                                        && (((unsafe { (*pPager).journalMode }) as u32) as i32)
                                            != (2 as i32)
                                    {
                                        std::ptr::write(__slate_slot_977, unsafe {
                                            (*pPager).pVfs
                                        });
                                        // /* True if journal file exists */
                                        *__slate_slot_975 = unsafe {
                                            sqlite3OsAccess(
                                                *__slate_slot_977,
                                                (unsafe { (*pPager).zJournal }) as *const i8,
                                                0 as i32,
                                                std::ptr::addr_of_mut!(*__slate_slot_978),
                                            )
                                        };
                                        if *__slate_slot_975 == (0 as i32)
                                            && *__slate_slot_978 != (0 as i32)
                                        {
                                            std::ptr::write(__slate_slot_979, 0 as i32);
                                            std::ptr::write(
                                                __slate_slot_980,
                                                (2 as i32) | (2048 as i32),
                                            );
                                            0 as i32;
                                            *__slate_slot_975 = unsafe {
                                                sqlite3OsOpen(
                                                    *__slate_slot_977,
                                                    (unsafe { (*pPager).zJournal }) as *const i8,
                                                    unsafe { (*pPager).jfd },
                                                    *__slate_slot_980,
                                                    std::ptr::addr_of_mut!(*__slate_slot_979),
                                                )
                                            };
                                            0 as i32;
                                            if *__slate_slot_975 == (0 as i32)
                                                && *__slate_slot_979 & (1 as i32) != (0 as i32)
                                            {
                                                *__slate_slot_975 =
                                                    unsafe { sqlite3CantopenError(5400 as i32) };
                                                unsafe { sqlite3OsClose(unsafe { (*pPager).jfd }) };
                                            }
                                        }
                                    }
                                    // /* Playback and delete the journal.  Drop the database write
                                    //       ** lock and reacquire the read lock. Purge the cache before
                                    //       ** playing back the hot-journal so that we don't end up with
                                    //       ** an inconsistent cache.  Sync the hot journal before playing
                                    //       ** it back since the process that crashed and left the hot journal
                                    //       ** probably did not sync it and we are required to always sync
                                    //       ** the journal before playing it back.
                                    //       */
                                    if (unsafe { (*unsafe { (*pPager).jfd }).pMethods })
                                        != std::ptr::null::<sqlite3_io_methods>()
                                    {
                                        0 as i32;
                                        *__slate_slot_975 = pagerSyncHotJournal(pPager);
                                        if *__slate_slot_975 == (0 as i32) {
                                            *__slate_slot_975 = pager_playback(
                                                pPager,
                                                !((unsafe { (*pPager).tempFile }) != (0 as u8))
                                                    as i32,
                                            );
                                            unsafe {
                                                (*pPager).eState = ((0 as i32) as i8) as u8;
                                            }
                                        }
                                    } else {
                                        if !((unsafe { (*pPager).exclusiveMode }) != (0 as u8)) {
                                            pagerUnlockDb(pPager, 1 as i32);
                                        }
                                    }
                                    if *__slate_slot_975 != (0 as i32) {
                                        // /* This branch is taken if an error occurs while trying to open
                                        //         ** or roll back a hot-journal while holding an EXCLUSIVE lock. The
                                        //         ** pager_unlock() routine will be called before returning to unlock
                                        //         ** the file. If the unlock attempt fails, then Pager.eLock must be
                                        //         ** set to UNKNOWN_LOCK (see the comment above the #define for
                                        //         ** UNKNOWN_LOCK above for an explanation).
                                        //         **
                                        //         ** In order to get pager_unlock() to do this, set Pager.eState to
                                        //         ** PAGER_ERROR now. This is not actually counted as a transition
                                        //         ** to ERROR state in the state diagram at the top of this file,
                                        //         ** since we know that the same call to pager_unlock() will very
                                        //         ** shortly transition the pager object to the OPEN state. Calling
                                        //         ** assert_pager_state() would fail now, as it should not be possible
                                        //         ** to be in ERROR state when there are zero outstanding page
                                        //         ** references.
                                        //         */
                                        pager_error(pPager, *__slate_slot_975);
                                        break '__join_3;
                                    } else {
                                        0 as i32;
                                        0 as i32;
                                    }
                                }
                            }
                        }
                        if !((unsafe { (*pPager).tempFile }) != (0 as u8))
                            && (unsafe { (*pPager).hasHeldSharedLock }) != (0 as u8)
                        {
                            // /* The shared-lock has just been acquired then check to
                            //       ** see if the database has been modified.  If the database has changed,
                            //       ** flush the cache.  The hasHeldSharedLock flag prevents this from
                            //       ** occurring on the very first access to a file, in order to save a
                            //       ** single unnecessary sqlite3OsRead() call at the start-up.
                            //       **
                            //       ** Database changes are detected by looking at 15 bytes beginning
                            //       ** at offset 24 into the file.  The first 4 of these 16 bytes are
                            //       ** a 32-bit counter that is incremented with each change.  The
                            //       ** other bytes change randomly with each file change when
                            //       ** a codec is in use.
                            //       **
                            //       ** There is a vanishingly small chance that a change will not be
                            //       ** detected.  The chance of an undetected change is so small that
                            //       ** it can be neglected.
                            //       */
                            {}
                            *__slate_slot_975 = unsafe {
                                sqlite3OsRead(
                                    unsafe { (*pPager).fd },
                                    std::ptr::addr_of_mut!(*__slate_slot_981) as *mut (),
                                    ((16 as u64) as u32) as i32,
                                    (24 as i32) as i64,
                                )
                            };
                            if *__slate_slot_975 != (0 as i32) {
                                if *__slate_slot_975 != (10 as i32) | (2 as i32) << (8 as i32) {
                                    break '__join_3;
                                } else {
                                    unsafe {
                                        memset(
                                            ((*__slate_slot_981).as_mut_ptr() as *mut i8)
                                                as *mut (),
                                            0 as i32,
                                            16 as u64,
                                        )
                                    };
                                }
                            }
                            if (unsafe {
                                memcmp(
                                    (unsafe { (*pPager).dbFileVers.as_mut_ptr() as *mut i8 })
                                        as *const (),
                                    ((*__slate_slot_981).as_mut_ptr() as *mut i8) as *const (),
                                    16 as u64,
                                )
                            }) != (0 as i32)
                            {
                                pager_reset(pPager);
                                // /* Unmap the database file. It is possible that external processes
                                //         ** may have truncated the database file and then extended it back
                                //         ** to its original size while this process was not holding a lock.
                                //         ** In this case there may exist a Pager.pMap mapping that appears
                                //         ** to be the right size but is not actually valid. Avoid this
                                //         ** possibility by unmapping the db here. */
                                if (unsafe { (*pPager).bUseFetch }) != (0 as u8) {
                                    unsafe {
                                        sqlite3OsUnfetch(
                                            unsafe { (*pPager).fd },
                                            (0 as i32) as i64,
                                            std::ptr::null_mut::<()>(),
                                        )
                                    };
                                }
                            }
                        }
                        // /* If there is a WAL file in the file-system, open this database in WAL
                        //     ** mode. Otherwise, the following function call is a no-op.
                        //     */
                        *__slate_slot_975 = pagerOpenWalIfPresent(pPager);
                        0 as i32;
                    }
                }
            }
            if (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
                0 as i32;
                *__slate_slot_975 = pagerBeginReadTransaction(pPager);
            }
            if (((unsafe { (*pPager).tempFile }) as u32) as i32) == (0 as i32)
                && (((unsafe { (*pPager).eState }) as u32) as i32) == (0 as i32)
                && *__slate_slot_975 == (0 as i32)
            {
                *__slate_slot_975 =
                    pagerPagecount(pPager, unsafe { std::ptr::addr_of_mut!((*pPager).dbSize) });
            }
        }
        if *__slate_slot_975 != (0 as i32) {
            0 as i32;
            pager_unlock(pPager);
            0 as i32;
        } else {
            unsafe {
                (*pPager).eState = ((1 as i32) as i8) as u8;
            }
            unsafe {
                (*pPager).hasHeldSharedLock = ((1 as i32) as i8) as u8;
            }
        }
        return *__slate_slot_975;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** This function is called when the user invokes "PRAGMA wal_checkpoint",
// ** "PRAGMA wal_blocking_checkpoint" or calls the sqlite3_wal_checkpoint()
// ** or wal_blocking_checkpoint() API functions.
// **
// ** Parameter eMode is one of SQLITE_CHECKPOINT_PASSIVE, FULL or RESTART.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerCheckpoint(
    mut pPager: *mut Pager,
    mut db: *mut sqlite3,
    mut eMode: i32,
    mut pnLog: *mut i32,
    mut pnCkpt: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pPager).pWal }) == std::ptr::null_mut::<Wal>()
        && (((unsafe { (*pPager).journalMode }) as u32) as i32) == (5 as i32)
    {
        // /* This only happens when a database file is zero bytes in size opened and
        //     ** then "PRAGMA journal_mode=WAL" is run and then sqlite3_wal_checkpoint()
        //     ** is invoked without any intervening transactions.  We need to start
        //     ** a transaction to initialize pWal.  The PRAGMA table_list statement is
        //     ** used for this since it starts transactions on every database file,
        //     ** including all ATTACHed databases.  This seems expensive for a single
        //     ** sqlite3_wal_checkpoint() call, but it happens very rarely.
        //     ** https://sqlite.org/forum/forumpost/fd0f19d229156939
        //     */
        unsafe {
            sqlite3_exec(
                db,
                (b"PRAGMA table_list\0".as_ptr() as *mut i8) as *const i8,
                None,
                std::ptr::null_mut::<()>(),
                std::ptr::null_mut::<*mut i8>(),
            )
        };
    }
    if (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
        rc = unsafe {
            sqlite3WalCheckpoint(
                unsafe { (*pPager).pWal },
                db,
                eMode,
                {
                    let __t0: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32> =
                        if eMode <= (0 as i32) {
                            None
                        } else {
                            unsafe { (*pPager).xBusyHandler }
                        };
                    __t0
                },
                unsafe { (*pPager).pBusyHandlerArg },
                ((unsafe { (*pPager).walSyncFlags }) as u32) as i32,
                (unsafe { (*pPager).pageSize }) as i32,
                (unsafe { (*pPager).pTmpSpace }) as *mut u8,
                pnLog,
                pnCkpt,
            )
        };
    }
    return rc;
}

// /*
// ** Return true if the underlying VFS for the given pager supports the
// ** primitives necessary for write-ahead logging.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerWalSupported(mut pPager: *mut Pager) -> i32 {
    let mut pMethods: *const sqlite3_io_methods = unsafe { (*unsafe { (*pPager).fd }).pMethods };
    if (unsafe { (*pPager).noLock }) != (0 as u8) {
        return 0 as i32;
    }
    return ((unsafe { (*pPager).exclusiveMode }) != (0 as u8)
        || (unsafe { (*pMethods).iVersion }) >= (2 as i32)
            && (unsafe { (*pMethods).xShmMap }) != None) as i32;
}

// /* Checkpoint on this pager */
// /* Db handle used to check for interrupts */
// /* Type of checkpoint */
// /* OUT: Final number of frames in log */
// /* OUT: Final number of checkpointed frames */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerWalCallback(mut pPager: *mut Pager) -> i32 {
    return unsafe { sqlite3WalCallback(unsafe { (*pPager).pWal }) };
}

// /*
// ** The caller must be holding a SHARED lock on the database file to call
// ** this function.
// **
// ** If the pager passed as the first argument is open on a real database
// ** file (not a temp file or an in-memory database), and the WAL file
// ** is not already open, make an attempt to open it now. If successful,
// ** return SQLITE_OK. If an error occurs or the VFS used by the pager does
// ** not support the xShmXXX() methods, return an error code. *pbOpen is
// ** not modified in either case.
// **
// ** If the pager is open on a temp-file (or in-memory database), or if
// ** the WAL file is already open, set *pbOpen to 1 and return SQLITE_OK
// ** without doing anything.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerOpenWal(mut pPager: *mut Pager, mut pbOpen: *mut i32) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if !((unsafe { (*pPager).tempFile }) != (0 as u8))
        && !((unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>())
    {
        if !(sqlite3PagerWalSupported(pPager) != (0 as i32)) {
            return 14 as i32;
        }
        // /* Close any rollback journal previously open */
        unsafe { sqlite3OsClose(unsafe { (*pPager).jfd }) };
        rc = pagerOpenWal(pPager);
        if rc == (0 as i32) {
            unsafe {
                (*pPager).journalMode = ((5 as i32) as i8) as u8;
            }
            unsafe {
                (*pPager).eState = ((0 as i32) as i8) as u8;
            }
        }
    } else {
        unsafe {
            *pbOpen = 1 as i32;
        }
    }
    return rc;
}

// /* Pager object */
// /* OUT: Set to true if call is a no-op */
// /*
// ** This function is called to close the connection to the log file prior
// ** to switching from WAL to rollback mode.
// **
// ** Before closing the log file, this function attempts to take an
// ** EXCLUSIVE lock on the database file. If this cannot be obtained, an
// ** error (SQLITE_BUSY) is returned and the log connection is not closed.
// ** If successful, the EXCLUSIVE lock is not released before returning.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerCloseWal(mut pPager: *mut Pager, mut db: *mut sqlite3) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    // /* If the log file is not already open, but does exist in the file-system,
    //   ** it may need to be checkpointed before the connection can switch to
    //   ** rollback mode. Open it now so this can happen.
    //   */
    if !((unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>()) {
        let mut logexists: i32 = 0 as i32;
        rc = pagerLockDb(pPager, 1 as i32);
        if rc == (0 as i32) {
            rc = unsafe {
                sqlite3OsAccess(
                    unsafe { (*pPager).pVfs },
                    (unsafe { (*pPager).zWal }) as *const i8,
                    0 as i32,
                    std::ptr::addr_of_mut!(logexists),
                )
            };
        }
        if rc == (0 as i32) && logexists != (0 as i32) {
            rc = pagerOpenWal(pPager);
        }
    }
    // /* Checkpoint and close the log. Because an EXCLUSIVE lock is held on
    //   ** the database file, the log and log-summary files will be deleted.
    //   */
    if rc == (0 as i32) && (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
        rc = pagerExclusiveLock(pPager);
        if rc == (0 as i32) {
            rc = unsafe {
                sqlite3WalClose(
                    unsafe { (*pPager).pWal },
                    db,
                    ((unsafe { (*pPager).walSyncFlags }) as u32) as i32,
                    (unsafe { (*pPager).pageSize }) as i32,
                    (unsafe { (*pPager).pTmpSpace }) as *mut u8,
                )
            };
            unsafe {
                (*pPager).pWal = std::ptr::null_mut::<Wal>();
            }
            pagerFixMaplimit(pPager);
            if rc != (0 as i32) && !((unsafe { (*pPager).exclusiveMode }) != (0 as u8)) {
                pagerUnlockDb(pPager, 1 as i32);
            }
        }
    }
    return rc;
}

// /*
// ** The size of the of each page record in the journal is given by
// ** the following macro.
// */
// /*
// ** The journal header size for this pager. This is usually the same
// ** size as a single disk sector. See also setSectorSize().
// */
// /*
// ** The macro MEMDB is true if we are dealing with an in-memory database.
// ** We do this as a macro so that if the SQLITE_OMIT_MEMORYDB macro is set,
// ** the value of MEMDB will be a constant and the compiler will optimize
// ** out code that would never execute.
// */
// /*
// ** The macro USEFETCH is true if we are allowed to use the xFetch and xUnfetch
// ** interfaces to access the database using memory-mapped I/O.
// */
// /*
// ** Return true if page pgno can be read directly from the database file
// ** by the b-tree layer. This is the case if:
// **
// **   (1)  the database file is open
// **   (2)  the VFS for the database is able to do unaligned sub-page reads
// **   (3)  there are no dirty pages in the cache, and
// **   (4)  the desired page is not currently in the wal file.
// */
// /* !SQLITE_OMIT_WAL */
// /* SQLITE_OMIT_DISKIO */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerDirectReadOk(mut pPager: *mut Pager, mut pgno: u32) -> i32 {
    0 as i32;
    0 as i32;
    // /* Case (1) */
    if (unsafe { (*unsafe { (*pPager).fd }).pMethods }) == std::ptr::null::<sqlite3_io_methods>() {
        return 0 as i32;
    }
    // /* Failed (3) */
    if (unsafe { sqlite3PCacheIsDirty(unsafe { (*pPager).pPCache }) }) != (0 as i32) {
        return 0 as i32;
    }
    if (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
        let mut iRead: u32 = (0 as i32) as u32;
        unsafe {
            sqlite3WalFindFrame(
                unsafe { (*pPager).pWal },
                pgno,
                std::ptr::addr_of_mut!(iRead),
            )
        };
        // /* Case (4) */
        if iRead != (0 as u32) {
            return 0 as i32;
        }
    }
    0 as i32;
    if (unsafe {
        unsafe { (*unsafe { (*unsafe { (*pPager).fd }).pMethods }).xDeviceCharacteristics }.unwrap()(
            unsafe { (*pPager).fd },
        )
    }) & (32768 as i32)
        == (0 as i32)
    {
        // /* Case (2) */
        return 0 as i32;
    }
    return 1 as i32;
}

// /*
// ** Return TRUE if the database file is opened read-only.  Return FALSE
// ** if the database is (in theory) writable.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerIsreadonly(mut pPager: *mut Pager) -> u8 {
    return unsafe { (*pPager).readOnly };
}

// /*
// ** Return the pPager->iDataVersion value
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerDataVersion(mut pPager: *mut Pager) -> u32 {
    return unsafe { (*pPager).iDataVersion };
}

// /*
// ** Return the approximate number of bytes of memory currently
// ** used by the pager and its associated cache.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerMemUsed(mut pPager: *mut Pager) -> i32 {
    let mut perPageSize: i32 = ((unsafe { (*pPager).pageSize })
        + ((((unsafe { (*pPager).nExtra }) as u32) as i32) as i64)
        + ((((80 as u64).wrapping_add((((5 as i32) as i64) as u64).wrapping_mul(8 as u64)) as u32)
            as i32) as i64)) as i32;
    return (((perPageSize * unsafe { sqlite3PcachePagecount(unsafe { (*pPager).pPCache }) }
        + unsafe { sqlite3MallocSize(pPager as *const ()) }) as i64)
        + unsafe { (*pPager).pageSize }) as i32;
}

// /*
// ** Return the full pathname of the database file.
// **
// ** Except, if the pager is in-memory only, then return an empty string if
// ** nullIfMemDb is true.  This routine is called with nullIfMemDb==1 when
// ** used to report the filename to the user, for compatibility with legacy
// ** behavior.  But when the Btree needs to know the filename for matching to
// ** shared cache, it uses nullIfMemDb==0 so that in-memory databases can
// ** participate in shared-cache.
// **
// ** The return value to this routine is always safe to use with
// ** sqlite3_uri_parameter() and sqlite3_filename_database() and friends.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerFilename(
    mut pPager: *const Pager,
    mut nullIfMemDb: i32,
) -> *const i8 {
    let __v1629: bool;
    if nullIfMemDb != (0 as i32) {
        let __v1630: bool;
        if (unsafe { (*pPager).memDb }) != (0 as u8) {
            __v1630 = true as bool;
        } else {
            __v1630 =
                (unsafe { sqlite3IsMemdb((unsafe { (*pPager).pVfs }) as *const sqlite3_vfs) })
                    != (0 as i32);
        }
        __v1629 = __v1630;
    } else {
        __v1629 = false as bool;
    }
    if __v1629 {
        return unsafe {
            unsafe { std::ptr::addr_of!(zFake) as *const i8 }.offset((4 as i32) as isize)
        };
    } else {
        return (unsafe { (*pPager).zFilename }) as *const i8;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Return the VFS structure for the pager.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerVfs(mut pPager: *mut Pager) -> *mut sqlite3_vfs {
    return unsafe { (*pPager).pVfs };
}

// /*
// ** Return the file handle for the database file associated
// ** with the pager.  This might return NULL if the file has
// ** not yet been opened.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerFile(mut pPager: *mut Pager) -> *mut sqlite3_file {
    return unsafe { (*pPager).fd };
}

// /*
// ** Return the file handle for the journal file (if it exists).
// ** This will be either the rollback journal or the WAL file.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerJrnlFile(mut pPager: *mut Pager) -> *mut sqlite3_file {
    let __v1631: *mut sqlite3_file;
    if (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
        __v1631 = unsafe { sqlite3WalFile(unsafe { (*pPager).pWal }) };
    } else {
        __v1631 = unsafe { (*pPager).jfd };
    }
    return __v1631;
}

// /*
// ** Return the full pathname of the journal file.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerJournalname(mut pPager: *mut Pager) -> *const i8 {
    return (unsafe { (*pPager).zJournal }) as *const i8;
}

// /*
// ** Return a pointer to the "temporary page" buffer held internally
// ** by the pager.  This is a buffer that is big enough to hold the
// ** entire content of a database page.  This buffer is used internally
// ** during rollback and will be overwritten whenever a rollback
// ** occurs.  But other modules are free to use it too, as long as
// ** no rollbacks are happening.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerTempSpace(mut pPager: *mut Pager) -> *mut () {
    return (unsafe { (*pPager).pTmpSpace }) as *mut ();
}

// /*
// ** Return true if this is an in-memory or temp-file backed pager.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerIsMemdb(mut pPager: *mut Pager) -> i32 {
    return ((unsafe { (*pPager).tempFile }) != (0 as u8)
        || (unsafe { (*pPager).memVfs }) != (0 as u8)) as i32;
}

// /*
// ** Parameter eStat must be one of SQLITE_DBSTATUS_CACHE_HIT, _MISS, _WRITE,
// ** or _WRITE+1.  The SQLITE_DBSTATUS_CACHE_WRITE+1 case is a translation
// ** of SQLITE_DBSTATUS_CACHE_SPILL.  The _SPILL case is not contiguous because
// ** it was added later.
// **
// ** Before returning, *pnVal is incremented by the
// ** current cache hit or miss count, according to the value of eStat. If the
// ** reset parameter is non-zero, the cache hit or miss count is zeroed before
// ** returning.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerCacheStat(
    mut pPager: *mut Pager,
    mut eStat: i32,
    mut reset: i32,
    mut pnVal: *mut u64,
) {
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    let __v1632: i32 = eStat;
    let __v1633: i32 = __v1632 - (7 as i32);
    eStat = __v1633;
    let __v1634: *mut u64 = pnVal;
    let __v1635: u64 = unsafe { *__v1634 };
    let __v1636: u64 = __v1635.wrapping_add(
        (unsafe {
            *unsafe { unsafe { (*pPager).aStat.as_mut_ptr() as *mut u32 }.offset(eStat as isize) }
        }) as u64,
    );
    unsafe {
        *__v1634 = __v1636;
    }
    if reset != (0 as i32) {
        unsafe {
            *unsafe {
                unsafe { (*pPager).aStat.as_mut_ptr() as *mut u32 }.offset(eStat as isize)
            } = (0 as i32) as u32;
        }
    }
}

// /*
// ** Unless this is an in-memory or temporary database, clear the pager cache.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerClearCache(mut pPager: *mut Pager) {
    0 as i32;
    if (((unsafe { (*pPager).tempFile }) as u32) as i32) == (0 as i32) {
        pager_reset(pPager);
    }
}

// /*
// ** Return a sanitized version of the sector-size of OS file pFile. The
// ** return value is guaranteed to lie between 32 and MAX_SECTOR_SIZE.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SectorSize(mut pFile: *mut sqlite3_file) -> i32 {
    let mut iRet: i32 = unsafe { sqlite3OsSectorSize(pFile) };
    if iRet < (32 as i32) {
        iRet = 512 as i32;
    } else {
        if iRet > (65536 as i32) {
            0 as i32;
            iRet = 65536 as i32;
        }
    }
    return iRet;
}

// /*
// ** Function assertTruncateConstraint(pPager) checks that one of the
// ** following is true for all dirty pages currently in the page-cache:
// **
// **   a) The page number is less than or equal to the size of the
// **      current database image, in pages, OR
// **
// **   b) if the page content were written at this time, it would not
// **      be necessary to write the current content out to the sub-journal.
// **
// ** If the condition asserted by this function were not true, and the
// ** dirty page were to be discarded from the cache via the pagerStress()
// ** routine, pagerStress() would not write the current page content to
// ** the database file. If a savepoint transaction were rolled back after
// ** this happened, the correct behavior would be to restore the current
// ** content of the page. However, since this content is not present in either
// ** the database file or the portion of the rollback journal and
// ** sub-journal rolled back the content could not be restored and the
// ** database image would become corrupt. It is therefore fortunate that
// ** this circumstance cannot arise.
// */
// /*
// ** Truncate the in-memory database file image to nPage pages. This
// ** function does not actually modify the database file on disk. It
// ** just sets the internal state of the pager object so that the
// ** truncation will be done when the current transaction is committed.
// **
// ** This function is only called right before committing a transaction.
// ** Once this function has been called, the transaction must either be
// ** rolled back or committed. It is not safe to call this function and
// ** then continue writing to the database.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerTruncateImage(mut pPager: *mut Pager, mut nPage: u32) {
    0 as i32;
    0 as i32;
    unsafe {
        (*pPager).dbSize = nPage;
    }
    // /* At one point the code here called assertTruncateConstraint() to
    //   ** ensure that all pages being truncated away by this operation are,
    //   ** if one or more savepoints are open, present in the savepoint
    //   ** journal so that they can be restored if the savepoint is rolled
    //   ** back. This is no longer necessary as this function is now only
    //   ** called right before committing a transaction. So although the
    //   ** Pager object may still have open savepoints (Pager.nSavepoint!=0),
    //   ** they cannot be rolled back. So the assertTruncateConstraint() call
    //   ** is no longer correct. */
}

// /*
// ** The page handle passed as the first argument refers to a dirty page
// ** with a page number other than iNew. This function changes the page's
// ** page number to iNew and sets the value of the PgHdr.flags field to
// ** the value passed as the third parameter.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PagerRekey(mut pPg: *mut PgHdr, mut iNew: u32, mut flags: u16) {
    0 as i32;
    unsafe {
        (*pPg).flags = flags;
    }
    unsafe { sqlite3PcacheMove(pPg, iNew) };
}

// /*
// ** The page getter methods each try to acquire a reference to a
// ** page with page number pgno. If the requested reference is
// ** successfully obtained, it is copied to *ppPage and SQLITE_OK returned.
// **
// ** There are different implementations of the getter method depending
// ** on the current state of the pager.
// **
// **     getPageNormal()         --  The normal getter
// **     getPageError()          --  Used if the pager is in an error state
// **     getPageMmap()           --  Used if memory-mapped I/O is enabled
// **
// ** If the requested page is already in the cache, it is returned.
// ** Otherwise, a new page object is allocated and populated with data
// ** read from the database file. In some cases, the pcache module may
// ** choose not to allocate a new page object and may reuse an existing
// ** object with no outstanding references.
// **
// ** The extra data appended to a page is always initialized to zeros the
// ** first time a page is loaded into memory. If the page requested is
// ** already in the cache when this function is called, then the extra
// ** data is left as it was when the page object was last used.
// **
// ** If the database image is smaller than the requested page or if
// ** the flags parameter contains the PAGER_GET_NOCONTENT bit and the
// ** requested page is not already stored in the cache, then no
// ** actual disk read occurs. In this case the memory image of the
// ** page is initialized to all zeros.
// **
// ** If PAGER_GET_NOCONTENT is true, it means that we do not care about
// ** the contents of the page. This occurs in two scenarios:
// **
// **   a) When reading a free-list leaf page from the database, and
// **
// **   b) When a savepoint is being rolled back and we need to load
// **      a new page into the cache to be filled with the data read
// **      from the savepoint journal.
// **
// ** If PAGER_GET_NOCONTENT is true, then the data returned is zeroed instead
// ** of being read from the database. Additionally, the bits corresponding
// ** to pgno in Pager.pInJournal (bitvec of pages already written to the
// ** journal file) and the PagerSavepoint.pInSavepoint bitvecs of any open
// ** savepoints are set. This means if the page is made writable at any
// ** point in the future, using a call to sqlite3PagerWrite(), its contents
// ** will not be journaled. This saves IO.
// **
// ** The acquisition might fail for several reasons.  In all cases,
// ** an appropriate error code is returned and *ppPage is set to NULL.
// **
// ** See also sqlite3PagerLookup().  Both this routine and Lookup() attempt
// ** to find a page in the in-memory cache first.  If the page is not already
// ** in memory, this routine goes to disk to read it in whereas Lookup()
// ** just returns 0.  This routine acquires a read-lock the first time it
// ** has to go to disk, and could also playback an old journal if necessary.
// ** Since Lookup() never goes to disk, it never has to deal with locks
// ** or journal files.
// */
#[unsafe(link_section = ".text.slate_distinct.pager.getPageNormal")]
extern "C-unwind" fn getPageNormal(
    mut pPager: *mut Pager,
    mut pgno: u32,
    mut ppPage: *mut *mut PgHdr,
    mut flags: i32,
) -> i32 {
    let mut __slate_storage_1640: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1640: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1640) as *mut u32;
    let mut __slate_storage_1639: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1639: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1639) as *mut u32;
    let mut __slate_storage_1638: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1638: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_1638) as *mut *mut u32;
    let mut __slate_storage_1643: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1643: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1643) as *mut u32;
    let mut __slate_storage_1642: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1642: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1642) as *mut u32;
    let mut __slate_storage_1641: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1641: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_1641) as *mut *mut u32;
    let mut __slate_storage_1637: std::mem::MaybeUninit<*mut PgHdr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1637: *mut *mut PgHdr =
        std::ptr::addr_of_mut!(__slate_storage_1637) as *mut *mut PgHdr;
    let mut __slate_storage_992: std::mem::MaybeUninit<*mut sqlite3_pcache_page> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_992: *mut *mut sqlite3_pcache_page =
        std::ptr::addr_of_mut!(__slate_storage_992) as *mut *mut sqlite3_pcache_page;
    let mut __slate_storage_991: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_991: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_991) as *mut u8;
    let mut __slate_storage_990: std::mem::MaybeUninit<*mut PgHdr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_990: *mut *mut PgHdr =
        std::ptr::addr_of_mut!(__slate_storage_990) as *mut *mut PgHdr;
    let mut __slate_storage_989: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_989: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_989) as *mut i32;
    unsafe {
        std::ptr::write(__slate_slot_989, 0 as i32);
        // /* True if PAGER_GET_NOCONTENT is set */
        0 as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        if pgno == ((0 as i32) as u32) {
            return unsafe { sqlite3CorruptError(5613 as i32) };
        } else {
            '__join_2: {
                *__slate_slot_992 =
                    unsafe { sqlite3PcacheFetch(unsafe { (*pPager).pPCache }, pgno, 3 as i32) };
                if *__slate_slot_992 == std::ptr::null_mut::<sqlite3_pcache_page>() {
                    *__slate_slot_990 = std::ptr::null_mut::<PgHdr>();
                    *__slate_slot_989 = unsafe {
                        sqlite3PcacheFetchStress(
                            unsafe { (*pPager).pPCache },
                            pgno,
                            std::ptr::addr_of_mut!(*__slate_slot_992),
                        )
                    };
                    if *__slate_slot_989 != (0 as i32) {
                        break '__join_2;
                    } else {
                        if *__slate_slot_992 == std::ptr::null_mut::<sqlite3_pcache_page>() {
                            *__slate_slot_989 = 7 as i32;
                            break '__join_2;
                        }
                    }
                }
                std::ptr::write(__slate_slot_1637, unsafe {
                    sqlite3PcacheFetchFinish(unsafe { (*pPager).pPCache }, pgno, *__slate_slot_992)
                });
                unsafe {
                    *ppPage = *__slate_slot_1637;
                }
                *__slate_slot_990 = *__slate_slot_1637;
                0 as i32;
                0 as i32;
                0 as i32;
                *__slate_slot_991 = (flags & (1 as i32) != (0 as i32)) as u8;
                if (unsafe { (*(*__slate_slot_990)).pPager }) != std::ptr::null_mut::<Pager>()
                    && !(*__slate_slot_991 != (0 as u8))
                {
                    // /* In this case the pcache already contains an initialized copy of
                    //     ** the page. Return without further ado.  */
                    0 as i32;
                    std::ptr::write(__slate_slot_1638, unsafe {
                        unsafe { (*pPager).aStat.as_mut_ptr() as *mut u32 }
                            .offset((0 as i32) as isize)
                    });
                    std::ptr::write(__slate_slot_1639, unsafe { *(*__slate_slot_1638) });
                    std::ptr::write(
                        __slate_slot_1640,
                        (*__slate_slot_1639).wrapping_add((1 as i32) as u32),
                    );
                    unsafe {
                        *(*__slate_slot_1638) = *__slate_slot_1640;
                    }
                    return 0 as i32;
                } else {
                    // /* The pager cache has created a new page. Its content needs to
                    //     ** be initialized. But first some error checks:
                    //     **
                    //     ** (*) obsolete.  Was: maximum page number is 2^31
                    //     ** (2) Never try to fetch the locking page
                    //     */
                    if pgno == unsafe { (*pPager).lckPgno } {
                        *__slate_slot_989 = unsafe { sqlite3CorruptError(5645 as i32) };
                    } else {
                        unsafe {
                            (*(*__slate_slot_990)).pPager = pPager;
                        }
                        0 as i32;
                        if !((unsafe { (*unsafe { (*pPager).fd }).pMethods })
                            != std::ptr::null::<sqlite3_io_methods>())
                            || (unsafe { (*pPager).dbSize }) < pgno
                            || *__slate_slot_991 != (0 as u8)
                        {
                            if pgno > unsafe { (*pPager).mxPgno } {
                                *__slate_slot_989 = 13 as i32;
                                if pgno <= unsafe { (*pPager).dbSize } {
                                    unsafe { sqlite3PcacheRelease(*__slate_slot_990) };
                                    *__slate_slot_990 = std::ptr::null_mut::<PgHdr>();
                                    break '__join_2;
                                } else {
                                    break '__join_2;
                                }
                            } else {
                                if *__slate_slot_991 != (0 as u8) {
                                    // /* Failure to set the bits in the InJournal bit-vectors is benign.
                                    //         ** It merely means that we might do some extra work to journal a
                                    //         ** page that does not need to be journaled.  Nevertheless, be sure
                                    //         ** to test the case where a malloc error occurs while trying to set
                                    //         ** a bit in a bit vector.
                                    //         */
                                    unsafe { sqlite3BeginBenignMalloc() };
                                    if pgno <= unsafe { (*pPager).dbOrigSize } {
                                        unsafe {
                                            sqlite3BitvecSet(unsafe { (*pPager).pInJournal }, pgno)
                                        };
                                        {}
                                    }
                                    addToSavepointBitvecs(pPager, pgno);
                                    {}
                                    unsafe { sqlite3EndBenignMalloc() };
                                }
                                unsafe {
                                    memset(
                                        unsafe { (*(*__slate_slot_990)).pData },
                                        0 as i32,
                                        (unsafe { (*pPager).pageSize }) as u64,
                                    )
                                };
                                {}
                            }
                        } else {
                            0 as i32;
                            std::ptr::write(__slate_slot_1641, unsafe {
                                unsafe { (*pPager).aStat.as_mut_ptr() as *mut u32 }
                                    .offset((1 as i32) as isize)
                            });
                            std::ptr::write(__slate_slot_1642, unsafe { *(*__slate_slot_1641) });
                            std::ptr::write(
                                __slate_slot_1643,
                                (*__slate_slot_1642).wrapping_add((1 as i32) as u32),
                            );
                            unsafe {
                                *(*__slate_slot_1641) = *__slate_slot_1643;
                            }
                            *__slate_slot_989 = readDbPage(*__slate_slot_990);
                            if *__slate_slot_989 != (0 as i32) {
                                break '__join_2;
                            }
                        }
                        {}
                        return 0 as i32;
                    }
                }
            }
            0 as i32;
            if *__slate_slot_990 != std::ptr::null_mut::<PgHdr>() {
                unsafe { sqlite3PcacheDrop(*__slate_slot_990) };
            }
            pagerUnlockIfUnused(pPager);
            unsafe {
                *ppPage = std::ptr::null_mut::<PgHdr>();
            }
            return *__slate_slot_989;
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /* The pager open on the database file */
// /* Page number to fetch */
// /* Write a pointer to the page here */
// /* PAGER_GET_XXX flags */
// /* SQLITE_MAX_MMAP_SIZE>0 */
// /* The page getter method for when the pager is an error state */
#[unsafe(link_section = ".text.slate_distinct.pager.getPageError")]
extern "C-unwind" fn getPageError(
    mut pPager: *mut Pager,
    mut pgno: u32,
    mut ppPage: *mut *mut PgHdr,
    mut flags: i32,
) -> i32 {
    pgno;
    flags;
    0 as i32;
    unsafe {
        *ppPage = std::ptr::null_mut::<PgHdr>();
    }
    return unsafe { (*pPager).errCode };
}

// /* The pager open on the database file */
// /* Page number to fetch */
// /* Write a pointer to the page here */
// /* PAGER_GET_XXX flags */
// /* The page getter for when memory-mapped I/O is enabled */
#[unsafe(link_section = ".text.slate_distinct.pager.getPageMMap")]
extern "C-unwind" fn getPageMMap(
    mut pPager: *mut Pager,
    mut pgno: u32,
    mut ppPage: *mut *mut PgHdr,
    mut flags: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pPg: *mut PgHdr = std::ptr::null_mut::<PgHdr>();
    // /* Frame to read from WAL file */
    let mut iFrame: u32 = (0 as i32) as u32;
    // /* It is acceptable to use a read-only (mmap) page for any page except
    //   ** page 1 if there is no write-transaction open or the ACQUIRE_READONLY
    //   ** flag was specified by the caller. And so long as the db is not a
    //   ** temporary or in-memory database.  */
    let mut bMmapOk: i32 = (pgno > ((1 as i32) as u32)
        && ((((unsafe { (*pPager).eState }) as u32) as i32) == (1 as i32)
            || flags & (2 as i32) != (0 as i32))) as i32;
    0 as i32;
    // /* Optimization note:  Adding the "pgno<=1" term before "pgno==0" here
    //   ** allows the compiler optimizer to reuse the results of the "pgno>1"
    //   ** test in the previous statement, and avoid testing pgno==0 in the
    //   ** common case where pgno is large. */
    if pgno <= ((1 as i32) as u32) && pgno == ((0 as i32) as u32) {
        return unsafe { sqlite3CorruptError(5728 as i32) };
    }
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if bMmapOk != (0 as i32) && (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
        rc = unsafe {
            sqlite3WalFindFrame(
                unsafe { (*pPager).pWal },
                pgno,
                std::ptr::addr_of_mut!(iFrame),
            )
        };
        if rc != (0 as i32) {
            unsafe {
                *ppPage = std::ptr::null_mut::<PgHdr>();
            }
            return rc;
        }
    }
    if bMmapOk != (0 as i32) && iFrame == ((0 as i32) as u32) {
        let mut pData: *mut () = std::ptr::null_mut::<()>();
        rc = unsafe {
            sqlite3OsFetch(
                unsafe { (*pPager).fd },
                ((pgno.wrapping_sub((1 as i32) as u32) as u64) as i64)
                    * unsafe { (*pPager).pageSize },
                (unsafe { (*pPager).pageSize }) as i32,
                std::ptr::addr_of_mut!(pData),
            )
        };
        if rc == (0 as i32) && pData != std::ptr::null_mut::<()>() {
            if (((unsafe { (*pPager).eState }) as u32) as i32) > (1 as i32)
                || (unsafe { (*pPager).tempFile }) != (0 as u8)
            {
                pPg = sqlite3PagerLookup(pPager, pgno);
            }
            if pPg == std::ptr::null_mut::<PgHdr>() {
                rc = pagerAcquireMapPage(pPager, pgno, pData, std::ptr::addr_of_mut!(pPg));
            } else {
                unsafe {
                    sqlite3OsUnfetch(
                        unsafe { (*pPager).fd },
                        ((pgno.wrapping_sub((1 as i32) as u32) as u64) as i64)
                            * unsafe { (*pPager).pageSize },
                        pData,
                    )
                };
            }
            if pPg != std::ptr::null_mut::<PgHdr>() {
                0 as i32;
                unsafe {
                    *ppPage = pPg;
                }
                return 0 as i32;
            }
        }
        if rc != (0 as i32) {
            unsafe {
                *ppPage = std::ptr::null_mut::<PgHdr>();
            }
            return rc;
        }
    }
    return getPageNormal(pPager, pgno, ppPage, flags);
}

// /* Forward references to the various page getters */
// /*
// ** Set the Pager.xGet method for the appropriate routine used to fetch
// ** content from the pager.
// */
fn setGetterMethod(mut pPager: *mut Pager) {
    if (unsafe { (*pPager).errCode }) != (0 as i32) {
        unsafe {
            (*pPager).xGet = Some(getPageError);
        }
    } else {
        if (unsafe { (*pPager).bUseFetch }) != (0 as u8) {
            unsafe {
                (*pPager).xGet = Some(getPageMMap);
            }
        // /* SQLITE_MAX_MMAP_SIZE>0 */
        } else {
            unsafe {
                (*pPager).xGet = Some(getPageNormal);
            }
        }
    }
}

// /*
// ** Return true if it is necessary to write page *pPg into the sub-journal.
// ** A page needs to be written into the sub-journal if there exists one
// ** or more open savepoints for which:
// **
// **   * The page-number is less than or equal to PagerSavepoint.nOrig, and
// **   * The bit corresponding to the page-number is not set in
// **     PagerSavepoint.pInSavepoint.
// */
fn subjRequiresPage(mut pPg: *mut PgHdr) -> i32 {
    let mut pPager: *mut Pager = unsafe { (*pPg).pPager };
    let mut p: *mut PagerSavepoint = unsafe { std::mem::zeroed() };
    let mut pgno: u32 = unsafe { (*pPg).pgno };
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_1508: loop {
        if !(i < unsafe { (*pPager).nSavepoint }) {
            break;
        }
        p = unsafe { unsafe { (*pPager).aSavepoint }.offset(i as isize) };
        let __v1646: bool;
        if (unsafe { (*p).nOrig }) >= pgno {
            __v1646 = (0 as i32)
                == unsafe { sqlite3BitvecTestNotNull(unsafe { (*p).pInSavepoint }, pgno) };
        } else {
            __v1646 = false as bool;
        }
        if __v1646 {
            i = i + (1 as i32);
            '__slate_break_1509: loop {
                if !(i < unsafe { (*pPager).nSavepoint }) {
                    break;
                }
                unsafe {
                    (*unsafe { unsafe { (*pPager).aSavepoint }.offset(i as isize) })
                        .bTruncateOnRelease = 0 as i32;
                }
                let __v1647: i32 = i;
                let __v1648: i32 = __v1647 + (1 as i32);
                i = __v1648;
            }
            return 1 as i32;
        }
        let __v1644: i32 = i;
        let __v1645: i32 = __v1644 + (1 as i32);
        i = __v1645;
    }
    return 0 as i32;
}

// /*
// ** Read a 32-bit integer from the given file descriptor.  Store the integer
// ** that is read in *pRes.  Return SQLITE_OK if everything worked, or an
// ** error code is something goes wrong.
// **
// ** All values are stored on disk as big-endian.
// */
fn read32bits(mut fd: *mut sqlite3_file, mut offset: i64, mut pRes: *mut u32) -> i32 {
    let mut ac: [u8; 4] = [0 as u8; 4];
    let mut rc: i32 = unsafe {
        sqlite3OsRead(
            fd,
            (ac.as_mut_ptr() as *mut u8) as *mut (),
            ((4 as u64) as u32) as i32,
            offset,
        )
    };
    if rc == (0 as i32) {
        unsafe {
            *pRes = unsafe { sqlite3Get4byte((ac.as_mut_ptr() as *mut u8) as *const u8) };
        }
    }
    return rc;
}

// /*
// ** Write a 32-bit integer into a string buffer in big-endian byte order.
// */
// /*
// ** Write a 32-bit integer into the given file descriptor.  Return SQLITE_OK
// ** on success or an error code is something goes wrong.
// */
fn write32bits(mut fd: *mut sqlite3_file, mut offset: i64, mut val: u32) -> i32 {
    let mut ac: [i8; 4] = [0 as i8; 4];
    unsafe { sqlite3Put4byte((ac.as_mut_ptr() as *mut i8) as *mut u8, val) };
    return unsafe {
        sqlite3OsWrite(
            fd,
            (ac.as_mut_ptr() as *mut i8) as *const (),
            4 as i32,
            offset,
        )
    };
}

// /*
// ** Unlock the database file to level eLock, which must be either NO_LOCK
// ** or SHARED_LOCK. Regardless of whether or not the call to xUnlock()
// ** succeeds, set the Pager.eLock variable to match the (attempted) new lock.
// **
// ** Except, if Pager.eLock is set to UNKNOWN_LOCK when this function is
// ** called, do not modify it. See the comment above the #define of
// ** UNKNOWN_LOCK for an explanation of this.
// */
fn pagerUnlockDb(mut pPager: *mut Pager, mut eLock: i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe { (*unsafe { (*pPager).fd }).pMethods }) != std::ptr::null::<sqlite3_io_methods>() {
        0 as i32;
        let __v1649: i32;
        if (unsafe { (*pPager).noLock }) != (0 as u8) {
            __v1649 = 0 as i32;
        } else {
            __v1649 = unsafe { sqlite3OsUnlock(unsafe { (*pPager).fd }, eLock) };
        }
        rc = __v1649;
        if (((unsafe { (*pPager).eLock }) as u32) as i32) != (4 as i32) + (1 as i32) {
            unsafe {
                (*pPager).eLock = (eLock as i8) as u8;
            }
        }
    }
    // /* ticket fb3b3024ea238d5c */
    unsafe {
        (*pPager).changeCountDone = unsafe { (*pPager).tempFile };
    }
    return rc;
}

// /*
// ** Lock the database file to level eLock, which must be either SHARED_LOCK,
// ** RESERVED_LOCK or EXCLUSIVE_LOCK. If the caller is successful, set the
// ** Pager.eLock variable to the new locking state.
// **
// ** Except, if Pager.eLock is set to UNKNOWN_LOCK when this function is
// ** called, do not modify it unless the new locking state is EXCLUSIVE_LOCK.
// ** See the comment above the #define of UNKNOWN_LOCK for an explanation
// ** of this.
// */
fn pagerLockDb(mut pPager: *mut Pager, mut eLock: i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    if (((unsafe { (*pPager).eLock }) as u32) as i32) < eLock
        || (((unsafe { (*pPager).eLock }) as u32) as i32) == (4 as i32) + (1 as i32)
    {
        let __v1650: i32;
        if (unsafe { (*pPager).noLock }) != (0 as u8) {
            __v1650 = 0 as i32;
        } else {
            __v1650 = unsafe { sqlite3OsLock(unsafe { (*pPager).fd }, eLock) };
        }
        rc = __v1650;
        if rc == (0 as i32)
            && ((((unsafe { (*pPager).eLock }) as u32) as i32) != (4 as i32) + (1 as i32)
                || eLock == (4 as i32))
        {
            unsafe {
                (*pPager).eLock = (eLock as i8) as u8;
            }
        }
    }
    return rc;
}

// /*
// ** This function determines whether or not the atomic-write or
// ** atomic-batch-write optimizations can be used with this pager. The
// ** atomic-write optimization can be used if:
// **
// **  (a) the value returned by OsDeviceCharacteristics() indicates that
// **      a database page may be written atomically, and
// **  (b) the value returned by OsSectorSize() is less than or equal
// **      to the page size.
// **
// ** If it can be used, then the value returned is the size of the journal
// ** file when it contains rollback data for exactly one page.
// **
// ** The atomic-batch-write optimization can be used if OsDeviceCharacteristics()
// ** returns a value with the SQLITE_IOCAP_BATCH_ATOMIC bit set. -1 is
// ** returned in this case.
// **
// ** If neither optimization can be used, 0 is returned.
// */
fn jrnlBufferSize(mut pPager: *mut Pager) -> i32 {
    0 as i32;
    pPager;
    return 0 as i32;
}

// /*
// ** If SQLITE_CHECK_PAGES is defined then we do some sanity checking
// ** on the cache using a hash function.  This is used for testing
// ** and debugging only.
// */
// /* SQLITE_CHECK_PAGES */
// /*
// ** Free a buffer allocated by the readSuperJournal() function.
// */
fn freeSuperJournal(mut zSuper: *mut i8) {
    if zSuper != std::ptr::null_mut::<i8>() {
        unsafe { sqlite3_free((unsafe { zSuper.offset(-(4 as i32) as isize) }) as *mut ()) };
    }
}

// /*
// ** Check if zSuper is a valid super-journal name. There are two valid
// ** formats:
// **
// **   + The 3rd and 4th last bytes of the filename are ".9", and the
// **     following 2 bytes are hex digits. This is a file created in 8.3
// **     filenames mode.
// **
// **   + The 3rd last byte of the filename is "9" and the filename
// **     contains the string "-mj" starting at the 12th last byte.
// **     All bytes following the "-mj" are hex digits.
// **
// ** If the filename matches either of these patterns, return non-zero.
// ** Otherwise, return zero.
// */
fn pagerIsSuperJrnlName(mut zSuper: *const i8) -> i32 {
    let mut nSuper: i32 = unsafe { sqlite3Strlen30(zSuper) };
    let mut ii: i32 = 0 as i32;
    if nSuper < (12 as i32) {
        return 0 as i32;
    }
    if (unsafe {
        memcmp(
            (unsafe { zSuper.offset((nSuper - (12 as i32)) as isize) }) as *const (),
            (b"-mj\0".as_ptr() as *mut i8) as *const (),
            ((3 as i32) as i64) as u64,
        )
    }) != (0 as i32)
    {
        return 0 as i32;
    }
    if ((unsafe { *unsafe { zSuper.offset((nSuper - (3 as i32)) as isize) } }) as i32)
        != (57 as i32)
    {
        return 0 as i32;
    }
    ii = nSuper - (9 as i32);
    '__slate_break_1511: loop {
        if !(ii < nSuper) {
            break;
        }
        if (((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                    ((((unsafe { *unsafe { zSuper.offset(ii as isize) } }) as u8) as u32) as i32)
                        as isize,
                )
            }
        }) as u32) as i32)
            & (8 as i32)
            == (0 as i32)
        {
            return 0 as i32;
        }
        let __v1651: i32 = ii;
        let __v1652: i32 = __v1651 + (1 as i32);
        ii = __v1652;
    }
    return 1 as i32;
}

// /*
// ** Parameter pJrnl is a file-handle open on a journal file. This function
// ** attempts to read a super-journal file name from the end of the journal
// ** file. If successful, it sets output parameter (*pzSuper) to point to a
// ** buffer containing the super-journal name as a nul-terminated string.
// ** The caller is responsible for freeing the buffer using freeSuperJournal().
// **
// ** Refer to comments above writeSuperJournal() for the format used to store
// ** a super-journal file name at the end of a journal file.
// **
// ** Parameter nSuper is passed the maximum allowable size of the super journal
// ** name in bytes. If the super-journal name in the journal is longer than
// ** nSuper bytes (including a nul-terminator), then this is handled as if no
// ** super-journal name were present in the journal.
// **
// ** If there is no super-journal name at the end of pJrnl, (*pzSuper) is
// ** set to 0 and SQLITE_OK is returned. Or, if an error occurs while reading
// ** the super-journal name, an SQLite error code is returned and (*pzSuper)
// ** is set to 0.
// */
fn readSuperJournal(
    mut pJrnl: *mut sqlite3_file,
    mut nSuper: u64,
    mut pzSuper: *mut *mut i8,
) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* Length in bytes of super-journal name */
    let mut len: u32 = 0 as u32;
    // /* Total size in bytes of journal file pJrnl */
    let mut szJ: i64 = 0 as i64;
    // /* MJ checksum value read from journal */
    let mut cksum: u32 = 0 as u32;
    // /* A buffer to hold the magic header */
    let mut aMagic: [u8; 8] = [0 as u8; 8];
    let mut zOut: *mut i8 = std::ptr::null_mut::<i8>();
    unsafe {
        *pzSuper = std::ptr::null_mut::<i8>();
    }
    let __v1653: i32 = unsafe { sqlite3OsFileSize(pJrnl, std::ptr::addr_of_mut!(szJ)) };
    rc = __v1653;
    let __v1654: bool;
    if (0 as i32) != __v1653 || szJ < ((16 as i32) as i64) {
        __v1654 = true as bool;
    } else {
        let __v1655: i32 = read32bits(
            pJrnl,
            szJ - ((16 as i32) as i64),
            std::ptr::addr_of_mut!(len),
        );
        rc = __v1655;
        __v1654 = (0 as i32) != __v1655;
    }
    let __v1656: bool;
    if __v1654
        || (len as u64) >= nSuper
        || ((len as u64) as i64) > szJ - ((16 as i32) as i64)
        || len == ((0 as i32) as u32)
    {
        __v1656 = true as bool;
    } else {
        let __v1657: i32 = read32bits(
            pJrnl,
            szJ - ((12 as i32) as i64),
            std::ptr::addr_of_mut!(cksum),
        );
        rc = __v1657;
        __v1656 = (0 as i32) != __v1657;
    }
    let __v1658: bool;
    if __v1656 {
        __v1658 = true as bool;
    } else {
        let __v1659: i32 = unsafe {
            sqlite3OsRead(
                pJrnl,
                (aMagic.as_mut_ptr() as *mut u8) as *mut (),
                8 as i32,
                szJ - ((8 as i32) as i64),
            )
        };
        rc = __v1659;
        __v1658 = (0 as i32) != __v1659;
    }
    if __v1658 {
        return rc;
    }
    zOut = (unsafe {
        sqlite3MallocZero(
            ((4 as i32) as u32)
                .wrapping_add(len)
                .wrapping_add((2 as i32) as u32) as u64,
        )
    }) as *mut i8;
    if !(zOut != std::ptr::null_mut::<i8>()) {
        rc = if (unsafe {
            memcmp(
                (aMagic.as_mut_ptr() as *mut u8) as *const (),
                (unsafe { std::ptr::addr_of!(aJournalMagic) as *const u8 }) as *const (),
                ((8 as i32) as i64) as u64,
            )
        }) != (0 as i32)
        {
            0 as i32
        } else {
            7 as i32
        };
    } else {
        zOut = unsafe { zOut.offset((4 as i32) as isize) };
        let __v1660: i32 = unsafe {
            sqlite3OsRead(
                pJrnl,
                zOut as *mut (),
                len as i32,
                szJ - ((16 as i32) as i64) - ((len as u64) as i64),
            )
        };
        rc = __v1660;
        if (0 as i32) == __v1660 {
            // /* Unsigned loop counter */
            let mut u: u32 = 0 as u32;
            // /* See if the checksum matches the super-journal name */
            u = (0 as i32) as u32;
            '__slate_break_1512: while u < len {
                let __v1663: u32 = cksum;
                let __v1664: u32 = __v1663
                    .wrapping_sub(((unsafe { *unsafe { zOut.offset(u as isize) } }) as i32) as u32);
                cksum = __v1664;
                let __v1661: u32 = u;
                let __v1662: u32 = __v1661.wrapping_add((1 as i32) as u32);
                u = __v1662;
            }
        }
        // /* Couldn't read the name */
        let __v1665: bool;
        if rc != (0 as i32) {
            __v1665 = true as bool;
        } else {
            __v1665 = !(pagerIsSuperJrnlName(zOut as *const i8) != (0 as i32));
        }
        if __v1665
            || cksum != (0 as u32)
            || (unsafe {
                memcmp(
                    (aMagic.as_mut_ptr() as *mut u8) as *const (),
                    (unsafe { std::ptr::addr_of!(aJournalMagic) as *const u8 }) as *const (),
                    ((8 as i32) as i64) as u64,
                )
            }) != (0 as i32)
        {
            // /* If any validity checks fail, that means the super-journal filename
            //       ** is corrupted, so rollback.  Return SQLITE_K and a NULL super-journal
            //       ** name */
            freeSuperJournal(zOut);
            zOut = std::ptr::null_mut::<i8>();
        }
        // /* Name is not valid */
        // /* checksum is incorrect */
        // /* Bad magic number */
    }
    unsafe {
        *pzSuper = zOut;
    }
    return rc;
}

// /*
// ** Return the offset of the sector boundary at or immediately
// ** following the value in pPager->journalOff, assuming a sector
// ** size of pPager->sectorSize bytes.
// **
// ** i.e for a sector size of 512:
// **
// **   Pager.journalOff          Return value
// **   ---------------------------------------
// **   0                         0
// **   512                       512
// **   100                       512
// **   2000                      2048
// **
// */
fn journalHdrOffset(mut pPager: *mut Pager) -> i64 {
    let mut offset: i64 = (0 as i32) as i64;
    let mut c: i64 = unsafe { (*pPager).journalOff };
    if c != (0 as i64) {
        offset = ((c - ((1 as i32) as i64)) / (((unsafe { (*pPager).sectorSize }) as u64) as i64)
            + ((1 as i32) as i64))
            * (((unsafe { (*pPager).sectorSize }) as u64) as i64);
    }
    0 as i32;
    0 as i32;
    0 as i32;
    return offset;
}

// /*
// ** The journal file must be open when this function is called.
// **
// ** This function is a no-op if the journal file has not been written to
// ** within the current transaction (i.e. if Pager.journalOff==0).
// **
// ** If doTruncate is non-zero or the Pager.journalSizeLimit variable is
// ** set to 0, then truncate the journal file to zero bytes in size. Otherwise,
// ** zero the 28-byte header at the start of the journal file. In either case,
// ** if the pager is not in no-sync mode, sync the journal file immediately
// ** after writing or truncating it.
// **
// ** If Pager.journalSizeLimit is set to a positive, non-zero value, and
// ** following the truncation or zeroing described above the size of the
// ** journal file in bytes is larger than this value, then truncate the
// ** journal file to Pager.journalSizeLimit bytes. The journal file does
// ** not need to be synced following this operation.
// **
// ** If an IO error occurs, abandon processing and return the IO error code.
// ** Otherwise, return SQLITE_OK.
// */
fn zeroJournalHdr(mut pPager: *mut Pager, mut doTruncate: i32) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe { (*pPager).journalOff }) != (0 as i64) {
        // /* Local cache of jsl */
        let mut iLimit: i64 = unsafe { (*pPager).journalSizeLimit };
        if doTruncate != (0 as i32) || iLimit == ((0 as i32) as i64) {
            rc = unsafe { sqlite3OsTruncate(unsafe { (*pPager).jfd }, (0 as i32) as i64) };
        } else {
            rc = unsafe {
                sqlite3OsWrite(
                    unsafe { (*pPager).jfd },
                    (unsafe { std::ptr::addr_of!(zeroHdr.0) as *const i8 }) as *const (),
                    ((28 as u64) as u32) as i32,
                    (0 as i32) as i64,
                )
            };
        }
        if rc == (0 as i32) && !((unsafe { (*pPager).noSync }) != (0 as u8)) {
            rc = unsafe {
                sqlite3OsSync(
                    unsafe { (*pPager).jfd },
                    (16 as i32) | (((unsafe { (*pPager).syncFlags }) as u32) as i32),
                )
            };
        }
        // /* At this point the transaction is committed but the write lock
        //     ** is still held on the file. If there is a size limit configured for
        //     ** the persistent journal and the journal file currently consumes more
        //     ** space than that limit allows for, truncate it now. There is no need
        //     ** to sync the file following this operation.
        //     */
        if rc == (0 as i32) && iLimit > ((0 as i32) as i64) {
            let mut sz: i64 = 0 as i64;
            rc = unsafe { sqlite3OsFileSize(unsafe { (*pPager).jfd }, std::ptr::addr_of_mut!(sz)) };
            if rc == (0 as i32) && sz > iLimit {
                rc = unsafe { sqlite3OsTruncate(unsafe { (*pPager).jfd }, iLimit) };
            }
        }
    }
    return rc;
}

// /*
// ** The journal file must be open when this routine is called. A journal
// ** header (JOURNAL_HDR_SZ bytes) is written into the journal file at the
// ** current location.
// **
// ** The format for the journal header is as follows:
// ** - 8 bytes: Magic identifying journal format.
// ** - 4 bytes: Number of records in journal, or -1 no-sync mode is on.
// ** - 4 bytes: Random number used for page hash.
// ** - 4 bytes: Initial database page count.
// ** - 4 bytes: Sector size used by the process that wrote this journal.
// ** - 4 bytes: Database page size.
// **
// ** Followed by (JOURNAL_HDR_SZ - 28) bytes of unused space.
// */
fn writeJournalHdr(mut pPager: *mut Pager) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* Temporary space used to build header */
    let mut zHeader: *mut i8 = unsafe { (*pPager).pTmpSpace };
    // /* Size of buffer pointed to by zHeader */
    let mut nHeader: u32 = ((unsafe { (*pPager).pageSize }) as i32) as u32;
    // /* Bytes of header sector written */
    let mut nWrite: u32 = 0 as u32;
    // /* Loop counter */
    let mut ii: i32 = 0 as i32;
    // /* Journal file must be open. */
    0 as i32;
    if nHeader > unsafe { (*pPager).sectorSize } {
        nHeader = unsafe { (*pPager).sectorSize };
    }
    // /* If there are active savepoints and any of them were created
    //   ** since the most recent journal header was written, update the
    //   ** PagerSavepoint.iHdrOffset fields now.
    //   */
    ii = 0 as i32;
    '__slate_break_1513: loop {
        if !(ii < unsafe { (*pPager).nSavepoint }) {
            break;
        }
        if (unsafe { (*unsafe { unsafe { (*pPager).aSavepoint }.offset(ii as isize) }).iHdrOffset })
            == ((0 as i32) as i64)
        {
            unsafe {
                (*unsafe { unsafe { (*pPager).aSavepoint }.offset(ii as isize) }).iHdrOffset =
                    unsafe { (*pPager).journalOff };
            }
        }
        let __v1666: i32 = ii;
        let __v1667: i32 = __v1666 + (1 as i32);
        ii = __v1667;
    }
    let __v1668: i64 = journalHdrOffset(pPager);
    unsafe {
        (*pPager).journalOff = __v1668;
    }
    unsafe {
        (*pPager).journalHdr = __v1668;
    }
    // /*
    //   ** Write the nRec Field - the number of page records that follow this
    //   ** journal header. Normally, zero is written to this value at this time.
    //   ** After the records are added to the journal (and the journal synced,
    //   ** if in full-sync mode), the zero is overwritten with the true number
    //   ** of records (see syncJournal()).
    //   **
    //   ** A faster alternative is to write 0xFFFFFFFF to the nRec field. When
    //   ** reading the journal this value tells SQLite to assume that the
    //   ** rest of the journal file contains valid page records. This assumption
    //   ** is dangerous, as if a failure occurred whilst writing to the journal
    //   ** file it may contain some garbage data. There are two scenarios
    //   ** where this risk can be ignored:
    //   **
    //   **   * When the pager is in no-sync mode. Corruption can follow a
    //   **     power failure in this case anyway.
    //   **
    //   **   * When the SQLITE_IOCAP_SAFE_APPEND flag is set. This guarantees
    //   **     that garbage data is never appended to the journal file.
    //   */
    0 as i32;
    let __v1669: bool;
    if (unsafe { (*pPager).noSync }) != (0 as u8)
        || (((unsafe { (*pPager).journalMode }) as u32) as i32) == (4 as i32)
    {
        __v1669 = true as bool;
    } else {
        __v1669 = (unsafe { sqlite3OsDeviceCharacteristics(unsafe { (*pPager).fd }) })
            & (512 as i32)
            != (0 as i32);
    }
    if __v1669 {
        unsafe {
            memcpy(
                zHeader as *mut (),
                (unsafe { std::ptr::addr_of!(aJournalMagic) as *const u8 }) as *const (),
                8 as u64,
            )
        };
        unsafe {
            sqlite3Put4byte(
                (unsafe { zHeader.offset((8 as u64) as isize) }) as *mut u8,
                4294967295 as u32,
            )
        };
    } else {
        unsafe {
            memset(
                zHeader as *mut (),
                0 as i32,
                (8 as u64).wrapping_add(((4 as i32) as i64) as u64),
            )
        };
    }
    // /* The random check-hash initializer */
    if (((unsafe { (*pPager).journalMode }) as u32) as i32) != (4 as i32) {
        unsafe {
            sqlite3_randomness(
                ((4 as u64) as u32) as i32,
                (unsafe { std::ptr::addr_of_mut!((*pPager).cksumInit) }) as *mut (),
            )
        };
    }
    unsafe {
        sqlite3Put4byte(
            (unsafe {
                zHeader.offset((8 as u64).wrapping_add(((4 as i32) as i64) as u64) as isize)
            }) as *mut u8,
            unsafe { (*pPager).cksumInit },
        )
    };
    // /* The initial database size */
    unsafe {
        sqlite3Put4byte(
            (unsafe {
                zHeader.offset((8 as u64).wrapping_add(((8 as i32) as i64) as u64) as isize)
            }) as *mut u8,
            unsafe { (*pPager).dbOrigSize },
        )
    };
    // /* The assumed sector size for this process */
    unsafe {
        sqlite3Put4byte(
            (unsafe {
                zHeader.offset((8 as u64).wrapping_add(((12 as i32) as i64) as u64) as isize)
            }) as *mut u8,
            unsafe { (*pPager).sectorSize },
        )
    };
    // /* The page size */
    unsafe {
        sqlite3Put4byte(
            (unsafe {
                zHeader.offset((8 as u64).wrapping_add(((16 as i32) as i64) as u64) as isize)
            }) as *mut u8,
            ((unsafe { (*pPager).pageSize }) as i32) as u32,
        )
    };
    // /* Initializing the tail of the buffer is not necessary.  Everything
    //   ** works find if the following memset() is omitted.  But initializing
    //   ** the memory prevents valgrind from complaining, so we are willing to
    //   ** take the performance hit.
    //   */
    unsafe {
        memset(
            (unsafe {
                zHeader.offset((8 as u64).wrapping_add(((20 as i32) as i64) as u64) as isize)
            }) as *mut (),
            0 as i32,
            (nHeader as u64).wrapping_sub((8 as u64).wrapping_add(((20 as i32) as i64) as u64)),
        )
    };
    // /* In theory, it is only necessary to write the 28 bytes that the
    //   ** journal header consumes to the journal file here. Then increment the
    //   ** Pager.journalOff variable by JOURNAL_HDR_SZ so that the next
    //   ** record is written to the following sector (leaving a gap in the file
    //   ** that will be implicitly filled in by the OS).
    //   **
    //   ** However it has been discovered that on some systems this pattern can
    //   ** be significantly slower than contiguously writing data to the file,
    //   ** even if that means explicitly writing data to the block of
    //   ** (JOURNAL_HDR_SZ - 28) bytes that will not be used. So that is what
    //   ** is done.
    //   **
    //   ** The loop is required here in case the sector-size is larger than the
    //   ** database page size. Since the zHeader buffer is only Pager.pageSize
    //   ** bytes in size, more than one call to sqlite3OsWrite() may be required
    //   ** to populate the entire journal header sector.
    //   */
    nWrite = (0 as i32) as u32;
    '__slate_break_1514: while rc == (0 as i32) && nWrite < unsafe { (*pPager).sectorSize } {
        rc = unsafe {
            sqlite3OsWrite(
                unsafe { (*pPager).jfd },
                zHeader as *const (),
                nHeader as i32,
                unsafe { (*pPager).journalOff },
            )
        };
        0 as i32;
        let __v1672: *mut Pager = pPager;
        let __v1673: i64 = unsafe { (*__v1672).journalOff };
        let __v1674: i64 = __v1673 + ((nHeader as u64) as i64);
        unsafe {
            (*__v1672).journalOff = __v1674;
        }
        let __v1670: u32 = nWrite;
        let __v1671: u32 = __v1670.wrapping_add(nHeader);
        nWrite = __v1671;
    }
    return rc;
}

// /*
// ** The journal file must be open when this is called. A journal header file
// ** (JOURNAL_HDR_SZ bytes) is read from the current location in the journal
// ** file. The current location in the journal file is given by
// ** pPager->journalOff. See comments above function writeJournalHdr() for
// ** a description of the journal header format.
// **
// ** If the header is read successfully, *pNRec is set to the number of
// ** page records following this header and *pDbSize is set to the size of the
// ** database before the transaction began, in pages. Also, pPager->cksumInit
// ** is set to the value read from the journal header. SQLITE_OK is returned
// ** in this case.
// **
// ** If the journal header file appears to be corrupted, SQLITE_DONE is
// ** returned and *pNRec and *PDbSize are undefined.  If JOURNAL_HDR_SZ bytes
// ** cannot be read from the journal file an error code is returned.
// */
fn readJournalHdr(
    mut pPager: *mut Pager,
    mut isHot: i32,
    mut journalSize: i64,
    mut pNRec: *mut u32,
    mut pDbSize: *mut u32,
) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* A buffer to hold the magic header */
    let mut aMagic: [u8; 8] = [0 as u8; 8];
    // /* Offset of journal header being read */
    let mut iHdrOff: i64 = 0 as i64;
    // /* Journal file must be open. */
    0 as i32;
    // /* Advance Pager.journalOff to the start of the next sector. If the
    //   ** journal file is too small for there to be a header stored at this
    //   ** point, return SQLITE_DONE.
    //   */
    unsafe {
        (*pPager).journalOff = journalHdrOffset(pPager);
    }
    if (unsafe { (*pPager).journalOff }) + (((unsafe { (*pPager).sectorSize }) as u64) as i64)
        > journalSize
    {
        return 101 as i32;
    }
    iHdrOff = unsafe { (*pPager).journalOff };
    // /* Read in the first 8 bytes of the journal header. If they do not match
    //   ** the  magic string found at the start of each journal header, return
    //   ** SQLITE_DONE. If an IO error occurs, return an error code. Otherwise,
    //   ** proceed.
    //   */
    if isHot != (0 as i32) || iHdrOff != unsafe { (*pPager).journalHdr } {
        rc = unsafe {
            sqlite3OsRead(
                unsafe { (*pPager).jfd },
                (aMagic.as_mut_ptr() as *mut u8) as *mut (),
                ((8 as u64) as u32) as i32,
                iHdrOff,
            )
        };
        if rc != (0 as i32) {
            return rc;
        }
        if (unsafe {
            memcmp(
                (aMagic.as_mut_ptr() as *mut u8) as *const (),
                (unsafe { std::ptr::addr_of!(aJournalMagic) as *const u8 }) as *const (),
                8 as u64,
            )
        }) != (0 as i32)
        {
            return 101 as i32;
        }
    }
    // /* Read the first three 32-bit fields of the journal header: The nRec
    //   ** field, the checksum-initializer and the database size at the start
    //   ** of the transaction. Return an error code if anything goes wrong.
    //   */
    let __v1675: i32 = read32bits(
        unsafe { (*pPager).jfd },
        iHdrOff + ((8 as i32) as i64),
        pNRec,
    );
    rc = __v1675;
    let __v1676: bool;
    if (0 as i32) != __v1675 {
        __v1676 = true as bool;
    } else {
        let __v1677: i32 = read32bits(
            unsafe { (*pPager).jfd },
            iHdrOff + ((12 as i32) as i64),
            unsafe { std::ptr::addr_of_mut!((*pPager).cksumInit) },
        );
        rc = __v1677;
        __v1676 = (0 as i32) != __v1677;
    }
    let __v1678: bool;
    if __v1676 {
        __v1678 = true as bool;
    } else {
        let __v1679: i32 = read32bits(
            unsafe { (*pPager).jfd },
            iHdrOff + ((16 as i32) as i64),
            pDbSize,
        );
        rc = __v1679;
        __v1678 = (0 as i32) != __v1679;
    }
    if __v1678 {
        return rc;
    }
    if (unsafe { (*pPager).journalOff }) == ((0 as i32) as i64) {
        // /* Page-size field of journal header */
        let mut iPageSize: u32 = 0 as u32;
        // /* Sector-size field of journal header */
        let mut iSectorSize: u32 = 0 as u32;
        // /* Read the page-size and sector-size journal header fields. */
        let __v1680: i32 = read32bits(
            unsafe { (*pPager).jfd },
            iHdrOff + ((20 as i32) as i64),
            std::ptr::addr_of_mut!(iSectorSize),
        );
        rc = __v1680;
        let __v1681: bool;
        if (0 as i32) != __v1680 {
            __v1681 = true as bool;
        } else {
            let __v1682: i32 = read32bits(
                unsafe { (*pPager).jfd },
                iHdrOff + ((24 as i32) as i64),
                std::ptr::addr_of_mut!(iPageSize),
            );
            rc = __v1682;
            __v1681 = (0 as i32) != __v1682;
        }
        if __v1681 {
            return rc;
        }
        // /* Versions of SQLite prior to 3.5.8 set the page-size field of the
        //     ** journal header to zero. In this case, assume that the Pager.pageSize
        //     ** variable is already set to the correct page size.
        //     */
        if iPageSize == ((0 as i32) as u32) {
            iPageSize = ((unsafe { (*pPager).pageSize }) as i32) as u32;
        }
        // /* Check that the values read from the page-size and sector-size fields
        //     ** are within range. To be 'in range', both values need to be a power
        //     ** of two greater than or equal to 512 or 32, and not greater than their
        //     ** respective compile time maximum limits.
        //     */
        if iPageSize < ((512 as i32) as u32)
            || iSectorSize < ((32 as i32) as u32)
            || iPageSize > ((65536 as i32) as u32)
            || iSectorSize > ((65536 as i32) as u32)
            || iPageSize.wrapping_sub((1 as i32) as u32) & iPageSize != ((0 as i32) as u32)
            || iSectorSize.wrapping_sub((1 as i32) as u32) & iSectorSize != ((0 as i32) as u32)
        {
            // /* If the either the page-size or sector-size in the journal-header is
            //       ** invalid, then the process that wrote the journal-header must have
            //       ** crashed before the header was synced. In this case stop reading
            //       ** the journal file here.
            //       */
            return 101 as i32;
        }
        // /* Update the page-size to match the value read from the journal.
        //     ** Use a testcase() macro to make sure that malloc failure within
        //     ** PagerSetPagesize() is tested.
        //     */
        rc = sqlite3PagerSetPagesize(pPager, std::ptr::addr_of_mut!(iPageSize), -(1 as i32));
        {}
        // /* Update the assumed sector-size to match the value used by
        //     ** the process that created this journal. If this journal was
        //     ** created by a process other than this one, then this routine
        //     ** is being called from within pager_playback(). The local value
        //     ** of Pager.sectorSize is restored at the end of that routine.
        //     */
        unsafe {
            (*pPager).sectorSize = iSectorSize;
        }
    }
    let __v1683: *mut Pager = pPager;
    let __v1684: i64 = unsafe { (*__v1683).journalOff };
    let __v1685: i64 = __v1684 + (((unsafe { (*pPager).sectorSize }) as u64) as i64);
    unsafe {
        (*__v1683).journalOff = __v1685;
    }
    return rc;
}

// /* Pager object */
// /* Size of the open journal file in bytes */
// /* OUT: Value read from the nRec field */
// /* OUT: Value of original database size field */
// /*
// ** Write the supplied super-journal name into the journal file for pager
// ** pPager at the current location. The super-journal name must be the last
// ** thing written to a journal file. If the pager is in full-sync mode, the
// ** journal file descriptor is advanced to the next sector boundary before
// ** anything is written. The format is:
// **
// **   + 4 bytes: PAGER_SJ_PGNO.
// **   + N bytes: super-journal filename in utf-8.
// **   + 4 bytes: N (length of super-journal name in bytes, no nul-terminator).
// **   + 4 bytes: super-journal name checksum.
// **   + 8 bytes: aJournalMagic[].
// **
// ** The super-journal page checksum is the sum of the bytes in the super-journal
// ** name, where each byte is interpreted as a signed 8-bit integer.
// **
// ** If zSuper is a NULL pointer (occurs for a single database transaction),
// ** this call is a no-op.
// */
fn writeSuperJournal(mut pPager: *mut Pager, mut zSuper: *const i8) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* Length of string zSuper */
    let mut nSuper: i32 = 0 as i32;
    // /* Offset of header in journal file */
    let mut iHdrOff: i64 = 0 as i64;
    // /* Size of journal file on disk */
    let mut jrnlSize: i64 = 0 as i64;
    // /* Checksum of string zSuper */
    let mut cksum: u32 = (0 as i32) as u32;
    0 as i32;
    0 as i32;
    0 as i32;
    if !(zSuper != std::ptr::null::<i8>())
        || (((unsafe { (*pPager).journalMode }) as u32) as i32) == (4 as i32)
        || !((unsafe { (*unsafe { (*pPager).jfd }).pMethods })
            != std::ptr::null::<sqlite3_io_methods>())
    {
        return 0 as i32;
    }
    unsafe {
        (*pPager).setSuper = ((1 as i32) as i8) as u8;
    }
    0 as i32;
    // /* Calculate the length in bytes and the checksum of zSuper */
    nSuper = 0 as i32;
    '__slate_break_1515: loop {
        if !((unsafe { *unsafe { zSuper.offset(nSuper as isize) } }) != (0 as i8)) {
            break;
        }
        let __v1688: u32 = cksum;
        let __v1689: u32 = __v1688
            .wrapping_add(((unsafe { *unsafe { zSuper.offset(nSuper as isize) } }) as i32) as u32);
        cksum = __v1689;
        let __v1686: i32 = nSuper;
        let __v1687: i32 = __v1686 + (1 as i32);
        nSuper = __v1687;
    }
    // /* If in full-sync mode, advance to the next disk sector before writing
    //   ** the super-journal name. This is in case the previous page written to
    //   ** the journal has already been synced.
    //   */
    if (unsafe { (*pPager).fullSync }) != (0 as u8) {
        unsafe {
            (*pPager).journalOff = journalHdrOffset(pPager);
        }
    }
    iHdrOff = unsafe { (*pPager).journalOff };
    // /* Write the super-journal data to the end of the journal file. If
    //   ** an error occurs, return the error code to the caller.
    //   */
    let __v1690: i32 = write32bits(unsafe { (*pPager).jfd }, iHdrOff, unsafe {
        (*pPager).lckPgno
    });
    rc = __v1690;
    let __v1691: bool;
    if (0 as i32) != __v1690 {
        __v1691 = true as bool;
    } else {
        let __v1692: i32 = unsafe {
            sqlite3OsWrite(
                unsafe { (*pPager).jfd },
                zSuper as *const (),
                nSuper,
                iHdrOff + ((4 as i32) as i64),
            )
        };
        rc = __v1692;
        __v1691 = (0 as i32) != __v1692;
    }
    let __v1693: bool;
    if __v1691 {
        __v1693 = true as bool;
    } else {
        let __v1694: i32 = write32bits(
            unsafe { (*pPager).jfd },
            iHdrOff + ((4 as i32) as i64) + (nSuper as i64),
            nSuper as u32,
        );
        rc = __v1694;
        __v1693 = (0 as i32) != __v1694;
    }
    let __v1695: bool;
    if __v1693 {
        __v1695 = true as bool;
    } else {
        let __v1696: i32 = write32bits(
            unsafe { (*pPager).jfd },
            iHdrOff + ((4 as i32) as i64) + (nSuper as i64) + ((4 as i32) as i64),
            cksum,
        );
        rc = __v1696;
        __v1695 = (0 as i32) != __v1696;
    }
    let __v1697: bool;
    if __v1695 {
        __v1697 = true as bool;
    } else {
        let __v1698: i32 = unsafe {
            sqlite3OsWrite(
                unsafe { (*pPager).jfd },
                (unsafe { std::ptr::addr_of!(aJournalMagic) as *const u8 }) as *const (),
                8 as i32,
                iHdrOff + ((4 as i32) as i64) + (nSuper as i64) + ((8 as i32) as i64),
            )
        };
        rc = __v1698;
        __v1697 = (0 as i32) != __v1698;
    }
    if __v1697 {
        return rc;
    }
    let __v1699: *mut Pager = pPager;
    let __v1700: i64 = unsafe { (*__v1699).journalOff };
    let __v1701: i64 = __v1700 + ((nSuper + (20 as i32)) as i64);
    unsafe {
        (*__v1699).journalOff = __v1701;
    }
    // /* If the pager is in persistent-journal mode, then the physical
    //   ** journal-file may extend past the end of the super-journal name
    //   ** and 8 bytes of magic data just written to the file. This is
    //   ** dangerous because the code to rollback a hot-journal file
    //   ** will not be able to find the super-journal name to determine
    //   ** whether or not the journal is hot.
    //   **
    //   ** Easiest thing to do in this scenario is to truncate the journal
    //   ** file to the required size.
    //   */
    let __v1702: i32 =
        unsafe { sqlite3OsFileSize(unsafe { (*pPager).jfd }, std::ptr::addr_of_mut!(jrnlSize)) };
    rc = __v1702;
    if (0 as i32) == __v1702 && jrnlSize > unsafe { (*pPager).journalOff } {
        rc =
            unsafe { sqlite3OsTruncate(unsafe { (*pPager).jfd }, unsafe { (*pPager).journalOff }) };
    }
    return rc;
}

// /*
// ** Discard the entire contents of the in-memory page-cache.
// */
fn pager_reset(mut pPager: *mut Pager) {
    let __v1703: *mut Pager = pPager;
    let __v1704: u32 = unsafe { (*__v1703).iDataVersion };
    let __v1705: u32 = __v1704.wrapping_add((1 as i32) as u32);
    unsafe {
        (*__v1703).iDataVersion = __v1705;
    }
    unsafe { sqlite3BackupRestart(unsafe { (*pPager).pBackup }) };
    unsafe { sqlite3PcacheClear(unsafe { (*pPager).pPCache }) };
}

// /*
// ** Free all structures in the Pager.aSavepoint[] array and set both
// ** Pager.aSavepoint and Pager.nSavepoint to zero. Close the sub-journal
// ** if it is open and the pager is not in exclusive mode.
// */
fn releaseAllSavepoints(mut pPager: *mut Pager) {
    // /* Iterator for looping through Pager.aSavepoint */
    let mut ii: i32 = 0 as i32;
    ii = 0 as i32;
    '__slate_break_1516: loop {
        if !(ii < unsafe { (*pPager).nSavepoint }) {
            break;
        }
        unsafe {
            sqlite3BitvecDestroy(unsafe {
                (*unsafe { unsafe { (*pPager).aSavepoint }.offset(ii as isize) }).pInSavepoint
            })
        };
        let __v1706: i32 = ii;
        let __v1707: i32 = __v1706 + (1 as i32);
        ii = __v1707;
    }
    let __v1708: bool;
    if !((unsafe { (*pPager).exclusiveMode }) != (0 as u8)) {
        __v1708 = true as bool;
    } else {
        __v1708 = (unsafe { sqlite3JournalIsInMemory(unsafe { (*pPager).sjfd }) }) != (0 as i32);
    }
    if __v1708 {
        unsafe { sqlite3OsClose(unsafe { (*pPager).sjfd }) };
    }
    unsafe { sqlite3_free((unsafe { (*pPager).aSavepoint }) as *mut ()) };
    unsafe {
        (*pPager).aSavepoint = std::ptr::null_mut::<PagerSavepoint>();
    }
    unsafe {
        (*pPager).nSavepoint = 0 as i32;
    }
    unsafe {
        (*pPager).nSubRec = (0 as i32) as u32;
    }
}

// /*
// ** Set the bit number pgno in the PagerSavepoint.pInSavepoint
// ** bitvecs of all open savepoints. Return SQLITE_OK if successful
// ** or SQLITE_NOMEM if a malloc failure occurs.
// */
fn addToSavepointBitvecs(mut pPager: *mut Pager, mut pgno: u32) -> i32 {
    // /* Loop counter */
    let mut ii: i32 = 0 as i32;
    // /* Result code */
    let mut rc: i32 = 0 as i32;
    ii = 0 as i32;
    '__slate_break_1517: loop {
        if !(ii < unsafe { (*pPager).nSavepoint }) {
            break;
        }
        let mut p: *mut PagerSavepoint =
            unsafe { unsafe { (*pPager).aSavepoint }.offset(ii as isize) };
        if pgno <= unsafe { (*p).nOrig } {
            let __v1711: i32 = rc;
            let __v1712: i32 =
                __v1711 | unsafe { sqlite3BitvecSet(unsafe { (*p).pInSavepoint }, pgno) };
            rc = __v1712;
            {}
            0 as i32;
        }
        let __v1709: i32 = ii;
        let __v1710: i32 = __v1709 + (1 as i32);
        ii = __v1710;
    }
    return rc;
}

// /*
// ** This function is a no-op if the pager is in exclusive mode and not
// ** in the ERROR state. Otherwise, it switches the pager to PAGER_OPEN
// ** state.
// **
// ** If the pager is not in exclusive-access mode, the database file is
// ** completely unlocked. If the file is unlocked and the file-system does
// ** not exhibit the UNDELETABLE_WHEN_OPEN property, the journal file is
// ** closed (if it is open).
// **
// ** If the pager is in ERROR state when this function is called, the
// ** contents of the pager cache are discarded before switching back to
// ** the OPEN state. Regardless of whether the pager is in exclusive-mode
// ** or not, any journal file left in the file-system will be treated
// ** as a hot-journal and rolled back the next time a read-transaction
// ** is opened (by this or by any other connection).
// */
fn pager_unlock(mut pPager: *mut Pager) {
    0 as i32;
    unsafe { sqlite3BitvecDestroy(unsafe { (*pPager).pInJournal }) };
    unsafe {
        (*pPager).pInJournal = std::ptr::null_mut::<Bitvec>();
    }
    releaseAllSavepoints(pPager);
    if (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
        0 as i32;
        if (((unsafe { (*pPager).eState }) as u32) as i32) == (6 as i32) {
            // /* If an IO error occurs in wal.c while attempting to wrap the wal file,
            //       ** then the Wal object may be holding a write-lock but no read-lock.
            //       ** This call ensures that the write-lock is dropped as well. We cannot
            //       ** have sqlite3WalEndReadTransaction() drop the write-lock, as it once
            //       ** did, because this would break "BEGIN EXCLUSIVE" handling for
            //       ** SQLITE_ENABLE_SETLK_TIMEOUT builds.  */
            unsafe { sqlite3WalEndWriteTransaction(unsafe { (*pPager).pWal }) };
        }
        unsafe { sqlite3WalEndReadTransaction(unsafe { (*pPager).pWal }) };
        unsafe {
            (*pPager).eState = ((0 as i32) as i8) as u8;
        }
    } else {
        if !((unsafe { (*pPager).exclusiveMode }) != (0 as u8)) {
            // /* Error code returned by pagerUnlockDb() */
            let mut rc: i32 = 0 as i32;
            let mut iDc: i32 = 0 as i32;
            let __v1713: i32;
            if (unsafe { (*unsafe { (*pPager).fd }).pMethods })
                != std::ptr::null::<sqlite3_io_methods>()
            {
                __v1713 = unsafe { sqlite3OsDeviceCharacteristics(unsafe { (*pPager).fd }) };
            } else {
                __v1713 = 0 as i32;
            }
            iDc = __v1713;
            // /* If the operating system support deletion of open files, then
            //     ** close the journal file when dropping the database lock.  Otherwise
            //     ** another connection with journal_mode=delete might delete the file
            //     ** out from under us.
            //     */
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            if (0 as i32) == iDc & (2048 as i32)
                || (1 as i32) != (((unsafe { (*pPager).journalMode }) as u32) as i32) & (5 as i32)
            {
                unsafe { sqlite3OsClose(unsafe { (*pPager).jfd }) };
            }
            // /* If the pager is in the ERROR state and the call to unlock the database
            //     ** file fails, set the current lock to UNKNOWN_LOCK. See the comment
            //     ** above the #define for UNKNOWN_LOCK for an explanation of why this
            //     ** is necessary.
            //     */
            rc = pagerUnlockDb(pPager, 0 as i32);
            if rc != (0 as i32) && (((unsafe { (*pPager).eState }) as u32) as i32) == (6 as i32) {
                unsafe {
                    (*pPager).eLock = (((4 as i32) + (1 as i32)) as i8) as u8;
                }
            }
            // /* The pager state may be changed from PAGER_ERROR to PAGER_OPEN here
            //     ** without clearing the error code. This is intentional - the error
            //     ** code is cleared and the cache reset in the block below.
            //     */
            0 as i32;
            unsafe {
                (*pPager).eState = ((0 as i32) as i8) as u8;
            }
        }
    }
    // /* If Pager.errCode is set, the contents of the pager cache cannot be
    //   ** trusted. Now that there are no outstanding references to the pager,
    //   ** it can safely move back to PAGER_OPEN state. This happens in both
    //   ** normal and exclusive-locking mode.
    //   */
    0 as i32;
    if (unsafe { (*pPager).errCode }) != (0 as i32) {
        if (((unsafe { (*pPager).tempFile }) as u32) as i32) == (0 as i32) {
            pager_reset(pPager);
            unsafe {
                (*pPager).changeCountDone = ((0 as i32) as i8) as u8;
            }
            unsafe {
                (*pPager).eState = ((0 as i32) as i8) as u8;
            }
        } else {
            unsafe {
                (*pPager).eState = ((if (unsafe { (*unsafe { (*pPager).jfd }).pMethods })
                    != std::ptr::null::<sqlite3_io_methods>()
                {
                    0 as i32
                } else {
                    1 as i32
                }) as i8) as u8;
            }
        }
        if (unsafe { (*pPager).bUseFetch }) != (0 as u8) {
            unsafe {
                sqlite3OsUnfetch(
                    unsafe { (*pPager).fd },
                    (0 as i32) as i64,
                    std::ptr::null_mut::<()>(),
                )
            };
        }
        unsafe {
            (*pPager).errCode = 0 as i32;
        }
        setGetterMethod(pPager);
    }
    unsafe {
        (*pPager).journalOff = (0 as i32) as i64;
    }
    unsafe {
        (*pPager).journalHdr = (0 as i32) as i64;
    }
    unsafe {
        (*pPager).setSuper = ((0 as i32) as i8) as u8;
    }
}

// /*
// ** This function is called whenever an IOERR or FULL error that requires
// ** the pager to transition into the ERROR state may have occurred.
// ** The first argument is a pointer to the pager structure, the second
// ** the error-code about to be returned by a pager API function. The
// ** value returned is a copy of the second argument to this function.
// **
// ** If the second argument is SQLITE_FULL, SQLITE_IOERR or one of the
// ** IOERR sub-codes, the pager enters the ERROR state and the error code
// ** is stored in Pager.errCode. While the pager remains in the ERROR state,
// ** all major API calls on the Pager will immediately return Pager.errCode.
// **
// ** The ERROR state indicates that the contents of the pager-cache
// ** cannot be trusted. This state can be cleared by completely discarding
// ** the contents of the pager-cache. If a transaction was active when
// ** the persistent error occurred, then the rollback journal may need
// ** to be replayed to restore the contents of the database file (as if
// ** it were a hot-journal).
// */
fn pager_error(mut pPager: *mut Pager, mut rc: i32) -> i32 {
    let mut rc2: i32 = rc & (255 as i32);
    0 as i32;
    0 as i32;
    if rc2 == (13 as i32) || rc2 == (10 as i32) {
        unsafe {
            (*pPager).errCode = rc;
        }
        unsafe {
            (*pPager).eState = ((6 as i32) as i8) as u8;
        }
        setGetterMethod(pPager);
    }
    return rc;
}

// /*
// ** This function is used to change the actual size of the database
// ** file in the file-system. This only happens when committing a transaction,
// ** or rolling back a transaction (including rolling back a hot-journal).
// **
// ** If the main database file is not open, or the pager is not in either
// ** DBMOD or OPEN state, this function is a no-op. Otherwise, the size
// ** of the file is changed to nPage pages (nPage*pPager->pageSize bytes).
// ** If the file on disk is currently larger than nPage pages, then use the VFS
// ** xTruncate() method to truncate it.
// **
// ** Or, it might be the case that the file on disk is smaller than
// ** nPage pages. Some operating system implementations can get confused if
// ** you try to truncate a file to some size that is larger than it
// ** currently is, so detect this case and write a single zero byte to
// ** the end of the new file instead.
// **
// ** If successful, return SQLITE_OK. If an IO error occurs while modifying
// ** the database file, return the error code to the caller.
// */
fn pager_truncate(mut pPager: *mut Pager, mut nPage: u32) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    {}
    if (unsafe { (*unsafe { (*pPager).fd }).pMethods }) != std::ptr::null::<sqlite3_io_methods>()
        && ((((unsafe { (*pPager).eState }) as u32) as i32) >= (4 as i32)
            || (((unsafe { (*pPager).eState }) as u32) as i32) == (0 as i32))
    {
        let mut currentSize: i64 = 0 as i64;
        let mut newSize: i64 = 0 as i64;
        let mut szPage: i32 = (unsafe { (*pPager).pageSize }) as i32;
        0 as i32;
        // /* TODO: Is it safe to use Pager.dbFileSize here? */
        rc = unsafe {
            sqlite3OsFileSize(unsafe { (*pPager).fd }, std::ptr::addr_of_mut!(currentSize))
        };
        newSize = (szPage as i64) * ((nPage as u64) as i64);
        if rc == (0 as i32) && currentSize != newSize {
            if currentSize > newSize {
                rc = unsafe { sqlite3OsTruncate(unsafe { (*pPager).fd }, newSize) };
            } else {
                if currentSize + (szPage as i64) <= newSize {
                    let mut pTmp: *mut i8 = unsafe { (*pPager).pTmpSpace };
                    unsafe { memset(pTmp as *mut (), 0 as i32, (szPage as i64) as u64) };
                    {}
                    {}
                    unsafe {
                        sqlite3OsFileControlHint(
                            unsafe { (*pPager).fd },
                            5 as i32,
                            std::ptr::addr_of_mut!(newSize) as *mut (),
                        )
                    };
                    rc = unsafe {
                        sqlite3OsWrite(
                            unsafe { (*pPager).fd },
                            pTmp as *const (),
                            szPage,
                            newSize - (szPage as i64),
                        )
                    };
                }
            }
            if rc == (0 as i32) {
                unsafe {
                    (*pPager).dbFileSize = nPage;
                }
            }
        }
    }
    return rc;
}

// /*
// ** The write transaction open on pPager is being committed (bCommit==1)
// ** or rolled back (bCommit==0).
// **
// ** Return TRUE if and only if all dirty pages should be flushed to disk.
// **
// ** Rules:
// **
// **   *  For non-TEMP databases, always sync to disk.  This is necessary
// **      for transactions to be durable.
// **
// **   *  Sync TEMP database only on a COMMIT (not a ROLLBACK) when the backing
// **      file has been created already (via a spill on pagerStress()) and
// **      when the number of dirty pages in memory exceeds 25% of the total
// **      cache size.
// */
fn pagerFlushOnCommit(mut pPager: *mut Pager, mut bCommit: i32) -> i32 {
    if (((unsafe { (*pPager).tempFile }) as u32) as i32) == (0 as i32) {
        return 1 as i32;
    }
    if !(bCommit != (0 as i32)) {
        return 0 as i32;
    }
    if !((unsafe { (*unsafe { (*pPager).fd }).pMethods }) != std::ptr::null::<sqlite3_io_methods>())
    {
        return 0 as i32;
    }
    return ((unsafe { sqlite3PCachePercentDirty(unsafe { (*pPager).pPCache }) }) >= (25 as i32))
        as i32;
}

// /*
// ** This routine ends a transaction. A transaction is usually ended by
// ** either a COMMIT or a ROLLBACK operation. This routine may be called
// ** after rollback of a hot-journal, or if an error occurs while opening
// ** the journal file or writing the very first journal-header of a
// ** database transaction.
// **
// ** This routine is never called in PAGER_ERROR state. If it is called
// ** in PAGER_OPEN or PAGER_READER state and the lock held is less
// ** exclusive than a RESERVED lock, it is a no-op.
// **
// ** Otherwise, any active savepoints are released.
// **
// ** If the journal file is open, then it is "finalized". Once a journal
// ** file has been finalized it is not possible to use it to roll back a
// ** transaction. Nor will it be considered to be a hot-journal by this
// ** or any other database connection. Exactly how a journal is finalized
// ** depends on whether or not the pager is running in exclusive mode and
// ** the current journal-mode (Pager.journalMode value), as follows:
// **
// **   journalMode==MEMORY
// **     Journal file descriptor is simply closed. This destroys an
// **     in-memory journal.
// **
// **   journalMode==TRUNCATE
// **     Journal file is truncated to zero bytes in size.
// **
// **   journalMode==PERSIST
// **     The first 28 bytes of the journal file are zeroed. This invalidates
// **     the first journal header in the file, and hence the entire journal
// **     file. An invalid journal file cannot be rolled back.
// **
// **   journalMode==DELETE
// **     The journal file is closed and deleted using sqlite3OsDelete().
// **
// **     If the pager is running in exclusive mode, this method of finalizing
// **     the journal file is never used. Instead, if the journalMode is
// **     DELETE and the pager is in exclusive mode, the method described under
// **     journalMode==PERSIST is used instead.
// **
// ** After the journal is finalized, the pager moves to PAGER_READER state.
// ** If running in non-exclusive rollback mode, the lock on the file is
// ** downgraded to a SHARED_LOCK.
// **
// ** SQLITE_OK is returned if no error occurs. If an error occurs during
// ** any of the IO operations to finalize the journal file or unlock the
// ** database then the IO error code is returned to the user. If the
// ** operation to finalize the journal file fails, then the code still
// ** tries to unlock the database file if not in exclusive mode. If the
// ** unlock operation fails as well, then the first error code related
// ** to the first error encountered (the journal finalization one) is
// ** returned.
// */
fn pager_end_transaction(mut pPager: *mut Pager, mut hasSuper: i32, mut bCommit: i32) -> i32 {
    // /* Error code from journal finalization operation */
    let mut rc: i32 = 0 as i32;
    // /* Error code from db file unlock operation */
    let mut rc2: i32 = 0 as i32;
    // /* Do nothing if the pager does not have an open write transaction
    //   ** or at least a RESERVED lock. This function may be called when there
    //   ** is no write-transaction active but a RESERVED or greater lock is
    //   ** held under two circumstances:
    //   **
    //   **   1. After a successful hot-journal rollback, it is called with
    //   **      eState==PAGER_NONE and eLock==EXCLUSIVE_LOCK.
    //   **
    //   **   2. If a connection with locking_mode=exclusive holding an EXCLUSIVE
    //   **      lock switches back to locking_mode=normal and then executes a
    //   **      read-transaction, this function is called with eState==PAGER_READER
    //   **      and eLock==EXCLUSIVE_LOCK when the read-transaction is closed.
    //   */
    0 as i32;
    0 as i32;
    if (((unsafe { (*pPager).eState }) as u32) as i32) < (2 as i32)
        && (((unsafe { (*pPager).eLock }) as u32) as i32) < (2 as i32)
    {
        return 0 as i32;
    }
    releaseAllSavepoints(pPager);
    0 as i32;
    if (unsafe { (*unsafe { (*pPager).jfd }).pMethods }) != std::ptr::null::<sqlite3_io_methods>() {
        0 as i32;
        // /* Finalize the journal file. */
        if (unsafe { sqlite3JournalIsInMemory(unsafe { (*pPager).jfd }) }) != (0 as i32) {
            // /* assert( pPager->journalMode==PAGER_JOURNALMODE_MEMORY ); */
            unsafe { sqlite3OsClose(unsafe { (*pPager).jfd }) };
        } else {
            if (((unsafe { (*pPager).journalMode }) as u32) as i32) == (3 as i32) {
                if (unsafe { (*pPager).journalOff }) == ((0 as i32) as i64) {
                    rc = 0 as i32;
                } else {
                    rc = unsafe { sqlite3OsTruncate(unsafe { (*pPager).jfd }, (0 as i32) as i64) };
                    if rc == (0 as i32) && (unsafe { (*pPager).fullSync }) != (0 as u8) {
                        // /* Make sure the new file size is written into the inode right away.
                        //           ** Otherwise the journal might resurrect following a power loss and
                        //           ** cause the last transaction to roll back.  See
                        //           ** https://bugzilla.mozilla.org/show_bug.cgi?id=1072773
                        //           */
                        rc = unsafe {
                            sqlite3OsSync(
                                unsafe { (*pPager).jfd },
                                ((unsafe { (*pPager).syncFlags }) as u32) as i32,
                            )
                        };
                    }
                }
                unsafe {
                    (*pPager).journalOff = (0 as i32) as i64;
                }
            } else {
                if (((unsafe { (*pPager).journalMode }) as u32) as i32) == (1 as i32)
                    || (unsafe { (*pPager).exclusiveMode }) != (0 as u8)
                        && (((unsafe { (*pPager).journalMode }) as u32) as i32) < (5 as i32)
                {
                    rc = zeroJournalHdr(
                        pPager,
                        (hasSuper != (0 as i32) || (unsafe { (*pPager).tempFile }) != (0 as u8))
                            as i32,
                    );
                    unsafe {
                        (*pPager).journalOff = (0 as i32) as i64;
                    }
                } else {
                    // /* This branch may be executed with Pager.journalMode==MEMORY if
                    //       ** a hot-journal was just rolled back. In this case the journal
                    //       ** file should be closed and deleted. If this connection writes to
                    //       ** the database file, it will do so using an in-memory journal.
                    //       */
                    let mut bDelete: i32 = !((unsafe { (*pPager).tempFile }) != (0 as u8)) as i32;
                    0 as i32;
                    0 as i32;
                    unsafe { sqlite3OsClose(unsafe { (*pPager).jfd }) };
                    if bDelete != (0 as i32) {
                        rc = unsafe {
                            sqlite3OsDelete(
                                unsafe { (*pPager).pVfs },
                                (unsafe { (*pPager).zJournal }) as *const i8,
                                ((unsafe { (*pPager).extraSync }) as u32) as i32,
                            )
                        };
                    }
                }
            }
        }
    }
    unsafe { sqlite3BitvecDestroy(unsafe { (*pPager).pInJournal }) };
    unsafe {
        (*pPager).pInJournal = std::ptr::null_mut::<Bitvec>();
    }
    unsafe {
        (*pPager).nRec = 0 as i32;
    }
    if rc == (0 as i32) {
        let __v1714: bool;
        if (unsafe { (*pPager).memDb }) != (0 as u8) {
            __v1714 = true as bool;
        } else {
            __v1714 = pagerFlushOnCommit(pPager, bCommit) != (0 as i32);
        }
        if __v1714 {
            unsafe { sqlite3PcacheCleanAll(unsafe { (*pPager).pPCache }) };
        } else {
            unsafe { sqlite3PcacheClearWritable(unsafe { (*pPager).pPCache }) };
        }
        unsafe { sqlite3PcacheTruncate(unsafe { (*pPager).pPCache }, unsafe { (*pPager).dbSize }) };
    }
    if (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
        // /* Drop the WAL write-lock, if any. Also, if the connection was in
        //     ** locking_mode=exclusive mode but is no longer, drop the EXCLUSIVE
        //     ** lock held on the database file.
        //     */
        rc2 = unsafe { sqlite3WalEndWriteTransaction(unsafe { (*pPager).pWal }) };
        0 as i32;
    } else {
        if rc == (0 as i32)
            && bCommit != (0 as i32)
            && (unsafe { (*pPager).dbFileSize }) > unsafe { (*pPager).dbSize }
        {
            // /* This branch is taken when committing a transaction in rollback-journal
            //     ** mode if the database file on disk is larger than the database image.
            //     ** At this point the journal has been finalized and the transaction
            //     ** successfully committed, but the EXCLUSIVE lock is still held on the
            //     ** file. So it is safe to truncate the database file to its minimum
            //     ** required size.  */
            0 as i32;
            rc = pager_truncate(pPager, unsafe { (*pPager).dbSize });
        }
    }
    if rc == (0 as i32) && bCommit != (0 as i32) {
        rc = unsafe {
            sqlite3OsFileControl(
                unsafe { (*pPager).fd },
                22 as i32,
                std::ptr::null_mut::<()>(),
            )
        };
        if rc == (12 as i32) {
            rc = 0 as i32;
        }
    }
    let __v1715: bool;
    if !((unsafe { (*pPager).exclusiveMode }) != (0 as u8)) {
        let __v1716: bool;
        if !((unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>()) {
            __v1716 = true as bool;
        } else {
            __v1716 = (unsafe { sqlite3WalExclusiveMode(unsafe { (*pPager).pWal }, 0 as i32) })
                != (0 as i32);
        }
        __v1715 = __v1716;
    } else {
        __v1715 = false as bool;
    }
    if __v1715 {
        rc2 = pagerUnlockDb(pPager, 1 as i32);
    }
    unsafe {
        (*pPager).eState = ((1 as i32) as i8) as u8;
    }
    unsafe {
        (*pPager).setSuper = ((0 as i32) as i8) as u8;
    }
    return if rc == (0 as i32) { rc2 } else { rc };
}

// /*
// ** Playback the journal and thus restore the database file to
// ** the state it was in before we started making changes.
// **
// ** The journal file format is as follows:
// **
// **  (1)  8 byte prefix.  A copy of aJournalMagic[].
// **  (2)  4 byte big-endian integer which is the number of valid page records
// **       in the journal.  If this value is 0xffffffff, then compute the
// **       number of page records from the journal size.
// **  (3)  4 byte big-endian integer which is the initial value for the
// **       sanity checksum.
// **  (4)  4 byte integer which is the number of pages to truncate the
// **       database to during a rollback.
// **  (5)  4 byte big-endian integer which is the sector size.  The header
// **       is this many bytes in size.
// **  (6)  4 byte big-endian integer which is the page size.
// **  (7)  zero padding out to the next sector size.
// **  (8)  Zero or more pages instances, each as follows:
// **        +  4 byte page number.
// **        +  pPager->pageSize bytes of data.
// **        +  4 byte checksum
// **
// ** When we speak of the journal header, we mean the first 7 items above.
// ** Each entry in the journal is an instance of the 8th item.
// **
// ** Call the value from the second bullet "nRec".  nRec is the number of
// ** valid page entries in the journal.  In most cases, you can compute the
// ** value of nRec from the size of the journal file.  But if a power
// ** failure occurred while the journal was being written, it could be the
// ** case that the size of the journal file had already been increased but
// ** the extra entries had not yet made it safely to disk.  In such a case,
// ** the value of nRec computed from the file size would be too large.  For
// ** that reason, we always use the nRec value in the header.
// **
// ** If the nRec value is 0xffffffff it means that nRec should be computed
// ** from the file size.  This value is used when the user selects the
// ** no-sync option for the journal.  A power failure could lead to corruption
// ** in this case.  But for things like temporary table (which will be
// ** deleted when the power is restored) we don't care.
// **
// ** If the file opened as the journal file is not a well-formed
// ** journal file then all pages up to the first corrupted page are rolled
// ** back (or no pages if the journal header is corrupted). The journal file
// ** is then deleted and SQLITE_OK returned, just as if no corruption had
// ** been encountered.
// **
// ** If an I/O or malloc() error occurs, the journal-file is not deleted
// ** and an error code is returned.
// **
// ** The isHot parameter indicates that we are trying to rollback a journal
// ** that might be a hot journal.  Or, it could be that the journal is
// ** preserved because of JOURNALMODE_PERSIST or JOURNALMODE_TRUNCATE.
// ** If the journal really is hot, reset the pager cache prior rolling
// ** back any content.  If the journal is merely persistent, no reset is
// ** needed.
// */
fn pager_playback(mut pPager: *mut Pager, mut isHot: i32) -> i32 {
    let mut __slate_storage_1718: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1718: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1718) as *mut u32;
    let mut __slate_storage_1717: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1717: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1717) as *mut u32;
    let mut __slate_storage_1720: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1720: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1720) as *mut i32;
    let mut __slate_storage_1719: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1719: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1719) as *mut i32;
    let mut __slate_storage_764: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_764: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_764) as *mut u32;
    let mut __slate_storage_763: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_763: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_763) as *mut i32;
    let mut __slate_storage_762: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_762: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_762) as *mut i32;
    let mut __slate_storage_761: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_761: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_761) as *mut *mut i8;
    let mut __slate_storage_760: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_760: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_760) as *mut i32;
    let mut __slate_storage_759: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_759: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_759) as *mut i32;
    let mut __slate_storage_758: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_758: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_758) as *mut u32;
    let mut __slate_storage_757: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_757: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_757) as *mut u32;
    let mut __slate_storage_756: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_756: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_756) as *mut u32;
    let mut __slate_storage_755: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_755: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_755) as *mut i64;
    let mut __slate_storage_754: std::mem::MaybeUninit<*mut sqlite3_vfs> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_754: *mut *mut sqlite3_vfs =
        std::ptr::addr_of_mut!(__slate_storage_754) as *mut *mut sqlite3_vfs;
    unsafe {
        '__join_10: {
            std::ptr::write(__slate_slot_754, unsafe { (*pPager).pVfs });
            // /* Size of the journal file in bytes */
            // /* Number of Records in the journal */
            // /* Unsigned loop counter */
            // /* Size of the original file in pages */
            std::ptr::write(__slate_slot_758, (0 as i32) as u32);
            // /* Result code of a subroutine */
            // /* Value returned by sqlite3OsAccess() */
            std::ptr::write(__slate_slot_760, 1 as i32);
            // /* Name of super-journal file if any */
            std::ptr::write(__slate_slot_761, std::ptr::null_mut::<i8>());
            // /* True to reset page prior to first page rollback */
            // /* Total number of pages restored from journal */
            std::ptr::write(__slate_slot_763, 0 as i32);
            std::ptr::write(
                __slate_slot_764,
                ((unsafe { (*pPager).pageSize }) as i32) as u32,
            );
            // /* Figure out how many records are in the journal.  Abort early if
            //   ** the journal is empty.
            //   */
            0 as i32;
            *__slate_slot_759 = unsafe {
                sqlite3OsFileSize(
                    unsafe { (*pPager).jfd },
                    std::ptr::addr_of_mut!(*__slate_slot_755),
                )
            };
            if *__slate_slot_759 != (0 as i32) {
            } else {
                // /* Read the super-journal name from the journal, if it is present.
                //   ** If a super-journal file name is specified, but the file is not
                //   ** present on disk, then the journal is not hot and does not need to be
                //   ** played back.
                //   */
                *__slate_slot_759 = readSuperJournal(
                    unsafe { (*pPager).jfd },
                    (((1 as i32) as i64)
                        + ((unsafe { (*unsafe { (*pPager).pVfs }).mxPathname }) as i64))
                        as u64,
                    std::ptr::addr_of_mut!(*__slate_slot_761),
                );
                if *__slate_slot_759 == (0 as i32)
                    && *__slate_slot_761 != std::ptr::null_mut::<i8>()
                {
                    *__slate_slot_759 = unsafe {
                        sqlite3OsAccess(
                            *__slate_slot_754,
                            *__slate_slot_761 as *const i8,
                            0 as i32,
                            std::ptr::addr_of_mut!(*__slate_slot_760),
                        )
                    };
                }
                if *__slate_slot_759 != (0 as i32) || !(*__slate_slot_760 != (0 as i32)) {
                } else {
                    unsafe {
                        (*pPager).journalOff = (0 as i32) as i64;
                    }
                    *__slate_slot_762 = isHot;
                    // /* This loop terminates either when a readJournalHdr() or
                    //   ** pager_playback_one_page() call returns SQLITE_DONE or an IO error
                    //   ** occurs.
                    //   */
                    '__join_33: {
                        '__join_18: {
                            '__loop_12: loop {
                                if (1 as i32) != (0 as i32) {
                                    // /* Read the next journal header from the journal file.  If there are
                                    //     ** not enough bytes left in the journal file for a complete header, or
                                    //     ** it is corrupted, then a process must have failed while writing it.
                                    //     ** This indicates nothing more needs to be rolled back.
                                    //     */
                                    *__slate_slot_759 = readJournalHdr(
                                        pPager,
                                        isHot,
                                        *__slate_slot_755,
                                        std::ptr::addr_of_mut!(*__slate_slot_756),
                                        std::ptr::addr_of_mut!(*__slate_slot_758),
                                    );
                                    if *__slate_slot_759 != (0 as i32) {
                                        break '__join_33;
                                    } else {
                                        // /* If nRec is 0xffffffff, then this journal was created by a process
                                        //     ** working in no-sync mode. This means that the rest of the journal
                                        //     ** file consists of pages, there are no more journal headers. Compute
                                        //     ** the value of nRec based on this assumption.
                                        //     */
                                        if *__slate_slot_756 == (4294967295 as u32) {
                                            0 as i32;
                                            *__slate_slot_756 = (((*__slate_slot_755
                                                - (((unsafe { (*pPager).sectorSize }) as u64)
                                                    as i64))
                                                / ((unsafe { (*pPager).pageSize })
                                                    + ((8 as i32) as i64)))
                                                as i32)
                                                as u32;
                                        }
                                        // /* If nRec is 0 and this rollback is of a transaction created by this
                                        //     ** process and if this is the final header in the journal, then it means
                                        //     ** that this part of the journal was being filled but has not yet been
                                        //     ** synced to disk.  Compute the number of pages based on the remaining
                                        //     ** size of the file.
                                        //     **
                                        //     ** The third term of the test was added to fix ticket #2565.
                                        //     ** When rolling back a hot journal, nRec==0 always means that the next
                                        //     ** chunk of the journal contains zero pages to be rolled back.  But
                                        //     ** when doing a ROLLBACK and the nRec==0 chunk is the last chunk in
                                        //     ** the journal, it means that the journal might contain additional
                                        //     ** pages that need to be rolled back and that the number of pages
                                        //     ** should be computed based on the journal file size.
                                        //     */
                                        if *__slate_slot_756 == ((0 as i32) as u32)
                                            && !(isHot != (0 as i32))
                                            && (unsafe { (*pPager).journalHdr })
                                                + (((unsafe { (*pPager).sectorSize }) as u64)
                                                    as i64)
                                                == unsafe { (*pPager).journalOff }
                                        {
                                            *__slate_slot_756 = (((*__slate_slot_755
                                                - unsafe { (*pPager).journalOff })
                                                / ((unsafe { (*pPager).pageSize })
                                                    + ((8 as i32) as i64)))
                                                as i32)
                                                as u32;
                                        }
                                        // /* If this is the first header read from the journal, truncate the
                                        //     ** database file back to its original size.
                                        //     */
                                        if (unsafe { (*pPager).journalOff })
                                            == (((unsafe { (*pPager).sectorSize }) as u64) as i64)
                                        {
                                            *__slate_slot_759 =
                                                pager_truncate(pPager, *__slate_slot_758);
                                            if *__slate_slot_759 != (0 as i32) {
                                                break '__join_10;
                                            } else {
                                                unsafe {
                                                    (*pPager).dbSize = *__slate_slot_758;
                                                }
                                                if (unsafe { (*pPager).mxPgno }) < *__slate_slot_758
                                                {
                                                    unsafe {
                                                        (*pPager).mxPgno = *__slate_slot_758;
                                                    }
                                                }
                                            }
                                        }
                                        // /* Copy original pages out of the journal and back into the
                                        //     ** database file and/or page cache.
                                        //     */
                                        *__slate_slot_757 = (0 as i32) as u32;
                                        loop {
                                            if *__slate_slot_757 < *__slate_slot_756 {
                                                if *__slate_slot_762 != (0 as i32) {
                                                    pager_reset(pPager);
                                                    *__slate_slot_762 = 0 as i32;
                                                }
                                                *__slate_slot_759 = pager_playback_one_page(
                                                    pPager,
                                                    unsafe {
                                                        std::ptr::addr_of_mut!((*pPager).journalOff)
                                                    },
                                                    std::ptr::null_mut::<Bitvec>(),
                                                    1 as i32,
                                                    0 as i32,
                                                );
                                                if *__slate_slot_759 == (0 as i32) {
                                                    std::ptr::write(
                                                        __slate_slot_1719,
                                                        *__slate_slot_763,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1720,
                                                        *__slate_slot_1719 + (1 as i32),
                                                    );
                                                    *__slate_slot_763 = *__slate_slot_1720;
                                                    std::ptr::write(
                                                        __slate_slot_1717,
                                                        *__slate_slot_757,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1718,
                                                        (*__slate_slot_1717)
                                                            .wrapping_add((1 as i32) as u32),
                                                    );
                                                    *__slate_slot_757 = *__slate_slot_1718;
                                                } else {
                                                    break;
                                                }
                                            } else {
                                                continue '__loop_12;
                                            }
                                        }
                                        if *__slate_slot_759 == (101 as i32) {
                                            unsafe {
                                                (*pPager).journalOff = *__slate_slot_755;
                                            }
                                        } else {
                                            break '__join_18;
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                            // /*NOTREACHED*/
                            0 as i32;
                            break '__join_10;
                        }
                        if *__slate_slot_759 == (10 as i32) | (2 as i32) << (8 as i32) {
                            // /* If the journal has been truncated, simply stop reading and
                            //           ** processing the journal. This might happen if the journal was
                            //           ** not completely written and synced prior to a crash.  In that
                            //           ** case, the database should have never been written in the
                            //           ** first place so it is OK to simply abandon the rollback. */
                            *__slate_slot_759 = 0 as i32;
                            break '__join_10;
                        } else {
                            // /* If we are unable to rollback, quit and return the error
                            //           ** code.  This will cause the pager to enter the error state
                            //           ** so that no further harm will be done.  Perhaps the next
                            //           ** process to come along will be able to rollback the database.
                            //           */
                            break '__join_10;
                        }
                    }
                    if *__slate_slot_759 == (101 as i32) {
                        *__slate_slot_759 = 0 as i32;
                    }
                }
            }
        }
        '__join_8: {
            if *__slate_slot_759 == (0 as i32) {
                *__slate_slot_759 = sqlite3PagerSetPagesize(
                    pPager,
                    std::ptr::addr_of_mut!(*__slate_slot_764),
                    -(1 as i32),
                );
            }
        }
        // /* Following a rollback, the database file should be back in its original
        //   ** state prior to the start of the transaction, so invoke the
        //   ** SQLITE_FCNTL_DB_UNCHANGED file-control method to disable the
        //   ** assertion that the transaction counter was modified.
        //   */
        // /* If this playback is happening automatically as a result of an IO or
        //   ** malloc error that occurred after the change-counter was updated but
        //   ** before the transaction was committed, then the change-counter
        //   ** modification may just have been reverted. If this happens in exclusive
        //   ** mode, then subsequent transactions performed by the connection will not
        //   ** update the change-counter at all. This may lead to cache inconsistency
        //   ** problems for other processes at some point in the future. So, just
        //   ** in case this has happened, clear the changeCountDone flag now.
        //   */
        unsafe {
            (*pPager).changeCountDone = unsafe { (*pPager).tempFile };
        }
        if *__slate_slot_759 == (0 as i32)
            && ((((unsafe { (*pPager).eState }) as u32) as i32) >= (4 as i32)
                || (((unsafe { (*pPager).eState }) as u32) as i32) == (0 as i32))
        {
            *__slate_slot_759 = sqlite3PagerSync(pPager, std::ptr::null::<i8>());
        }
        if *__slate_slot_759 == (0 as i32) {
            *__slate_slot_759 = pager_end_transaction(
                pPager,
                (*__slate_slot_761 != std::ptr::null_mut::<i8>()) as i32,
                0 as i32,
            );
            {}
        }
        if *__slate_slot_759 == (0 as i32)
            && *__slate_slot_761 != std::ptr::null_mut::<i8>()
            && *__slate_slot_760 != (0 as i32)
        {
            // /* If there was a super-journal and this routine will return success,
            //     ** see if it is possible to delete the super-journal.
            //     */
            0 as i32;
            *__slate_slot_759 = pager_delsuper(pPager, *__slate_slot_761 as *const i8);
            {}
        }
        if isHot != (0 as i32) && *__slate_slot_763 != (0 as i32) {
            unsafe {
                sqlite3_log(
                    (27 as i32) | (2 as i32) << (8 as i32),
                    (b"recovered %d pages from %s\0".as_ptr() as *mut i8) as *const i8,
                    *__slate_slot_763,
                    unsafe { (*pPager).zJournal },
                )
            };
        }
        // /* The Pager.sectorSize variable may have been updated while rolling
        //   ** back a journal created by a process with a different sector size
        //   ** value. Reset it to the correct value for this process.
        //   */
        freeSuperJournal(*__slate_slot_761);
        setSectorSize(pPager);
        return *__slate_slot_759;
    }
    return unsafe { std::mem::zeroed() };
}

// /* Forward reference */
// /*
// ** Execute a rollback if a transaction is active and unlock the
// ** database file.
// **
// ** If the pager has already entered the ERROR state, do not attempt
// ** the rollback at this time. Instead, pager_unlock() is called. The
// ** call to pager_unlock() will discard all in-memory pages, unlock
// ** the database file and move the pager back to OPEN state. If this
// ** means that there is a hot-journal left in the file-system, the next
// ** connection to obtain a shared lock on the pager (which may be this one)
// ** will roll it back.
// **
// ** If the pager has not already entered the ERROR state, but an IO or
// ** malloc error occurs during a rollback, then this will itself cause
// ** the pager to enter the ERROR state. Which will be cleared by the
// ** call to pager_unlock(), as described above.
// */
fn pagerUnlockAndRollback(mut pPager: *mut Pager) {
    if (((unsafe { (*pPager).eState }) as u32) as i32) != (6 as i32)
        && (((unsafe { (*pPager).eState }) as u32) as i32) != (0 as i32)
    {
        0 as i32;
        if (((unsafe { (*pPager).eState }) as u32) as i32) >= (2 as i32) {
            unsafe { sqlite3BeginBenignMalloc() };
            sqlite3PagerRollback(pPager);
            unsafe { sqlite3EndBenignMalloc() };
        } else {
            if !((unsafe { (*pPager).exclusiveMode }) != (0 as u8)) {
                0 as i32;
                pager_end_transaction(pPager, 0 as i32, 0 as i32);
            }
        }
    } else {
        if (((unsafe { (*pPager).eState }) as u32) as i32) == (6 as i32)
            && (((unsafe { (*pPager).journalMode }) as u32) as i32) == (4 as i32)
            && (unsafe { (*unsafe { (*pPager).jfd }).pMethods })
                != std::ptr::null::<sqlite3_io_methods>()
        {
            // /* Special case for a ROLLBACK due to I/O error with an in-memory
            //     ** journal:  We have to rollback immediately, before the journal is
            //     ** closed, because once it is closed, all content is forgotten. */
            let mut errCode: i32 = unsafe { (*pPager).errCode };
            let mut eLock: u8 = unsafe { (*pPager).eLock };
            unsafe {
                (*pPager).eState = ((0 as i32) as i8) as u8;
            }
            unsafe {
                (*pPager).errCode = 0 as i32;
            }
            unsafe {
                (*pPager).eLock = ((4 as i32) as i8) as u8;
            }
            pager_playback(pPager, 1 as i32);
            unsafe {
                (*pPager).errCode = errCode;
            }
            unsafe {
                (*pPager).eLock = eLock;
            }
        }
    }
    pager_unlock(pPager);
}

// /*
// ** Parameter aData must point to a buffer of pPager->pageSize bytes
// ** of data. Compute and return a checksum based on the contents of the
// ** page of data and the current value of pPager->cksumInit.
// **
// ** This is not a real checksum. It is really just the sum of the
// ** random initial value (pPager->cksumInit) and every 200th byte
// ** of the page data, starting with byte offset (pPager->pageSize%200).
// ** Each byte is interpreted as an 8-bit unsigned integer.
// **
// ** Changing the formula used to compute this checksum results in an
// ** incompatible journal file format.
// **
// ** If journal corruption occurs due to a power failure, the most likely
// ** scenario is that one end or the other of the record will be changed.
// ** It is much less likely that the two ends of the journal record will be
// ** correct and the middle be corrupt.  Thus, this "checksum" scheme,
// ** though fast and simple, catches the mostly likely kind of corruption.
// */
fn pager_cksum(mut pPager: *mut Pager, mut aData: *const u8) -> u32 {
    // /* Checksum value to return */
    let mut cksum: u32 = unsafe { (*pPager).cksumInit };
    // /* Loop counter */
    let mut i: i32 = ((unsafe { (*pPager).pageSize }) - ((200 as i32) as i64)) as i32;
    '__slate_break_1522: while i > (0 as i32) {
        let __v1721: u32 = cksum;
        let __v1722: u32 = __v1721.wrapping_add(
            (((unsafe { *unsafe { aData.offset(i as isize) } }) as u32) as i32) as u32,
        );
        cksum = __v1722;
        let __v1723: i32 = i;
        let __v1724: i32 = __v1723 - (200 as i32);
        i = __v1724;
    }
    return cksum;
}

// /*
// ** Read a single page from either the journal file (if isMainJrnl==1) or
// ** from the sub-journal (if isMainJrnl==0) and playback that page.
// ** The page begins at offset *pOffset into the file. The *pOffset
// ** value is increased to the start of the next page in the journal.
// **
// ** The main rollback journal uses checksums - the statement journal does
// ** not.
// **
// ** If the page number of the page record read from the (sub-)journal file
// ** is greater than the current value of Pager.dbSize, then playback is
// ** skipped and SQLITE_OK is returned.
// **
// ** If pDone is not NULL, then it is a record of pages that have already
// ** been played back.  If the page at *pOffset has already been played back
// ** (if the corresponding pDone bit is set) then skip the playback.
// ** Make sure the pDone bit corresponding to the *pOffset page is set
// ** prior to returning.
// **
// ** If the page record is successfully read from the (sub-)journal file
// ** and played back, then SQLITE_OK is returned. If an IO error occurs
// ** while reading the record from the (sub-)journal file or while writing
// ** to the database file, then the IO error code is returned. If data
// ** is successfully read from the (sub-)journal file but appears to be
// ** corrupted, SQLITE_DONE is returned. Data is considered corrupted in
// ** two circumstances:
// **
// **   * If the record page-number is illegal (0 or PAGER_SJ_PGNO), or
// **   * If the record is being rolled back from the main journal file
// **     and the checksum field does not match the record content.
// **
// ** Neither of these two scenarios are possible during a savepoint rollback.
// **
// ** If this is a savepoint rollback, then memory may have to be dynamically
// ** allocated by this function. If this is the case and an allocation fails,
// ** SQLITE_NOMEM is returned.
// */
fn pager_playback_one_page(
    mut pPager: *mut Pager,
    mut pOffset: *mut i64,
    mut pDone: *mut Bitvec,
    mut isMainJrnl: i32,
    mut isSavepnt: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    // /* An existing page in the cache */
    let mut pPg: *mut PgHdr = unsafe { std::mem::zeroed() };
    // /* The page number of a page in journal */
    let mut pgno: u32 = 0 as u32;
    // /* Checksum used for sanity checking */
    let mut cksum: u32 = 0 as u32;
    // /* Temporary storage for the page */
    let mut aData: *mut i8 = unsafe { std::mem::zeroed() };
    // /* The file descriptor for the journal file */
    let mut jfd: *mut sqlite3_file = unsafe { std::mem::zeroed() };
    // /* True if journal page is synced */
    let mut isSynced: i32 = 0 as i32;
    // /* isMainJrnl is 0 or 1 */
    0 as i32;
    // /* isSavepnt is 0 or 1 */
    0 as i32;
    // /* pDone always used on sub-journals */
    0 as i32;
    // /* pDone never used on non-savepoint */
    0 as i32;
    aData = unsafe { (*pPager).pTmpSpace };
    // /* Temp storage must have already been allocated */
    0 as i32;
    0 as i32;
    // /* Either the state is greater than PAGER_WRITER_CACHEMOD (a transaction
    //   ** or savepoint rollback done at the request of the caller) or this is
    //   ** a hot-journal rollback. If it is a hot-journal rollback, the pager
    //   ** is in state OPEN and holds an EXCLUSIVE lock. Hot-journal rollback
    //   ** only reads from the main journal, not the sub-journal.
    //   */
    0 as i32;
    0 as i32;
    // /* Read the page number and page data from the journal or sub-journal
    //   ** file. Return an error code to the caller if an IO error occurs.
    //   */
    jfd = if isMainJrnl != (0 as i32) {
        unsafe { (*pPager).jfd }
    } else {
        unsafe { (*pPager).sjfd }
    };
    rc = read32bits(jfd, unsafe { *pOffset }, std::ptr::addr_of_mut!(pgno));
    if rc != (0 as i32) {
        return rc;
    }
    rc = unsafe {
        sqlite3OsRead(
            jfd,
            (aData as *mut u8) as *mut (),
            (unsafe { (*pPager).pageSize }) as i32,
            (unsafe { *pOffset }) + ((4 as i32) as i64),
        )
    };
    if rc != (0 as i32) {
        return rc;
    }
    let __v1725: *mut i64 = pOffset;
    let __v1726: i64 = unsafe { *__v1725 };
    let __v1727: i64 = __v1726
        + ((unsafe { (*pPager).pageSize })
            + ((4 as i32) as i64)
            + ((isMainJrnl * (4 as i32)) as i64));
    unsafe {
        *__v1725 = __v1727;
    }
    // /* Sanity checking on the page.  This is more important that I originally
    //   ** thought.  If a power failure occurs while the journal is being written,
    //   ** it could cause invalid data to be written into the journal.  We need to
    //   ** detect this invalid data (with high probability) and ignore it.
    //   */
    if pgno == ((0 as i32) as u32) || pgno == unsafe { (*pPager).lckPgno } {
        0 as i32;
        return 101 as i32;
    }
    let __v1728: bool;
    if pgno > unsafe { (*pPager).dbSize } {
        __v1728 = true as bool;
    } else {
        __v1728 = (unsafe { sqlite3BitvecTest(pDone, pgno) }) != (0 as i32);
    }
    if __v1728 {
        return 0 as i32;
    }
    if isMainJrnl != (0 as i32) {
        rc = read32bits(
            jfd,
            (unsafe { *pOffset }) - ((4 as i32) as i64),
            std::ptr::addr_of_mut!(cksum),
        );
        if rc != (0 as i32) {
            return rc;
        }
        let __v1729: bool;
        if !(isSavepnt != (0 as i32)) {
            __v1729 = pager_cksum(pPager, (aData as *mut u8) as *const u8) != cksum;
        } else {
            __v1729 = false as bool;
        }
        if __v1729 {
            return 101 as i32;
        }
    }
    // /* If this page has already been played back before during the current
    //   ** rollback, then don't bother to play it back again.
    //   */
    let __v1730: bool;
    if pDone != std::ptr::null_mut::<Bitvec>() {
        let __v1731: i32 = unsafe { sqlite3BitvecSet(pDone, pgno) };
        rc = __v1731;
        __v1730 = __v1731 != (0 as i32);
    } else {
        __v1730 = false as bool;
    }
    if __v1730 {
        return rc;
    }
    // /* When playing back page 1, restore the nReserve setting
    //   */
    if pgno == ((1 as i32) as u32)
        && ((unsafe { (*pPager).nReserve }) as i32)
            != (((unsafe { *unsafe { (aData as *mut u8).offset((20 as i32) as isize) } }) as u32)
                as i32)
    {
        unsafe {
            (*pPager).nReserve =
                ((unsafe { *unsafe { (aData as *mut u8).offset((20 as i32) as isize) } }) as u16)
                    as i16;
        }
    }
    // /* If the pager is in CACHEMOD state, then there must be a copy of this
    //   ** page in the pager cache. In this case just update the pager cache,
    //   ** not the database file. The page is left marked dirty in this case.
    //   **
    //   ** An exception to the above rule: If the database is in no-sync mode
    //   ** and a page is moved during an incremental vacuum then the page may
    //   ** not be in the pager cache. Later: if a malloc() or IO error occurs
    //   ** during a Movepage() call, then the page may not be in the cache
    //   ** either. So the condition described in the above paragraph is not
    //   ** assert()able.
    //   **
    //   ** If in WRITER_DBMOD, WRITER_FINISHED or OPEN state, then we update the
    //   ** pager cache if it exists and the main file. The page is then marked
    //   ** not dirty. Since this code is only executed in PAGER_OPEN state for
    //   ** a hot-journal rollback, it is guaranteed that the page-cache is empty
    //   ** if the pager is in OPEN state.
    //   **
    //   ** Ticket #1171:  The statement journal might contain page content that is
    //   ** different from the page content at the start of the transaction.
    //   ** This occurs when a page is changed prior to the start of a statement
    //   ** then changed again within the statement.  When rolling back such a
    //   ** statement we must not write to the original database unless we know
    //   ** for certain that original page contents are synced into the main rollback
    //   ** journal.  Otherwise, a power loss might leave modified data in the
    //   ** database file without an entry in the rollback journal that can
    //   ** restore the database to its original form.  Two conditions must be
    //   ** met before writing to the database files. (1) the database must be
    //   ** locked.  (2) we know that the original page content is fully synced
    //   ** in the main journal either because the page is not in cache or else
    //   ** the page is marked as needSync==0.
    //   **
    //   ** 2008-04-14:  When attempting to vacuum a corrupt database file, it
    //   ** is possible to fail a statement on a database that does not yet exist.
    //   ** Do not attempt to write if database file has never been opened.
    //   */
    if (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
        pPg = std::ptr::null_mut::<PgHdr>();
    } else {
        pPg = sqlite3PagerLookup(pPager, pgno);
    }
    0 as i32;
    0 as i32;
    {}
    if isMainJrnl != (0 as i32) {
        isSynced = ((unsafe { (*pPager).noSync }) != (0 as u8)
            || (unsafe { *pOffset }) <= unsafe { (*pPager).journalHdr }) as i32;
    } else {
        isSynced = (pPg == std::ptr::null_mut::<PgHdr>()
            || (0 as i32) == (((unsafe { (*pPg).flags }) as u32) as i32) & (8 as i32))
            as i32;
    }
    if (unsafe { (*unsafe { (*pPager).fd }).pMethods }) != std::ptr::null::<sqlite3_io_methods>()
        && ((((unsafe { (*pPager).eState }) as u32) as i32) >= (4 as i32)
            || (((unsafe { (*pPager).eState }) as u32) as i32) == (0 as i32))
        && isSynced != (0 as i32)
    {
        let mut ofst: i64 =
            ((pgno.wrapping_sub((1 as i32) as u32) as u64) as i64) * unsafe { (*pPager).pageSize };
        {}
        0 as i32;
        // /* Write the data read from the journal back into the database file.
        //     ** This is usually safe even for an encrypted database - as the data
        //     ** was encrypted before it was written to the journal file. The exception
        //     ** is if the data was just read from an in-memory sub-journal. In that
        //     ** case it must be encrypted here before it is copied into the database
        //     ** file.  */
        rc = unsafe {
            sqlite3OsWrite(
                unsafe { (*pPager).fd },
                (aData as *mut u8) as *const (),
                (unsafe { (*pPager).pageSize }) as i32,
                ofst,
            )
        };
        if pgno > unsafe { (*pPager).dbFileSize } {
            unsafe {
                (*pPager).dbFileSize = pgno;
            }
        }
        if (unsafe { (*pPager).pBackup }) != std::ptr::null_mut::<sqlite3_backup>() {
            unsafe {
                sqlite3BackupUpdate(
                    unsafe { (*pPager).pBackup },
                    pgno,
                    (aData as *mut u8) as *const u8,
                )
            };
        }
    } else {
        if !(isMainJrnl != (0 as i32)) && pPg == std::ptr::null_mut::<PgHdr>() {
            // /* If this is a rollback of a savepoint and data was not written to
            //     ** the database and the page is not in-memory, there is a potential
            //     ** problem. When the page is next fetched by the b-tree layer, it
            //     ** will be read from the database file, which may or may not be
            //     ** current.
            //     **
            //     ** There are a couple of different ways this can happen. All are quite
            //     ** obscure. When running in synchronous mode, this can only happen
            //     ** if the page is on the free-list at the start of the transaction, then
            //     ** populated, then moved using sqlite3PagerMovepage().
            //     **
            //     ** The solution is to add an in-memory page to the cache containing
            //     ** the data just read from the sub-journal. Mark the page as dirty
            //     ** and if the pager requires a journal-sync, then mark the page as
            //     ** requiring a journal-sync before it is written.
            //     */
            0 as i32;
            0 as i32;
            let __v1732: *mut Pager = pPager;
            let __v1733: u8 = unsafe { (*__v1732).doNotSpill };
            let __v1734: u8 = ((((__v1733 as u32) as i32) | (2 as i32)) as i8) as u8;
            unsafe {
                (*__v1732).doNotSpill = __v1734;
            }
            rc = sqlite3PagerGet(pPager, pgno, std::ptr::addr_of_mut!(pPg), 1 as i32);
            0 as i32;
            let __v1735: *mut Pager = pPager;
            let __v1736: u8 = unsafe { (*__v1735).doNotSpill };
            let __v1737: u8 = ((((__v1736 as u32) as i32) & !(2 as i32)) as i8) as u8;
            unsafe {
                (*__v1735).doNotSpill = __v1737;
            }
            if rc != (0 as i32) {
                return rc;
            }
            unsafe { sqlite3PcacheMakeDirty(pPg) };
        }
    }
    if pPg != std::ptr::null_mut::<PgHdr>() {
        // /* No page should ever be explicitly rolled back that is in use, except
        //     ** for page 1 which is held in use in order to keep the lock on the
        //     ** database active. However such a page may be rolled back as a result
        //     ** of an internal error resulting in an automatic call to
        //     ** sqlite3PagerRollback().
        //     */
        let mut pData: *mut () = unsafe { std::mem::zeroed() };
        pData = unsafe { (*pPg).pData };
        unsafe {
            memcpy(
                pData,
                (aData as *mut u8) as *const (),
                (unsafe { (*pPager).pageSize }) as u64,
            )
        };
        unsafe { unsafe { (*pPager).xReiniter }.unwrap()(pPg) };
        // /* It used to be that sqlite3PcacheMakeClean(pPg) was called here.  But
        //     ** that call was dangerous and had no detectable benefit since the cache
        //     ** is normally cleaned by sqlite3PcacheCleanAll() after rollback and so
        //     ** has been removed. */
        {}
        // /* If this was page 1, then restore the value of Pager.dbFileVers.
        //     ** Do this before any decoding. */
        if pgno == ((1 as i32) as u32) {
            unsafe {
                memcpy(
                    (unsafe { std::ptr::addr_of_mut!((*pPager).dbFileVers) }) as *mut (),
                    (unsafe { (pData as *mut u8).offset((24 as i32) as isize) }) as *const (),
                    16 as u64,
                )
            };
        }
        unsafe { sqlite3PcacheRelease(pPg) };
    }
    return rc;
}

// /* The pager being played back */
// /* Offset of record to playback */
// /* Bitvec of pages already played back */
// /* 1 -> main journal. 0 -> sub-journal. */
// /* True for a savepoint rollback */
// /*
// ** Parameter zSuper is the name of a super-journal file. A single journal
// ** file that referred to the super-journal file has just been rolled back.
// ** This routine checks if it is possible to delete the super-journal file,
// ** and does so if it is.
// **
// ** Argument zSuper may point to Pager.pTmpSpace. So that buffer is not
// ** available for use within this function.
// **
// ** When a super-journal file is created, it is populated with the names
// ** of all of its child journals, one after another, formatted as utf-8
// ** encoded text. The end of each child journal file is marked with a
// ** nul-terminator byte (0x00). i.e. the entire contents of a super-journal
// ** file for a transaction involving two databases might be:
// **
// **   "/home/bill/a.db-journal\x00/home/bill/b.db-journal\x00"
// **
// ** A super-journal file may only be deleted once all of its child
// ** journals have been rolled back.
// **
// ** This function reads the contents of the super-journal file into
// ** memory and loops through each of the child journal names. For
// ** each child journal, it checks if:
// **
// **   * if the child journal exists, and if so
// **   * if the child journal contains a reference to super-journal
// **     file zSuper
// **
// ** If a child journal can be found that matches both of the criteria
// ** above, this function returns without doing anything. Otherwise, if
// ** no such child journal can be found, file zSuper is deleted from
// ** the file-system using sqlite3OsDelete().
// **
// ** If an IO error within this function, an error code is returned. This
// ** function allocates memory by calling sqlite3Malloc(). If an allocation
// ** fails, SQLITE_NOMEM is returned. Otherwise, if no IO or malloc errors
// ** occur, SQLITE_OK is returned.
// **
// ** TODO: This function allocates a single block of memory to load
// ** the entire contents of the super-journal file. This could be
// ** a couple of kilobytes or so - potentially larger than the page
// ** size.
// */
fn pager_delsuper(mut pPager: *mut Pager, mut zSuper: *const i8) -> i32 {
    let mut __slate_storage_1739: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1739: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1739) as *mut *mut i8;
    let mut __slate_storage_1738: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1738: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1738) as *mut *mut i8;
    let mut __slate_storage_739: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_739: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_739) as *mut i32;
    let mut __slate_storage_738: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_738: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_738) as *mut i32;
    let mut __slate_storage_737: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_737: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_737) as *mut *mut i8;
    let mut __slate_storage_736: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_736: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_736) as *mut i32;
    let mut __slate_storage_735: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_735: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_735) as *mut i32;
    let mut __slate_storage_734: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_734: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_734) as *mut i32;
    let mut __slate_storage_733: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_733: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_733) as *mut *mut i8;
    let mut __slate_storage_732: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_732: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_732) as *mut *mut i8;
    let mut __slate_storage_731: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_731: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_731) as *mut i64;
    let mut __slate_storage_730: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_730: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_730) as *mut *mut i8;
    let mut __slate_storage_729: std::mem::MaybeUninit<*mut sqlite3_file> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_729: *mut *mut sqlite3_file =
        std::ptr::addr_of_mut!(__slate_storage_729) as *mut *mut sqlite3_file;
    let mut __slate_storage_728: std::mem::MaybeUninit<*mut sqlite3_file> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_728: *mut *mut sqlite3_file =
        std::ptr::addr_of_mut!(__slate_storage_728) as *mut *mut sqlite3_file;
    let mut __slate_storage_727: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_727: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_727) as *mut i32;
    let mut __slate_storage_726: std::mem::MaybeUninit<*mut sqlite3_vfs> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_726: *mut *mut sqlite3_vfs =
        std::ptr::addr_of_mut!(__slate_storage_726) as *mut *mut sqlite3_vfs;
    unsafe {
        std::ptr::write(__slate_slot_726, unsafe { (*pPager).pVfs });
        // /* Return code */
        // /* Malloc'd super-journal file descriptor */
        // /* Malloc'd child-journal file descriptor */
        // /* Contents of super-journal file */
        std::ptr::write(__slate_slot_730, std::ptr::null_mut::<i8>());
        // /* Size of super-journal file */
        // /* Pointer to one journal within MJ file */
        // /* Free this buffer */
        std::ptr::write(__slate_slot_733, std::ptr::null_mut::<i8>());
        // /* If super-journal contains pPager->zJournal */
        std::ptr::write(__slate_slot_734, 0 as i32);
        // /* Check if this looks like a real super-journal name. If it does not,
        //   ** return SQLITE_OK without attempting to delete it. This is to limit
        //   ** the degree to which a crafted journal file can be used to cause
        //   ** SQLite to delete arbitrary files.
        //   **
        //   ** This test never fails, becaue the super journal name is checked
        //   ** by readSuperJournal().
        //   */
        if pagerIsSuperJrnlName(zSuper) == (0 as i32) {
            return 0 as i32;
        } else {
            // /* Allocate space for both the pJournal and pSuper file descriptors.
            //   ** If successful, open the super-journal file for reading.
            //   */
            *__slate_slot_728 = (unsafe {
                sqlite3MallocZero(
                    (((2 as i32) as i64) * ((unsafe { (*(*__slate_slot_726)).szOsFile }) as i64))
                        as u64,
                )
            }) as *mut sqlite3_file;
            if !(*__slate_slot_728 != std::ptr::null_mut::<sqlite3_file>()) {
                *__slate_slot_727 = 7 as i32;
                *__slate_slot_729 = std::ptr::null_mut::<sqlite3_file>();
            } else {
                std::ptr::write(__slate_slot_735, (1 as i32) | (16384 as i32));
                *__slate_slot_727 = unsafe {
                    sqlite3OsOpen(
                        *__slate_slot_726,
                        zSuper,
                        *__slate_slot_728,
                        *__slate_slot_735,
                        std::ptr::null_mut::<i32>(),
                    )
                };
                *__slate_slot_729 = (unsafe {
                    (*__slate_slot_728 as *mut u8)
                        .offset((unsafe { (*(*__slate_slot_726)).szOsFile }) as isize)
                }) as *mut sqlite3_file;
            }
            '__join_2: {
                if *__slate_slot_727 != (0 as i32) {
                } else {
                    // /* Load the entire super-journal file into space obtained from
                    //   ** sqlite3_malloc() and pointed to by zSuperJournal.   Also obtain
                    //   ** sufficient space (in zSuperPtr) to hold the names of super-journal
                    //   ** files extracted from regular rollback-journals.
                    //   */
                    *__slate_slot_727 = unsafe {
                        sqlite3OsFileSize(
                            *__slate_slot_728,
                            std::ptr::addr_of_mut!(*__slate_slot_731),
                        )
                    };
                    if *__slate_slot_727 != (0 as i32) {
                    } else {
                        0 as i32;
                        *__slate_slot_733 = (unsafe {
                            sqlite3Malloc(
                                (((4 as i32) as i64) + *__slate_slot_731 + ((2 as i32) as i64))
                                    as u64,
                            )
                        }) as *mut i8;
                        if !(*__slate_slot_733 != std::ptr::null_mut::<i8>()) {
                            *__slate_slot_727 = 7 as i32;
                        } else {
                            0 as i32;
                            unsafe {
                                *unsafe { (*__slate_slot_733).offset((3 as i32) as isize) } =
                                    (0 as i32) as i8;
                            }
                            unsafe {
                                *unsafe { (*__slate_slot_733).offset((2 as i32) as isize) } =
                                    (0 as i32) as i8;
                            }
                            unsafe {
                                *unsafe { (*__slate_slot_733).offset((1 as i32) as isize) } =
                                    (0 as i32) as i8;
                            }
                            unsafe {
                                *unsafe { (*__slate_slot_733).offset((0 as i32) as isize) } =
                                    (0 as i32) as i8;
                            }
                            *__slate_slot_730 =
                                unsafe { (*__slate_slot_733).offset((4 as i32) as isize) };
                            *__slate_slot_727 = unsafe {
                                sqlite3OsRead(
                                    *__slate_slot_728,
                                    *__slate_slot_730 as *mut (),
                                    *__slate_slot_731 as i32,
                                    (0 as i32) as i64,
                                )
                            };
                            if *__slate_slot_727 != (0 as i32) {
                            } else {
                                unsafe {
                                    *unsafe {
                                        (*__slate_slot_730).offset(*__slate_slot_731 as isize)
                                    } = (0 as i32) as i8;
                                }
                                unsafe {
                                    *unsafe {
                                        (*__slate_slot_730).offset(
                                            (*__slate_slot_731 + ((1 as i32) as i64)) as isize,
                                        )
                                    } = (0 as i32) as i8;
                                }
                                *__slate_slot_732 = *__slate_slot_730;
                                '__join_10: {
                                    '__join_8: {
                                        loop {
                                            if ((unsafe {
                                                (*__slate_slot_732)
                                                    .offset_from(*__slate_slot_730 as *mut i8)
                                            })
                                                as i64)
                                                < *__slate_slot_731
                                            {
                                                if (unsafe {
                                                    strcmp(
                                                        *__slate_slot_732 as *const i8,
                                                        (unsafe { (*pPager).zJournal })
                                                            as *const i8,
                                                    )
                                                }) == (0 as i32)
                                                {
                                                    *__slate_slot_734 = 1 as i32;
                                                } else {
                                                    *__slate_slot_727 = unsafe {
                                                        sqlite3OsAccess(
                                                            *__slate_slot_726,
                                                            *__slate_slot_732 as *const i8,
                                                            0 as i32,
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_736
                                                            ),
                                                        )
                                                    };
                                                    if *__slate_slot_727 != (0 as i32) {
                                                        break '__join_2;
                                                    } else {
                                                        if *__slate_slot_736 != (0 as i32) {
                                                            std::ptr::write(
                                                                __slate_slot_737,
                                                                std::ptr::null_mut::<i8>(),
                                                            );
                                                            // /* One of the journals pointed to by the super-journal exists.
                                                            //         ** Open it and check if it points at the super-journal. If
                                                            //         ** so, return without deleting the super-journal file.
                                                            //         ** NB:  zJournal is really a MAIN_JOURNAL.  But call it a
                                                            //         ** SUPER_JOURNAL here so that the VFS will not send the zJournal
                                                            //         ** name into sqlite3_database_file_object().
                                                            //         */
                                                            std::ptr::write(
                                                                __slate_slot_739,
                                                                (1 as i32) | (16384 as i32),
                                                            );
                                                            *__slate_slot_727 = unsafe {
                                                                sqlite3OsOpen(
                                                                    *__slate_slot_726,
                                                                    *__slate_slot_732 as *const i8,
                                                                    *__slate_slot_729,
                                                                    *__slate_slot_739,
                                                                    std::ptr::null_mut::<i32>(),
                                                                )
                                                            };
                                                            if *__slate_slot_727 != (0 as i32) {
                                                                break '__join_2;
                                                            } else {
                                                                *__slate_slot_727 = readSuperJournal(*__slate_slot_729, (((1 as i32) as i64) as u64).wrapping_add(((unsafe { (*(*__slate_slot_726)).mxPathname }) as i64) as u64), std::ptr::addr_of_mut!(*__slate_slot_737));
                                                                unsafe {
                                                                    sqlite3OsClose(
                                                                        *__slate_slot_729,
                                                                    )
                                                                };
                                                                if *__slate_slot_727 != (0 as i32) {
                                                                    break '__join_10;
                                                                } else {
                                                                    *__slate_slot_738 =
                                                                        (*__slate_slot_737
                                                                            != std::ptr::null_mut::<
                                                                                i8,
                                                                            >(
                                                                            )
                                                                            && (unsafe {
                                                                                strcmp(*__slate_slot_737 as *const i8, zSuper)
                                                                            }) == (0 as i32))
                                                                            as i32;
                                                                    freeSuperJournal(
                                                                        *__slate_slot_737,
                                                                    );
                                                                    if *__slate_slot_738
                                                                        != (0 as i32)
                                                                    {
                                                                        break '__join_8;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                std::ptr::write(
                                                    __slate_slot_1738,
                                                    *__slate_slot_732,
                                                );
                                                std::ptr::write(__slate_slot_1739, unsafe {
                                                    (*__slate_slot_1738).offset(
                                                        ((unsafe {
                                                            sqlite3Strlen30(
                                                                *__slate_slot_732 as *const i8,
                                                            )
                                                        }) + (1 as i32))
                                                            as isize,
                                                    )
                                                });
                                                *__slate_slot_732 = *__slate_slot_1739;
                                            } else {
                                                break;
                                            }
                                        }
                                        unsafe { sqlite3OsClose(*__slate_slot_728) };
                                        if *__slate_slot_734 != (0 as i32) {
                                            // /* Only delete the super-journal if bSeen is true - indicating that
                                            //     ** the super-journal contained a pointer to this database's journal
                                            //     ** file. */
                                            *__slate_slot_727 = unsafe {
                                                sqlite3OsDelete(*__slate_slot_726, zSuper, 0 as i32)
                                            };
                                            break '__join_2;
                                        } else {
                                            break '__join_2;
                                        }
                                    }
                                    // /* We have a match. Do not delete the super-journal file. */
                                    break '__join_2;
                                }
                                0 as i32;
                            }
                        }
                    }
                }
            }
            unsafe { sqlite3_free(*__slate_slot_733 as *mut ()) };
            if *__slate_slot_728 != std::ptr::null_mut::<sqlite3_file>() {
                unsafe { sqlite3OsClose(*__slate_slot_728) };
                0 as i32;
                unsafe { sqlite3_free(*__slate_slot_728 as *mut ()) };
            }
            return *__slate_slot_727;
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Set the value of the Pager.sectorSize variable for the given
// ** pager based on the value returned by the xSectorSize method
// ** of the open database file. The sector size will be used
// ** to determine the size and alignment of journal header and
// ** super-journal pointers within created journal files.
// **
// ** For temporary files the effective sector size is always 512 bytes.
// **
// ** Otherwise, for non-temporary files, the effective sector size is
// ** the value returned by the xSectorSize() method rounded up to 32 if
// ** it is less than 32, or rounded down to MAX_SECTOR_SIZE if it
// ** is greater than MAX_SECTOR_SIZE.
// **
// ** If the file has the SQLITE_IOCAP_POWERSAFE_OVERWRITE property, then set
// ** the effective sector size to its minimum value (512).  The purpose of
// ** pPager->sectorSize is to define the "blast radius" of bytes that
// ** might change if a crash occurs while writing to a single byte in
// ** that range.  But with POWERSAFE_OVERWRITE, the blast radius is zero
// ** (that is what POWERSAFE_OVERWRITE means), so we minimize the sector
// ** size.  For backwards compatibility of the rollback journal file format,
// ** we cannot reduce the effective sector size below 512.
// */
fn setSectorSize(mut pPager: *mut Pager) {
    0 as i32;
    let __v1740: bool;
    if (unsafe { (*pPager).tempFile }) != (0 as u8) {
        __v1740 = true as bool;
    } else {
        __v1740 = (unsafe { sqlite3OsDeviceCharacteristics(unsafe { (*pPager).fd }) })
            & (4096 as i32)
            != (0 as i32);
    }
    if __v1740 {
        // /* Sector size doesn't matter for temporary files. Also, the file
        //     ** may not have been opened yet, in which case the OsSectorSize()
        //     ** call will segfault. */
        unsafe {
            (*pPager).sectorSize = (512 as i32) as u32;
        }
    } else {
        unsafe {
            (*pPager).sectorSize = sqlite3SectorSize(unsafe { (*pPager).fd }) as u32;
        }
    }
}

// /*
// ** Read the content for page pPg out of the database file (or out of
// ** the WAL if that is where the most recent copy if found) into
// ** pPg->pData. A shared lock or greater must be held on the database
// ** file before this function is called.
// **
// ** If page 1 is read, then the value of Pager.dbFileVers[] is set to
// ** the value read from the database file.
// **
// ** If an IO error occurs, then the IO error is returned to the caller.
// ** Otherwise, SQLITE_OK is returned.
// */
fn readDbPage(mut pPg: *mut PgHdr) -> i32 {
    // /* Pager object associated with page pPg */
    let mut pPager: *mut Pager = unsafe { (*pPg).pPager };
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* Frame of WAL containing pgno */
    let mut iFrame: u32 = (0 as i32) as u32;
    0 as i32;
    0 as i32;
    if (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
        rc = unsafe {
            sqlite3WalFindFrame(
                unsafe { (*pPager).pWal },
                unsafe { (*pPg).pgno },
                std::ptr::addr_of_mut!(iFrame),
            )
        };
        if rc != (0 as i32) {
            return rc;
        }
    }
    if iFrame != (0 as u32) {
        rc = unsafe {
            sqlite3WalReadFrame(
                unsafe { (*pPager).pWal },
                iFrame,
                (unsafe { (*pPager).pageSize }) as i32,
                (unsafe { (*pPg).pData }) as *mut u8,
            )
        };
    } else {
        let mut iOffset: i64 = ((unsafe { (*pPg).pgno }.wrapping_sub((1 as i32) as u32) as u64)
            as i64)
            * unsafe { (*pPager).pageSize };
        rc = unsafe {
            sqlite3OsRead(
                unsafe { (*pPager).fd },
                unsafe { (*pPg).pData },
                (unsafe { (*pPager).pageSize }) as i32,
                iOffset,
            )
        };
        if rc == (10 as i32) | (2 as i32) << (8 as i32) {
            rc = 0 as i32;
        }
    }
    if (unsafe { (*pPg).pgno }) == ((1 as i32) as u32) {
        if rc != (0 as i32) {
            // /* If the read is unsuccessful, set the dbFileVers[] to something
            //       ** that will never be a valid file version.  dbFileVers[] is a copy
            //       ** of bytes 24..39 of the database.  Bytes 28..31 should always be
            //       ** zero or the size of the database in page. Bytes 32..35 and 35..39
            //       ** should be page numbers which are never 0xffffffff.  So filling
            //       ** pPager->dbFileVers[] with all 0xff bytes should suffice.
            //       **
            //       ** For an encrypted database, the situation is more complex:  bytes
            //       ** 24..39 of the database are white noise.  But the probability of
            //       ** white noise equaling 16 bytes of 0xff is vanishingly small so
            //       ** we should still be ok.
            //       */
            unsafe {
                memset(
                    (unsafe { (*pPager).dbFileVers.as_mut_ptr() as *mut i8 }) as *mut (),
                    255 as i32,
                    16 as u64,
                )
            };
        } else {
            let mut dbFileVers: *mut u8 =
                unsafe { ((unsafe { (*pPg).pData }) as *mut u8).offset((24 as i32) as isize) };
            unsafe {
                memcpy(
                    (unsafe { std::ptr::addr_of_mut!((*pPager).dbFileVers) }) as *mut (),
                    dbFileVers as *const (),
                    16 as u64,
                )
            };
        }
    }
    {}
    {}
    {}
    {}
    return rc;
}

// /*
// ** Update the value of the change-counter at offsets 24 and 92 in
// ** the header and the sqlite version number at offset 96.
// **
// ** This is an unconditional update.  See also the pager_incr_changecounter()
// ** routine which only updates the change-counter if the update is actually
// ** needed, as determined by the pPager->changeCountDone state variable.
// */
fn pager_write_changecounter(mut pPg: *mut PgHdr) {
    let mut change_counter: u32 = 0 as u32;
    if pPg == std::ptr::null_mut::<PgHdr>() {
        return;
    }
    // /* Increment the value just read and write it back to byte 24. */
    change_counter = unsafe {
        sqlite3Get4byte(
            ((unsafe { (*unsafe { (*pPg).pPager }).dbFileVers.as_mut_ptr() as *mut i8 }) as *mut u8)
                as *const u8,
        )
    }
    .wrapping_add((1 as i32) as u32);
    unsafe {
        sqlite3Put4byte(
            unsafe {
                (((unsafe { (*pPg).pData }) as *mut i8) as *mut u8).offset((24 as i32) as isize)
            },
            change_counter,
        )
    };
    // /* Also store the SQLite version number in bytes 96..99 and in
    //   ** bytes 92..95 store the change counter for which the version number
    //   ** is valid. */
    unsafe {
        sqlite3Put4byte(
            unsafe {
                (((unsafe { (*pPg).pData }) as *mut i8) as *mut u8).offset((92 as i32) as isize)
            },
            change_counter,
        )
    };
    unsafe {
        sqlite3Put4byte(
            unsafe {
                (((unsafe { (*pPg).pData }) as *mut i8) as *mut u8).offset((96 as i32) as isize)
            },
            (3054000 as i32) as u32,
        )
    };
}

// /*
// ** This function is invoked once for each page that has already been
// ** written into the log file when a WAL transaction is rolled back.
// ** Parameter iPg is the page number of said page. The pCtx argument
// ** is actually a pointer to the Pager structure.
// **
// ** If page iPg is present in the cache, and has no outstanding references,
// ** it is discarded. Otherwise, if there are one or more outstanding
// ** references, the page content is reloaded from the database. If the
// ** attempt to reload content from the database is required and fails,
// ** return an SQLite error code. Otherwise, SQLITE_OK.
// */
#[unsafe(link_section = ".text.slate_distinct.pager.pagerUndoCallback")]
extern "C-unwind" fn pagerUndoCallback(mut pCtx: *mut (), mut iPg: u32) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pPager: *mut Pager = pCtx as *mut Pager;
    let mut pPg: *mut PgHdr = unsafe { std::mem::zeroed() };
    0 as i32;
    pPg = sqlite3PagerLookup(pPager, iPg);
    if pPg != std::ptr::null_mut::<PgHdr>() {
        if (unsafe { sqlite3PcachePageRefcount(pPg) }) == ((1 as i32) as i64) {
            unsafe { sqlite3PcacheDrop(pPg) };
        } else {
            rc = readDbPage(pPg);
            if rc == (0 as i32) {
                unsafe { unsafe { (*pPager).xReiniter }.unwrap()(pPg) };
            }
            sqlite3PagerUnrefNotNull(pPg);
        }
    }
    // /* Normally, if a transaction is rolled back, any backup processes are
    //   ** updated as data is copied out of the rollback journal and into the
    //   ** database. This is not generally possible with a WAL database, as
    //   ** rollback involves simply truncating the log file. Therefore, if one
    //   ** or more frames have already been written to the log (and therefore
    //   ** also copied into the backup databases) as part of this transaction,
    //   ** the backups must be restarted.
    //   */
    unsafe { sqlite3BackupRestart(unsafe { (*pPager).pBackup }) };
    return rc;
}

// /*
// ** This function is called to rollback a transaction on a WAL database.
// */
fn pagerRollbackWal(mut pPager: *mut Pager) -> i32 {
    // /* Return Code */
    let mut rc: i32 = 0 as i32;
    // /* List of dirty pages to revert */
    let mut pList: *mut PgHdr = unsafe { std::mem::zeroed() };
    // /* For all pages in the cache that are currently dirty or have already
    //   ** been written (but not committed) to the log file, do one of the
    //   ** following:
    //   **
    //   **   + Discard the cached page (if refcount==0), or
    //   **   + Reload page content from the database (if refcount>0).
    //   */
    unsafe {
        (*pPager).dbSize = unsafe { (*pPager).dbOrigSize };
    }
    rc = unsafe {
        sqlite3WalUndo(
            unsafe { (*pPager).pWal },
            Some(pagerUndoCallback),
            pPager as *mut (),
        )
    };
    pList = unsafe { sqlite3PcacheDirtyList(unsafe { (*pPager).pPCache }) };
    '__slate_break_1527: while pList != std::ptr::null_mut::<PgHdr>() && rc == (0 as i32) {
        let mut pNext: *mut PgHdr = unsafe { (*pList).pDirty };
        rc = pagerUndoCallback(pPager as *mut (), unsafe { (*pList).pgno });
        pList = pNext;
    }
    return rc;
}

// /*
// ** This function is a wrapper around sqlite3WalFrames(). As well as logging
// ** the contents of the list of pages headed by pList (connected by pDirty),
// ** this function notifies any active backup processes that the pages have
// ** changed.
// **
// ** The list of pages passed into this routine is always sorted by page number.
// ** Hence, if page 1 appears anywhere on the list, it will be the first page.
// */
fn pagerWalFrames(
    mut pPager: *mut Pager,
    mut pList: *mut PgHdr,
    mut nTruncate: u32,
    mut isCommit: i32,
) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* Number of pages in pList */
    let mut nList: i32 = 0 as i32;
    // /* For looping over pages */
    let mut p: *mut PgHdr = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    if isCommit != (0 as i32) {
        // /* If a WAL transaction is being committed, there is no point in writing
        //     ** any pages with page numbers greater than nTruncate into the WAL file.
        //     ** They will never be read by any client. So remove them from the pDirty
        //     ** list here. */
        let mut ppNext: *mut *mut PgHdr = std::ptr::addr_of_mut!(pList);
        nList = 0 as i32;
        p = pList;
        '__slate_break_1528: loop {
            let __v1741: *mut PgHdr = p;
            unsafe {
                *ppNext = __v1741;
            }
            if !(__v1741 != std::ptr::null_mut::<PgHdr>()) {
                break;
            }
            if (unsafe { (*p).pgno }) <= nTruncate {
                ppNext = unsafe { std::ptr::addr_of_mut!((*p).pDirty) };
                let __v1742: i32 = nList;
                let __v1743: i32 = __v1742 + (1 as i32);
                nList = __v1743;
            }
            p = unsafe { (*p).pDirty };
        }
        0 as i32;
    } else {
        nList = 1 as i32;
    }
    let __v1744: *mut u32 =
        unsafe { unsafe { (*pPager).aStat.as_mut_ptr() as *mut u32 }.offset((2 as i32) as isize) };
    let __v1745: u32 = unsafe { *__v1744 };
    let __v1746: u32 = __v1745.wrapping_add(nList as u32);
    unsafe {
        *__v1744 = __v1746;
    }
    if (unsafe { (*pList).pgno }) == ((1 as i32) as u32) {
        pager_write_changecounter(pList);
    }
    rc = unsafe {
        sqlite3WalFrames(
            unsafe { (*pPager).pWal },
            (unsafe { (*pPager).pageSize }) as i32,
            pList,
            nTruncate,
            isCommit,
            ((unsafe { (*pPager).walSyncFlags }) as u32) as i32,
        )
    };
    if rc == (0 as i32) && (unsafe { (*pPager).pBackup }) != std::ptr::null_mut::<sqlite3_backup>()
    {
        p = pList;
        '__slate_break_1529: while p != std::ptr::null_mut::<PgHdr>() {
            unsafe {
                sqlite3BackupUpdate(
                    unsafe { (*pPager).pBackup },
                    unsafe { (*p).pgno },
                    ((unsafe { (*p).pData }) as *mut u8) as *const u8,
                )
            };
            p = unsafe { (*p).pDirty };
        }
    }
    return rc;
}

// /* Pager object */
// /* List of frames to log */
// /* Database size after this commit */
// /* True if this is a commit */
// /*
// ** Begin a read transaction on the WAL.
// **
// ** This routine used to be called "pagerOpenSnapshot()" because it essentially
// ** makes a snapshot of the database at the current point in time and preserves
// ** that snapshot for use by the reader in spite of concurrently changes by
// ** other writers or checkpointers.
// */
fn pagerBeginReadTransaction(mut pPager: *mut Pager) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* True if cache must be reset */
    let mut changed: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    // /* sqlite3WalEndReadTransaction() was not called for the previous
    //   ** transaction in locking_mode=EXCLUSIVE.  So call it now.  If we
    //   ** are in locking_mode=NORMAL and EndRead() was previously called,
    //   ** the duplicate call is harmless.
    //   */
    unsafe { sqlite3WalEndReadTransaction(unsafe { (*pPager).pWal }) };
    rc = unsafe {
        sqlite3WalBeginReadTransaction(unsafe { (*pPager).pWal }, std::ptr::addr_of_mut!(changed))
    };
    if rc != (0 as i32) || changed != (0 as i32) {
        pager_reset(pPager);
        if (unsafe { (*pPager).bUseFetch }) != (0 as u8) {
            unsafe {
                sqlite3OsUnfetch(
                    unsafe { (*pPager).fd },
                    (0 as i32) as i64,
                    std::ptr::null_mut::<()>(),
                )
            };
        }
    }
    return rc;
}

// /*
// ** This function is called as part of the transition from PAGER_OPEN
// ** to PAGER_READER state to determine the size of the database file
// ** in pages (assuming the page size currently stored in Pager.pageSize).
// **
// ** If no error occurs, SQLITE_OK is returned and the size of the database
// ** in pages is stored in *pnPage. Otherwise, an error code (perhaps
// ** SQLITE_IOERR_FSTAT) is returned and *pnPage is left unmodified.
// */
fn pagerPagecount(mut pPager: *mut Pager, mut pnPage: *mut u32) -> i32 {
    // /* Value to return via *pnPage */
    let mut nPage: u32 = 0 as u32;
    // /* Query the WAL sub-system for the database size. The WalDbsize()
    //   ** function returns zero if the WAL is not open (i.e. Pager.pWal==0), or
    //   ** if the database size is not available. The database size is not
    //   ** available from the WAL sub-system if the log file is empty or
    //   ** contains no valid committed transactions.
    //   */
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    nPage = unsafe { sqlite3WalDbsize(unsafe { (*pPager).pWal }) };
    // /* If the number of pages in the database is not available from the
    //   ** WAL sub-system, determine the page count based on the size of
    //   ** the database file.  If the size of the database file is not an
    //   ** integer multiple of the page-size, round up the result.
    //   */
    if nPage == ((0 as i32) as u32)
        && (unsafe { (*unsafe { (*pPager).fd }).pMethods })
            != std::ptr::null::<sqlite3_io_methods>()
    {
        // /* Size of db file in bytes */
        let mut n: i64 = (0 as i32) as i64;
        let mut rc: i32 =
            unsafe { sqlite3OsFileSize(unsafe { (*pPager).fd }, std::ptr::addr_of_mut!(n)) };
        if rc != (0 as i32) {
            return rc;
        }
        nPage = (((n + unsafe { (*pPager).pageSize } - ((1 as i32) as i64))
            / unsafe { (*pPager).pageSize }) as i32) as u32;
    }
    // /* If the current number of pages in the file is greater than the
    //   ** configured maximum pager number, increase the allowed limit so
    //   ** that the file can be read.
    //   */
    if nPage > unsafe { (*pPager).mxPgno } {
        unsafe {
            (*pPager).mxPgno = nPage;
        }
    }
    unsafe {
        *pnPage = nPage;
    }
    return 0 as i32;
}

// /*
// ** Check if the *-wal file that corresponds to the database opened by pPager
// ** exists if the database is not empty, or verify that the *-wal file does
// ** not exist (by deleting it) if the database file is empty.
// **
// ** If the database is not empty and the *-wal file exists, open the pager
// ** in WAL mode.  If the database is empty or if no *-wal file exists and
// ** if no error occurs, make sure Pager.journalMode is not set to
// ** PAGER_JOURNALMODE_WAL.
// **
// ** Return SQLITE_OK or an error code.
// **
// ** The caller must hold a SHARED lock on the database file to call this
// ** function. Because an EXCLUSIVE lock on the db file is required to delete
// ** a WAL on a none-empty database, this ensures there is no race condition
// ** between the xAccess() below and an xDelete() being executed by some
// ** other connection.
// */
fn pagerOpenWalIfPresent(mut pPager: *mut Pager) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    if !((unsafe { (*pPager).tempFile }) != (0 as u8)) {
        // /* True if WAL file exists */
        let mut isWal: i32 = 0 as i32;
        rc = unsafe {
            sqlite3OsAccess(
                unsafe { (*pPager).pVfs },
                (unsafe { (*pPager).zWal }) as *const i8,
                0 as i32,
                std::ptr::addr_of_mut!(isWal),
            )
        };
        if rc == (0 as i32) {
            if isWal != (0 as i32) {
                // /* Size of the database file */
                let mut nPage: u32 = 0 as u32;
                rc = pagerPagecount(pPager, std::ptr::addr_of_mut!(nPage));
                if rc != (0 as i32) {
                    return rc;
                }
                if nPage == ((0 as i32) as u32) {
                    rc = unsafe {
                        sqlite3OsDelete(
                            unsafe { (*pPager).pVfs },
                            (unsafe { (*pPager).zWal }) as *const i8,
                            0 as i32,
                        )
                    };
                } else {
                    {}
                    rc = sqlite3PagerOpenWal(pPager, std::ptr::null_mut::<i32>());
                }
            } else {
                if (((unsafe { (*pPager).journalMode }) as u32) as i32) == (5 as i32) {
                    unsafe {
                        (*pPager).journalMode = ((0 as i32) as i8) as u8;
                    }
                }
            }
        }
    }
    return rc;
}

// /*
// ** Playback savepoint pSavepoint. Or, if pSavepoint==NULL, then playback
// ** the entire super-journal file. The case pSavepoint==NULL occurs when
// ** a ROLLBACK TO command is invoked on a SAVEPOINT that is a transaction
// ** savepoint.
// **
// ** When pSavepoint is not NULL (meaning a non-transaction savepoint is
// ** being rolled back), then the rollback consists of up to three stages,
// ** performed in the order specified:
// **
// **   * Pages are played back from the main journal starting at byte
// **     offset PagerSavepoint.iOffset and continuing to
// **     PagerSavepoint.iHdrOffset, or to the end of the main journal
// **     file if PagerSavepoint.iHdrOffset is zero.
// **
// **   * If PagerSavepoint.iHdrOffset is not zero, then pages are played
// **     back starting from the journal header immediately following
// **     PagerSavepoint.iHdrOffset to the end of the main journal file.
// **
// **   * Pages are then played back from the sub-journal file, starting
// **     with the PagerSavepoint.iSubRec and continuing to the end of
// **     the journal file.
// **
// ** Throughout the rollback process, each time a page is rolled back, the
// ** corresponding bit is set in a bitvec structure (variable pDone in the
// ** implementation below). This is used to ensure that a page is only
// ** rolled back the first time it is encountered in either journal.
// **
// ** If pSavepoint is NULL, then pages are only played back from the main
// ** journal file. There is no need for a bitvec in this case.
// **
// ** In either case, before playback commences the Pager.dbSize variable
// ** is reset to the value that it held at the start of the savepoint
// ** (or transaction). No page with a page-number greater than this value
// ** is played back. If one is encountered it is simply skipped.
// */
fn pagerPlaybackSavepoint(mut pPager: *mut Pager, mut pSavepoint: *mut PagerSavepoint) -> i32 {
    // /* Effective size of the main journal */
    let mut szJ: i64 = 0 as i64;
    // /* End of first segment of main-journal records */
    let mut iHdrOff: i64 = 0 as i64;
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* Bitvec to ensure pages played back only once */
    let mut pDone: *mut Bitvec = std::ptr::null_mut::<Bitvec>();
    0 as i32;
    0 as i32;
    // /* Allocate a bitvec to use to store the set of pages rolled back */
    if pSavepoint != std::ptr::null_mut::<PagerSavepoint>() {
        pDone = unsafe { sqlite3BitvecCreate(unsafe { (*pSavepoint).nOrig }) };
        if !(pDone != std::ptr::null_mut::<Bitvec>()) {
            return 7 as i32;
        }
    }
    // /* Set the database size back to the value it was before the savepoint
    //   ** being reverted was opened.
    //   */
    unsafe {
        (*pPager).dbSize = if pSavepoint != std::ptr::null_mut::<PagerSavepoint>() {
            unsafe { (*pSavepoint).nOrig }
        } else {
            unsafe { (*pPager).dbOrigSize }
        };
    }
    unsafe {
        (*pPager).changeCountDone = unsafe { (*pPager).tempFile };
    }
    if !(pSavepoint != std::ptr::null_mut::<PagerSavepoint>())
        && (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>()
    {
        return pagerRollbackWal(pPager);
    }
    // /* Use pPager->journalOff as the effective size of the main rollback
    //   ** journal.  The actual file might be larger than this in
    //   ** PAGER_JOURNALMODE_TRUNCATE or PAGER_JOURNALMODE_PERSIST.  But anything
    //   ** past pPager->journalOff is off-limits to us.
    //   */
    szJ = unsafe { (*pPager).journalOff };
    0 as i32;
    // /* Begin by rolling back records from the main journal starting at
    //   ** PagerSavepoint.iOffset and continuing to the next journal header.
    //   ** There might be records in the main journal that have a page number
    //   ** greater than the current database size (pPager->dbSize) but those
    //   ** will be skipped automatically.  Pages are added to pDone as they
    //   ** are played back.
    //   */
    if pSavepoint != std::ptr::null_mut::<PagerSavepoint>()
        && !((unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>())
    {
        iHdrOff = if (unsafe { (*pSavepoint).iHdrOffset }) != (0 as i64) {
            unsafe { (*pSavepoint).iHdrOffset }
        } else {
            szJ
        };
        unsafe {
            (*pPager).journalOff = unsafe { (*pSavepoint).iOffset };
        }
        '__slate_break_1530: while rc == (0 as i32) && (unsafe { (*pPager).journalOff }) < iHdrOff {
            rc = pager_playback_one_page(
                pPager,
                unsafe { std::ptr::addr_of_mut!((*pPager).journalOff) },
                pDone,
                1 as i32,
                1 as i32,
            );
        }
        0 as i32;
    } else {
        unsafe {
            (*pPager).journalOff = (0 as i32) as i64;
        }
    }
    // /* Continue rolling back records out of the main journal starting at
    //   ** the first journal header seen and continuing until the effective end
    //   ** of the main journal file.  Continue to skip out-of-range pages and
    //   ** continue adding pages rolled back to pDone.
    //   */
    '__slate_break_1531: while rc == (0 as i32) && (unsafe { (*pPager).journalOff }) < szJ {
        // /* Loop counter */
        let mut ii: u32 = 0 as u32;
        // /* Number of Journal Records */
        let mut nJRec: u32 = (0 as i32) as u32;
        let mut dummy: u32 = 0 as u32;
        rc = readJournalHdr(
            pPager,
            0 as i32,
            szJ,
            std::ptr::addr_of_mut!(nJRec),
            std::ptr::addr_of_mut!(dummy),
        );
        0 as i32;
        // /*
        //     ** The "pPager->journalHdr+JOURNAL_HDR_SZ(pPager)==pPager->journalOff"
        //     ** test is related to ticket #2565.  See the discussion in the
        //     ** pager_playback() function for additional information.
        //     */
        if nJRec == ((0 as i32) as u32)
            && (unsafe { (*pPager).journalHdr })
                + (((unsafe { (*pPager).sectorSize }) as u64) as i64)
                == unsafe { (*pPager).journalOff }
        {
            nJRec = (((szJ - unsafe { (*pPager).journalOff })
                / ((unsafe { (*pPager).pageSize }) + ((8 as i32) as i64)))
                as i32) as u32;
        }
        ii = (0 as i32) as u32;
        '__slate_break_1532: while rc == (0 as i32)
            && ii < nJRec
            && (unsafe { (*pPager).journalOff }) < szJ
        {
            rc = pager_playback_one_page(
                pPager,
                unsafe { std::ptr::addr_of_mut!((*pPager).journalOff) },
                pDone,
                1 as i32,
                1 as i32,
            );
            let __v1747: u32 = ii;
            let __v1748: u32 = __v1747.wrapping_add((1 as i32) as u32);
            ii = __v1748;
        }
        0 as i32;
    }
    0 as i32;
    // /* Finally,  rollback pages from the sub-journal.  Page that were
    //   ** previously rolled back out of the main journal (and are hence in pDone)
    //   ** will be skipped.  Out-of-range pages are also skipped.
    //   */
    if pSavepoint != std::ptr::null_mut::<PagerSavepoint>() {
        // /* Loop counter */
        let mut ii: u32 = 0 as u32;
        let mut offset: i64 = (((unsafe { (*pSavepoint).iSubRec }) as u64) as i64)
            * (((4 as i32) as i64) + unsafe { (*pPager).pageSize });
        if (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
            rc = unsafe {
                sqlite3WalSavepointUndo(unsafe { (*pPager).pWal }, unsafe {
                    (*pSavepoint).aWalData.as_mut_ptr() as *mut u32
                })
            };
        }
        ii = unsafe { (*pSavepoint).iSubRec };
        '__slate_break_1533: while rc == (0 as i32) && ii < unsafe { (*pPager).nSubRec } {
            0 as i32;
            rc = pager_playback_one_page(
                pPager,
                std::ptr::addr_of_mut!(offset),
                pDone,
                0 as i32,
                1 as i32,
            );
            let __v1749: u32 = ii;
            let __v1750: u32 = __v1749.wrapping_add((1 as i32) as u32);
            ii = __v1750;
        }
        0 as i32;
    }
    unsafe { sqlite3BitvecDestroy(pDone) };
    if rc == (0 as i32) {
        unsafe {
            (*pPager).journalOff = szJ;
        }
    }
    return rc;
}

// /*
// ** Invoke SQLITE_FCNTL_MMAP_SIZE based on the current value of szMmap.
// */
fn pagerFixMaplimit(mut pPager: *mut Pager) {
    let mut fd: *mut sqlite3_file = unsafe { (*pPager).fd };
    if (unsafe { (*fd).pMethods }) != std::ptr::null::<sqlite3_io_methods>()
        && (unsafe { (*unsafe { (*fd).pMethods }).iVersion }) >= (3 as i32)
    {
        let mut sz: i64 = 0 as i64;
        sz = unsafe { (*pPager).szMmap };
        unsafe {
            (*pPager).bUseFetch = (sz > ((0 as i32) as i64)) as u8;
        }
        setGetterMethod(pPager);
        unsafe {
            sqlite3OsFileControlHint(
                unsafe { (*pPager).fd },
                18 as i32,
                std::ptr::addr_of_mut!(sz) as *mut (),
            )
        };
    }
}

// /* The pager to set safety level for */
// /* Various flags */
// /*
// ** The following global variable is incremented whenever the library
// ** attempts to open a temporary file.  This information is used for
// ** testing and analysis only.
// */
// /*
// ** Open a temporary file.
// **
// ** Write the file descriptor into *pFile. Return SQLITE_OK on success
// ** or some other error code if we fail. The OS will automatically
// ** delete the temporary file when it is closed.
// **
// ** The flags passed to the VFS layer xOpen() call are those specified
// ** by parameter vfsFlags ORed with the following:
// **
// **     SQLITE_OPEN_READWRITE
// **     SQLITE_OPEN_CREATE
// **     SQLITE_OPEN_EXCLUSIVE
// **     SQLITE_OPEN_DELETEONCLOSE
// */
fn pagerOpentemp(mut pPager: *mut Pager, mut pFile: *mut sqlite3_file, mut vfsFlags: i32) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    let __v1751: i32 = vfsFlags;
    let __v1752: i32 = __v1751 | ((2 as i32) | (4 as i32) | (16 as i32) | (8 as i32));
    vfsFlags = __v1752;
    rc = unsafe {
        sqlite3OsOpen(
            unsafe { (*pPager).pVfs },
            std::ptr::null::<i8>(),
            pFile,
            vfsFlags,
            std::ptr::null_mut::<i32>(),
        )
    };
    0 as i32;
    return rc;
}

// /*
// ** Try to obtain a lock of type locktype on the database file. If
// ** a similar or greater lock is already held, this function is a no-op
// ** (returning SQLITE_OK immediately).
// **
// ** Otherwise, attempt to obtain the lock using sqlite3OsLock(). Invoke
// ** the busy callback if the lock is currently not available. Repeat
// ** until the busy callback returns false or until the attempt to
// ** obtain the lock succeeds.
// **
// ** Return SQLITE_OK on success and an error code if we cannot obtain
// ** the lock. If the lock is obtained successfully, set the Pager.state
// ** variable to locktype before returning.
// */
fn pager_wait_on_lock(mut pPager: *mut Pager, mut locktype: i32) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* Check that this is either a no-op (because the requested lock is
    //   ** already held), or one of the transitions that the busy-handler
    //   ** may be invoked during, according to the comment above
    //   ** sqlite3PagerSetBusyhandler().
    //   */
    0 as i32;
    '__slate_break_1534: loop {
        rc = pagerLockDb(pPager, locktype);
        let __v1753: bool;
        if rc == (5 as i32) {
            __v1753 = (unsafe {
                unsafe { (*pPager).xBusyHandler }.unwrap()(unsafe { (*pPager).pBusyHandlerArg })
            }) != (0 as i32);
        } else {
            __v1753 = false as bool;
        }
        if !__v1753 {
            break;
        }
    }
    return rc;
}

// /*
// ** This function is called before attempting a hot-journal rollback. It
// ** syncs the journal file to disk, then sets pPager->journalHdr to the
// ** size of the journal file so that the pager_playback() routine knows
// ** that the entire journal file has been synced.
// **
// ** Syncing a hot-journal to disk before attempting to roll it back ensures
// ** that if a power-failure occurs during the rollback, the process that
// ** attempts rollback following system recovery sees the same journal
// ** content as this process.
// **
// ** If everything goes as planned, SQLITE_OK is returned. Otherwise,
// ** an SQLite error code.
// */
fn pagerSyncHotJournal(mut pPager: *mut Pager) -> i32 {
    let mut rc: i32 = 0 as i32;
    if !((unsafe { (*pPager).noSync }) != (0 as u8)) {
        rc = unsafe { sqlite3OsSync(unsafe { (*pPager).jfd }, 2 as i32) };
    }
    if rc == (0 as i32) {
        rc = unsafe {
            sqlite3OsFileSize(unsafe { (*pPager).jfd }, unsafe {
                std::ptr::addr_of_mut!((*pPager).journalHdr)
            })
        };
    }
    return rc;
}

// /*
// ** Obtain a reference to a memory mapped page object for page number pgno.
// ** The new object will use the pointer pData, obtained from xFetch().
// ** If successful, set *ppPage to point to the new page reference
// ** and return SQLITE_OK. Otherwise, return an SQLite error code and set
// ** *ppPage to zero.
// **
// ** Page references obtained by calling this function should be released
// ** by calling pagerReleaseMapPage().
// */
fn pagerAcquireMapPage(
    mut pPager: *mut Pager,
    mut pgno: u32,
    mut pData: *mut (),
    mut ppPage: *mut *mut PgHdr,
) -> i32 {
    // /* Memory mapped page to return */
    let mut p: *mut PgHdr = unsafe { std::mem::zeroed() };
    if (unsafe { (*pPager).pMmapFreelist }) != std::ptr::null_mut::<PgHdr>() {
        let __v1754: *mut PgHdr = unsafe { (*pPager).pMmapFreelist };
        p = __v1754;
        unsafe {
            *ppPage = __v1754;
        }
        unsafe {
            (*pPager).pMmapFreelist = unsafe { (*p).pDirty };
        }
        unsafe {
            (*p).pDirty = std::ptr::null_mut::<PgHdr>();
        }
        0 as i32;
        unsafe { memset(unsafe { (*p).pExtra }, 0 as i32, ((8 as i32) as i64) as u64) };
    } else {
        let __v1755: *mut PgHdr = (unsafe {
            sqlite3MallocZero(
                (80 as u64)
                    .wrapping_add(((((unsafe { (*pPager).nExtra }) as u32) as i32) as i64) as u64),
            )
        }) as *mut PgHdr;
        p = __v1755;
        unsafe {
            *ppPage = __v1755;
        }
        if p == std::ptr::null_mut::<PgHdr>() {
            unsafe {
                sqlite3OsUnfetch(
                    unsafe { (*pPager).fd },
                    ((pgno.wrapping_sub((1 as i32) as u32) as u64) as i64)
                        * unsafe { (*pPager).pageSize },
                    pData,
                )
            };
            return 7 as i32;
        }
        unsafe {
            (*p).pExtra = (unsafe { p.offset((1 as i32) as isize) }) as *mut ();
        }
        0 as i32;
        unsafe {
            (*p).flags = ((32 as i32) as i16) as u16;
        }
        unsafe {
            (*p).nRef = (1 as i32) as i64;
        }
        unsafe {
            (*p).pPager = pPager;
        }
    }
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe {
        (*p).pgno = pgno;
    }
    unsafe {
        (*p).pData = pData;
    }
    let __v1756: *mut Pager = pPager;
    let __v1757: i32 = unsafe { (*__v1756).nMmapOut };
    let __v1758: i32 = __v1757 + (1 as i32);
    unsafe {
        (*__v1756).nMmapOut = __v1758;
    }
    return 0 as i32;
}

// /* Pager object */
// /* Page number */
// /* xFetch()'d data for this page */
// /* OUT: Acquired page object */
// /*
// ** Release a reference to page pPg. pPg must have been returned by an
// ** earlier call to pagerAcquireMapPage().
// */
fn pagerReleaseMapPage(mut pPg: *mut PgHdr) {
    let mut pPager: *mut Pager = unsafe { (*pPg).pPager };
    let __v1759: *mut Pager = pPager;
    let __v1760: i32 = unsafe { (*__v1759).nMmapOut };
    let __v1761: i32 = __v1760 - (1 as i32);
    unsafe {
        (*__v1759).nMmapOut = __v1761;
    }
    unsafe {
        (*pPg).pDirty = unsafe { (*pPager).pMmapFreelist };
    }
    unsafe {
        (*pPager).pMmapFreelist = pPg;
    }
    0 as i32;
    unsafe {
        sqlite3OsUnfetch(
            unsafe { (*pPager).fd },
            ((unsafe { (*pPg).pgno }.wrapping_sub((1 as i32) as u32) as u64) as i64)
                * unsafe { (*pPager).pageSize },
            unsafe { (*pPg).pData },
        )
    };
}

// /*
// ** Free all PgHdr objects stored in the Pager.pMmapFreelist list.
// */
fn pagerFreeMapHdrs(mut pPager: *mut Pager) {
    let mut p: *mut PgHdr = unsafe { std::mem::zeroed() };
    let mut pNext: *mut PgHdr = unsafe { std::mem::zeroed() };
    p = unsafe { (*pPager).pMmapFreelist };
    '__slate_break_1535: while p != std::ptr::null_mut::<PgHdr>() {
        pNext = unsafe { (*p).pDirty };
        unsafe { sqlite3_free(p as *mut ()) };
        p = pNext;
    }
}

// /* Verify that the database file has not be deleted or renamed out from
// ** under the pager.  Return SQLITE_OK if the database is still where it ought
// ** to be on disk.  Return non-zero (SQLITE_READONLY_DBMOVED or some other error
// ** code from sqlite3OsAccess()) if the database has gone missing.
// */
fn databaseIsUnmoved(mut pPager: *mut Pager) -> i32 {
    let mut bHasMoved: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pPager).tempFile }) != (0 as u8) {
        return 0 as i32;
    }
    if (unsafe { (*pPager).dbSize }) == ((0 as i32) as u32) {
        return 0 as i32;
    }
    0 as i32;
    rc = unsafe {
        sqlite3OsFileControl(
            unsafe { (*pPager).fd },
            20 as i32,
            std::ptr::addr_of_mut!(bHasMoved) as *mut (),
        )
    };
    if rc == (12 as i32) {
        // /* If the HAS_MOVED file-control is unimplemented, assume that the file
        //     ** has not been moved.  That is the historical behavior of SQLite: prior to
        //     ** version 3.8.3, it never checked */
        rc = 0 as i32;
    } else {
        if rc == (0 as i32) && bHasMoved != (0 as i32) {
            rc = (8 as i32) | (4 as i32) << (8 as i32);
        }
    }
    return rc;
}

// /*
// ** Sync the journal. In other words, make sure all the pages that have
// ** been written to the journal have actually reached the surface of the
// ** disk and can be restored in the event of a hot-journal rollback.
// **
// ** If the Pager.noSync flag is set, then this function is a no-op.
// ** Otherwise, the actions required depend on the journal-mode and the
// ** device characteristics of the file-system, as follows:
// **
// **   * If the journal file is an in-memory journal file, no action need
// **     be taken.
// **
// **   * Otherwise, if the device does not support the SAFE_APPEND property,
// **     then the nRec field of the most recently written journal header
// **     is updated to contain the number of journal records that have
// **     been written following it. If the pager is operating in full-sync
// **     mode, then the journal file is synced before this field is updated.
// **
// **   * If the device does not support the SEQUENTIAL property, then
// **     journal file is synced.
// **
// ** Or, in pseudo-code:
// **
// **   if( NOT <in-memory journal> ){
// **     if( NOT SAFE_APPEND ){
// **       if( <full-sync mode> ) xSync(<journal file>);
// **       <update nRec field>
// **     }
// **     if( NOT SEQUENTIAL ) xSync(<journal file>);
// **   }
// **
// ** If successful, this routine clears the PGHDR_NEED_SYNC flag of every
// ** page currently held in memory before returning SQLITE_OK. If an IO
// ** error is encountered, then the IO error code is returned to the caller.
// */
fn syncJournal(mut pPager: *mut Pager, mut newHdr: i32) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    rc = sqlite3PagerExclusiveLock(pPager);
    if rc != (0 as i32) {
        return rc;
    }
    if !((unsafe { (*pPager).noSync }) != (0 as u8)) {
        0 as i32;
        if (unsafe { (*unsafe { (*pPager).jfd }).pMethods })
            != std::ptr::null::<sqlite3_io_methods>()
            && (((unsafe { (*pPager).journalMode }) as u32) as i32) != (4 as i32)
        {
            let mut iDc: i32 = unsafe { sqlite3OsDeviceCharacteristics(unsafe { (*pPager).fd }) };
            0 as i32;
            if (0 as i32) == iDc & (512 as i32) {
                // /* This block deals with an obscure problem. If the last connection
                //         ** that wrote to this database was operating in persistent-journal
                //         ** mode, then the journal file may at this point actually be larger
                //         ** than Pager.journalOff bytes. If the next thing in the journal
                //         ** file happens to be a journal-header (written as part of the
                //         ** previous connection's transaction), and a crash or power-failure
                //         ** occurs after nRec is updated but before this connection writes
                //         ** anything else to the journal file (or commits/rolls back its
                //         ** transaction), then SQLite may become confused when doing the
                //         ** hot-journal rollback following recovery. It may roll back all
                //         ** of this connections data, then proceed to rolling back the old,
                //         ** out-of-date data that follows it. Database corruption.
                //         **
                //         ** To work around this, if the journal file does appear to contain
                //         ** a valid header following Pager.journalOff, then write a 0x00
                //         ** byte to the start of it to prevent it from being recognized.
                //         **
                //         ** Variable iNextHdrOffset is set to the offset at which this
                //         ** problematic header will occur, if it exists. aMagic is used
                //         ** as a temporary buffer to inspect the first couple of bytes of
                //         ** the potential journal header.
                //         */
                let mut iNextHdrOffset: i64 = 0 as i64;
                let mut aMagic: [u8; 8] = [0 as u8; 8];
                let mut zHeader: [u8; 12] = [0 as u8; 12];
                unsafe {
                    memcpy(
                        (zHeader.as_mut_ptr() as *mut u8) as *mut (),
                        (unsafe { std::ptr::addr_of!(aJournalMagic) as *const u8 }) as *const (),
                        8 as u64,
                    )
                };
                unsafe {
                    sqlite3Put4byte(
                        unsafe { (zHeader.as_mut_ptr() as *mut u8).offset((8 as u64) as isize) },
                        (unsafe { (*pPager).nRec }) as u32,
                    )
                };
                iNextHdrOffset = journalHdrOffset(pPager);
                rc = unsafe {
                    sqlite3OsRead(
                        unsafe { (*pPager).jfd },
                        (aMagic.as_mut_ptr() as *mut u8) as *mut (),
                        8 as i32,
                        iNextHdrOffset,
                    )
                };
                if rc == (0 as i32)
                    && (0 as i32)
                        == unsafe {
                            memcmp(
                                (aMagic.as_mut_ptr() as *mut u8) as *const (),
                                (unsafe { std::ptr::addr_of!(aJournalMagic) as *const u8 })
                                    as *const (),
                                ((8 as i32) as i64) as u64,
                            )
                        }
                {
                    rc = unsafe {
                        sqlite3OsWrite(
                            unsafe { (*pPager).jfd },
                            (unsafe { std::ptr::addr_of!(zerobyte) }) as *const (),
                            1 as i32,
                            iNextHdrOffset,
                        )
                    };
                }
                if rc != (0 as i32) && rc != (10 as i32) | (2 as i32) << (8 as i32) {
                    return rc;
                }
                // /* Write the nRec value into the journal file header. If in
                //         ** full-synchronous mode, sync the journal first. This ensures that
                //         ** all data has really hit the disk before nRec is updated to mark
                //         ** it as a candidate for rollback.
                //         **
                //         ** This is not required if the persistent media supports the
                //         ** SAFE_APPEND property. Because in this case it is not possible
                //         ** for garbage data to be appended to the file, the nRec field
                //         ** is populated with 0xFFFFFFFF when the journal header is written
                //         ** and never needs to be updated.
                //         */
                if (unsafe { (*pPager).fullSync }) != (0 as u8) && (0 as i32) == iDc & (1024 as i32)
                {
                    {}
                    rc = unsafe {
                        sqlite3OsSync(
                            unsafe { (*pPager).jfd },
                            ((unsafe { (*pPager).syncFlags }) as u32) as i32,
                        )
                    };
                    if rc != (0 as i32) {
                        return rc;
                    }
                }
                {}
                rc = unsafe {
                    sqlite3OsWrite(
                        unsafe { (*pPager).jfd },
                        (zHeader.as_mut_ptr() as *mut u8) as *const (),
                        ((12 as u64) as u32) as i32,
                        unsafe { (*pPager).journalHdr },
                    )
                };
                if rc != (0 as i32) {
                    return rc;
                }
            }
            if (0 as i32) == iDc & (1024 as i32) {
                {}
                rc = unsafe {
                    sqlite3OsSync(
                        unsafe { (*pPager).jfd },
                        (((unsafe { (*pPager).syncFlags }) as u32) as i32)
                            | if (((unsafe { (*pPager).syncFlags }) as u32) as i32) == (3 as i32) {
                                16 as i32
                            } else {
                                0 as i32
                            },
                    )
                };
                if rc != (0 as i32) {
                    return rc;
                }
            }
            unsafe {
                (*pPager).journalHdr = unsafe { (*pPager).journalOff };
            }
            if newHdr != (0 as i32) && (0 as i32) == iDc & (512 as i32) {
                unsafe {
                    (*pPager).nRec = 0 as i32;
                }
                rc = writeJournalHdr(pPager);
                if rc != (0 as i32) {
                    return rc;
                }
            }
        } else {
            unsafe {
                (*pPager).journalHdr = unsafe { (*pPager).journalOff };
            }
        }
    }
    // /* Unless the pager is in noSync mode, the journal file was just
    //   ** successfully synced. Either way, clear the PGHDR_NEED_SYNC flag on
    //   ** all pages.
    //   */
    unsafe { sqlite3PcacheClearSyncFlags(unsafe { (*pPager).pPCache }) };
    unsafe {
        (*pPager).eState = ((4 as i32) as i8) as u8;
    }
    0 as i32;
    return 0 as i32;
}

// /*
// ** The argument is the first in a linked list of dirty pages connected
// ** by the PgHdr.pDirty pointer. This function writes each one of the
// ** in-memory pages in the list to the database file. The argument may
// ** be NULL, representing an empty list. In this case this function is
// ** a no-op.
// **
// ** The pager must hold at least a RESERVED lock when this function
// ** is called. Before writing anything to the database file, this lock
// ** is upgraded to an EXCLUSIVE lock. If the lock cannot be obtained,
// ** SQLITE_BUSY is returned and no data is written to the database file.
// **
// ** If the pager is a temp-file pager and the actual file-system file
// ** is not yet open, it is created and opened before any data is
// ** written out.
// **
// ** Once the lock has been upgraded and, if necessary, the file opened,
// ** the pages are written out to the database file in list order. Writing
// ** a page is skipped if it meets either of the following criteria:
// **
// **   * The page number is greater than Pager.dbSize, or
// **   * The PGHDR_DONT_WRITE flag is set on the page.
// **
// ** If writing out a page causes the database file to grow, Pager.dbFileSize
// ** is updated accordingly. If page 1 is written out, then the value cached
// ** in Pager.dbFileVers[] is updated to match the new value stored in
// ** the database file.
// **
// ** If everything is successful, SQLITE_OK is returned. If an IO error
// ** occurs, an IO error code is returned. Or, if the EXCLUSIVE lock cannot
// ** be obtained, SQLITE_BUSY is returned.
// */
fn pager_write_pagelist(mut pPager: *mut Pager, mut pList: *mut PgHdr) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* This function is only called for rollback pagers in WRITER_DBMOD state. */
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    // /* If the file is a temp-file has not yet been opened, open it now. It
    //   ** is not possible for rc to be other than SQLITE_OK if this branch
    //   ** is taken, as pager_wait_on_lock() is a no-op for temp-files.
    //   */
    if !((unsafe { (*unsafe { (*pPager).fd }).pMethods }) != std::ptr::null::<sqlite3_io_methods>())
    {
        0 as i32;
        rc = pagerOpentemp(
            pPager,
            unsafe { (*pPager).fd },
            (unsafe { (*pPager).vfsFlags }) as i32,
        );
    }
    // /* Before the first write, give the VFS a hint of what the final
    //   ** file size will be.
    //   */
    0 as i32;
    if rc == (0 as i32)
        && (unsafe { (*pPager).dbHintSize }) < unsafe { (*pPager).dbSize }
        && ((unsafe { (*pList).pDirty }) != std::ptr::null_mut::<PgHdr>()
            || (unsafe { (*pList).pgno }) > unsafe { (*pPager).dbHintSize })
    {
        let mut szFile: i64 =
            (unsafe { (*pPager).pageSize }) * (((unsafe { (*pPager).dbSize }) as u64) as i64);
        unsafe {
            sqlite3OsFileControlHint(
                unsafe { (*pPager).fd },
                5 as i32,
                std::ptr::addr_of_mut!(szFile) as *mut (),
            )
        };
        unsafe {
            (*pPager).dbHintSize = unsafe { (*pPager).dbSize };
        }
    }
    '__slate_break_1536: while rc == (0 as i32) && pList != std::ptr::null_mut::<PgHdr>() {
        let mut pgno: u32 = unsafe { (*pList).pgno };
        // /* If there are dirty pages in the page cache with page numbers greater
        //     ** than Pager.dbSize, this means sqlite3PagerTruncateImage() was called to
        //     ** make the file smaller (presumably by auto-vacuum code). Do not write
        //     ** any such pages to the file.
        //     **
        //     ** Also, do not write out any page that has the PGHDR_DONT_WRITE flag
        //     ** set (set by sqlite3PagerDontWrite()).
        //     */
        if pgno <= unsafe { (*pPager).dbSize }
            && (0 as i32) == (((unsafe { (*pList).flags }) as u32) as i32) & (16 as i32)
        {
            // /* Offset to write */
            let mut offset: i64 = ((pgno.wrapping_sub((1 as i32) as u32) as u64) as i64)
                * unsafe { (*pPager).pageSize };
            // /* Data to write */
            let mut pData: *mut i8 = unsafe { std::mem::zeroed() };
            0 as i32;
            if (unsafe { (*pList).pgno }) == ((1 as i32) as u32) {
                pager_write_changecounter(pList);
            }
            pData = (unsafe { (*pList).pData }) as *mut i8;
            // /* Write out the page data. */
            rc = unsafe {
                sqlite3OsWrite(
                    unsafe { (*pPager).fd },
                    pData as *const (),
                    (unsafe { (*pPager).pageSize }) as i32,
                    offset,
                )
            };
            // /* If page 1 was just written, update Pager.dbFileVers to match
            //       ** the value now stored in the database file. If writing this
            //       ** page caused the database file to grow, update dbFileSize.
            //       */
            if pgno == ((1 as i32) as u32) {
                unsafe {
                    memcpy(
                        (unsafe { std::ptr::addr_of_mut!((*pPager).dbFileVers) }) as *mut (),
                        (unsafe { pData.offset((24 as i32) as isize) }) as *const (),
                        16 as u64,
                    )
                };
            }
            if pgno > unsafe { (*pPager).dbFileSize } {
                unsafe {
                    (*pPager).dbFileSize = pgno;
                }
            }
            let __v1762: *mut u32 = unsafe {
                unsafe { (*pPager).aStat.as_mut_ptr() as *mut u32 }.offset((2 as i32) as isize)
            };
            let __v1763: u32 = unsafe { *__v1762 };
            let __v1764: u32 = __v1763.wrapping_add((1 as i32) as u32);
            unsafe {
                *__v1762 = __v1764;
            }
            // /* Update any backup objects copying the contents of this pager. */
            unsafe {
                sqlite3BackupUpdate(
                    unsafe { (*pPager).pBackup },
                    pgno,
                    ((unsafe { (*pList).pData }) as *mut u8) as *const u8,
                )
            };
            {}
            {}
            {}
        } else {
            {}
        }
        {}
        pList = unsafe { (*pList).pDirty };
    }
    return rc;
}

// /*
// ** Ensure that the sub-journal file is open. If it is already open, this
// ** function is a no-op.
// **
// ** SQLITE_OK is returned if everything goes according to plan. An
// ** SQLITE_IOERR_XXX error code is returned if a call to sqlite3OsOpen()
// ** fails.
// */
fn openSubJournal(mut pPager: *mut Pager) -> i32 {
    let mut rc: i32 = 0 as i32;
    if !((unsafe { (*unsafe { (*pPager).sjfd }).pMethods })
        != std::ptr::null::<sqlite3_io_methods>())
    {
        let mut flags: i32 = (8192 as i32) | (2 as i32) | (4 as i32) | (16 as i32) | (8 as i32);
        let mut nStmtSpill: i32 = unsafe { sqlite3Config.nStmtSpill };
        if (((unsafe { (*pPager).journalMode }) as u32) as i32) == (4 as i32)
            || (unsafe { (*pPager).subjInMemory }) != (0 as u8)
        {
            nStmtSpill = -(1 as i32);
        }
        rc = unsafe {
            sqlite3JournalOpen(
                unsafe { (*pPager).pVfs },
                std::ptr::null::<i8>(),
                unsafe { (*pPager).sjfd },
                flags,
                nStmtSpill,
            )
        };
    }
    return rc;
}

// /*
// ** Append a record of the current state of page pPg to the sub-journal.
// **
// ** If successful, set the bit corresponding to pPg->pgno in the bitvecs
// ** for all open savepoints before returning.
// **
// ** This function returns SQLITE_OK if everything is successful, an IO
// ** error code if the attempt to write to the sub-journal fails, or
// ** SQLITE_NOMEM if a malloc fails while setting a bit in a savepoint
// ** bitvec.
// */
fn subjournalPage(mut pPg: *mut PgHdr) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pPager: *mut Pager = unsafe { (*pPg).pPager };
    if (((unsafe { (*pPager).journalMode }) as u32) as i32) != (2 as i32) {
        // /* Open the sub-journal, if it has not already been opened */
        0 as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        rc = openSubJournal(pPager);
        // /* If the sub-journal was opened successfully (or was already open),
        //     ** write the journal record into the file.  */
        if rc == (0 as i32) {
            let mut pData: *mut () = unsafe { (*pPg).pData };
            let mut offset: i64 = (((unsafe { (*pPager).nSubRec }) as u64) as i64)
                * (((4 as i32) as i64) + unsafe { (*pPager).pageSize });
            let mut pData2: *mut i8 = unsafe { std::mem::zeroed() };
            pData2 = pData as *mut i8;
            {}
            rc = write32bits(unsafe { (*pPager).sjfd }, offset, unsafe { (*pPg).pgno });
            if rc == (0 as i32) {
                rc = unsafe {
                    sqlite3OsWrite(
                        unsafe { (*pPager).sjfd },
                        pData2 as *const (),
                        (unsafe { (*pPager).pageSize }) as i32,
                        offset + ((4 as i32) as i64),
                    )
                };
            }
        }
    }
    if rc == (0 as i32) {
        let __v1765: *mut Pager = pPager;
        let __v1766: u32 = unsafe { (*__v1765).nSubRec };
        let __v1767: u32 = __v1766.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v1765).nSubRec = __v1767;
        }
        0 as i32;
        rc = addToSavepointBitvecs(pPager, unsafe { (*pPg).pgno });
    }
    return rc;
}

fn subjournalPageIfRequired(mut pPg: *mut PgHdr) -> i32 {
    if subjRequiresPage(pPg) != (0 as i32) {
        return subjournalPage(pPg);
    } else {
        return 0 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** This function is called by the pcache layer when it has reached some
// ** soft memory limit. The first argument is a pointer to a Pager object
// ** (cast as a void*). The pager is always 'purgeable' (not an in-memory
// ** database). The second argument is a reference to a page that is
// ** currently dirty but has no outstanding references. The page
// ** is always associated with the Pager object passed as the first
// ** argument.
// **
// ** The job of this function is to make pPg clean by writing its contents
// ** out to the database file, if possible. This may involve syncing the
// ** journal file.
// **
// ** If successful, sqlite3PcacheMakeClean() is called on the page and
// ** SQLITE_OK returned. If an IO error occurs while trying to make the
// ** page clean, the IO error code is returned. If the page cannot be
// ** made clean for some other reason, but no error occurs, then SQLITE_OK
// ** is returned by sqlite3PcacheMakeClean() is not called.
// */
#[unsafe(link_section = ".text.slate_distinct.pager.pagerStress")]
extern "C-unwind" fn pagerStress(mut p: *mut (), mut pPg: *mut PgHdr) -> i32 {
    let mut pPager: *mut Pager = p as *mut Pager;
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    // /* The doNotSpill NOSYNC bit is set during times when doing a sync of
    //   ** journal (and adding a new header) is not allowed.  This occurs
    //   ** during calls to sqlite3PagerWrite() while trying to journal multiple
    //   ** pages belonging to the same sector.
    //   **
    //   ** The doNotSpill ROLLBACK and OFF bits inhibits all cache spilling
    //   ** regardless of whether or not a sync is required.  This is set during
    //   ** a rollback or by user request, respectively.
    //   **
    //   ** Spilling is also prohibited when in an error state since that could
    //   ** lead to database corruption.   In the current implementation it
    //   ** is impossible for sqlite3PcacheFetch() to be called with createFlag==3
    //   ** while in the error state, hence it is impossible for this routine to
    //   ** be called in the error state.  Nevertheless, we include a NEVER()
    //   ** test for the error state as a safeguard against future changes.
    //   */
    if (unsafe { (*pPager).errCode }) != (0 as i32) {
        return 0 as i32;
    }
    {}
    {}
    {}
    if (unsafe { (*pPager).doNotSpill }) != (0 as u8)
        && ((((unsafe { (*pPager).doNotSpill }) as u32) as i32) & ((2 as i32) | (1 as i32))
            != (0 as i32)
            || (((unsafe { (*pPg).flags }) as u32) as i32) & (8 as i32) != (0 as i32))
    {
        return 0 as i32;
    }
    let __v1768: *mut u32 =
        unsafe { unsafe { (*pPager).aStat.as_mut_ptr() as *mut u32 }.offset((3 as i32) as isize) };
    let __v1769: u32 = unsafe { *__v1768 };
    let __v1770: u32 = __v1769.wrapping_add((1 as i32) as u32);
    unsafe {
        *__v1768 = __v1770;
    }
    unsafe {
        (*pPg).pDirty = std::ptr::null_mut::<PgHdr>();
    }
    if (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
        // /* Write a single frame for this page to the log. */
        rc = subjournalPageIfRequired(pPg);
        if rc == (0 as i32) {
            rc = pagerWalFrames(pPager, pPg, (0 as i32) as u32, 0 as i32);
        }
    } else {
        // /* Sync the journal file if required. */
        if (((unsafe { (*pPg).flags }) as u32) as i32) & (8 as i32) != (0 as i32)
            || (((unsafe { (*pPager).eState }) as u32) as i32) == (3 as i32)
        {
            rc = syncJournal(pPager, 1 as i32);
        }
        // /* Write the contents of the page out to the database file. */
        if rc == (0 as i32) {
            0 as i32;
            rc = pager_write_pagelist(pPager, pPg);
        }
    }
    // /* Mark the page as clean. */
    if rc == (0 as i32) {
        {}
        unsafe { sqlite3PcacheMakeClean(pPg) };
    }
    return pager_error(pPager, rc);
}

// /*
// ** This function is called while transitioning from PAGER_OPEN to a
// ** higher state. It tests if there is a hot journal present in
// ** the file-system for the given pager. A hot journal is one that
// ** needs to be played back. According to this function, a hot-journal
// ** file exists if the following criteria are met:
// **
// **   * The journal file exists in the file system, and
// **   * No process holds a RESERVED or greater lock on the database file, and
// **   * The database file itself is greater than 0 bytes in size, and
// **   * The first byte of the journal file exists and is not 0x00.
// **
// ** If the current size of the database file is 0 but a journal file
// ** exists, that is probably an old journal left over from a prior
// ** database with the same name. In this case the journal file is
// ** just deleted using OsDelete, *pExists is set to 0 and SQLITE_OK
// ** is returned.
// **
// ** This routine does not check if there is a super-journal filename
// ** at the end of the file. If there is, and that super-journal file
// ** does not exist, then the journal file is not really hot. In this
// ** case this routine will return a false-positive. The pager_playback()
// ** routine will discover that the journal file is not really hot and
// ** will not roll it back.
// **
// ** If a hot-journal file is found to exist, *pExists is set to 1 and
// ** SQLITE_OK returned. If no hot-journal file is present, *pExists is
// ** set to 0 and SQLITE_OK returned. If an IO error occurs while trying
// ** to determine whether or not a hot-journal file exists, the IO error
// ** code is returned and the value of *pExists is undefined.
// */
fn hasHotJournal(mut pPager: *mut Pager, mut pExists: *mut i32) -> i32 {
    let mut pVfs: *mut sqlite3_vfs = unsafe { (*pPager).pVfs };
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* True if a journal file is present */
    let mut exists: i32 = 1 as i32;
    let mut jrnlOpen: i32 = !(!((unsafe { (*unsafe { (*pPager).jfd }).pMethods })
        != std::ptr::null::<sqlite3_io_methods>())) as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe {
        *pExists = 0 as i32;
    }
    if !(jrnlOpen != (0 as i32)) {
        rc = unsafe {
            sqlite3OsAccess(
                pVfs,
                (unsafe { (*pPager).zJournal }) as *const i8,
                0 as i32,
                std::ptr::addr_of_mut!(exists),
            )
        };
    }
    if rc == (0 as i32) && exists != (0 as i32) {
        // /* True if some process holds a RESERVED lock */
        let mut locked: i32 = 0 as i32;
        // /* Race condition here:  Another process might have been holding the
        //     ** the RESERVED lock and have a journal open at the sqlite3OsAccess()
        //     ** call above, but then delete the journal and drop the lock before
        //     ** we get to the following sqlite3OsCheckReservedLock() call.  If that
        //     ** is the case, this routine might think there is a hot journal when
        //     ** in fact there is none.  This results in a false-positive which will
        //     ** be dealt with by the playback routine.  Ticket #3883.
        //     */
        rc = unsafe {
            sqlite3OsCheckReservedLock(unsafe { (*pPager).fd }, std::ptr::addr_of_mut!(locked))
        };
        if rc == (0 as i32) && !(locked != (0 as i32)) {
            // /* Number of pages in database file */
            let mut nPage: u32 = 0 as u32;
            0 as i32;
            rc = pagerPagecount(pPager, std::ptr::addr_of_mut!(nPage));
            if rc == (0 as i32) {
                // /* If the database is zero pages in size, that means that either (1) the
                //         ** journal is a remnant from a prior database with the same name where
                //         ** the database file but not the journal was deleted, or (2) the initial
                //         ** transaction that populates a new database is being rolled back.
                //         ** In either case, the journal file can be deleted.  However, take care
                //         ** not to delete the journal file if it is already open due to
                //         ** journal_mode=PERSIST.
                //         */
                if nPage == ((0 as i32) as u32) && !(jrnlOpen != (0 as i32)) {
                    unsafe { sqlite3BeginBenignMalloc() };
                    if pagerLockDb(pPager, 2 as i32) == (0 as i32) {
                        unsafe {
                            sqlite3OsDelete(
                                pVfs,
                                (unsafe { (*pPager).zJournal }) as *const i8,
                                0 as i32,
                            )
                        };
                        if !((unsafe { (*pPager).exclusiveMode }) != (0 as u8)) {
                            pagerUnlockDb(pPager, 1 as i32);
                        }
                    }
                    unsafe { sqlite3EndBenignMalloc() };
                } else {
                    // /* The journal file exists and no other connection has a reserved
                    //           ** or greater lock on the database file. Now check that there is
                    //           ** at least one non-zero bytes at the start of the journal file.
                    //           ** If there is, then we consider this journal to be hot. If not,
                    //           ** it can be ignored.
                    //           */
                    if !(jrnlOpen != (0 as i32)) {
                        let mut f: i32 = (1 as i32) | (2048 as i32);
                        rc = unsafe {
                            sqlite3OsOpen(
                                pVfs,
                                (unsafe { (*pPager).zJournal }) as *const i8,
                                unsafe { (*pPager).jfd },
                                f,
                                std::ptr::addr_of_mut!(f),
                            )
                        };
                    }
                    if rc == (0 as i32) {
                        let mut first: u8 = ((0 as i32) as i8) as u8;
                        rc = unsafe {
                            sqlite3OsRead(
                                unsafe { (*pPager).jfd },
                                std::ptr::addr_of_mut!(first) as *mut (),
                                1 as i32,
                                (0 as i32) as i64,
                            )
                        };
                        if rc == (10 as i32) | (2 as i32) << (8 as i32) {
                            rc = 0 as i32;
                        }
                        if !(jrnlOpen != (0 as i32)) {
                            unsafe { sqlite3OsClose(unsafe { (*pPager).jfd }) };
                        }
                        unsafe {
                            *pExists = (((first as u32) as i32) != (0 as i32)) as i32;
                        }
                    } else {
                        if rc == (14 as i32) {
                            // /* If we cannot open the rollback journal file in order to see if
                            //             ** it has a zero header, that might be due to an I/O error, or
                            //             ** it might be due to the race condition described above and in
                            //             ** ticket #3883.  Either way, assume that the journal is hot.
                            //             ** This might be a false positive.  But if it is, then the
                            //             ** automatic journal playback and recovery mechanism will deal
                            //             ** with it under an EXCLUSIVE lock where we do not need to
                            //             ** worry so much with race conditions.
                            //             */
                            unsafe {
                                *pExists = 1 as i32;
                            }
                            rc = 0 as i32;
                        }
                    }
                }
            }
        }
    }
    return rc;
}

// /*
// ** If the reference count has reached zero, rollback any active
// ** transaction and unlock the pager.
// **
// ** Except, in locking_mode=EXCLUSIVE when there is nothing to in
// ** the rollback journal, the unlock is not performed and there is
// ** nothing to rollback, so this routine is a no-op.
// */
fn pagerUnlockIfUnused(mut pPager: *mut Pager) {
    if (unsafe { sqlite3PcacheRefCount(unsafe { (*pPager).pPCache }) }) == ((0 as i32) as i64) {
        // /* because page1 is never memory mapped */
        0 as i32;
        pagerUnlockAndRollback(pPager);
    }
}

// /*
// ** This function is called at the start of every write transaction.
// ** There must already be a RESERVED or EXCLUSIVE lock on the database
// ** file when this routine is called.
// **
// ** Open the journal file for pager pPager and write a journal header
// ** to the start of it. If there are active savepoints, open the sub-journal
// ** as well. This function is only used when the journal file is being
// ** opened to write a rollback log for a transaction. It is not used
// ** when opening a hot journal file to roll it back.
// **
// ** If the journal file is already open (as it may be in exclusive mode),
// ** then this function just writes a journal header to the start of the
// ** already open file.
// **
// ** Whether or not the journal file is opened by this function, the
// ** Pager.pInJournal bitvec structure is allocated.
// **
// ** Return SQLITE_OK if everything is successful. Otherwise, return
// ** SQLITE_NOMEM if the attempt to allocate Pager.pInJournal fails, or
// ** an IO error code if opening or writing the journal file fails.
// */
fn pager_open_journal(mut pPager: *mut Pager) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* Local cache of vfs pointer */
    let mut pVfs: *mut sqlite3_vfs = unsafe { (*pPager).pVfs };
    0 as i32;
    0 as i32;
    0 as i32;
    // /* If already in the error state, this function is a no-op.  But on
    //   ** the other hand, this routine is never called if we are already in
    //   ** an error state. */
    if (unsafe { (*pPager).errCode }) != (0 as i32) {
        return unsafe { (*pPager).errCode };
    }
    if !((unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>())
        && (((unsafe { (*pPager).journalMode }) as u32) as i32) != (2 as i32)
    {
        unsafe {
            (*pPager).pInJournal = unsafe { sqlite3BitvecCreate(unsafe { (*pPager).dbSize }) };
        }
        if (unsafe { (*pPager).pInJournal }) == std::ptr::null_mut::<Bitvec>() {
            return 7 as i32;
        }
        // /* Open the journal file if it is not already open. */
        if !((unsafe { (*unsafe { (*pPager).jfd }).pMethods })
            != std::ptr::null::<sqlite3_io_methods>())
        {
            if (((unsafe { (*pPager).journalMode }) as u32) as i32) == (4 as i32) {
                unsafe { sqlite3MemJournalOpen(unsafe { (*pPager).jfd }) };
            } else {
                let mut flags: i32 = (2 as i32) | (4 as i32);
                let mut nSpill: i32 = 0 as i32;
                if (unsafe { (*pPager).tempFile }) != (0 as u8) {
                    let __v1771: i32 = flags;
                    let __v1772: i32 = __v1771 | ((8 as i32) | (4096 as i32));
                    flags = __v1772;
                    let __v1773: i32 = flags;
                    let __v1774: i32 = __v1773 | (16 as i32);
                    flags = __v1774;
                    nSpill = unsafe { sqlite3Config.nStmtSpill };
                } else {
                    let __v1775: i32 = flags;
                    let __v1776: i32 = __v1775 | (2048 as i32);
                    flags = __v1776;
                    nSpill = jrnlBufferSize(pPager);
                }
                // /* Verify that the database still has the same name as it did when
                //         ** it was originally opened. */
                rc = databaseIsUnmoved(pPager);
                if rc == (0 as i32) {
                    rc = unsafe {
                        sqlite3JournalOpen(
                            pVfs,
                            (unsafe { (*pPager).zJournal }) as *const i8,
                            unsafe { (*pPager).jfd },
                            flags,
                            nSpill,
                        )
                    };
                }
            }
            0 as i32;
        }
        // /* Write the first journal header to the journal file and open
        //     ** the sub-journal if necessary.
        //     */
        if rc == (0 as i32) {
            // /* TODO: Check if all of these are really required. */
            unsafe {
                (*pPager).nRec = 0 as i32;
            }
            unsafe {
                (*pPager).journalOff = (0 as i32) as i64;
            }
            unsafe {
                (*pPager).setSuper = ((0 as i32) as i8) as u8;
            }
            unsafe {
                (*pPager).journalHdr = (0 as i32) as i64;
            }
            rc = writeJournalHdr(pPager);
        }
    }
    if rc != (0 as i32) {
        unsafe { sqlite3BitvecDestroy(unsafe { (*pPager).pInJournal }) };
        unsafe {
            (*pPager).pInJournal = std::ptr::null_mut::<Bitvec>();
        }
        unsafe {
            (*pPager).journalOff = (0 as i32) as i64;
        }
    } else {
        0 as i32;
        unsafe {
            (*pPager).eState = ((3 as i32) as i8) as u8;
        }
    }
    return rc;
}

// /*
// ** Write page pPg onto the end of the rollback journal.
// */
fn pagerAddPageToRollbackJournal(mut pPg: *mut PgHdr) -> i32 {
    let mut pPager: *mut Pager = unsafe { (*pPg).pPager };
    let mut rc: i32 = 0 as i32;
    let mut cksum: u32 = 0 as u32;
    let mut pData2: *mut i8 = unsafe { std::mem::zeroed() };
    let mut iOff: i64 = unsafe { (*pPager).journalOff };
    // /* We should never write to the journal file the page that
    //   ** contains the database locks.  The following assert verifies
    //   ** that we do not. */
    0 as i32;
    0 as i32;
    pData2 = (unsafe { (*pPg).pData }) as *mut i8;
    cksum = pager_cksum(pPager, (pData2 as *mut u8) as *const u8);
    // /* Even if an IO or diskfull error occurs while journalling the
    //   ** page in the block above, set the need-sync flag for the page.
    //   ** Otherwise, when the transaction is rolled back, the logic in
    //   ** playback_one_page() will think that the page needs to be restored
    //   ** in the database file. And if an IO error occurs while doing so,
    //   ** then corruption may follow.
    //   */
    let __v1777: *mut PgHdr = pPg;
    let __v1778: u16 = unsafe { (*__v1777).flags };
    let __v1779: u16 = ((((__v1778 as u32) as i32) | (8 as i32)) as i16) as u16;
    unsafe {
        (*__v1777).flags = __v1779;
    }
    rc = write32bits(unsafe { (*pPager).jfd }, iOff, unsafe { (*pPg).pgno });
    if rc != (0 as i32) {
        return rc;
    }
    rc = unsafe {
        sqlite3OsWrite(
            unsafe { (*pPager).jfd },
            pData2 as *const (),
            (unsafe { (*pPager).pageSize }) as i32,
            iOff + ((4 as i32) as i64),
        )
    };
    if rc != (0 as i32) {
        return rc;
    }
    rc = write32bits(
        unsafe { (*pPager).jfd },
        iOff + unsafe { (*pPager).pageSize } + ((4 as i32) as i64),
        cksum,
    );
    if rc != (0 as i32) {
        return rc;
    }
    {}
    {}
    {}
    let __v1780: *mut Pager = pPager;
    let __v1781: i64 = unsafe { (*__v1780).journalOff };
    let __v1782: i64 = __v1781 + (((8 as i32) as i64) + unsafe { (*pPager).pageSize });
    unsafe {
        (*__v1780).journalOff = __v1782;
    }
    let __v1783: *mut Pager = pPager;
    let __v1784: i32 = unsafe { (*__v1783).nRec };
    let __v1785: i32 = __v1784 + (1 as i32);
    unsafe {
        (*__v1783).nRec = __v1785;
    }
    0 as i32;
    rc = unsafe { sqlite3BitvecSet(unsafe { (*pPager).pInJournal }, unsafe { (*pPg).pgno }) };
    {}
    0 as i32;
    let __v1786: i32 = rc;
    let __v1787: i32 = __v1786 | addToSavepointBitvecs(pPager, unsafe { (*pPg).pgno });
    rc = __v1787;
    0 as i32;
    return rc;
}

// /*
// ** Mark a single data page as writeable. The page is written into the
// ** main journal or sub-journal as required. If the page is written into
// ** one of the journals, the corresponding bit is set in the
// ** Pager.pInJournal bitvec and the PagerSavepoint.pInSavepoint bitvecs
// ** of any open savepoints as appropriate.
// */
fn pager_write(mut pPg: *mut PgHdr) -> i32 {
    let mut pPager: *mut Pager = unsafe { (*pPg).pPager };
    let mut rc: i32 = 0 as i32;
    // /* This routine is not called unless a write-transaction has already
    //   ** been started. The journal file may or may not be open at this point.
    //   ** It is never called in the ERROR state.
    //   */
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    {}
    // /* The journal file needs to be opened. Higher level routines have already
    //   ** obtained the necessary locks to begin the write-transaction, but the
    //   ** rollback journal might not yet be open. Open it now if this is the case.
    //   **
    //   ** This is done before calling sqlite3PcacheMakeDirty() on the page.
    //   ** Otherwise, if it were done after calling sqlite3PcacheMakeDirty(), then
    //   ** an error might occur and the pager would end up in WRITER_LOCKED state
    //   ** with pages marked as dirty in the cache.
    //   */
    if (((unsafe { (*pPager).eState }) as u32) as i32) == (2 as i32) {
        rc = pager_open_journal(pPager);
        if rc != (0 as i32) {
            return rc;
        }
    }
    0 as i32;
    0 as i32;
    // /* Mark the page that is about to be modified as dirty. */
    unsafe { sqlite3PcacheMakeDirty(pPg) };
    // /* If a rollback journal is in use, them make sure the page that is about
    //   ** to change is in the rollback journal, or if the page is a new page off
    //   ** then end of the file, make sure it is marked as PGHDR_NEED_SYNC.
    //   */
    0 as i32;
    let __v1788: bool;
    if (unsafe { (*pPager).pInJournal }) != std::ptr::null_mut::<Bitvec>() {
        __v1788 = (unsafe {
            sqlite3BitvecTestNotNull(unsafe { (*pPager).pInJournal }, unsafe { (*pPg).pgno })
        }) == (0 as i32);
    } else {
        __v1788 = false as bool;
    }
    if __v1788 {
        0 as i32;
        if (unsafe { (*pPg).pgno }) <= unsafe { (*pPager).dbOrigSize } {
            rc = pagerAddPageToRollbackJournal(pPg);
            if rc != (0 as i32) {
                return rc;
            }
        } else {
            if (((unsafe { (*pPager).eState }) as u32) as i32) != (4 as i32) {
                let __v1789: *mut PgHdr = pPg;
                let __v1790: u16 = unsafe { (*__v1789).flags };
                let __v1791: u16 = ((((__v1790 as u32) as i32) | (8 as i32)) as i16) as u16;
                unsafe {
                    (*__v1789).flags = __v1791;
                }
            }
            {}
        }
    }
    // /* The PGHDR_DIRTY bit is set above when the page was added to the dirty-list
    //   ** and before writing the page into the rollback journal.  Wait until now,
    //   ** after the page has been successfully journalled, before setting the
    //   ** PGHDR_WRITEABLE bit that indicates that the page can be safely modified.
    //   */
    let __v1792: *mut PgHdr = pPg;
    let __v1793: u16 = unsafe { (*__v1792).flags };
    let __v1794: u16 = ((((__v1793 as u32) as i32) | (4 as i32)) as i16) as u16;
    unsafe {
        (*__v1792).flags = __v1794;
    }
    // /* If the statement journal is open and the page is not in it,
    //   ** then write the page into the statement journal.
    //   */
    if (unsafe { (*pPager).nSavepoint }) > (0 as i32) {
        rc = subjournalPageIfRequired(pPg);
    }
    // /* Update the database size and return. */
    if (unsafe { (*pPager).dbSize }) < unsafe { (*pPg).pgno } {
        unsafe {
            (*pPager).dbSize = unsafe { (*pPg).pgno };
        }
    }
    return rc;
}

// /*
// ** This is a variant of sqlite3PagerWrite() that runs when the sector size
// ** is larger than the page size.  SQLite makes the (reasonable) assumption that
// ** all bytes of a sector are written together by hardware.  Hence, all bytes of
// ** a sector need to be journalled in case of a power loss in the middle of
// ** a write.
// **
// ** Usually, the sector size is less than or equal to the page size, in which
// ** case pages can be individually written.  This routine only runs in the
// ** exceptional case where the page size is smaller than the sector size.
// */
fn pagerWriteLargeSector(mut pPg: *mut PgHdr) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* Total number of pages in database file */
    let mut nPageCount: u32 = 0 as u32;
    // /* First page of the sector pPg is located on. */
    let mut pg1: u32 = 0 as u32;
    // /* Number of pages starting at pg1 to journal */
    let mut nPage: i32 = 0 as i32;
    // /* Loop counter */
    let mut ii: i32 = 0 as i32;
    // /* True if any page has PGHDR_NEED_SYNC */
    let mut needSync: i32 = 0 as i32;
    // /* The pager that owns pPg */
    let mut pPager: *mut Pager = unsafe { (*pPg).pPager };
    let mut nPagePerSector: u32 = (((((unsafe { (*pPager).sectorSize }) as u64) as i64)
        / unsafe { (*pPager).pageSize }) as i32) as u32;
    // /* Set the doNotSpill NOSYNC bit to 1. This is because we cannot allow
    //   ** a journal header to be written between the pages journaled by
    //   ** this function.
    //   */
    0 as i32;
    0 as i32;
    let __v1795: *mut Pager = pPager;
    let __v1796: u8 = unsafe { (*__v1795).doNotSpill };
    let __v1797: u8 = ((((__v1796 as u32) as i32) | (4 as i32)) as i8) as u8;
    unsafe {
        (*__v1795).doNotSpill = __v1797;
    }
    // /* This trick assumes that both the page-size and sector-size are
    //   ** an integer power of 2. It sets variable pg1 to the identifier
    //   ** of the first page of the sector pPg is located on.
    //   */
    pg1 = (unsafe { (*pPg).pgno }.wrapping_sub((1 as i32) as u32)
        & !nPagePerSector.wrapping_sub((1 as i32) as u32))
    .wrapping_add((1 as i32) as u32);
    nPageCount = unsafe { (*pPager).dbSize };
    if (unsafe { (*pPg).pgno }) > nPageCount {
        nPage = unsafe { (*pPg).pgno }
            .wrapping_sub(pg1)
            .wrapping_add((1 as i32) as u32) as i32;
    } else {
        if pg1
            .wrapping_add(nPagePerSector)
            .wrapping_sub((1 as i32) as u32)
            > nPageCount
        {
            nPage = nPageCount.wrapping_add((1 as i32) as u32).wrapping_sub(pg1) as i32;
        } else {
            nPage = nPagePerSector as i32;
        }
    }
    0 as i32;
    0 as i32;
    0 as i32;
    ii = 0 as i32;
    '__slate_break_1544: loop {
        if !(ii < nPage && rc == (0 as i32)) {
            break;
        }
        let mut pg: u32 = pg1.wrapping_add(ii as u32);
        let mut pPage: *mut PgHdr = unsafe { std::mem::zeroed() };
        let __v1800: bool;
        if pg == unsafe { (*pPg).pgno } {
            __v1800 = true as bool;
        } else {
            __v1800 = !((unsafe { sqlite3BitvecTest(unsafe { (*pPager).pInJournal }, pg) })
                != (0 as i32));
        }
        if __v1800 {
            if pg != unsafe { (*pPager).lckPgno } {
                rc = sqlite3PagerGet(pPager, pg, std::ptr::addr_of_mut!(pPage), 0 as i32);
                if rc == (0 as i32) {
                    rc = pager_write(pPage);
                    if (((unsafe { (*pPage).flags }) as u32) as i32) & (8 as i32) != (0 as i32) {
                        needSync = 1 as i32;
                    }
                    sqlite3PagerUnrefNotNull(pPage);
                }
            }
        } else {
            let __v1801: *mut PgHdr = sqlite3PagerLookup(pPager, pg);
            pPage = __v1801;
            if __v1801 != std::ptr::null_mut::<PgHdr>() {
                if (((unsafe { (*pPage).flags }) as u32) as i32) & (8 as i32) != (0 as i32) {
                    needSync = 1 as i32;
                }
                sqlite3PagerUnrefNotNull(pPage);
            }
        }
        let __v1798: i32 = ii;
        let __v1799: i32 = __v1798 + (1 as i32);
        ii = __v1799;
    }
    // /* If the PGHDR_NEED_SYNC flag is set for any of the nPage pages
    //   ** starting at pg1, then it needs to be set for all of them. Because
    //   ** writing to any of these nPage pages may damage the others, the
    //   ** journal file must contain sync()ed copies of all of them
    //   ** before any of them can be written out to the database file.
    //   */
    if rc == (0 as i32) && needSync != (0 as i32) {
        0 as i32;
        ii = 0 as i32;
        '__slate_break_1545: loop {
            if !(ii < nPage) {
                break;
            }
            let mut pPage: *mut PgHdr = sqlite3PagerLookup(pPager, pg1.wrapping_add(ii as u32));
            if pPage != std::ptr::null_mut::<PgHdr>() {
                let __v1804: *mut PgHdr = pPage;
                let __v1805: u16 = unsafe { (*__v1804).flags };
                let __v1806: u16 = ((((__v1805 as u32) as i32) | (8 as i32)) as i16) as u16;
                unsafe {
                    (*__v1804).flags = __v1806;
                }
                sqlite3PagerUnrefNotNull(pPage);
            }
            let __v1802: i32 = ii;
            let __v1803: i32 = __v1802 + (1 as i32);
            ii = __v1803;
        }
    }
    0 as i32;
    let __v1807: *mut Pager = pPager;
    let __v1808: u8 = unsafe { (*__v1807).doNotSpill };
    let __v1809: u8 = ((((__v1808 as u32) as i32) & !(4 as i32)) as i8) as u8;
    unsafe {
        (*__v1807).doNotSpill = __v1809;
    }
    return rc;
}

// /*
// ** This routine is called to increment the value of the database file
// ** change-counter, stored as a 4-byte big-endian integer starting at
// ** byte offset 24 of the pager file.  The secondary change counter at
// ** 92 is also updated, as is the SQLite version number at offset 96.
// **
// ** But this only happens if the pPager->changeCountDone flag is false.
// ** To avoid excess churning of page 1, the update only happens once.
// ** See also the pager_write_changecounter() routine that does an
// ** unconditional update of the change counters.
// **
// ** If the isDirectMode flag is zero, then this is done by calling
// ** sqlite3PagerWrite() on page 1, then modifying the contents of the
// ** page data. In this case the file will be updated when the current
// ** transaction is committed.
// **
// ** The isDirectMode flag may only be non-zero if the library was compiled
// ** with the SQLITE_ENABLE_ATOMIC_WRITE macro defined. In this case,
// ** if isDirect is non-zero, then the database file is updated directly
// ** by writing an updated version of page 1 using a call to the
// ** sqlite3OsWrite() function.
// */
fn pager_incr_changecounter(mut pPager: *mut Pager, mut isDirectMode: i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    // /* Declare and initialize constant integer 'isDirect'. If the
    //   ** atomic-write optimization is enabled in this build, then isDirect
    //   ** is initialized to the value passed as the isDirectMode parameter
    //   ** to this function. Otherwise, it is always set to zero.
    //   **
    //   ** The idea is that if the atomic-write optimization is not
    //   ** enabled at compile time, the compiler can omit the tests of
    //   ** 'isDirect' below, as well as the block enclosed in the
    //   ** "if( isDirect )" condition.
    //   */
    0 as i32;
    isDirectMode;
    if !((unsafe { (*pPager).changeCountDone }) != (0 as u8))
        && (unsafe { (*pPager).dbSize }) > ((0 as i32) as u32)
    {
        // /* Reference to page 1 */
        let mut pPgHdr: *mut PgHdr = unsafe { std::mem::zeroed() };
        0 as i32;
        // /* Open page 1 of the file for writing. */
        rc = sqlite3PagerGet(
            pPager,
            (1 as i32) as u32,
            std::ptr::addr_of_mut!(pPgHdr),
            0 as i32,
        );
        0 as i32;
        // /* If page one was fetched successfully, and this function is not
        //     ** operating in direct-mode, make page 1 writable.  When not in
        //     ** direct mode, page 1 is always held in cache and hence the PagerGet()
        //     ** above is always successful - hence the ALWAYS on rc==SQLITE_OK.
        //     */
        if !((0 as i32) != (0 as i32)) && rc == (0 as i32) {
            rc = sqlite3PagerWrite(pPgHdr);
        }
        if rc == (0 as i32) {
            // /* Actually do the update of the change counter */
            pager_write_changecounter(pPgHdr);
            // /* If running in direct mode, write the contents of page 1 to the file. */
            if (0 as i32) != (0 as i32) {
                let mut zBuf: *const () = unsafe { std::mem::zeroed() };
                0 as i32;
                zBuf = (unsafe { (*pPgHdr).pData }) as *const ();
                if rc == (0 as i32) {
                    rc = unsafe {
                        sqlite3OsWrite(
                            unsafe { (*pPager).fd },
                            zBuf,
                            (unsafe { (*pPager).pageSize }) as i32,
                            (0 as i32) as i64,
                        )
                    };
                    let __v1810: *mut u32 = unsafe {
                        unsafe { (*pPager).aStat.as_mut_ptr() as *mut u32 }
                            .offset((2 as i32) as isize)
                    };
                    let __v1811: u32 = unsafe { *__v1810 };
                    let __v1812: u32 = __v1811.wrapping_add((1 as i32) as u32);
                    unsafe {
                        *__v1810 = __v1812;
                    }
                }
                if rc == (0 as i32) {
                    // /* Update the pager's copy of the change-counter. Otherwise, the
                    //           ** next time a read transaction is opened the cache will be
                    //           ** flushed (as the change-counter values will not match).  */
                    let mut pCopy: *const () =
                        (unsafe { (zBuf as *const i8).offset((24 as i32) as isize) }) as *const ();
                    unsafe {
                        memcpy(
                            (unsafe { std::ptr::addr_of_mut!((*pPager).dbFileVers) }) as *mut (),
                            pCopy,
                            16 as u64,
                        )
                    };
                    unsafe {
                        (*pPager).changeCountDone = ((1 as i32) as i8) as u8;
                    }
                }
            } else {
                unsafe {
                    (*pPager).changeCountDone = ((1 as i32) as i8) as u8;
                }
            }
        }
        // /* Release the page reference. */
        sqlite3PagerUnref(pPgHdr);
    }
    return rc;
}

// /*
// ** Check that there are at least nSavepoint savepoints open. If there are
// ** currently less than nSavepoints open, then open one or more savepoints
// ** to make up the difference. If the number of savepoints is already
// ** equal to nSavepoint, then this function is a no-op.
// **
// ** If a memory allocation fails, SQLITE_NOMEM is returned. If an error
// ** occurs while opening the sub-journal file, then an IO error code is
// ** returned. Otherwise, SQLITE_OK.
// */
fn pagerOpenSavepoint(mut pPager: *mut Pager, mut nSavepoint: i32) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* Current number of savepoints */
    let mut nCurrent: i32 = unsafe { (*pPager).nSavepoint };
    // /* Iterator variable */
    let mut ii: i32 = 0 as i32;
    // /* New Pager.aSavepoint array */
    let mut aNew: *mut PagerSavepoint = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    // /* Grow the Pager.aSavepoint array using realloc(). Return SQLITE_NOMEM
    //   ** if the allocation fails. Otherwise, zero the new portion in case a
    //   ** malloc failure occurs while populating it in the for(...) loop below.
    //   */
    aNew = (unsafe {
        sqlite3Realloc(
            (unsafe { (*pPager).aSavepoint }) as *mut (),
            (56 as u64).wrapping_mul((nSavepoint as i64) as u64),
        )
    }) as *mut PagerSavepoint;
    if !(aNew != std::ptr::null_mut::<PagerSavepoint>()) {
        return 7 as i32;
    }
    unsafe {
        memset(
            (unsafe { aNew.offset(nCurrent as isize) }) as *mut (),
            0 as i32,
            (((nSavepoint - nCurrent) as i64) as u64).wrapping_mul(56 as u64),
        )
    };
    unsafe {
        (*pPager).aSavepoint = aNew;
    }
    // /* Populate the PagerSavepoint structures just allocated. */
    ii = nCurrent;
    '__slate_break_1546: loop {
        if !(ii < nSavepoint) {
            break;
        }
        unsafe {
            (*unsafe { aNew.offset(ii as isize) }).nOrig = unsafe { (*pPager).dbSize };
        }
        if (unsafe { (*unsafe { (*pPager).jfd }).pMethods })
            != std::ptr::null::<sqlite3_io_methods>()
            && (unsafe { (*pPager).journalOff }) > ((0 as i32) as i64)
        {
            unsafe {
                (*unsafe { aNew.offset(ii as isize) }).iOffset = unsafe { (*pPager).journalOff };
            }
        } else {
            unsafe {
                (*unsafe { aNew.offset(ii as isize) }).iOffset =
                    ((unsafe { (*pPager).sectorSize }) as u64) as i64;
            }
        }
        unsafe {
            (*unsafe { aNew.offset(ii as isize) }).iSubRec = unsafe { (*pPager).nSubRec };
        }
        unsafe {
            (*unsafe { aNew.offset(ii as isize) }).pInSavepoint =
                unsafe { sqlite3BitvecCreate(unsafe { (*pPager).dbSize }) };
        }
        unsafe {
            (*unsafe { aNew.offset(ii as isize) }).bTruncateOnRelease = 1 as i32;
        }
        if !((unsafe { (*unsafe { aNew.offset(ii as isize) }).pInSavepoint })
            != std::ptr::null_mut::<Bitvec>())
        {
            return 7 as i32;
        }
        if (unsafe { (*pPager).pWal }) != std::ptr::null_mut::<Wal>() {
            unsafe {
                sqlite3WalSavepoint(unsafe { (*pPager).pWal }, unsafe {
                    (*unsafe { aNew.offset(ii as isize) }).aWalData.as_mut_ptr() as *mut u32
                })
            };
        }
        unsafe {
            (*pPager).nSavepoint = ii + (1 as i32);
        }
        let __v1813: i32 = ii;
        let __v1814: i32 = __v1813 + (1 as i32);
        ii = __v1814;
    }
    0 as i32;
    {}
    return rc;
}

// /*
// ** Attempt to take an exclusive lock on the database file. If a PENDING lock
// ** is obtained instead, immediately release it.
// */
fn pagerExclusiveLock(mut pPager: *mut Pager) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* Original lock */
    let mut eOrigLock: u8 = 0 as u8;
    0 as i32;
    eOrigLock = unsafe { (*pPager).eLock };
    rc = pagerLockDb(pPager, 4 as i32);
    if rc != (0 as i32) {
        // /* If the attempt to grab the exclusive lock failed, release the
        //     ** pending lock that may have been obtained instead.  */
        pagerUnlockDb(pPager, (eOrigLock as u32) as i32);
    }
    return rc;
}

// /*
// ** Call sqlite3WalOpen() to open the WAL handle. If the pager is in
// ** exclusive-locking mode when this function is called, take an EXCLUSIVE
// ** lock on the database file and use heap-memory to store the wal-index
// ** in. Otherwise, use the normal shared-memory.
// */
fn pagerOpenWal(mut pPager: *mut Pager) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    // /* If the pager is already in exclusive-mode, the WAL module will use
    //   ** heap-memory for the wal-index instead of the VFS shared-memory
    //   ** implementation. Take the exclusive lock now, before opening the WAL
    //   ** file, to make sure this is safe.
    //   */
    if (unsafe { (*pPager).exclusiveMode }) != (0 as u8) {
        rc = pagerExclusiveLock(pPager);
    }
    // /* Open the connection to the log file. If this operation fails,
    //   ** (e.g. due to malloc() failure), return an error code.
    //   */
    if rc == (0 as i32) {
        rc = unsafe {
            sqlite3WalOpen(
                unsafe { (*pPager).pVfs },
                unsafe { (*pPager).fd },
                (unsafe { (*pPager).zWal }) as *const i8,
                ((unsafe { (*pPager).exclusiveMode }) as u32) as i32,
                unsafe { (*pPager).journalSizeLimit },
                unsafe { std::ptr::addr_of_mut!((*pPager).pWal) },
            )
        };
    }
    pagerFixMaplimit(pPager);
    return rc;
}
