unsafe extern "C" {
    static mut sqlite3OpcodeProperty: [u8; 0];
    static mut sqlite3StrBINARY: [i8; 0];
    fn sqlite3_stricmp(__v798: *const i8, __v799: *const i8) -> i32;
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3VdbeAddOp0(__v806: *mut Vdbe, __v807: i32) -> i32;
    fn sqlite3VdbeAddOp1(__v808: *mut Vdbe, __v809: i32, __v810: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v811: *mut Vdbe, __v812: i32, __v813: i32, __v814: i32) -> i32;
    fn sqlite3VdbeGoto(__v815: *mut Vdbe, __v816: i32) -> i32;
    fn sqlite3VdbeLoadString(__v817: *mut Vdbe, __v818: i32, __v819: *const i8) -> i32;
    fn sqlite3VdbeAddOp3(
        __v820: *mut Vdbe,
        __v821: i32,
        __v822: i32,
        __v823: i32,
        __v824: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4(
        __v825: *mut Vdbe,
        __v826: i32,
        __v827: i32,
        __v828: i32,
        __v829: i32,
        zP4: *const i8,
        __v831: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4Int(
        __v832: *mut Vdbe,
        __v833: i32,
        __v834: i32,
        __v835: i32,
        __v836: i32,
        __v837: i32,
    ) -> i32;
    fn sqlite3VdbeEndCoroutine(__v838: *mut Vdbe, __v839: i32);
    fn sqlite3VdbeAddOpList(
        __v840: *mut Vdbe,
        nOp: i32,
        aOp: *const VdbeOpList,
        iLineno: i32,
    ) -> *mut VdbeOp;
    fn sqlite3VdbeExplain(__v844: *mut Parse, __v845: u8, __v846: *const i8, ...) -> i32;
    fn sqlite3VdbeChangeP5(__v847: *mut Vdbe, P5: u16);
    fn sqlite3VdbeJumpHere(__v849: *mut Vdbe, addr: i32);
    fn sqlite3VdbeChangeP4(__v851: *mut Vdbe, addr: i32, zP4: *const i8, N: i32);
    fn sqlite3VdbeAppendP4(__v855: *mut Vdbe, pP4: *mut (), p4type: i32);
    fn sqlite3VdbeSetP4KeyInfo(__v858: *mut Parse, __v859: *mut Index);
    fn sqlite3VdbeGetOp(__v860: *mut Vdbe, __v861: i32) -> *mut VdbeOp;
    fn sqlite3VdbeGetLastOp(__v862: *mut Vdbe) -> *mut VdbeOp;
    fn sqlite3VdbeMakeLabel(__v863: *mut Parse) -> i32;
    fn sqlite3VdbeResolveLabel(__v864: *mut Vdbe, __v865: i32);
    fn sqlite3VdbeCurrentAddr(__v866: *mut Vdbe) -> i32;
    fn sqlite3VdbeCountChanges(__v867: *mut Vdbe);
    fn sqlite3VdbeDb(__v868: *mut Vdbe) -> *mut sqlite3;
    fn sqlite3VdbeHasSubProgram(__v869: *mut Vdbe) -> i32;
    fn sqlite3WalkExpr(__v870: *mut Walker, __v871: *mut Expr) -> i32;
    fn sqlite3WalkExprList(__v872: *mut Walker, __v873: *mut ExprList) -> i32;
    fn sqlite3DbMallocZero(__v879: *mut sqlite3, __v880: u64) -> *mut ();
    fn sqlite3DbMallocRaw(__v881: *mut sqlite3, __v882: u64) -> *mut ();
    fn sqlite3DbMallocRawNN(__v883: *mut sqlite3, __v884: u64) -> *mut ();
    fn sqlite3DbFree(__v885: *mut sqlite3, __v886: *mut ());
    fn sqlite3DbNNFreeNN(__v887: *mut sqlite3, __v888: *mut ());
    fn sqlite3MPrintf(__v889: *mut sqlite3, __v890: *const i8, ...) -> *mut i8;
    fn sqlite3ErrorMsg(__v891: *mut Parse, __v892: *const i8, ...);
    fn sqlite3GetTempReg(__v893: *mut Parse) -> i32;
    fn sqlite3ReleaseTempReg(__v894: *mut Parse, __v895: i32);
    fn sqlite3GetTempRange(__v896: *mut Parse, __v897: i32) -> i32;
    fn sqlite3ReleaseTempRange(__v898: *mut Parse, __v899: i32, __v900: i32);
    fn sqlite3ExprDelete(__v901: *mut sqlite3, __v902: *mut Expr);
    fn sqlite3ExprListDelete(__v903: *mut sqlite3, __v904: *mut ExprList);
    fn sqlite3ColumnExpr(__v905: *mut Table, __v906: *mut Column) -> *mut Expr;
    fn sqlite3ColumnColl(__v907: *mut Column) -> *const i8;
    fn sqlite3PrimaryKeyIndex(__v908: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToIndex(__v909: *mut Index, __v910: i32) -> i32;
    fn sqlite3TableColumnToStorage(__v911: *mut Table, __v912: i16) -> i16;
    fn sqlite3FaultSim(__v913: i32) -> i32;
    fn sqlite3ViewGetColumnNames(__v914: *mut Parse, __v915: *mut Table) -> i32;
    fn sqlite3SrcItemAttachSubquery(
        __v927: *mut Parse,
        __v928: *mut SrcItem,
        __v929: *mut Select,
        __v930: i32,
    ) -> i32;
    fn sqlite3IdListDelete(__v931: *mut sqlite3, __v932: *mut IdList);
    fn sqlite3SrcListDelete(__v933: *mut sqlite3, __v934: *mut SrcList);
    fn sqlite3Select(__v935: *mut Parse, __v936: *mut Select, __v937: *mut SelectDest) -> i32;
    fn sqlite3SelectNew(
        __v938: *mut Parse,
        __v939: *mut ExprList,
        __v940: *mut SrcList,
        __v941: *mut Expr,
        __v942: *mut ExprList,
        __v943: *mut Expr,
        __v944: *mut ExprList,
        __v945: u32,
        __v946: *mut Expr,
    ) -> *mut Select;
    fn sqlite3SelectDelete(__v947: *mut sqlite3, __v948: *mut Select);
    fn sqlite3SrcListLookup(__v949: *mut Parse, __v950: *mut SrcList) -> *mut Table;
    fn sqlite3IsReadOnly(__v951: *mut Parse, __v952: *mut Table, __v953: *mut Trigger) -> i32;
    fn sqlite3CodeChangeCount(__v959: *mut Vdbe, __v960: i32, __v961: *const i8);
    fn sqlite3ExprCode(__v962: *mut Parse, __v963: *mut Expr, __v964: i32);
    fn sqlite3ExprCodeGeneratedColumn(
        __v965: *mut Parse,
        __v966: *mut Table,
        __v967: *mut Column,
        __v968: i32,
    );
    fn sqlite3ExprCodeCopy(__v969: *mut Parse, __v970: *mut Expr, __v971: i32);
    fn sqlite3ExprCodeFactorable(__v972: *mut Parse, __v973: *mut Expr, __v974: i32);
    fn sqlite3ExprCodeTarget(__v975: *mut Parse, __v976: *mut Expr, __v977: i32) -> i32;
    fn sqlite3ExprCodeExprList(
        __v978: *mut Parse,
        __v979: *mut ExprList,
        __v980: i32,
        __v981: i32,
        __v982: u8,
    ) -> i32;
    fn sqlite3ExprIfTrue(__v983: *mut Parse, __v984: *mut Expr, __v985: i32, __v986: i32);
    fn sqlite3ExprIfFalseDup(__v987: *mut Parse, __v988: *mut Expr, __v989: i32, __v990: i32);
    fn sqlite3LocateTableItem(__v991: *mut Parse, flags: u32, __v993: *mut SrcItem) -> *mut Table;
    fn sqlite3ExprCompare(
        __v994: *const Parse,
        __v995: *const Expr,
        __v996: *const Expr,
        __v997: i32,
    ) -> i32;
    fn sqlite3ExprListCompare(
        __v998: *const ExprList,
        __v999: *const ExprList,
        __v1000: i32,
    ) -> i32;
    fn sqlite3GetVdbe(__v1001: *mut Parse) -> *mut Vdbe;
    fn sqlite3CodeVerifySchema(__v1002: *mut Parse, __v1003: i32);
    fn sqlite3ExprListIsConstant(pParse: *mut Parse, pList: *mut ExprList, bNoIs: i32) -> i32;
    fn sqlite3IsRowid(__v1007: *const i8) -> i32;
    fn sqlite3GenerateRowDelete(
        __v1008: *mut Parse,
        __v1009: *mut Table,
        __v1010: *mut Trigger,
        __v1011: i32,
        __v1012: i32,
        __v1013: i32,
        __v1014: i16,
        __v1015: u8,
        __v1016: u8,
        __v1017: u8,
        __v1018: i32,
    );
    fn sqlite3GenerateRowIndexDelete(
        __v1019: *mut Parse,
        __v1020: *mut Table,
        __v1021: i32,
        __v1022: i32,
        __v1023: *mut i32,
        __v1024: i32,
    );
    fn sqlite3BeginWriteOperation(__v1058: *mut Parse, __v1059: i32, __v1060: i32);
    fn sqlite3MultiWrite(__v1061: *mut Parse);
    fn sqlite3MayAbort(__v1062: *mut Parse);
    fn sqlite3HaltConstraint(
        __v1063: *mut Parse,
        __v1064: i32,
        __v1065: i32,
        __v1066: *mut i8,
        __v1067: i8,
        __v1068: u8,
    );
    fn sqlite3UniqueConstraint(__v1069: *mut Parse, __v1070: i32, __v1071: *mut Index);
    fn sqlite3RowidConstraint(__v1072: *mut Parse, __v1073: i32, __v1074: *mut Table);
    fn sqlite3ExprDup(__v1075: *mut sqlite3, __v1076: *const Expr, __v1077: i32) -> *mut Expr;
    fn sqlite3TriggersExist(
        __v1078: *mut Parse,
        __v1079: *mut Table,
        __v1080: i32,
        __v1081: *mut ExprList,
        pMask: *mut i32,
    ) -> *mut Trigger;
    fn sqlite3CodeRowTrigger(
        __v1083: *mut Parse,
        __v1084: *mut Trigger,
        __v1085: i32,
        __v1086: *mut ExprList,
        __v1087: i32,
        __v1088: *mut Table,
        __v1089: i32,
        __v1090: i32,
        __v1091: i32,
    );
    fn sqlite3ColumnIndex(pTab: *mut Table, zCol: *const i8) -> i32;
    fn sqlite3AuthCheck(
        __v1094: *mut Parse,
        __v1095: i32,
        __v1096: *const i8,
        __v1097: *const i8,
        __v1098: *const i8,
    ) -> i32;
    fn sqlite3AuthReadCol(
        __v1099: *mut Parse,
        __v1100: *const i8,
        __v1101: *const i8,
        __v1102: i32,
    ) -> i32;
    fn sqlite3ExprAffinity(pExpr: *const Expr) -> i8;
    fn sqlite3ReadSchema(pParse: *mut Parse) -> i32;
    fn sqlite3LocateCollSeq(pParse: *mut Parse, zName: *const i8) -> *mut CollSeq;
    fn sqlite3SelectWrongNumTermsError(pParse: *mut Parse, p: *mut Select);
    fn sqlite3ResolveExprListNames(__v1116: *mut NameContext, __v1117: *mut ExprList) -> i32;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, __v1119: *mut Schema) -> i32;
    fn sqlite3HasExplicitNulls(__v1120: *mut Parse, __v1121: *mut ExprList) -> i32;
    fn sqlite3OomFault(__v1122: *mut sqlite3) -> *mut ();
    fn sqlite3SelectDestInit(__v1123: *mut SelectDest, __v1124: i32, __v1125: i32);
    fn sqlite3TableLock(
        __v1126: *mut Parse,
        __v1127: i32,
        __v1128: u32,
        __v1129: u8,
        __v1130: *const i8,
    );
    fn sqlite3GetVTable(__v1131: *mut sqlite3, __v1132: *mut Table) -> *mut VTable;
    fn sqlite3VtabMakeWritable(__v1133: *mut Parse, __v1134: *mut Table);
    fn sqlite3ParserAddCleanup(
        __v1135: *mut Parse,
        __v1136: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
        __v1137: *mut (),
    ) -> *mut ();
    fn sqlite3UpsertDelete(__v1138: *mut sqlite3, __v1139: *mut Upsert);
    fn sqlite3UpsertAnalyzeTarget(
        __v1140: *mut Parse,
        __v1141: *mut SrcList,
        __v1142: *mut Upsert,
        __v1143: *mut Upsert,
    ) -> i32;
    fn sqlite3UpsertDoUpdate(
        __v1144: *mut Parse,
        __v1145: *mut Upsert,
        __v1146: *mut Table,
        __v1147: *mut Index,
        __v1148: i32,
    );
    fn sqlite3UpsertOfIndex(__v1149: *mut Upsert, __v1150: *mut Index) -> *mut Upsert;
    fn sqlite3UpsertNextIsIPK(__v1151: *mut Upsert) -> i32;
    fn sqlite3FkCheck(
        __v1152: *mut Parse,
        __v1153: *mut Table,
        __v1154: i32,
        __v1155: i32,
        __v1156: *mut i32,
        __v1157: i32,
    );
    fn sqlite3FkRequired(
        __v1158: *mut Parse,
        __v1159: *mut Table,
        __v1160: *mut i32,
        __v1161: i32,
    ) -> i32;
    fn sqlite3FkReferences(__v1162: *mut Table) -> *mut FKey;
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
    trace: __SlateRecord168,
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
    u1: __SlateRecord169,
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
    __slate_bits_0: __slate_bits::__SlateBits65U0,
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
    u: __SlateRecord179,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord180,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord181,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord182,
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
    u: __SlateRecord170,
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
    __slate_bits_0: __slate_bits::__SlateBits91U0,
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
    uNC: __SlateRecord194,
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
    u1: __SlateRecord196,
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
    fg: __SlateRecord189,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord190,
    u2: __SlateRecord191,
    u3: __SlateRecord192,
    u4: __SlateRecord193,
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
    u: __SlateRecord171,
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
    u: __SlateRecord199,
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
struct VdbeOpList {
    opcode: u8,
    p1: i8,
    p2: i8,
    p3: i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3InitInfo {
    newTnum: u32,
    iDb: u8,
    busy: u8,
    __slate_bits_0: __slate_bits::__SlateBits167U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord168 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord169 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord170 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord171 {
    tab: __SlateRecord172,
    view: __SlateRecord173,
    vtab: __SlateRecord174,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord172 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord173 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord174 {
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
union __SlateRecord179 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord183,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord183 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord185,
    u: __SlateRecord186,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord185 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits185U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    x: __SlateRecord187,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord187 {
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
struct __SlateRecord189 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits189U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord190 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord192 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord193 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord194 {
    pEList: *mut ExprList,
    pAggInfo: *mut AggInfo,
    pUpsert: *mut Upsert,
    iBaseReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord196 {
    cr: __SlateRecord197,
    d: __SlateRecord198,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord197 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord198 {
    pReturning: *mut Returning,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord199 {
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
// ** to handle INSERT statements in SQLite.
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

#[repr(C)]
#[derive(Clone, Copy)]
struct CoveringIndexCheck {}

#[repr(C)]
#[derive(Clone, Copy)]
struct CheckOnCtx {}

// /* The expression to be checked */
// /* aiChng[x]>=0 if column x changed by the UPDATE */
// /* True if UPDATE changes the rowid */
// /*
// ** The sqlite3GenerateConstraintChecks() routine usually wants to visit
// ** the indexes of a table in the order provided in the Table->pIndex list.
// ** However, sometimes (rarely - when there is an upsert) it wants to visit
// ** the indexes in a different order.  The following data structures accomplish
// ** this.
// **
// ** The IndexIterator object is used to walk through all of the indexes
// ** of a table in either Index.pNext order, or in some other order established
// ** by an array of IndexListTerm objects.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct IndexListTerm {
    // /* When IndexIterator.eType==1, then each index is an array of instances
    // ** of the following object
    // */
    // /* Size of the array */
    // /* Array of IndexListTerms */
    // /* Index of the current item from the list */
    p: *mut Index,
    // /* The index */
    ix: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct IndexIterator {
    eType: i32,
    // /* 0 for Index.pNext list.  1 for an array of IndexListTerm */
    i: i32,
    u: __SlateRecord212,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord212 {
    lx: __SlateRecord213,
    ax: __SlateRecord214,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord213 {
    // /* Use this object for eType==0: A Index.pNext list */
    pIdx: *mut Index,
    // /* The current Index */
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord214 {
    // /* Use this object for eType==1; Array of IndexListTerm */
    nIdx: i32,
    aIdx: *mut IndexListTerm,
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
    pub struct __SlateBits65U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits185U0 {
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
    pub struct __SlateBits189U0 {
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
    pub struct __SlateBits91U0 {
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
    pub struct __SlateBits167U0 {
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
}

static mut iLn_523: i32 = 0 as i32;

static mut autoInc: __SlateAlign16<[VdbeOpList; 12]> = __SlateAlign16([
    VdbeOpList {
        opcode: ((77 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((36 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (10 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((96 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((53 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (9 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((137 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((96 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (1 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((88 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((82 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((9 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (11 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((40 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (2 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((73 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((124 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
]);

static mut iLn_535: i32 = 0 as i32;

static mut autoIncEnd: __SlateAlign16<[VdbeOpList; 5]> = __SlateAlign16([
    VdbeOpList {
        opcode: ((52 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (2 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((129 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((99 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (2 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((130 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((124 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
]);

// /*
// ** This function is called by the parser for the second and subsequent
// ** rows of a multi-row VALUES clause. Argument pLeft is the part of
// ** the VALUES clause already parsed, argument pRow is the vector of values
// ** for the new row. The Select object returned represents the complete
// ** VALUES clause, including the new row.
// **
// ** There are two ways in which this may be achieved - by incremental
// ** coding of a co-routine (the "co-routine" method) or by returning a
// ** Select object equivalent to the following (the "UNION ALL" method):
// **
// **        "pLeft UNION ALL SELECT pRow"
// **
// ** If the VALUES clause contains a lot of rows, this compound Select
// ** object may consume a lot of memory.
// **
// ** When the co-routine method is used, each row that will be returned
// ** by the VALUES clause is coded into part of a co-routine as it is
// ** passed to this function. The returned Select object is equivalent to:
// **
// **     SELECT * FROM (
// **       Select object to read co-routine
// **     )
// **
// ** The co-routine method is used in most cases. Exceptions are:
// **
// **    a) If the current statement has a WITH clause. This is to avoid
// **       statements like:
// **
// **            WITH cte AS ( VALUES('x'), ('y') ... )
// **            SELECT * FROM cte AS a, cte AS b;
// **
// **       This will not work, as the co-routine uses a hard-coded register
// **       for its OP_Yield instructions, and so it is not possible for two
// **       cursors to iterate through it concurrently.
// **
// **    b) The schema is currently being parsed (i.e. the VALUES clause is part
// **       of a schema item like a VIEW or TRIGGER). In this case there is no VM
// **       being generated when parsing is taking place, and so generating
// **       a co-routine is not possible.
// **
// **    c) There are non-constant expressions in the VALUES clause (e.g.
// **       the VALUES clause is part of a correlated sub-query).
// **
// **    d) One or more of the values in the first row of the VALUES clause
// **       has an affinity (i.e. is a CAST expression). This causes problems
// **       because the complex rules SQLite uses (see function
// **       sqlite3SubqueryColumnTypes() in select.c) to determine the effective
// **       affinity of such a column for all rows require access to all values in
// **       the column simultaneously.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MultiValues(
    mut pParse: *mut Parse,
    mut pLeft: *mut Select,
    mut pRow: *mut ExprList,
) -> *mut Select {
    // /* condition (a) above */
    let __v1224: bool;
    if ((unsafe { (*pParse).__slate_bits_0.__get_bHasWith() }) as i32) != (0 as i32)
        || (unsafe { (*unsafe { (*pParse).db }).init.busy }) != (0 as u8)
    {
        __v1224 = true as bool;
    } else {
        __v1224 = (unsafe { sqlite3ExprListIsConstant(pParse, pRow, 1 as i32) }) == (0 as i32);
    }
    let __v1225: bool;
    if __v1224 {
        __v1225 = true as bool;
    } else {
        let __v1226: bool;
        if (unsafe { (*unsafe { (*pLeft).pSrc }).nSrc }) == (0 as i32) {
            __v1226 = exprListIsNoAffinity(pParse, unsafe { (*pLeft).pEList }) == (0 as i32);
        } else {
            __v1226 = false as bool;
        }
        __v1225 = __v1226;
    }
    if __v1225 || (((unsafe { (*pParse).eParseMode }) as u32) as i32) != (0 as i32) {
        // /* The co-routine method cannot be used. Fall back to UNION ALL. */
        let mut pSelect: *mut Select = std::ptr::null_mut::<Select>();
        let mut f: i32 = (512 as i32) | (1024 as i32);
        if (unsafe { (*unsafe { (*pLeft).pSrc }).nSrc }) != (0 as i32) {
            sqlite3MultiValuesEnd(pParse, pLeft);
            f = 512 as i32;
        } else {
            if (unsafe { (*pLeft).pPrior }) != std::ptr::null_mut::<Select>() {
                // /* In this case set the SF_MultiValue flag only if it was set on pLeft */
                f = ((f as u32) & unsafe { (*pLeft).selFlags }) as i32;
            }
        }
        pSelect = unsafe {
            sqlite3SelectNew(
                pParse,
                pRow,
                std::ptr::null_mut::<SrcList>(),
                std::ptr::null_mut::<Expr>(),
                std::ptr::null_mut::<ExprList>(),
                std::ptr::null_mut::<Expr>(),
                std::ptr::null_mut::<ExprList>(),
                f as u32,
                std::ptr::null_mut::<Expr>(),
            )
        };
        let __v1227: *mut Select = pLeft;
        let __v1228: u32 = unsafe { (*__v1227).selFlags };
        let __v1229: u32 = __v1228 & !((1024 as i32) as u32);
        unsafe {
            (*__v1227).selFlags = __v1229;
        }
        if pSelect != std::ptr::null_mut::<Select>() {
            unsafe {
                (*pSelect).op = ((136 as i32) as i8) as u8;
            }
            unsafe {
                (*pSelect).pPrior = pLeft;
            }
            pLeft = pSelect;
        }
    } else {
        // /* SrcItem that reads from co-routine */
        let mut p: *mut SrcItem = std::ptr::null_mut::<SrcItem>();
        if (unsafe { (*unsafe { (*pLeft).pSrc }).nSrc }) == (0 as i32) {
            // /* Co-routine has not yet been started and the special Select object
            //       ** that accesses the co-routine has not yet been created. This block
            //       ** does both those things. */
            let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
            let mut pRet: *mut Select = unsafe {
                sqlite3SelectNew(
                    pParse,
                    std::ptr::null_mut::<ExprList>(),
                    std::ptr::null_mut::<SrcList>(),
                    std::ptr::null_mut::<Expr>(),
                    std::ptr::null_mut::<ExprList>(),
                    std::ptr::null_mut::<Expr>(),
                    std::ptr::null_mut::<ExprList>(),
                    (0 as i32) as u32,
                    std::ptr::null_mut::<Expr>(),
                )
            };
            // /* Ensure the database schema has been read. This is to ensure we have
            //       ** the correct text encoding.  */
            if (unsafe { (*unsafe { (*pParse).db }).mDbFlags }) & ((16 as i32) as u32)
                == ((0 as i32) as u32)
            {
                unsafe { sqlite3ReadSchema(pParse) };
            }
            if pRet != std::ptr::null_mut::<Select>() {
                let mut dest: SelectDest = unsafe { std::mem::zeroed() };
                let mut pSubq: *mut Subquery = unsafe { std::mem::zeroed() };
                unsafe {
                    (*unsafe { (*pRet).pSrc }).nSrc = 1 as i32;
                }
                unsafe {
                    (*pRet).pPrior = unsafe { (*pLeft).pPrior };
                }
                unsafe {
                    (*pRet).op = unsafe { (*pLeft).op };
                }
                if (unsafe { (*pRet).pPrior }) != std::ptr::null_mut::<Select>() {
                    let __v1230: *mut Select = pRet;
                    let __v1231: u32 = unsafe { (*__v1230).selFlags };
                    let __v1232: u32 = __v1231 | ((512 as i32) as u32);
                    unsafe {
                        (*__v1230).selFlags = __v1232;
                    }
                }
                unsafe {
                    (*pLeft).pPrior = std::ptr::null_mut::<Select>();
                }
                unsafe {
                    (*pLeft).op = ((139 as i32) as i8) as u8;
                }
                0 as i32;
                0 as i32;
                p = unsafe {
                    unsafe { std::ptr::addr_of_mut!((*unsafe { (*pRet).pSrc }).a) as *mut SrcItem }
                        .offset((0 as i32) as isize)
                };
                unsafe {
                    (*p).fg.__slate_bits_0.__set_viaCoroutine((1 as i32) as u32);
                }
                unsafe {
                    (*p).iCursor = -(1 as i32);
                }
                0 as i32;
                unsafe {
                    (*p).u1.nRow = (2 as i32) as u32;
                }
                if (unsafe { sqlite3SrcItemAttachSubquery(pParse, p, pLeft, 0 as i32) })
                    != (0 as i32)
                {
                    pSubq = unsafe { (*p).u4.pSubq };
                    unsafe {
                        (*pSubq).addrFillSub = (unsafe { sqlite3VdbeCurrentAddr(v) }) + (1 as i32);
                    }
                    let __v1233: *mut Parse = pParse;
                    let __v1234: i32 = unsafe { (*__v1233).nMem };
                    let __v1235: i32 = __v1234 + (1 as i32);
                    unsafe {
                        (*__v1233).nMem = __v1235;
                    }
                    unsafe {
                        (*pSubq).regReturn = __v1235;
                    }
                    unsafe {
                        sqlite3VdbeAddOp3(
                            v,
                            11 as i32,
                            unsafe { (*pSubq).regReturn },
                            0 as i32,
                            unsafe { (*pSubq).addrFillSub },
                        )
                    };
                    unsafe {
                        sqlite3SelectDestInit(std::ptr::addr_of_mut!(dest), 11 as i32, unsafe {
                            (*pSubq).regReturn
                        })
                    };
                    // /* Allocate registers for the output of the co-routine. Do so so
                    //           ** that there are two unused registers immediately before those
                    //           ** used by the co-routine. This allows the code in sqlite3Insert()
                    //           ** to use these registers directly, instead of copying the output
                    //           ** of the co-routine to a separate array for processing.  */
                    dest.iSdst = (unsafe { (*pParse).nMem }) + (3 as i32);
                    dest.nSdst = unsafe { (*unsafe { (*pLeft).pEList }).nExpr };
                    let __v1236: *mut Parse = pParse;
                    let __v1237: i32 = unsafe { (*__v1236).nMem };
                    let __v1238: i32 = __v1237 + ((2 as i32) + dest.nSdst);
                    unsafe {
                        (*__v1236).nMem = __v1238;
                    }
                    let __v1239: *mut Select = pLeft;
                    let __v1240: u32 = unsafe { (*__v1239).selFlags };
                    let __v1241: u32 = __v1240 | ((1024 as i32) as u32);
                    unsafe {
                        (*__v1239).selFlags = __v1241;
                    }
                    unsafe { sqlite3Select(pParse, pLeft, std::ptr::addr_of_mut!(dest)) };
                    unsafe {
                        (*pSubq).regResult = dest.iSdst;
                    }
                    0 as i32;
                }
                pLeft = pRet;
            }
        } else {
            p = unsafe {
                unsafe { std::ptr::addr_of_mut!((*unsafe { (*pLeft).pSrc }).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            };
            0 as i32;
            let __v1242: *mut SrcItem = p;
            let __v1243: u32 = unsafe { (*__v1242).u1.nRow };
            let __v1244: u32 = __v1243.wrapping_add((1 as i32) as u32);
            unsafe {
                (*__v1242).u1.nRow = __v1244;
            }
        }
        if (unsafe { (*pParse).nErr }) == (0 as i32) {
            let mut pSubq: *mut Subquery = unsafe { std::mem::zeroed() };
            0 as i32;
            0 as i32;
            pSubq = unsafe { (*p).u4.pSubq };
            0 as i32;
            0 as i32;
            0 as i32;
            if (unsafe { (*unsafe { (*unsafe { (*pSubq).pSelect }).pEList }).nExpr })
                != unsafe { (*pRow).nExpr }
            {
                unsafe { sqlite3SelectWrongNumTermsError(pParse, unsafe { (*pSubq).pSelect }) };
            } else {
                unsafe {
                    sqlite3ExprCodeExprList(
                        pParse,
                        pRow,
                        unsafe { (*pSubq).regResult },
                        0 as i32,
                        ((0 as i32) as i8) as u8,
                    )
                };
                unsafe {
                    sqlite3VdbeAddOp1(unsafe { (*pParse).pVdbe }, 12 as i32, unsafe {
                        (*pSubq).regReturn
                    })
                };
            }
        }
        unsafe { sqlite3ExprListDelete(unsafe { (*pParse).db }, pRow) };
    }
    // /* condition (b) above */
    // /* condition (c) above */
    // /* condition (d) above */
    return pLeft;
}

// /*
// ** If argument pVal is a Select object returned by an sqlite3MultiValues()
// ** that was able to use the co-routine optimization, finish coding the
// ** co-routine.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MultiValuesEnd(mut pParse: *mut Parse, mut pVal: *mut Select) {
    if pVal != std::ptr::null_mut::<Select>()
        && (unsafe { (*unsafe { (*pVal).pSrc }).nSrc }) > (0 as i32)
    {
        let mut pItem: *mut SrcItem = unsafe {
            unsafe { std::ptr::addr_of_mut!((*unsafe { (*pVal).pSrc }).a) as *mut SrcItem }
                .offset((0 as i32) as isize)
        };
        0 as i32;
        if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isSubquery() }) as i32) != (0 as i32) {
            unsafe {
                sqlite3VdbeEndCoroutine(unsafe { (*pParse).pVdbe }, unsafe {
                    (*unsafe { (*pItem).u4.pSubq }).regReturn
                })
            };
            unsafe {
                sqlite3VdbeJumpHere(
                    unsafe { (*pParse).pVdbe },
                    (unsafe { (*unsafe { (*pItem).u4.pSubq }).addrFillSub }) - (1 as i32),
                )
            };
        }
    }
}

// /* Parsing context */
// /* Index of the database holding pTab */
// /* The table we are writing to */
// /*
// ** This routine generates code that will initialize all of the
// ** register used by the autoincrement tracker.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AutoincrementBegin(mut pParse: *mut Parse) {
    // /* Information about an AUTOINCREMENT */
    let mut p: *mut AutoincInfo = unsafe { std::mem::zeroed() };
    // /* The database connection */
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    // /* Database only autoinc table */
    let mut pDb: *mut Db = unsafe { std::mem::zeroed() };
    // /* Register holding max rowid */
    let mut memId: i32 = 0 as i32;
    // /* VDBE under construction */
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    // /* This routine is never called during trigger-generation.  It is
    //   ** only called from the top-level */
    0 as i32;
    0 as i32;
    // /* We failed long ago if this is not so */
    0 as i32;
    0 as i32;
    p = unsafe { (*pParse).pAinc };
    '__slate_break_1174: while p != std::ptr::null_mut::<AutoincInfo>() {
        // /* 0  */
        // /* 1  */
        // /* 2  */
        // /* 3  */
        // /* 4  */
        // /* 5  */
        // /* 6  */
        // /* 7  */
        // /* 8  */
        // /* 9  */
        // /* 10 */
        // /* 11 */
        let mut aOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
        pDb = unsafe { unsafe { (*db).aDb }.offset((unsafe { (*p).iDb }) as isize) };
        memId = unsafe { (*p).regCtr };
        0 as i32;
        sqlite3OpenTable(
            pParse,
            0 as i32,
            unsafe { (*p).iDb },
            unsafe { (*unsafe { (*pDb).pSchema }).pSeqTab },
            114 as i32,
        );
        unsafe {
            sqlite3VdbeLoadString(
                v,
                memId - (1 as i32),
                (unsafe { (*unsafe { (*p).pTab }).zName }) as *const i8,
            )
        };
        aOp = unsafe {
            sqlite3VdbeAddOpList(
                v,
                (((48 as u64) / (4 as u64)) as u32) as i32,
                unsafe { std::ptr::addr_of!(autoInc.0) as *const VdbeOpList },
                unsafe { iLn_523 },
            )
        };
        if aOp == std::ptr::null_mut::<VdbeOp>() {
            break '__slate_break_1174;
        }
        unsafe {
            (*unsafe { aOp.offset((0 as i32) as isize) }).p2 = memId;
        }
        unsafe {
            (*unsafe { aOp.offset((0 as i32) as isize) }).p3 = memId + (2 as i32);
        }
        unsafe {
            (*unsafe { aOp.offset((2 as i32) as isize) }).p3 = memId;
        }
        unsafe {
            (*unsafe { aOp.offset((3 as i32) as isize) }).p1 = memId - (1 as i32);
        }
        unsafe {
            (*unsafe { aOp.offset((3 as i32) as isize) }).p3 = memId;
        }
        unsafe {
            (*unsafe { aOp.offset((3 as i32) as isize) }).p5 = ((16 as i32) as i16) as u16;
        }
        unsafe {
            (*unsafe { aOp.offset((4 as i32) as isize) }).p2 = memId + (1 as i32);
        }
        unsafe {
            (*unsafe { aOp.offset((5 as i32) as isize) }).p3 = memId;
        }
        unsafe {
            (*unsafe { aOp.offset((6 as i32) as isize) }).p1 = memId;
        }
        unsafe {
            (*unsafe { aOp.offset((7 as i32) as isize) }).p2 = memId + (2 as i32);
        }
        unsafe {
            (*unsafe { aOp.offset((7 as i32) as isize) }).p1 = memId;
        }
        unsafe {
            (*unsafe { aOp.offset((10 as i32) as isize) }).p2 = memId;
        }
        if (unsafe { (*pParse).nTab }) == (0 as i32) {
            unsafe {
                (*pParse).nTab = 1 as i32;
            }
        }
        p = unsafe { (*p).pNext };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AutoincrementEnd(mut pParse: *mut Parse) {
    if ((unsafe { (*pParse).__slate_bits_0.__get_usesAinc() }) as i32) != (0 as i32) {
        autoIncrementEnd(pParse);
    }
}

// /* Forward declaration */
// /* Parser context */
// /* The table we are inserting into */
// /* A SELECT statement to use as the data source */
// /* How to handle constraint errors */
// /* The database of pDest */
// /*
// ** This routine is called to handle SQL of the following forms:
// **
// **    insert into TABLE (IDLIST) values(EXPRLIST),(EXPRLIST),...
// **    insert into TABLE (IDLIST) select
// **    insert into TABLE (IDLIST) default values
// **
// ** The IDLIST following the table name is always optional.  If omitted,
// ** then a list of all (non-hidden) columns for the table is substituted.
// ** The IDLIST appears in the pColumn parameter.  pColumn is NULL if IDLIST
// ** is omitted.
// **
// ** For the pSelect parameter holds the values to be inserted for the
// ** first two forms shown above.  A VALUES clause is really just short-hand
// ** for a SELECT statement that omits the FROM clause and everything else
// ** that follows.  If the pSelect parameter is NULL, that means that the
// ** DEFAULT VALUES form of the INSERT statement is intended.
// **
// ** The code generated follows one of four templates.  For a simple
// ** insert with data coming from a single-row VALUES clause, the code executes
// ** once straight down through.  Pseudo-code follows (we call this
// ** the "1st template"):
// **
// **         open write cursor to <table> and its indices
// **         put VALUES clause expressions into registers
// **         write the resulting record into <table>
// **         cleanup
// **
// ** The three remaining templates assume the statement is of the form
// **
// **   INSERT INTO <table> SELECT ...
// **
// ** If the SELECT clause is of the restricted form "SELECT * FROM <table2>" -
// ** in other words if the SELECT pulls all columns from a single table
// ** and there is no WHERE or LIMIT or GROUP BY or ORDER BY clauses, and
// ** if <table2> and <table1> are distinct tables but have identical
// ** schemas, including all the same indices, then a special optimization
// ** is invoked that copies raw records from <table2> over to <table1>.
// ** See the xferOptimization() function for the implementation of this
// ** template.  This is the 2nd template.
// **
// **         open a write cursor to <table>
// **         open read cursor on <table2>
// **         transfer all records in <table2> over to <table>
// **         close cursors
// **         foreach index on <table>
// **           open a write cursor on the <table> index
// **           open a read cursor on the corresponding <table2> index
// **           transfer all records from the read to the write cursors
// **           close cursors
// **         end foreach
// **
// ** The 3rd template is for when the second template does not apply
// ** and the SELECT clause does not read from <table> at any time.
// ** The generated code follows this template:
// **
// **         X <- A
// **         goto B
// **      A: setup for the SELECT
// **         loop over the rows in the SELECT
// **           load values into registers R..R+n
// **           yield X
// **         end loop
// **         cleanup after the SELECT
// **         end-coroutine X
// **      B: open write cursor to <table> and its indices
// **      C: yield X, at EOF goto D
// **         insert the select result into <table> from R..R+n
// **         goto C
// **      D: cleanup
// **
// ** The 4th template is used if the insert statement takes its
// ** values from a SELECT but the data is being inserted into a table
// ** that is also read as part of the SELECT.  In the third form,
// ** we have to use an intermediate table to store the results of
// ** the select.  The template is like this:
// **
// **         X <- A
// **         goto B
// **      A: setup for the SELECT
// **         loop over the tables in the SELECT
// **           load value into register R..R+n
// **           yield X
// **         end loop
// **         cleanup after the SELECT
// **         end co-routine R
// **      B: open temp table
// **      L: yield X, at EOF goto M
// **         insert row from R..R+n into temp table
// **         goto L
// **      M: open write cursor to <table> and its indices
// **         rewind temp table
// **      C: loop over rows of intermediate table
// **           transfer values form intermediate table into <table>
// **         end loop
// **      D: cleanup
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Insert(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pSelect: *mut Select,
    mut pColumn: *mut IdList,
    mut onError: i32,
    mut pUpsert: *mut Upsert,
) {
    let mut __slate_storage_628: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_628: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_628) as *mut *const i8;
    let mut __slate_storage_1297: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1297: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1297) as *mut bool;
    let mut __slate_storage_630: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_630: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_630) as *mut i32;
    let mut __slate_storage_629: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_629: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_629) as *mut i32;
    let mut __slate_storage_627: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_627: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_627) as *mut i32;
    let mut __slate_storage_626: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_626: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_626) as *mut *mut Expr;
    let mut __slate_storage_625: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_625: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_625) as *mut i32;
    let mut __slate_storage_624: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_624: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_624) as *mut i32;
    let mut __slate_storage_1291: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1291: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1291) as *mut i32;
    let mut __slate_storage_1290: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1290: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1290) as *mut i32;
    let mut __slate_storage_1289: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1289: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1289) as *mut i32;
    let mut __slate_storage_1288: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1288: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1288) as *mut i32;
    let mut __slate_storage_623: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_623: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_623) as *mut i32;
    let mut __slate_storage_622: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_622: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_622) as *mut *mut Expr;
    let mut __slate_storage_1296: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1296: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1296) as *mut i32;
    let mut __slate_storage_1295: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1295: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1295) as *mut i32;
    let mut __slate_storage_1294: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1294: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1294) as *mut i32;
    let mut __slate_storage_1293: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1293: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1293) as *mut i32;
    let mut __slate_storage_1292: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1292: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1292) as *mut u32;
    let mut __slate_storage_621: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_621: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_621) as *mut u32;
    let mut __slate_storage_620: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_620: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_620) as *mut i32;
    let mut __slate_storage_1287: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1287: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1287) as *mut i32;
    let mut __slate_storage_619: std::mem::MaybeUninit<*mut Upsert> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_619: *mut *mut Upsert =
        std::ptr::addr_of_mut!(__slate_storage_619) as *mut *mut Upsert;
    let mut __slate_storage_1286: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1286: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1286) as *mut i32;
    let mut __slate_storage_1285: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1285: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1285) as *mut i32;
    let mut __slate_storage_1284: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1284: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1284) as *mut *mut Parse;
    let mut __slate_storage_1277: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1277: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1277) as *mut i32;
    let mut __slate_storage_1276: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1276: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1276) as *mut i32;
    let mut __slate_storage_1283: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1283: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1283) as *mut i32;
    let mut __slate_storage_1282: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1282: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1282) as *mut i32;
    let mut __slate_storage_1281: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1281: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1281) as *mut *mut Parse;
    let mut __slate_storage_1280: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1280: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1280) as *mut i32;
    let mut __slate_storage_1279: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1279: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1279) as *mut i32;
    let mut __slate_storage_1278: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1278: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1278) as *mut *mut Parse;
    let mut __slate_storage_1275: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1275: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1275) as *mut *mut Index;
    let mut __slate_storage_618: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_618: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_618) as *mut i32;
    let mut __slate_storage_1274: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1274: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1274) as *mut i32;
    let mut __slate_storage_1273: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1273: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1273) as *mut i32;
    let mut __slate_storage_1272: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1272: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1272) as *mut *mut Parse;
    let mut __slate_storage_1269: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1269: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1269) as *mut i32;
    let mut __slate_storage_1268: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1268: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1268) as *mut i32;
    let mut __slate_storage_1271: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1271: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1271) as *mut i32;
    let mut __slate_storage_1270: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1270: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1270) as *mut i32;
    let mut __slate_storage_1265: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1265: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1265) as *mut i32;
    let mut __slate_storage_1264: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1264: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1264) as *mut i32;
    let mut __slate_storage_1267: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1267: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1267) as *mut i32;
    let mut __slate_storage_1266: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1266: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1266) as *mut i32;
    let mut __slate_storage_1263: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1263: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1263) as *mut i32;
    let mut __slate_storage_1262: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1262: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1262) as *mut i32;
    let mut __slate_storage_1261: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1261: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1261) as *mut *mut Parse;
    let mut __slate_storage_616: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_616: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_616) as *mut i32;
    let mut __slate_storage_615: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_615: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_615) as *mut i32;
    let mut __slate_storage_614: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_614: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_614) as *mut i32;
    let mut __slate_storage_1260: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1260: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1260) as *mut bool;
    let mut __slate_storage_611: std::mem::MaybeUninit<*mut Subquery> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_611: *mut *mut Subquery =
        std::ptr::addr_of_mut!(__slate_storage_611) as *mut *mut Subquery;
    let mut __slate_storage_610: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_610: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_610) as *mut *mut SrcItem;
    let mut __slate_storage_1259: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1259: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1259) as *mut i32;
    let mut __slate_storage_1258: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1258: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1258) as *mut i32;
    let mut __slate_storage_1257: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1257: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1257) as *mut *mut Parse;
    let mut __slate_storage_613: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_613: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_613) as *mut i32;
    let mut __slate_storage_612: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_612: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_612) as *mut i32;
    let mut __slate_storage_609: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_609: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_609) as *mut i32;
    let mut __slate_storage_617: std::mem::MaybeUninit<NameContext> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_617: *mut NameContext =
        std::ptr::addr_of_mut!(__slate_storage_617) as *mut NameContext;
    let mut __slate_storage_1256: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1256: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1256) as *mut i32;
    let mut __slate_storage_1255: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1255: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1255) as *mut i32;
    let mut __slate_storage_1254: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1254: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1254) as *mut i32;
    let mut __slate_storage_1253: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1253: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1253) as *mut i32;
    let mut __slate_storage_1252: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1252: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1252) as *mut *mut Parse;
    let mut __slate_storage_1251: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1251: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1251) as *mut i32;
    let mut __slate_storage_1250: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1250: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1250) as *mut i32;
    let mut __slate_storage_1249: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1249: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1249) as *mut i32;
    let mut __slate_storage_1248: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1248: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1248) as *mut i32;
    let mut __slate_storage_1247: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1247: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1247) as *mut *mut Parse;
    let mut __slate_storage_1246: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1246: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1246) as *mut i32;
    let mut __slate_storage_1245: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1245: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1245) as *mut bool;
    let mut __slate_storage_608: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_608: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_608) as *mut i32;
    let mut __slate_storage_607: std::mem::MaybeUninit<*mut Trigger> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_607: *mut *mut Trigger =
        std::ptr::addr_of_mut!(__slate_storage_607) as *mut *mut Trigger;
    let mut __slate_storage_606: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_606: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_606) as *mut i32;
    let mut __slate_storage_605: std::mem::MaybeUninit<*mut i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_605: *mut *mut i32 =
        std::ptr::addr_of_mut!(__slate_storage_605) as *mut *mut i32;
    let mut __slate_storage_604: std::mem::MaybeUninit<*mut i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_604: *mut *mut i32 =
        std::ptr::addr_of_mut!(__slate_storage_604) as *mut *mut i32;
    let mut __slate_storage_603: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_603: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_603) as *mut i32;
    let mut __slate_storage_602: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_602: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_602) as *mut i32;
    let mut __slate_storage_601: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_601: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_601) as *mut i32;
    let mut __slate_storage_600: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_600: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_600) as *mut i32;
    let mut __slate_storage_599: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_599: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_599) as *mut i32;
    let mut __slate_storage_598: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_598: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_598) as *mut i32;
    let mut __slate_storage_597: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_597: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_597) as *mut i32;
    let mut __slate_storage_596: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_596: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_596) as *mut *mut ExprList;
    let mut __slate_storage_595: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_595: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_595) as *mut u8;
    let mut __slate_storage_594: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_594: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_594) as *mut u8;
    let mut __slate_storage_593: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_593: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_593) as *mut u8;
    let mut __slate_storage_592: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_592: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_592) as *mut u8;
    let mut __slate_storage_591: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_591: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_591) as *mut i32;
    let mut __slate_storage_590: std::mem::MaybeUninit<SelectDest> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_590: *mut SelectDest =
        std::ptr::addr_of_mut!(__slate_storage_590) as *mut SelectDest;
    let mut __slate_storage_589: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_589: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_589) as *mut i32;
    let mut __slate_storage_588: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_588: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_588) as *mut i32;
    let mut __slate_storage_587: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_587: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_587) as *mut i32;
    let mut __slate_storage_586: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_586: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_586) as *mut i32;
    let mut __slate_storage_585: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_585: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_585) as *mut i32;
    let mut __slate_storage_584: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_584: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_584) as *mut i32;
    let mut __slate_storage_583: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_583: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_583) as *mut i32;
    let mut __slate_storage_582: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_582: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_582) as *mut i32;
    let mut __slate_storage_581: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_581: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_581) as *mut i32;
    let mut __slate_storage_580: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_580: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_580) as *mut *mut Index;
    let mut __slate_storage_579: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_579: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_579) as *mut *mut Vdbe;
    let mut __slate_storage_578: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_578: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_578) as *mut i32;
    let mut __slate_storage_577: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_577: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_577) as *mut i32;
    let mut __slate_storage_576: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_576: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_576) as *mut *mut Table;
    let mut __slate_storage_575: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_575: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_575) as *mut *mut sqlite3;
    unsafe {
        '__join_4: {
            // /* The main database structure */
            // /* The table to insert into.  aka TABLE */
            // /* Loop counters */
            // /* Generate code into this virtual machine */
            // /* For looping over indices of the table */
            // /* Number of columns in the data */
            // /* Number of hidden columns if TABLE is virtual */
            std::ptr::write(__slate_slot_582, 0 as i32);
            // /* VDBE cursor that is the main data repository */
            std::ptr::write(__slate_slot_583, 0 as i32);
            // /* First index cursor */
            std::ptr::write(__slate_slot_584, 0 as i32);
            // /* Column that is the INTEGER PRIMARY KEY */
            std::ptr::write(__slate_slot_585, -(1 as i32));
            // /* Label for the end of the insertion loop */
            // /* Data comes from this temporary cursor if >=0 */
            std::ptr::write(__slate_slot_587, 0 as i32);
            // /* Jump to label "D" */
            std::ptr::write(__slate_slot_588, 0 as i32);
            // /* Top of insert loop. Label "C" in templates 3 and 4 */
            std::ptr::write(__slate_slot_589, 0 as i32);
            // /* Destination for SELECT on rhs of INSERT */
            // /* Index of database holding TABLE */
            // /* Store SELECT results in intermediate table */
            std::ptr::write(__slate_slot_592, ((0 as i32) as i8) as u8);
            // /* True if the insert is likely to be an append */
            std::ptr::write(__slate_slot_593, ((0 as i32) as i8) as u8);
            // /* 0 for normal table.  1 for WITHOUT ROWID table */
            // /* True if IDLIST is in table order */
            // /* List of VALUES() to be inserted  */
            std::ptr::write(__slate_slot_596, std::ptr::null_mut::<ExprList>());
            // /* Register in which to store next column */
            // /* Register allocations */
            // /* Base register for data coming from SELECT */
            std::ptr::write(__slate_slot_598, 0 as i32);
            // /* Register holding the AUTOINCREMENT counter */
            std::ptr::write(__slate_slot_599, 0 as i32);
            // /* Memory cell used for the row counter */
            std::ptr::write(__slate_slot_600, 0 as i32);
            // /* Block of regs holding rowid+data being inserted */
            // /* registers holding insert rowid */
            // /* register holding first column to insert */
            // /* One register allocated to each index */
            std::ptr::write(__slate_slot_604, std::ptr::null_mut::<i32>());
            // /* Mapping from pTab columns to pCol entries */
            std::ptr::write(__slate_slot_605, std::ptr::null_mut::<i32>());
            // /* True if attempting to insert into a view */
            // /* List of triggers on pTab, if required */
            // /* Mask of trigger times */
            *__slate_slot_575 = unsafe { (*pParse).db };
            0 as i32;
            if (unsafe { (*pParse).nErr }) != (0 as i32) {
            } else {
                0 as i32;
                // /* Suppress a harmless compiler warning */
                (*__slate_slot_590).iSDParm = 0 as i32;
                // /* If the Select object is really just a simple VALUES() list with a
                //   ** single row (the common case) then keep that one row of values
                //   ** and discard the other (unused) parts of the pSelect object
                //   */
                if pSelect != std::ptr::null_mut::<Select>()
                    && (unsafe { (*pSelect).selFlags }) & ((512 as i32) as u32)
                        != ((0 as i32) as u32)
                    && (unsafe { (*pSelect).pPrior }) == std::ptr::null_mut::<Select>()
                {
                    *__slate_slot_596 = unsafe { (*pSelect).pEList };
                    unsafe {
                        (*pSelect).pEList = std::ptr::null_mut::<ExprList>();
                    }
                    unsafe { sqlite3SelectDelete(*__slate_slot_575, pSelect) };
                    pSelect = std::ptr::null_mut::<Select>();
                }
                // /* Locate the table into which we will be inserting new information.
                //   */
                0 as i32;
                *__slate_slot_576 = unsafe { sqlite3SrcListLookup(pParse, pTabList) };
                if *__slate_slot_576 == std::ptr::null_mut::<Table>() {
                } else {
                    *__slate_slot_591 = unsafe {
                        sqlite3SchemaToIndex(*__slate_slot_575, unsafe {
                            (*(*__slate_slot_576)).pSchema
                        })
                    };
                    0 as i32;
                    if (unsafe {
                        sqlite3AuthCheck(
                            pParse,
                            18 as i32,
                            (unsafe { (*(*__slate_slot_576)).zName }) as *const i8,
                            std::ptr::null::<i8>(),
                            (unsafe {
                                (*unsafe {
                                    unsafe { (*(*__slate_slot_575)).aDb }
                                        .offset(*__slate_slot_591 as isize)
                                })
                                .zDbSName
                            }) as *const i8,
                        )
                    }) != (0 as i32)
                    {
                    } else {
                        *__slate_slot_594 =
                            !((unsafe { (*(*__slate_slot_576)).tabFlags }) & ((128 as i32) as u32)
                                == ((0 as i32) as u32)) as u8;
                        // /* Figure out if we have any triggers and if the table being
                        //   ** inserted into is a view
                        //   */
                        *__slate_slot_607 = unsafe {
                            sqlite3TriggersExist(
                                pParse,
                                *__slate_slot_576,
                                128 as i32,
                                std::ptr::null_mut::<ExprList>(),
                                std::ptr::addr_of_mut!(*__slate_slot_608),
                            )
                        };
                        *__slate_slot_606 = ((((unsafe { (*(*__slate_slot_576)).eTabType }) as u32)
                            as i32)
                            == (2 as i32)) as i32;
                        0 as i32;
                        // /* If pTab is really a view, make sure it has been initialized.
                        //   ** ViewGetColumnNames() is a no-op if pTab is not a view.
                        //   */
                        if (unsafe { sqlite3ViewGetColumnNames(pParse, *__slate_slot_576) })
                            != (0 as i32)
                        {
                        } else {
                            // /* Cannot insert into a read-only table.
                            //   */
                            if (unsafe {
                                sqlite3IsReadOnly(pParse, *__slate_slot_576, *__slate_slot_607)
                            }) != (0 as i32)
                            {
                            } else {
                                // /* Allocate a VDBE
                                //   */
                                *__slate_slot_579 = unsafe { sqlite3GetVdbe(pParse) };
                                if *__slate_slot_579 == std::ptr::null_mut::<Vdbe>() {
                                } else {
                                    if (((unsafe { (*pParse).nested }) as u32) as i32) == (0 as i32)
                                    {
                                        unsafe { sqlite3VdbeCountChanges(*__slate_slot_579) };
                                    }
                                    unsafe {
                                        sqlite3BeginWriteOperation(
                                            pParse,
                                            (pSelect != std::ptr::null_mut::<Select>()
                                                || *__slate_slot_607
                                                    != std::ptr::null_mut::<Trigger>())
                                                as i32,
                                            *__slate_slot_591,
                                        )
                                    };
                                    // /* If the statement is of the form
                                    //   **
                                    //   **       INSERT INTO <table1> SELECT * FROM <table2>;
                                    //   **
                                    //   ** Then special optimizations can be applied that make the transfer
                                    //   ** very fast and which reduce fragmentation of indices.
                                    //   **
                                    //   ** This is the 2nd template.
                                    //   */
                                    if pColumn == std::ptr::null_mut::<IdList>()
                                        && pSelect != std::ptr::null_mut::<Select>()
                                        && *__slate_slot_607 == std::ptr::null_mut::<Trigger>()
                                    {
                                        *__slate_slot_1245 = xferOptimization(
                                            pParse,
                                            *__slate_slot_576,
                                            pSelect,
                                            onError,
                                            *__slate_slot_591,
                                        ) != (0 as i32);
                                    } else {
                                        *__slate_slot_1245 = false as bool;
                                    }
                                    if *__slate_slot_1245 {
                                        0 as i32;
                                        0 as i32;
                                    } else {
                                        // /* SQLITE_OMIT_XFER_OPT */
                                        // /* If this is an AUTOINCREMENT table, look up the sequence number in the
                                        //   ** sqlite_sequence table and store it in memory cell regAutoinc.
                                        //   */
                                        *__slate_slot_599 = autoIncBegin(
                                            pParse,
                                            *__slate_slot_591,
                                            *__slate_slot_576,
                                        );
                                        // /* Allocate a block registers to hold the rowid and the values
                                        //   ** for all columns of the new row.
                                        //   */
                                        std::ptr::write(
                                            __slate_slot_1246,
                                            (unsafe { (*pParse).nMem }) + (1 as i32),
                                        );
                                        *__slate_slot_601 = *__slate_slot_1246;
                                        *__slate_slot_602 = *__slate_slot_1246;
                                        std::ptr::write(__slate_slot_1247, pParse);
                                        std::ptr::write(__slate_slot_1248, unsafe {
                                            (*(*__slate_slot_1247)).nMem
                                        });
                                        std::ptr::write(
                                            __slate_slot_1249,
                                            *__slate_slot_1248
                                                + (((unsafe { (*(*__slate_slot_576)).nCol })
                                                    as i32)
                                                    + (1 as i32)),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_1247)).nMem = *__slate_slot_1249;
                                        }
                                        if (((unsafe { (*(*__slate_slot_576)).eTabType }) as u32)
                                            as i32)
                                            == (1 as i32)
                                        {
                                            std::ptr::write(__slate_slot_1250, *__slate_slot_602);
                                            std::ptr::write(
                                                __slate_slot_1251,
                                                *__slate_slot_1250 + (1 as i32),
                                            );
                                            *__slate_slot_602 = *__slate_slot_1251;
                                            std::ptr::write(__slate_slot_1252, pParse);
                                            std::ptr::write(__slate_slot_1253, unsafe {
                                                (*(*__slate_slot_1252)).nMem
                                            });
                                            std::ptr::write(
                                                __slate_slot_1254,
                                                *__slate_slot_1253 + (1 as i32),
                                            );
                                            unsafe {
                                                (*(*__slate_slot_1252)).nMem = *__slate_slot_1254;
                                            }
                                        }
                                        '__join_140: {
                                            *__slate_slot_603 = *__slate_slot_602 + (1 as i32);
                                            // /* If the INSERT statement included an IDLIST term, then make sure
                                            //   ** all elements of the IDLIST really are columns of the table and
                                            //   ** remember the column indices.
                                            //   **
                                            //   ** If the table has an INTEGER PRIMARY KEY column and that column
                                            //   ** is named in the IDLIST, then record in the ipkColumn variable
                                            //   ** the index into IDLIST of the primary key column.  ipkColumn is
                                            //   ** the index of the primary key as it appears in IDLIST, not as
                                            //   ** is appears in the original table.  (The index of the INTEGER
                                            //   ** PRIMARY KEY in the original table is pTab->iPKey.)  After this
                                            //   ** loop, if ipkColumn==(-1), that means that integer primary key
                                            //   ** is unspecified, and hence the table is either WITHOUT ROWID or
                                            //   ** it will automatically generated an integer primary key.
                                            //   **
                                            //   ** bIdListInOrder is true if the columns in IDLIST are in storage
                                            //   ** order.  This enables an optimization that avoids shuffling the
                                            //   ** columns into storage order.  False negatives are harmless,
                                            //   ** but false positives will cause database corruption.
                                            //   */
                                            *__slate_slot_595 =
                                                ((unsafe { (*(*__slate_slot_576)).tabFlags })
                                                    & (((1024 as i32) | (64 as i32)) as u32)
                                                    == ((0 as i32) as u32))
                                                    as u8;
                                            if pColumn != std::ptr::null_mut::<IdList>() {
                                                *__slate_slot_605 = (unsafe {
                                                    sqlite3DbMallocZero(
                                                        *__slate_slot_575,
                                                        ((((unsafe { (*(*__slate_slot_576)).nCol })
                                                            as i32)
                                                            as i64)
                                                            as u64)
                                                            .wrapping_mul(4 as u64),
                                                    )
                                                })
                                                    as *mut i32;
                                                if *__slate_slot_605 == std::ptr::null_mut::<i32>()
                                                {
                                                    break '__join_4;
                                                } else {
                                                    *__slate_slot_577 = 0 as i32;
                                                    '__join_143: {
                                                        '__loop_141: loop {
                                                            if *__slate_slot_577
                                                                < unsafe { (*pColumn).nId }
                                                            {
                                                                *__slate_slot_578 = unsafe {
                                                                    sqlite3ColumnIndex(
                                                                        *__slate_slot_576,
                                                                        (unsafe {
                                                                            (*unsafe { unsafe { std::ptr::addr_of_mut!((*pColumn).a) as *mut IdList_item }.offset(*__slate_slot_577 as isize) }).zName
                                                                        })
                                                                            as *const i8,
                                                                    )
                                                                };
                                                                if *__slate_slot_578 >= (0 as i32) {
                                                                    if (unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_605)
                                                                                .offset(
                                                                                *__slate_slot_578
                                                                                    as isize,
                                                                            )
                                                                        }
                                                                    }) == (0 as i32)
                                                                    {
                                                                        unsafe {
                                                                            *unsafe {
                                                                                (*__slate_slot_605).offset(*__slate_slot_578 as isize)
                                                                            } = *__slate_slot_577
                                                                                + (1 as i32);
                                                                        }
                                                                    }
                                                                    if *__slate_slot_577
                                                                        != *__slate_slot_578
                                                                    {
                                                                        *__slate_slot_595 =
                                                                            ((0 as i32) as i8)
                                                                                as u8;
                                                                    }
                                                                    if *__slate_slot_578
                                                                        == ((unsafe {
                                                                            (*(*__slate_slot_576))
                                                                                .iPKey
                                                                        })
                                                                            as i32)
                                                                    {
                                                                        *__slate_slot_585 =
                                                                            *__slate_slot_577;
                                                                        0 as i32;
                                                                    }
                                                                    if (((unsafe {
                                                                        (*unsafe { unsafe { (*(*__slate_slot_576)).aCol }.offset(*__slate_slot_578 as isize) }).colFlags
                                                                    })
                                                                        as u32)
                                                                        as i32)
                                                                        & ((64 as i32)
                                                                            | (32 as i32))
                                                                        != (0 as i32)
                                                                    {
                                                                        break '__join_143;
                                                                    }
                                                                } else {
                                                                    if (unsafe {
                                                                        sqlite3IsRowid(
                                                                            (unsafe {
                                                                                (*unsafe { unsafe { std::ptr::addr_of_mut!((*pColumn).a) as *mut IdList_item }.offset(*__slate_slot_577 as isize) }).zName
                                                                            })
                                                                                as *const i8,
                                                                        )
                                                                    }) != (0 as i32)
                                                                        && !(*__slate_slot_594
                                                                            != (0 as u8))
                                                                    {
                                                                        *__slate_slot_585 =
                                                                            *__slate_slot_577;
                                                                        *__slate_slot_595 =
                                                                            ((0 as i32) as i8)
                                                                                as u8;
                                                                    } else {
                                                                        break '__loop_141;
                                                                    }
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_1255,
                                                                    *__slate_slot_577,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1256,
                                                                    *__slate_slot_1255 + (1 as i32),
                                                                );
                                                                *__slate_slot_577 =
                                                                    *__slate_slot_1256;
                                                            } else {
                                                                break '__join_140;
                                                            }
                                                        }
                                                        unsafe {
                                                            sqlite3ErrorMsg(pParse, (b"table %S has no column named %s\0".as_ptr() as *mut i8) as *const i8, unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }, unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*pColumn).a) as *mut IdList_item }.offset(*__slate_slot_577 as isize) }).zName })
                                                        };
                                                        unsafe {
                                                            (*pParse)
                                                                .__slate_bits_0
                                                                .__set_checkSchema(
                                                                    (1 as i32) as u32,
                                                                );
                                                        }
                                                        break '__join_4;
                                                    }
                                                    unsafe {
                                                        sqlite3ErrorMsg(pParse, (b"cannot INSERT into generated column \"%s\"\0".as_ptr() as *mut i8) as *const i8, unsafe { (*unsafe { unsafe { (*(*__slate_slot_576)).aCol }.offset(*__slate_slot_578 as isize) }).zCnName })
                                                    };
                                                    break '__join_4;
                                                }
                                            }
                                        }
                                        // /* Figure out how many columns of data are supplied.  If the data
                                        //   ** is coming from a SELECT statement, then generate a co-routine that
                                        //   ** produces a single row of the SELECT on each invocation.  The
                                        //   ** co-routine is the common header to the 3rd and 4th templates.
                                        //   */
                                        if pSelect != std::ptr::null_mut::<Select>() {
                                            // /* Data is coming from a SELECT or from a multi-row VALUES clause.
                                            //     ** Generate a co-routine to run the SELECT. */
                                            // /* Result code */
                                            if (unsafe { (*unsafe { (*pSelect).pSrc }).nSrc })
                                                == (1 as i32)
                                                && ((unsafe {
                                                    (*unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of_mut!(
                                                                (*unsafe { (*pSelect).pSrc }).a
                                                            )
                                                                as *mut SrcItem
                                                        }
                                                        .offset((0 as i32) as isize)
                                                    })
                                                    .fg
                                                    .__slate_bits_0
                                                    .__get_viaCoroutine()
                                                })
                                                    as i32)
                                                    != (0 as i32)
                                                && (unsafe { (*pSelect).pPrior })
                                                    == std::ptr::null_mut::<Select>()
                                            {
                                                std::ptr::write(__slate_slot_610, unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of_mut!(
                                                            (*unsafe { (*pSelect).pSrc }).a
                                                        )
                                                            as *mut SrcItem
                                                    }
                                                    .offset((0 as i32) as isize)
                                                });
                                                0 as i32;
                                                *__slate_slot_611 =
                                                    unsafe { (*(*__slate_slot_610)).u4.pSubq };
                                                (*__slate_slot_590).iSDParm =
                                                    unsafe { (*(*__slate_slot_611)).regReturn };
                                                *__slate_slot_598 =
                                                    unsafe { (*(*__slate_slot_611)).regResult };
                                                0 as i32;
                                                0 as i32;
                                                *__slate_slot_581 = unsafe {
                                                    (*unsafe {
                                                        (*unsafe { (*(*__slate_slot_611)).pSelect })
                                                            .pEList
                                                    })
                                                    .nExpr
                                                };
                                                unsafe {
                                                    sqlite3VdbeExplain(
                                                        pParse,
                                                        ((0 as i32) as i8) as u8,
                                                        (b"SCAN %S\0".as_ptr() as *mut i8)
                                                            as *const i8,
                                                        *__slate_slot_610,
                                                    )
                                                };
                                                if *__slate_slot_595 != (0 as u8)
                                                    && *__slate_slot_581
                                                        == ((unsafe { (*(*__slate_slot_576)).nCol })
                                                            as i32)
                                                {
                                                    *__slate_slot_603 = *__slate_slot_598;
                                                    *__slate_slot_602 =
                                                        *__slate_slot_603 - (1 as i32);
                                                    *__slate_slot_601 = *__slate_slot_602
                                                        - if (((unsafe {
                                                            (*(*__slate_slot_576)).eTabType
                                                        })
                                                            as u32)
                                                            as i32)
                                                            == (1 as i32)
                                                        {
                                                            1 as i32
                                                        } else {
                                                            0 as i32
                                                        };
                                                }
                                            } else {
                                                // /* Top of the co-routine */
                                                std::ptr::write(__slate_slot_1257, pParse);
                                                std::ptr::write(__slate_slot_1258, unsafe {
                                                    (*(*__slate_slot_1257)).nMem
                                                });
                                                std::ptr::write(
                                                    __slate_slot_1259,
                                                    *__slate_slot_1258 + (1 as i32),
                                                );
                                                unsafe {
                                                    (*(*__slate_slot_1257)).nMem =
                                                        *__slate_slot_1259;
                                                }
                                                *__slate_slot_613 = *__slate_slot_1259;
                                                *__slate_slot_612 = (unsafe {
                                                    sqlite3VdbeCurrentAddr(*__slate_slot_579)
                                                }) + (1 as i32);
                                                unsafe {
                                                    sqlite3VdbeAddOp3(
                                                        *__slate_slot_579,
                                                        11 as i32,
                                                        *__slate_slot_613,
                                                        0 as i32,
                                                        *__slate_slot_612,
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3SelectDestInit(
                                                        std::ptr::addr_of_mut!(*__slate_slot_590),
                                                        11 as i32,
                                                        *__slate_slot_613,
                                                    )
                                                };
                                                (*__slate_slot_590).iSdst =
                                                    if *__slate_slot_595 != (0 as u8) {
                                                        *__slate_slot_603
                                                    } else {
                                                        0 as i32
                                                    };
                                                (*__slate_slot_590).nSdst =
                                                    (unsafe { (*(*__slate_slot_576)).nCol }) as i32;
                                                *__slate_slot_609 = unsafe {
                                                    sqlite3Select(
                                                        pParse,
                                                        pSelect,
                                                        std::ptr::addr_of_mut!(*__slate_slot_590),
                                                    )
                                                };
                                                *__slate_slot_598 = (*__slate_slot_590).iSdst;
                                                0 as i32;
                                                if *__slate_slot_609 != (0 as i32)
                                                    || (unsafe { (*pParse).nErr }) != (0 as i32)
                                                {
                                                    break '__join_4;
                                                } else {
                                                    0 as i32;
                                                    unsafe {
                                                        sqlite3VdbeEndCoroutine(
                                                            *__slate_slot_579,
                                                            *__slate_slot_613,
                                                        )
                                                    };
                                                    // /* label B: */
                                                    unsafe {
                                                        sqlite3VdbeJumpHere(
                                                            *__slate_slot_579,
                                                            *__slate_slot_612 - (1 as i32),
                                                        )
                                                    };
                                                    0 as i32;
                                                    *__slate_slot_581 = unsafe {
                                                        (*unsafe { (*pSelect).pEList }).nExpr
                                                    };
                                                }
                                            }
                                            // /* Set useTempTable to TRUE if the result of the SELECT statement
                                            //     ** should be written into a temporary table (template 4).  Set to
                                            //     ** FALSE if each output row of the SELECT can be written directly into
                                            //     ** the destination table (template 3).
                                            //     **
                                            //     ** A temp table must be used if the table being updated is also one
                                            //     ** of the tables being read by the SELECT statement.  Also use a
                                            //     ** temp table in the case of row triggers.
                                            //     */
                                            if *__slate_slot_607 != std::ptr::null_mut::<Trigger>()
                                            {
                                                *__slate_slot_1260 = true as bool;
                                            } else {
                                                *__slate_slot_1260 = readsTable(
                                                    pParse,
                                                    *__slate_slot_591,
                                                    *__slate_slot_576,
                                                ) != (0 as i32);
                                            }
                                            if *__slate_slot_1260 {
                                                *__slate_slot_592 = ((1 as i32) as i8) as u8;
                                            }
                                            if *__slate_slot_592 != (0 as u8) {
                                                // /* Invoke the coroutine to extract information from the SELECT
                                                //       ** and add it to a transient table srcTab.  The code generated
                                                //       ** here is from the 4th template:
                                                //       **
                                                //       **      B: open temp table
                                                //       **      L: yield X, goto M at EOF
                                                //       **         insert row from R..R+n into temp table
                                                //       **         goto L
                                                //       **      M: ...
                                                //       */
                                                // /* Register to hold packed record */
                                                // /* Register to hold temp table ROWID */
                                                // /* Label "L" */
                                                std::ptr::write(__slate_slot_1261, pParse);
                                                std::ptr::write(__slate_slot_1262, unsafe {
                                                    (*(*__slate_slot_1261)).nTab
                                                });
                                                std::ptr::write(
                                                    __slate_slot_1263,
                                                    *__slate_slot_1262 + (1 as i32),
                                                );
                                                unsafe {
                                                    (*(*__slate_slot_1261)).nTab =
                                                        *__slate_slot_1263;
                                                }
                                                *__slate_slot_587 = *__slate_slot_1262;
                                                *__slate_slot_614 =
                                                    unsafe { sqlite3GetTempReg(pParse) };
                                                *__slate_slot_615 =
                                                    unsafe { sqlite3GetTempReg(pParse) };
                                                unsafe {
                                                    sqlite3VdbeAddOp2(
                                                        *__slate_slot_579,
                                                        120 as i32,
                                                        *__slate_slot_587,
                                                        *__slate_slot_581,
                                                    )
                                                };
                                                *__slate_slot_616 = unsafe {
                                                    sqlite3VdbeAddOp1(
                                                        *__slate_slot_579,
                                                        12 as i32,
                                                        (*__slate_slot_590).iSDParm,
                                                    )
                                                };
                                                {}
                                                unsafe {
                                                    sqlite3VdbeAddOp3(
                                                        *__slate_slot_579,
                                                        99 as i32,
                                                        *__slate_slot_598,
                                                        *__slate_slot_581,
                                                        *__slate_slot_614,
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3VdbeAddOp2(
                                                        *__slate_slot_579,
                                                        129 as i32,
                                                        *__slate_slot_587,
                                                        *__slate_slot_615,
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3VdbeAddOp3(
                                                        *__slate_slot_579,
                                                        130 as i32,
                                                        *__slate_slot_587,
                                                        *__slate_slot_614,
                                                        *__slate_slot_615,
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3VdbeGoto(
                                                        *__slate_slot_579,
                                                        *__slate_slot_616,
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3VdbeJumpHere(
                                                        *__slate_slot_579,
                                                        *__slate_slot_616,
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3ReleaseTempReg(pParse, *__slate_slot_614)
                                                };
                                                unsafe {
                                                    sqlite3ReleaseTempReg(pParse, *__slate_slot_615)
                                                };
                                            }
                                        } else {
                                            // /* This is the case if the data for the INSERT is coming from a
                                            //     ** single-row VALUES clause
                                            //     */
                                            unsafe {
                                                memset(
                                                    std::ptr::addr_of_mut!(*__slate_slot_617)
                                                        as *mut (),
                                                    0 as i32,
                                                    56 as u64,
                                                )
                                            };
                                            (*__slate_slot_617).pParse = pParse;
                                            *__slate_slot_587 = -(1 as i32);
                                            0 as i32;
                                            if *__slate_slot_596 != std::ptr::null_mut::<ExprList>()
                                            {
                                                *__slate_slot_581 =
                                                    unsafe { (*(*__slate_slot_596)).nExpr };
                                                if (unsafe {
                                                    sqlite3ResolveExprListNames(
                                                        std::ptr::addr_of_mut!(*__slate_slot_617),
                                                        *__slate_slot_596,
                                                    )
                                                }) != (0 as i32)
                                                {
                                                    break '__join_4;
                                                }
                                            } else {
                                                *__slate_slot_581 = 0 as i32;
                                            }
                                        }
                                        // /* If there is no IDLIST term but the table has an integer primary
                                        //   ** key, the set the ipkColumn variable to the integer primary key
                                        //   ** column index in the original table definition.
                                        //   */
                                        if pColumn == std::ptr::null_mut::<IdList>()
                                            && *__slate_slot_581 > (0 as i32)
                                        {
                                            '__join_117: {
                                                *__slate_slot_585 =
                                                    (unsafe { (*(*__slate_slot_576)).iPKey })
                                                        as i32;
                                                if *__slate_slot_585 >= (0 as i32)
                                                    && (unsafe { (*(*__slate_slot_576)).tabFlags })
                                                        & ((96 as i32) as u32)
                                                        != ((0 as i32) as u32)
                                                {
                                                    {}
                                                    {}
                                                    *__slate_slot_577 =
                                                        *__slate_slot_585 - (1 as i32);
                                                    loop {
                                                        if *__slate_slot_577 >= (0 as i32) {
                                                            if (((unsafe {
                                                                (*unsafe {
                                                                    unsafe {
                                                                        (*(*__slate_slot_576)).aCol
                                                                    }
                                                                    .offset(
                                                                        *__slate_slot_577 as isize,
                                                                    )
                                                                })
                                                                .colFlags
                                                            })
                                                                as u32)
                                                                as i32)
                                                                & (96 as i32)
                                                                != (0 as i32)
                                                            {
                                                                {}
                                                                {}
                                                                std::ptr::write(
                                                                    __slate_slot_1266,
                                                                    *__slate_slot_585,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1267,
                                                                    *__slate_slot_1266 - (1 as i32),
                                                                );
                                                                *__slate_slot_585 =
                                                                    *__slate_slot_1267;
                                                            }
                                                            std::ptr::write(
                                                                __slate_slot_1264,
                                                                *__slate_slot_577,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1265,
                                                                *__slate_slot_1264 - (1 as i32),
                                                            );
                                                            *__slate_slot_577 = *__slate_slot_1265;
                                                        } else {
                                                            break '__join_117;
                                                        }
                                                    }
                                                }
                                            }
                                            '__join_111: {
                                                // /* Make sure the number of columns in the source data matches the number
                                                //     ** of columns to be inserted into the table.
                                                //     */
                                                0 as i32;
                                                0 as i32;
                                                0 as i32;
                                                if (unsafe { (*(*__slate_slot_576)).tabFlags })
                                                    & (((96 as i32) | (2 as i32)) as u32)
                                                    != ((0 as i32) as u32)
                                                {
                                                    *__slate_slot_577 = 0 as i32;
                                                    loop {
                                                        if *__slate_slot_577
                                                            < ((unsafe {
                                                                (*(*__slate_slot_576)).nCol
                                                            })
                                                                as i32)
                                                        {
                                                            if (((unsafe {
                                                                (*unsafe {
                                                                    unsafe {
                                                                        (*(*__slate_slot_576)).aCol
                                                                    }
                                                                    .offset(
                                                                        *__slate_slot_577 as isize,
                                                                    )
                                                                })
                                                                .colFlags
                                                            })
                                                                as u32)
                                                                as i32)
                                                                & (98 as i32)
                                                                != (0 as i32)
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_1270,
                                                                    *__slate_slot_582,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1271,
                                                                    *__slate_slot_1270 + (1 as i32),
                                                                );
                                                                *__slate_slot_582 =
                                                                    *__slate_slot_1271;
                                                            }
                                                            std::ptr::write(
                                                                __slate_slot_1268,
                                                                *__slate_slot_577,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1269,
                                                                *__slate_slot_1268 + (1 as i32),
                                                            );
                                                            *__slate_slot_577 = *__slate_slot_1269;
                                                        } else {
                                                            break '__join_111;
                                                        }
                                                    }
                                                }
                                            }
                                            if *__slate_slot_581
                                                != ((unsafe { (*(*__slate_slot_576)).nCol }) as i32)
                                                    - *__slate_slot_582
                                            {
                                                unsafe {
                                                    sqlite3ErrorMsg(pParse, (b"table %S has %d columns but %d values were supplied\0".as_ptr() as *mut i8) as *const i8, unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }, ((unsafe { (*(*__slate_slot_576)).nCol }) as i32) - *__slate_slot_582, *__slate_slot_581)
                                                };
                                                break '__join_4;
                                            }
                                        }
                                        if pColumn != std::ptr::null_mut::<IdList>()
                                            && *__slate_slot_581 != unsafe { (*pColumn).nId }
                                        {
                                            unsafe {
                                                sqlite3ErrorMsg(
                                                    pParse,
                                                    (b"%d values for %d columns\0".as_ptr()
                                                        as *mut i8)
                                                        as *const i8,
                                                    *__slate_slot_581,
                                                    unsafe { (*pColumn).nId },
                                                )
                                            };
                                            break '__join_4;
                                        } else {
                                            // /* Initialize the count of rows to be inserted
                                            //   */
                                            if (unsafe { (*(*__slate_slot_575)).flags })
                                                & (((1 as i32) as i64) as u64) << (32 as i32)
                                                != (((0 as i32) as i64) as u64)
                                                && !((unsafe { (*pParse).nested }) != (0 as u8))
                                                && !((unsafe { (*pParse).pTriggerTab })
                                                    != std::ptr::null_mut::<Table>())
                                                && !(((unsafe {
                                                    (*pParse).__slate_bits_0.__get_bReturning()
                                                })
                                                    as i32)
                                                    != (0 as i32))
                                            {
                                                std::ptr::write(__slate_slot_1272, pParse);
                                                std::ptr::write(__slate_slot_1273, unsafe {
                                                    (*(*__slate_slot_1272)).nMem
                                                });
                                                std::ptr::write(
                                                    __slate_slot_1274,
                                                    *__slate_slot_1273 + (1 as i32),
                                                );
                                                unsafe {
                                                    (*(*__slate_slot_1272)).nMem =
                                                        *__slate_slot_1274;
                                                }
                                                *__slate_slot_600 = *__slate_slot_1274;
                                                unsafe {
                                                    sqlite3VdbeAddOp2(
                                                        *__slate_slot_579,
                                                        73 as i32,
                                                        0 as i32,
                                                        *__slate_slot_600,
                                                    )
                                                };
                                            }
                                            // /* If this is not a view, open the table and and all indices */
                                            if !(*__slate_slot_606 != (0 as i32)) {
                                                *__slate_slot_618 = sqlite3OpenTableAndIndices(
                                                    pParse,
                                                    *__slate_slot_576,
                                                    116 as i32,
                                                    ((0 as i32) as i8) as u8,
                                                    -(1 as i32),
                                                    std::ptr::null_mut::<u8>(),
                                                    std::ptr::addr_of_mut!(*__slate_slot_583),
                                                    std::ptr::addr_of_mut!(*__slate_slot_584),
                                                );
                                                *__slate_slot_604 = (unsafe {
                                                    sqlite3DbMallocRawNN(
                                                        *__slate_slot_575,
                                                        (4 as u64).wrapping_mul(
                                                            ((*__slate_slot_618 + (2 as i32))
                                                                as i64)
                                                                as u64,
                                                        ),
                                                    )
                                                })
                                                    as *mut i32;
                                                if *__slate_slot_604 == std::ptr::null_mut::<i32>()
                                                {
                                                    break '__join_4;
                                                } else {
                                                    *__slate_slot_577 = 0 as i32;
                                                    std::ptr::write(__slate_slot_1275, unsafe {
                                                        (*(*__slate_slot_576)).pIndex
                                                    });
                                                    *__slate_slot_580 = *__slate_slot_1275;
                                                    loop {
                                                        if *__slate_slot_577 < *__slate_slot_618 {
                                                            0 as i32;
                                                            std::ptr::write(
                                                                __slate_slot_1278,
                                                                pParse,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1279,
                                                                unsafe {
                                                                    (*(*__slate_slot_1278)).nMem
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1280,
                                                                *__slate_slot_1279 + (1 as i32),
                                                            );
                                                            unsafe {
                                                                (*(*__slate_slot_1278)).nMem =
                                                                    *__slate_slot_1280;
                                                            }
                                                            unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_604).offset(
                                                                        *__slate_slot_577 as isize,
                                                                    )
                                                                } = *__slate_slot_1280;
                                                            }
                                                            std::ptr::write(
                                                                __slate_slot_1281,
                                                                pParse,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1282,
                                                                unsafe {
                                                                    (*(*__slate_slot_1281)).nMem
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1283,
                                                                *__slate_slot_1282
                                                                    + (((unsafe {
                                                                        (*(*__slate_slot_580))
                                                                            .nColumn
                                                                    })
                                                                        as u32)
                                                                        as i32),
                                                            );
                                                            unsafe {
                                                                (*(*__slate_slot_1281)).nMem =
                                                                    *__slate_slot_1283;
                                                            }
                                                            *__slate_slot_580 = unsafe {
                                                                (*(*__slate_slot_580)).pNext
                                                            };
                                                            std::ptr::write(
                                                                __slate_slot_1276,
                                                                *__slate_slot_577,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1277,
                                                                *__slate_slot_1276 + (1 as i32),
                                                            );
                                                            *__slate_slot_577 = *__slate_slot_1277;
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                    // /* Register to store the table record */
                                                    std::ptr::write(__slate_slot_1284, pParse);
                                                    std::ptr::write(__slate_slot_1285, unsafe {
                                                        (*(*__slate_slot_1284)).nMem
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_1286,
                                                        *__slate_slot_1285 + (1 as i32),
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_1284)).nMem =
                                                            *__slate_slot_1286;
                                                    }
                                                    unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_604)
                                                                .offset(*__slate_slot_577 as isize)
                                                        } = *__slate_slot_1286;
                                                    }
                                                }
                                            }
                                            '__join_89: {
                                                if pUpsert != std::ptr::null_mut::<Upsert>() {
                                                    if (((unsafe {
                                                        (*(*__slate_slot_576)).eTabType
                                                    })
                                                        as u32)
                                                        as i32)
                                                        == (1 as i32)
                                                    {
                                                        unsafe {
                                                            sqlite3ErrorMsg(pParse, (b"UPSERT not implemented for virtual table \"%s\"\0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_576)).zName })
                                                        };
                                                        break '__join_4;
                                                    } else {
                                                        if (((unsafe {
                                                            (*(*__slate_slot_576)).eTabType
                                                        })
                                                            as u32)
                                                            as i32)
                                                            == (2 as i32)
                                                        {
                                                            unsafe {
                                                                sqlite3ErrorMsg(
                                                                    pParse,
                                                                    (b"cannot UPSERT a view\0"
                                                                        .as_ptr()
                                                                        as *mut i8)
                                                                        as *const i8,
                                                                )
                                                            };
                                                            break '__join_4;
                                                        } else {
                                                            if (unsafe {
                                                                sqlite3HasExplicitNulls(
                                                                    pParse,
                                                                    unsafe {
                                                                        (*pUpsert).pUpsertTarget
                                                                    },
                                                                )
                                                            }) != (0 as i32)
                                                            {
                                                                break '__join_4;
                                                            } else {
                                                                unsafe {
                                                                    (*unsafe {
                                                                        unsafe {
                                                                            std::ptr::addr_of_mut!(
                                                                                (*pTabList).a
                                                                            )
                                                                                as *mut SrcItem
                                                                        }
                                                                        .offset((0 as i32) as isize)
                                                                    })
                                                                    .iCursor = *__slate_slot_583;
                                                                }
                                                                *__slate_slot_619 = pUpsert;
                                                                loop {
                                                                    unsafe {
                                                                        (*(*__slate_slot_619))
                                                                            .pUpsertSrc = pTabList;
                                                                    }
                                                                    unsafe {
                                                                        (*(*__slate_slot_619))
                                                                            .regData =
                                                                            *__slate_slot_603;
                                                                    }
                                                                    unsafe {
                                                                        (*(*__slate_slot_619))
                                                                            .iDataCur =
                                                                            *__slate_slot_583;
                                                                    }
                                                                    unsafe {
                                                                        (*(*__slate_slot_619))
                                                                            .iIdxCur =
                                                                            *__slate_slot_584;
                                                                    }
                                                                    if (unsafe {
                                                                        (*(*__slate_slot_619))
                                                                            .pUpsertTarget
                                                                    }) != std::ptr::null_mut::<
                                                                        ExprList,
                                                                    >(
                                                                    ) {
                                                                        if (unsafe {
                                                                            sqlite3UpsertAnalyzeTarget(pParse, pTabList, *__slate_slot_619, pUpsert)
                                                                        }) != (0 as i32)
                                                                        {
                                                                            break '__join_4;
                                                                        }
                                                                    }
                                                                    *__slate_slot_619 = unsafe {
                                                                        (*(*__slate_slot_619))
                                                                            .pNextUpsert
                                                                    };
                                                                    if !(*__slate_slot_619
                                                                        != std::ptr::null_mut::<
                                                                            Upsert,
                                                                        >(
                                                                        ))
                                                                    {
                                                                        break '__join_89;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            // /* This is the top of the main insertion loop */
                                            if *__slate_slot_592 != (0 as u8) {
                                                // /* This block codes the top of loop only.  The complete loop is the
                                                //     ** following pseudocode (template 4):
                                                //     **
                                                //     **         rewind temp table, if empty goto D
                                                //     **      C: loop over rows of intermediate table
                                                //     **           transfer values form intermediate table into <table>
                                                //     **         end loop
                                                //     **      D: ...
                                                //     */
                                                *__slate_slot_588 = unsafe {
                                                    sqlite3VdbeAddOp1(
                                                        *__slate_slot_579,
                                                        36 as i32,
                                                        *__slate_slot_587,
                                                    )
                                                };
                                                {}
                                                *__slate_slot_589 = unsafe {
                                                    sqlite3VdbeCurrentAddr(*__slate_slot_579)
                                                };
                                            } else {
                                                if pSelect != std::ptr::null_mut::<Select>() {
                                                    // /* This block codes the top of loop only.  The complete loop is the
                                                    //     ** following pseudocode (template 3):
                                                    //     **
                                                    //     **      C: yield X, at EOF goto D
                                                    //     **         insert the select result into <table> from R..R+n
                                                    //     **         goto C
                                                    //     **      D: ...
                                                    //     */
                                                    {}
                                                    std::ptr::write(__slate_slot_1287, unsafe {
                                                        sqlite3VdbeAddOp1(
                                                            *__slate_slot_579,
                                                            12 as i32,
                                                            (*__slate_slot_590).iSDParm,
                                                        )
                                                    });
                                                    *__slate_slot_589 = *__slate_slot_1287;
                                                    *__slate_slot_588 = *__slate_slot_1287;
                                                    {}
                                                    if *__slate_slot_585 >= (0 as i32) {
                                                        // /* tag-20191021-001: If the INTEGER PRIMARY KEY is being generated by the
                                                        //       ** SELECT, go ahead and copy the value into the rowid slot now, so that
                                                        //       ** the value does not get overwritten by a NULL at tag-20191021-002. */
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_579,
                                                                82 as i32,
                                                                *__slate_slot_598
                                                                    + *__slate_slot_585,
                                                                *__slate_slot_602,
                                                            )
                                                        };
                                                    }
                                                }
                                            }
                                            // /* Compute data for ordinary columns of the new entry.  Values
                                            //   ** are written in storage order into registers starting with regData.
                                            //   ** Only ordinary columns are computed in this loop. The rowid
                                            //   ** (if there is one) is computed later and generated columns are
                                            //   ** computed after the rowid since they might depend on the value
                                            //   ** of the rowid.
                                            //   */
                                            *__slate_slot_582 = 0 as i32;
                                            *__slate_slot_597 = *__slate_slot_603;
                                            0 as i32;
                                            *__slate_slot_577 = 0 as i32;
                                            loop {
                                                if *__slate_slot_577
                                                    < ((unsafe { (*(*__slate_slot_576)).nCol })
                                                        as i32)
                                                {
                                                    '__join_59: {
                                                        0 as i32;
                                                        if *__slate_slot_577
                                                            == ((unsafe {
                                                                (*(*__slate_slot_576)).iPKey
                                                            })
                                                                as i32)
                                                        {
                                                            // /* tag-20191021-002: References to the INTEGER PRIMARY KEY are filled
                                                            //       ** using the rowid. So put a NULL in the IPK slot of the record to avoid
                                                            //       ** using excess space.  The file format definition requires this extra
                                                            //       ** NULL - we cannot optimize further by skipping the column completely */
                                                            unsafe {
                                                                sqlite3VdbeAddOp1(
                                                                    *__slate_slot_579,
                                                                    78 as i32,
                                                                    *__slate_slot_597,
                                                                )
                                                            };
                                                        } else {
                                                            std::ptr::write(
                                                                __slate_slot_1292,
                                                                (unsafe {
                                                                    (*unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_576))
                                                                                .aCol
                                                                        }
                                                                        .offset(
                                                                            *__slate_slot_577
                                                                                as isize,
                                                                        )
                                                                    })
                                                                    .colFlags
                                                                })
                                                                    as u32,
                                                            );
                                                            *__slate_slot_621 = *__slate_slot_1292;
                                                            if *__slate_slot_1292
                                                                & ((98 as i32) as u32)
                                                                != ((0 as i32) as u32)
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_1293,
                                                                    *__slate_slot_582,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1294,
                                                                    *__slate_slot_1293 + (1 as i32),
                                                                );
                                                                *__slate_slot_582 =
                                                                    *__slate_slot_1294;
                                                                if *__slate_slot_621
                                                                    & ((32 as i32) as u32)
                                                                    != ((0 as i32) as u32)
                                                                {
                                                                    // /* Virtual columns do not participate in OP_MakeRecord.  So back up
                                                                    //         ** iRegStore by one slot to compensate for the iRegStore++ in the
                                                                    //         ** outer for() loop */
                                                                    std::ptr::write(
                                                                        __slate_slot_1295,
                                                                        *__slate_slot_597,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_1296,
                                                                        *__slate_slot_1295
                                                                            - (1 as i32),
                                                                    );
                                                                    *__slate_slot_597 =
                                                                        *__slate_slot_1296;
                                                                    break '__join_59;
                                                                } else {
                                                                    if *__slate_slot_621
                                                                        & ((64 as i32) as u32)
                                                                        != ((0 as i32) as u32)
                                                                    {
                                                                        // /* Stored columns are computed later.  But if there are BEFORE
                                                                        //         ** triggers, the slots used for stored columns will be OP_Copy-ed
                                                                        //         ** to a second block of registers, so the register needs to be
                                                                        //         ** initialized to NULL to avoid an uninitialized register read */
                                                                        if *__slate_slot_608
                                                                            & (1 as i32)
                                                                            != (0 as i32)
                                                                        {
                                                                            unsafe {
                                                                                sqlite3VdbeAddOp1(*__slate_slot_579, 78 as i32, *__slate_slot_597)
                                                                            };
                                                                            break '__join_59;
                                                                        } else {
                                                                            break '__join_59;
                                                                        }
                                                                    } else {
                                                                        if pColumn
                                                                            == std::ptr::null_mut::<
                                                                                IdList,
                                                                            >(
                                                                            )
                                                                        {
                                                                            // /* Hidden columns that are not explicitly named in the INSERT
                                                                            //         ** get their default value */
                                                                            unsafe {
                                                                                sqlite3ExprCodeFactorable(pParse, unsafe { sqlite3ColumnExpr(*__slate_slot_576, unsafe { unsafe { (*(*__slate_slot_576)).aCol }.offset(*__slate_slot_577 as isize) }) }, *__slate_slot_597)
                                                                            };
                                                                            break '__join_59;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            if pColumn
                                                                != std::ptr::null_mut::<IdList>()
                                                            {
                                                                *__slate_slot_578 = unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_605).offset(
                                                                            *__slate_slot_577
                                                                                as isize,
                                                                        )
                                                                    }
                                                                };
                                                                0 as i32;
                                                                if *__slate_slot_578 == (0 as i32) {
                                                                    // /* A column not named in the insert column list gets its
                                                                    //         ** default value */
                                                                    unsafe {
                                                                        sqlite3ExprCodeFactorable(
                                                                            pParse,
                                                                            unsafe {
                                                                                sqlite3ColumnExpr(*__slate_slot_576, unsafe { unsafe { (*(*__slate_slot_576)).aCol }.offset(*__slate_slot_577 as isize) })
                                                                            },
                                                                            *__slate_slot_597,
                                                                        )
                                                                    };
                                                                    break '__join_59;
                                                                } else {
                                                                    *__slate_slot_620 =
                                                                        *__slate_slot_578
                                                                            - (1 as i32);
                                                                }
                                                            } else {
                                                                if *__slate_slot_581 == (0 as i32) {
                                                                    // /* This is INSERT INTO ... DEFAULT VALUES.  Load the default value. */
                                                                    unsafe {
                                                                        sqlite3ExprCodeFactorable(
                                                                            pParse,
                                                                            unsafe {
                                                                                sqlite3ColumnExpr(*__slate_slot_576, unsafe { unsafe { (*(*__slate_slot_576)).aCol }.offset(*__slate_slot_577 as isize) })
                                                                            },
                                                                            *__slate_slot_597,
                                                                        )
                                                                    };
                                                                    break '__join_59;
                                                                } else {
                                                                    *__slate_slot_620 =
                                                                        *__slate_slot_577
                                                                            - *__slate_slot_582;
                                                                }
                                                            }
                                                            if *__slate_slot_592 != (0 as u8) {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp3(
                                                                        *__slate_slot_579,
                                                                        96 as i32,
                                                                        *__slate_slot_587,
                                                                        *__slate_slot_620,
                                                                        *__slate_slot_597,
                                                                    )
                                                                };
                                                            } else {
                                                                if pSelect
                                                                    != std::ptr::null_mut::<Select>(
                                                                    )
                                                                {
                                                                    if *__slate_slot_598
                                                                        != *__slate_slot_603
                                                                    {
                                                                        unsafe {
                                                                            sqlite3VdbeAddOp2(*__slate_slot_579, 83 as i32, *__slate_slot_598 + *__slate_slot_620, *__slate_slot_597)
                                                                        };
                                                                    }
                                                                } else {
                                                                    std::ptr::write(
                                                                        __slate_slot_622,
                                                                        unsafe {
                                                                            (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_596)).a) as *mut ExprList_item }.offset(*__slate_slot_620 as isize) }).pExpr
                                                                        },
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_623,
                                                                        unsafe {
                                                                            sqlite3ExprCodeTarget(
                                                                                pParse,
                                                                                *__slate_slot_622,
                                                                                *__slate_slot_597,
                                                                            )
                                                                        },
                                                                    );
                                                                    if *__slate_slot_623
                                                                        != *__slate_slot_597
                                                                    {
                                                                        unsafe {
                                                                            sqlite3VdbeAddOp2(
                                                                                *__slate_slot_579,
                                                                                if (unsafe {
                                                                                    (*(*__slate_slot_622)).flags
                                                                                }) & ((4194304
                                                                                    as i32)
                                                                                    as u32)
                                                                                    != ((0 as i32)
                                                                                        as u32)
                                                                                {
                                                                                    82 as i32
                                                                                } else {
                                                                                    83 as i32
                                                                                },
                                                                                *__slate_slot_623,
                                                                                *__slate_slot_597,
                                                                            )
                                                                        };
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_1288,
                                                        *__slate_slot_577,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1289,
                                                        *__slate_slot_1288 + (1 as i32),
                                                    );
                                                    *__slate_slot_577 = *__slate_slot_1289;
                                                    std::ptr::write(
                                                        __slate_slot_1290,
                                                        *__slate_slot_597,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1291,
                                                        *__slate_slot_1290 + (1 as i32),
                                                    );
                                                    *__slate_slot_597 = *__slate_slot_1291;
                                                } else {
                                                    break;
                                                }
                                            }
                                            // /* Run the BEFORE and INSTEAD OF triggers, if there are any
                                            //   */
                                            *__slate_slot_586 =
                                                unsafe { sqlite3VdbeMakeLabel(pParse) };
                                            if *__slate_slot_608 & (1 as i32) != (0 as i32) {
                                                std::ptr::write(__slate_slot_624, unsafe {
                                                    sqlite3GetTempRange(
                                                        pParse,
                                                        ((unsafe { (*(*__slate_slot_576)).nCol })
                                                            as i32)
                                                            + (1 as i32),
                                                    )
                                                });
                                                // /* build the NEW.* reference row.  Note that if there is an INTEGER
                                                //     ** PRIMARY KEY into which a NULL is being inserted, that NULL will be
                                                //     ** translated into a unique ID for the row.  But on a BEFORE trigger,
                                                //     ** we do not know what the unique ID will be (because the insert has
                                                //     ** not happened yet) so we substitute a rowid of -1
                                                //     */
                                                if *__slate_slot_585 < (0 as i32) {
                                                    unsafe {
                                                        sqlite3VdbeAddOp2(
                                                            *__slate_slot_579,
                                                            73 as i32,
                                                            -(1 as i32),
                                                            *__slate_slot_624,
                                                        )
                                                    };
                                                } else {
                                                    0 as i32;
                                                    if *__slate_slot_592 != (0 as u8) {
                                                        unsafe {
                                                            sqlite3VdbeAddOp3(
                                                                *__slate_slot_579,
                                                                96 as i32,
                                                                *__slate_slot_587,
                                                                *__slate_slot_585,
                                                                *__slate_slot_624,
                                                            )
                                                        };
                                                    } else {
                                                        // /* Otherwise useTempTable is true */
                                                        0 as i32;
                                                        unsafe {
                                                            sqlite3ExprCode(
                                                                pParse,
                                                                unsafe {
                                                                    (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_596)).a) as *mut ExprList_item }.offset(*__slate_slot_585 as isize) }).pExpr
                                                                },
                                                                *__slate_slot_624,
                                                            )
                                                        };
                                                    }
                                                    *__slate_slot_625 = unsafe {
                                                        sqlite3VdbeAddOp1(
                                                            *__slate_slot_579,
                                                            52 as i32,
                                                            *__slate_slot_624,
                                                        )
                                                    };
                                                    {}
                                                    unsafe {
                                                        sqlite3VdbeAddOp2(
                                                            *__slate_slot_579,
                                                            73 as i32,
                                                            -(1 as i32),
                                                            *__slate_slot_624,
                                                        )
                                                    };
                                                    unsafe {
                                                        sqlite3VdbeJumpHere(
                                                            *__slate_slot_579,
                                                            *__slate_slot_625,
                                                        )
                                                    };
                                                    unsafe {
                                                        sqlite3VdbeAddOp1(
                                                            *__slate_slot_579,
                                                            13 as i32,
                                                            *__slate_slot_624,
                                                        )
                                                    };
                                                    {}
                                                }
                                                // /* Copy the new data already generated. */
                                                0 as i32;
                                                unsafe {
                                                    sqlite3VdbeAddOp3(
                                                        *__slate_slot_579,
                                                        82 as i32,
                                                        *__slate_slot_602 + (1 as i32),
                                                        *__slate_slot_624 + (1 as i32),
                                                        ((unsafe { (*(*__slate_slot_576)).nNVCol })
                                                            as i32)
                                                            - (1 as i32),
                                                    )
                                                };
                                                // /* Compute the new value for generated columns after all other
                                                //     ** columns have already been computed.  This must be done after
                                                //     ** computing the ROWID in case one of the generated columns
                                                //     ** refers to the ROWID. */
                                                if (unsafe { (*(*__slate_slot_576)).tabFlags })
                                                    & ((96 as i32) as u32)
                                                    != (0 as u32)
                                                {
                                                    {}
                                                    {}
                                                    sqlite3ComputeGeneratedColumns(
                                                        pParse,
                                                        *__slate_slot_624 + (1 as i32),
                                                        *__slate_slot_576,
                                                    );
                                                }
                                                // /* If this is an INSERT on a view with an INSTEAD OF INSERT trigger,
                                                //     ** do not attempt any conversions before assembling the record.
                                                //     ** If this is a real table, attempt conversions as required by the
                                                //     ** table column affinities.
                                                //     */
                                                if !(*__slate_slot_606 != (0 as i32)) {
                                                    sqlite3TableAffinity(
                                                        *__slate_slot_579,
                                                        *__slate_slot_576,
                                                        *__slate_slot_624 + (1 as i32),
                                                    );
                                                }
                                                // /* Fire BEFORE or INSTEAD OF triggers */
                                                unsafe {
                                                    sqlite3CodeRowTrigger(
                                                        pParse,
                                                        *__slate_slot_607,
                                                        128 as i32,
                                                        std::ptr::null_mut::<ExprList>(),
                                                        1 as i32,
                                                        *__slate_slot_576,
                                                        *__slate_slot_624
                                                            - ((unsafe {
                                                                (*(*__slate_slot_576)).nCol
                                                            })
                                                                as i32)
                                                            - (1 as i32),
                                                        onError,
                                                        *__slate_slot_586,
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3ReleaseTempRange(
                                                        pParse,
                                                        *__slate_slot_624,
                                                        ((unsafe { (*(*__slate_slot_576)).nCol })
                                                            as i32)
                                                            + (1 as i32),
                                                    )
                                                };
                                            }
                                            if !(*__slate_slot_606 != (0 as i32)) {
                                                if (((unsafe { (*(*__slate_slot_576)).eTabType })
                                                    as u32)
                                                    as i32)
                                                    == (1 as i32)
                                                {
                                                    // /* The row that the VUpdate opcode will delete: none */
                                                    unsafe {
                                                        sqlite3VdbeAddOp2(
                                                            *__slate_slot_579,
                                                            77 as i32,
                                                            0 as i32,
                                                            *__slate_slot_601,
                                                        )
                                                    };
                                                }
                                                if *__slate_slot_585 >= (0 as i32) {
                                                    // /* Compute the new rowid */
                                                    if *__slate_slot_592 != (0 as u8) {
                                                        unsafe {
                                                            sqlite3VdbeAddOp3(
                                                                *__slate_slot_579,
                                                                96 as i32,
                                                                *__slate_slot_587,
                                                                *__slate_slot_585,
                                                                *__slate_slot_602,
                                                            )
                                                        };
                                                    } else {
                                                        if pSelect != std::ptr::null_mut::<Select>()
                                                        {
                                                            // /* Rowid already initialized at tag-20191021-001 */
                                                        } else {
                                                            std::ptr::write(
                                                                __slate_slot_626,
                                                                unsafe {
                                                                    (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_596)).a) as *mut ExprList_item }.offset(*__slate_slot_585 as isize) }).pExpr
                                                                },
                                                            );
                                                            if (((unsafe {
                                                                (*(*__slate_slot_626)).op
                                                            })
                                                                as u32)
                                                                as i32)
                                                                == (122 as i32)
                                                                && !((((unsafe {
                                                                    (*(*__slate_slot_576)).eTabType
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    == (1 as i32))
                                                            {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp3(
                                                                        *__slate_slot_579,
                                                                        129 as i32,
                                                                        *__slate_slot_583,
                                                                        *__slate_slot_602,
                                                                        *__slate_slot_599,
                                                                    )
                                                                };
                                                                *__slate_slot_593 =
                                                                    ((1 as i32) as i8) as u8;
                                                            } else {
                                                                unsafe {
                                                                    sqlite3ExprCode(
                                                                        pParse,
                                                                        unsafe {
                                                                            (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_596)).a) as *mut ExprList_item }.offset(*__slate_slot_585 as isize) }).pExpr
                                                                        },
                                                                        *__slate_slot_602,
                                                                    )
                                                                };
                                                            }
                                                        }
                                                    }
                                                    // /* If the PRIMARY KEY expression is NULL, then use OP_NewRowid
                                                    //       ** to generate a unique primary key value.
                                                    //       */
                                                    if !(*__slate_slot_593 != (0 as u8)) {
                                                        if !((((unsafe {
                                                            (*(*__slate_slot_576)).eTabType
                                                        })
                                                            as u32)
                                                            as i32)
                                                            == (1 as i32))
                                                        {
                                                            *__slate_slot_627 = unsafe {
                                                                sqlite3VdbeAddOp1(
                                                                    *__slate_slot_579,
                                                                    52 as i32,
                                                                    *__slate_slot_602,
                                                                )
                                                            };
                                                            {}
                                                            unsafe {
                                                                sqlite3VdbeAddOp3(
                                                                    *__slate_slot_579,
                                                                    129 as i32,
                                                                    *__slate_slot_583,
                                                                    *__slate_slot_602,
                                                                    *__slate_slot_599,
                                                                )
                                                            };
                                                            unsafe {
                                                                sqlite3VdbeJumpHere(
                                                                    *__slate_slot_579,
                                                                    *__slate_slot_627,
                                                                )
                                                            };
                                                        } else {
                                                            *__slate_slot_627 = unsafe {
                                                                sqlite3VdbeCurrentAddr(
                                                                    *__slate_slot_579,
                                                                )
                                                            };
                                                            unsafe {
                                                                sqlite3VdbeAddOp2(
                                                                    *__slate_slot_579,
                                                                    51 as i32,
                                                                    *__slate_slot_602,
                                                                    *__slate_slot_627 + (2 as i32),
                                                                )
                                                            };
                                                            {}
                                                        }
                                                        unsafe {
                                                            sqlite3VdbeAddOp1(
                                                                *__slate_slot_579,
                                                                13 as i32,
                                                                *__slate_slot_602,
                                                            )
                                                        };
                                                        {}
                                                    }
                                                } else {
                                                    if (((unsafe {
                                                        (*(*__slate_slot_576)).eTabType
                                                    })
                                                        as u32)
                                                        as i32)
                                                        == (1 as i32)
                                                        || *__slate_slot_594 != (0 as u8)
                                                    {
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_579,
                                                                77 as i32,
                                                                0 as i32,
                                                                *__slate_slot_602,
                                                            )
                                                        };
                                                    } else {
                                                        unsafe {
                                                            sqlite3VdbeAddOp3(
                                                                *__slate_slot_579,
                                                                129 as i32,
                                                                *__slate_slot_583,
                                                                *__slate_slot_602,
                                                                *__slate_slot_599,
                                                            )
                                                        };
                                                        *__slate_slot_593 =
                                                            ((1 as i32) as i8) as u8;
                                                    }
                                                }
                                                autoIncStep(
                                                    pParse,
                                                    *__slate_slot_599,
                                                    *__slate_slot_602,
                                                );
                                                // /* Compute the new value for generated columns after all other
                                                //     ** columns have already been computed.  This must be done after
                                                //     ** computing the ROWID in case one of the generated columns
                                                //     ** is derived from the INTEGER PRIMARY KEY. */
                                                if (unsafe { (*(*__slate_slot_576)).tabFlags })
                                                    & ((96 as i32) as u32)
                                                    != (0 as u32)
                                                {
                                                    sqlite3ComputeGeneratedColumns(
                                                        pParse,
                                                        *__slate_slot_602 + (1 as i32),
                                                        *__slate_slot_576,
                                                    );
                                                }
                                                // /* Generate code to check constraints and generate index keys and
                                                //     ** do the insertion.
                                                //     */
                                                if (((unsafe { (*(*__slate_slot_576)).eTabType })
                                                    as u32)
                                                    as i32)
                                                    == (1 as i32)
                                                {
                                                    std::ptr::write(
                                                        __slate_slot_628,
                                                        (unsafe {
                                                            sqlite3GetVTable(
                                                                *__slate_slot_575,
                                                                *__slate_slot_576,
                                                            )
                                                        })
                                                            as *const i8,
                                                    );
                                                    unsafe {
                                                        sqlite3VtabMakeWritable(
                                                            pParse,
                                                            *__slate_slot_576,
                                                        )
                                                    };
                                                    unsafe {
                                                        sqlite3VdbeAddOp4(
                                                            *__slate_slot_579,
                                                            7 as i32,
                                                            1 as i32,
                                                            ((unsafe {
                                                                (*(*__slate_slot_576)).nCol
                                                            })
                                                                as i32)
                                                                + (2 as i32),
                                                            *__slate_slot_601,
                                                            *__slate_slot_628,
                                                            -(12 as i32),
                                                        )
                                                    };
                                                    unsafe {
                                                        sqlite3VdbeChangeP5(
                                                            *__slate_slot_579,
                                                            ((if onError == (11 as i32) {
                                                                2 as i32
                                                            } else {
                                                                onError
                                                            })
                                                                as i16)
                                                                as u16,
                                                        )
                                                    };
                                                    unsafe { sqlite3MayAbort(pParse) };
                                                } else {
                                                    // /* Set to true if constraints may cause a replace */
                                                    std::ptr::write(__slate_slot_629, 0 as i32);
                                                    // /* True to use OPFLAG_SEEKRESULT */
                                                    sqlite3GenerateConstraintChecks(
                                                        pParse,
                                                        *__slate_slot_576,
                                                        *__slate_slot_604,
                                                        *__slate_slot_583,
                                                        *__slate_slot_584,
                                                        *__slate_slot_601,
                                                        0 as i32,
                                                        (*__slate_slot_585 >= (0 as i32)) as u8,
                                                        (onError as i8) as u8,
                                                        *__slate_slot_586,
                                                        std::ptr::addr_of_mut!(*__slate_slot_629),
                                                        std::ptr::null_mut::<i32>(),
                                                        pUpsert,
                                                    );
                                                    if (unsafe { (*(*__slate_slot_575)).flags })
                                                        & (((16384 as i32) as i64) as u64)
                                                        != (0 as u64)
                                                    {
                                                        unsafe {
                                                            sqlite3FkCheck(
                                                                pParse,
                                                                *__slate_slot_576,
                                                                0 as i32,
                                                                *__slate_slot_601,
                                                                std::ptr::null_mut::<i32>(),
                                                                0 as i32,
                                                            )
                                                        };
                                                    }
                                                    // /* Set the OPFLAG_USESEEKRESULT flag if either (a) there are no REPLACE
                                                    //       ** constraints or (b) there are no triggers and this table is not a
                                                    //       ** parent table in a foreign key constraint. It is safe to set the
                                                    //       ** flag in the second case as if any REPLACE constraint is hit, an
                                                    //       ** OP_Delete or OP_IdxDelete instruction will be executed on each
                                                    //       ** cursor that is disturbed. And these instructions both clear the
                                                    //       ** VdbeCursor.seekResult variable, disabling the OPFLAG_USESEEKRESULT
                                                    //       ** functionality.  */
                                                    if *__slate_slot_629 == (0 as i32) {
                                                        *__slate_slot_1297 = true as bool;
                                                    } else {
                                                        *__slate_slot_1297 = !((unsafe {
                                                            sqlite3VdbeHasSubProgram(
                                                                *__slate_slot_579,
                                                            )
                                                        }) != (0 as i32));
                                                    }
                                                    *__slate_slot_630 = *__slate_slot_1297 as i32;
                                                    sqlite3CompleteInsertion(
                                                        pParse,
                                                        *__slate_slot_576,
                                                        *__slate_slot_583,
                                                        *__slate_slot_584,
                                                        *__slate_slot_601,
                                                        *__slate_slot_604,
                                                        0 as i32,
                                                        (*__slate_slot_593 as u32) as i32,
                                                        *__slate_slot_630,
                                                    );
                                                }
                                            }
                                            // /* Update the count of rows that are inserted
                                            //   */
                                            if *__slate_slot_600 != (0 as i32) {
                                                unsafe {
                                                    sqlite3VdbeAddOp2(
                                                        *__slate_slot_579,
                                                        88 as i32,
                                                        *__slate_slot_600,
                                                        1 as i32,
                                                    )
                                                };
                                            }
                                            if *__slate_slot_607 != std::ptr::null_mut::<Trigger>()
                                            {
                                                // /* Code AFTER triggers */
                                                unsafe {
                                                    sqlite3CodeRowTrigger(
                                                        pParse,
                                                        *__slate_slot_607,
                                                        128 as i32,
                                                        std::ptr::null_mut::<ExprList>(),
                                                        2 as i32,
                                                        *__slate_slot_576,
                                                        *__slate_slot_603
                                                            - (2 as i32)
                                                            - ((unsafe {
                                                                (*(*__slate_slot_576)).nCol
                                                            })
                                                                as i32),
                                                        onError,
                                                        *__slate_slot_586,
                                                    )
                                                };
                                            }
                                            // /* The bottom of the main insertion loop, if the data source
                                            //   ** is a SELECT statement.
                                            //   */
                                            unsafe {
                                                sqlite3VdbeResolveLabel(
                                                    *__slate_slot_579,
                                                    *__slate_slot_586,
                                                )
                                            };
                                            if *__slate_slot_592 != (0 as u8) {
                                                unsafe {
                                                    sqlite3VdbeAddOp2(
                                                        *__slate_slot_579,
                                                        40 as i32,
                                                        *__slate_slot_587,
                                                        *__slate_slot_589,
                                                    )
                                                };
                                                {}
                                                unsafe {
                                                    sqlite3VdbeJumpHere(
                                                        *__slate_slot_579,
                                                        *__slate_slot_588,
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3VdbeAddOp1(
                                                        *__slate_slot_579,
                                                        124 as i32,
                                                        *__slate_slot_587,
                                                    )
                                                };
                                            } else {
                                                if pSelect != std::ptr::null_mut::<Select>() {
                                                    unsafe {
                                                        sqlite3VdbeGoto(
                                                            *__slate_slot_579,
                                                            *__slate_slot_589,
                                                        )
                                                    };
                                                    unsafe {
                                                        sqlite3VdbeJumpHere(
                                                            *__slate_slot_579,
                                                            *__slate_slot_588,
                                                        )
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    if (((unsafe { (*pParse).nested }) as u32) as i32) == (0 as i32)
                                        && (unsafe { (*pParse).pTriggerTab })
                                            == std::ptr::null_mut::<Table>()
                                    {
                                        sqlite3AutoincrementEnd(pParse);
                                    }
                                    // /* SQLITE_OMIT_XFER_OPT */
                                    // /* Update the sqlite_sequence table by storing the content of the
                                    //   ** maximum rowid counter values recorded while inserting into
                                    //   ** autoincrement tables.
                                    //   */
                                    // /*
                                    //   ** Return the number of rows inserted. If this routine is
                                    //   ** generating code because of a call to sqlite3NestedParse(), do not
                                    //   ** invoke the callback function.
                                    //   */
                                    if *__slate_slot_600 != (0 as i32) {
                                        unsafe {
                                            sqlite3CodeChangeCount(
                                                *__slate_slot_579,
                                                *__slate_slot_600,
                                                (b"rows inserted\0".as_ptr() as *mut i8)
                                                    as *const i8,
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
        unsafe { sqlite3SrcListDelete(*__slate_slot_575, pTabList) };
        unsafe { sqlite3ExprListDelete(*__slate_slot_575, *__slate_slot_596) };
        unsafe { sqlite3UpsertDelete(*__slate_slot_575, pUpsert) };
        unsafe { sqlite3SelectDelete(*__slate_slot_575, pSelect) };
        if pColumn != std::ptr::null_mut::<IdList>() {
            unsafe { sqlite3IdListDelete(*__slate_slot_575, pColumn) };
            unsafe { sqlite3DbFree(*__slate_slot_575, *__slate_slot_605 as *mut ()) };
        }
        if *__slate_slot_604 != std::ptr::null_mut::<i32>() {
            unsafe { sqlite3DbNNFreeNN(*__slate_slot_575, *__slate_slot_604 as *mut ()) };
        }
    }
}

// /*
// ** All regular columns for table pTab have been puts into registers
// ** starting with iRegStore.  The registers that correspond to STORED
// ** or VIRTUAL columns have not yet been initialized.  This routine goes
// ** back and computes the values for those columns based on the previously
// ** computed normal columns.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ComputeGeneratedColumns(
    mut pParse: *mut Parse,
    mut iRegStore: i32,
    mut pTab: *mut Table,
) {
    let mut i: i32 = 0 as i32;
    let mut w: Walker = unsafe { std::mem::zeroed() };
    let mut pRedo: *mut Column = unsafe { std::mem::zeroed() };
    let mut eProgress: i32 = 0 as i32;
    let mut pOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
    0 as i32;
    {}
    {}
    // /* Before computing generated columns, first go through and make sure
    //   ** that appropriate affinity has been applied to the regular columns
    //   */
    sqlite3TableAffinity(unsafe { (*pParse).pVdbe }, pTab, iRegStore);
    if (unsafe { (*pTab).tabFlags }) & ((64 as i32) as u32) != ((0 as i32) as u32) {
        pOp = unsafe { sqlite3VdbeGetLastOp(unsafe { (*pParse).pVdbe }) };
        if (((unsafe { (*pOp).opcode }) as u32) as i32) == (98 as i32) {
            // /* Change the OP_Affinity argument to '@' (NONE) for all stored
            //       ** columns.  '@' is the no-op affinity and those columns have not
            //       ** yet been computed. */
            let mut ii: i32 = 0 as i32;
            let mut jj: i32 = 0 as i32;
            let mut zP4: *mut i8 = unsafe { (*pOp).p4.z };
            0 as i32;
            0 as i32;
            jj = 0 as i32;
            ii = 0 as i32;
            '__slate_break_1168: loop {
                if !((unsafe { *unsafe { zP4.offset(jj as isize) } }) != (0 as i8)) {
                    break;
                }
                if (((unsafe { (*unsafe { unsafe { (*pTab).aCol }.offset(ii as isize) }).colFlags })
                    as u32) as i32)
                    & (32 as i32)
                    != (0 as i32)
                {
                } else {
                    if (((unsafe {
                        (*unsafe { unsafe { (*pTab).aCol }.offset(ii as isize) }).colFlags
                    }) as u32) as i32)
                        & (64 as i32)
                        != (0 as i32)
                    {
                        unsafe {
                            *unsafe { zP4.offset(jj as isize) } = (64 as i32) as i8;
                        }
                    }
                    let __v1300: i32 = jj;
                    let __v1301: i32 = __v1300 + (1 as i32);
                    jj = __v1301;
                }
                let __v1298: i32 = ii;
                let __v1299: i32 = __v1298 + (1 as i32);
                ii = __v1299;
            }
        } else {
            if (((unsafe { (*pOp).opcode }) as u32) as i32) == (97 as i32) {
                // /* If an OP_TypeCheck was generated because the table is STRICT,
                //       ** then set the P3 operand to indicate that generated columns should
                //       ** not be checked */
                unsafe {
                    (*pOp).p3 = 1 as i32;
                }
            }
        }
    }
    // /* Because there can be multiple generated columns that refer to one another,
    //   ** this is a two-pass algorithm.  On the first pass, mark all generated
    //   ** columns as "not available".
    //   */
    i = 0 as i32;
    '__slate_break_1169: loop {
        if !(i < ((unsafe { (*pTab).nCol }) as i32)) {
            break;
        }
        if (((unsafe { (*unsafe { unsafe { (*pTab).aCol }.offset(i as isize) }).colFlags }) as u32)
            as i32)
            & (96 as i32)
            != (0 as i32)
        {
            {}
            {}
            let __v1304: *mut Column = unsafe { unsafe { (*pTab).aCol }.offset(i as isize) };
            let __v1305: u16 = unsafe { (*__v1304).colFlags };
            let __v1306: u16 = ((((__v1305 as u32) as i32) | (128 as i32)) as i16) as u16;
            unsafe {
                (*__v1304).colFlags = __v1306;
            }
        }
        let __v1302: i32 = i;
        let __v1303: i32 = __v1302 + (1 as i32);
        i = __v1303;
    }
    unsafe {
        w.u.pTab = pTab;
    }
    w.xExprCallback = Some(exprColumnFlagUnion);
    w.xSelectCallback = None;
    w.xSelectCallback2 = None;
    // /* On the second pass, compute the value of each NOT-AVAILABLE column.
    //   ** Companion code in the TK_COLUMN case of sqlite3ExprCodeTarget() will
    //   ** compute dependencies and mark remove the COLSPAN_NOTAVAIL mark, as
    //   ** they are needed.
    //   */
    unsafe {
        (*pParse).iSelfTab = -iRegStore;
    }
    '__slate_break_1170: loop {
        eProgress = 0 as i32;
        pRedo = std::ptr::null_mut::<Column>();
        i = 0 as i32;
        '__slate_break_1171: loop {
            if !(i < ((unsafe { (*pTab).nCol }) as i32)) {
                break;
            }
            let mut pCol: *mut Column = unsafe { unsafe { (*pTab).aCol }.offset(i as isize) };
            if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (128 as i32) != (0 as i32) {
                let mut x: i32 = 0 as i32;
                let __v1309: *mut Column = pCol;
                let __v1310: u16 = unsafe { (*__v1309).colFlags };
                let __v1311: u16 = ((((__v1310 as u32) as i32) | (256 as i32)) as i16) as u16;
                unsafe {
                    (*__v1309).colFlags = __v1311;
                }
                w.eCode = ((0 as i32) as i16) as u16;
                unsafe {
                    sqlite3WalkExpr(std::ptr::addr_of_mut!(w), unsafe {
                        sqlite3ColumnExpr(pTab, pCol)
                    })
                };
                let __v1312: *mut Column = pCol;
                let __v1313: u16 = unsafe { (*__v1312).colFlags };
                let __v1314: u16 = ((((__v1313 as u32) as i32) & !(256 as i32)) as i16) as u16;
                unsafe {
                    (*__v1312).colFlags = __v1314;
                }
                if ((w.eCode as u32) as i32) & (128 as i32) != (0 as i32) {
                    pRedo = pCol;
                } else {
                    eProgress = 1 as i32;
                    0 as i32;
                    x = ((unsafe { sqlite3TableColumnToStorage(pTab, i as i16) }) as i32)
                        + iRegStore;
                    unsafe { sqlite3ExprCodeGeneratedColumn(pParse, pTab, pCol, x) };
                    let __v1315: *mut Column = pCol;
                    let __v1316: u16 = unsafe { (*__v1315).colFlags };
                    let __v1317: u16 = ((((__v1316 as u32) as i32) & !(128 as i32)) as i16) as u16;
                    unsafe {
                        (*__v1315).colFlags = __v1317;
                    }
                }
            }
            let __v1307: i32 = i;
            let __v1308: i32 = __v1307 + (1 as i32);
            i = __v1308;
        }
        if !(pRedo != std::ptr::null_mut::<Column>() && eProgress != (0 as i32)) {
            break;
        }
    }
    if pRedo != std::ptr::null_mut::<Column>() {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"generated column loop on \"%s\"\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pRedo).zCnName },
            )
        };
    }
    unsafe {
        (*pParse).iSelfTab = 0 as i32;
    }
}

// /*
// ** Generate code that will
// **
// **   (1) acquire a lock for table pTab then
// **   (2) open pTab as cursor iCur.
// **
// ** If pTab is a WITHOUT ROWID table, then it is the PRIMARY KEY index
// ** for that table that is actually opened.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OpenTable(
    mut pParse: *mut Parse,
    mut iCur: i32,
    mut iDb: i32,
    mut pTab: *mut Table,
    mut opcode: i32,
) {
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    v = unsafe { (*pParse).pVdbe };
    0 as i32;
    if !((unsafe { (*unsafe { (*pParse).db }).noSharedCache }) != (0 as u8)) {
        unsafe {
            sqlite3TableLock(
                pParse,
                iDb,
                unsafe { (*pTab).tnum },
                ((if opcode == (116 as i32) {
                    1 as i32
                } else {
                    0 as i32
                }) as i8) as u8,
                (unsafe { (*pTab).zName }) as *const i8,
            )
        };
    }
    if (unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
        unsafe {
            sqlite3VdbeAddOp4Int(
                v,
                opcode,
                iCur,
                (unsafe { (*pTab).tnum }) as i32,
                iDb,
                (unsafe { (*pTab).nNVCol }) as i32,
            )
        };
        {}
    } else {
        let mut pPk: *mut Index = unsafe { sqlite3PrimaryKeyIndex(pTab) };
        0 as i32;
        0 as i32;
        unsafe { sqlite3VdbeAddOp3(v, opcode, iCur, (unsafe { (*pPk).tnum }) as i32, iDb) };
        unsafe { sqlite3VdbeSetP4KeyInfo(pParse, pPk) };
        {}
    }
}

// /*
// ** pExpr is a CHECK constraint on a row that is being UPDATE-ed.  The
// ** only columns that are modified by the UPDATE are those for which
// ** aiChng[i]>=0, and also the ROWID is modified if chngRowid is true.
// **
// ** Return true if CHECK constraint pExpr uses any of the
// ** changing columns (or the rowid if it is changing).  In other words,
// ** return true if this CHECK constraint must be validated for
// ** the new row in the UPDATE statement.
// **
// ** 2018-09-15: pExpr might also be an expression for an index-on-expressions.
// ** The operation of this routine is the same - return true if an only if
// ** the expression uses one or more of columns identified by the second and
// ** third arguments.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprReferencesUpdatedColumn(
    mut pExpr: *mut Expr,
    mut aiChng: *mut i32,
    mut chngRowid: i32,
) -> i32 {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    unsafe { memset(std::ptr::addr_of_mut!(w) as *mut (), 0 as i32, 48 as u64) };
    w.eCode = ((0 as i32) as i16) as u16;
    w.xExprCallback = Some(checkConstraintExprNode);
    unsafe {
        w.u.aiCol = aiChng;
    }
    unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(w), pExpr) };
    if !(chngRowid != (0 as i32)) {
        {}
        let __v1318: u16 = w.eCode;
        let __v1319: u16 = ((((__v1318 as u32) as i32) & !(2 as i32)) as i16) as u16;
        w.eCode = __v1319;
    }
    {}
    {}
    {}
    {}
    return (((w.eCode as u32) as i32) != (0 as i32)) as i32;
}

// /*
// ** Generate code to do constraint checks prior to an INSERT or an UPDATE
// ** on table pTab.
// **
// ** The regNewData parameter is the first register in a range that contains
// ** the data to be inserted or the data after the update.  There will be
// ** pTab->nCol+1 registers in this range.  The first register (the one
// ** that regNewData points to) will contain the new rowid, or NULL in the
// ** case of a WITHOUT ROWID table.  The second register in the range will
// ** contain the content of the first table column.  The third register will
// ** contain the content of the second table column.  And so forth.
// **
// ** The regOldData parameter is similar to regNewData except that it contains
// ** the data prior to an UPDATE rather than afterwards.  regOldData is zero
// ** for an INSERT.  This routine can distinguish between UPDATE and INSERT by
// ** checking regOldData for zero.
// **
// ** For an UPDATE, the pkChng boolean is true if the true primary key (the
// ** rowid for a normal table or the PRIMARY KEY for a WITHOUT ROWID table)
// ** might be modified by the UPDATE.  If pkChng is false, then the key of
// ** the iDataCur content table is guaranteed to be unchanged by the UPDATE.
// **
// ** For an INSERT, the pkChng boolean indicates whether or not the rowid
// ** was explicitly specified as part of the INSERT statement.  If pkChng
// ** is zero, it means that the either rowid is computed automatically or
// ** that the table is a WITHOUT ROWID table and has no rowid.  On an INSERT,
// ** pkChng will only be true if the INSERT statement provides an integer
// ** value for either the rowid column or its INTEGER PRIMARY KEY alias.
// **
// ** The code generated by this routine will store new index entries into
// ** registers identified by aRegIdx[].  No index entry is created for
// ** indices where aRegIdx[i]==0.  The order of indices in aRegIdx[] is
// ** the same as the order of indices on the linked list of indices
// ** at pTab->pIndex.
// **
// ** (2019-05-07) The generated code also creates a new record for the
// ** main table, if pTab is a rowid table, and stores that record in the
// ** register identified by aRegIdx[nIdx] - in other words in the first
// ** entry of aRegIdx[] past the last index.  It is important that the
// ** record be generated during constraint checks to avoid affinity changes
// ** to the register content that occur after constraint checks but before
// ** the new record is inserted.
// **
// ** The caller must have already opened writeable cursors on the main
// ** table and all applicable indices (that is to say, all indices for which
// ** aRegIdx[] is not zero).  iDataCur is the cursor for the main table when
// ** inserting or updating a rowid table, or the cursor for the PRIMARY KEY
// ** index when operating on a WITHOUT ROWID table.  iIdxCur is the cursor
// ** for the first index in the pTab->pIndex list.  Cursors for other indices
// ** are at iIdxCur+N for the N-th element of the pTab->pIndex list.
// **
// ** This routine also generates code to check constraints.  NOT NULL,
// ** CHECK, and UNIQUE constraints are all checked.  If a constraint fails,
// ** then the appropriate action is performed.  There are five possible
// ** actions: ROLLBACK, ABORT, FAIL, REPLACE, and IGNORE.
// **
// **  Constraint type  Action       What Happens
// **  ---------------  ----------   ----------------------------------------
// **  any              ROLLBACK     The current transaction is rolled back and
// **                                sqlite3_step() returns immediately with a
// **                                return code of SQLITE_CONSTRAINT.
// **
// **  any              ABORT        Back out changes from the current command
// **                                only (do not do a complete rollback) then
// **                                cause sqlite3_step() to return immediately
// **                                with SQLITE_CONSTRAINT.
// **
// **  any              FAIL         Sqlite3_step() returns immediately with a
// **                                return code of SQLITE_CONSTRAINT.  The
// **                                transaction is not rolled back and any
// **                                changes to prior rows are retained.
// **
// **  any              IGNORE       The attempt in insert or update the current
// **                                row is skipped, without throwing an error.
// **                                Processing continues with the next row.
// **                                (There is an immediate jump to ignoreDest.)
// **
// **  NOT NULL         REPLACE      The NULL value is replace by the default
// **                                value for that column.  If the default value
// **                                is NULL, the action is the same as ABORT.
// **
// **  UNIQUE           REPLACE      The other row that conflicts with the row
// **                                being inserted is removed.
// **
// **  CHECK            REPLACE      Illegal.  The results in an exception.
// **
// ** Which action to take is determined by the overrideError parameter.
// ** Or if overrideError==OE_Default, then the pParse->onError parameter
// ** is used.  Or if pParse->onError==OE_Default then the onError value
// ** for the constraint is used.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GenerateConstraintChecks(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut aRegIdx: *mut i32,
    mut iDataCur: i32,
    mut iIdxCur: i32,
    mut regNewData: i32,
    mut regOldData: i32,
    mut pkChng: u8,
    mut overrideError: u8,
    mut ignoreDest: i32,
    mut pbMayReplace: *mut i32,
    mut aiChng: *mut i32,
    mut pUpsert: *mut Upsert,
) {
    // /* VDBE under construction */
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    // /* Pointer to one of the indices */
    let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
    // /* The PRIMARY KEY index for WITHOUT ROWID tables */
    let mut pPk: *mut Index = std::ptr::null_mut::<Index>();
    // /* Database connection */
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    // /* loop counter */
    let mut i: i32 = 0 as i32;
    // /* Index loop counter */
    let mut ix: i32 = 0 as i32;
    // /* Number of columns */
    let mut nCol: i32 = 0 as i32;
    // /* Conflict resolution strategy */
    let mut onError: i32 = 0 as i32;
    // /* True if REPLACE is used to resolve INT PK conflict */
    let mut seenReplace: i32 = 0 as i32;
    // /* Number of fields in PRIMARY KEY. 1 for ROWID tables */
    let mut nPkField: i32 = 0 as i32;
    // /* The specific ON CONFLICT clause for pIdx */
    let mut pUpsertClause: *mut Upsert = std::ptr::null_mut::<Upsert>();
    // /* True if this is an UPDATE operation */
    let mut isUpdate: u8 = 0 as u8;
    // /* True if the OP_Affinity operation has been run */
    let mut bAffinityDone: u8 = ((0 as i32) as i8) as u8;
    // /* Address of Goto at end of IPK uniqueness check */
    let mut upsertIpkReturn: i32 = 0 as i32;
    // /* Address of Goto to bypass initial IPK check */
    let mut upsertIpkDelay: i32 = 0 as i32;
    // /* Top of the IPK uniqueness check */
    let mut ipkTop: i32 = 0 as i32;
    // /* OP_Goto at the end of the IPK uniqueness check */
    let mut ipkBottom: i32 = 0 as i32;
    // /* Variables associated with retesting uniqueness constraints after
    //   ** replace triggers fire have run */
    // /* Register used to count replace trigger invocations */
    let mut regTrigCnt: i32 = 0 as i32;
    // /* Jump here to recheck all uniqueness constraints */
    let mut addrRecheck: i32 = 0 as i32;
    // /* Each recheck jumps to this label if it passes */
    let mut lblRecheckOk: i32 = 0 as i32;
    // /* List of DELETE triggers on the table pTab */
    let mut pTrigger: *mut Trigger = unsafe { std::mem::zeroed() };
    // /* Number of replace triggers coded */
    let mut nReplaceTrig: i32 = 0 as i32;
    // /* Index iterator */
    let mut sIdxIter: IndexIterator = unsafe { std::mem::zeroed() };
    isUpdate = (regOldData != (0 as i32)) as u8;
    db = unsafe { (*pParse).db };
    v = unsafe { (*pParse).pVdbe };
    0 as i32;
    // /* This table is not a VIEW */
    0 as i32;
    nCol = (unsafe { (*pTab).nCol }) as i32;
    // /* pPk is the PRIMARY KEY index for WITHOUT ROWID tables and NULL for
    //   ** normal rowid tables.  nPkField is the number of key fields in the
    //   ** pPk index or 1 for a rowid table.  In other words, nPkField is the
    //   ** number of fields in the true primary key of the table. */
    if (unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
        pPk = std::ptr::null_mut::<Index>();
        nPkField = 1 as i32;
    } else {
        pPk = unsafe { sqlite3PrimaryKeyIndex(pTab) };
        nPkField = ((unsafe { (*pPk).nKeyCol }) as u32) as i32;
    }
    // /* Record that this module has started */
    {}
    // /* Test all NOT NULL constraints.
    //   */
    if (unsafe { (*pTab).tabFlags }) & ((2048 as i32) as u32) != (0 as u32) {
        // /* True if currently running 2nd pass */
        let mut b2ndPass: i32 = 0 as i32;
        // /* Number of ON CONFLICT REPLACE operations */
        let mut nSeenReplace: i32 = 0 as i32;
        // /* Number of generated columns with NOT NULL */
        let mut nGenerated: i32 = 0 as i32;
        // /* Make 2 passes over columns. Exit loop via "break" */
        '__slate_break_1196: while (1 as i32) != (0 as i32) {
            i = 0 as i32;
            '__slate_break_1197: loop {
                if !(i < nCol) {
                    break;
                }
                '__slate_continue_1197: {
                    // /* Register holding column value */
                    let mut iReg: i32 = 0 as i32;
                    // /* The column to check for NOT NULL */
                    let mut pCol: *mut Column =
                        unsafe { unsafe { (*pTab).aCol }.offset(i as isize) };
                    // /* non-zero if column is generated */
                    let mut isGenerated: i32 = 0 as i32;
                    onError = (unsafe { (*pCol).__slate_bits_0.__get_notNull() }) as i32;
                    // /* No NOT NULL on this column */
                    if onError == (0 as i32) {
                    } else {
                        if i == ((unsafe { (*pTab).iPKey }) as i32) {
                            // /* ROWID is never NULL */
                        } else {
                            isGenerated =
                                (((unsafe { (*pCol).colFlags }) as u32) as i32) & (96 as i32);
                            if isGenerated != (0 as i32) && !(b2ndPass != (0 as i32)) {
                                let __v1322: i32 = nGenerated;
                                let __v1323: i32 = __v1322 + (1 as i32);
                                nGenerated = __v1323;
                            // /* Generated columns processed on 2nd pass */
                            } else {
                                if aiChng != std::ptr::null_mut::<i32>()
                                    && (unsafe { *unsafe { aiChng.offset(i as isize) } })
                                        < (0 as i32)
                                    && !(isGenerated != (0 as i32))
                                {
                                    // /* Do not check NOT NULL on columns that do not change */
                                } else {
                                    if ((overrideError as u32) as i32) != (11 as i32) {
                                        onError = (overrideError as u32) as i32;
                                    } else {
                                        if onError == (11 as i32) {
                                            onError = 2 as i32;
                                        }
                                    }
                                    if onError == (5 as i32) {
                                        // /* REPLACE becomes ABORT on the 2nd pass */
                                        if b2ndPass != (0 as i32)
                                            || (((unsafe { (*pCol).iDflt }) as u32) as i32)
                                                == (0 as i32)
                                        {
                                            {}
                                            {}
                                            {}
                                            onError = 2 as i32;
                                        } else {
                                            0 as i32;
                                        }
                                    // /* REPLACE is ABORT if no DEFAULT value */
                                    } else {
                                        if b2ndPass != (0 as i32) && !(isGenerated != (0 as i32)) {
                                            break '__slate_continue_1197;
                                        }
                                    }
                                    0 as i32;
                                    {}
                                    iReg = ((unsafe { sqlite3TableColumnToStorage(pTab, i as i16) })
                                        as i32)
                                        + regNewData
                                        + (1 as i32);
                                    '__slate_break_1198: {
                                        match onError {
                                            5 => {
                                                let mut addr1: i32 = unsafe {
                                                    sqlite3VdbeAddOp1(v, 52 as i32, iReg)
                                                };
                                                {}
                                                0 as i32;
                                                let __v1324: i32 = nSeenReplace;
                                                let __v1325: i32 = __v1324 + (1 as i32);
                                                nSeenReplace = __v1325;
                                                unsafe {
                                                    sqlite3ExprCodeCopy(
                                                        pParse,
                                                        unsafe { sqlite3ColumnExpr(pTab, pCol) },
                                                        iReg,
                                                    )
                                                };
                                                unsafe { sqlite3VdbeJumpHere(v, addr1) };
                                            }
                                            2 => {
                                                unsafe { sqlite3MayAbort(pParse) };
                                                // /* no break */
                                                {}
                                                let mut zMsg: *mut i8 = unsafe {
                                                    sqlite3MPrintf(
                                                        db,
                                                        (b"%s.%s\0".as_ptr() as *mut i8)
                                                            as *const i8,
                                                        unsafe { (*pTab).zName },
                                                        unsafe { (*pCol).zCnName },
                                                    )
                                                };
                                                {}
                                                unsafe {
                                                    sqlite3VdbeAddOp3(
                                                        v,
                                                        71 as i32,
                                                        (19 as i32) | (5 as i32) << (8 as i32),
                                                        onError,
                                                        iReg,
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3VdbeAppendP4(
                                                        v,
                                                        zMsg as *mut (),
                                                        -(7 as i32),
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3VdbeChangeP5(
                                                        v,
                                                        ((1 as i32) as i16) as u16,
                                                    )
                                                };
                                                {}
                                            }
                                            1 | 3 => {
                                                let mut zMsg: *mut i8 = unsafe {
                                                    sqlite3MPrintf(
                                                        db,
                                                        (b"%s.%s\0".as_ptr() as *mut i8)
                                                            as *const i8,
                                                        unsafe { (*pTab).zName },
                                                        unsafe { (*pCol).zCnName },
                                                    )
                                                };
                                                {}
                                                unsafe {
                                                    sqlite3VdbeAddOp3(
                                                        v,
                                                        71 as i32,
                                                        (19 as i32) | (5 as i32) << (8 as i32),
                                                        onError,
                                                        iReg,
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3VdbeAppendP4(
                                                        v,
                                                        zMsg as *mut (),
                                                        -(7 as i32),
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3VdbeChangeP5(
                                                        v,
                                                        ((1 as i32) as i16) as u16,
                                                    )
                                                };
                                                {}
                                            }
                                            _ => {
                                                0 as i32;
                                                unsafe {
                                                    sqlite3VdbeAddOp2(
                                                        v, 51 as i32, iReg, ignoreDest,
                                                    )
                                                };
                                                {}
                                                break '__slate_break_1198;
                                                // /* end switch(onError) */
                                            }
                                        }
                                    }
                                    // /* end loop i over columns */
                                }
                            }
                        }
                    }
                }
                let __v1320: i32 = i;
                let __v1321: i32 = __v1320 + (1 as i32);
                i = __v1321;
            }
            if nGenerated == (0 as i32) && nSeenReplace == (0 as i32) {
                // /* If there are no generated columns with NOT NULL constraints
                //         ** and no NOT NULL ON CONFLICT REPLACE constraints, then a single
                //         ** pass is sufficient */
                break '__slate_break_1196;
            }
            // /* Never need more than 2 passes */
            if b2ndPass != (0 as i32) {
                break '__slate_break_1196;
            }
            b2ndPass = 1 as i32;
            if nSeenReplace > (0 as i32)
                && (unsafe { (*pTab).tabFlags }) & ((96 as i32) as u32) != ((0 as i32) as u32)
            {
                // /* If any NOT NULL ON CONFLICT REPLACE constraints fired on the
                //         ** first pass, recomputed values for all generated columns, as
                //         ** those values might depend on columns affected by the REPLACE.
                //         */
                sqlite3ComputeGeneratedColumns(pParse, regNewData + (1 as i32), pTab);
            }
            // /* end of 2-pass loop */
        }
        // /* end if( has-not-null-constraints ) */
    }
    // /* Test all CHECK constraints
    //   */
    if (unsafe { (*pTab).pCheck }) != std::ptr::null_mut::<ExprList>()
        && (unsafe { (*db).flags }) & (((512 as i32) as i64) as u64) == (((0 as i32) as i64) as u64)
    {
        let mut pCheck: *mut ExprList = unsafe { (*pTab).pCheck };
        unsafe {
            (*pParse).iSelfTab = -(regNewData + (1 as i32));
        }
        onError = if ((overrideError as u32) as i32) != (11 as i32) {
            (overrideError as u32) as i32
        } else {
            2 as i32
        };
        i = 0 as i32;
        '__slate_break_1200: loop {
            if !(i < unsafe { (*pCheck).nExpr }) {
                break;
            }
            let mut allOk: i32 = 0 as i32;
            let mut pCopy: *mut Expr = unsafe { std::mem::zeroed() };
            let mut pExpr: *mut Expr = unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pCheck).a) as *mut ExprList_item }
                        .offset(i as isize)
                })
                .pExpr
            };
            let __v1328: bool;
            if aiChng != std::ptr::null_mut::<i32>() {
                __v1328 =
                    !(sqlite3ExprReferencesUpdatedColumn(pExpr, aiChng, (pkChng as u32) as i32)
                        != (0 as i32));
            } else {
                __v1328 = false as bool;
            }
            if __v1328 {
                // /* The check constraints do not reference any of the columns being
                //         ** updated so there is no point it verifying the check constraint */
            } else {
                if ((bAffinityDone as u32) as i32) == (0 as i32) {
                    sqlite3TableAffinity(v, pTab, regNewData + (1 as i32));
                    bAffinityDone = ((1 as i32) as i8) as u8;
                }
                allOk = unsafe { sqlite3VdbeMakeLabel(pParse) };
                {}
                pCopy = unsafe { sqlite3ExprDup(db, pExpr as *const Expr, 0 as i32) };
                if !((unsafe { (*db).mallocFailed }) != (0 as u8)) {
                    unsafe { sqlite3ExprIfTrue(pParse, pCopy, allOk, 16 as i32) };
                }
                unsafe { sqlite3ExprDelete(db, pCopy) };
                if onError == (4 as i32) {
                    unsafe { sqlite3VdbeGoto(v, ignoreDest) };
                } else {
                    let mut zName: *mut i8 = unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pCheck).a) as *mut ExprList_item }
                                .offset(i as isize)
                        })
                        .zEName
                    };
                    0 as i32;
                    // /* IMP: R-26383-51744 */
                    if onError == (5 as i32) {
                        onError = 2 as i32;
                    }
                    unsafe {
                        sqlite3HaltConstraint(
                            pParse,
                            (19 as i32) | (1 as i32) << (8 as i32),
                            onError,
                            zName,
                            (0 as i32) as i8,
                            ((3 as i32) as i8) as u8,
                        )
                    };
                }
                unsafe { sqlite3VdbeResolveLabel(v, allOk) };
            }
            let __v1326: i32 = i;
            let __v1327: i32 = __v1326 + (1 as i32);
            i = __v1327;
        }
        unsafe {
            (*pParse).iSelfTab = 0 as i32;
        }
    }
    // /* !defined(SQLITE_OMIT_CHECK) */
    // /* UNIQUE and PRIMARY KEY constraints should be handled in the following
    //   ** order:
    //   **
    //   **   (1)  OE_Update
    //   **   (2)  OE_Abort, OE_Fail, OE_Rollback, OE_Ignore
    //   **   (3)  OE_Replace
    //   **
    //   ** OE_Fail and OE_Ignore must happen before any changes are made.
    //   ** OE_Update guarantees that only a single row will change, so it
    //   ** must happen before OE_Replace.  Technically, OE_Abort and OE_Rollback
    //   ** could happen in any order, but they are grouped up front for
    //   ** convenience.
    //   **
    //   ** 2018-08-14: Ticket https://sqlite.org/src/info/908f001483982c43
    //   ** The order of constraints used to have OE_Update as (2) and OE_Abort
    //   ** and so forth as (1). But apparently PostgreSQL checks the OE_Update
    //   ** constraint before any others, so it had to be moved.
    //   **
    //   ** Constraint checking code is generated in this order:
    //   **   (A)  The rowid constraint
    //   **   (B)  Unique index constraints that do not have OE_Replace as their
    //   **        default conflict resolution strategy
    //   **   (C)  Unique index that do use OE_Replace by default.
    //   **
    //   ** The ordering of (2) and (3) is accomplished by making sure the linked
    //   ** list of indexes attached to a table puts all OE_Replace indexes last
    //   ** in the list.  See sqlite3CreateIndex() for where that happens.
    //   */
    sIdxIter.eType = 0 as i32;
    sIdxIter.i = 0 as i32;
    // /* Silence harmless compiler warning */
    unsafe {
        sIdxIter.u.ax.aIdx = std::ptr::null_mut::<IndexListTerm>();
    }
    unsafe {
        sIdxIter.u.lx.pIdx = unsafe { (*pTab).pIndex };
    }
    if pUpsert != std::ptr::null_mut::<Upsert>() {
        if (unsafe { (*pUpsert).pUpsertTarget }) == std::ptr::null_mut::<ExprList>() {
            // /* There is just on ON CONFLICT clause and it has no constraint-target */
            0 as i32;
            if (((unsafe { (*pUpsert).isDoUpdate }) as u32) as i32) == (0 as i32) {
                // /* A single ON CONFLICT DO NOTHING clause, without a constraint-target.
                //         ** Make all unique constraint resolution be OE_Ignore */
                overrideError = ((4 as i32) as i8) as u8;
                pUpsert = std::ptr::null_mut::<Upsert>();
            } else {
                // /* A single ON CONFLICT DO UPDATE.  Make all resolutions OE_Update */
                overrideError = ((6 as i32) as i8) as u8;
            }
        } else {
            if (unsafe { (*pTab).pIndex }) != std::ptr::null_mut::<Index>() {
                // /* Otherwise, we'll need to run the IndexListTerm array version of the
                //       ** iterator to ensure that all of the ON CONFLICT conditions are
                //       ** checked first and in order. */
                let mut nIdx: i32 = 0 as i32;
                let mut jj: i32 = 0 as i32;
                let mut nByte: u64 = 0 as u64;
                let mut pTerm: *mut Upsert = unsafe { std::mem::zeroed() };
                let mut bUsed: *mut u8 = unsafe { std::mem::zeroed() };
                nIdx = 0 as i32;
                let __v1329: *mut Index = unsafe { (*pTab).pIndex };
                pIdx = __v1329;
                '__slate_break_1201: loop {
                    if !(pIdx != std::ptr::null_mut::<Index>()) {
                        break;
                    }
                    0 as i32;
                    pIdx = unsafe { (*pIdx).pNext };
                    let __v1330: i32 = nIdx;
                    let __v1331: i32 = __v1330 + (1 as i32);
                    nIdx = __v1331;
                }
                sIdxIter.eType = 1 as i32;
                unsafe {
                    sIdxIter.u.ax.nIdx = nIdx;
                }
                nByte = (16 as u64)
                    .wrapping_add(((1 as i32) as i64) as u64)
                    .wrapping_mul((nIdx as i64) as u64)
                    .wrapping_add((nIdx as i64) as u64);
                unsafe {
                    sIdxIter.u.ax.aIdx =
                        (unsafe { sqlite3DbMallocZero(db, nByte) }) as *mut IndexListTerm;
                }
                // /* OOM */
                if (unsafe { sIdxIter.u.ax.aIdx }) == std::ptr::null_mut::<IndexListTerm>() {
                    return;
                }
                bUsed = (unsafe { unsafe { sIdxIter.u.ax.aIdx }.offset(nIdx as isize) }) as *mut u8;
                unsafe {
                    (*pUpsert).pToFree = (unsafe { sIdxIter.u.ax.aIdx }) as *mut ();
                }
                i = 0 as i32;
                let __v1332: *mut Upsert = pUpsert;
                pTerm = __v1332;
                '__slate_break_1202: while pTerm != std::ptr::null_mut::<Upsert>() {
                    if (unsafe { (*pTerm).pUpsertTarget }) == std::ptr::null_mut::<ExprList>() {
                        break '__slate_break_1202;
                    }
                    // /* Skip ON CONFLICT for the IPK */
                    if (unsafe { (*pTerm).pUpsertIdx }) == std::ptr::null_mut::<Index>() {
                    } else {
                        jj = 0 as i32;
                        pIdx = unsafe { (*pTab).pIndex };
                        '__slate_break_1203: while pIdx != std::ptr::null_mut::<Index>()
                            && pIdx != unsafe { (*pTerm).pUpsertIdx }
                        {
                            pIdx = unsafe { (*pIdx).pNext };
                            let __v1333: i32 = jj;
                            let __v1334: i32 = __v1333 + (1 as i32);
                            jj = __v1334;
                        }
                        // /* Duplicate ON CONFLICT clause ignored */
                        if (unsafe { *unsafe { bUsed.offset(jj as isize) } }) != (0 as u8) {
                        } else {
                            unsafe {
                                *unsafe { bUsed.offset(jj as isize) } = ((1 as i32) as i8) as u8;
                            }
                            unsafe {
                                (*unsafe { unsafe { sIdxIter.u.ax.aIdx }.offset(i as isize) }).p =
                                    pIdx;
                            }
                            unsafe {
                                (*unsafe { unsafe { sIdxIter.u.ax.aIdx }.offset(i as isize) }).ix =
                                    jj;
                            }
                            let __v1335: i32 = i;
                            let __v1336: i32 = __v1335 + (1 as i32);
                            i = __v1336;
                        }
                    }
                    pTerm = unsafe { (*pTerm).pNextUpsert };
                }
                jj = 0 as i32;
                let __v1337: *mut Index = unsafe { (*pTab).pIndex };
                pIdx = __v1337;
                '__slate_break_1204: loop {
                    if !(pIdx != std::ptr::null_mut::<Index>()) {
                        break;
                    }
                    if (unsafe { *unsafe { bUsed.offset(jj as isize) } }) != (0 as u8) {
                    } else {
                        unsafe {
                            (*unsafe { unsafe { sIdxIter.u.ax.aIdx }.offset(i as isize) }).p = pIdx;
                        }
                        unsafe {
                            (*unsafe { unsafe { sIdxIter.u.ax.aIdx }.offset(i as isize) }).ix = jj;
                        }
                        let __v1340: i32 = i;
                        let __v1341: i32 = __v1340 + (1 as i32);
                        i = __v1341;
                    }
                    pIdx = unsafe { (*pIdx).pNext };
                    let __v1338: i32 = jj;
                    let __v1339: i32 = __v1338 + (1 as i32);
                    jj = __v1339;
                }
                0 as i32;
            }
        }
    }
    // /* Determine if it is possible that triggers (either explicitly coded
    //   ** triggers or FK resolution actions) might run as a result of deletes
    //   ** that happen when OE_Replace conflict resolution occurs. (Call these
    //   ** "replace triggers".)  If any replace triggers run, we will need to
    //   ** recheck all of the uniqueness constraints after they have all run.
    //   ** But on the recheck, the resolution is OE_Abort instead of OE_Replace.
    //   **
    //   ** If replace triggers are a possibility, then
    //   **
    //   **   (1) Allocate register regTrigCnt and initialize it to zero.
    //   **       That register will count the number of replace triggers that
    //   **       fire.  Constraint recheck only occurs if the number is positive.
    //   **   (2) Initialize pTrigger to the list of all DELETE triggers on pTab.
    //   **   (3) Initialize addrRecheck and lblRecheckOk
    //   **
    //   ** The uniqueness rechecking code will create a series of tests to run
    //   ** in a second pass.  The addrRecheck and lblRecheckOk variables are
    //   ** used to link together these tests which are separated from each other
    //   ** in the generate bytecode.
    //   */
    if (unsafe { (*db).flags }) & ((((8192 as i32) | (16384 as i32)) as i64) as u64)
        == (((0 as i32) as i64) as u64)
    {
        // /* There are not DELETE triggers nor FK constraints.  No constraint
        //     ** rechecks are needed. */
        pTrigger = std::ptr::null_mut::<Trigger>();
        regTrigCnt = 0 as i32;
    } else {
        if (unsafe { (*db).flags }) & (((8192 as i32) as i64) as u64) != (0 as u64) {
            pTrigger = unsafe {
                sqlite3TriggersExist(
                    pParse,
                    pTab,
                    129 as i32,
                    std::ptr::null_mut::<ExprList>(),
                    std::ptr::null_mut::<i32>(),
                )
            };
            let __v1342: bool;
            if pTrigger != std::ptr::null_mut::<Trigger>() {
                __v1342 = true as bool;
            } else {
                __v1342 = (unsafe {
                    sqlite3FkRequired(pParse, pTab, std::ptr::null_mut::<i32>(), 0 as i32)
                }) != (0 as i32);
            }
            regTrigCnt = __v1342 as i32;
        } else {
            pTrigger = std::ptr::null_mut::<Trigger>();
            regTrigCnt =
                unsafe { sqlite3FkRequired(pParse, pTab, std::ptr::null_mut::<i32>(), 0 as i32) };
        }
        if regTrigCnt != (0 as i32) {
            // /* At this point regTrigCnt is non-zero if there are DELETE triggers
            //       ** or FK triggers. But we only care about these things if there is
            //       ** a chance that a row will be deleted by an ON CONFLICT REPLACE
            //       ** constraint. So zero regTrigCnt if no such constraint can be found. */
            if ((overrideError as u32) as i32) != (5 as i32) {
                if ((overrideError as u32) as i32) != (11 as i32) {
                    regTrigCnt = 0 as i32;
                } else {
                    if ((pkChng as u32) as i32) == (0 as i32)
                        || pPk != std::ptr::null_mut::<Index>()
                        || (((unsafe { (*pTab).keyConf }) as u32) as i32) != (5 as i32)
                    {
                        pIdx = unsafe { (*pTab).pIndex };
                        '__slate_break_1205: while pIdx != std::ptr::null_mut::<Index>() {
                            if (((unsafe { (*pIdx).onError }) as u32) as i32) == (5 as i32) {
                                break '__slate_break_1205;
                            }
                            pIdx = unsafe { (*pIdx).pNext };
                        }
                        if pIdx == std::ptr::null_mut::<Index>() {
                            regTrigCnt = 0 as i32;
                        }
                    }
                }
            }
        }
        if regTrigCnt != (0 as i32) {
            // /* Replace triggers might exist.  Allocate the counter and
            //       ** initialize it to zero. */
            let __v1343: *mut Parse = pParse;
            let __v1344: i32 = unsafe { (*__v1343).nMem };
            let __v1345: i32 = __v1344 + (1 as i32);
            unsafe {
                (*__v1343).nMem = __v1345;
            }
            regTrigCnt = __v1345;
            unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, regTrigCnt) };
            {}
            lblRecheckOk = unsafe { sqlite3VdbeMakeLabel(pParse) };
            addrRecheck = lblRecheckOk;
        }
    }
    // /* If rowid is changing, make sure the new rowid does not previously
    //   ** exist in the table.
    //   */
    if pkChng != (0 as u8) && pPk == std::ptr::null_mut::<Index>() {
        let mut addrRowidOk: i32 = unsafe { sqlite3VdbeMakeLabel(pParse) };
        // /* Figure out what action to take in case of a rowid collision */
        onError = ((unsafe { (*pTab).keyConf }) as u32) as i32;
        if ((overrideError as u32) as i32) != (11 as i32) {
            onError = (overrideError as u32) as i32;
        } else {
            if onError == (11 as i32) {
                onError = 2 as i32;
            }
        }
        // /* figure out whether or not upsert applies in this case */
        if pUpsert != std::ptr::null_mut::<Upsert>() {
            pUpsertClause = unsafe { sqlite3UpsertOfIndex(pUpsert, std::ptr::null_mut::<Index>()) };
            if pUpsertClause != std::ptr::null_mut::<Upsert>() {
                if (((unsafe { (*pUpsertClause).isDoUpdate }) as u32) as i32) == (0 as i32) {
                    // /* DO NOTHING is the same as INSERT OR IGNORE */
                    onError = 4 as i32;
                } else {
                    // /* DO UPDATE */
                    onError = 6 as i32;
                }
            }
            if pUpsertClause != pUpsert {
                // /* The first ON CONFLICT clause has a conflict target other than
                //         ** the IPK.  We have to jump ahead to that first ON CONFLICT clause
                //         ** and then come back here and deal with the IPK afterwards */
                upsertIpkDelay = unsafe { sqlite3VdbeAddOp0(v, 9 as i32) };
            }
        }
        // /* If the response to a rowid conflict is REPLACE but the response
        //     ** to some other UNIQUE constraint is FAIL or IGNORE, then we need
        //     ** to defer the running of the rowid conflict checking until after
        //     ** the UNIQUE constraints have run.
        //     */
        // /* IPK rule is REPLACE */
        if onError == (5 as i32)
            && onError != ((overrideError as u32) as i32)
            && (unsafe { (*pTab).pIndex }) != std::ptr::null_mut::<Index>()
        {
            if upsertIpkDelay != (0 as i32) {
                ipkTop = upsertIpkDelay + (1 as i32);
                upsertIpkDelay = 0 as i32;
            } else {
                ipkTop = (unsafe { sqlite3VdbeAddOp0(v, 9 as i32) }) + (1 as i32);
            }
            {}
        }
        // /* Rules for other constraints are different */
        // /* There exist other constraints */
        if isUpdate != (0 as u8) {
            // /* pkChng!=0 does not mean that the rowid has changed, only that
            //       ** it might have changed.  Skip the conflict logic below if the rowid
            //       ** is unchanged. */
            unsafe { sqlite3VdbeAddOp3(v, 54 as i32, regNewData, addrRowidOk, regOldData) };
            unsafe { sqlite3VdbeChangeP5(v, ((144 as i32) as i16) as u16) };
            {}
        }
        // /* Check to see if the new rowid already exists in the table.  Skip
        //     ** the following conflict logic if it does not. */
        {}
        {}
        unsafe { sqlite3VdbeAddOp3(v, 31 as i32, iDataCur, addrRowidOk, regNewData) };
        {}
        match onError {
            1 | 2 | 3 => {
                {}
                {}
                {}
                unsafe { sqlite3RowidConstraint(pParse, onError, pTab) };
            }
            5 => {
                // /* If there are DELETE triggers on this table and the
                //         ** recursive-triggers flag is set, call GenerateRowDelete() to
                //         ** remove the conflicting row from the table. This will fire
                //         ** the triggers and remove both the table and index b-tree entries.
                //         **
                //         ** Otherwise, if there are no triggers or the recursive-triggers
                //         ** flag is not set, but the table has one or more indexes, call
                //         ** GenerateRowIndexDelete(). This removes the index b-tree entries
                //         ** only. The table b-tree entry will be replaced by the new entry
                //         ** when it is inserted.
                //         **
                //         ** If either GenerateRowDelete() or GenerateRowIndexDelete() is called,
                //         ** also invoke MultiWrite() to indicate that this VDBE may require
                //         ** statement rollback (if the statement is aborted after the delete
                //         ** takes place). Earlier versions called sqlite3MultiWrite() regardless,
                //         ** but being more selective here allows statements like:
                //         **
                //         **   REPLACE INTO t(rowid) VALUES($newrowid)
                //         **
                //         ** to run without a statement journal if there are no indexes on the
                //         ** table.
                //         */
                if regTrigCnt != (0 as i32) {
                    unsafe { sqlite3MultiWrite(pParse) };
                    unsafe {
                        sqlite3GenerateRowDelete(
                            pParse,
                            pTab,
                            pTrigger,
                            iDataCur,
                            iIdxCur,
                            regNewData,
                            (1 as i32) as i16,
                            ((0 as i32) as i8) as u8,
                            ((5 as i32) as i8) as u8,
                            ((1 as i32) as i8) as u8,
                            -(1 as i32),
                        )
                    };
                    // /* incr trigger cnt */
                    unsafe { sqlite3VdbeAddOp2(v, 88 as i32, regTrigCnt, 1 as i32) };
                    let __v1346: i32 = nReplaceTrig;
                    let __v1347: i32 = __v1346 + (1 as i32);
                    nReplaceTrig = __v1347;
                } else {
                    if (unsafe { (*pTab).pIndex }) != std::ptr::null_mut::<Index>() {
                        unsafe { sqlite3MultiWrite(pParse) };
                        unsafe {
                            sqlite3GenerateRowIndexDelete(
                                pParse,
                                pTab,
                                iDataCur,
                                iIdxCur,
                                std::ptr::null_mut::<i32>(),
                                -(1 as i32),
                            )
                        };
                    }
                }
                seenReplace = 1 as i32;
            }
            6 => {
                unsafe {
                    sqlite3UpsertDoUpdate(
                        pParse,
                        pUpsert,
                        pTab,
                        std::ptr::null_mut::<Index>(),
                        iDataCur,
                    )
                };
                // /* no break */
                {}
                {}
                unsafe { sqlite3VdbeGoto(v, ignoreDest) };
            }
            4 => {
                {}
                unsafe { sqlite3VdbeGoto(v, ignoreDest) };
            }
            _ => {
                onError = 2 as i32;
                // /* no break */
                {}
                {}
                {}
                {}
                unsafe { sqlite3RowidConstraint(pParse, onError, pTab) };
            }
        }
        unsafe { sqlite3VdbeResolveLabel(v, addrRowidOk) };
        if ipkTop != (0 as i32) {
            ipkBottom = unsafe { sqlite3VdbeAddOp0(v, 9 as i32) };
            unsafe { sqlite3VdbeJumpHere(v, ipkTop - (1 as i32)) };
        } else {
            if pUpsert != std::ptr::null_mut::<Upsert>() && pUpsertClause != pUpsert {
                upsertIpkReturn = unsafe { sqlite3VdbeAddOp0(v, 9 as i32) };
            }
        }
    }
    // /* Test all UNIQUE constraints by creating entries for each UNIQUE
    //   ** index and making sure that duplicate entries do not already exist.
    //   ** Compute the revised record entries for indices as we go.
    //   **
    //   ** This loop also handles the case of the PRIMARY KEY index for a
    //   ** WITHOUT ROWID table.
    //   */
    pIdx = indexIteratorFirst(std::ptr::addr_of_mut!(sIdxIter), std::ptr::addr_of_mut!(ix));
    '__slate_break_1207: while pIdx != std::ptr::null_mut::<Index>() {
        // /* Range of registers holding content for pIdx */
        let mut regIdx: i32 = 0 as i32;
        // /* Range of registers holding conflicting PK */
        let mut regR: i32 = 0 as i32;
        // /* Cursor for this UNIQUE index */
        let mut iThisCur: i32 = 0 as i32;
        // /* Jump here if the UNIQUE constraint is satisfied */
        let mut addrUniqueOk: i32 = 0 as i32;
        // /* First opcode in the conflict check logic */
        let mut addrConflictCk: i32 = 0 as i32;
        // /* Number of opcodes in conflict check logic */
        let mut nConflictCk: i32 = 0 as i32;
        // /* Skip indices that do not change */
        if (unsafe { *unsafe { aRegIdx.offset(ix as isize) } }) == (0 as i32) {
        } else {
            if pUpsert != std::ptr::null_mut::<Upsert>() {
                pUpsertClause = unsafe { sqlite3UpsertOfIndex(pUpsert, pIdx) };
                if upsertIpkDelay != (0 as i32) && pUpsertClause == pUpsert {
                    unsafe { sqlite3VdbeJumpHere(v, upsertIpkDelay) };
                }
            }
            addrUniqueOk = unsafe { sqlite3VdbeMakeLabel(pParse) };
            if ((bAffinityDone as u32) as i32) == (0 as i32) {
                sqlite3TableAffinity(v, pTab, regNewData + (1 as i32));
                bAffinityDone = ((1 as i32) as i8) as u8;
            }
            {}
            iThisCur = iIdxCur + ix;
            // /* Skip partial indices for which the WHERE clause is not true */
            if (unsafe { (*pIdx).pPartIdxWhere }) != std::ptr::null_mut::<Expr>() {
                unsafe {
                    sqlite3VdbeAddOp2(v, 77 as i32, 0 as i32, unsafe {
                        *unsafe { aRegIdx.offset(ix as isize) }
                    })
                };
                unsafe {
                    (*pParse).iSelfTab = -(regNewData + (1 as i32));
                }
                unsafe {
                    sqlite3ExprIfFalseDup(
                        pParse,
                        unsafe { (*pIdx).pPartIdxWhere },
                        addrUniqueOk,
                        16 as i32,
                    )
                };
                unsafe {
                    (*pParse).iSelfTab = 0 as i32;
                }
            }
            // /* Create a record for this index entry as it should appear after
            //     ** the insert or update.  Store that record in the aRegIdx[ix] register
            //     */
            regIdx = (unsafe { *unsafe { aRegIdx.offset(ix as isize) } }) + (1 as i32);
            i = 0 as i32;
            '__slate_break_1208: loop {
                if !(i < (((unsafe { (*pIdx).nColumn }) as u32) as i32)) {
                    break;
                }
                let mut iField: i32 =
                    (unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) } }) as i32;
                let mut x: i32 = 0 as i32;
                if iField == -(2 as i32) {
                    unsafe {
                        (*pParse).iSelfTab = -(regNewData + (1 as i32));
                    }
                    unsafe {
                        sqlite3ExprCodeCopy(
                            pParse,
                            unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*unsafe { (*pIdx).aColExpr }).a)
                                            as *mut ExprList_item
                                    }
                                    .offset(i as isize)
                                })
                                .pExpr
                            },
                            regIdx + i,
                        )
                    };
                    unsafe {
                        (*pParse).iSelfTab = 0 as i32;
                    }
                    {}
                } else {
                    if iField == -(1 as i32) || iField == ((unsafe { (*pTab).iPKey }) as i32) {
                        x = regNewData;
                        unsafe { sqlite3VdbeAddOp2(v, 84 as i32, x, regIdx + i) };
                        {}
                    } else {
                        {}
                        x = ((unsafe { sqlite3TableColumnToStorage(pTab, iField as i16) }) as i32)
                            + regNewData
                            + (1 as i32);
                        unsafe { sqlite3VdbeAddOp2(v, 83 as i32, x, regIdx + i) };
                        {}
                    }
                }
                let __v1348: i32 = i;
                let __v1349: i32 = __v1348 + (1 as i32);
                i = __v1349;
            }
            unsafe {
                sqlite3VdbeAddOp3(
                    v,
                    99 as i32,
                    regIdx,
                    ((unsafe { (*pIdx).nColumn }) as u32) as i32,
                    unsafe { *unsafe { aRegIdx.offset(ix as isize) } },
                )
            };
            {}
            {}
            // /* In an UPDATE operation, if this index is the PRIMARY KEY index
            //     ** of a WITHOUT ROWID table and there has been no change the
            //     ** primary key, then no collision is possible.  The collision detection
            //     ** logic below can all be skipped. */
            if isUpdate != (0 as u8) && pPk == pIdx && ((pkChng as u32) as i32) == (0 as i32) {
                unsafe { sqlite3VdbeResolveLabel(v, addrUniqueOk) };
            } else {
                // /* Find out what action to take in case there is a uniqueness conflict */
                onError = ((unsafe { (*pIdx).onError }) as u32) as i32;
                if onError == (0 as i32) {
                    unsafe { sqlite3VdbeResolveLabel(v, addrUniqueOk) };
                // /* pIdx is not a UNIQUE index */
                } else {
                    if ((overrideError as u32) as i32) != (11 as i32) {
                        onError = (overrideError as u32) as i32;
                    } else {
                        if onError == (11 as i32) {
                            onError = 2 as i32;
                        }
                    }
                    // /* Figure out if the upsert clause applies to this index */
                    if pUpsertClause != std::ptr::null_mut::<Upsert>() {
                        if (((unsafe { (*pUpsertClause).isDoUpdate }) as u32) as i32) == (0 as i32)
                        {
                            // /* DO NOTHING is the same as INSERT OR IGNORE */
                            onError = 4 as i32;
                        } else {
                            // /* DO UPDATE */
                            onError = 6 as i32;
                        }
                    }
                    // /* Collision detection may be omitted if all of the following are true:
                    //     **   (1) The conflict resolution algorithm is REPLACE
                    //     **   (2) The table is a WITHOUT ROWID table
                    //     **   (3) There are no secondary indexes on the table
                    //     **   (4) No delete triggers need to be fired if there is a conflict
                    //     **   (5) No FK constraint counters need to be updated if a conflict occurs.
                    //     **
                    //     ** This is not possible for ENABLE_PREUPDATE_HOOK builds, as the row
                    //     ** must be explicitly deleted in order to ensure any pre-update hook
                    //     ** is invoked.  */
                    0 as i32;
                    // /* Condition 3 */
                    let __v1350: bool;
                    if ix == (0 as i32)
                        && (unsafe { (*pIdx).pNext }) == std::ptr::null_mut::<Index>()
                        && pPk == pIdx
                        && onError == (5 as i32)
                    {
                        let __v1351: bool;
                        if (((0 as i32) as i64) as u64)
                            == (unsafe { (*db).flags }) & (((8192 as i32) as i64) as u64)
                        {
                            __v1351 = true as bool;
                        } else {
                            __v1351 = std::ptr::null_mut::<Trigger>()
                                == unsafe {
                                    sqlite3TriggersExist(
                                        pParse,
                                        pTab,
                                        129 as i32,
                                        std::ptr::null_mut::<ExprList>(),
                                        std::ptr::null_mut::<i32>(),
                                    )
                                };
                        }
                        __v1350 = __v1351;
                    } else {
                        __v1350 = false as bool;
                    }
                    let __v1352: bool;
                    if __v1350 {
                        let __v1353: bool;
                        if (((0 as i32) as i64) as u64)
                            == (unsafe { (*db).flags }) & (((16384 as i32) as i64) as u64)
                        {
                            __v1353 = true as bool;
                        } else {
                            let __v1354: bool;
                            if std::ptr::null_mut::<FKey>() == unsafe { (*pTab).u.tab.pFKey } {
                                __v1354 = std::ptr::null_mut::<FKey>()
                                    == unsafe { sqlite3FkReferences(pTab) };
                            } else {
                                __v1354 = false as bool;
                            }
                            __v1353 = __v1354;
                        }
                        __v1352 = __v1353;
                    } else {
                        __v1352 = false as bool;
                    }
                    if __v1352 {
                        unsafe { sqlite3VdbeResolveLabel(v, addrUniqueOk) };
                    } else {
                        // /* Condition 2 */
                        // /* Condition 1 */
                        // /* Condition 4 */
                        // /* Condition 5 */
                        // /* ifndef SQLITE_ENABLE_PREUPDATE_HOOK */
                        // /* Check to see if the new index entry will be unique */
                        {}
                        addrConflictCk = unsafe {
                            sqlite3VdbeAddOp4Int(
                                v,
                                27 as i32,
                                iThisCur,
                                addrUniqueOk,
                                regIdx,
                                ((unsafe { (*pIdx).nKeyCol }) as u32) as i32,
                            )
                        };
                        {}
                        // /* Generate code to handle collisions */
                        let __v1355: i32;
                        if pIdx == pPk {
                            __v1355 = regIdx;
                        } else {
                            __v1355 = unsafe { sqlite3GetTempRange(pParse, nPkField) };
                        }
                        regR = __v1355;
                        if isUpdate != (0 as u8) || onError == (5 as i32) {
                            if (unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32)
                                == ((0 as i32) as u32)
                            {
                                unsafe { sqlite3VdbeAddOp2(v, 144 as i32, iThisCur, regR) };
                                // /* Conflict only if the rowid of the existing index entry
                                //         ** is different from old-rowid */
                                if isUpdate != (0 as u8) {
                                    unsafe {
                                        sqlite3VdbeAddOp3(
                                            v,
                                            54 as i32,
                                            regR,
                                            addrUniqueOk,
                                            regOldData,
                                        )
                                    };
                                    unsafe { sqlite3VdbeChangeP5(v, ((144 as i32) as i16) as u16) };
                                    {}
                                }
                            } else {
                                let mut x: i32 = 0 as i32;
                                // /* Extract the PRIMARY KEY from the end of the index entry and
                                //         ** store it in registers regR..regR+nPk-1 */
                                if pIdx != pPk {
                                    i = 0 as i32;
                                    '__slate_break_1209: loop {
                                        if !(i < (((unsafe { (*pPk).nKeyCol }) as u32) as i32)) {
                                            break;
                                        }
                                        0 as i32;
                                        x = unsafe {
                                            sqlite3TableColumnToIndex(
                                                pIdx,
                                                (unsafe {
                                                    *unsafe {
                                                        unsafe { (*pPk).aiColumn }
                                                            .offset(i as isize)
                                                    }
                                                })
                                                    as i32,
                                            )
                                        };
                                        unsafe {
                                            sqlite3VdbeAddOp3(v, 96 as i32, iThisCur, x, regR + i)
                                        };
                                        {}
                                        let __v1356: i32 = i;
                                        let __v1357: i32 = __v1356 + (1 as i32);
                                        i = __v1357;
                                    }
                                }
                                if isUpdate != (0 as u8) {
                                    // /* If currently processing the PRIMARY KEY of a WITHOUT ROWID
                                    //           ** table, only conflict if the new PRIMARY KEY values are actually
                                    //           ** different from the old.  See TH3 withoutrowid04.test.
                                    //           **
                                    //           ** For a UNIQUE index, only conflict if the PRIMARY KEY values
                                    //           ** of the matched index row are different from the original PRIMARY
                                    //           ** KEY values of this row before the update.  */
                                    let mut addrJump: i32 = (unsafe { sqlite3VdbeCurrentAddr(v) })
                                        + (((unsafe { (*pPk).nKeyCol }) as u32) as i32);
                                    let mut op: i32 = 53 as i32;
                                    let mut regCmp: i32 =
                                        if ((unsafe { (*pIdx).__slate_bits_0.__get_idxType() })
                                            as i32)
                                            == (2 as i32)
                                        {
                                            regIdx
                                        } else {
                                            regR
                                        };
                                    i = 0 as i32;
                                    '__slate_break_1210: loop {
                                        if !(i < (((unsafe { (*pPk).nKeyCol }) as u32) as i32)) {
                                            break;
                                        }
                                        let mut p4: *mut i8 = (unsafe {
                                            sqlite3LocateCollSeq(pParse, unsafe {
                                                *unsafe {
                                                    unsafe { (*pPk).azColl }.offset(i as isize)
                                                }
                                            })
                                        })
                                            as *mut i8;
                                        x = (unsafe {
                                            *unsafe {
                                                unsafe { (*pPk).aiColumn }.offset(i as isize)
                                            }
                                        }) as i32;
                                        0 as i32;
                                        if i == (((unsafe { (*pPk).nKeyCol }) as u32) as i32)
                                            - (1 as i32)
                                        {
                                            addrJump = addrUniqueOk;
                                            op = 54 as i32;
                                        }
                                        x = (unsafe { sqlite3TableColumnToStorage(pTab, x as i16) })
                                            as i32;
                                        unsafe {
                                            sqlite3VdbeAddOp4(
                                                v,
                                                op,
                                                regOldData + (1 as i32) + x,
                                                addrJump,
                                                regCmp + i,
                                                p4 as *const i8,
                                                -(2 as i32),
                                            )
                                        };
                                        unsafe {
                                            sqlite3VdbeChangeP5(v, ((144 as i32) as i16) as u16)
                                        };
                                        {}
                                        {}
                                        let __v1358: i32 = i;
                                        let __v1359: i32 = __v1358 + (1 as i32);
                                        i = __v1359;
                                    }
                                }
                            }
                        }
                        nConflictCk = (unsafe { sqlite3VdbeCurrentAddr(v) }) - addrConflictCk;
                        0 as i32;
                        {}
                        {}
                        // /* Generate code that executes if the new index entry is not unique */
                        0 as i32;
                        match onError {
                            1 | 2 | 3 => {
                                {}
                                {}
                                {}
                                unsafe { sqlite3UniqueConstraint(pParse, onError, pIdx) };
                            }
                            6 => {
                                unsafe {
                                    sqlite3UpsertDoUpdate(pParse, pUpsert, pTab, pIdx, iIdxCur + ix)
                                };
                                // /* no break */
                                {}
                                {}
                                unsafe { sqlite3VdbeGoto(v, ignoreDest) };
                            }
                            4 => {
                                {}
                                unsafe { sqlite3VdbeGoto(v, ignoreDest) };
                            }
                            _ => {
                                0 as i32;
                                if regTrigCnt != (0 as i32) {
                                    unsafe { sqlite3MultiWrite(pParse) };
                                    let __v1360: i32 = nReplaceTrig;
                                    let __v1361: i32 = __v1360 + (1 as i32);
                                    nReplaceTrig = __v1361;
                                }
                                if pTrigger != std::ptr::null_mut::<Trigger>()
                                    && isUpdate != (0 as u8)
                                {
                                    unsafe { sqlite3VdbeAddOp1(v, 169 as i32, iDataCur) };
                                }
                                unsafe {
                                    sqlite3GenerateRowDelete(
                                        pParse,
                                        pTab,
                                        pTrigger,
                                        iDataCur,
                                        iIdxCur,
                                        regR,
                                        nPkField as i16,
                                        ((0 as i32) as i8) as u8,
                                        ((5 as i32) as i8) as u8,
                                        ((if pIdx == pPk { 1 as i32 } else { 0 as i32 }) as i8)
                                            as u8,
                                        iThisCur,
                                    )
                                };
                                if pTrigger != std::ptr::null_mut::<Trigger>()
                                    && isUpdate != (0 as u8)
                                {
                                    unsafe { sqlite3VdbeAddOp1(v, 170 as i32, iDataCur) };
                                }
                                seenReplace = 1 as i32;
                            }
                        }
                        if regTrigCnt != (0 as i32) {
                            // /* Jump destination to bypass recheck logic */
                            let mut addrBypass: i32 = 0 as i32;
                            // /* incr trigger cnt */
                            unsafe { sqlite3VdbeAddOp2(v, 88 as i32, regTrigCnt, 1 as i32) };
                            // /* Bypass recheck */
                            addrBypass = unsafe { sqlite3VdbeAddOp0(v, 9 as i32) };
                            {}
                            // /* Here we insert code that will be invoked after all constraint
                            //       ** checks have run, if and only if one or more replace triggers
                            //       ** fired. */
                            unsafe { sqlite3VdbeResolveLabel(v, lblRecheckOk) };
                            lblRecheckOk = unsafe { sqlite3VdbeMakeLabel(pParse) };
                            if (unsafe { (*pIdx).pPartIdxWhere }) != std::ptr::null_mut::<Expr>() {
                                // /* Bypass the recheck if this partial index is not defined
                                //         ** for the current row */
                                unsafe {
                                    sqlite3VdbeAddOp2(
                                        v,
                                        51 as i32,
                                        regIdx - (1 as i32),
                                        lblRecheckOk,
                                    )
                                };
                                {}
                            }
                            // /* Copy the constraint check code from above, except change
                            //       ** the constraint-ok jump destination to be the address of
                            //       ** the next retest block */
                            '__slate_break_1212: while nConflictCk > (0 as i32) {
                                // /* Conflict check opcode to copy */
                                let mut x: VdbeOp = unsafe { std::mem::zeroed() };
                                // /* The sqlite3VdbeAddOp4() call might reallocate the opcode array.
                                //         ** Hence, make a complete copy of the opcode, rather than using
                                //         ** a pointer to the opcode. */
                                x = unsafe { *unsafe { sqlite3VdbeGetOp(v, addrConflictCk) } };
                                if ((x.opcode as u32) as i32) != (144 as i32) {
                                    // /* New P2 value for copied conflict check opcode */
                                    let mut p2: i32 = 0 as i32;
                                    let mut zP4: *const i8 = unsafe { std::mem::zeroed() };
                                    if (((unsafe {
                                        *unsafe {
                                            unsafe {
                                                std::ptr::addr_of!(sqlite3OpcodeProperty)
                                                    as *const u8
                                            }
                                            .offset(((x.opcode as u32) as i32) as isize)
                                        }
                                    }) as u32) as i32)
                                        & (1 as i32)
                                        != (0 as i32)
                                    {
                                        p2 = lblRecheckOk;
                                    } else {
                                        p2 = x.p2;
                                    }
                                    zP4 = (if (x.p4type as i32) == -(3 as i32) {
                                        ((unsafe { x.p4.i }) as i64) as *mut ()
                                    } else {
                                        (unsafe { x.p4.z }) as *mut ()
                                    }) as *const i8;
                                    unsafe {
                                        sqlite3VdbeAddOp4(
                                            v,
                                            (x.opcode as u32) as i32,
                                            x.p1,
                                            p2,
                                            x.p3,
                                            zP4,
                                            x.p4type as i32,
                                        )
                                    };
                                    unsafe { sqlite3VdbeChangeP5(v, x.p5) };
                                    {}
                                }
                                let __v1362: i32 = nConflictCk;
                                let __v1363: i32 = __v1362 - (1 as i32);
                                nConflictCk = __v1363;
                                let __v1364: i32 = addrConflictCk;
                                let __v1365: i32 = __v1364 + (1 as i32);
                                addrConflictCk = __v1365;
                            }
                            // /* If the retest fails, issue an abort */
                            unsafe { sqlite3UniqueConstraint(pParse, 2 as i32, pIdx) };
                            // /* Terminate the recheck bypass */
                            unsafe { sqlite3VdbeJumpHere(v, addrBypass) };
                        }
                        unsafe { sqlite3VdbeResolveLabel(v, addrUniqueOk) };
                        if regR != regIdx {
                            unsafe { sqlite3ReleaseTempRange(pParse, regR, nPkField) };
                        }
                        let __v1366: bool;
                        if pUpsertClause != std::ptr::null_mut::<Upsert>()
                            && upsertIpkReturn != (0 as i32)
                        {
                            __v1366 =
                                (unsafe { sqlite3UpsertNextIsIPK(pUpsertClause) }) != (0 as i32);
                        } else {
                            __v1366 = false as bool;
                        }
                        if __v1366 {
                            unsafe { sqlite3VdbeGoto(v, upsertIpkDelay + (1 as i32)) };
                            unsafe { sqlite3VdbeJumpHere(v, upsertIpkReturn) };
                            upsertIpkReturn = 0 as i32;
                        }
                    }
                }
            }
        }
        pIdx = indexIteratorNext(std::ptr::addr_of_mut!(sIdxIter), std::ptr::addr_of_mut!(ix));
    }
    // /* If the IPK constraint is a REPLACE, run it last */
    if ipkTop != (0 as i32) {
        unsafe { sqlite3VdbeGoto(v, ipkTop) };
        {}
        0 as i32;
        unsafe { sqlite3VdbeJumpHere(v, ipkBottom) };
    }
    // /* Recheck all uniqueness constraints after replace triggers have run */
    {}
    0 as i32;
    if nReplaceTrig != (0 as i32) {
        unsafe { sqlite3VdbeAddOp2(v, 17 as i32, regTrigCnt, lblRecheckOk) };
        {}
        if !(pPk != std::ptr::null_mut::<Index>()) {
            if isUpdate != (0 as u8) {
                unsafe { sqlite3VdbeAddOp3(v, 54 as i32, regNewData, addrRecheck, regOldData) };
                unsafe { sqlite3VdbeChangeP5(v, ((144 as i32) as i16) as u16) };
                {}
            }
            unsafe { sqlite3VdbeAddOp3(v, 31 as i32, iDataCur, addrRecheck, regNewData) };
            {}
            unsafe { sqlite3RowidConstraint(pParse, 2 as i32, pTab) };
        } else {
            unsafe { sqlite3VdbeGoto(v, addrRecheck) };
        }
    }
    if regTrigCnt != (0 as i32) {
        unsafe { sqlite3VdbeResolveLabel(v, lblRecheckOk) };
    }
    // /* Generate the table record */
    if (unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
        let mut regRec: i32 = unsafe { *unsafe { aRegIdx.offset(ix as isize) } };
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                99 as i32,
                regNewData + (1 as i32),
                (unsafe { (*pTab).nNVCol }) as i32,
                regRec,
            )
        };
        {}
        if !(bAffinityDone != (0 as u8)) {
            sqlite3TableAffinity(v, pTab, 0 as i32);
        }
    }
    unsafe {
        *pbMayReplace = seenReplace;
    }
    {}
}

// /* The parser context */
// /* The table being inserted or updated */
// /* Use register aRegIdx[i] for index i.  0 for unused */
// /* Canonical data cursor (main table or PK index) */
// /* First index cursor */
// /* First register in a range holding values to insert */
// /* Previous content.  0 for INSERTs */
// /* Non-zero if the rowid or PRIMARY KEY changed */
// /* Override onError to this if not OE_Default */
// /* Jump to this label on an OE_Ignore resolution */
// /* OUT: Set to true if constraint may cause a replace */
// /* column i is unchanged if aiChng[i]<0 */
// /* ON CONFLICT clauses, if any.  NULL otherwise */
// /*
// ** Table pTab is a WITHOUT ROWID table that is being written to. The cursor
// ** number is iCur, and register regData contains the new record for the
// ** PK index. This function adds code to invoke the pre-update hook,
// ** if one is registered.
// */
// /*
// ** This routine generates code to finish the INSERT or UPDATE operation
// ** that was started by a prior call to sqlite3GenerateConstraintChecks.
// ** A consecutive range of registers starting at regNewData contains the
// ** rowid and the content to be inserted.
// **
// ** The arguments to this routine should be the same as the first six
// ** arguments to sqlite3GenerateConstraintChecks.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CompleteInsertion(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut iDataCur: i32,
    mut iIdxCur: i32,
    mut regNewData: i32,
    mut aRegIdx: *mut i32,
    mut update_flags: i32,
    mut appendBias: i32,
    mut useSeekResult: i32,
) {
    // /* Prepared statements under construction */
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    // /* An index being inserted or updated */
    let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
    // /* flag values passed to the btree insert */
    let mut pik_flags: u8 = 0 as u8;
    // /* Loop counter */
    let mut i: i32 = 0 as i32;
    0 as i32;
    v = unsafe { (*pParse).pVdbe };
    0 as i32;
    // /* This table is not a VIEW */
    0 as i32;
    i = 0 as i32;
    let __v1367: *mut Index = unsafe { (*pTab).pIndex };
    pIdx = __v1367;
    '__slate_break_1213: loop {
        if !(pIdx != std::ptr::null_mut::<Index>()) {
            break;
        }
        // /* All REPLACE indexes are at the end of the list */
        0 as i32;
        if (unsafe { *unsafe { aRegIdx.offset(i as isize) } }) == (0 as i32) {
        } else {
            if (unsafe { (*pIdx).pPartIdxWhere }) != std::ptr::null_mut::<Expr>()
                || update_flags != (0 as i32)
                    && ((unsafe { (*pIdx).__slate_bits_0.__get_bHasExpr() }) as i32) != (0 as i32)
            {
                // /* If this is a partial index, or an UPDATE of an index on an
                //       ** expression, then the record register may be set to NULL to indicate
                //       ** that no record should be inserted into this index. */
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        51 as i32,
                        unsafe { *unsafe { aRegIdx.offset(i as isize) } },
                        (unsafe { sqlite3VdbeCurrentAddr(v) }) + (2 as i32),
                    )
                };
                {}
            }
            pik_flags = ((if useSeekResult != (0 as i32) {
                16 as i32
            } else {
                0 as i32
            }) as i8) as u8;
            if ((unsafe { (*pIdx).__slate_bits_0.__get_idxType() }) as i32) == (2 as i32)
                && !((unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32))
            {
                let __v1370: u8 = pik_flags;
                let __v1371: u8 = ((((__v1370 as u32) as i32) | (1 as i32)) as i8) as u8;
                pik_flags = __v1371;
                let __v1372: u8 = pik_flags;
                let __v1373: u8 =
                    ((((__v1372 as u32) as i32) | update_flags & (2 as i32)) as i8) as u8;
                pik_flags = __v1373;
                if update_flags == (0 as i32) {
                    {}
                }
            }
            unsafe {
                sqlite3VdbeAddOp4Int(
                    v,
                    140 as i32,
                    iIdxCur + i,
                    unsafe { *unsafe { aRegIdx.offset(i as isize) } },
                    (unsafe { *unsafe { aRegIdx.offset(i as isize) } }) + (1 as i32),
                    if ((unsafe { (*pIdx).__slate_bits_0.__get_uniqNotNull() }) as i32)
                        != (0 as i32)
                    {
                        ((unsafe { (*pIdx).nKeyCol }) as u32) as i32
                    } else {
                        ((unsafe { (*pIdx).nColumn }) as u32) as i32
                    },
                )
            };
            unsafe { sqlite3VdbeChangeP5(v, pik_flags as u16) };
        }
        pIdx = unsafe { (*pIdx).pNext };
        let __v1368: i32 = i;
        let __v1369: i32 = __v1368 + (1 as i32);
        i = __v1369;
    }
    if !((unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32)) {
        return;
    }
    if (unsafe { (*pParse).nested }) != (0 as u8) {
        pik_flags = ((0 as i32) as i8) as u8;
    } else {
        pik_flags = ((1 as i32) as i8) as u8;
        let __v1374: u8 = pik_flags;
        let __v1375: u8 = ((((__v1374 as u32) as i32)
            | if update_flags != (0 as i32) {
                update_flags
            } else {
                32 as i32
            }) as i8) as u8;
        pik_flags = __v1375;
    }
    if appendBias != (0 as i32) {
        let __v1376: u8 = pik_flags;
        let __v1377: u8 = ((((__v1376 as u32) as i32) | (8 as i32)) as i8) as u8;
        pik_flags = __v1377;
    }
    if useSeekResult != (0 as i32) {
        let __v1378: u8 = pik_flags;
        let __v1379: u8 = ((((__v1378 as u32) as i32) | (16 as i32)) as i8) as u8;
        pik_flags = __v1379;
    }
    unsafe {
        sqlite3VdbeAddOp3(
            v,
            130 as i32,
            iDataCur,
            unsafe { *unsafe { aRegIdx.offset(i as isize) } },
            regNewData,
        )
    };
    if !((unsafe { (*pParse).nested }) != (0 as u8)) {
        unsafe { sqlite3VdbeAppendP4(v, pTab as *mut (), -(5 as i32)) };
    }
    unsafe { sqlite3VdbeChangeP5(v, pik_flags as u16) };
}

// /* The parser context */
// /* the table into which we are inserting */
// /* Cursor of the canonical data source */
// /* First index cursor */
// /* Range of content */
// /* Register used by each index.  0 for unused indices */
// /* True for UPDATE, False for INSERT */
// /* True if this is likely to be an append */
// /* True to set the USESEEKRESULT flag on OP_[Idx]Insert */
// /*
// ** Allocate cursors for the pTab table and all its indices and generate
// ** code to open and initialized those cursors.
// **
// ** The cursor for the object that contains the complete data (normally
// ** the table itself, but the PRIMARY KEY index in the case of a WITHOUT
// ** ROWID table) is returned in *piDataCur.  The first index cursor is
// ** returned in *piIdxCur.  The number of indices is returned.
// **
// ** Use iBase as the first cursor (either the *piDataCur for rowid tables
// ** or the first index for WITHOUT ROWID tables) if it is non-negative.
// ** If iBase is negative, then allocate the next available cursor.
// **
// ** For a rowid table, *piDataCur will be exactly one less than *piIdxCur.
// ** For a WITHOUT ROWID table, *piDataCur will be somewhere in the range
// ** of *piIdxCurs, depending on where the PRIMARY KEY index appears on the
// ** pTab->pIndex list.
// **
// ** If pTab is a virtual table, then this routine is a no-op and the
// ** *piDataCur and *piIdxCur values are left uninitialized.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OpenTableAndIndices(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut op: i32,
    mut p5: u8,
    mut iBase: i32,
    mut aToOpen: *mut u8,
    mut piDataCur: *mut i32,
    mut piIdxCur: *mut i32,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut iDb: i32 = 0 as i32;
    let mut iDataCur: i32 = 0 as i32;
    let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32) {
        // /* This routine is a no-op for virtual tables. Leave the output
        //     ** variables *piDataCur and *piIdxCur set to illegal cursor numbers
        //     ** for improved error detection. */
        let __v1380: i32 = -(999 as i32);
        unsafe {
            *piIdxCur = __v1380;
        }
        unsafe {
            *piDataCur = __v1380;
        }
        return 0 as i32;
    }
    iDb = unsafe { sqlite3SchemaToIndex(unsafe { (*pParse).db }, unsafe { (*pTab).pSchema }) };
    v = unsafe { (*pParse).pVdbe };
    0 as i32;
    if iBase < (0 as i32) {
        iBase = unsafe { (*pParse).nTab };
    }
    let __v1381: i32 = iBase;
    let __v1382: i32 = __v1381 + (1 as i32);
    iBase = __v1382;
    iDataCur = __v1381;
    unsafe {
        *piDataCur = iDataCur;
    }
    if (unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32)
        && (aToOpen == std::ptr::null_mut::<u8>()
            || (unsafe { *unsafe { aToOpen.offset((0 as i32) as isize) } }) != (0 as u8))
    {
        sqlite3OpenTable(pParse, iDataCur, iDb, pTab, op);
    } else {
        if (((unsafe { (*unsafe { (*pParse).db }).noSharedCache }) as u32) as i32) == (0 as i32) {
            unsafe {
                sqlite3TableLock(
                    pParse,
                    iDb,
                    unsafe { (*pTab).tnum },
                    (op == (116 as i32)) as u8,
                    (unsafe { (*pTab).zName }) as *const i8,
                )
            };
        }
    }
    unsafe {
        *piIdxCur = iBase;
    }
    i = 0 as i32;
    let __v1383: *mut Index = unsafe { (*pTab).pIndex };
    pIdx = __v1383;
    '__slate_break_1214: loop {
        if !(pIdx != std::ptr::null_mut::<Index>()) {
            break;
        }
        let mut iIdxCur: i32 = 0 as i32;
        let __v1386: i32 = iBase;
        let __v1387: i32 = __v1386 + (1 as i32);
        iBase = __v1387;
        iIdxCur = __v1386;
        0 as i32;
        if ((unsafe { (*pIdx).__slate_bits_0.__get_idxType() }) as i32) == (2 as i32)
            && !((unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32))
        {
            unsafe {
                *piDataCur = iIdxCur;
            }
            p5 = ((0 as i32) as i8) as u8;
        }
        if aToOpen == std::ptr::null_mut::<u8>()
            || (unsafe { *unsafe { aToOpen.offset((i + (1 as i32)) as isize) } }) != (0 as u8)
        {
            unsafe { sqlite3VdbeAddOp3(v, op, iIdxCur, (unsafe { (*pIdx).tnum }) as i32, iDb) };
            unsafe { sqlite3VdbeSetP4KeyInfo(pParse, pIdx) };
            unsafe { sqlite3VdbeChangeP5(v, p5 as u16) };
            {}
        }
        pIdx = unsafe { (*pIdx).pNext };
        let __v1384: i32 = i;
        let __v1385: i32 = __v1384 + (1 as i32);
        i = __v1385;
    }
    if iBase > unsafe { (*pParse).nTab } {
        unsafe {
            (*pParse).nTab = iBase;
        }
    }
    return i;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IndexAffinityStr(
    mut db: *mut sqlite3,
    mut pIdx: *mut Index,
) -> *const i8 {
    if !((unsafe { (*pIdx).zColAff }) != std::ptr::null_mut::<i8>()) {
        return computeIndexAffStr(db, pIdx);
    }
    return (unsafe { (*pIdx).zColAff }) as *const i8;
}

// /*
// ** Compute an affinity string for a table.   Space is obtained
// ** from sqlite3DbMalloc().  The caller is responsible for freeing
// ** the space when done.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TableAffinityStr(
    mut db: *mut sqlite3,
    mut pTab: *const Table,
) -> *mut i8 {
    let mut zColAff: *mut i8 = unsafe { std::mem::zeroed() };
    zColAff = (unsafe {
        sqlite3DbMallocRaw(
            db,
            ((((unsafe { (*pTab).nCol }) as i32) + (1 as i32)) as i64) as u64,
        )
    }) as *mut i8;
    if zColAff != std::ptr::null_mut::<i8>() {
        let mut i: i32 = 0 as i32;
        let mut j: i32 = 0 as i32;
        j = 0 as i32;
        i = 0 as i32;
        '__slate_break_1164: loop {
            if !(i < ((unsafe { (*pTab).nCol }) as i32)) {
                break;
            }
            if (((unsafe { (*unsafe { unsafe { (*pTab).aCol }.offset(i as isize) }).colFlags })
                as u32) as i32)
                & (32 as i32)
                == (0 as i32)
            {
                let __v1390: i32 = j;
                let __v1391: i32 = __v1390 + (1 as i32);
                j = __v1391;
                unsafe {
                    *unsafe { zColAff.offset(__v1390 as isize) } = unsafe {
                        (*unsafe { unsafe { (*pTab).aCol }.offset(i as isize) }).affinity
                    };
                }
            }
            let __v1388: i32 = i;
            let __v1389: i32 = __v1388 + (1 as i32);
            i = __v1389;
        }
        '__slate_break_1165: loop {
            let __v1392: i32 = j;
            let __v1393: i32 = __v1392 - (1 as i32);
            j = __v1393;
            unsafe {
                *unsafe { zColAff.offset(__v1392 as isize) } = (0 as i32) as i8;
            }
            if !(j >= (0 as i32)
                && ((unsafe { *unsafe { zColAff.offset(j as isize) } }) as i32) <= (65 as i32))
            {
                break;
            }
        }
    }
    return zColAff;
}

// /*
// ** Make changes to the evolving bytecode to do affinity transformations
// ** of values that are about to be gathered into a row for table pTab.
// **
// ** For ordinary (legacy, non-strict) tables:
// ** -----------------------------------------
// **
// ** Compute the affinity string for table pTab, if it has not already been
// ** computed.  As an optimization, omit trailing SQLITE_AFF_BLOB affinities.
// **
// ** If the affinity string is empty (because it was all SQLITE_AFF_BLOB entries
// ** which were then optimized out) then this routine becomes a no-op.
// **
// ** Otherwise if iReg>0 then code an OP_Affinity opcode that will set the
// ** affinities for register iReg and following.  Or if iReg==0,
// ** then just set the P4 operand of the previous opcode (which should  be
// ** an OP_MakeRecord) to the affinity string.
// **
// ** A column affinity string has one character per column:
// **
// **    Character      Column affinity
// **    ---------      ---------------
// **    'A'            BLOB
// **    'B'            TEXT
// **    'C'            NUMERIC
// **    'D'            INTEGER
// **    'E'            REAL
// **
// ** For STRICT tables:
// ** ------------------
// **
// ** Generate an appropriate OP_TypeCheck opcode that will verify the
// ** datatypes against the column definitions in pTab.  If iReg==0, that
// ** means an OP_MakeRecord opcode has already been generated and should be
// ** the last opcode generated.  The new OP_TypeCheck needs to be inserted
// ** before the OP_MakeRecord.  The new OP_TypeCheck should use the same
// ** register set as the OP_MakeRecord.  If iReg>0 then register iReg is
// ** the first of a series of registers that will form the new record.
// ** Apply the type checking to that array of registers.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TableAffinity(mut v: *mut Vdbe, mut pTab: *mut Table, mut iReg: i32) {
    let mut i: i32 = 0 as i32;
    let mut zColAff: *mut i8 = unsafe { std::mem::zeroed() };
    if (unsafe { (*pTab).tabFlags }) & ((65536 as i32) as u32) != (0 as u32) {
        if iReg == (0 as i32) {
            // /* Move the previous opcode (which should be OP_MakeRecord) forward
            //       ** by one slot and insert a new OP_TypeCheck where the current
            //       ** OP_MakeRecord is found */
            let mut pPrev: *mut VdbeOp = unsafe { std::mem::zeroed() };
            let mut p3: i32 = 0 as i32;
            unsafe { sqlite3VdbeAppendP4(v, pTab as *mut (), -(5 as i32)) };
            pPrev = unsafe { sqlite3VdbeGetLastOp(v) };
            0 as i32;
            0 as i32;
            unsafe {
                (*pPrev).opcode = ((97 as i32) as i8) as u8;
            }
            p3 = unsafe { (*pPrev).p3 };
            unsafe {
                (*pPrev).p3 = 0 as i32;
            }
            unsafe {
                sqlite3VdbeAddOp3(
                    v,
                    99 as i32,
                    unsafe { (*pPrev).p1 },
                    unsafe { (*pPrev).p2 },
                    p3,
                )
            };
        } else {
            // /* Insert an isolated OP_Typecheck */
            unsafe { sqlite3VdbeAddOp2(v, 97 as i32, iReg, (unsafe { (*pTab).nNVCol }) as i32) };
            unsafe { sqlite3VdbeAppendP4(v, pTab as *mut (), -(5 as i32)) };
        }
        return;
    }
    zColAff = unsafe { (*pTab).zColAff };
    if zColAff == std::ptr::null_mut::<i8>() {
        zColAff = sqlite3TableAffinityStr(std::ptr::null_mut::<sqlite3>(), pTab as *const Table);
        if !(zColAff != std::ptr::null_mut::<i8>()) {
            unsafe { sqlite3OomFault(unsafe { sqlite3VdbeDb(v) }) };
            return;
        }
        unsafe {
            (*pTab).zColAff = zColAff;
        }
    }
    0 as i32;
    i = (((unsafe { strlen(zColAff as *const i8) }) & (((1073741823 as i32) as i64) as u64)) as u32)
        as i32;
    if i != (0 as i32) {
        if iReg != (0 as i32) {
            unsafe { sqlite3VdbeAddOp4(v, 98 as i32, iReg, i, 0 as i32, zColAff as *const i8, i) };
        } else {
            0 as i32;
            unsafe { sqlite3VdbeChangeP4(v, -(1 as i32), zColAff as *const i8, i) };
        }
    }
}

// /* Generate code into this VDBE */
// /* The cursor number of the table */
// /* The database index in sqlite3.aDb[] */
// /* The table to be opened */
// /* OP_OpenRead or OP_OpenWrite */
// /*
// ** Return a pointer to the column affinity string associated with index
// ** pIdx. A column affinity string has one character for each column in
// ** the table, according to the affinity of the column:
// **
// **  Character      Column affinity
// **  ------------------------------
// **  'A'            BLOB
// **  'B'            TEXT
// **  'C'            NUMERIC
// **  'D'            INTEGER
// **  'F'            REAL
// **
// ** An extra 'D' is appended to the end of the string to cover the
// ** rowid that appears as the last column in every index.
// **
// ** Memory for the buffer containing the column index affinity string
// ** is managed along with the rest of the Index structure. It will be
// ** released when sqlite3DeleteIndex() is called.
// */
fn computeIndexAffStr(mut db: *mut sqlite3, mut pIdx: *mut Index) -> *const i8 {
    // /* The first time a column affinity string for a particular index is
    //   ** required, it is allocated and populated here. It is then stored as
    //   ** a member of the Index structure for subsequent use.
    //   **
    //   ** The column affinity string will eventually be deleted by
    //   ** sqliteDeleteIndex() when the Index structure itself is cleaned
    //   ** up.
    //   */
    let mut n: i32 = 0 as i32;
    let mut pTab: *mut Table = unsafe { (*pIdx).pTable };
    unsafe {
        (*pIdx).zColAff = (unsafe {
            sqlite3DbMallocRaw(
                std::ptr::null_mut::<sqlite3>(),
                (((((unsafe { (*pIdx).nColumn }) as u32) as i32) + (1 as i32)) as i64) as u64,
            )
        }) as *mut i8;
    }
    if !((unsafe { (*pIdx).zColAff }) != std::ptr::null_mut::<i8>()) {
        unsafe { sqlite3OomFault(db) };
        return std::ptr::null::<i8>();
    }
    n = 0 as i32;
    '__slate_break_1163: loop {
        if !(n < (((unsafe { (*pIdx).nColumn }) as u32) as i32)) {
            break;
        }
        let mut x: i16 = unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(n as isize) } };
        let mut aff: i8 = 0 as i8;
        if (x as i32) >= (0 as i32) {
            aff = unsafe {
                (*unsafe { unsafe { (*pTab).aCol }.offset((x as i32) as isize) }).affinity
            };
        } else {
            if (x as i32) == -(1 as i32) {
                aff = (68 as i32) as i8;
            } else {
                0 as i32;
                0 as i32;
                0 as i32;
                aff = unsafe {
                    sqlite3ExprAffinity(
                        (unsafe {
                            (*unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*unsafe { (*pIdx).aColExpr }).a)
                                        as *mut ExprList_item
                                }
                                .offset(n as isize)
                            })
                            .pExpr
                        }) as *const Expr,
                    )
                };
            }
        }
        if (aff as i32) < (65 as i32) {
            aff = (65 as i32) as i8;
        }
        if (aff as i32) > (67 as i32) {
            aff = (67 as i32) as i8;
        }
        unsafe {
            *unsafe { unsafe { (*pIdx).zColAff }.offset(n as isize) } = aff;
        }
        let __v1394: i32 = n;
        let __v1395: i32 = __v1394 + (1 as i32);
        n = __v1395;
    }
    unsafe {
        *unsafe { unsafe { (*pIdx).zColAff }.offset(n as isize) } = (0 as i32) as i8;
    }
    return (unsafe { (*pIdx).zColAff }) as *const i8;
}

// /*
// ** Return non-zero if the table pTab in database iDb or any of its indices
// ** have been opened at any point in the VDBE program. This is used to see if
// ** a statement of the form  "INSERT INTO <iDb, pTab> SELECT ..." can
// ** run without using a temporary table for the results of the SELECT.
// */
fn readsTable(mut p: *mut Parse, mut iDb: i32, mut pTab: *mut Table) -> i32 {
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(p) };
    let mut i: i32 = 0 as i32;
    let mut iEnd: i32 = unsafe { sqlite3VdbeCurrentAddr(v) };
    let mut pVTab: *mut VTable = unsafe { std::mem::zeroed() };
    let __v1396: *mut VTable;
    if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32) {
        __v1396 = unsafe { sqlite3GetVTable(unsafe { (*p).db }, pTab) };
    } else {
        __v1396 = std::ptr::null_mut::<VTable>();
    }
    pVTab = __v1396;
    i = 1 as i32;
    '__slate_break_1166: loop {
        if !(i < iEnd) {
            break;
        }
        let mut pOp: *mut VdbeOp = unsafe { sqlite3VdbeGetOp(v, i) };
        0 as i32;
        if (((unsafe { (*pOp).opcode }) as u32) as i32) == (114 as i32)
            && (unsafe { (*pOp).p3 }) == iDb
        {
            let mut pIndex: *mut Index = unsafe { std::mem::zeroed() };
            let mut tnum: u32 = (unsafe { (*pOp).p2 }) as u32;
            if tnum == unsafe { (*pTab).tnum } {
                return 1 as i32;
            }
            pIndex = unsafe { (*pTab).pIndex };
            '__slate_break_1167: while pIndex != std::ptr::null_mut::<Index>() {
                if tnum == unsafe { (*pIndex).tnum } {
                    return 1 as i32;
                }
                pIndex = unsafe { (*pIndex).pNext };
            }
        }
        if (((unsafe { (*pOp).opcode }) as u32) as i32) == (175 as i32)
            && (unsafe { (*pOp).p4.pVtab }) == pVTab
        {
            0 as i32;
            0 as i32;
            return 1 as i32;
        }
        let __v1397: i32 = i;
        let __v1398: i32 = __v1397 + (1 as i32);
        i = __v1398;
    }
    return 0 as i32;
}

// /* This walker callback will compute the union of colFlags flags for all
// ** referenced columns in a CHECK constraint or generated column expression.
// */
#[unsafe(link_section = ".text.slate_distinct.insert.exprColumnFlagUnion")]
extern "C-unwind" fn exprColumnFlagUnion(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (168 as i32)
        && ((unsafe { (*pExpr).iColumn }) as i32) >= (0 as i32)
    {
        0 as i32;
        let __v1399: *mut Walker = pWalker;
        let __v1400: u16 = unsafe { (*__v1399).eCode };
        let __v1401: u16 = ((((__v1400 as u32) as i32)
            | (((unsafe {
                (*unsafe {
                    unsafe { (*unsafe { (*pWalker).u.pTab }).aCol }
                        .offset(((unsafe { (*pExpr).iColumn }) as i32) as isize)
                })
                .colFlags
            }) as u32) as i32)) as i16) as u16;
        unsafe {
            (*__v1399).eCode = __v1401;
        }
    }
    return 0 as i32;
}

// /* Parsing context */
// /* Register holding the first column */
// /* The table */
// /* SQLITE_OMIT_GENERATED_COLUMNS */
// /*
// ** Locate or create an AutoincInfo structure associated with table pTab
// ** which is in database iDb.  Return the register number for the register
// ** that holds the maximum rowid.  Return zero if pTab is not an AUTOINCREMENT
// ** table.  (Also return zero when doing a VACUUM since we do not want to
// ** update the AUTOINCREMENT counters during a VACUUM.)
// **
// ** There is at most one AutoincInfo structure per table even if the
// ** same table is autoincremented multiple times due to inserts within
// ** triggers.  A new AutoincInfo structure is created if this is the
// ** first use of table pTab.  On 2nd and subsequent uses, the original
// ** AutoincInfo structure is used.
// **
// ** Four consecutive registers are allocated:
// **
// **   (1)  The name of the pTab table.
// **   (2)  The maximum ROWID of pTab.
// **   (3)  The rowid in sqlite_sequence of pTab
// **   (4)  The original value of the max ROWID in pTab, or NULL if none
// **
// ** The 2nd register is the one that is returned.  That is all the
// ** insert routine needs to know about.
// */
fn autoIncBegin(mut pParse: *mut Parse, mut iDb: i32, mut pTab: *mut Table) -> i32 {
    // /* Register holding maximum rowid */
    let mut memId: i32 = 0 as i32;
    0 as i32;
    if (unsafe { (*pTab).tabFlags }) & ((8 as i32) as u32) != ((0 as i32) as u32)
        && (unsafe { (*unsafe { (*pParse).db }).mDbFlags }) & ((4 as i32) as u32)
            == ((0 as i32) as u32)
    {
        let mut pToplevel: *mut Parse =
            if (unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>() {
                unsafe { (*pParse).pToplevel }
            } else {
                pParse
            };
        let mut pInfo: *mut AutoincInfo = unsafe { std::mem::zeroed() };
        let mut pSeqTab: *mut Table = unsafe {
            (*unsafe {
                (*unsafe { unsafe { (*unsafe { (*pParse).db }).aDb }.offset(iDb as isize) }).pSchema
            })
            .pSeqTab
        };
        // /* Verify that the sqlite_sequence table exists and is an ordinary
        //     ** rowid table with exactly two columns.
        //     ** Ticket d8dc2b3a58cd5dc2918a1d4acb 2018-05-23 */
        if pSeqTab == std::ptr::null_mut::<Table>()
            || !((unsafe { (*pSeqTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32))
            || (((unsafe { (*pSeqTab).eTabType }) as u32) as i32) == (1 as i32)
            || ((unsafe { (*pSeqTab).nCol }) as i32) != (2 as i32)
        {
            let __v1402: *mut Parse = pParse;
            let __v1403: i32 = unsafe { (*__v1402).nErr };
            let __v1404: i32 = __v1403 + (1 as i32);
            unsafe {
                (*__v1402).nErr = __v1404;
            }
            unsafe {
                (*pParse).rc = (11 as i32) | (2 as i32) << (8 as i32);
            }
            return 0 as i32;
        }
        if ((unsafe { (*pToplevel).__slate_bits_0.__get_usesAinc() }) as i32) == (0 as i32) {
            unsafe {
                (*pToplevel).pAinc = std::ptr::null_mut::<AutoincInfo>();
            }
        }
        pInfo = unsafe { (*pToplevel).pAinc };
        '__slate_break_1173: while pInfo != std::ptr::null_mut::<AutoincInfo>()
            && (unsafe { (*pInfo).pTab }) != pTab
        {
            pInfo = unsafe { (*pInfo).pNext };
        }
        if pInfo == std::ptr::null_mut::<AutoincInfo>() {
            pInfo = (unsafe { sqlite3DbMallocRawNN(unsafe { (*pParse).db }, 24 as u64) })
                as *mut AutoincInfo;
            unsafe {
                sqlite3ParserAddCleanup(
                    pToplevel,
                    unsafe {
                        std::mem::transmute::<
                            *const (),
                            Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
                        >(sqlite3DbFree as *const ())
                    },
                    pInfo as *mut (),
                )
            };
            {}
            if (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8) {
                return 0 as i32;
            }
            unsafe {
                (*pInfo).pNext = unsafe { (*pToplevel).pAinc };
            }
            unsafe {
                (*pToplevel).pAinc = pInfo;
            }
            unsafe {
                (*pToplevel)
                    .__slate_bits_0
                    .__set_usesAinc((1 as i32) as u32);
            }
            unsafe {
                (*pInfo).pTab = pTab;
            }
            unsafe {
                (*pInfo).iDb = iDb;
            }
            // /* Register to hold name of table */
            let __v1405: *mut Parse = pToplevel;
            let __v1406: i32 = unsafe { (*__v1405).nMem };
            let __v1407: i32 = __v1406 + (1 as i32);
            unsafe {
                (*__v1405).nMem = __v1407;
            }
            // /* Max rowid register */
            let __v1408: *mut Parse = pToplevel;
            let __v1409: i32 = unsafe { (*__v1408).nMem };
            let __v1410: i32 = __v1409 + (1 as i32);
            unsafe {
                (*__v1408).nMem = __v1410;
            }
            unsafe {
                (*pInfo).regCtr = __v1410;
            }
            // /* Rowid in sqlite_sequence + orig max val */
            let __v1411: *mut Parse = pToplevel;
            let __v1412: i32 = unsafe { (*__v1411).nMem };
            let __v1413: i32 = __v1412 + (2 as i32);
            unsafe {
                (*__v1411).nMem = __v1413;
            }
        }
        memId = unsafe { (*pInfo).regCtr };
    }
    return memId;
}

// /*
// ** Update the maximum rowid for an autoincrement calculation.
// **
// ** This routine should be called when the regRowid register holds a
// ** new rowid that is about to be inserted.  If that new rowid is
// ** larger than the maximum rowid in the memId memory cell, then the
// ** memory cell is updated.
// */
fn autoIncStep(mut pParse: *mut Parse, mut memId: i32, mut regRowid: i32) {
    if memId > (0 as i32) {
        unsafe { sqlite3VdbeAddOp2(unsafe { (*pParse).pVdbe }, 161 as i32, memId, regRowid) };
    }
}

// /*
// ** This routine generates the code needed to write autoincrement
// ** maximum rowid values back into the sqlite_sequence register.
// ** Every statement that might do an INSERT into an autoincrement
// ** table (either directly or through triggers) needs to call this
// ** routine just before the "exit" code.
// */
fn autoIncrementEnd(mut pParse: *mut Parse) {
    let mut p: *mut AutoincInfo = unsafe { std::mem::zeroed() };
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    0 as i32;
    0 as i32;
    p = unsafe { (*pParse).pAinc };
    '__slate_break_1175: while p != std::ptr::null_mut::<AutoincInfo>() {
        // /* 0 */
        // /* 1 */
        // /* 2 */
        // /* 3 */
        // /* 4 */
        let mut aOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
        let mut pDb: *mut Db =
            unsafe { unsafe { (*db).aDb }.offset((unsafe { (*p).iDb }) as isize) };
        let mut iRec: i32 = 0 as i32;
        let mut memId: i32 = unsafe { (*p).regCtr };
        iRec = unsafe { sqlite3GetTempReg(pParse) };
        0 as i32;
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                56 as i32,
                memId + (2 as i32),
                (unsafe { sqlite3VdbeCurrentAddr(v) }) + (7 as i32),
                memId,
            )
        };
        {}
        sqlite3OpenTable(
            pParse,
            0 as i32,
            unsafe { (*p).iDb },
            unsafe { (*unsafe { (*pDb).pSchema }).pSeqTab },
            116 as i32,
        );
        aOp = unsafe {
            sqlite3VdbeAddOpList(
                v,
                (((20 as u64) / (4 as u64)) as u32) as i32,
                unsafe { std::ptr::addr_of!(autoIncEnd.0) as *const VdbeOpList },
                unsafe { iLn_535 },
            )
        };
        if aOp == std::ptr::null_mut::<VdbeOp>() {
            break '__slate_break_1175;
        }
        unsafe {
            (*unsafe { aOp.offset((0 as i32) as isize) }).p1 = memId + (1 as i32);
        }
        unsafe {
            (*unsafe { aOp.offset((1 as i32) as isize) }).p2 = memId + (1 as i32);
        }
        unsafe {
            (*unsafe { aOp.offset((2 as i32) as isize) }).p1 = memId - (1 as i32);
        }
        unsafe {
            (*unsafe { aOp.offset((2 as i32) as isize) }).p3 = iRec;
        }
        unsafe {
            (*unsafe { aOp.offset((3 as i32) as isize) }).p2 = iRec;
        }
        unsafe {
            (*unsafe { aOp.offset((3 as i32) as isize) }).p3 = memId + (1 as i32);
        }
        unsafe {
            (*unsafe { aOp.offset((3 as i32) as isize) }).p5 = ((8 as i32) as i16) as u16;
        }
        unsafe { sqlite3ReleaseTempReg(pParse, iRec) };
        p = unsafe { (*p).pNext };
    }
}

// /*
// ** Return true if all expressions in the expression-list passed as the
// ** only argument are both constant and have no affinity.
// */
fn exprListIsNoAffinity(mut pParse: *mut Parse, mut pRow: *mut ExprList) -> i32 {
    let mut ii: i32 = 0 as i32;
    if (unsafe { sqlite3ExprListIsConstant(pParse, pRow, 0 as i32) }) == (0 as i32) {
        return 0 as i32;
    }
    ii = 0 as i32;
    '__slate_break_1176: loop {
        if !(ii < unsafe { (*pRow).nExpr }) {
            break;
        }
        let mut pExpr: *mut Expr = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pRow).a) as *mut ExprList_item }
                    .offset(ii as isize)
            })
            .pExpr
        };
        0 as i32;
        0 as i32;
        if (0 as i32) != ((unsafe { sqlite3ExprAffinity(pExpr as *const Expr) }) as i32) {
            return 0 as i32;
        }
        let __v1414: i32 = ii;
        let __v1415: i32 = __v1414 + (1 as i32);
        ii = __v1415;
    }
    return 1 as i32;
}

// /*
// ** Attempt the transfer optimization on INSERTs of the form
// **
// **     INSERT INTO tab1 SELECT * FROM tab2;
// **
// ** The xfer optimization transfers raw records from tab2 over to tab1.
// ** Columns are not decoded and reassembled, which greatly improves
// ** performance.  Raw index records are transferred in the same way.
// **
// ** The xfer optimization is only attempted if tab1 and tab2 are compatible.
// ** There are lots of rules for determining compatibility - see comments
// ** embedded in the code for details.
// **
// ** This routine returns TRUE if the optimization is guaranteed to be used.
// ** Sometimes the xfer optimization will only work if the destination table
// ** is empty - a factor that can only be determined at run-time.  In that
// ** case, this routine generates code for the xfer optimization but also
// ** does a test to see if the destination table is empty and jumps over the
// ** xfer optimization code if the test fails.  In that case, this routine
// ** returns FALSE so that the caller will know to go ahead and generate
// ** an unoptimized transfer.  This routine also returns FALSE if there
// ** is no chance that the xfer optimization can be applied.
// **
// ** This optimization is particularly useful at making VACUUM run faster.
// */
fn xferOptimization(
    mut pParse: *mut Parse,
    mut pDest: *mut Table,
    mut pSelect: *mut Select,
    mut onError: i32,
    mut iDbDest: i32,
) -> i32 {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    // /* The result set of the SELECT */
    let mut pEList: *mut ExprList = unsafe { std::mem::zeroed() };
    // /* The table in the FROM clause of SELECT */
    let mut pSrc: *mut Table = unsafe { std::mem::zeroed() };
    // /* Source and destination indices */
    let mut pSrcIdx: *mut Index = unsafe { std::mem::zeroed() };
    let mut pDestIdx: *mut Index = unsafe { std::mem::zeroed() };
    // /* An element of pSelect->pSrc */
    let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
    // /* Loop counter */
    let mut i: i32 = 0 as i32;
    // /* The database of pSrc */
    let mut iDbSrc: i32 = 0 as i32;
    // /* Cursors from source and destination */
    let mut iSrc: i32 = 0 as i32;
    let mut iDest: i32 = 0 as i32;
    // /* Loop addresses */
    let mut addr1: i32 = 0 as i32;
    let mut addr2: i32 = 0 as i32;
    // /* Address of test for empty pDest */
    let mut emptyDestTest: i32 = 0 as i32;
    // /* Address of test for empty pSrc */
    let mut emptySrcTest: i32 = 0 as i32;
    // /* The VDBE we are building */
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    // /* Memory register used by AUTOINC */
    let mut regAutoinc: i32 = 0 as i32;
    // /* True if pDest has a UNIQUE index */
    let mut destHasUniqueIdx: i32 = 0 as i32;
    // /* Registers holding data and rowid */
    let mut regData: i32 = 0 as i32;
    let mut regRowid: i32 = 0 as i32;
    0 as i32;
    if (unsafe { (*pParse).pWith }) != std::ptr::null_mut::<With>()
        || (unsafe { (*pSelect).pWith }) != std::ptr::null_mut::<With>()
    {
        // /* Do not attempt to process this query if there are an WITH clauses
        //     ** attached to it. Proceeding may generate a false "no such table: xxx"
        //     ** error if pSelect reads from a CTE named "xxx".  */
        return 0 as i32;
    }
    if (((unsafe { (*pDest).eTabType }) as u32) as i32) == (1 as i32) {
        // /* tab1 must not be a virtual table */
        return 0 as i32;
    }
    if onError == (11 as i32) {
        if ((unsafe { (*pDest).iPKey }) as i32) >= (0 as i32) {
            onError = ((unsafe { (*pDest).keyConf }) as u32) as i32;
        }
        if onError == (11 as i32) {
            onError = 2 as i32;
        }
    }
    // /* allocated even if there is no FROM clause */
    0 as i32;
    if (unsafe { (*unsafe { (*pSelect).pSrc }).nSrc }) != (1 as i32) {
        // /* FROM clause must have exactly one term */
        return 0 as i32;
    }
    if ((unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*unsafe { (*pSelect).pSrc }).a) as *mut SrcItem }
                .offset((0 as i32) as isize)
        })
        .fg
        .__slate_bits_0
        .__get_isSubquery()
    }) as i32)
        != (0 as i32)
    {
        // /* FROM clause cannot contain a subquery */
        return 0 as i32;
    }
    if (unsafe { (*pSelect).pWhere }) != std::ptr::null_mut::<Expr>() {
        // /* SELECT may not have a WHERE clause */
        return 0 as i32;
    }
    if (unsafe { (*pSelect).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
        // /* SELECT may not have an ORDER BY clause */
        return 0 as i32;
    }
    // /* Do not need to test for a HAVING clause.  If HAVING is present but
    //   ** there is no ORDER BY, we will get an error. */
    if (unsafe { (*pSelect).pGroupBy }) != std::ptr::null_mut::<ExprList>() {
        // /* SELECT may not have a GROUP BY clause */
        return 0 as i32;
    }
    if (unsafe { (*pSelect).pLimit }) != std::ptr::null_mut::<Expr>() {
        // /* SELECT may not have a LIMIT clause */
        return 0 as i32;
    }
    if (unsafe { (*pSelect).pPrior }) != std::ptr::null_mut::<Select>() {
        // /* SELECT may not be a compound query */
        return 0 as i32;
    }
    if (unsafe { (*pSelect).selFlags }) & ((1 as i32) as u32) != (0 as u32) {
        // /* SELECT may not be DISTINCT */
        return 0 as i32;
    }
    pEList = unsafe { (*pSelect).pEList };
    0 as i32;
    if (unsafe { (*pEList).nExpr }) != (1 as i32) {
        // /* The result set must have exactly one column */
        return 0 as i32;
    }
    0 as i32;
    if (((unsafe {
        (*unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                    .offset((0 as i32) as isize)
            })
            .pExpr
        })
        .op
    }) as u32) as i32)
        != (180 as i32)
    {
        // /* The result set must be the special operator "*" */
        return 0 as i32;
    }
    // /* At this point we have established that the statement is of the
    //   ** correct syntactic form to participate in this optimization.  Now
    //   ** we have to check the semantics.
    //   */
    pItem = unsafe { std::ptr::addr_of_mut!((*unsafe { (*pSelect).pSrc }).a) as *mut SrcItem };
    pSrc = unsafe { sqlite3LocateTableItem(pParse, (0 as i32) as u32, pItem) };
    if pSrc == std::ptr::null_mut::<Table>() {
        // /* FROM clause does not contain a real table */
        return 0 as i32;
    }
    if (unsafe { (*pSrc).tnum }) == unsafe { (*pDest).tnum }
        && (unsafe { (*pSrc).pSchema }) == unsafe { (*pDest).pSchema }
    {
        // /* Possible due to bad sqlite_schema.rootpage */
        {}
        // /* tab1 and tab2 may not be the same table */
        return 0 as i32;
    }
    if (((unsafe { (*pDest).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32)) as i32)
        != (((unsafe { (*pSrc).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32)) as i32)
    {
        // /* source and destination must both be WITHOUT ROWID or not */
        return 0 as i32;
    }
    if !((((unsafe { (*pSrc).eTabType }) as u32) as i32) == (0 as i32)) {
        // /* tab2 may not be a view or virtual table */
        return 0 as i32;
    }
    if ((unsafe { (*pDest).nCol }) as i32) != ((unsafe { (*pSrc).nCol }) as i32) {
        // /* Number of columns must be the same in tab1 and tab2 */
        return 0 as i32;
    }
    if ((unsafe { (*pDest).iPKey }) as i32) != ((unsafe { (*pSrc).iPKey }) as i32) {
        // /* Both tables must have the same INTEGER PRIMARY KEY */
        return 0 as i32;
    }
    if (unsafe { (*pDest).tabFlags }) & ((65536 as i32) as u32) != ((0 as i32) as u32) {
        if (unsafe { (*pSrc).tabFlags }) & ((65536 as i32) as u32) == ((0 as i32) as u32) {
            // /* Cannot feed from a non-strict into a strict table */
            return 0 as i32;
        }
        i = 0 as i32;
        '__slate_break_1216: loop {
            if !(i < ((unsafe { (*pDest).nCol }) as i32)) {
                break;
            }
            let mut eDestType: u32 = ((unsafe {
                (*unsafe { unsafe { (*pDest).aCol }.offset(i as isize) })
                    .__slate_bits_0
                    .__get_eCType()
            }) as i32) as u32;
            let mut eSrcType: u32 = ((unsafe {
                (*unsafe { unsafe { (*pSrc).aCol }.offset(i as isize) })
                    .__slate_bits_0
                    .__get_eCType()
            }) as i32) as u32;
            if eDestType == ((1 as i32) as u32) {
            } else {
                if eDestType == eSrcType {
                } else {
                    if eDestType == ((3 as i32) as u32) && eSrcType == ((4 as i32) as u32) {
                    } else {
                        if eDestType == ((4 as i32) as u32) && eSrcType == ((3 as i32) as u32) {
                        } else {
                            // /* Incompatible types in source and destination */
                            return 0 as i32;
                        }
                    }
                }
            }
            let __v1416: i32 = i;
            let __v1417: i32 = __v1416 + (1 as i32);
            i = __v1417;
        }
    }
    i = 0 as i32;
    '__slate_break_1217: loop {
        if !(i < ((unsafe { (*pDest).nCol }) as i32)) {
            break;
        }
        let mut pDestCol: *mut Column = unsafe { unsafe { (*pDest).aCol }.offset(i as isize) };
        let mut pSrcCol: *mut Column = unsafe { unsafe { (*pSrc).aCol }.offset(i as isize) };
        // /* Even if tables t1 and t2 have identical schemas, if they contain
        //     ** generated columns, then this statement is semantically incorrect:
        //     **
        //     **     INSERT INTO t2 SELECT * FROM t1;
        //     **
        //     ** The reason is that generated column values are returned by the
        //     ** the SELECT statement on the right but the INSERT statement on the
        //     ** left wants them to be omitted.
        //     **
        //     ** Nevertheless, this is a useful notational shorthand to tell SQLite
        //     ** to do a bulk transfer all of the content from t1 over to t2.
        //     **
        //     ** We could, in theory, disable this (except for internal use by the
        //     ** VACUUM command where it is actually needed).  But why do that?  It
        //     ** seems harmless enough, and provides a useful service.
        //     */
        if (((unsafe { (*pDestCol).colFlags }) as u32) as i32) & (96 as i32)
            != (((unsafe { (*pSrcCol).colFlags }) as u32) as i32) & (96 as i32)
        {
            // /* Both columns have the same generated-column type */
            return 0 as i32;
        }
        // /* But the transfer is only allowed if both the source and destination
        //     ** tables have the exact same expressions for generated columns.
        //     ** This requirement could be relaxed for VIRTUAL columns, I suppose.
        //     */
        if (((unsafe { (*pDestCol).colFlags }) as u32) as i32) & (96 as i32) != (0 as i32) {
            if (unsafe {
                sqlite3ExprCompare(
                    std::ptr::null::<Parse>(),
                    (unsafe { sqlite3ColumnExpr(pSrc, pSrcCol) }) as *const Expr,
                    (unsafe { sqlite3ColumnExpr(pDest, pDestCol) }) as *const Expr,
                    -(1 as i32),
                )
            }) != (0 as i32)
            {
                {}
                {}
                // /* Different generator expressions */
                return 0 as i32;
            }
        }
        if ((unsafe { (*pDestCol).affinity }) as i32) != ((unsafe { (*pSrcCol).affinity }) as i32) {
            // /* Affinity must be the same on all columns */
            return 0 as i32;
        }
        if (unsafe {
            sqlite3_stricmp(unsafe { sqlite3ColumnColl(pDestCol) }, unsafe {
                sqlite3ColumnColl(pSrcCol)
            })
        }) != (0 as i32)
        {
            // /* Collating sequence must be the same on all columns */
            return 0 as i32;
        }
        if ((unsafe { (*pDestCol).__slate_bits_0.__get_notNull() }) as i32) != (0 as i32)
            && !(((unsafe { (*pSrcCol).__slate_bits_0.__get_notNull() }) as i32) != (0 as i32))
        {
            // /* tab2 must be NOT NULL if tab1 is */
            return 0 as i32;
        }
        // /* Default values for second and subsequent columns need to match. */
        if (((unsafe { (*pDestCol).colFlags }) as u32) as i32) & (96 as i32) == (0 as i32)
            && i > (0 as i32)
        {
            let mut pDestExpr: *mut Expr = unsafe { sqlite3ColumnExpr(pDest, pDestCol) };
            let mut pSrcExpr: *mut Expr = unsafe { sqlite3ColumnExpr(pSrc, pSrcCol) };
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            if ((pDestExpr == std::ptr::null_mut::<Expr>()) as i32)
                != ((pSrcExpr == std::ptr::null_mut::<Expr>()) as i32)
                || pDestExpr != std::ptr::null_mut::<Expr>()
                    && (unsafe {
                        strcmp(
                            (unsafe { (*pDestExpr).u.zToken }) as *const i8,
                            (unsafe { (*pSrcExpr).u.zToken }) as *const i8,
                        )
                    }) != (0 as i32)
            {
                // /* Default values must be the same for all columns */
                return 0 as i32;
            }
        }
        let __v1418: i32 = i;
        let __v1419: i32 = __v1418 + (1 as i32);
        i = __v1419;
    }
    pDestIdx = unsafe { (*pDest).pIndex };
    '__slate_break_1218: while pDestIdx != std::ptr::null_mut::<Index>() {
        if (((unsafe { (*pDestIdx).onError }) as u32) as i32) != (0 as i32) {
            destHasUniqueIdx = 1 as i32;
        }
        pSrcIdx = unsafe { (*pSrc).pIndex };
        '__slate_break_1219: while pSrcIdx != std::ptr::null_mut::<Index>() {
            if xferCompatibleIndex(pDestIdx, pSrcIdx) != (0 as i32) {
                break '__slate_break_1219;
            }
            pSrcIdx = unsafe { (*pSrcIdx).pNext };
        }
        if pSrcIdx == std::ptr::null_mut::<Index>() {
            // /* pDestIdx has no corresponding index in pSrc */
            return 0 as i32;
        }
        let __v1420: bool;
        if (unsafe { (*pSrcIdx).tnum }) == unsafe { (*pDestIdx).tnum }
            && (unsafe { (*pSrc).pSchema }) == unsafe { (*pDest).pSchema }
        {
            __v1420 = (unsafe { sqlite3FaultSim(411 as i32) }) == (0 as i32);
        } else {
            __v1420 = false as bool;
        }
        if __v1420 {
            // /* The sqlite3FaultSim() call allows this corruption test to be
            //       ** bypassed during testing, in order to exercise other corruption tests
            //       ** further downstream. */
            // /* Corrupt schema - two indexes on the same btree */
            return 0 as i32;
        }
        pDestIdx = unsafe { (*pDestIdx).pNext };
    }
    let __v1421: bool;
    if (unsafe { (*pDest).pCheck }) != std::ptr::null_mut::<ExprList>()
        && (unsafe { (*db).mDbFlags }) & ((4 as i32) as u32) == ((0 as i32) as u32)
    {
        __v1421 = !(xferCompatibleCheck(pDest, pSrc) != (0 as i32));
    } else {
        __v1421 = false as bool;
    }
    if __v1421 {
        // /* Tables have different CHECK constraints.  Ticket #2252 */
        return 0 as i32;
    }
    // /* Disallow the transfer optimization if the destination table contains
    //   ** any foreign key constraints.  This is more restrictive than necessary.
    //   ** But the main beneficiary of the transfer optimization is the VACUUM
    //   ** command, and the VACUUM command disables foreign key constraints.  So
    //   ** the extra complication to make this rule less restrictive is probably
    //   ** not worth the effort.  Ticket [6284df89debdfa61db8073e062908af0c9b6118e]
    //   */
    0 as i32;
    if (unsafe { (*db).flags }) & (((16384 as i32) as i64) as u64) != (((0 as i32) as i64) as u64)
        && (unsafe { (*pDest).u.tab.pFKey }) != std::ptr::null_mut::<FKey>()
    {
        return 0 as i32;
    }
    if (unsafe { (*db).flags }) & (((1 as i32) as i64) as u64) << (32 as i32)
        != (((0 as i32) as i64) as u64)
    {
        // /* xfer opt does not play well with PRAGMA count_changes */
        return 0 as i32;
    }
    if (unsafe { (*db).xAuth }) != None {
        let mut iDb: i32 = unsafe { sqlite3SchemaToIndex(db, unsafe { (*pSrc).pSchema }) };
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
            return 0 as i32;
        }
        i = 0 as i32;
        '__slate_break_1220: loop {
            if !(i < ((unsafe { (*pSrc).nCol }) as i32)) {
                break;
            }
            let mut pSrcCol: *mut Column = unsafe { unsafe { (*pSrc).aCol }.offset(i as isize) };
            if (unsafe {
                sqlite3AuthReadCol(
                    pParse,
                    (unsafe { (*pSrc).zName }) as *const i8,
                    (unsafe { (*pSrcCol).zCnName }) as *const i8,
                    iDb,
                )
            }) != (0 as i32)
            {
                return 0 as i32;
            }
            let __v1422: i32 = i;
            let __v1423: i32 = __v1422 + (1 as i32);
            i = __v1423;
        }
    }
    // /* If we get this far, it means that the xfer optimization is at
    //   ** least a possibility, though it might only work if the destination
    //   ** table (tab1) is initially empty.
    //   */
    iDbSrc = unsafe { sqlite3SchemaToIndex(db, unsafe { (*pSrc).pSchema }) };
    v = unsafe { sqlite3GetVdbe(pParse) };
    unsafe { sqlite3CodeVerifySchema(pParse, iDbSrc) };
    let __v1424: *mut Parse = pParse;
    let __v1425: i32 = unsafe { (*__v1424).nTab };
    let __v1426: i32 = __v1425 + (1 as i32);
    unsafe {
        (*__v1424).nTab = __v1426;
    }
    iSrc = __v1425;
    let __v1427: *mut Parse = pParse;
    let __v1428: i32 = unsafe { (*__v1427).nTab };
    let __v1429: i32 = __v1428 + (1 as i32);
    unsafe {
        (*__v1427).nTab = __v1429;
    }
    iDest = __v1428;
    regAutoinc = autoIncBegin(pParse, iDbDest, pDest);
    regData = unsafe { sqlite3GetTempReg(pParse) };
    unsafe { sqlite3VdbeAddOp2(v, 77 as i32, 0 as i32, regData) };
    regRowid = unsafe { sqlite3GetTempReg(pParse) };
    sqlite3OpenTable(pParse, iDest, iDbDest, pDest, 116 as i32);
    0 as i32;
    if (unsafe { (*db).mDbFlags }) & ((4 as i32) as u32) == ((0 as i32) as u32)
        && (((unsafe { (*pDest).iPKey }) as i32) < (0 as i32)
            && (unsafe { (*pDest).pIndex }) != std::ptr::null_mut::<Index>()
            || destHasUniqueIdx != (0 as i32)
            || onError != (2 as i32) && onError != (1 as i32))
    {
        // /* In some circumstances, we are able to run the xfer optimization
        //     ** only if the destination table is initially empty. Unless the
        //     ** DBFLAG_Vacuum flag is set, this block generates code to make
        //     ** that determination. If DBFLAG_Vacuum is set, then the destination
        //     ** table is always empty.
        //     **
        //     ** Conditions under which the destination must be empty:
        //     **
        //     ** (1) There is no INTEGER PRIMARY KEY but there are indices.
        //     **     (If the destination is not initially empty, the rowid fields
        //     **     of index entries might need to change.)
        //     **
        //     ** (2) The destination has a unique index.  (The xfer optimization
        //     **     is unable to test uniqueness.)
        //     **
        //     ** (3) onError is something other than OE_Abort and OE_Rollback.
        //     */
        addr1 = unsafe { sqlite3VdbeAddOp2(v, 36 as i32, iDest, 0 as i32) };
        {}
        emptyDestTest = unsafe { sqlite3VdbeAddOp0(v, 9 as i32) };
        unsafe { sqlite3VdbeJumpHere(v, addr1) };
    }
    // /* (1) */
    // /* (2) */
    // /* (3) */
    if (unsafe { (*pSrc).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
        let mut insFlags: u8 = 0 as u8;
        sqlite3OpenTable(pParse, iSrc, iDbSrc, pSrc, 114 as i32);
        emptySrcTest = unsafe { sqlite3VdbeAddOp2(v, 36 as i32, iSrc, 0 as i32) };
        {}
        if ((unsafe { (*pDest).iPKey }) as i32) >= (0 as i32) {
            addr1 = unsafe { sqlite3VdbeAddOp2(v, 137 as i32, iSrc, regRowid) };
            if (unsafe { (*db).mDbFlags }) & ((4 as i32) as u32) == ((0 as i32) as u32) {
                {}
                addr2 = unsafe { sqlite3VdbeAddOp3(v, 31 as i32, iDest, 0 as i32, regRowid) };
                {}
                unsafe { sqlite3RowidConstraint(pParse, onError, pDest) };
                unsafe { sqlite3VdbeJumpHere(v, addr2) };
            }
            autoIncStep(pParse, regAutoinc, regRowid);
        } else {
            if (unsafe { (*pDest).pIndex }) == std::ptr::null_mut::<Index>()
                && !((unsafe { (*db).mDbFlags }) & ((8 as i32) as u32) != (0 as u32))
            {
                addr1 = unsafe { sqlite3VdbeAddOp2(v, 129 as i32, iDest, regRowid) };
            } else {
                addr1 = unsafe { sqlite3VdbeAddOp2(v, 137 as i32, iSrc, regRowid) };
                0 as i32;
            }
        }
        if (unsafe { (*db).mDbFlags }) & ((4 as i32) as u32) != (0 as u32) {
            unsafe { sqlite3VdbeAddOp1(v, 139 as i32, iDest) };
            insFlags = (((8 as i32) | (16 as i32) | (128 as i32)) as i8) as u8;
        } else {
            insFlags = (((1 as i32) | (32 as i32) | (8 as i32) | (128 as i32)) as i8) as u8;
        }
        unsafe { sqlite3VdbeAddOp3(v, 131 as i32, iDest, iSrc, regRowid) };
        unsafe { sqlite3VdbeAddOp3(v, 130 as i32, iDest, regData, regRowid) };
        if (unsafe { (*db).mDbFlags }) & ((4 as i32) as u32) == ((0 as i32) as u32) {
            unsafe {
                sqlite3VdbeChangeP4(v, -(1 as i32), (pDest as *mut i8) as *const i8, -(5 as i32))
            };
        }
        unsafe { sqlite3VdbeChangeP5(v, insFlags as u16) };
        unsafe { sqlite3VdbeAddOp2(v, 40 as i32, iSrc, addr1) };
        {}
        unsafe { sqlite3VdbeAddOp2(v, 124 as i32, iSrc, 0 as i32) };
        unsafe { sqlite3VdbeAddOp2(v, 124 as i32, iDest, 0 as i32) };
    } else {
        unsafe {
            sqlite3TableLock(
                pParse,
                iDbDest,
                unsafe { (*pDest).tnum },
                ((1 as i32) as i8) as u8,
                (unsafe { (*pDest).zName }) as *const i8,
            )
        };
        unsafe {
            sqlite3TableLock(
                pParse,
                iDbSrc,
                unsafe { (*pSrc).tnum },
                ((0 as i32) as i8) as u8,
                (unsafe { (*pSrc).zName }) as *const i8,
            )
        };
    }
    pDestIdx = unsafe { (*pDest).pIndex };
    '__slate_break_1221: while pDestIdx != std::ptr::null_mut::<Index>() {
        let mut idxInsFlags: u8 = ((0 as i32) as i8) as u8;
        pSrcIdx = unsafe { (*pSrc).pIndex };
        '__slate_break_1222: while pSrcIdx != std::ptr::null_mut::<Index>() {
            if xferCompatibleIndex(pDestIdx, pSrcIdx) != (0 as i32) {
                break '__slate_break_1222;
            }
            pSrcIdx = unsafe { (*pSrcIdx).pNext };
        }
        0 as i32;
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                114 as i32,
                iSrc,
                (unsafe { (*pSrcIdx).tnum }) as i32,
                iDbSrc,
            )
        };
        unsafe { sqlite3VdbeSetP4KeyInfo(pParse, pSrcIdx) };
        {}
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                116 as i32,
                iDest,
                (unsafe { (*pDestIdx).tnum }) as i32,
                iDbDest,
            )
        };
        unsafe { sqlite3VdbeSetP4KeyInfo(pParse, pDestIdx) };
        unsafe { sqlite3VdbeChangeP5(v, ((1 as i32) as i16) as u16) };
        {}
        addr1 = unsafe { sqlite3VdbeAddOp2(v, 36 as i32, iSrc, 0 as i32) };
        {}
        if (unsafe { (*db).mDbFlags }) & ((4 as i32) as u32) != (0 as u32) {
            // /* This INSERT command is part of a VACUUM operation, which guarantees
            //       ** that the destination table is empty. If all indexed columns use
            //       ** collation sequence BINARY, then it can also be assumed that the
            //       ** index will be populated by inserting keys in strictly sorted
            //       ** order. In this case, instead of seeking within the b-tree as part
            //       ** of every OP_IdxInsert opcode, an OP_SeekEnd is added before the
            //       ** OP_IdxInsert to seek to the point within the b-tree where each key
            //       ** should be inserted. This is faster.
            //       **
            //       ** If any of the indexed columns use a collation sequence other than
            //       ** BINARY, this optimization is disabled. This is because the user
            //       ** might change the definition of a collation sequence and then run
            //       ** a VACUUM command. In that case keys may not be written in strictly
            //       ** sorted order.  */
            i = 0 as i32;
            '__slate_break_1223: loop {
                if !(i < (((unsafe { (*pSrcIdx).nColumn }) as u32) as i32)) {
                    break;
                }
                let mut zColl: *const i8 =
                    unsafe { *unsafe { unsafe { (*pSrcIdx).azColl }.offset(i as isize) } };
                if (unsafe {
                    sqlite3_stricmp(
                        unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 },
                        zColl,
                    )
                }) != (0 as i32)
                {
                    break '__slate_break_1223;
                }
                let __v1430: i32 = i;
                let __v1431: i32 = __v1430 + (1 as i32);
                i = __v1431;
            }
            if i == (((unsafe { (*pSrcIdx).nColumn }) as u32) as i32) {
                idxInsFlags = (((16 as i32) | (128 as i32)) as i8) as u8;
                unsafe { sqlite3VdbeAddOp1(v, 139 as i32, iDest) };
                unsafe { sqlite3VdbeAddOp2(v, 131 as i32, iDest, iSrc) };
            }
        } else {
            if !((unsafe { (*pSrc).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32))
                && ((unsafe { (*pDestIdx).__slate_bits_0.__get_idxType() }) as i32) == (2 as i32)
            {
                let __v1432: u8 = idxInsFlags;
                let __v1433: u8 = ((((__v1432 as u32) as i32) | (1 as i32)) as i8) as u8;
                idxInsFlags = __v1433;
            }
        }
        if ((idxInsFlags as u32) as i32) != (16 as i32) | (128 as i32) {
            unsafe { sqlite3VdbeAddOp3(v, 136 as i32, iSrc, regData, 1 as i32) };
            if (unsafe { (*db).mDbFlags }) & ((4 as i32) as u32) == ((0 as i32) as u32)
                && !((unsafe { (*pDest).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32))
                && ((unsafe { (*pDestIdx).__slate_bits_0.__get_idxType() }) as i32) == (2 as i32)
            {
                {}
            }
        }
        unsafe { sqlite3VdbeAddOp2(v, 140 as i32, iDest, regData) };
        unsafe {
            sqlite3VdbeChangeP5(
                v,
                ((((idxInsFlags as u32) as i32) | (8 as i32)) as i16) as u16,
            )
        };
        unsafe { sqlite3VdbeAddOp2(v, 40 as i32, iSrc, addr1 + (1 as i32)) };
        {}
        unsafe { sqlite3VdbeJumpHere(v, addr1) };
        unsafe { sqlite3VdbeAddOp2(v, 124 as i32, iSrc, 0 as i32) };
        unsafe { sqlite3VdbeAddOp2(v, 124 as i32, iDest, 0 as i32) };
        pDestIdx = unsafe { (*pDestIdx).pNext };
    }
    if emptySrcTest != (0 as i32) {
        unsafe { sqlite3VdbeJumpHere(v, emptySrcTest) };
    }
    unsafe { sqlite3ReleaseTempReg(pParse, regRowid) };
    unsafe { sqlite3ReleaseTempReg(pParse, regData) };
    if emptyDestTest != (0 as i32) {
        sqlite3AutoincrementEnd(pParse);
        unsafe { sqlite3VdbeAddOp2(v, 72 as i32, 0 as i32, 0 as i32) };
        unsafe { sqlite3VdbeJumpHere(v, emptyDestTest) };
        unsafe { sqlite3VdbeAddOp2(v, 124 as i32, iDest, 0 as i32) };
        return 0 as i32;
    } else {
        return 1 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

// /* Parser context */
// /* Name of table into which we are inserting */
// /* A SELECT statement to use as the data source */
// /* Column names corresponding to IDLIST, or NULL. */
// /* How to handle constraint errors */
// /* ON CONFLICT clauses for upsert, or NULL */
// /* Make sure "isView" and other macros defined above are undefined. Otherwise
// ** they may interfere with compilation of other functions in this file
// ** (or in another file, if this file becomes part of the amalgamation).  */
// /*
// ** Meanings of bits in of pWalker->eCode for
// ** sqlite3ExprReferencesUpdatedColumn()
// */
// /* CHECK constraint uses a changing column */
// /* CHECK constraint references the ROWID */
// /* This is the Walker callback from sqlite3ExprReferencesUpdatedColumn().
// *  Set bit 0x01 of pWalker->eCode if pWalker->eCode to 0 and if this
// ** expression node references any of the
// ** columns that are being modified by an UPDATE statement.
// */
// /* Parser context */
// /* The table we are inserting into */
// /* A SELECT statement to use as the data source */
// /* How to handle constraint errors */
// /* The database of pDest */
// /* SQLITE_OMIT_XFER_OPT */
#[unsafe(link_section = ".text.slate_distinct.insert.checkConstraintExprNode")]
extern "C-unwind" fn checkConstraintExprNode(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> i32 {
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (168 as i32) {
        0 as i32;
        if ((unsafe { (*pExpr).iColumn }) as i32) >= (0 as i32) {
            if (unsafe {
                *unsafe {
                    unsafe { (*pWalker).u.aiCol }
                        .offset(((unsafe { (*pExpr).iColumn }) as i32) as isize)
                }
            }) >= (0 as i32)
            {
                let __v1434: *mut Walker = pWalker;
                let __v1435: u16 = unsafe { (*__v1434).eCode };
                let __v1436: u16 = ((((__v1435 as u32) as i32) | (1 as i32)) as i16) as u16;
                unsafe {
                    (*__v1434).eCode = __v1436;
                }
            }
        } else {
            let __v1437: *mut Walker = pWalker;
            let __v1438: u16 = unsafe { (*__v1437).eCode };
            let __v1439: u16 = ((((__v1438 as u32) as i32) | (2 as i32)) as i16) as u16;
            unsafe {
                (*__v1437).eCode = __v1439;
            }
        }
    }
    return 0 as i32;
}

// /* Which entry in the original Table.pIndex list is this index*/
// /* Return the first index on the list */
fn indexIteratorFirst(mut pIter: *mut IndexIterator, mut pIx: *mut i32) -> *mut Index {
    0 as i32;
    if (unsafe { (*pIter).eType }) != (0 as i32) {
        unsafe {
            *pIx = unsafe {
                (*unsafe { unsafe { (*pIter).u.ax.aIdx }.offset((0 as i32) as isize) }).ix
            };
        }
        return unsafe {
            (*unsafe { unsafe { (*pIter).u.ax.aIdx }.offset((0 as i32) as isize) }).p
        };
    } else {
        unsafe {
            *pIx = 0 as i32;
        }
        return unsafe { (*pIter).u.lx.pIdx };
    }
    return unsafe { std::mem::zeroed() };
}

// /* Return the next index from the list.  Return NULL when out of indexes */
fn indexIteratorNext(mut pIter: *mut IndexIterator, mut pIx: *mut i32) -> *mut Index {
    if (unsafe { (*pIter).eType }) != (0 as i32) {
        let mut i: i32 = 0 as i32;
        let __v1440: *mut IndexIterator = pIter;
        let __v1441: i32 = unsafe { (*__v1440).i };
        let __v1442: i32 = __v1441 + (1 as i32);
        unsafe {
            (*__v1440).i = __v1442;
        }
        i = __v1442;
        if i >= unsafe { (*pIter).u.ax.nIdx } {
            unsafe {
                *pIx = i;
            }
            return std::ptr::null_mut::<Index>();
        }
        unsafe {
            *pIx = unsafe { (*unsafe { unsafe { (*pIter).u.ax.aIdx }.offset(i as isize) }).ix };
        }
        return unsafe { (*unsafe { unsafe { (*pIter).u.ax.aIdx }.offset(i as isize) }).p };
    } else {
        let __v1443: *mut i32 = pIx;
        let __v1444: i32 = unsafe { *__v1443 };
        let __v1445: i32 = __v1444 + (1 as i32);
        unsafe {
            *__v1443 = __v1445;
        }
        unsafe {
            (*pIter).u.lx.pIdx = unsafe { (*unsafe { (*pIter).u.lx.pIdx }).pNext };
        }
        return unsafe { (*pIter).u.lx.pIdx };
    }
    return unsafe { std::mem::zeroed() };
}

// /* Parsing context */
// /* Table to be opened */
// /* OP_OpenRead or OP_OpenWrite */
// /* P5 value for OP_Open* opcodes (except on WITHOUT ROWID) */
// /* Use this for the table cursor, if there is one */
// /* If not NULL: boolean for each table and index */
// /* Write the database source cursor number here */
// /* Write the first index cursor number here */
// /*
// ** Check to see if index pSrc is compatible as a source of data
// ** for index pDest in an insert transfer optimization.  The rules
// ** for a compatible index:
// **
// **    *   The index is over the same set of columns
// **    *   The same DESC and ASC markings occurs on all columns
// **    *   The same onError processing (OE_Abort, OE_Ignore, etc)
// **    *   The same collating sequence on each column
// **    *   The index has the exact same WHERE clause
// */
fn xferCompatibleIndex(mut pDest: *mut Index, mut pSrc: *mut Index) -> i32 {
    let mut i: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    if (((unsafe { (*pDest).nKeyCol }) as u32) as i32)
        != (((unsafe { (*pSrc).nKeyCol }) as u32) as i32)
        || (((unsafe { (*pDest).nColumn }) as u32) as i32)
            != (((unsafe { (*pSrc).nColumn }) as u32) as i32)
    {
        // /* Different number of columns */
        return 0 as i32;
    }
    if (((unsafe { (*pDest).onError }) as u32) as i32)
        != (((unsafe { (*pSrc).onError }) as u32) as i32)
    {
        // /* Different conflict resolution strategies */
        return 0 as i32;
    }
    i = 0 as i32;
    '__slate_break_1215: loop {
        if !(i < (((unsafe { (*pSrc).nKeyCol }) as u32) as i32)) {
            break;
        }
        if ((unsafe { *unsafe { unsafe { (*pSrc).aiColumn }.offset(i as isize) } }) as i32)
            != ((unsafe { *unsafe { unsafe { (*pDest).aiColumn }.offset(i as isize) } }) as i32)
        {
            // /* Different columns indexed */
            return 0 as i32;
        }
        if ((unsafe { *unsafe { unsafe { (*pSrc).aiColumn }.offset(i as isize) } }) as i32)
            == -(2 as i32)
        {
            0 as i32;
            if (unsafe {
                sqlite3ExprCompare(
                    std::ptr::null::<Parse>(),
                    (unsafe {
                        (*unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!((*unsafe { (*pSrc).aColExpr }).a)
                                    as *mut ExprList_item
                            }
                            .offset(i as isize)
                        })
                        .pExpr
                    }) as *const Expr,
                    (unsafe {
                        (*unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!((*unsafe { (*pDest).aColExpr }).a)
                                    as *mut ExprList_item
                            }
                            .offset(i as isize)
                        })
                        .pExpr
                    }) as *const Expr,
                    -(1 as i32),
                )
            }) != (0 as i32)
            {
                // /* Different expressions in the index */
                return 0 as i32;
            }
        }
        if (((unsafe { *unsafe { unsafe { (*pSrc).aSortOrder }.offset(i as isize) } }) as u32)
            as i32)
            != (((unsafe { *unsafe { unsafe { (*pDest).aSortOrder }.offset(i as isize) } }) as u32)
                as i32)
        {
            // /* Different sort orders */
            return 0 as i32;
        }
        if (unsafe {
            sqlite3_stricmp(
                unsafe { *unsafe { unsafe { (*pSrc).azColl }.offset(i as isize) } },
                unsafe { *unsafe { unsafe { (*pDest).azColl }.offset(i as isize) } },
            )
        }) != (0 as i32)
        {
            // /* Different collating sequences */
            return 0 as i32;
        }
        let __v1446: i32 = i;
        let __v1447: i32 = __v1446 + (1 as i32);
        i = __v1447;
    }
    if (unsafe {
        sqlite3ExprCompare(
            std::ptr::null::<Parse>(),
            (unsafe { (*pSrc).pPartIdxWhere }) as *const Expr,
            (unsafe { (*pDest).pPartIdxWhere }) as *const Expr,
            -(1 as i32),
        )
    }) != (0 as i32)
    {
        // /* Different WHERE clauses */
        return 0 as i32;
    }
    // /* If no test above fails then the indices must be compatible */
    return 1 as i32;
}

// /*
// ** Examine an expression node and abort if it references the ROWID.
// ** This is a Walker callback used by xferCompatibleCheck()
// */
#[unsafe(link_section = ".text.slate_distinct.insert.xferCheckRowid")]
extern "C-unwind" fn xferCheckRowid(mut pWalk: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (168 as i32)
        && ((unsafe { (*pExpr).iColumn }) as i32) < (0 as i32)
    {
        unsafe {
            (*pWalk).eCode = ((1 as i32) as i16) as u16;
        }
        return 2 as i32;
    } else {
        return 0 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Analyze CHECK constraints on the source and destination tables and
// ** return true if those CHECK constraints are compatible with the
// ** xfer-optimization.
// **
// **    *  The pDest and pSrc tables must have identical CHECK constraints.
// **
// **    *  If the destination table, pDest, does not have an
// **       INTEGER PRIMARY KEY column, then no CHECK constraint may
// **       referenced the ROWID.  (See forum post 2026-05-11T13:15:57Z)
// */
fn xferCompatibleCheck(mut pDest: *mut Table, mut pSrc: *mut Table) -> i32 {
    if (unsafe {
        sqlite3ExprListCompare(
            (unsafe { (*pSrc).pCheck }) as *const ExprList,
            (unsafe { (*pDest).pCheck }) as *const ExprList,
            -(1 as i32),
        )
    }) != (0 as i32)
    {
        return 0 as i32;
    }
    if ((unsafe { (*pDest).iPKey }) as i32) < (0 as i32) {
        let mut w: Walker = unsafe { std::mem::zeroed() };
        unsafe { memset(std::ptr::addr_of_mut!(w) as *mut (), 0 as i32, 48 as u64) };
        w.xExprCallback = Some(xferCheckRowid);
        unsafe { sqlite3WalkExprList(std::ptr::addr_of_mut!(w), unsafe { (*pDest).pCheck }) };
        if w.eCode != (0 as u16) {
            return 0 as i32;
        }
    }
    return 1 as i32;
}
