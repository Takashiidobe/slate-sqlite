unsafe extern "C" {
    fn sqlite3_malloc(__v902: i32) -> *mut ();
    fn sqlite3_free(__v903: *mut ());
    fn sqlite3_mutex_enter(__v1095: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(__v1096: *mut sqlite3_mutex);
    fn sqlite3_str_free(__v1097: *mut sqlite3_str);
    fn sqlite3_str_reset(__v1101: *mut sqlite3_str);
    fn sqlite3_str_value(__v1102: *mut sqlite3_str) -> *mut i8;
    fn sqlite3_log(iErrCode: i32, zFormat: *const i8, ...);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn sqlite3OsCurrentTimeInt64(__v1121: *mut sqlite3_vfs, __v1122: *mut i64) -> i32;
    fn sqlite3PagerWalCallback(pPager: *mut Pager) -> i32;
    fn sqlite3BtreeFirst(__v1124: *mut BtCursor, pRes: *mut i32) -> i32;
    fn sqlite3BtreeNext(__v1126: *mut BtCursor, flags: i32) -> i32;
    fn sqlite3BtreeEof(__v1128: *mut BtCursor) -> i32;
    fn sqlite3BtreePayloadSize(__v1129: *mut BtCursor) -> u32;
    fn sqlite3BtreePager(__v1130: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeEnter(__v1131: *mut Btree);
    fn sqlite3BtreeLeave(__v1132: *mut Btree);
    fn sqlite3VdbeDelete(__v1133: *mut Vdbe);
    fn sqlite3VdbeRewind(__v1134: *mut Vdbe);
    fn sqlite3VdbeReset(__v1135: *mut Vdbe) -> i32;
    fn sqlite3VdbeExpandSql(__v1136: *mut Vdbe, __v1137: *const i8) -> *mut i8;
    fn sqlite3MisuseError(__v1138: i32) -> i32;
    fn sqlite3Strlen30(__v1139: *const i8) -> i32;
    fn sqlite3DbMallocZero(__v1140: *mut sqlite3, __v1141: u64) -> *mut ();
    fn sqlite3DbStrDup(__v1142: *mut sqlite3, __v1143: *const i8) -> *mut i8;
    fn sqlite3DbFree(__v1144: *mut sqlite3, __v1145: *mut ());
    fn sqlite3RowSetClear(__v1146: *mut ());
    fn sqlite3LeaveMutexAndCloseZombie(__v1147: *mut sqlite3);
    fn sqlite3VListNumToName(__v1148: *mut i32, __v1149: i32) -> *const i8;
    fn sqlite3VListNameToNum(__v1150: *mut i32, __v1151: *const i8, __v1152: i32) -> i32;
    fn sqlite3GetVarint32(__v1153: *const u8, __v1154: *mut u32) -> u8;
    fn sqlite3Error(__v1155: *mut sqlite3, __v1156: i32);
    fn sqlite3ErrStr(__v1157: i32) -> *const i8;
    fn sqlite3ValueText(__v1158: *mut sqlite3_value, __v1159: u8) -> *const ();
    fn sqlite3ValueBytes(__v1160: *mut sqlite3_value, __v1161: u8) -> i32;
    fn sqlite3ValueFree(__v1162: *mut sqlite3_value);
    fn sqlite3OomFault(__v1164: *mut sqlite3) -> *mut ();
    fn sqlite3OomClear(__v1165: *mut sqlite3);
    fn sqlite3ApiExit(db: *mut sqlite3, __v1167: i32) -> i32;
    fn sqlite3StrAccumInit(
        __v1168: *mut sqlite3_str,
        __v1169: *mut sqlite3,
        __v1170: *mut i8,
        __v1171: i32,
        __v1172: i32,
    );
    fn sqlite3Reprepare(__v1179: *mut Vdbe) -> i32;
    fn sqlite3VdbeSerialGet(__v1180: *const u8, __v1181: u32, __v1182: *mut sqlite3_value);
    fn sqlite3VdbeExec(__v1183: *mut Vdbe) -> i32;
    fn sqlite3VdbeList(__v1184: *mut Vdbe) -> i32;
    fn sqlite3VdbeChangeEncoding(__v1185: *mut sqlite3_value, __v1186: i32) -> i32;
    fn sqlite3VdbeMemTooBig(__v1187: *mut sqlite3_value) -> i32;
    fn sqlite3VdbeMemCopy(__v1188: *mut sqlite3_value, __v1189: *const sqlite3_value) -> i32;
    fn sqlite3VdbeMemMove(__v1190: *mut sqlite3_value, __v1191: *mut sqlite3_value);
    fn sqlite3VdbeMemSetStr(
        __v1192: *mut sqlite3_value,
        __v1193: *const i8,
        __v1194: i64,
        __v1195: u8,
        __v1196: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3VdbeMemSetText(
        __v1197: *mut sqlite3_value,
        __v1198: *const i8,
        __v1199: i64,
        __v1200: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3VdbeMemSetInt64(__v1201: *mut sqlite3_value, __v1202: i64);
    fn sqlite3VdbeMemSetDouble(__v1203: *mut sqlite3_value, __v1204: f64);
    fn sqlite3VdbeMemSetPointer(
        __v1205: *mut sqlite3_value,
        __v1206: *mut (),
        __v1207: *const i8,
        __v1208: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3VdbeMemSetNull(__v1209: *mut sqlite3_value);
    fn sqlite3VdbeMemSetZeroBlob(__v1210: *mut sqlite3_value, __v1211: i32);
    fn sqlite3VdbeMemZeroTerminateIfAble(__v1212: *mut sqlite3_value) -> i32;
    fn sqlite3VdbeMemMakeWriteable(__v1213: *mut sqlite3_value) -> i32;
    fn sqlite3VdbeIntValue(__v1214: *const sqlite3_value) -> i64;
    fn sqlite3VdbeRealValue(__v1215: *mut sqlite3_value) -> f64;
    fn sqlite3VdbeMemFromBtreeZeroOffset(
        __v1216: *mut BtCursor,
        __v1217: u32,
        __v1218: *mut sqlite3_value,
    ) -> i32;
    fn sqlite3VdbeMemRelease(p: *mut sqlite3_value);
    fn sqlite3VdbeMemClearAndResize(pMem: *mut sqlite3_value, n: i32) -> i32;
    fn sqlite3VdbeTransferError(p: *mut Vdbe) -> i32;
    fn sqlite3VdbeMemExpandBlob(__v1224: *mut sqlite3_value) -> i32;
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
struct sqlite3_stmt {}

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
    __slate_bits_0: __slate_bits::__SlateBits69U0,
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
    __slate_bits_0: __slate_bits::__SlateBits93U0,
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
    __slate_bits_0: __slate_bits::__SlateBits105U0,
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
    u1: __SlateRecord194,
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
struct Pager {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Btree {}

#[repr(C)]
#[derive(Clone, Copy)]
struct BtCursor {}

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
    __slate_bits_0: __slate_bits::__SlateBits155U0,
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
union __SlateRecord194 {
    cr: __SlateRecord195,
    d: __SlateRecord196,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord195 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord196 {
    pReturning: *mut Returning,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VdbeSorter {}

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
struct VdbeTxtBlbCache {
    pCValue: *mut i8,
    iOffset: i64,
    iCol: i32,
    cacheStatus: u32,
    colCacheCtr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VdbeCursor {
    eCurType: u8,
    iDb: i8,
    nullRow: u8,
    deferredMoveto: u8,
    isTable: u8,
    __slate_bits_0: __slate_bits::__SlateBits205U0,
    seekHit: u16,
    ub: __SlateRecord207,
    seqCount: i64,
    cacheStatus: u32,
    seekResult: i32,
    pAltCursor: *mut VdbeCursor,
    uc: __SlateRecord208,
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
union __SlateRecord207 {
    pBtx: *mut Btree,
    aAltMap: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord208 {
    pCursor: *mut BtCursor,
    pVCur: *mut sqlite3_vtab_cursor,
    pSorter: *mut VdbeSorter,
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
    pub struct __SlateBits69U0 {
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
    pub struct __SlateBits93U0 {
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
    pub struct __SlateBits105U0 {
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
    pub struct __SlateBits205U0 {
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
    pub struct __SlateBits155U0 {
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
}

static mut aType: __SlateAlign16<[u8; 64]> = __SlateAlign16([
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
]);

static mut nullMem: sqlite3_value = sqlite3_value {
    u: {
        let mut __t0: MemValue = unsafe { std::mem::zeroed() };
        __t0.r = (0 as i32) as f64;
        __t0
    },
    z: std::ptr::null_mut::<i8>(),
    n: 0 as i32,
    flags: ((1 as i32) as i16) as u16,
    enc: ((0 as i32) as i8) as u8,
    eSubtype: ((0 as i32) as i8) as u8,
    db: std::ptr::null_mut::<sqlite3>(),
    szMalloc: 0 as i32,
    uTemp: (0 as i32) as u32,
    zMalloc: std::ptr::null_mut::<i8>(),
    xDel: None,
};

// /*
// ** Column names appropriate for EXPLAIN or EXPLAIN QUERY PLAN.
// */
static mut azExplainColNames8: __SlateAlign16<[*const i8; 12]> = __SlateAlign16([
    (b"addr\0".as_ptr() as *mut i8) as *const i8,
    (b"opcode\0".as_ptr() as *mut i8) as *const i8,
    (b"p1\0".as_ptr() as *mut i8) as *const i8,
    (b"p2\0".as_ptr() as *mut i8) as *const i8,
    (b"p3\0".as_ptr() as *mut i8) as *const i8,
    (b"p4\0".as_ptr() as *mut i8) as *const i8,
    (b"p5\0".as_ptr() as *mut i8) as *const i8,
    (b"comment\0".as_ptr() as *mut i8) as *const i8,
    (b"id\0".as_ptr() as *mut i8) as *const i8,
    (b"parent\0".as_ptr() as *mut i8) as *const i8,
    (b"notused\0".as_ptr() as *mut i8) as *const i8,
    (b"detail\0".as_ptr() as *mut i8) as *const i8,
]);

// /* EXPLAIN */
// /* EQP */
static mut azExplainColNames16data: __SlateAlign16<[u16; 60]> = __SlateAlign16([
    ((97 as i32) as i16) as u16,
    ((100 as i32) as i16) as u16,
    ((100 as i32) as i16) as u16,
    ((114 as i32) as i16) as u16,
    ((0 as i32) as i16) as u16,
    ((111 as i32) as i16) as u16,
    ((112 as i32) as i16) as u16,
    ((99 as i32) as i16) as u16,
    ((111 as i32) as i16) as u16,
    ((100 as i32) as i16) as u16,
    ((101 as i32) as i16) as u16,
    ((0 as i32) as i16) as u16,
    ((112 as i32) as i16) as u16,
    ((49 as i32) as i16) as u16,
    ((0 as i32) as i16) as u16,
    ((112 as i32) as i16) as u16,
    ((50 as i32) as i16) as u16,
    ((0 as i32) as i16) as u16,
    ((112 as i32) as i16) as u16,
    ((51 as i32) as i16) as u16,
    ((0 as i32) as i16) as u16,
    ((112 as i32) as i16) as u16,
    ((52 as i32) as i16) as u16,
    ((0 as i32) as i16) as u16,
    ((112 as i32) as i16) as u16,
    ((53 as i32) as i16) as u16,
    ((0 as i32) as i16) as u16,
    ((99 as i32) as i16) as u16,
    ((111 as i32) as i16) as u16,
    ((109 as i32) as i16) as u16,
    ((109 as i32) as i16) as u16,
    ((101 as i32) as i16) as u16,
    ((110 as i32) as i16) as u16,
    ((116 as i32) as i16) as u16,
    ((0 as i32) as i16) as u16,
    ((105 as i32) as i16) as u16,
    ((100 as i32) as i16) as u16,
    ((0 as i32) as i16) as u16,
    ((112 as i32) as i16) as u16,
    ((97 as i32) as i16) as u16,
    ((114 as i32) as i16) as u16,
    ((101 as i32) as i16) as u16,
    ((110 as i32) as i16) as u16,
    ((116 as i32) as i16) as u16,
    ((0 as i32) as i16) as u16,
    ((110 as i32) as i16) as u16,
    ((111 as i32) as i16) as u16,
    ((116 as i32) as i16) as u16,
    ((117 as i32) as i16) as u16,
    ((115 as i32) as i16) as u16,
    ((101 as i32) as i16) as u16,
    ((100 as i32) as i16) as u16,
    ((0 as i32) as i16) as u16,
    ((100 as i32) as i16) as u16,
    ((101 as i32) as i16) as u16,
    ((116 as i32) as i16) as u16,
    ((97 as i32) as i16) as u16,
    ((105 as i32) as i16) as u16,
    ((108 as i32) as i16) as u16,
    ((0 as i32) as i16) as u16,
]);

// /*   0 */
// /*   5 */
// /*  12 */
// /*  15 */
// /*  18 */
// /*  21 */
// /*  24 */
// /*  27 */
// /*  35 */
// /*  38 */
// /*  45 */
// /*  53 */
static mut iExplainColNames16: [u8; 12] = [
    ((0 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((15 as i32) as i8) as u8,
    ((18 as i32) as i8) as u8,
    ((21 as i32) as i8) as u8,
    ((24 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((35 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((45 as i32) as i8) as u8,
    ((53 as i32) as i8) as u8,
];

// /*
// ** Return the SQL associated with a prepared statement
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_sql")]
extern "C-unwind" fn sqlite3_sql(mut pStmt: *mut sqlite3_stmt) -> *const i8 {
    let mut p: *mut Vdbe = pStmt as *mut Vdbe;
    return (if p != std::ptr::null_mut::<Vdbe>() {
        unsafe { (*p).zSql }
    } else {
        std::ptr::null_mut::<i8>()
    }) as *const i8;
}

// /*
// ** Return the SQL associated with a prepared statement with
// ** bound parameters expanded.  Space to hold the returned string is
// ** obtained from sqlite3_malloc().  The caller is responsible for
// ** freeing the returned string by passing it to sqlite3_free().
// **
// ** The SQLITE_TRACE_SIZE_LIMIT puts an upper bound on the size of
// ** expanded bound parameters.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_expanded_sql")]
extern "C-unwind" fn sqlite3_expanded_sql(mut pStmt: *mut sqlite3_stmt) -> *mut i8 {
    let mut z: *mut i8 = std::ptr::null_mut::<i8>();
    let mut zSql: *const i8 = sqlite3_sql(pStmt);
    if zSql != std::ptr::null::<i8>() {
        let mut p: *mut Vdbe = pStmt as *mut Vdbe;
        unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*p).db }).mutex }) };
        z = unsafe { sqlite3VdbeExpandSql(p, zSql) };
        unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex }) };
    }
    return z;
}

// /*
// ** Return true if the prepared statement is guaranteed to not modify the
// ** database.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_stmt_readonly")]
extern "C-unwind" fn sqlite3_stmt_readonly(mut pStmt: *mut sqlite3_stmt) -> i32 {
    return if pStmt != std::ptr::null_mut::<sqlite3_stmt>() {
        (unsafe { (*(pStmt as *mut Vdbe)).__slate_bits_0.__get_readOnly() }) as i32
    } else {
        1 as i32
    };
}

// /*
// ** Return 1 if the statement is an EXPLAIN and return 2 if the
// ** statement is an EXPLAIN QUERY PLAN
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_stmt_isexplain")]
extern "C-unwind" fn sqlite3_stmt_isexplain(mut pStmt: *mut sqlite3_stmt) -> i32 {
    return if pStmt != std::ptr::null_mut::<sqlite3_stmt>() {
        (unsafe { (*(pStmt as *mut Vdbe)).__slate_bits_0.__get_explain() }) as i32
    } else {
        0 as i32
    };
}

// /*
// ** Set the explain mode for a statement.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_stmt_explain")]
extern "C-unwind" fn sqlite3_stmt_explain(mut pStmt: *mut sqlite3_stmt, mut eMode: i32) -> i32 {
    let mut v: *mut Vdbe = pStmt as *mut Vdbe;
    let mut rc: i32 = 0 as i32;
    unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*v).db }).mutex }) };
    if ((unsafe { (*v).__slate_bits_0.__get_explain() }) as i32) == eMode {
        rc = 0 as i32;
    } else {
        if eMode < (0 as i32) || eMode > (2 as i32) {
            rc = 1 as i32;
        } else {
            if (((unsafe { (*v).prepFlags }) as u32) as i32) & (128 as i32) == (0 as i32) {
                rc = 1 as i32;
            } else {
                if (((unsafe { (*v).eVdbeState }) as u32) as i32) != (1 as i32) {
                    rc = 5 as i32;
                } else {
                    if (unsafe { (*v).nMem }) >= (10 as i32)
                        && (eMode != (2 as i32)
                            || ((unsafe { (*v).__slate_bits_0.__get_haveEqpOps() }) as i32)
                                != (0 as i32))
                    {
                        // /* No reprepare necessary */
                        unsafe {
                            (*v).__slate_bits_0.__set_explain(eMode as u32);
                        }
                        rc = 0 as i32;
                    } else {
                        unsafe {
                            (*v).__slate_bits_0.__set_explain(eMode as u32);
                        }
                        rc = unsafe { sqlite3Reprepare(v) };
                        unsafe {
                            (*v).__slate_bits_0
                                .__set_haveEqpOps((eMode == (2 as i32)) as u32);
                        }
                    }
                }
            }
        }
    }
    if ((unsafe { (*v).__slate_bits_0.__get_explain() }) as i32) != (0 as i32) {
        unsafe {
            (*v).nResColumn = (((12 as i32)
                - (4 as i32) * ((unsafe { (*v).__slate_bits_0.__get_explain() }) as i32))
                as i16) as u16;
        }
    } else {
        unsafe {
            (*v).nResColumn = unsafe { (*v).nResAlloc };
        }
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*v).db }).mutex }) };
    return rc;
}

// /*
// ** Return true if the prepared statement is in need of being reset.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_stmt_busy")]
extern "C-unwind" fn sqlite3_stmt_busy(mut pStmt: *mut sqlite3_stmt) -> i32 {
    let mut v: *mut Vdbe = pStmt as *mut Vdbe;
    return (v != std::ptr::null_mut::<Vdbe>()
        && (((unsafe { (*v).eVdbeState }) as u32) as i32) == (2 as i32)) as i32;
}

// /* The statement to bind against */
// /* Index of the parameter to bind */
// /* Pointer to the data to be bound */
// /* Number of bytes of data to be bound */
// /* Destructor for the data */
// /* Encoding for the data */
// /*
// ** Bind a blob value to an SQL statement variable.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_blob")]
extern "C-unwind" fn sqlite3_bind_blob(
    mut pStmt: *mut sqlite3_stmt,
    mut i: i32,
    mut zData: *const (),
    mut nData: i32,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    return bindText(
        pStmt,
        i,
        zData,
        nData as i64,
        xDel,
        ((0 as i32) as i8) as u8,
    );
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_blob64")]
extern "C-unwind" fn sqlite3_bind_blob64(
    mut pStmt: *mut sqlite3_stmt,
    mut i: i32,
    mut zData: *const (),
    mut nData: u64,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    0 as i32;
    return bindText(
        pStmt,
        i,
        zData,
        nData as i64,
        xDel,
        ((0 as i32) as i8) as u8,
    );
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_double")]
extern "C-unwind" fn sqlite3_bind_double(
    mut pStmt: *mut sqlite3_stmt,
    mut i: i32,
    mut rValue: f64,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut p: *mut Vdbe = pStmt as *mut Vdbe;
    rc = vdbeUnbind(p, (i - (1 as i32)) as u32);
    if rc == (0 as i32) {
        // /* tag-20240917-01 */
        0 as i32;
        unsafe {
            sqlite3VdbeMemSetDouble(
                unsafe { unsafe { (*p).aVar }.offset((i - (1 as i32)) as isize) },
                rValue,
            )
        };
        unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex }) };
    }
    return rc;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_int")]
extern "C-unwind" fn sqlite3_bind_int(
    mut p: *mut sqlite3_stmt,
    mut i: i32,
    mut iValue: i32,
) -> i32 {
    return sqlite3_bind_int64(p, i, iValue as i64);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_int64")]
extern "C-unwind" fn sqlite3_bind_int64(
    mut pStmt: *mut sqlite3_stmt,
    mut i: i32,
    mut iValue: i64,
) -> i32 {
    let mut pVar: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    let mut p: *mut Vdbe = pStmt as *mut Vdbe;
    if vdbeSafetyNotNull(p) != (0 as i32) {
        return unsafe { sqlite3MisuseError(1860 as i32) };
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*p).db }).mutex }) };
    if (((unsafe { (*p).eVdbeState }) as u32) as i32) != (1 as i32) {
        unsafe {
            sqlite3Error(unsafe { (*p).db }, unsafe {
                sqlite3MisuseError(1864 as i32)
            })
        };
        unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex }) };
        unsafe {
            sqlite3_log(
                21 as i32,
                (b"bind on a busy prepared statement: [%s]\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*p).zSql },
            )
        };
        return unsafe { sqlite3MisuseError(1868 as i32) };
    }
    let __v1250: bool;
    if i <= (0 as i32) {
        __v1250 = true as bool;
    } else {
        let __v1251: i32 = i;
        let __v1252: i32 = __v1251 - (1 as i32);
        i = __v1252;
        __v1250 = __v1252 >= ((unsafe { (*p).nVar }) as i32);
    }
    if __v1250 {
        unsafe { sqlite3Error(unsafe { (*p).db }, 25 as i32) };
        unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex }) };
        return 25 as i32;
    }
    pVar = unsafe { unsafe { (*p).aVar }.offset(i as isize) };
    if (unsafe { (*p).expmask }) != ((0 as i32) as u32) {
        let mut expireMask: u32 = if i >= (32 as i32) {
            2147483648 as u32
        } else {
            ((1 as i32) as u32) << i
        };
        0 as i32;
        if (unsafe { (*p).expmask }) & expireMask != ((0 as i32) as u32) {
            if (unsafe { (*p).smimask }) & expireMask != ((0 as i32) as u32) {
                // /* If the smimask bit is set, only expire the prepared statement
                //         ** if the value is changing to or from (0,1) and something else */
                let mut sm1: i32 = ((((unsafe { (*pVar).flags }) as u32) as i32) & (4 as i32)
                    != (0 as i32)
                    && (unsafe { (*pVar).u.i }) >= ((0 as i32) as i64)
                    && (unsafe { (*pVar).u.i }) <= ((1 as i32) as i64))
                    as i32;
                let mut sm2: i32 =
                    (iValue >= ((0 as i32) as i64) && iValue <= ((1 as i32) as i64)) as i32;
                if sm1 != sm2 {
                    unsafe {
                        (*p).__slate_bits_0.__set_expired((1 as i32) as u32);
                    }
                }
            } else {
                if (((unsafe { (*pVar).flags }) as u32) as i32) & (4 as i32) == (0 as i32)
                    || (unsafe { (*pVar).u.i }) != iValue
                {
                    // /* Always expire if the value really is changing */
                    unsafe {
                        (*p).__slate_bits_0.__set_expired((1 as i32) as u32);
                    }
                }
            }
        }
    }
    unsafe { sqlite3VdbeMemSetInt64(pVar, iValue) };
    unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex }) };
    return 0 as i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_null")]
extern "C-unwind" fn sqlite3_bind_null(mut pStmt: *mut sqlite3_stmt, mut i: i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut p: *mut Vdbe = pStmt as *mut Vdbe;
    rc = vdbeUnbind(p, (i - (1 as i32)) as u32);
    if rc == (0 as i32) {
        // /* tag-20240917-01 */
        0 as i32;
        unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex }) };
    }
    return rc;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_text")]
extern "C-unwind" fn sqlite3_bind_text(
    mut pStmt: *mut sqlite3_stmt,
    mut i: i32,
    mut zData: *const i8,
    mut nData: i32,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    return bindText(
        pStmt,
        i,
        zData as *const (),
        nData as i64,
        xDel,
        ((1 as i32) as i8) as u8,
    );
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_text16")]
extern "C-unwind" fn sqlite3_bind_text16(
    mut pStmt: *mut sqlite3_stmt,
    mut i: i32,
    mut zData: *const (),
    mut n: i32,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    return bindText(
        pStmt,
        i,
        zData,
        (((n as i64) as u64) & !(((1 as i32) as i64) as u64)) as i64,
        xDel,
        ((2 as i32) as i8) as u8,
    );
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_text64")]
extern "C-unwind" fn sqlite3_bind_text64(
    mut pStmt: *mut sqlite3_stmt,
    mut i: i32,
    mut zData: *const i8,
    mut nData: u64,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
    mut enc: u8,
) -> i32 {
    0 as i32;
    if ((enc as u32) as i32) != (1 as i32) && ((enc as u32) as i32) != (16 as i32) {
        if ((enc as u32) as i32) == (4 as i32) {
            enc = ((2 as i32) as i8) as u8;
        }
        let __v1253: u64 = nData;
        let __v1254: u64 = __v1253 & !(((1 as i32) as i64) as u64);
        nData = __v1254;
    }
    return bindText(pStmt, i, zData as *const (), nData as i64, xDel, enc);
}

// /* SQLITE_OMIT_UTF16 */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_value")]
extern "C-unwind" fn sqlite3_bind_value(
    mut pStmt: *mut sqlite3_stmt,
    mut i: i32,
    mut pValue: *const sqlite3_value,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    '__slate_break_1248: {
        match sqlite3_value_type(pValue as *mut sqlite3_value) {
            1 => {
                rc = sqlite3_bind_int64(pStmt, i, unsafe { (*pValue).u.i });
            }
            2 => {
                0 as i32;
                rc = sqlite3_bind_double(
                    pStmt,
                    i,
                    if (((unsafe { (*pValue).flags }) as u32) as i32) & (8 as i32) != (0 as i32) {
                        unsafe { (*pValue).u.r }
                    } else {
                        (unsafe { (*pValue).u.i }) as f64
                    },
                );
            }
            4 => {
                if (((unsafe { (*pValue).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
                    rc = sqlite3_bind_zeroblob(pStmt, i, unsafe { (*pValue).u.nZero });
                } else {
                    rc = sqlite3_bind_blob(
                        pStmt,
                        i,
                        (unsafe { (*pValue).z }) as *const (),
                        unsafe { (*pValue).n },
                        unsafe {
                            std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                                -(1 as i32) as usize,
                            )
                        },
                    );
                }
            }
            3 => {
                rc = bindText(
                    pStmt,
                    i,
                    (unsafe { (*pValue).z }) as *const (),
                    (unsafe { (*pValue).n }) as i64,
                    unsafe {
                        std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                            -(1 as i32) as usize,
                        )
                    },
                    unsafe { (*pValue).enc },
                );
            }
            _ => {
                rc = sqlite3_bind_null(pStmt, i);
            }
        }
    }
    return rc;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_pointer")]
extern "C-unwind" fn sqlite3_bind_pointer(
    mut pStmt: *mut sqlite3_stmt,
    mut i: i32,
    mut pPtr: *mut (),
    mut zPTtype: *const i8,
    mut xDestructor: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut p: *mut Vdbe = pStmt as *mut Vdbe;
    rc = vdbeUnbind(p, (i - (1 as i32)) as u32);
    if rc == (0 as i32) {
        // /* tag-20240917-01 */
        0 as i32;
        unsafe {
            sqlite3VdbeMemSetPointer(
                unsafe { unsafe { (*p).aVar }.offset((i - (1 as i32)) as isize) },
                pPtr,
                zPTtype,
                xDestructor,
            )
        };
        unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex }) };
    } else {
        if xDestructor != None {
            unsafe { xDestructor.unwrap()(pPtr) };
        }
    }
    return rc;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_zeroblob")]
extern "C-unwind" fn sqlite3_bind_zeroblob(
    mut pStmt: *mut sqlite3_stmt,
    mut i: i32,
    mut n: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut p: *mut Vdbe = pStmt as *mut Vdbe;
    rc = vdbeUnbind(p, (i - (1 as i32)) as u32);
    if rc == (0 as i32) {
        // /* tag-20240917-01 */
        0 as i32;
        unsafe {
            sqlite3VdbeMemSetZeroBlob(
                unsafe { unsafe { (*p).aVar }.offset((i - (1 as i32)) as isize) },
                n,
            )
        };
        unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex }) };
    }
    return rc;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_zeroblob64")]
extern "C-unwind" fn sqlite3_bind_zeroblob64(
    mut pStmt: *mut sqlite3_stmt,
    mut i: i32,
    mut n: u64,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut p: *mut Vdbe = pStmt as *mut Vdbe;
    unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*p).db }).mutex }) };
    if n > (((unsafe {
        *unsafe {
            unsafe { (*unsafe { (*p).db }).aLimit.as_mut_ptr() as *mut i32 }
                .offset((0 as i32) as isize)
        }
    }) as i64) as u64)
    {
        rc = 18 as i32;
    } else {
        0 as i32;
        rc = sqlite3_bind_zeroblob(pStmt, i, (n as u32) as i32);
    }
    rc = unsafe { sqlite3ApiExit(unsafe { (*p).db }, rc) };
    unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex }) };
    return rc;
}

// /*
// ** Return the number of wildcards that can be potentially bound to.
// ** This routine is added to support DBD::SQLite.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_parameter_count")]
extern "C-unwind" fn sqlite3_bind_parameter_count(mut pStmt: *mut sqlite3_stmt) -> i32 {
    let mut p: *mut Vdbe = pStmt as *mut Vdbe;
    return if p != std::ptr::null_mut::<Vdbe>() {
        (unsafe { (*p).nVar }) as i32
    } else {
        0 as i32
    };
}

// /*
// ** Return the name of a wildcard parameter.  Return NULL if the index
// ** is out of range or if the wildcard is unnamed.
// **
// ** The result is always UTF-8.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_parameter_name")]
extern "C-unwind" fn sqlite3_bind_parameter_name(
    mut pStmt: *mut sqlite3_stmt,
    mut i: i32,
) -> *const i8 {
    let mut p: *mut Vdbe = pStmt as *mut Vdbe;
    if p == std::ptr::null_mut::<Vdbe>() {
        return std::ptr::null::<i8>();
    }
    return unsafe { sqlite3VListNumToName(unsafe { (*p).pVList }, i) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_bind_parameter_index")]
extern "C-unwind" fn sqlite3_bind_parameter_index(
    mut pStmt: *mut sqlite3_stmt,
    mut zName: *const i8,
) -> i32 {
    return sqlite3VdbeParameterIndex(pStmt as *mut Vdbe, zName, unsafe { sqlite3Strlen30(zName) });
}

// /*
// ** Set all the parameters in the compiled SQL statement to NULL.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_clear_bindings")]
extern "C-unwind" fn sqlite3_clear_bindings(mut pStmt: *mut sqlite3_stmt) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut p: *mut Vdbe = pStmt as *mut Vdbe;
    let mut mutex: *mut sqlite3_mutex = unsafe { std::mem::zeroed() };
    mutex = unsafe { (*unsafe { (*p).db }).mutex };
    unsafe { sqlite3_mutex_enter(mutex) };
    i = 0 as i32;
    '__slate_break_1227: loop {
        if !(i < ((unsafe { (*p).nVar }) as i32)) {
            break;
        }
        unsafe { sqlite3VdbeMemRelease(unsafe { unsafe { (*p).aVar }.offset(i as isize) }) };
        unsafe {
            (*unsafe { unsafe { (*p).aVar }.offset(i as isize) }).flags =
                ((1 as i32) as i16) as u16;
        }
        let __v1255: i32 = i;
        let __v1256: i32 = __v1255 + (1 as i32);
        i = __v1256;
    }
    0 as i32;
    if (unsafe { (*p).expmask }) != (0 as u32) {
        unsafe {
            (*p).__slate_bits_0.__set_expired((1 as i32) as u32);
        }
    }
    unsafe { sqlite3_mutex_leave(mutex) };
    return rc;
}

// /*
// ** Return the number of columns in the result set for the statement pStmt.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_column_count")]
extern "C-unwind" fn sqlite3_column_count(mut pStmt: *mut sqlite3_stmt) -> i32 {
    let mut pVm: *mut Vdbe = pStmt as *mut Vdbe;
    if pVm == std::ptr::null_mut::<Vdbe>() {
        return 0 as i32;
    }
    return ((unsafe { (*pVm).nResColumn }) as u32) as i32;
}

// /* The statement */
// /* Which column to get the name for */
// /* True to return the name as UTF16 */
// /* What type of name */
// /*
// ** Return the name of the Nth column of the result set returned by SQL
// ** statement pStmt.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_column_name")]
extern "C-unwind" fn sqlite3_column_name(mut pStmt: *mut sqlite3_stmt, mut N: i32) -> *const i8 {
    return columnName(pStmt, N, 0 as i32, 0 as i32) as *const i8;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_column_name16")]
extern "C-unwind" fn sqlite3_column_name16(mut pStmt: *mut sqlite3_stmt, mut N: i32) -> *const () {
    return columnName(pStmt, N, 1 as i32, 0 as i32);
}

// /*
// ** Constraint:  If you have ENABLE_COLUMN_METADATA then you must
// ** not define OMIT_DECLTYPE.
// */
// /*
// ** Return the column declaration type (if applicable) of the 'i'th column
// ** of the result set of SQL statement pStmt.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_column_decltype")]
extern "C-unwind" fn sqlite3_column_decltype(
    mut pStmt: *mut sqlite3_stmt,
    mut N: i32,
) -> *const i8 {
    return columnName(pStmt, N, 0 as i32, 1 as i32) as *const i8;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_column_decltype16")]
extern "C-unwind" fn sqlite3_column_decltype16(
    mut pStmt: *mut sqlite3_stmt,
    mut N: i32,
) -> *const () {
    return columnName(pStmt, N, 1 as i32, 1 as i32);
}

// /*
// ** This is the top-level implementation of sqlite3_step().  Call
// ** sqlite3Step() to do most of the work.  If a schema error occurs,
// ** call sqlite3Reprepare() and try again.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_step")]
extern "C-unwind" fn sqlite3_step(mut pStmt: *mut sqlite3_stmt) -> i32 {
    // /* Result from sqlite3Step() */
    let mut rc: i32 = 0 as i32;
    // /* the prepared statement */
    let mut v: *mut Vdbe = pStmt as *mut Vdbe;
    // /* Counter to prevent infinite loop of reprepares */
    let mut cnt: i32 = 0 as i32;
    // /* The database connection */
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    if vdbeSafetyNotNull(v) != (0 as i32) {
        return unsafe { sqlite3MisuseError(987 as i32) };
    }
    db = unsafe { (*v).db };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    '__slate_break_1231: loop {
        let __v1257: i32 = sqlite3Step(v);
        rc = __v1257;
        let __v1258: bool;
        if __v1257 == (17 as i32) {
            let __v1259: i32 = cnt;
            let __v1260: i32 = __v1259 + (1 as i32);
            cnt = __v1260;
            __v1258 = __v1259 < (50 as i32);
        } else {
            __v1258 = false as bool;
        }
        if !__v1258 {
            break;
        }
        let mut savedPc: i32 = unsafe { (*v).pc };
        rc = unsafe { sqlite3Reprepare(v) };
        if rc != (0 as i32) {
            // /* This case occurs after failing to recompile an sql statement.
            //       ** The error message from the SQL compiler has already been loaded
            //       ** into the database handle. This block copies the error message
            //       ** from the database handle into the statement and sets the statement
            //       ** program counter to 0 to ensure that when the statement is
            //       ** finalized or reset the parser error message is available via
            //       ** sqlite3_errmsg() and sqlite3_errcode().
            //       */
            let mut zErr: *const i8 = sqlite3_value_text(unsafe { (*db).pErr }) as *const i8;
            unsafe { sqlite3DbFree(db, (unsafe { (*v).zErrMsg }) as *mut ()) };
            if !((unsafe { (*db).mallocFailed }) != (0 as u8)) {
                unsafe {
                    (*v).zErrMsg = unsafe { sqlite3DbStrDup(db, zErr) };
                }
                let __v1261: i32 = unsafe { sqlite3ApiExit(db, rc) };
                rc = __v1261;
                unsafe {
                    (*v).rc = __v1261;
                }
            } else {
                unsafe {
                    (*v).zErrMsg = std::ptr::null_mut::<i8>();
                }
                rc = 7 as i32;
                unsafe {
                    (*v).rc = 7 as i32;
                }
            }
            break '__slate_break_1231;
        }
        sqlite3_reset(pStmt);
        if savedPc >= (0 as i32) {
            // /* Setting minWriteFileFormat to 254 is a signal to the OP_Init and
            //       ** OP_Trace opcodes to *not* perform SQLITE_TRACE_STMT because it has
            //       ** already been done once on a prior invocation that failed due to
            //       ** SQLITE_SCHEMA.   tag-20220401a  */
            unsafe {
                (*v).minWriteFileFormat = ((254 as i32) as i8) as u8;
            }
        }
        0 as i32;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}

// /*
// ** Return the number of values available from the current row of the
// ** currently executing statement pStmt.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_data_count")]
extern "C-unwind" fn sqlite3_data_count(mut pStmt: *mut sqlite3_stmt) -> i32 {
    let mut pVm: *mut Vdbe = pStmt as *mut Vdbe;
    if pVm == std::ptr::null_mut::<Vdbe>()
        || (unsafe { (*pVm).pResultRow }) == std::ptr::null_mut::<sqlite3_value>()
    {
        return 0 as i32;
    }
    return ((unsafe { (*pVm).nResColumn }) as u32) as i32;
}

// /**************************** sqlite3_column_  *******************************
// ** The following routines are used to access elements of the current row
// ** in the result set.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_column_blob")]
extern "C-unwind" fn sqlite3_column_blob(mut pStmt: *mut sqlite3_stmt, mut i: i32) -> *const () {
    let mut val: *const () = unsafe { std::mem::zeroed() };
    val = sqlite3_value_blob(columnMem(pStmt, i));
    // /* Even though there is no encoding conversion, value_blob() might
    //   ** need to call malloc() to expand the result of a zeroblob()
    //   ** expression.
    //   */
    columnMallocFailure(pStmt);
    return val;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_column_double")]
extern "C-unwind" fn sqlite3_column_double(mut pStmt: *mut sqlite3_stmt, mut i: i32) -> f64 {
    let mut val: f64 = sqlite3_value_double(columnMem(pStmt, i));
    columnMallocFailure(pStmt);
    return val;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_column_int")]
extern "C-unwind" fn sqlite3_column_int(mut pStmt: *mut sqlite3_stmt, mut i: i32) -> i32 {
    let mut val: i32 = sqlite3_value_int(columnMem(pStmt, i));
    columnMallocFailure(pStmt);
    return val;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_column_int64")]
extern "C-unwind" fn sqlite3_column_int64(mut pStmt: *mut sqlite3_stmt, mut i: i32) -> i64 {
    let mut val: i64 = sqlite3_value_int64(columnMem(pStmt, i));
    columnMallocFailure(pStmt);
    return val;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_column_text")]
extern "C-unwind" fn sqlite3_column_text(mut pStmt: *mut sqlite3_stmt, mut i: i32) -> *const u8 {
    let mut val: *const u8 = sqlite3_value_text(columnMem(pStmt, i));
    columnMallocFailure(pStmt);
    return val;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_column_text16")]
extern "C-unwind" fn sqlite3_column_text16(mut pStmt: *mut sqlite3_stmt, mut i: i32) -> *const () {
    let mut val: *const () = sqlite3_value_text16(columnMem(pStmt, i));
    columnMallocFailure(pStmt);
    return val;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_column_value")]
extern "C-unwind" fn sqlite3_column_value(
    mut pStmt: *mut sqlite3_stmt,
    mut i: i32,
) -> *mut sqlite3_value {
    let mut pOut: *mut sqlite3_value = columnMem(pStmt, i);
    if (((unsafe { (*pOut).flags }) as u32) as i32) & (8192 as i32) != (0 as i32) {
        let __v1262: *mut sqlite3_value = pOut;
        let __v1263: u16 = unsafe { (*__v1262).flags };
        let __v1264: u16 = ((((__v1263 as u32) as i32) & !(8192 as i32)) as i16) as u16;
        unsafe {
            (*__v1262).flags = __v1264;
        }
        let __v1265: *mut sqlite3_value = pOut;
        let __v1266: u16 = unsafe { (*__v1265).flags };
        let __v1267: u16 = ((((__v1266 as u32) as i32) | (16384 as i32)) as i16) as u16;
        unsafe {
            (*__v1265).flags = __v1267;
        }
    }
    columnMallocFailure(pStmt);
    return pOut;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_column_bytes")]
extern "C-unwind" fn sqlite3_column_bytes(mut pStmt: *mut sqlite3_stmt, mut i: i32) -> i32 {
    let mut val: i32 = sqlite3_value_bytes(columnMem(pStmt, i));
    columnMallocFailure(pStmt);
    return val;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_column_bytes16")]
extern "C-unwind" fn sqlite3_column_bytes16(mut pStmt: *mut sqlite3_stmt, mut i: i32) -> i32 {
    let mut val: i32 = sqlite3_value_bytes16(columnMem(pStmt, i));
    columnMallocFailure(pStmt);
    return val;
}

// /* SQLITE_OMIT_UTF16 */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_column_type")]
extern "C-unwind" fn sqlite3_column_type(mut pStmt: *mut sqlite3_stmt, mut i: i32) -> i32 {
    let mut iType: i32 = sqlite3_value_type(columnMem(pStmt, i));
    columnMallocFailure(pStmt);
    return iType;
}

// /*
// ** The checkProfileCallback(DB,P) macro checks to see if a profile callback
// ** is needed, and it invokes the callback if it is needed.
// */
// /*
// ** The following routine destroys a virtual machine that is created by
// ** the sqlite3_compile() routine. The integer returned is an SQLITE_
// ** success/failure code that describes the result of executing the virtual
// ** machine.
// **
// ** This routine sets the error code and string returned by
// ** sqlite3_errcode(), sqlite3_errmsg() and sqlite3_errmsg16().
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_finalize")]
extern "C-unwind" fn sqlite3_finalize(mut pStmt: *mut sqlite3_stmt) -> i32 {
    let mut rc: i32 = 0 as i32;
    if pStmt == std::ptr::null_mut::<sqlite3_stmt>() {
        // /* IMPLEMENTATION-OF: R-57228-12904 Invoking sqlite3_finalize() on a NULL
        //     ** pointer is a harmless no-op. */
        rc = 0 as i32;
    } else {
        let mut v: *mut Vdbe = pStmt as *mut Vdbe;
        let mut db: *mut sqlite3 = unsafe { (*v).db };
        if vdbeSafety(v) != (0 as i32) {
            return unsafe { sqlite3MisuseError(114 as i32) };
        }
        unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
        if (unsafe { (*v).startTime }) > ((0 as i32) as i64) {
            invokeProfileCallback(db, v);
        }
        {}
        0 as i32;
        rc = unsafe { sqlite3VdbeReset(v) };
        unsafe { sqlite3VdbeDelete(v) };
        rc = unsafe { sqlite3ApiExit(db, rc) };
        unsafe { sqlite3LeaveMutexAndCloseZombie(db) };
    }
    return rc;
}

// /*
// ** Terminate the current execution of an SQL statement and reset it
// ** back to its starting state so that it can be reused. A success code from
// ** the prior execution is returned.
// **
// ** This routine sets the error code and string returned by
// ** sqlite3_errcode(), sqlite3_errmsg() and sqlite3_errmsg16().
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_reset")]
extern "C-unwind" fn sqlite3_reset(mut pStmt: *mut sqlite3_stmt) -> i32 {
    let mut rc: i32 = 0 as i32;
    if pStmt == std::ptr::null_mut::<sqlite3_stmt>() {
        rc = 0 as i32;
    } else {
        let mut v: *mut Vdbe = pStmt as *mut Vdbe;
        let mut db: *mut sqlite3 = unsafe { (*v).db };
        unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
        if (unsafe { (*v).startTime }) > ((0 as i32) as i64) {
            invokeProfileCallback(db, v);
        }
        {}
        rc = unsafe { sqlite3VdbeReset(v) };
        unsafe { sqlite3VdbeRewind(v) };
        0 as i32;
        rc = unsafe { sqlite3ApiExit(db, rc) };
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    }
    return rc;
}

// /*
// ** Return the number of times the Step function of an aggregate has been
// ** called.
// **
// ** This function is deprecated.  Do not use it for new code.  It is
// ** provide only to avoid breaking legacy code.  New aggregate function
// ** implementations should keep their own counts within their aggregate
// ** context.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_aggregate_count")]
extern "C-unwind" fn sqlite3_aggregate_count(mut p: *mut sqlite3_context) -> i32 {
    0 as i32;
    return unsafe { (*unsafe { (*p).pMem }).n };
}

// /*
// ** 2004 May 26
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// **
// ** This file contains code use to implement APIs that are part of the
// ** VDBE.
// */
// /*
// ** Return TRUE (non-zero) of the statement supplied as an argument needs
// ** to be recompiled.  A statement needs to be recompiled whenever the
// ** execution environment changes in a way that would alter the program
// ** that sqlite3_prepare() generates.  For example, if new functions or
// ** collating sequences are registered or if an authorizer function is
// ** added or changed.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_expired")]
extern "C-unwind" fn sqlite3_expired(mut pStmt: *mut sqlite3_stmt) -> i32 {
    let mut iRet: i32 = 1 as i32;
    if pStmt != std::ptr::null_mut::<sqlite3_stmt>() {
        let mut p: *mut Vdbe = pStmt as *mut Vdbe;
        unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*p).db }).mutex }) };
        iRet = (unsafe { (*p).__slate_bits_0.__get_expired() }) as i32;
        unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex }) };
    }
    return iRet;
}

// /*
// ** Deprecated external interface.  Internal/core SQLite code
// ** should call sqlite3TransferBindings.
// **
// ** It is misuse to call this routine with statements from different
// ** database connections.  But as this is a deprecated interface, we
// ** will not bother to check for that condition.
// **
// ** If the two statements contain a different number of bindings, then
// ** an SQLITE_ERROR is returned.  Nothing else can go wrong, so otherwise
// ** SQLITE_OK is returned.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_transfer_bindings")]
extern "C-unwind" fn sqlite3_transfer_bindings(
    mut pFromStmt: *mut sqlite3_stmt,
    mut pToStmt: *mut sqlite3_stmt,
) -> i32 {
    let mut pFrom: *mut Vdbe = pFromStmt as *mut Vdbe;
    let mut pTo: *mut Vdbe = pToStmt as *mut Vdbe;
    if ((unsafe { (*pFrom).nVar }) as i32) != ((unsafe { (*pTo).nVar }) as i32) {
        return 1 as i32;
    }
    0 as i32;
    if (unsafe { (*pTo).expmask }) != (0 as u32) {
        unsafe {
            (*pTo).__slate_bits_0.__set_expired((1 as i32) as u32);
        }
    }
    0 as i32;
    if (unsafe { (*pFrom).expmask }) != (0 as u32) {
        unsafe {
            (*pFrom).__slate_bits_0.__set_expired((1 as i32) as u32);
        }
    }
    return sqlite3TransferBindings(pFromStmt, pToStmt);
}

// /**************************** sqlite3_value_  *******************************
// ** The following routines extract information from a Mem or sqlite3_value
// ** structure.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_blob")]
extern "C-unwind" fn sqlite3_value_blob(mut pVal: *mut sqlite3_value) -> *const () {
    let mut p: *mut sqlite3_value = pVal;
    if (((unsafe { (*p).flags }) as u32) as i32) & ((16 as i32) | (2 as i32)) != (0 as i32) {
        let __v1268: i32;
        if (((unsafe { (*p).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
            __v1268 = unsafe { sqlite3VdbeMemExpandBlob(p) };
        } else {
            __v1268 = 0 as i32;
        }
        if __v1268 != (0 as i32) {
            0 as i32;
            return std::ptr::null::<()>();
        }
        let __v1269: *mut sqlite3_value = p;
        let __v1270: u16 = unsafe { (*__v1269).flags };
        let __v1271: u16 = ((((__v1270 as u32) as i32) | (16 as i32)) as i16) as u16;
        unsafe {
            (*__v1269).flags = __v1271;
        }
        return (if (unsafe { (*p).n }) != (0 as i32) {
            unsafe { (*p).z }
        } else {
            std::ptr::null_mut::<i8>()
        }) as *const ();
    } else {
        return sqlite3_value_text(pVal) as *const ();
    }
    return unsafe { std::mem::zeroed() };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_double")]
extern "C-unwind" fn sqlite3_value_double(mut pVal: *mut sqlite3_value) -> f64 {
    return unsafe { sqlite3VdbeRealValue(pVal) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_int")]
extern "C-unwind" fn sqlite3_value_int(mut pVal: *mut sqlite3_value) -> i32 {
    return (unsafe { sqlite3VdbeIntValue(pVal as *const sqlite3_value) }) as i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_int64")]
extern "C-unwind" fn sqlite3_value_int64(mut pVal: *mut sqlite3_value) -> i64 {
    return unsafe { sqlite3VdbeIntValue(pVal as *const sqlite3_value) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_pointer")]
extern "C-unwind" fn sqlite3_value_pointer(
    mut pVal: *mut sqlite3_value,
    mut zPType: *const i8,
) -> *mut () {
    let mut p: *mut sqlite3_value = pVal;
    if (((unsafe { (*p).flags }) as u32) as i32) & ((3519 as i32) | (512 as i32) | (2048 as i32))
        == (1 as i32) | (512 as i32) | (2048 as i32)
        && zPType != std::ptr::null::<i8>()
        && (((unsafe { (*p).eSubtype }) as u32) as i32) == (112 as i32)
        && (unsafe { strcmp(unsafe { (*p).u.zPType }, zPType) }) == (0 as i32)
    {
        return (unsafe { (*p).z }) as *mut ();
    } else {
        return std::ptr::null_mut::<()>();
    }
    return unsafe { std::mem::zeroed() };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_text")]
extern "C-unwind" fn sqlite3_value_text(mut pVal: *mut sqlite3_value) -> *const u8 {
    return (unsafe { sqlite3ValueText(pVal, ((1 as i32) as i8) as u8) }) as *const u8;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_text16")]
extern "C-unwind" fn sqlite3_value_text16(mut pVal: *mut sqlite3_value) -> *const () {
    return unsafe { sqlite3ValueText(pVal, ((2 as i32) as i8) as u8) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_text16le")]
extern "C-unwind" fn sqlite3_value_text16le(mut pVal: *mut sqlite3_value) -> *const () {
    return unsafe { sqlite3ValueText(pVal, ((2 as i32) as i8) as u8) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_text16be")]
extern "C-unwind" fn sqlite3_value_text16be(mut pVal: *mut sqlite3_value) -> *const () {
    return unsafe { sqlite3ValueText(pVal, ((3 as i32) as i8) as u8) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_bytes")]
extern "C-unwind" fn sqlite3_value_bytes(mut pVal: *mut sqlite3_value) -> i32 {
    return unsafe { sqlite3ValueBytes(pVal, ((1 as i32) as i8) as u8) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_bytes16")]
extern "C-unwind" fn sqlite3_value_bytes16(mut pVal: *mut sqlite3_value) -> i32 {
    return unsafe { sqlite3ValueBytes(pVal, ((2 as i32) as i8) as u8) };
}

// /* SQLITE_OMIT_UTF16 */
// /* EVIDENCE-OF: R-12793-43283 Every value in SQLite has one of five
// ** fundamental datatypes: 64-bit signed integer 64-bit IEEE floating
// ** point number string BLOB NULL
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_type")]
extern "C-unwind" fn sqlite3_value_type(mut pVal: *mut sqlite3_value) -> i32 {
    // /* 0x00 (not possible) */
    // /* 0x01 NULL */
    // /* 0x02 TEXT */
    // /* 0x03 (not possible) */
    // /* 0x04 INTEGER */
    // /* 0x05 (not possible) */
    // /* 0x06 INTEGER + TEXT */
    // /* 0x07 (not possible) */
    // /* 0x08 FLOAT */
    // /* 0x09 (not possible) */
    // /* 0x0a FLOAT + TEXT */
    // /* 0x0b (not possible) */
    // /* 0x0c (not possible) */
    // /* 0x0d (not possible) */
    // /* 0x0e (not possible) */
    // /* 0x0f (not possible) */
    // /* 0x10 BLOB */
    // /* 0x11 (not possible) */
    // /* 0x12 (not possible) */
    // /* 0x13 (not possible) */
    // /* 0x14 INTEGER + BLOB */
    // /* 0x15 (not possible) */
    // /* 0x16 (not possible) */
    // /* 0x17 (not possible) */
    // /* 0x18 FLOAT + BLOB */
    // /* 0x19 (not possible) */
    // /* 0x1a (not possible) */
    // /* 0x1b (not possible) */
    // /* 0x1c (not possible) */
    // /* 0x1d (not possible) */
    // /* 0x1e (not possible) */
    // /* 0x1f (not possible) */
    // /* 0x20 INTREAL */
    // /* 0x21 (not possible) */
    // /* 0x22 INTREAL + TEXT */
    // /* 0x23 (not possible) */
    // /* 0x24 (not possible) */
    // /* 0x25 (not possible) */
    // /* 0x26 (not possible) */
    // /* 0x27 (not possible) */
    // /* 0x28 (not possible) */
    // /* 0x29 (not possible) */
    // /* 0x2a (not possible) */
    // /* 0x2b (not possible) */
    // /* 0x2c (not possible) */
    // /* 0x2d (not possible) */
    // /* 0x2e (not possible) */
    // /* 0x2f (not possible) */
    // /* 0x30 (not possible) */
    // /* 0x31 (not possible) */
    // /* 0x32 (not possible) */
    // /* 0x33 (not possible) */
    // /* 0x34 (not possible) */
    // /* 0x35 (not possible) */
    // /* 0x36 (not possible) */
    // /* 0x37 (not possible) */
    // /* 0x38 (not possible) */
    // /* 0x39 (not possible) */
    // /* 0x3a (not possible) */
    // /* 0x3b (not possible) */
    // /* 0x3c (not possible) */
    // /* 0x3d (not possible) */
    // /* 0x3e (not possible) */
    // /* 0x3f (not possible) */
    return ((unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(aType.0) as *const u8 }
                .offset(((((unsafe { (*pVal).flags }) as u32) as i32) & (63 as i32)) as isize)
        }
    }) as u32) as i32;
}

// /* Return true if a parameter to xUpdate represents an unchanged column */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_nochange")]
extern "C-unwind" fn sqlite3_value_nochange(mut pVal: *mut sqlite3_value) -> i32 {
    return ((((unsafe { (*pVal).flags }) as u32) as i32) & ((1 as i32) | (1024 as i32))
        == (1 as i32) | (1024 as i32)) as i32;
}

// /* Return true if a parameter value originated from an sqlite3_bind() */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_frombind")]
extern "C-unwind" fn sqlite3_value_frombind(mut pVal: *mut sqlite3_value) -> i32 {
    return ((((unsafe { (*pVal).flags }) as u32) as i32) & (64 as i32) != (0 as i32)) as i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_encoding")]
extern "C-unwind" fn sqlite3_value_encoding(mut pVal: *mut sqlite3_value) -> i32 {
    return ((unsafe { (*pVal).enc }) as u32) as i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_subtype")]
extern "C-unwind" fn sqlite3_value_subtype(mut pVal: *mut sqlite3_value) -> u32 {
    let mut pMem: *mut sqlite3_value = pVal;
    return (if (((unsafe { (*pMem).flags }) as u32) as i32) & (2048 as i32) != (0 as i32) {
        ((unsafe { (*pMem).eSubtype }) as u32) as i32
    } else {
        0 as i32
    }) as u32;
}

// /* Make a copy of an sqlite3_value object
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_dup")]
extern "C-unwind" fn sqlite3_value_dup(mut pOrig: *const sqlite3_value) -> *mut sqlite3_value {
    let mut pNew: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    if pOrig == std::ptr::null::<sqlite3_value>() {
        return std::ptr::null_mut::<sqlite3_value>();
    }
    pNew = (unsafe { sqlite3_malloc(((56 as u64) as u32) as i32) }) as *mut sqlite3_value;
    if pNew == std::ptr::null_mut::<sqlite3_value>() {
        return std::ptr::null_mut::<sqlite3_value>();
    }
    unsafe { memset(pNew as *mut (), 0 as i32, 56 as u64) };
    unsafe { memcpy(pNew as *mut (), pOrig as *const (), 24 as u64) };
    let __v1272: *mut sqlite3_value = pNew;
    let __v1273: u16 = unsafe { (*__v1272).flags };
    let __v1274: u16 = ((((__v1273 as u32) as i32) & !(4096 as i32)) as i16) as u16;
    unsafe {
        (*__v1272).flags = __v1274;
    }
    unsafe {
        (*pNew).db = std::ptr::null_mut::<sqlite3>();
    }
    if (((unsafe { (*pNew).flags }) as u32) as i32) & ((2 as i32) | (16 as i32)) != (0 as i32) {
        let __v1275: *mut sqlite3_value = pNew;
        let __v1276: u16 = unsafe { (*__v1275).flags };
        let __v1277: u16 =
            ((((__v1276 as u32) as i32) & !((8192 as i32) | (4096 as i32))) as i16) as u16;
        unsafe {
            (*__v1275).flags = __v1277;
        }
        let __v1278: *mut sqlite3_value = pNew;
        let __v1279: u16 = unsafe { (*__v1278).flags };
        let __v1280: u16 = ((((__v1279 as u32) as i32) | (16384 as i32)) as i16) as u16;
        unsafe {
            (*__v1278).flags = __v1280;
        }
        if (unsafe { sqlite3VdbeMemMakeWriteable(pNew) }) != (0 as i32) {
            unsafe { sqlite3ValueFree(pNew) };
            pNew = std::ptr::null_mut::<sqlite3_value>();
        }
    } else {
        if (((unsafe { (*pNew).flags }) as u32) as i32) & (1 as i32) != (0 as i32) {
            // /* Do not duplicate pointer values */
            let __v1281: *mut sqlite3_value = pNew;
            let __v1282: u16 = unsafe { (*__v1281).flags };
            let __v1283: u16 =
                ((((__v1282 as u32) as i32) & !((512 as i32) | (2048 as i32))) as i16) as u16;
            unsafe {
                (*__v1281).flags = __v1283;
            }
        }
    }
    return pNew;
}

// /* Destroy an sqlite3_value object previously obtained from
// ** sqlite3_value_dup().
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_value_free")]
extern "C-unwind" fn sqlite3_value_free(mut pOld: *mut sqlite3_value) {
    unsafe { sqlite3ValueFree(pOld) };
}

// /*
// ** Allocate or return the aggregate context for a user function.  A new
// ** context is allocated on the first call.  Subsequent calls return the
// ** same context that was returned on prior calls.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_aggregate_context")]
extern "C-unwind" fn sqlite3_aggregate_context(
    mut p: *mut sqlite3_context,
    mut nByte: i32,
) -> *mut () {
    0 as i32;
    0 as i32;
    {}
    if (((unsafe { (*unsafe { (*p).pMem }).flags }) as u32) as i32) & (32768 as i32) == (0 as i32) {
        return createAggContext(p, nByte);
    } else {
        return (unsafe { (*unsafe { (*p).pMem }).z }) as *mut ();
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Extract the user data from a sqlite3_context structure and return a
// ** pointer to it.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_user_data")]
extern "C-unwind" fn sqlite3_user_data(mut p: *mut sqlite3_context) -> *mut () {
    0 as i32;
    return unsafe { (*unsafe { (*p).pFunc }).pUserData };
}

// /*
// ** Extract the user data from a sqlite3_context structure and return a
// ** pointer to it.
// **
// ** IMPLEMENTATION-OF: R-46798-50301 The sqlite3_context_db_handle() interface
// ** returns a copy of the pointer to the database connection (the 1st
// ** parameter) of the sqlite3_create_function() and
// ** sqlite3_create_function16() routines that originally registered the
// ** application defined function.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_context_db_handle")]
extern "C-unwind" fn sqlite3_context_db_handle(mut p: *mut sqlite3_context) -> *mut sqlite3 {
    0 as i32;
    return unsafe { (*unsafe { (*p).pOut }).db };
}

// /*
// ** Return the auxiliary data pointer, if any, for the iArg'th argument to
// ** the user-function defined by pCtx.
// **
// ** The left-most argument is 0.
// **
// ** Undocumented behavior:  If iArg is negative then access a cache of
// ** auxiliary data pointers that is available to all functions within a
// ** single prepared statement.  The iArg values must match.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_get_auxdata")]
extern "C-unwind" fn sqlite3_get_auxdata(mut pCtx: *mut sqlite3_context, mut iArg: i32) -> *mut () {
    let mut pAuxData: *mut AuxData = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    pAuxData = unsafe { (*unsafe { (*pCtx).pVdbe }).pAuxData };
    '__slate_break_1232: while pAuxData != std::ptr::null_mut::<AuxData>() {
        if (unsafe { (*pAuxData).iAuxArg }) == iArg
            && ((unsafe { (*pAuxData).iAuxOp }) == unsafe { (*pCtx).iOp } || iArg < (0 as i32))
        {
            return unsafe { (*pAuxData).pAux };
        }
        pAuxData = unsafe { (*pAuxData).pNextAux };
    }
    return std::ptr::null_mut::<()>();
}

// /*
// ** Set the auxiliary data pointer and delete function, for the iArg'th
// ** argument to the user-function defined by pCtx. Any previous value is
// ** deleted by calling the delete function specified when it was set.
// **
// ** The left-most argument is 0.
// **
// ** Undocumented behavior:  If iArg is negative then make the data available
// ** to all functions within the current prepared statement using iArg as an
// ** access code.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_set_auxdata")]
extern "C-unwind" fn sqlite3_set_auxdata(
    mut pCtx: *mut sqlite3_context,
    mut iArg: i32,
    mut pAux: *mut (),
    mut xDelete: Option<unsafe extern "C-unwind" fn(*mut ())>,
) {
    let mut __slate_storage_709: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_709: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_709) as *mut *mut Vdbe;
    let mut __slate_storage_708: std::mem::MaybeUninit<*mut AuxData> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_708: *mut *mut AuxData =
        std::ptr::addr_of_mut!(__slate_storage_708) as *mut *mut AuxData;
    unsafe {
        '__slate_dispatch: {
            *__slate_slot_709 = unsafe { (*pCtx).pVdbe };
            0 as i32;
            0 as i32;
            *__slate_slot_708 = unsafe { (*(*__slate_slot_709)).pAuxData };
            loop {
                if *__slate_slot_708 != std::ptr::null_mut::<AuxData>() {
                    if (unsafe { (*(*__slate_slot_708)).iAuxArg }) == iArg
                        && ((unsafe { (*(*__slate_slot_708)).iAuxOp }) == unsafe { (*pCtx).iOp }
                            || iArg < (0 as i32))
                    {
                        break;
                    } else {
                        *__slate_slot_708 = unsafe { (*(*__slate_slot_708)).pNextAux };
                    }
                } else {
                    break;
                }
            }
            if *__slate_slot_708 == std::ptr::null_mut::<AuxData>() {
                *__slate_slot_708 = (unsafe {
                    sqlite3DbMallocZero(unsafe { (*(*__slate_slot_709)).db }, 32 as u64)
                }) as *mut AuxData;
                if !(*__slate_slot_708 != std::ptr::null_mut::<AuxData>()) {
                    if xDelete != None {
                        unsafe { xDelete.unwrap()(pAux) };
                    }
                    break '__slate_dispatch;
                } else {
                    unsafe {
                        (*(*__slate_slot_708)).iAuxOp = unsafe { (*pCtx).iOp };
                    }
                    unsafe {
                        (*(*__slate_slot_708)).iAuxArg = iArg;
                    }
                    unsafe {
                        (*(*__slate_slot_708)).pNextAux =
                            unsafe { (*(*__slate_slot_709)).pAuxData };
                    }
                    unsafe {
                        (*(*__slate_slot_709)).pAuxData = *__slate_slot_708;
                    }
                    if (unsafe { (*pCtx).isError }) == (0 as i32) {
                        unsafe {
                            (*pCtx).isError = -(1 as i32);
                        }
                    }
                }
            } else {
                if (unsafe { (*(*__slate_slot_708)).xDeleteAux }) != None {
                    unsafe {
                        unsafe { (*(*__slate_slot_708)).xDeleteAux }.unwrap()(unsafe {
                            (*(*__slate_slot_708)).pAux
                        })
                    };
                }
            }
            unsafe {
                (*(*__slate_slot_708)).pAux = pAux;
            }
            unsafe {
                (*(*__slate_slot_708)).xDeleteAux = xDelete;
            }
            return;
        }
    }
}

// /* Value to destroy */
// /* The destructor */
// /* Set a SQLITE_TOOBIG error if not NULL */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_blob")]
extern "C-unwind" fn sqlite3_result_blob(
    mut pCtx: *mut sqlite3_context,
    mut z: *const (),
    mut n: i32,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) {
    0 as i32;
    0 as i32;
    setResultStrOrError(pCtx, z as *const i8, n, ((0 as i32) as i8) as u8, xDel);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_blob64")]
extern "C-unwind" fn sqlite3_result_blob64(
    mut pCtx: *mut sqlite3_context,
    mut z: *const (),
    mut n: u64,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) {
    0 as i32;
    0 as i32;
    if n > (((2147483647 as i32) as i64) as u64) {
        invokeValueDestructor(z, xDel, pCtx);
    } else {
        setResultStrOrError(
            pCtx,
            z as *const i8,
            (n as u32) as i32,
            ((0 as i32) as i8) as u8,
            xDel,
        );
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_double")]
extern "C-unwind" fn sqlite3_result_double(mut pCtx: *mut sqlite3_context, mut rVal: f64) {
    0 as i32;
    unsafe { sqlite3VdbeMemSetDouble(unsafe { (*pCtx).pOut }, rVal) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_error")]
extern "C-unwind" fn sqlite3_result_error(
    mut pCtx: *mut sqlite3_context,
    mut z: *const i8,
    mut n: i32,
) {
    0 as i32;
    unsafe {
        (*pCtx).isError = 1 as i32;
    }
    unsafe {
        sqlite3VdbeMemSetStr(
            unsafe { (*pCtx).pOut },
            z,
            n as i64,
            ((1 as i32) as i8) as u8,
            unsafe {
                std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    -(1 as i32) as usize,
                )
            },
        )
    };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_error16")]
extern "C-unwind" fn sqlite3_result_error16(
    mut pCtx: *mut sqlite3_context,
    mut z: *const (),
    mut n: i32,
) {
    0 as i32;
    unsafe {
        (*pCtx).isError = 1 as i32;
    }
    unsafe {
        sqlite3VdbeMemSetStr(
            unsafe { (*pCtx).pOut },
            z as *const i8,
            n as i64,
            ((2 as i32) as i8) as u8,
            unsafe {
                std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    -(1 as i32) as usize,
                )
            },
        )
    };
}

// /* Cause the SQL function to raise an SQLITE_TOOBIG error. */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_error_toobig")]
extern "C-unwind" fn sqlite3_result_error_toobig(mut pCtx: *mut sqlite3_context) {
    0 as i32;
    unsafe {
        (*pCtx).isError = 18 as i32;
    }
    unsafe {
        sqlite3VdbeMemSetStr(
            unsafe { (*pCtx).pOut },
            (b"string or blob too big\0".as_ptr() as *mut i8) as *const i8,
            -(1 as i32) as i64,
            ((1 as i32) as i8) as u8,
            None,
        )
    };
}

// /* Cause the SQL function to raise an SQLITE_NOMEM error. */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_error_nomem")]
extern "C-unwind" fn sqlite3_result_error_nomem(mut pCtx: *mut sqlite3_context) {
    0 as i32;
    unsafe { sqlite3VdbeMemSetNull(unsafe { (*pCtx).pOut }) };
    unsafe {
        (*pCtx).isError = 7 as i32;
    }
    unsafe { sqlite3OomFault(unsafe { (*unsafe { (*pCtx).pOut }).db }) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_error_code")]
extern "C-unwind" fn sqlite3_result_error_code(mut pCtx: *mut sqlite3_context, mut errCode: i32) {
    unsafe {
        (*pCtx).isError = if errCode != (0 as i32) {
            errCode
        } else {
            -(1 as i32)
        };
    }
    if (((unsafe { (*unsafe { (*pCtx).pOut }).flags }) as u32) as i32) & (1 as i32) != (0 as i32) {
        setResultStrOrError(
            pCtx,
            unsafe { sqlite3ErrStr(errCode) },
            -(1 as i32),
            ((1 as i32) as i8) as u8,
            None,
        );
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_int")]
extern "C-unwind" fn sqlite3_result_int(mut pCtx: *mut sqlite3_context, mut iVal: i32) {
    0 as i32;
    unsafe { sqlite3VdbeMemSetInt64(unsafe { (*pCtx).pOut }, iVal as i64) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_int64")]
extern "C-unwind" fn sqlite3_result_int64(mut pCtx: *mut sqlite3_context, mut iVal: i64) {
    0 as i32;
    unsafe { sqlite3VdbeMemSetInt64(unsafe { (*pCtx).pOut }, iVal) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_null")]
extern "C-unwind" fn sqlite3_result_null(mut pCtx: *mut sqlite3_context) {
    0 as i32;
    unsafe { sqlite3VdbeMemSetNull(unsafe { (*pCtx).pOut }) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_text")]
extern "C-unwind" fn sqlite3_result_text(
    mut pCtx: *mut sqlite3_context,
    mut z: *const i8,
    mut n: i32,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) {
    0 as i32;
    setResultStrOrError(pCtx, z, n, ((1 as i32) as i8) as u8, xDel);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_text64")]
extern "C-unwind" fn sqlite3_result_text64(
    mut pCtx: *mut sqlite3_context,
    mut z: *const i8,
    mut n: u64,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
    mut enc: u8,
) {
    0 as i32;
    0 as i32;
    if ((enc as u32) as i32) != (1 as i32) && ((enc as u32) as i32) != (16 as i32) {
        if ((enc as u32) as i32) == (4 as i32) {
            enc = ((2 as i32) as i8) as u8;
        }
        let __v1284: u64 = n;
        let __v1285: u64 = __v1284 & !(((1 as i32) as i64) as u64);
        n = __v1285;
    }
    if n > (((2147483647 as i32) as i64) as u64) {
        invokeValueDestructor(z as *const (), xDel, pCtx);
    } else {
        setResultStrOrError(pCtx, z, (n as u32) as i32, enc, xDel);
        unsafe { sqlite3VdbeMemZeroTerminateIfAble(unsafe { (*pCtx).pOut }) };
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_text16")]
extern "C-unwind" fn sqlite3_result_text16(
    mut pCtx: *mut sqlite3_context,
    mut z: *const (),
    mut n: i32,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) {
    0 as i32;
    setResultStrOrError(
        pCtx,
        z as *const i8,
        ((((n as i64) as u64) & !(((1 as i32) as i64) as u64)) as u32) as i32,
        ((2 as i32) as i8) as u8,
        xDel,
    );
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_text16le")]
extern "C-unwind" fn sqlite3_result_text16le(
    mut pCtx: *mut sqlite3_context,
    mut z: *const (),
    mut n: i32,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) {
    0 as i32;
    setResultStrOrError(
        pCtx,
        z as *const i8,
        ((((n as i64) as u64) & !(((1 as i32) as i64) as u64)) as u32) as i32,
        ((2 as i32) as i8) as u8,
        xDel,
    );
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_text16be")]
extern "C-unwind" fn sqlite3_result_text16be(
    mut pCtx: *mut sqlite3_context,
    mut z: *const (),
    mut n: i32,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) {
    0 as i32;
    setResultStrOrError(
        pCtx,
        z as *const i8,
        ((((n as i64) as u64) & !(((1 as i32) as i64) as u64)) as u32) as i32,
        ((3 as i32) as i8) as u8,
        xDel,
    );
}

// /* SQLITE_OMIT_UTF16 */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_value")]
extern "C-unwind" fn sqlite3_result_value(
    mut pCtx: *mut sqlite3_context,
    mut pValue: *mut sqlite3_value,
) {
    let mut pOut: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    pOut = unsafe { (*pCtx).pOut };
    0 as i32;
    unsafe { sqlite3VdbeMemCopy(pOut, pValue as *const sqlite3_value) };
    unsafe { sqlite3VdbeChangeEncoding(pOut, ((unsafe { (*pCtx).enc }) as u32) as i32) };
    if (unsafe { sqlite3VdbeMemTooBig(pOut) }) != (0 as i32) {
        sqlite3_result_error_toobig(pCtx);
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_pointer")]
extern "C-unwind" fn sqlite3_result_pointer(
    mut pCtx: *mut sqlite3_context,
    mut pPtr: *mut (),
    mut zPType: *const i8,
    mut xDestructor: Option<unsafe extern "C-unwind" fn(*mut ())>,
) {
    let mut pOut: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    pOut = unsafe { (*pCtx).pOut };
    0 as i32;
    unsafe { sqlite3VdbeMemRelease(pOut) };
    unsafe {
        (*pOut).flags = ((1 as i32) as i16) as u16;
    }
    unsafe { sqlite3VdbeMemSetPointer(pOut, pPtr, zPType, xDestructor) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_zeroblob")]
extern "C-unwind" fn sqlite3_result_zeroblob(mut pCtx: *mut sqlite3_context, mut n: i32) {
    sqlite3_result_zeroblob64(
        pCtx,
        ((if n > (0 as i32) { n } else { 0 as i32 }) as i64) as u64,
    );
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_zeroblob64")]
extern "C-unwind" fn sqlite3_result_zeroblob64(mut pCtx: *mut sqlite3_context, mut n: u64) -> i32 {
    let mut pOut: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    pOut = unsafe { (*pCtx).pOut };
    0 as i32;
    if n > (((unsafe {
        *unsafe {
            unsafe { (*unsafe { (*pOut).db }).aLimit.as_mut_ptr() as *mut i32 }
                .offset((0 as i32) as isize)
        }
    }) as i64) as u64)
    {
        sqlite3_result_error_toobig(pCtx);
        return 18 as i32;
    }
    unsafe { sqlite3VdbeMemSetZeroBlob(unsafe { (*pCtx).pOut }, (n as u32) as i32) };
    return 0 as i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_subtype")]
extern "C-unwind" fn sqlite3_result_subtype(mut pCtx: *mut sqlite3_context, mut eSubtype: u32) {
    let mut pOut: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    pOut = unsafe { (*pCtx).pOut };
    0 as i32;
    unsafe {
        (*pOut).eSubtype = (eSubtype & ((255 as i32) as u32)) as u8;
    }
    let __v1286: *mut sqlite3_value = pOut;
    let __v1287: u16 = unsafe { (*__v1286).flags };
    let __v1288: u16 = ((((__v1287 as u32) as i32) | (2048 as i32)) as i16) as u16;
    unsafe {
        (*__v1286).flags = __v1288;
    }
}

// /*
// ** Return the sqlite3* database handle to which the prepared statement given
// ** in the argument belongs.  This is the same database handle that was
// ** the first argument to the sqlite3_prepare() that was used to create
// ** the statement in the first place.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_db_handle")]
extern "C-unwind" fn sqlite3_db_handle(mut pStmt: *mut sqlite3_stmt) -> *mut sqlite3 {
    return if pStmt != std::ptr::null_mut::<sqlite3_stmt>() {
        unsafe { (*(pStmt as *mut Vdbe)).db }
    } else {
        std::ptr::null_mut::<sqlite3>()
    };
}

// /*
// ** Return a pointer to the next prepared statement after pStmt associated
// ** with database connection pDb.  If pStmt is NULL, return the first
// ** prepared statement for the database connection.  Return NULL if there
// ** are no more.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_next_stmt")]
extern "C-unwind" fn sqlite3_next_stmt(
    mut pDb: *mut sqlite3,
    mut pStmt: *mut sqlite3_stmt,
) -> *mut sqlite3_stmt {
    let mut pNext: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_mutex_enter(unsafe { (*pDb).mutex }) };
    if pStmt == std::ptr::null_mut::<sqlite3_stmt>() {
        pNext = (unsafe { (*pDb).pVdbe }) as *mut sqlite3_stmt;
    } else {
        pNext = (unsafe { (*(pStmt as *mut Vdbe)).pVNext }) as *mut sqlite3_stmt;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*pDb).mutex }) };
    return pNext;
}

// /* Make the return value of the SQL function or virtual table pCtx
// ** be the content of the sqlite3_str object pStr.  The eOwn flag
// ** determines ownership of the sqlite3_str object and its content.
// **
// **    eOwn             Ownership transfer
// **    -------------    ------------------------------------------------
// **
// **    SQLITE_COPY      The SQL function returns a copy the sqlite3_str
// **                     content and leaves the sqlite3_str object itself
// **                     unchanged.
// **
// **    SQLITE_XFER      The content of the sqlite3_str is transferred to
// **                     the SQL function and the SQL function takes
// **                     responsibility for freeing that content when it is
// **                     no longer needed.  The sqlite3_str object is reset
// **                     to an empty string.
// **
// **    SQLITE_FINISH    Like SQLITE_XFER except that the pStr is also
// **                     freed using sqlite3_str_free().
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_result_str")]
extern "C-unwind" fn sqlite3_result_str(
    mut pCtx: *mut sqlite3_context,
    mut pStr: *mut sqlite3_str,
    mut eOwn: i32,
) {
    {}
    if (((unsafe { (*pStr).accError }) as u32) as i32) == (0 as i32) {
        if (unsafe { (*pStr).nChar }) == ((0 as i32) as u32) {
            setResultStrOrError(
                pCtx,
                (b"\0".as_ptr() as *mut i8) as *const i8,
                0 as i32,
                ((16 as i32) as i8) as u8,
                None,
            );
            unsafe { sqlite3_str_reset(pStr) };
        } else {
            let mut zText: *const i8 = (unsafe { sqlite3_str_value(pStr) }) as *const i8;
            // /* Only internal code has the ability to capture a pointer to
            //       ** an sqlite3_str object that uses static buffer.  And none of
            //       ** those internal use cases every invoke the sqlite3_result_str()
            //       ** interface on a static-buffer sqlite3_str.  Should this change
            //       ** in the future, the following assert() will let us know. */
            0 as i32;
            if eOwn == (0 as i32) {
                setResultStrOrError(
                    pCtx,
                    zText,
                    (unsafe { (*pStr).nChar }) as i32,
                    ((1 as i32) as i8) as u8,
                    unsafe {
                        std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                            -(1 as i32) as usize,
                        )
                    },
                );
            } else {
                setResultStrOrError(
                    pCtx,
                    zText,
                    (unsafe { (*pStr).nChar }) as i32,
                    ((16 as i32) as i8) as u8,
                    unsafe {
                        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                            sqlite3RowSetClear as *const (),
                        )
                    },
                );
                unsafe {
                    sqlite3StrAccumInit(
                        pStr,
                        unsafe { (*pStr).db },
                        std::ptr::null_mut::<i8>(),
                        0 as i32,
                        (unsafe { (*pStr).mxAlloc }) as i32,
                    )
                };
            }
        }
    } else {
        if (((unsafe { (*pStr).accError }) as u32) as i32) == (7 as i32) {
            sqlite3_result_error_nomem(pCtx);
        } else {
            0 as i32;
            sqlite3_result_error_toobig(pCtx);
        }
    }
    if eOwn == (2 as i32) {
        unsafe { sqlite3_str_free(pStr) };
    }
}

// /*
// ** Return the value of a status counter for a prepared statement
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_stmt_status")]
extern "C-unwind" fn sqlite3_stmt_status(
    mut pStmt: *mut sqlite3_stmt,
    mut op: i32,
    mut resetFlag: i32,
) -> i32 {
    let mut pVdbe: *mut Vdbe = pStmt as *mut Vdbe;
    let mut v: u32 = 0 as u32;
    if op == (99 as i32) {
        let mut db: *mut sqlite3 = unsafe { (*pVdbe).db };
        unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
        v = (0 as i32) as u32;
        unsafe {
            (*db).pnBytesFreed = std::ptr::addr_of_mut!(v) as *mut i32;
        }
        0 as i32;
        unsafe {
            (*db).lookaside.pEnd = unsafe { (*db).lookaside.pStart };
        }
        unsafe { sqlite3VdbeDelete(pVdbe) };
        unsafe {
            (*db).pnBytesFreed = std::ptr::null_mut::<i32>();
        }
        unsafe {
            (*db).lookaside.pEnd = unsafe { (*db).lookaside.pTrueEnd };
        }
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    } else {
        v = unsafe {
            *unsafe { unsafe { (*pVdbe).aCounter.as_mut_ptr() as *mut u32 }.offset(op as isize) }
        };
        if resetFlag != (0 as i32) {
            unsafe {
                *unsafe {
                    unsafe { (*pVdbe).aCounter.as_mut_ptr() as *mut u32 }.offset(op as isize)
                } = (0 as i32) as u32;
            }
        }
    }
    return v as i32;
}

// /*
// ** If this routine is invoked from within an xColumn method of a virtual
// ** table, then it returns true if and only if the the call is during an
// ** UPDATE operation and the value of the column will not be modified
// ** by the UPDATE.
// **
// ** If this routine is called from any context other than within the
// ** xColumn method of a virtual table, then the return value is meaningless
// ** and arbitrary.
// **
// ** Virtual table implements might use this routine to optimize their
// ** performance by substituting a NULL result, or some other light-weight
// ** value, as a signal to the xUpdate routine that the column is unchanged.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_vtab_nochange")]
extern "C-unwind" fn sqlite3_vtab_nochange(mut p: *mut sqlite3_context) -> i32 {
    0 as i32;
    return sqlite3_value_nochange(unsafe { (*p).pOut });
}

// /* Pointer to the ValueList object */
// /* Store the next value from the list here */
// /* 1 for _next(). 0 for _first() */
// /*
// ** Set the iterator value pVal to point to the first value in the set.
// ** Set (*ppOut) to point to this value before returning.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_vtab_in_first")]
extern "C-unwind" fn sqlite3_vtab_in_first(
    mut pVal: *mut sqlite3_value,
    mut ppOut: *mut *mut sqlite3_value,
) -> i32 {
    return valueFromValueList(pVal, ppOut, 0 as i32);
}

// /*
// ** Set the iterator value pVal to point to the next value in the set.
// ** Set (*ppOut) to point to this value before returning.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3_vtab_in_next")]
extern "C-unwind" fn sqlite3_vtab_in_next(
    mut pVal: *mut sqlite3_value,
    mut ppOut: *mut *mut sqlite3_value,
) -> i32 {
    return valueFromValueList(pVal, ppOut, 1 as i32);
}

// /* Force the INT64 value currently stored as the result to be
// ** a MEM_IntReal value.  See the SQLITE_TESTCTRL_RESULT_INTREAL
// ** test-control.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ResultIntReal(mut pCtx: *mut sqlite3_context) {
    0 as i32;
    if (((unsafe { (*unsafe { (*pCtx).pOut }).flags }) as u32) as i32) & (4 as i32) != (0 as i32) {
        let __v1289: *mut sqlite3_value = unsafe { (*pCtx).pOut };
        let __v1290: u16 = unsafe { (*__v1289).flags };
        let __v1291: u16 = ((((__v1290 as u32) as i32) & !(4 as i32)) as i16) as u16;
        unsafe {
            (*__v1289).flags = __v1291;
        }
        let __v1292: *mut sqlite3_value = unsafe { (*pCtx).pOut };
        let __v1293: u16 = unsafe { (*__v1292).flags };
        let __v1294: u16 = ((((__v1293 as u32) as i32) | (32 as i32)) as i16) as u16;
        unsafe {
            (*__v1292).flags = __v1294;
        }
    }
}

// /*
// ** Return the current time for a statement.  If the current time
// ** is requested more than once within the same run of a single prepared
// ** statement, the exact same time is returned for each invocation regardless
// ** of the amount of time that elapses between invocations.  In other words,
// ** the time returned is always the time of the first call.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3StmtCurrentTime(mut p: *mut sqlite3_context) -> i64 {
    let mut rc: i32 = 0 as i32;
    let mut piTime: *mut i64 =
        unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pVdbe }).iCurrentTime) };
    0 as i32;
    if (unsafe { *piTime }) == ((0 as i32) as i64) {
        rc = unsafe {
            sqlite3OsCurrentTimeInt64(
                unsafe { (*unsafe { (*unsafe { (*p).pOut }).db }).pVfs },
                piTime,
            )
        };
        if rc != (0 as i32) {
            unsafe {
                *piTime = (0 as i32) as i64;
            }
        }
    }
    return unsafe { *piTime };
}

// /*
// ** Given a wildcard parameter name, return the index of the variable
// ** with that name.  If there is no variable with the given name,
// ** return 0.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeParameterIndex(
    mut p: *mut Vdbe,
    mut zName: *const i8,
    mut nName: i32,
) -> i32 {
    if p == std::ptr::null_mut::<Vdbe>() || zName == std::ptr::null::<i8>() {
        return 0 as i32;
    }
    return unsafe { sqlite3VListNameToNum(unsafe { (*p).pVList }, zName, nName) };
}

// /*
// ** Transfer all bindings from the first statement over to the second.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TransferBindings(
    mut pFromStmt: *mut sqlite3_stmt,
    mut pToStmt: *mut sqlite3_stmt,
) -> i32 {
    let mut pFrom: *mut Vdbe = pFromStmt as *mut Vdbe;
    let mut pTo: *mut Vdbe = pToStmt as *mut Vdbe;
    let mut i: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*pTo).db }).mutex }) };
    i = 0 as i32;
    '__slate_break_1249: loop {
        if !(i < ((unsafe { (*pFrom).nVar }) as i32)) {
            break;
        }
        unsafe {
            sqlite3VdbeMemMove(
                unsafe { unsafe { (*pTo).aVar }.offset(i as isize) },
                unsafe { unsafe { (*pFrom).aVar }.offset(i as isize) },
            )
        };
        let __v1295: i32 = i;
        let __v1296: i32 = __v1295 + (1 as i32);
        i = __v1296;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*pTo).db }).mutex }) };
    return 0 as i32;
}

// /*
// ** The destructor function for a ValueList object.  This needs to be
// ** a separate function, unknowable to the application, to ensure that
// ** calls to sqlite3_vtab_in_first()/sqlite3_vtab_in_next() that are not
// ** preceded by activation of IN processing via sqlite3_vtab_int() do not
// ** try to access a fake ValueList object inserted by a hostile extension.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeapi.sqlite3VdbeValueListFree")]
extern "C-unwind" fn sqlite3VdbeValueListFree(mut pToDelete: *mut ()) {
    unsafe { sqlite3_free(pToDelete) };
}

// /*
// ** Check on a Vdbe to make sure it has not been finalized.  Log
// ** an error and return true if it has been finalized (or is otherwise
// ** invalid).  Return false if it is ok.
// */
fn vdbeSafety(mut p: *mut Vdbe) -> i32 {
    if (unsafe { (*p).db }) == std::ptr::null_mut::<sqlite3>() {
        unsafe {
            sqlite3_log(
                21 as i32,
                (b"API called with finalized prepared statement\0".as_ptr() as *mut i8)
                    as *const i8,
            )
        };
        return 1 as i32;
    } else {
        return 0 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

fn vdbeSafetyNotNull(mut p: *mut Vdbe) -> i32 {
    if p == std::ptr::null_mut::<Vdbe>() {
        unsafe {
            sqlite3_log(
                21 as i32,
                (b"API called with NULL prepared statement\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        return 1 as i32;
    } else {
        return vdbeSafety(p);
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Invoke the profile callback.  This routine is only called if we already
// ** know that the profile callback is defined and needs to be invoked.
// */
fn invokeProfileCallback(mut db: *mut sqlite3, mut p: *mut Vdbe) {
    let mut iNow: i64 = 0 as i64;
    let mut iElapse: i64 = 0 as i64;
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe { sqlite3OsCurrentTimeInt64(unsafe { (*db).pVfs }, std::ptr::addr_of_mut!(iNow)) };
    iElapse = (iNow - unsafe { (*p).startTime }) * ((1000000 as i32) as i64);
    if (unsafe { (*db).xProfile }) != None {
        unsafe {
            unsafe { (*db).xProfile }.unwrap()(
                unsafe { (*db).pProfileArg },
                (unsafe { (*p).zSql }) as *const i8,
                iElapse as u64,
            )
        };
    }
    if (((unsafe { (*db).mTrace }) as u32) as i32) & (2 as i32) != (0 as i32) {
        unsafe {
            unsafe { (*db).trace.xV2 }.unwrap()(
                (2 as i32) as u32,
                unsafe { (*db).pTraceArg },
                p as *mut (),
                std::ptr::addr_of_mut!(iElapse) as *mut (),
            )
        };
    }
    unsafe {
        (*p).startTime = (0 as i32) as i64;
    }
}

// /**************************** sqlite3_result_  *******************************
// ** The following routines are used by application-defined SQL functions to
// ** specify the function return value.  There are many variations on
// ** sqlite3_result_xxxx() for different types of return values.
// **
// ** The setStrOrError() function is a helper function that invokes
// ** sqlite3VdbeMemSetStr() to store the result as a string or blob.
// ** Appropriate errors are set if the string/blob is too big or if
// ** an OOM occurs.
// **
// ** The invokeValueDestructor(P,X) helper function invokes the destructor
// ** function X() on value P if P is not going to be used and need to
// ** be destroyed.
// */
fn setResultStrOrError(
    mut pCtx: *mut sqlite3_context,
    mut z: *const i8,
    mut n: i32,
    mut enc: u8,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) {
    let mut pOut: *mut sqlite3_value = unsafe { (*pCtx).pOut };
    let mut rc: i32 = 0 as i32;
    if ((enc as u32) as i32) == (1 as i32) {
        rc = unsafe { sqlite3VdbeMemSetText(pOut, z, n as i64, xDel) };
    } else {
        if ((enc as u32) as i32) == (16 as i32) {
            // /* It is usually considered improper to assert() on an input. However,
            //     ** the following assert() is checking for inputs that are documented
            //     ** to result in undefined behavior. */
            0 as i32;
            rc = unsafe { sqlite3VdbeMemSetText(pOut, z, n as i64, xDel) };
            let __v1297: *mut sqlite3_value = pOut;
            let __v1298: u16 = unsafe { (*__v1297).flags };
            let __v1299: u16 = ((((__v1298 as u32) as i32) | (512 as i32)) as i16) as u16;
            unsafe {
                (*__v1297).flags = __v1299;
            }
        } else {
            rc = unsafe { sqlite3VdbeMemSetStr(pOut, z, n as i64, enc, xDel) };
        }
    }
    if rc != (0 as i32) {
        if rc == (18 as i32) {
            sqlite3_result_error_toobig(pCtx);
        } else {
            // /* The only errors possible from sqlite3VdbeMemSetStr are
            //       ** SQLITE_TOOBIG and SQLITE_NOMEM */
            0 as i32;
            sqlite3_result_error_nomem(pCtx);
        }
        return;
    }
    unsafe { sqlite3VdbeChangeEncoding(pOut, ((unsafe { (*pCtx).enc }) as u32) as i32) };
    if (unsafe { sqlite3VdbeMemTooBig(pOut) }) != (0 as i32) {
        sqlite3_result_error_toobig(pCtx);
    }
}

// /* Function context */
// /* String pointer */
// /* Bytes in string, or negative */
// /* Encoding of z.  0 for BLOBs */
// /* Destructor function */
fn invokeValueDestructor(
    mut p: *const (),
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
    mut pCtx: *mut sqlite3_context,
) -> i32 {
    0 as i32;
    if xDel == None {
        // /* noop */
    } else {
        if xDel
            == unsafe {
                std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    -(1 as i32) as usize,
                )
            }
        {
            // /* noop */
        } else {
            unsafe { xDel.unwrap()(p as *mut ()) };
        }
    }
    0 as i32;
    sqlite3_result_error_toobig(pCtx);
    return 18 as i32;
}

// /*
// ** This function is called after a transaction has been committed. It
// ** invokes callbacks registered with sqlite3_wal_hook() as required.
// */
fn doWalCallbacks(mut db: *mut sqlite3) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_1230: loop {
        if !(i < unsafe { (*db).nDb }) {
            break;
        }
        let mut pBt: *mut Btree =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt };
        if pBt != std::ptr::null_mut::<Btree>() {
            let mut nEntry: i32 = 0 as i32;
            unsafe { sqlite3BtreeEnter(pBt) };
            nEntry = unsafe { sqlite3PagerWalCallback(unsafe { sqlite3BtreePager(pBt) }) };
            unsafe { sqlite3BtreeLeave(pBt) };
            if nEntry > (0 as i32) && (unsafe { (*db).xWalCallback }) != None && rc == (0 as i32) {
                rc = unsafe {
                    unsafe { (*db).xWalCallback }.unwrap()(
                        unsafe { (*db).pWalArg },
                        db,
                        (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).zDbSName })
                            as *const i8,
                        nEntry,
                    )
                };
            }
        }
        let __v1300: i32 = i;
        let __v1301: i32 = __v1300 + (1 as i32);
        i = __v1301;
    }
    return rc;
}

// /*
// ** Execute the statement pStmt, either until a row of data is ready, the
// ** statement is completely executed or an error occurs.
// **
// ** This routine implements the bulk of the logic behind the sqlite_step()
// ** API.  The only thing omitted is the automatic recompile if a
// ** schema change has occurred.  That detail is handled by the
// ** outer sqlite3_step() wrapper procedure.
// */
fn sqlite3Step(mut p: *mut Vdbe) -> i32 {
    let mut __slate_storage_1316: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1316: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1316) as *mut i32;
    let mut __slate_storage_1315: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1315: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1315) as *mut i32;
    let mut __slate_storage_1314: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1314: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1314) as *mut *mut sqlite3;
    let mut __slate_storage_1313: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1313: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1313) as *mut i32;
    let mut __slate_storage_1312: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1312: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1312) as *mut i32;
    let mut __slate_storage_1311: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1311: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1311) as *mut *mut sqlite3;
    let mut __slate_storage_1310: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1310: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1310) as *mut i32;
    let mut __slate_storage_1309: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1309: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1309) as *mut i32;
    let mut __slate_storage_1308: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1308: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1308) as *mut *mut sqlite3;
    let mut __slate_storage_1307: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1307: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1307) as *mut i32;
    let mut __slate_storage_1306: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1306: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1306) as *mut i32;
    let mut __slate_storage_1305: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1305: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1305) as *mut *mut sqlite3;
    let mut __slate_storage_1304: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1304: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1304) as *mut i32;
    let mut __slate_storage_1303: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1303: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1303) as *mut i32;
    let mut __slate_storage_1302: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1302: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1302) as *mut *mut sqlite3;
    let mut __slate_storage_662: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_662: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_662) as *mut i32;
    let mut __slate_storage_661: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_661: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_661) as *mut *mut sqlite3;
    unsafe {
        '__join_0: {
            '__join_15: {
                0 as i32;
                *__slate_slot_661 = unsafe { (*p).db };
                if (((unsafe { (*p).eVdbeState }) as u32) as i32) != (2 as i32) {
                    loop {
                        if (((unsafe { (*p).eVdbeState }) as u32) as i32) == (1 as i32) {
                            break;
                        } else {
                            if (((unsafe { (*p).eVdbeState }) as u32) as i32) == (3 as i32) {
                                // /* We used to require that sqlite3_reset() be called before retrying
                                //       ** sqlite3_step() after any error or after SQLITE_DONE.  But beginning
                                //       ** with version 3.7.0, we changed this so that sqlite3_reset() would
                                //       ** be called automatically instead of throwing the SQLITE_MISUSE error.
                                //       ** This "automatic-reset" change is not technically an incompatibility,
                                //       ** since any application that receives an SQLITE_MISUSE is broken by
                                //       ** definition.
                                //       **
                                //       ** Nevertheless, some published applications that were originally written
                                //       ** for version 3.6.23 or earlier do in fact depend on SQLITE_MISUSE
                                //       ** returns, and those were broken by the automatic-reset change.  As a
                                //       ** a work-around, the SQLITE_OMIT_AUTORESET compile-time restores the
                                //       ** legacy behavior of returning SQLITE_MISUSE for cases where the
                                //       ** previous sqlite3_step() returned something other than a SQLITE_LOCKED
                                //       ** or SQLITE_BUSY error.
                                //       */
                                sqlite3_reset(p as *mut sqlite3_stmt);
                                0 as i32;
                            } else {
                                break '__join_15;
                            }
                        }
                    }
                    if ((unsafe { (*p).__slate_bits_0.__get_expired() }) as i32) != (0 as i32) {
                        unsafe {
                            (*p).rc = 17 as i32;
                        }
                        *__slate_slot_662 = 1 as i32;
                        if (((unsafe { (*p).prepFlags }) as u32) as i32) & (128 as i32)
                            != (0 as i32)
                        {
                            // /* If this statement was prepared using saved SQL and an
                            //           ** error has occurred, then return the error code in p->rc to the
                            //           ** caller. Set the error code in the database handle to the same
                            //           ** value.
                            //           */
                            *__slate_slot_662 = unsafe { sqlite3VdbeTransferError(p) };
                            break '__join_0;
                        } else {
                            break '__join_0;
                        }
                    } else {
                        // /* If there are no other statements currently running, then
                        //       ** reset the interrupt flag.  This prevents a call to sqlite3_interrupt
                        //       ** from interrupting a statement that has not yet started.
                        //       */
                        if (unsafe { (*(*__slate_slot_661)).nVdbeActive }) == (0 as i32) {
                            unsafe {
                                std::sync::atomic::AtomicI32::store_volatile(
                                    std::sync::atomic::AtomicI32::from_ptr_raw(
                                        (unsafe {
                                            std::ptr::addr_of_mut!(
                                                (*(*__slate_slot_661)).u1.isInterrupted
                                            )
                                        }) as *mut i32,
                                    ),
                                    0 as i32,
                                    std::sync::atomic::Ordering::Relaxed,
                                )
                            };
                        }
                        0 as i32;
                        if (((unsafe { (*(*__slate_slot_661)).mTrace }) as u32) as i32)
                            & ((2 as i32) | (128 as i32))
                            != (0 as i32)
                            && !((unsafe { (*(*__slate_slot_661)).init.busy }) != (0 as u8))
                            && (unsafe { (*p).zSql }) != std::ptr::null_mut::<i8>()
                        {
                            unsafe {
                                sqlite3OsCurrentTimeInt64(
                                    unsafe { (*(*__slate_slot_661)).pVfs },
                                    unsafe { std::ptr::addr_of_mut!((*p).startTime) },
                                )
                            };
                        } else {
                            0 as i32;
                        }
                        std::ptr::write(__slate_slot_1302, *__slate_slot_661);
                        std::ptr::write(__slate_slot_1303, unsafe {
                            (*(*__slate_slot_1302)).nVdbeActive
                        });
                        std::ptr::write(__slate_slot_1304, *__slate_slot_1303 + (1 as i32));
                        unsafe {
                            (*(*__slate_slot_1302)).nVdbeActive = *__slate_slot_1304;
                        }
                        if ((unsafe { (*p).__slate_bits_0.__get_readOnly() }) as i32) == (0 as i32)
                        {
                            std::ptr::write(__slate_slot_1305, *__slate_slot_661);
                            std::ptr::write(__slate_slot_1306, unsafe {
                                (*(*__slate_slot_1305)).nVdbeWrite
                            });
                            std::ptr::write(__slate_slot_1307, *__slate_slot_1306 + (1 as i32));
                            unsafe {
                                (*(*__slate_slot_1305)).nVdbeWrite = *__slate_slot_1307;
                            }
                        }
                        if ((unsafe { (*p).__slate_bits_0.__get_bIsReader() }) as i32) != (0 as i32)
                        {
                            std::ptr::write(__slate_slot_1308, *__slate_slot_661);
                            std::ptr::write(__slate_slot_1309, unsafe {
                                (*(*__slate_slot_1308)).nVdbeRead
                            });
                            std::ptr::write(__slate_slot_1310, *__slate_slot_1309 + (1 as i32));
                            unsafe {
                                (*(*__slate_slot_1308)).nVdbeRead = *__slate_slot_1310;
                            }
                        }
                        unsafe {
                            (*p).pc = 0 as i32;
                        }
                        unsafe {
                            (*p).eVdbeState = ((2 as i32) as i8) as u8;
                        }
                    }
                }
            }
            if ((unsafe { (*p).__slate_bits_0.__get_explain() }) as i32) != (0 as i32) {
                *__slate_slot_662 = unsafe { sqlite3VdbeList(p) };
            } else {
                std::ptr::write(__slate_slot_1311, *__slate_slot_661);
                std::ptr::write(__slate_slot_1312, unsafe {
                    (*(*__slate_slot_1311)).nVdbeExec
                });
                std::ptr::write(__slate_slot_1313, *__slate_slot_1312 + (1 as i32));
                unsafe {
                    (*(*__slate_slot_1311)).nVdbeExec = *__slate_slot_1313;
                }
                *__slate_slot_662 = unsafe { sqlite3VdbeExec(p) };
                std::ptr::write(__slate_slot_1314, *__slate_slot_661);
                std::ptr::write(__slate_slot_1315, unsafe {
                    (*(*__slate_slot_1314)).nVdbeExec
                });
                std::ptr::write(__slate_slot_1316, *__slate_slot_1315 - (1 as i32));
                unsafe {
                    (*(*__slate_slot_1314)).nVdbeExec = *__slate_slot_1316;
                }
            }
            // /* SQLITE_OMIT_EXPLAIN */
            if *__slate_slot_662 == (100 as i32) {
                0 as i32;
                0 as i32;
                unsafe {
                    (*(*__slate_slot_661)).errCode = 100 as i32;
                }
                return 100 as i32;
            } else {
                // /* If the statement completed successfully, invoke the profile callback */
                if (unsafe { (*p).startTime }) > ((0 as i32) as i64) {
                    invokeProfileCallback(*__slate_slot_661, p);
                }
                {}
                unsafe {
                    (*p).pResultRow = std::ptr::null_mut::<sqlite3_value>();
                }
                if *__slate_slot_662 == (101 as i32)
                    && (unsafe { (*(*__slate_slot_661)).autoCommit }) != (0 as u8)
                {
                    0 as i32;
                    unsafe {
                        (*p).rc = doWalCallbacks(*__slate_slot_661);
                    }
                    if (unsafe { (*p).rc }) != (0 as i32) {
                        *__slate_slot_662 = 1 as i32;
                    }
                } else {
                    if *__slate_slot_662 != (101 as i32)
                        && (((unsafe { (*p).prepFlags }) as u32) as i32) & (128 as i32)
                            != (0 as i32)
                    {
                        // /* If this statement was prepared using saved SQL and an
                        //       ** error has occurred, then return the error code in p->rc to the
                        //       ** caller. Set the error code in the database handle to the same value.
                        //       */
                        *__slate_slot_662 = unsafe { sqlite3VdbeTransferError(p) };
                    }
                }
                unsafe {
                    (*(*__slate_slot_661)).errCode = *__slate_slot_662;
                }
                if (7 as i32) == unsafe { sqlite3ApiExit(unsafe { (*p).db }, unsafe { (*p).rc }) } {
                    unsafe {
                        (*p).rc = 7 as i32;
                    }
                    if (((unsafe { (*p).prepFlags }) as u32) as i32) & (128 as i32) != (0 as i32) {
                        *__slate_slot_662 = unsafe { (*p).rc };
                    }
                }
            }
        }
        0 as i32;
        // /* There are only a limited number of result codes allowed from the
        //   ** statements prepared using the legacy sqlite3_prepare() interface */
        return *__slate_slot_662 & unsafe { (*(*__slate_slot_661)).errMask };
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Implementation of sqlite3_vtab_in_first() (if bNext==0) and
// ** sqlite3_vtab_in_next() (if bNext!=0).
// */
fn valueFromValueList(
    mut pVal: *mut sqlite3_value,
    mut ppOut: *mut *mut sqlite3_value,
    mut bNext: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pRhs: *mut ValueList = unsafe { std::mem::zeroed() };
    unsafe {
        *ppOut = std::ptr::null_mut::<sqlite3_value>();
    }
    if pVal == std::ptr::null_mut::<sqlite3_value>() {
        return unsafe { sqlite3MisuseError(1108 as i32) };
    }
    if (((unsafe { (*pVal).flags }) as u32) as i32) & (4096 as i32) == (0 as i32)
        || (unsafe { (*pVal).xDel }) != Some(sqlite3VdbeValueListFree)
    {
        return 1 as i32;
    } else {
        0 as i32;
        0 as i32;
        0 as i32;
        pRhs = (unsafe { (*pVal).z }) as *mut ValueList;
    }
    if bNext != (0 as i32) {
        rc = unsafe { sqlite3BtreeNext(unsafe { (*pRhs).pCsr }, 0 as i32) };
    } else {
        let mut dummy: i32 = 0 as i32;
        rc = unsafe { sqlite3BtreeFirst(unsafe { (*pRhs).pCsr }, std::ptr::addr_of_mut!(dummy)) };
        0 as i32;
        if (unsafe { sqlite3BtreeEof(unsafe { (*pRhs).pCsr }) }) != (0 as i32) {
            rc = 101 as i32;
        }
    }
    if rc == (0 as i32) {
        // /* Size of current row in bytes */
        let mut sz: u32 = 0 as u32;
        // /* Raw content of current row */
        let mut sMem: sqlite3_value = unsafe { std::mem::zeroed() };
        unsafe { memset(std::ptr::addr_of_mut!(sMem) as *mut (), 0 as i32, 56 as u64) };
        sz = unsafe { sqlite3BtreePayloadSize(unsafe { (*pRhs).pCsr }) };
        rc = unsafe {
            sqlite3VdbeMemFromBtreeZeroOffset(
                unsafe { (*pRhs).pCsr },
                sz,
                std::ptr::addr_of_mut!(sMem),
            )
        };
        if rc == (0 as i32) {
            let mut zBuf: *mut u8 = sMem.z as *mut u8;
            let mut iSerial: u32 = 0 as u32;
            let mut pOut: *mut sqlite3_value = unsafe { (*pRhs).pOut };
            let mut iOff: i32 = 0 as i32;
            let __v1317: i32;
            if (((unsafe { *unsafe { zBuf.offset((1 as i32) as isize) } }) as u32) as i32)
                < (((((128 as i32) as i8) as u8) as u32) as i32)
            {
                iSerial = (unsafe { *unsafe { zBuf.offset((1 as i32) as isize) } }) as u32;
                __v1317 = 1 as i32;
            } else {
                __v1317 = ((unsafe {
                    sqlite3GetVarint32(
                        (unsafe { zBuf.offset((1 as i32) as isize) }) as *const u8,
                        std::ptr::addr_of_mut!(iSerial),
                    )
                }) as u32) as i32;
            }
            iOff = (1 as i32) + ((((__v1317 as i8) as u8) as u32) as i32);
            unsafe {
                sqlite3VdbeSerialGet(
                    (unsafe { zBuf.offset(iOff as isize) }) as *const u8,
                    iSerial,
                    pOut,
                )
            };
            unsafe {
                (*pOut).enc = unsafe { (*unsafe { (*pOut).db }).enc };
            }
            let __v1318: bool;
            if (((unsafe { (*pOut).flags }) as u32) as i32) & (16384 as i32) != (0 as i32) {
                __v1318 = (unsafe { sqlite3VdbeMemMakeWriteable(pOut) }) != (0 as i32);
            } else {
                __v1318 = false as bool;
            }
            if __v1318 {
                rc = 7 as i32;
            } else {
                unsafe {
                    *ppOut = pOut;
                }
            }
        }
        unsafe { sqlite3VdbeMemRelease(std::ptr::addr_of_mut!(sMem)) };
    }
    return rc;
}

// /*
// ** Create a new aggregate context for p and return a pointer to
// ** its pMem->z element.
// */
fn createAggContext(mut p: *mut sqlite3_context, mut nByte: i32) -> *mut () {
    let mut pMem: *mut sqlite3_value = unsafe { (*p).pMem };
    0 as i32;
    if nByte <= (0 as i32) {
        unsafe { sqlite3VdbeMemSetNull(pMem) };
        unsafe {
            (*pMem).z = std::ptr::null_mut::<i8>();
        }
    } else {
        unsafe { sqlite3VdbeMemClearAndResize(pMem, nByte) };
        unsafe {
            (*pMem).flags = ((32768 as i32) as i16) as u16;
        }
        unsafe {
            (*pMem).u.pDef = unsafe { (*p).pFunc };
        }
        if (unsafe { (*pMem).z }) != std::ptr::null_mut::<i8>() {
            unsafe {
                memset(
                    (unsafe { (*pMem).z }) as *mut (),
                    0 as i32,
                    (nByte as i64) as u64,
                )
            };
        }
    }
    return (unsafe { (*pMem).z }) as *mut ();
}

// /*
// ** Return a pointer to static memory containing an SQL NULL value.
// */
fn columnNullValue() -> *const sqlite3_value {
    // /* Even though the Mem structure contains an element
    //   ** of type i64, on certain architectures (x86) with certain compiler
    //   ** switches (-Os), gcc may align this Mem object on a 4-byte boundary
    //   ** instead of an 8-byte one. This all works fine, except that when
    //   ** running with SQLITE_DEBUG defined the SQLite code sometimes assert()s
    //   ** that a Mem structure is located on an 8-byte boundary. To prevent
    //   ** these assert()s from failing, when building with SQLITE_DEBUG defined
    //   ** using gcc, we force nullMem to be 8-byte aligned using the magical
    //   ** __attribute__((aligned(8))) macro.  */
    // /* .u          = */
    // /* .z          = */
    // /* .n          = */
    // /* .flags      = */
    // /* .enc        = */
    // /* .eSubtype   = */
    // /* .db         = */
    // /* .szMalloc   = */
    // /* .uTemp      = */
    // /* .zMalloc    = */
    // /* .xDel       = */
    return unsafe { std::ptr::addr_of!(nullMem) };
}

// /*
// ** Check to see if column iCol of the given statement is valid.  If
// ** it is, return a pointer to the Mem for the value of that column.
// ** If iCol is not valid, return a pointer to a Mem which has a value
// ** of NULL.
// */
fn columnMem(mut pStmt: *mut sqlite3_stmt, mut i: i32) -> *mut sqlite3_value {
    let mut pVm: *mut Vdbe = unsafe { std::mem::zeroed() };
    let mut pOut: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    pVm = pStmt as *mut Vdbe;
    if pVm == std::ptr::null_mut::<Vdbe>() {
        return columnNullValue() as *mut sqlite3_value;
    }
    0 as i32;
    unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*pVm).db }).mutex }) };
    if (unsafe { (*pVm).pResultRow }) != std::ptr::null_mut::<sqlite3_value>()
        && i < (((unsafe { (*pVm).nResColumn }) as u32) as i32)
        && i >= (0 as i32)
    {
        pOut = unsafe { unsafe { (*pVm).pResultRow }.offset(i as isize) };
    } else {
        unsafe { sqlite3Error(unsafe { (*pVm).db }, 25 as i32) };
        pOut = columnNullValue() as *mut sqlite3_value;
    }
    return pOut;
}

// /*
// ** This function is called after invoking an sqlite3_value_XXX function on a
// ** column value (i.e. a value returned by evaluating an SQL expression in the
// ** select list of a SELECT statement) that may cause a malloc() failure. If
// ** malloc() has failed, the threads mallocFailed flag is cleared and the result
// ** code of statement pStmt set to SQLITE_NOMEM.
// **
// ** Specifically, this is called from within:
// **
// **     sqlite3_column_int()
// **     sqlite3_column_int64()
// **     sqlite3_column_text()
// **     sqlite3_column_text16()
// **     sqlite3_column_double()
// **     sqlite3_column_bytes()
// **     sqlite3_column_bytes16()
// **     sqlite3_column_blob()
// */
fn columnMallocFailure(mut pStmt: *mut sqlite3_stmt) {
    // /* If malloc() failed during an encoding conversion within an
    //   ** sqlite3_column_XXX API, then set the return code of the statement to
    //   ** SQLITE_NOMEM. The next call to _step() (if any) will return SQLITE_ERROR
    //   ** and _finalize() will return NOMEM.
    //   */
    let mut p: *mut Vdbe = pStmt as *mut Vdbe;
    if p != std::ptr::null_mut::<Vdbe>() {
        0 as i32;
        0 as i32;
        unsafe {
            (*p).rc = unsafe { sqlite3ApiExit(unsafe { (*p).db }, unsafe { (*p).rc }) };
        }
        unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex }) };
    }
}

// /*
// ** Convert the N-th element of pStmt->pColName[] into a string using
// ** xFunc() then return that string.  If N is out of range, return 0.
// **
// ** There are up to 5 names for each column.  useType determines which
// ** name is returned.  Here are the names:
// **
// **    0      The column name as it should be displayed for output
// **    1      The datatype name for the column
// **    2      The name of the database that the column derives from
// **    3      The name of the table that the column derives from
// **    4      The name of the table column that the result column derives from
// **
// ** If the result is not a simple column reference (if it is an expression
// ** or a constant) then useTypes 2, 3, and 4 return NULL.
// */
fn columnName(
    mut pStmt: *mut sqlite3_stmt,
    mut N: i32,
    mut useUtf16: i32,
    mut useType: i32,
) -> *const () {
    let mut __slate_storage_1320: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1320: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1320) as *mut i32;
    let mut __slate_storage_1319: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1319: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1319) as *mut i32;
    let mut __slate_storage_769: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_769: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_769) as *mut u8;
    let mut __slate_storage_768: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_768: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_768) as *mut i32;
    let mut __slate_storage_767: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_767: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_767) as *mut *mut sqlite3;
    let mut __slate_storage_766: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_766: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_766) as *mut i32;
    let mut __slate_storage_765: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_765: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_765) as *mut *mut Vdbe;
    let mut __slate_storage_764: std::mem::MaybeUninit<*const ()> = std::mem::MaybeUninit::uninit();
    let __slate_slot_764: *mut *const () =
        std::ptr::addr_of_mut!(__slate_storage_764) as *mut *const ();
    unsafe {
        if N < (0 as i32) {
            return std::ptr::null::<()>();
        } else {
            *__slate_slot_764 = std::ptr::null::<()>();
            *__slate_slot_765 = pStmt as *mut Vdbe;
            *__slate_slot_767 = unsafe { (*(*__slate_slot_765)).db };
            0 as i32;
            unsafe { sqlite3_mutex_enter(unsafe { (*(*__slate_slot_767)).mutex }) };
            if ((unsafe { (*(*__slate_slot_765)).__slate_bits_0.__get_explain() }) as i32)
                != (0 as i32)
            {
                if useType > (0 as i32) {
                } else {
                    *__slate_slot_766 =
                        if ((unsafe { (*(*__slate_slot_765)).__slate_bits_0.__get_explain() })
                            as i32)
                            == (1 as i32)
                        {
                            8 as i32
                        } else {
                            4 as i32
                        };
                    if N >= *__slate_slot_766 {
                    } else {
                        if useUtf16 != (0 as i32) {
                            std::ptr::write(
                                __slate_slot_768,
                                ((unsafe {
                                    *unsafe {
                                        unsafe {
                                            std::ptr::addr_of!(iExplainColNames16) as *const u8
                                        }
                                        .offset(
                                            (N + (8 as i32)
                                                * ((unsafe {
                                                    (*(*__slate_slot_765))
                                                        .__slate_bits_0
                                                        .__get_explain()
                                                })
                                                    as i32)
                                                - (8 as i32))
                                                as isize,
                                        )
                                    }
                                }) as u32) as i32,
                            );
                            *__slate_slot_764 = ((unsafe {
                                unsafe {
                                    std::ptr::addr_of!(azExplainColNames16data.0) as *const u16
                                }
                                .offset(*__slate_slot_768 as isize)
                            }) as *mut ())
                                as *const ();
                        } else {
                            *__slate_slot_764 = ((unsafe {
                                *unsafe {
                                    unsafe {
                                        std::ptr::addr_of!(azExplainColNames8.0) as *const *const i8
                                    }
                                    .offset(
                                        (N + (8 as i32)
                                            * ((unsafe {
                                                (*(*__slate_slot_765))
                                                    .__slate_bits_0
                                                    .__get_explain()
                                            })
                                                as i32)
                                            - (8 as i32))
                                            as isize,
                                    )
                                }
                            }) as *mut ())
                                as *const ();
                        }
                    }
                }
            } else {
                *__slate_slot_766 = ((unsafe { (*(*__slate_slot_765)).nResColumn }) as u32) as i32;
                if N < *__slate_slot_766 {
                    std::ptr::write(__slate_slot_769, unsafe {
                        (*(*__slate_slot_767)).mallocFailed
                    });
                    std::ptr::write(__slate_slot_1319, N);
                    std::ptr::write(
                        __slate_slot_1320,
                        *__slate_slot_1319 + useType * *__slate_slot_766,
                    );
                    N = *__slate_slot_1320;
                    if useUtf16 != (0 as i32) {
                        *__slate_slot_764 = sqlite3_value_text16(unsafe {
                            unsafe { (*(*__slate_slot_765)).aColName }.offset(N as isize)
                        });
                    } else {
                        *__slate_slot_764 = sqlite3_value_text(unsafe {
                            unsafe { (*(*__slate_slot_765)).aColName }.offset(N as isize)
                        }) as *const ();
                    }
                    // /* A malloc may have failed inside of the _text() call. If this
                    //     ** is the case, clear the mallocFailed flag and return NULL.
                    //     */
                    0 as i32;
                    if (((unsafe { (*(*__slate_slot_767)).mallocFailed }) as u32) as i32)
                        > ((*__slate_slot_769 as u32) as i32)
                    {
                        unsafe { sqlite3OomClear(*__slate_slot_767) };
                        *__slate_slot_764 = std::ptr::null::<()>();
                    }
                }
            }
            unsafe { sqlite3_mutex_leave(unsafe { (*(*__slate_slot_767)).mutex }) };
            return *__slate_slot_764;
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /* SQLITE_OMIT_UTF16 */
// /* SQLITE_OMIT_DECLTYPE */
// /******************************* sqlite3_bind_  ***************************
// **
// ** Routines used to attach values to wildcards in a compiled SQL statement.
// */
// /*
// ** Unbind the value bound to variable i in virtual machine p. This is the
// ** the same as binding a NULL value to the column. If the "i" parameter is
// ** out of range, then SQLITE_RANGE is returned. Otherwise SQLITE_OK.
// **
// ** A successful evaluation of this routine acquires the mutex on p.
// ** the mutex is released if any kind of error occurs.
// **
// ** The error code stored in database p->db is overwritten with the return
// ** value in any case.
// **
// ** (tag-20240917-01) If  vdbeUnbind(p,(u32)(i-1))  returns SQLITE_OK,
// ** that means all of the the following will be true:
// **
// **     p!=0
// **     p->pVar!=0
// **     i>0
// **     i<=p->nVar
// **
// ** An assert() is normally added after vdbeUnbind() to help static analyzers
// ** realize this.
// */
fn vdbeUnbind(mut p: *mut Vdbe, mut i: u32) -> i32 {
    let mut pVar: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    if vdbeSafetyNotNull(p) != (0 as i32) {
        return unsafe { sqlite3MisuseError(1724 as i32) };
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*p).db }).mutex }) };
    if (((unsafe { (*p).eVdbeState }) as u32) as i32) != (1 as i32) {
        unsafe {
            sqlite3Error(unsafe { (*p).db }, unsafe {
                sqlite3MisuseError(1728 as i32)
            })
        };
        unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex }) };
        unsafe {
            sqlite3_log(
                21 as i32,
                (b"bind on a busy prepared statement: [%s]\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*p).zSql },
            )
        };
        return unsafe { sqlite3MisuseError(1732 as i32) };
    }
    if i >= (((unsafe { (*p).nVar }) as i32) as u32) {
        unsafe { sqlite3Error(unsafe { (*p).db }, 25 as i32) };
        unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex }) };
        return 25 as i32;
    }
    pVar = unsafe { unsafe { (*p).aVar }.offset(i as isize) };
    unsafe { sqlite3VdbeMemRelease(pVar) };
    unsafe {
        (*pVar).flags = ((1 as i32) as i16) as u16;
    }
    unsafe {
        (*unsafe { (*p).db }).errCode = 0 as i32;
    }
    // /* If the bit corresponding to this variable in Vdbe.expmask is set, then
    //   ** binding a new value to this variable invalidates the current query plan.
    //   **
    //   ** IMPLEMENTATION-OF: R-57496-20354 If the specific value bound to a host
    //   ** parameter in the WHERE clause might influence the choice of query plan
    //   ** for a statement, then the statement will be automatically recompiled,
    //   ** as if there had been a schema change, on the first sqlite3_step() call
    //   ** following any change to the bindings of that parameter.
    //   */
    0 as i32;
    0 as i32;
    if (unsafe { (*p).expmask }) != ((0 as i32) as u32)
        && (unsafe { (*p).expmask })
            & if i >= ((31 as i32) as u32) {
                2147483648 as u32
            } else {
                ((1 as i32) as u32) << i
            }
            != ((0 as i32) as u32)
    {
        // /* We might avoid a reprepare here if p->smimask is set and the old
        //     ** value is an integer other than (0,1).  But that is such a corner
        //     ** case that it does not seem worth the extra code to implement. */
        unsafe {
            (*p).__slate_bits_0.__set_expired((1 as i32) as u32);
        }
    }
    return 0 as i32;
}

// /*
// ** Bind a text or BLOB value.
// */
fn bindText(
    mut pStmt: *mut sqlite3_stmt,
    mut i: i32,
    mut zData: *const (),
    mut nData: i64,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
    mut encoding: u8,
) -> i32 {
    let mut p: *mut Vdbe = pStmt as *mut Vdbe;
    let mut pVar: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    rc = vdbeUnbind(p, (i - (1 as i32)) as u32);
    if rc == (0 as i32) {
        // /* tag-20240917-01 */
        0 as i32;
        if zData != std::ptr::null::<()>() {
            pVar = unsafe { unsafe { (*p).aVar }.offset((i - (1 as i32)) as isize) };
            if ((encoding as u32) as i32) == (1 as i32) {
                rc = unsafe { sqlite3VdbeMemSetText(pVar, zData as *const i8, nData, xDel) };
            } else {
                if ((encoding as u32) as i32) == (16 as i32) {
                    // /* It is usually consider improper to assert() on an input.
                    //         ** However, the following assert() is checking for inputs
                    //         ** that are documented to result in undefined behavior. */
                    0 as i32;
                    rc = unsafe { sqlite3VdbeMemSetText(pVar, zData as *const i8, nData, xDel) };
                    let __v1321: *mut sqlite3_value = pVar;
                    let __v1322: u16 = unsafe { (*__v1321).flags };
                    let __v1323: u16 = ((((__v1322 as u32) as i32) | (512 as i32)) as i16) as u16;
                    unsafe {
                        (*__v1321).flags = __v1323;
                    }
                } else {
                    rc = unsafe {
                        sqlite3VdbeMemSetStr(pVar, zData as *const i8, nData, encoding, xDel)
                    };
                    if ((encoding as u32) as i32) == (0 as i32) {
                        unsafe {
                            (*pVar).enc = unsafe { (*unsafe { (*p).db }).enc };
                        }
                    }
                }
            }
            if rc == (0 as i32) && ((encoding as u32) as i32) != (0 as i32) {
                rc = unsafe {
                    sqlite3VdbeChangeEncoding(
                        pVar,
                        ((unsafe { (*unsafe { (*p).db }).enc }) as u32) as i32,
                    )
                };
            }
            if rc != (0 as i32) {
                unsafe { sqlite3Error(unsafe { (*p).db }, rc) };
                rc = unsafe { sqlite3ApiExit(unsafe { (*p).db }, rc) };
            }
        }
        unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).db }).mutex }) };
    } else {
        if xDel != None
            && xDel
                != unsafe {
                    std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        -(1 as i32) as usize,
                    )
                }
        {
            unsafe { xDel.unwrap()(zData as *mut ()) };
        }
    }
    return rc;
}
