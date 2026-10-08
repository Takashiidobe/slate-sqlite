//! 2002 February 23
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This file contains the C-language implementations for many of the SQL
//! functions of SQLite.  (Some function, and in particular the date and
//! time functions, are implemented separately.)
unsafe extern "C" {
    static mut sqlite3UpperToLower: [u8; 0];
    static mut sqlite3CtypeMap: [u8; 0];
    fn sqlite3_libversion() -> *const i8;
    fn sqlite3_sourceid() -> *const i8;
    fn sqlite3_compileoption_used(zOptName: *const i8) -> i32;
    fn sqlite3_compileoption_get(N: i32) -> *const i8;
    fn sqlite3_last_insert_rowid(__v1074: *mut sqlite3) -> i64;
    fn sqlite3_changes64(__v1075: *mut sqlite3) -> i64;
    fn sqlite3_total_changes64(__v1076: *mut sqlite3) -> i64;
    fn sqlite3_mprintf(__v1077: *const i8, ...) -> *mut i8;
    fn sqlite3_vmprintf(__v1078: *const i8, __v1079: core::ffi::VaList<'_>) -> *mut i8;
    fn sqlite3_malloc64(__v1080: u64) -> *mut ();
    fn sqlite3_realloc64(__v1081: *mut (), __v1082: u64) -> *mut ();
    fn sqlite3_free(__v1083: *mut ());
    fn sqlite3_randomness(N: i32, P: *mut ());
    fn sqlite3_value_blob(__v1086: *mut sqlite3_value) -> *const ();
    fn sqlite3_value_double(__v1087: *mut sqlite3_value) -> f64;
    fn sqlite3_value_int(__v1088: *mut sqlite3_value) -> i32;
    fn sqlite3_value_int64(__v1089: *mut sqlite3_value) -> i64;
    fn sqlite3_value_text(__v1090: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_bytes(__v1091: *mut sqlite3_value) -> i32;
    fn sqlite3_value_bytes16(__v1092: *mut sqlite3_value) -> i32;
    fn sqlite3_value_type(__v1093: *mut sqlite3_value) -> i32;
    fn sqlite3_value_numeric_type(__v1094: *mut sqlite3_value) -> i32;
    fn sqlite3_value_encoding(__v1095: *mut sqlite3_value) -> i32;
    fn sqlite3_value_subtype(__v1096: *mut sqlite3_value) -> u32;
    fn sqlite3_value_dup(__v1097: *const sqlite3_value) -> *mut sqlite3_value;
    fn sqlite3_value_free(__v1098: *mut sqlite3_value);
    fn sqlite3_aggregate_context(__v1099: *mut sqlite3_context, nBytes: i32) -> *mut ();
    fn sqlite3_user_data(__v1101: *mut sqlite3_context) -> *mut ();
    fn sqlite3_context_db_handle(__v1102: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_blob(
        __v1103: *mut sqlite3_context,
        __v1104: *const (),
        __v1105: i32,
        __v1106: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_blob64(
        __v1107: *mut sqlite3_context,
        __v1108: *const (),
        __v1109: u64,
        __v1110: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_double(__v1111: *mut sqlite3_context, __v1112: f64);
    fn sqlite3_result_error(__v1113: *mut sqlite3_context, __v1114: *const i8, __v1115: i32);
    fn sqlite3_result_error_toobig(__v1116: *mut sqlite3_context);
    fn sqlite3_result_error_nomem(__v1117: *mut sqlite3_context);
    fn sqlite3_result_error_code(__v1118: *mut sqlite3_context, __v1119: i32);
    fn sqlite3_result_int(__v1120: *mut sqlite3_context, __v1121: i32);
    fn sqlite3_result_int64(__v1122: *mut sqlite3_context, __v1123: i64);
    fn sqlite3_result_null(__v1124: *mut sqlite3_context);
    fn sqlite3_result_text(
        __v1125: *mut sqlite3_context,
        __v1126: *const i8,
        __v1127: i32,
        __v1128: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_text64(
        __v1129: *mut sqlite3_context,
        z: *const i8,
        n: u64,
        __v1132: Option<unsafe extern "C-unwind" fn(*mut ())>,
        encoding: u8,
    );
    fn sqlite3_result_value(__v1134: *mut sqlite3_context, __v1135: *mut sqlite3_value);
    fn sqlite3_result_zeroblob64(__v1136: *mut sqlite3_context, n: u64) -> i32;
    fn sqlite3_load_extension(
        db: *mut sqlite3,
        zFile: *const i8,
        zProc: *const i8,
        pzErrMsg: *mut *mut i8,
    ) -> i32;
    fn sqlite3_overload_function(__v1142: *mut sqlite3, zFuncName: *const i8, nArg: i32) -> i32;
    fn sqlite3_result_str(__v1145: *mut sqlite3_context, __v1146: *mut sqlite3_str, __v1147: i32);
    fn sqlite3_str_appendf(__v1148: *mut sqlite3_str, zFormat: *const i8, ...);
    fn sqlite3_str_append(__v1150: *mut sqlite3_str, zIn: *const i8, N: i32);
    fn sqlite3_str_appendchar(__v1153: *mut sqlite3_str, N: i32, C: i8);
    fn sqlite3_log(iErrCode: i32, zFormat: *const i8, ...);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memmove(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn strchr(__s: *const i8, __c: i32) -> *mut i8;
    fn strcspn(__s: *const i8, __reject: *const i8) -> u64;
    fn sqlite3MemCompare(
        __v1179: *const sqlite3_value,
        __v1180: *const sqlite3_value,
        __v1181: *const CollSeq,
    ) -> i32;
    fn sqlite3VdbeFuncName(__v1182: *const sqlite3_context) -> *const i8;
    fn sqlite3WindowFunctions();
    fn sqlite3Malloc(__v1183: u64) -> *mut ();
    fn sqlite3Realloc(__v1184: *mut (), __v1185: u64) -> *mut ();
    fn sqlite3IsOverflow(__v1186: f64) -> i32;
    fn sqlite3InsertBuiltinFuncs(__v1187: *mut FuncDef, __v1188: i32);
    fn sqlite3FindFunction(
        __v1189: *mut sqlite3,
        __v1190: *const i8,
        __v1191: i32,
        __v1192: u8,
        __v1193: u8,
    ) -> *mut FuncDef;
    fn sqlite3AppendOneUtf8Character(__v1197: *mut i8, __v1198: u32) -> i32;
    fn sqlite3RegisterDateTimeFunctions();
    fn sqlite3RegisterJsonFunctions();
    fn sqlite3AtoF(z: *const i8, __v1201: *mut f64) -> i32;
    fn sqlite3Utf8CharLen(pData: *const i8, nByte: i32) -> i32;
    fn sqlite3Utf8Read(__v1204: *mut *const u8) -> u32;
    fn sqlite3HexToInt(h: i32) -> u8;
    fn sqlite3AddInt64(__v1206: *mut i64, __v1207: i64) -> i32;
    fn sqlite3SubInt64(__v1208: *mut i64, __v1209: i64) -> i32;
    fn sqlite3AlterFunctions();
    fn sqlite3CreateFunc(
        __v1216: *mut sqlite3,
        __v1217: *const i8,
        __v1218: i32,
        __v1219: i32,
        __v1220: *mut (),
        __v1221: Option<
            unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
        >,
        __v1222: Option<
            unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
        >,
        __v1223: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
        __v1224: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
        __v1225: Option<
            unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
        >,
        pDestructor: *mut FuncDestructor,
    ) -> i32;
    fn sqlite3OomFault(__v1227: *mut sqlite3) -> *mut ();
    fn sqlite3StrAccumInit(
        __v1228: *mut sqlite3_str,
        __v1229: *mut sqlite3,
        __v1230: *mut i8,
        __v1231: i32,
        __v1232: i32,
    );
    fn sqlite3StrAccumEnlarge(__v1233: *mut sqlite3_str, __v1234: i64) -> i32;
    fn sqlite3StrAccumSetError(__v1235: *mut sqlite3_str, __v1236: u8);
    fn acos(__x: f64) -> f64;
    fn asin(__x: f64) -> f64;
    fn atan(__x: f64) -> f64;
    fn atan2(__y: f64, __x: f64) -> f64;
    fn cos(__x: f64) -> f64;
    fn sin(__x: f64) -> f64;
    fn tan(__x: f64) -> f64;
    fn cosh(__x: f64) -> f64;
    fn sinh(__x: f64) -> f64;
    fn tanh(__x: f64) -> f64;
    fn acosh(__x: f64) -> f64;
    fn asinh(__x: f64) -> f64;
    fn atanh(__x: f64) -> f64;
    fn exp(__x: f64) -> f64;
    fn log(__x: f64) -> f64;
    fn log10(__x: f64) -> f64;
    fn log2(__x: f64) -> f64;
    fn pow(__x: f64, __y: f64) -> f64;
    fn sqrt(__x: f64) -> f64;
    fn ceil(__x: f64) -> f64;
    fn fabs(__x: f64) -> f64;
    fn floor(__x: f64) -> f64;
    fn fmod(__x: f64, __y: f64) -> f64;
    fn trunc(__x: f64) -> f64;
    fn sqlite3VdbeMemCopy(__v1264: *mut sqlite3_value, __v1265: *const sqlite3_value) -> i32;
    fn sqlite3VdbeMemRelease(p: *mut sqlite3_value);
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
    trace: __SlateRecord169,
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
    u1: __SlateRecord170,
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
    u: __SlateRecord171,
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
    u: __SlateRecord172,
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
    u: __SlateRecord180,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord181,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord182,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord183,
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
    fg: __SlateRecord190,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord191,
    u2: __SlateRecord192,
    u3: __SlateRecord193,
    u4: __SlateRecord194,
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
struct RenameToken {}

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
struct TableLock {}

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
struct VtabCtx {}

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
struct Btree {}

#[repr(C)]
#[derive(Clone, Copy)]
struct BtCursor {}

#[repr(C)]
#[derive(Clone, Copy)]
struct PrintfArguments {
    nArg: i32,
    nUsed: i32,
    apArg: *mut *mut sqlite3_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VdbeCursor {
    eCurType: u8,
    iDb: i8,
    nullRow: u8,
    deferredMoveto: u8,
    isTable: u8,
    __slate_bits_0: __slate_bits::__SlateBits207U0,
    seekHit: u16,
    ub: __SlateRecord209,
    seqCount: i64,
    cacheStatus: u32,
    seekResult: i32,
    pAltCursor: *mut VdbeCursor,
    uc: __SlateRecord210,
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
    __slate_bits_0: __slate_bits::__SlateBits168U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord169 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord170 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord171 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord172 {
    tab: __SlateRecord173,
    view: __SlateRecord174,
    vtab: __SlateRecord175,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord173 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord174 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord175 {
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
union __SlateRecord180 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord183 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord184,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord184 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord186,
    u: __SlateRecord187,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord186 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits186U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    x: __SlateRecord188,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord188 {
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
struct __SlateRecord190 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits190U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord192 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord193 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord194 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
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
union __SlateRecord209 {
    pBtx: *mut Btree,
    aAltMap: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord210 {
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
    __slate_bits_0: __slate_bits::__SlateBits157U0,
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

/// Return the collating function associated with a function.
fn sqlite3GetFuncCollSeq(mut context: *mut sqlite3_context) -> *mut CollSeq {
    let mut pOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
    0 as i32;
    pOp = unsafe {
        unsafe { (*unsafe { (*context).pVdbe }).aOp }
            .offset(((unsafe { (*context).iOp }) - (1 as i32)) as isize)
    };
    0 as i32;
    0 as i32;
    return unsafe { (*pOp).p4.pColl };
}

/// Indicate that the accumulator load should be skipped on this
/// iteration of the aggregate loop.
fn sqlite3SkipAccumulatorLoad(mut context: *mut sqlite3_context) {
    0 as i32;
    unsafe {
        (*context).isError = -(1 as i32);
    }
    unsafe {
        (*context).skipFlag = ((1 as i32) as i8) as u8;
    }
}

/// Implementation of the non-aggregate min() and max() functions
#[unsafe(link_section = ".text.slate_distinct.func.minmaxFunc")]
extern "C-unwind" fn minmaxFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut i: i32 = 0 as i32;
    let mut mask: i32 = 0 as i32; // 0 for min() or 0xffffffff for max()
    let mut iBest: i32 = 0 as i32;
    let mut pColl: *mut CollSeq = unsafe { std::mem::zeroed() };
    0 as i32;
    mask = if (unsafe { sqlite3_user_data(context) }) == std::ptr::null_mut::<()>() {
        0 as i32
    } else {
        -(1 as i32)
    };
    pColl = sqlite3GetFuncCollSeq(context);
    0 as i32;
    0 as i32;
    iBest = 0 as i32;
    if (unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
        == (5 as i32)
    {
        return;
    }
    i = 1 as i32;
    '__slate_break_1267: loop {
        if !(i < argc) {
            break;
        }
        if (unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset(i as isize) } }) })
            == (5 as i32)
        {
            return;
        }
        if (unsafe {
            sqlite3MemCompare(
                (unsafe { *unsafe { argv.offset(iBest as isize) } }) as *const sqlite3_value,
                (unsafe { *unsafe { argv.offset(i as isize) } }) as *const sqlite3_value,
                pColl as *const CollSeq,
            )
        }) ^ mask
            >= (0 as i32)
        {
            {}
            iBest = i;
        }
        let __v1468: i32 = i;
        let __v1469: i32 = __v1468 + (1 as i32);
        i = __v1469;
    }
    unsafe { sqlite3_result_value(context, unsafe { *unsafe { argv.offset(iBest as isize) } }) };
}

/// Return the type of the argument.
#[unsafe(link_section = ".text.slate_distinct.func.typeofFunc")]
extern "C-unwind" fn typeofFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut i: i32 =
        (unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
            - (1 as i32);
    NotUsed;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    // EVIDENCE-OF: R-01470-60482 The sqlite3_value_type(V) interface returns
    // the datatype code for the initial datatype of the sqlite3_value object
    // V. The returned value is one of SQLITE_INTEGER, SQLITE_FLOAT,
    // SQLITE_TEXT, SQLITE_BLOB, or SQLITE_NULL.
    unsafe {
        sqlite3_result_text(
            context,
            unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!(azType.0) as *mut *const i8 }.offset(i as isize)
                }
            },
            -(1 as i32),
            None,
        )
    };
}

static mut azType: __SlateAlign16<[*const i8; 5]> = __SlateAlign16([
    (b"integer\0".as_ptr() as *mut i8) as *const i8,
    (b"real\0".as_ptr() as *mut i8) as *const i8,
    (b"text\0".as_ptr() as *mut i8) as *const i8,
    (b"blob\0".as_ptr() as *mut i8) as *const i8,
    (b"null\0".as_ptr() as *mut i8) as *const i8,
]);

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
    pub struct __SlateBits190U0 {
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
    pub struct __SlateBits186U0 {
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
    pub struct __SlateBits207U0 {
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
    pub struct __SlateBits157U0 {
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
    pub struct __SlateBits168U0 {
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

/// subtype(X)
///
/// Return the subtype of X
#[unsafe(link_section = ".text.slate_distinct.func.subtypeFunc")]
extern "C-unwind" fn subtypeFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    argc;
    unsafe {
        sqlite3_result_int(
            context,
            (unsafe {
                sqlite3_value_subtype(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
            }) as i32,
        )
    };
}

/// Implementation of the length() function
#[unsafe(link_section = ".text.slate_distinct.func.lengthFunc")]
extern "C-unwind" fn lengthFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    0 as i32;
    argc;
    '__slate_break_1273: {
        match unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) }
        {
            4 | 1 | 2 => {
                unsafe {
                    sqlite3_result_int(context, unsafe {
                        sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                    })
                };
            }
            3 => {
                let mut z: *const u8 = unsafe {
                    sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                };
                let mut z0: *const u8 = unsafe { std::mem::zeroed() };
                if z == std::ptr::null::<u8>() {
                    return;
                }
                z0 = z;
                '__slate_break_1274: while (1 as i32) != (0 as i32) {
                    // exit-by-break
                    //  vvvvvv----  See tag-20260418-01
                    if ((((((((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32)
                        as i32)
                        - (1 as i32)) as i8) as u8) as u32) as i32)
                        < (128 as i32) - (1 as i32)
                    {
                        let __v1470: *const u8 = z;
                        let __v1471: *const u8 = unsafe { __v1470.offset((1 as i32) as isize) };
                        z = __v1471;
                    } else {
                        if (((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32) as i32)
                            == (0 as i32)
                        {
                            break '__slate_break_1274;
                        } else {
                            let __v1472: *const u8 = z;
                            let __v1473: *const u8 = unsafe { __v1472.offset((1 as i32) as isize) };
                            z = __v1473;
                            '__slate_break_1275: while (((unsafe {
                                *unsafe { z.offset((0 as i32) as isize) }
                            }) as u32)
                                as i32)
                                & (192 as i32)
                                == (128 as i32)
                            {
                                let __v1474: *const u8 = z;
                                let __v1475: *const u8 =
                                    unsafe { __v1474.offset((1 as i32) as isize) };
                                z = __v1475;
                                let __v1476: *const u8 = z0;
                                let __v1477: *const u8 =
                                    unsafe { __v1476.offset((1 as i32) as isize) };
                                z0 = __v1477;
                            }
                        }
                    }
                }
                unsafe {
                    sqlite3_result_int(
                        context,
                        ((unsafe { z.offset_from(z0 as *const u8) }) as i64) as i32,
                    )
                };
            }
            _ => {
                unsafe { sqlite3_result_null(context) };
            }
        }
    }
}

/// Implementation of the octet_length() function
#[unsafe(link_section = ".text.slate_distinct.func.bytelengthFunc")]
extern "C-unwind" fn bytelengthFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    0 as i32;
    argc;
    '__slate_break_1276: {
        match unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) }
        {
            4 => {
                unsafe {
                    sqlite3_result_int(context, unsafe {
                        sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                    })
                };
            }
            1 | 2 => {
                let mut m: i64 =
                    (if (((unsafe { (*unsafe { sqlite3_context_db_handle(context) }).enc }) as u32)
                        as i32)
                        <= (1 as i32)
                    {
                        1 as i32
                    } else {
                        2 as i32
                    }) as i64;
                unsafe {
                    sqlite3_result_int64(
                        context,
                        ((unsafe {
                            sqlite3_value_bytes(unsafe {
                                *unsafe { argv.offset((0 as i32) as isize) }
                            })
                        }) as i64)
                            * m,
                    )
                };
            }
            3 => {
                if (unsafe {
                    sqlite3_value_encoding(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                }) <= (1 as i32)
                {
                    unsafe {
                        sqlite3_result_int(context, unsafe {
                            sqlite3_value_bytes(unsafe {
                                *unsafe { argv.offset((0 as i32) as isize) }
                            })
                        })
                    };
                } else {
                    unsafe {
                        sqlite3_result_int(context, unsafe {
                            sqlite3_value_bytes16(unsafe {
                                *unsafe { argv.offset((0 as i32) as isize) }
                            })
                        })
                    };
                }
            }
            _ => {
                unsafe { sqlite3_result_null(context) };
            }
        }
    }
}

/// Implementation of the abs() function.
///
/// IMP: R-23979-26855 The abs(X) function returns the absolute value of
/// the numeric argument X.
#[unsafe(link_section = ".text.slate_distinct.func.absFunc")]
extern "C-unwind" fn absFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    0 as i32;
    argc;
    '__slate_break_1277: {
        match unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) }
        {
            1 => {
                let mut iVal: i64 = unsafe {
                    sqlite3_value_int64(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                };
                if iVal < ((0 as i32) as i64) {
                    if iVal
                        == (-(1 as i32) as i64)
                            - ((((4294967295 as u32) as u64) as i64)
                                | ((2147483647 as i32) as i64) << (32 as i32))
                    {
                        // IMP: R-31676-45509 If X is the integer -9223372036854775808
                        // then abs(X) throws an integer overflow error since there is no
                        // equivalent positive 64-bit two complement value.
                        unsafe {
                            sqlite3_result_error(
                                context,
                                (b"integer overflow\0".as_ptr() as *mut i8) as *const i8,
                                -(1 as i32),
                            )
                        };
                        return;
                    }
                    iVal = -iVal;
                }
                unsafe { sqlite3_result_int64(context, iVal) };
            }
            5 => {
                // IMP: R-37434-19929 Abs(X) returns NULL if X is NULL.
                unsafe { sqlite3_result_null(context) };
            }
            _ => {
                // Because sqlite3_value_double() returns 0.0 if the argument is not
                // something that can be converted into a number, we have:
                // IMP: R-01992-00519 Abs(X) returns 0.0 if X is a string or blob
                // that cannot be converted to a numeric value.
                let mut rVal: f64 = unsafe {
                    sqlite3_value_double(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                };
                if rVal < ((0 as i32) as f64) {
                    rVal = -rVal;
                }
                unsafe { sqlite3_result_double(context, rVal) };
            }
        }
    }
}

/// Implementation of the instr() function.
///
/// instr(haystack,needle) finds the first occurrence of needle
/// in haystack and returns the number of previous characters plus 1,
/// or 0 if needle does not occur within haystack.
///
/// If both haystack and needle are BLOBs, then the result is one more than
/// the number of bytes in haystack prior to the first occurrence of needle,
/// or 0 if needle never occurs in haystack.
#[unsafe(link_section = ".text.slate_distinct.func.instrFunc")]
extern "C-unwind" fn instrFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut __slate_storage_1483: std::mem::MaybeUninit<*const u8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1483: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_1483) as *mut *const u8;
    let mut __slate_storage_1482: std::mem::MaybeUninit<*const u8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1482: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_1482) as *mut *const u8;
    let mut __slate_storage_1481: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1481: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1481) as *mut i32;
    let mut __slate_storage_1480: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1480: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1480) as *mut i32;
    let mut __slate_storage_1479: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1479: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1479) as *mut i32;
    let mut __slate_storage_1478: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1478: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1478) as *mut i32;
    let mut __slate_storage_540: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_540: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_540) as *mut *mut sqlite3_value;
    let mut __slate_storage_539: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_539: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_539) as *mut *mut sqlite3_value;
    let mut __slate_storage_538: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_538: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_538) as *mut u8;
    let mut __slate_storage_537: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_537: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_537) as *mut i32;
    let mut __slate_storage_536: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_536: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_536) as *mut i32;
    let mut __slate_storage_535: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_535: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_535) as *mut i32;
    let mut __slate_storage_534: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_534: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_534) as *mut i32;
    let mut __slate_storage_533: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_533: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_533) as *mut i32;
    let mut __slate_storage_532: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_532: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_532) as *mut i32;
    let mut __slate_storage_531: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_531: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_531) as *mut *const u8;
    let mut __slate_storage_530: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_530: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_530) as *mut *const u8;
    unsafe {
        std::ptr::write(__slate_slot_536, 1 as i32);
        std::ptr::write(__slate_slot_539, std::ptr::null_mut::<sqlite3_value>());
        std::ptr::write(__slate_slot_540, std::ptr::null_mut::<sqlite3_value>());
        argc;
        *__slate_slot_534 =
            unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
        *__slate_slot_535 =
            unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) };
        if *__slate_slot_534 == (5 as i32) || *__slate_slot_535 == (5 as i32) {
            return;
        } else {
            '__join_1: {
                '__join_2: {
                    *__slate_slot_532 = unsafe {
                        sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                    };
                    *__slate_slot_533 = unsafe {
                        sqlite3_value_bytes(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
                    };
                    if *__slate_slot_533 > (0 as i32) {
                        '__join_0: {
                            if *__slate_slot_534 == (4 as i32) && *__slate_slot_535 == (4 as i32) {
                                *__slate_slot_530 = (unsafe {
                                    sqlite3_value_blob(unsafe {
                                        *unsafe { argv.offset((0 as i32) as isize) }
                                    })
                                }) as *const u8;
                                *__slate_slot_531 = (unsafe {
                                    sqlite3_value_blob(unsafe {
                                        *unsafe { argv.offset((1 as i32) as isize) }
                                    })
                                }) as *const u8;
                                *__slate_slot_537 = 0 as i32;
                            } else {
                                if *__slate_slot_534 != (4 as i32)
                                    && *__slate_slot_535 != (4 as i32)
                                {
                                    *__slate_slot_530 = unsafe {
                                        sqlite3_value_text(unsafe {
                                            *unsafe { argv.offset((0 as i32) as isize) }
                                        })
                                    };
                                    *__slate_slot_531 = unsafe {
                                        sqlite3_value_text(unsafe {
                                            *unsafe { argv.offset((1 as i32) as isize) }
                                        })
                                    };
                                    *__slate_slot_537 = 1 as i32;
                                } else {
                                    *__slate_slot_539 = unsafe {
                                        sqlite3_value_dup(
                                            (unsafe {
                                                *unsafe { argv.offset((0 as i32) as isize) }
                                            })
                                                as *const sqlite3_value,
                                        )
                                    };
                                    *__slate_slot_530 =
                                        unsafe { sqlite3_value_text(*__slate_slot_539) };
                                    if *__slate_slot_530 == std::ptr::null::<u8>() {
                                        break '__join_0;
                                    } else {
                                        *__slate_slot_532 =
                                            unsafe { sqlite3_value_bytes(*__slate_slot_539) };
                                        *__slate_slot_540 = unsafe {
                                            sqlite3_value_dup(
                                                (unsafe {
                                                    *unsafe { argv.offset((1 as i32) as isize) }
                                                })
                                                    as *const sqlite3_value,
                                            )
                                        };
                                        *__slate_slot_531 =
                                            unsafe { sqlite3_value_text(*__slate_slot_540) };
                                        if *__slate_slot_531 == std::ptr::null::<u8>() {
                                            break '__join_0;
                                        } else {
                                            *__slate_slot_533 =
                                                unsafe { sqlite3_value_bytes(*__slate_slot_540) };
                                            *__slate_slot_537 = 1 as i32;
                                        }
                                    }
                                }
                            }
                            if *__slate_slot_531 == std::ptr::null::<u8>()
                                || *__slate_slot_532 != (0 as i32)
                                    && *__slate_slot_530 == std::ptr::null::<u8>()
                            {
                            } else {
                                *__slate_slot_538 = unsafe {
                                    *unsafe { (*__slate_slot_531).offset((0 as i32) as isize) }
                                };
                                '__loop_5: loop {
                                    if *__slate_slot_533 <= *__slate_slot_532
                                        && ((((unsafe {
                                            *unsafe {
                                                (*__slate_slot_530).offset((0 as i32) as isize)
                                            }
                                        }) as u32)
                                            as i32)
                                            != ((*__slate_slot_538 as u32) as i32)
                                            || (unsafe {
                                                memcmp(
                                                    *__slate_slot_530 as *const (),
                                                    *__slate_slot_531 as *const (),
                                                    (*__slate_slot_533 as i64) as u64,
                                                )
                                            }) != (0 as i32))
                                    {
                                        std::ptr::write(__slate_slot_1478, *__slate_slot_536);
                                        std::ptr::write(
                                            __slate_slot_1479,
                                            *__slate_slot_1478 + (1 as i32),
                                        );
                                        *__slate_slot_536 = *__slate_slot_1479;
                                        loop {
                                            std::ptr::write(__slate_slot_1480, *__slate_slot_532);
                                            std::ptr::write(
                                                __slate_slot_1481,
                                                *__slate_slot_1480 - (1 as i32),
                                            );
                                            *__slate_slot_532 = *__slate_slot_1481;
                                            std::ptr::write(__slate_slot_1482, *__slate_slot_530);
                                            std::ptr::write(__slate_slot_1483, unsafe {
                                                (*__slate_slot_1482).offset((1 as i32) as isize)
                                            });
                                            *__slate_slot_530 = *__slate_slot_1483;
                                            if !(*__slate_slot_537 != (0 as i32)
                                                && (((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_530)
                                                            .offset((0 as i32) as isize)
                                                    }
                                                })
                                                    as u32)
                                                    as i32)
                                                    & (192 as i32)
                                                    == (128 as i32))
                                            {
                                                continue '__loop_5;
                                            }
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                if *__slate_slot_533 > *__slate_slot_532 {
                                    *__slate_slot_536 = 0 as i32;
                                    break '__join_2;
                                } else {
                                    break '__join_2;
                                }
                            }
                        }
                        unsafe { sqlite3_result_error_nomem(context) };
                        break '__join_1;
                    }
                }
                unsafe { sqlite3_result_int(context, *__slate_slot_536) };
            }
            unsafe { sqlite3_value_free(*__slate_slot_539) };
            unsafe { sqlite3_value_free(*__slate_slot_540) };
            return;
        }
    }
}

/// Implementation of the printf() (a.k.a. format()) SQL function.
#[unsafe(link_section = ".text.slate_distinct.func.printfFunc")]
extern "C-unwind" fn printfFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut x: PrintfArguments = unsafe { std::mem::zeroed() };
    let mut str: sqlite3_str = unsafe { std::mem::zeroed() };
    let mut zFormat: *const i8 = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    let __v1484: bool;
    if argc >= (1 as i32) {
        let __v1485: *const i8 = (unsafe {
            sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
        }) as *const i8;
        zFormat = __v1485;
        __v1484 = __v1485 != std::ptr::null::<i8>();
    } else {
        __v1484 = false as bool;
    }
    if __v1484 {
        x.nArg = argc - (1 as i32);
        x.nUsed = 0 as i32;
        x.apArg = unsafe { argv.offset((1 as i32) as isize) };
        unsafe {
            sqlite3StrAccumInit(
                std::ptr::addr_of_mut!(str),
                db,
                std::ptr::null_mut::<i8>(),
                0 as i32,
                unsafe {
                    *unsafe {
                        unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize)
                    }
                },
            )
        };
        str.printfFlags = ((2 as i32) as i8) as u8;
        unsafe {
            sqlite3_str_appendf(
                std::ptr::addr_of_mut!(str),
                zFormat,
                std::ptr::addr_of_mut!(x),
            )
        };
        unsafe { sqlite3_result_str(context, std::ptr::addr_of_mut!(str), 1 as i32) };
    }
}

/// Implementation of the substr() function.
///
/// substr(x,p1,p2)  returns p2 characters of x[] beginning with p1.
/// p1 is 1-indexed.  So substr(x,1,1) returns the first character
/// of x.  If x is text, then we actually count UTF-8 characters.
/// If x is a blob, then we count bytes.
///
/// If p1 is negative, then we begin abs(p1) from the end of x[].
///
/// If p2 is negative, return the p2 characters preceding p1.
#[unsafe(link_section = ".text.slate_distinct.func.substrFunc")]
extern "C-unwind" fn substrFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut z: *const u8 = unsafe { std::mem::zeroed() };
    let mut z2: *const u8 = unsafe { std::mem::zeroed() };
    let mut len: i32 = 0 as i32;
    let mut p0type: i32 = 0 as i32;
    let mut p1: i64 = 0 as i64;
    let mut p2: i64 = 0 as i64;
    0 as i32;
    p0type = unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    p1 = unsafe { sqlite3_value_int64(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) };
    if p0type == (4 as i32) {
        len =
            unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
        z = (unsafe { sqlite3_value_blob(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
            as *const u8;
        if z == std::ptr::null::<u8>() {
            return;
        }
        0 as i32;
    } else {
        z = unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
        if z == std::ptr::null::<u8>() {
            return;
        }
        len = 0 as i32;
        if p1 < ((0 as i32) as i64) {
            z2 = z;
            '__slate_break_1281: loop {
                if !((unsafe { *z2 }) != (0 as u8)) {
                    break;
                }
                let __v1488: *const u8 = z2;
                let __v1489: *const u8 = unsafe { __v1488.offset((1 as i32) as isize) };
                z2 = __v1489;
                if (((unsafe { *__v1488 }) as u32) as i32) >= (192 as i32) {
                    '__slate_break_1282: while (((unsafe { *z2 }) as u32) as i32) & (192 as i32)
                        == (128 as i32)
                    {
                        let __v1490: *const u8 = z2;
                        let __v1491: *const u8 = unsafe { __v1490.offset((1 as i32) as isize) };
                        z2 = __v1491;
                    }
                }
                {}
                let __v1486: i32 = len;
                let __v1487: i32 = __v1486 + (1 as i32);
                len = __v1487;
            }
        }
    }
    if argc == (3 as i32) {
        p2 =
            unsafe { sqlite3_value_int64(unsafe { *unsafe { argv.offset((2 as i32) as isize) } }) };
        let __v1492: bool;
        if p2 == ((0 as i32) as i64) {
            __v1492 = (unsafe {
                sqlite3_value_type(unsafe { *unsafe { argv.offset((2 as i32) as isize) } })
            }) == (5 as i32);
        } else {
            __v1492 = false as bool;
        }
        if __v1492 {
            return;
        }
    } else {
        p2 = (unsafe {
            *unsafe {
                unsafe {
                    (*unsafe { sqlite3_context_db_handle(context) })
                        .aLimit
                        .as_mut_ptr() as *mut i32
                }
                .offset((0 as i32) as isize)
            }
        }) as i64;
    }
    if p1 == ((0 as i32) as i64) {
        if (unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) })
            == (5 as i32)
        {
            return;
        }
    }
    if p1 < ((0 as i32) as i64) {
        let __v1493: i64 = p1;
        let __v1494: i64 = __v1493 + (len as i64);
        p1 = __v1494;
        if p1 < ((0 as i32) as i64) {
            if p2 < ((0 as i32) as i64) {
                p2 = (0 as i32) as i64;
            } else {
                let __v1495: i64 = p2;
                let __v1496: i64 = __v1495 + p1;
                p2 = __v1496;
            }
            p1 = (0 as i32) as i64;
        }
    } else {
        if p1 > ((0 as i32) as i64) {
            let __v1497: i64 = p1;
            let __v1498: i64 = __v1497 - ((1 as i32) as i64);
            p1 = __v1498;
        } else {
            if p2 > ((0 as i32) as i64) {
                let __v1499: i64 = p2;
                let __v1500: i64 = __v1499 - ((1 as i32) as i64);
                p2 = __v1500;
            }
        }
    }
    if p2 < ((0 as i32) as i64) {
        if p2 < -p1 {
            p2 = p1;
        } else {
            p2 = -p2;
        }
        let __v1501: i64 = p1;
        let __v1502: i64 = __v1501 - p2;
        p1 = __v1502;
    }
    0 as i32;
    if p0type != (4 as i32) {
        '__slate_break_1283: loop {
            if !(p1 > ((0 as i32) as i64)) {
                break;
            }
            //  vvvvvv----  See tag-20260418-01
            if ((((((((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32) as i32)
                - (1 as i32)) as i8) as u8) as u32) as i32)
                < (128 as i32) - (1 as i32)
            {
                let __v1505: *const u8 = z;
                let __v1506: *const u8 = unsafe { __v1505.offset((1 as i32) as isize) };
                z = __v1506;
            } else {
                if (((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32) as i32)
                    == (0 as i32)
                {
                    break '__slate_break_1283;
                } else {
                    '__slate_break_1284: loop {
                        let __v1507: *const u8 = z;
                        let __v1508: *const u8 = unsafe { __v1507.offset((1 as i32) as isize) };
                        z = __v1508;
                        if !((((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32)
                            as i32)
                            & (192 as i32)
                            == (128 as i32))
                        {
                            break;
                        }
                    }
                }
            }
            let __v1503: i64 = p1;
            let __v1504: i64 = __v1503 - ((1 as i32) as i64);
            p1 = __v1504;
        }
        z2 = z;
        '__slate_break_1285: loop {
            if !(p2 > ((0 as i32) as i64)) {
                break;
            }
            //  vvvvvv----  See tag-20260418-01
            if ((((((((unsafe { *unsafe { z2.offset((0 as i32) as isize) } }) as u32) as i32)
                - (1 as i32)) as i8) as u8) as u32) as i32)
                < (128 as i32) - (1 as i32)
            {
                let __v1511: *const u8 = z2;
                let __v1512: *const u8 = unsafe { __v1511.offset((1 as i32) as isize) };
                z2 = __v1512;
            } else {
                if (((unsafe { *unsafe { z2.offset((0 as i32) as isize) } }) as u32) as i32)
                    == (0 as i32)
                {
                    break '__slate_break_1285;
                } else {
                    '__slate_break_1286: loop {
                        let __v1513: *const u8 = z2;
                        let __v1514: *const u8 = unsafe { __v1513.offset((1 as i32) as isize) };
                        z2 = __v1514;
                        if !((((unsafe { *unsafe { z2.offset((0 as i32) as isize) } }) as u32)
                            as i32)
                            & (192 as i32)
                            == (128 as i32))
                        {
                            break;
                        }
                    }
                }
            }
            let __v1509: i64 = p2;
            let __v1510: i64 = __v1509 - ((1 as i32) as i64);
            p2 = __v1510;
        }
        unsafe {
            sqlite3_result_text64(
                context,
                (z as *mut i8) as *const i8,
                ((unsafe { z2.offset_from(z as *const u8) }) as i64) as u64,
                unsafe {
                    std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        -(1 as i32) as usize,
                    )
                },
                ((1 as i32) as i8) as u8,
            )
        };
    } else {
        if p1 >= (len as i64) {
            p2 = (0 as i32) as i64;
            p1 = (0 as i32) as i64;
        } else {
            if p2 > (len as i64) - p1 {
                p2 = (len as i64) - p1;
                0 as i32;
            }
        }
        unsafe {
            sqlite3_result_blob64(
                context,
                ((unsafe { z.offset(p1 as isize) }) as *mut i8) as *const (),
                p2 as u64,
                unsafe {
                    std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        -(1 as i32) as usize,
                    )
                },
            )
        };
    }
}

/// Implementation of the round() function
#[unsafe(link_section = ".text.slate_distinct.func.roundFunc")]
extern "C-unwind" fn roundFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut n: i64 = (0 as i32) as i64;
    let mut r: f64 = 0 as f64;
    let mut zBuf: *mut i8 = unsafe { std::mem::zeroed() };
    0 as i32;
    if argc == (2 as i32) {
        if (5 as i32)
            == unsafe {
                sqlite3_value_type(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
            }
        {
            return;
        }
        n = unsafe { sqlite3_value_int64(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) };
        if n > ((30 as i32) as i64) {
            n = (30 as i32) as i64;
        }
        if n < ((0 as i32) as i64) {
            n = (0 as i32) as i64;
        }
    }
    if (unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
        == (5 as i32)
    {
        return;
    }
    r = unsafe { sqlite3_value_double(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    // If Y==0 and X will fit in a 64-bit int,
    // handle the rounding directly,
    // otherwise use printf.
    if r < -4503599627370496.0f64 || r > 4503599627370496.0f64 {
        // The value has no fractional part so there is nothing to round
    } else {
        if n == ((0 as i32) as i64) {
            r = ((r + if r < ((0 as i32) as f64) {
                -0.5f64
            } else {
                0.5f64
            }) as i64) as f64;
        } else {
            zBuf = unsafe {
                sqlite3_mprintf((b"%!.*f\0".as_ptr() as *mut i8) as *const i8, n as i32, r)
            };
            if zBuf == std::ptr::null_mut::<i8>() {
                unsafe { sqlite3_result_error_nomem(context) };
                return;
            }
            unsafe { sqlite3AtoF(zBuf as *const i8, std::ptr::addr_of_mut!(r)) };
            unsafe { sqlite3_free(zBuf as *mut ()) };
        }
    }
    unsafe { sqlite3_result_double(context, r) };
}

/// Allocate nByte bytes of space using sqlite3Malloc(). If the
/// allocation fails, call sqlite3_result_error_nomem() to notify
/// the database handle that malloc() has failed and return NULL.
/// If nByte is larger than the maximum string or blob length, then
/// raise an SQLITE_TOOBIG exception and return NULL.
fn contextMalloc(mut context: *mut sqlite3_context, mut nByte: i64) -> *mut () {
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    0 as i32;
    {}
    {}
    if nByte
        > ((unsafe {
            *unsafe { unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize) }
        }) as i64)
    {
        unsafe { sqlite3_result_error_toobig(context) };
        z = std::ptr::null_mut::<i8>();
    } else {
        z = (unsafe { sqlite3Malloc(nByte as u64) }) as *mut i8;
        if !(z != std::ptr::null_mut::<i8>()) {
            unsafe { sqlite3_result_error_nomem(context) };
        }
    }
    return z as *mut ();
}

/// Implementation of the upper() and lower() SQL functions.
#[unsafe(link_section = ".text.slate_distinct.func.upperFunc")]
extern "C-unwind" fn upperFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut z1: *mut i8 = unsafe { std::mem::zeroed() };
    let mut z2: *const i8 = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let mut n: i32 = 0 as i32;
    argc;
    z2 = ((unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
        as *mut i8) as *const i8;
    n = unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    // Verify that the call to _bytes() does not invalidate the _text() pointer
    0 as i32;
    if z2 != std::ptr::null::<i8>() {
        z1 = contextMalloc(context, (n as i64) + ((1 as i32) as i64)) as *mut i8;
        if z1 != std::ptr::null_mut::<i8>() {
            i = 0 as i32;
            '__slate_break_1288: loop {
                if !(i < n) {
                    break;
                }
                unsafe {
                    *unsafe { z1.offset(i as isize) } =
                        (((unsafe { *unsafe { z2.offset(i as isize) } }) as i32)
                            & !((((unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                        .offset(
                                            ((((unsafe { *unsafe { z2.offset(i as isize) } }) as u8)
                                                as u32)
                                                as i32)
                                                as isize,
                                        )
                                }
                            }) as u32) as i32)
                                & (32 as i32))) as i8;
                }
                let __v1515: i32 = i;
                let __v1516: i32 = __v1515 + (1 as i32);
                i = __v1516;
            }
            unsafe {
                sqlite3_result_text(context, z1 as *const i8, n, unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        sqlite3_free as *const (),
                    )
                })
            };
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.func.lowerFunc")]
extern "C-unwind" fn lowerFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut z1: *mut i8 = unsafe { std::mem::zeroed() };
    let mut z2: *const i8 = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let mut n: i32 = 0 as i32;
    argc;
    z2 = ((unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
        as *mut i8) as *const i8;
    n = unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    // Verify that the call to _bytes() does not invalidate the _text() pointer
    0 as i32;
    if z2 != std::ptr::null::<i8>() {
        z1 = contextMalloc(context, (n as i64) + ((1 as i32) as i64)) as *mut i8;
        if z1 != std::ptr::null_mut::<i8>() {
            i = 0 as i32;
            '__slate_break_1289: loop {
                if !(i < n) {
                    break;
                }
                unsafe {
                    *unsafe { z1.offset(i as isize) } = (unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }.offset(
                                ((((unsafe { *unsafe { z2.offset(i as isize) } }) as u8) as u32)
                                    as i32) as isize,
                            )
                        }
                    }) as i8;
                }
                let __v1517: i32 = i;
                let __v1518: i32 = __v1517 + (1 as i32);
                i = __v1518;
            }
            unsafe {
                sqlite3_result_text(context, z1 as *const i8, n, unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        sqlite3_free as *const (),
                    )
                })
            };
        }
    }
}

// Some functions like COALESCE() and IFNULL() and UNLIKELY() are implemented
// as VDBE code so that unused argument values do not have to be computed.
// However, we still need some kind of function implementation for this
// routines in the function table.  The noopFunc macro provides this.
// noopFunc will never be called so it doesn't matter what the implementation
// is.  We might as well use the "version()" function as a substitute.
// Substitute function - never called
/// Implementation of random().  Return a random integer.
#[unsafe(link_section = ".text.slate_distinct.func.randomFunc")]
extern "C-unwind" fn randomFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    let mut r: i64 = 0 as i64;
    NotUsed;
    NotUsed2;
    unsafe {
        sqlite3_randomness(
            ((8 as u64) as u32) as i32,
            std::ptr::addr_of_mut!(r) as *mut (),
        )
    };
    if r < ((0 as i32) as i64) {
        // We need to prevent a random number of 0x8000000000000000
        // (or -9223372036854775808) since when you do abs() of that
        // number of you get the same value back again.  To do this
        // in a way that is testable, mask the sign bit off of negative
        // values, resulting in a positive value.  Then take the
        // 2s complement of that positive value.  The end result can
        // therefore be no less than -9223372036854775807.
        r = -(r
            & ((((4294967295 as u32) as u64) as i64)
                | ((2147483647 as i32) as i64) << (32 as i32)));
    }
    unsafe { sqlite3_result_int64(context, r) };
}

/// Implementation of randomblob(N).  Return a random blob
/// that is N bytes long.
#[unsafe(link_section = ".text.slate_distinct.func.randomBlob")]
extern "C-unwind" fn randomBlob(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut n: i64 = 0 as i64;
    let mut p: *mut u8 = unsafe { std::mem::zeroed() };
    0 as i32;
    argc;
    n = unsafe { sqlite3_value_int64(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    if n < ((1 as i32) as i64) {
        n = (1 as i32) as i64;
    }
    p = contextMalloc(context, n) as *mut u8;
    if p != std::ptr::null_mut::<u8>() {
        unsafe { sqlite3_randomness(n as i32, p as *mut ()) };
        unsafe {
            sqlite3_result_blob(context, (p as *mut i8) as *const (), n as i32, unsafe {
                std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    sqlite3_free as *const (),
                )
            })
        };
    }
}

/// Implementation of the last_insert_rowid() SQL function.  The return
/// value is the same as the sqlite3_last_insert_rowid() API function.
#[unsafe(link_section = ".text.slate_distinct.func.last_insert_rowid")]
extern "C-unwind" fn last_insert_rowid(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    NotUsed;
    NotUsed2;
    // IMP: R-51513-12026 The last_insert_rowid() SQL function is a
    // wrapper around the sqlite3_last_insert_rowid() C/C++ interface
    // function.
    unsafe { sqlite3_result_int64(context, unsafe { sqlite3_last_insert_rowid(db) }) };
}

/// Implementation of the changes() SQL function.
///
/// IMP: R-32760-32347 The changes() SQL function is a wrapper
/// around the sqlite3_changes64() C/C++ function and hence follows the
/// same rules for counting changes.
#[unsafe(link_section = ".text.slate_distinct.func.changes")]
extern "C-unwind" fn changes(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    NotUsed;
    NotUsed2;
    unsafe { sqlite3_result_int64(context, unsafe { sqlite3_changes64(db) }) };
}

/// Implementation of the total_changes() SQL function.  The return value is
/// the same as the sqlite3_total_changes64() API function.
#[unsafe(link_section = ".text.slate_distinct.func.total_changes")]
extern "C-unwind" fn total_changes(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    NotUsed;
    NotUsed2;
    // IMP: R-11217-42568 This function is a wrapper around the
    // sqlite3_total_changes64() C/C++ interface.
    unsafe { sqlite3_result_int64(context, unsafe { sqlite3_total_changes64(db) }) };
}

/// A structure defining how to do GLOB-style comparisons.
#[repr(C)]
#[derive(Clone, Copy)]
struct compareInfo {
    /// "*" or "%"
    matchAll: u8,
    /// "?" or "_"
    matchOne: u8,
    /// "[" or 0
    matchSet: u8,
    /// true to ignore case differences
    noCase: u8,
}

// For LIKE and GLOB matching on EBCDIC machines, assume that every
// character is exactly one byte in size.  Also, provide the Utf8Read()
// macro for fast reading of the next character in the common case where
// the next character is ASCII.
static mut globInfo: compareInfo = compareInfo {
    matchAll: ((42 as i32) as i8) as u8,
    matchOne: ((63 as i32) as i8) as u8,
    matchSet: ((91 as i32) as i8) as u8,
    noCase: ((0 as i32) as i8) as u8,
};

/// The correct SQL-92 behavior is for the LIKE operator to ignore
/// case.  Thus  'a' LIKE 'A' would be true.
static mut likeInfoNorm: compareInfo = compareInfo {
    matchAll: ((37 as i32) as i8) as u8,
    matchOne: ((95 as i32) as i8) as u8,
    matchSet: ((0 as i32) as i8) as u8,
    noCase: ((1 as i32) as i8) as u8,
};

/// If SQLITE_CASE_SENSITIVE_LIKE is defined, then the LIKE operator
/// is case sensitive causing 'a' LIKE 'A' to be false
static mut likeInfoAlt: compareInfo = compareInfo {
    matchAll: ((37 as i32) as i8) as u8,
    matchOne: ((95 as i32) as i8) as u8,
    matchSet: ((0 as i32) as i8) as u8,
    noCase: ((0 as i32) as i8) as u8,
};

// Possible error returns from patternMatch()
/// Compare two UTF-8 strings for equality where the first string is
/// a GLOB or LIKE expression.  Return values:
///
///    SQLITE_MATCH:            Match
///    SQLITE_NOMATCH:          No match
///    SQLITE_NOWILDCARDMATCH:  No match in spite of having * or % wildcards.
///
/// Globbing rules:
///
///      '*'       Matches any sequence of zero or more characters.
///
///      '?'       Matches exactly one character.
///
///     [...]      Matches one character from the enclosed list of
///                characters.
///
///     [^...]     Matches one character not in the enclosed list.
///
/// With the [...] and [^...] matching, a ']' character can be included
/// in the list by making it the first character after '[' or '^'.  A
/// range of characters can be specified using '-'.  Example:
/// "[a-z]" matches any single lower-case letter.  To match a '-', make
/// it the last character in the list.
///
/// Like matching rules:
///
///      '%'       Matches any sequence of zero or more characters
///
///     '_'       Matches any one character
///
///      Ec        Where E is the "esc" character and c is any other
///                character, including '%', '_', and esc, match exactly c.
///
/// The comments within this routine usually assume glob matching.
///
/// This routine is usually quick, but can be N**2 in the worst case.
///
/// # Arguments
///
/// * `zPattern` - The glob pattern
/// * `zString` - The string to compare against the glob
/// * `pInfo` - Information about how to do the compare
/// * `matchOther` - The escape char (LIKE) or '[' (GLOB)
fn patternCompare(
    mut zPattern: *const u8,
    mut zString: *const u8,
    mut pInfo: *const compareInfo,
    mut matchOther: u32,
) -> i32 {
    let mut c: u32 = 0 as u32;
    let mut c2: u32 = 0 as u32; // Next pattern and input string chars
    let mut matchOne: u32 = (unsafe { (*pInfo).matchOne }) as u32; // "?" or "_"
    let mut matchAll: u32 = (unsafe { (*pInfo).matchAll }) as u32; // "*" or "%"
    let mut noCase: u8 = unsafe { (*pInfo).noCase }; // True if uppercase==lowercase
    let mut zEscaped: *const u8 = std::ptr::null::<u8>(); // One past the last escaped input char
    '__slate_break_1290: loop {
        let __v1519: u32;
        if (((unsafe { *unsafe { zPattern.offset((0 as i32) as isize) } }) as u32) as i32)
            < (128 as i32)
        {
            let __v1520: *const u8 = zPattern;
            let __v1521: *const u8 = unsafe { __v1520.offset((1 as i32) as isize) };
            zPattern = __v1521;
            __v1519 = (((unsafe { *__v1520 }) as u32) as i32) as u32;
        } else {
            __v1519 = unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(zPattern)) };
        }
        let __v1522: u32 = __v1519;
        c = __v1522;
        if !(__v1522 != ((0 as i32) as u32)) {
            break;
        }
        '__slate_continue_1290: {
            if c == matchAll {
                // Match "*"
                // Skip over multiple "*" characters in the pattern.  If there
                // are also "?" characters, skip those as well, but consume a
                // single character of the input string for each "?" skipped
                '__slate_break_1291: loop {
                    let __v1523: u32;
                    if (((unsafe { *unsafe { zPattern.offset((0 as i32) as isize) } }) as u32)
                        as i32)
                        < (128 as i32)
                    {
                        let __v1524: *const u8 = zPattern;
                        let __v1525: *const u8 = unsafe { __v1524.offset((1 as i32) as isize) };
                        zPattern = __v1525;
                        __v1523 = (((unsafe { *__v1524 }) as u32) as i32) as u32;
                    } else {
                        __v1523 = unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(zPattern)) };
                    }
                    let __v1526: u32 = __v1523;
                    c = __v1526;
                    if !(__v1526 == matchAll || c == matchOne && matchOne != ((0 as i32) as u32)) {
                        break;
                    }
                    let __v1527: bool;
                    if c == matchOne {
                        __v1527 = (unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(zString)) })
                            == ((0 as i32) as u32);
                    } else {
                        __v1527 = false as bool;
                    }
                    if __v1527 {
                        return 2 as i32;
                    }
                }
                if c == ((0 as i32) as u32) {
                    return 0 as i32; // "*" at the end of the pattern matches
                } else {
                    if c == matchOther {
                        if (((unsafe { (*pInfo).matchSet }) as u32) as i32) == (0 as i32) {
                            c = unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(zPattern)) };
                            if c == ((0 as i32) as u32) {
                                return 2 as i32;
                            }
                        } else {
                            // "[...]" immediately follows the "*".  We have to do a slow
                            // recursive search in this case, but it is an unusual case.
                            0 as i32; // '[' is a single-byte character
                            '__slate_break_1292: while (unsafe { *zString }) != (0 as u8) {
                                let mut bMatch: i32 = patternCompare(
                                    unsafe { zPattern.offset(-(1 as i32) as isize) },
                                    zString,
                                    pInfo,
                                    matchOther,
                                );
                                if bMatch != (1 as i32) {
                                    return bMatch;
                                }
                                let __v1528: *const u8 = zString;
                                let __v1529: *const u8 =
                                    unsafe { __v1528.offset((1 as i32) as isize) };
                                zString = __v1529;
                                if (((unsafe { *__v1528 }) as u32) as i32) >= (192 as i32) {
                                    '__slate_break_1293: while (((unsafe { *zString }) as u32)
                                        as i32)
                                        & (192 as i32)
                                        == (128 as i32)
                                    {
                                        let __v1530: *const u8 = zString;
                                        let __v1531: *const u8 =
                                            unsafe { __v1530.offset((1 as i32) as isize) };
                                        zString = __v1531;
                                    }
                                }
                                {}
                            }
                            return 2 as i32;
                        }
                    }
                }
                // At this point variable c contains the first character of the
                // pattern string past the "*".  Search in the input string for the
                // first matching character and recursively continue the match from
                // that point.
                //
                // For a case-insensitive search, set variable cx to be the same as
                // c but in the other case and search the input string for either
                // c or cx.
                if c < ((128 as i32) as u32) {
                    let mut zStop: [i8; 3] = [0 as i8; 3];
                    let mut bMatch: i32 = 0 as i32;
                    if noCase != (0 as u8) {
                        unsafe {
                            *unsafe {
                                (zStop.as_mut_ptr() as *mut i8).offset((0 as i32) as isize)
                            } = ((c
                                & (!((((unsafe {
                                    *unsafe {
                                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                            .offset((((c as u8) as u32) as i32) as isize)
                                    }
                                }) as u32) as i32)
                                    & (32 as i32)) as u32)) as u8)
                                as i8;
                        }
                        unsafe {
                            *unsafe {
                                (zStop.as_mut_ptr() as *mut i8).offset((1 as i32) as isize)
                            } = (unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }
                                        .offset((((c as u8) as u32) as i32) as isize)
                                }
                            }) as i8;
                        }
                        unsafe {
                            *unsafe {
                                (zStop.as_mut_ptr() as *mut i8).offset((2 as i32) as isize)
                            } = (0 as i32) as i8;
                        }
                    } else {
                        unsafe {
                            *unsafe {
                                (zStop.as_mut_ptr() as *mut i8).offset((0 as i32) as isize)
                            } = (c as u8) as i8;
                        }
                        unsafe {
                            *unsafe {
                                (zStop.as_mut_ptr() as *mut i8).offset((1 as i32) as isize)
                            } = (0 as i32) as i8;
                        }
                    }
                    '__slate_break_1294: while (1 as i32) != (0 as i32) {
                        let __v1532: *const u8 = zString;
                        let __v1533: *const u8 = unsafe {
                            __v1532.offset(
                                (unsafe {
                                    strcspn(
                                        zString as *const i8,
                                        (zStop.as_mut_ptr() as *mut i8) as *const i8,
                                    )
                                }) as isize,
                            )
                        };
                        zString = __v1533;
                        if (((unsafe { *unsafe { zString.offset((0 as i32) as isize) } }) as u32)
                            as i32)
                            == (0 as i32)
                        {
                            break '__slate_break_1294;
                        }
                        let __v1534: *const u8 = zString;
                        let __v1535: *const u8 = unsafe { __v1534.offset((1 as i32) as isize) };
                        zString = __v1535;
                        bMatch = patternCompare(zPattern, zString, pInfo, matchOther);
                        if bMatch != (1 as i32) {
                            return bMatch;
                        }
                    }
                } else {
                    let mut bMatch: i32 = 0 as i32;
                    '__slate_break_1295: loop {
                        let __v1536: u32;
                        if (((unsafe { *unsafe { zString.offset((0 as i32) as isize) } }) as u32)
                            as i32)
                            < (128 as i32)
                        {
                            let __v1537: *const u8 = zString;
                            let __v1538: *const u8 = unsafe { __v1537.offset((1 as i32) as isize) };
                            zString = __v1538;
                            __v1536 = (((unsafe { *__v1537 }) as u32) as i32) as u32;
                        } else {
                            __v1536 = unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(zString)) };
                        }
                        let __v1539: u32 = __v1536;
                        c2 = __v1539;
                        if !(__v1539 != ((0 as i32) as u32)) {
                            break;
                        }
                        if c2 != c {
                        } else {
                            bMatch = patternCompare(zPattern, zString, pInfo, matchOther);
                            if bMatch != (1 as i32) {
                                return bMatch;
                            }
                        }
                    }
                }
                return 2 as i32;
            }
            if c == matchOther {
                if (((unsafe { (*pInfo).matchSet }) as u32) as i32) == (0 as i32) {
                    c = unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(zPattern)) };
                    if c == ((0 as i32) as u32) {
                        return 1 as i32;
                    }
                    zEscaped = zPattern;
                } else {
                    let mut prior_c: u32 = (0 as i32) as u32;
                    let mut seen: i32 = 0 as i32;
                    let mut invert: i32 = 0 as i32;
                    c = unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(zString)) };
                    if c == ((0 as i32) as u32) {
                        return 1 as i32;
                    }
                    c2 = unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(zPattern)) };
                    if c2 == ((94 as i32) as u32) {
                        invert = 1 as i32;
                        c2 = unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(zPattern)) };
                    }
                    if c2 == ((93 as i32) as u32) {
                        if c == ((93 as i32) as u32) {
                            seen = 1 as i32;
                        }
                        c2 = unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(zPattern)) };
                    }
                    '__slate_break_1296: while c2 != (0 as u32) && c2 != ((93 as i32) as u32) {
                        if c2 == ((45 as i32) as u32)
                            && (((unsafe { *unsafe { zPattern.offset((0 as i32) as isize) } })
                                as u32) as i32)
                                != (93 as i32)
                            && (((unsafe { *unsafe { zPattern.offset((0 as i32) as isize) } })
                                as u32) as i32)
                                != (0 as i32)
                            && prior_c > ((0 as i32) as u32)
                        {
                            c2 = unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(zPattern)) };
                            if c >= prior_c && c <= c2 {
                                seen = 1 as i32;
                            }
                            prior_c = (0 as i32) as u32;
                        } else {
                            if c == c2 {
                                seen = 1 as i32;
                            }
                            prior_c = c2;
                        }
                        c2 = unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(zPattern)) };
                    }
                    if c2 == ((0 as i32) as u32) || seen ^ invert == (0 as i32) {
                        return 1 as i32;
                    }
                    break '__slate_continue_1290;
                }
            }
            let __v1540: u32;
            if (((unsafe { *unsafe { zString.offset((0 as i32) as isize) } }) as u32) as i32)
                < (128 as i32)
            {
                let __v1541: *const u8 = zString;
                let __v1542: *const u8 = unsafe { __v1541.offset((1 as i32) as isize) };
                zString = __v1542;
                __v1540 = (((unsafe { *__v1541 }) as u32) as i32) as u32;
            } else {
                __v1540 = unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(zString)) };
            }
            c2 = __v1540;
            if c == c2 {
            } else {
                if noCase != (0 as u8)
                    && (((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }
                                .offset((((c as u8) as u32) as i32) as isize)
                        }
                    }) as u32) as i32)
                        == (((unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }
                                    .offset((((c2 as u8) as u32) as i32) as isize)
                            }
                        }) as u32) as i32)
                    && c < ((128 as i32) as u32)
                    && c2 < ((128 as i32) as u32)
                {
                } else {
                    if c == matchOne && zPattern != zEscaped && c2 != ((0 as i32) as u32) {
                    } else {
                        return 1 as i32;
                    }
                }
            }
        }
    }
    return if (((unsafe { *zString }) as u32) as i32) == (0 as i32) {
        0 as i32
    } else {
        1 as i32
    };
}

/// The sqlite3_strglob() interface.  Return 0 on a match (like strcmp()) and
/// non-zero if there is no match.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.func.sqlite3_strglob")]
extern "C-unwind" fn sqlite3_strglob(mut zGlobPattern: *const i8, mut zString: *const i8) -> i32 {
    if zString == std::ptr::null::<i8>() {
        return (zGlobPattern != std::ptr::null::<i8>()) as i32;
    } else {
        if zGlobPattern == std::ptr::null::<i8>() {
            return 1 as i32;
        } else {
            return patternCompare(
                (zGlobPattern as *mut u8) as *const u8,
                (zString as *mut u8) as *const u8,
                unsafe { std::ptr::addr_of!(globInfo) },
                (91 as i32) as u32,
            );
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// The sqlite3_strlike() interface.  Return 0 on a match and non-zero for
/// a miss - like strcmp().
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.func.sqlite3_strlike")]
extern "C-unwind" fn sqlite3_strlike(
    mut zPattern: *const i8,
    mut zStr: *const i8,
    mut esc: u32,
) -> i32 {
    if zStr == std::ptr::null::<i8>() {
        return (zPattern != std::ptr::null::<i8>()) as i32;
    } else {
        if zPattern == std::ptr::null::<i8>() {
            return 1 as i32;
        } else {
            return patternCompare(
                (zPattern as *mut u8) as *const u8,
                (zStr as *mut u8) as *const u8,
                unsafe { std::ptr::addr_of!(likeInfoNorm) },
                esc,
            );
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Count the number of times that the LIKE operator (or GLOB which is
/// just a variation of LIKE) gets called.  This is used for testing
/// only.
/// Implementation of the like() SQL function.  This function implements
/// the built-in LIKE operator.  The first argument to the function is the
/// pattern and the second argument is the string.  So, the SQL statements:
///
///       A LIKE B
///
/// is implemented as like(B,A).
///
/// This same function (with a different compareInfo structure) computes
/// the GLOB operator.
#[unsafe(link_section = ".text.slate_distinct.func.likeFunc")]
extern "C-unwind" fn likeFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut zA: *const u8 = unsafe { std::mem::zeroed() };
    let mut zB: *const u8 = unsafe { std::mem::zeroed() };
    let mut escape: u32 = 0 as u32;
    let mut nPat: i32 = 0 as i32;
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    let mut pInfo: *mut compareInfo = (unsafe { sqlite3_user_data(context) }) as *mut compareInfo;
    let mut backupInfo: compareInfo = unsafe { std::mem::zeroed() };
    // Limit the length of the LIKE or GLOB pattern to avoid problems
    // of deep recursion and N*N behavior in patternCompare().
    nPat = unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    {}
    {}
    if nPat
        > unsafe {
            *unsafe { unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((8 as i32) as isize) }
        }
    {
        unsafe {
            sqlite3_result_error(
                context,
                (b"LIKE or GLOB pattern too complex\0".as_ptr() as *mut i8) as *const i8,
                -(1 as i32),
            )
        };
        return;
    }
    if argc == (3 as i32) {
        // The escape character string must consist of a single UTF-8 character.
        // Otherwise, return an error.
        let mut zEsc: *const u8 =
            unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((2 as i32) as isize) } }) };
        if zEsc == std::ptr::null::<u8>() {
            return;
        }
        if (unsafe { sqlite3Utf8CharLen((zEsc as *mut i8) as *const i8, -(1 as i32)) })
            != (1 as i32)
        {
            unsafe {
                sqlite3_result_error(
                    context,
                    (b"ESCAPE expression must be a single character\0".as_ptr() as *mut i8)
                        as *const i8,
                    -(1 as i32),
                )
            };
            return;
        }
        escape = unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(zEsc)) };
        if escape == ((((unsafe { (*pInfo).matchAll }) as u32) as i32) as u32)
            || escape == ((((unsafe { (*pInfo).matchOne }) as u32) as i32) as u32)
        {
            unsafe {
                memcpy(
                    std::ptr::addr_of_mut!(backupInfo) as *mut (),
                    pInfo as *const (),
                    4 as u64,
                )
            };
            pInfo = std::ptr::addr_of_mut!(backupInfo);
            if escape == ((((unsafe { (*pInfo).matchAll }) as u32) as i32) as u32) {
                unsafe {
                    (*pInfo).matchAll = ((0 as i32) as i8) as u8;
                }
            }
            if escape == ((((unsafe { (*pInfo).matchOne }) as u32) as i32) as u32) {
                unsafe {
                    (*pInfo).matchOne = ((0 as i32) as i8) as u8;
                }
            }
        }
    } else {
        escape = (unsafe { (*pInfo).matchSet }) as u32;
    }
    zB = unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    zA = unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) };
    if zA != std::ptr::null::<u8>() && zB != std::ptr::null::<u8>() {
        unsafe {
            sqlite3_result_int(
                context,
                (patternCompare(zB, zA, pInfo as *const compareInfo, escape) == (0 as i32)) as i32,
            )
        };
    }
}

/// Implementation of the NULLIF(x,y) function.  The result is the first
/// argument if the arguments are different.  The result is NULL if the
/// arguments are equal to each other.
#[unsafe(link_section = ".text.slate_distinct.func.nullifFunc")]
extern "C-unwind" fn nullifFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut pColl: *mut CollSeq = sqlite3GetFuncCollSeq(context);
    NotUsed;
    if (unsafe {
        sqlite3MemCompare(
            (unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) as *const sqlite3_value,
            (unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) as *const sqlite3_value,
            pColl as *const CollSeq,
        )
    }) != (0 as i32)
    {
        unsafe {
            sqlite3_result_value(context, unsafe {
                *unsafe { argv.offset((0 as i32) as isize) }
            })
        };
    }
}

/// Implementation of the sqlite_version() function.  The result is the version
/// of the SQLite library that is running.
#[unsafe(link_section = ".text.slate_distinct.func.versionFunc")]
extern "C-unwind" fn versionFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    NotUsed;
    NotUsed2;
    // IMP: R-48699-48617 This function is an SQL wrapper around the
    // sqlite3_libversion() C-interface.
    unsafe { sqlite3_result_text(context, unsafe { sqlite3_libversion() }, -(1 as i32), None) };
}

/// Implementation of the sqlite_source_id() function. The result is a string
/// that identifies the particular version of the source code used to build
/// SQLite.
#[unsafe(link_section = ".text.slate_distinct.func.sourceidFunc")]
extern "C-unwind" fn sourceidFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    NotUsed;
    NotUsed2;
    // IMP: R-24470-31136 This function is an SQL wrapper around the
    // sqlite3_sourceid() C interface.
    unsafe { sqlite3_result_text(context, unsafe { sqlite3_sourceid() }, -(1 as i32), None) };
}

/// Implementation of the sqlite_log() function.  This is a wrapper around
/// sqlite3_log().  The return value is NULL.  The function exists purely for
/// its side-effects.
#[unsafe(link_section = ".text.slate_distinct.func.errlogFunc")]
extern "C-unwind" fn errlogFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    argc;
    context;
    unsafe {
        sqlite3_log(
            unsafe { sqlite3_value_int(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) },
            (b"%s\0".as_ptr() as *mut i8) as *const i8,
            unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) },
        )
    };
}

/// Implementation of the sqlite_compileoption_used() function.
/// The result is an integer that identifies if the compiler option
/// was used to build SQLite.
#[unsafe(link_section = ".text.slate_distinct.func.compileoptionusedFunc")]
extern "C-unwind" fn compileoptionusedFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut zOptName: *const i8 = unsafe { std::mem::zeroed() };
    0 as i32;
    argc;
    // IMP: R-39564-36305 The sqlite_compileoption_used() SQL
    // function is a wrapper around the sqlite3_compileoption_used() C/C++
    // function.
    let __v1543: *const i8 =
        (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
            as *const i8;
    zOptName = __v1543;
    if __v1543 != std::ptr::null::<i8>() {
        unsafe { sqlite3_result_int(context, unsafe { sqlite3_compileoption_used(zOptName) }) };
    }
}

/// Implementation of the sqlite_compileoption_get() function.
/// The result is a string that identifies the compiler options
/// used to build SQLite.
#[unsafe(link_section = ".text.slate_distinct.func.compileoptiongetFunc")]
extern "C-unwind" fn compileoptiongetFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut n: i32 = 0 as i32;
    0 as i32;
    argc;
    // IMP: R-04922-24076 The sqlite_compileoption_get() SQL function
    // is a wrapper around the sqlite3_compileoption_get() C/C++ function.
    n = unsafe { sqlite3_value_int(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    unsafe {
        sqlite3_result_text(
            context,
            unsafe { sqlite3_compileoption_get(n) },
            -(1 as i32),
            None,
        )
    };
}

/// Array for converting from half-bytes (nybbles) into ASCII hex
/// digits.
static mut hexdigits: __SlateAlign16<[i8; 16]> = __SlateAlign16([
    (48 as i32) as i8,
    (49 as i32) as i8,
    (50 as i32) as i8,
    (51 as i32) as i8,
    (52 as i32) as i8,
    (53 as i32) as i8,
    (54 as i32) as i8,
    (55 as i32) as i8,
    (56 as i32) as i8,
    (57 as i32) as i8,
    (65 as i32) as i8,
    (66 as i32) as i8,
    (67 as i32) as i8,
    (68 as i32) as i8,
    (69 as i32) as i8,
    (70 as i32) as i8,
]);

/// Append to pStr text that is the SQL literal representation of the
/// value contained in pValue.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3QuoteValue(
    mut pStr: *mut sqlite3_str,
    mut pValue: *mut sqlite3_value,
    mut bEscape: i32,
) {
    // As currently implemented, the string must be initially empty.
    // we might relax this requirement in the future, but that will
    // require enhancements to the implementation.
    0 as i32;
    '__slate_break_1300: {
        match unsafe { sqlite3_value_type(pValue) } {
            2 => {
                //    ,---  Show infinity as 9.0e+999
                // |
                // | ,--- 17 precision guarantees round-trip
                // v v
                unsafe {
                    sqlite3_str_appendf(
                        pStr,
                        (b"%!0.17g\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { sqlite3_value_double(pValue) },
                    )
                };
            }
            1 => {
                unsafe {
                    sqlite3_str_appendf(
                        pStr,
                        (b"%lld\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { sqlite3_value_int64(pValue) },
                    )
                };
            }
            4 => {
                let mut zBlob: *const i8 = (unsafe { sqlite3_value_blob(pValue) }) as *const i8;
                let mut nBlob: i64 = (unsafe { sqlite3_value_bytes(pValue) }) as i64;
                0 as i32; // No encoding change
                unsafe {
                    sqlite3StrAccumEnlarge(pStr, nBlob * ((2 as i32) as i64) + ((4 as i32) as i64))
                };
                if (((unsafe { (*pStr).accError }) as u32) as i32) == (0 as i32) {
                    let mut zText: *mut i8 = unsafe { (*pStr).zText };
                    let mut i: i32 = 0 as i32;
                    i = 0 as i32;
                    '__slate_break_1303: loop {
                        if !((i as i64) < nBlob) {
                            break;
                        }
                        unsafe {
                            *unsafe { zText.offset((i * (2 as i32) + (2 as i32)) as isize) } = unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(hexdigits.0) as *const i8 }.offset(
                                        (((unsafe { *unsafe { zBlob.offset(i as isize) } }) as i32)
                                            >> (4 as i32)
                                            & (15 as i32))
                                            as isize,
                                    )
                                }
                            };
                        }
                        unsafe {
                            *unsafe { zText.offset((i * (2 as i32) + (3 as i32)) as isize) } = unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(hexdigits.0) as *const i8 }.offset(
                                        (((unsafe { *unsafe { zBlob.offset(i as isize) } }) as i32)
                                            & (15 as i32))
                                            as isize,
                                    )
                                }
                            };
                        }
                        let __v1458: i32 = i;
                        let __v1459: i32 = __v1458 + (1 as i32);
                        i = __v1459;
                    }
                    unsafe {
                        *unsafe {
                            zText.offset(
                                (nBlob * ((2 as i32) as i64) + ((2 as i32) as i64)) as isize,
                            )
                        } = (39 as i32) as i8;
                    }
                    unsafe {
                        *unsafe {
                            zText.offset(
                                (nBlob * ((2 as i32) as i64) + ((3 as i32) as i64)) as isize,
                            )
                        } = (0 as i32) as i8;
                    }
                    unsafe {
                        *unsafe { zText.offset((0 as i32) as isize) } = (88 as i32) as i8;
                    }
                    unsafe {
                        *unsafe { zText.offset((1 as i32) as isize) } = (39 as i32) as i8;
                    }
                    unsafe {
                        (*pStr).nChar =
                            ((nBlob * ((2 as i32) as i64) + ((3 as i32) as i64)) as i32) as u32;
                    }
                }
            }
            3 => {
                let mut zArg: *const u8 = unsafe { sqlite3_value_text(pValue) };
                unsafe {
                    sqlite3_str_appendf(
                        pStr,
                        (if bEscape != (0 as i32) {
                            b"%#Q\0".as_ptr() as *mut i8
                        } else {
                            b"%Q\0".as_ptr() as *mut i8
                        }) as *const i8,
                        zArg,
                    )
                };
            }
            _ => {
                0 as i32;
                unsafe {
                    sqlite3_str_append(pStr, (b"NULL\0".as_ptr() as *mut i8) as *const i8, 4 as i32)
                };
            }
        }
    }
}

/// Return true if z[] begins with N hexadecimal digits, and write
/// a decoding of those digits into *pVal.  Or return false if any
/// one of the first N characters in z[] is not a hexadecimal digit.
fn isNHex(mut z: *const i8, mut N: i32, mut pVal: *mut u32) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut v: u32 = (0 as i32) as u32;
    i = 0 as i32;
    '__slate_break_1307: loop {
        if !(i < N) {
            break;
        }
        if !((((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                    ((((unsafe { *unsafe { z.offset(i as isize) } }) as u8) as u32) as i32)
                        as isize,
                )
            }
        }) as u32) as i32)
            & (8 as i32)
            != (0 as i32))
        {
            return 0 as i32;
        }
        v = (v << (4 as i32)).wrapping_add(
            (((unsafe { sqlite3HexToInt((unsafe { *unsafe { z.offset(i as isize) } }) as i32) })
                as u32) as i32) as u32,
        );
        let __v1544: i32 = i;
        let __v1545: i32 = __v1544 + (1 as i32);
        i = __v1545;
    }
    unsafe {
        *pVal = v;
    }
    return 1 as i32;
}

/// Implementation of the UNISTR() function.
///
/// This is intended to be a work-alike of the UNISTR() function in
/// PostgreSQL.  Quoting from the PG documentation (PostgreSQL 17 -
/// scraped on 2025-02-22):
///
///    Evaluate escaped Unicode characters in the argument. Unicode
///    characters can be specified as \XXXX (4 hexadecimal digits),
///    \+XXXXXX (6 hexadecimal digits), \uXXXX (4 hexadecimal digits),
///    or \UXXXXXXXX (8 hexadecimal digits). To specify a backslash,
///    write two backslashes. All other characters are taken literally.
#[unsafe(link_section = ".text.slate_distinct.func.unistrFunc")]
extern "C-unwind" fn unistrFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut __slate_storage_1555: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1555: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1555) as *mut i32;
    let mut __slate_storage_1554: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1554: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1554) as *mut i32;
    let mut __slate_storage_1553: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1553: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1553) as *mut i32;
    let mut __slate_storage_1552: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1552: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1552) as *mut i32;
    let mut __slate_storage_1559: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1559: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1559) as *mut i32;
    let mut __slate_storage_1558: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1558: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1558) as *mut i32;
    let mut __slate_storage_1557: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1557: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1557) as *mut i32;
    let mut __slate_storage_1556: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1556: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1556) as *mut i32;
    let mut __slate_storage_1563: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1563: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1563) as *mut i32;
    let mut __slate_storage_1562: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1562: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1562) as *mut i32;
    let mut __slate_storage_1561: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1561: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1561) as *mut i32;
    let mut __slate_storage_1560: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1560: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1560) as *mut i32;
    let mut __slate_storage_1567: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1567: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1567) as *mut i32;
    let mut __slate_storage_1566: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1566: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1566) as *mut i32;
    let mut __slate_storage_1565: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1565: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1565) as *mut i32;
    let mut __slate_storage_1564: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1564: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1564) as *mut i32;
    let mut __slate_storage_1571: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1571: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1571) as *mut i32;
    let mut __slate_storage_1570: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1570: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1570) as *mut i32;
    let mut __slate_storage_1569: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1569: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1569) as *mut i32;
    let mut __slate_storage_1568: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1568: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1568) as *mut i32;
    let mut __slate_storage_1551: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1551: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1551) as *mut i32;
    let mut __slate_storage_1550: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1550: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1550) as *mut i32;
    let mut __slate_storage_1549: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1549: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1549) as *mut i32;
    let mut __slate_storage_1548: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1548: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1548) as *mut i32;
    let mut __slate_storage_1547: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1547: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1547) as *mut i32;
    let mut __slate_storage_1546: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1546: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1546) as *mut i32;
    let mut __slate_storage_706: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_706: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_706) as *mut *const i8;
    let mut __slate_storage_705: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_705: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_705) as *mut u32;
    let mut __slate_storage_704: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_704: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_704) as *mut i32;
    let mut __slate_storage_703: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_703: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_703) as *mut i32;
    let mut __slate_storage_702: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_702: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_702) as *mut i32;
    let mut __slate_storage_701: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_701: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_701) as *mut i32;
    let mut __slate_storage_700: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_700: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_700) as *mut *const i8;
    let mut __slate_storage_699: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_699: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_699) as *mut *mut i8;
    unsafe {
        0 as i32;
        argc;
        *__slate_slot_700 = (unsafe {
            sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
        }) as *const i8;
        if *__slate_slot_700 == std::ptr::null::<i8>() {
            return;
        } else {
            *__slate_slot_701 = unsafe {
                sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
            };
            *__slate_slot_699 =
                (unsafe { sqlite3_malloc64(((*__slate_slot_701 + (1 as i32)) as i64) as u64) })
                    as *mut i8;
            if *__slate_slot_699 == std::ptr::null_mut::<i8>() {
                unsafe { sqlite3_result_error_nomem(context) };
                return;
            } else {
                *__slate_slot_703 = 0 as i32;
                *__slate_slot_702 = 0 as i32;
                '__join_1: {
                    '__join_19: {
                        loop {
                            if *__slate_slot_702 < *__slate_slot_701 {
                                std::ptr::write(
                                    __slate_slot_706,
                                    (unsafe {
                                        strchr(
                                            unsafe {
                                                (*__slate_slot_700)
                                                    .offset(*__slate_slot_702 as isize)
                                            },
                                            92 as i32,
                                        )
                                    }) as *const i8,
                                );
                                if *__slate_slot_706 == std::ptr::null::<i8>() {
                                    break '__join_19;
                                } else {
                                    *__slate_slot_704 =
                                        ((unsafe {
                                            (*__slate_slot_706).offset_from(
                                                (unsafe {
                                                    (*__slate_slot_700)
                                                        .offset(*__slate_slot_702 as isize)
                                                })
                                                    as *const i8,
                                            )
                                        }) as i64) as i32;
                                    if *__slate_slot_704 > (0 as i32) {
                                        unsafe {
                                            memmove(
                                                (unsafe {
                                                    (*__slate_slot_699)
                                                        .offset(*__slate_slot_703 as isize)
                                                })
                                                    as *mut (),
                                                (unsafe {
                                                    (*__slate_slot_700)
                                                        .offset(*__slate_slot_702 as isize)
                                                })
                                                    as *const (),
                                                (*__slate_slot_704 as i64) as u64,
                                            )
                                        };
                                        std::ptr::write(__slate_slot_1548, *__slate_slot_703);
                                        std::ptr::write(
                                            __slate_slot_1549,
                                            *__slate_slot_1548 + *__slate_slot_704,
                                        );
                                        *__slate_slot_703 = *__slate_slot_1549;
                                        std::ptr::write(__slate_slot_1550, *__slate_slot_702);
                                        std::ptr::write(
                                            __slate_slot_1551,
                                            *__slate_slot_1550 + *__slate_slot_704,
                                        );
                                        *__slate_slot_702 = *__slate_slot_1551;
                                    }
                                    if ((unsafe {
                                        *unsafe {
                                            (*__slate_slot_700)
                                                .offset((*__slate_slot_702 + (1 as i32)) as isize)
                                        }
                                    }) as i32)
                                        == (92 as i32)
                                    {
                                        std::ptr::write(__slate_slot_1552, *__slate_slot_702);
                                        std::ptr::write(
                                            __slate_slot_1553,
                                            *__slate_slot_1552 + (2 as i32),
                                        );
                                        *__slate_slot_702 = *__slate_slot_1553;
                                        std::ptr::write(__slate_slot_1554, *__slate_slot_703);
                                        std::ptr::write(
                                            __slate_slot_1555,
                                            *__slate_slot_1554 + (1 as i32),
                                        );
                                        *__slate_slot_703 = *__slate_slot_1555;
                                        unsafe {
                                            *unsafe {
                                                (*__slate_slot_699)
                                                    .offset(*__slate_slot_1554 as isize)
                                            } = (92 as i32) as i8;
                                        }
                                    } else {
                                        if (((unsafe {
                                            *unsafe {
                                                unsafe {
                                                    std::ptr::addr_of!(sqlite3CtypeMap) as *const u8
                                                }
                                                .offset(
                                                    ((((unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_700).offset(
                                                                (*__slate_slot_702 + (1 as i32))
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
                                        }) as u32)
                                            as i32)
                                            & (8 as i32)
                                            != (0 as i32)
                                        {
                                            if !(isNHex(
                                                unsafe {
                                                    (*__slate_slot_700).offset(
                                                        (*__slate_slot_702 + (1 as i32)) as isize,
                                                    )
                                                },
                                                4 as i32,
                                                std::ptr::addr_of_mut!(*__slate_slot_705),
                                            ) != (0 as i32))
                                            {
                                                break;
                                            } else {
                                                std::ptr::write(
                                                    __slate_slot_1556,
                                                    *__slate_slot_702,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_1557,
                                                    *__slate_slot_1556 + (5 as i32),
                                                );
                                                *__slate_slot_702 = *__slate_slot_1557;
                                                std::ptr::write(
                                                    __slate_slot_1558,
                                                    *__slate_slot_703,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_1559,
                                                    *__slate_slot_1558
                                                        + unsafe {
                                                            sqlite3AppendOneUtf8Character(
                                                                unsafe {
                                                                    (*__slate_slot_699).offset(
                                                                        *__slate_slot_703 as isize,
                                                                    )
                                                                },
                                                                *__slate_slot_705,
                                                            )
                                                        },
                                                );
                                                *__slate_slot_703 = *__slate_slot_1559;
                                            }
                                        } else {
                                            if ((unsafe {
                                                *unsafe {
                                                    (*__slate_slot_700).offset(
                                                        (*__slate_slot_702 + (1 as i32)) as isize,
                                                    )
                                                }
                                            })
                                                as i32)
                                                == (43 as i32)
                                            {
                                                if !(isNHex(
                                                    unsafe {
                                                        (*__slate_slot_700).offset(
                                                            (*__slate_slot_702 + (2 as i32))
                                                                as isize,
                                                        )
                                                    },
                                                    6 as i32,
                                                    std::ptr::addr_of_mut!(*__slate_slot_705),
                                                ) != (0 as i32))
                                                {
                                                    break;
                                                } else {
                                                    std::ptr::write(
                                                        __slate_slot_1560,
                                                        *__slate_slot_702,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1561,
                                                        *__slate_slot_1560 + (8 as i32),
                                                    );
                                                    *__slate_slot_702 = *__slate_slot_1561;
                                                    std::ptr::write(
                                                        __slate_slot_1562,
                                                        *__slate_slot_703,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1563,
                                                        *__slate_slot_1562
                                                            + unsafe {
                                                                sqlite3AppendOneUtf8Character(
                                                                    unsafe {
                                                                        (*__slate_slot_699).offset(
                                                                            *__slate_slot_703
                                                                                as isize,
                                                                        )
                                                                    },
                                                                    *__slate_slot_705,
                                                                )
                                                            },
                                                    );
                                                    *__slate_slot_703 = *__slate_slot_1563;
                                                }
                                            } else {
                                                if ((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_700).offset(
                                                            (*__slate_slot_702 + (1 as i32))
                                                                as isize,
                                                        )
                                                    }
                                                })
                                                    as i32)
                                                    == (117 as i32)
                                                {
                                                    if !(isNHex(
                                                        unsafe {
                                                            (*__slate_slot_700).offset(
                                                                (*__slate_slot_702 + (2 as i32))
                                                                    as isize,
                                                            )
                                                        },
                                                        4 as i32,
                                                        std::ptr::addr_of_mut!(*__slate_slot_705),
                                                    ) != (0 as i32))
                                                    {
                                                        break;
                                                    } else {
                                                        std::ptr::write(
                                                            __slate_slot_1564,
                                                            *__slate_slot_702,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1565,
                                                            *__slate_slot_1564 + (6 as i32),
                                                        );
                                                        *__slate_slot_702 = *__slate_slot_1565;
                                                        std::ptr::write(
                                                            __slate_slot_1566,
                                                            *__slate_slot_703,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1567,
                                                            *__slate_slot_1566
                                                                + unsafe {
                                                                    sqlite3AppendOneUtf8Character(
                                                                        unsafe {
                                                                            (*__slate_slot_699)
                                                                                .offset(
                                                                                *__slate_slot_703
                                                                                    as isize,
                                                                            )
                                                                        },
                                                                        *__slate_slot_705,
                                                                    )
                                                                },
                                                        );
                                                        *__slate_slot_703 = *__slate_slot_1567;
                                                    }
                                                } else {
                                                    if ((unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_700).offset(
                                                                (*__slate_slot_702 + (1 as i32))
                                                                    as isize,
                                                            )
                                                        }
                                                    })
                                                        as i32)
                                                        == (85 as i32)
                                                    {
                                                        if !(isNHex(
                                                            unsafe {
                                                                (*__slate_slot_700).offset(
                                                                    (*__slate_slot_702 + (2 as i32))
                                                                        as isize,
                                                                )
                                                            },
                                                            8 as i32,
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_705
                                                            ),
                                                        ) != (0 as i32))
                                                        {
                                                            break;
                                                        } else {
                                                            std::ptr::write(
                                                                __slate_slot_1568,
                                                                *__slate_slot_702,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1569,
                                                                *__slate_slot_1568 + (10 as i32),
                                                            );
                                                            *__slate_slot_702 = *__slate_slot_1569;
                                                            std::ptr::write(
                                                                __slate_slot_1570,
                                                                *__slate_slot_703,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1571,
                                                                *__slate_slot_1570
                                                                    + unsafe {
                                                                        sqlite3AppendOneUtf8Character(unsafe { (*__slate_slot_699).offset(*__slate_slot_703 as isize) }, *__slate_slot_705)
                                                                    },
                                                            );
                                                            *__slate_slot_703 = *__slate_slot_1571;
                                                        }
                                                    } else {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                break '__join_1;
                            }
                        }
                        unsafe { sqlite3_free(*__slate_slot_699 as *mut ()) };
                        unsafe {
                            sqlite3_result_error(
                                context,
                                (b"invalid Unicode escape\0".as_ptr() as *mut i8) as *const i8,
                                -(1 as i32),
                            )
                        };
                        return;
                    }
                    *__slate_slot_704 = *__slate_slot_701 - *__slate_slot_702;
                    unsafe {
                        memmove(
                            (unsafe { (*__slate_slot_699).offset(*__slate_slot_703 as isize) })
                                as *mut (),
                            (unsafe { (*__slate_slot_700).offset(*__slate_slot_702 as isize) })
                                as *const (),
                            (*__slate_slot_704 as i64) as u64,
                        )
                    };
                    std::ptr::write(__slate_slot_1546, *__slate_slot_703);
                    std::ptr::write(__slate_slot_1547, *__slate_slot_1546 + *__slate_slot_704);
                    *__slate_slot_703 = *__slate_slot_1547;
                }
                unsafe {
                    *unsafe { (*__slate_slot_699).offset(*__slate_slot_703 as isize) } =
                        (0 as i32) as i8;
                }
                unsafe {
                    sqlite3_result_text64(
                        context,
                        *__slate_slot_699 as *const i8,
                        (*__slate_slot_703 as i64) as u64,
                        unsafe {
                            std::mem::transmute::<
                                *const (),
                                Option<unsafe extern "C-unwind" fn(*mut ())>,
                            >(sqlite3_free as *const ())
                        },
                        ((16 as i32) as i8) as u8,
                    )
                };
                return;
            }
        }
    }
}

/// Implementation of the QUOTE() function.
///
/// The quote(X) function returns the text of an SQL literal which is the
/// value of its argument suitable for inclusion into an SQL statement.
/// Strings are surrounded by single-quotes with escapes on interior quotes
/// as needed. BLOBs are encoded as hexadecimal literals. Strings with
/// embedded NUL characters cannot be represented as string literals in SQL
/// and hence the returned string literal is truncated prior to the first NUL.
///
/// If sqlite3_user_data() is non-zero, then the UNISTR_QUOTE() function is
/// implemented instead.  The difference is that UNISTR_QUOTE() uses the
/// UNISTR() function to escape control characters.
#[unsafe(link_section = ".text.slate_distinct.func.quoteFunc")]
extern "C-unwind" fn quoteFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut str: sqlite3_str = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    0 as i32;
    argc;
    unsafe {
        sqlite3StrAccumInit(
            std::ptr::addr_of_mut!(str),
            db,
            std::ptr::null_mut::<i8>(),
            0 as i32,
            unsafe {
                *unsafe {
                    unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize)
                }
            },
        )
    };
    sqlite3QuoteValue(
        std::ptr::addr_of_mut!(str),
        unsafe { *unsafe { argv.offset((0 as i32) as isize) } },
        ((unsafe { sqlite3_user_data(context) }) as i64) as i32,
    );
    unsafe { sqlite3_result_str(context, std::ptr::addr_of_mut!(str), 1 as i32) };
}

/// The unicode() function.  Return the integer unicode code-point value
/// for the first character of the input string.
#[unsafe(link_section = ".text.slate_distinct.func.unicodeFunc")]
extern "C-unwind" fn unicodeFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut z: *const u8 =
        unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    argc;
    if z != std::ptr::null::<u8>()
        && (unsafe { *unsafe { z.offset((0 as i32) as isize) } }) != (0 as u8)
    {
        unsafe {
            sqlite3_result_int(
                context,
                (unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(z)) }) as i32,
            )
        };
    }
}

/// The char() function takes zero or more arguments, each of which is
/// an integer.  It constructs a string where each character of the string
/// is the unicode character for the corresponding integer argument.
#[unsafe(link_section = ".text.slate_distinct.func.charFunc")]
extern "C-unwind" fn charFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut z: *mut u8 = unsafe { std::mem::zeroed() };
    let mut zOut: *mut u8 = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let __v1572: *mut u8 =
        (unsafe { sqlite3_malloc64(((argc * (4 as i32) + (1 as i32)) as i64) as u64) }) as *mut u8;
    z = __v1572;
    zOut = __v1572;
    if z == std::ptr::null_mut::<u8>() {
        unsafe { sqlite3_result_error_nomem(context) };
        return;
    }
    i = 0 as i32;
    '__slate_break_1310: loop {
        if !(i < argc) {
            break;
        }
        let mut x: i64 = 0 as i64;
        let mut c: u32 = 0 as u32;
        x = unsafe { sqlite3_value_int64(unsafe { *unsafe { argv.offset(i as isize) } }) };
        if x < ((0 as i32) as i64) || x > ((1114111 as i32) as i64) {
            x = (65533 as i32) as i64;
        }
        c = ((x & ((2097151 as i32) as i64)) as i32) as u32;
        if c < ((128 as i32) as u32) {
            let __v1575: *mut u8 = zOut;
            let __v1576: *mut u8 = unsafe { __v1575.offset((1 as i32) as isize) };
            zOut = __v1576;
            unsafe {
                *__v1575 = (c & ((255 as i32) as u32)) as u8;
            }
        } else {
            if c < ((2048 as i32) as u32) {
                let __v1577: *mut u8 = zOut;
                let __v1578: *mut u8 = unsafe { __v1577.offset((1 as i32) as isize) };
                zOut = __v1578;
                unsafe {
                    *__v1577 = (((192 as i32)
                        + ((((c >> (6 as i32) & ((31 as i32) as u32)) as u8) as u32) as i32))
                        as i8) as u8;
                }
                let __v1579: *mut u8 = zOut;
                let __v1580: *mut u8 = unsafe { __v1579.offset((1 as i32) as isize) };
                zOut = __v1580;
                unsafe {
                    *__v1579 = (((128 as i32)
                        + ((((c & ((63 as i32) as u32)) as u8) as u32) as i32))
                        as i8) as u8;
                }
            } else {
                if c < ((65536 as i32) as u32) {
                    let __v1581: *mut u8 = zOut;
                    let __v1582: *mut u8 = unsafe { __v1581.offset((1 as i32) as isize) };
                    zOut = __v1582;
                    unsafe {
                        *__v1581 = (((224 as i32)
                            + ((((c >> (12 as i32) & ((15 as i32) as u32)) as u8) as u32) as i32))
                            as i8) as u8;
                    }
                    let __v1583: *mut u8 = zOut;
                    let __v1584: *mut u8 = unsafe { __v1583.offset((1 as i32) as isize) };
                    zOut = __v1584;
                    unsafe {
                        *__v1583 = (((128 as i32)
                            + ((((c >> (6 as i32) & ((63 as i32) as u32)) as u8) as u32) as i32))
                            as i8) as u8;
                    }
                    let __v1585: *mut u8 = zOut;
                    let __v1586: *mut u8 = unsafe { __v1585.offset((1 as i32) as isize) };
                    zOut = __v1586;
                    unsafe {
                        *__v1585 = (((128 as i32)
                            + ((((c & ((63 as i32) as u32)) as u8) as u32) as i32))
                            as i8) as u8;
                    }
                } else {
                    let __v1587: *mut u8 = zOut;
                    let __v1588: *mut u8 = unsafe { __v1587.offset((1 as i32) as isize) };
                    zOut = __v1588;
                    unsafe {
                        *__v1587 = (((240 as i32)
                            + ((((c >> (18 as i32) & ((7 as i32) as u32)) as u8) as u32) as i32))
                            as i8) as u8;
                    }
                    let __v1589: *mut u8 = zOut;
                    let __v1590: *mut u8 = unsafe { __v1589.offset((1 as i32) as isize) };
                    zOut = __v1590;
                    unsafe {
                        *__v1589 = (((128 as i32)
                            + ((((c >> (12 as i32) & ((63 as i32) as u32)) as u8) as u32) as i32))
                            as i8) as u8;
                    }
                    let __v1591: *mut u8 = zOut;
                    let __v1592: *mut u8 = unsafe { __v1591.offset((1 as i32) as isize) };
                    zOut = __v1592;
                    unsafe {
                        *__v1591 = (((128 as i32)
                            + ((((c >> (6 as i32) & ((63 as i32) as u32)) as u8) as u32) as i32))
                            as i8) as u8;
                    }
                    let __v1593: *mut u8 = zOut;
                    let __v1594: *mut u8 = unsafe { __v1593.offset((1 as i32) as isize) };
                    zOut = __v1594;
                    unsafe {
                        *__v1593 = (((128 as i32)
                            + ((((c & ((63 as i32) as u32)) as u8) as u32) as i32))
                            as i8) as u8;
                    }
                }
            }
        }
        let __v1573: i32 = i;
        let __v1574: i32 = __v1573 + (1 as i32);
        i = __v1574;
    }
    unsafe {
        *zOut = ((0 as i32) as i8) as u8;
    }
    unsafe {
        sqlite3_result_text64(
            context,
            (z as *mut i8) as *const i8,
            ((unsafe { zOut.offset_from(z as *mut u8) }) as i64) as u64,
            unsafe {
                std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    sqlite3_free as *const (),
                )
            },
            ((16 as i32) as i8) as u8,
        )
    };
}

/// The hex() function.  Interpret the argument as a blob.  Return
/// a hexadecimal rendering as text.
#[unsafe(link_section = ".text.slate_distinct.func.hexFunc")]
extern "C-unwind" fn hexFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut i: i32 = 0 as i32;
    let mut n: i32 = 0 as i32;
    let mut pBlob: *const u8 = unsafe { std::mem::zeroed() };
    let mut zHex: *mut i8 = unsafe { std::mem::zeroed() };
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    0 as i32;
    argc;
    pBlob = (unsafe { sqlite3_value_blob(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
        as *const u8;
    n = unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    0 as i32; // No encoding change
    let __v1595: *mut i8 = contextMalloc(
        context,
        (n as i64) * ((2 as i32) as i64) + ((1 as i32) as i64),
    ) as *mut i8;
    zHex = __v1595;
    z = __v1595;
    if zHex != std::ptr::null_mut::<i8>() {
        i = 0 as i32;
        '__slate_break_1311: while i < n {
            let mut c: u8 = unsafe { *pBlob };
            let __v1600: *mut i8 = z;
            let __v1601: *mut i8 = unsafe { __v1600.offset((1 as i32) as isize) };
            z = __v1601;
            unsafe {
                *__v1600 = unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(hexdigits.0) as *const i8 }
                            .offset((((c as u32) as i32) >> (4 as i32) & (15 as i32)) as isize)
                    }
                };
            }
            let __v1602: *mut i8 = z;
            let __v1603: *mut i8 = unsafe { __v1602.offset((1 as i32) as isize) };
            z = __v1603;
            unsafe {
                *__v1602 = unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(hexdigits.0) as *const i8 }
                            .offset((((c as u32) as i32) & (15 as i32)) as isize)
                    }
                };
            }
            let __v1596: i32 = i;
            let __v1597: i32 = __v1596 + (1 as i32);
            i = __v1597;
            let __v1598: *const u8 = pBlob;
            let __v1599: *const u8 = unsafe { __v1598.offset((1 as i32) as isize) };
            pBlob = __v1599;
        }
        unsafe {
            *z = (0 as i32) as i8;
        }
        unsafe {
            sqlite3_result_text64(
                context,
                zHex as *const i8,
                ((unsafe { z.offset_from(zHex as *mut i8) }) as i64) as u64,
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        sqlite3_free as *const (),
                    )
                },
                ((16 as i32) as i8) as u8,
            )
        };
    }
}

/// Buffer zStr contains nStr bytes of utf-8 encoded text. Return 1 if zStr
/// contains character ch, or 0 if it does not.
fn strContainsChar(mut zStr: *const u8, mut nStr: i32, mut ch: u32) -> i32 {
    let mut zEnd: *const u8 = unsafe { zStr.offset(nStr as isize) };
    let mut z: *const u8 = zStr;
    '__slate_break_1312: while z < zEnd {
        let mut tst: u32 = 0 as u32;
        let __v1604: u32;
        if (((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32) as i32) < (128 as i32) {
            let __v1605: *const u8 = z;
            let __v1606: *const u8 = unsafe { __v1605.offset((1 as i32) as isize) };
            z = __v1606;
            __v1604 = (((unsafe { *__v1605 }) as u32) as i32) as u32;
        } else {
            __v1604 = unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(z)) };
        }
        tst = __v1604;
        if tst == ch {
            return 1 as i32;
        }
    }
    return 0 as i32;
}

/// The unhex() function. This function may be invoked with either one or
/// two arguments. In both cases the first argument is interpreted as text
/// a text value containing a set of pairs of hexadecimal digits which are
/// decoded and returned as a blob.
///
/// If there is only a single argument, then it must consist only of an
/// even number of hexadecimal digits. Otherwise, return NULL.
///
/// Or, if there is a second argument, then any character that appears in
/// the second argument is also allowed to appear between pairs of hexadecimal
/// digits in the first argument. If any other character appears in the
/// first argument, or if one of the allowed characters appears between
/// two hexadecimal digits that make up a single byte, NULL is returned.
///
/// The following expressions are all true:
///
///     unhex('ABCD')       IS x'ABCD'
///     unhex('AB CD')      IS NULL
///     unhex('AB CD', ' ') IS x'ABCD'
///     unhex('A BCD', ' ') IS NULL
#[unsafe(link_section = ".text.slate_distinct.func.unhexFunc")]
extern "C-unwind" fn unhexFunc(
    mut pCtx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut __slate_storage_1608: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1608: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1608) as *mut u8;
    let mut __slate_storage_1617: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1617: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1617) as *mut *mut u8;
    let mut __slate_storage_1616: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1616: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1616) as *mut *mut u8;
    let mut __slate_storage_1615: std::mem::MaybeUninit<*const u8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1615: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_1615) as *mut *const u8;
    let mut __slate_storage_1614: std::mem::MaybeUninit<*const u8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1614: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_1614) as *mut *const u8;
    let mut __slate_storage_1613: std::mem::MaybeUninit<*const u8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1613: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_1613) as *mut *const u8;
    let mut __slate_storage_1612: std::mem::MaybeUninit<*const u8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1612: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_1612) as *mut *const u8;
    let mut __slate_storage_1611: std::mem::MaybeUninit<*const u8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1611: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_1611) as *mut *const u8;
    let mut __slate_storage_1610: std::mem::MaybeUninit<*const u8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1610: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_1610) as *mut *const u8;
    let mut __slate_storage_1609: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1609: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1609) as *mut u32;
    let mut __slate_storage_758: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_758: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_758) as *mut u32; // Least significant digit of next byte
    let mut __slate_storage_757: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_757: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_757) as *mut u8; // Most significant digit of next byte
    let mut __slate_storage_756: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_756: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_756) as *mut u8;
    let mut __slate_storage_1607: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1607: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1607) as *mut *mut u8;
    let mut __slate_storage_755: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_755: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_755) as *mut *mut u8;
    let mut __slate_storage_754: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_754: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_754) as *mut *mut u8;
    let mut __slate_storage_753: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_753: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_753) as *mut i32;
    let mut __slate_storage_752: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_752: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_752) as *mut *const u8;
    let mut __slate_storage_751: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_751: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_751) as *mut i32;
    let mut __slate_storage_750: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_750: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_750) as *mut *const u8;
    unsafe {
        '__join_13: {
            std::ptr::write(__slate_slot_750, (b"\0".as_ptr() as *mut i8) as *const u8);
            std::ptr::write(__slate_slot_751, 0 as i32);
            std::ptr::write(__slate_slot_752, unsafe {
                sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
            });
            std::ptr::write(__slate_slot_753, unsafe {
                sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
            });
            std::ptr::write(__slate_slot_754, std::ptr::null_mut::<u8>());
            std::ptr::write(__slate_slot_755, std::ptr::null_mut::<u8>());
            0 as i32;
            if argc == (2 as i32) {
                *__slate_slot_750 = unsafe {
                    sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
                };
                *__slate_slot_751 = unsafe {
                    sqlite3_value_bytes(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
                };
            }
        }
        if !(*__slate_slot_752 != std::ptr::null::<u8>())
            || !(*__slate_slot_750 != std::ptr::null::<u8>())
        {
            return;
        } else {
            '__join_1: {
                std::ptr::write(
                    __slate_slot_1607,
                    contextMalloc(pCtx, (*__slate_slot_753 / (2 as i32) + (1 as i32)) as i64)
                        as *mut u8,
                );
                *__slate_slot_754 = *__slate_slot_1607;
                *__slate_slot_755 = *__slate_slot_1607;
                if *__slate_slot_754 != std::ptr::null_mut::<u8>() {
                    '__loop_2: loop {
                        std::ptr::write(__slate_slot_1608, unsafe { *(*__slate_slot_752) });
                        *__slate_slot_756 = *__slate_slot_1608;
                        if ((*__slate_slot_1608 as u32) as i32) != (0 as i32) {
                            loop {
                                if !((((unsafe {
                                    *unsafe {
                                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                            .offset(((*__slate_slot_756 as u32) as i32) as isize)
                                    }
                                }) as u32) as i32)
                                    & (8 as i32)
                                    != (0 as i32))
                                {
                                    if (((unsafe {
                                        *unsafe { (*__slate_slot_752).offset((0 as i32) as isize) }
                                    }) as u32) as i32)
                                        < (128 as i32)
                                    {
                                        std::ptr::write(__slate_slot_1610, *__slate_slot_752);
                                        std::ptr::write(__slate_slot_1611, unsafe {
                                            (*__slate_slot_1610).offset((1 as i32) as isize)
                                        });
                                        *__slate_slot_752 = *__slate_slot_1611;
                                        *__slate_slot_1609 =
                                            (((unsafe { *(*__slate_slot_1610) }) as u32) as i32)
                                                as u32;
                                    } else {
                                        *__slate_slot_1609 = unsafe {
                                            sqlite3Utf8Read(std::ptr::addr_of_mut!(
                                                *__slate_slot_752
                                            ))
                                        };
                                    }
                                    *__slate_slot_758 = *__slate_slot_1609;
                                    0 as i32;
                                    if !(strContainsChar(
                                        *__slate_slot_750,
                                        *__slate_slot_751,
                                        *__slate_slot_758,
                                    ) != (0 as i32))
                                    {
                                        break '__loop_2;
                                    } else {
                                        *__slate_slot_756 = unsafe { *(*__slate_slot_752) };
                                        if ((*__slate_slot_756 as u32) as i32) == (0 as i32) {
                                            break '__join_1;
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                            std::ptr::write(__slate_slot_1612, *__slate_slot_752);
                            std::ptr::write(__slate_slot_1613, unsafe {
                                (*__slate_slot_1612).offset((1 as i32) as isize)
                            });
                            *__slate_slot_752 = *__slate_slot_1613;
                            0 as i32;
                            0 as i32;
                            std::ptr::write(__slate_slot_1614, *__slate_slot_752);
                            std::ptr::write(__slate_slot_1615, unsafe {
                                (*__slate_slot_1614).offset((1 as i32) as isize)
                            });
                            *__slate_slot_752 = *__slate_slot_1615;
                            *__slate_slot_757 = unsafe { *(*__slate_slot_1614) };
                            if !((((unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                        .offset(((*__slate_slot_757 as u32) as i32) as isize)
                                }
                            }) as u32) as i32)
                                & (8 as i32)
                                != (0 as i32))
                            {
                                break;
                            } else {
                                std::ptr::write(__slate_slot_1616, *__slate_slot_755);
                                std::ptr::write(__slate_slot_1617, unsafe {
                                    (*__slate_slot_1616).offset((1 as i32) as isize)
                                });
                                *__slate_slot_755 = *__slate_slot_1617;
                                unsafe {
                                    *(*__slate_slot_1616) = (((((unsafe {
                                        sqlite3HexToInt((*__slate_slot_756 as u32) as i32)
                                    })
                                        as u32)
                                        as i32)
                                        << (4 as i32)
                                        | (((unsafe {
                                            sqlite3HexToInt((*__slate_slot_757 as u32) as i32)
                                        }) as u32)
                                            as i32))
                                        as i8)
                                        as u8;
                                }
                            }
                        } else {
                            break '__join_1;
                        }
                    }
                    unsafe { sqlite3_free(*__slate_slot_754 as *mut ()) };
                    return;
                }
            }
            unsafe {
                sqlite3_result_blob(
                    pCtx,
                    *__slate_slot_754 as *const (),
                    ((unsafe { (*__slate_slot_755).offset_from(*__slate_slot_754 as *mut u8) })
                        as i64) as i32,
                    unsafe {
                        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                            sqlite3_free as *const (),
                        )
                    },
                )
            };
            return;
        }
    }
}

/// The zeroblob(N) function returns a zero-filled blob of size N bytes.
#[unsafe(link_section = ".text.slate_distinct.func.zeroblobFunc")]
extern "C-unwind" fn zeroblobFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut n: i64 = 0 as i64;
    let mut rc: i32 = 0 as i32;
    0 as i32;
    argc;
    n = unsafe { sqlite3_value_int64(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    if n < ((0 as i32) as i64) {
        n = (0 as i32) as i64;
    }
    rc = unsafe { sqlite3_result_zeroblob64(context, n as u64) }; // IMP: R-00293-64994
    if rc != (0 as i32) {
        unsafe { sqlite3_result_error_code(context, rc) };
    }
}

/// The replace() function.  Three arguments are all strings: call
/// them A, B, and C. The result is also a string which is derived
/// from A by replacing every occurrence of B with C.  The match
/// must be exact.  Collating sequences are not used.
#[unsafe(link_section = ".text.slate_distinct.func.replaceFunc")]
extern "C-unwind" fn replaceFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut zStr: *const u8 = unsafe { std::mem::zeroed() }; // The input string A
    let mut zPattern: *const u8 = unsafe { std::mem::zeroed() }; // The pattern string B
    let mut zRep: *const u8 = unsafe { std::mem::zeroed() }; // The replacement string C
    let mut zOut: *mut u8 = unsafe { std::mem::zeroed() }; // The output
    let mut nStr: i32 = 0 as i32; // Size of zStr
    let mut nPattern: i32 = 0 as i32; // Size of zPattern
    let mut nRep: i32 = 0 as i32; // Size of zRep
    let mut nOut: i64 = 0 as i64; // Maximum size of zOut
    let mut loopLimit: i32 = 0 as i32; // Last zStr[] that might match zPattern[]
    let mut i: i64 = 0 as i64;
    let mut j: i64 = 0 as i64; // Loop counters
    let mut cntExpand: u32 = 0 as u32; // Number zOut expansions
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    0 as i32;
    argc;
    zStr = unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    if zStr == std::ptr::null::<u8>() {
        return;
    }
    nStr = unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    0 as i32; // No encoding change
    zPattern =
        unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) };
    if zPattern == std::ptr::null::<u8>() {
        0 as i32;
        return;
    }
    if (((unsafe { *unsafe { zPattern.offset((0 as i32) as isize) } }) as u32) as i32) == (0 as i32)
    {
        0 as i32;
        unsafe {
            sqlite3_result_text(context, zStr as *const i8, nStr, unsafe {
                std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    -(1 as i32) as usize,
                )
            })
        };
        return;
    }
    nPattern =
        unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) };
    0 as i32; // No encoding change
    zRep = unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((2 as i32) as isize) } }) };
    if zRep == std::ptr::null::<u8>() {
        return;
    }
    nRep = unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset((2 as i32) as isize) } }) };
    0 as i32;
    nOut = (nStr + (1 as i32)) as i64;
    0 as i32;
    zOut = contextMalloc(context, nOut) as *mut u8;
    if zOut == std::ptr::null_mut::<u8>() {
        return;
    }
    loopLimit = nStr - nPattern;
    cntExpand = (0 as i32) as u32;
    j = (0 as i32) as i64;
    i = (0 as i32) as i64;
    '__slate_break_1316: loop {
        if !(i <= (loopLimit as i64)) {
            break;
        }
        if (((unsafe { *unsafe { zStr.offset(i as isize) } }) as u32) as i32)
            != (((unsafe { *unsafe { zPattern.offset((0 as i32) as isize) } }) as u32) as i32)
            || (unsafe {
                memcmp(
                    (unsafe { zStr.offset(i as isize) }) as *const (),
                    zPattern as *const (),
                    (nPattern as i64) as u64,
                )
            }) != (0 as i32)
        {
            let __v1620: i64 = j;
            let __v1621: i64 = __v1620 + ((1 as i32) as i64);
            j = __v1621;
            unsafe {
                *unsafe { zOut.offset(__v1620 as isize) } =
                    unsafe { *unsafe { zStr.offset(i as isize) } };
            }
        } else {
            if nRep > nPattern {
                let __v1622: i64 = nOut;
                let __v1623: i64 = __v1622 + ((nRep - nPattern) as i64);
                nOut = __v1623;
                {}
                {}
                if nOut - ((1 as i32) as i64)
                    > ((unsafe {
                        *unsafe {
                            unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }
                                .offset((0 as i32) as isize)
                        }
                    }) as i64)
                {
                    unsafe { sqlite3_result_error_toobig(context) };
                    unsafe { sqlite3_free(zOut as *mut ()) };
                    return;
                }
                let __v1624: u32 = cntExpand;
                let __v1625: u32 = __v1624.wrapping_add((1 as i32) as u32);
                cntExpand = __v1625;
                if cntExpand & cntExpand.wrapping_sub((1 as i32) as u32) == ((0 as i32) as u32) {
                    // Grow the size of the output buffer only on substitutions
                    // whose index is a power of two: 1, 2, 4, 8, 16, 32, ...
                    let mut zOld: *mut u8 = unsafe { std::mem::zeroed() };
                    zOld = zOut;
                    zOut = (unsafe {
                        sqlite3Realloc(
                            zOut as *mut (),
                            (((nOut as i32) as i64) + (nOut - (nStr as i64) - ((1 as i32) as i64)))
                                as u64,
                        )
                    }) as *mut u8;
                    if zOut == std::ptr::null_mut::<u8>() {
                        unsafe { sqlite3_result_error_nomem(context) };
                        unsafe { sqlite3_free(zOld as *mut ()) };
                        return;
                    }
                }
            }
            unsafe {
                memcpy(
                    (unsafe { zOut.offset(j as isize) }) as *mut (),
                    zRep as *const (),
                    (nRep as i64) as u64,
                )
            };
            let __v1626: i64 = j;
            let __v1627: i64 = __v1626 + (nRep as i64);
            j = __v1627;
            let __v1628: i64 = i;
            let __v1629: i64 = __v1628 + ((nPattern - (1 as i32)) as i64);
            i = __v1629;
        }
        let __v1618: i64 = i;
        let __v1619: i64 = __v1618 + ((1 as i32) as i64);
        i = __v1619;
    }
    0 as i32;
    unsafe {
        memcpy(
            (unsafe { zOut.offset(j as isize) }) as *mut (),
            (unsafe { zStr.offset(i as isize) }) as *const (),
            ((nStr as i64) - i) as u64,
        )
    };
    let __v1630: i64 = j;
    let __v1631: i64 = __v1630 + ((nStr as i64) - i);
    j = __v1631;
    0 as i32;
    unsafe {
        *unsafe { zOut.offset(j as isize) } = ((0 as i32) as i8) as u8;
    }
    unsafe {
        sqlite3_result_text(context, (zOut as *mut i8) as *const i8, j as i32, unsafe {
            std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                sqlite3_free as *const (),
            )
        })
    };
}

/// Implementation of the TRIM(), LTRIM(), and RTRIM() functions.
/// The userdata is 0x1 for left trim, 0x2 for right trim, 0x3 for both.
#[unsafe(link_section = ".text.slate_distinct.func.trimFunc")]
extern "C-unwind" fn trimFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut zIn: *const u8 = unsafe { std::mem::zeroed() }; // Input string
    let mut zCharSet: *const u8 = unsafe { std::mem::zeroed() }; // Set of characters to trim
    let mut nIn: u32 = 0 as u32; // Number of bytes in input
    let mut flags: i32 = 0 as i32; // 1: trimleft  2: trimright  3: trim
    let mut i: i32 = 0 as i32; // Loop counter
    let mut aLen: *mut u32 = std::ptr::null_mut::<u32>(); // Length of each character in zCharSet
    let mut azChar: *mut *mut u8 = std::ptr::null_mut::<*mut u8>(); // Individual characters in zCharSet
    let mut nChar: i32 = 0 as i32; // Number of characters in zCharSet
    if (unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
        == (5 as i32)
    {
        return;
    }
    zIn = unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    if zIn == std::ptr::null::<u8>() {
        return;
    }
    nIn = (unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
        as u32;
    0 as i32;
    if argc == (1 as i32) {
        nChar = 1 as i32;
        aLen = (unsafe { std::ptr::addr_of!(lenOne) as *const u32 }) as *mut u32;
        azChar = (unsafe { std::ptr::addr_of!(azOne) as *const *mut u8 }) as *mut *mut u8;
        zCharSet = std::ptr::null::<u8>();
    } else {
        let __v1632: *const u8 =
            unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) };
        zCharSet = __v1632;
        if __v1632 == std::ptr::null::<u8>() {
            return;
        } else {
            let mut z: *const u8 = unsafe { std::mem::zeroed() };
            z = zCharSet;
            nChar = 0 as i32;
            '__slate_break_1318: loop {
                if !((unsafe { *z }) != (0 as u8)) {
                    break;
                }
                let __v1635: *const u8 = z;
                let __v1636: *const u8 = unsafe { __v1635.offset((1 as i32) as isize) };
                z = __v1636;
                if (((unsafe { *__v1635 }) as u32) as i32) >= (192 as i32) {
                    '__slate_break_1319: while (((unsafe { *z }) as u32) as i32) & (192 as i32)
                        == (128 as i32)
                    {
                        let __v1637: *const u8 = z;
                        let __v1638: *const u8 = unsafe { __v1637.offset((1 as i32) as isize) };
                        z = __v1638;
                    }
                }
                {}
                let __v1633: i32 = nChar;
                let __v1634: i32 = __v1633 + (1 as i32);
                nChar = __v1634;
            }
            if nChar > (0 as i32) {
                azChar = contextMalloc(
                    context,
                    ((nChar as i64) as u64).wrapping_mul((8 as u64).wrapping_add(4 as u64)) as i64,
                ) as *mut *mut u8;
                if azChar == std::ptr::null_mut::<*mut u8>() {
                    return;
                }
                aLen = (unsafe { azChar.offset(nChar as isize) }) as *mut u32;
                z = zCharSet;
                nChar = 0 as i32;
                '__slate_break_1320: loop {
                    if !((unsafe { *z }) != (0 as u8)) {
                        break;
                    }
                    unsafe {
                        *unsafe { azChar.offset(nChar as isize) } = z as *mut u8;
                    }
                    let __v1641: *const u8 = z;
                    let __v1642: *const u8 = unsafe { __v1641.offset((1 as i32) as isize) };
                    z = __v1642;
                    if (((unsafe { *__v1641 }) as u32) as i32) >= (192 as i32) {
                        '__slate_break_1321: while (((unsafe { *z }) as u32) as i32) & (192 as i32)
                            == (128 as i32)
                        {
                            let __v1643: *const u8 = z;
                            let __v1644: *const u8 = unsafe { __v1643.offset((1 as i32) as isize) };
                            z = __v1644;
                        }
                    }
                    {}
                    unsafe {
                        *unsafe { aLen.offset(nChar as isize) } =
                            (((unsafe {
                                z.offset_from(
                                    (unsafe { *unsafe { azChar.offset(nChar as isize) } })
                                        as *const u8,
                                )
                            }) as i64) as i32) as u32;
                    }
                    let __v1639: i32 = nChar;
                    let __v1640: i32 = __v1639 + (1 as i32);
                    nChar = __v1640;
                }
            }
        }
    }
    if nChar > (0 as i32) {
        flags = ((unsafe { sqlite3_user_data(context) }) as i64) as i32;
        if flags & (1 as i32) != (0 as i32) {
            '__slate_break_1322: while nIn > ((0 as i32) as u32) {
                let mut len: u32 = (0 as i32) as u32;
                i = 0 as i32;
                '__slate_break_1323: loop {
                    if !(i < nChar) {
                        break;
                    }
                    len = unsafe { *unsafe { aLen.offset(i as isize) } };
                    if len <= nIn
                        && (unsafe {
                            memcmp(
                                zIn as *const (),
                                (unsafe { *unsafe { azChar.offset(i as isize) } }) as *const (),
                                len as u64,
                            )
                        }) == (0 as i32)
                    {
                        break '__slate_break_1323;
                    }
                    let __v1645: i32 = i;
                    let __v1646: i32 = __v1645 + (1 as i32);
                    i = __v1646;
                }
                if i >= nChar {
                    break '__slate_break_1322;
                }
                let __v1647: *const u8 = zIn;
                let __v1648: *const u8 = unsafe { __v1647.offset(len as isize) };
                zIn = __v1648;
                let __v1649: u32 = nIn;
                let __v1650: u32 = __v1649.wrapping_sub(len);
                nIn = __v1650;
            }
        }
        if flags & (2 as i32) != (0 as i32) {
            '__slate_break_1324: while nIn > ((0 as i32) as u32) {
                let mut len: u32 = (0 as i32) as u32;
                i = 0 as i32;
                '__slate_break_1325: loop {
                    if !(i < nChar) {
                        break;
                    }
                    len = unsafe { *unsafe { aLen.offset(i as isize) } };
                    if len <= nIn
                        && (unsafe {
                            memcmp(
                                (unsafe { zIn.offset(nIn.wrapping_sub(len) as isize) })
                                    as *const (),
                                (unsafe { *unsafe { azChar.offset(i as isize) } }) as *const (),
                                len as u64,
                            )
                        }) == (0 as i32)
                    {
                        break '__slate_break_1325;
                    }
                    let __v1651: i32 = i;
                    let __v1652: i32 = __v1651 + (1 as i32);
                    i = __v1652;
                }
                if i >= nChar {
                    break '__slate_break_1324;
                }
                let __v1653: u32 = nIn;
                let __v1654: u32 = __v1653.wrapping_sub(len);
                nIn = __v1654;
            }
        }
        if zCharSet != std::ptr::null::<u8>() {
            unsafe { sqlite3_free(azChar as *mut ()) };
        }
    }
    unsafe {
        sqlite3_result_text(context, (zIn as *mut i8) as *const i8, nIn as i32, unsafe {
            std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                -(1 as i32) as usize,
            )
        })
    };
}

static mut lenOne: [u32; 1] = [(1 as i32) as u32];

static mut azOne: [*mut u8; 1] = [(b" \0".as_ptr() as *mut i8) as *mut u8];

/// The core implementation of the CONCAT(...) and CONCAT_WS(SEP,...)
/// functions.
///
/// Return a string value that is the concatenation of all non-null
/// entries in argv[].  Use zSep as the separator.
fn concatFuncCore(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
    mut nSep: i32,
    mut zSep: *const i8,
) {
    let mut j: i64 = 0 as i64;
    let mut n: i64 = (0 as i32) as i64;
    let mut i: i32 = 0 as i32;
    let mut bNotNull: i32 = 0 as i32; // True after at least NOT NULL argument seen
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    i = 0 as i32;
    '__slate_break_1326: loop {
        if !(i < argc) {
            break;
        }
        let __v1657: i64 = n;
        let __v1658: i64 = __v1657
            + ((unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset(i as isize) } }) })
                as i64);
        n = __v1658;
        let __v1655: i32 = i;
        let __v1656: i32 = __v1655 + (1 as i32);
        i = __v1656;
    }
    let __v1659: i64 = n;
    let __v1660: i64 = __v1659 + ((argc - (1 as i32)) as i64) * (nSep as i64);
    n = __v1660;
    z = (unsafe { sqlite3_malloc64((n + ((1 as i32) as i64)) as u64) }) as *mut i8;
    if z == std::ptr::null_mut::<i8>() {
        unsafe { sqlite3_result_error_nomem(context) };
        return;
    }
    j = (0 as i32) as i64;
    i = 0 as i32;
    '__slate_break_1327: loop {
        if !(i < argc) {
            break;
        }
        if (unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset(i as isize) } }) })
            != (5 as i32)
        {
            let mut k: i32 =
                unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset(i as isize) } }) };
            let mut v: *const i8 =
                (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset(i as isize) } }) })
                    as *const i8;
            if v != std::ptr::null::<i8>() {
                if bNotNull != (0 as i32) && nSep > (0 as i32) {
                    unsafe {
                        memcpy(
                            (unsafe { z.offset(j as isize) }) as *mut (),
                            zSep as *const (),
                            (nSep as i64) as u64,
                        )
                    };
                    let __v1663: i64 = j;
                    let __v1664: i64 = __v1663 + (nSep as i64);
                    j = __v1664;
                }
                unsafe {
                    memcpy(
                        (unsafe { z.offset(j as isize) }) as *mut (),
                        v as *const (),
                        (k as i64) as u64,
                    )
                };
                let __v1665: i64 = j;
                let __v1666: i64 = __v1665 + (k as i64);
                j = __v1666;
                bNotNull = 1 as i32;
            }
        }
        let __v1661: i32 = i;
        let __v1662: i32 = __v1661 + (1 as i32);
        i = __v1662;
    }
    unsafe {
        *unsafe { z.offset(j as isize) } = (0 as i32) as i8;
    }
    0 as i32;
    unsafe {
        sqlite3_result_text64(
            context,
            z as *const i8,
            j as u64,
            unsafe {
                std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    sqlite3_free as *const (),
                )
            },
            ((16 as i32) as i8) as u8,
        )
    };
}

/// The CONCAT(...) function.  Generate a string result that is the
/// concatentation of all non-null arguments.
#[unsafe(link_section = ".text.slate_distinct.func.concatFunc")]
extern "C-unwind" fn concatFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    concatFuncCore(
        context,
        argc,
        argv,
        0 as i32,
        (b"\0".as_ptr() as *mut i8) as *const i8,
    );
}

/// The CONCAT_WS(separator, ...) function.
///
/// Generate a string that is the concatenation of 2nd through the Nth
/// argument.  Use the first argument (which must be non-NULL) as the
/// separator.
#[unsafe(link_section = ".text.slate_distinct.func.concatwsFunc")]
extern "C-unwind" fn concatwsFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut nSep: i32 =
        unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    let mut zSep: *const i8 =
        (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
            as *const i8;
    if zSep == std::ptr::null::<i8>() {
        return;
    }
    concatFuncCore(
        context,
        argc - (1 as i32),
        unsafe { argv.offset((1 as i32) as isize) },
        nSep,
        zSep,
    );
}

/// The "unknown" function is automatically substituted in place of
/// any unrecognized function name when doing an EXPLAIN or EXPLAIN QUERY PLAN
/// when the SQLITE_ENABLE_UNKNOWN_SQL_FUNCTION compile-time option is used.
/// When the "sqlite3" command-line shell is built using this functionality,
/// that allows an EXPLAIN or EXPLAIN QUERY PLAN for complex queries
/// involving application-defined functions to be examined in a generic
/// sqlite3 shell.
#[unsafe(link_section = ".text.slate_distinct.func.unknownFunc")]
extern "C-unwind" fn unknownFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    // no-op
    context;
    argc;
    argv;
}

/// IMP: R-25361-16150 This function is omitted from SQLite by default. It
/// is only available if the SQLITE_SOUNDEX compile-time option is used
/// when SQLite is built.
///
/// A function that loads a shared-library extension then returns NULL.
#[unsafe(link_section = ".text.slate_distinct.func.loadExt")]
extern "C-unwind" fn loadExt(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut zFile: *const i8 =
        (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
            as *const i8;
    let mut zProc: *const i8 = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    let mut zErrMsg: *mut i8 = std::ptr::null_mut::<i8>();
    // Disallow the load_extension() SQL function unless the SQLITE_LoadExtFunc
    // flag is set.  See the sqlite3_enable_load_extension() API.
    if (unsafe { (*db).flags }) & (((131072 as i32) as i64) as u64) == (((0 as i32) as i64) as u64)
    {
        unsafe {
            sqlite3_result_error(
                context,
                (b"not authorized\0".as_ptr() as *mut i8) as *const i8,
                -(1 as i32),
            )
        };
        return;
    }
    if argc == (2 as i32) {
        zProc = (unsafe {
            sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
        }) as *const i8;
    } else {
        zProc = std::ptr::null::<i8>();
    }
    let __v1667: bool;
    if zFile != std::ptr::null::<i8>() {
        __v1667 =
            (unsafe { sqlite3_load_extension(db, zFile, zProc, std::ptr::addr_of_mut!(zErrMsg)) })
                != (0 as i32);
    } else {
        __v1667 = false as bool;
    }
    if __v1667 {
        unsafe { sqlite3_result_error(context, zErrMsg as *const i8, -(1 as i32)) };
        unsafe { sqlite3_free(zErrMsg as *mut ()) };
    }
}

/// An instance of the following structure holds the context of a
/// sum() or avg() aggregate computation.
#[repr(C)]
#[derive(Clone, Copy)]
struct SumCtx {
    /// Running sum as as a double
    rSum: f64,
    /// Error term for Kahan-Babushka-Neumaier summation
    rErr: f64,
    /// Running sum as a signed integer
    iSum: i64,
    /// Number of elements summed
    cnt: i64,
    /// True if any non-integer value was input to the sum
    approx: u8,
    /// Integer overflow seen
    ovrfl: u8,
}

/// Do one step of the Kahan-Babushka-Neumaier summation.
///
/// https://en.wikipedia.org/wiki/Kahan_summation_algorithm
///
/// Variables are marked "volatile" to defeat c89 x86 floating point
/// optimizations can mess up this algorithm.
fn kahanBabuskaNeumaierStep(mut pSum: *mut SumCtx, mut r: f64) {
    let mut s: f64 = unsafe { std::ptr::read_volatile(std::ptr::addr_of!((*pSum).rSum)) };
    let mut t: f64 = (unsafe { std::ptr::read_volatile(std::ptr::addr_of!(s)) })
        + unsafe { std::ptr::read_volatile(std::ptr::addr_of!(r)) };
    if (unsafe { fabs(unsafe { std::ptr::read_volatile(std::ptr::addr_of!(s)) }) })
        > unsafe { fabs(unsafe { std::ptr::read_volatile(std::ptr::addr_of!(r)) }) }
    {
        let __v1668: *mut SumCtx = pSum;
        let __v1669: f64 = unsafe { std::ptr::read_volatile(std::ptr::addr_of!((*__v1668).rErr)) };
        let __v1670: f64 = __v1669
            + ((unsafe { std::ptr::read_volatile(std::ptr::addr_of!(s)) })
                - unsafe { std::ptr::read_volatile(std::ptr::addr_of!(t)) }
                + unsafe { std::ptr::read_volatile(std::ptr::addr_of!(r)) });
        unsafe { std::ptr::write_volatile(std::ptr::addr_of_mut!((*__v1668).rErr), __v1670) };
    } else {
        let __v1671: *mut SumCtx = pSum;
        let __v1672: f64 = unsafe { std::ptr::read_volatile(std::ptr::addr_of!((*__v1671).rErr)) };
        let __v1673: f64 = __v1672
            + ((unsafe { std::ptr::read_volatile(std::ptr::addr_of!(r)) })
                - unsafe { std::ptr::read_volatile(std::ptr::addr_of!(t)) }
                + unsafe { std::ptr::read_volatile(std::ptr::addr_of!(s)) });
        unsafe { std::ptr::write_volatile(std::ptr::addr_of_mut!((*__v1671).rErr), __v1673) };
    }
    unsafe {
        std::ptr::write_volatile(std::ptr::addr_of_mut!((*pSum).rSum), unsafe {
            std::ptr::read_volatile(std::ptr::addr_of!(t))
        })
    };
}

/// Add a (possibly large) integer to the running sum.
fn kahanBabuskaNeumaierStepInt64(mut pSum: *mut SumCtx, mut iVal: i64) {
    if iVal <= -(4503599627370496 as i64) || iVal >= (4503599627370496 as i64) {
        let mut iBig: i64 = 0 as i64;
        let mut iSm: i64 = 0 as i64;
        iSm = iVal % ((16384 as i32) as i64);
        iBig = iVal - iSm;
        kahanBabuskaNeumaierStep(pSum, iBig as f64);
        kahanBabuskaNeumaierStep(pSum, iSm as f64);
    } else {
        kahanBabuskaNeumaierStep(pSum, iVal as f64);
    }
}

/// Initialize the Kahan-Babaska-Neumaier sum from a 64-bit integer
fn kahanBabuskaNeumaierInit(mut p: *mut SumCtx, mut iVal: i64) {
    if iVal <= -(4503599627370496 as i64) || iVal >= (4503599627370496 as i64) {
        let mut iSm: i64 = iVal % ((16384 as i32) as i64);
        unsafe { std::ptr::write_volatile(std::ptr::addr_of_mut!((*p).rSum), (iVal - iSm) as f64) };
        unsafe { std::ptr::write_volatile(std::ptr::addr_of_mut!((*p).rErr), iSm as f64) };
    } else {
        unsafe { std::ptr::write_volatile(std::ptr::addr_of_mut!((*p).rSum), iVal as f64) };
        unsafe { std::ptr::write_volatile(std::ptr::addr_of_mut!((*p).rErr), 0.0f64) };
    }
}

/// Routines used to compute the sum, average, and total.
///
/// The SUM() function follows the (broken) SQL standard which means
/// that it returns NULL if it sums over no inputs.  TOTAL returns
/// 0.0 in that case.  In addition, TOTAL always returns a float where
/// SUM might return an integer if it never encounters a floating point
/// value.  TOTAL never fails, but SUM might throw an exception if
/// it overflows an integer.
#[unsafe(link_section = ".text.slate_distinct.func.sumStep")]
extern "C-unwind" fn sumStep(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut p: *mut SumCtx = unsafe { std::mem::zeroed() };
    let mut r#type: i32 = 0 as i32;
    0 as i32;
    argc;
    p = (unsafe { sqlite3_aggregate_context(context, ((40 as u64) as u32) as i32) }) as *mut SumCtx;
    r#type = unsafe {
        sqlite3_value_numeric_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
    };
    if p != std::ptr::null_mut::<SumCtx>() && r#type != (5 as i32) {
        let __v1674: *mut SumCtx = p;
        let __v1675: i64 = unsafe { (*__v1674).cnt };
        let __v1676: i64 = __v1675 + ((1 as i32) as i64);
        unsafe {
            (*__v1674).cnt = __v1676;
        }
        if (((unsafe { (*p).approx }) as u32) as i32) == (0 as i32) {
            if r#type != (1 as i32) {
                kahanBabuskaNeumaierInit(p as *mut SumCtx, unsafe { (*p).iSum });
                unsafe {
                    (*p).approx = ((1 as i32) as i8) as u8;
                }
                kahanBabuskaNeumaierStep(p as *mut SumCtx, unsafe {
                    sqlite3_value_double(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                });
            } else {
                let mut x: i64 = unsafe { (*p).iSum };
                if (unsafe {
                    sqlite3AddInt64(std::ptr::addr_of_mut!(x), unsafe {
                        sqlite3_value_int64(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                    })
                }) == (0 as i32)
                {
                    unsafe {
                        (*p).iSum = x;
                    }
                } else {
                    unsafe {
                        (*p).ovrfl = ((1 as i32) as i8) as u8;
                    }
                    kahanBabuskaNeumaierInit(p as *mut SumCtx, unsafe { (*p).iSum });
                    unsafe {
                        (*p).approx = ((1 as i32) as i8) as u8;
                    }
                    kahanBabuskaNeumaierStepInt64(p as *mut SumCtx, unsafe {
                        sqlite3_value_int64(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                    });
                }
            }
        } else {
            if r#type == (1 as i32) {
                kahanBabuskaNeumaierStepInt64(p as *mut SumCtx, unsafe {
                    sqlite3_value_int64(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                });
            } else {
                unsafe {
                    (*p).ovrfl = ((0 as i32) as i8) as u8;
                }
                kahanBabuskaNeumaierStep(p as *mut SumCtx, unsafe {
                    sqlite3_value_double(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                });
            }
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.func.sumInverse")]
extern "C-unwind" fn sumInverse(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut p: *mut SumCtx = unsafe { std::mem::zeroed() };
    let mut r#type: i32 = 0 as i32;
    0 as i32;
    argc;
    p = (unsafe { sqlite3_aggregate_context(context, ((40 as u64) as u32) as i32) }) as *mut SumCtx;
    r#type = unsafe {
        sqlite3_value_numeric_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
    };
    // p is always non-NULL because sumStep() will have been called first
    // to initialize it
    if p != std::ptr::null_mut::<SumCtx>() && r#type != (5 as i32) {
        0 as i32;
        let __v1677: *mut SumCtx = p;
        let __v1678: i64 = unsafe { (*__v1677).cnt };
        let __v1679: i64 = __v1678 - ((1 as i32) as i64);
        unsafe {
            (*__v1677).cnt = __v1679;
        }
        if !((unsafe { (*p).approx }) != (0 as u8)) {
            let mut x: i64 = unsafe { (*p).iSum };
            if (unsafe {
                sqlite3SubInt64(std::ptr::addr_of_mut!(x), unsafe {
                    sqlite3_value_int64(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                })
            }) == (0 as i32)
            {
                unsafe {
                    (*p).iSum = x;
                }
                return;
            }
            unsafe {
                (*p).ovrfl = ((1 as i32) as i8) as u8;
            }
            unsafe {
                (*p).approx = ((1 as i32) as i8) as u8;
            }
            kahanBabuskaNeumaierInit(p as *mut SumCtx, unsafe { (*p).iSum });
        }
        if r#type == (1 as i32) {
            let mut iVal: i64 = unsafe {
                sqlite3_value_int64(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
            };
            if iVal
                != (-(1 as i32) as i64)
                    - ((((4294967295 as u32) as u64) as i64)
                        | ((2147483647 as i32) as i64) << (32 as i32))
            {
                kahanBabuskaNeumaierStepInt64(p as *mut SumCtx, -iVal);
            } else {
                kahanBabuskaNeumaierStepInt64(
                    p as *mut SumCtx,
                    (((4294967295 as u32) as u64) as i64)
                        | ((2147483647 as i32) as i64) << (32 as i32),
                );
                kahanBabuskaNeumaierStepInt64(p as *mut SumCtx, (1 as i32) as i64);
            }
        } else {
            kahanBabuskaNeumaierStep(p as *mut SumCtx, -unsafe {
                sqlite3_value_double(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
            });
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.func.sumFinalize")]
extern "C-unwind" fn sumFinalize(mut context: *mut sqlite3_context) {
    let mut p: *mut SumCtx = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(context, 0 as i32) }) as *mut SumCtx;
    if p != std::ptr::null_mut::<SumCtx>() && (unsafe { (*p).cnt }) > ((0 as i32) as i64) {
        if (unsafe { (*p).approx }) != (0 as u8) {
            if (unsafe { (*p).ovrfl }) != (0 as u8) {
                unsafe {
                    sqlite3_result_error(
                        context,
                        (b"integer overflow\0".as_ptr() as *mut i8) as *const i8,
                        -(1 as i32),
                    )
                };
            } else {
                if !((unsafe { sqlite3IsOverflow(unsafe { (*p).rErr }) }) != (0 as i32)) {
                    unsafe {
                        sqlite3_result_double(
                            context,
                            (unsafe { (*p).rSum }) + unsafe { (*p).rErr },
                        )
                    };
                } else {
                    unsafe { sqlite3_result_double(context, unsafe { (*p).rSum }) };
                }
            }
        } else {
            unsafe { sqlite3_result_int64(context, unsafe { (*p).iSum }) };
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.func.avgFinalize")]
extern "C-unwind" fn avgFinalize(mut context: *mut sqlite3_context) {
    let mut p: *mut SumCtx = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(context, 0 as i32) }) as *mut SumCtx;
    if p != std::ptr::null_mut::<SumCtx>() && (unsafe { (*p).cnt }) > ((0 as i32) as i64) {
        let mut r: f64 = 0 as f64;
        if (unsafe { (*p).approx }) != (0 as u8) {
            r = unsafe { (*p).rSum };
            if !((unsafe { sqlite3IsOverflow(unsafe { (*p).rErr }) }) != (0 as i32)) {
                let __v1680: f64 = r;
                let __v1681: f64 = __v1680 + unsafe { (*p).rErr };
                r = __v1681;
            }
        } else {
            r = (unsafe { (*p).iSum }) as f64;
        }
        unsafe { sqlite3_result_double(context, r / ((unsafe { (*p).cnt }) as f64)) };
    }
}

#[unsafe(link_section = ".text.slate_distinct.func.totalFinalize")]
extern "C-unwind" fn totalFinalize(mut context: *mut sqlite3_context) {
    let mut p: *mut SumCtx = unsafe { std::mem::zeroed() };
    let mut r: f64 = 0.0f64;
    p = (unsafe { sqlite3_aggregate_context(context, 0 as i32) }) as *mut SumCtx;
    if p != std::ptr::null_mut::<SumCtx>() {
        if (unsafe { (*p).approx }) != (0 as u8) {
            r = unsafe { (*p).rSum };
            if !((unsafe { sqlite3IsOverflow(unsafe { (*p).rErr }) }) != (0 as i32)) {
                let __v1682: f64 = r;
                let __v1683: f64 = __v1682 + unsafe { (*p).rErr };
                r = __v1683;
            }
        } else {
            r = (unsafe { (*p).iSum }) as f64;
        }
    }
    unsafe { sqlite3_result_double(context, r) };
}

/// The following structure keeps track of state information for the
/// count() aggregate function.
#[repr(C)]
#[derive(Clone, Copy)]
struct CountCtx {
    n: i64,
}

/// Routines to implement the count() aggregate function.
#[unsafe(link_section = ".text.slate_distinct.func.countStep")]
extern "C-unwind" fn countStep(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut p: *mut CountCtx = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(context, ((8 as u64) as u32) as i32) })
        as *mut CountCtx;
    let __v1684: bool;
    if argc == (0 as i32) {
        __v1684 = true as bool;
    } else {
        __v1684 = (5 as i32)
            != unsafe {
                sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
            };
    }
    if __v1684 && p != std::ptr::null_mut::<CountCtx>() {
        let __v1685: *mut CountCtx = p;
        let __v1686: i64 = unsafe { (*__v1685).n };
        let __v1687: i64 = __v1686 + ((1 as i32) as i64);
        unsafe {
            (*__v1685).n = __v1687;
        }
    }
    // The sqlite3_aggregate_count() function is deprecated.  But just to make
    // sure it still operates correctly, verify that its count agrees with our
    // internal count when using count(*) and when the total count can be
    // expressed as a 32-bit integer.
    0 as i32;
}

#[unsafe(link_section = ".text.slate_distinct.func.countFinalize")]
extern "C-unwind" fn countFinalize(mut context: *mut sqlite3_context) {
    let mut p: *mut CountCtx = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(context, 0 as i32) }) as *mut CountCtx;
    unsafe {
        sqlite3_result_int64(
            context,
            if p != std::ptr::null_mut::<CountCtx>() {
                unsafe { (*p).n }
            } else {
                (0 as i32) as i64
            },
        )
    };
}

#[unsafe(link_section = ".text.slate_distinct.func.countInverse")]
extern "C-unwind" fn countInverse(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut p: *mut CountCtx = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(ctx, ((8 as u64) as u32) as i32) }) as *mut CountCtx;
    // p is always non-NULL since countStep() will have been called first
    let __v1688: bool;
    if argc == (0 as i32) {
        __v1688 = true as bool;
    } else {
        __v1688 = (5 as i32)
            != unsafe {
                sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
            };
    }
    if __v1688 && p != std::ptr::null_mut::<CountCtx>() {
        let __v1689: *mut CountCtx = p;
        let __v1690: i64 = unsafe { (*__v1689).n };
        let __v1691: i64 = __v1690 - ((1 as i32) as i64);
        unsafe {
            (*__v1689).n = __v1691;
        }
    }
}

/// Routines to implement min() and max() aggregate functions.
#[unsafe(link_section = ".text.slate_distinct.func.minmaxStep")]
extern "C-unwind" fn minmaxStep(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut pArg: *mut sqlite3_value = unsafe { *unsafe { argv.offset((0 as i32) as isize) } };
    let mut pBest: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    NotUsed;
    pBest = (unsafe { sqlite3_aggregate_context(context, ((56 as u64) as u32) as i32) })
        as *mut sqlite3_value;
    if !(pBest != std::ptr::null_mut::<sqlite3_value>()) {
        return;
    }
    if (unsafe { sqlite3_value_type(pArg) }) == (5 as i32) {
        if (unsafe { (*pBest).flags }) != (0 as u16) {
            sqlite3SkipAccumulatorLoad(context);
        }
    } else {
        if (unsafe { (*pBest).flags }) != (0 as u16) {
            let mut max: i32 = 0 as i32;
            let mut cmp: i32 = 0 as i32;
            let mut pColl: *mut CollSeq = sqlite3GetFuncCollSeq(context);
            // This step function is used for both the min() and max() aggregates,
            // the only difference between the two being that the sense of the
            // comparison is inverted. For the max() aggregate, the
            // sqlite3_user_data() function returns (void *)-1. For min() it
            // returns (void *)db, where db is the sqlite3* database pointer.
            // Therefore the next statement sets variable 'max' to 1 for the max()
            // aggregate, or 0 for min().
            max = ((unsafe { sqlite3_user_data(context) }) != std::ptr::null_mut::<()>()) as i32;
            cmp = unsafe {
                sqlite3MemCompare(
                    pBest as *const sqlite3_value,
                    pArg as *const sqlite3_value,
                    pColl as *const CollSeq,
                )
            };
            if max != (0 as i32) && cmp < (0 as i32) || !(max != (0 as i32)) && cmp > (0 as i32) {
                unsafe { sqlite3VdbeMemCopy(pBest, pArg as *const sqlite3_value) };
            } else {
                sqlite3SkipAccumulatorLoad(context);
            }
        } else {
            unsafe {
                (*pBest).db = unsafe { sqlite3_context_db_handle(context) };
            }
            unsafe { sqlite3VdbeMemCopy(pBest, pArg as *const sqlite3_value) };
        }
    }
}

fn minMaxValueFinalize(mut context: *mut sqlite3_context, mut bValue: i32) {
    let mut pRes: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    pRes = (unsafe { sqlite3_aggregate_context(context, 0 as i32) }) as *mut sqlite3_value;
    if pRes != std::ptr::null_mut::<sqlite3_value>() {
        if (unsafe { (*pRes).flags }) != (0 as u16) {
            unsafe { sqlite3_result_value(context, pRes) };
        }
        if bValue == (0 as i32) {
            unsafe { sqlite3VdbeMemRelease(pRes) };
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.func.minMaxValue")]
extern "C-unwind" fn minMaxValue(mut context: *mut sqlite3_context) {
    minMaxValueFinalize(context, 1 as i32);
}

#[unsafe(link_section = ".text.slate_distinct.func.minMaxFinalize")]
extern "C-unwind" fn minMaxFinalize(mut context: *mut sqlite3_context) {
    minMaxValueFinalize(context, 0 as i32);
}

/// group_concat(EXPR, ?SEPARATOR?)
/// string_agg(EXPR, SEPARATOR)
///
/// Content is accumulated in GroupConcatCtx.str with the SEPARATOR
/// coming before the EXPR value, except for the first entry which
/// omits the SEPARATOR.
///
/// It is tragic that the SEPARATOR goes before the EXPR string.  The
/// groupConcatInverse() implementation would have been easier if the
/// SEPARATOR were appended after EXPR.  And the order is undocumented,
/// so we could change it, in theory.  But the old behavior has been
/// around for so long that we dare not, for fear of breaking something.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord219 {
    /// The accumulated concatenation
    str: sqlite3_str,
    /// Number of strings presently concatenated
    nAccum: i32,
    /// Used to detect separator length change
    nFirstSepLength: i32,
    /// If pnSepLengths!=0, refs an array of inter-string separator lengths,
    /// stored as actually incorporated into presently accumulated result.
    /// (Hence, its slots in use number nAccum-1 between method calls.)
    /// If pnSepLengths==0, nFirstSepLength is the length used throughout.
    pnSepLengths: *mut i32,
}

#[unsafe(link_section = ".text.slate_distinct.func.groupConcatStep")]
extern "C-unwind" fn groupConcatStep(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut zVal: *const i8 = unsafe { std::mem::zeroed() };
    let mut pGCC: *mut __SlateRecord219 = unsafe { std::mem::zeroed() };
    let mut zSep: *const i8 = unsafe { std::mem::zeroed() };
    let mut nVal: i32 = 0 as i32;
    let mut nSep: i32 = 0 as i32;
    0 as i32;
    if (unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
        == (5 as i32)
    {
        return;
    }
    pGCC = (unsafe { sqlite3_aggregate_context(context, ((48 as u64) as u32) as i32) })
        as *mut __SlateRecord219;
    if pGCC != std::ptr::null_mut::<__SlateRecord219>() {
        let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(context) };
        let mut firstTerm: i32 = ((unsafe { (*pGCC).str.mxAlloc }) == ((0 as i32) as u32)) as i32;
        unsafe {
            (*pGCC).str.mxAlloc = (unsafe {
                *unsafe {
                    unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize)
                }
            }) as u32;
        }
        if argc == (1 as i32) {
            if !(firstTerm != (0 as i32)) {
                unsafe {
                    sqlite3_str_appendchar(
                        unsafe { std::ptr::addr_of_mut!((*pGCC).str) },
                        1 as i32,
                        (44 as i32) as i8,
                    )
                };
            } else {
                unsafe {
                    (*pGCC).nFirstSepLength = 1 as i32;
                }
            }
        } else {
            if !(firstTerm != (0 as i32)) {
                zSep = ((unsafe {
                    sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
                }) as *mut i8) as *const i8;
                nSep = unsafe {
                    sqlite3_value_bytes(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
                };
                if zSep != std::ptr::null::<i8>() {
                    unsafe {
                        sqlite3_str_append(
                            unsafe { std::ptr::addr_of_mut!((*pGCC).str) },
                            zSep,
                            nSep,
                        )
                    };
                } else {
                    nSep = 0 as i32;
                }
                if nSep != unsafe { (*pGCC).nFirstSepLength }
                    || (unsafe { (*pGCC).pnSepLengths }) != std::ptr::null_mut::<i32>()
                {
                    let mut pnsl: *mut i32 = unsafe { (*pGCC).pnSepLengths };
                    if pnsl == std::ptr::null_mut::<i32>() {
                        // First separator length variation seen, start tracking them.
                        pnsl = (unsafe {
                            sqlite3_malloc64(
                                ((((unsafe { (*pGCC).nAccum }) + (1 as i32)) as i64) as u64)
                                    .wrapping_mul(4 as u64),
                            )
                        }) as *mut i32;
                        if pnsl != std::ptr::null_mut::<i32>() {
                            let mut i: i32 = 0 as i32;
                            let mut nA: i32 = (unsafe { (*pGCC).nAccum }) - (1 as i32);
                            '__slate_break_1331: while i < nA {
                                let __v1692: i32 = i;
                                let __v1693: i32 = __v1692 + (1 as i32);
                                i = __v1693;
                                unsafe {
                                    *unsafe { pnsl.offset(__v1692 as isize) } =
                                        unsafe { (*pGCC).nFirstSepLength };
                                }
                            }
                        }
                    } else {
                        pnsl = (unsafe {
                            sqlite3_realloc64(
                                pnsl as *mut (),
                                (((unsafe { (*pGCC).nAccum }) as i64) as u64)
                                    .wrapping_mul(4 as u64),
                            )
                        }) as *mut i32;
                    }
                    if pnsl != std::ptr::null_mut::<i32>() {
                        if (unsafe { (*pGCC).nAccum }) > (0 as i32) {
                            unsafe {
                                *unsafe {
                                    pnsl.offset(((unsafe { (*pGCC).nAccum }) - (1 as i32)) as isize)
                                } = nSep;
                            }
                        }
                        unsafe {
                            (*pGCC).pnSepLengths = pnsl;
                        }
                    } else {
                        unsafe {
                            sqlite3StrAccumSetError(
                                unsafe { std::ptr::addr_of_mut!((*pGCC).str) },
                                ((7 as i32) as i8) as u8,
                            )
                        };
                    }
                }
            } else {
                unsafe {
                    (*pGCC).nFirstSepLength = unsafe {
                        sqlite3_value_bytes(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
                    };
                }
            }
        }
        let __v1694: *mut __SlateRecord219 = pGCC;
        let __v1695: i32 = unsafe { (*__v1694).nAccum };
        let __v1696: i32 = __v1695 + (1 as i32);
        unsafe {
            (*__v1694).nAccum = __v1696;
        }
        zVal = ((unsafe {
            sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
        }) as *mut i8) as *const i8;
        nVal =
            unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
        if zVal != std::ptr::null::<i8>() {
            unsafe {
                sqlite3_str_append(unsafe { std::ptr::addr_of_mut!((*pGCC).str) }, zVal, nVal)
            };
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.func.groupConcatInverse")]
extern "C-unwind" fn groupConcatInverse(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut pGCC: *mut __SlateRecord219 = unsafe { std::mem::zeroed() };
    0 as i32;
    argc; // Suppress unused parameter warning
    if (unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
        == (5 as i32)
    {
        return;
    }
    pGCC = (unsafe { sqlite3_aggregate_context(context, ((48 as u64) as u32) as i32) })
        as *mut __SlateRecord219;
    // pGCC is always non-NULL since groupConcatStep() will have always
    // run first to initialize it
    if pGCC != std::ptr::null_mut::<__SlateRecord219>() {
        let mut nVS: i32 = 0 as i32; // Number of characters to remove
        // Must call sqlite3_value_text() to convert the argument into text prior
        // to invoking sqlite3_value_bytes(), in case the text encoding is UTF16
        unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
        nVS =
            unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
        let __v1697: *mut __SlateRecord219 = pGCC;
        let __v1698: i32 = unsafe { (*__v1697).nAccum };
        let __v1699: i32 = __v1698 - (1 as i32);
        unsafe {
            (*__v1697).nAccum = __v1699;
        }
        if (unsafe { (*pGCC).pnSepLengths }) != std::ptr::null_mut::<i32>() {
            0 as i32;
            if (unsafe { (*pGCC).nAccum }) > (0 as i32) {
                let __v1700: i32 = nVS;
                let __v1701: i32 = __v1700 + unsafe { *unsafe { (*pGCC).pnSepLengths } };
                nVS = __v1701;
                unsafe {
                    memmove(
                        (unsafe { (*pGCC).pnSepLengths }) as *mut (),
                        (unsafe { unsafe { (*pGCC).pnSepLengths }.offset((1 as i32) as isize) })
                            as *const (),
                        ((((unsafe { (*pGCC).nAccum }) - (1 as i32)) as i64) as u64)
                            .wrapping_mul(4 as u64),
                    )
                };
            }
        } else {
            // If removing single accumulated string, harmlessly over-do.
            let __v1702: i32 = nVS;
            let __v1703: i32 = __v1702 + unsafe { (*pGCC).nFirstSepLength };
            nVS = __v1703;
        }
        if nVS >= ((unsafe { (*pGCC).str.nChar }) as i32) {
            unsafe {
                (*pGCC).str.nChar = (0 as i32) as u32;
            }
        } else {
            let __v1704: *mut __SlateRecord219 = pGCC;
            let __v1705: u32 = unsafe { (*__v1704).str.nChar };
            let __v1706: u32 = __v1705.wrapping_sub(nVS as u32);
            unsafe {
                (*__v1704).str.nChar = __v1706;
            }
            unsafe {
                memmove(
                    (unsafe { (*pGCC).str.zText }) as *mut (),
                    (unsafe { unsafe { (*pGCC).str.zText }.offset(nVS as isize) }) as *const (),
                    (unsafe { (*pGCC).str.nChar }) as u64,
                )
            };
        }
        if (unsafe { (*pGCC).str.nChar }) == ((0 as i32) as u32) {
            unsafe {
                (*pGCC).str.mxAlloc = (0 as i32) as u32;
            }
            unsafe { sqlite3_free((unsafe { (*pGCC).pnSepLengths }) as *mut ()) };
            unsafe {
                (*pGCC).pnSepLengths = std::ptr::null_mut::<i32>();
            }
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.func.groupConcatFinalize")]
extern "C-unwind" fn groupConcatFinalize(mut context: *mut sqlite3_context) {
    let mut pGCC: *mut __SlateRecord219 =
        (unsafe { sqlite3_aggregate_context(context, 0 as i32) }) as *mut __SlateRecord219;
    if pGCC != std::ptr::null_mut::<__SlateRecord219>() {
        unsafe {
            sqlite3_result_str(
                context,
                unsafe { std::ptr::addr_of_mut!((*pGCC).str) },
                1 as i32,
            )
        };
        unsafe { sqlite3_free((unsafe { (*pGCC).pnSepLengths }) as *mut ()) };
    }
}

#[unsafe(link_section = ".text.slate_distinct.func.groupConcatValue")]
extern "C-unwind" fn groupConcatValue(mut context: *mut sqlite3_context) {
    let mut pGCC: *mut __SlateRecord219 =
        (unsafe { sqlite3_aggregate_context(context, 0 as i32) }) as *mut __SlateRecord219;
    if pGCC != std::ptr::null_mut::<__SlateRecord219>() && (unsafe { (*pGCC).nAccum }) > (0 as i32)
    {
        unsafe {
            sqlite3_result_str(
                context,
                unsafe { std::ptr::addr_of_mut!((*pGCC).str) },
                0 as i32,
            )
        };
    }
}

/// This routine does per-connection function registration.  Most
/// of the built-in functions above are part of the global function set.
/// This routine only deals with those that are not global.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RegisterPerConnectionBuiltinFunctions(mut db: *mut sqlite3) {
    let mut rc: i32 = unsafe {
        sqlite3_overload_function(db, (b"MATCH\0".as_ptr() as *mut i8) as *const i8, 2 as i32)
    };
    0 as i32;
    if rc == (7 as i32) {
        unsafe { sqlite3OomFault(db) };
    }
}

/// Re-register the built-in LIKE functions.  The caseSensitive
/// parameter determines whether or not the LIKE operator is case
/// sensitive.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RegisterLikeFunctions(mut db: *mut sqlite3, mut caseSensitive: i32) {
    let mut pDef: *mut FuncDef = unsafe { std::mem::zeroed() };
    let mut pInfo: *mut compareInfo = unsafe { std::mem::zeroed() };
    let mut flags: i32 = 0 as i32;
    let mut nArg: i32 = 0 as i32;
    if caseSensitive != (0 as i32) {
        pInfo = (unsafe { std::ptr::addr_of!(likeInfoAlt) }) as *mut compareInfo;
        flags = (4 as i32) | (8 as i32);
    } else {
        pInfo = (unsafe { std::ptr::addr_of!(likeInfoNorm) }) as *mut compareInfo;
        flags = 4 as i32;
    }
    nArg = 2 as i32;
    '__slate_break_1333: loop {
        if !(nArg <= (3 as i32)) {
            break;
        }
        unsafe {
            sqlite3CreateFunc(
                db,
                (b"like\0".as_ptr() as *mut i8) as *const i8,
                nArg,
                1 as i32,
                pInfo as *mut (),
                Some(likeFunc),
                None,
                None,
                None,
                None,
                std::ptr::null_mut::<FuncDestructor>(),
            )
        };
        pDef = unsafe {
            sqlite3FindFunction(
                db,
                (b"like\0".as_ptr() as *mut i8) as *const i8,
                nArg,
                ((1 as i32) as i8) as u8,
                ((0 as i32) as i8) as u8,
            )
        };
        0 as i32;
        // The sqlite3CreateFunc() call above cannot fail
        // because the "like" SQL-function already exists
        let __v1462: *mut FuncDef = pDef;
        let __v1463: u32 = unsafe { (*__v1462).funcFlags };
        let __v1464: u32 = __v1463 | (flags as u32);
        unsafe {
            (*__v1462).funcFlags = __v1464;
        }
        let __v1465: *mut FuncDef = pDef;
        let __v1466: u32 = unsafe { (*__v1465).funcFlags };
        let __v1467: u32 = __v1466 & (!(2097152 as i32) as u32);
        unsafe {
            (*__v1465).funcFlags = __v1467;
        }
        let __v1460: i32 = nArg;
        let __v1461: i32 = __v1460 + (1 as i32);
        nArg = __v1461;
    }
}

/// pExpr points to an expression which implements a function.  If
/// it is appropriate to apply the LIKE optimization to that function
/// then set aWc[0] through aWc[2] to the wildcard characters and the
/// escape character and then return TRUE.  If the function is not a
/// LIKE-style function then return FALSE.
///
/// The expression "a LIKE b ESCAPE c" is only considered a valid LIKE
/// operator if c is a string literal that is exactly one byte in length.
/// That one byte is stored in aWc[3].  aWc[3] is set to zero if there is
/// no ESCAPE clause.
///
/// *pIsNocase is set to true if uppercase and lowercase are equivalent for
/// the function (default for LIKE).  If the function makes the distinction
/// between uppercase and lowercase (as does GLOB) then *pIsNocase is set to
/// false.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IsLikeFunction(
    mut db: *mut sqlite3,
    mut pExpr: *mut Expr,
    mut pIsNocase: *mut i32,
    mut aWc: *mut i8,
) -> i32 {
    let mut pDef: *mut FuncDef = unsafe { std::mem::zeroed() };
    let mut nExpr: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if !((unsafe { (*pExpr).x.pList }) != std::ptr::null_mut::<ExprList>()) {
        return 0 as i32;
    }
    nExpr = unsafe { (*unsafe { (*pExpr).x.pList }).nExpr };
    0 as i32;
    pDef = unsafe {
        sqlite3FindFunction(
            db,
            (unsafe { (*pExpr).u.zToken }) as *const i8,
            nExpr,
            ((1 as i32) as i8) as u8,
            ((0 as i32) as i8) as u8,
        )
    };
    if pDef == std::ptr::null_mut::<FuncDef>() {
        return 0 as i32;
    }
    if pDef == std::ptr::null_mut::<FuncDef>()
        || (unsafe { (*pDef).funcFlags }) & ((4 as i32) as u32) == ((0 as i32) as u32)
    {
        return 0 as i32;
    }
    // The memcpy() statement assumes that the wildcard characters are
    // the first three statements in the compareInfo structure.  The
    // asserts() that follow verify that assumption
    unsafe {
        memcpy(
            aWc as *mut (),
            (unsafe { (*pDef).pUserData }) as *const (),
            ((3 as i32) as i64) as u64,
        )
    };
    0 as i32;
    0 as i32;
    0 as i32;
    if nExpr < (3 as i32) {
        unsafe {
            *unsafe { aWc.offset((3 as i32) as isize) } = (0 as i32) as i8;
        }
    } else {
        let mut pEscape: *mut Expr = unsafe {
            (*unsafe {
                unsafe {
                    std::ptr::addr_of_mut!((*unsafe { (*pExpr).x.pList }).a) as *mut ExprList_item
                }
                .offset((2 as i32) as isize)
            })
            .pExpr
        };
        let mut zEscape: *mut i8 = unsafe { std::mem::zeroed() };
        if (((unsafe { (*pEscape).op }) as u32) as i32) != (118 as i32) {
            return 0 as i32;
        }
        0 as i32;
        zEscape = unsafe { (*pEscape).u.zToken };
        if ((unsafe { *unsafe { zEscape.offset((0 as i32) as isize) } }) as i32) == (0 as i32)
            || ((unsafe { *unsafe { zEscape.offset((1 as i32) as isize) } }) as i32) != (0 as i32)
        {
            return 0 as i32;
        }
        if ((unsafe { *unsafe { zEscape.offset((0 as i32) as isize) } }) as i32)
            == ((unsafe { *unsafe { aWc.offset((0 as i32) as isize) } }) as i32)
        {
            return 0 as i32;
        }
        if ((unsafe { *unsafe { zEscape.offset((0 as i32) as isize) } }) as i32)
            == ((unsafe { *unsafe { aWc.offset((1 as i32) as isize) } }) as i32)
        {
            return 0 as i32;
        }
        unsafe {
            *unsafe { aWc.offset((3 as i32) as isize) } =
                unsafe { *unsafe { zEscape.offset((0 as i32) as isize) } };
        }
    }
    unsafe {
        *pIsNocase =
            ((unsafe { (*pDef).funcFlags }) & ((8 as i32) as u32) == ((0 as i32) as u32)) as i32;
    }
    return 1 as i32;
}

// Mathematical Constants
/// Extra math functions that require linking with -lm
///
/// Implementation SQL functions:
///
///   ceil(X)
///   ceiling(X)
///   floor(X)
///
/// The sqlite3_user_data() pointer is a pointer to the libm implementation
/// of the underlying C function.
#[unsafe(link_section = ".text.slate_distinct.func.ceilingFunc")]
extern "C-unwind" fn ceilingFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    0 as i32;
    '__slate_break_1336: {
        match unsafe {
            sqlite3_value_numeric_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
        } {
            1 => {
                unsafe {
                    sqlite3_result_int64(context, unsafe {
                        sqlite3_value_int64(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                    })
                };
            }
            2 => {
                let mut x: Option<unsafe extern "C-unwind" fn(f64) -> f64> = unsafe {
                    std::mem::transmute::<*mut (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        unsafe { sqlite3_user_data(context) },
                    )
                };
                unsafe {
                    sqlite3_result_double(context, unsafe {
                        x.unwrap()(unsafe {
                            sqlite3_value_double(unsafe {
                                *unsafe { argv.offset((0 as i32) as isize) }
                            })
                        })
                    })
                };
            }
            _ => {}
        }
    }
}

/// On some systems, ceil() and floor() are intrinsic function.  You are
/// unable to take a pointer to these functions.  Hence, we here wrap them
/// in our own actual functions.
#[unsafe(link_section = ".text.slate_distinct.func.xCeil")]
extern "C-unwind" fn xCeil(mut x: f64) -> f64 {
    return unsafe { ceil(x) };
}

#[unsafe(link_section = ".text.slate_distinct.func.xFloor")]
extern "C-unwind" fn xFloor(mut x: f64) -> f64 {
    return unsafe { floor(x) };
}

// Some systems do not have log2() and log10() in their standard math
// libraries.
/// Implementation of SQL functions:
///
///   ln(X)       - natural logarithm
///   log(X)      - log X base 10
///   log10(X)    - log X base 10
///   log(B,X)    - log X base B
#[unsafe(link_section = ".text.slate_distinct.func.logFunc")]
extern "C-unwind" fn logFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut x: f64 = 0 as f64;
    let mut b: f64 = 0 as f64;
    let mut ans: f64 = 0 as f64;
    0 as i32;
    '__slate_break_1337: {
        match unsafe {
            sqlite3_value_numeric_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
        } {
            1 | 2 => {
                x = unsafe {
                    sqlite3_value_double(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                };
                if x <= 0.0f64 {
                    return;
                }
            }
            _ => {
                return;
            }
        }
    }
    if argc == (2 as i32) {
        match unsafe {
            sqlite3_value_numeric_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
        } {
            1 | 2 => {
                b = unsafe { log(x) };
                if b <= 0.0f64 {
                    return;
                }
                x = unsafe {
                    sqlite3_value_double(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
                };
                if x <= 0.0f64 {
                    return;
                }
            }
            _ => {
                return;
            }
        }
        ans = (unsafe { log(x) }) / b;
    } else {
        '__slate_break_1339: {
            match ((unsafe { sqlite3_user_data(context) }) as i64) as i32 {
                1 => {
                    ans = unsafe { log10(x) };
                }
                2 => {
                    ans = unsafe { log2(x) };
                }
                _ => {
                    ans = unsafe { log(x) };
                }
            }
        }
    }
    unsafe { sqlite3_result_double(context, ans) };
}

/// Functions to converts degrees to radians and radians to degrees.
#[unsafe(link_section = ".text.slate_distinct.func.degToRad")]
extern "C-unwind" fn degToRad(mut x: f64) -> f64 {
    return x * (3.141592653589793f64 / 180.0f64);
}

#[unsafe(link_section = ".text.slate_distinct.func.radToDeg")]
extern "C-unwind" fn radToDeg(mut x: f64) -> f64 {
    return x * (180.0f64 / 3.141592653589793f64);
}

/// Implementation of 1-argument SQL math functions:
///
///   exp(X)  - Compute e to the X-th power
#[unsafe(link_section = ".text.slate_distinct.func.math1Func")]
extern "C-unwind" fn math1Func(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut type0: i32 = 0 as i32;
    let mut v0: f64 = 0 as f64;
    let mut ans: f64 = 0 as f64;
    let mut x: Option<unsafe extern "C-unwind" fn(f64) -> f64> = unsafe { std::mem::zeroed() };
    0 as i32;
    type0 = unsafe {
        sqlite3_value_numeric_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
    };
    if type0 != (1 as i32) && type0 != (2 as i32) {
        return;
    }
    v0 = unsafe { sqlite3_value_double(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    x = unsafe {
        std::mem::transmute::<*mut (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(unsafe {
            sqlite3_user_data(context)
        })
    };
    ans = unsafe { x.unwrap()(v0) };
    unsafe { sqlite3_result_double(context, ans) };
}

/// Implementation of 2-argument SQL math functions:
///
///   power(X,Y)  - Compute X to the Y-th power
#[unsafe(link_section = ".text.slate_distinct.func.math2Func")]
extern "C-unwind" fn math2Func(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut type0: i32 = 0 as i32;
    let mut type1: i32 = 0 as i32;
    let mut v0: f64 = 0 as f64;
    let mut v1: f64 = 0 as f64;
    let mut ans: f64 = 0 as f64;
    let mut x: Option<unsafe extern "C-unwind" fn(f64, f64) -> f64> = unsafe { std::mem::zeroed() };
    0 as i32;
    type0 = unsafe {
        sqlite3_value_numeric_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
    };
    if type0 != (1 as i32) && type0 != (2 as i32) {
        return;
    }
    type1 = unsafe {
        sqlite3_value_numeric_type(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
    };
    if type1 != (1 as i32) && type1 != (2 as i32) {
        return;
    }
    v0 = unsafe { sqlite3_value_double(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    v1 = unsafe { sqlite3_value_double(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) };
    x = unsafe {
        std::mem::transmute::<*mut (), Option<unsafe extern "C-unwind" fn(f64, f64) -> f64>>(
            unsafe { sqlite3_user_data(context) },
        )
    };
    ans = unsafe { x.unwrap()(v0, v1) };
    unsafe { sqlite3_result_double(context, ans) };
}

/// Implementation of 0-argument pi() function.
#[unsafe(link_section = ".text.slate_distinct.func.piFunc")]
extern "C-unwind" fn piFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    0 as i32;
    argv;
    unsafe { sqlite3_result_double(context, 3.141592653589793f64) };
}

/// Implementation of sign(X) function.
#[unsafe(link_section = ".text.slate_distinct.func.signFunc")]
extern "C-unwind" fn signFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut type0: i32 = 0 as i32;
    let mut x: f64 = 0 as f64;
    argc;
    0 as i32;
    type0 = unsafe {
        sqlite3_value_numeric_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
    };
    if type0 != (1 as i32) && type0 != (2 as i32) {
        return;
    }
    x = unsafe { sqlite3_value_double(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    unsafe {
        sqlite3_result_int(
            context,
            if x < 0.0f64 {
                -(1 as i32)
            } else {
                if x > 0.0f64 { 1 as i32 } else { 0 as i32 }
            },
        )
    };
}

// This section implements the percentile(Y,P) SQL function and similar.
// Requirements:
//
//   (1)  The percentile(Y,P) function is an aggregate function taking
//        exactly two arguments.
//
//   (2)  If the P argument to percentile(Y,P) is not the same for every
//        row in the aggregate then an error is thrown.  The word "same"
//        in the previous sentence means that the value differ by less
//        than 0.001.
//
//   (3)  If the P argument to percentile(Y,P) evaluates to anything other
//        than a number in the range of 0.0 to 100.0 inclusive then an
//        error is thrown.
//
//   (4)  If any Y argument to percentile(Y,P) evaluates to a value that
//        is not NULL and is not numeric then an error is thrown.
//
//   (5)  If any Y argument to percentile(Y,P) evaluates to plus or minus
//        infinity then an error is thrown.  (SQLite always interprets NaN
//        values as NULL.)
//
//   (6)  Both Y and P in percentile(Y,P) can be arbitrary expressions,
//        including CASE WHEN expressions.
//
//   (7)  The percentile(Y,P) aggregate is able to handle inputs of at least
//        one million (1,000,000) rows.
//
//   (8)  If there are no non-NULL values for Y, then percentile(Y,P)
//        returns NULL.
//
//   (9)  If there is exactly one non-NULL value for Y, the percentile(Y,P)
//        returns the one Y value.
//
//  (10)  If there N non-NULL values of Y where N is two or more and
//        the Y values are ordered from least to greatest and a graph is
//        drawn from 0 to N-1 such that the height of the graph at J is
//        the J-th Y value and such that straight lines are drawn between
//        adjacent Y values, then the percentile(Y,P) function returns
//        the height of the graph at P*(N-1)/100.
//
//  (11)  The percentile(Y,P) function always returns either a floating
//        point number or NULL.
//
//  (12)  The percentile(Y,P) is implemented as a single C99 source-code
//        file that compiles into a shared-library or DLL that can be loaded
//        into SQLite using the sqlite3_load_extension() interface.
//
//  (13)  A separate median(Y) function is the equivalent percentile(Y,50).
//
//  (14)  A separate percentile_cont(Y,P) function is equivalent to
//        percentile(Y,P/100.0).  In other words, the fraction value in
//        the second argument is in the range of 0 to 1 instead of 0 to 100.
//
//  (15)  A separate percentile_disc(Y,P) function is like
//        percentile_cont(Y,P) except that instead of returning the weighted
//        average of the nearest two input values, it returns the next lower
//        value.  So the percentile_disc(Y,P) will always return a value
//        that was one of the inputs.
//
//  (16)  All of median(), percentile(Y,P), percentile_cont(Y,P) and
//        percentile_disc(Y,P) can be used as window functions.
//
// Differences from standard SQL:
//
//  *  The percentile_cont(X,P) function is equivalent to the following in
//     standard SQL:
//
//         (percentile_cont(P) WITHIN GROUP (ORDER BY X))
//
//     The SQLite syntax is much more compact.  The standard SQL syntax
//     is also supported if SQLite is compiled with the
//     -DSQLITE_ENABLE_ORDERED_SET_AGGREGATES option.
//
//  *  No median(X) function exists in the SQL standard.  App developers
//     are expected to write "percentile_cont(0.5)WITHIN GROUP(ORDER BY X)".
//
//  *  No percentile(Y,P) function exists in the SQL standard.  Instead of
//     percential(Y,P), developers must write this:
//     "percentile_cont(P/100.0) WITHIN GROUP (ORDER BY Y)".  Note that
//     the fraction parameter to percentile() goes from 0 to 100 whereas
//     the fraction parameter in SQL standard percentile_cont() goes from
//     0 to 1.
//
// Implementation notes as of 2024-08-31:
//
//  *  The regular aggregate-function versions of these routines work
//     by accumulating all values in an array of doubles, then sorting
//     that array using quicksort before computing the answer. Thus
//     the runtime is O(NlogN) where N is the number of rows of input.
//
//  *  For the window-function versions of these routines, the array of
//     inputs is sorted as soon as the first value is computed.  Thereafter,
//     the array is kept in sorted order using an insert-sort.  This
//     results in O(N*K) performance where K is the size of the window.
//     One can imagine alternative implementations that give O(N*logN*logK)
//     performance, but they require more complex logic and data structures.
//     The developers have elected to keep the asymptotically slower
//     algorithm for now, for simplicity, under the theory that window
//     functions are seldom used and when they are, the window size K is
//     often small.  The developers might revisit that decision later,
//     should the need arise.
/// The following object is the group context for a single percentile()
/// aggregate.  Remember all input Y values until the very end.
/// Those values are accumulated in the Percentile.a[] array.
#[repr(C)]
#[derive(Clone, Copy)]
struct Percentile {
    /// Number of slots allocated for a[]
    nAlloc: u64,
    /// Number of slots actually used in a[]
    nUsed: u64,
    /// True if a[] is already in sorted order
    bSorted: i8,
    /// True if advantageous to keep a[] sorted
    bKeepSorted: i8,
    /// True if rPct is valid
    bPctValid: i8,
    /// Fraction.  0.0 to 1.0
    rPct: f64,
    /// Array of Y values
    a: *mut f64,
}

/// Return TRUE if the input floating-point number is an infinity.
fn percentIsInfinity(mut r: f64) -> i32 {
    let mut u: u64 = 0 as u64;
    0 as i32;
    unsafe {
        memcpy(
            std::ptr::addr_of_mut!(u) as *mut (),
            std::ptr::addr_of_mut!(r) as *const (),
            8 as u64,
        )
    };
    return (u >> (52 as i32) & (((2047 as i32) as i64) as u64) == (((2047 as i32) as i64) as u64))
        as i32;
}

/// Return TRUE if two doubles differ by 0.001 or less.
fn percentSameValue(mut a: f64, mut b: f64) -> i32 {
    let __v1707: f64 = a;
    let __v1708: f64 = __v1707 - b;
    a = __v1708;
    return (a >= -0.001f64 && a <= 0.001f64) as i32;
}

/// Search p (which must have p->bSorted) looking for an entry with
/// value y.  Return the index of that entry.
///
/// If bExact is true, return -1 if the entry is not found.
///
/// If bExact is false, return the index at which a new entry with
/// value y should be insert in order to keep the values in sorted
/// order.  The smallest return value in this case will be 0, and
/// the largest return value will be p->nUsed.
fn percentBinarySearch(mut p: *mut Percentile, mut y: f64, mut bExact: i32) -> i64 {
    let mut iFirst: i64 = (0 as i32) as i64; // First element of search range
    let mut iLast: i64 = ((unsafe { (*p).nUsed }) as i64) - ((1 as i32) as i64); // Last element of search range
    '__slate_break_1340: while iLast >= iFirst {
        let mut iMid: i64 = (iFirst + iLast) / ((2 as i32) as i64);
        let mut x: f64 = unsafe { *unsafe { unsafe { (*p).a }.offset(iMid as isize) } };
        if x < y {
            iFirst = iMid + ((1 as i32) as i64);
        } else {
            if x > y {
                iLast = iMid - ((1 as i32) as i64);
            } else {
                return iMid;
            }
        }
    }
    if bExact != (0 as i32) {
        return -(1 as i32) as i64;
    }
    return iFirst;
}

/// Generate an error for a percentile function.
///
/// The error format string must have exactly one occurrence of "%%s()"
/// (with two '%' characters).  That substring will be replaced by the name
/// of the function.
unsafe extern "C-unwind" fn percentError(
    mut pCtx: *mut sqlite3_context,
    mut zFormat: *const i8,
    mut __va_args: ...
) {
    let mut zMsg1: *mut i8 = unsafe { std::mem::zeroed() };
    let mut zMsg2: *mut i8 = unsafe { std::mem::zeroed() };
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    ap = __va_args.clone();
    zMsg1 = unsafe { sqlite3_vmprintf(zFormat, ap.clone()) };
    {}
    let __v1709: *mut i8;
    if zMsg1 != std::ptr::null_mut::<i8>() {
        __v1709 = unsafe {
            sqlite3_mprintf(zMsg1 as *const i8, unsafe {
                sqlite3VdbeFuncName(pCtx as *const sqlite3_context)
            })
        };
    } else {
        __v1709 = std::ptr::null_mut::<i8>();
    }
    zMsg2 = __v1709;
    unsafe { sqlite3_result_error(pCtx, zMsg2 as *const i8, -(1 as i32)) };
    unsafe { sqlite3_free(zMsg1 as *mut ()) };
    unsafe { sqlite3_free(zMsg2 as *mut ()) };
}

/// The "step" function for percentile(Y,P) is called once for each
/// input row.
#[unsafe(link_section = ".text.slate_distinct.func.percentStep")]
extern "C-unwind" fn percentStep(
    mut pCtx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut p: *mut Percentile = unsafe { std::mem::zeroed() };
    let mut rPct: f64 = 0 as f64;
    let mut eType: i32 = 0 as i32;
    let mut y: f64 = 0 as f64;
    0 as i32;
    if argc == (1 as i32) {
        // Requirement 13:  median(Y) is the same as percentile(Y,50).
        rPct = 0.5f64;
    } else {
        // P must be a number between 0 and 100 for percentile() or between
        // 0.0 and 1.0 for percentile_cont() and percentile_disc().
        //
        // The user-data is an integer which is 10 times the upper bound.
        let mut mxFrac: f64 =
            if (((unsafe { sqlite3_user_data(pCtx) }) as i64) as i32) & (2 as i32) != (0 as i32) {
                100.0f64
            } else {
                1.0f64
            };
        eType = unsafe {
            sqlite3_value_numeric_type(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
        };
        rPct = (unsafe {
            sqlite3_value_double(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
        }) / mxFrac;
        if eType != (1 as i32) && eType != (2 as i32) || rPct < 0.0f64 || rPct > 1.0f64 {
            unsafe {
                percentError(
                    pCtx,
                    (b"the fraction argument to %%s() is not between 0.0 and %.1f\0".as_ptr()
                        as *mut i8) as *const i8,
                    mxFrac,
                )
            };
            return;
        }
    }
    // Allocate the session context.
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((40 as u64) as u32) as i32) })
        as *mut Percentile;
    if p == std::ptr::null_mut::<Percentile>() {
        return;
    }
    // Remember the P value.  Throw an error if the P value is different
    // from any prior row, per Requirement (2).
    if !((unsafe { (*p).bPctValid }) != (0 as i8)) {
        unsafe {
            (*p).rPct = rPct;
        }
        unsafe {
            (*p).bPctValid = (1 as i32) as i8;
        }
    } else {
        if !(percentSameValue(unsafe { (*p).rPct }, rPct) != (0 as i32)) {
            unsafe {
                percentError(
                    pCtx,
                    (b"the fraction argument to %%s() is not the same for all input rows\0".as_ptr()
                        as *mut i8) as *const i8,
                )
            };
            return;
        }
    }
    // Ignore rows for which Y is NULL
    eType = unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    if eType == (5 as i32) {
        return;
    }
    // If not NULL, then Y must be numeric.  Otherwise throw an error.
    // Requirement 4
    if eType != (1 as i32) && eType != (2 as i32) {
        unsafe {
            percentError(
                pCtx,
                (b"input to %%s() is not numeric\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        return;
    }
    // Throw an error if the Y value is infinity or NaN
    y = unsafe { sqlite3_value_double(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    if percentIsInfinity(y) != (0 as i32) {
        unsafe {
            percentError(
                pCtx,
                (b"Inf input to %%s()\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        return;
    }
    // Allocate and store the Y
    if (unsafe { (*p).nUsed }) >= unsafe { (*p).nAlloc } {
        let mut n: u64 = unsafe { (*p).nAlloc }
            .wrapping_mul(((2 as i32) as i64) as u64)
            .wrapping_add(((250 as i32) as i64) as u64);
        let mut a: *mut f64 = (unsafe {
            sqlite3_realloc64((unsafe { (*p).a }) as *mut (), (8 as u64).wrapping_mul(n))
        }) as *mut f64;
        if a == std::ptr::null_mut::<f64>() {
            unsafe { sqlite3_free((unsafe { (*p).a }) as *mut ()) };
            unsafe { memset(p as *mut (), 0 as i32, 40 as u64) };
            unsafe { sqlite3_result_error_nomem(pCtx) };
            return;
        }
        unsafe {
            (*p).nAlloc = n;
        }
        unsafe {
            (*p).a = a;
        }
    }
    if (unsafe { (*p).nUsed }) == (((0 as i32) as i64) as u64) {
        let __v1710: *mut Percentile = p;
        let __v1711: u64 = unsafe { (*__v1710).nUsed };
        let __v1712: u64 = __v1711.wrapping_add(((1 as i32) as i64) as u64);
        unsafe {
            (*__v1710).nUsed = __v1712;
        }
        unsafe {
            *unsafe { unsafe { (*p).a }.offset(__v1711 as isize) } = y;
        }
        unsafe {
            (*p).bSorted = (1 as i32) as i8;
        }
    } else {
        if !((unsafe { (*p).bSorted }) != (0 as i8))
            || y >= unsafe {
                *unsafe {
                    unsafe { (*p).a }.offset(
                        unsafe { (*p).nUsed }.wrapping_sub(((1 as i32) as i64) as u64) as isize,
                    )
                }
            }
        {
            let __v1713: *mut Percentile = p;
            let __v1714: u64 = unsafe { (*__v1713).nUsed };
            let __v1715: u64 = __v1714.wrapping_add(((1 as i32) as i64) as u64);
            unsafe {
                (*__v1713).nUsed = __v1715;
            }
            unsafe {
                *unsafe { unsafe { (*p).a }.offset(__v1714 as isize) } = y;
            }
        } else {
            if (unsafe { (*p).bKeepSorted }) != (0 as i8) {
                let mut i: i64 = 0 as i64;
                i = percentBinarySearch(p, y, 0 as i32);
                if i < ((((unsafe { (*p).nUsed }) as u32) as i32) as i64) {
                    unsafe {
                        memmove(
                            (unsafe {
                                unsafe { (*p).a }.offset((i + ((1 as i32) as i64)) as isize)
                            }) as *mut (),
                            (unsafe { unsafe { (*p).a }.offset(i as isize) }) as *const (),
                            unsafe { (*p).nUsed }
                                .wrapping_sub(i as u64)
                                .wrapping_mul(8 as u64),
                        )
                    };
                }
                unsafe {
                    *unsafe { unsafe { (*p).a }.offset(i as isize) } = y;
                }
                let __v1716: *mut Percentile = p;
                let __v1717: u64 = unsafe { (*__v1716).nUsed };
                let __v1718: u64 = __v1717.wrapping_add(((1 as i32) as i64) as u64);
                unsafe {
                    (*__v1716).nUsed = __v1718;
                }
            } else {
                let __v1719: *mut Percentile = p;
                let __v1720: u64 = unsafe { (*__v1719).nUsed };
                let __v1721: u64 = __v1720.wrapping_add(((1 as i32) as i64) as u64);
                unsafe {
                    (*__v1719).nUsed = __v1721;
                }
                unsafe {
                    *unsafe { unsafe { (*p).a }.offset(__v1720 as isize) } = y;
                }
                unsafe {
                    (*p).bSorted = (0 as i32) as i8;
                }
            }
        }
    }
}

// Interchange two doubles.
/// Sort an array of doubles.
///
/// Algorithm: quicksort
///
/// This is implemented separately rather than using the qsort() routine
/// from the standard library because:
///
///    (1)  To avoid a dependency on qsort()
///    (2)  To avoid the function call to the comparison routine for each
///         comparison.
///
/// If parameter iReq is non-negative, then the caller will only access
/// elements a[iReq] and a[iReq+1] (if it exists) of the sorted array and
/// so it is not necessary to position any other elements. Or if iReq is
/// negative, then the final array must be fully sorted.
///
/// # Arguments
///
/// * `a` - Array to sort
/// * `n` - Number of elements in array a[]
/// * `iReq` - Element caller cares about (or -ve)
fn percentSort(mut a: *mut f64, mut n: u32, mut iReq: i32) {
    let mut iLt: i32 = 0 as i32; // Entries before a[iLt] are less than or equal to rPivot
    let mut iGt: i32 = 0 as i32; // Entries a[iGt] and after are greater or equal to rPivot
    let mut i: i32 = 0 as i32; // Loop counter
    let mut rPivot: f64 = 0 as f64; // The pivot value
    0 as i32;
    '__slate_break_1345: loop {
        // Put the first, middle, and last elements in sorted order.
        // After doing so, return immediately if the array contains
        // three or fewer elements as there is nothing more to do.
        if (unsafe { *unsafe { a.offset((0 as i32) as isize) } })
            > unsafe { *unsafe { a.offset(n.wrapping_sub((1 as i32) as u32) as isize) } }
        {
            let mut ttt: f64 = unsafe { *unsafe { a.offset((0 as i32) as isize) } };
            unsafe {
                *unsafe { a.offset((0 as i32) as isize) } =
                    unsafe { *unsafe { a.offset(n.wrapping_sub((1 as i32) as u32) as isize) } };
            }
            unsafe {
                *unsafe { a.offset(n.wrapping_sub((1 as i32) as u32) as isize) } = ttt;
            }
        }
        if n == ((2 as i32) as u32) {
            return;
        }
        iGt = n.wrapping_sub((1 as i32) as u32) as i32;
        i = (n / ((2 as i32) as u32)) as i32;
        if (unsafe { *unsafe { a.offset((0 as i32) as isize) } })
            > unsafe { *unsafe { a.offset(i as isize) } }
        {
            let mut ttt: f64 = unsafe { *unsafe { a.offset((0 as i32) as isize) } };
            unsafe {
                *unsafe { a.offset((0 as i32) as isize) } =
                    unsafe { *unsafe { a.offset(i as isize) } };
            }
            unsafe {
                *unsafe { a.offset(i as isize) } = ttt;
            }
        } else {
            if (unsafe { *unsafe { a.offset(i as isize) } })
                > unsafe { *unsafe { a.offset(iGt as isize) } }
            {
                let mut ttt: f64 = unsafe { *unsafe { a.offset(i as isize) } };
                unsafe {
                    *unsafe { a.offset(i as isize) } =
                        unsafe { *unsafe { a.offset(iGt as isize) } };
                }
                unsafe {
                    *unsafe { a.offset(iGt as isize) } = ttt;
                }
            }
        }
        if n == ((3 as i32) as u32) {
            return;
        }
        // Take the value of the middle element as the pivot.  Shuffle
        // values around so that all elements less than the pivot come
        // before all elements greater than the pivot.
        rPivot = unsafe { *unsafe { a.offset(i as isize) } };
        i = 1 as i32;
        iLt = 1 as i32;
        '__slate_break_1346: loop {
            if (unsafe { *unsafe { a.offset(i as isize) } }) < rPivot {
                if i > iLt {
                    let mut ttt: f64 = unsafe { *unsafe { a.offset(i as isize) } };
                    unsafe {
                        *unsafe { a.offset(i as isize) } =
                            unsafe { *unsafe { a.offset(iLt as isize) } };
                    }
                    unsafe {
                        *unsafe { a.offset(iLt as isize) } = ttt;
                    }
                }
                let __v1722: i32 = iLt;
                let __v1723: i32 = __v1722 + (1 as i32);
                iLt = __v1723;
                let __v1724: i32 = i;
                let __v1725: i32 = __v1724 + (1 as i32);
                i = __v1725;
            } else {
                if (unsafe { *unsafe { a.offset(i as isize) } }) > rPivot {
                    '__slate_break_1347: loop {
                        let __v1726: i32 = iGt;
                        let __v1727: i32 = __v1726 - (1 as i32);
                        iGt = __v1727;
                        if !(iGt > i && (unsafe { *unsafe { a.offset(iGt as isize) } }) > rPivot) {
                            break;
                        }
                    }
                    let mut ttt: f64 = unsafe { *unsafe { a.offset(i as isize) } };
                    unsafe {
                        *unsafe { a.offset(i as isize) } =
                            unsafe { *unsafe { a.offset(iGt as isize) } };
                    }
                    unsafe {
                        *unsafe { a.offset(iGt as isize) } = ttt;
                    }
                } else {
                    let __v1728: i32 = i;
                    let __v1729: i32 = __v1728 + (1 as i32);
                    i = __v1729;
                }
            }
            if !(i < iGt) {
                break;
            }
        }
        0 as i32;
        {}
        0 as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        if iReq >= (0 as i32) {
            // In this case, the only elements that the caller requires sorted into
            // the correct positions are elements a[iReq] and a[iReq+1]. At this
            // point we know that element a[iLt] is in the correct position and
            // all elements smaller than a[iLt] are in the left-hand partition.
            // So if (iReq<iLt), then it is only necessary to sort the left
            // partition.
            //
            // If (iReq>=iLt), then elements iReq and iReq+1 are either in the
            // right partition or the equal partition (elements for which
            // iLt<=iElem<iGt). Therefore it is always sufficient to sort only
            // the right partition in this case.
            if iReq < iLt {
                n = iLt as u32;
            } else {
                let __v1730: *mut f64 = a;
                let __v1731: *mut f64 = unsafe { __v1730.offset(iGt as isize) };
                a = __v1731;
                let __v1732: u32 = n;
                let __v1733: u32 = __v1732.wrapping_sub(iGt as u32);
                n = __v1733;
                iReq = if (0 as i32) > iReq - iGt {
                    0 as i32
                } else {
                    iReq - iGt
                };
            }
        } else {
            // Recurse on the smaller partition only.  The smaller partition
            // will hold n/2 or fewer entries, which assures that the stack
            // depth will not exceed O(log(n)), even for pathological cases.
            // Loop without recursion for the larger partition.
            if iLt > ((n / ((2 as i32) as u32)) as i32) {
                if n.wrapping_sub(iGt as u32) >= ((2 as i32) as u32) {
                    percentSort(
                        unsafe { a.offset(iGt as isize) },
                        n.wrapping_sub(iGt as u32),
                        -(1 as i32),
                    );
                }
                n = iLt as u32;
            } else {
                if iLt >= (2 as i32) {
                    percentSort(a, iLt as u32, -(1 as i32));
                }
                let __v1734: *mut f64 = a;
                let __v1735: *mut f64 = unsafe { __v1734.offset(iGt as isize) };
                a = __v1735;
                let __v1736: u32 = n;
                let __v1737: u32 = __v1736.wrapping_sub(iGt as u32);
                n = __v1737;
            }
        }
        if !(n >= ((2 as i32) as u32)) {
            break;
        }
    }
}

/// The "inverse" function for percentile(Y,P) is called to remove a
/// row that was previously inserted by "step".
#[unsafe(link_section = ".text.slate_distinct.func.percentInverse")]
extern "C-unwind" fn percentInverse(
    mut pCtx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut p: *mut Percentile = unsafe { std::mem::zeroed() };
    let mut eType: i32 = 0 as i32;
    let mut y: f64 = 0 as f64;
    let mut i: i64 = 0 as i64;
    0 as i32;
    // Allocate the session context.
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((40 as u64) as u32) as i32) })
        as *mut Percentile;
    0 as i32;
    // Ignore rows for which Y is NULL
    eType = unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    if eType == (5 as i32) {
        return;
    }
    // If not NULL, then Y must be numeric.  Otherwise throw an error.
    // Requirement 4
    if eType != (1 as i32) && eType != (2 as i32) {
        return;
    }
    // Ignore the Y value if it is infinity or NaN
    y = unsafe { sqlite3_value_double(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    if percentIsInfinity(y) != (0 as i32) {
        return;
    }
    if ((unsafe { (*p).bSorted }) as i32) == (0 as i32) {
        0 as i32;
        percentSort(
            unsafe { (*p).a },
            (unsafe { (*p).nUsed }) as u32,
            -(1 as i32),
        );
        unsafe {
            (*p).bSorted = (1 as i32) as i8;
        }
    }
    unsafe {
        (*p).bKeepSorted = (1 as i32) as i8;
    }
    // Find and remove the row
    i = percentBinarySearch(p, y, 1 as i32);
    if i >= ((0 as i32) as i64) {
        let __v1738: *mut Percentile = p;
        let __v1739: u64 = unsafe { (*__v1738).nUsed };
        let __v1740: u64 = __v1739.wrapping_sub(((1 as i32) as i64) as u64);
        unsafe {
            (*__v1738).nUsed = __v1740;
        }
        if i < ((((unsafe { (*p).nUsed }) as u32) as i32) as i64) {
            unsafe {
                memmove(
                    (unsafe { unsafe { (*p).a }.offset(i as isize) }) as *mut (),
                    (unsafe { unsafe { (*p).a }.offset((i + ((1 as i32) as i64)) as isize) })
                        as *const (),
                    unsafe { (*p).nUsed }
                        .wrapping_sub(i as u64)
                        .wrapping_mul(8 as u64),
                )
            };
        }
    }
}

/// Compute the final output of percentile().  Clean up all allocated
/// memory if and only if bIsFinal is true.
fn percentCompute(mut pCtx: *mut sqlite3_context, mut bIsFinal: i32) {
    let mut p: *mut Percentile = unsafe { std::mem::zeroed() };
    let mut settings: i32 = (((unsafe { sqlite3_user_data(pCtx) }) as i64) as i32) & (1 as i32); // Discrete?
    let mut i1: u32 = 0 as u32;
    let mut i2: u32 = 0 as u32;
    let mut v1: f64 = 0 as f64;
    let mut v2: f64 = 0 as f64;
    let mut ix: f64 = 0 as f64;
    let mut vx: f64 = 0 as f64;
    p = (unsafe { sqlite3_aggregate_context(pCtx, 0 as i32) }) as *mut Percentile;
    if p == std::ptr::null_mut::<Percentile>() {
        return;
    }
    if (unsafe { (*p).a }) == std::ptr::null_mut::<f64>() {
        return;
    }
    if (unsafe { (*p).nUsed }) != (0 as u64) {
        ix = (unsafe { (*p).rPct })
            * (unsafe { (*p).nUsed }.wrapping_sub(((1 as i32) as i64) as u64) as f64);
        i1 = ix as u32;
        if ((unsafe { (*p).bSorted }) as i32) == (0 as i32) {
            // In cases where bIsFinal is non-zero, setting Percentile.bSorted
            // after the percentSort() call here is not technically correct, as
            // the array is not fully sorted. But in this case the object will be
            // freed below anyway, so it doesn't matter.
            0 as i32;
            percentSort(
                unsafe { (*p).a },
                (unsafe { (*p).nUsed }) as u32,
                if bIsFinal != (0 as i32) {
                    i1 as i32
                } else {
                    -(1 as i32)
                },
            );
            unsafe {
                (*p).bSorted = (1 as i32) as i8;
            }
        }
        if settings & (1 as i32) != (0 as i32) {
            vx = unsafe { *unsafe { unsafe { (*p).a }.offset(i1 as isize) } };
        } else {
            i2 = if ix == (i1 as f64)
                || (i1 as u64) == unsafe { (*p).nUsed }.wrapping_sub(((1 as i32) as i64) as u64)
            {
                i1
            } else {
                i1.wrapping_add((1 as i32) as u32)
            };
            v1 = unsafe { *unsafe { unsafe { (*p).a }.offset(i1 as isize) } };
            v2 = unsafe { *unsafe { unsafe { (*p).a }.offset(i2 as isize) } };
            vx = v1 + (v2 - v1) * (ix - (i1 as f64));
        }
        unsafe { sqlite3_result_double(pCtx, vx) };
    }
    if bIsFinal != (0 as i32) {
        unsafe { sqlite3_free((unsafe { (*p).a }) as *mut ()) };
        unsafe { memset(p as *mut (), 0 as i32, 40 as u64) };
    } else {
        unsafe {
            (*p).bKeepSorted = (1 as i32) as i8;
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.func.percentFinal")]
extern "C-unwind" fn percentFinal(mut pCtx: *mut sqlite3_context) {
    percentCompute(pCtx, 1 as i32);
}

#[unsafe(link_section = ".text.slate_distinct.func.percentValue")]
extern "C-unwind" fn percentValue(mut pCtx: *mut sqlite3_context) {
    percentCompute(pCtx, 0 as i32);
}

// End of percentile family of functions
/// All of the FuncDef structures in the aBuiltinFunc[] array above
/// to the global function hash table.  This occurs at start-time (as
/// a consequence of calling sqlite3_initialize()).
///
/// After this routine runs
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RegisterBuiltinFunctions() {
    // The following array holds FuncDef structures for all of the functions
    // defined in this file.
    //
    // The array cannot be constant since changes are made to the
    // FuncDef.pHash elements at start-time.  The elements of this array
    // are read-only after initialization is complete.
    //
    // For peak efficiency, put the most frequently used function last.
    // Functions only available with SQLITE_TESTCTRL_INTERNAL_FUNCTIONS
    // Regular functions
    unsafe { sqlite3AlterFunctions() };
    unsafe { sqlite3WindowFunctions() };
    unsafe { sqlite3RegisterDateTimeFunctions() };
    unsafe { sqlite3RegisterJsonFunctions() };
    unsafe {
        sqlite3InsertBuiltinFuncs(
            unsafe { std::ptr::addr_of_mut!(aBuiltinFunc.0) as *mut FuncDef },
            (((7920 as u64) / (72 as u64)) as u32) as i32,
        )
    };
}

static mut aBuiltinFunc: __SlateAlign16<[FuncDef; 110]> = __SlateAlign16([
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (262144 as i32)
            | (16384 as i32)
            | (4194304 as i32)
            | (2048 as i32)
            | (0 as i32)) as u32,
        pUserData: ((1 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(versionFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"implies_nonnull_row\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t0: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t0.pHash = std::ptr::null_mut::<FuncDef>();
            __t0
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (262144 as i32)
            | (16384 as i32)
            | (4194304 as i32)
            | (2048 as i32)
            | (0 as i32)) as u32,
        pUserData: ((3 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(versionFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"expr_compare\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t1: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t1.pHash = std::ptr::null_mut::<FuncDef>();
            __t1
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (262144 as i32)
            | (16384 as i32)
            | (4194304 as i32)
            | (2048 as i32)
            | (0 as i32)) as u32,
        pUserData: ((2 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(versionFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"expr_implies_expr\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t2: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t2.pHash = std::ptr::null_mut::<FuncDef>();
            __t2
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (262144 as i32)
            | (16384 as i32)
            | (4194304 as i32)
            | (2048 as i32)
            | (0 as i32)) as u32,
        pUserData: ((4 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(versionFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"affinity\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t3: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t3.pHash = std::ptr::null_mut::<FuncDef>();
            __t3
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (524288 as i32) | (2097152 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(loadExt),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"load_extension\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t4: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t4.pHash = std::ptr::null_mut::<FuncDef>();
            __t4
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (524288 as i32) | (2097152 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(loadExt),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"load_extension\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t5: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t5.pHash = std::ptr::null_mut::<FuncDef>();
            __t5
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (8192 as i32) | (1 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(compileoptionusedFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqlite_compileoption_used\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t6: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t6.pHash = std::ptr::null_mut::<FuncDef>();
            __t6
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (8192 as i32) | (1 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(compileoptiongetFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqlite_compileoption_get\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t7: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t7.pHash = std::ptr::null_mut::<FuncDef>();
            __t7
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (4194304 as i32)
            | (2048 as i32)
            | (1024 as i32)) as u32,
        pUserData: ((99 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(versionFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"unlikely\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t8: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t8.pHash = std::ptr::null_mut::<FuncDef>();
            __t8
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (4194304 as i32)
            | (2048 as i32)
            | (1024 as i32)) as u32,
        pUserData: ((99 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(versionFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"likelihood\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t9: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t9.pHash = std::ptr::null_mut::<FuncDef>();
            __t9
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (4194304 as i32)
            | (2048 as i32)
            | (1024 as i32)) as u32,
        pUserData: ((99 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(versionFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"likely\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t10: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t10.pHash = std::ptr::null_mut::<FuncDef>();
            __t10
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (4194304 as i32) | (2048 as i32) | (0 as i32))
            as u32,
        pUserData: ((6 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(versionFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqlite_offset\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t11: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t11.pHash = std::ptr::null_mut::<FuncDef>();
            __t11
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: ((1 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(trimFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"ltrim\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t12: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t12.pHash = std::ptr::null_mut::<FuncDef>();
            __t12
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: ((1 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(trimFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"ltrim\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t13: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t13.pHash = std::ptr::null_mut::<FuncDef>();
            __t13
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: ((2 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(trimFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"rtrim\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t14: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t14.pHash = std::ptr::null_mut::<FuncDef>();
            __t14
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: ((2 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(trimFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"rtrim\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t15: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t15.pHash = std::ptr::null_mut::<FuncDef>();
            __t15
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: ((3 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(trimFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"trim\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t16: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t16.pHash = std::ptr::null_mut::<FuncDef>();
            __t16
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: ((3 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(trimFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"trim\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t17: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t17.pHash = std::ptr::null_mut::<FuncDef>();
            __t17
        },
    },
    FuncDef {
        nArg: -(3 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (1 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(minmaxFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"min\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t18: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t18.pHash = std::ptr::null_mut::<FuncDef>();
            __t18
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (1 as i32) * (32 as i32)
            | (4096 as i32)
            | (134217728 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(minmaxStep),
        xFinalize: Some(minMaxFinalize),
        xValue: Some(minMaxValue),
        xInverse: None,
        zName: (b"min\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t19: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t19.pHash = std::ptr::null_mut::<FuncDef>();
            __t19
        },
    },
    FuncDef {
        nArg: -(3 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (1 as i32) * (32 as i32))
            as u32,
        pUserData: ((1 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(minmaxFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"max\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t20: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t20.pHash = std::ptr::null_mut::<FuncDef>();
            __t20
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (1 as i32) * (32 as i32)
            | (4096 as i32)
            | (134217728 as i32)) as u32,
        pUserData: ((1 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(minmaxStep),
        xFinalize: Some(minMaxFinalize),
        xValue: Some(minMaxValue),
        xInverse: None,
        zName: (b"max\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t21: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t21.pHash = std::ptr::null_mut::<FuncDef>();
            __t21
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (0 as i32) * (32 as i32)
            | (128 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(typeofFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"typeof\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t22: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t22.pHash = std::ptr::null_mut::<FuncDef>();
            __t22
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (0 as i32) * (32 as i32)
            | (128 as i32)
            | (1048576 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(subtypeFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"subtype\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t23: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t23.pHash = std::ptr::null_mut::<FuncDef>();
            __t23
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (0 as i32) * (32 as i32)
            | (64 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(lengthFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"length\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t24: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t24.pHash = std::ptr::null_mut::<FuncDef>();
            __t24
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (0 as i32) * (32 as i32)
            | (192 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(bytelengthFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"octet_length\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t25: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t25.pHash = std::ptr::null_mut::<FuncDef>();
            __t25
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(instrFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"instr\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t26: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t26.pHash = std::ptr::null_mut::<FuncDef>();
            __t26
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(printfFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"printf\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t27: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t27.pHash = std::ptr::null_mut::<FuncDef>();
            __t27
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(printfFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"format\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t28: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t28.pHash = std::ptr::null_mut::<FuncDef>();
            __t28
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(unicodeFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"unicode\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t29: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t29.pHash = std::ptr::null_mut::<FuncDef>();
            __t29
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(charFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"char\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t30: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t30.pHash = std::ptr::null_mut::<FuncDef>();
            __t30
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(absFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"abs\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t31: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t31.pHash = std::ptr::null_mut::<FuncDef>();
            __t31
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(roundFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"round\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t32: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t32.pHash = std::ptr::null_mut::<FuncDef>();
            __t32
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(roundFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"round\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t33: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t33.pHash = std::ptr::null_mut::<FuncDef>();
            __t33
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(upperFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"upper\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t34: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t34.pHash = std::ptr::null_mut::<FuncDef>();
            __t34
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(lowerFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"lower\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t35: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t35.pHash = std::ptr::null_mut::<FuncDef>();
            __t35
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(hexFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"hex\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t36: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t36.pHash = std::ptr::null_mut::<FuncDef>();
            __t36
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(unhexFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"unhex\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t37: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t37.pHash = std::ptr::null_mut::<FuncDef>();
            __t37
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(unhexFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"unhex\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t38: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t38.pHash = std::ptr::null_mut::<FuncDef>();
            __t38
        },
    },
    FuncDef {
        nArg: -(3 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(concatFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"concat\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t39: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t39.pHash = std::ptr::null_mut::<FuncDef>();
            __t39
        },
    },
    FuncDef {
        nArg: -(4 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(concatwsFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"concat_ws\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t40: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t40.pHash = std::ptr::null_mut::<FuncDef>();
            __t40
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (4194304 as i32) | (2048 as i32) | (0 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(versionFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"ifnull\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t41: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t41.pHash = std::ptr::null_mut::<FuncDef>();
            __t41
        },
    },
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (0 as i32) * (32 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(randomFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"random\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t42: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t42.pHash = std::ptr::null_mut::<FuncDef>();
            __t42
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (0 as i32) * (32 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(randomBlob),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"randomblob\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t43: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t43.pHash = std::ptr::null_mut::<FuncDef>();
            __t43
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (1 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(nullifFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"nullif\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t44: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t44.pHash = std::ptr::null_mut::<FuncDef>();
            __t44
        },
    },
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32) | (8192 as i32) | (1 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(versionFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqlite_version\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t45: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t45.pHash = std::ptr::null_mut::<FuncDef>();
            __t45
        },
    },
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32) | (8192 as i32) | (1 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(sourceidFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqlite_source_id\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t46: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t46.pHash = std::ptr::null_mut::<FuncDef>();
            __t46
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (524288 as i32) | (2097152 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(errlogFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqlite_log\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t47: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t47.pHash = std::ptr::null_mut::<FuncDef>();
            __t47
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(unistrFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"unistr\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t48: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t48.pHash = std::ptr::null_mut::<FuncDef>();
            __t48
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(quoteFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"quote\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t49: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t49.pHash = std::ptr::null_mut::<FuncDef>();
            __t49
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: ((1 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(quoteFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"unistr_quote\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t50: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t50.pHash = std::ptr::null_mut::<FuncDef>();
            __t50
        },
    },
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (0 as i32) * (32 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(last_insert_rowid),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"last_insert_rowid\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t51: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t51.pHash = std::ptr::null_mut::<FuncDef>();
            __t51
        },
    },
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (0 as i32) * (32 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(changes),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"changes\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t52: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t52.pHash = std::ptr::null_mut::<FuncDef>();
            __t52
        },
    },
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (0 as i32) * (32 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(total_changes),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"total_changes\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t53: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t53.pHash = std::ptr::null_mut::<FuncDef>();
            __t53
        },
    },
    FuncDef {
        nArg: (3 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(replaceFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"replace\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t54: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t54.pHash = std::ptr::null_mut::<FuncDef>();
            __t54
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(zeroblobFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"zeroblob\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t55: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t55.pHash = std::ptr::null_mut::<FuncDef>();
            __t55
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(substrFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"substr\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t56: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t56.pHash = std::ptr::null_mut::<FuncDef>();
            __t56
        },
    },
    FuncDef {
        nArg: (3 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(substrFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"substr\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t57: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t57.pHash = std::ptr::null_mut::<FuncDef>();
            __t57
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(substrFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"substring\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t58: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t58.pHash = std::ptr::null_mut::<FuncDef>();
            __t58
        },
    },
    FuncDef {
        nArg: (3 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(substrFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"substring\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t59: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t59.pHash = std::ptr::null_mut::<FuncDef>();
            __t59
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (0 as i32) * (32 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(sumStep),
        xFinalize: Some(sumFinalize),
        xValue: Some(sumFinalize),
        xInverse: Some(sumInverse),
        zName: (b"sum\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t60: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t60.pHash = std::ptr::null_mut::<FuncDef>();
            __t60
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (0 as i32) * (32 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(sumStep),
        xFinalize: Some(totalFinalize),
        xValue: Some(totalFinalize),
        xInverse: Some(sumInverse),
        zName: (b"total\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t61: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t61.pHash = std::ptr::null_mut::<FuncDef>();
            __t61
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (0 as i32) * (32 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(sumStep),
        xFinalize: Some(avgFinalize),
        xValue: Some(avgFinalize),
        xInverse: Some(sumInverse),
        zName: (b"avg\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t62: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t62.pHash = std::ptr::null_mut::<FuncDef>();
            __t62
        },
    },
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (0 as i32) * (32 as i32)
            | (256 as i32)
            | (134217728 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(countStep),
        xFinalize: Some(countFinalize),
        xValue: Some(countFinalize),
        xInverse: Some(countInverse),
        zName: (b"count\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t63: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t63.pHash = std::ptr::null_mut::<FuncDef>();
            __t63
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (0 as i32) * (32 as i32) | (134217728 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(countStep),
        xFinalize: Some(countFinalize),
        xValue: Some(countFinalize),
        xInverse: Some(countInverse),
        zName: (b"count\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t64: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t64.pHash = std::ptr::null_mut::<FuncDef>();
            __t64
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (0 as i32) * (32 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(groupConcatStep),
        xFinalize: Some(groupConcatFinalize),
        xValue: Some(groupConcatValue),
        xInverse: Some(groupConcatInverse),
        zName: (b"group_concat\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t65: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t65.pHash = std::ptr::null_mut::<FuncDef>();
            __t65
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (0 as i32) * (32 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(groupConcatStep),
        xFinalize: Some(groupConcatFinalize),
        xValue: Some(groupConcatValue),
        xInverse: Some(groupConcatInverse),
        zName: (b"group_concat\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t66: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t66.pHash = std::ptr::null_mut::<FuncDef>();
            __t66
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (0 as i32) * (32 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(groupConcatStep),
        xFinalize: Some(groupConcatFinalize),
        xValue: Some(groupConcatValue),
        xInverse: Some(groupConcatInverse),
        zName: (b"string_agg\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t67: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t67.pHash = std::ptr::null_mut::<FuncDef>();
            __t67
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (0 as i32) * (32 as i32)
            | (2097152 as i32)
            | (33554432 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(percentStep),
        xFinalize: Some(percentFinal),
        xValue: Some(percentValue),
        xInverse: Some(percentInverse),
        zName: (b"median\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t68: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t68.pHash = std::ptr::null_mut::<FuncDef>();
            __t68
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (0 as i32) * (32 as i32)
            | (2097152 as i32)
            | (33554432 as i32)) as u32,
        pUserData: ((2 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(percentStep),
        xFinalize: Some(percentFinal),
        xValue: Some(percentValue),
        xInverse: Some(percentInverse),
        zName: (b"percentile\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t69: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t69.pHash = std::ptr::null_mut::<FuncDef>();
            __t69
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (0 as i32) * (32 as i32)
            | (2097152 as i32)
            | (33554432 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(percentStep),
        xFinalize: Some(percentFinal),
        xValue: Some(percentValue),
        xInverse: Some(percentInverse),
        zName: (b"percentile_cont\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t70: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t70.pHash = std::ptr::null_mut::<FuncDef>();
            __t70
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (0 as i32) * (32 as i32)
            | (2097152 as i32)
            | (33554432 as i32)) as u32,
        pUserData: ((1 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(percentStep),
        xFinalize: Some(percentFinal),
        xValue: Some(percentValue),
        xInverse: Some(percentInverse),
        zName: (b"percentile_disc\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t71: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t71.pHash = std::ptr::null_mut::<FuncDef>();
            __t71
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (4 as i32) | (8 as i32)) as u32,
        pUserData: (unsafe { std::ptr::addr_of!(globInfo) }) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(likeFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"glob\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t72: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t72.pHash = std::ptr::null_mut::<FuncDef>();
            __t72
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (4 as i32)) as u32,
        pUserData: (unsafe { std::ptr::addr_of!(likeInfoNorm) }) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(likeFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"like\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t73: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t73.pHash = std::ptr::null_mut::<FuncDef>();
            __t73
        },
    },
    FuncDef {
        nArg: (3 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (4 as i32)) as u32,
        pUserData: (unsafe { std::ptr::addr_of!(likeInfoNorm) }) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(likeFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"like\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t74: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t74.pHash = std::ptr::null_mut::<FuncDef>();
            __t74
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(unknownFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"unknown\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t75: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t75.pHash = std::ptr::null_mut::<FuncDef>();
            __t75
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(Some(
                xCeil,
            ))
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(ceilingFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"ceil\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t76: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t76.pHash = std::ptr::null_mut::<FuncDef>();
            __t76
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(Some(
                xCeil,
            ))
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(ceilingFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"ceiling\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t77: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t77.pHash = std::ptr::null_mut::<FuncDef>();
            __t77
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(Some(
                xFloor,
            ))
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(ceilingFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"floor\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t78: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t78.pHash = std::ptr::null_mut::<FuncDef>();
            __t78
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        trunc as *const (),
                    )
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(ceilingFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"trunc\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t79: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t79.pHash = std::ptr::null_mut::<FuncDef>();
            __t79
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(logFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"ln\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t80: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t80.pHash = std::ptr::null_mut::<FuncDef>();
            __t80
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: ((1 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(logFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"log\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t81: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t81.pHash = std::ptr::null_mut::<FuncDef>();
            __t81
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: ((1 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(logFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"log10\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t82: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t82.pHash = std::ptr::null_mut::<FuncDef>();
            __t82
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: ((2 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(logFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"log2\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t83: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t83.pHash = std::ptr::null_mut::<FuncDef>();
            __t83
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(logFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"log\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t84: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t84.pHash = std::ptr::null_mut::<FuncDef>();
            __t84
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        exp as *const (),
                    )
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"exp\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t85: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t85.pHash = std::ptr::null_mut::<FuncDef>();
            __t85
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64, f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<
                        *const (),
                        Option<unsafe extern "C-unwind" fn(f64, f64) -> f64>,
                    >(pow as *const ())
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math2Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"pow\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t86: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t86.pHash = std::ptr::null_mut::<FuncDef>();
            __t86
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64, f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<
                        *const (),
                        Option<unsafe extern "C-unwind" fn(f64, f64) -> f64>,
                    >(pow as *const ())
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math2Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"power\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t87: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t87.pHash = std::ptr::null_mut::<FuncDef>();
            __t87
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64, f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<
                        *const (),
                        Option<unsafe extern "C-unwind" fn(f64, f64) -> f64>,
                    >(fmod as *const ())
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math2Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"mod\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t88: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t88.pHash = std::ptr::null_mut::<FuncDef>();
            __t88
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        acos as *const (),
                    )
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"acos\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t89: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t89.pHash = std::ptr::null_mut::<FuncDef>();
            __t89
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        asin as *const (),
                    )
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"asin\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t90: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t90.pHash = std::ptr::null_mut::<FuncDef>();
            __t90
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        atan as *const (),
                    )
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"atan\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t91: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t91.pHash = std::ptr::null_mut::<FuncDef>();
            __t91
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64, f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<
                        *const (),
                        Option<unsafe extern "C-unwind" fn(f64, f64) -> f64>,
                    >(atan2 as *const ())
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math2Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"atan2\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t92: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t92.pHash = std::ptr::null_mut::<FuncDef>();
            __t92
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        cos as *const (),
                    )
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"cos\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t93: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t93.pHash = std::ptr::null_mut::<FuncDef>();
            __t93
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        sin as *const (),
                    )
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sin\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t94: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t94.pHash = std::ptr::null_mut::<FuncDef>();
            __t94
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        tan as *const (),
                    )
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"tan\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t95: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t95.pHash = std::ptr::null_mut::<FuncDef>();
            __t95
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        cosh as *const (),
                    )
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"cosh\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t96: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t96.pHash = std::ptr::null_mut::<FuncDef>();
            __t96
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        sinh as *const (),
                    )
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sinh\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t97: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t97.pHash = std::ptr::null_mut::<FuncDef>();
            __t97
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        tanh as *const (),
                    )
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"tanh\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t98: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t98.pHash = std::ptr::null_mut::<FuncDef>();
            __t98
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        acosh as *const (),
                    )
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"acosh\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t99: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t99.pHash = std::ptr::null_mut::<FuncDef>();
            __t99
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        asinh as *const (),
                    )
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"asinh\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t100: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t100.pHash = std::ptr::null_mut::<FuncDef>();
            __t100
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        atanh as *const (),
                    )
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"atanh\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t101: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t101.pHash = std::ptr::null_mut::<FuncDef>();
            __t101
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(f64) -> f64>>(
                        sqrt as *const (),
                    )
                },
            )
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqrt\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t102: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t102.pHash = std::ptr::null_mut::<FuncDef>();
            __t102
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(Some(
                degToRad,
            ))
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"radians\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t103: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t103.pHash = std::ptr::null_mut::<FuncDef>();
            __t103
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: unsafe {
            std::mem::transmute::<Option<unsafe extern "C-unwind" fn(f64) -> f64>, *mut ()>(Some(
                radToDeg,
            ))
        },
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(math1Func),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"degrees\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t104: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t104.pHash = std::ptr::null_mut::<FuncDef>();
            __t104
        },
    },
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(piFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"pi\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t105: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t105.pHash = std::ptr::null_mut::<FuncDef>();
            __t105
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (2048 as i32) | (1 as i32) | (0 as i32) * (32 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(signFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sign\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t106: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t106.pHash = std::ptr::null_mut::<FuncDef>();
            __t106
        },
    },
    FuncDef {
        nArg: -(4 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (4194304 as i32) | (2048 as i32) | (0 as i32))
            as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(versionFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"coalesce\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t107: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t107.pHash = std::ptr::null_mut::<FuncDef>();
            __t107
        },
    },
    FuncDef {
        nArg: -(4 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (4194304 as i32) | (2048 as i32) | (0 as i32))
            as u32,
        pUserData: ((5 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(versionFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"iif\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t108: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t108.pHash = std::ptr::null_mut::<FuncDef>();
            __t108
        },
    },
    FuncDef {
        nArg: -(4 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (4194304 as i32) | (2048 as i32) | (0 as i32))
            as u32,
        pUserData: ((5 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(versionFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"if\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t109: __SlateRecord171 = unsafe { std::mem::zeroed() };
            __t109.pHash = std::ptr::null_mut::<FuncDef>();
            __t109
        },
    },
]);
