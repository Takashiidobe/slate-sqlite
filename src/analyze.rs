unsafe extern "C" {
    fn sqlite3_exec(
        __v510: *mut sqlite3,
        sql: *const i8,
        callback: Option<
            unsafe extern "C-unwind" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32,
        >,
        __v513: *mut (),
        errmsg: *mut *mut i8,
    ) -> i32;
    fn sqlite3_value_blob(__v515: *mut sqlite3_value) -> *const ();
    fn sqlite3_value_int(__v516: *mut sqlite3_value) -> i32;
    fn sqlite3_value_int64(__v517: *mut sqlite3_value) -> i64;
    fn sqlite3_context_db_handle(__v518: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_blob(
        __v519: *mut sqlite3_context,
        __v520: *const (),
        __v521: i32,
        __v522: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_error_nomem(__v523: *mut sqlite3_context);
    fn sqlite3_result_int(__v524: *mut sqlite3_context, __v525: i32);
    fn sqlite3_result_str(__v526: *mut sqlite3_context, __v527: *mut sqlite3_str, __v528: i32);
    fn sqlite3_str_appendf(__v529: *mut sqlite3_str, zFormat: *const i8, ...);
    fn sqlite3_stricmp(__v531: *const i8, __v532: *const i8) -> i32;
    fn sqlite3_strglob(zGlob: *const i8, zStr: *const i8) -> i32;
    fn sqlite3_strlike(zGlob: *const i8, zStr: *const i8, cEsc: u32) -> i32;
    fn sqlite3VdbeAddOp0(__v538: *mut Vdbe, __v539: i32) -> i32;
    fn sqlite3VdbeAddOp1(__v540: *mut Vdbe, __v541: i32, __v542: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v543: *mut Vdbe, __v544: i32, __v545: i32, __v546: i32) -> i32;
    fn sqlite3VdbeGoto(__v547: *mut Vdbe, __v548: i32) -> i32;
    fn sqlite3VdbeLoadString(__v549: *mut Vdbe, __v550: i32, __v551: *const i8) -> i32;
    fn sqlite3VdbeAddOp3(
        __v552: *mut Vdbe,
        __v553: i32,
        __v554: i32,
        __v555: i32,
        __v556: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4(
        __v557: *mut Vdbe,
        __v558: i32,
        __v559: i32,
        __v560: i32,
        __v561: i32,
        zP4: *const i8,
        __v563: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4Int(
        __v564: *mut Vdbe,
        __v565: i32,
        __v566: i32,
        __v567: i32,
        __v568: i32,
        __v569: i32,
    ) -> i32;
    fn sqlite3VdbeAddFunctionCall(
        __v570: *mut Parse,
        __v571: i32,
        __v572: i32,
        __v573: i32,
        __v574: i32,
        __v575: *const FuncDef,
        __v576: i32,
    ) -> i32;
    fn sqlite3VdbeChangeP5(__v577: *mut Vdbe, P5: u16);
    fn sqlite3VdbeJumpHere(__v579: *mut Vdbe, addr: i32);
    fn sqlite3VdbeSetP4KeyInfo(__v581: *mut Parse, __v582: *mut Index);
    fn sqlite3VdbeMakeLabel(__v583: *mut Parse) -> i32;
    fn sqlite3VdbeResolveLabel(__v584: *mut Vdbe, __v585: i32);
    fn sqlite3VdbeCurrentAddr(__v586: *mut Vdbe) -> i32;
    fn sqlite3DbMallocZero(__v587: *mut sqlite3, __v588: u64) -> *mut ();
    fn sqlite3DbMallocRawNN(__v589: *mut sqlite3, __v590: u64) -> *mut ();
    fn sqlite3DbFree(__v591: *mut sqlite3, __v592: *mut ());
    fn sqlite3MPrintf(__v593: *mut sqlite3, __v594: *const i8, ...) -> *mut i8;
    fn sqlite3TouchRegister(__v595: *mut Parse, __v596: i32);
    fn sqlite3PrimaryKeyIndex(__v597: *mut Table) -> *mut Index;
    fn sqlite3OpenTable(__v598: *mut Parse, iCur: i32, iDb: i32, __v601: *mut Table, __v602: i32);
    fn sqlite3FindTable(__v603: *mut sqlite3, __v604: *const i8, __v605: *const i8) -> *mut Table;
    fn sqlite3LocateTable(
        __v606: *mut Parse,
        flags: u32,
        __v608: *const i8,
        __v609: *const i8,
    ) -> *mut Table;
    fn sqlite3FindIndex(__v610: *mut sqlite3, __v611: *const i8, __v612: *const i8) -> *mut Index;
    fn sqlite3NameFromToken(__v613: *mut sqlite3, __v614: *const Token) -> *mut i8;
    fn sqlite3GetVdbe(__v615: *mut Parse) -> *mut Vdbe;
    fn sqlite3BeginWriteOperation(__v616: *mut Parse, __v617: i32, __v618: i32);
    fn sqlite3AuthCheck(
        __v619: *mut Parse,
        __v620: i32,
        __v621: *const i8,
        __v622: *const i8,
        __v623: *const i8,
    ) -> i32;
    fn sqlite3Atoi(__v624: *const i8) -> i32;
    fn sqlite3LogEst(__v625: u64) -> i16;
    fn sqlite3TwoPartName(
        __v626: *mut Parse,
        __v627: *mut Token,
        __v628: *mut Token,
        __v629: *mut *mut Token,
    ) -> i32;
    fn sqlite3ReadSchema(pParse: *mut Parse) -> i32;
    fn sqlite3LocateCollSeq(pParse: *mut Parse, zName: *const i8) -> *mut CollSeq;
    fn sqlite3NestedParse(__v633: *mut Parse, __v634: *const i8, ...);
    fn sqlite3FindDb(__v638: *mut sqlite3, __v639: *mut Token) -> i32;
    fn sqlite3DefaultRowEst(__v644: *mut Index);
    fn sqlite3SchemaToIndex(db: *mut sqlite3, __v646: *mut Schema) -> i32;
    fn sqlite3OomFault(__v647: *mut sqlite3) -> *mut ();
    fn sqlite3StrAccumInit(
        __v648: *mut sqlite3_str,
        __v649: *mut sqlite3,
        __v650: *mut i8,
        __v651: i32,
        __v652: i32,
    );
    fn sqlite3TableLock(
        __v653: *mut Parse,
        __v654: i32,
        __v655: u32,
        __v656: u8,
        __v657: *const i8,
    );
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
    trace: __SlateRecord157,
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
    u1: __SlateRecord158,
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
    __slate_bits_0: __slate_bits::__SlateBits63U0,
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
    u: __SlateRecord168,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord169,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord170,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord171,
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
    u: __SlateRecord159,
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
    __slate_bits_0: __slate_bits::__SlateBits87U0,
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
    __slate_bits_0: __slate_bits::__SlateBits99U0,
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
    u1: __SlateRecord184,
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
    fg: __SlateRecord178,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord179,
    u2: __SlateRecord180,
    u3: __SlateRecord181,
    u4: __SlateRecord182,
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
    u: __SlateRecord160,
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
    __slate_bits_0: __slate_bits::__SlateBits156U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord157 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord158 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord159 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord160 {
    tab: __SlateRecord161,
    view: __SlateRecord162,
    vtab: __SlateRecord163,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord161 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord162 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord163 {
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
union __SlateRecord168 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord169 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord170 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord171 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord172,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord172 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord174,
    u: __SlateRecord175,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord174 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits174U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord175 {
    x: __SlateRecord176,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord176 {
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
struct __SlateRecord178 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits178U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord184 {
    cr: __SlateRecord185,
    d: __SlateRecord186,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord185 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord186 {
    pReturning: *mut Returning,
}

// /* Parsing context */
// /* The database we are looking in */
// /* Open the sqlite_stat1 table on this cursor */
// /* Delete entries for this table or index */
// /* Either "tbl" or "idx" */
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord187 {
    zName: *const i8,
    zCols: *const i8,
}

// /*
// ** Recommended number of samples for sqlite_stat4
// */
// /*
// ** Three SQL functions - stat_init(), stat_push(), and stat_get() -
// ** share an instance of the following structure to hold their state
// ** information.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct StatAccum {
    // /* sqlite_stat4.nDLt */
    db: *mut sqlite3,
    // /* Database connection, for malloc() */
    nEst: u64,
    // /* Estimated number of rows */
    nRow: u64,
    // /* Number of rows visited so far */
    nLimit: i32,
    // /* Analysis row-scan limit */
    nCol: i32,
    // /* Number of columns in index + pk/rowid */
    nKeyCol: i32,
    // /* Number of index columns w/o the pk/rowid */
    nSkipAhead: u8,
    // /* Number of times of skip-ahead */
    current: StatSample,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct StatSample {
    anDLt: *mut u64,
}

// /*
// ** Used to pass information from the analyzer reader through to the
// ** callback routine.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct analysisInfo {
    db: *mut sqlite3,
    zDatabase: *const i8,
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
    pub struct __SlateBits63U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits174U0 {
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
    pub struct __SlateBits178U0 {
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
    pub struct __SlateBits87U0 {
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
    pub struct __SlateBits156U0 {
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
    pub struct __SlateBits99U0 {
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

static mut aTable: __SlateAlign16<[__SlateRecord187; 3]> = __SlateAlign16([
    __SlateRecord187 {
        zName: (b"sqlite_stat1\0".as_ptr() as *mut i8) as *const i8,
        zCols: (b"tbl,idx,stat\0".as_ptr() as *mut i8) as *const i8,
    },
    __SlateRecord187 {
        zName: (b"sqlite_stat4\0".as_ptr() as *mut i8) as *const i8,
        zCols: std::ptr::null::<i8>(),
    },
    __SlateRecord187 {
        zName: (b"sqlite_stat3\0".as_ptr() as *mut i8) as *const i8,
        zCols: std::ptr::null::<i8>(),
    },
]);

static mut statInitFuncdef: FuncDef = FuncDef {
    nArg: (4 as i32) as i16,
    funcFlags: (1 as i32) as u32,
    pUserData: std::ptr::null_mut::<()>(),
    pNext: std::ptr::null_mut::<FuncDef>(),
    xSFunc: Some(statInit),
    xFinalize: None,
    xValue: None,
    xInverse: None,
    zName: (b"stat_init\0".as_ptr() as *mut i8) as *const i8,
    u: {
        let mut __t0: __SlateRecord159 = unsafe { std::mem::zeroed() };
        __t0.pHash = std::ptr::null_mut::<FuncDef>();
        __t0
    },
};

static mut statPushFuncdef: FuncDef = FuncDef {
    nArg: ((2 as i32) + (0 as i32)) as i16,
    funcFlags: (1 as i32) as u32,
    pUserData: std::ptr::null_mut::<()>(),
    pNext: std::ptr::null_mut::<FuncDef>(),
    xSFunc: Some(statPush),
    xFinalize: None,
    xValue: None,
    xInverse: None,
    zName: (b"stat_push\0".as_ptr() as *mut i8) as *const i8,
    u: {
        let mut __t0: __SlateRecord159 = unsafe { std::mem::zeroed() };
        __t0.pHash = std::ptr::null_mut::<FuncDef>();
        __t0
    },
};

static mut statGetFuncdef: FuncDef = FuncDef {
    nArg: ((1 as i32) + (0 as i32)) as i16,
    funcFlags: (1 as i32) as u32,
    pUserData: std::ptr::null_mut::<()>(),
    pNext: std::ptr::null_mut::<FuncDef>(),
    xSFunc: Some(statGet),
    xFinalize: None,
    xValue: None,
    xInverse: None,
    zName: (b"stat_get\0".as_ptr() as *mut i8) as *const i8,
    u: {
        let mut __t0: __SlateRecord159 = unsafe { std::mem::zeroed() };
        __t0.pHash = std::ptr::null_mut::<FuncDef>();
        __t0
    },
};

// /*
// ** Generate code for the ANALYZE command.  The parser calls this routine
// ** when it recognizes an ANALYZE command.
// **
// **        ANALYZE                            -- 1
// **        ANALYZE  <database>                -- 2
// **        ANALYZE  ?<database>.?<tablename>  -- 3
// **
// ** Form 1 causes all indices in all attached databases to be analyzed.
// ** Form 2 analyzes all indices the single database named.
// ** Form 3 analyzes all indices associated with the named table.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Analyze(
    mut pParse: *mut Parse,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut iDb: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    let mut zDb: *mut i8 = unsafe { std::mem::zeroed() };
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
    let mut pTableName: *mut Token = unsafe { std::mem::zeroed() };
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    // /* Read the database schema. If an error occurs, leave an error message
    //   ** and code in pParse and return NULL. */
    0 as i32;
    if (0 as i32) != unsafe { sqlite3ReadSchema(pParse) } {
        return;
    }
    0 as i32;
    if pName1 == std::ptr::null_mut::<Token>() {
        // /* Form 1:  Analyze everything */
        i = 0 as i32;
        '__slate_break_682: loop {
            if !(i < unsafe { (*db).nDb }) {
                break;
            }
            // /* Do not analyze the TEMP database */
            if i == (1 as i32) {
            } else {
                analyzeDatabase(pParse, i);
            }
            let __v696: i32 = i;
            let __v697: i32 = __v696 + (1 as i32);
            i = __v697;
        }
    } else {
        let __v698: bool;
        if (unsafe { (*pName2).n }) == ((0 as i32) as u32) {
            let __v699: i32 = unsafe { sqlite3FindDb(db, pName1) };
            iDb = __v699;
            __v698 = __v699 >= (0 as i32);
        } else {
            __v698 = false as bool;
        }
        if __v698 {
            // /* Analyze the schema named as the argument */
            analyzeDatabase(pParse, iDb);
        } else {
            // /* Form 3: Analyze the table or index named as an argument */
            iDb = unsafe {
                sqlite3TwoPartName(pParse, pName1, pName2, std::ptr::addr_of_mut!(pTableName))
            };
            if iDb >= (0 as i32) {
                zDb = if (unsafe { (*pName2).n }) != (0 as u32) {
                    unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName }
                } else {
                    std::ptr::null_mut::<i8>()
                };
                z = unsafe { sqlite3NameFromToken(db, pTableName as *const Token) };
                if z != std::ptr::null_mut::<i8>() {
                    let __v700: *mut Index =
                        unsafe { sqlite3FindIndex(db, z as *const i8, zDb as *const i8) };
                    pIdx = __v700;
                    if __v700 != std::ptr::null_mut::<Index>() {
                        analyzeTable(pParse, unsafe { (*pIdx).pTable }, pIdx);
                    } else {
                        let __v701: *mut Table = unsafe {
                            sqlite3LocateTable(
                                pParse,
                                (0 as i32) as u32,
                                z as *const i8,
                                zDb as *const i8,
                            )
                        };
                        pTab = __v701;
                        if __v701 != std::ptr::null_mut::<Table>() {
                            analyzeTable(pParse, pTab, std::ptr::null_mut::<Index>());
                        }
                    }
                    unsafe { sqlite3DbFree(db, z as *mut ()) };
                }
            }
        }
    }
    let __v702: bool;
    if (((unsafe { (*db).nSqlExec }) as u32) as i32) == (0 as i32) {
        let __v703: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
        v = __v703;
        __v702 = __v703 != std::ptr::null_mut::<Vdbe>();
    } else {
        __v702 = false as bool;
    }
    if __v702 {
        unsafe { sqlite3VdbeAddOp0(v, 168 as i32) };
    }
}

// /*
// ** Load the content of the sqlite_stat1 and sqlite_stat4 tables. The
// ** contents of sqlite_stat1 are used to populate the Index.aiRowEst[]
// ** arrays. The contents of sqlite_stat4 are used to populate the
// ** Index.aSample[] arrays.
// **
// ** If the sqlite_stat1 table is not present in the database, SQLITE_ERROR
// ** is returned. In this case, even if SQLITE_ENABLE_STAT4 was defined
// ** during compilation and the sqlite_stat4 table is present, no data is
// ** read from it.
// **
// ** If SQLITE_ENABLE_STAT4 was defined during compilation and the
// ** sqlite_stat4 table is not present in the database, SQLITE_ERROR is
// ** returned. However, in this case, data is read from the sqlite_stat1
// ** table (if it is present) before returning.
// **
// ** If an OOM error occurs, this function always sets db->mallocFailed.
// ** This means if the caller does not care about other errors, the return
// ** code may be ignored.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AnalysisLoad(mut db: *mut sqlite3, mut iDb: i32) -> i32 {
    let mut sInfo: analysisInfo = unsafe { std::mem::zeroed() };
    let mut i: *mut HashElem = unsafe { std::mem::zeroed() };
    let mut zSql: *mut i8 = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    let mut pSchema: *mut Schema =
        unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pSchema };
    let mut pStat1: *const Table = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    // /* Clear any prior statistics */
    0 as i32;
    i = unsafe { (*unsafe { std::ptr::addr_of_mut!((*pSchema).tblHash) }).first };
    '__slate_break_691: while i != std::ptr::null_mut::<HashElem>() {
        let mut pTab: *mut Table = (unsafe { (*i).data }) as *mut Table;
        let __v704: *mut Table = pTab;
        let __v705: u32 = unsafe { (*__v704).tabFlags };
        let __v706: u32 = __v705 & (!(16 as i32) as u32);
        unsafe {
            (*__v704).tabFlags = __v706;
        }
        i = unsafe { (*i).next };
    }
    i = unsafe { (*unsafe { std::ptr::addr_of_mut!((*pSchema).idxHash) }).first };
    '__slate_break_692: while i != std::ptr::null_mut::<HashElem>() {
        let mut pIdx: *mut Index = (unsafe { (*i).data }) as *mut Index;
        unsafe {
            (*pIdx).__slate_bits_0.__set_hasStat1((0 as i32) as u32);
        }
        i = unsafe { (*i).next };
    }
    // /* Load new statistics out of the sqlite_stat1 table */
    sInfo.db = db;
    sInfo.zDatabase =
        (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName }) as *const i8;
    let __v707: *const Table = (unsafe {
        sqlite3FindTable(
            db,
            (b"sqlite_stat1\0".as_ptr() as *mut i8) as *const i8,
            sInfo.zDatabase,
        )
    }) as *const Table;
    pStat1 = __v707;
    if __v707 != std::ptr::null::<Table>()
        && (((unsafe { (*pStat1).eTabType }) as u32) as i32) == (0 as i32)
    {
        zSql = unsafe {
            sqlite3MPrintf(
                db,
                (b"SELECT tbl,idx,stat FROM %Q.sqlite_stat1\0".as_ptr() as *mut i8) as *const i8,
                sInfo.zDatabase,
            )
        };
        if zSql == std::ptr::null_mut::<i8>() {
            rc = 7 as i32;
        } else {
            rc = unsafe {
                sqlite3_exec(
                    db,
                    zSql as *const i8,
                    Some(analysisLoader),
                    std::ptr::addr_of_mut!(sInfo) as *mut (),
                    std::ptr::null_mut::<*mut i8>(),
                )
            };
            unsafe { sqlite3DbFree(db, zSql as *mut ()) };
        }
    }
    // /* Set appropriate defaults on all indexes not in the sqlite_stat1 table */
    0 as i32;
    i = unsafe { (*unsafe { std::ptr::addr_of_mut!((*pSchema).idxHash) }).first };
    '__slate_break_695: while i != std::ptr::null_mut::<HashElem>() {
        let mut pIdx: *mut Index = (unsafe { (*i).data }) as *mut Index;
        if !(((unsafe { (*pIdx).__slate_bits_0.__get_hasStat1() }) as i32) != (0 as i32)) {
            unsafe { sqlite3DefaultRowEst(pIdx) };
        }
        i = unsafe { (*i).next };
    }
    // /* Load the statistics from the sqlite_stat4 table. */
    if rc == (7 as i32) {
        unsafe { sqlite3OomFault(db) };
    }
    return rc;
}

// /*
// ** If the Index.aSample variable is not NULL, delete the aSample[] array
// ** and its contents.
// */
// /* SQLITE_OMIT_ANALYZE */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DeleteIndexSamples(mut db: *mut sqlite3, mut pIdx: *mut Index) {
    0 as i32;
    0 as i32;
    db;
    pIdx;
    // /* SQLITE_ENABLE_STAT4 */
}

// /*
// ** 2005-07-08
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// ** This file contains code associated with the ANALYZE command.
// **
// ** The ANALYZE command gather statistics about the content of tables
// ** and indices.  These statistics are made available to the query planner
// ** to help it make better decisions about how to perform queries.
// **
// ** The following system tables are or have been supported:
// **
// **    CREATE TABLE sqlite_stat1(tbl, idx, stat);
// **    CREATE TABLE sqlite_stat2(tbl, idx, sampleno, sample);
// **    CREATE TABLE sqlite_stat3(tbl, idx, nEq, nLt, nDLt, sample);
// **    CREATE TABLE sqlite_stat4(tbl, idx, nEq, nLt, nDLt, sample);
// **
// ** Additional tables might be added in future releases of SQLite.
// ** The sqlite_stat2 table is not created or used unless the SQLite version
// ** is between 3.6.18 and 3.7.8, inclusive, and unless SQLite is compiled
// ** with SQLITE_ENABLE_STAT2.  The sqlite_stat2 table is deprecated.
// ** The sqlite_stat2 table is superseded by sqlite_stat3, which is only
// ** created and used by SQLite versions 3.7.9 through 3.29.0 when
// ** SQLITE_ENABLE_STAT3 defined.  The functionality of sqlite_stat3
// ** is a superset of sqlite_stat2 and is also now deprecated.  The
// ** sqlite_stat4 is an enhanced version of sqlite_stat3 and is only
// ** available when compiled with SQLITE_ENABLE_STAT4 and in SQLite
// ** versions 3.8.1 and later.  STAT4 is the only variant that is still
// ** supported.
// **
// ** For most applications, sqlite_stat1 provides all the statistics required
// ** for the query planner to make good choices.
// **
// ** Format of sqlite_stat1:
// **
// ** There is normally one row per index, with the index identified by the
// ** name in the idx column.  The tbl column is the name of the table to
// ** which the index belongs.  In each such row, the stat column will be
// ** a string consisting of a list of integers.  The first integer in this
// ** list is the number of rows in the index.  (This is the same as the
// ** number of rows in the table, except for partial indices.)  The second
// ** integer is the average number of rows in the index that have the same
// ** value in the first column of the index.  The third integer is the average
// ** number of rows in the index that have the same value for the first two
// ** columns.  The N-th integer (for N>1) is the average number of rows in
// ** the index which have the same value for the first N-1 columns.  For
// ** a K-column index, there will be K+1 integers in the stat column.  If
// ** the index is unique, then the last integer will be 1.
// **
// ** The list of integers in the stat column can optionally be followed
// ** by the keyword "unordered".  The "unordered" keyword, if it is present,
// ** must be separated from the last integer by a single space.  If the
// ** "unordered" keyword is present, then the query planner assumes that
// ** the index is unordered and will not use the index for a range query.
// **
// ** If the sqlite_stat1.idx column is NULL, then the sqlite_stat1.stat
// ** column contains a single integer which is the (estimated) number of
// ** rows in the table identified by sqlite_stat1.tbl.
// **
// ** Format of sqlite_stat2:
// **
// ** The sqlite_stat2 is only created and is only used if SQLite is compiled
// ** with SQLITE_ENABLE_STAT2 and if the SQLite version number is between
// ** 3.6.18 and 3.7.8.  The "stat2" table contains additional information
// ** about the distribution of keys within an index.  The index is identified by
// ** the "idx" column and the "tbl" column is the name of the table to which
// ** the index belongs.  There are usually 10 rows in the sqlite_stat2
// ** table for each index.
// **
// ** The sqlite_stat2 entries for an index that have sampleno between 0 and 9
// ** inclusive are samples of the left-most key value in the index taken at
// ** evenly spaced points along the index.  Let the number of samples be S
// ** (10 in the standard build) and let C be the number of rows in the index.
// ** Then the sampled rows are given by:
// **
// **     rownumber = (i*C*2 + C)/(S*2)
// **
// ** For i between 0 and S-1.  Conceptually, the index space is divided into
// ** S uniform buckets and the samples are the middle row from each bucket.
// **
// ** The format for sqlite_stat2 is recorded here for legacy reference.  This
// ** version of SQLite does not support sqlite_stat2.  It neither reads nor
// ** writes the sqlite_stat2 table.  This version of SQLite only supports
// ** sqlite_stat3.
// **
// ** Format for sqlite_stat3:
// **
// ** The sqlite_stat3 format is a subset of sqlite_stat4.  Hence, the
// ** sqlite_stat4 format will be described first.  Further information
// ** about sqlite_stat3 follows the sqlite_stat4 description.
// **
// ** Format for sqlite_stat4:
// **
// ** As with sqlite_stat2, the sqlite_stat4 table contains histogram data
// ** to aid the query planner in choosing good indices based on the values
// ** that indexed columns are compared against in the WHERE clauses of
// ** queries.
// **
// ** The sqlite_stat4 table contains multiple entries for each index.
// ** The idx column names the index and the tbl column is the table of the
// ** index.  If the idx and tbl columns are the same, then the sample is
// ** of the INTEGER PRIMARY KEY.  The sample column is a blob which is the
// ** binary encoding of a key from the index.  The nEq column is a
// ** list of integers.  The first integer is the approximate number
// ** of entries in the index whose left-most column exactly matches
// ** the left-most column of the sample.  The second integer in nEq
// ** is the approximate number of entries in the index where the
// ** first two columns match the first two columns of the sample.
// ** And so forth.  nLt is another list of integers that show the approximate
// ** number of entries that are strictly less than the sample.  The first
// ** integer in nLt contains the number of entries in the index where the
// ** left-most column is less than the left-most column of the sample.
// ** The K-th integer in the nLt entry is the number of index entries
// ** where the first K columns are less than the first K columns of the
// ** sample.  The nDLt column is like nLt except that it contains the
// ** number of distinct entries in the index that are less than the
// ** sample.
// **
// ** There can be an arbitrary number of sqlite_stat4 entries per index.
// ** The ANALYZE command will typically generate sqlite_stat4 tables
// ** that contain between 10 and 40 samples which are distributed across
// ** the key space, though not uniformly, and which include samples with
// ** large nEq values.
// **
// ** Format for sqlite_stat3 redux:
// **
// ** The sqlite_stat3 table is like sqlite_stat4 except that it only
// ** looks at the left-most column of the index.  The sqlite_stat3.sample
// ** column contains the actual value of the left-most column instead
// ** of a blob encoding of the complete index key as is found in
// ** sqlite_stat4.sample.  The nEq, nLt, and nDLt entries of sqlite_stat3
// ** all contain just a single integer which is the same as the first
// ** integer in the equivalent columns in sqlite_stat4.
// */
// /*
// ** This routine generates code that opens the sqlite_statN tables.
// ** The sqlite_stat1 table is always relevant.  sqlite_stat2 is now
// ** obsolete.  sqlite_stat3 and sqlite_stat4 are only opened when
// ** appropriate compile-time options are provided.
// **
// ** If the sqlite_statN tables do not previously exist, it is created.
// **
// ** Argument zWhere may be a pointer to a buffer containing a table name,
// ** or it may be a NULL pointer. If it is not NULL, then all entries in
// ** the sqlite_statN tables associated with the named table are deleted.
// ** If zWhere==0, then code is generated to delete all stat table entries.
// */
fn openStatTable(
    mut pParse: *mut Parse,
    mut iDb: i32,
    mut iStatCur: i32,
    mut zWhere: *const i8,
    mut zWhereType: *const i8,
) {
    let mut i: i32 = 0 as i32;
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pDb: *mut Db = unsafe { std::mem::zeroed() };
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
    let mut aRoot: [u32; 3] = [0 as u32; 3];
    let mut aCreateTbl: [u8; 3] = [0 as u8; 3];
    let mut nToOpen: i32 = 1 as i32;
    if v == std::ptr::null_mut::<Vdbe>() {
        return;
    }
    0 as i32;
    0 as i32;
    pDb = unsafe { unsafe { (*db).aDb }.offset(iDb as isize) };
    // /* Create new statistic tables if they do not exist, or clear them
    //   ** if they do already exist.
    //   */
    i = 0 as i32;
    '__slate_break_662: loop {
        if !(i < ((((48 as u64) / (16 as u64)) as u32) as i32)) {
            break;
        }
        let mut zTab: *const i8 = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of!(aTable.0) as *const __SlateRecord187 }
                    .offset(i as isize)
            })
            .zName
        };
        let mut pStat: *mut Table = unsafe { std::mem::zeroed() };
        unsafe {
            *unsafe { (aCreateTbl.as_mut_ptr() as *mut u8).offset(i as isize) } =
                ((0 as i32) as i8) as u8;
        }
        let __v710: *mut Table =
            unsafe { sqlite3FindTable(db, zTab, (unsafe { (*pDb).zDbSName }) as *const i8) };
        pStat = __v710;
        if __v710 == std::ptr::null_mut::<Table>() {
            if i < nToOpen {
                // /* The sqlite_statN table does not exist. Create it. Note that a
                //         ** side-effect of the CREATE TABLE statement is to leave the rootpage
                //         ** of the new table in register pParse->regRoot. This is important
                //         ** because the OpenWrite opcode below will be needing it. */
                unsafe {
                    sqlite3NestedParse(
                        pParse,
                        (b"CREATE TABLE %Q.%s(%s)\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { (*pDb).zDbSName },
                        zTab,
                        unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of!(aTable.0) as *const __SlateRecord187 }
                                    .offset(i as isize)
                            })
                            .zCols
                        },
                    )
                };
                0 as i32;
                unsafe {
                    *unsafe { (aRoot.as_mut_ptr() as *mut u32).offset(i as isize) } =
                        (unsafe { (*pParse).u1.cr.regRoot }) as u32;
                }
                unsafe {
                    *unsafe { (aCreateTbl.as_mut_ptr() as *mut u8).offset(i as isize) } =
                        ((16 as i32) as i8) as u8;
                }
            }
        } else {
            // /* The table already exists. If zWhere is not NULL, delete all entries
            //       ** associated with the table zWhere. If zWhere is NULL, delete the
            //       ** entire contents of the table. */
            unsafe {
                *unsafe { (aRoot.as_mut_ptr() as *mut u32).offset(i as isize) } =
                    unsafe { (*pStat).tnum };
            }
            unsafe {
                sqlite3TableLock(
                    pParse,
                    iDb,
                    unsafe { *unsafe { (aRoot.as_mut_ptr() as *mut u32).offset(i as isize) } },
                    ((1 as i32) as i8) as u8,
                    zTab,
                )
            };
            if zWhere != std::ptr::null::<i8>() {
                unsafe {
                    sqlite3NestedParse(
                        pParse,
                        (b"DELETE FROM %Q.%s WHERE %s=%Q\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { (*pDb).zDbSName },
                        zTab,
                        zWhereType,
                        zWhere,
                    )
                };
            } else {
                // /* The sqlite_stat[134] table already exists.  Delete all rows. */
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        147 as i32,
                        (unsafe { *unsafe { (aRoot.as_mut_ptr() as *mut u32).offset(i as isize) } })
                            as i32,
                        iDb,
                    )
                };
            }
        }
        let __v708: i32 = i;
        let __v709: i32 = __v708 + (1 as i32);
        i = __v709;
    }
    // /* Open the sqlite_stat[134] tables for writing. */
    i = 0 as i32;
    '__slate_break_665: loop {
        if !(i < nToOpen) {
            break;
        }
        0 as i32;
        unsafe {
            sqlite3VdbeAddOp4Int(
                v,
                116 as i32,
                iStatCur + i,
                (unsafe { *unsafe { (aRoot.as_mut_ptr() as *mut u32).offset(i as isize) } }) as i32,
                iDb,
                3 as i32,
            )
        };
        unsafe {
            sqlite3VdbeChangeP5(
                v,
                (unsafe { *unsafe { (aCreateTbl.as_mut_ptr() as *mut u8).offset(i as isize) } })
                    as u16,
            )
        };
        {}
        let __v711: i32 = i;
        let __v712: i32 = __v711 + (1 as i32);
        i = __v712;
    }
}

// /* Current row as a StatSample */
// /* Reclaim memory used by a StatSample
// */
// /* Initialize the BLOB value of a ROWID
// */
// /* Initialize the INTEGER value of a ROWID.
// */
// /*
// ** Copy the contents of object (*pFrom) into (*pTo).
// */
// /*
// ** Reclaim all memory of a StatAccum structure.
// */
#[unsafe(link_section = ".text.slate_distinct.analyze.statAccumDestructor")]
extern "C-unwind" fn statAccumDestructor(mut pOld: *mut ()) {
    let mut p: *mut StatAccum = pOld as *mut StatAccum;
    unsafe { sqlite3DbFree(unsafe { (*p).db }, p as *mut ()) };
}

// /*
// ** Implementation of the stat_init(N,K,C,L) SQL function. The four parameters
// ** are:
// **     N:    The number of columns in the index including the rowid/pk (note 1)
// **     K:    The number of columns in the index excluding the rowid/pk.
// **     C:    Estimated number of rows in the index
// **     L:    A limit on the number of rows to scan, or 0 for no-limit
// **
// ** Note 1:  In the special case of the covering index that implements a
// ** WITHOUT ROWID table, N is the number of PRIMARY KEY columns, not the
// ** total number of columns in the table.
// **
// ** For indexes on ordinary rowid tables, N==K+1.  But for indexes on
// ** WITHOUT ROWID tables, N=K+P where P is the number of columns in the
// ** PRIMARY KEY of the table.  The covering index that implements the
// ** original WITHOUT ROWID table as N==K as a special case.
// **
// ** This routine allocates the StatAccum object in heap memory. The return
// ** value is a pointer to the StatAccum object.  The datatype of the
// ** return value is BLOB, but it is really just a pointer to the StatAccum
// ** object.
// */
#[unsafe(link_section = ".text.slate_distinct.analyze.statInit")]
extern "C-unwind" fn statInit(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut p: *mut StatAccum = unsafe { std::mem::zeroed() };
    // /* Number of columns in index being sampled */
    let mut nCol: i32 = 0 as i32;
    // /* Number of key columns */
    let mut nKeyCol: i32 = 0 as i32;
    // /* nCol rounded up for alignment */
    let mut nColUp: i32 = 0 as i32;
    // /* Bytes of space to allocate */
    let mut n: i64 = 0 as i64;
    // /* Database connection */
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    // /* Decode the three function arguments */
    argc;
    nCol = unsafe { sqlite3_value_int(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    0 as i32;
    nColUp = if (8 as u64) < (((8 as i32) as i64) as u64) {
        nCol + (1 as i32) & !(1 as i32)
    } else {
        nCol
    };
    nKeyCol = unsafe { sqlite3_value_int(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) };
    0 as i32;
    0 as i32;
    // /* Allocate the space required for the StatAccum object */
    n = (48 as u64).wrapping_add((8 as u64).wrapping_mul((nColUp as i64) as u64)) as i64;
    // /* StatAccum.anDLt */
    p = (unsafe { sqlite3DbMallocZero(db, n as u64) }) as *mut StatAccum;
    if p == std::ptr::null_mut::<StatAccum>() {
        unsafe { sqlite3_result_error_nomem(context) };
        return;
    }
    unsafe {
        (*p).db = db;
    }
    unsafe {
        (*p).nEst = (unsafe {
            sqlite3_value_int64(unsafe { *unsafe { argv.offset((2 as i32) as isize) } })
        }) as u64;
    }
    unsafe {
        (*p).nRow = ((0 as i32) as i64) as u64;
    }
    unsafe {
        (*p).nLimit =
            unsafe { sqlite3_value_int(unsafe { *unsafe { argv.offset((3 as i32) as isize) } }) };
    }
    unsafe {
        (*p).nCol = nCol;
    }
    unsafe {
        (*p).nKeyCol = nKeyCol;
    }
    unsafe {
        (*p).nSkipAhead = ((0 as i32) as i8) as u8;
    }
    unsafe {
        (*p).current.anDLt = (unsafe { p.offset((1 as i32) as isize) }) as *mut u64;
    }
    // /* Return a pointer to the allocated object to the caller.  Note that
    //   ** only the pointer (the 2nd parameter) matters.  The size of the object
    //   ** (given by the 3rd parameter) is never used and can be any positive
    //   ** value. */
    unsafe {
        sqlite3_result_blob(
            context,
            p as *const (),
            ((48 as u64) as u32) as i32,
            Some(statAccumDestructor),
        )
    };
}

// /* nArg */
// /* funcFlags */
// /* pUserData */
// /* pNext */
// /* xSFunc */
// /* xFinalize */
// /* xValue, xInverse */
// /* zName */
// /*
// ** Implementation of the stat_push SQL function:  stat_push(P,C,R)
// ** Arguments:
// **
// **    P     Pointer to the StatAccum object created by stat_init()
// **    C     Index of left-most column to differ from previous row
// **    R     Rowid for the current row.  Might be a key record for
// **          WITHOUT ROWID tables.
// **
// ** The purpose of this routine is to collect statistical data and/or
// ** samples from the index being analyzed into the StatAccum object.
// ** The stat_get() SQL function will be used afterwards to
// ** retrieve the information gathered.
// **
// ** This SQL function usually returns NULL, but might return an integer
// ** if it wants the byte-code to do special processing.
// **
// ** The R parameter is only used for STAT4
// */
#[unsafe(link_section = ".text.slate_distinct.analyze.statPush")]
extern "C-unwind" fn statPush(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut i: i32 = 0 as i32;
    // /* The three function arguments */
    let mut p: *mut StatAccum =
        (unsafe { sqlite3_value_blob(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
            as *mut StatAccum;
    let mut iChng: i32 =
        unsafe { sqlite3_value_int(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) };
    argc;
    context;
    0 as i32;
    0 as i32;
    if (unsafe { (*p).nRow }) == (((0 as i32) as i64) as u64) {
        // /* This is the first call to this function. Do initialization. */
    } else {
        // /* Second and subsequent calls get processed here */
        // /* Update anDLt[], anLt[] and anEq[] to reflect the values that apply
        //     ** to the current row of the index. */
        i = iChng;
        '__slate_break_667: loop {
            if !(i < unsafe { (*p).nCol }) {
                break;
            }
            let __v715: *mut u64 = unsafe { unsafe { (*p).current.anDLt }.offset(i as isize) };
            let __v716: u64 = unsafe { *__v715 };
            let __v717: u64 = __v716.wrapping_add(((1 as i32) as i64) as u64);
            unsafe {
                *__v715 = __v717;
            }
            let __v713: i32 = i;
            let __v714: i32 = __v713 + (1 as i32);
            i = __v714;
        }
    }
    let __v718: *mut StatAccum = p;
    let __v719: u64 = unsafe { (*__v718).nRow };
    let __v720: u64 = __v719.wrapping_add(((1 as i32) as i64) as u64);
    unsafe {
        (*__v718).nRow = __v720;
    }
    if (unsafe { (*p).nLimit }) != (0 as i32)
        && (unsafe { (*p).nRow })
            > (((unsafe { (*p).nLimit }) as i64) as u64).wrapping_mul(
                (((((unsafe { (*p).nSkipAhead }) as u32) as i32) + (1 as i32)) as i64) as u64,
            )
    {
        let __v721: *mut StatAccum = p;
        let __v722: u8 = unsafe { (*__v721).nSkipAhead };
        let __v723: u8 = ((((__v722 as u32) as i32) + (1 as i32)) as i8) as u8;
        unsafe {
            (*__v721).nSkipAhead = __v723;
        }
        unsafe {
            sqlite3_result_int(
                context,
                ((unsafe { *unsafe { unsafe { (*p).current.anDLt }.offset((0 as i32) as isize) } })
                    > (((0 as i32) as i64) as u64)) as i32,
            )
        };
    }
}

// /* nArg */
// /* funcFlags */
// /* pUserData */
// /* pNext */
// /* xSFunc */
// /* xFinalize */
// /* xValue, xInverse */
// /* zName */
// /* "stat" column of stat1 table */
// /* "rowid" column of stat[34] entry */
// /* "neq" column of stat[34] entry */
// /* "nlt" column of stat[34] entry */
// /* "ndlt" column of stat[34] entry */
// /*
// ** Implementation of the stat_get(P,J) SQL function.  This routine is
// ** used to query statistical information that has been gathered into
// ** the StatAccum object by prior calls to stat_push().  The P parameter
// ** has type BLOB but it is really just a pointer to the StatAccum object.
// ** The content to returned is determined by the parameter J
// ** which is one of the STAT_GET_xxxx values defined above.
// **
// ** The stat_get(P,J) function is not available to generic SQL.  It is
// ** inserted as part of a manually constructed bytecode program.  (See
// ** the callStatGet() routine below.)  It is guaranteed that the P
// ** parameter will always be a pointer to a StatAccum object, never a
// ** NULL.
// **
// ** If STAT4 is not enabled, then J is always
// ** STAT_GET_STAT1 and is hence omitted and this routine becomes
// ** a one-parameter function, stat_get(P), that always returns the
// ** stat1 table entry information.
// */
#[unsafe(link_section = ".text.slate_distinct.analyze.statGet")]
extern "C-unwind" fn statGet(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut p: *mut StatAccum =
        (unsafe { sqlite3_value_blob(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
            as *mut StatAccum;
    0 as i32;
    // /* Return the value to store in the "stat" column of the sqlite_stat1
    //     ** table for this index.
    //     **
    //     ** The value is a string composed of a list of integers describing
    //     ** the index. The first integer in the list is the total number of
    //     ** entries in the index. There is one additional integer in the list
    //     ** for each indexed column. This additional integer is an estimate of
    //     ** the number of rows matched by a equality query on the index using
    //     ** a key with the corresponding number of fields. In other words,
    //     ** if the index is on columns (a,b) and the sqlite_stat1 value is
    //     ** "100 10 2", then SQLite estimates that:
    //     **
    //     **   * the index contains 100 rows,
    //     **   * "WHERE a=?" matches 10 rows, and
    //     **   * "WHERE a=? AND b=?" matches 2 rows.
    //     **
    //     ** If D is the count of distinct values and K is the total number of
    //     ** rows, then each estimate is usually computed as:
    //     **
    //     **        I = (K+D-1)/D
    //     **
    //     ** In other words, I is K/D rounded up to the next whole integer.
    //     ** However, if I is between 1.0 and 1.1 (in other words if I is
    //     ** close to 1.0 but just a little larger) then do not round up but
    //     ** instead keep the I value at 1.0.
    //     */
    // /* Text of the constructed "stat" line */
    let mut sStat: sqlite3_str = unsafe { std::mem::zeroed() };
    // /* Loop counter */
    let mut i: i32 = 0 as i32;
    unsafe {
        sqlite3StrAccumInit(
            std::ptr::addr_of_mut!(sStat),
            std::ptr::null_mut::<sqlite3>(),
            std::ptr::null_mut::<i8>(),
            0 as i32,
            ((unsafe { (*p).nKeyCol }) + (1 as i32)) * (100 as i32),
        )
    };
    unsafe {
        sqlite3_str_appendf(
            std::ptr::addr_of_mut!(sStat),
            (b"%llu\0".as_ptr() as *mut i8) as *const i8,
            if (unsafe { (*p).nSkipAhead }) != (0 as u8) {
                unsafe { (*p).nEst }
            } else {
                unsafe { (*p).nRow }
            },
        )
    };
    i = 0 as i32;
    '__slate_break_670: loop {
        if !(i < unsafe { (*p).nKeyCol }) {
            break;
        }
        let mut nDistinct: u64 =
            unsafe { *unsafe { unsafe { (*p).current.anDLt }.offset(i as isize) } }
                .wrapping_add(((1 as i32) as i64) as u64);
        let mut iVal: u64 = unsafe { (*p).nRow }
            .wrapping_add(nDistinct)
            .wrapping_sub(((1 as i32) as i64) as u64)
            / nDistinct;
        if iVal == (((2 as i32) as i64) as u64)
            && unsafe { (*p).nRow }.wrapping_mul(((10 as i32) as i64) as u64)
                <= nDistinct.wrapping_mul(((11 as i32) as i64) as u64)
        {
            iVal = ((1 as i32) as i64) as u64;
        }
        unsafe {
            sqlite3_str_appendf(
                std::ptr::addr_of_mut!(sStat),
                (b" %llu\0".as_ptr() as *mut i8) as *const i8,
                iVal,
            )
        };
        let __v724: i32 = i;
        let __v725: i32 = __v724 + (1 as i32);
        i = __v725;
    }
    unsafe { sqlite3_result_str(context, std::ptr::addr_of_mut!(sStat), 1 as i32) };
    argc;
}

// /* nArg */
// /* funcFlags */
// /* pUserData */
// /* pNext */
// /* xSFunc */
// /* xFinalize */
// /* xValue, xInverse */
// /* zName */
fn callStatGet(mut pParse: *mut Parse, mut regStat: i32, mut iParam: i32, mut regOut: i32) {
    iParam;
    0 as i32;
    unsafe {
        sqlite3VdbeAddFunctionCall(
            pParse,
            0 as i32,
            regStat,
            regOut,
            (1 as i32) + (0 as i32),
            unsafe { std::ptr::addr_of!(statGetFuncdef) },
            0 as i32,
        )
    };
}

// /* SQLITE_DEBUG */
// /*
// ** Generate code to do an analysis of all indices associated with
// ** a single table.
// */
fn analyzeOneTable(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pOnlyIdx: *mut Index,
    mut iStatCur: i32,
    mut iMem: i32,
    mut iTab: i32,
) {
    // /* Database handle */
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    // /* An index to being analyzed */
    let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
    // /* Cursor open on index being analyzed */
    let mut iIdxCur: i32 = 0 as i32;
    // /* Table cursor */
    let mut iTabCur: i32 = 0 as i32;
    // /* The virtual machine being built up */
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    // /* Loop counter */
    let mut i: i32 = 0 as i32;
    // /* Jump from here if number of rows is zero */
    let mut jZeroRows: i32 = -(1 as i32);
    // /* Index of database containing pTab */
    let mut iDb: i32 = 0 as i32;
    // /* True to count the table */
    let mut needTableCnt: u8 = ((1 as i32) as i8) as u8;
    // /* Rowid for the inserted record */
    let mut regNewRowid: i32 = 0 as i32;
    let __v726: i32 = iMem;
    let __v727: i32 = __v726 + (1 as i32);
    iMem = __v727;
    regNewRowid = __v726;
    // /* Register to hold StatAccum object */
    let mut regStat: i32 = 0 as i32;
    let __v728: i32 = iMem;
    let __v729: i32 = __v728 + (1 as i32);
    iMem = __v729;
    regStat = __v728;
    // /* Index of changed index field */
    let mut regChng: i32 = 0 as i32;
    let __v730: i32 = iMem;
    let __v731: i32 = __v730 + (1 as i32);
    iMem = __v731;
    regChng = __v730;
    // /* Rowid argument passed to stat_push() */
    let mut regRowid: i32 = 0 as i32;
    let __v732: i32 = iMem;
    let __v733: i32 = __v732 + (1 as i32);
    iMem = __v733;
    regRowid = __v732;
    // /* Temporary use register */
    let mut regTemp: i32 = 0 as i32;
    let __v734: i32 = iMem;
    let __v735: i32 = __v734 + (1 as i32);
    iMem = __v735;
    regTemp = __v734;
    // /* Second temporary use register */
    let mut regTemp2: i32 = 0 as i32;
    let __v736: i32 = iMem;
    let __v737: i32 = __v736 + (1 as i32);
    iMem = __v737;
    regTemp2 = __v736;
    // /* Register containing table name */
    let mut regTabname: i32 = 0 as i32;
    let __v738: i32 = iMem;
    let __v739: i32 = __v738 + (1 as i32);
    iMem = __v739;
    regTabname = __v738;
    // /* Register containing index name */
    let mut regIdxname: i32 = 0 as i32;
    let __v740: i32 = iMem;
    let __v741: i32 = __v740 + (1 as i32);
    iMem = __v741;
    regIdxname = __v740;
    // /* Value for the stat column of sqlite_stat1 */
    let mut regStat1: i32 = 0 as i32;
    let __v742: i32 = iMem;
    let __v743: i32 = __v742 + (1 as i32);
    iMem = __v743;
    regStat1 = __v742;
    // /* MUST BE LAST (see below) */
    let mut regPrev: i32 = iMem;
    unsafe { sqlite3TouchRegister(pParse, iMem) };
    0 as i32;
    v = unsafe { sqlite3GetVdbe(pParse) };
    if v == std::ptr::null_mut::<Vdbe>() || pTab == std::ptr::null_mut::<Table>() {
        return;
    }
    if !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (0 as i32)) {
        // /* Do not gather statistics on views or virtual tables */
        return;
    }
    if (unsafe {
        sqlite3_strlike(
            (b"sqlite\\_%\0".as_ptr() as *mut i8) as *const i8,
            (unsafe { (*pTab).zName }) as *const i8,
            (92 as i32) as u32,
        )
    }) == (0 as i32)
    {
        // /* Do not gather statistics on system tables */
        return;
    }
    0 as i32;
    iDb = unsafe { sqlite3SchemaToIndex(db, unsafe { (*pTab).pSchema }) };
    0 as i32;
    0 as i32;
    if (unsafe {
        sqlite3AuthCheck(
            pParse,
            28 as i32,
            (unsafe { (*pTab).zName }) as *const i8,
            std::ptr::null::<i8>(),
            (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName })
                as *const i8,
        )
    }) != (0 as i32)
    {
        return;
    }
    // /* Establish a read-lock on the table at the shared-cache level.
    //   ** Open a read-only cursor on the table. Also allocate a cursor number
    //   ** to use for scanning indexes (iIdxCur). No index cursor is opened at
    //   ** this time though.  */
    unsafe {
        sqlite3TableLock(
            pParse,
            iDb,
            unsafe { (*pTab).tnum },
            ((0 as i32) as i8) as u8,
            (unsafe { (*pTab).zName }) as *const i8,
        )
    };
    let __v744: i32 = iTab;
    let __v745: i32 = __v744 + (1 as i32);
    iTab = __v745;
    iTabCur = __v744;
    let __v746: i32 = iTab;
    let __v747: i32 = __v746 + (1 as i32);
    iTab = __v747;
    iIdxCur = __v746;
    unsafe {
        (*pParse).nTab = if (unsafe { (*pParse).nTab }) > iTab {
            unsafe { (*pParse).nTab }
        } else {
            iTab
        };
    }
    unsafe { sqlite3OpenTable(pParse, iTabCur, iDb, pTab, 114 as i32) };
    unsafe { sqlite3VdbeLoadString(v, regTabname, (unsafe { (*pTab).zName }) as *const i8) };
    pIdx = unsafe { (*pTab).pIndex };
    '__slate_break_674: while pIdx != std::ptr::null_mut::<Index>() {
        '__slate_continue_674: {
            // /* Number of columns in pIdx. "N" */
            let mut nCol: i32 = 0 as i32;
            // /* Address of "OP_Rewind iIdxCur" */
            let mut addrGotoEnd: i32 = 0 as i32;
            // /* Address of "next_row:" */
            let mut addrNextRow: i32 = 0 as i32;
            // /* Name of the index */
            let mut zIdxName: *const i8 = unsafe { std::mem::zeroed() };
            // /* Number of columns to test for changes */
            let mut nColTest: i32 = 0 as i32;
            if pOnlyIdx != std::ptr::null_mut::<Index>() && pOnlyIdx != pIdx {
            } else {
                if (unsafe { (*pIdx).pPartIdxWhere }) == std::ptr::null_mut::<Expr>() {
                    needTableCnt = ((0 as i32) as i8) as u8;
                }
                if !((unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32))
                    && ((unsafe { (*pIdx).__slate_bits_0.__get_idxType() }) as i32) == (2 as i32)
                {
                    nCol = ((unsafe { (*pIdx).nKeyCol }) as u32) as i32;
                    zIdxName = (unsafe { (*pTab).zName }) as *const i8;
                    nColTest = nCol - (1 as i32);
                } else {
                    nCol = ((unsafe { (*pIdx).nColumn }) as u32) as i32;
                    zIdxName = (unsafe { (*pIdx).zName }) as *const i8;
                    nColTest = if ((unsafe { (*pIdx).__slate_bits_0.__get_uniqNotNull() }) as i32)
                        != (0 as i32)
                    {
                        (((unsafe { (*pIdx).nKeyCol }) as u32) as i32) - (1 as i32)
                    } else {
                        nCol - (1 as i32)
                    };
                }
                // /* Populate the register containing the index name. */
                unsafe { sqlite3VdbeLoadString(v, regIdxname, zIdxName) };
                {}
                // /*
                //     ** Pseudo-code for loop that calls stat_push():
                //     **
                //     **   regChng = 0
                //     **   Rewind csr
                //     **   if eof(csr){
                //     **      stat_init() with count = 0;
                //     **      goto end_of_scan;
                //     **   }
                //     **   count()
                //     **   stat_init()
                //     **   goto chng_addr_0;
                //     **
                //     **  next_row:
                //     **   regChng = 0
                //     **   if( idx(0) != regPrev(0) ) goto chng_addr_0
                //     **   regChng = 1
                //     **   if( idx(1) != regPrev(1) ) goto chng_addr_1
                //     **   ...
                //     **   regChng = N
                //     **   goto chng_addr_N
                //     **
                //     **  chng_addr_0:
                //     **   regPrev(0) = idx(0)
                //     **  chng_addr_1:
                //     **   regPrev(1) = idx(1)
                //     **  ...
                //     **
                //     **  endDistinctTest:
                //     **   regRowid = idx(rowid)
                //     **   stat_push(P, regChng, regRowid)
                //     **   Next csr
                //     **   if !eof(csr) goto next_row;
                //     **
                //     **  end_of_scan:
                //     */
                // /* Make sure there are enough memory cells allocated to accommodate
                //     ** the regPrev array and a trailing rowid (the rowid slot is required
                //     ** when building a record to insert into the sample column of
                //     ** the sqlite_stat4 table.  */
                unsafe { sqlite3TouchRegister(pParse, regPrev + nColTest) };
                // /* Open a read-only cursor on the index being analyzed. */
                0 as i32;
                unsafe {
                    sqlite3VdbeAddOp3(
                        v,
                        114 as i32,
                        iIdxCur,
                        (unsafe { (*pIdx).tnum }) as i32,
                        iDb,
                    )
                };
                unsafe { sqlite3VdbeSetP4KeyInfo(pParse, pIdx) };
                {}
                // /* Implementation of the following:
                //     **
                //     **   regChng = 0
                //     **   Rewind csr
                //     **   if eof(csr){
                //     **      stat_init() with count = 0;
                //     **      goto end_of_scan;
                //     **   }
                //     **   count()
                //     **   stat_init()
                //     **   goto chng_addr_0;
                //     */
                0 as i32;
                unsafe {
                    sqlite3VdbeAddOp2(v, 73 as i32, unsafe { (*db).nAnalysisLimit }, regTemp2)
                };
                // /* Arguments to stat_init():
                //     **    (1) the number of columns in the index including the rowid
                //     **        (or for a WITHOUT ROWID table, the number of PK columns),
                //     **    (2) the number of columns in the key without the rowid/pk
                //     **    (3) estimated number of rows in the index. */
                unsafe { sqlite3VdbeAddOp2(v, 73 as i32, nCol, regStat + (1 as i32)) };
                0 as i32;
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        73 as i32,
                        ((unsafe { (*pIdx).nKeyCol }) as u32) as i32,
                        regRowid,
                    )
                };
                unsafe {
                    sqlite3VdbeAddOp3(
                        v,
                        100 as i32,
                        iIdxCur,
                        regTemp,
                        ((unsafe { (*db).dbOptFlags }) & ((2048 as i32) as u32)
                            != ((0 as i32) as u32)) as i32,
                    )
                };
                unsafe {
                    sqlite3VdbeAddFunctionCall(
                        pParse,
                        0 as i32,
                        regStat + (1 as i32),
                        regStat,
                        4 as i32,
                        unsafe { std::ptr::addr_of!(statInitFuncdef) },
                        0 as i32,
                    )
                };
                addrGotoEnd = unsafe { sqlite3VdbeAddOp1(v, 36 as i32, iIdxCur) };
                {}
                unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, regChng) };
                addrNextRow = unsafe { sqlite3VdbeCurrentAddr(v) };
                if nColTest > (0 as i32) {
                    let mut endDistinctTest: i32 = unsafe { sqlite3VdbeMakeLabel(pParse) };
                    // /* Array of jump instruction addresses */
                    let mut aGotoChng: *mut i32 = unsafe { std::mem::zeroed() };
                    aGotoChng = (unsafe {
                        sqlite3DbMallocRawNN(db, (4 as u64).wrapping_mul((nColTest as i64) as u64))
                    }) as *mut i32;
                    if aGotoChng == std::ptr::null_mut::<i32>() {
                        break '__slate_continue_674;
                    }
                    // /*
                    //       **  next_row:
                    //       **   regChng = 0
                    //       **   if( idx(0) != regPrev(0) ) goto chng_addr_0
                    //       **   regChng = 1
                    //       **   if( idx(1) != regPrev(1) ) goto chng_addr_1
                    //       **   ...
                    //       **   regChng = N
                    //       **   goto endDistinctTest
                    //       */
                    unsafe { sqlite3VdbeAddOp0(v, 9 as i32) };
                    addrNextRow = unsafe { sqlite3VdbeCurrentAddr(v) };
                    if nColTest == (1 as i32)
                        && (((unsafe { (*pIdx).nKeyCol }) as u32) as i32) == (1 as i32)
                        && (((unsafe { (*pIdx).onError }) as u32) as i32) != (0 as i32)
                    {
                        // /* For a single-column UNIQUE index, once we have found a non-NULL
                        //         ** row, we know that all the rest will be distinct, so skip
                        //         ** subsequent distinctness tests. */
                        unsafe { sqlite3VdbeAddOp2(v, 52 as i32, regPrev, endDistinctTest) };
                        {}
                    }
                    i = 0 as i32;
                    '__slate_break_675: loop {
                        if !(i < nColTest) {
                            break;
                        }
                        let mut pColl: *mut i8 = (unsafe {
                            sqlite3LocateCollSeq(pParse, unsafe {
                                *unsafe { unsafe { (*pIdx).azColl }.offset(i as isize) }
                            })
                        }) as *mut i8;
                        unsafe { sqlite3VdbeAddOp2(v, 73 as i32, i, regChng) };
                        unsafe { sqlite3VdbeAddOp3(v, 96 as i32, iIdxCur, i, regTemp) };
                        {}
                        unsafe {
                            *unsafe { aGotoChng.offset(i as isize) } = unsafe {
                                sqlite3VdbeAddOp4(
                                    v,
                                    53 as i32,
                                    regTemp,
                                    0 as i32,
                                    regPrev + i,
                                    pColl as *const i8,
                                    -(2 as i32),
                                )
                            };
                        }
                        unsafe { sqlite3VdbeChangeP5(v, ((128 as i32) as i16) as u16) };
                        {}
                        let __v748: i32 = i;
                        let __v749: i32 = __v748 + (1 as i32);
                        i = __v749;
                    }
                    unsafe { sqlite3VdbeAddOp2(v, 73 as i32, nColTest, regChng) };
                    unsafe { sqlite3VdbeGoto(v, endDistinctTest) };
                    // /*
                    //       **  chng_addr_0:
                    //       **   regPrev(0) = idx(0)
                    //       **  chng_addr_1:
                    //       **   regPrev(1) = idx(1)
                    //       **  ...
                    //       */
                    unsafe { sqlite3VdbeJumpHere(v, addrNextRow - (1 as i32)) };
                    i = 0 as i32;
                    '__slate_break_676: loop {
                        if !(i < nColTest) {
                            break;
                        }
                        unsafe {
                            sqlite3VdbeJumpHere(v, unsafe {
                                *unsafe { aGotoChng.offset(i as isize) }
                            })
                        };
                        unsafe { sqlite3VdbeAddOp3(v, 96 as i32, iIdxCur, i, regPrev + i) };
                        {}
                        let __v750: i32 = i;
                        let __v751: i32 = __v750 + (1 as i32);
                        i = __v751;
                    }
                    unsafe { sqlite3VdbeResolveLabel(v, endDistinctTest) };
                    unsafe { sqlite3DbFree(db, aGotoChng as *mut ()) };
                }
                // /*
                //     **  chng_addr_N:
                //     **   regRowid = idx(rowid)            // STAT4 only
                //     **   stat_push(P, regChng, regRowid)  // 3rd parameter STAT4 only
                //     **   Next csr
                //     **   if !eof(csr) goto next_row;
                //     */
                0 as i32;
                unsafe {
                    sqlite3VdbeAddFunctionCall(
                        pParse,
                        1 as i32,
                        regStat,
                        regTemp,
                        (2 as i32) + (0 as i32),
                        unsafe { std::ptr::addr_of!(statPushFuncdef) },
                        0 as i32,
                    )
                };
                if (unsafe { (*db).nAnalysisLimit }) != (0 as i32) {
                    let mut j1: i32 = 0 as i32;
                    let mut j2: i32 = 0 as i32;
                    let mut j3: i32 = 0 as i32;
                    j1 = unsafe { sqlite3VdbeAddOp1(v, 51 as i32, regTemp) };
                    {}
                    j2 = unsafe { sqlite3VdbeAddOp1(v, 16 as i32, regTemp) };
                    {}
                    j3 = unsafe {
                        sqlite3VdbeAddOp4Int(v, 24 as i32, iIdxCur, 0 as i32, regPrev, 1 as i32)
                    };
                    {}
                    unsafe { sqlite3VdbeJumpHere(v, j1) };
                    unsafe { sqlite3VdbeAddOp2(v, 40 as i32, iIdxCur, addrNextRow) };
                    {}
                    unsafe { sqlite3VdbeJumpHere(v, j2) };
                    unsafe { sqlite3VdbeJumpHere(v, j3) };
                } else {
                    unsafe { sqlite3VdbeAddOp2(v, 40 as i32, iIdxCur, addrNextRow) };
                    {}
                }
                // /* Add the entry to the stat1 table. */
                if (unsafe { (*pIdx).pPartIdxWhere }) != std::ptr::null_mut::<Expr>() {
                    // /* Partial indexes might get a zero-entry in sqlite_stat1.  But
                    //       ** an empty table is omitted from sqlite_stat1. */
                    unsafe { sqlite3VdbeJumpHere(v, addrGotoEnd) };
                    addrGotoEnd = 0 as i32;
                }
                callStatGet(pParse, regStat, 0 as i32, regStat1);
                0 as i32;
                unsafe {
                    sqlite3VdbeAddOp4(
                        v,
                        99 as i32,
                        regTabname,
                        3 as i32,
                        regTemp,
                        (b"BBB\0".as_ptr() as *mut i8) as *const i8,
                        0 as i32,
                    )
                };
                unsafe { sqlite3VdbeAddOp2(v, 129 as i32, iStatCur, regNewRowid) };
                unsafe { sqlite3VdbeAddOp3(v, 130 as i32, iStatCur, regTemp, regNewRowid) };
                unsafe { sqlite3VdbeChangeP5(v, ((8 as i32) as i16) as u16) };
                // /* Add the entries to the stat4 table. */
                // /* End of analysis */
                if addrGotoEnd != (0 as i32) {
                    unsafe { sqlite3VdbeJumpHere(v, addrGotoEnd) };
                }
            }
        }
        pIdx = unsafe { (*pIdx).pNext };
    }
    // /* Create a single sqlite_stat1 entry containing NULL as the index
    //   ** name and the row count as the content.
    //   */
    if pOnlyIdx == std::ptr::null_mut::<Index>() && needTableCnt != (0 as u8) {
        {}
        unsafe { sqlite3VdbeAddOp2(v, 100 as i32, iTabCur, regStat1) };
        jZeroRows = unsafe { sqlite3VdbeAddOp1(v, 17 as i32, regStat1) };
        {}
        unsafe { sqlite3VdbeAddOp2(v, 77 as i32, 0 as i32, regIdxname) };
        0 as i32;
        unsafe {
            sqlite3VdbeAddOp4(
                v,
                99 as i32,
                regTabname,
                3 as i32,
                regTemp,
                (b"BBB\0".as_ptr() as *mut i8) as *const i8,
                0 as i32,
            )
        };
        unsafe { sqlite3VdbeAddOp2(v, 129 as i32, iStatCur, regNewRowid) };
        unsafe { sqlite3VdbeAddOp3(v, 130 as i32, iStatCur, regTemp, regNewRowid) };
        unsafe { sqlite3VdbeChangeP5(v, ((8 as i32) as i16) as u16) };
        unsafe { sqlite3VdbeJumpHere(v, jZeroRows) };
    }
}

// /* Parser context */
// /* Table whose indices are to be analyzed */
// /* If not NULL, only analyze this one index */
// /* Index of VdbeCursor that writes the sqlite_stat1 table */
// /* Available memory locations begin here */
// /* Next available cursor */
// /*
// ** Generate code that will cause the most recent index analysis to
// ** be loaded into internal hash tables where is can be used.
// */
fn loadAnalysis(mut pParse: *mut Parse, mut iDb: i32) {
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
    if v != std::ptr::null_mut::<Vdbe>() {
        unsafe { sqlite3VdbeAddOp1(v, 152 as i32, iDb) };
    }
}

// /*
// ** Generate code that will do an analysis of an entire database
// */
fn analyzeDatabase(mut pParse: *mut Parse, mut iDb: i32) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    // /* Schema of database iDb */
    let mut pSchema: *mut Schema =
        unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pSchema };
    let mut k: *mut HashElem = unsafe { std::mem::zeroed() };
    let mut iStatCur: i32 = 0 as i32;
    let mut iMem: i32 = 0 as i32;
    let mut iTab: i32 = 0 as i32;
    unsafe { sqlite3BeginWriteOperation(pParse, 0 as i32, iDb) };
    iStatCur = unsafe { (*pParse).nTab };
    let __v752: *mut Parse = pParse;
    let __v753: i32 = unsafe { (*__v752).nTab };
    let __v754: i32 = __v753 + (3 as i32);
    unsafe {
        (*__v752).nTab = __v754;
    }
    openStatTable(
        pParse,
        iDb,
        iStatCur,
        std::ptr::null::<i8>(),
        std::ptr::null::<i8>(),
    );
    iMem = (unsafe { (*pParse).nMem }) + (1 as i32);
    iTab = unsafe { (*pParse).nTab };
    0 as i32;
    k = unsafe { (*unsafe { std::ptr::addr_of_mut!((*pSchema).tblHash) }).first };
    '__slate_break_679: while k != std::ptr::null_mut::<HashElem>() {
        let mut pTab: *mut Table = (unsafe { (*k).data }) as *mut Table;
        analyzeOneTable(
            pParse,
            pTab,
            std::ptr::null_mut::<Index>(),
            iStatCur,
            iMem,
            iTab,
        );
        0 as i32;
        k = unsafe { (*k).next };
    }
    loadAnalysis(pParse, iDb);
}

// /*
// ** Generate code that will do an analysis of a single table in
// ** a database.  If pOnlyIdx is not NULL then it is a single index
// ** in pTab that should be analyzed.
// */
fn analyzeTable(mut pParse: *mut Parse, mut pTab: *mut Table, mut pOnlyIdx: *mut Index) {
    let mut iDb: i32 = 0 as i32;
    let mut iStatCur: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    iDb = unsafe { sqlite3SchemaToIndex(unsafe { (*pParse).db }, unsafe { (*pTab).pSchema }) };
    unsafe { sqlite3BeginWriteOperation(pParse, 0 as i32, iDb) };
    iStatCur = unsafe { (*pParse).nTab };
    let __v755: *mut Parse = pParse;
    let __v756: i32 = unsafe { (*__v755).nTab };
    let __v757: i32 = __v756 + (3 as i32);
    unsafe {
        (*__v755).nTab = __v757;
    }
    if pOnlyIdx != std::ptr::null_mut::<Index>() {
        openStatTable(
            pParse,
            iDb,
            iStatCur,
            (unsafe { (*pOnlyIdx).zName }) as *const i8,
            (b"idx\0".as_ptr() as *mut i8) as *const i8,
        );
    } else {
        openStatTable(
            pParse,
            iDb,
            iStatCur,
            (unsafe { (*pTab).zName }) as *const i8,
            (b"tbl\0".as_ptr() as *mut i8) as *const i8,
        );
    }
    analyzeOneTable(
        pParse,
        pTab,
        pOnlyIdx,
        iStatCur,
        (unsafe { (*pParse).nMem }) + (1 as i32),
        unsafe { (*pParse).nTab },
    );
    loadAnalysis(pParse, iDb);
}

// /*
// ** The first argument points to a nul-terminated string containing a
// ** list of space separated integers. Read the first nOut of these into
// ** the array aOut[].
// */
fn decodeIntArray(
    mut zIntArray: *mut i8,
    mut nOut: i32,
    mut aOut: *mut u64,
    mut aLog: *mut i16,
    mut pIndex: *mut Index,
) {
    let mut z: *mut i8 = zIntArray;
    let mut c: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    let mut v: u64 = 0 as u64;
    0 as i32;
    i = 0 as i32;
    '__slate_break_683: loop {
        if !((unsafe { *z }) != (0 as i8) && i < nOut) {
            break;
        }
        v = ((0 as i32) as i64) as u64;
        '__slate_break_684: loop {
            let __v760: i32 = (unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32;
            c = __v760;
            if !(__v760 >= (48 as i32) && c <= (57 as i32)) {
                break;
            }
            v = v
                .wrapping_mul(((10 as i32) as i64) as u64)
                .wrapping_add((c as i64) as u64)
                .wrapping_sub(((48 as i32) as i64) as u64);
            let __v761: *mut i8 = z;
            let __v762: *mut i8 = unsafe { __v761.offset((1 as i32) as isize) };
            z = __v762;
        }
        0 as i32;
        aOut;
        0 as i32;
        unsafe {
            *unsafe { aLog.offset(i as isize) } = unsafe { sqlite3LogEst(v) };
        }
        if ((unsafe { *z }) as i32) == (32 as i32) {
            let __v763: *mut i8 = z;
            let __v764: *mut i8 = unsafe { __v763.offset((1 as i32) as isize) };
            z = __v764;
        }
        let __v758: i32 = i;
        let __v759: i32 = __v758 + (1 as i32);
        i = __v759;
    }
    0 as i32;
    unsafe {
        (*pIndex).__slate_bits_0.__set_bUnordered((0 as i32) as u32);
    }
    unsafe {
        (*pIndex).__slate_bits_0.__set_noSkipScan((0 as i32) as u32);
    }
    '__slate_break_685: while (unsafe { *unsafe { z.offset((0 as i32) as isize) } }) != (0 as i8) {
        if (unsafe {
            sqlite3_strglob(
                (b"unordered*\0".as_ptr() as *mut i8) as *const i8,
                z as *const i8,
            )
        }) == (0 as i32)
        {
            unsafe {
                (*pIndex).__slate_bits_0.__set_bUnordered((1 as i32) as u32);
            }
        } else {
            if (unsafe {
                sqlite3_strglob(
                    (b"sz=[0-9]*\0".as_ptr() as *mut i8) as *const i8,
                    z as *const i8,
                )
            }) == (0 as i32)
            {
                let mut sz: i32 =
                    unsafe { sqlite3Atoi((unsafe { z.offset((3 as i32) as isize) }) as *const i8) };
                if sz < (2 as i32) {
                    sz = 2 as i32;
                }
                unsafe {
                    (*pIndex).szIdxRow = unsafe { sqlite3LogEst((sz as i64) as u64) };
                }
            } else {
                if (unsafe {
                    sqlite3_strglob(
                        (b"noskipscan*\0".as_ptr() as *mut i8) as *const i8,
                        z as *const i8,
                    )
                }) == (0 as i32)
                {
                    unsafe {
                        (*pIndex).__slate_bits_0.__set_noSkipScan((1 as i32) as u32);
                    }
                }
            }
        }
        '__slate_break_689: while ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32)
            != (0 as i32)
            && ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) != (32 as i32)
        {
            let __v765: *mut i8 = z;
            let __v766: *mut i8 = unsafe { __v765.offset((1 as i32) as isize) };
            z = __v766;
        }
        '__slate_break_690: while ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32)
            == (32 as i32)
        {
            let __v767: *mut i8 = z;
            let __v768: *mut i8 = unsafe { __v767.offset((1 as i32) as isize) };
            z = __v768;
        }
    }
}

// /* String containing int array to decode */
// /* Number of slots in aOut[] */
// /* Store integers here */
// /* Or, if aOut==0, here */
// /* Handle extra flags for this index, if not NULL */
// /*
// ** This callback is invoked once for each index when reading the
// ** sqlite_stat1 table.
// **
// **     argv[0] = name of the table
// **     argv[1] = name of the index (might be NULL)
// **     argv[2] = results of analysis - on integer for each column
// **
// ** Entries for which argv[1]==NULL simply record the number of rows in
// ** the table.
// */
#[unsafe(link_section = ".text.slate_distinct.analyze.analysisLoader")]
extern "C-unwind" fn analysisLoader(
    mut pData: *mut (),
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut NotUsed: *mut *mut i8,
) -> i32 {
    let mut pInfo: *mut analysisInfo = pData as *mut analysisInfo;
    let mut pIndex: *mut Index = unsafe { std::mem::zeroed() };
    let mut pTable: *mut Table = unsafe { std::mem::zeroed() };
    let mut z: *const i8 = unsafe { std::mem::zeroed() };
    0 as i32;
    NotUsed;
    argc;
    if argv == std::ptr::null_mut::<*mut i8>()
        || (unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) == std::ptr::null_mut::<i8>()
        || (unsafe { *unsafe { argv.offset((2 as i32) as isize) } }) == std::ptr::null_mut::<i8>()
    {
        return 0 as i32;
    }
    pTable = unsafe {
        sqlite3FindTable(
            unsafe { (*pInfo).db },
            (unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) as *const i8,
            unsafe { (*pInfo).zDatabase },
        )
    };
    if pTable == std::ptr::null_mut::<Table>() {
        return 0 as i32;
    }
    if (unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) == std::ptr::null_mut::<i8>() {
        pIndex = std::ptr::null_mut::<Index>();
    } else {
        if (unsafe {
            sqlite3_stricmp(
                (unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) as *const i8,
                (unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) as *const i8,
            )
        }) == (0 as i32)
        {
            pIndex = unsafe { sqlite3PrimaryKeyIndex(pTable) };
        } else {
            pIndex = unsafe {
                sqlite3FindIndex(
                    unsafe { (*pInfo).db },
                    (unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) as *const i8,
                    unsafe { (*pInfo).zDatabase },
                )
            };
        }
    }
    z = (unsafe { *unsafe { argv.offset((2 as i32) as isize) } }) as *const i8;
    if pIndex != std::ptr::null_mut::<Index>() {
        let mut aiRowEst: *mut u64 = std::ptr::null_mut::<u64>();
        let mut nCol: i32 = (((unsafe { (*pIndex).nKeyCol }) as u32) as i32) + (1 as i32);
        unsafe {
            (*pIndex).__slate_bits_0.__set_bUnordered((0 as i32) as u32);
        }
        decodeIntArray(
            z as *mut i8,
            nCol,
            aiRowEst,
            unsafe { (*pIndex).aiRowLogEst },
            pIndex,
        );
        unsafe {
            (*pIndex).__slate_bits_0.__set_hasStat1((1 as i32) as u32);
        }
        if (unsafe { (*pIndex).pPartIdxWhere }) == std::ptr::null_mut::<Expr>() {
            unsafe {
                (*pTable).nRowLogEst = unsafe {
                    *unsafe { unsafe { (*pIndex).aiRowLogEst }.offset((0 as i32) as isize) }
                };
            }
            let __v769: *mut Table = pTable;
            let __v770: u32 = unsafe { (*__v769).tabFlags };
            let __v771: u32 = __v770 | ((16 as i32) as u32);
            unsafe {
                (*__v769).tabFlags = __v771;
            }
        }
    } else {
        let mut fakeIdx: Index = unsafe { std::mem::zeroed() };
        fakeIdx.szIdxRow = unsafe { (*pTable).szTabRow };
        decodeIntArray(
            z as *mut i8,
            1 as i32,
            std::ptr::null_mut::<u64>(),
            unsafe { std::ptr::addr_of_mut!((*pTable).nRowLogEst) },
            std::ptr::addr_of_mut!(fakeIdx),
        );
        unsafe {
            (*pTable).szTabRow = fakeIdx.szIdxRow;
        }
        let __v772: *mut Table = pTable;
        let __v773: u32 = unsafe { (*__v772).tabFlags };
        let __v774: u32 = __v773 | ((16 as i32) as u32);
        unsafe {
            (*__v772).tabFlags = __v774;
        }
    }
    return 0 as i32;
}
