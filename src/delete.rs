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
//! This file contains C code routines that are called by the parser
//! in order to generate code for DELETE FROM statements.
unsafe extern "C" {
    fn sqlite3_stricmp(__v494: *const i8, __v495: *const i8) -> i32;
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3VdbeAddOp0(__v499: *mut Vdbe, __v500: i32) -> i32;
    fn sqlite3VdbeAddOp1(__v501: *mut Vdbe, __v502: i32, __v503: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v504: *mut Vdbe, __v505: i32, __v506: i32, __v507: i32) -> i32;
    fn sqlite3VdbeGoto(__v508: *mut Vdbe, __v509: i32) -> i32;
    fn sqlite3VdbeAddOp3(
        __v510: *mut Vdbe,
        __v511: i32,
        __v512: i32,
        __v513: i32,
        __v514: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4(
        __v515: *mut Vdbe,
        __v516: i32,
        __v517: i32,
        __v518: i32,
        __v519: i32,
        zP4: *const i8,
        __v521: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4Int(
        __v522: *mut Vdbe,
        __v523: i32,
        __v524: i32,
        __v525: i32,
        __v526: i32,
        __v527: i32,
    ) -> i32;
    fn sqlite3VdbeChangeP5(__v528: *mut Vdbe, P5: u16);
    fn sqlite3VdbeJumpHere(__v530: *mut Vdbe, addr: i32);
    fn sqlite3VdbeJumpHereOrPopInst(__v532: *mut Vdbe, addr: i32);
    fn sqlite3VdbeChangeToNoop(__v534: *mut Vdbe, addr: i32) -> i32;
    fn sqlite3VdbeDeletePriorOpcode(__v536: *mut Vdbe, op: u8) -> i32;
    fn sqlite3VdbeChangeP4(__v538: *mut Vdbe, addr: i32, zP4: *const i8, N: i32);
    fn sqlite3VdbeAppendP4(__v542: *mut Vdbe, pP4: *mut (), p4type: i32);
    fn sqlite3VdbeSetP4KeyInfo(__v545: *mut Parse, __v546: *mut Index);
    fn sqlite3VdbeMakeLabel(__v547: *mut Parse) -> i32;
    fn sqlite3VdbeResolveLabel(__v548: *mut Vdbe, __v549: i32);
    fn sqlite3VdbeCurrentAddr(__v550: *mut Vdbe) -> i32;
    fn sqlite3VdbeSetNumCols(__v551: *mut Vdbe, __v552: i32);
    fn sqlite3VdbeSetColName(
        __v553: *mut Vdbe,
        __v554: i32,
        __v555: i32,
        __v556: *const i8,
        __v557: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3VdbeCountChanges(__v558: *mut Vdbe);
    fn sqlite3DbMallocRawNN(__v559: *mut sqlite3, __v560: u64) -> *mut ();
    fn sqlite3DbStrDup(__v561: *mut sqlite3, __v562: *const i8) -> *mut i8;
    fn sqlite3DbNNFreeNN(__v563: *mut sqlite3, __v564: *mut ());
    fn sqlite3ErrorMsg(__v565: *mut Parse, __v566: *const i8, ...);
    fn sqlite3GetTempRange(__v567: *mut Parse, __v568: i32) -> i32;
    fn sqlite3ReleaseTempRange(__v569: *mut Parse, __v570: i32, __v571: i32);
    fn sqlite3ExprDelete(__v572: *mut sqlite3, __v573: *mut Expr);
    fn sqlite3PrimaryKeyIndex(__v574: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToStorage(__v575: *mut Table, __v576: i16) -> i16;
    fn sqlite3ViewGetColumnNames(__v577: *mut Parse, __v578: *mut Table) -> i32;
    fn sqlite3DeleteTable(__v579: *mut sqlite3, __v580: *mut Table);
    fn sqlite3AutoincrementEnd(pParse: *mut Parse);
    fn sqlite3SrcListAppend(
        __v582: *mut Parse,
        __v583: *mut SrcList,
        __v584: *mut Token,
        __v585: *mut Token,
    ) -> *mut SrcList;
    fn sqlite3IndexedByLookup(__v586: *mut Parse, __v587: *mut SrcItem) -> i32;
    fn sqlite3SrcListDelete(__v588: *mut sqlite3, __v589: *mut SrcList);
    fn sqlite3Select(__v590: *mut Parse, __v591: *mut Select, __v592: *mut SelectDest) -> i32;
    fn sqlite3SelectNew(
        __v593: *mut Parse,
        __v594: *mut ExprList,
        __v595: *mut SrcList,
        __v596: *mut Expr,
        __v597: *mut ExprList,
        __v598: *mut Expr,
        __v599: *mut ExprList,
        __v600: u32,
        __v601: *mut Expr,
    ) -> *mut Select;
    fn sqlite3SelectDelete(__v602: *mut sqlite3, __v603: *mut Select);
    fn sqlite3WhereBegin(
        __v617: *mut Parse,
        __v618: *mut SrcList,
        __v619: *mut Expr,
        __v620: *mut ExprList,
        __v621: *mut ExprList,
        __v622: *mut Select,
        __v623: u16,
        __v624: i32,
    ) -> *mut WhereInfo;
    fn sqlite3WhereEnd(__v625: *mut WhereInfo);
    fn sqlite3WhereOkOnePass(__v626: *mut WhereInfo, __v627: *mut i32) -> i32;
    fn sqlite3WhereUsesDeferredSeek(__v628: *mut WhereInfo) -> i32;
    fn sqlite3ExprCodeLoadIndexColumn(
        __v629: *mut Parse,
        __v630: *mut Index,
        __v631: i32,
        __v632: i32,
        __v633: i32,
    );
    fn sqlite3ExprCodeGetColumnOfTable(
        __v634: *mut Vdbe,
        __v635: *mut Table,
        __v636: i32,
        __v637: i32,
        __v638: i32,
    );
    fn sqlite3ExprIfFalseDup(__v639: *mut Parse, __v640: *mut Expr, __v641: i32, __v642: i32);
    fn sqlite3LocateTableItem(__v643: *mut Parse, flags: u32, __v645: *mut SrcItem) -> *mut Table;
    fn sqlite3GetVdbe(__v646: *mut Parse) -> *mut Vdbe;
    fn sqlite3OpenTableAndIndices(
        __v674: *mut Parse,
        __v675: *mut Table,
        __v676: i32,
        __v677: u8,
        __v678: i32,
        __v679: *mut u8,
        __v680: *mut i32,
        __v681: *mut i32,
    ) -> i32;
    fn sqlite3BeginWriteOperation(__v682: *mut Parse, __v683: i32, __v684: i32);
    fn sqlite3MultiWrite(__v685: *mut Parse);
    fn sqlite3MayAbort(__v686: *mut Parse);
    fn sqlite3ExprDup(__v687: *mut sqlite3, __v688: *const Expr, __v689: i32) -> *mut Expr;
    fn sqlite3TriggersExist(
        __v696: *mut Parse,
        __v697: *mut Table,
        __v698: i32,
        __v699: *mut ExprList,
        pMask: *mut i32,
    ) -> *mut Trigger;
    fn sqlite3CodeRowTrigger(
        __v701: *mut Parse,
        __v702: *mut Trigger,
        __v703: i32,
        __v704: *mut ExprList,
        __v705: i32,
        __v706: *mut Table,
        __v707: i32,
        __v708: i32,
        __v709: i32,
    );
    fn sqlite3TriggerColmask(
        __v710: *mut Parse,
        __v711: *mut Trigger,
        __v712: *mut ExprList,
        __v713: i32,
        __v714: i32,
        __v715: *mut Table,
        __v716: i32,
    ) -> u32;
    fn sqlite3AuthCheck(
        __v717: *mut Parse,
        __v718: i32,
        __v719: *const i8,
        __v720: *const i8,
        __v721: *const i8,
    ) -> i32;
    fn sqlite3AuthContextPush(__v722: *mut Parse, __v723: *mut AuthContext, __v724: *const i8);
    fn sqlite3AuthContextPop(__v725: *mut AuthContext);
    fn sqlite3IndexAffinityStr(__v726: *mut sqlite3, __v727: *mut Index) -> *const i8;
    fn sqlite3WritableSchema(__v728: *mut sqlite3) -> i32;
    fn sqlite3ResolveExprNames(__v729: *mut NameContext, __v730: *mut Expr) -> i32;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, __v732: *mut Schema) -> i32;
    fn sqlite3SelectDestInit(__v733: *mut SelectDest, __v734: i32, __v735: i32);
    fn sqlite3TableLock(
        __v736: *mut Parse,
        __v737: i32,
        __v738: u32,
        __v739: u8,
        __v740: *const i8,
    );
    fn sqlite3GetVTable(__v741: *mut sqlite3, __v742: *mut Table) -> *mut VTable;
    fn sqlite3ReadOnlyShadowTables(db: *mut sqlite3) -> i32;
    fn sqlite3VtabMakeWritable(__v744: *mut Parse, __v745: *mut Table);
    fn sqlite3FkCheck(
        __v746: *mut Parse,
        __v747: *mut Table,
        __v748: i32,
        __v749: i32,
        __v750: *mut i32,
        __v751: i32,
    );
    fn sqlite3FkActions(
        __v752: *mut Parse,
        __v753: *mut Table,
        __v754: *mut ExprList,
        __v755: i32,
        __v756: *mut i32,
        __v757: i32,
    );
    fn sqlite3FkRequired(
        __v758: *mut Parse,
        __v759: *mut Table,
        __v760: *mut i32,
        __v761: i32,
    ) -> i32;
    fn sqlite3FkOldmask(__v762: *mut Parse, __v763: *mut Table) -> u32;
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
struct Hash {
    htsize: u32,
    count: u32,
    first: *mut HashElem,
    ht: *mut _ht,
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
    trace: __SlateRecord163,
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
    u1: __SlateRecord164,
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
    u: __SlateRecord165,
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
    __slate_bits_0: __slate_bits::__SlateBits64U0,
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
    u: __SlateRecord166,
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
    __slate_bits_0: __slate_bits::__SlateBits88U0,
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
    u: __SlateRecord174,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord175,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord176,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord177,
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
    fg: __SlateRecord184,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord185,
    u2: __SlateRecord186,
    u3: __SlateRecord187,
    u4: __SlateRecord188,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RenameToken {}

#[repr(C)]
#[derive(Clone, Copy)]
struct SrcList {
    nSrc: i32,
    nAlloc: u32,
    a: [SrcItem; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct NameContext {
    pParse: *mut Parse,
    pSrcList: *mut SrcList,
    uNC: __SlateRecord189,
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
    __slate_bits_0: __slate_bits::__SlateBits102U0,
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
    u1: __SlateRecord191,
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
struct AuthContext {
    zAuthContext: *const i8,
    pParse: *mut Parse,
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
struct WhereInfo {}

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
    __slate_bits_0: __slate_bits::__SlateBits162U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord163 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord164 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord165 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord166 {
    tab: __SlateRecord167,
    view: __SlateRecord168,
    vtab: __SlateRecord169,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord167 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord168 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord169 {
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
union __SlateRecord174 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord175 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord178,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord178 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord180,
    u: __SlateRecord181,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord180 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits180U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    x: __SlateRecord182,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord182 {
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
struct __SlateRecord184 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits184U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord185 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord189 {
    pEList: *mut ExprList,
    pAggInfo: *mut AggInfo,
    pUpsert: *mut Upsert,
    iBaseReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    cr: __SlateRecord192,
    d: __SlateRecord193,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord192 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord193 {
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
    pub struct __SlateBits64U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits180U0 {
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
    pub struct __SlateBits184U0 {
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
    pub struct __SlateBits162U0 {
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
    pub struct __SlateBits102U0 {
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

/// While a SrcList can in general represent multiple tables and subqueries
/// (as in the FROM clause of a SELECT statement) in this case it contains
/// the name of a single table, as one might find in an INSERT, DELETE,
/// or UPDATE statement.  Look up that table in the symbol table and
/// return a pointer.  Set an error message and return NULL if the table
/// name is not found or if any other error occurs.
///
/// The following fields are initialized appropriate in pSrc:
///
///    pSrc->a[0].spTab        Pointer to the Table object
///    pSrc->a[0].u2.pIBIndex  Pointer to the INDEXED BY index, if there is one
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SrcListLookup(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
) -> *mut Table {
    let mut pItem: *mut SrcItem = unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem };
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    0 as i32;
    pTab = unsafe { sqlite3LocateTableItem(pParse, (0 as i32) as u32, pItem) };
    if (unsafe { (*pItem).pSTab }) != std::ptr::null_mut::<Table>() {
        unsafe { sqlite3DeleteTable(unsafe { (*pParse).db }, unsafe { (*pItem).pSTab }) };
    }
    unsafe {
        (*pItem).pSTab = pTab;
    }
    unsafe {
        (*pItem).fg.__slate_bits_0.__set_notCte((1 as i32) as u32);
    }
    if pTab != std::ptr::null_mut::<Table>() {
        let __v775: *mut Table = pTab;
        let __v776: u32 = unsafe { (*__v775).nTabRef };
        let __v777: u32 = __v776.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v775).nTabRef = __v777;
        }
        let __v778: bool;
        if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isIndexedBy() }) as i32) != (0 as i32) {
            __v778 = (unsafe { sqlite3IndexedByLookup(pParse, pItem) }) != (0 as i32);
        } else {
            __v778 = false as bool;
        }
        if __v778 {
            pTab = std::ptr::null_mut::<Table>();
        }
    }
    return pTab;
}

/// Generate byte-code that will report the number of rows modified
/// by a DELETE, INSERT, or UPDATE statement.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CodeChangeCount(
    mut v: *mut Vdbe,
    mut regCounter: i32,
    mut zColName: *const i8,
) {
    unsafe { sqlite3VdbeAddOp0(v, 85 as i32) };
    unsafe { sqlite3VdbeAddOp2(v, 86 as i32, regCounter, 1 as i32) };
    unsafe { sqlite3VdbeSetNumCols(v, 1 as i32) };
    unsafe { sqlite3VdbeSetColName(v, 0 as i32, 0 as i32, zColName, None) };
}

/// Return true if table pTab is read-only.
///
/// A table is read-only if any of the following are true:
///
///   1) It is a virtual table and no implementation of the xUpdate method
///      has been provided
///
///   2) A trigger is currently being coded and the table is a virtual table
///      that is SQLITE_VTAB_DIRECTONLY or if PRAGMA trusted_schema=OFF and
///      the table is not SQLITE_VTAB_INNOCUOUS.
///
///   3) It is a system table (i.e. sqlite_schema), this call is not
///      part of a nested parse and writable_schema pragma has not
///      been specified
///
///   4) The table is a shadow table, the database connection is in
///      defensive mode, and the current sqlite3_prepare()
///      is for a top-level SQL statement.
fn vtabIsReadOnly(mut pParse: *mut Parse, mut pTab: *mut Table) -> i32 {
    0 as i32;
    if (unsafe {
        (*unsafe {
            (*unsafe { (*unsafe { sqlite3GetVTable(unsafe { (*pParse).db }, pTab) }).pMod }).pModule
        })
        .xUpdate
    }) == None
    {
        return 1 as i32;
    }
    // Within triggers:
    // *  Do not allow DELETE, INSERT, or UPDATE of SQLITE_VTAB_DIRECTONLY
    //    virtual tables
    // *  Only allow DELETE, INSERT, or UPDATE of non-SQLITE_VTAB_INNOCUOUS
    //    virtual tables if PRAGMA trusted_schema=ON.
    if ((unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>()
        || (((unsafe { (*pParse).prepFlags }) as u32) as i32) & (32 as i32) != (0 as i32))
        && (((unsafe { (*unsafe { (*pTab).u.vtab.p }).eVtabRisk }) as u32) as i32)
            > (((unsafe { (*unsafe { (*pParse).db }).flags }) & (((128 as i32) as i64) as u64)
                != (((0 as i32) as i64) as u64)) as i32)
    {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"unsafe use of virtual table \"%s\"\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pTab).zName },
            )
        };
    }
    return 0 as i32;
}

fn tabIsReadOnly(mut pParse: *mut Parse, mut pTab: *mut Table) -> i32 {
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32) {
        return vtabIsReadOnly(pParse, pTab);
    }
    if (unsafe { (*pTab).tabFlags }) & (((1 as i32) | (4096 as i32)) as u32) == ((0 as i32) as u32)
    {
        return 0 as i32;
    }
    db = unsafe { (*pParse).db };
    if (unsafe { (*pTab).tabFlags }) & ((1 as i32) as u32) != ((0 as i32) as u32) {
        return ((unsafe { sqlite3WritableSchema(db) }) == (0 as i32)
            && (((unsafe { (*pParse).nested }) as u32) as i32) == (0 as i32))
            as i32;
    }
    0 as i32;
    return unsafe { sqlite3ReadOnlyShadowTables(db) };
}

/// Check to make sure the given table is writable.
///
/// If pTab is not writable  ->  generate an error message and return 1.
/// If pTab is writable but other errors have occurred -> return 1.
/// If pTab is writable and no prior errors -> return 0;
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IsReadOnly(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pTrigger: *mut Trigger,
) -> i32 {
    if tabIsReadOnly(pParse, pTab) != (0 as i32) {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"table %s may not be modified\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pTab).zName },
            )
        };
        return 1 as i32;
    }
    if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (2 as i32)
        && (pTrigger == std::ptr::null_mut::<Trigger>()
            || (unsafe { (*pTrigger).bReturning }) != (0 as u8)
                && (unsafe { (*pTrigger).pNext }) == std::ptr::null_mut::<Trigger>())
    {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"cannot modify %s because it is a view\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pTab).zName },
            )
        };
        return 1 as i32;
    }
    return 0 as i32;
}

/// Evaluate a view and store its result in an ephemeral table.  The
/// pWhere argument is an optional WHERE clause that restricts the
/// set of rows in the view that are to be added to the ephemeral table.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pView` - View definition
/// * `pWhere` - Optional WHERE clause to be added
/// * `pOrderBy` - Optional ORDER BY clause
/// * `pLimit` - Optional LIMIT clause
/// * `iCur` - Cursor number for ephemeral table
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MaterializeView(
    mut pParse: *mut Parse,
    mut pView: *mut Table,
    mut pWhere: *mut Expr,
    mut pOrderBy: *mut ExprList,
    mut pLimit: *mut Expr,
    mut iCur: i32,
) {
    let mut dest: SelectDest = unsafe { std::mem::zeroed() };
    let mut pSel: *mut Select = unsafe { std::mem::zeroed() };
    let mut pFrom: *mut SrcList = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut iDb: i32 = unsafe { sqlite3SchemaToIndex(db, unsafe { (*pView).pSchema }) };
    pWhere = unsafe { sqlite3ExprDup(db, pWhere as *const Expr, 0 as i32) };
    pFrom = unsafe {
        sqlite3SrcListAppend(
            pParse,
            std::ptr::null_mut::<SrcList>(),
            std::ptr::null_mut::<Token>(),
            std::ptr::null_mut::<Token>(),
        )
    };
    if pFrom != std::ptr::null_mut::<SrcList>() {
        0 as i32;
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pFrom).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .zName = unsafe { sqlite3DbStrDup(db, (unsafe { (*pView).zName }) as *const i8) };
        }
        0 as i32;
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pFrom).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .u4
            .zDatabase = unsafe {
                sqlite3DbStrDup(
                    db,
                    (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName })
                        as *const i8,
                )
            };
        }
        0 as i32;
        0 as i32;
    }
    pSel = unsafe {
        sqlite3SelectNew(
            pParse,
            std::ptr::null_mut::<ExprList>(),
            pFrom,
            pWhere,
            std::ptr::null_mut::<ExprList>(),
            std::ptr::null_mut::<Expr>(),
            pOrderBy,
            (131072 as i32) as u32,
            pLimit,
        )
    };
    unsafe { sqlite3SelectDestInit(std::ptr::addr_of_mut!(dest), 10 as i32, iCur) };
    unsafe { sqlite3Select(pParse, pSel, std::ptr::addr_of_mut!(dest)) };
    unsafe { sqlite3SelectDelete(db, pSel) };
}

//      && !defined(SQLITE_OMIT_SUBQUERY)
/// Generate code for a DELETE FROM statement.
///
///     DELETE FROM table_wxyz WHERE a<5 AND b NOT NULL;
///                 \________/       \________________/
///                  pTabList              pWhere
///
/// # Arguments
///
/// * `pParse` - The parser context
/// * `pTabList` - The table from which we should delete things
/// * `pWhere` - The WHERE clause.  May be null
/// * `pOrderBy` - ORDER BY clause. May be null
/// * `pLimit` - LIMIT clause. May be null
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DeleteFrom(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pWhere: *mut Expr,
    mut pOrderBy: *mut ExprList,
    mut pLimit: *mut Expr,
) {
    let mut __slate_storage_444: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_444: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_444) as *mut *const i8; // True to count changes
    let mut __slate_storage_445: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_445: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_445) as *mut i32;
    let mut __slate_storage_443: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_443: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_443) as *mut i32;
    let mut __slate_storage_812: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_812: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_812) as *mut i32;
    let mut __slate_storage_811: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_811: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_811) as *mut i32;
    // Add the PK key for this row to the temporary table
    let mut __slate_storage_810: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_810: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_810) as *mut *mut Parse;
    let mut __slate_storage_806: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_806: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_806) as *mut i32;
    let mut __slate_storage_805: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_805: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_805) as *mut i32;
    let mut __slate_storage_809: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_809: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_809) as *mut i32;
    let mut __slate_storage_808: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_808: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_808) as *mut i32;
    let mut __slate_storage_807: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_807: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_807) as *mut *mut Parse;
    let mut __slate_storage_798: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_798: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_798) as *mut i32;
    let mut __slate_storage_797: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_797: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_797) as *mut i32;
    let mut __slate_storage_796: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_796: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_796) as *mut *mut Parse;
    let mut __slate_storage_804: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_804: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_804) as *mut i32;
    let mut __slate_storage_803: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_803: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_803) as *mut i32;
    let mut __slate_storage_802: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_802: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_802) as *mut *mut Parse;
    let mut __slate_storage_801: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_801: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_801) as *mut i32;
    let mut __slate_storage_800: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_800: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_800) as *mut i32;
    let mut __slate_storage_799: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_799: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_799) as *mut *mut Parse;
    let mut __slate_storage_795: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_795: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_795) as *mut u16;
    let mut __slate_storage_794: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_794: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_794) as *mut u16;
    let mut __slate_storage_442: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_442: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_442) as *mut u16;
    let mut __slate_storage_793: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_793: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_793) as *mut i32;
    let mut __slate_storage_792: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_792: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_792) as *mut i32;
    let mut __slate_storage_791: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_791: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_791) as *mut *mut Parse;
    let mut __slate_storage_790: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_790: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_790) as *mut i32;
    let mut __slate_storage_786: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_786: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_786) as *mut i32;
    let mut __slate_storage_785: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_785: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_785) as *mut i32;
    let mut __slate_storage_789: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_789: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_789) as *mut i32;
    let mut __slate_storage_788: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_788: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_788) as *mut i32;
    let mut __slate_storage_787: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_787: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_787) as *mut *mut Parse;
    let mut __slate_storage_784: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_784: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_784) as *mut *mut Index;
    let mut __slate_storage_783: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_783: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_783) as *mut i32;
    let mut __slate_storage_782: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_782: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_782) as *mut i32;
    let mut __slate_storage_781: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_781: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_781) as *mut i32;
    let mut __slate_storage_780: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_780: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_780) as *mut *mut Parse;
    let mut __slate_storage_779: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_779: *mut bool = std::ptr::addr_of_mut!(__slate_storage_779) as *mut bool; // List of table triggers, if required
    let mut __slate_storage_441: std::mem::MaybeUninit<*mut Trigger> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_441: *mut *mut Trigger =
        std::ptr::addr_of_mut!(__slate_storage_441) as *mut *mut Trigger; // True if attempting to delete from a view
    let mut __slate_storage_440: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_440: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_440) as *mut i32;
    // True if there are triggers or FKs or
    // subqueries in the WHERE clause
    let mut __slate_storage_439: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_439: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_439) as *mut i32; // Instruction to open the Ephemeral table
    let mut __slate_storage_438: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_438: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_438) as *mut i32; // Top of the delete loop
    let mut __slate_storage_437: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_437: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_437) as *mut i32; // Address of jump over the delete logic
    let mut __slate_storage_436: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_436: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_436) as *mut i32; // Register for rowset of rows to delete
    let mut __slate_storage_435: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_435: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_435) as *mut i32; // Ephemeral table holding all primary key values
    let mut __slate_storage_434: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_434: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_434) as *mut i32; // Number of memory cells in the row key
    let mut __slate_storage_433: std::mem::MaybeUninit<i16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_433: *mut i16 = std::ptr::addr_of_mut!(__slate_storage_433) as *mut i16; // Memory cell holding key of row to be deleted
    let mut __slate_storage_432: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_432: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_432) as *mut i32; // Number of columns in the PRIMARY KEY
    let mut __slate_storage_431: std::mem::MaybeUninit<i16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_431: *mut i16 = std::ptr::addr_of_mut!(__slate_storage_431) as *mut i16; // First of nPk registers holding PRIMARY KEY value
    let mut __slate_storage_430: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_430: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_430) as *mut i32; // The PRIMARY KEY index on the table
    let mut __slate_storage_429: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_429: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_429) as *mut *mut Index; // Open cursor iTabCur+j if aToOpen[j] is true
    let mut __slate_storage_428: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_428: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_428) as *mut *mut u8; // The write cursors opened by WHERE_ONEPASS
    let mut __slate_storage_427: std::mem::MaybeUninit<[i32; 2]> = std::mem::MaybeUninit::uninit();
    let __slate_slot_427: *mut [i32; 2] =
        std::ptr::addr_of_mut!(__slate_storage_427) as *mut [i32; 2]; // ONEPASS_OFF or _SINGLE or _MULTI
    let mut __slate_storage_426: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_426: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_426) as *mut i32; // Value returned by authorization callback
    let mut __slate_storage_425: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_425: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_425) as *mut i32; // Memory cell used for change counting
    let mut __slate_storage_424: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_424: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_424) as *mut i32; // Database number
    let mut __slate_storage_423: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_423: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_423) as *mut i32; // Name context to resolve expressions in
    let mut __slate_storage_422: std::mem::MaybeUninit<NameContext> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_422: *mut NameContext =
        std::ptr::addr_of_mut!(__slate_storage_422) as *mut NameContext; // Authorization context
    let mut __slate_storage_421: std::mem::MaybeUninit<AuthContext> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_421: *mut AuthContext =
        std::ptr::addr_of_mut!(__slate_storage_421) as *mut AuthContext; // Main database structure
    let mut __slate_storage_420: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_420: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_420) as *mut *mut sqlite3; // Number of indices
    let mut __slate_storage_419: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_419: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_419) as *mut i32; // Cursor number of the first index
    let mut __slate_storage_418: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_418: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_418) as *mut i32; // VDBE cursor for the canonical data source
    let mut __slate_storage_417: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_417: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_417) as *mut i32; // Cursor number for the table
    let mut __slate_storage_416: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_416: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_416) as *mut i32; // For looping over indices of the table
    let mut __slate_storage_415: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_415: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_415) as *mut *mut Index; // Information about the WHERE clause
    let mut __slate_storage_414: std::mem::MaybeUninit<*mut WhereInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_414: *mut *mut WhereInfo =
        std::ptr::addr_of_mut!(__slate_storage_414) as *mut *mut WhereInfo; // Loop counter
    let mut __slate_storage_413: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_413: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_413) as *mut i32; // The table from which records will be deleted
    let mut __slate_storage_412: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_412: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_412) as *mut *mut Table; // The virtual database engine
    let mut __slate_storage_411: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_411: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_411) as *mut *mut Vdbe;
    unsafe {
        '__join_2: {
            std::ptr::write(__slate_slot_417, 0 as i32);
            std::ptr::write(__slate_slot_418, 0 as i32);
            std::ptr::write(__slate_slot_424, 0 as i32);
            std::ptr::write(__slate_slot_428, std::ptr::null_mut::<u8>());
            std::ptr::write(__slate_slot_430, 0 as i32);
            std::ptr::write(__slate_slot_431, (1 as i32) as i16);
            std::ptr::write(__slate_slot_434, 0 as i32);
            std::ptr::write(__slate_slot_435, 0 as i32);
            std::ptr::write(__slate_slot_436, 0 as i32);
            std::ptr::write(__slate_slot_437, 0 as i32);
            std::ptr::write(__slate_slot_438, 0 as i32);
            unsafe {
                memset(
                    std::ptr::addr_of_mut!(*__slate_slot_421) as *mut (),
                    0 as i32,
                    16 as u64,
                )
            };
            *__slate_slot_420 = unsafe { (*pParse).db };
            0 as i32;
            if (unsafe { (*pParse).nErr }) != (0 as i32) {
            } else {
                0 as i32;
                0 as i32;
                // Locate the table which we want to delete.  This table has to be
                // put in an SrcList structure because some of the subroutines we
                // will be calling are designed to work with multiple tables and expect
                // an SrcList* parameter instead of just a Table* parameter.
                *__slate_slot_412 = sqlite3SrcListLookup(pParse, pTabList);
                if *__slate_slot_412 == std::ptr::null_mut::<Table>() {
                } else {
                    // Figure out if we have any triggers and if the table being
                    // deleted from is a view
                    *__slate_slot_441 = unsafe {
                        sqlite3TriggersExist(
                            pParse,
                            *__slate_slot_412,
                            129 as i32,
                            std::ptr::null_mut::<ExprList>(),
                            std::ptr::null_mut::<i32>(),
                        )
                    };
                    *__slate_slot_440 = ((((unsafe { (*(*__slate_slot_412)).eTabType }) as u32)
                        as i32)
                        == (2 as i32)) as i32;
                    if *__slate_slot_441 != std::ptr::null_mut::<Trigger>() {
                        *__slate_slot_779 = true as bool;
                    } else {
                        *__slate_slot_779 = (unsafe {
                            sqlite3FkRequired(
                                pParse,
                                *__slate_slot_412,
                                std::ptr::null_mut::<i32>(),
                                0 as i32,
                            )
                        }) != (0 as i32);
                    }
                    *__slate_slot_439 = *__slate_slot_779 as i32;
                    // If pTab is really a view, make sure it has been initialized.
                    if (unsafe { sqlite3ViewGetColumnNames(pParse, *__slate_slot_412) })
                        != (0 as i32)
                    {
                    } else {
                        if sqlite3IsReadOnly(pParse, *__slate_slot_412, *__slate_slot_441)
                            != (0 as i32)
                        {
                        } else {
                            *__slate_slot_423 = unsafe {
                                sqlite3SchemaToIndex(*__slate_slot_420, unsafe {
                                    (*(*__slate_slot_412)).pSchema
                                })
                            };
                            0 as i32;
                            *__slate_slot_425 = unsafe {
                                sqlite3AuthCheck(
                                    pParse,
                                    9 as i32,
                                    (unsafe { (*(*__slate_slot_412)).zName }) as *const i8,
                                    std::ptr::null::<i8>(),
                                    (unsafe {
                                        (*unsafe {
                                            unsafe { (*(*__slate_slot_420)).aDb }
                                                .offset(*__slate_slot_423 as isize)
                                        })
                                        .zDbSName
                                    }) as *const i8,
                                )
                            };
                            0 as i32;
                            if *__slate_slot_425 == (1 as i32) {
                            } else {
                                0 as i32;
                                // Assign cursor numbers to the table and all its indices.
                                0 as i32;
                                std::ptr::write(__slate_slot_780, pParse);
                                std::ptr::write(__slate_slot_781, unsafe {
                                    (*(*__slate_slot_780)).nTab
                                });
                                std::ptr::write(__slate_slot_782, *__slate_slot_781 + (1 as i32));
                                unsafe {
                                    (*(*__slate_slot_780)).nTab = *__slate_slot_782;
                                }
                                std::ptr::write(__slate_slot_783, *__slate_slot_781);
                                unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem
                                        }
                                        .offset((0 as i32) as isize)
                                    })
                                    .iCursor = *__slate_slot_783;
                                }
                                *__slate_slot_416 = *__slate_slot_783;
                                *__slate_slot_419 = 0 as i32;
                                std::ptr::write(__slate_slot_784, unsafe {
                                    (*(*__slate_slot_412)).pIndex
                                });
                                *__slate_slot_415 = *__slate_slot_784;
                                loop {
                                    if *__slate_slot_415 != std::ptr::null_mut::<Index>() {
                                        std::ptr::write(__slate_slot_787, pParse);
                                        std::ptr::write(__slate_slot_788, unsafe {
                                            (*(*__slate_slot_787)).nTab
                                        });
                                        std::ptr::write(
                                            __slate_slot_789,
                                            *__slate_slot_788 + (1 as i32),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_787)).nTab = *__slate_slot_789;
                                        }
                                        *__slate_slot_415 = unsafe { (*(*__slate_slot_415)).pNext };
                                        std::ptr::write(__slate_slot_785, *__slate_slot_419);
                                        std::ptr::write(
                                            __slate_slot_786,
                                            *__slate_slot_785 + (1 as i32),
                                        );
                                        *__slate_slot_419 = *__slate_slot_786;
                                    } else {
                                        break;
                                    }
                                }
                                // Start the view context
                                if *__slate_slot_440 != (0 as i32) {
                                    unsafe {
                                        sqlite3AuthContextPush(
                                            pParse,
                                            std::ptr::addr_of_mut!(*__slate_slot_421),
                                            (unsafe { (*(*__slate_slot_412)).zName }) as *const i8,
                                        )
                                    };
                                }
                                // Begin generating code.
                                *__slate_slot_411 = unsafe { sqlite3GetVdbe(pParse) };
                                if *__slate_slot_411 == std::ptr::null_mut::<Vdbe>() {
                                } else {
                                    if (((unsafe { (*pParse).nested }) as u32) as i32) == (0 as i32)
                                    {
                                        unsafe { sqlite3VdbeCountChanges(*__slate_slot_411) };
                                    }
                                    unsafe {
                                        sqlite3BeginWriteOperation(
                                            pParse,
                                            *__slate_slot_439,
                                            *__slate_slot_423,
                                        )
                                    };
                                    // If we are trying to delete from a view, realize that view into
                                    // an ephemeral table.
                                    if *__slate_slot_440 != (0 as i32) {
                                        sqlite3MaterializeView(
                                            pParse,
                                            *__slate_slot_412,
                                            pWhere,
                                            pOrderBy,
                                            pLimit,
                                            *__slate_slot_416,
                                        );
                                        std::ptr::write(__slate_slot_790, *__slate_slot_416);
                                        *__slate_slot_418 = *__slate_slot_790;
                                        *__slate_slot_417 = *__slate_slot_790;
                                        pOrderBy = std::ptr::null_mut::<ExprList>();
                                        pLimit = std::ptr::null_mut::<Expr>();
                                    }
                                    // Resolve the column names in the WHERE clause.
                                    unsafe {
                                        memset(
                                            std::ptr::addr_of_mut!(*__slate_slot_422) as *mut (),
                                            0 as i32,
                                            56 as u64,
                                        )
                                    };
                                    (*__slate_slot_422).pParse = pParse;
                                    (*__slate_slot_422).pSrcList = pTabList;
                                    if (unsafe {
                                        sqlite3ResolveExprNames(
                                            std::ptr::addr_of_mut!(*__slate_slot_422),
                                            pWhere,
                                        )
                                    }) != (0 as i32)
                                    {
                                    } else {
                                        // Initialize the counter of the number of rows deleted, if
                                        // we are counting rows.
                                        if (unsafe { (*(*__slate_slot_420)).flags })
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
                                            std::ptr::write(__slate_slot_791, pParse);
                                            std::ptr::write(__slate_slot_792, unsafe {
                                                (*(*__slate_slot_791)).nMem
                                            });
                                            std::ptr::write(
                                                __slate_slot_793,
                                                *__slate_slot_792 + (1 as i32),
                                            );
                                            unsafe {
                                                (*(*__slate_slot_791)).nMem = *__slate_slot_793;
                                            }
                                            *__slate_slot_424 = *__slate_slot_793;
                                            unsafe {
                                                sqlite3VdbeAddOp2(
                                                    *__slate_slot_411,
                                                    73 as i32,
                                                    0 as i32,
                                                    *__slate_slot_424,
                                                )
                                            };
                                        }
                                        '__join_6: {
                                            // Special case: A DELETE without a WHERE clause deletes everything.
                                            // It is easier just to erase the whole table. Prior to version 3.6.5,
                                            // this optimization caused the row change count (the value returned by
                                            // API function sqlite3_count_changes) to be set incorrectly.
                                            //
                                            // The "rcauth==SQLITE_OK" terms is the
                                            // IMPLEMENTATION-OF: R-17228-37124 If the action code is SQLITE_DELETE and
                                            // the callback returns SQLITE_IGNORE then the DELETE operation proceeds but
                                            // the truncate optimization is disabled and all rows are deleted
                                            // individually.
                                            if *__slate_slot_425 == (0 as i32)
                                                && pWhere == std::ptr::null_mut::<Expr>()
                                                && !(*__slate_slot_439 != (0 as i32))
                                                && !((((unsafe { (*(*__slate_slot_412)).eTabType })
                                                    as u32)
                                                    as i32)
                                                    == (1 as i32))
                                            {
                                                0 as i32;
                                                unsafe {
                                                    sqlite3TableLock(
                                                        pParse,
                                                        *__slate_slot_423,
                                                        unsafe { (*(*__slate_slot_412)).tnum },
                                                        ((1 as i32) as i8) as u8,
                                                        (unsafe { (*(*__slate_slot_412)).zName })
                                                            as *const i8,
                                                    )
                                                };
                                                if (unsafe { (*(*__slate_slot_412)).tabFlags })
                                                    & ((128 as i32) as u32)
                                                    == ((0 as i32) as u32)
                                                {
                                                    unsafe {
                                                        sqlite3VdbeAddOp4(
                                                            *__slate_slot_411,
                                                            147 as i32,
                                                            (unsafe { (*(*__slate_slot_412)).tnum })
                                                                as i32,
                                                            *__slate_slot_423,
                                                            if *__slate_slot_424 != (0 as i32) {
                                                                *__slate_slot_424
                                                            } else {
                                                                -(1 as i32)
                                                            },
                                                            (unsafe {
                                                                (*(*__slate_slot_412)).zName
                                                            })
                                                                as *const i8,
                                                            -(1 as i32),
                                                        )
                                                    };
                                                }
                                                *__slate_slot_415 =
                                                    unsafe { (*(*__slate_slot_412)).pIndex };
                                                loop {
                                                    if *__slate_slot_415
                                                        != std::ptr::null_mut::<Index>()
                                                    {
                                                        0 as i32;
                                                        if ((unsafe {
                                                            (*(*__slate_slot_415))
                                                                .__slate_bits_0
                                                                .__get_idxType()
                                                        })
                                                            as i32)
                                                            == (2 as i32)
                                                            && !((unsafe {
                                                                (*(*__slate_slot_412)).tabFlags
                                                            }) & ((128 as i32) as u32)
                                                                == ((0 as i32) as u32))
                                                        {
                                                            unsafe {
                                                                sqlite3VdbeAddOp3(
                                                                    *__slate_slot_411,
                                                                    147 as i32,
                                                                    (unsafe {
                                                                        (*(*__slate_slot_415)).tnum
                                                                    })
                                                                        as i32,
                                                                    *__slate_slot_423,
                                                                    if *__slate_slot_424
                                                                        != (0 as i32)
                                                                    {
                                                                        *__slate_slot_424
                                                                    } else {
                                                                        -(1 as i32)
                                                                    },
                                                                )
                                                            };
                                                        } else {
                                                            unsafe {
                                                                sqlite3VdbeAddOp2(
                                                                    *__slate_slot_411,
                                                                    147 as i32,
                                                                    (unsafe {
                                                                        (*(*__slate_slot_415)).tnum
                                                                    })
                                                                        as i32,
                                                                    *__slate_slot_423,
                                                                )
                                                            };
                                                        }
                                                        *__slate_slot_415 =
                                                            unsafe { (*(*__slate_slot_415)).pNext };
                                                    } else {
                                                        break '__join_6;
                                                    }
                                                }
                                            } else {
                                                std::ptr::write(
                                                    __slate_slot_442,
                                                    (((4 as i32) | (16 as i32)) as i16) as u16,
                                                );
                                                if (*__slate_slot_422).ncFlags & (64 as i32)
                                                    != (0 as i32)
                                                {
                                                    *__slate_slot_439 = 1 as i32;
                                                }
                                                std::ptr::write(
                                                    __slate_slot_794,
                                                    *__slate_slot_442,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_795,
                                                    ((((*__slate_slot_794 as u32) as i32)
                                                        | if *__slate_slot_439 != (0 as i32) {
                                                            0 as i32
                                                        } else {
                                                            8 as i32
                                                        })
                                                        as i16)
                                                        as u16,
                                                );
                                                *__slate_slot_442 = *__slate_slot_795;
                                                if (unsafe { (*(*__slate_slot_412)).tabFlags })
                                                    & ((128 as i32) as u32)
                                                    == ((0 as i32) as u32)
                                                {
                                                    // For a rowid table, initialize the RowSet to an empty set
                                                    *__slate_slot_429 =
                                                        std::ptr::null_mut::<Index>();
                                                    0 as i32;
                                                    std::ptr::write(__slate_slot_796, pParse);
                                                    std::ptr::write(__slate_slot_797, unsafe {
                                                        (*(*__slate_slot_796)).nMem
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_798,
                                                        *__slate_slot_797 + (1 as i32),
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_796)).nMem =
                                                            *__slate_slot_798;
                                                    }
                                                    *__slate_slot_435 = *__slate_slot_798;
                                                    unsafe {
                                                        sqlite3VdbeAddOp2(
                                                            *__slate_slot_411,
                                                            77 as i32,
                                                            0 as i32,
                                                            *__slate_slot_435,
                                                        )
                                                    };
                                                } else {
                                                    // For a WITHOUT ROWID table, create an ephemeral table used to
                                                    // hold all primary keys for rows to be deleted.
                                                    *__slate_slot_429 = unsafe {
                                                        sqlite3PrimaryKeyIndex(*__slate_slot_412)
                                                    };
                                                    0 as i32;
                                                    *__slate_slot_431 =
                                                        (unsafe { (*(*__slate_slot_429)).nKeyCol })
                                                            as i16;
                                                    *__slate_slot_430 =
                                                        (unsafe { (*pParse).nMem }) + (1 as i32);
                                                    std::ptr::write(__slate_slot_799, pParse);
                                                    std::ptr::write(__slate_slot_800, unsafe {
                                                        (*(*__slate_slot_799)).nMem
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_801,
                                                        *__slate_slot_800
                                                            + (*__slate_slot_431 as i32),
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_799)).nMem =
                                                            *__slate_slot_801;
                                                    }
                                                    std::ptr::write(__slate_slot_802, pParse);
                                                    std::ptr::write(__slate_slot_803, unsafe {
                                                        (*(*__slate_slot_802)).nTab
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_804,
                                                        *__slate_slot_803 + (1 as i32),
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_802)).nTab =
                                                            *__slate_slot_804;
                                                    }
                                                    *__slate_slot_434 = *__slate_slot_803;
                                                    *__slate_slot_438 = unsafe {
                                                        sqlite3VdbeAddOp2(
                                                            *__slate_slot_411,
                                                            120 as i32,
                                                            *__slate_slot_434,
                                                            *__slate_slot_431 as i32,
                                                        )
                                                    };
                                                    unsafe {
                                                        sqlite3VdbeSetP4KeyInfo(
                                                            pParse,
                                                            *__slate_slot_429,
                                                        )
                                                    };
                                                }
                                                // Construct a query to find the rowid or primary key for every row
                                                // to be deleted, based on the WHERE clause. Set variable eOnePass
                                                // to indicate the strategy used to implement this delete:
                                                //
                                                //  ONEPASS_OFF:    Two-pass approach - use a FIFO for rowids/PK values.
                                                //  ONEPASS_SINGLE: One-pass approach - at most one row deleted.
                                                //  ONEPASS_MULTI:  One-pass approach - any number of rows may be deleted.
                                                *__slate_slot_414 = unsafe {
                                                    sqlite3WhereBegin(
                                                        pParse,
                                                        pTabList,
                                                        pWhere,
                                                        std::ptr::null_mut::<ExprList>(),
                                                        std::ptr::null_mut::<ExprList>(),
                                                        std::ptr::null_mut::<Select>(),
                                                        *__slate_slot_442,
                                                        *__slate_slot_416 + (1 as i32),
                                                    )
                                                };
                                                if *__slate_slot_414
                                                    == std::ptr::null_mut::<WhereInfo>()
                                                {
                                                    break '__join_2;
                                                } else {
                                                    *__slate_slot_426 = unsafe {
                                                        sqlite3WhereOkOnePass(
                                                            *__slate_slot_414,
                                                            (*__slate_slot_427).as_mut_ptr()
                                                                as *mut i32,
                                                        )
                                                    };
                                                    0 as i32;
                                                    0 as i32;
                                                    if *__slate_slot_426 != (1 as i32) {
                                                        unsafe { sqlite3MultiWrite(pParse) };
                                                    }
                                                    if (unsafe {
                                                        sqlite3WhereUsesDeferredSeek(
                                                            *__slate_slot_414,
                                                        )
                                                    }) != (0 as i32)
                                                    {
                                                        unsafe {
                                                            sqlite3VdbeAddOp1(
                                                                *__slate_slot_411,
                                                                145 as i32,
                                                                *__slate_slot_416,
                                                            )
                                                        };
                                                    }
                                                    // Keep track of the number of rows to be deleted
                                                    if *__slate_slot_424 != (0 as i32) {
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_411,
                                                                88 as i32,
                                                                *__slate_slot_424,
                                                                1 as i32,
                                                            )
                                                        };
                                                    }
                                                    // Extract the rowid or primary key for the current row
                                                    if *__slate_slot_429
                                                        != std::ptr::null_mut::<Index>()
                                                    {
                                                        *__slate_slot_413 = 0 as i32;
                                                        loop {
                                                            if *__slate_slot_413
                                                                < (*__slate_slot_431 as i32)
                                                            {
                                                                0 as i32;
                                                                unsafe {
                                                                    sqlite3ExprCodeGetColumnOfTable(
                                                                        *__slate_slot_411,
                                                                        *__slate_slot_412,
                                                                        *__slate_slot_416,
                                                                        (unsafe {
                                                                            *unsafe {
                                                                                unsafe { (*(*__slate_slot_429)).aiColumn }.offset(*__slate_slot_413 as isize)
                                                                            }
                                                                        })
                                                                            as i32,
                                                                        *__slate_slot_430
                                                                            + *__slate_slot_413,
                                                                    )
                                                                };
                                                                std::ptr::write(
                                                                    __slate_slot_805,
                                                                    *__slate_slot_413,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_806,
                                                                    *__slate_slot_805 + (1 as i32),
                                                                );
                                                                *__slate_slot_413 =
                                                                    *__slate_slot_806;
                                                            } else {
                                                                break;
                                                            }
                                                        }
                                                        *__slate_slot_432 = *__slate_slot_430;
                                                    } else {
                                                        std::ptr::write(__slate_slot_807, pParse);
                                                        std::ptr::write(__slate_slot_808, unsafe {
                                                            (*(*__slate_slot_807)).nMem
                                                        });
                                                        std::ptr::write(
                                                            __slate_slot_809,
                                                            *__slate_slot_808 + (1 as i32),
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_807)).nMem =
                                                                *__slate_slot_809;
                                                        }
                                                        *__slate_slot_432 = *__slate_slot_809;
                                                        unsafe {
                                                            sqlite3ExprCodeGetColumnOfTable(
                                                                *__slate_slot_411,
                                                                *__slate_slot_412,
                                                                *__slate_slot_416,
                                                                -(1 as i32),
                                                                *__slate_slot_432,
                                                            )
                                                        };
                                                    }
                                                    if *__slate_slot_426 != (0 as i32) {
                                                        // For ONEPASS, no need to store the rowid/primary-key. There is only
                                                        // one, so just keep it in its register(s) and fall through to the
                                                        // delete code.
                                                        *__slate_slot_433 = *__slate_slot_431; // OP_Found will use an unpacked key
                                                        *__slate_slot_428 = (unsafe {
                                                            sqlite3DbMallocRawNN(
                                                                *__slate_slot_420,
                                                                ((*__slate_slot_419 + (2 as i32))
                                                                    as i64)
                                                                    as u64,
                                                            )
                                                        })
                                                            as *mut u8;
                                                        if *__slate_slot_428
                                                            == std::ptr::null_mut::<u8>()
                                                        {
                                                            unsafe {
                                                                sqlite3WhereEnd(*__slate_slot_414)
                                                            };
                                                            break '__join_2;
                                                        } else {
                                                            unsafe {
                                                                memset(
                                                                    *__slate_slot_428 as *mut (),
                                                                    1 as i32,
                                                                    ((*__slate_slot_419
                                                                        + (1 as i32))
                                                                        as i64)
                                                                        as u64,
                                                                )
                                                            };
                                                            unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_428).offset(
                                                                        (*__slate_slot_419
                                                                            + (1 as i32))
                                                                            as isize,
                                                                    )
                                                                } = ((0 as i32) as i8) as u8;
                                                            }
                                                            if (unsafe {
                                                                *unsafe {
                                                                    ((*__slate_slot_427)
                                                                        .as_mut_ptr()
                                                                        as *mut i32)
                                                                        .offset((0 as i32) as isize)
                                                                }
                                                            }) >= (0 as i32)
                                                            {
                                                                unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_428).offset(((unsafe { *unsafe { ((*__slate_slot_427).as_mut_ptr() as *mut i32).offset((0 as i32) as isize) } }) - *__slate_slot_416) as isize)
                                                                    } = ((0 as i32) as i8) as u8;
                                                                }
                                                            }
                                                            if (unsafe {
                                                                *unsafe {
                                                                    ((*__slate_slot_427)
                                                                        .as_mut_ptr()
                                                                        as *mut i32)
                                                                        .offset((1 as i32) as isize)
                                                                }
                                                            }) >= (0 as i32)
                                                            {
                                                                unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_428).offset(((unsafe { *unsafe { ((*__slate_slot_427).as_mut_ptr() as *mut i32).offset((1 as i32) as isize) } }) - *__slate_slot_416) as isize)
                                                                    } = ((0 as i32) as i8) as u8;
                                                                }
                                                            }
                                                            if *__slate_slot_438 != (0 as i32) {
                                                                unsafe {
                                                                    sqlite3VdbeChangeToNoop(
                                                                        *__slate_slot_411,
                                                                        *__slate_slot_438,
                                                                    )
                                                                };
                                                            }
                                                            *__slate_slot_436 = unsafe {
                                                                sqlite3VdbeMakeLabel(pParse)
                                                            };
                                                        }
                                                    } else {
                                                        if *__slate_slot_429
                                                            != std::ptr::null_mut::<Index>()
                                                        {
                                                            std::ptr::write(
                                                                __slate_slot_810,
                                                                pParse,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_811,
                                                                unsafe {
                                                                    (*(*__slate_slot_810)).nMem
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_812,
                                                                *__slate_slot_811 + (1 as i32),
                                                            );
                                                            unsafe {
                                                                (*(*__slate_slot_810)).nMem =
                                                                    *__slate_slot_812;
                                                            }
                                                            *__slate_slot_432 = *__slate_slot_812;
                                                            *__slate_slot_433 = (0 as i32) as i16; // Zero tells OP_Found to use a composite key
                                                            unsafe {
                                                                sqlite3VdbeAddOp4(
                                                                    *__slate_slot_411,
                                                                    99 as i32,
                                                                    *__slate_slot_430,
                                                                    *__slate_slot_431 as i32,
                                                                    *__slate_slot_432,
                                                                    unsafe {
                                                                        sqlite3IndexAffinityStr(
                                                                            unsafe { (*pParse).db },
                                                                            *__slate_slot_429,
                                                                        )
                                                                    },
                                                                    *__slate_slot_431 as i32,
                                                                )
                                                            };
                                                            unsafe {
                                                                sqlite3VdbeAddOp4Int(
                                                                    *__slate_slot_411,
                                                                    140 as i32,
                                                                    *__slate_slot_434,
                                                                    *__slate_slot_432,
                                                                    *__slate_slot_430,
                                                                    *__slate_slot_431 as i32,
                                                                )
                                                            };
                                                        } else {
                                                            // Add the rowid of the row to be deleted to the RowSet
                                                            *__slate_slot_433 = (1 as i32) as i16; // OP_DeferredSeek always uses a single rowid
                                                            unsafe {
                                                                sqlite3VdbeAddOp2(
                                                                    *__slate_slot_411,
                                                                    158 as i32,
                                                                    *__slate_slot_435,
                                                                    *__slate_slot_432,
                                                                )
                                                            };
                                                        }
                                                        unsafe {
                                                            sqlite3WhereEnd(*__slate_slot_414)
                                                        };
                                                    }
                                                    // Unless this is a view, open cursors for the table we are
                                                    // deleting from and all its indices. If this is a view, then the
                                                    // only effect this statement has is to fire the INSTEAD OF
                                                    // triggers.
                                                    if !(*__slate_slot_440 != (0 as i32)) {
                                                        std::ptr::write(__slate_slot_443, 0 as i32);
                                                        if *__slate_slot_426 == (2 as i32) {
                                                            *__slate_slot_443 = unsafe {
                                                                sqlite3VdbeAddOp0(
                                                                    *__slate_slot_411,
                                                                    15 as i32,
                                                                )
                                                            };
                                                            {}
                                                        }
                                                        {}
                                                        unsafe {
                                                            sqlite3OpenTableAndIndices(
                                                                pParse,
                                                                *__slate_slot_412,
                                                                116 as i32,
                                                                ((8 as i32) as i8) as u8,
                                                                *__slate_slot_416,
                                                                *__slate_slot_428,
                                                                std::ptr::addr_of_mut!(
                                                                    *__slate_slot_417
                                                                ),
                                                                std::ptr::addr_of_mut!(
                                                                    *__slate_slot_418
                                                                ),
                                                            )
                                                        };
                                                        0 as i32;
                                                        0 as i32;
                                                        if *__slate_slot_426 == (2 as i32) {
                                                            unsafe {
                                                                sqlite3VdbeJumpHereOrPopInst(
                                                                    *__slate_slot_411,
                                                                    *__slate_slot_443,
                                                                )
                                                            };
                                                        }
                                                    }
                                                    // Set up a loop over the rowids/primary-keys that were found in the
                                                    // where-clause loop above.
                                                    if *__slate_slot_426 != (0 as i32) {
                                                        0 as i32; // OP_Found will use an unpacked key
                                                        if !((((unsafe {
                                                            (*(*__slate_slot_412)).eTabType
                                                        })
                                                            as u32)
                                                            as i32)
                                                            == (1 as i32))
                                                            && (unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_428).offset(
                                                                        (*__slate_slot_417
                                                                            - *__slate_slot_416)
                                                                            as isize,
                                                                    )
                                                                }
                                                            }) != (0 as u8)
                                                        {
                                                            0 as i32;
                                                            unsafe {
                                                                sqlite3VdbeAddOp4Int(
                                                                    *__slate_slot_411,
                                                                    28 as i32,
                                                                    *__slate_slot_417,
                                                                    *__slate_slot_436,
                                                                    *__slate_slot_432,
                                                                    *__slate_slot_433 as i32,
                                                                )
                                                            };
                                                            {}
                                                        }
                                                    } else {
                                                        if *__slate_slot_429
                                                            != std::ptr::null_mut::<Index>()
                                                        {
                                                            *__slate_slot_437 = unsafe {
                                                                sqlite3VdbeAddOp1(
                                                                    *__slate_slot_411,
                                                                    36 as i32,
                                                                    *__slate_slot_434,
                                                                )
                                                            };
                                                            {}
                                                            if (((unsafe {
                                                                (*(*__slate_slot_412)).eTabType
                                                            })
                                                                as u32)
                                                                as i32)
                                                                == (1 as i32)
                                                            {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp3(
                                                                        *__slate_slot_411,
                                                                        96 as i32,
                                                                        *__slate_slot_434,
                                                                        0 as i32,
                                                                        *__slate_slot_432,
                                                                    )
                                                                };
                                                            } else {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp2(
                                                                        *__slate_slot_411,
                                                                        136 as i32,
                                                                        *__slate_slot_434,
                                                                        *__slate_slot_432,
                                                                    )
                                                                };
                                                            }
                                                            0 as i32; // OP_Found will use a composite key
                                                        } else {
                                                            *__slate_slot_437 = unsafe {
                                                                sqlite3VdbeAddOp3(
                                                                    *__slate_slot_411,
                                                                    48 as i32,
                                                                    *__slate_slot_435,
                                                                    0 as i32,
                                                                    *__slate_slot_432,
                                                                )
                                                            };
                                                            {}
                                                            0 as i32;
                                                        }
                                                    }
                                                    // Delete the row
                                                    if (((unsafe {
                                                        (*(*__slate_slot_412)).eTabType
                                                    })
                                                        as u32)
                                                        as i32)
                                                        == (1 as i32)
                                                    {
                                                        std::ptr::write(
                                                            __slate_slot_444,
                                                            (unsafe {
                                                                sqlite3GetVTable(
                                                                    *__slate_slot_420,
                                                                    *__slate_slot_412,
                                                                )
                                                            })
                                                                as *const i8,
                                                        );
                                                        unsafe {
                                                            sqlite3VtabMakeWritable(
                                                                pParse,
                                                                *__slate_slot_412,
                                                            )
                                                        };
                                                        0 as i32;
                                                        unsafe { sqlite3MayAbort(pParse) };
                                                        if *__slate_slot_426 == (1 as i32) {
                                                            unsafe {
                                                                sqlite3VdbeAddOp1(
                                                                    *__slate_slot_411,
                                                                    124 as i32,
                                                                    *__slate_slot_416,
                                                                )
                                                            };
                                                            if (unsafe { (*pParse).pToplevel })
                                                                == std::ptr::null_mut::<Parse>()
                                                            {
                                                                unsafe {
                                                                    (*pParse).isMultiWrite =
                                                                        ((0 as i32) as i8) as u8;
                                                                }
                                                            }
                                                        }
                                                        unsafe {
                                                            sqlite3VdbeAddOp4(
                                                                *__slate_slot_411,
                                                                7 as i32,
                                                                0 as i32,
                                                                1 as i32,
                                                                *__slate_slot_432,
                                                                *__slate_slot_444,
                                                                -(12 as i32),
                                                            )
                                                        };
                                                        unsafe {
                                                            sqlite3VdbeChangeP5(
                                                                *__slate_slot_411,
                                                                ((2 as i32) as i16) as u16,
                                                            )
                                                        };
                                                    } else {
                                                        std::ptr::write(
                                                            __slate_slot_445,
                                                            ((((unsafe { (*pParse).nested }) as u32)
                                                                as i32)
                                                                == (0 as i32))
                                                                as i32,
                                                        );
                                                        sqlite3GenerateRowDelete(
                                                            pParse,
                                                            *__slate_slot_412,
                                                            *__slate_slot_441,
                                                            *__slate_slot_417,
                                                            *__slate_slot_418,
                                                            *__slate_slot_432,
                                                            *__slate_slot_433,
                                                            (*__slate_slot_445 as i8) as u8,
                                                            ((11 as i32) as i8) as u8,
                                                            (*__slate_slot_426 as i8) as u8,
                                                            unsafe {
                                                                *unsafe {
                                                                    ((*__slate_slot_427)
                                                                        .as_mut_ptr()
                                                                        as *mut i32)
                                                                        .offset((1 as i32) as isize)
                                                                }
                                                            },
                                                        );
                                                    }
                                                    // End of the loop over all rowids/primary-keys.
                                                    if *__slate_slot_426 != (0 as i32) {
                                                        unsafe {
                                                            sqlite3VdbeResolveLabel(
                                                                *__slate_slot_411,
                                                                *__slate_slot_436,
                                                            )
                                                        };
                                                        unsafe {
                                                            sqlite3WhereEnd(*__slate_slot_414)
                                                        };
                                                    } else {
                                                        if *__slate_slot_429
                                                            != std::ptr::null_mut::<Index>()
                                                        {
                                                            unsafe {
                                                                sqlite3VdbeAddOp2(
                                                                    *__slate_slot_411,
                                                                    40 as i32,
                                                                    *__slate_slot_434,
                                                                    *__slate_slot_437 + (1 as i32),
                                                                )
                                                            };
                                                            {}
                                                            unsafe {
                                                                sqlite3VdbeJumpHere(
                                                                    *__slate_slot_411,
                                                                    *__slate_slot_437,
                                                                )
                                                            };
                                                        } else {
                                                            unsafe {
                                                                sqlite3VdbeGoto(
                                                                    *__slate_slot_411,
                                                                    *__slate_slot_437,
                                                                )
                                                            };
                                                            unsafe {
                                                                sqlite3VdbeJumpHere(
                                                                    *__slate_slot_411,
                                                                    *__slate_slot_437,
                                                                )
                                                            };
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        // End non-truncate path
                                        // Update the sqlite_sequence table by storing the content of the
                                        // maximum rowid counter values recorded while inserting into
                                        // autoincrement tables.
                                        if (((unsafe { (*pParse).nested }) as u32) as i32)
                                            == (0 as i32)
                                            && (unsafe { (*pParse).pTriggerTab })
                                                == std::ptr::null_mut::<Table>()
                                        {
                                            unsafe { sqlite3AutoincrementEnd(pParse) };
                                        }
                                        // Return the number of rows that were deleted. If this routine is
                                        // generating code because of a call to sqlite3NestedParse(), do not
                                        // invoke the callback function.
                                        if *__slate_slot_424 != (0 as i32) {
                                            sqlite3CodeChangeCount(
                                                *__slate_slot_411,
                                                *__slate_slot_424,
                                                (b"rows deleted\0".as_ptr() as *mut i8)
                                                    as *const i8,
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
        unsafe { sqlite3AuthContextPop(std::ptr::addr_of_mut!(*__slate_slot_421)) };
        unsafe { sqlite3SrcListDelete(*__slate_slot_420, pTabList) };
        unsafe { sqlite3ExprDelete(*__slate_slot_420, pWhere) };
        if *__slate_slot_428 != std::ptr::null_mut::<u8>() {
            unsafe { sqlite3DbNNFreeNN(*__slate_slot_420, *__slate_slot_428 as *mut ()) };
        }
        return;
    }
}

// Make sure "isView" and other macros defined above are undefined. Otherwise
// they may interfere with compilation of other functions in this file
// (or in another file, if this file becomes part of the amalgamation).
/// This routine generates VDBE code that causes a single row of a
/// single table to be deleted.  Both the original table entry and
/// all indices are removed.
///
/// Preconditions:
///
///   1.  iDataCur is an open cursor on the btree that is the canonical data
///       store for the table.  (This will be either the table itself,
///       in the case of a rowid table, or the PRIMARY KEY index in the case
///       of a WITHOUT ROWID table.)
///
///   2.  Read/write cursors for all indices of pTab must be open as
///       cursor number iIdxCur+i for the i-th index.
///
///   3.  The primary key for the row to be deleted must be stored in a
///       sequence of nPk memory cells starting at iPk.  If nPk==0 that means
///       that a search record formed from OP_MakeRecord is contained in the
///       single memory location iPk.
///
/// eMode:
///   Parameter eMode may be passed either ONEPASS_OFF (0), ONEPASS_SINGLE, or
///   ONEPASS_MULTI.  If eMode is not ONEPASS_OFF, then the cursor
///   iDataCur already points to the row to delete. If eMode is ONEPASS_OFF
///   then this function must seek iDataCur to the entry identified by iPk
///   and nPk before reading from it.
///
///   If eMode is ONEPASS_MULTI, then this call is being made as part
///   of a ONEPASS delete that affects multiple rows. In this case, if
///   iIdxNoSeek is a valid cursor number (>=0) and is not the same as
///   iDataCur, then its position should be preserved following the delete
///   operation. Or, if iIdxNoSeek is not a valid cursor number, the
///   position of iDataCur should be preserved instead.
///
/// iIdxNoSeek:
///   If iIdxNoSeek is a valid cursor number (>=0) not equal to iDataCur,
///   then it identifies an index cursor (from within array of cursors
///   starting at iIdxCur) that already points to the index entry to be deleted.
///   Except, this optimization is disabled if there are BEFORE triggers since
///   the trigger body might have moved the cursor.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pTab` - Table containing the row to be deleted
/// * `pTrigger` - List of triggers to (potentially) fire
/// * `iDataCur` - Cursor from which column data is extracted
/// * `iIdxCur` - First index cursor
/// * `iPk` - First memory cell containing the PRIMARY KEY
/// * `nPk` - Number of PRIMARY KEY memory cells
/// * `count` - If non-zero, increment the row change counter
/// * `onconf` - Default ON CONFLICT policy for triggers
/// * `eMode` - ONEPASS_OFF, _SINGLE, or _MULTI.  See above
/// * `iIdxNoSeek` - Cursor number of cursor that does not need seeking
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GenerateRowDelete(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pTrigger: *mut Trigger,
    mut iDataCur: i32,
    mut iIdxCur: i32,
    mut iPk: i32,
    mut nPk: i16,
    mut count: u8,
    mut onconf: u8,
    mut eMode: u8,
    mut iIdxNoSeek: i32,
) {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe }; // Vdbe
    let mut iOld: i32 = 0 as i32; // First register in OLD.* array
    let mut iLabel: i32 = 0 as i32; // Label resolved to end of generated code
    let mut opSeek: u8 = 0 as u8; // Seek opcode
    // Vdbe is guaranteed to have been allocated by this stage.
    0 as i32;
    {}
    // Seek cursor iCur to the row to delete. If this row no longer exists
    // (this can happen if a trigger program has already deleted it), do
    // not attempt to delete it or fire any DELETE triggers.
    iLabel = unsafe { sqlite3VdbeMakeLabel(pParse) };
    opSeek = ((if (unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
        31 as i32
    } else {
        28 as i32
    }) as i8) as u8;
    if ((eMode as u32) as i32) == (0 as i32) {
        unsafe {
            sqlite3VdbeAddOp4Int(v, (opSeek as u32) as i32, iDataCur, iLabel, iPk, nPk as i32)
        };
        {}
        {}
    }
    // If there are any triggers to fire, allocate a range of registers to
    // use for the old.* references in the triggers.
    if (unsafe { sqlite3FkRequired(pParse, pTab, std::ptr::null_mut::<i32>(), 0 as i32) })
        != (0 as i32)
        || pTrigger != std::ptr::null_mut::<Trigger>()
    {
        let mut mask: u32 = 0 as u32; // Mask of OLD.* columns in use
        let mut iCol: i32 = 0 as i32; // Iterator used while populating OLD.*
        let mut addrStart: i32 = 0 as i32; // Start of BEFORE trigger programs
        // TODO: Could use temporary registers here. Also could attempt to
        // avoid copying the contents of the rowid register.
        mask = unsafe {
            sqlite3TriggerColmask(
                pParse,
                pTrigger,
                std::ptr::null_mut::<ExprList>(),
                0 as i32,
                (1 as i32) | (2 as i32),
                pTab,
                (onconf as u32) as i32,
            )
        };
        let __v813: u32 = mask;
        let __v814: u32 = __v813 | unsafe { sqlite3FkOldmask(pParse, pTab) };
        mask = __v814;
        iOld = (unsafe { (*pParse).nMem }) + (1 as i32);
        let __v815: *mut Parse = pParse;
        let __v816: i32 = unsafe { (*__v815).nMem };
        let __v817: i32 = __v816 + ((1 as i32) + ((unsafe { (*pTab).nCol }) as i32));
        unsafe {
            (*__v815).nMem = __v817;
        }
        // Populate the OLD.* pseudo-table register array. These values will be
        // used by any BEFORE and AFTER triggers that exist.
        unsafe { sqlite3VdbeAddOp2(v, 82 as i32, iPk, iOld) };
        iCol = 0 as i32;
        '__slate_break_771: loop {
            if !(iCol < ((unsafe { (*pTab).nCol }) as i32)) {
                break;
            }
            {}
            {}
            if mask == (4294967295 as u32)
                || iCol <= (31 as i32) && mask & ((1 as i32) as u32) << iCol != ((0 as i32) as u32)
            {
                let mut kk: i32 =
                    (unsafe { sqlite3TableColumnToStorage(pTab, iCol as i16) }) as i32;
                unsafe {
                    sqlite3ExprCodeGetColumnOfTable(v, pTab, iDataCur, iCol, iOld + kk + (1 as i32))
                };
            }
            let __v818: i32 = iCol;
            let __v819: i32 = __v818 + (1 as i32);
            iCol = __v819;
        }
        // Invoke BEFORE DELETE trigger programs.
        addrStart = unsafe { sqlite3VdbeCurrentAddr(v) };
        unsafe {
            sqlite3CodeRowTrigger(
                pParse,
                pTrigger,
                129 as i32,
                std::ptr::null_mut::<ExprList>(),
                1 as i32,
                pTab,
                iOld,
                (onconf as u32) as i32,
                iLabel,
            )
        };
        // If any BEFORE triggers were coded, then seek the cursor to the
        // row to be deleted again. It may be that the BEFORE triggers moved
        // the cursor or already deleted the row that the cursor was
        // pointing to.
        //
        // Also disable the iIdxNoSeek optimization since the BEFORE trigger
        // may have moved that cursor.
        if addrStart < unsafe { sqlite3VdbeCurrentAddr(v) } {
            unsafe {
                sqlite3VdbeAddOp4Int(v, (opSeek as u32) as i32, iDataCur, iLabel, iPk, nPk as i32)
            };
            {}
            {}
            {}
            iIdxNoSeek = -(1 as i32);
        }
        // Do FK processing. This call checks that any FK constraints that
        // refer to this table (i.e. constraints attached to other tables)
        // are not violated by deleting this row.
        unsafe {
            sqlite3FkCheck(
                pParse,
                pTab,
                iOld,
                0 as i32,
                std::ptr::null_mut::<i32>(),
                0 as i32,
            )
        };
    }
    // Delete the index and table entries. Skip this step if pTab is really
    // a view (in which case the only effect of the DELETE statement is to
    // fire the INSTEAD OF triggers).
    //
    // If variable 'count' is non-zero, then this OP_Delete instruction should
    // invoke the update-hook. The pre-update-hook, on the other hand should
    // be invoked unless table pTab is a system table. The difference is that
    // the update-hook is not invoked for rows removed by REPLACE, but the
    // pre-update-hook is.
    if !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (2 as i32)) {
        let mut p5: u8 = ((0 as i32) as i8) as u8;
        sqlite3GenerateRowIndexDelete(
            pParse,
            pTab,
            iDataCur,
            iIdxCur,
            std::ptr::null_mut::<i32>(),
            iIdxNoSeek,
        );
        unsafe {
            sqlite3VdbeAddOp2(
                v,
                132 as i32,
                iDataCur,
                if count != (0 as u8) {
                    1 as i32
                } else {
                    0 as i32
                },
            )
        };
        let __v820: bool;
        if (((unsafe { (*pParse).nested }) as u32) as i32) == (0 as i32) {
            __v820 = true as bool;
        } else {
            __v820 = (0 as i32)
                == unsafe {
                    sqlite3_stricmp(
                        (unsafe { (*pTab).zName }) as *const i8,
                        (b"sqlite_stat1\0".as_ptr() as *mut i8) as *const i8,
                    )
                };
        }
        if __v820 {
            unsafe { sqlite3VdbeAppendP4(v, (pTab as *mut i8) as *mut (), -(5 as i32)) };
        }
        if ((eMode as u32) as i32) != (0 as i32) {
            unsafe { sqlite3VdbeChangeP5(v, ((4 as i32) as i16) as u16) };
        }
        if iIdxNoSeek >= (0 as i32) && iIdxNoSeek != iDataCur {
            unsafe { sqlite3VdbeAddOp1(v, 132 as i32, iIdxNoSeek) };
        }
        if ((eMode as u32) as i32) == (2 as i32) {
            let __v821: u8 = p5;
            let __v822: u8 = ((((__v821 as u32) as i32) | (2 as i32)) as i8) as u8;
            p5 = __v822;
        }
        unsafe { sqlite3VdbeChangeP5(v, p5 as u16) };
    }
    // Do any ON CASCADE, SET NULL or SET DEFAULT operations required to
    // handle rows (possibly in other tables) that refer via a foreign key
    // to the row just deleted.
    unsafe {
        sqlite3FkActions(
            pParse,
            pTab,
            std::ptr::null_mut::<ExprList>(),
            iOld,
            std::ptr::null_mut::<i32>(),
            0 as i32,
        )
    };
    // Invoke AFTER DELETE trigger programs.
    if pTrigger != std::ptr::null_mut::<Trigger>() {
        unsafe {
            sqlite3CodeRowTrigger(
                pParse,
                pTrigger,
                129 as i32,
                std::ptr::null_mut::<ExprList>(),
                2 as i32,
                pTab,
                iOld,
                (onconf as u32) as i32,
                iLabel,
            )
        };
    }
    // Jump here if the row had already been deleted before any BEFORE
    // trigger programs were invoked. Or if a trigger program throws a
    // RAISE(IGNORE) exception.
    unsafe { sqlite3VdbeResolveLabel(v, iLabel) };
    {}
}

/// This routine generates VDBE code that causes the deletion of all
/// index entries associated with a single row of a single table, pTab
///
/// Preconditions:
///
///   1.  A read/write cursor "iDataCur" must be open on the canonical storage
///       btree for the table pTab.  (This will be either the table itself
///       for rowid tables or to the primary key index for WITHOUT ROWID
///       tables.)
///
///   2.  Read/write cursors for all indices of pTab must be open as
///       cursor number iIdxCur+i for the i-th index.  (The pTab->pIndex
///       index is the 0-th index.)
///
///   3.  The "iDataCur" cursor must be already be positioned on the row
///       that is to be deleted.
///
/// # Arguments
///
/// * `pParse` - Parsing and code generating context
/// * `pTab` - Table containing the row to be deleted
/// * `iDataCur` - Cursor of table holding data.
/// * `iIdxCur` - First index cursor
/// * `aRegIdx` - Only delete if aRegIdx!=0 && aRegIdx[i]>0
/// * `iIdxNoSeek` - Do not delete from this cursor
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GenerateRowIndexDelete(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut iDataCur: i32,
    mut iIdxCur: i32,
    mut aRegIdx: *mut i32,
    mut iIdxNoSeek: i32,
) {
    let mut i: i32 = 0 as i32; // Index loop counter
    let mut r1: i32 = -(1 as i32); // Register holding an index key
    let mut iPartIdxLabel: i32 = 0 as i32; // Jump destination for skipping partial index entries
    let mut pIdx: *mut Index = unsafe { std::mem::zeroed() }; // Current index
    let mut pPrior: *mut Index = std::ptr::null_mut::<Index>(); // Prior index
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() }; // The prepared statement under construction
    let mut pPk: *mut Index = unsafe { std::mem::zeroed() }; // PRIMARY KEY index, or NULL for rowid tables
    v = unsafe { (*pParse).pVdbe };
    let __v823: *mut Index;
    if (unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
        __v823 = std::ptr::null_mut::<Index>();
    } else {
        __v823 = unsafe { sqlite3PrimaryKeyIndex(pTab) };
    }
    pPk = __v823;
    i = 0 as i32;
    let __v824: *mut Index = unsafe { (*pTab).pIndex };
    pIdx = __v824;
    '__slate_break_773: loop {
        if !(pIdx != std::ptr::null_mut::<Index>()) {
            break;
        }
        let mut p3: i32 = 0 as i32;
        0 as i32;
        if aRegIdx != std::ptr::null_mut::<i32>()
            && (unsafe { *unsafe { aRegIdx.offset(i as isize) } }) == (0 as i32)
        {
        } else {
            if pIdx == pPk {
            } else {
                if iIdxCur + i == iIdxNoSeek {
                } else {
                    {}
                    r1 = sqlite3GenerateIndexKey(
                        pParse,
                        pIdx,
                        iDataCur,
                        0 as i32,
                        1 as i32,
                        std::ptr::addr_of_mut!(iPartIdxLabel),
                        pPrior,
                        r1,
                    );
                    if ((unsafe { (*pIdx).__slate_bits_0.__get_bHasExpr() }) as i32) != (0 as i32)
                        && aRegIdx != std::ptr::null_mut::<i32>()
                    {
                        p3 = unsafe { *unsafe { aRegIdx.offset(i as isize) } };
                    }
                    unsafe { sqlite3VdbeAddOp3(v, 142 as i32, iIdxCur + i, r1, p3) };
                    unsafe { sqlite3VdbeChangeP4(v, -(1 as i32), pIdx as *const i8, -(6 as i32)) };
                    unsafe {
                        sqlite3VdbeChangeP5(
                            v,
                            ((if ((unsafe { (*pIdx).__slate_bits_0.__get_uniqNotNull() }) as i32)
                                != (0 as i32)
                            {
                                ((unsafe { (*pIdx).nKeyCol }) as u32) as i32
                            } else {
                                ((unsafe { (*pIdx).nColumn }) as u32) as i32
                            }) as i16) as u16,
                        )
                    };
                    sqlite3ResolvePartIdxLabel(pParse, iPartIdxLabel);
                    pPrior = pIdx;
                }
            }
        }
        let __v825: i32 = i;
        let __v826: i32 = __v825 + (1 as i32);
        i = __v826;
        let __v827: *mut Index = unsafe { (*pIdx).pNext };
        pIdx = __v827;
    }
}

/// Generate code that will assemble an index key and stores it in register
/// regOut.  The key with be for index pIdx which is an index on pTab.
/// iCur is the index of a cursor open on the pTab table and pointing to
/// the entry that needs indexing.  If pTab is a WITHOUT ROWID table, then
/// iCur must be the cursor of the PRIMARY KEY index.
///
/// Return a register number which is the first in a block of
/// registers that holds the elements of the index key.  The
/// block of registers has already been deallocated by the time
/// this routine returns.
///
/// If *piPartIdxLabel is not NULL, fill it in with a label and jump
/// to that label if pIdx is a partial index that should be skipped.
/// The label should be resolved using sqlite3ResolvePartIdxLabel().
/// A partial index should be skipped if its WHERE clause evaluates
/// to false or null.  If pIdx is not a partial index, *piPartIdxLabel
/// will be set to zero which is an empty label that is ignored by
/// sqlite3ResolvePartIdxLabel().
///
/// The pPrior and regPrior parameters are used to implement a cache to
/// avoid unnecessary register loads.  If pPrior is not NULL, then it is
/// a pointer to a different index for which an index key has just been
/// computed into register regPrior.  If the current pIdx index is generating
/// its key into the same sequence of registers and if pPrior and pIdx share
/// a column in common, then the register corresponding to that column already
/// holds the correct value and the loading of that register is skipped.
/// This optimization is helpful when doing a DELETE or an INTEGRITY_CHECK
/// on a table with multiple indices, and especially with the ROWID or
/// PRIMARY KEY columns of the index.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pIdx` - The index for which to generate a key
/// * `iDataCur` - Cursor number from which to take column data
/// * `regOut` - Put the new key into this register if not 0
/// * `prefixOnly` - Compute only a unique prefix of the key
/// * `piPartIdxLabel` - OUT: Jump to this label to skip partial index
/// * `pPrior` - Previously generated index key
/// * `regPrior` - Register holding previous generated key
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GenerateIndexKey(
    mut pParse: *mut Parse,
    mut pIdx: *mut Index,
    mut iDataCur: i32,
    mut regOut: i32,
    mut prefixOnly: i32,
    mut piPartIdxLabel: *mut i32,
    mut pPrior: *mut Index,
    mut regPrior: i32,
) -> i32 {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut j: i32 = 0 as i32;
    let mut regBase: i32 = 0 as i32;
    let mut nCol: i32 = 0 as i32;
    if piPartIdxLabel != std::ptr::null_mut::<i32>() {
        if (unsafe { (*pIdx).pPartIdxWhere }) != std::ptr::null_mut::<Expr>() {
            unsafe {
                *piPartIdxLabel = unsafe { sqlite3VdbeMakeLabel(pParse) };
            }
            unsafe {
                (*pParse).iSelfTab = iDataCur + (1 as i32);
            }
            unsafe {
                sqlite3ExprIfFalseDup(
                    pParse,
                    unsafe { (*pIdx).pPartIdxWhere },
                    unsafe { *piPartIdxLabel },
                    16 as i32,
                )
            };
            unsafe {
                (*pParse).iSelfTab = 0 as i32;
            }
            pPrior = std::ptr::null_mut::<Index>();
        // Ticket a9efb42811fa41ee 2019-11-02;
        // pPartIdxWhere may have corrupted regPrior registers
        } else {
            unsafe {
                *piPartIdxLabel = 0 as i32;
            }
        }
    }
    nCol = if prefixOnly != (0 as i32)
        && ((unsafe { (*pIdx).__slate_bits_0.__get_uniqNotNull() }) as i32) != (0 as i32)
    {
        ((unsafe { (*pIdx).nKeyCol }) as u32) as i32
    } else {
        ((unsafe { (*pIdx).nColumn }) as u32) as i32
    };
    regBase = unsafe { sqlite3GetTempRange(pParse, nCol) };
    if pPrior != std::ptr::null_mut::<Index>()
        && (regBase != regPrior
            || (unsafe { (*pPrior).pPartIdxWhere }) != std::ptr::null_mut::<Expr>())
    {
        pPrior = std::ptr::null_mut::<Index>();
    }
    j = 0 as i32;
    '__slate_break_774: loop {
        if !(j < nCol) {
            break;
        }
        if pPrior != std::ptr::null_mut::<Index>()
            && ((unsafe { *unsafe { unsafe { (*pPrior).aiColumn }.offset(j as isize) } }) as i32)
                == ((unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(j as isize) } }) as i32)
            && ((unsafe { *unsafe { unsafe { (*pPrior).aiColumn }.offset(j as isize) } }) as i32)
                != -(2 as i32)
        {
            // This column was already computed by the previous index
        } else {
            unsafe { sqlite3ExprCodeLoadIndexColumn(pParse, pIdx, iDataCur, j, regBase + j) };
            if ((unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(j as isize) } }) as i32)
                >= (0 as i32)
            {
                // If the column affinity is REAL but the number is an integer, then it
                // might be stored in the table as an integer (using a compact
                // representation) then converted to REAL by an OP_RealAffinity opcode.
                // But we are getting ready to store this value back into an index, where
                // it should be converted by to INTEGER again.  So omit the
                // OP_RealAffinity opcode if it is present
                unsafe { sqlite3VdbeDeletePriorOpcode(v, ((89 as i32) as i8) as u8) };
            }
        }
        let __v828: i32 = j;
        let __v829: i32 = __v828 + (1 as i32);
        j = __v829;
    }
    if regOut != (0 as i32) {
        unsafe { sqlite3VdbeAddOp3(v, 99 as i32, regBase, nCol, regOut) };
    }
    unsafe { sqlite3ReleaseTempRange(pParse, regBase, nCol) };
    return regBase;
}

/// If a prior call to sqlite3GenerateIndexKey() generated a jump-over label
/// because it was a partial index, then this routine should be called to
/// resolve that label.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ResolvePartIdxLabel(mut pParse: *mut Parse, mut iLabel: i32) {
    if iLabel != (0 as i32) {
        unsafe { sqlite3VdbeResolveLabel(unsafe { (*pParse).pVdbe }, iLabel) };
    }
}
