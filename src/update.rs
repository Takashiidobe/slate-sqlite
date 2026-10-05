unsafe extern "C" {
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3VdbeAddOp0(__v524: *mut Vdbe, __v525: i32) -> i32;
    fn sqlite3VdbeAddOp1(__v526: *mut Vdbe, __v527: i32, __v528: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v529: *mut Vdbe, __v530: i32, __v531: i32, __v532: i32) -> i32;
    fn sqlite3VdbeAddOp3(
        __v533: *mut Vdbe,
        __v534: i32,
        __v535: i32,
        __v536: i32,
        __v537: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4(
        __v538: *mut Vdbe,
        __v539: i32,
        __v540: i32,
        __v541: i32,
        __v542: i32,
        zP4: *const i8,
        __v544: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4Int(
        __v545: *mut Vdbe,
        __v546: i32,
        __v547: i32,
        __v548: i32,
        __v549: i32,
        __v550: i32,
    ) -> i32;
    fn sqlite3VdbeChangeP5(__v551: *mut Vdbe, P5: u16);
    fn sqlite3VdbeJumpHere(__v553: *mut Vdbe, addr: i32);
    fn sqlite3VdbeJumpHereOrPopInst(__v555: *mut Vdbe, addr: i32);
    fn sqlite3VdbeChangeToNoop(__v557: *mut Vdbe, addr: i32) -> i32;
    fn sqlite3VdbeAppendP4(__v559: *mut Vdbe, pP4: *mut (), p4type: i32);
    fn sqlite3VdbeMakeLabel(__v562: *mut Parse) -> i32;
    fn sqlite3VdbeResolveLabel(__v563: *mut Vdbe, __v564: i32);
    fn sqlite3VdbeCurrentAddr(__v565: *mut Vdbe) -> i32;
    fn sqlite3VdbeCountChanges(__v566: *mut Vdbe);
    fn sqlite3VdbeDb(__v567: *mut Vdbe) -> *mut sqlite3;
    fn sqlite3DbMallocRawNN(__v568: *mut sqlite3, __v569: u64) -> *mut ();
    fn sqlite3DbFree(__v570: *mut sqlite3, __v571: *mut ());
    fn sqlite3ErrorMsg(__v572: *mut Parse, __v573: *const i8, ...);
    fn sqlite3PExpr(
        __v574: *mut Parse,
        __v575: i32,
        __v576: *mut Expr,
        __v577: *mut Expr,
    ) -> *mut Expr;
    fn sqlite3ExprDelete(__v578: *mut sqlite3, __v579: *mut Expr);
    fn sqlite3ExprListAppend(
        __v580: *mut Parse,
        __v581: *mut ExprList,
        __v582: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListDelete(__v583: *mut sqlite3, __v584: *mut ExprList);
    fn sqlite3ColumnExpr(__v585: *mut Table, __v586: *mut Column) -> *mut Expr;
    fn sqlite3PrimaryKeyIndex(__v587: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToStorage(__v588: *mut Table, __v589: i16) -> i16;
    fn sqlite3ViewGetColumnNames(__v590: *mut Parse, __v591: *mut Table) -> i32;
    fn sqlite3AutoincrementEnd(pParse: *mut Parse);
    fn sqlite3ComputeGeneratedColumns(__v593: *mut Parse, __v594: i32, __v595: *mut Table);
    fn sqlite3SrcListDelete(__v596: *mut sqlite3, __v597: *mut SrcList);
    fn sqlite3Select(__v598: *mut Parse, __v599: *mut Select, __v600: *mut SelectDest) -> i32;
    fn sqlite3SelectNew(
        __v601: *mut Parse,
        __v602: *mut ExprList,
        __v603: *mut SrcList,
        __v604: *mut Expr,
        __v605: *mut ExprList,
        __v606: *mut Expr,
        __v607: *mut ExprList,
        __v608: u32,
        __v609: *mut Expr,
    ) -> *mut Select;
    fn sqlite3SelectDelete(__v610: *mut sqlite3, __v611: *mut Select);
    fn sqlite3SrcListLookup(__v612: *mut Parse, __v613: *mut SrcList) -> *mut Table;
    fn sqlite3IsReadOnly(__v614: *mut Parse, __v615: *mut Table, __v616: *mut Trigger) -> i32;
    fn sqlite3CodeChangeCount(__v617: *mut Vdbe, __v618: i32, __v619: *const i8);
    fn sqlite3WhereBegin(
        __v628: *mut Parse,
        __v629: *mut SrcList,
        __v630: *mut Expr,
        __v631: *mut ExprList,
        __v632: *mut ExprList,
        __v633: *mut Select,
        __v634: u16,
        __v635: i32,
    ) -> *mut WhereInfo;
    fn sqlite3WhereEnd(__v636: *mut WhereInfo);
    fn sqlite3WhereOkOnePass(__v637: *mut WhereInfo, __v638: *mut i32) -> i32;
    fn sqlite3WhereUsesDeferredSeek(__v639: *mut WhereInfo) -> i32;
    fn sqlite3ExprCodeGetColumnOfTable(
        __v640: *mut Vdbe,
        __v641: *mut Table,
        __v642: i32,
        __v643: i32,
        __v644: i32,
    );
    fn sqlite3ExprCode(__v645: *mut Parse, __v646: *mut Expr, __v647: i32);
    fn sqlite3ExprIfFalse(__v648: *mut Parse, __v649: *mut Expr, __v650: i32, __v651: i32);
    fn sqlite3GetVdbe(__v652: *mut Parse) -> *mut Vdbe;
    fn sqlite3IsRowid(__v653: *const i8) -> i32;
    fn sqlite3GenerateRowIndexDelete(
        __v654: *mut Parse,
        __v655: *mut Table,
        __v656: i32,
        __v657: i32,
        __v658: *mut i32,
        __v659: i32,
    );
    fn sqlite3ExprReferencesUpdatedColumn(__v660: *mut Expr, __v661: *mut i32, __v662: i32) -> i32;
    fn sqlite3GenerateConstraintChecks(
        __v663: *mut Parse,
        __v664: *mut Table,
        __v665: *mut i32,
        __v666: i32,
        __v667: i32,
        __v668: i32,
        __v669: i32,
        __v670: u8,
        __v671: u8,
        __v672: i32,
        __v673: *mut i32,
        __v674: *mut i32,
        __v675: *mut Upsert,
    );
    fn sqlite3CompleteInsertion(
        __v676: *mut Parse,
        __v677: *mut Table,
        __v678: i32,
        __v679: i32,
        __v680: i32,
        __v681: *mut i32,
        __v682: i32,
        __v683: i32,
        __v684: i32,
    );
    fn sqlite3OpenTableAndIndices(
        __v685: *mut Parse,
        __v686: *mut Table,
        __v687: i32,
        __v688: u8,
        __v689: i32,
        __v690: *mut u8,
        __v691: *mut i32,
        __v692: *mut i32,
    ) -> i32;
    fn sqlite3BeginWriteOperation(__v693: *mut Parse, __v694: i32, __v695: i32);
    fn sqlite3MultiWrite(__v696: *mut Parse);
    fn sqlite3MayAbort(__v697: *mut Parse);
    fn sqlite3ExprDup(__v698: *mut sqlite3, __v699: *const Expr, __v700: i32) -> *mut Expr;
    fn sqlite3SrcListDup(__v701: *mut sqlite3, __v702: *const SrcList, __v703: i32)
    -> *mut SrcList;
    fn sqlite3MaterializeView(
        __v704: *mut Parse,
        __v705: *mut Table,
        __v706: *mut Expr,
        __v707: *mut ExprList,
        __v708: *mut Expr,
        __v709: i32,
    );
    fn sqlite3TriggersExist(
        __v710: *mut Parse,
        __v711: *mut Table,
        __v712: i32,
        __v713: *mut ExprList,
        pMask: *mut i32,
    ) -> *mut Trigger;
    fn sqlite3CodeRowTrigger(
        __v715: *mut Parse,
        __v716: *mut Trigger,
        __v717: i32,
        __v718: *mut ExprList,
        __v719: i32,
        __v720: *mut Table,
        __v721: i32,
        __v722: i32,
        __v723: i32,
    );
    fn sqlite3TriggerColmask(
        __v724: *mut Parse,
        __v725: *mut Trigger,
        __v726: *mut ExprList,
        __v727: i32,
        __v728: i32,
        __v729: *mut Table,
        __v730: i32,
    ) -> u32;
    fn sqlite3ColumnIndex(pTab: *mut Table, zCol: *const i8) -> i32;
    fn sqlite3AuthCheck(
        __v733: *mut Parse,
        __v734: i32,
        __v735: *const i8,
        __v736: *const i8,
        __v737: *const i8,
    ) -> i32;
    fn sqlite3AuthContextPush(__v738: *mut Parse, __v739: *mut AuthContext, __v740: *const i8);
    fn sqlite3AuthContextPop(__v741: *mut AuthContext);
    fn sqlite3IndexAffinityStr(__v742: *mut sqlite3, __v743: *mut Index) -> *const i8;
    fn sqlite3TableAffinity(__v744: *mut Vdbe, __v745: *mut Table, __v746: i32);
    fn sqlite3ValueFromExpr(
        __v747: *mut sqlite3,
        __v748: *const Expr,
        __v749: u8,
        __v750: u8,
        __v751: *mut *mut sqlite3_value,
    ) -> i32;
    fn sqlite3ResolveExprNames(__v752: *mut NameContext, __v753: *mut Expr) -> i32;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, __v759: *mut Schema) -> i32;
    fn sqlite3KeyInfoOfIndex(__v760: *mut Parse, __v761: *mut Index) -> *mut KeyInfo;
    fn sqlite3SelectDestInit(__v762: *mut SelectDest, __v763: i32, __v764: i32);
    fn sqlite3GetVTable(__v765: *mut sqlite3, __v766: *mut Table) -> *mut VTable;
    fn sqlite3VtabMakeWritable(__v767: *mut Parse, __v768: *mut Table);
    fn sqlite3FkCheck(
        __v769: *mut Parse,
        __v770: *mut Table,
        __v771: i32,
        __v772: i32,
        __v773: *mut i32,
        __v774: i32,
    );
    fn sqlite3FkActions(
        __v775: *mut Parse,
        __v776: *mut Table,
        __v777: *mut ExprList,
        __v778: i32,
        __v779: *mut i32,
        __v780: i32,
    );
    fn sqlite3FkRequired(
        __v781: *mut Parse,
        __v782: *mut Table,
        __v783: *mut i32,
        __v784: i32,
    ) -> i32;
    fn sqlite3FkOldmask(__v785: *mut Parse, __v786: *mut Table) -> u32;
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
    trace: __SlateRecord162,
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
    u1: __SlateRecord163,
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
struct AuthContext {
    zAuthContext: *const i8,
    pParse: *mut Parse,
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
    u: __SlateRecord173,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord174,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord175,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord176,
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
    u: __SlateRecord164,
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
struct NameContext {
    pParse: *mut Parse,
    pSrcList: *mut SrcList,
    uNC: __SlateRecord188,
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
    __slate_bits_0: __slate_bits::__SlateBits101U0,
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
    u1: __SlateRecord190,
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
    fg: __SlateRecord183,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord184,
    u2: __SlateRecord185,
    u3: __SlateRecord186,
    u4: __SlateRecord187,
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
    u: __SlateRecord165,
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
    __slate_bits_0: __slate_bits::__SlateBits161U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord162 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord163 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord164 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord165 {
    tab: __SlateRecord166,
    view: __SlateRecord167,
    vtab: __SlateRecord168,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord166 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord167 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord168 {
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
union __SlateRecord173 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord174 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord175 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord177,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord177 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord179,
    u: __SlateRecord180,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord179 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits179U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    x: __SlateRecord181,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord181 {
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
struct __SlateRecord183 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits183U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord184 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord185 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    pEList: *mut ExprList,
    pAggInfo: *mut AggInfo,
    pUpsert: *mut Upsert,
    iBaseReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord190 {
    cr: __SlateRecord191,
    d: __SlateRecord192,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord191 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord192 {
    pReturning: *mut Returning,
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
    pub struct __SlateBits63U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits179U0 {
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
    pub struct __SlateBits183U0 {
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
    pub struct __SlateBits161U0 {
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
    pub struct __SlateBits101U0 {
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

// /* Parse context */
// /* Cursor for open eph. table */
// /* PK if table 0 is WITHOUT ROWID */
// /* List of expressions to return */
// /* List of tables to select from */
// /* WHERE clause for query */
// /* ORDER BY clause */
// /* LIMIT clause */
// /*
// ** Process an UPDATE statement.
// **
// **   UPDATE OR IGNORE tbl SET a=b, c=d FROM tbl2... WHERE e<5 AND f NOT NULL;
// **          \_______/ \_/     \______/      \_____/       \________________/
// **           onError   |      pChanges         |                pWhere
// **                     \_______________________/
// **                               pTabList
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Update(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pChanges: *mut ExprList,
    mut pWhere: *mut Expr,
    mut onError: i32,
    mut pOrderBy: *mut ExprList,
    mut pLimit: *mut Expr,
    mut pUpsert: *mut Upsert,
) {
    let mut __slate_storage_915: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_915: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_915) as *mut i32;
    let mut __slate_storage_914: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_914: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_914) as *mut i32;
    let mut __slate_storage_913: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_913: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_913) as *mut i32;
    let mut __slate_storage_912: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_912: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_912) as *mut i32;
    let mut __slate_storage_917: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_917: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_917) as *mut i32;
    let mut __slate_storage_916: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_916: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_916) as *mut i32;
    let mut __slate_storage_911: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_911: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_911) as *mut i32;
    let mut __slate_storage_908: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_908: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_908) as *mut i32;
    let mut __slate_storage_907: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_907: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_907) as *mut i32;
    let mut __slate_storage_906: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_906: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_906) as *mut i32;
    let mut __slate_storage_905: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_905: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_905) as *mut i32;
    let mut __slate_storage_910: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_910: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_910) as *mut i32;
    let mut __slate_storage_909: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_909: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_909) as *mut i32;
    let mut __slate_storage_491: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_491: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_491) as *mut i32;
    let mut __slate_storage_904: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_904: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_904) as *mut i32;
    let mut __slate_storage_903: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_903: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_903) as *mut i32;
    let mut __slate_storage_902: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_902: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_902) as *mut i32;
    let mut __slate_storage_490: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_490: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_490) as *mut u32;
    let mut __slate_storage_901: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_901: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_901) as *mut u32;
    let mut __slate_storage_900: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_900: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_900) as *mut u32;
    let mut __slate_storage_899: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_899: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_899) as *mut u32;
    let mut __slate_storage_489: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_489: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_489) as *mut u32;
    let mut __slate_storage_898: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_898: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_898) as *mut i32;
    let mut __slate_storage_897: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_897: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_897) as *mut i32;
    let mut __slate_storage_488: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_488: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_488) as *mut i32;
    let mut __slate_storage_487: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_487: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_487) as *mut i32;
    let mut __slate_storage_486: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_486: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_486) as *mut i32;
    let mut __slate_storage_894: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_894: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_894) as *mut i32;
    let mut __slate_storage_893: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_893: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_893) as *mut i32;
    let mut __slate_storage_892: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_892: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_892) as *mut *mut Parse;
    let mut __slate_storage_896: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_896: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_896) as *mut i32;
    let mut __slate_storage_895: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_895: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_895) as *mut i32;
    let mut __slate_storage_485: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_485: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_485) as *mut i32;
    let mut __slate_storage_891: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_891: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_891) as *mut i32;
    let mut __slate_storage_890: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_890: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_890) as *mut i32;
    let mut __slate_storage_877: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_877: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_877) as *mut i32;
    let mut __slate_storage_876: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_876: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_876) as *mut i32;
    let mut __slate_storage_875: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_875: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_875) as *mut *mut Parse;
    let mut __slate_storage_484: std::mem::MaybeUninit<*mut KeyInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_484: *mut *mut KeyInfo =
        std::ptr::addr_of_mut!(__slate_storage_484) as *mut *mut KeyInfo;
    let mut __slate_storage_889: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_889: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_889) as *mut i32;
    let mut __slate_storage_888: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_888: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_888) as *mut i32;
    let mut __slate_storage_887: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_887: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_887) as *mut *mut Parse;
    let mut __slate_storage_483: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_483: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_483) as *mut i32;
    let mut __slate_storage_886: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_886: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_886) as *mut i32;
    let mut __slate_storage_885: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_885: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_885) as *mut i32;
    let mut __slate_storage_884: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_884: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_884) as *mut *mut Parse;
    let mut __slate_storage_883: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_883: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_883) as *mut i32;
    let mut __slate_storage_882: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_882: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_882) as *mut i32;
    let mut __slate_storage_881: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_881: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_881) as *mut *mut Parse;
    let mut __slate_storage_880: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_880: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_880) as *mut i32;
    let mut __slate_storage_879: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_879: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_879) as *mut i32;
    let mut __slate_storage_878: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_878: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_878) as *mut *mut Parse;
    let mut __slate_storage_874: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_874: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_874) as *mut i32;
    let mut __slate_storage_873: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_873: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_873) as *mut i32;
    let mut __slate_storage_872: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_872: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_872) as *mut *mut Parse;
    let mut __slate_storage_871: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_871: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_871) as *mut i32;
    let mut __slate_storage_870: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_870: *mut bool = std::ptr::addr_of_mut!(__slate_storage_870) as *mut bool;
    let mut __slate_storage_869: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_869: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_869) as *mut i32;
    let mut __slate_storage_868: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_868: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_868) as *mut i32;
    let mut __slate_storage_867: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_867: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_867) as *mut *mut Parse;
    let mut __slate_storage_866: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_866: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_866) as *mut i32;
    let mut __slate_storage_865: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_865: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_865) as *mut i32;
    let mut __slate_storage_864: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_864: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_864) as *mut *mut Parse;
    let mut __slate_storage_863: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_863: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_863) as *mut i32;
    let mut __slate_storage_862: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_862: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_862) as *mut i32;
    let mut __slate_storage_861: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_861: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_861) as *mut *mut Parse;
    let mut __slate_storage_860: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_860: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_860) as *mut i32;
    let mut __slate_storage_859: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_859: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_859) as *mut i32;
    let mut __slate_storage_858: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_858: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_858) as *mut i32;
    let mut __slate_storage_857: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_857: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_857) as *mut *mut Parse;
    let mut __slate_storage_856: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_856: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_856) as *mut i32;
    let mut __slate_storage_855: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_855: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_855) as *mut i32;
    let mut __slate_storage_854: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_854: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_854) as *mut *mut Parse;
    let mut __slate_storage_838: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_838: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_838) as *mut i32;
    let mut __slate_storage_837: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_837: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_837) as *mut i32;
    let mut __slate_storage_845: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_845: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_845) as *mut i32;
    let mut __slate_storage_844: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_844: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_844) as *mut i32;
    let mut __slate_storage_843: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_843: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_843) as *mut *mut Parse;
    let mut __slate_storage_842: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_842: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_842) as *mut i32;
    let mut __slate_storage_841: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_841: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_841) as *mut i32;
    let mut __slate_storage_840: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_840: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_840) as *mut *mut Parse;
    let mut __slate_storage_847: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_847: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_847) as *mut i32;
    let mut __slate_storage_846: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_846: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_846) as *mut i32;
    let mut __slate_storage_853: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_853: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_853) as *mut i32;
    let mut __slate_storage_852: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_852: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_852) as *mut i32;
    let mut __slate_storage_851: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_851: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_851) as *mut *mut Parse;
    let mut __slate_storage_850: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_850: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_850) as *mut i32;
    let mut __slate_storage_849: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_849: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_849) as *mut i32;
    let mut __slate_storage_848: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_848: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_848) as *mut *mut Parse;
    let mut __slate_storage_839: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_839: *mut bool = std::ptr::addr_of_mut!(__slate_storage_839) as *mut bool;
    let mut __slate_storage_482: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_482: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_482) as *mut i32;
    let mut __slate_storage_836: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_836: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_836) as *mut *mut Index;
    let mut __slate_storage_835: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_835: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_835) as *mut i32;
    let mut __slate_storage_834: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_834: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_834) as *mut i32;
    let mut __slate_storage_481: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_481: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_481) as *mut i32;
    let mut __slate_storage_831: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_831: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_831) as *mut i32;
    let mut __slate_storage_830: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_830: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_830) as *mut i32;
    let mut __slate_storage_480: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_480: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_480) as *mut i32;
    let mut __slate_storage_833: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_833: *mut bool = std::ptr::addr_of_mut!(__slate_storage_833) as *mut bool;
    let mut __slate_storage_832: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_832: *mut bool = std::ptr::addr_of_mut!(__slate_storage_832) as *mut bool;
    let mut __slate_storage_829: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_829: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_829) as *mut i32;
    let mut __slate_storage_828: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_828: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_828) as *mut i32;
    let mut __slate_storage_824: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_824: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_824) as *mut i32;
    let mut __slate_storage_823: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_823: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_823) as *mut i32;
    let mut __slate_storage_827: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_827: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_827) as *mut i32;
    let mut __slate_storage_826: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_826: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_826) as *mut i32;
    let mut __slate_storage_825: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_825: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_825) as *mut *mut Parse;
    let mut __slate_storage_822: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_822: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_822) as *mut *mut Index;
    let mut __slate_storage_821: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_821: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_821) as *mut *mut Index;
    let mut __slate_storage_820: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_820: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_820) as *mut i32;
    let mut __slate_storage_819: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_819: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_819) as *mut i32;
    let mut __slate_storage_818: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_818: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_818) as *mut i32;
    let mut __slate_storage_817: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_817: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_817) as *mut *mut Parse;
    let mut __slate_storage_479: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_479: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_479) as *mut i32;
    let mut __slate_storage_478: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_478: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_478) as *mut i32;
    let mut __slate_storage_477: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_477: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_477) as *mut i32;
    let mut __slate_storage_476: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_476: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_476) as *mut i32;
    let mut __slate_storage_475: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_475: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_475) as *mut i32;
    let mut __slate_storage_474: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_474: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_474) as *mut i32;
    let mut __slate_storage_473: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_473: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_473) as *mut i32;
    let mut __slate_storage_472: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_472: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_472) as *mut i32;
    let mut __slate_storage_471: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_471: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_471) as *mut i32;
    let mut __slate_storage_470: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_470: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_470) as *mut i32;
    let mut __slate_storage_469: std::mem::MaybeUninit<i16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_469: *mut i16 = std::ptr::addr_of_mut!(__slate_storage_469) as *mut i16;
    let mut __slate_storage_468: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_468: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_468) as *mut i32;
    let mut __slate_storage_467: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_467: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_467) as *mut i32;
    let mut __slate_storage_466: std::mem::MaybeUninit<[i32; 2]> = std::mem::MaybeUninit::uninit();
    let __slate_slot_466: *mut [i32; 2] =
        std::ptr::addr_of_mut!(__slate_storage_466) as *mut [i32; 2];
    let mut __slate_storage_465: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_465: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_465) as *mut i32;
    let mut __slate_storage_464: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_464: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_464) as *mut i32;
    let mut __slate_storage_463: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_463: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_463) as *mut i32;
    let mut __slate_storage_462: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_462: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_462) as *mut i32;
    let mut __slate_storage_461: std::mem::MaybeUninit<*mut Trigger> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_461: *mut *mut Trigger =
        std::ptr::addr_of_mut!(__slate_storage_461) as *mut *mut Trigger;
    let mut __slate_storage_460: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_460: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_460) as *mut i32;
    let mut __slate_storage_459: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_459: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_459) as *mut i32;
    let mut __slate_storage_458: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_458: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_458) as *mut i32;
    let mut __slate_storage_457: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_457: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_457) as *mut i32;
    let mut __slate_storage_456: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_456: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_456) as *mut i32;
    let mut __slate_storage_455: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_455: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_455) as *mut i32;
    let mut __slate_storage_454: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_454: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_454) as *mut i32;
    let mut __slate_storage_453: std::mem::MaybeUninit<NameContext> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_453: *mut NameContext =
        std::ptr::addr_of_mut!(__slate_storage_453) as *mut NameContext;
    let mut __slate_storage_452: std::mem::MaybeUninit<AuthContext> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_452: *mut AuthContext =
        std::ptr::addr_of_mut!(__slate_storage_452) as *mut AuthContext;
    let mut __slate_storage_451: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_451: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_451) as *mut i32;
    let mut __slate_storage_450: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_450: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_450) as *mut *mut Expr;
    let mut __slate_storage_449: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_449: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_449) as *mut u8;
    let mut __slate_storage_448: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_448: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_448) as *mut u8;
    let mut __slate_storage_447: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_447: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_447) as *mut u8;
    let mut __slate_storage_446: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_446: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_446) as *mut *mut u8;
    let mut __slate_storage_445: std::mem::MaybeUninit<*mut i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_445: *mut *mut i32 =
        std::ptr::addr_of_mut!(__slate_storage_445) as *mut *mut i32;
    let mut __slate_storage_444: std::mem::MaybeUninit<*mut i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_444: *mut *mut i32 =
        std::ptr::addr_of_mut!(__slate_storage_444) as *mut *mut i32;
    let mut __slate_storage_443: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_443: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_443) as *mut *mut sqlite3;
    let mut __slate_storage_442: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_442: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_442) as *mut i32;
    let mut __slate_storage_441: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_441: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_441) as *mut i32;
    let mut __slate_storage_440: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_440: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_440) as *mut i32;
    let mut __slate_storage_439: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_439: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_439) as *mut i32;
    let mut __slate_storage_438: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_438: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_438) as *mut i32;
    let mut __slate_storage_437: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_437: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_437) as *mut *mut Index;
    let mut __slate_storage_436: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_436: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_436) as *mut *mut Index;
    let mut __slate_storage_435: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_435: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_435) as *mut *mut Vdbe;
    let mut __slate_storage_434: std::mem::MaybeUninit<*mut WhereInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_434: *mut *mut WhereInfo =
        std::ptr::addr_of_mut!(__slate_storage_434) as *mut *mut WhereInfo;
    let mut __slate_storage_433: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_433: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_433) as *mut i32;
    let mut __slate_storage_432: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_432: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_432) as *mut *mut Table;
    let mut __slate_storage_431: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_431: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_431) as *mut i32;
    let mut __slate_storage_430: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_430: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_430) as *mut i32;
    let mut __slate_storage_429: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_429: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_429) as *mut i32;
    unsafe {
        '__join_0: {
            // /* Loop counters */
            // /* The table to be updated */
            // /* VDBE instruction address of the start of the loop */
            std::ptr::write(__slate_slot_433, 0 as i32);
            // /* Information about the WHERE clause */
            std::ptr::write(__slate_slot_434, std::ptr::null_mut::<WhereInfo>());
            // /* The virtual database engine */
            // /* For looping over indices */
            // /* The PRIMARY KEY index for WITHOUT ROWID tables */
            // /* Number of indices that need updating */
            // /* Total number of indexes */
            // /* Base cursor number */
            // /* Cursor for the canonical data btree */
            // /* Cursor for the first index */
            // /* The database structure */
            // /* Registers for to each index and the main table */
            std::ptr::write(__slate_slot_444, std::ptr::null_mut::<i32>());
            // /* aXRef[i] is the index in pChanges->a[] of the
            //                          ** an expression for the i-th column of the table.
            //                          ** aXRef[i]==-1 if the i-th column is not changed. */
            std::ptr::write(__slate_slot_445, std::ptr::null_mut::<i32>());
            // /* 1 for tables and indices to be opened */
            // /* PRIMARY KEY changed in a WITHOUT ROWID table */
            // /* Rowid changed in a normal table */
            // /* Either chngPk or chngRowid */
            // /* Expression defining the new record number */
            std::ptr::write(__slate_slot_450, std::ptr::null_mut::<Expr>());
            // /* Index of "rowid=" (or IPK) assignment in pChanges */
            std::ptr::write(__slate_slot_451, -(1 as i32));
            // /* The authorization context */
            // /* The name-context to resolve expressions in */
            // /* Database containing the table being updated */
            // /* ONEPASS_XXX value from where.c */
            // /* True if foreign key processing is required */
            // /* Jump here to break out of UPDATE loop */
            // /* Jump here to continue next step of UPDATE loop */
            // /* Flags for sqlite3WhereBegin() */
            // /* True when updating a view (INSTEAD OF trigger) */
            // /* List of triggers on pTab, if required */
            // /* Mask of TRIGGER_BEFORE|TRIGGER_AFTER */
            // /* Mask of NEW.* columns accessed by BEFORE triggers */
            // /* Ephemeral table holding all primary key values */
            std::ptr::write(__slate_slot_464, 0 as i32);
            // /* Number of elements in regKey for WITHOUT ROWID */
            std::ptr::write(__slate_slot_465, 0 as i32);
            // /* The write cursors opened by WHERE_ONEPASS */
            // /* Address of OP_OpenEphemeral */
            std::ptr::write(__slate_slot_467, 0 as i32);
            // /* First of nPk cells holding PRIMARY KEY value */
            std::ptr::write(__slate_slot_468, 0 as i32);
            // /* Number of components of the PRIMARY KEY */
            std::ptr::write(__slate_slot_469, (0 as i32) as i16);
            // /* True if REPLACE conflict resolution might happen */
            std::ptr::write(__slate_slot_470, 0 as i32);
            // /* The OP_FinishSeek opcode is needed */
            std::ptr::write(__slate_slot_471, 1 as i32);
            // /* If there is a FROM, pChanges->nExpr, else 0 */
            std::ptr::write(__slate_slot_472, 0 as i32);
            // /* Register Allocations */
            // /* A count of rows changed */
            std::ptr::write(__slate_slot_473, 0 as i32);
            // /* The old rowid */
            std::ptr::write(__slate_slot_474, 0 as i32);
            // /* The new rowid */
            std::ptr::write(__slate_slot_475, 0 as i32);
            // /* Content of the NEW.* table in triggers */
            std::ptr::write(__slate_slot_476, 0 as i32);
            // /* Content of OLD.* table in triggers */
            std::ptr::write(__slate_slot_477, 0 as i32);
            // /* Rowset of rows to be updated */
            std::ptr::write(__slate_slot_478, 0 as i32);
            // /* composite PRIMARY KEY value */
            std::ptr::write(__slate_slot_479, 0 as i32);
            unsafe {
                memset(
                    std::ptr::addr_of_mut!(*__slate_slot_452) as *mut (),
                    0 as i32,
                    16 as u64,
                )
            };
            *__slate_slot_443 = unsafe { (*pParse).db };
            0 as i32;
            if (unsafe { (*pParse).nErr }) != (0 as i32) {
            } else {
                0 as i32;
                // /* Locate the table which we want to update.
                //   */
                *__slate_slot_432 = unsafe { sqlite3SrcListLookup(pParse, pTabList) };
                if *__slate_slot_432 == std::ptr::null_mut::<Table>() {
                } else {
                    *__slate_slot_454 = unsafe {
                        sqlite3SchemaToIndex(unsafe { (*pParse).db }, unsafe {
                            (*(*__slate_slot_432)).pSchema
                        })
                    };
                    // /* Figure out if we have any triggers and if the table being
                    //   ** updated is a view.
                    //   */
                    *__slate_slot_461 = unsafe {
                        sqlite3TriggersExist(
                            pParse,
                            *__slate_slot_432,
                            130 as i32,
                            pChanges,
                            std::ptr::addr_of_mut!(*__slate_slot_462),
                        )
                    };
                    *__slate_slot_460 = ((((unsafe { (*(*__slate_slot_432)).eTabType }) as u32)
                        as i32)
                        == (2 as i32)) as i32;
                    0 as i32;
                    // /* If there was a FROM clause, set nChangeFrom to the number of expressions
                    //   ** in the change-list. Otherwise, set it to 0. There cannot be a FROM
                    //   ** clause if this function is being called to generate code for part of
                    //   ** an UPSERT statement.  */
                    *__slate_slot_472 = if (unsafe { (*pTabList).nSrc }) > (1 as i32) {
                        unsafe { (*pChanges).nExpr }
                    } else {
                        0 as i32
                    };
                    0 as i32;
                    if (unsafe { sqlite3ViewGetColumnNames(pParse, *__slate_slot_432) })
                        != (0 as i32)
                    {
                    } else {
                        if (unsafe {
                            sqlite3IsReadOnly(pParse, *__slate_slot_432, *__slate_slot_461)
                        }) != (0 as i32)
                        {
                        } else {
                            // /* Allocate a cursors for the main database table and for all indices.
                            //   ** The index cursors might not be used, but if they are used they
                            //   ** need to occur right after the database cursor.  So go ahead and
                            //   ** allocate enough space, just in case.
                            //   */
                            std::ptr::write(__slate_slot_817, pParse);
                            std::ptr::write(__slate_slot_818, unsafe {
                                (*(*__slate_slot_817)).nTab
                            });
                            std::ptr::write(__slate_slot_819, *__slate_slot_818 + (1 as i32));
                            unsafe {
                                (*(*__slate_slot_817)).nTab = *__slate_slot_819;
                            }
                            std::ptr::write(__slate_slot_820, *__slate_slot_818);
                            *__slate_slot_441 = *__slate_slot_820;
                            *__slate_slot_440 = *__slate_slot_820;
                            *__slate_slot_442 = *__slate_slot_441 + (1 as i32);
                            if (unsafe { (*(*__slate_slot_432)).tabFlags }) & ((128 as i32) as u32)
                                == ((0 as i32) as u32)
                            {
                                *__slate_slot_821 = std::ptr::null_mut::<Index>();
                            } else {
                                *__slate_slot_821 =
                                    unsafe { sqlite3PrimaryKeyIndex(*__slate_slot_432) };
                            }
                            *__slate_slot_437 = *__slate_slot_821;
                            {}
                            *__slate_slot_438 = 0 as i32;
                            std::ptr::write(__slate_slot_822, unsafe {
                                (*(*__slate_slot_432)).pIndex
                            });
                            *__slate_slot_436 = *__slate_slot_822;
                            loop {
                                if *__slate_slot_436 != std::ptr::null_mut::<Index>() {
                                    if *__slate_slot_437 == *__slate_slot_436 {
                                        *__slate_slot_441 = unsafe { (*pParse).nTab };
                                    }
                                    std::ptr::write(__slate_slot_825, pParse);
                                    std::ptr::write(__slate_slot_826, unsafe {
                                        (*(*__slate_slot_825)).nTab
                                    });
                                    std::ptr::write(
                                        __slate_slot_827,
                                        *__slate_slot_826 + (1 as i32),
                                    );
                                    unsafe {
                                        (*(*__slate_slot_825)).nTab = *__slate_slot_827;
                                    }
                                    *__slate_slot_436 = unsafe { (*(*__slate_slot_436)).pNext };
                                    std::ptr::write(__slate_slot_823, *__slate_slot_438);
                                    std::ptr::write(
                                        __slate_slot_824,
                                        *__slate_slot_823 + (1 as i32),
                                    );
                                    *__slate_slot_438 = *__slate_slot_824;
                                } else {
                                    break;
                                }
                            }
                            if pUpsert != std::ptr::null_mut::<Upsert>() {
                                // /* On an UPSERT, reuse the same cursors already opened by INSERT */
                                *__slate_slot_441 = unsafe { (*pUpsert).iDataCur };
                                *__slate_slot_442 = unsafe { (*pUpsert).iIdxCur };
                                unsafe {
                                    (*pParse).nTab = *__slate_slot_440;
                                }
                            }
                            unsafe {
                                (*unsafe {
                                    unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                                        .offset((0 as i32) as isize)
                                })
                                .iCursor = *__slate_slot_441;
                            }
                            // /* Allocate space for aXRef[], aRegIdx[], and aToOpen[].
                            //   ** Initialize aXRef[] and aToOpen[] to their default values.
                            //   */
                            *__slate_slot_445 = (unsafe {
                                sqlite3DbMallocRawNN(
                                    *__slate_slot_443,
                                    (4 as u64)
                                        .wrapping_mul(
                                            ((((unsafe { (*(*__slate_slot_432)).nCol }) as i32)
                                                + *__slate_slot_438
                                                + (1 as i32))
                                                as i64)
                                                as u64,
                                        )
                                        .wrapping_add((*__slate_slot_438 as i64) as u64)
                                        .wrapping_add(((2 as i32) as i64) as u64),
                                )
                            }) as *mut i32;
                            if *__slate_slot_445 == std::ptr::null_mut::<i32>() {
                            } else {
                                *__slate_slot_444 = unsafe {
                                    (*__slate_slot_445).offset(
                                        ((unsafe { (*(*__slate_slot_432)).nCol }) as i32) as isize,
                                    )
                                };
                                *__slate_slot_446 = (unsafe {
                                    unsafe {
                                        (*__slate_slot_444).offset(*__slate_slot_438 as isize)
                                    }
                                    .offset((1 as i32) as isize)
                                }) as *mut u8;
                                unsafe {
                                    memset(
                                        *__slate_slot_446 as *mut (),
                                        1 as i32,
                                        ((*__slate_slot_438 + (1 as i32)) as i64) as u64,
                                    )
                                };
                                unsafe {
                                    *unsafe {
                                        (*__slate_slot_446)
                                            .offset((*__slate_slot_438 + (1 as i32)) as isize)
                                    } = ((0 as i32) as i8) as u8;
                                }
                                *__slate_slot_429 = 0 as i32;
                                loop {
                                    if *__slate_slot_429
                                        < ((unsafe { (*(*__slate_slot_432)).nCol }) as i32)
                                    {
                                        unsafe {
                                            *unsafe {
                                                (*__slate_slot_445)
                                                    .offset(*__slate_slot_429 as isize)
                                            } = -(1 as i32);
                                        }
                                        std::ptr::write(__slate_slot_828, *__slate_slot_429);
                                        std::ptr::write(
                                            __slate_slot_829,
                                            *__slate_slot_828 + (1 as i32),
                                        );
                                        *__slate_slot_429 = *__slate_slot_829;
                                    } else {
                                        break;
                                    }
                                }
                                // /* Initialize the name-context */
                                unsafe {
                                    memset(
                                        std::ptr::addr_of_mut!(*__slate_slot_453) as *mut (),
                                        0 as i32,
                                        56 as u64,
                                    )
                                };
                                (*__slate_slot_453).pParse = pParse;
                                (*__slate_slot_453).pSrcList = pTabList;
                                unsafe {
                                    (*__slate_slot_453).uNC.pUpsert = pUpsert;
                                }
                                (*__slate_slot_453).ncFlags = 512 as i32;
                                // /* Begin generating code. */
                                *__slate_slot_435 = unsafe { sqlite3GetVdbe(pParse) };
                                if *__slate_slot_435 == std::ptr::null_mut::<Vdbe>() {
                                } else {
                                    // /* Resolve the column names in all the expressions of the
                                    //   ** of the UPDATE statement.  Also find the column index
                                    //   ** for each column to be updated in the pChanges array.  For each
                                    //   ** column to be updated, make sure we have authorization to change
                                    //   ** that column.
                                    //   */
                                    *__slate_slot_447 = ((0 as i32) as i8) as u8;
                                    *__slate_slot_448 = ((0 as i32) as i8) as u8;
                                    *__slate_slot_429 = 0 as i32;
                                    '__join_203: {
                                        '__join_208: {
                                            loop {
                                                if *__slate_slot_429 < unsafe { (*pChanges).nExpr }
                                                {
                                                    // /* If this is an UPDATE with a FROM clause, do not resolve expressions
                                                    //     ** here. The call to sqlite3Select() below will do that. */
                                                    if *__slate_slot_472 == (0 as i32) {
                                                        *__slate_slot_832 = (unsafe {
                                                            sqlite3ResolveExprNames(
                                                                std::ptr::addr_of_mut!(
                                                                    *__slate_slot_453
                                                                ),
                                                                unsafe {
                                                                    (*unsafe { unsafe { std::ptr::addr_of_mut!((*pChanges).a) as *mut ExprList_item }.offset(*__slate_slot_429 as isize) }).pExpr
                                                                },
                                                            )
                                                        }) != (0 as i32);
                                                    } else {
                                                        *__slate_slot_832 = false as bool;
                                                    }
                                                    if *__slate_slot_832 {
                                                        break '__join_0;
                                                    } else {
                                                        *__slate_slot_430 = unsafe {
                                                            sqlite3ColumnIndex(
                                                                *__slate_slot_432,
                                                                (unsafe {
                                                                    (*unsafe { unsafe { std::ptr::addr_of_mut!((*pChanges).a) as *mut ExprList_item }.offset(*__slate_slot_429 as isize) }).zEName
                                                                })
                                                                    as *const i8,
                                                            )
                                                        };
                                                        if *__slate_slot_430 >= (0 as i32) {
                                                            if *__slate_slot_430
                                                                == ((unsafe {
                                                                    (*(*__slate_slot_432)).iPKey
                                                                })
                                                                    as i32)
                                                            {
                                                                *__slate_slot_448 =
                                                                    ((1 as i32) as i8) as u8;
                                                                *__slate_slot_450 = unsafe {
                                                                    (*unsafe { unsafe { std::ptr::addr_of_mut!((*pChanges).a) as *mut ExprList_item }.offset(*__slate_slot_429 as isize) }).pExpr
                                                                };
                                                                *__slate_slot_451 =
                                                                    *__slate_slot_429;
                                                            } else {
                                                                if *__slate_slot_437
                                                                    != std::ptr::null_mut::<Index>()
                                                                    && (((unsafe {
                                                                        (*unsafe { unsafe { (*(*__slate_slot_432)).aCol }.offset(*__slate_slot_430 as isize) }).colFlags
                                                                    })
                                                                        as u32)
                                                                        as i32)
                                                                        & (1 as i32)
                                                                        != (0 as i32)
                                                                {
                                                                    *__slate_slot_447 =
                                                                        ((1 as i32) as i8) as u8;
                                                                } else {
                                                                    if (((unsafe {
                                                                        (*unsafe { unsafe { (*(*__slate_slot_432)).aCol }.offset(*__slate_slot_430 as isize) }).colFlags
                                                                    })
                                                                        as u32)
                                                                        as i32)
                                                                        & (96 as i32)
                                                                        != (0 as i32)
                                                                    {
                                                                        break '__join_203;
                                                                    }
                                                                }
                                                            }
                                                            unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_445).offset(
                                                                        *__slate_slot_430 as isize,
                                                                    )
                                                                } = *__slate_slot_429;
                                                            }
                                                        } else {
                                                            if *__slate_slot_437
                                                                == std::ptr::null_mut::<Index>()
                                                            {
                                                                *__slate_slot_833 = (unsafe {
                                                                    sqlite3IsRowid(
                                                                        (unsafe {
                                                                            (*unsafe { unsafe { std::ptr::addr_of_mut!((*pChanges).a) as *mut ExprList_item }.offset(*__slate_slot_429 as isize) }).zEName
                                                                        })
                                                                            as *const i8,
                                                                    )
                                                                }) != (0
                                                                    as i32);
                                                            } else {
                                                                *__slate_slot_833 = false as bool;
                                                            }
                                                            if *__slate_slot_833 {
                                                                *__slate_slot_430 = -(1 as i32);
                                                                *__slate_slot_448 =
                                                                    ((1 as i32) as i8) as u8;
                                                                *__slate_slot_450 = unsafe {
                                                                    (*unsafe { unsafe { std::ptr::addr_of_mut!((*pChanges).a) as *mut ExprList_item }.offset(*__slate_slot_429 as isize) }).pExpr
                                                                };
                                                                *__slate_slot_451 =
                                                                    *__slate_slot_429;
                                                            } else {
                                                                break '__join_208;
                                                            }
                                                        }
                                                        *__slate_slot_480 = unsafe {
                                                            sqlite3AuthCheck(
                                                                pParse,
                                                                23 as i32,
                                                                (unsafe {
                                                                    (*(*__slate_slot_432)).zName
                                                                })
                                                                    as *const i8,
                                                                (if *__slate_slot_430 < (0 as i32) {
                                                                    b"ROWID\0".as_ptr() as *mut i8
                                                                } else {
                                                                    unsafe {
                                                                        (*unsafe { unsafe { (*(*__slate_slot_432)).aCol }.offset(*__slate_slot_430 as isize) }).zCnName
                                                                    }
                                                                })
                                                                    as *const i8,
                                                                (unsafe {
                                                                    (*unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_443))
                                                                                .aDb
                                                                        }
                                                                        .offset(
                                                                            *__slate_slot_454
                                                                                as isize,
                                                                        )
                                                                    })
                                                                    .zDbSName
                                                                })
                                                                    as *const i8,
                                                            )
                                                        };
                                                        if *__slate_slot_480 == (1 as i32) {
                                                            break '__join_0;
                                                        } else {
                                                            if *__slate_slot_480 == (2 as i32) {
                                                                unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_445).offset(
                                                                            *__slate_slot_430
                                                                                as isize,
                                                                        )
                                                                    } = -(1 as i32);
                                                                }
                                                            }
                                                            std::ptr::write(
                                                                __slate_slot_830,
                                                                *__slate_slot_429,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_831,
                                                                *__slate_slot_830 + (1 as i32),
                                                            );
                                                            *__slate_slot_429 = *__slate_slot_831;
                                                        }
                                                    }
                                                } else {
                                                    break;
                                                }
                                            }
                                            '__join_184: {
                                                0 as i32;
                                                0 as i32;
                                                0 as i32;
                                                *__slate_slot_449 = ((((*__slate_slot_448 as u32)
                                                    as i32)
                                                    + ((*__slate_slot_447 as u32) as i32))
                                                    as i8)
                                                    as u8;
                                                // /* Mark generated columns as changing if their generator expressions
                                                //   ** reference any changing column.  The actual aXRef[] value for
                                                //   ** generated expressions is not used, other than to check to see that it
                                                //   ** is non-negative, so the value of aXRef[] for generated columns can be
                                                //   ** set to any non-negative number.  We use 99999 so that the value is
                                                //   ** obvious when looking at aXRef[] in a symbolic debugger.
                                                //   */
                                                if (unsafe { (*(*__slate_slot_432)).tabFlags })
                                                    & ((96 as i32) as u32)
                                                    != (0 as u32)
                                                {
                                                    {}
                                                    {}
                                                    loop {
                                                        *__slate_slot_481 = 0 as i32;
                                                        *__slate_slot_429 = 0 as i32;
                                                        loop {
                                                            if *__slate_slot_429
                                                                < ((unsafe {
                                                                    (*(*__slate_slot_432)).nCol
                                                                })
                                                                    as i32)
                                                            {
                                                                if (unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_445).offset(
                                                                            *__slate_slot_429
                                                                                as isize,
                                                                        )
                                                                    }
                                                                }) >= (0 as i32)
                                                                {
                                                                } else {
                                                                    if (((unsafe {
                                                                        (*unsafe { unsafe { (*(*__slate_slot_432)).aCol }.offset(*__slate_slot_429 as isize) }).colFlags
                                                                    })
                                                                        as u32)
                                                                        as i32)
                                                                        & (96 as i32)
                                                                        == (0 as i32)
                                                                    {
                                                                    } else {
                                                                        if (unsafe {
                                                                            sqlite3ExprReferencesUpdatedColumn(unsafe { sqlite3ColumnExpr(*__slate_slot_432, unsafe { unsafe { (*(*__slate_slot_432)).aCol }.offset(*__slate_slot_429 as isize) }) }, *__slate_slot_445, (*__slate_slot_448 as u32) as i32)
                                                                        }) != (0 as i32)
                                                                        {
                                                                            unsafe {
                                                                                *unsafe {
                                                                                    (*__slate_slot_445).offset(*__slate_slot_429 as isize)
                                                                                } = 99999 as i32;
                                                                            }
                                                                            *__slate_slot_481 =
                                                                                1 as i32;
                                                                        }
                                                                    }
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_834,
                                                                    *__slate_slot_429,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_835,
                                                                    *__slate_slot_834 + (1 as i32),
                                                                );
                                                                *__slate_slot_429 =
                                                                    *__slate_slot_835;
                                                            } else {
                                                                break;
                                                            }
                                                        }
                                                        if !(*__slate_slot_481 != (0 as i32)) {
                                                            break '__join_184;
                                                        }
                                                    }
                                                }
                                            }
                                            // /* The SET expressions are not actually used inside the WHERE loop.
                                            //   ** So reset the colUsed mask. Unless this is a virtual table. In that
                                            //   ** case, set all bits of the colUsed mask (to ensure that the virtual
                                            //   ** table implementation makes all columns available).
                                            //   */
                                            unsafe {
                                                (*unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of_mut!((*pTabList).a)
                                                            as *mut SrcItem
                                                    }
                                                    .offset((0 as i32) as isize)
                                                })
                                                .colUsed = if (((unsafe {
                                                    (*(*__slate_slot_432)).eTabType
                                                })
                                                    as u32)
                                                    as i32)
                                                    == (1 as i32)
                                                {
                                                    (-(1 as i32) as i64) as u64
                                                } else {
                                                    ((0 as i32) as i64) as u64
                                                };
                                            }
                                            *__slate_slot_456 = unsafe {
                                                sqlite3FkRequired(
                                                    pParse,
                                                    *__slate_slot_432,
                                                    *__slate_slot_445,
                                                    (*__slate_slot_449 as u32) as i32,
                                                )
                                            };
                                            // /* There is one entry in the aRegIdx[] array for each index on the table
                                            //   ** being updated.  Fill in aRegIdx[] with a register number that will hold
                                            //   ** the key for accessing each index.
                                            //   */
                                            if onError == (5 as i32) {
                                                *__slate_slot_470 = 1 as i32;
                                            }
                                            *__slate_slot_439 = 0 as i32;
                                            std::ptr::write(__slate_slot_836, unsafe {
                                                (*(*__slate_slot_432)).pIndex
                                            });
                                            *__slate_slot_436 = *__slate_slot_836;
                                            loop {
                                                if *__slate_slot_436
                                                    != std::ptr::null_mut::<Index>()
                                                {
                                                    '__join_178: {
                                                        if *__slate_slot_449 != (0 as u8)
                                                            || *__slate_slot_456 > (1 as i32)
                                                            || *__slate_slot_436
                                                                == *__slate_slot_437
                                                        {
                                                            *__slate_slot_839 = true as bool;
                                                        } else {
                                                            *__slate_slot_839 =
                                                                indexWhereClauseMightChange(
                                                                    *__slate_slot_436,
                                                                    *__slate_slot_445,
                                                                    (*__slate_slot_448 as u32)
                                                                        as i32,
                                                                ) != (0 as i32);
                                                        }
                                                    }
                                                    '__join_170: {
                                                        if *__slate_slot_839 {
                                                            std::ptr::write(
                                                                __slate_slot_840,
                                                                pParse,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_841,
                                                                unsafe {
                                                                    (*(*__slate_slot_840)).nMem
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_842,
                                                                *__slate_slot_841 + (1 as i32),
                                                            );
                                                            unsafe {
                                                                (*(*__slate_slot_840)).nMem =
                                                                    *__slate_slot_842;
                                                            }
                                                            *__slate_slot_482 = *__slate_slot_842;
                                                            std::ptr::write(
                                                                __slate_slot_843,
                                                                pParse,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_844,
                                                                unsafe {
                                                                    (*(*__slate_slot_843)).nMem
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_845,
                                                                *__slate_slot_844
                                                                    + (((unsafe {
                                                                        (*(*__slate_slot_436))
                                                                            .nColumn
                                                                    })
                                                                        as u32)
                                                                        as i32),
                                                            );
                                                            unsafe {
                                                                (*(*__slate_slot_843)).nMem =
                                                                    *__slate_slot_845;
                                                            }
                                                        } else {
                                                            *__slate_slot_482 = 0 as i32;
                                                            *__slate_slot_429 = 0 as i32;
                                                            loop {
                                                                if *__slate_slot_429
                                                                    < (((unsafe {
                                                                        (*(*__slate_slot_436))
                                                                            .nKeyCol
                                                                    })
                                                                        as u32)
                                                                        as i32)
                                                                {
                                                                    if indexColumnIsBeingUpdated(
                                                                        *__slate_slot_436,
                                                                        *__slate_slot_429,
                                                                        *__slate_slot_445,
                                                                        (*__slate_slot_448 as u32)
                                                                            as i32,
                                                                    ) != (0 as i32)
                                                                    {
                                                                        break;
                                                                    } else {
                                                                        std::ptr::write(
                                                                            __slate_slot_846,
                                                                            *__slate_slot_429,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_847,
                                                                            *__slate_slot_846
                                                                                + (1 as i32),
                                                                        );
                                                                        *__slate_slot_429 =
                                                                            *__slate_slot_847;
                                                                    }
                                                                } else {
                                                                    break '__join_170;
                                                                }
                                                            }
                                                            std::ptr::write(
                                                                __slate_slot_848,
                                                                pParse,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_849,
                                                                unsafe {
                                                                    (*(*__slate_slot_848)).nMem
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_850,
                                                                *__slate_slot_849 + (1 as i32),
                                                            );
                                                            unsafe {
                                                                (*(*__slate_slot_848)).nMem =
                                                                    *__slate_slot_850;
                                                            }
                                                            *__slate_slot_482 = *__slate_slot_850;
                                                            std::ptr::write(
                                                                __slate_slot_851,
                                                                pParse,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_852,
                                                                unsafe {
                                                                    (*(*__slate_slot_851)).nMem
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_853,
                                                                *__slate_slot_852
                                                                    + (((unsafe {
                                                                        (*(*__slate_slot_436))
                                                                            .nColumn
                                                                    })
                                                                        as u32)
                                                                        as i32),
                                                            );
                                                            unsafe {
                                                                (*(*__slate_slot_851)).nMem =
                                                                    *__slate_slot_853;
                                                            }
                                                            if onError == (11 as i32)
                                                                && (((unsafe {
                                                                    (*(*__slate_slot_436)).onError
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    == (5 as i32)
                                                            {
                                                                *__slate_slot_470 = 1 as i32;
                                                            }
                                                        }
                                                    }
                                                    if *__slate_slot_482 == (0 as i32) {
                                                        unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_446).offset(
                                                                    (*__slate_slot_439 + (1 as i32))
                                                                        as isize,
                                                                )
                                                            } = ((0 as i32) as i8) as u8;
                                                        }
                                                    }
                                                    unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_444)
                                                                .offset(*__slate_slot_439 as isize)
                                                        } = *__slate_slot_482;
                                                    }
                                                    *__slate_slot_436 =
                                                        unsafe { (*(*__slate_slot_436)).pNext };
                                                    std::ptr::write(
                                                        __slate_slot_837,
                                                        *__slate_slot_439,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_838,
                                                        *__slate_slot_837 + (1 as i32),
                                                    );
                                                    *__slate_slot_439 = *__slate_slot_838;
                                                } else {
                                                    break;
                                                }
                                            }
                                            // /* Register storing the table record */
                                            std::ptr::write(__slate_slot_854, pParse);
                                            std::ptr::write(__slate_slot_855, unsafe {
                                                (*(*__slate_slot_854)).nMem
                                            });
                                            std::ptr::write(
                                                __slate_slot_856,
                                                *__slate_slot_855 + (1 as i32),
                                            );
                                            unsafe {
                                                (*(*__slate_slot_854)).nMem = *__slate_slot_856;
                                            }
                                            unsafe {
                                                *unsafe {
                                                    (*__slate_slot_444)
                                                        .offset(*__slate_slot_439 as isize)
                                                } = *__slate_slot_856;
                                            }
                                            if *__slate_slot_470 != (0 as i32) {
                                                // /* If REPLACE conflict resolution might be invoked, open cursors on all
                                                //     ** indexes in case they are needed to delete records.  */
                                                unsafe {
                                                    memset(
                                                        *__slate_slot_446 as *mut (),
                                                        1 as i32,
                                                        ((*__slate_slot_438 + (1 as i32)) as i64)
                                                            as u64,
                                                    )
                                                };
                                            }
                                            if (((unsafe { (*pParse).nested }) as u32) as i32)
                                                == (0 as i32)
                                            {
                                                unsafe {
                                                    sqlite3VdbeCountChanges(*__slate_slot_435)
                                                };
                                            }
                                            unsafe {
                                                sqlite3BeginWriteOperation(
                                                    pParse,
                                                    (*__slate_slot_461
                                                        != std::ptr::null_mut::<Trigger>()
                                                        || *__slate_slot_456 != (0 as i32))
                                                        as i32,
                                                    *__slate_slot_454,
                                                )
                                            };
                                            // /* Allocate required registers. */
                                            if !((((unsafe { (*(*__slate_slot_432)).eTabType })
                                                as u32)
                                                as i32)
                                                == (1 as i32))
                                            {
                                                // /* For now, regRowSet and aRegIdx[nAllIdx] share the same register.
                                                //     ** If regRowSet turns out to be needed, then aRegIdx[nAllIdx] will be
                                                //     ** reallocated.  aRegIdx[nAllIdx] is the register in which the main
                                                //     ** table record is written.  regRowSet holds the RowSet for the
                                                //     ** two-pass update algorithm. */
                                                0 as i32;
                                                *__slate_slot_478 = unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_444)
                                                            .offset(*__slate_slot_439 as isize)
                                                    }
                                                };
                                                std::ptr::write(__slate_slot_857, pParse);
                                                std::ptr::write(__slate_slot_858, unsafe {
                                                    (*(*__slate_slot_857)).nMem
                                                });
                                                std::ptr::write(
                                                    __slate_slot_859,
                                                    *__slate_slot_858 + (1 as i32),
                                                );
                                                unsafe {
                                                    (*(*__slate_slot_857)).nMem = *__slate_slot_859;
                                                }
                                                std::ptr::write(
                                                    __slate_slot_860,
                                                    *__slate_slot_859,
                                                );
                                                *__slate_slot_475 = *__slate_slot_860;
                                                *__slate_slot_474 = *__slate_slot_860;
                                                if *__slate_slot_447 != (0 as u8)
                                                    || *__slate_slot_461
                                                        != std::ptr::null_mut::<Trigger>()
                                                    || *__slate_slot_456 != (0 as i32)
                                                {
                                                    *__slate_slot_477 =
                                                        (unsafe { (*pParse).nMem }) + (1 as i32);
                                                    std::ptr::write(__slate_slot_861, pParse);
                                                    std::ptr::write(__slate_slot_862, unsafe {
                                                        (*(*__slate_slot_861)).nMem
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_863,
                                                        *__slate_slot_862
                                                            + ((unsafe {
                                                                (*(*__slate_slot_432)).nCol
                                                            })
                                                                as i32),
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_861)).nMem =
                                                            *__slate_slot_863;
                                                    }
                                                }
                                                if *__slate_slot_449 != (0 as u8)
                                                    || *__slate_slot_461
                                                        != std::ptr::null_mut::<Trigger>()
                                                    || *__slate_slot_456 != (0 as i32)
                                                {
                                                    std::ptr::write(__slate_slot_864, pParse);
                                                    std::ptr::write(__slate_slot_865, unsafe {
                                                        (*(*__slate_slot_864)).nMem
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_866,
                                                        *__slate_slot_865 + (1 as i32),
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_864)).nMem =
                                                            *__slate_slot_866;
                                                    }
                                                    *__slate_slot_475 = *__slate_slot_866;
                                                }
                                                *__slate_slot_476 =
                                                    (unsafe { (*pParse).nMem }) + (1 as i32);
                                                std::ptr::write(__slate_slot_867, pParse);
                                                std::ptr::write(__slate_slot_868, unsafe {
                                                    (*(*__slate_slot_867)).nMem
                                                });
                                                std::ptr::write(
                                                    __slate_slot_869,
                                                    *__slate_slot_868
                                                        + ((unsafe { (*(*__slate_slot_432)).nCol })
                                                            as i32),
                                                );
                                                unsafe {
                                                    (*(*__slate_slot_867)).nMem = *__slate_slot_869;
                                                }
                                            }
                                            // /* Start the view context. */
                                            if *__slate_slot_460 != (0 as i32) {
                                                unsafe {
                                                    sqlite3AuthContextPush(
                                                        pParse,
                                                        std::ptr::addr_of_mut!(*__slate_slot_452),
                                                        (unsafe { (*(*__slate_slot_432)).zName })
                                                            as *const i8,
                                                    )
                                                };
                                            }
                                            // /* If we are trying to update a view, realize that view into
                                            //   ** an ephemeral table.
                                            //   */
                                            if *__slate_slot_472 == (0 as i32)
                                                && *__slate_slot_460 != (0 as i32)
                                            {
                                                unsafe {
                                                    sqlite3MaterializeView(
                                                        pParse,
                                                        *__slate_slot_432,
                                                        pWhere,
                                                        pOrderBy,
                                                        pLimit,
                                                        *__slate_slot_441,
                                                    )
                                                };
                                                pOrderBy = std::ptr::null_mut::<ExprList>();
                                                pLimit = std::ptr::null_mut::<Expr>();
                                            }
                                            // /* Resolve the column names in all the expressions in the
                                            //   ** WHERE clause.
                                            //   */
                                            if *__slate_slot_472 == (0 as i32) {
                                                *__slate_slot_870 = (unsafe {
                                                    sqlite3ResolveExprNames(
                                                        std::ptr::addr_of_mut!(*__slate_slot_453),
                                                        pWhere,
                                                    )
                                                }) != (0 as i32);
                                            } else {
                                                *__slate_slot_870 = false as bool;
                                            }
                                            if *__slate_slot_870 {
                                                break '__join_0;
                                            } else {
                                                // /* Virtual tables must be handled separately */
                                                if (((unsafe { (*(*__slate_slot_432)).eTabType })
                                                    as u32)
                                                    as i32)
                                                    == (1 as i32)
                                                {
                                                    updateVirtualTable(
                                                        pParse,
                                                        pTabList,
                                                        *__slate_slot_432,
                                                        pChanges,
                                                        *__slate_slot_450,
                                                        *__slate_slot_445,
                                                        pWhere,
                                                        onError,
                                                    );
                                                    break '__join_0;
                                                } else {
                                                    // /* Jump to labelBreak to abandon further processing of this UPDATE */
                                                    std::ptr::write(__slate_slot_871, unsafe {
                                                        sqlite3VdbeMakeLabel(pParse)
                                                    });
                                                    *__slate_slot_457 = *__slate_slot_871;
                                                    *__slate_slot_458 = *__slate_slot_871;
                                                    // /* Not an UPSERT.  Normal processing.  Begin by
                                                    //   ** initialize the count of updated rows */
                                                    if (unsafe { (*(*__slate_slot_443)).flags })
                                                        & (((1 as i32) as i64) as u64)
                                                            << (32 as i32)
                                                        != (((0 as i32) as i64) as u64)
                                                        && !((unsafe { (*pParse).pTriggerTab })
                                                            != std::ptr::null_mut::<Table>())
                                                        && !((unsafe { (*pParse).nested })
                                                            != (0 as u8))
                                                        && !(((unsafe {
                                                            (*pParse)
                                                                .__slate_bits_0
                                                                .__get_bReturning()
                                                        })
                                                            as i32)
                                                            != (0 as i32))
                                                        && pUpsert == std::ptr::null_mut::<Upsert>()
                                                    {
                                                        std::ptr::write(__slate_slot_872, pParse);
                                                        std::ptr::write(__slate_slot_873, unsafe {
                                                            (*(*__slate_slot_872)).nMem
                                                        });
                                                        std::ptr::write(
                                                            __slate_slot_874,
                                                            *__slate_slot_873 + (1 as i32),
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_872)).nMem =
                                                                *__slate_slot_874;
                                                        }
                                                        *__slate_slot_473 = *__slate_slot_874;
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_435,
                                                                73 as i32,
                                                                0 as i32,
                                                                *__slate_slot_473,
                                                            )
                                                        };
                                                    }
                                                    if *__slate_slot_472 == (0 as i32)
                                                        && (unsafe {
                                                            (*(*__slate_slot_432)).tabFlags
                                                        }) & ((128 as i32) as u32)
                                                            == ((0 as i32) as u32)
                                                    {
                                                        unsafe {
                                                            sqlite3VdbeAddOp3(
                                                                *__slate_slot_435,
                                                                77 as i32,
                                                                0 as i32,
                                                                *__slate_slot_478,
                                                                *__slate_slot_474,
                                                            )
                                                        };
                                                        std::ptr::write(__slate_slot_875, pParse);
                                                        std::ptr::write(__slate_slot_876, unsafe {
                                                            (*(*__slate_slot_875)).nTab
                                                        });
                                                        std::ptr::write(
                                                            __slate_slot_877,
                                                            *__slate_slot_876 + (1 as i32),
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_875)).nTab =
                                                                *__slate_slot_877;
                                                        }
                                                        *__slate_slot_464 = *__slate_slot_876;
                                                        *__slate_slot_467 = unsafe {
                                                            sqlite3VdbeAddOp3(
                                                                *__slate_slot_435,
                                                                120 as i32,
                                                                *__slate_slot_464,
                                                                0 as i32,
                                                                *__slate_slot_478,
                                                            )
                                                        };
                                                    } else {
                                                        0 as i32;
                                                        *__slate_slot_469 = (if *__slate_slot_437
                                                            != std::ptr::null_mut::<Index>()
                                                        {
                                                            ((unsafe {
                                                                (*(*__slate_slot_437)).nKeyCol
                                                            })
                                                                as u32)
                                                                as i32
                                                        } else {
                                                            0 as i32
                                                        })
                                                            as i16;
                                                        *__slate_slot_468 =
                                                            (unsafe { (*pParse).nMem })
                                                                + (1 as i32);
                                                        std::ptr::write(__slate_slot_878, pParse);
                                                        std::ptr::write(__slate_slot_879, unsafe {
                                                            (*(*__slate_slot_878)).nMem
                                                        });
                                                        std::ptr::write(
                                                            __slate_slot_880,
                                                            *__slate_slot_879
                                                                + (*__slate_slot_469 as i32),
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_878)).nMem =
                                                                *__slate_slot_880;
                                                        }
                                                        std::ptr::write(__slate_slot_881, pParse);
                                                        std::ptr::write(__slate_slot_882, unsafe {
                                                            (*(*__slate_slot_881)).nMem
                                                        });
                                                        std::ptr::write(
                                                            __slate_slot_883,
                                                            *__slate_slot_882 + *__slate_slot_472,
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_881)).nMem =
                                                                *__slate_slot_883;
                                                        }
                                                        std::ptr::write(__slate_slot_884, pParse);
                                                        std::ptr::write(__slate_slot_885, unsafe {
                                                            (*(*__slate_slot_884)).nMem
                                                        });
                                                        std::ptr::write(
                                                            __slate_slot_886,
                                                            *__slate_slot_885 + (1 as i32),
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_884)).nMem =
                                                                *__slate_slot_886;
                                                        }
                                                        *__slate_slot_479 = *__slate_slot_886;
                                                        if pUpsert == std::ptr::null_mut::<Upsert>()
                                                        {
                                                            std::ptr::write(
                                                                __slate_slot_483,
                                                                (*__slate_slot_469 as i32)
                                                                    + *__slate_slot_472
                                                                    + if *__slate_slot_460
                                                                        != (0 as i32)
                                                                    {
                                                                        (unsafe {
                                                                            (*(*__slate_slot_432))
                                                                                .nCol
                                                                        })
                                                                            as i32
                                                                    } else {
                                                                        0 as i32
                                                                    },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_887,
                                                                pParse,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_888,
                                                                unsafe {
                                                                    (*(*__slate_slot_887)).nTab
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_889,
                                                                *__slate_slot_888 + (1 as i32),
                                                            );
                                                            unsafe {
                                                                (*(*__slate_slot_887)).nTab =
                                                                    *__slate_slot_889;
                                                            }
                                                            *__slate_slot_464 = *__slate_slot_888;
                                                            if *__slate_slot_437
                                                                != std::ptr::null_mut::<Index>()
                                                            {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp3(
                                                                        *__slate_slot_435,
                                                                        77 as i32,
                                                                        0 as i32,
                                                                        *__slate_slot_468,
                                                                        *__slate_slot_468
                                                                            + (*__slate_slot_469
                                                                                as i32)
                                                                            - (1 as i32),
                                                                    )
                                                                };
                                                            }
                                                            *__slate_slot_467 = unsafe {
                                                                sqlite3VdbeAddOp2(
                                                                    *__slate_slot_435,
                                                                    120 as i32,
                                                                    *__slate_slot_464,
                                                                    *__slate_slot_483,
                                                                )
                                                            };
                                                            if *__slate_slot_437
                                                                != std::ptr::null_mut::<Index>()
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_484,
                                                                    unsafe {
                                                                        sqlite3KeyInfoOfIndex(
                                                                            pParse,
                                                                            *__slate_slot_437,
                                                                        )
                                                                    },
                                                                );
                                                                if *__slate_slot_484
                                                                    != std::ptr::null_mut::<KeyInfo>(
                                                                    )
                                                                {
                                                                    unsafe {
                                                                        (*(*__slate_slot_484))
                                                                            .nAllField =
                                                                            (*__slate_slot_483
                                                                                as i16)
                                                                                as u16;
                                                                    }
                                                                    unsafe {
                                                                        sqlite3VdbeAppendP4(
                                                                            *__slate_slot_435,
                                                                            *__slate_slot_484
                                                                                as *mut (),
                                                                            -(9 as i32),
                                                                        )
                                                                    };
                                                                }
                                                            }
                                                            if *__slate_slot_472 != (0 as i32) {
                                                                updateFromSelect(
                                                                    pParse,
                                                                    *__slate_slot_464,
                                                                    *__slate_slot_437,
                                                                    pChanges,
                                                                    pTabList,
                                                                    pWhere,
                                                                    pOrderBy,
                                                                    pLimit,
                                                                );
                                                                if *__slate_slot_460 != (0 as i32) {
                                                                    *__slate_slot_441 =
                                                                        *__slate_slot_464;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    if *__slate_slot_472 != (0 as i32) {
                                                        unsafe { sqlite3MultiWrite(pParse) };
                                                        *__slate_slot_455 = 0 as i32;
                                                        *__slate_slot_465 =
                                                            *__slate_slot_469 as i32;
                                                        *__slate_slot_479 = *__slate_slot_468;
                                                    } else {
                                                        if pUpsert != std::ptr::null_mut::<Upsert>()
                                                        {
                                                            // /* If this is an UPSERT, then all cursors have already been opened by
                                                            //       ** the outer INSERT and the data cursor should be pointing at the row
                                                            //       ** that is to be updated.  So bypass the code that searches for the
                                                            //       ** row(s) to be updated.
                                                            //       */
                                                            *__slate_slot_434 =
                                                                std::ptr::null_mut::<WhereInfo>();
                                                            *__slate_slot_455 = 1 as i32;
                                                            unsafe {
                                                                sqlite3ExprIfFalse(
                                                                    pParse,
                                                                    pWhere,
                                                                    *__slate_slot_457,
                                                                    16 as i32,
                                                                )
                                                            };
                                                            *__slate_slot_471 = 0 as i32;
                                                        } else {
                                                            // /* Begin the database scan.
                                                            //       **
                                                            //       ** Do not consider a single-pass strategy for a multi-row update if
                                                            //       ** there is anything that might disrupt the cursor being used to do
                                                            //       ** the UPDATE:
                                                            //       **   (1) This is a nested UPDATE
                                                            //       **   (2) There are triggers
                                                            //       **   (3) There are FOREIGN KEY constraints
                                                            //       **   (4) There are REPLACE conflict handlers
                                                            //       **   (5) There are subqueries in the WHERE clause
                                                            //       */
                                                            *__slate_slot_459 = 4 as i32;
                                                            if !((unsafe { (*pParse).nested })
                                                                != (0 as u8))
                                                                && !(*__slate_slot_461
                                                                    != std::ptr::null_mut::<Trigger>(
                                                                    ))
                                                                && !(*__slate_slot_456
                                                                    != (0 as i32))
                                                                && !(*__slate_slot_449 != (0 as u8))
                                                                && !(*__slate_slot_470
                                                                    != (0 as i32))
                                                                && (pWhere
                                                                    == std::ptr::null_mut::<Expr>()
                                                                    || !((unsafe {
                                                                        (*pWhere).flags
                                                                    }) & ((4194304 as i32)
                                                                        as u32)
                                                                        != ((0 as i32) as u32)))
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_890,
                                                                    *__slate_slot_459,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_891,
                                                                    *__slate_slot_890 | (8 as i32),
                                                                );
                                                                *__slate_slot_459 =
                                                                    *__slate_slot_891;
                                                            }
                                                            *__slate_slot_434 = unsafe {
                                                                sqlite3WhereBegin(
                                                                    pParse,
                                                                    pTabList,
                                                                    pWhere,
                                                                    std::ptr::null_mut::<ExprList>(
                                                                    ),
                                                                    std::ptr::null_mut::<ExprList>(
                                                                    ),
                                                                    std::ptr::null_mut::<Select>(),
                                                                    (*__slate_slot_459 as i16)
                                                                        as u16,
                                                                    *__slate_slot_442,
                                                                )
                                                            };
                                                            if *__slate_slot_434
                                                                == std::ptr::null_mut::<WhereInfo>()
                                                            {
                                                                break '__join_0;
                                                            } else {
                                                                // /* A one-pass strategy that might update more than one row may not
                                                                //       ** be used if any column of the index used for the scan is being
                                                                //       ** updated. Otherwise, if there is an index on "b", statements like
                                                                //       ** the following could create an infinite loop:
                                                                //       **
                                                                //       **   UPDATE t1 SET b=b+1 WHERE b>?
                                                                //       **
                                                                //       ** Fall back to ONEPASS_OFF if where.c has selected a ONEPASS_MULTI
                                                                //       ** strategy that uses an index for which one or more columns are being
                                                                //       ** updated.  */
                                                                *__slate_slot_455 = unsafe {
                                                                    sqlite3WhereOkOnePass(
                                                                        *__slate_slot_434,
                                                                        (*__slate_slot_466)
                                                                            .as_mut_ptr()
                                                                            as *mut i32,
                                                                    )
                                                                };
                                                                *__slate_slot_471 = unsafe {
                                                                    sqlite3WhereUsesDeferredSeek(
                                                                        *__slate_slot_434,
                                                                    )
                                                                };
                                                                if *__slate_slot_455 != (1 as i32) {
                                                                    unsafe {
                                                                        sqlite3MultiWrite(pParse)
                                                                    };
                                                                    if *__slate_slot_455
                                                                        == (2 as i32)
                                                                    {
                                                                        std::ptr::write(
                                                                            __slate_slot_485,
                                                                            unsafe {
                                                                                *unsafe {
                                                                                    ((*__slate_slot_466).as_mut_ptr() as *mut i32).offset((1 as i32) as isize)
                                                                                }
                                                                            },
                                                                        );
                                                                        if *__slate_slot_485
                                                                            >= (0 as i32)
                                                                            && *__slate_slot_485
                                                                                != *__slate_slot_441
                                                                            && (unsafe {
                                                                                *unsafe {
                                                                                    (*__slate_slot_446).offset((*__slate_slot_485 - *__slate_slot_440) as isize)
                                                                                }
                                                                            }) != (0 as u8)
                                                                        {
                                                                            *__slate_slot_455 =
                                                                                0 as i32;
                                                                        }
                                                                        0 as i32;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        if (unsafe {
                                                            (*(*__slate_slot_432)).tabFlags
                                                        }) & ((128 as i32) as u32)
                                                            == ((0 as i32) as u32)
                                                        {
                                                            // /* Read the rowid of the current row of the WHERE scan. In ONEPASS_OFF
                                                            //       ** mode, write the rowid into the FIFO. In either of the one-pass modes,
                                                            //       ** leave it in register regOldRowid.  */
                                                            unsafe {
                                                                sqlite3VdbeAddOp2(
                                                                    *__slate_slot_435,
                                                                    137 as i32,
                                                                    *__slate_slot_441,
                                                                    *__slate_slot_474,
                                                                )
                                                            };
                                                            if *__slate_slot_455 == (0 as i32) {
                                                                std::ptr::write(
                                                                    __slate_slot_892,
                                                                    pParse,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_893,
                                                                    unsafe {
                                                                        (*(*__slate_slot_892)).nMem
                                                                    },
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_894,
                                                                    *__slate_slot_893 + (1 as i32),
                                                                );
                                                                unsafe {
                                                                    (*(*__slate_slot_892)).nMem =
                                                                        *__slate_slot_894;
                                                                }
                                                                unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_444).offset(
                                                                            *__slate_slot_439
                                                                                as isize,
                                                                        )
                                                                    } = *__slate_slot_894;
                                                                }
                                                                unsafe {
                                                                    sqlite3VdbeAddOp3(
                                                                        *__slate_slot_435,
                                                                        130 as i32,
                                                                        *__slate_slot_464,
                                                                        *__slate_slot_478,
                                                                        *__slate_slot_474,
                                                                    )
                                                                };
                                                            } else {
                                                                if *__slate_slot_467 != (0 as i32) {
                                                                    unsafe {
                                                                        sqlite3VdbeChangeToNoop(
                                                                            *__slate_slot_435,
                                                                            *__slate_slot_467,
                                                                        )
                                                                    };
                                                                }
                                                            }
                                                        } else {
                                                            // /* Read the PK of the current row into an array of registers. In
                                                            //       ** ONEPASS_OFF mode, serialize the array into a record and store it in
                                                            //       ** the ephemeral table. Or, in ONEPASS_SINGLE or MULTI mode, change
                                                            //       ** the OP_OpenEphemeral instruction to a Noop (the ephemeral table
                                                            //       ** is not required) and leave the PK fields in the array of registers.  */
                                                            *__slate_slot_429 = 0 as i32;
                                                            loop {
                                                                if *__slate_slot_429
                                                                    < (*__slate_slot_469 as i32)
                                                                {
                                                                    0 as i32;
                                                                    unsafe {
                                                                        sqlite3ExprCodeGetColumnOfTable(*__slate_slot_435, *__slate_slot_432, *__slate_slot_441, (unsafe { *unsafe { unsafe { (*(*__slate_slot_437)).aiColumn }.offset(*__slate_slot_429 as isize) } }) as i32, *__slate_slot_468 + *__slate_slot_429)
                                                                    };
                                                                    std::ptr::write(
                                                                        __slate_slot_895,
                                                                        *__slate_slot_429,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_896,
                                                                        *__slate_slot_895
                                                                            + (1 as i32),
                                                                    );
                                                                    *__slate_slot_429 =
                                                                        *__slate_slot_896;
                                                                } else {
                                                                    break;
                                                                }
                                                            }
                                                            if *__slate_slot_455 != (0 as i32) {
                                                                if *__slate_slot_467 != (0 as i32) {
                                                                    unsafe {
                                                                        sqlite3VdbeChangeToNoop(
                                                                            *__slate_slot_435,
                                                                            *__slate_slot_467,
                                                                        )
                                                                    };
                                                                }
                                                                *__slate_slot_465 =
                                                                    *__slate_slot_469 as i32;
                                                                *__slate_slot_479 =
                                                                    *__slate_slot_468;
                                                            } else {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp4(
                                                                        *__slate_slot_435,
                                                                        99 as i32,
                                                                        *__slate_slot_468,
                                                                        *__slate_slot_469 as i32,
                                                                        *__slate_slot_479,
                                                                        unsafe {
                                                                            sqlite3IndexAffinityStr(
                                                                                *__slate_slot_443,
                                                                                *__slate_slot_437,
                                                                            )
                                                                        },
                                                                        *__slate_slot_469 as i32,
                                                                    )
                                                                };
                                                                unsafe {
                                                                    sqlite3VdbeAddOp4Int(
                                                                        *__slate_slot_435,
                                                                        140 as i32,
                                                                        *__slate_slot_464,
                                                                        *__slate_slot_479,
                                                                        *__slate_slot_468,
                                                                        *__slate_slot_469 as i32,
                                                                    )
                                                                };
                                                            }
                                                        }
                                                    }
                                                    if pUpsert == std::ptr::null_mut::<Upsert>() {
                                                        if *__slate_slot_472 == (0 as i32)
                                                            && *__slate_slot_455 != (2 as i32)
                                                        {
                                                            unsafe {
                                                                sqlite3WhereEnd(*__slate_slot_434)
                                                            };
                                                        }
                                                        if !(*__slate_slot_460 != (0 as i32)) {
                                                            std::ptr::write(
                                                                __slate_slot_486,
                                                                0 as i32,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_487,
                                                                0 as i32,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_488,
                                                                0 as i32,
                                                            );
                                                            // /* Open every index that needs updating. */
                                                            if *__slate_slot_455 != (0 as i32) {
                                                                if (unsafe {
                                                                    *unsafe {
                                                                        ((*__slate_slot_466)
                                                                            .as_mut_ptr()
                                                                            as *mut i32)
                                                                            .offset(
                                                                                (0 as i32) as isize,
                                                                            )
                                                                    }
                                                                }) >= (0 as i32)
                                                                {
                                                                    unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_446).offset(((unsafe { *unsafe { ((*__slate_slot_466).as_mut_ptr() as *mut i32).offset((0 as i32) as isize) } }) - *__slate_slot_440) as isize)
                                                                        } = ((0 as i32) as i8)
                                                                            as u8;
                                                                    }
                                                                }
                                                                if (unsafe {
                                                                    *unsafe {
                                                                        ((*__slate_slot_466)
                                                                            .as_mut_ptr()
                                                                            as *mut i32)
                                                                            .offset(
                                                                                (1 as i32) as isize,
                                                                            )
                                                                    }
                                                                }) >= (0 as i32)
                                                                {
                                                                    unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_446).offset(((unsafe { *unsafe { ((*__slate_slot_466).as_mut_ptr() as *mut i32).offset((1 as i32) as isize) } }) - *__slate_slot_440) as isize)
                                                                        } = ((0 as i32) as i8)
                                                                            as u8;
                                                                    }
                                                                }
                                                            }
                                                            if *__slate_slot_455 == (2 as i32)
                                                                && *__slate_slot_438
                                                                    - (((unsafe {
                                                                        *unsafe {
                                                                            ((*__slate_slot_466)
                                                                                .as_mut_ptr()
                                                                                as *mut i32)
                                                                                .offset(
                                                                                    (1 as i32)
                                                                                        as isize,
                                                                                )
                                                                        }
                                                                    }) >= (0 as i32))
                                                                        as i32)
                                                                    > (0 as i32)
                                                            {
                                                                *__slate_slot_486 = unsafe {
                                                                    sqlite3VdbeAddOp0(
                                                                        *__slate_slot_435,
                                                                        15 as i32,
                                                                    )
                                                                };
                                                                {}
                                                            }
                                                            unsafe {
                                                                sqlite3OpenTableAndIndices(
                                                                    pParse,
                                                                    *__slate_slot_432,
                                                                    116 as i32,
                                                                    ((0 as i32) as i8) as u8,
                                                                    *__slate_slot_440,
                                                                    *__slate_slot_446,
                                                                    std::ptr::addr_of_mut!(
                                                                        *__slate_slot_487
                                                                    ),
                                                                    std::ptr::addr_of_mut!(
                                                                        *__slate_slot_488
                                                                    ),
                                                                )
                                                            };
                                                            if *__slate_slot_486 != (0 as i32) {
                                                                unsafe {
                                                                    sqlite3VdbeJumpHereOrPopInst(
                                                                        *__slate_slot_435,
                                                                        *__slate_slot_486,
                                                                    )
                                                                };
                                                            }
                                                        }
                                                        // /* Top of the update loop */
                                                        if *__slate_slot_455 != (0 as i32) {
                                                            if (unsafe {
                                                                *unsafe {
                                                                    ((*__slate_slot_466)
                                                                        .as_mut_ptr()
                                                                        as *mut i32)
                                                                        .offset((0 as i32) as isize)
                                                                }
                                                            }) != *__slate_slot_441
                                                                && (unsafe {
                                                                    *unsafe {
                                                                        ((*__slate_slot_466)
                                                                            .as_mut_ptr()
                                                                            as *mut i32)
                                                                            .offset(
                                                                                (1 as i32) as isize,
                                                                            )
                                                                    }
                                                                }) != *__slate_slot_441
                                                            {
                                                                0 as i32;
                                                                unsafe {
                                                                    sqlite3VdbeAddOp4Int(
                                                                        *__slate_slot_435,
                                                                        28 as i32,
                                                                        *__slate_slot_441,
                                                                        *__slate_slot_457,
                                                                        *__slate_slot_479,
                                                                        *__slate_slot_465,
                                                                    )
                                                                };
                                                                {}
                                                            }
                                                            if *__slate_slot_455 != (1 as i32) {
                                                                *__slate_slot_458 = unsafe {
                                                                    sqlite3VdbeMakeLabel(pParse)
                                                                };
                                                            }
                                                            unsafe {
                                                                sqlite3VdbeAddOp2(
                                                                    *__slate_slot_435,
                                                                    51 as i32,
                                                                    if *__slate_slot_437
                                                                        != std::ptr::null_mut::<Index>(
                                                                        )
                                                                    {
                                                                        *__slate_slot_479
                                                                    } else {
                                                                        *__slate_slot_474
                                                                    },
                                                                    *__slate_slot_457,
                                                                )
                                                            };
                                                            {}
                                                            {}
                                                        } else {
                                                            if *__slate_slot_437
                                                                != std::ptr::null_mut::<Index>()
                                                                || *__slate_slot_472 != (0 as i32)
                                                            {
                                                                *__slate_slot_458 = unsafe {
                                                                    sqlite3VdbeMakeLabel(pParse)
                                                                };
                                                                unsafe {
                                                                    sqlite3VdbeAddOp2(
                                                                        *__slate_slot_435,
                                                                        36 as i32,
                                                                        *__slate_slot_464,
                                                                        *__slate_slot_457,
                                                                    )
                                                                };
                                                                {}
                                                                *__slate_slot_433 = unsafe {
                                                                    sqlite3VdbeCurrentAddr(
                                                                        *__slate_slot_435,
                                                                    )
                                                                };
                                                                if *__slate_slot_472 != (0 as i32) {
                                                                    if !(*__slate_slot_460
                                                                        != (0 as i32))
                                                                    {
                                                                        if *__slate_slot_437
                                                                            != std::ptr::null_mut::<
                                                                                Index,
                                                                            >(
                                                                            )
                                                                        {
                                                                            *__slate_slot_429 =
                                                                                0 as i32;
                                                                            loop {
                                                                                if *__slate_slot_429 < (*__slate_slot_469 as i32) {
unsafe { sqlite3VdbeAddOp3(*__slate_slot_435, 96 as i32, *__slate_slot_464, *__slate_slot_429, *__slate_slot_468 + *__slate_slot_429) };
std::ptr::write(__slate_slot_897, *__slate_slot_429);
std::ptr::write(__slate_slot_898, *__slate_slot_897 + (1 as i32));
*__slate_slot_429 = *__slate_slot_898;
} else {
break;
}
                                                                            }
                                                                            unsafe {
                                                                                sqlite3VdbeAddOp4Int(*__slate_slot_435, 28 as i32, *__slate_slot_441, *__slate_slot_458, *__slate_slot_468, *__slate_slot_469 as i32)
                                                                            };
                                                                            {}
                                                                        } else {
                                                                            unsafe {
                                                                                sqlite3VdbeAddOp2(*__slate_slot_435, 137 as i32, *__slate_slot_464, *__slate_slot_474)
                                                                            };
                                                                            unsafe {
                                                                                sqlite3VdbeAddOp3(*__slate_slot_435, 31 as i32, *__slate_slot_441, *__slate_slot_458, *__slate_slot_474)
                                                                            };
                                                                            {}
                                                                        }
                                                                    }
                                                                } else {
                                                                    unsafe {
                                                                        sqlite3VdbeAddOp2(
                                                                            *__slate_slot_435,
                                                                            136 as i32,
                                                                            *__slate_slot_464,
                                                                            *__slate_slot_479,
                                                                        )
                                                                    };
                                                                    unsafe {
                                                                        sqlite3VdbeAddOp4Int(
                                                                            *__slate_slot_435,
                                                                            28 as i32,
                                                                            *__slate_slot_441,
                                                                            *__slate_slot_458,
                                                                            *__slate_slot_479,
                                                                            0 as i32,
                                                                        )
                                                                    };
                                                                    {}
                                                                }
                                                            } else {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp2(
                                                                        *__slate_slot_435,
                                                                        36 as i32,
                                                                        *__slate_slot_464,
                                                                        *__slate_slot_457,
                                                                    )
                                                                };
                                                                {}
                                                                *__slate_slot_458 = unsafe {
                                                                    sqlite3VdbeMakeLabel(pParse)
                                                                };
                                                                *__slate_slot_433 = unsafe {
                                                                    sqlite3VdbeAddOp2(
                                                                        *__slate_slot_435,
                                                                        137 as i32,
                                                                        *__slate_slot_464,
                                                                        *__slate_slot_474,
                                                                    )
                                                                };
                                                                {}
                                                                unsafe {
                                                                    sqlite3VdbeAddOp3(
                                                                        *__slate_slot_435,
                                                                        31 as i32,
                                                                        *__slate_slot_441,
                                                                        *__slate_slot_458,
                                                                        *__slate_slot_474,
                                                                    )
                                                                };
                                                                {}
                                                            }
                                                        }
                                                    }
                                                    // /* If the rowid value will change, set register regNewRowid to
                                                    //   ** contain the new value. If the rowid is not being modified,
                                                    //   ** then regNewRowid is the same register as regOldRowid, which is
                                                    //   ** already populated.  */
                                                    0 as i32;
                                                    if *__slate_slot_448 != (0 as u8) {
                                                        0 as i32;
                                                        if *__slate_slot_472 == (0 as i32) {
                                                            unsafe {
                                                                sqlite3ExprCode(
                                                                    pParse,
                                                                    *__slate_slot_450,
                                                                    *__slate_slot_475,
                                                                )
                                                            };
                                                        } else {
                                                            unsafe {
                                                                sqlite3VdbeAddOp3(
                                                                    *__slate_slot_435,
                                                                    96 as i32,
                                                                    *__slate_slot_464,
                                                                    *__slate_slot_451,
                                                                    *__slate_slot_475,
                                                                )
                                                            };
                                                        }
                                                        unsafe {
                                                            sqlite3VdbeAddOp1(
                                                                *__slate_slot_435,
                                                                13 as i32,
                                                                *__slate_slot_475,
                                                            )
                                                        };
                                                        {}
                                                    }
                                                    // /* Compute the old pre-UPDATE content of the row being changed, if that
                                                    //   ** information is needed */
                                                    if *__slate_slot_447 != (0 as u8)
                                                        || *__slate_slot_456 != (0 as i32)
                                                        || *__slate_slot_461
                                                            != std::ptr::null_mut::<Trigger>()
                                                    {
                                                        if *__slate_slot_456 != (0 as i32) {
                                                            *__slate_slot_899 = unsafe {
                                                                sqlite3FkOldmask(
                                                                    pParse,
                                                                    *__slate_slot_432,
                                                                )
                                                            };
                                                        } else {
                                                            *__slate_slot_899 = (0 as i32) as u32;
                                                        }
                                                        *__slate_slot_489 = *__slate_slot_899;
                                                        std::ptr::write(
                                                            __slate_slot_900,
                                                            *__slate_slot_489,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_901,
                                                            *__slate_slot_900
                                                                | unsafe {
                                                                    sqlite3TriggerColmask(
                                                                        pParse,
                                                                        *__slate_slot_461,
                                                                        pChanges,
                                                                        0 as i32,
                                                                        (1 as i32) | (2 as i32),
                                                                        *__slate_slot_432,
                                                                        onError,
                                                                    )
                                                                },
                                                        );
                                                        *__slate_slot_489 = *__slate_slot_901;
                                                        *__slate_slot_429 = 0 as i32;
                                                        loop {
                                                            if *__slate_slot_429
                                                                < ((unsafe {
                                                                    (*(*__slate_slot_432)).nCol
                                                                })
                                                                    as i32)
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_490,
                                                                    (unsafe {
                                                                        (*unsafe { unsafe { (*(*__slate_slot_432)).aCol }.offset(*__slate_slot_429 as isize) }).colFlags
                                                                    })
                                                                        as u32,
                                                                );
                                                                *__slate_slot_431 = ((unsafe {
                                                                    sqlite3TableColumnToStorage(
                                                                        *__slate_slot_432,
                                                                        *__slate_slot_429 as i16,
                                                                    )
                                                                })
                                                                    as i32)
                                                                    + *__slate_slot_477;
                                                                if *__slate_slot_489
                                                                    == (4294967295 as u32)
                                                                    || *__slate_slot_429
                                                                        < (32 as i32)
                                                                        && *__slate_slot_489
                                                                            & ((1 as i32) as u32)
                                                                                << *__slate_slot_429
                                                                            != ((0 as i32) as u32)
                                                                    || *__slate_slot_490
                                                                        & ((1 as i32) as u32)
                                                                        != ((0 as i32) as u32)
                                                                {
                                                                    {}
                                                                    unsafe {
                                                                        sqlite3ExprCodeGetColumnOfTable(*__slate_slot_435, *__slate_slot_432, *__slate_slot_441, *__slate_slot_429, *__slate_slot_431)
                                                                    };
                                                                } else {
                                                                    unsafe {
                                                                        sqlite3VdbeAddOp2(
                                                                            *__slate_slot_435,
                                                                            77 as i32,
                                                                            0 as i32,
                                                                            *__slate_slot_431,
                                                                        )
                                                                    };
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_902,
                                                                    *__slate_slot_429,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_903,
                                                                    *__slate_slot_902 + (1 as i32),
                                                                );
                                                                *__slate_slot_429 =
                                                                    *__slate_slot_903;
                                                            } else {
                                                                break;
                                                            }
                                                        }
                                                        if ((*__slate_slot_448 as u32) as i32)
                                                            == (0 as i32)
                                                            && *__slate_slot_437
                                                                == std::ptr::null_mut::<Index>()
                                                        {
                                                            unsafe {
                                                                sqlite3VdbeAddOp2(
                                                                    *__slate_slot_435,
                                                                    82 as i32,
                                                                    *__slate_slot_474,
                                                                    *__slate_slot_475,
                                                                )
                                                            };
                                                        }
                                                    }
                                                    // /* Populate the array of registers beginning at regNew with the new
                                                    //   ** row data. This array is used to check constants, create the new
                                                    //   ** table and index records, and as the values for any new.* references
                                                    //   ** made by triggers.
                                                    //   **
                                                    //   ** If there are one or more BEFORE triggers, then do not populate the
                                                    //   ** registers associated with columns that are (a) not modified by
                                                    //   ** this UPDATE statement and (b) not accessed by new.* references. The
                                                    //   ** values for registers not modified by the UPDATE must be reloaded from
                                                    //   ** the database after the BEFORE triggers are fired anyway (as the trigger
                                                    //   ** may have modified them). So not loading those that are not going to
                                                    //   ** be used eliminates some redundant opcodes.
                                                    //   */
                                                    *__slate_slot_463 = (unsafe {
                                                        sqlite3TriggerColmask(
                                                            pParse,
                                                            *__slate_slot_461,
                                                            pChanges,
                                                            1 as i32,
                                                            1 as i32,
                                                            *__slate_slot_432,
                                                            onError,
                                                        )
                                                    })
                                                        as i32;
                                                    *__slate_slot_429 = 0 as i32;
                                                    std::ptr::write(
                                                        __slate_slot_904,
                                                        *__slate_slot_476,
                                                    );
                                                    *__slate_slot_431 = *__slate_slot_904;
                                                    loop {
                                                        if *__slate_slot_429
                                                            < ((unsafe {
                                                                (*(*__slate_slot_432)).nCol
                                                            })
                                                                as i32)
                                                        {
                                                            if *__slate_slot_429
                                                                == ((unsafe {
                                                                    (*(*__slate_slot_432)).iPKey
                                                                })
                                                                    as i32)
                                                            {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp2(
                                                                        *__slate_slot_435,
                                                                        77 as i32,
                                                                        0 as i32,
                                                                        *__slate_slot_431,
                                                                    )
                                                                };
                                                            } else {
                                                                if (((unsafe {
                                                                    (*unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_432))
                                                                                .aCol
                                                                        }
                                                                        .offset(
                                                                            *__slate_slot_429
                                                                                as isize,
                                                                        )
                                                                    })
                                                                    .colFlags
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    & (96 as i32)
                                                                    != (0 as i32)
                                                                {
                                                                    if (((unsafe {
                                                                        (*unsafe { unsafe { (*(*__slate_slot_432)).aCol }.offset(*__slate_slot_429 as isize) }).colFlags
                                                                    })
                                                                        as u32)
                                                                        as i32)
                                                                        & (32 as i32)
                                                                        != (0 as i32)
                                                                    {
                                                                        std::ptr::write(
                                                                            __slate_slot_909,
                                                                            *__slate_slot_431,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_910,
                                                                            *__slate_slot_909
                                                                                - (1 as i32),
                                                                        );
                                                                        *__slate_slot_431 =
                                                                            *__slate_slot_910;
                                                                    }
                                                                } else {
                                                                    *__slate_slot_430 = unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_445)
                                                                                .offset(
                                                                                *__slate_slot_429
                                                                                    as isize,
                                                                            )
                                                                        }
                                                                    };
                                                                    if *__slate_slot_430
                                                                        >= (0 as i32)
                                                                    {
                                                                        if *__slate_slot_472
                                                                            != (0 as i32)
                                                                        {
                                                                            std::ptr::write(
                                                                                __slate_slot_491,
                                                                                if *__slate_slot_460
                                                                                    != (0 as i32)
                                                                                {
                                                                                    (unsafe {
                                                                                        (*(*__slate_slot_432)).nCol
                                                                                    })
                                                                                        as i32
                                                                                } else {
                                                                                    *__slate_slot_469 as i32
                                                                                },
                                                                            );
                                                                            0 as i32;
                                                                            unsafe {
                                                                                sqlite3VdbeAddOp3(*__slate_slot_435, 96 as i32, *__slate_slot_464, *__slate_slot_491 + *__slate_slot_430, *__slate_slot_431)
                                                                            };
                                                                        } else {
                                                                            unsafe {
                                                                                sqlite3ExprCode(pParse, unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*pChanges).a) as *mut ExprList_item }.offset(*__slate_slot_430 as isize) }).pExpr }, *__slate_slot_431)
                                                                            };
                                                                        }
                                                                    } else {
                                                                        if (0 as i32) == *__slate_slot_462 & (1 as i32) || *__slate_slot_429 > (31 as i32) || (*__slate_slot_463 as u32) & ((1 as i32) as u32) << *__slate_slot_429 != (0 as u32) {
// /* This branch loads the value of a column that will not be changed
//         ** into a register. This is done if there are no BEFORE triggers, or
//         ** if there are one or more BEFORE triggers that use this value via
//         ** a new.* reference in a trigger program.
//         */
{
}
{
}
unsafe { sqlite3ExprCodeGetColumnOfTable(*__slate_slot_435, *__slate_slot_432, *__slate_slot_441, *__slate_slot_429, *__slate_slot_431) };
*__slate_slot_471 = 0 as i32;
} else {
unsafe { sqlite3VdbeAddOp2(*__slate_slot_435, 77 as i32, 0 as i32, *__slate_slot_431) };
}
                                                                    }
                                                                }
                                                            }
                                                            std::ptr::write(
                                                                __slate_slot_905,
                                                                *__slate_slot_429,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_906,
                                                                *__slate_slot_905 + (1 as i32),
                                                            );
                                                            *__slate_slot_429 = *__slate_slot_906;
                                                            std::ptr::write(
                                                                __slate_slot_907,
                                                                *__slate_slot_431,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_908,
                                                                *__slate_slot_907 + (1 as i32),
                                                            );
                                                            *__slate_slot_431 = *__slate_slot_908;
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                    if (unsafe { (*(*__slate_slot_432)).tabFlags })
                                                        & ((96 as i32) as u32)
                                                        != (0 as u32)
                                                    {
                                                        {}
                                                        {}
                                                        unsafe {
                                                            sqlite3ComputeGeneratedColumns(
                                                                pParse,
                                                                *__slate_slot_476,
                                                                *__slate_slot_432,
                                                            )
                                                        };
                                                    }
                                                    // /* Fire any BEFORE UPDATE triggers. This happens before constraints are
                                                    //   ** verified. One could argue that this is wrong.
                                                    //   */
                                                    if *__slate_slot_462 & (1 as i32) != (0 as i32)
                                                    {
                                                        unsafe {
                                                            sqlite3TableAffinity(
                                                                *__slate_slot_435,
                                                                *__slate_slot_432,
                                                                *__slate_slot_476,
                                                            )
                                                        };
                                                        unsafe {
                                                            sqlite3CodeRowTrigger(
                                                                pParse,
                                                                *__slate_slot_461,
                                                                130 as i32,
                                                                pChanges,
                                                                1 as i32,
                                                                *__slate_slot_432,
                                                                *__slate_slot_474,
                                                                onError,
                                                                *__slate_slot_458,
                                                            )
                                                        };
                                                        if !(*__slate_slot_460 != (0 as i32)) {
                                                            // /* The row-trigger may have deleted the row being updated. In this
                                                            //       ** case, jump to the next row. No updates or AFTER triggers are
                                                            //       ** required. This behavior - what happens when the row being updated
                                                            //       ** is deleted or renamed by a BEFORE trigger - is left undefined in the
                                                            //       ** documentation.
                                                            //       */
                                                            if *__slate_slot_437
                                                                != std::ptr::null_mut::<Index>()
                                                            {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp4Int(
                                                                        *__slate_slot_435,
                                                                        28 as i32,
                                                                        *__slate_slot_441,
                                                                        *__slate_slot_458,
                                                                        *__slate_slot_479,
                                                                        *__slate_slot_465,
                                                                    )
                                                                };
                                                                {}
                                                            } else {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp3(
                                                                        *__slate_slot_435,
                                                                        31 as i32,
                                                                        *__slate_slot_441,
                                                                        *__slate_slot_458,
                                                                        *__slate_slot_474,
                                                                    )
                                                                };
                                                                {}
                                                            }
                                                            // /* After-BEFORE-trigger-reload-loop:
                                                            //       ** If it did not delete it, the BEFORE trigger may still have modified
                                                            //       ** some of the columns of the row being updated. Load the values for
                                                            //       ** all columns not modified by the update statement into their registers
                                                            //       ** in case this has happened. Only unmodified columns are reloaded.
                                                            //       ** The values computed for modified columns use the values before the
                                                            //       ** BEFORE trigger runs.  See test case trigger1-18.0 (added 2018-04-26)
                                                            //       ** for an example.
                                                            //       */
                                                            *__slate_slot_429 = 0 as i32;
                                                            std::ptr::write(
                                                                __slate_slot_911,
                                                                *__slate_slot_476,
                                                            );
                                                            *__slate_slot_431 = *__slate_slot_911;
                                                            loop {
                                                                if *__slate_slot_429
                                                                    < ((unsafe {
                                                                        (*(*__slate_slot_432)).nCol
                                                                    })
                                                                        as i32)
                                                                {
                                                                    if (((unsafe {
                                                                        (*unsafe { unsafe { (*(*__slate_slot_432)).aCol }.offset(*__slate_slot_429 as isize) }).colFlags
                                                                    })
                                                                        as u32)
                                                                        as i32)
                                                                        & (96 as i32)
                                                                        != (0 as i32)
                                                                    {
                                                                        if (((unsafe {
                                                                            (*unsafe { unsafe { (*(*__slate_slot_432)).aCol }.offset(*__slate_slot_429 as isize) }).colFlags
                                                                        })
                                                                            as u32)
                                                                            as i32)
                                                                            & (32 as i32)
                                                                            != (0 as i32)
                                                                        {
                                                                            std::ptr::write(
                                                                                __slate_slot_916,
                                                                                *__slate_slot_431,
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_917,
                                                                                *__slate_slot_916
                                                                                    - (1 as i32),
                                                                            );
                                                                            *__slate_slot_431 =
                                                                                *__slate_slot_917;
                                                                        }
                                                                    } else {
                                                                        if (unsafe {
                                                                            *unsafe {
                                                                                (*__slate_slot_445).offset(*__slate_slot_429 as isize)
                                                                            }
                                                                        }) < (0 as i32)
                                                                            && *__slate_slot_429
                                                                                != ((unsafe {
                                                                                    (*(*__slate_slot_432)).iPKey
                                                                                })
                                                                                    as i32)
                                                                        {
                                                                            unsafe {
                                                                                sqlite3ExprCodeGetColumnOfTable(*__slate_slot_435, *__slate_slot_432, *__slate_slot_441, *__slate_slot_429, *__slate_slot_431)
                                                                            };
                                                                        }
                                                                    }
                                                                    std::ptr::write(
                                                                        __slate_slot_912,
                                                                        *__slate_slot_429,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_913,
                                                                        *__slate_slot_912
                                                                            + (1 as i32),
                                                                    );
                                                                    *__slate_slot_429 =
                                                                        *__slate_slot_913;
                                                                    std::ptr::write(
                                                                        __slate_slot_914,
                                                                        *__slate_slot_431,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_915,
                                                                        *__slate_slot_914
                                                                            + (1 as i32),
                                                                    );
                                                                    *__slate_slot_431 =
                                                                        *__slate_slot_915;
                                                                } else {
                                                                    break;
                                                                }
                                                            }
                                                            if (unsafe {
                                                                (*(*__slate_slot_432)).tabFlags
                                                            }) & ((96 as i32) as u32)
                                                                != (0 as u32)
                                                            {
                                                                {}
                                                                {}
                                                                unsafe {
                                                                    sqlite3ComputeGeneratedColumns(
                                                                        pParse,
                                                                        *__slate_slot_476,
                                                                        *__slate_slot_432,
                                                                    )
                                                                };
                                                            }
                                                        }
                                                    }
                                                    if !(*__slate_slot_460 != (0 as i32)) {
                                                        // /* Do constraint checks. */
                                                        0 as i32;
                                                        unsafe {
                                                            sqlite3GenerateConstraintChecks(
                                                                pParse,
                                                                *__slate_slot_432,
                                                                *__slate_slot_444,
                                                                *__slate_slot_441,
                                                                *__slate_slot_442,
                                                                *__slate_slot_475,
                                                                *__slate_slot_474,
                                                                *__slate_slot_449,
                                                                (onError as i8) as u8,
                                                                *__slate_slot_458,
                                                                std::ptr::addr_of_mut!(
                                                                    *__slate_slot_470
                                                                ),
                                                                *__slate_slot_445,
                                                                std::ptr::null_mut::<Upsert>(),
                                                            )
                                                        };
                                                        // /* If REPLACE conflict handling may have been used, or if the PK of the
                                                        //     ** row is changing, then the GenerateConstraintChecks() above may have
                                                        //     ** moved cursor iDataCur. Reseek it. */
                                                        if *__slate_slot_470 != (0 as i32)
                                                            || *__slate_slot_449 != (0 as u8)
                                                        {
                                                            if *__slate_slot_437
                                                                != std::ptr::null_mut::<Index>()
                                                            {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp4Int(
                                                                        *__slate_slot_435,
                                                                        28 as i32,
                                                                        *__slate_slot_441,
                                                                        *__slate_slot_458,
                                                                        *__slate_slot_479,
                                                                        *__slate_slot_465,
                                                                    )
                                                                };
                                                            } else {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp3(
                                                                        *__slate_slot_435,
                                                                        31 as i32,
                                                                        *__slate_slot_441,
                                                                        *__slate_slot_458,
                                                                        *__slate_slot_474,
                                                                    )
                                                                };
                                                            }
                                                            {}
                                                        }
                                                        // /* Do FK constraint checks. */
                                                        if *__slate_slot_456 != (0 as i32) {
                                                            unsafe {
                                                                sqlite3FkCheck(
                                                                    pParse,
                                                                    *__slate_slot_432,
                                                                    *__slate_slot_474,
                                                                    0 as i32,
                                                                    *__slate_slot_445,
                                                                    (*__slate_slot_449 as u32)
                                                                        as i32,
                                                                )
                                                            };
                                                        }
                                                        // /* Delete the index entries associated with the current record.  */
                                                        unsafe {
                                                            sqlite3GenerateRowIndexDelete(
                                                                pParse,
                                                                *__slate_slot_432,
                                                                *__slate_slot_441,
                                                                *__slate_slot_442,
                                                                *__slate_slot_444,
                                                                -(1 as i32),
                                                            )
                                                        };
                                                        // /* We must run the OP_FinishSeek opcode to resolve a prior
                                                        //     ** OP_DeferredSeek if there is any possibility that there have been
                                                        //     ** no OP_Column opcodes since the OP_DeferredSeek was issued.  But
                                                        //     ** we want to avoid the OP_FinishSeek if possible, as running it
                                                        //     ** costs CPU cycles. */
                                                        if *__slate_slot_471 != (0 as i32) {
                                                            unsafe {
                                                                sqlite3VdbeAddOp1(
                                                                    *__slate_slot_435,
                                                                    145 as i32,
                                                                    *__slate_slot_441,
                                                                )
                                                            };
                                                        }
                                                        // /* If changing the rowid value, or if there are foreign key constraints
                                                        //     ** to process, delete the old record. Otherwise, add a noop OP_Delete
                                                        //     ** to invoke the pre-update hook.
                                                        //     **
                                                        //     ** That (regNew==regnewRowid+1) is true is also important for the
                                                        //     ** pre-update hook. If the caller invokes preupdate_new(), the returned
                                                        //     ** value is copied from memory cell (regNewRowid+1+iCol), where iCol
                                                        //     ** is the column index supplied by the user.
                                                        //     */
                                                        0 as i32;
                                                        if *__slate_slot_456 > (1 as i32)
                                                            || *__slate_slot_449 != (0 as u8)
                                                        {
                                                            unsafe {
                                                                sqlite3VdbeAddOp2(
                                                                    *__slate_slot_435,
                                                                    132 as i32,
                                                                    *__slate_slot_441,
                                                                    0 as i32,
                                                                )
                                                            };
                                                        }
                                                        if *__slate_slot_456 != (0 as i32) {
                                                            unsafe {
                                                                sqlite3FkCheck(
                                                                    pParse,
                                                                    *__slate_slot_432,
                                                                    0 as i32,
                                                                    *__slate_slot_475,
                                                                    *__slate_slot_445,
                                                                    (*__slate_slot_449 as u32)
                                                                        as i32,
                                                                )
                                                            };
                                                        }
                                                        // /* Insert the new index entries and the new record. */
                                                        unsafe {
                                                            sqlite3CompleteInsertion(
                                                                pParse,
                                                                *__slate_slot_432,
                                                                *__slate_slot_441,
                                                                *__slate_slot_442,
                                                                *__slate_slot_475,
                                                                *__slate_slot_444,
                                                                (4 as i32)
                                                                    | if *__slate_slot_455
                                                                        == (2 as i32)
                                                                    {
                                                                        2 as i32
                                                                    } else {
                                                                        0 as i32
                                                                    },
                                                                0 as i32,
                                                                0 as i32,
                                                            )
                                                        };
                                                        // /* Do any ON CASCADE, SET NULL or SET DEFAULT operations required to
                                                        //     ** handle rows (possibly in other tables) that refer via a foreign key
                                                        //     ** to the row just updated. */
                                                        if *__slate_slot_456 != (0 as i32) {
                                                            unsafe {
                                                                sqlite3FkActions(
                                                                    pParse,
                                                                    *__slate_slot_432,
                                                                    pChanges,
                                                                    *__slate_slot_474,
                                                                    *__slate_slot_445,
                                                                    (*__slate_slot_449 as u32)
                                                                        as i32,
                                                                )
                                                            };
                                                        }
                                                    }
                                                    // /* Increment the row counter
                                                    //   */
                                                    if *__slate_slot_473 != (0 as i32) {
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_435,
                                                                88 as i32,
                                                                *__slate_slot_473,
                                                                1 as i32,
                                                            )
                                                        };
                                                    }
                                                    if *__slate_slot_461
                                                        != std::ptr::null_mut::<Trigger>()
                                                    {
                                                        unsafe {
                                                            sqlite3CodeRowTrigger(
                                                                pParse,
                                                                *__slate_slot_461,
                                                                130 as i32,
                                                                pChanges,
                                                                2 as i32,
                                                                *__slate_slot_432,
                                                                *__slate_slot_474,
                                                                onError,
                                                                *__slate_slot_458,
                                                            )
                                                        };
                                                    }
                                                    // /* Repeat the above with the next record to be updated, until
                                                    //   ** all record selected by the WHERE clause have been updated.
                                                    //   */
                                                    if *__slate_slot_455 == (1 as i32) {
                                                        // /* Nothing to do at end-of-loop for a single-pass */
                                                    } else {
                                                        if *__slate_slot_455 == (2 as i32) {
                                                            unsafe {
                                                                sqlite3VdbeResolveLabel(
                                                                    *__slate_slot_435,
                                                                    *__slate_slot_458,
                                                                )
                                                            };
                                                            unsafe {
                                                                sqlite3WhereEnd(*__slate_slot_434)
                                                            };
                                                        } else {
                                                            unsafe {
                                                                sqlite3VdbeResolveLabel(
                                                                    *__slate_slot_435,
                                                                    *__slate_slot_458,
                                                                )
                                                            };
                                                            unsafe {
                                                                sqlite3VdbeAddOp2(
                                                                    *__slate_slot_435,
                                                                    40 as i32,
                                                                    *__slate_slot_464,
                                                                    *__slate_slot_433,
                                                                )
                                                            };
                                                            {}
                                                        }
                                                    }
                                                    unsafe {
                                                        sqlite3VdbeResolveLabel(
                                                            *__slate_slot_435,
                                                            *__slate_slot_457,
                                                        )
                                                    };
                                                    // /* Update the sqlite_sequence table by storing the content of the
                                                    //   ** maximum rowid counter values recorded while inserting into
                                                    //   ** autoincrement tables.
                                                    //   */
                                                    if (((unsafe { (*pParse).nested }) as u32)
                                                        as i32)
                                                        == (0 as i32)
                                                        && (unsafe { (*pParse).pTriggerTab })
                                                            == std::ptr::null_mut::<Table>()
                                                        && pUpsert == std::ptr::null_mut::<Upsert>()
                                                    {
                                                        unsafe { sqlite3AutoincrementEnd(pParse) };
                                                    }
                                                    // /*
                                                    //   ** Return the number of rows that were changed, if we are tracking
                                                    //   ** that information.
                                                    //   */
                                                    if *__slate_slot_473 != (0 as i32) {
                                                        unsafe {
                                                            sqlite3CodeChangeCount(
                                                                *__slate_slot_435,
                                                                *__slate_slot_473,
                                                                (b"rows updated\0".as_ptr()
                                                                    as *mut i8)
                                                                    as *const i8,
                                                            )
                                                        };
                                                        break '__join_0;
                                                    } else {
                                                        break '__join_0;
                                                    }
                                                }
                                            }
                                        }
                                        unsafe {
                                            sqlite3ErrorMsg(
                                                pParse,
                                                (b"no such column: %s\0".as_ptr() as *mut i8)
                                                    as *const i8,
                                                unsafe {
                                                    (*unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of_mut!((*pChanges).a)
                                                                as *mut ExprList_item
                                                        }
                                                        .offset(*__slate_slot_429 as isize)
                                                    })
                                                    .zEName
                                                },
                                            )
                                        };
                                        unsafe {
                                            (*pParse)
                                                .__slate_bits_0
                                                .__set_checkSchema((1 as i32) as u32);
                                        }
                                        break '__join_0;
                                    }
                                    {}
                                    {}
                                    unsafe {
                                        sqlite3ErrorMsg(
                                            pParse,
                                            (b"cannot UPDATE generated column \"%s\"\0".as_ptr()
                                                as *mut i8)
                                                as *const i8,
                                            unsafe {
                                                (*unsafe {
                                                    unsafe { (*(*__slate_slot_432)).aCol }
                                                        .offset(*__slate_slot_430 as isize)
                                                })
                                                .zCnName
                                            },
                                        )
                                    };
                                }
                            }
                        }
                    }
                }
            }
        }
        unsafe { sqlite3AuthContextPop(std::ptr::addr_of_mut!(*__slate_slot_452)) };
        // /* Also frees aRegIdx[] and aToOpen[] */
        unsafe { sqlite3DbFree(*__slate_slot_443, *__slate_slot_445 as *mut ()) };
        unsafe { sqlite3SrcListDelete(*__slate_slot_443, pTabList) };
        unsafe { sqlite3ExprListDelete(*__slate_slot_443, pChanges) };
        unsafe { sqlite3ExprDelete(*__slate_slot_443, pWhere) };
        return;
    }
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
// ** to handle UPDATE statements.
// */
// /* Forward declaration */
// /* The parsing context */
// /* The virtual table to be modified */
// /* The virtual table */
// /* The columns to change in the UPDATE statement */
// /* Expression used to recompute the rowid */
// /* Mapping from columns of pTab to entries in pChanges */
// /* WHERE clause of the UPDATE statement */
// /* ON CONFLICT strategy */
// /* SQLITE_OMIT_VIRTUALTABLE */
// /*
// ** The most recently coded instruction was an OP_Column to retrieve the
// ** i-th column of table pTab. This routine sets the P4 parameter of the
// ** OP_Column to the default value, if any.
// **
// ** The default value of a column is specified by a DEFAULT clause in the
// ** column definition. This was either supplied by the user when the table
// ** was created, or added later to the table definition by an ALTER TABLE
// ** command. If the latter, then the row-records in the table btree on disk
// ** may not contain a value for the column and the default value, taken
// ** from the P4 parameter of the OP_Column instruction, is returned instead.
// ** If the former, then all row-records are guaranteed to include a value
// ** for the column and the P4 value is not required.
// **
// ** Column definitions created by an ALTER TABLE command may only have
// ** literal default values specified: a number, null or a string. (If a more
// ** complicated default expression value was provided, it is evaluated
// ** when the ALTER TABLE is executed and one of the literal values written
// ** into the sqlite_schema table.)
// **
// ** Therefore, the P4 parameter is only required if the default value for
// ** the column is a literal number, string or null. The sqlite3ValueFromExpr()
// ** function is capable of transforming these types of expressions into
// ** sqlite3_value objects.
// **
// ** If column as REAL affinity and the table is an ordinary b-tree table
// ** (not a virtual table) then the value might have been stored as an
// ** integer.  In that case, add an OP_RealAffinity opcode to make sure
// ** it has been converted into REAL.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ColumnDefault(
    mut v: *mut Vdbe,
    mut pTab: *mut Table,
    mut i: i32,
    mut iReg: i32,
) {
    let mut pCol: *mut Column = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    pCol = unsafe { unsafe { (*pTab).aCol }.offset(i as isize) };
    if (unsafe { (*pCol).iDflt }) != (0 as u16) {
        let mut pValue: *mut sqlite3_value = std::ptr::null_mut::<sqlite3_value>();
        let mut enc: u8 = unsafe { (*unsafe { sqlite3VdbeDb(v) }).enc };
        0 as i32;
        {}
        0 as i32;
        unsafe {
            sqlite3ValueFromExpr(
                unsafe { sqlite3VdbeDb(v) },
                (unsafe { sqlite3ColumnExpr(pTab, pCol) }) as *const Expr,
                enc,
                (unsafe { (*pCol).affinity }) as u8,
                std::ptr::addr_of_mut!(pValue),
            )
        };
        if pValue != std::ptr::null_mut::<sqlite3_value>() {
            unsafe { sqlite3VdbeAppendP4(v, pValue as *mut (), -(11 as i32)) };
        }
    }
    if ((unsafe { (*pCol).affinity }) as i32) == (69 as i32)
        && !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32))
    {
        unsafe { sqlite3VdbeAddOp1(v, 89 as i32, iReg) };
    }
}

// /* The parser context */
// /* The table in which we should change things */
// /* Things to be changed */
// /* The WHERE clause.  May be null */
// /* How to handle constraint errors */
// /* ORDER BY clause. May be null */
// /* LIMIT clause. May be null */
// /* ON CONFLICT clause, or null */
// /* Make sure "isView" and other macros defined above are undefined. Otherwise
// ** they may interfere with compilation of other functions in this file
// ** (or in another file, if this file becomes part of the amalgamation).  */
// /*
// ** Generate code for an UPDATE of a virtual table.
// **
// ** There are two possible strategies - the default and the special
// ** "onepass" strategy. Onepass is only used if the virtual table
// ** implementation indicates that pWhere may match at most one row.
// **
// ** The default strategy is to create an ephemeral table that contains
// ** for each row to be changed:
// **
// **   (A)  The original rowid of that row.
// **   (B)  The revised rowid for the row.
// **   (C)  The content of every column in the row.
// **
// ** Then loop through the contents of this ephemeral table executing a
// ** VUpdate for each row. When finished, drop the ephemeral table.
// **
// ** The "onepass" strategy does not use an ephemeral table. Instead, it
// ** stores the same values (A, B and C above) in a register array and
// ** makes a single invocation of VUpdate.
// */
fn updateVirtualTable(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut pTab: *mut Table,
    mut pChanges: *mut ExprList,
    mut pRowid: *mut Expr,
    mut aXRef: *mut i32,
    mut pWhere: *mut Expr,
    mut onError: i32,
) {
    // /* Virtual machine under construction */
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    // /* Table holding the result of the SELECT */
    let mut ephemTab: i32 = 0 as i32;
    // /* Loop counter */
    let mut i: i32 = 0 as i32;
    // /* Database connection */
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pVTab: *const i8 = (unsafe { sqlite3GetVTable(db, pTab) }) as *const i8;
    let mut pWInfo: *mut WhereInfo = std::ptr::null_mut::<WhereInfo>();
    // /* Number of arguments to VUpdate */
    let mut nArg: i32 = (2 as i32) + ((unsafe { (*pTab).nCol }) as i32);
    // /* First register in VUpdate arg array */
    let mut regArg: i32 = 0 as i32;
    // /* Register in which to assemble record */
    let mut regRec: i32 = 0 as i32;
    // /* Register for ephemeral table rowid */
    let mut regRowid: i32 = 0 as i32;
    // /* Cursor used for virtual table scan */
    let mut iCsr: i32 = unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset((0 as i32) as isize)
        })
        .iCursor
    };
    // /* Unused arg for sqlite3WhereOkOnePass() */
    let mut aDummy: [i32; 2] = [0 as i32; 2];
    // /* True to use onepass strategy */
    let mut eOnePass: i32 = 0 as i32;
    // /* Address of OP_OpenEphemeral */
    let mut addr: i32 = 0 as i32;
    // /* Allocate nArg registers in which to gather the arguments for VUpdate. Then
    //   ** create and open the ephemeral table in which the records created from
    //   ** these arguments will be temporarily stored. */
    0 as i32;
    let __v918: *mut Parse = pParse;
    let __v919: i32 = unsafe { (*__v918).nTab };
    let __v920: i32 = __v919 + (1 as i32);
    unsafe {
        (*__v918).nTab = __v920;
    }
    ephemTab = __v919;
    addr = unsafe { sqlite3VdbeAddOp2(v, 120 as i32, ephemTab, nArg) };
    regArg = (unsafe { (*pParse).nMem }) + (1 as i32);
    let __v921: *mut Parse = pParse;
    let __v922: i32 = unsafe { (*__v921).nMem };
    let __v923: i32 = __v922 + nArg;
    unsafe {
        (*__v921).nMem = __v923;
    }
    if (unsafe { (*pSrc).nSrc }) > (1 as i32) {
        let mut pPk: *mut Index = std::ptr::null_mut::<Index>();
        let mut pRow: *mut Expr = unsafe { std::mem::zeroed() };
        let mut pList: *mut ExprList = unsafe { std::mem::zeroed() };
        if (unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
            if pRowid != std::ptr::null_mut::<Expr>() {
                pRow = unsafe { sqlite3ExprDup(db, pRowid as *const Expr, 0 as i32) };
            } else {
                pRow = unsafe {
                    sqlite3PExpr(
                        pParse,
                        76 as i32,
                        std::ptr::null_mut::<Expr>(),
                        std::ptr::null_mut::<Expr>(),
                    )
                };
            }
        } else {
            // /* PRIMARY KEY column */
            let mut iPk: i16 = 0 as i16;
            pPk = unsafe { sqlite3PrimaryKeyIndex(pTab) };
            0 as i32;
            0 as i32;
            iPk = unsafe { *unsafe { unsafe { (*pPk).aiColumn }.offset((0 as i32) as isize) } };
            if (unsafe { *unsafe { aXRef.offset((iPk as i32) as isize) } }) >= (0 as i32) {
                pRow = unsafe {
                    sqlite3ExprDup(
                        db,
                        (unsafe {
                            (*unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*pChanges).a) as *mut ExprList_item
                                }
                                .offset(
                                    (unsafe { *unsafe { aXRef.offset((iPk as i32) as isize) } })
                                        as isize,
                                )
                            })
                            .pExpr
                        }) as *const Expr,
                        0 as i32,
                    )
                };
            } else {
                pRow = exprRowColumn(pParse, iPk as i32);
            }
        }
        pList = unsafe { sqlite3ExprListAppend(pParse, std::ptr::null_mut::<ExprList>(), pRow) };
        i = 0 as i32;
        '__slate_break_814: loop {
            if !(i < ((unsafe { (*pTab).nCol }) as i32)) {
                break;
            }
            if (unsafe { *unsafe { aXRef.offset(i as isize) } }) >= (0 as i32) {
                pList = unsafe {
                    sqlite3ExprListAppend(pParse, pList, unsafe {
                        sqlite3ExprDup(
                            db,
                            (unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pChanges).a) as *mut ExprList_item
                                    }
                                    .offset(
                                        (unsafe { *unsafe { aXRef.offset(i as isize) } }) as isize,
                                    )
                                })
                                .pExpr
                            }) as *const Expr,
                            0 as i32,
                        )
                    })
                };
            } else {
                let mut pRowExpr: *mut Expr = exprRowColumn(pParse, i);
                if pRowExpr != std::ptr::null_mut::<Expr>() {
                    unsafe {
                        (*pRowExpr).op2 = ((1 as i32) as i8) as u8;
                    }
                }
                pList = unsafe { sqlite3ExprListAppend(pParse, pList, pRowExpr) };
            }
            let __v924: i32 = i;
            let __v925: i32 = __v924 + (1 as i32);
            i = __v925;
        }
        updateFromSelect(
            pParse,
            ephemTab,
            pPk,
            pList,
            pSrc,
            pWhere,
            std::ptr::null_mut::<ExprList>(),
            std::ptr::null_mut::<Expr>(),
        );
        unsafe { sqlite3ExprListDelete(db, pList) };
        eOnePass = 0 as i32;
    } else {
        let __v926: *mut Parse = pParse;
        let __v927: i32 = unsafe { (*__v926).nMem };
        let __v928: i32 = __v927 + (1 as i32);
        unsafe {
            (*__v926).nMem = __v928;
        }
        regRec = __v928;
        let __v929: *mut Parse = pParse;
        let __v930: i32 = unsafe { (*__v929).nMem };
        let __v931: i32 = __v930 + (1 as i32);
        unsafe {
            (*__v929).nMem = __v931;
        }
        regRowid = __v931;
        // /* Start scanning the virtual table */
        pWInfo = unsafe {
            sqlite3WhereBegin(
                pParse,
                pSrc,
                pWhere,
                std::ptr::null_mut::<ExprList>(),
                std::ptr::null_mut::<ExprList>(),
                std::ptr::null_mut::<Select>(),
                ((4 as i32) as i16) as u16,
                0 as i32,
            )
        };
        if pWInfo == std::ptr::null_mut::<WhereInfo>() {
            return;
        }
        // /* Populate the argument registers. */
        i = 0 as i32;
        '__slate_break_815: loop {
            if !(i < ((unsafe { (*pTab).nCol }) as i32)) {
                break;
            }
            0 as i32;
            if (unsafe { *unsafe { aXRef.offset(i as isize) } }) >= (0 as i32) {
                unsafe {
                    sqlite3ExprCode(
                        pParse,
                        unsafe {
                            (*unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*pChanges).a) as *mut ExprList_item
                                }
                                .offset((unsafe { *unsafe { aXRef.offset(i as isize) } }) as isize)
                            })
                            .pExpr
                        },
                        regArg + (2 as i32) + i,
                    )
                };
            } else {
                unsafe { sqlite3VdbeAddOp3(v, 178 as i32, iCsr, i, regArg + (2 as i32) + i) };
                // /* For sqlite3_vtab_nochange() */
                unsafe { sqlite3VdbeChangeP5(v, ((1 as i32) as i16) as u16) };
            }
            let __v932: i32 = i;
            let __v933: i32 = __v932 + (1 as i32);
            i = __v933;
        }
        if (unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
            unsafe { sqlite3VdbeAddOp2(v, 137 as i32, iCsr, regArg) };
            if pRowid != std::ptr::null_mut::<Expr>() {
                unsafe { sqlite3ExprCode(pParse, pRowid, regArg + (1 as i32)) };
            } else {
                unsafe { sqlite3VdbeAddOp2(v, 137 as i32, iCsr, regArg + (1 as i32)) };
            }
        } else {
            // /* PRIMARY KEY index */
            let mut pPk: *mut Index = unsafe { std::mem::zeroed() };
            // /* PRIMARY KEY column */
            let mut iPk: i16 = 0 as i16;
            pPk = unsafe { sqlite3PrimaryKeyIndex(pTab) };
            0 as i32;
            0 as i32;
            iPk = unsafe { *unsafe { unsafe { (*pPk).aiColumn }.offset((0 as i32) as isize) } };
            unsafe { sqlite3VdbeAddOp3(v, 178 as i32, iCsr, iPk as i32, regArg) };
            unsafe {
                sqlite3VdbeAddOp2(
                    v,
                    83 as i32,
                    regArg + (2 as i32) + (iPk as i32),
                    regArg + (1 as i32),
                )
            };
        }
        eOnePass = unsafe { sqlite3WhereOkOnePass(pWInfo, aDummy.as_mut_ptr() as *mut i32) };
        // /* There is no ONEPASS_MULTI on virtual tables */
        0 as i32;
        if eOnePass != (0 as i32) {
            // /* If using the onepass strategy, no-op out the OP_OpenEphemeral coded
            //       ** above. */
            unsafe { sqlite3VdbeChangeToNoop(v, addr) };
            unsafe { sqlite3VdbeAddOp1(v, 124 as i32, iCsr) };
        } else {
            // /* Create a record from the argument register contents and insert it into
            //       ** the ephemeral table. */
            unsafe { sqlite3MultiWrite(pParse) };
            unsafe { sqlite3VdbeAddOp3(v, 99 as i32, regArg, nArg, regRec) };
            unsafe { sqlite3VdbeAddOp2(v, 129 as i32, ephemTab, regRowid) };
            unsafe { sqlite3VdbeAddOp3(v, 130 as i32, ephemTab, regRec, regRowid) };
        }
    }
    if eOnePass == (0 as i32) {
        // /* End the virtual table scan */
        if (unsafe { (*pSrc).nSrc }) == (1 as i32) {
            unsafe { sqlite3WhereEnd(pWInfo) };
        }
        // /* Begin scanning through the ephemeral table. */
        addr = unsafe { sqlite3VdbeAddOp1(v, 36 as i32, ephemTab) };
        {}
        // /* Extract arguments from the current row of the ephemeral table and
        //     ** invoke the VUpdate method.  */
        i = 0 as i32;
        '__slate_break_816: loop {
            if !(i < nArg) {
                break;
            }
            unsafe { sqlite3VdbeAddOp3(v, 96 as i32, ephemTab, i, regArg + i) };
            let __v934: i32 = i;
            let __v935: i32 = __v934 + (1 as i32);
            i = __v935;
        }
    }
    unsafe { sqlite3VtabMakeWritable(pParse, pTab) };
    unsafe { sqlite3VdbeAddOp4(v, 7 as i32, 0 as i32, nArg, regArg, pVTab, -(12 as i32)) };
    unsafe {
        sqlite3VdbeChangeP5(
            v,
            ((if onError == (11 as i32) {
                2 as i32
            } else {
                onError
            }) as i16) as u16,
        )
    };
    unsafe { sqlite3MayAbort(pParse) };
    // /* End of the ephemeral table scan. Or, if using the onepass strategy,
    //   ** jump to here if the scan visited zero rows. */
    if eOnePass == (0 as i32) {
        unsafe { sqlite3VdbeAddOp2(v, 40 as i32, ephemTab, addr + (1 as i32)) };
        {}
        unsafe { sqlite3VdbeJumpHere(v, addr) };
        unsafe { sqlite3VdbeAddOp2(v, 124 as i32, ephemTab, 0 as i32) };
    } else {
        unsafe { sqlite3WhereEnd(pWInfo) };
    }
}

// /*
// ** Check to see if column iCol of index pIdx references any of the
// ** columns defined by aXRef and chngRowid.  Return true if it does
// ** and false if not.  This is an optimization.  False-positives are a
// ** performance degradation, but false-negatives can result in a corrupt
// ** index and incorrect answers.
// **
// ** aXRef[j] will be non-negative if column j of the original table is
// ** being updated.  chngRowid will be true if the rowid of the table is
// ** being updated.
// */
// /* The parsing context */
// /* The virtual table to be modified */
// /* The virtual table */
// /* The columns to change in the UPDATE statement */
// /* Expression used to recompute the rowid */
// /* Mapping from columns of pTab to entries in pChanges */
// /* WHERE clause of the UPDATE statement */
// /* ON CONFLICT strategy */
// /* SQLITE_OMIT_VIRTUALTABLE */
fn indexColumnIsBeingUpdated(
    mut pIdx: *mut Index,
    mut iCol: i32,
    mut aXRef: *mut i32,
    mut chngRowid: i32,
) -> i32 {
    let mut iIdxCol: i16 = unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(iCol as isize) } };
    // /* Cannot index rowid */
    0 as i32;
    if (iIdxCol as i32) >= (0 as i32) {
        return ((unsafe { *unsafe { aXRef.offset((iIdxCol as i32) as isize) } }) >= (0 as i32))
            as i32;
    }
    0 as i32;
    0 as i32;
    0 as i32;
    return unsafe {
        sqlite3ExprReferencesUpdatedColumn(
            unsafe {
                (*unsafe {
                    unsafe {
                        std::ptr::addr_of_mut!((*unsafe { (*pIdx).aColExpr }).a)
                            as *mut ExprList_item
                    }
                    .offset(iCol as isize)
                })
                .pExpr
            },
            aXRef,
            chngRowid,
        )
    };
}

// /* The index to check */
// /* Which column of the index to check */
// /* aXRef[j]>=0 if column j is being updated */
// /* true if the rowid is being updated */
// /*
// ** Check to see if index pIdx is a partial index whose conditional
// ** expression might change values due to an UPDATE.  Return true if
// ** the index is subject to change and false if the index is guaranteed
// ** to be unchanged.  This is an optimization.  False-positives are a
// ** performance degradation, but false-negatives can result in a corrupt
// ** index and incorrect answers.
// **
// ** aXRef[j] will be non-negative if column j of the original table is
// ** being updated.  chngRowid will be true if the rowid of the table is
// ** being updated.
// */
fn indexWhereClauseMightChange(
    mut pIdx: *mut Index,
    mut aXRef: *mut i32,
    mut chngRowid: i32,
) -> i32 {
    if (unsafe { (*pIdx).pPartIdxWhere }) == std::ptr::null_mut::<Expr>() {
        return 0 as i32;
    }
    return unsafe {
        sqlite3ExprReferencesUpdatedColumn(unsafe { (*pIdx).pPartIdxWhere }, aXRef, chngRowid)
    };
}

// /* The index to check */
// /* aXRef[j]>=0 if column j is being updated */
// /* true if the rowid is being updated */
// /*
// ** Allocate and return a pointer to an expression of type TK_ROW with
// ** Expr.iColumn set to value (iCol+1). The resolver will modify the
// ** expression to be a TK_COLUMN reading column iCol of the first
// ** table in the source-list (pSrc->a[0]).
// */
fn exprRowColumn(mut pParse: *mut Parse, mut iCol: i32) -> *mut Expr {
    let mut pRet: *mut Expr = unsafe {
        sqlite3PExpr(
            pParse,
            76 as i32,
            std::ptr::null_mut::<Expr>(),
            std::ptr::null_mut::<Expr>(),
        )
    };
    if pRet != std::ptr::null_mut::<Expr>() {
        unsafe {
            (*pRet).iColumn = (iCol + (1 as i32)) as i16;
        }
    }
    return pRet;
}

// /*
// ** Assuming both the pLimit and pOrderBy parameters are NULL, this function
// ** generates VM code to run the query:
// **
// **   SELECT <other-columns>, pChanges FROM pTabList WHERE pWhere
// **
// ** and write the results to the ephemeral table already opened as cursor
// ** iEph. None of pChanges, pTabList or pWhere are modified or consumed by
// ** this function, they must be deleted by the caller.
// **
// ** Or, if pLimit and pOrderBy are not NULL, and pTab is not a view:
// **
// **   SELECT <other-columns>, pChanges FROM pTabList
// **   WHERE pWhere
// **   GROUP BY <other-columns>
// **   ORDER BY pOrderBy LIMIT pLimit
// **
// ** If pTab is a view, the GROUP BY clause is omitted.
// **
// ** Exactly how results are written to table iEph, and exactly what
// ** the <other-columns> in the query above are is determined by the type
// ** of table pTabList->a[0].pTab.
// **
// ** If the table is a WITHOUT ROWID table, then argument pPk must be its
// ** PRIMARY KEY. In this case <other-columns> are the primary key columns
// ** of the table, in order. The results of the query are written to ephemeral
// ** table iEph as index keys, using OP_IdxInsert.
// **
// ** If the table is actually a view, then <other-columns> are all columns of
// ** the view. The results are written to the ephemeral table iEph as records
// ** with automatically assigned integer keys.
// **
// ** If the table is a virtual or ordinary intkey table, then <other-columns>
// ** is its rowid. For a virtual table, the results are written to iEph as
// ** records with automatically assigned integer keys For intkey tables, the
// ** rowid value in <other-columns> is used as the integer key, and the
// ** remaining fields make up the table record.
// */
fn updateFromSelect(
    mut pParse: *mut Parse,
    mut iEph: i32,
    mut pPk: *mut Index,
    mut pChanges: *mut ExprList,
    mut pTabList: *mut SrcList,
    mut pWhere: *mut Expr,
    mut pOrderBy: *mut ExprList,
    mut pLimit: *mut Expr,
) {
    let mut i: i32 = 0 as i32;
    let mut dest: SelectDest = unsafe { std::mem::zeroed() };
    let mut pSelect: *mut Select = std::ptr::null_mut::<Select>();
    let mut pList: *mut ExprList = std::ptr::null_mut::<ExprList>();
    let mut pGrp: *mut ExprList = std::ptr::null_mut::<ExprList>();
    let mut pLimit2: *mut Expr = std::ptr::null_mut::<Expr>();
    let mut pOrderBy2: *mut ExprList = std::ptr::null_mut::<ExprList>();
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pTab: *mut Table = unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                .offset((0 as i32) as isize)
        })
        .pSTab
    };
    let mut pSrc: *mut SrcList = unsafe { std::mem::zeroed() };
    let mut pWhere2: *mut Expr = unsafe { std::mem::zeroed() };
    let mut eDest: i32 = 0 as i32;
    pOrderBy;
    pLimit;
    pSrc = unsafe { sqlite3SrcListDup(db, pTabList as *const SrcList, 0 as i32) };
    pWhere2 = unsafe { sqlite3ExprDup(db, pWhere as *const Expr, 0 as i32) };
    0 as i32;
    if pSrc != std::ptr::null_mut::<SrcList>() {
        0 as i32;
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .iCursor = -(1 as i32);
        }
        let __v936: *mut Table = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .pSTab
        };
        let __v937: u32 = unsafe { (*__v936).nTabRef };
        let __v938: u32 = __v937.wrapping_sub((1 as i32) as u32);
        unsafe {
            (*__v936).nTabRef = __v938;
        }
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .pSTab = std::ptr::null_mut::<Table>();
        }
    }
    if pPk != std::ptr::null_mut::<Index>() {
        i = 0 as i32;
        '__slate_break_795: loop {
            if !(i < (((unsafe { (*pPk).nKeyCol }) as u32) as i32)) {
                break;
            }
            let mut pNew: *mut Expr = exprRowColumn(
                pParse,
                (unsafe { *unsafe { unsafe { (*pPk).aiColumn }.offset(i as isize) } }) as i32,
            );
            pList = unsafe { sqlite3ExprListAppend(pParse, pList, pNew) };
            let __v939: i32 = i;
            let __v940: i32 = __v939 + (1 as i32);
            i = __v940;
        }
        eDest = if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32) {
            12 as i32
        } else {
            13 as i32
        };
    } else {
        if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (2 as i32) {
            i = 0 as i32;
            '__slate_break_796: loop {
                if !(i < ((unsafe { (*pTab).nCol }) as i32)) {
                    break;
                }
                pList = unsafe { sqlite3ExprListAppend(pParse, pList, exprRowColumn(pParse, i)) };
                let __v941: i32 = i;
                let __v942: i32 = __v941 + (1 as i32);
                i = __v942;
            }
            eDest = 12 as i32;
        } else {
            eDest = if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32) {
                12 as i32
            } else {
                13 as i32
            };
            pList = unsafe {
                sqlite3ExprListAppend(pParse, std::ptr::null_mut::<ExprList>(), unsafe {
                    sqlite3PExpr(
                        pParse,
                        76 as i32,
                        std::ptr::null_mut::<Expr>(),
                        std::ptr::null_mut::<Expr>(),
                    )
                })
            };
        }
    }
    0 as i32;
    if pChanges != std::ptr::null_mut::<ExprList>() {
        i = 0 as i32;
        '__slate_break_797: loop {
            if !(i < unsafe { (*pChanges).nExpr }) {
                break;
            }
            pList = unsafe {
                sqlite3ExprListAppend(pParse, pList, unsafe {
                    sqlite3ExprDup(
                        db,
                        (unsafe {
                            (*unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*pChanges).a) as *mut ExprList_item
                                }
                                .offset(i as isize)
                            })
                            .pExpr
                        }) as *const Expr,
                        0 as i32,
                    )
                })
            };
            let __v943: i32 = i;
            let __v944: i32 = __v943 + (1 as i32);
            i = __v944;
        }
    }
    pSelect = unsafe {
        sqlite3SelectNew(
            pParse,
            pList,
            pSrc,
            pWhere2,
            pGrp,
            std::ptr::null_mut::<Expr>(),
            pOrderBy2,
            ((8388608 as i32) | (131072 as i32) | (268435456 as i32)) as u32,
            pLimit2,
        )
    };
    if pSelect != std::ptr::null_mut::<Select>() {
        let __v945: *mut Select = pSelect;
        let __v946: u32 = unsafe { (*__v945).selFlags };
        let __v947: u32 = __v946 | ((134217728 as i32) as u32);
        unsafe {
            (*__v945).selFlags = __v947;
        }
    }
    unsafe { sqlite3SelectDestInit(std::ptr::addr_of_mut!(dest), eDest, iEph) };
    dest.iSDParm2 = if pPk != std::ptr::null_mut::<Index>() {
        ((unsafe { (*pPk).nKeyCol }) as u32) as i32
    } else {
        -(1 as i32)
    };
    unsafe { sqlite3Select(pParse, pSelect, std::ptr::addr_of_mut!(dest)) };
    unsafe { sqlite3SelectDelete(db, pSelect) };
}
