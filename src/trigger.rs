//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This file contains the implementation for TRIGGERs
unsafe extern "C" {
    static mut sqlite3CtypeMap: [u8; 0];
    fn sqlite3_strnicmp(__v678: *const i8, __v679: *const i8, __v680: i32) -> i32;
    fn sqlite3HashInsert(__v681: *mut Hash, pKey: *const i8, pData: *mut ()) -> *mut ();
    fn sqlite3HashFind(__v684: *const Hash, pKey: *const i8) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3VdbeAddOp0(__v689: *mut Vdbe, __v690: i32) -> i32;
    fn sqlite3VdbeAddOp1(__v691: *mut Vdbe, __v692: i32, __v693: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v694: *mut Vdbe, __v695: i32, __v696: i32, __v697: i32) -> i32;
    fn sqlite3VdbeAddOp3(
        __v698: *mut Vdbe,
        __v699: i32,
        __v700: i32,
        __v701: i32,
        __v702: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4(
        __v703: *mut Vdbe,
        __v704: i32,
        __v705: i32,
        __v706: i32,
        __v707: i32,
        zP4: *const i8,
        __v709: i32,
    ) -> i32;
    fn sqlite3VdbeAddParseSchemaOp(__v710: *mut Vdbe, __v711: i32, __v712: *mut i8, __v713: u16);
    fn sqlite3VdbeChangeP5(__v714: *mut Vdbe, P5: u16);
    fn sqlite3VdbeChangeP4(__v716: *mut Vdbe, addr: i32, zP4: *const i8, N: i32);
    fn sqlite3VdbeMakeLabel(__v720: *mut Parse) -> i32;
    fn sqlite3VdbeDelete(__v721: *mut Vdbe);
    fn sqlite3VdbeResolveLabel(__v722: *mut Vdbe, __v723: i32);
    fn sqlite3VdbeTakeOpArray(__v724: *mut Vdbe, __v725: *mut i32, __v726: *mut i32)
    -> *mut VdbeOp;
    fn sqlite3VdbeLinkSubProgram(__v727: *mut Vdbe, __v728: *mut SubProgram);
    fn sqlite3VdbeComment(__v729: *mut Vdbe, __v730: *const i8, ...);
    fn sqlite3WalkExprList(__v731: *mut Walker, __v732: *mut ExprList) -> i32;
    fn sqlite3ExprWalkNoop(__v733: *mut Walker, __v734: *mut Expr) -> i32;
    fn sqlite3SelectWalkNoop(__v735: *mut Walker, __v736: *mut Select) -> i32;
    fn sqlite3StrICmp(__v737: *const i8, __v738: *const i8) -> i32;
    fn sqlite3DbMallocZero(__v739: *mut sqlite3, __v740: u64) -> *mut ();
    fn sqlite3DbStrDup(__v741: *mut sqlite3, __v742: *const i8) -> *mut i8;
    fn sqlite3DbStrNDup(__v743: *mut sqlite3, __v744: *const i8, __v745: u64) -> *mut i8;
    fn sqlite3DbSpanDup(__v746: *mut sqlite3, __v747: *const i8, __v748: *const i8) -> *mut i8;
    fn sqlite3DbFree(__v749: *mut sqlite3, __v750: *mut ());
    fn sqlite3MPrintf(__v751: *mut sqlite3, __v752: *const i8, ...) -> *mut i8;
    fn sqlite3ErrorMsg(__v753: *mut Parse, __v754: *const i8, ...);
    fn sqlite3TokenInit(__v755: *mut Token, __v756: *mut i8);
    fn sqlite3Expr(__v757: *mut sqlite3, __v758: i32, __v759: *const i8) -> *mut Expr;
    fn sqlite3ExprDelete(__v760: *mut sqlite3, __v761: *mut Expr);
    fn sqlite3ExprListAppend(
        __v762: *mut Parse,
        __v763: *mut ExprList,
        __v764: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListDelete(__v765: *mut sqlite3, __v766: *mut ExprList);
    fn sqlite3GenerateColumnNames(pParse: *mut Parse, pSelect: *mut Select);
    fn sqlite3Insert(
        __v769: *mut Parse,
        __v770: *mut SrcList,
        __v771: *mut Select,
        __v772: *mut IdList,
        __v773: i32,
        __v774: *mut Upsert,
    );
    fn sqlite3IdListIndex(__v775: *mut IdList, __v776: *const i8) -> i32;
    fn sqlite3SrcListAppendList(
        pParse: *mut Parse,
        p1: *mut SrcList,
        p2: *mut SrcList,
    ) -> *mut SrcList;
    fn sqlite3SrcListAppendFromTerm(
        __v780: *mut Parse,
        __v781: *mut SrcList,
        __v782: *mut Token,
        __v783: *mut Token,
        __v784: *mut Token,
        __v785: *mut Select,
        __v786: *mut OnOrUsing,
    ) -> *mut SrcList;
    fn sqlite3IdListDelete(__v787: *mut sqlite3, __v788: *mut IdList);
    fn sqlite3SrcListDelete(__v789: *mut sqlite3, __v790: *mut SrcList);
    fn sqlite3Select(__v791: *mut Parse, __v792: *mut Select, __v793: *mut SelectDest) -> i32;
    fn sqlite3SelectNew(
        __v794: *mut Parse,
        __v795: *mut ExprList,
        __v796: *mut SrcList,
        __v797: *mut Expr,
        __v798: *mut ExprList,
        __v799: *mut Expr,
        __v800: *mut ExprList,
        __v801: u32,
        __v802: *mut Expr,
    ) -> *mut Select;
    fn sqlite3SelectDelete(__v803: *mut sqlite3, __v804: *mut Select);
    fn sqlite3SrcListLookup(__v805: *mut Parse, __v806: *mut SrcList) -> *mut Table;
    fn sqlite3DeleteFrom(
        __v807: *mut Parse,
        __v808: *mut SrcList,
        __v809: *mut Expr,
        __v810: *mut ExprList,
        __v811: *mut Expr,
    );
    fn sqlite3Update(
        __v812: *mut Parse,
        __v813: *mut SrcList,
        __v814: *mut ExprList,
        __v815: *mut Expr,
        __v816: i32,
        __v817: *mut ExprList,
        __v818: *mut Expr,
        __v819: *mut Upsert,
    );
    fn sqlite3ExprCodeFactorable(__v820: *mut Parse, __v821: *mut Expr, __v822: i32);
    fn sqlite3ExprIfFalse(__v823: *mut Parse, __v824: *mut Expr, __v825: i32, __v826: i32);
    fn sqlite3NameFromToken(__v827: *mut sqlite3, __v828: *const Token) -> *mut i8;
    fn sqlite3GetVdbe(__v829: *mut Parse) -> *mut Vdbe;
    fn sqlite3CodeVerifySchema(__v830: *mut Parse, __v831: i32);
    fn sqlite3CodeVerifyNamedSchema(__v832: *mut Parse, zDb: *const i8);
    fn sqlite3BeginWriteOperation(__v834: *mut Parse, __v835: i32, __v836: i32);
    fn sqlite3ExprDup(__v837: *mut sqlite3, __v838: *const Expr, __v839: i32) -> *mut Expr;
    fn sqlite3ExprListDup(
        __v840: *mut sqlite3,
        __v841: *const ExprList,
        __v842: i32,
    ) -> *mut ExprList;
    fn sqlite3SrcListDup(__v843: *mut sqlite3, __v844: *const SrcList, __v845: i32)
    -> *mut SrcList;
    fn sqlite3IdListDup(__v846: *mut sqlite3, __v847: *const IdList) -> *mut IdList;
    fn sqlite3SelectDup(__v848: *mut sqlite3, __v849: *const Select, __v850: i32) -> *mut Select;
    fn sqlite3ChangeCookie(__v851: *mut Parse, __v852: i32);
    fn sqlite3AuthCheck(
        __v932: *mut Parse,
        __v933: i32,
        __v934: *const i8,
        __v935: *const i8,
        __v936: *const i8,
    ) -> i32;
    fn sqlite3DbIsNamed(db: *mut sqlite3, iDb: i32, zName: *const i8) -> i32;
    fn sqlite3FixInit(
        __v940: *mut DbFixer,
        __v941: *mut Parse,
        __v942: i32,
        __v943: *const i8,
        __v944: *const Token,
    );
    fn sqlite3FixSrcList(__v945: *mut DbFixer, __v946: *mut SrcList) -> i32;
    fn sqlite3FixExpr(__v947: *mut DbFixer, __v948: *mut Expr) -> i32;
    fn sqlite3FixTriggerStep(__v949: *mut DbFixer, __v950: *mut TriggerStep) -> i32;
    fn sqlite3ExprAffinity(pExpr: *const Expr) -> i8;
    fn sqlite3TwoPartName(
        __v952: *mut Parse,
        __v953: *mut Token,
        __v954: *mut Token,
        __v955: *mut *mut Token,
    ) -> i32;
    fn sqlite3ReadSchema(pParse: *mut Parse) -> i32;
    fn sqlite3CheckObjectName(
        __v957: *mut Parse,
        __v958: *const i8,
        __v959: *const i8,
        __v960: *const i8,
    ) -> i32;
    fn sqlite3NestedParse(__v961: *mut Parse, __v962: *const i8, ...);
    fn sqlite3SelectPrep(__v963: *mut Parse, __v964: *mut Select, __v965: *mut NameContext);
    fn sqlite3ResolveExprNames(__v966: *mut NameContext, __v967: *mut Expr) -> i32;
    fn sqlite3ResolveExprListNames(__v968: *mut NameContext, __v969: *mut ExprList) -> i32;
    fn sqlite3RenameTokenRemap(__v970: *mut Parse, pTo: *const (), pFrom: *const ());
    fn sqlite3SchemaToIndex(db: *mut sqlite3, __v974: *mut Schema) -> i32;
    fn sqlite3HasExplicitNulls(__v975: *mut Parse, __v976: *mut ExprList) -> i32;
    fn sqlite3OomFault(__v977: *mut sqlite3) -> *mut ();
    fn sqlite3SelectDestInit(__v978: *mut SelectDest, __v979: i32, __v980: i32);
    fn sqlite3ReadOnlyShadowTables(db: *mut sqlite3) -> i32;
    fn sqlite3ShadowTableName(db: *mut sqlite3, zName: *const i8) -> i32;
    fn sqlite3ParseObjectInit(__v984: *mut Parse, __v985: *mut sqlite3);
    fn sqlite3ParseObjectReset(__v986: *mut Parse);
    fn sqlite3UpsertDelete(__v987: *mut sqlite3, __v988: *mut Upsert);
    fn sqlite3UpsertDup(__v989: *mut sqlite3, __v990: *mut Upsert) -> *mut Upsert;
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
    trace: __SlateRecord164,
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
    u1: __SlateRecord165,
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
    u: __SlateRecord166,
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
    __slate_bits_0: __slate_bits::__SlateBits61U0,
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
    u: __SlateRecord167,
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
    __slate_bits_0: __slate_bits::__SlateBits87U0,
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
    u: __SlateRecord175,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord176,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord177,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord178,
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
    fg: __SlateRecord185,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord186,
    u2: __SlateRecord187,
    u3: __SlateRecord188,
    u4: __SlateRecord189,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct OnOrUsing {
    pOn: *mut Expr,
    pUsing: *mut IdList,
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
    uNC: __SlateRecord190,
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
    __slate_bits_0: __slate_bits::__SlateBits103U0,
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
    u1: __SlateRecord192,
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
struct Walker {
    pParse: *mut Parse,
    xExprCallback: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Expr) -> i32>,
    xSelectCallback: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
    xSelectCallback2: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select)>,
    walkerDepth: i32,
    eCode: u16,
    mWFlags: u16,
    u: __SlateRecord195,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VtabCtx {}

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
    __slate_bits_0: __slate_bits::__SlateBits163U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord164 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord165 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord166 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord167 {
    tab: __SlateRecord168,
    view: __SlateRecord169,
    vtab: __SlateRecord170,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord168 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord169 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord170 {
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
union __SlateRecord175 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord179,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord179 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord181,
    u: __SlateRecord182,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord181 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits181U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    x: __SlateRecord183,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord183 {
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
struct __SlateRecord185 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits185U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord189 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord190 {
    pEList: *mut ExprList,
    pAggInfo: *mut AggInfo,
    pUpsert: *mut Upsert,
    iBaseReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord192 {
    cr: __SlateRecord193,
    d: __SlateRecord194,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord193 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord194 {
    pReturning: *mut Returning,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord195 {
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

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord204 {
    sSrc: SrcList,
    fromSpace: [u8; 80],
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
    pub struct __SlateBits61U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits181U0 {
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
    pub struct __SlateBits185U0 {
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
    pub struct __SlateBits163U0 {
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
    pub struct __SlateBits103U0 {
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

/// Delete a linked list of TriggerStep structures.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DeleteTriggerStep(
    mut db: *mut sqlite3,
    mut pTriggerStep: *mut TriggerStep,
) {
    '__slate_break_991: while pTriggerStep != std::ptr::null_mut::<TriggerStep>() {
        let mut pTmp: *mut TriggerStep = pTriggerStep;
        pTriggerStep = unsafe { (*pTriggerStep).pNext };
        unsafe { sqlite3ExprDelete(db, unsafe { (*pTmp).pWhere }) };
        unsafe { sqlite3ExprListDelete(db, unsafe { (*pTmp).pExprList }) };
        unsafe { sqlite3SelectDelete(db, unsafe { (*pTmp).pSelect }) };
        unsafe { sqlite3IdListDelete(db, unsafe { (*pTmp).pIdList }) };
        unsafe { sqlite3UpsertDelete(db, unsafe { (*pTmp).pUpsert }) };
        unsafe { sqlite3SrcListDelete(db, unsafe { (*pTmp).pSrc }) };
        unsafe { sqlite3DbFree(db, (unsafe { (*pTmp).zSpan }) as *mut ()) };
        unsafe { sqlite3DbFree(db, pTmp as *mut ()) };
    }
}

/// Given table pTab, return a list of all the triggers attached to
/// the table. The list is connected by Trigger.pNext pointers.
///
/// All of the triggers on pTab that are in the same database as pTab
/// are already attached to pTab->pTrigger.  But there might be additional
/// triggers on pTab in the TEMP schema.  This routine prepends all
/// TEMP triggers on pTab to the beginning of the pTab->pTrigger list
/// and returns the combined list.
///
/// To state it another way:  This routine returns a list of all triggers
/// that fire off of pTab.  The list will include any TEMP triggers on
/// pTab as well as the triggers lised in pTab->pTrigger.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TriggerList(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
) -> *mut Trigger {
    let mut pTmpSchema: *mut Schema = unsafe { std::mem::zeroed() }; // Schema of the pTab table
    let mut pList: *mut Trigger = unsafe { std::mem::zeroed() }; // List of triggers to return
    let mut p: *mut HashElem = unsafe { std::mem::zeroed() }; // Loop variable for TEMP triggers
    0 as i32;
    pTmpSchema = unsafe {
        (*unsafe { unsafe { (*unsafe { (*pParse).db }).aDb }.offset((1 as i32) as isize) }).pSchema
    };
    p = unsafe { (*unsafe { std::ptr::addr_of_mut!((*pTmpSchema).trigHash) }).first };
    pList = unsafe { (*pTab).pTrigger };
    '__slate_break_992: while p != std::ptr::null_mut::<HashElem>() {
        let mut pTrig: *mut Trigger = (unsafe { (*p).data }) as *mut Trigger;
        let __v1074: bool;
        if (unsafe { (*pTrig).pTabSchema }) == unsafe { (*pTab).pSchema }
            && (unsafe { (*pTrig).table }) != std::ptr::null_mut::<i8>()
        {
            __v1074 = (0 as i32)
                == unsafe {
                    sqlite3StrICmp(
                        (unsafe { (*pTrig).table }) as *const i8,
                        (unsafe { (*pTab).zName }) as *const i8,
                    )
                };
        } else {
            __v1074 = false as bool;
        }
        if __v1074
            && ((unsafe { (*pTrig).pTabSchema }) != pTmpSchema
                || (unsafe { (*pTrig).bReturning }) != (0 as u8))
        {
            unsafe {
                (*pTrig).pNext = pList;
            }
            pList = pTrig;
        } else {
            if (((unsafe { (*pTrig).op }) as u32) as i32) == (151 as i32) {
                0 as i32;
                0 as i32;
                0 as i32;
                0 as i32;
                unsafe {
                    (*pTrig).table = unsafe { (*pTab).zName };
                }
                unsafe {
                    (*pTrig).pTabSchema = unsafe { (*pTab).pSchema };
                }
                unsafe {
                    (*pTrig).pNext = pList;
                }
                pList = pTrig;
            }
        }
        p = unsafe { (*p).next };
    }
    return pList;
}

/// This is called by the parser when it sees a CREATE TRIGGER statement
/// up to the point of the BEGIN before the trigger actions.  A Trigger
/// structure is generated based on the information available and stored
/// in pParse->pNewTrigger.  After the trigger actions have been parsed, the
/// sqlite3FinishTrigger() function is called to complete the trigger
/// construction process.
///
/// # Arguments
///
/// * `pParse` - The parse context of the CREATE TRIGGER statement
/// * `pName1` - The name of the trigger
/// * `pName2` - The name of the trigger
/// * `tr_tm` - One of TK_BEFORE, TK_AFTER, TK_INSTEAD
/// * `op` - One of TK_INSERT, TK_UPDATE, TK_DELETE
/// * `pColumns` - column list if this is an UPDATE OF trigger
/// * `pTableName` - The name of the table/view the trigger applies to
/// * `pWhen` - WHEN clause
/// * `isTemp` - True if the TEMPORARY keyword is present
/// * `noErr` - Suppress errors if the trigger already exists
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BeginTrigger(
    mut pParse: *mut Parse,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
    mut tr_tm: i32,
    mut op: i32,
    mut pColumns: *mut IdList,
    mut pTableName: *mut SrcList,
    mut pWhen: *mut Expr,
    mut isTemp: i32,
    mut noErr: i32,
) {
    let mut __slate_storage_447: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_447: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_447) as *mut *const i8;
    let mut __slate_storage_446: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_446: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_446) as *mut *const i8;
    let mut __slate_storage_445: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_445: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_445) as *mut i32;
    let mut __slate_storage_444: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_444: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_444) as *mut i32;
    let mut __slate_storage_1063: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1063: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1063) as *mut bool; // State vector for the DB fixer
    let mut __slate_storage_443: std::mem::MaybeUninit<DbFixer> = std::mem::MaybeUninit::uninit();
    let __slate_slot_443: *mut DbFixer =
        std::ptr::addr_of_mut!(__slate_storage_443) as *mut DbFixer; // The unqualified db name
    let mut __slate_storage_442: std::mem::MaybeUninit<*mut Token> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_442: *mut *mut Token =
        std::ptr::addr_of_mut!(__slate_storage_442) as *mut *mut Token; // The database to store the trigger in
    let mut __slate_storage_441: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_441: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_441) as *mut i32; // The database connection
    let mut __slate_storage_440: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_440: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_440) as *mut *mut sqlite3; // Name of the trigger
    let mut __slate_storage_439: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_439: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_439) as *mut *mut i8; // Table that the trigger fires off of
    let mut __slate_storage_438: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_438: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_438) as *mut *mut Table; // The new trigger
    let mut __slate_storage_437: std::mem::MaybeUninit<*mut Trigger> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_437: *mut *mut Trigger =
        std::ptr::addr_of_mut!(__slate_storage_437) as *mut *mut Trigger;
    unsafe {
        '__join_5: {
            std::ptr::write(__slate_slot_437, std::ptr::null_mut::<Trigger>());
            std::ptr::write(__slate_slot_439, std::ptr::null_mut::<i8>());
            std::ptr::write(__slate_slot_440, unsafe { (*pParse).db });
            0 as i32; // pName1->z might be NULL, but not pName1 itself
            0 as i32;
            0 as i32;
            0 as i32;
            if isTemp != (0 as i32) {
                // If TEMP was specified, then the trigger name may not be qualified.
                if (unsafe { (*pName2).n }) > ((0 as i32) as u32) {
                    unsafe {
                        sqlite3ErrorMsg(
                            pParse,
                            (b"temporary trigger may not have qualified name\0".as_ptr() as *mut i8)
                                as *const i8,
                        )
                    };
                    break '__join_5;
                } else {
                    *__slate_slot_441 = 1 as i32;
                    *__slate_slot_442 = pName1;
                }
            } else {
                // Figure out the db that the trigger will be created in
                *__slate_slot_441 = unsafe {
                    sqlite3TwoPartName(
                        pParse,
                        pName1,
                        pName2,
                        std::ptr::addr_of_mut!(*__slate_slot_442),
                    )
                };
                if *__slate_slot_441 < (0 as i32) {
                    break '__join_5;
                }
            }
            if !(pTableName != std::ptr::null_mut::<SrcList>())
                || (unsafe { (*(*__slate_slot_440)).mallocFailed }) != (0 as u8)
            {
            } else {
                // A long-standing parser bug is that this syntax was allowed:
                //
                //    CREATE TRIGGER attached.demo AFTER INSERT ON attached.tab ....
                //                                                 ^^^^^^^^
                //
                // To maintain backwards compatibility, ignore the database
                // name on pTableName if we are reparsing out of the schema table
                if (unsafe { (*(*__slate_slot_440)).init.busy }) != (0 as u8)
                    && *__slate_slot_441 != (1 as i32)
                {
                    0 as i32;
                    0 as i32;
                    unsafe {
                        sqlite3DbFree(
                            *__slate_slot_440,
                            (unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pTableName).a) as *mut SrcItem
                                    }
                                    .offset((0 as i32) as isize)
                                })
                                .u4
                                .zDatabase
                            }) as *mut (),
                        )
                    };
                    unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pTableName).a) as *mut SrcItem }
                                .offset((0 as i32) as isize)
                        })
                        .u4
                        .zDatabase = std::ptr::null_mut::<i8>();
                    }
                }
                // If the trigger name was unqualified, and the table is a temp table,
                // then set iDb to 1 to create the trigger in the temporary database.
                // If sqlite3SrcListLookup() returns 0, indicating the table does not
                // exist, the error is caught by the block below.
                *__slate_slot_438 = unsafe { sqlite3SrcListLookup(pParse, pTableName) };
                if (((unsafe { (*(*__slate_slot_440)).init.busy }) as u32) as i32) == (0 as i32)
                    && (unsafe { (*pName2).n }) == ((0 as i32) as u32)
                    && *__slate_slot_438 != std::ptr::null_mut::<Table>()
                    && (unsafe { (*(*__slate_slot_438)).pSchema })
                        == unsafe {
                            (*unsafe {
                                unsafe { (*(*__slate_slot_440)).aDb }.offset((1 as i32) as isize)
                            })
                            .pSchema
                        }
                {
                    *__slate_slot_441 = 1 as i32;
                }
                // Ensure the table name matches database name and that the table exists
                if (unsafe { (*(*__slate_slot_440)).mallocFailed }) != (0 as u8) {
                } else {
                    0 as i32;
                    unsafe {
                        sqlite3FixInit(
                            std::ptr::addr_of_mut!(*__slate_slot_443),
                            pParse,
                            *__slate_slot_441,
                            (b"trigger\0".as_ptr() as *mut i8) as *const i8,
                            *__slate_slot_442 as *const Token,
                        )
                    };
                    if (unsafe {
                        sqlite3FixSrcList(std::ptr::addr_of_mut!(*__slate_slot_443), pTableName)
                    }) != (0 as i32)
                    {
                    } else {
                        *__slate_slot_438 = unsafe { sqlite3SrcListLookup(pParse, pTableName) };
                        if !(*__slate_slot_438 != std::ptr::null_mut::<Table>()) {
                            // The table does not exist.
                        } else {
                            if (((unsafe { (*(*__slate_slot_438)).eTabType }) as u32) as i32)
                                == (1 as i32)
                            {
                                unsafe {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        (b"cannot create triggers on virtual tables\0".as_ptr()
                                            as *mut i8)
                                            as *const i8,
                                    )
                                };
                            } else {
                                if (unsafe { (*(*__slate_slot_438)).tabFlags })
                                    & ((4096 as i32) as u32)
                                    != ((0 as i32) as u32)
                                {
                                    *__slate_slot_1063 =
                                        (unsafe { sqlite3ReadOnlyShadowTables(*__slate_slot_440) })
                                            != (0 as i32);
                                } else {
                                    *__slate_slot_1063 = false as bool;
                                }
                                if *__slate_slot_1063 {
                                    unsafe {
                                        sqlite3ErrorMsg(
                                            pParse,
                                            (b"cannot create triggers on shadow tables\0".as_ptr()
                                                as *mut i8)
                                                as *const i8,
                                        )
                                    };
                                } else {
                                    // Check that the trigger name is not reserved and that no trigger of the
                                    // specified name exists
                                    *__slate_slot_439 = unsafe {
                                        sqlite3NameFromToken(
                                            *__slate_slot_440,
                                            *__slate_slot_442 as *const Token,
                                        )
                                    };
                                    if *__slate_slot_439 == std::ptr::null_mut::<i8>() {
                                        0 as i32;
                                        break '__join_5;
                                    } else {
                                        if (unsafe {
                                            sqlite3CheckObjectName(
                                                pParse,
                                                *__slate_slot_439 as *const i8,
                                                (b"trigger\0".as_ptr() as *mut i8) as *const i8,
                                                (unsafe { (*(*__slate_slot_438)).zName })
                                                    as *const i8,
                                            )
                                        }) != (0 as i32)
                                        {
                                            break '__join_5;
                                        } else {
                                            0 as i32;
                                            if !((((unsafe { (*pParse).eParseMode }) as u32)
                                                as i32)
                                                >= (2 as i32))
                                            {
                                                if (unsafe {
                                                    sqlite3HashFind(
                                                        (unsafe {
                                                            std::ptr::addr_of_mut!(
                                                                (*unsafe {
                                                                    (*unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_440))
                                                                                .aDb
                                                                        }
                                                                        .offset(
                                                                            *__slate_slot_441
                                                                                as isize,
                                                                        )
                                                                    })
                                                                    .pSchema
                                                                })
                                                                .trigHash
                                                            )
                                                        })
                                                            as *const Hash,
                                                        *__slate_slot_439 as *const i8,
                                                    )
                                                }) != std::ptr::null_mut::<()>()
                                                {
                                                    if !(noErr != (0 as i32)) {
                                                        unsafe {
                                                            sqlite3ErrorMsg(
                                                                pParse,
                                                                (b"trigger %T already exists\0"
                                                                    .as_ptr()
                                                                    as *mut i8)
                                                                    as *const i8,
                                                                *__slate_slot_442,
                                                            )
                                                        };
                                                        break '__join_5;
                                                    } else {
                                                        if (unsafe {
                                                            (*(*__slate_slot_440)).init.busy
                                                        }) != (0 as u8)
                                                        {
                                                            unsafe {
                                                                sqlite3ErrorMsg(
                                                                    pParse,
                                                                    (b"\0".as_ptr() as *mut i8)
                                                                        as *const i8,
                                                                )
                                                            }; // Err msg generated by corruptSchema()
                                                            break '__join_5;
                                                        } else {
                                                            unsafe {
                                                                sqlite3CodeVerifySchema(
                                                                    pParse,
                                                                    *__slate_slot_441,
                                                                )
                                                            };
                                                            break '__join_5;
                                                        }
                                                    }
                                                }
                                            }
                                            // NB: The SQLITE_ALLOW_TRIGGERS_ON_SYSTEM_TABLES compile-time option is
                                            // experimental and unsupported. Do not use it unless understand the
                                            // implications and you cannot get by without this capability.
                                            //
                                            // Do not create a trigger on a system table
                                            if (unsafe {
                                                sqlite3_strnicmp(
                                                    (unsafe { (*(*__slate_slot_438)).zName })
                                                        as *const i8,
                                                    (b"sqlite_\0".as_ptr() as *mut i8) as *const i8,
                                                    7 as i32,
                                                )
                                            }) == (0 as i32)
                                            {
                                                unsafe {
                                                    sqlite3ErrorMsg(
                                                        pParse,
                                                        (b"cannot create trigger on system table\0"
                                                            .as_ptr()
                                                            as *mut i8)
                                                            as *const i8,
                                                    )
                                                };
                                                break '__join_5;
                                            } else {
                                                // INSTEAD of triggers are only for views and views only support INSTEAD
                                                // of triggers.
                                                if (((unsafe { (*(*__slate_slot_438)).eTabType })
                                                    as u32)
                                                    as i32)
                                                    == (2 as i32)
                                                    && tr_tm != (66 as i32)
                                                {
                                                    unsafe {
                                                        sqlite3ErrorMsg(pParse, (b"cannot create %s trigger on view: %S\0".as_ptr() as *mut i8) as *const i8, if tr_tm == (33 as i32) { b"BEFORE\0".as_ptr() as *mut i8 } else { b"AFTER\0".as_ptr() as *mut i8 }, unsafe { std::ptr::addr_of_mut!((*pTableName).a) as *mut SrcItem })
                                                    };
                                                } else {
                                                    if !((((unsafe {
                                                        (*(*__slate_slot_438)).eTabType
                                                    })
                                                        as u32)
                                                        as i32)
                                                        == (2 as i32))
                                                        && tr_tm == (66 as i32)
                                                    {
                                                        unsafe {
                                                            sqlite3ErrorMsg(pParse, (b"cannot create INSTEAD OF trigger on table: %S\0".as_ptr() as *mut i8) as *const i8, unsafe { std::ptr::addr_of_mut!((*pTableName).a) as *mut SrcItem })
                                                        };
                                                    } else {
                                                        if !((((unsafe { (*pParse).eParseMode })
                                                            as u32)
                                                            as i32)
                                                            >= (2 as i32))
                                                        {
                                                            std::ptr::write(
                                                                __slate_slot_444,
                                                                unsafe {
                                                                    sqlite3SchemaToIndex(
                                                                        *__slate_slot_440,
                                                                        unsafe {
                                                                            (*(*__slate_slot_438))
                                                                                .pSchema
                                                                        },
                                                                    )
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_445,
                                                                7 as i32,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_446,
                                                                (unsafe {
                                                                    (*unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_440))
                                                                                .aDb
                                                                        }
                                                                        .offset(
                                                                            *__slate_slot_444
                                                                                as isize,
                                                                        )
                                                                    })
                                                                    .zDbSName
                                                                })
                                                                    as *const i8,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_447,
                                                                if isTemp != (0 as i32) {
                                                                    (unsafe {
                                                                        (*unsafe { unsafe { (*(*__slate_slot_440)).aDb }.offset((1 as i32) as isize) }).zDbSName
                                                                    })
                                                                        as *const i8
                                                                } else {
                                                                    *__slate_slot_446
                                                                },
                                                            );
                                                            if *__slate_slot_444 == (1 as i32)
                                                                || isTemp != (0 as i32)
                                                            {
                                                                *__slate_slot_445 = 5 as i32;
                                                            }
                                                            if (unsafe {
                                                                sqlite3AuthCheck(
                                                                    pParse,
                                                                    *__slate_slot_445,
                                                                    *__slate_slot_439 as *const i8,
                                                                    (unsafe {
                                                                        (*(*__slate_slot_438)).zName
                                                                    })
                                                                        as *const i8,
                                                                    *__slate_slot_447,
                                                                )
                                                            }) != (0 as i32)
                                                            {
                                                                break '__join_5;
                                                            } else {
                                                                if (unsafe {
                                                                    sqlite3AuthCheck(
                                                                        pParse,
                                                                        18 as i32,
                                                                        (if !((0 as i32)
                                                                            != (0 as i32))
                                                                            && *__slate_slot_444
                                                                                == (1 as i32)
                                                                        {
                                                                            b"sqlite_temp_master\0"
                                                                                .as_ptr()
                                                                                as *mut i8
                                                                        } else {
                                                                            b"sqlite_master\0"
                                                                                .as_ptr()
                                                                                as *mut i8
                                                                        })
                                                                            as *const i8,
                                                                        std::ptr::null::<i8>(),
                                                                        *__slate_slot_446,
                                                                    )
                                                                }) != (0 as i32)
                                                                {
                                                                    break '__join_5;
                                                                }
                                                            }
                                                        }
                                                        // INSTEAD OF triggers can only appear on views and BEFORE triggers
                                                        // cannot appear on views.  So we might as well translate every
                                                        // INSTEAD OF trigger into a BEFORE trigger.  It simplifies code
                                                        // elsewhere.
                                                        if tr_tm == (66 as i32) {
                                                            tr_tm = 33 as i32;
                                                        }
                                                        // Build the Trigger object
                                                        *__slate_slot_437 = (unsafe {
                                                            sqlite3DbMallocZero(
                                                                *__slate_slot_440,
                                                                72 as u64,
                                                            )
                                                        })
                                                            as *mut Trigger;
                                                        if *__slate_slot_437
                                                            == std::ptr::null_mut::<Trigger>()
                                                        {
                                                            break '__join_5;
                                                        } else {
                                                            unsafe {
                                                                (*(*__slate_slot_437)).zName =
                                                                    *__slate_slot_439;
                                                            }
                                                            *__slate_slot_439 =
                                                                std::ptr::null_mut::<i8>();
                                                            unsafe {
                                                                (*(*__slate_slot_437)).table = unsafe {
                                                                    sqlite3DbStrDup(
                                                                        *__slate_slot_440,
                                                                        (unsafe {
                                                                            (*unsafe { unsafe { std::ptr::addr_of_mut!((*pTableName).a) as *mut SrcItem }.offset((0 as i32) as isize) }).zName
                                                                        })
                                                                            as *const i8,
                                                                    )
                                                                };
                                                            }
                                                            unsafe {
                                                                (*(*__slate_slot_437)).pSchema = unsafe {
                                                                    (*unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_440))
                                                                                .aDb
                                                                        }
                                                                        .offset(
                                                                            *__slate_slot_441
                                                                                as isize,
                                                                        )
                                                                    })
                                                                    .pSchema
                                                                };
                                                            }
                                                            unsafe {
                                                                (*(*__slate_slot_437)).pTabSchema = unsafe {
                                                                    (*(*__slate_slot_438)).pSchema
                                                                };
                                                            }
                                                            unsafe {
                                                                (*(*__slate_slot_437)).op =
                                                                    (op as i8) as u8;
                                                            }
                                                            unsafe {
                                                                (*(*__slate_slot_437)).tr_tm =
                                                                    ((if tr_tm == (33 as i32) {
                                                                        1 as i32
                                                                    } else {
                                                                        2 as i32
                                                                    })
                                                                        as i8)
                                                                        as u8;
                                                            }
                                                            if (((unsafe { (*pParse).eParseMode })
                                                                as u32)
                                                                as i32)
                                                                >= (2 as i32)
                                                            {
                                                                unsafe {
                                                                    sqlite3RenameTokenRemap(
                                                                        pParse,
                                                                        (unsafe {
                                                                            (*(*__slate_slot_437))
                                                                                .table
                                                                        })
                                                                            as *const (),
                                                                        (unsafe {
                                                                            (*unsafe { unsafe { std::ptr::addr_of_mut!((*pTableName).a) as *mut SrcItem }.offset((0 as i32) as isize) }).zName
                                                                        })
                                                                            as *const (),
                                                                    )
                                                                };
                                                                unsafe {
                                                                    (*(*__slate_slot_437)).pWhen =
                                                                        pWhen;
                                                                }
                                                                pWhen =
                                                                    std::ptr::null_mut::<Expr>();
                                                            } else {
                                                                unsafe {
                                                                    (*(*__slate_slot_437)).pWhen = unsafe {
                                                                        sqlite3ExprDup(
                                                                            *__slate_slot_440,
                                                                            pWhen as *const Expr,
                                                                            1 as i32,
                                                                        )
                                                                    };
                                                                }
                                                            }
                                                            unsafe {
                                                                (*(*__slate_slot_437)).pColumns =
                                                                    pColumns;
                                                            }
                                                            pColumns =
                                                                std::ptr::null_mut::<IdList>();
                                                            0 as i32;
                                                            unsafe {
                                                                (*pParse).pNewTrigger =
                                                                    *__slate_slot_437;
                                                            }
                                                            break '__join_5;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if (((unsafe { (*(*__slate_slot_440)).init.iDb }) as u32) as i32)
                            == (1 as i32)
                        {
                            // Ticket #3810.
                            // Normally, whenever a table is dropped, all associated triggers are
                            // dropped too.  But if a TEMP trigger is created on a non-TEMP table
                            // and the table is dropped by a different database connection, the
                            // trigger is not visible to the database connection that does the
                            // drop so the trigger cannot be dropped.  This results in an
                            // "orphaned trigger" - a trigger whose associated table is missing.
                            //
                            // 2020-11-05 see also https://sqlite.org/forum/forumpost/157dc791df
                            unsafe {
                                (*(*__slate_slot_440))
                                    .init
                                    .__slate_bits_0
                                    .__set_orphanTrigger((1 as i32) as u32);
                            }
                        }
                    }
                }
            }
        }
        unsafe { sqlite3DbFree(*__slate_slot_440, *__slate_slot_439 as *mut ()) };
        unsafe { sqlite3SrcListDelete(*__slate_slot_440, pTableName) };
        unsafe { sqlite3IdListDelete(*__slate_slot_440, pColumns) };
        unsafe { sqlite3ExprDelete(*__slate_slot_440, pWhen) };
        if !((unsafe { (*pParse).pNewTrigger }) != std::ptr::null_mut::<Trigger>()) {
            sqlite3DeleteTrigger(*__slate_slot_440, *__slate_slot_437);
        } else {
            0 as i32;
        }
        return;
    }
}

/// This routine is called after all of the trigger actions have been parsed
/// in order to complete the process of building the trigger.
///
/// # Arguments
///
/// * `pParse` - Parser context
/// * `pStepList` - The triggered program
/// * `pAll` - Token that describes the complete CREATE TRIGGER
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FinishTrigger(
    mut pParse: *mut Parse,
    mut pStepList: *mut TriggerStep,
    mut pAll: *mut Token,
) {
    let mut __slate_storage_464: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_464: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_464) as *mut *mut Table;
    let mut __slate_storage_463: std::mem::MaybeUninit<*mut Hash> = std::mem::MaybeUninit::uninit();
    let __slate_slot_463: *mut *mut Hash =
        std::ptr::addr_of_mut!(__slate_storage_463) as *mut *mut Hash;
    let mut __slate_storage_462: std::mem::MaybeUninit<*mut Trigger> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_462: *mut *mut Trigger =
        std::ptr::addr_of_mut!(__slate_storage_462) as *mut *mut Trigger;
    let mut __slate_storage_1067: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1067: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1067) as *mut bool;
    let mut __slate_storage_461: std::mem::MaybeUninit<*mut TriggerStep> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_461: *mut *mut TriggerStep =
        std::ptr::addr_of_mut!(__slate_storage_461) as *mut *mut TriggerStep;
    let mut __slate_storage_460: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_460: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_460) as *mut *mut i8;
    let mut __slate_storage_459: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_459: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_459) as *mut *mut Vdbe;
    let mut __slate_storage_1066: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1066: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1066) as *mut bool;
    let mut __slate_storage_1065: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1065: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1065) as *mut i64;
    let mut __slate_storage_1064: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1064: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1064) as *mut i64; // Number of steps
    let mut __slate_storage_458: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_458: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_458) as *mut i64; // Trigger name for error reporting
    let mut __slate_storage_457: std::mem::MaybeUninit<Token> = std::mem::MaybeUninit::uninit();
    let __slate_slot_457: *mut Token = std::ptr::addr_of_mut!(__slate_storage_457) as *mut Token; // Database containing the trigger
    let mut __slate_storage_456: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_456: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_456) as *mut i32; // Fixer object
    let mut __slate_storage_455: std::mem::MaybeUninit<DbFixer> = std::mem::MaybeUninit::uninit();
    let __slate_slot_455: *mut DbFixer =
        std::ptr::addr_of_mut!(__slate_storage_455) as *mut DbFixer; // The database
    let mut __slate_storage_454: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_454: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_454) as *mut *mut sqlite3; // Name of trigger
    let mut __slate_storage_453: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_453: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_453) as *mut *mut i8; // Trigger being finished
    let mut __slate_storage_452: std::mem::MaybeUninit<*mut Trigger> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_452: *mut *mut Trigger =
        std::ptr::addr_of_mut!(__slate_storage_452) as *mut *mut Trigger;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_452, unsafe { (*pParse).pNewTrigger });
            std::ptr::write(__slate_slot_454, unsafe { (*pParse).db });
            unsafe {
                (*pParse).pNewTrigger = std::ptr::null_mut::<Trigger>();
            }
            if (unsafe { (*pParse).nErr }) != (0 as i32)
                || !(*__slate_slot_452 != std::ptr::null_mut::<Trigger>())
            {
            } else {
                *__slate_slot_453 = unsafe { (*(*__slate_slot_452)).zName };
                *__slate_slot_456 = unsafe {
                    sqlite3SchemaToIndex(unsafe { (*pParse).db }, unsafe {
                        (*(*__slate_slot_452)).pSchema
                    })
                };
                0 as i32;
                unsafe {
                    (*(*__slate_slot_452)).step_list = pStepList;
                }
                *__slate_slot_458 = (0 as i32) as i64;
                loop {
                    if pStepList != std::ptr::null_mut::<TriggerStep>() {
                        unsafe {
                            (*pStepList).pTrig = *__slate_slot_452;
                        }
                        pStepList = unsafe { (*pStepList).pNext };
                        std::ptr::write(__slate_slot_1064, *__slate_slot_458);
                        std::ptr::write(
                            __slate_slot_1065,
                            *__slate_slot_1064 + ((1 as i32) as i64),
                        );
                        *__slate_slot_458 = *__slate_slot_1065;
                    } else {
                        break;
                    }
                }
                if *__slate_slot_458
                    > ((unsafe {
                        *unsafe {
                            unsafe { (*unsafe { (*pParse).db }).aLimit.as_mut_ptr() as *mut i32 }
                                .offset((14 as i32) as isize)
                        }
                    }) as i64)
                {
                    unsafe {
                        sqlite3ErrorMsg(
                            pParse,
                            (b"trigger \"%w\" contains too many steps\0".as_ptr() as *mut i8)
                                as *const i8,
                            *__slate_slot_453,
                        )
                    };
                } else {
                    unsafe {
                        sqlite3TokenInit(std::ptr::addr_of_mut!(*__slate_slot_457), unsafe {
                            (*(*__slate_slot_452)).zName
                        })
                    };
                    unsafe {
                        sqlite3FixInit(
                            std::ptr::addr_of_mut!(*__slate_slot_455),
                            pParse,
                            *__slate_slot_456,
                            (b"trigger\0".as_ptr() as *mut i8) as *const i8,
                            std::ptr::addr_of_mut!(*__slate_slot_457) as *const Token,
                        )
                    };
                    if (unsafe {
                        sqlite3FixTriggerStep(std::ptr::addr_of_mut!(*__slate_slot_455), unsafe {
                            (*(*__slate_slot_452)).step_list
                        })
                    }) != (0 as i32)
                    {
                        *__slate_slot_1066 = true as bool;
                    } else {
                        *__slate_slot_1066 = (unsafe {
                            sqlite3FixExpr(std::ptr::addr_of_mut!(*__slate_slot_455), unsafe {
                                (*(*__slate_slot_452)).pWhen
                            })
                        }) != (0 as i32);
                    }
                    if *__slate_slot_1066 {
                    } else {
                        if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
                            0 as i32;
                            unsafe {
                                (*pParse).pNewTrigger = *__slate_slot_452;
                            }
                            *__slate_slot_452 = std::ptr::null_mut::<Trigger>();
                        } else {
                            if !((unsafe { (*(*__slate_slot_454)).init.busy }) != (0 as u8)) {
                                '__join_8: {
                                    // If this is a new CREATE TABLE statement, and if shadow tables
                                    // are read-only, and the trigger makes a change to a shadow table,
                                    // then raise an error - do not allow the trigger to be created.
                                    if (unsafe { sqlite3ReadOnlyShadowTables(*__slate_slot_454) })
                                        != (0 as i32)
                                    {
                                        *__slate_slot_461 =
                                            unsafe { (*(*__slate_slot_452)).step_list };
                                        loop {
                                            if *__slate_slot_461
                                                != std::ptr::null_mut::<TriggerStep>()
                                            {
                                                if (unsafe { (*(*__slate_slot_461)).pSrc })
                                                    != std::ptr::null_mut::<SrcList>()
                                                {
                                                    *__slate_slot_1067 = (unsafe {
                                                        sqlite3ShadowTableName(
                                                            *__slate_slot_454,
                                                            (unsafe {
                                                                (*unsafe { unsafe { std::ptr::addr_of_mut!((*unsafe { (*(*__slate_slot_461)).pSrc }).a) as *mut SrcItem }.offset((0 as i32) as isize) }).zName
                                                            })
                                                                as *const i8,
                                                        )
                                                    }) != (0 as i32);
                                                } else {
                                                    *__slate_slot_1067 = false as bool;
                                                }
                                                if *__slate_slot_1067 {
                                                    break;
                                                } else {
                                                    *__slate_slot_461 =
                                                        unsafe { (*(*__slate_slot_461)).pNext };
                                                }
                                            } else {
                                                break '__join_8;
                                            }
                                        }
                                        unsafe {
                                            sqlite3ErrorMsg(pParse, (b"trigger \"%s\" may not write to shadow table \"%s\"\0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_452)).zName }, unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*unsafe { (*(*__slate_slot_461)).pSrc }).a) as *mut SrcItem }.offset((0 as i32) as isize) }).zName })
                                        };
                                        break '__join_0;
                                    }
                                }
                                // Make an entry in the sqlite_schema table
                                *__slate_slot_459 = unsafe { sqlite3GetVdbe(pParse) };
                                if *__slate_slot_459 == std::ptr::null_mut::<Vdbe>() {
                                    break '__join_0;
                                } else {
                                    unsafe {
                                        sqlite3BeginWriteOperation(
                                            pParse,
                                            0 as i32,
                                            *__slate_slot_456,
                                        )
                                    };
                                    *__slate_slot_460 = unsafe {
                                        sqlite3DbStrNDup(
                                            *__slate_slot_454,
                                            ((unsafe { (*pAll).z }) as *mut i8) as *const i8,
                                            (unsafe { (*pAll).n }) as u64,
                                        )
                                    };
                                    {}
                                    unsafe {
                                        sqlite3NestedParse(pParse, (b"INSERT INTO %Q.sqlite_master VALUES('trigger',%Q,%Q,0,'CREATE TRIGGER %q')\0".as_ptr() as *mut i8) as *const i8, unsafe { (*unsafe { unsafe { (*(*__slate_slot_454)).aDb }.offset(*__slate_slot_456 as isize) }).zDbSName }, *__slate_slot_453, unsafe { (*(*__slate_slot_452)).table }, *__slate_slot_460)
                                    };
                                    unsafe {
                                        sqlite3DbFree(
                                            *__slate_slot_454,
                                            *__slate_slot_460 as *mut (),
                                        )
                                    };
                                    unsafe { sqlite3ChangeCookie(pParse, *__slate_slot_456) };
                                    unsafe {
                                        sqlite3VdbeAddParseSchemaOp(
                                            *__slate_slot_459,
                                            *__slate_slot_456,
                                            unsafe {
                                                sqlite3MPrintf(
                                                    *__slate_slot_454,
                                                    (b"type='trigger' AND name='%q'\0".as_ptr()
                                                        as *mut i8)
                                                        as *const i8,
                                                    *__slate_slot_453,
                                                )
                                            },
                                            ((0 as i32) as i16) as u16,
                                        )
                                    };
                                }
                            }
                        }
                        // if we are not initializing,
                        // build the sqlite_schema entry
                        if (unsafe { (*(*__slate_slot_454)).init.busy }) != (0 as u8) {
                            std::ptr::write(__slate_slot_462, *__slate_slot_452);
                            std::ptr::write(__slate_slot_463, unsafe {
                                std::ptr::addr_of_mut!(
                                    (*unsafe {
                                        (*unsafe {
                                            unsafe { (*(*__slate_slot_454)).aDb }
                                                .offset(*__slate_slot_456 as isize)
                                        })
                                        .pSchema
                                    })
                                    .trigHash
                                )
                            });
                            0 as i32;
                            0 as i32;
                            *__slate_slot_452 = (unsafe {
                                sqlite3HashInsert(
                                    *__slate_slot_463,
                                    *__slate_slot_453 as *const i8,
                                    *__slate_slot_452 as *mut (),
                                )
                            }) as *mut Trigger;
                            if *__slate_slot_452 != std::ptr::null_mut::<Trigger>() {
                                unsafe { sqlite3OomFault(*__slate_slot_454) };
                            } else {
                                if (unsafe { (*(*__slate_slot_462)).pSchema })
                                    == unsafe { (*(*__slate_slot_462)).pTabSchema }
                                {
                                    *__slate_slot_464 = (unsafe {
                                        sqlite3HashFind(
                                            (unsafe {
                                                std::ptr::addr_of_mut!(
                                                    (*unsafe { (*(*__slate_slot_462)).pTabSchema })
                                                        .tblHash
                                                )
                                            })
                                                as *const Hash,
                                            (unsafe { (*(*__slate_slot_462)).table }) as *const i8,
                                        )
                                    })
                                        as *mut Table;
                                    0 as i32;
                                    unsafe {
                                        (*(*__slate_slot_462)).pNext =
                                            unsafe { (*(*__slate_slot_464)).pTrigger };
                                    }
                                    unsafe {
                                        (*(*__slate_slot_464)).pTrigger = *__slate_slot_462;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        sqlite3DeleteTrigger(*__slate_slot_454, *__slate_slot_452);
        0 as i32;
        sqlite3DeleteTriggerStep(*__slate_slot_454, pStepList);
    }
}

/// Duplicate a range of text from an SQL statement, then convert all
/// whitespace characters into ordinary space characters.
fn triggerSpanDup(mut db: *mut sqlite3, mut zStart: *const i8, mut zEnd: *const i8) -> *mut i8 {
    let mut z: *mut i8 = unsafe { sqlite3DbSpanDup(db, zStart, zEnd) };
    let mut i: i32 = 0 as i32;
    if z != std::ptr::null_mut::<i8>() {
        i = 0 as i32;
        '__slate_break_1015: loop {
            if !((unsafe { *unsafe { z.offset(i as isize) } }) != (0 as i8)) {
                break;
            }
            if (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                        ((((unsafe { *unsafe { z.offset(i as isize) } }) as u8) as u32) as i32)
                            as isize,
                    )
                }
            }) as u32) as i32)
                & (1 as i32)
                != (0 as i32)
            {
                unsafe {
                    *unsafe { z.offset(i as isize) } = (32 as i32) as i8;
                }
            }
            let __v1085: i32 = i;
            let __v1086: i32 = __v1085 + (1 as i32);
            i = __v1086;
        }
    }
    return z;
}

/// Turn a SELECT statement (that the pSelect parameter points to) into
/// a trigger step.  Return a pointer to a TriggerStep structure.
///
/// The parser calls this routine when it finds a SELECT statement in
/// body of a TRIGGER.
///
/// # Arguments
///
/// * `db` - Database connection
/// * `pSelect` - The SELECT statement
/// * `zStart` - Start of SQL text
/// * `zEnd` - End of SQL text
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TriggerSelectStep(
    mut db: *mut sqlite3,
    mut pSelect: *mut Select,
    mut zStart: *const i8,
    mut zEnd: *const i8,
) -> *mut TriggerStep {
    let mut pTriggerStep: *mut TriggerStep =
        (unsafe { sqlite3DbMallocZero(db, 88 as u64) }) as *mut TriggerStep;
    if pTriggerStep == std::ptr::null_mut::<TriggerStep>() {
        unsafe { sqlite3SelectDelete(db, pSelect) };
        return std::ptr::null_mut::<TriggerStep>();
    }
    unsafe {
        (*pTriggerStep).op = ((139 as i32) as i8) as u8;
    }
    unsafe {
        (*pTriggerStep).pSelect = pSelect;
    }
    unsafe {
        (*pTriggerStep).orconf = ((11 as i32) as i8) as u8;
    }
    unsafe {
        (*pTriggerStep).zSpan = triggerSpanDup(db, zStart, zEnd);
    }
    return pTriggerStep;
}

/// Allocate space to hold a new trigger step.  The allocated space
/// holds both the TriggerStep object and the TriggerStep.target.z string.
///
/// If an OOM error occurs, NULL is returned and db->mallocFailed is set.
///
/// # Arguments
///
/// * `pParse` - Parser context
/// * `op` - Trigger opcode
/// * `pTabList` - Target table
/// * `zStart` - Start of SQL text
/// * `zEnd` - End of SQL text
fn triggerStepAllocate(
    mut pParse: *mut Parse,
    mut op: u8,
    mut pTabList: *mut SrcList,
    mut zStart: *const i8,
    mut zEnd: *const i8,
) -> *mut TriggerStep {
    let mut pNew: *mut Trigger = unsafe { (*pParse).pNewTrigger };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pTriggerStep: *mut TriggerStep = std::ptr::null_mut::<TriggerStep>();
    if (unsafe { (*pParse).nErr }) == (0 as i32) {
        if pNew != std::ptr::null_mut::<Trigger>()
            && (unsafe { (*pNew).pSchema })
                != unsafe { (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pSchema }
            && (unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                        .offset((0 as i32) as isize)
                })
                .u4
                .zDatabase
            }) != std::ptr::null_mut::<i8>()
        {
            unsafe {
                sqlite3ErrorMsg(pParse, (b"qualified table names are not allowed on INSERT, UPDATE, and DELETE statements within triggers\0".as_ptr() as *mut i8) as *const i8)
            };
        } else {
            pTriggerStep = (unsafe { sqlite3DbMallocZero(db, 88 as u64) }) as *mut TriggerStep;
            if pTriggerStep != std::ptr::null_mut::<TriggerStep>() {
                unsafe {
                    (*pTriggerStep).pSrc =
                        unsafe { sqlite3SrcListDup(db, pTabList as *const SrcList, 1 as i32) };
                }
                unsafe {
                    (*pTriggerStep).op = op;
                }
                unsafe {
                    (*pTriggerStep).zSpan = triggerSpanDup(db, zStart, zEnd);
                }
                if (unsafe { (*pTriggerStep).pSrc }) != std::ptr::null_mut::<SrcList>()
                    && (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32)
                {
                    unsafe {
                        sqlite3RenameTokenRemap(
                            pParse,
                            (unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*unsafe { (*pTriggerStep).pSrc }).a)
                                            as *mut SrcItem
                                    }
                                    .offset((0 as i32) as isize)
                                })
                                .zName
                            }) as *const (),
                            (unsafe {
                                (*unsafe {
                                    unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                                        .offset((0 as i32) as isize)
                                })
                                .zName
                            }) as *const (),
                        )
                    };
                }
            }
        }
    }
    unsafe { sqlite3SrcListDelete(db, pTabList) };
    return pTriggerStep;
}

/// Build a trigger step out of an INSERT statement.  Return a pointer
/// to the new trigger step.
///
/// The parser calls this routine when it sees an INSERT inside the
/// body of a trigger.
///
/// # Arguments
///
/// * `pParse` - Parser
/// * `pTabList` - Table to INSERT into
/// * `pColumn` - List of columns in pTableName to insert into
/// * `pSelect` - A SELECT statement that supplies values
/// * `orconf` - The conflict algorithm (OE_Abort, OE_Replace, etc.)
/// * `pUpsert` - ON CONFLICT clauses for upsert
/// * `zStart` - Start of SQL text
/// * `zEnd` - End of SQL text
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TriggerInsertStep(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pColumn: *mut IdList,
    mut pSelect: *mut Select,
    mut orconf: u8,
    mut pUpsert: *mut Upsert,
    mut zStart: *const i8,
    mut zEnd: *const i8,
) -> *mut TriggerStep {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pTriggerStep: *mut TriggerStep = unsafe { std::mem::zeroed() };
    0 as i32;
    pTriggerStep = triggerStepAllocate(pParse, ((128 as i32) as i8) as u8, pTabList, zStart, zEnd);
    if pTriggerStep != std::ptr::null_mut::<TriggerStep>() {
        if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
            unsafe {
                (*pTriggerStep).pSelect = pSelect;
            }
            pSelect = std::ptr::null_mut::<Select>();
        } else {
            unsafe {
                (*pTriggerStep).pSelect =
                    unsafe { sqlite3SelectDup(db, pSelect as *const Select, 1 as i32) };
            }
        }
        unsafe {
            (*pTriggerStep).pIdList = pColumn;
        }
        unsafe {
            (*pTriggerStep).pUpsert = pUpsert;
        }
        unsafe {
            (*pTriggerStep).orconf = orconf;
        }
        if pUpsert != std::ptr::null_mut::<Upsert>() {
            unsafe { sqlite3HasExplicitNulls(pParse, unsafe { (*pUpsert).pUpsertTarget }) };
        }
    } else {
        {}
        unsafe { sqlite3IdListDelete(db, pColumn) };
        {}
        unsafe { sqlite3UpsertDelete(db, pUpsert) };
    }
    unsafe { sqlite3SelectDelete(db, pSelect) };
    return pTriggerStep;
}

/// Construct a trigger step that implements an UPDATE statement and return
/// a pointer to that trigger step.  The parser calls this routine when it
/// sees an UPDATE statement inside the body of a CREATE TRIGGER.
///
/// # Arguments
///
/// * `pParse` - Parser
/// * `pTabList` - Name of the table to be updated
/// * `pFrom` - FROM clause for an UPDATE-FROM, or NULL
/// * `pEList` - The SET clause: list of column and new values
/// * `pWhere` - The WHERE clause
/// * `orconf` - The conflict algorithm. (OE_Abort, OE_Ignore, etc)
/// * `zStart` - Start of SQL text
/// * `zEnd` - End of SQL text
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TriggerUpdateStep(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pFrom: *mut SrcList,
    mut pEList: *mut ExprList,
    mut pWhere: *mut Expr,
    mut orconf: u8,
    mut zStart: *const i8,
    mut zEnd: *const i8,
) -> *mut TriggerStep {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pTriggerStep: *mut TriggerStep = unsafe { std::mem::zeroed() };
    pTriggerStep = triggerStepAllocate(pParse, ((130 as i32) as i8) as u8, pTabList, zStart, zEnd);
    if pTriggerStep != std::ptr::null_mut::<TriggerStep>() {
        let mut pFromDup: *mut SrcList = std::ptr::null_mut::<SrcList>();
        if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
            unsafe {
                (*pTriggerStep).pExprList = pEList;
            }
            unsafe {
                (*pTriggerStep).pWhere = pWhere;
            }
            pFromDup = pFrom;
            pEList = std::ptr::null_mut::<ExprList>();
            pWhere = std::ptr::null_mut::<Expr>();
            pFrom = std::ptr::null_mut::<SrcList>();
        } else {
            unsafe {
                (*pTriggerStep).pExprList =
                    unsafe { sqlite3ExprListDup(db, pEList as *const ExprList, 1 as i32) };
            }
            unsafe {
                (*pTriggerStep).pWhere =
                    unsafe { sqlite3ExprDup(db, pWhere as *const Expr, 1 as i32) };
            }
            pFromDup = unsafe { sqlite3SrcListDup(db, pFrom as *const SrcList, 1 as i32) };
        }
        unsafe {
            (*pTriggerStep).orconf = orconf;
        }
        if pFromDup != std::ptr::null_mut::<SrcList>()
            && !((((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32))
        {
            let mut pSub: *mut Select = unsafe { std::mem::zeroed() };
            let mut r#as: Token = Token {
                z: std::ptr::null::<i8>(),
                n: (0 as i32) as u32,
            };
            pSub = unsafe {
                sqlite3SelectNew(
                    pParse,
                    std::ptr::null_mut::<ExprList>(),
                    pFromDup,
                    std::ptr::null_mut::<Expr>(),
                    std::ptr::null_mut::<ExprList>(),
                    std::ptr::null_mut::<Expr>(),
                    std::ptr::null_mut::<ExprList>(),
                    (2048 as i32) as u32,
                    std::ptr::null_mut::<Expr>(),
                )
            };
            pFromDup = unsafe {
                sqlite3SrcListAppendFromTerm(
                    pParse,
                    std::ptr::null_mut::<SrcList>(),
                    std::ptr::null_mut::<Token>(),
                    std::ptr::null_mut::<Token>(),
                    std::ptr::addr_of_mut!(r#as),
                    pSub,
                    std::ptr::null_mut::<OnOrUsing>(),
                )
            };
        }
        if pFromDup != std::ptr::null_mut::<SrcList>()
            && (unsafe { (*pTriggerStep).pSrc }) != std::ptr::null_mut::<SrcList>()
        {
            unsafe {
                (*pTriggerStep).pSrc = unsafe {
                    sqlite3SrcListAppendList(pParse, unsafe { (*pTriggerStep).pSrc }, pFromDup)
                };
            }
        } else {
            unsafe { sqlite3SrcListDelete(db, pFromDup) };
        }
    }
    unsafe { sqlite3ExprListDelete(db, pEList) };
    unsafe { sqlite3ExprDelete(db, pWhere) };
    unsafe { sqlite3SrcListDelete(db, pFrom) };
    return pTriggerStep;
}

/// Construct a trigger step that implements a DELETE statement and return
/// a pointer to that trigger step.  The parser calls this routine when it
/// sees a DELETE statement inside the body of a CREATE TRIGGER.
///
/// # Arguments
///
/// * `pParse` - Parser
/// * `pTabList` - The table from which rows are deleted
/// * `pWhere` - The WHERE clause
/// * `zStart` - Start of SQL text
/// * `zEnd` - End of SQL text
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TriggerDeleteStep(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pWhere: *mut Expr,
    mut zStart: *const i8,
    mut zEnd: *const i8,
) -> *mut TriggerStep {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pTriggerStep: *mut TriggerStep = unsafe { std::mem::zeroed() };
    pTriggerStep = triggerStepAllocate(pParse, ((129 as i32) as i8) as u8, pTabList, zStart, zEnd);
    if pTriggerStep != std::ptr::null_mut::<TriggerStep>() {
        if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
            unsafe {
                (*pTriggerStep).pWhere = pWhere;
            }
            pWhere = std::ptr::null_mut::<Expr>();
        } else {
            unsafe {
                (*pTriggerStep).pWhere =
                    unsafe { sqlite3ExprDup(db, pWhere as *const Expr, 1 as i32) };
            }
        }
        unsafe {
            (*pTriggerStep).orconf = ((11 as i32) as i8) as u8;
        }
    }
    unsafe { sqlite3ExprDelete(db, pWhere) };
    return pTriggerStep;
}

/// Recursively delete a Trigger structure
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DeleteTrigger(mut db: *mut sqlite3, mut pTrigger: *mut Trigger) {
    if pTrigger == std::ptr::null_mut::<Trigger>()
        || (unsafe { (*pTrigger).bReturning }) != (0 as u8)
    {
        return;
    }
    sqlite3DeleteTriggerStep(db, unsafe { (*pTrigger).step_list });
    unsafe { sqlite3DbFree(db, (unsafe { (*pTrigger).zName }) as *mut ()) };
    unsafe { sqlite3DbFree(db, (unsafe { (*pTrigger).table }) as *mut ()) };
    unsafe { sqlite3ExprDelete(db, unsafe { (*pTrigger).pWhen }) };
    unsafe { sqlite3IdListDelete(db, unsafe { (*pTrigger).pColumns }) };
    unsafe { sqlite3DbFree(db, pTrigger as *mut ()) };
}

/// This function is called to drop a trigger from the database schema.
///
/// This may be called directly from the parser and therefore identifies
/// the trigger by name.  The sqlite3DropTriggerPtr() routine does the
/// same job as this routine except it takes a pointer to the trigger
/// instead of the trigger name.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DropTrigger(
    mut pParse: *mut Parse,
    mut pName: *mut SrcList,
    mut noErr: i32,
) {
    let mut __slate_storage_1069: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1069: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1069) as *mut i32;
    let mut __slate_storage_1068: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1068: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1068) as *mut i32;
    let mut __slate_storage_1070: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1070: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1070) as *mut bool; // Search TEMP before MAIN
    let mut __slate_storage_526: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_526: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_526) as *mut i32;
    let mut __slate_storage_525: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_525: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_525) as *mut *mut sqlite3;
    let mut __slate_storage_524: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_524: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_524) as *mut *const i8;
    let mut __slate_storage_523: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_523: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_523) as *mut *const i8;
    let mut __slate_storage_522: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_522: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_522) as *mut i32;
    let mut __slate_storage_521: std::mem::MaybeUninit<*mut Trigger> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_521: *mut *mut Trigger =
        std::ptr::addr_of_mut!(__slate_storage_521) as *mut *mut Trigger;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_521, std::ptr::null_mut::<Trigger>());
            std::ptr::write(__slate_slot_525, unsafe { (*pParse).db });
            if (unsafe { (*(*__slate_slot_525)).mallocFailed }) != (0 as u8) {
            } else {
                if (0 as i32) != unsafe { sqlite3ReadSchema(pParse) } {
                } else {
                    0 as i32;
                    0 as i32;
                    *__slate_slot_523 = (unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pName).a) as *mut SrcItem }
                                .offset((0 as i32) as isize)
                        })
                        .u4
                        .zDatabase
                    }) as *const i8;
                    *__slate_slot_524 = (unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pName).a) as *mut SrcItem }
                                .offset((0 as i32) as isize)
                        })
                        .zName
                    }) as *const i8;
                    0 as i32;
                    *__slate_slot_522 = 0 as i32;
                    '__loop_7: loop {
                        if *__slate_slot_522 < unsafe { (*(*__slate_slot_525)).nDb } {
                            std::ptr::write(
                                __slate_slot_526,
                                if *__slate_slot_522 < (2 as i32) {
                                    *__slate_slot_522 ^ (1 as i32)
                                } else {
                                    *__slate_slot_522
                                },
                            );
                            if *__slate_slot_523 != std::ptr::null::<i8>() {
                                *__slate_slot_1070 = (unsafe {
                                    sqlite3DbIsNamed(
                                        *__slate_slot_525,
                                        *__slate_slot_526,
                                        *__slate_slot_523,
                                    )
                                }) == (0 as i32);
                            } else {
                                *__slate_slot_1070 = false as bool;
                            }
                            if *__slate_slot_1070 {
                            } else {
                                0 as i32;
                                *__slate_slot_521 = (unsafe {
                                    sqlite3HashFind(
                                        (unsafe {
                                            std::ptr::addr_of_mut!(
                                                (*unsafe {
                                                    (*unsafe {
                                                        unsafe { (*(*__slate_slot_525)).aDb }
                                                            .offset(*__slate_slot_526 as isize)
                                                    })
                                                    .pSchema
                                                })
                                                .trigHash
                                            )
                                        }) as *const Hash,
                                        *__slate_slot_524,
                                    )
                                })
                                    as *mut Trigger;
                                if *__slate_slot_521 != std::ptr::null_mut::<Trigger>() {
                                    break '__loop_7;
                                }
                            }
                            std::ptr::write(__slate_slot_1068, *__slate_slot_522);
                            std::ptr::write(__slate_slot_1069, *__slate_slot_1068 + (1 as i32));
                            *__slate_slot_522 = *__slate_slot_1069;
                        } else {
                            break;
                        }
                    }
                    if !(*__slate_slot_521 != std::ptr::null_mut::<Trigger>()) {
                        if !(noErr != (0 as i32)) {
                            unsafe {
                                sqlite3ErrorMsg(
                                    pParse,
                                    (b"no such trigger: %S\0".as_ptr() as *mut i8) as *const i8,
                                    unsafe { std::ptr::addr_of_mut!((*pName).a) as *mut SrcItem },
                                )
                            };
                        } else {
                            unsafe { sqlite3CodeVerifyNamedSchema(pParse, *__slate_slot_523) };
                        }
                        unsafe {
                            (*pParse)
                                .__slate_bits_0
                                .__set_checkSchema((1 as i32) as u32);
                        }
                    } else {
                        sqlite3DropTriggerPtr(pParse, *__slate_slot_521);
                    }
                }
            }
        }
        unsafe { sqlite3SrcListDelete(*__slate_slot_525, pName) };
    }
}

/// Return a pointer to the Table structure for the table that a trigger
/// is set on.
fn tableOfTrigger(mut pTrigger: *mut Trigger) -> *mut Table {
    return (unsafe {
        sqlite3HashFind(
            (unsafe { std::ptr::addr_of_mut!((*unsafe { (*pTrigger).pTabSchema }).tblHash) })
                as *const Hash,
            (unsafe { (*pTrigger).table }) as *const i8,
        )
    }) as *mut Table;
}

/// Drop a trigger given a pointer to that trigger.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DropTriggerPtr(mut pParse: *mut Parse, mut pTrigger: *mut Trigger) {
    let mut pTable: *mut Table = unsafe { std::mem::zeroed() };
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut iDb: i32 = 0 as i32;
    iDb = unsafe { sqlite3SchemaToIndex(unsafe { (*pParse).db }, unsafe { (*pTrigger).pSchema }) };
    0 as i32;
    pTable = tableOfTrigger(pTrigger);
    0 as i32;
    if pTable != std::ptr::null_mut::<Table>() {
        let mut code: i32 = 16 as i32;
        let mut zDb: *const i8 =
            (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName })
                as *const i8;
        let mut zTab: *const i8 = (if !((0 as i32) != (0 as i32)) && iDb == (1 as i32) {
            b"sqlite_temp_master\0".as_ptr() as *mut i8
        } else {
            b"sqlite_master\0".as_ptr() as *mut i8
        }) as *const i8;
        if iDb == (1 as i32) {
            code = 14 as i32;
        }
        let __v1071: bool;
        if (unsafe {
            sqlite3AuthCheck(
                pParse,
                code,
                (unsafe { (*pTrigger).zName }) as *const i8,
                (unsafe { (*pTable).zName }) as *const i8,
                zDb,
            )
        }) != (0 as i32)
        {
            __v1071 = true as bool;
        } else {
            __v1071 =
                (unsafe { sqlite3AuthCheck(pParse, 9 as i32, zTab, std::ptr::null::<i8>(), zDb) })
                    != (0 as i32);
        }
        if __v1071 {
            return;
        }
    }
    // Generate code to destroy the database record of the trigger.
    let __v1072: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
    v = __v1072;
    if __v1072 != std::ptr::null_mut::<Vdbe>() {
        unsafe {
            sqlite3NestedParse(
                pParse,
                (b"DELETE FROM %Q.sqlite_master WHERE name=%Q AND type='trigger'\0".as_ptr()
                    as *mut i8) as *const i8,
                unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName },
                unsafe { (*pTrigger).zName },
            )
        };
        unsafe { sqlite3ChangeCookie(pParse, iDb) };
        unsafe {
            sqlite3VdbeAddOp4(
                v,
                156 as i32,
                iDb,
                0 as i32,
                0 as i32,
                (unsafe { (*pTrigger).zName }) as *const i8,
                0 as i32,
            )
        };
    }
}

/// Remove a trigger from the hash tables of the sqlite* pointer.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3UnlinkAndDeleteTrigger(
    mut db: *mut sqlite3,
    mut iDb: i32,
    mut zName: *const i8,
) {
    let mut pTrigger: *mut Trigger = unsafe { std::mem::zeroed() };
    let mut pHash: *mut Hash = unsafe { std::mem::zeroed() };
    0 as i32;
    pHash = unsafe {
        std::ptr::addr_of_mut!(
            (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pSchema }).trigHash
        )
    };
    pTrigger =
        (unsafe { sqlite3HashInsert(pHash, zName, std::ptr::null_mut::<()>()) }) as *mut Trigger;
    if pTrigger != std::ptr::null_mut::<Trigger>() {
        if (unsafe { (*pTrigger).pSchema }) == unsafe { (*pTrigger).pTabSchema } {
            let mut pTab: *mut Table = tableOfTrigger(pTrigger);
            if pTab != std::ptr::null_mut::<Table>() {
                let mut pp: *mut *mut Trigger = unsafe { std::mem::zeroed() };
                pp = unsafe { std::ptr::addr_of_mut!((*pTab).pTrigger) };
                '__slate_break_1022: while (unsafe { *pp }) != std::ptr::null_mut::<Trigger>() {
                    if (unsafe { *pp }) == pTrigger {
                        unsafe {
                            *pp = unsafe { (*unsafe { *pp }).pNext };
                        }
                        break '__slate_break_1022;
                    }
                    pp = unsafe { std::ptr::addr_of_mut!((*unsafe { *pp }).pNext) };
                }
            }
        }
        sqlite3DeleteTrigger(db, pTrigger);
        let __v1079: *mut sqlite3 = db;
        let __v1080: u32 = unsafe { (*__v1079).mDbFlags };
        let __v1081: u32 = __v1080 | ((1 as i32) as u32);
        unsafe {
            (*__v1079).mDbFlags = __v1081;
        }
    }
}

/// pEList is the SET clause of an UPDATE statement.  Each entry
/// in pEList is of the format <id>=<expr>.  If any of the entries
/// in pEList have an <id> which matches an identifier in pIdList,
/// then return TRUE.  If pIdList==NULL, then it is considered a
/// wildcard that matches anything.  Likewise if pEList==NULL then
/// it matches anything so always return true.  Return false only
/// if there is no match.
fn checkColumnOverlap(mut pIdList: *mut IdList, mut pEList: *mut ExprList) -> i32 {
    let mut e: i32 = 0 as i32;
    if pIdList == std::ptr::null_mut::<IdList>() || pEList == std::ptr::null_mut::<ExprList>() {
        return 1 as i32;
    }
    e = 0 as i32;
    '__slate_break_1023: loop {
        if !(e < unsafe { (*pEList).nExpr }) {
            break;
        }
        if (unsafe {
            sqlite3IdListIndex(
                pIdList,
                (unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                            .offset(e as isize)
                    })
                    .zEName
                }) as *const i8,
            )
        }) >= (0 as i32)
        {
            return 1 as i32;
        }
        let __v1087: i32 = e;
        let __v1088: i32 = __v1087 + (1 as i32);
        e = __v1088;
    }
    return 0 as i32;
}

/// Return true if any TEMP triggers exist
fn tempTriggersExist(mut db: *mut sqlite3) -> i32 {
    if (unsafe { (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pSchema })
        == std::ptr::null_mut::<Schema>()
    {
        return 0 as i32;
    }
    if (unsafe {
        (*unsafe {
            std::ptr::addr_of_mut!(
                (*unsafe {
                    (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pSchema
                })
                .trigHash
            )
        })
        .first
    }) == std::ptr::null_mut::<HashElem>()
    {
        return 0 as i32;
    }
    return 1 as i32;
}

/// Return a list of all triggers on table pTab if there exists at least
/// one trigger that must be fired when an operation of type 'op' is
/// performed on the table, and, if that operation is an UPDATE, if at
/// least one of the columns in pChanges is being modified.
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `pTab` - The table the contains the triggers
/// * `op` - one of TK_DELETE, TK_INSERT, TK_UPDATE
/// * `pChanges` - Columns that change in an UPDATE statement
/// * `pMask` - OUT: Mask of TRIGGER_BEFORE|TRIGGER_AFTER
fn triggersReallyExist(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut op: i32,
    mut pChanges: *mut ExprList,
    mut pMask: *mut i32,
) -> *mut Trigger {
    let mut __slate_storage_1092: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1092: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1092) as *mut i32;
    let mut __slate_storage_1091: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1091: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1091) as *mut i32;
    let mut __slate_storage_1094: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1094: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1094) as *mut i32;
    let mut __slate_storage_1093: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1093: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1093) as *mut i32;
    let mut __slate_storage_1096: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1096: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1096) as *mut i32;
    // Also fire a RETURNING trigger for an UPSERT
    let mut __slate_storage_1095: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1095: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1095) as *mut i32;
    let mut __slate_storage_1090: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1090: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1090) as *mut bool;
    let mut __slate_storage_1089: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1089: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1089) as *mut bool;
    let mut __slate_storage_560: std::mem::MaybeUninit<*mut Trigger> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_560: *mut *mut Trigger =
        std::ptr::addr_of_mut!(__slate_storage_560) as *mut *mut Trigger;
    let mut __slate_storage_559: std::mem::MaybeUninit<*mut Trigger> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_559: *mut *mut Trigger =
        std::ptr::addr_of_mut!(__slate_storage_559) as *mut *mut Trigger;
    let mut __slate_storage_558: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_558: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_558) as *mut i32;
    unsafe {
        '__join_2: {
            std::ptr::write(__slate_slot_558, 0 as i32);
            std::ptr::write(__slate_slot_559, std::ptr::null_mut::<Trigger>());
            *__slate_slot_559 = sqlite3TriggerList(pParse, pTab);
            0 as i32;
            if *__slate_slot_559 != std::ptr::null_mut::<Trigger>() {
                *__slate_slot_560 = *__slate_slot_559;
                if (unsafe { (*unsafe { (*pParse).db }).flags }) & (((262144 as i32) as i64) as u64)
                    == (((0 as i32) as i64) as u64)
                    && (unsafe { (*pTab).pTrigger }) != std::ptr::null_mut::<Trigger>()
                {
                    *__slate_slot_1089 = (unsafe {
                        sqlite3SchemaToIndex(unsafe { (*pParse).db }, unsafe {
                            (*unsafe { (*pTab).pTrigger }).pSchema
                        })
                    }) != (1 as i32);
                } else {
                    *__slate_slot_1089 = false as bool;
                }
                if *__slate_slot_1089 {
                    // The SQLITE_DBCONFIG_ENABLE_TRIGGER setting is off.  That means that
                    // only TEMP triggers are allowed.  Truncate the pList so that it
                    // includes only TEMP triggers
                    if *__slate_slot_559 == unsafe { (*pTab).pTrigger } {
                        *__slate_slot_559 = std::ptr::null_mut::<Trigger>();
                        break '__join_2;
                    } else {
                        loop {
                            if (unsafe { (*(*__slate_slot_560)).pNext })
                                != std::ptr::null_mut::<Trigger>()
                                && (unsafe { (*(*__slate_slot_560)).pNext })
                                    != unsafe { (*pTab).pTrigger }
                            {
                                *__slate_slot_560 = unsafe { (*(*__slate_slot_560)).pNext };
                            } else {
                                break;
                            }
                        }
                        unsafe {
                            (*(*__slate_slot_560)).pNext = std::ptr::null_mut::<Trigger>();
                        }
                        *__slate_slot_560 = *__slate_slot_559;
                    }
                }
                loop {
                    if (((unsafe { (*(*__slate_slot_560)).op }) as u32) as i32) == op {
                        *__slate_slot_1090 = checkColumnOverlap(
                            unsafe { (*(*__slate_slot_560)).pColumns },
                            pChanges,
                        ) != (0 as i32);
                    } else {
                        *__slate_slot_1090 = false as bool;
                    }
                    if *__slate_slot_1090 {
                        std::ptr::write(__slate_slot_1091, *__slate_slot_558);
                        std::ptr::write(
                            __slate_slot_1092,
                            *__slate_slot_1091
                                | (((unsafe { (*(*__slate_slot_560)).tr_tm }) as u32) as i32),
                        );
                        *__slate_slot_558 = *__slate_slot_1092;
                    } else {
                        if (((unsafe { (*(*__slate_slot_560)).op }) as u32) as i32) == (151 as i32)
                        {
                            // The first time a RETURNING trigger is seen, the "op" value tells
                            // us what time of trigger it should be.
                            0 as i32;
                            unsafe {
                                (*(*__slate_slot_560)).op = (op as i8) as u8;
                            }
                            if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32) {
                                if op != (128 as i32) {
                                    unsafe {
                                        sqlite3ErrorMsg(
                                            pParse,
                                            (b"%s RETURNING is not available on virtual tables\0"
                                                .as_ptr()
                                                as *mut i8)
                                                as *const i8,
                                            if op == (129 as i32) {
                                                b"DELETE\0".as_ptr() as *mut i8
                                            } else {
                                                b"UPDATE\0".as_ptr() as *mut i8
                                            },
                                        )
                                    };
                                }
                                unsafe {
                                    (*(*__slate_slot_560)).tr_tm = ((1 as i32) as i8) as u8;
                                }
                            } else {
                                unsafe {
                                    (*(*__slate_slot_560)).tr_tm = ((2 as i32) as i8) as u8;
                                }
                            }
                            std::ptr::write(__slate_slot_1093, *__slate_slot_558);
                            std::ptr::write(
                                __slate_slot_1094,
                                *__slate_slot_1093
                                    | (((unsafe { (*(*__slate_slot_560)).tr_tm }) as u32) as i32),
                            );
                            *__slate_slot_558 = *__slate_slot_1094;
                        } else {
                            if (unsafe { (*(*__slate_slot_560)).bReturning }) != (0 as u8)
                                && (((unsafe { (*(*__slate_slot_560)).op }) as u32) as i32)
                                    == (128 as i32)
                                && op == (130 as i32)
                                && (unsafe { (*pParse).pToplevel }) == std::ptr::null_mut::<Parse>()
                            {
                                std::ptr::write(__slate_slot_1095, *__slate_slot_558);
                                std::ptr::write(
                                    __slate_slot_1096,
                                    *__slate_slot_1095
                                        | (((unsafe { (*(*__slate_slot_560)).tr_tm }) as u32)
                                            as i32),
                                );
                                *__slate_slot_558 = *__slate_slot_1096;
                            }
                        }
                    }
                    *__slate_slot_560 = unsafe { (*(*__slate_slot_560)).pNext };
                    if !(*__slate_slot_560 != std::ptr::null_mut::<Trigger>()) {
                        break '__join_2;
                    }
                }
            }
        }
        if pMask != std::ptr::null_mut::<i32>() {
            unsafe {
                *pMask = *__slate_slot_558;
            }
        }
        return if *__slate_slot_558 != (0 as i32) {
            *__slate_slot_559
        } else {
            std::ptr::null_mut::<Trigger>()
        };
    }
    return unsafe { std::mem::zeroed() };
}

/// # Arguments
///
/// * `pParse` - Parse context
/// * `pTab` - The table the contains the triggers
/// * `op` - one of TK_DELETE, TK_INSERT, TK_UPDATE
/// * `pChanges` - Columns that change in an UPDATE statement
/// * `pMask` - OUT: Mask of TRIGGER_BEFORE|TRIGGER_AFTER
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TriggersExist(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut op: i32,
    mut pChanges: *mut ExprList,
    mut pMask: *mut i32,
) -> *mut Trigger {
    0 as i32;
    let __v1073: bool;
    if (unsafe { (*pTab).pTrigger }) == std::ptr::null_mut::<Trigger>() {
        __v1073 = !(tempTriggersExist(unsafe { (*pParse).db }) != (0 as i32));
    } else {
        __v1073 = false as bool;
    }
    if __v1073
        || ((unsafe { (*pParse).__slate_bits_0.__get_disableTriggers() }) as i32) != (0 as i32)
    {
        if pMask != std::ptr::null_mut::<i32>() {
            unsafe {
                *pMask = 0 as i32;
            }
        }
        return std::ptr::null_mut::<Trigger>();
    }
    return triggersReallyExist(pParse, pTab, op, pChanges, pMask);
}

/// Return true if the pExpr term from the RETURNING clause argument
/// list is of the form "*".  Raise an error if the terms if of the
/// form "table.*".
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pTerm` - A term in the RETURNING clause
fn isAsteriskTerm(mut pParse: *mut Parse, mut pTerm: *mut Expr) -> i32 {
    0 as i32;
    if (((unsafe { (*pTerm).op }) as u32) as i32) == (180 as i32) {
        return 1 as i32;
    }
    if (((unsafe { (*pTerm).op }) as u32) as i32) != (142 as i32) {
        return 0 as i32;
    }
    0 as i32;
    0 as i32;
    if (((unsafe { (*unsafe { (*pTerm).pRight }).op }) as u32) as i32) != (180 as i32) {
        return 0 as i32;
    }
    unsafe {
        sqlite3ErrorMsg(
            pParse,
            (b"RETURNING may not use \"TABLE.*\" wildcards\0".as_ptr() as *mut i8) as *const i8,
        )
    };
    return 1 as i32;
}

/// The input list pList is the list of result set terms from a RETURNING
/// clause.  The table that we are returning from is pTab.
///
/// This routine makes a copy of the pList, and at the same time expands
/// any "*" wildcards to be the complete set of columns from pTab.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pList` - The arguments to RETURNING
/// * `pTab` - The table being updated
fn sqlite3ExpandReturning(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut pTab: *mut Table,
) -> *mut ExprList {
    let mut pNew: *mut ExprList = std::ptr::null_mut::<ExprList>();
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_1030: loop {
        if !(i < unsafe { (*pList).nExpr }) {
            break;
        }
        let mut pOldExpr: *mut Expr = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                    .offset(i as isize)
            })
            .pExpr
        };
        if pOldExpr == std::ptr::null_mut::<Expr>() {
        } else {
            if isAsteriskTerm(pParse, pOldExpr) != (0 as i32) {
                let mut jj: i32 = 0 as i32;
                jj = 0 as i32;
                '__slate_break_1031: loop {
                    if !(jj < ((unsafe { (*pTab).nCol }) as i32)) {
                        break;
                    }
                    let mut pNewExpr: *mut Expr = unsafe { std::mem::zeroed() };
                    if (((unsafe {
                        (*unsafe { unsafe { (*pTab).aCol }.offset(jj as isize) }).colFlags
                    }) as u32) as i32)
                        & (2 as i32)
                        != (0 as i32)
                    {
                    } else {
                        pNewExpr = unsafe {
                            sqlite3Expr(
                                db,
                                60 as i32,
                                (unsafe {
                                    (*unsafe { unsafe { (*pTab).aCol }.offset(jj as isize) })
                                        .zCnName
                                }) as *const i8,
                            )
                        };
                        pNew = unsafe { sqlite3ExprListAppend(pParse, pNew, pNewExpr) };
                        if !((unsafe { (*db).mallocFailed }) != (0 as u8)) {
                            let mut pItem: *mut ExprList_item = unsafe {
                                unsafe { std::ptr::addr_of_mut!((*pNew).a) as *mut ExprList_item }
                                    .offset(((unsafe { (*pNew).nExpr }) - (1 as i32)) as isize)
                            };
                            unsafe {
                                (*pItem).zEName = unsafe {
                                    sqlite3DbStrDup(
                                        db,
                                        (unsafe {
                                            (*unsafe {
                                                unsafe { (*pTab).aCol }.offset(jj as isize)
                                            })
                                            .zCnName
                                        }) as *const i8,
                                    )
                                };
                            }
                            unsafe {
                                (*pItem).fg.__slate_bits_0.__set_eEName((0 as i32) as u32);
                            }
                        }
                    }
                    let __v1099: i32 = jj;
                    let __v1100: i32 = __v1099 + (1 as i32);
                    jj = __v1100;
                }
            } else {
                let mut pNewExpr: *mut Expr =
                    unsafe { sqlite3ExprDup(db, pOldExpr as *const Expr, 0 as i32) };
                pNew = unsafe { sqlite3ExprListAppend(pParse, pNew, pNewExpr) };
                if !((unsafe { (*db).mallocFailed }) != (0 as u8))
                    && (unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                                .offset(i as isize)
                        })
                        .zEName
                    }) != std::ptr::null_mut::<i8>()
                {
                    let mut pItem: *mut ExprList_item = unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pNew).a) as *mut ExprList_item }
                            .offset(((unsafe { (*pNew).nExpr }) - (1 as i32)) as isize)
                    };
                    unsafe {
                        (*pItem).zEName = unsafe {
                            sqlite3DbStrDup(
                                db,
                                (unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item
                                        }
                                        .offset(i as isize)
                                    })
                                    .zEName
                                }) as *const i8,
                            )
                        };
                    }
                    unsafe {
                        (*pItem).fg.__slate_bits_0.__set_eEName(
                            ((unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item
                                    }
                                    .offset(i as isize)
                                })
                                .fg
                                .__slate_bits_0
                                .__get_eEName()
                            }) as i32) as u32,
                        );
                    }
                }
            }
        }
        let __v1097: i32 = i;
        let __v1098: i32 = __v1097 + (1 as i32);
        i = __v1098;
    }
    return pNew;
}

/// If the Expr node is a subquery or an EXISTS operator or an IN operator that
/// uses a subquery, and if the subquery is SF_Correlated, then mark the
/// expression as EP_VarSelect.
#[unsafe(link_section = ".text.slate_distinct.trigger.sqlite3ReturningSubqueryVarSelect")]
extern "C-unwind" fn sqlite3ReturningSubqueryVarSelect(
    mut NotUsed: *mut Walker,
    mut pExpr: *mut Expr,
) -> i32 {
    NotUsed;
    if (unsafe { (*pExpr).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32)
        && (unsafe { (*unsafe { (*pExpr).x.pSelect }).selFlags }) & ((536870912 as i32) as u32)
            != ((0 as i32) as u32)
    {
        {}
        let __v1101: *mut Expr = pExpr;
        let __v1102: u32 = unsafe { (*__v1101).flags };
        let __v1103: u32 = __v1102 | ((64 as i32) as u32);
        unsafe {
            (*__v1101).flags = __v1103;
        }
    }
    return 0 as i32;
}

/// If the SELECT references the table pWalker->u.pTab, then do two things:
///
///    (1) Mark the SELECT as as SF_Correlated.
///    (2) Set pWalker->eCode to non-zero so that the caller will know
///        that (1) has happened.
#[unsafe(link_section = ".text.slate_distinct.trigger.sqlite3ReturningSubqueryCorrelated")]
extern "C-unwind" fn sqlite3ReturningSubqueryCorrelated(
    mut pWalker: *mut Walker,
    mut pSelect: *mut Select,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut pSrc: *mut SrcList = unsafe { std::mem::zeroed() };
    0 as i32;
    pSrc = unsafe { (*pSelect).pSrc };
    0 as i32;
    i = 0 as i32;
    '__slate_break_1032: loop {
        if !(i < unsafe { (*pSrc).nSrc }) {
            break;
        }
        if (unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset(i as isize)
            })
            .pSTab
        }) == unsafe { (*pWalker).u.pTab }
        {
            {}
            let __v1106: *mut Select = pSelect;
            let __v1107: u32 = unsafe { (*__v1106).selFlags };
            let __v1108: u32 = __v1107 | ((536870912 as i32) as u32);
            unsafe {
                (*__v1106).selFlags = __v1108;
            }
            unsafe {
                (*pWalker).eCode = ((1 as i32) as i16) as u16;
            }
            break '__slate_break_1032;
        }
        let __v1104: i32 = i;
        let __v1105: i32 = __v1104 + (1 as i32);
        i = __v1105;
    }
    return 0 as i32;
}

/// Scan the expression list that is the argument to RETURNING looking
/// for subqueries that depend on the table which is being modified in the
/// statement that is hosting the RETURNING clause (pTab).  Mark all such
/// subqueries as SF_Correlated.  If the subqueries are part of an
/// expression, mark the expression as EP_VarSelect.
///
/// https://sqlite.org/forum/forumpost/2c83569ce8945d39
fn sqlite3ProcessReturningSubqueries(mut pEList: *mut ExprList, mut pTab: *mut Table) {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    unsafe { memset(std::ptr::addr_of_mut!(w) as *mut (), 0 as i32, 48 as u64) };
    w.xExprCallback = unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Expr) -> i32>,
        >(sqlite3ExprWalkNoop as *const ())
    };
    w.xSelectCallback = Some(sqlite3ReturningSubqueryCorrelated);
    unsafe {
        w.u.pTab = pTab;
    }
    unsafe { sqlite3WalkExprList(std::ptr::addr_of_mut!(w), pEList) };
    if w.eCode != (0 as u16) {
        w.xExprCallback = Some(sqlite3ReturningSubqueryVarSelect);
        w.xSelectCallback = unsafe {
            std::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
            >(sqlite3SelectWalkNoop as *const ())
        };
        unsafe { sqlite3WalkExprList(std::ptr::addr_of_mut!(w), pEList) };
    }
}

/// Generate code for the RETURNING trigger.  Unlike other triggers
/// that invoke a subprogram in the bytecode, the code for RETURNING
/// is generated in-line.
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `pTrigger` - The trigger step that defines the RETURNING
/// * `pTab` - The table to code triggers from
/// * `regIn` - The first in an array of registers
fn codeReturningTrigger(
    mut pParse: *mut Parse,
    mut pTrigger: *mut Trigger,
    mut pTab: *mut Table,
    mut regIn: i32,
) {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pNew: *mut ExprList = unsafe { std::mem::zeroed() };
    let mut pReturning: *mut Returning = unsafe { std::mem::zeroed() };
    let mut sSelect: Select = unsafe { std::mem::zeroed() };
    let mut pFrom: *mut SrcList = unsafe { std::mem::zeroed() };
    let mut uSrc: __SlateRecord204 = unsafe { std::mem::zeroed() };
    0 as i32;
    if !(((unsafe { (*pParse).__slate_bits_0.__get_bReturning() }) as i32) != (0 as i32)) {
        // This RETURNING trigger must be for a different statement as
        // this statement lacks a RETURNING clause.
        return;
    }
    0 as i32;
    0 as i32;
    pReturning = unsafe { (*pParse).u1.d.pReturning };
    if pTrigger != unsafe { std::ptr::addr_of_mut!((*pReturning).retTrig) } {
        // This RETURNING trigger is for a different statement
        return;
    }
    unsafe {
        memset(
            std::ptr::addr_of_mut!(sSelect) as *mut (),
            0 as i32,
            120 as u64,
        )
    };
    unsafe { memset(std::ptr::addr_of_mut!(uSrc) as *mut (), 0 as i32, 80 as u64) };
    pFrom = unsafe { std::ptr::addr_of_mut!(uSrc.sSrc) };
    sSelect.pEList = unsafe {
        sqlite3ExprListDup(
            db,
            (unsafe { (*pReturning).pReturnEL }) as *const ExprList,
            0 as i32,
        )
    };
    sSelect.pSrc = pFrom;
    unsafe {
        (*pFrom).nSrc = 1 as i32;
    }
    unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pFrom).a) as *mut SrcItem }
                .offset((0 as i32) as isize)
        })
        .pSTab = pTab;
    }
    unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pFrom).a) as *mut SrcItem }
                .offset((0 as i32) as isize)
        })
        .zName = unsafe { (*pTab).zName };
    }
    // tag-20240424-1
    unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pFrom).a) as *mut SrcItem }
                .offset((0 as i32) as isize)
        })
        .iCursor = -(1 as i32);
    }
    unsafe {
        sqlite3SelectPrep(
            pParse,
            std::ptr::addr_of_mut!(sSelect),
            std::ptr::null_mut::<NameContext>(),
        )
    };
    if (unsafe { (*pParse).nErr }) == (0 as i32) {
        0 as i32;
        unsafe { sqlite3GenerateColumnNames(pParse, std::ptr::addr_of_mut!(sSelect)) };
    }
    unsafe { sqlite3ExprListDelete(db, sSelect.pEList) };
    pNew = sqlite3ExpandReturning(pParse, unsafe { (*pReturning).pReturnEL }, pTab);
    if (unsafe { (*pParse).nErr }) == (0 as i32) {
        let mut sNC: NameContext = unsafe { std::mem::zeroed() };
        unsafe { memset(std::ptr::addr_of_mut!(sNC) as *mut (), 0 as i32, 56 as u64) };
        if (unsafe { (*pReturning).nRetCol }) == (0 as i32) {
            unsafe {
                (*pReturning).nRetCol = unsafe { (*pNew).nExpr };
            }
            let __v1109: *mut Parse = pParse;
            let __v1110: i32 = unsafe { (*__v1109).nTab };
            let __v1111: i32 = __v1110 + (1 as i32);
            unsafe {
                (*__v1109).nTab = __v1111;
            }
            unsafe {
                (*pReturning).iRetCur = __v1110;
            }
        }
        sNC.pParse = pParse;
        unsafe {
            sNC.uNC.iBaseReg = regIn;
        }
        sNC.ncFlags = 1024 as i32;
        unsafe {
            (*pParse).eTriggerOp = unsafe { (*pTrigger).op };
        }
        unsafe {
            (*pParse).pTriggerTab = pTab;
        }
        if (unsafe { sqlite3ResolveExprListNames(std::ptr::addr_of_mut!(sNC), pNew) }) == (0 as i32)
            && !((unsafe { (*db).mallocFailed }) != (0 as u8))
        {
            let mut i: i32 = 0 as i32;
            let mut nCol: i32 = unsafe { (*pNew).nExpr };
            let mut reg: i32 = (unsafe { (*pParse).nMem }) + (1 as i32);
            sqlite3ProcessReturningSubqueries(pNew, pTab);
            let __v1112: *mut Parse = pParse;
            let __v1113: i32 = unsafe { (*__v1112).nMem };
            let __v1114: i32 = __v1113 + (nCol + (2 as i32));
            unsafe {
                (*__v1112).nMem = __v1114;
            }
            unsafe {
                (*pReturning).iRetReg = reg;
            }
            i = 0 as i32;
            '__slate_break_1033: loop {
                if !(i < nCol) {
                    break;
                }
                let mut pCol: *mut Expr = unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pNew).a) as *mut ExprList_item }
                            .offset(i as isize)
                    })
                    .pExpr
                };
                0 as i32; // Due to !db->mallocFailed ~9 lines above
                unsafe { sqlite3ExprCodeFactorable(pParse, pCol, reg + i) };
                if ((unsafe { sqlite3ExprAffinity(pCol as *const Expr) }) as i32) == (69 as i32) {
                    unsafe { sqlite3VdbeAddOp1(v, 89 as i32, reg + i) };
                }
                let __v1115: i32 = i;
                let __v1116: i32 = __v1115 + (1 as i32);
                i = __v1116;
            }
            unsafe { sqlite3VdbeAddOp3(v, 99 as i32, reg, i, reg + i) };
            unsafe {
                sqlite3VdbeAddOp2(
                    v,
                    129 as i32,
                    unsafe { (*pReturning).iRetCur },
                    reg + i + (1 as i32),
                )
            };
            unsafe {
                sqlite3VdbeAddOp3(
                    v,
                    130 as i32,
                    unsafe { (*pReturning).iRetCur },
                    reg + i,
                    reg + i + (1 as i32),
                )
            };
        }
    }
    unsafe { sqlite3ExprListDelete(db, pNew) };
    unsafe {
        (*pParse).eTriggerOp = ((0 as i32) as i8) as u8;
    }
    unsafe {
        (*pParse).pTriggerTab = std::ptr::null_mut::<Table>();
    }
}

/// Generate VDBE code for the statements inside the body of a single
/// trigger.
///
/// # Arguments
///
/// * `pParse` - The parser context
/// * `pStepList` - List of statements inside the trigger body
/// * `orconf` - Conflict algorithm. (OE_Abort, etc)
fn codeTriggerProgram(
    mut pParse: *mut Parse,
    mut pStepList: *mut TriggerStep,
    mut orconf: i32,
) -> i32 {
    let mut pStep: *mut TriggerStep = unsafe { std::mem::zeroed() };
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    0 as i32;
    0 as i32;
    0 as i32;
    pStep = pStepList;
    '__slate_break_1034: while pStep != std::ptr::null_mut::<TriggerStep>() {
        // Figure out the ON CONFLICT policy that will be used for this step
        // of the trigger program. If the statement that caused this trigger
        // to fire had an explicit ON CONFLICT, then use it. Otherwise, use
        // the ON CONFLICT policy that was specified as part of the trigger
        // step statement. Example:
        //
        //   CREATE TRIGGER AFTER INSERT ON t1 BEGIN;
        //     INSERT OR REPLACE INTO t2 VALUES(new.a, new.b);
        //   END;
        //
        //   INSERT INTO t1 ... ;            -- insert into t2 uses REPLACE policy
        //   INSERT OR IGNORE INTO t1 ... ;  -- insert into t2 uses IGNORE policy
        unsafe {
            (*pParse).eOrconf = ((if orconf == (11 as i32) {
                ((unsafe { (*pStep).orconf }) as u32) as i32
            } else {
                (((orconf as i8) as u8) as u32) as i32
            }) as i8) as u8;
        }
        0 as i32;
        if (unsafe { (*pStep).zSpan }) != std::ptr::null_mut::<i8>() {
            unsafe {
                sqlite3VdbeAddOp4(
                    v,
                    186 as i32,
                    2147483647 as i32,
                    1 as i32,
                    0 as i32,
                    (unsafe {
                        sqlite3MPrintf(db, (b"-- %s\0".as_ptr() as *mut i8) as *const i8, unsafe {
                            (*pStep).zSpan
                        })
                    }) as *const i8,
                    -(7 as i32),
                )
            };
        }
        '__slate_break_1036: {
            match ((unsafe { (*pStep).op }) as u32) as i32 {
                130 => {
                    unsafe {
                        sqlite3Update(
                            pParse,
                            unsafe {
                                sqlite3SrcListDup(
                                    db,
                                    (unsafe { (*pStep).pSrc }) as *const SrcList,
                                    0 as i32,
                                )
                            },
                            unsafe {
                                sqlite3ExprListDup(
                                    db,
                                    (unsafe { (*pStep).pExprList }) as *const ExprList,
                                    0 as i32,
                                )
                            },
                            unsafe {
                                sqlite3ExprDup(
                                    db,
                                    (unsafe { (*pStep).pWhere }) as *const Expr,
                                    0 as i32,
                                )
                            },
                            ((unsafe { (*pParse).eOrconf }) as u32) as i32,
                            std::ptr::null_mut::<ExprList>(),
                            std::ptr::null_mut::<Expr>(),
                            std::ptr::null_mut::<Upsert>(),
                        )
                    };
                    unsafe { sqlite3VdbeAddOp0(v, 133 as i32) };
                }
                128 => {
                    unsafe {
                        sqlite3Insert(
                            pParse,
                            unsafe {
                                sqlite3SrcListDup(
                                    db,
                                    (unsafe { (*pStep).pSrc }) as *const SrcList,
                                    0 as i32,
                                )
                            },
                            unsafe {
                                sqlite3SelectDup(
                                    db,
                                    (unsafe { (*pStep).pSelect }) as *const Select,
                                    0 as i32,
                                )
                            },
                            unsafe {
                                sqlite3IdListDup(db, (unsafe { (*pStep).pIdList }) as *const IdList)
                            },
                            ((unsafe { (*pParse).eOrconf }) as u32) as i32,
                            unsafe { sqlite3UpsertDup(db, unsafe { (*pStep).pUpsert }) },
                        )
                    };
                    unsafe { sqlite3VdbeAddOp0(v, 133 as i32) };
                }
                129 => {
                    unsafe {
                        sqlite3DeleteFrom(
                            pParse,
                            unsafe {
                                sqlite3SrcListDup(
                                    db,
                                    (unsafe { (*pStep).pSrc }) as *const SrcList,
                                    0 as i32,
                                )
                            },
                            unsafe {
                                sqlite3ExprDup(
                                    db,
                                    (unsafe { (*pStep).pWhere }) as *const Expr,
                                    0 as i32,
                                )
                            },
                            std::ptr::null_mut::<ExprList>(),
                            std::ptr::null_mut::<Expr>(),
                        )
                    };
                    unsafe { sqlite3VdbeAddOp0(v, 133 as i32) };
                }
                _ => {
                    0 as i32;
                    let mut sDest: SelectDest = unsafe { std::mem::zeroed() };
                    let mut pSelect: *mut Select = unsafe {
                        sqlite3SelectDup(
                            db,
                            (unsafe { (*pStep).pSelect }) as *const Select,
                            0 as i32,
                        )
                    };
                    unsafe {
                        sqlite3SelectDestInit(std::ptr::addr_of_mut!(sDest), 2 as i32, 0 as i32)
                    };
                    unsafe { sqlite3Select(pParse, pSelect, std::ptr::addr_of_mut!(sDest)) };
                    unsafe { sqlite3SelectDelete(db, pSelect) };
                }
            }
        }
        pStep = unsafe { (*pStep).pNext };
    }
    return 0 as i32;
}

/// This function is used to add VdbeComment() annotations to a VDBE
/// program. It is not used in production code, only for debugging.
fn onErrorText(mut onError: i32) -> *const i8 {
    match onError {
        2 => {
            return (b"abort\0".as_ptr() as *mut i8) as *const i8;
        }
        1 => {
            return (b"rollback\0".as_ptr() as *mut i8) as *const i8;
        }
        3 => {
            return (b"fail\0".as_ptr() as *mut i8) as *const i8;
        }
        5 => {
            return (b"replace\0".as_ptr() as *mut i8) as *const i8;
        }
        4 => {
            return (b"ignore\0".as_ptr() as *mut i8) as *const i8;
        }
        11 => {
            return (b"default\0".as_ptr() as *mut i8) as *const i8;
        }
        _ => {}
    }
    return (b"n/a\0".as_ptr() as *mut i8) as *const i8;
}

/// Parse context structure pFrom has just been used to create a sub-vdbe
/// (trigger program). If an error has occurred, transfer error information
/// from pFrom to pTo.
fn transferParseError(mut pTo: *mut Parse, mut pFrom: *mut Parse) {
    0 as i32;
    0 as i32;
    if (unsafe { (*pTo).nErr }) == (0 as i32) {
        unsafe {
            (*pTo).zErrMsg = unsafe { (*pFrom).zErrMsg };
        }
        unsafe {
            (*pTo).nErr = unsafe { (*pFrom).nErr };
        }
        unsafe {
            (*pTo).rc = unsafe { (*pFrom).rc };
        }
    } else {
        unsafe {
            sqlite3DbFree(
                unsafe { (*pFrom).db },
                (unsafe { (*pFrom).zErrMsg }) as *mut (),
            )
        };
    }
}

/// Create and populate a new TriggerPrg object with a sub-program
/// implementing trigger pTrigger with ON CONFLICT policy orconf.
///
/// # Arguments
///
/// * `pParse` - Current parse context
/// * `pTrigger` - Trigger to code
/// * `pTab` - The table pTrigger is attached to
/// * `orconf` - ON CONFLICT policy to code trigger program with
fn codeRowTrigger(
    mut pParse: *mut Parse,
    mut pTrigger: *mut Trigger,
    mut pTab: *mut Table,
    mut orconf: i32,
) -> *mut TriggerPrg {
    let mut pTop: *mut Parse = unsafe { std::mem::zeroed() }; // Top level Parse object
    let mut db: *mut sqlite3 = unsafe { (*pParse).db }; // Database handle
    let mut pPrg: *mut TriggerPrg = unsafe { std::mem::zeroed() }; // Value to return
    let mut pWhen: *mut Expr = std::ptr::null_mut::<Expr>(); // Duplicate of trigger WHEN expression
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() }; // Temporary VM
    let mut sNC: NameContext = unsafe { std::mem::zeroed() }; // Name context for sub-vdbe
    let mut pProgram: *mut SubProgram = std::ptr::null_mut::<SubProgram>(); // Sub-vdbe for trigger program
    let mut iEndTrigger: i32 = 0 as i32; // Label to jump to if WHEN is false
    let mut sSubParse: Parse = unsafe { std::mem::zeroed() }; // Parse context for sub-vdbe
    let mut nDepth: i32 = 0 as i32; // Trigger depth
    // Ensure that triggers are not chained too deep.  This test is linear
    // in the chaining depth, but sensible code ought not be chaining
    // triggers excessively, so that shouldn't be a problem.
    pTop = pParse;
    nDepth = 0 as i32;
    '__slate_break_1045: loop {
        if !((unsafe { (*pTop).pOuterParse }) != std::ptr::null_mut::<Parse>()) {
            break;
        }
        pTop = unsafe { (*pTop).pOuterParse };
        let __v1117: i32 = nDepth;
        let __v1118: i32 = __v1117 + (1 as i32);
        nDepth = __v1118;
    }
    if nDepth
        >= unsafe {
            *unsafe {
                unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((10 as i32) as isize)
            }
        }
    {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"triggers nested too deep\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        return std::ptr::null_mut::<TriggerPrg>();
    }
    pTop = if (unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>() {
        unsafe { (*pParse).pToplevel }
    } else {
        pParse
    };
    0 as i32;
    0 as i32;
    // Allocate the TriggerPrg and SubProgram objects. To ensure that they
    // are freed if an error occurs, link them into the Parse.pTriggerPrg
    // list of the top-level Parse object sooner rather than later.
    pPrg = (unsafe { sqlite3DbMallocZero(db, 40 as u64) }) as *mut TriggerPrg;
    if !(pPrg != std::ptr::null_mut::<TriggerPrg>()) {
        return std::ptr::null_mut::<TriggerPrg>();
    }
    unsafe {
        (*pPrg).pNext = unsafe { (*pTop).pTriggerPrg };
    }
    unsafe {
        (*pTop).pTriggerPrg = pPrg;
    }
    let __v1119: *mut SubProgram =
        (unsafe { sqlite3DbMallocZero(db, 48 as u64) }) as *mut SubProgram;
    pProgram = __v1119;
    unsafe {
        (*pPrg).pProgram = __v1119;
    }
    if !(pProgram != std::ptr::null_mut::<SubProgram>()) {
        return std::ptr::null_mut::<TriggerPrg>();
    }
    unsafe { sqlite3VdbeLinkSubProgram(unsafe { (*pTop).pVdbe }, pProgram) };
    unsafe {
        (*pPrg).pTrigger = pTrigger;
    }
    unsafe {
        (*pPrg).orconf = orconf;
    }
    unsafe {
        *unsafe {
            unsafe { (*pPrg).aColmask.as_mut_ptr() as *mut u32 }.offset((0 as i32) as isize)
        } = 4294967295 as u32;
    }
    unsafe {
        *unsafe {
            unsafe { (*pPrg).aColmask.as_mut_ptr() as *mut u32 }.offset((1 as i32) as isize)
        } = 4294967295 as u32;
    }
    // Allocate and populate a new Parse context to use for coding the
    // trigger sub-program.
    unsafe { sqlite3ParseObjectInit(std::ptr::addr_of_mut!(sSubParse), db) };
    unsafe { memset(std::ptr::addr_of_mut!(sNC) as *mut (), 0 as i32, 56 as u64) };
    sNC.pParse = std::ptr::addr_of_mut!(sSubParse);
    sSubParse.pTriggerTab = pTab;
    sSubParse.pToplevel = pTop;
    sSubParse.zAuthContext = (unsafe { (*pTrigger).zName }) as *const i8;
    sSubParse.eTriggerOp = unsafe { (*pTrigger).op };
    sSubParse.nQueryLoop = unsafe { (*pParse).nQueryLoop };
    sSubParse.prepFlags = unsafe { (*pParse).prepFlags };
    sSubParse.oldmask = (0 as i32) as u32;
    sSubParse.newmask = (0 as i32) as u32;
    v = unsafe { sqlite3GetVdbe(std::ptr::addr_of_mut!(sSubParse)) };
    if v != std::ptr::null_mut::<Vdbe>() {
        unsafe {
            sqlite3VdbeComment(
                v,
                (b"Start: %s.%s (%s %s%s%s ON %s)\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pTrigger).zName },
                onErrorText(orconf),
                if (((unsafe { (*pTrigger).tr_tm }) as u32) as i32) == (1 as i32) {
                    b"BEFORE\0".as_ptr() as *mut i8
                } else {
                    b"AFTER\0".as_ptr() as *mut i8
                },
                if (((unsafe { (*pTrigger).op }) as u32) as i32) == (130 as i32) {
                    b"UPDATE\0".as_ptr() as *mut i8
                } else {
                    b"\0".as_ptr() as *mut i8
                },
                if (((unsafe { (*pTrigger).op }) as u32) as i32) == (128 as i32) {
                    b"INSERT\0".as_ptr() as *mut i8
                } else {
                    b"\0".as_ptr() as *mut i8
                },
                if (((unsafe { (*pTrigger).op }) as u32) as i32) == (129 as i32) {
                    b"DELETE\0".as_ptr() as *mut i8
                } else {
                    b"\0".as_ptr() as *mut i8
                },
                unsafe { (*pTab).zName },
            )
        };
        if (unsafe { (*pTrigger).zName }) != std::ptr::null_mut::<i8>() {
            unsafe {
                sqlite3VdbeChangeP4(
                    v,
                    -(1 as i32),
                    (unsafe {
                        sqlite3MPrintf(
                            db,
                            (b"-- TRIGGER %s\0".as_ptr() as *mut i8) as *const i8,
                            unsafe { (*pTrigger).zName },
                        )
                    }) as *const i8,
                    -(7 as i32),
                )
            };
        }
        // If one was specified, code the WHEN clause. If it evaluates to false
        // (or NULL) the sub-vdbe is immediately halted by jumping to the
        // OP_Halt inserted at the end of the program.
        if (unsafe { (*pTrigger).pWhen }) != std::ptr::null_mut::<Expr>() {
            pWhen = unsafe {
                sqlite3ExprDup(db, (unsafe { (*pTrigger).pWhen }) as *const Expr, 0 as i32)
            };
            let __v1120: bool;
            if (((unsafe { (*db).mallocFailed }) as u32) as i32) == (0 as i32) {
                __v1120 = (0 as i32)
                    == unsafe { sqlite3ResolveExprNames(std::ptr::addr_of_mut!(sNC), pWhen) };
            } else {
                __v1120 = false as bool;
            }
            if __v1120 {
                iEndTrigger = unsafe { sqlite3VdbeMakeLabel(std::ptr::addr_of_mut!(sSubParse)) };
                unsafe {
                    sqlite3ExprIfFalse(
                        std::ptr::addr_of_mut!(sSubParse),
                        pWhen,
                        iEndTrigger,
                        16 as i32,
                    )
                };
            }
            unsafe { sqlite3ExprDelete(db, pWhen) };
        }
        // Code the trigger program into the sub-vdbe.
        codeTriggerProgram(
            std::ptr::addr_of_mut!(sSubParse),
            unsafe { (*pTrigger).step_list },
            orconf,
        );
        // Insert an OP_Halt at the end of the sub-program.
        if iEndTrigger != (0 as i32) {
            unsafe { sqlite3VdbeResolveLabel(v, iEndTrigger) };
        }
        unsafe { sqlite3VdbeAddOp0(v, 72 as i32) };
        unsafe {
            sqlite3VdbeComment(
                v,
                (b"End: %s.%s\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pTrigger).zName },
                onErrorText(orconf),
            )
        };
        transferParseError(pParse, std::ptr::addr_of_mut!(sSubParse));
        if (unsafe { (*pParse).nErr }) == (0 as i32) {
            0 as i32;
            unsafe {
                (*pProgram).aOp = unsafe {
                    sqlite3VdbeTakeOpArray(
                        v,
                        unsafe { std::ptr::addr_of_mut!((*pProgram).nOp) },
                        unsafe { std::ptr::addr_of_mut!((*pTop).nMaxArg) },
                    )
                };
            }
        }
        unsafe {
            (*pProgram).nMem = sSubParse.nMem;
        }
        unsafe {
            (*pProgram).nCsr = sSubParse.nTab;
        }
        unsafe {
            (*pProgram).token = pTrigger as *mut ();
        }
        unsafe {
            *unsafe {
                unsafe { (*pPrg).aColmask.as_mut_ptr() as *mut u32 }.offset((0 as i32) as isize)
            } = sSubParse.oldmask;
        }
        unsafe {
            *unsafe {
                unsafe { (*pPrg).aColmask.as_mut_ptr() as *mut u32 }.offset((1 as i32) as isize)
            } = sSubParse.newmask;
        }
        unsafe { sqlite3VdbeDelete(v) };
    } else {
        transferParseError(pParse, std::ptr::addr_of_mut!(sSubParse));
    }
    0 as i32;
    unsafe { sqlite3ParseObjectReset(std::ptr::addr_of_mut!(sSubParse)) };
    return pPrg;
}

/// Return a pointer to a TriggerPrg object containing the sub-program for
/// trigger pTrigger with default ON CONFLICT algorithm orconf. If no such
/// TriggerPrg object exists, a new object is allocated and populated before
/// being returned.
///
/// # Arguments
///
/// * `pParse` - Current parse context
/// * `pTrigger` - Trigger to code
/// * `pTab` - The table trigger pTrigger is attached to
/// * `orconf` - ON CONFLICT algorithm.
fn getRowTrigger(
    mut pParse: *mut Parse,
    mut pTrigger: *mut Trigger,
    mut pTab: *mut Table,
    mut orconf: i32,
) -> *mut TriggerPrg {
    let mut pRoot: *mut Parse = if (unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>()
    {
        unsafe { (*pParse).pToplevel }
    } else {
        pParse
    };
    let mut pPrg: *mut TriggerPrg = unsafe { std::mem::zeroed() };
    0 as i32;
    // It may be that this trigger has already been coded (or is in the
    // process of being coded). If this is the case, then an entry with
    // a matching TriggerPrg.pTrigger field will be present somewhere
    // in the Parse.pTriggerPrg list. Search for such an entry.
    pPrg = unsafe { (*pRoot).pTriggerPrg };
    '__slate_break_1058: while pPrg != std::ptr::null_mut::<TriggerPrg>()
        && ((unsafe { (*pPrg).pTrigger }) != pTrigger || (unsafe { (*pPrg).orconf }) != orconf)
    {
        {}
        pPrg = unsafe { (*pPrg).pNext };
    }
    // If an existing TriggerPrg could not be located, create a new one.
    if !(pPrg != std::ptr::null_mut::<TriggerPrg>()) {
        pPrg = codeRowTrigger(pParse, pTrigger, pTab, orconf);
        unsafe {
            (*unsafe { (*pParse).db }).errByteOffset = -(1 as i32);
        }
    }
    return pPrg;
}

/// Generate code for the trigger program associated with trigger p on
/// table pTab. The reg, orconf and ignoreJump parameters passed to this
/// function are the same as those described in the header function for
/// sqlite3CodeRowTrigger()
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `p` - Trigger to code
/// * `pTab` - The table to code triggers from
/// * `reg` - Reg array containing OLD.* and NEW.* values
/// * `orconf` - ON CONFLICT policy
/// * `ignoreJump` - Instruction to jump to for RAISE(IGNORE)
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CodeRowTriggerDirect(
    mut pParse: *mut Parse,
    mut p: *mut Trigger,
    mut pTab: *mut Table,
    mut reg: i32,
    mut orconf: i32,
    mut ignoreJump: i32,
) {
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) }; // Main VM
    let mut pPrg: *mut TriggerPrg = unsafe { std::mem::zeroed() };
    pPrg = getRowTrigger(pParse, p, pTab, orconf);
    0 as i32;
    // Code the OP_Program opcode in the parent VDBE. P4 of the OP_Program
    // is a pointer to the sub-vdbe containing the trigger program.
    if pPrg != std::ptr::null_mut::<TriggerPrg>() {
        let mut bRecursive: i32 = ((unsafe { (*p).zName }) != std::ptr::null_mut::<i8>()
            && (((0 as i32) as i64) as u64)
                == (unsafe { (*unsafe { (*pParse).db }).flags }) & (((8192 as i32) as i64) as u64))
            as i32;
        let __v1076: *mut Parse = pParse;
        let __v1077: i32 = unsafe { (*__v1076).nMem };
        let __v1078: i32 = __v1077 + (1 as i32);
        unsafe {
            (*__v1076).nMem = __v1078;
        }
        unsafe {
            sqlite3VdbeAddOp4(
                v,
                50 as i32,
                reg,
                ignoreJump,
                __v1078,
                (unsafe { (*pPrg).pProgram }) as *const i8,
                -(4 as i32),
            )
        };
        unsafe {
            sqlite3VdbeComment(
                v,
                (b"Call: %s.%s\0".as_ptr() as *mut i8) as *const i8,
                if (unsafe { (*p).zName }) != std::ptr::null_mut::<i8>() {
                    unsafe { (*p).zName }
                } else {
                    b"fkey\0".as_ptr() as *mut i8
                },
                onErrorText(orconf),
            )
        };
        // Set the P5 operand of the OP_Program instruction to non-zero if
        // recursive invocation of this trigger program is disallowed. Recursive
        // invocation is disallowed if (a) the sub-program is really a trigger,
        // not a foreign key action, and (b) the flag to enable recursive triggers
        // is clear.
        unsafe { sqlite3VdbeChangeP5(v, (bRecursive as i16) as u16) };
    }
}

/// This is called to code the required FOR EACH ROW triggers for an operation
/// on table pTab. The operation to code triggers for (INSERT, UPDATE or DELETE)
/// is given by the op parameter. The tr_tm parameter determines whether the
/// BEFORE or AFTER triggers are coded. If the operation is an UPDATE, then
/// parameter pChanges is passed the list of columns being modified.
///
/// If there are no triggers that fire at the specified time for the specified
/// operation on pTab, this function is a no-op.
///
/// The reg argument is the address of the first in an array of registers
/// that contain the values substituted for the new.* and old.* references
/// in the trigger program. If N is the number of columns in table pTab
/// (a copy of pTab->nCol), then registers are populated as follows:
///
///   Register       Contains
///   reg+0          OLD.rowid
///   reg+1          OLD.* value of left-most column of pTab
///   ...            ...
///   reg+N          OLD.* value of right-most column of pTab
///   reg+N+1        NEW.rowid
///   reg+N+2        NEW.* value of left-most column of pTab
///   ...            ...
///   reg+N+N+1      NEW.* value of right-most column of pTab
///
/// For ON DELETE triggers, the registers containing the NEW.* values will
/// never be accessed by the trigger program, so they are not allocated or
/// populated by the caller (there is no data to populate them with anyway).
/// Similarly, for ON INSERT triggers the values stored in the OLD.* registers
/// are never accessed, and so are not allocated by the caller. So, for an
/// ON INSERT trigger, the value passed to this function as parameter reg
/// is not a readable register, although registers (reg+N) through
/// (reg+N+N+1) are.
///
/// Parameter orconf is the default conflict resolution algorithm for the
/// trigger program to use (REPLACE, IGNORE etc.). Parameter ignoreJump
/// is the instruction that control should jump to if a trigger program
/// raises an IGNORE exception.
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `pTrigger` - List of triggers on table pTab
/// * `op` - One of TK_UPDATE, TK_INSERT, TK_DELETE
/// * `pChanges` - Changes list for any UPDATE OF triggers
/// * `tr_tm` - One of TRIGGER_BEFORE, TRIGGER_AFTER
/// * `pTab` - The table to code triggers from
/// * `reg` - The first in an array of registers (see above)
/// * `orconf` - ON CONFLICT policy
/// * `ignoreJump` - Instruction to jump to for RAISE(IGNORE)
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CodeRowTrigger(
    mut pParse: *mut Parse,
    mut pTrigger: *mut Trigger,
    mut op: i32,
    mut pChanges: *mut ExprList,
    mut tr_tm: i32,
    mut pTab: *mut Table,
    mut reg: i32,
    mut orconf: i32,
    mut ignoreJump: i32,
) {
    let mut p: *mut Trigger = unsafe { std::mem::zeroed() }; // Used to iterate through pTrigger list
    0 as i32;
    0 as i32;
    0 as i32;
    p = pTrigger;
    '__slate_break_1061: while p != std::ptr::null_mut::<Trigger>() {
        // Sanity checking:  The schema for the trigger and for the table are
        // always defined.  The trigger must be in the same schema as the table
        // or else it must be a TEMP trigger.
        0 as i32;
        0 as i32;
        0 as i32;
        // Determine whether we should code this trigger.  One of two choices:
        // 1. The trigger is an exact match to the current DML statement
        // 2. This is a RETURNING trigger for INSERT but we are currently
        //    doing the UPDATE part of an UPSERT.
        let __v1075: bool;
        if ((((unsafe { (*p).op }) as u32) as i32) == op
            || (unsafe { (*p).bReturning }) != (0 as u8)
                && (((unsafe { (*p).op }) as u32) as i32) == (128 as i32)
                && op == (130 as i32))
            && (((unsafe { (*p).tr_tm }) as u32) as i32) == tr_tm
        {
            __v1075 = checkColumnOverlap(unsafe { (*p).pColumns }, pChanges) != (0 as i32);
        } else {
            __v1075 = false as bool;
        }
        if __v1075 {
            if !((unsafe { (*p).bReturning }) != (0 as u8)) {
                sqlite3CodeRowTriggerDirect(pParse, p, pTab, reg, orconf, ignoreJump);
            } else {
                if (unsafe { (*pParse).pToplevel }) == std::ptr::null_mut::<Parse>() {
                    codeReturningTrigger(pParse, p, pTab, reg);
                }
            }
        }
        p = unsafe { (*p).pNext };
    }
}

/// Triggers may access values stored in the old.* or new.* pseudo-table.
/// This function returns a 32-bit bitmask indicating which columns of the
/// old.* or new.* tables actually are used by triggers. This information
/// may be used by the caller, for example, to avoid having to load the entire
/// old.* record into memory when executing an UPDATE or DELETE command.
///
/// Bit 0 of the returned mask is set if the left-most column of the
/// table may be accessed using an [old|new].<col> reference. Bit 1 is set if
/// the second leftmost column value is required, and so on. If there
/// are more than 32 columns in the table, and at least one of the columns
/// with an index greater than 32 may be accessed, 0xffffffff is returned.
///
/// It is not possible to determine if the old.rowid or new.rowid column is
/// accessed by triggers. The caller must always assume that it is.
///
/// Parameter isNew must be either 1 or 0. If it is 0, then the mask returned
/// applies to the old.* table. If 1, the new.* table.
///
/// Parameter tr_tm must be a mask with one or both of the TRIGGER_BEFORE
/// and TRIGGER_AFTER bits set. Values accessed by BEFORE triggers are only
/// included in the returned mask if the TRIGGER_BEFORE bit is set in the
/// tr_tm parameter. Similarly, values accessed by AFTER triggers are only
/// included in the returned mask if the TRIGGER_AFTER bit is set in tr_tm.
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `pTrigger` - List of triggers on table pTab
/// * `pChanges` - Changes list for any UPDATE OF triggers
/// * `isNew` - 1 for new.* ref mask, 0 for old.* ref mask
/// * `tr_tm` - Mask of TRIGGER_BEFORE|TRIGGER_AFTER
/// * `pTab` - The table to code triggers from
/// * `orconf` - Default ON CONFLICT policy for trigger steps
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TriggerColmask(
    mut pParse: *mut Parse,
    mut pTrigger: *mut Trigger,
    mut pChanges: *mut ExprList,
    mut isNew: i32,
    mut tr_tm: i32,
    mut pTab: *mut Table,
    mut orconf: i32,
) -> u32 {
    let mut op: i32 = if pChanges != std::ptr::null_mut::<ExprList>() {
        130 as i32
    } else {
        129 as i32
    };
    let mut mask: u32 = (0 as i32) as u32;
    let mut p: *mut Trigger = unsafe { std::mem::zeroed() };
    0 as i32;
    if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (2 as i32) {
        return 4294967295 as u32;
    }
    p = pTrigger;
    '__slate_break_1062: while p != std::ptr::null_mut::<Trigger>() {
        let __v1082: bool;
        if (((unsafe { (*p).op }) as u32) as i32) == op
            && tr_tm & (((unsafe { (*p).tr_tm }) as u32) as i32) != (0 as i32)
        {
            __v1082 = checkColumnOverlap(unsafe { (*p).pColumns }, pChanges) != (0 as i32);
        } else {
            __v1082 = false as bool;
        }
        if __v1082 {
            if (unsafe { (*p).bReturning }) != (0 as u8) {
                mask = 4294967295 as u32;
            } else {
                let mut pPrg: *mut TriggerPrg = unsafe { std::mem::zeroed() };
                pPrg = getRowTrigger(pParse, p, pTab, orconf);
                if pPrg != std::ptr::null_mut::<TriggerPrg>() {
                    let __v1083: u32 = mask;
                    let __v1084: u32 = __v1083
                        | unsafe {
                            *unsafe {
                                unsafe { (*pPrg).aColmask.as_mut_ptr() as *mut u32 }
                                    .offset(isNew as isize)
                            }
                        };
                    mask = __v1084;
                }
            }
        }
        p = unsafe { (*p).pNext };
    }
    return mask;
}
