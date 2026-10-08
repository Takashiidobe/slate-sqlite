//! 2015-08-12
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//!
//! SQLite JSON functions.
//!
//! This file began as an extension in ext/misc/json1.c in 2015.  That
//! extension proved so useful that it has now been moved into the core.
//!
//! The original design stored all JSON as pure text, canonical RFC-8259.
//! Support for JSON-5 extensions was added with version 3.42.0 (2023-05-16).
//! All generated JSON text still conforms strictly to RFC-8259, but text
//! with JSON-5 extensions is accepted as input.
//!
//! Beginning with version 3.45.0 (circa 2024-01-01), these routines also
//! accept BLOB values that have JSON encoded using a binary representation
//! called "JSONB".  The name JSONB comes from PostgreSQL, however the on-disk
//! format for SQLite-JSONB is completely different and incompatible with
//! PostgreSQL-JSONB.
//!
//! Decoding and interpreting JSONB is still O(N) where N is the size of
//! the input, the same as text JSON.  However, the constant of proportionality
//! for JSONB is much smaller due to faster parsing.  The size of each
//! element in JSONB is encoded in its header, so there is no need to search
//! for delimiters using persnickety syntax rules.  JSONB seems to be about
//! 3x faster than text JSON as a result.  JSONB is also tends to be slightly
//! smaller than text JSON, by 5% or 10%, but there are corner cases where
//! JSONB can be slightly larger.  So you are not far mistaken to say that
//! a JSONB blob is the same size as the equivalent RFC-8259 text.
//!
//!
//! THE JSONB ENCODING:
//!
//! Every JSON element is encoded in JSONB as a header and a payload.
//! The header is between 1 and 9 bytes in size.  The payload is zero
//! or more bytes.
//!
//! The lower 4 bits of the first byte of the header determines the
//! element type:
//!
//!    0:   NULL
//!    1:   TRUE
//!    2:   FALSE
//!    3:   INT        -- RFC-8259 integer literal
//!    4:   INT5       -- JSON5 integer literal
//!    5:   FLOAT      -- RFC-8259 floating point literal
//!    6:   FLOAT5     -- JSON5 floating point literal
//!    7:   TEXT       -- Text literal acceptable to both SQL and JSON
//!    8:   TEXTJ      -- Text containing RFC-8259 escapes
//!    9:   TEXT5      -- Text containing JSON5 and/or RFC-8259 escapes
//!   10:   TEXTRAW    -- Text containing unescaped syntax characters
//!   11:   ARRAY
//!   12:   OBJECT
//!
//! The other three possible values (13-15) are reserved for future
//! enhancements.
//!
//! The upper 4 bits of the first byte determine the size of the header
//! and sometimes also the size of the payload.  If X is the first byte
//! of the element and if X>>4 is between 0 and 11, then the payload
//! will be that many bytes in size and the header is exactly one byte
//! in size.  Other four values for X>>4 (12-15) indicate that the header
//! is more than one byte in size and that the payload size is determined
//! by the remainder of the header, interpreted as a unsigned big-endian
//! integer.
//!
//!   Value of X>>4         Size integer        Total header size
//!   -------------     --------------------    -----------------
//!        12           1 byte (0-255)                2
//!        13           2 byte (0-65535)              3
//!        14           4 byte (0-4294967295)         5
//!        15           8 byte (0-1.8e19)             9
//!
//! The payload size need not be expressed in its minimal form.  For example,
//! if the payload size is 10, the size can be expressed in any of 5 different
//! ways: (1) (X>>4)==10, (2) (X>>4)==12 following by one 0x0a byte,
//! (3) (X>>4)==13 followed by 0x00 and 0x0a, (4) (X>>4)==14 followed by
//! 0x00 0x00 0x00 0x0a, or (5) (X>>4)==15 followed by 7 bytes of 0x00 and
//! a single byte of 0x0a.  The shorter forms are preferred, of course, but
//! sometimes when generating JSONB, the payload size is not known in advance
//! and it is convenient to reserve sufficient header space to cover the
//! largest possible payload size and then come back later and patch up
//! the size when it becomes known, resulting in a non-minimal encoding.
//!
//! The value (X>>4)==15 is not actually used in the current implementation
//! (as SQLite is currently unable to handle BLOBs larger than about 2GB)
//! but is included in the design to allow for future enhancements.
//!
//! The payload follows the header.  NULL, TRUE, and FALSE have no payload and
//! their payload size must always be zero.  The payload for INT, INT5,
//! FLOAT, FLOAT5, TEXT, TEXTJ, TEXT5, and TEXTROW is text.  Note that the
//! "..." or '...' delimiters are omitted from the various text encodings.
//! The payload for ARRAY and OBJECT is a list of additional elements that
//! are the content for the array or object.  The payload for an OBJECT
//! must be an even number of elements.  The first element of each pair is
//! the label and must be of type TEXT, TEXTJ, TEXT5, or TEXTRAW.
//!
//! A valid JSONB blob consists of a single element, as described above.
//! Usually this will be an ARRAY or OBJECT element which has many more
//! elements as its content.  But the overall blob is just a single element.
//!
//! Input validation for JSONB blobs simply checks that the element type
//! code is between 0 and 12 and that the total size of the element
//! (header plus payload) is the same as the size of the BLOB.  If those
//! checks are true, the BLOB is assumed to be JSONB and processing continues.
//! Errors are only raised if some other miscoding is discovered during
//! processing.
//!
//! Additional information can be found in the doc/jsonb.md file of the
//! canonical SQLite source tree.
unsafe extern "C" {
    static mut sqlite3CtypeMap: [u8; 0];
    fn sqlite3_mprintf(__v1128: *const i8, ...) -> *mut i8;
    fn sqlite3_vsnprintf(
        __v1129: i32,
        __v1130: *mut i8,
        __v1131: *const i8,
        __v1132: core::ffi::VaList<'_>,
    ) -> *mut i8;
    fn sqlite3_free(__v1133: *mut ());
    fn sqlite3_value_blob(__v1134: *mut sqlite3_value) -> *const ();
    fn sqlite3_value_double(__v1135: *mut sqlite3_value) -> f64;
    fn sqlite3_value_int64(__v1136: *mut sqlite3_value) -> i64;
    fn sqlite3_value_text(__v1137: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_bytes(__v1138: *mut sqlite3_value) -> i32;
    fn sqlite3_value_type(__v1139: *mut sqlite3_value) -> i32;
    fn sqlite3_value_subtype(__v1140: *mut sqlite3_value) -> u32;
    fn sqlite3_aggregate_context(__v1141: *mut sqlite3_context, nBytes: i32) -> *mut ();
    fn sqlite3_user_data(__v1143: *mut sqlite3_context) -> *mut ();
    fn sqlite3_context_db_handle(__v1144: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_get_auxdata(__v1145: *mut sqlite3_context, N: i32) -> *mut ();
    fn sqlite3_set_auxdata(
        __v1147: *mut sqlite3_context,
        N: i32,
        __v1149: *mut (),
        __v1150: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_blob(
        __v1151: *mut sqlite3_context,
        __v1152: *const (),
        __v1153: i32,
        __v1154: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_double(__v1155: *mut sqlite3_context, __v1156: f64);
    fn sqlite3_result_error(__v1157: *mut sqlite3_context, __v1158: *const i8, __v1159: i32);
    fn sqlite3_result_error_nomem(__v1160: *mut sqlite3_context);
    fn sqlite3_result_int(__v1161: *mut sqlite3_context, __v1162: i32);
    fn sqlite3_result_int64(__v1163: *mut sqlite3_context, __v1164: i64);
    fn sqlite3_result_null(__v1165: *mut sqlite3_context);
    fn sqlite3_result_text(
        __v1166: *mut sqlite3_context,
        __v1167: *const i8,
        __v1168: i32,
        __v1169: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_text64(
        __v1170: *mut sqlite3_context,
        z: *const i8,
        n: u64,
        __v1173: Option<unsafe extern "C-unwind" fn(*mut ())>,
        encoding: u8,
    );
    fn sqlite3_result_subtype(__v1175: *mut sqlite3_context, __v1176: u32);
    fn sqlite3_declare_vtab(__v1177: *mut sqlite3, zSQL: *const i8) -> i32;
    fn sqlite3_strnicmp(__v1179: *const i8, __v1180: *const i8, __v1181: i32) -> i32;
    fn sqlite3_strglob(zGlob: *const i8, zStr: *const i8) -> i32;
    fn sqlite3_vtab_config(__v1184: *mut sqlite3, op: i32, ...) -> i32;
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memmove(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn memchr(__s: *const (), __c: i32, __n: u64) -> *mut ();
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64) -> i32;
    fn strchr(__s: *const i8, __c: i32) -> *mut i8;
    fn strspn(__s: *const i8, __accept: *const i8) -> u64;
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3StrICmp(__v1209: *const i8, __v1210: *const i8) -> i32;
    fn sqlite3Strlen30(__v1211: *const i8) -> i32;
    fn sqlite3DbMallocZero(__v1212: *mut sqlite3, __v1213: u64) -> *mut ();
    fn sqlite3DbMallocRaw(__v1214: *mut sqlite3, __v1215: u64) -> *mut ();
    fn sqlite3DbStrNDup(__v1216: *mut sqlite3, __v1217: *const i8, __v1218: u64) -> *mut i8;
    fn sqlite3DbRealloc(__v1219: *mut sqlite3, __v1220: *mut (), __v1221: u64) -> *mut ();
    fn sqlite3DbFree(__v1222: *mut sqlite3, __v1223: *mut ());
    fn sqlite3IsNaN(__v1224: f64) -> i32;
    fn sqlite3RowSetClear(__v1225: *mut ());
    fn sqlite3InsertBuiltinFuncs(__v1226: *mut FuncDef, __v1227: i32);
    fn sqlite3AtoF(z: *const i8, __v1231: *mut f64) -> i32;
    fn sqlite3Utf8ReadLimited(__v1232: *const u8, __v1233: i32, __v1234: *mut u32) -> i32;
    fn sqlite3Atoi64(__v1235: *const i8, __v1236: *mut i64, __v1237: i32, __v1238: u8) -> i32;
    fn sqlite3DecOrHexToI64(__v1239: *const i8, __v1240: *mut i64) -> i32;
    fn sqlite3HexToInt(h: i32) -> u8;
    fn sqlite3ValueIsOfClass(
        __v1242: *const sqlite3_value,
        __v1243: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3RCStrRef(__v1244: *mut i8) -> *mut i8;
    fn sqlite3RCStrUnref(__v1245: *mut ());
    fn sqlite3RCStrNew(__v1246: u64) -> *mut i8;
    fn sqlite3RCStrResize(__v1247: *mut i8, __v1248: u64) -> *mut i8;
    fn sqlite3VtabCreateModule(
        __v1249: *mut sqlite3,
        __v1250: *const i8,
        __v1251: *const sqlite3_module,
        __v1252: *mut (),
        __v1253: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> *mut Module;
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
    trace: __SlateRecord159,
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
    u1: __SlateRecord160,
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
    u: __SlateRecord161,
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
    __slate_bits_0: __slate_bits::__SlateBits66U0,
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
    u: __SlateRecord162,
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
    __slate_bits_0: __slate_bits::__SlateBits90U0,
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
    u: __SlateRecord170,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord171,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord172,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord173,
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
    fg: __SlateRecord180,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord181,
    u2: __SlateRecord182,
    u3: __SlateRecord183,
    u4: __SlateRecord184,
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
    u1: __SlateRecord186,
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
    __slate_bits_0: __slate_bits::__SlateBits158U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord159 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord160 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord161 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord162 {
    tab: __SlateRecord163,
    view: __SlateRecord164,
    vtab: __SlateRecord165,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord163 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord164 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord165 {
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
union __SlateRecord170 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord171 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord172 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord173 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord174,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord174 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord176,
    u: __SlateRecord177,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord176 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits176U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    x: __SlateRecord178,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord178 {
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
struct __SlateRecord180 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits180U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord183 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord184 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    cr: __SlateRecord187,
    d: __SlateRecord188,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord187 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord188 {
    pReturning: *mut Returning,
}

// JSONB element types
// "null"
// "true"
// "false"
// integer acceptable to JSON and SQL
// integer in 0x000 notation
// float acceptable to JSON and SQL
// float with JSON5 extensions
// Text compatible with both JSON and SQL
// Text with JSON escapes
// Text with JSON-5 escape
// SQL text that needs escaping for JSON
// An array
// An object
// Human-readable names for the JSONB values.  The index for each
// string must correspond to the JSONB_* integer above.
static mut jsonbType: __SlateAlign16<[*const i8; 17]> = __SlateAlign16([
    (b"null\0".as_ptr() as *mut i8) as *const i8,
    (b"true\0".as_ptr() as *mut i8) as *const i8,
    (b"false\0".as_ptr() as *mut i8) as *const i8,
    (b"integer\0".as_ptr() as *mut i8) as *const i8,
    (b"integer\0".as_ptr() as *mut i8) as *const i8,
    (b"real\0".as_ptr() as *mut i8) as *const i8,
    (b"real\0".as_ptr() as *mut i8) as *const i8,
    (b"text\0".as_ptr() as *mut i8) as *const i8,
    (b"text\0".as_ptr() as *mut i8) as *const i8,
    (b"text\0".as_ptr() as *mut i8) as *const i8,
    (b"text\0".as_ptr() as *mut i8) as *const i8,
    (b"array\0".as_ptr() as *mut i8) as *const i8,
    (b"object\0".as_ptr() as *mut i8) as *const i8,
    (b"\0".as_ptr() as *mut i8) as *const i8,
    (b"\0".as_ptr() as *mut i8) as *const i8,
    (b"\0".as_ptr() as *mut i8) as *const i8,
    (b"\0".as_ptr() as *mut i8) as *const i8,
]);

/// Growing our own isspace() routine this way is twice as fast as
/// the library isspace() function, resulting in a 7% overall performance
/// increase for the text-JSON parser.  (Ubuntu14.10 gcc 4.8.4 x64 with -Os).
/// 0  1  2  3  4  5  6  7   8  9  a  b  c  d  e  f
/// 0
/// 1
/// 2
/// 3
/// 4
/// 5
/// 6
/// 7
/// 8
/// 9
/// a
/// b
/// c
/// d
/// e
/// f
static mut jsonIsSpace: __SlateAlign16<[i8; 256]> = __SlateAlign16([
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (1 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (1 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
]);

/// The set of all space characters recognized by jsonIsspace().
/// Useful as the second argument to strspn().
static mut jsonSpaces: [i8; 5] = [9 as i8, 10 as i8, 13 as i8, 32 as i8, 0 as i8];

/// Characters that are special to JSON.  Control characters,
/// '"' and '\\' and '\''.  Actually, '\'' is not special to
/// canonical JSON, but it is special in JSON-5, so we include
/// it in the set of special characters.
/// 0  1  2  3  4  5  6  7   8  9  a  b  c  d  e  f
/// 0
/// 1
/// 2
/// 3
/// 4
/// 5
/// 6
/// 7
/// 8
/// 9
/// a
/// b
/// c
/// d
/// e
/// f
static mut jsonIsOk: __SlateAlign16<[i8; 256]> = __SlateAlign16([
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (0 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (0 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (0 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
]);

// Magic number used for the JSON parse cache in sqlite3_get_auxdata()
// Cache entry
// Max number of cache entries
// jsonUnescapeOneChar() returns this invalid code point if it encounters
// a syntax error.
/// A cache mapping JSON text into JSONB blobs.
///
/// Each cache entry is a JsonParse object with the following restrictions:
///
///    *   The bReadOnly flag must be set
///
///    *   The aBlob[] array must be owned by the JsonParse object.  In other
///        words, nBlobAlloc must be non-zero.
///
///    *   eEdit and delta must be zero.
///
///    *   zJson must be an RCStr.  In other words bJsonIsRCStr must be true.
#[repr(C)]
#[derive(Clone, Copy)]
struct JsonCache {
    /// Database connection
    db: *mut sqlite3,
    /// Number of active entries in the cache
    nUsed: i32,
    /// One line for each cache entry
    a: [*mut JsonParse; 4],
}

/// An instance of this object represents a JSON string
/// under construction.  Really, this is a generic string accumulator
/// that can be and is used to create strings other than JSON.
///
/// If the generated string is longer than will fit into the zSpace[] buffer,
/// then it will be an RCStr string.  This aids with caching of large
/// JSON strings.
#[repr(C)]
#[derive(Clone, Copy)]
struct JsonString {
    /// Function context - put error messages here
    pCtx: *mut sqlite3_context,
    /// Append JSON content here
    zBuf: *mut i8,
    /// Bytes of storage available in zBuf[]
    nAlloc: u64,
    /// Bytes of zBuf[] currently used
    nUsed: u64,
    /// True if zBuf is static space
    bStatic: u8,
    /// True if an error has been encountered
    eErr: u8,
    /// Initial static space
    zSpace: [i8; 100],
}

// Allowed values for JsonString.eErr
// Out of memory
// Malformed JSONB
// JSON nested too deep
// Error already sent to sqlite3_result
// The "subtype" set for text JSON values passed through using
// sqlite3_result_subtype() and sqlite3_value_subtype().
// Ascii for "J"
// Bit values for the flags passed into various SQL function implementations
// via the sqlite3_user_data() value.
// Result is always JSON
// Result is always SQL
// Allow abbreviated JSON path specs
// json_set(), not json_insert()
// json_array_insert(), not json_insert()
// Use the BLOB output format
/// A parsed JSON value.  Lifecycle:
///
/// 1.  JSON comes in and is parsed into a JSONB value in aBlob.  The
///     original text is stored in zJson.  This step is skipped if the
///     input is JSONB instead of text JSON.
///
/// 2.  The aBlob[] array is searched using the JSON path notation, if needed.
///
/// 3.  Zero or more changes are made to aBlob[] (via json_remove() or
///     json_replace() or json_patch() or similar).
///
/// 4.  New JSON text is generated from the aBlob[] for output.  This step
///     is skipped if the function is one of the jsonb_* functions that
///     returns JSONB instead of text JSON.
#[repr(C)]
#[derive(Clone, Copy)]
struct JsonParse {
    /// JSONB representation of JSON value
    aBlob: *mut u8,
    /// Bytes of aBlob[] actually used
    nBlob: u32,
    /// Bytes allocated to aBlob[].  0 if aBlob is external
    nBlobAlloc: u32,
    /// Json text used for parsing
    zJson: *mut i8,
    /// The database connection to which this object belongs
    db: *mut sqlite3,
    /// Length of the zJson string in bytes
    nJson: i32,
    /// Number of references to this object
    nJPRef: u32,
    /// Error location in zJson[]
    iErr: u32,
    /// Nesting depth
    iDepth: u16,
    /// Number of errors seen
    nErr: u8,
    /// Set to true if out of memory
    oom: u8,
    /// True if zJson is an RCStr
    bJsonIsRCStr: u8,
    /// True if input uses non-standard features like JSON5
    hasNonstd: u8,
    /// Do not modify.
    bReadOnly: u8,
    /// Search and edit information.  See jsonLookupStep()
    /// Edit operation to apply
    eEdit: u8,
    /// Size change due to the edit
    delta: i32,
    /// Number of bytes to insert
    nIns: u32,
    /// Location of label if search landed on an object value
    iLabel: u32,
    /// Content to be inserted
    aIns: *mut u8,
}

// Allowed values for JsonParse.eEdit
// Delete if exists
// Overwrite if exists
// Insert if not exists
// Insert or overwrite
// array_insert()
// Maximum nesting depth of JSON for this implementation.
//
// This limit is needed to avoid a stack overflow in the recursive
// descent parser.  A depth of 1000 is far deeper than any sane JSON
// should go.  Historical note: This limit was 2000 prior to version 3.42.0
// Allowed values for the flgs argument to jsonParseFuncArg();
// Generate a writable JsonParse object
// Return non-NULL even if there is an error
// Utility routines for dealing with JsonCache objects
/// Free a JsonCache object.
fn jsonCacheDelete(mut p: *mut JsonCache) {
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_1289: loop {
        if !(i < unsafe { (*p).nUsed }) {
            break;
        }
        jsonParseFree(unsafe {
            *unsafe { unsafe { (*p).a.as_mut_ptr() as *mut *mut JsonParse }.offset(i as isize) }
        });
        let __v1594: i32 = i;
        let __v1595: i32 = __v1594 + (1 as i32);
        i = __v1595;
    }
    unsafe { sqlite3DbFree(unsafe { (*p).db }, p as *mut ()) };
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
    pub struct __SlateBits66U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits176U0 {
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
    pub struct __SlateBits180U0 {
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
    pub struct __SlateBits90U0 {
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
    pub struct __SlateBits158U0 {
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

#[unsafe(link_section = ".text.slate_distinct.json.jsonCacheDeleteGeneric")]
extern "C-unwind" fn jsonCacheDeleteGeneric(mut p: *mut ()) {
    jsonCacheDelete(p as *mut JsonCache);
}

/// Insert a new entry into the cache.  If the cache is full, expel
/// the least recently used entry.  Return SQLITE_OK on success or a
/// result code otherwise.
///
/// Cache entries are stored in age order, oldest first.
///
/// # Arguments
///
/// * `ctx` - The SQL statement context holding the cache
/// * `pParse` - The parse object to be added to the cache
fn jsonCacheInsert(mut ctx: *mut sqlite3_context, mut pParse: *mut JsonParse) -> i32 {
    let mut p: *mut JsonCache = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    p = (unsafe { sqlite3_get_auxdata(ctx, -(429938 as i32)) }) as *mut JsonCache;
    if p == std::ptr::null_mut::<JsonCache>() {
        let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(ctx) };
        p = (unsafe { sqlite3DbMallocZero(db, 48 as u64) }) as *mut JsonCache;
        if p == std::ptr::null_mut::<JsonCache>() {
            return 7 as i32;
        }
        unsafe {
            (*p).db = db;
        }
        unsafe {
            sqlite3_set_auxdata(
                ctx,
                -(429938 as i32),
                p as *mut (),
                Some(jsonCacheDeleteGeneric),
            )
        };
        p = (unsafe { sqlite3_get_auxdata(ctx, -(429938 as i32)) }) as *mut JsonCache;
        if p == std::ptr::null_mut::<JsonCache>() {
            return 7 as i32;
        }
    }
    if (unsafe { (*p).nUsed }) >= (4 as i32) {
        jsonParseFree(unsafe {
            *unsafe {
                unsafe { (*p).a.as_mut_ptr() as *mut *mut JsonParse }.offset((0 as i32) as isize)
            }
        });
        unsafe {
            memmove(
                (unsafe { (*p).a.as_mut_ptr() as *mut *mut JsonParse }) as *mut (),
                (unsafe {
                    unsafe { (*p).a.as_mut_ptr() as *mut *mut JsonParse }
                        .offset((1 as i32) as isize)
                }) as *const (),
                ((((4 as i32) - (1 as i32)) as i64) as u64).wrapping_mul(8 as u64),
            )
        };
        unsafe {
            (*p).nUsed = (4 as i32) - (1 as i32);
        }
    }
    0 as i32;
    unsafe {
        (*pParse).eEdit = ((0 as i32) as i8) as u8;
    }
    let __v1596: *mut JsonParse = pParse;
    let __v1597: u32 = unsafe { (*__v1596).nJPRef };
    let __v1598: u32 = __v1597.wrapping_add((1 as i32) as u32);
    unsafe {
        (*__v1596).nJPRef = __v1598;
    }
    unsafe {
        (*pParse).bReadOnly = ((1 as i32) as i8) as u8;
    }
    unsafe {
        *unsafe {
            unsafe { (*p).a.as_mut_ptr() as *mut *mut JsonParse }
                .offset((unsafe { (*p).nUsed }) as isize)
        } = pParse;
    }
    let __v1599: *mut JsonCache = p;
    let __v1600: i32 = unsafe { (*__v1599).nUsed };
    let __v1601: i32 = __v1600 + (1 as i32);
    unsafe {
        (*__v1599).nUsed = __v1601;
    }
    return 0 as i32;
}

/// Search for a cached translation the json text supplied by pArg.  Return
/// the JsonParse object if found.  Return NULL if not found.
///
/// When a match if found, the matching entry is moved to become the
/// most-recently used entry if it isn't so already.
///
/// The JsonParse object returned still belongs to the Cache and might
/// be deleted at any moment.  If the caller wants the JsonParse to
/// linger, it needs to increment the nPJRef reference counter.
///
/// # Arguments
///
/// * `ctx` - The SQL statement context holding the cache
/// * `pArg` - Function argument containing SQL text
fn jsonCacheSearch(mut ctx: *mut sqlite3_context, mut pArg: *mut sqlite3_value) -> *mut JsonParse {
    let mut p: *mut JsonCache = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let mut zJson: *const i8 = unsafe { std::mem::zeroed() };
    let mut nJson: i32 = 0 as i32;
    if (unsafe { sqlite3_value_type(pArg) }) != (3 as i32) {
        return std::ptr::null_mut::<JsonParse>();
    }
    zJson = (unsafe { sqlite3_value_text(pArg) }) as *const i8;
    if zJson == std::ptr::null::<i8>() {
        return std::ptr::null_mut::<JsonParse>();
    }
    nJson = unsafe { sqlite3_value_bytes(pArg) };
    p = (unsafe { sqlite3_get_auxdata(ctx, -(429938 as i32)) }) as *mut JsonCache;
    if p == std::ptr::null_mut::<JsonCache>() {
        return std::ptr::null_mut::<JsonParse>();
    }
    i = 0 as i32;
    '__slate_break_1290: loop {
        if !(i < unsafe { (*p).nUsed }) {
            break;
        }
        if (unsafe {
            (*unsafe {
                *unsafe { unsafe { (*p).a.as_mut_ptr() as *mut *mut JsonParse }.offset(i as isize) }
            })
            .zJson
        }) == (zJson as *mut i8)
        {
            break '__slate_break_1290;
        }
        let __v1602: i32 = i;
        let __v1603: i32 = __v1602 + (1 as i32);
        i = __v1603;
    }
    if i >= unsafe { (*p).nUsed } {
        i = 0 as i32;
        '__slate_break_1291: loop {
            if !(i < unsafe { (*p).nUsed }) {
                break;
            }
            if (unsafe {
                (*unsafe {
                    *unsafe {
                        unsafe { (*p).a.as_mut_ptr() as *mut *mut JsonParse }.offset(i as isize)
                    }
                })
                .nJson
            }) != nJson
            {
            } else {
                if (unsafe {
                    memcmp(
                        (unsafe {
                            (*unsafe {
                                *unsafe {
                                    unsafe { (*p).a.as_mut_ptr() as *mut *mut JsonParse }
                                        .offset(i as isize)
                                }
                            })
                            .zJson
                        }) as *const (),
                        zJson as *const (),
                        (nJson as i64) as u64,
                    )
                }) == (0 as i32)
                {
                    break '__slate_break_1291;
                }
            }
            let __v1604: i32 = i;
            let __v1605: i32 = __v1604 + (1 as i32);
            i = __v1605;
        }
    }
    if i < unsafe { (*p).nUsed } {
        if i < (unsafe { (*p).nUsed }) - (1 as i32) {
            // Make the matching entry the most recently used entry
            let mut tmp: *mut JsonParse = unsafe {
                *unsafe { unsafe { (*p).a.as_mut_ptr() as *mut *mut JsonParse }.offset(i as isize) }
            };
            unsafe {
                memmove(
                    (unsafe {
                        unsafe { (*p).a.as_mut_ptr() as *mut *mut JsonParse }.offset(i as isize)
                    }) as *mut (),
                    (unsafe {
                        unsafe { (*p).a.as_mut_ptr() as *mut *mut JsonParse }
                            .offset((i + (1 as i32)) as isize)
                    }) as *const (),
                    ((((unsafe { (*p).nUsed }) - i - (1 as i32)) as i64) as u64)
                        .wrapping_mul(8 as u64),
                )
            };
            unsafe {
                *unsafe {
                    unsafe { (*p).a.as_mut_ptr() as *mut *mut JsonParse }
                        .offset(((unsafe { (*p).nUsed }) - (1 as i32)) as isize)
                } = tmp;
            }
            i = (unsafe { (*p).nUsed }) - (1 as i32);
        }
        0 as i32;
        return unsafe {
            *unsafe { unsafe { (*p).a.as_mut_ptr() as *mut *mut JsonParse }.offset(i as isize) }
        };
    } else {
        return std::ptr::null_mut::<JsonParse>();
    }
    return unsafe { std::mem::zeroed() };
}

// Utility routines for dealing with JsonString objects
/// Turn uninitialized bulk memory into a valid JsonString object
/// holding a zero-length string.
fn jsonStringZero(mut p: *mut JsonString) {
    unsafe {
        (*p).zBuf = unsafe { (*p).zSpace.as_mut_ptr() as *mut i8 };
    }
    unsafe {
        (*p).nAlloc = 100 as u64;
    }
    unsafe {
        (*p).nUsed = ((0 as i32) as i64) as u64;
    }
    unsafe {
        (*p).bStatic = ((1 as i32) as i8) as u8;
    }
}

/// Initialize the JsonString object
fn jsonStringInit(mut p: *mut JsonString, mut pCtx: *mut sqlite3_context) {
    unsafe {
        (*p).pCtx = pCtx;
    }
    unsafe {
        (*p).eErr = ((0 as i32) as i8) as u8;
    }
    jsonStringZero(p);
}

/// Free all allocated memory and reset the JsonString object back to its
/// initial state.
fn jsonStringReset(mut p: *mut JsonString) {
    if !((unsafe { (*p).bStatic }) != (0 as u8)) {
        unsafe { sqlite3RCStrUnref((unsafe { (*p).zBuf }) as *mut ()) };
    }
    jsonStringZero(p);
}

/// Report an out-of-memory (OOM) condition
fn jsonStringOom(mut p: *mut JsonString) {
    let __v1606: *mut JsonString = p;
    let __v1607: u8 = unsafe { (*__v1606).eErr };
    let __v1608: u8 = ((((__v1607 as u32) as i32) | (1 as i32)) as i8) as u8;
    unsafe {
        (*__v1606).eErr = __v1608;
    }
    if (unsafe { (*p).pCtx }) != std::ptr::null_mut::<sqlite3_context>() {
        unsafe { sqlite3_result_error_nomem(unsafe { (*p).pCtx }) };
    }
    jsonStringReset(p);
}

/// Report JSON nested too deep
fn jsonStringTooDeep(mut p: *mut JsonString) {
    let __v1609: *mut JsonString = p;
    let __v1610: u8 = unsafe { (*__v1609).eErr };
    let __v1611: u8 = ((((__v1610 as u32) as i32) | (4 as i32)) as i8) as u8;
    unsafe {
        (*__v1609).eErr = __v1611;
    }
    0 as i32;
    unsafe {
        sqlite3_result_error(
            unsafe { (*p).pCtx },
            (b"JSON nested too deep\0".as_ptr() as *mut i8) as *const i8,
            -(1 as i32),
        )
    };
    jsonStringReset(p);
}

/// Enlarge pJson->zBuf so that it can hold at least N more bytes.
/// Return zero on success.  Return non-zero on an OOM error
fn jsonStringGrow(mut p: *mut JsonString, mut N: u32) -> i32 {
    let mut nTotal: u64 = if (N as u64) < unsafe { (*p).nAlloc } {
        unsafe { (*p).nAlloc }.wrapping_mul(((2 as i32) as i64) as u64)
    } else {
        unsafe { (*p).nAlloc }
            .wrapping_add(N as u64)
            .wrapping_add(((10 as i32) as i64) as u64)
    };
    let mut zNew: *mut i8 = unsafe { std::mem::zeroed() };
    if (unsafe { (*p).bStatic }) != (0 as u8) {
        if (unsafe { (*p).eErr }) != (0 as u8) {
            return 1 as i32;
        }
        zNew = unsafe { sqlite3RCStrNew(nTotal) };
        if zNew == std::ptr::null_mut::<i8>() {
            jsonStringOom(p);
            return 7 as i32;
        }
        unsafe {
            memcpy(
                zNew as *mut (),
                (unsafe { (*p).zBuf }) as *const (),
                unsafe { (*p).nUsed },
            )
        };
        unsafe {
            (*p).zBuf = zNew;
        }
        unsafe {
            (*p).bStatic = ((0 as i32) as i8) as u8;
        }
    } else {
        unsafe {
            (*p).zBuf = unsafe { sqlite3RCStrResize(unsafe { (*p).zBuf }, nTotal) };
        }
        if (unsafe { (*p).zBuf }) == std::ptr::null_mut::<i8>() {
            let __v1612: *mut JsonString = p;
            let __v1613: u8 = unsafe { (*__v1612).eErr };
            let __v1614: u8 = ((((__v1613 as u32) as i32) | (1 as i32)) as i8) as u8;
            unsafe {
                (*__v1612).eErr = __v1614;
            }
            jsonStringZero(p);
            return 7 as i32;
        }
    }
    unsafe {
        (*p).nAlloc = nTotal;
    }
    return 0 as i32;
}

/// Append N bytes from zIn onto the end of the JsonString string.
fn jsonStringExpandAndAppend(mut p: *mut JsonString, mut zIn: *const i8, mut N: u32) {
    0 as i32;
    if jsonStringGrow(p, N) != (0 as i32) {
        return;
    }
    unsafe {
        memcpy(
            (unsafe { unsafe { (*p).zBuf }.offset((unsafe { (*p).nUsed }) as isize) }) as *mut (),
            zIn as *const (),
            N as u64,
        )
    };
    let __v1615: *mut JsonString = p;
    let __v1616: u64 = unsafe { (*__v1615).nUsed };
    let __v1617: u64 = __v1616.wrapping_add(N as u64);
    unsafe {
        (*__v1615).nUsed = __v1617;
    }
}

fn jsonAppendRaw(mut p: *mut JsonString, mut zIn: *const i8, mut N: u32) {
    if N == ((0 as i32) as u32) {
        return;
    }
    if (N as u64).wrapping_add(unsafe { (*p).nUsed }) >= unsafe { (*p).nAlloc } {
        jsonStringExpandAndAppend(p, zIn, N);
    } else {
        unsafe {
            memcpy(
                (unsafe { unsafe { (*p).zBuf }.offset((unsafe { (*p).nUsed }) as isize) })
                    as *mut (),
                zIn as *const (),
                N as u64,
            )
        };
        let __v1618: *mut JsonString = p;
        let __v1619: u64 = unsafe { (*__v1618).nUsed };
        let __v1620: u64 = __v1619.wrapping_add(N as u64);
        unsafe {
            (*__v1618).nUsed = __v1620;
        }
    }
}

fn jsonAppendRawNZ(mut p: *mut JsonString, mut zIn: *const i8, mut N: u32) {
    0 as i32;
    if (N as u64).wrapping_add(unsafe { (*p).nUsed }) >= unsafe { (*p).nAlloc } {
        jsonStringExpandAndAppend(p, zIn, N);
    } else {
        unsafe {
            memcpy(
                (unsafe { unsafe { (*p).zBuf }.offset((unsafe { (*p).nUsed }) as isize) })
                    as *mut (),
                zIn as *const (),
                N as u64,
            )
        };
        let __v1621: *mut JsonString = p;
        let __v1622: u64 = unsafe { (*__v1621).nUsed };
        let __v1623: u64 = __v1622.wrapping_add(N as u64);
        unsafe {
            (*__v1621).nUsed = __v1623;
        }
    }
}

/// Append formatted text (not to exceed N bytes) to the JsonString.
unsafe extern "C-unwind" fn jsonPrintf(
    mut N: i32,
    mut p: *mut JsonString,
    mut zFormat: *const i8,
    mut __va_args: ...
) {
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    let __v1624: bool;
    if unsafe { (*p).nUsed }.wrapping_add((N as i64) as u64) >= unsafe { (*p).nAlloc } {
        __v1624 = jsonStringGrow(p, N as u32) != (0 as i32);
    } else {
        __v1624 = false as bool;
    }
    if __v1624 {
        return;
    }
    ap = __va_args.clone();
    unsafe {
        sqlite3_vsnprintf(
            N,
            unsafe { unsafe { (*p).zBuf }.offset((unsafe { (*p).nUsed }) as isize) },
            zFormat,
            ap.clone(),
        )
    };
    {}
    let __v1625: *mut JsonString = p;
    let __v1626: u64 = unsafe { (*__v1625).nUsed };
    let __v1627: u64 = __v1626.wrapping_add(
        ((((unsafe {
            strlen(
                (unsafe { unsafe { (*p).zBuf }.offset((unsafe { (*p).nUsed }) as isize) })
                    as *const i8,
            )
        }) as u32) as i32) as i64) as u64,
    );
    unsafe {
        (*__v1625).nUsed = __v1627;
    }
}

/// Append a single character
fn jsonAppendCharExpand(mut p: *mut JsonString, mut c: i8) {
    if jsonStringGrow(p, (1 as i32) as u32) != (0 as i32) {
        return;
    }
    let __v1628: *mut JsonString = p;
    let __v1629: u64 = unsafe { (*__v1628).nUsed };
    let __v1630: u64 = __v1629.wrapping_add(((1 as i32) as i64) as u64);
    unsafe {
        (*__v1628).nUsed = __v1630;
    }
    unsafe {
        *unsafe { unsafe { (*p).zBuf }.offset(__v1629 as isize) } = c;
    }
}

fn jsonAppendChar(mut p: *mut JsonString, mut c: i8) {
    if (unsafe { (*p).nUsed }) >= unsafe { (*p).nAlloc } {
        jsonAppendCharExpand(p, c);
    } else {
        let __v1631: *mut JsonString = p;
        let __v1632: u64 = unsafe { (*__v1631).nUsed };
        let __v1633: u64 = __v1632.wrapping_add(((1 as i32) as i64) as u64);
        unsafe {
            (*__v1631).nUsed = __v1633;
        }
        unsafe {
            *unsafe { unsafe { (*p).zBuf }.offset(__v1632 as isize) } = c;
        }
    }
}

/// Remove a single character from the end of the string
fn jsonStringTrimOneChar(mut p: *mut JsonString) {
    if (((unsafe { (*p).eErr }) as u32) as i32) == (0 as i32) {
        0 as i32;
        let __v1634: *mut JsonString = p;
        let __v1635: u64 = unsafe { (*__v1634).nUsed };
        let __v1636: u64 = __v1635.wrapping_sub(((1 as i32) as i64) as u64);
        unsafe {
            (*__v1634).nUsed = __v1636;
        }
    }
}

/// Make sure there is a zero terminator on p->zBuf[]
///
/// Return true on success.  Return false if an OOM prevents this
/// from happening.
fn jsonStringTerminate(mut p: *mut JsonString) -> i32 {
    jsonAppendChar(p, (0 as i32) as i8);
    jsonStringTrimOneChar(p);
    return ((((unsafe { (*p).eErr }) as u32) as i32) == (0 as i32)) as i32;
}

/// Append a comma separator to the output buffer, if the previous
/// character is not '[' or '{'.
fn jsonAppendSeparator(mut p: *mut JsonString) {
    let mut c: i8 = 0 as i8;
    if (unsafe { (*p).nUsed }) == (((0 as i32) as i64) as u64) {
        return;
    }
    c = unsafe {
        *unsafe {
            unsafe { (*p).zBuf }
                .offset(unsafe { (*p).nUsed }.wrapping_sub(((1 as i32) as i64) as u64) as isize)
        }
    };
    if (c as i32) == (91 as i32) || (c as i32) == (123 as i32) {
        return;
    }
    jsonAppendChar(p, (44 as i32) as i8);
}

/// c is a control character.  Append the canonical JSON representation
/// of that control character to p.
///
/// This routine assumes that the output buffer has already been enlarged
/// sufficiently to hold the worst-case encoding plus a nul terminator.
fn jsonAppendControlChar(mut p: *mut JsonString, mut c: u8) {
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(aSpecial.0) as *const i8 }
                .offset(((c as u32) as i32) as isize)
        }
    }) != (0 as i8)
    {
        unsafe {
            *unsafe { unsafe { (*p).zBuf }.offset((unsafe { (*p).nUsed }) as isize) } =
                (92 as i32) as i8;
        }
        unsafe {
            *unsafe {
                unsafe { (*p).zBuf }
                    .offset(unsafe { (*p).nUsed }.wrapping_add(((1 as i32) as i64) as u64) as isize)
            } = unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(aSpecial.0) as *const i8 }
                        .offset(((c as u32) as i32) as isize)
                }
            };
        }
        let __v1637: *mut JsonString = p;
        let __v1638: u64 = unsafe { (*__v1637).nUsed };
        let __v1639: u64 = __v1638.wrapping_add(((2 as i32) as i64) as u64);
        unsafe {
            (*__v1637).nUsed = __v1639;
        }
    } else {
        unsafe {
            *unsafe { unsafe { (*p).zBuf }.offset((unsafe { (*p).nUsed }) as isize) } =
                (92 as i32) as i8;
        }
        unsafe {
            *unsafe {
                unsafe { (*p).zBuf }
                    .offset(unsafe { (*p).nUsed }.wrapping_add(((1 as i32) as i64) as u64) as isize)
            } = (117 as i32) as i8;
        }
        unsafe {
            *unsafe {
                unsafe { (*p).zBuf }
                    .offset(unsafe { (*p).nUsed }.wrapping_add(((2 as i32) as i64) as u64) as isize)
            } = (48 as i32) as i8;
        }
        unsafe {
            *unsafe {
                unsafe { (*p).zBuf }
                    .offset(unsafe { (*p).nUsed }.wrapping_add(((3 as i32) as i64) as u64) as isize)
            } = (48 as i32) as i8;
        }
        unsafe {
            *unsafe {
                unsafe { (*p).zBuf }
                    .offset(unsafe { (*p).nUsed }.wrapping_add(((4 as i32) as i64) as u64) as isize)
            } = unsafe {
                *unsafe {
                    (b"0123456789abcdef\0".as_ptr() as *mut i8)
                        .offset((((c as u32) as i32) >> (4 as i32)) as isize)
                }
            };
        }
        unsafe {
            *unsafe {
                unsafe { (*p).zBuf }
                    .offset(unsafe { (*p).nUsed }.wrapping_add(((5 as i32) as i64) as u64) as isize)
            } = unsafe {
                *unsafe {
                    (b"0123456789abcdef\0".as_ptr() as *mut i8)
                        .offset((((c as u32) as i32) & (15 as i32)) as isize)
                }
            };
        }
        let __v1640: *mut JsonString = p;
        let __v1641: u64 = unsafe { (*__v1640).nUsed };
        let __v1642: u64 = __v1641.wrapping_add(((6 as i32) as i64) as u64);
        unsafe {
            (*__v1640).nUsed = __v1642;
        }
    }
}

static mut aSpecial: __SlateAlign16<[i8; 32]> = __SlateAlign16([
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (98 as i32) as i8,
    (116 as i32) as i8,
    (110 as i32) as i8,
    (0 as i32) as i8,
    (102 as i32) as i8,
    (114 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
]);

/// Append the N-byte string in zIn to the end of the JsonString string
/// under construction.  Enclose the string in double-quotes ("...") and
/// escape any double-quotes or backslash characters contained within the
/// string.
///
/// This routine is a high-runner.  There is a measurable performance
/// increase associated with unwinding the jsonIsOk[] loop.
fn jsonAppendString(mut p: *mut JsonString, mut zIn: *const i8, mut N: u32) {
    let mut k: u32 = 0 as u32;
    let mut c: u8 = 0 as u8;
    let mut z: *const u8 = zIn as *const u8;
    if z == std::ptr::null::<u8>() {
        return;
    }
    let __v1643: bool;
    if (N as u64)
        .wrapping_add(unsafe { (*p).nUsed })
        .wrapping_add(((2 as i32) as i64) as u64)
        >= unsafe { (*p).nAlloc }
    {
        __v1643 = jsonStringGrow(p, N.wrapping_add((2 as i32) as u32)) != (0 as i32);
    } else {
        __v1643 = false as bool;
    }
    if __v1643 {
        return;
    }
    let __v1644: *mut JsonString = p;
    let __v1645: u64 = unsafe { (*__v1644).nUsed };
    let __v1646: u64 = __v1645.wrapping_add(((1 as i32) as i64) as u64);
    unsafe {
        (*__v1644).nUsed = __v1646;
    }
    unsafe {
        *unsafe { unsafe { (*p).zBuf }.offset(__v1645 as isize) } = (34 as i32) as i8;
    }
    '__slate_break_1295: while (1 as i32) != (0 as i32) {
        // exit-by-break
        k = (0 as i32) as u32;
        // The following while() is the 4-way unwound equivalent of
        //
        // while( k<N && jsonIsOk[z[k]] ){ k++; }
        '__slate_break_1296: while (1 as i32) != (0 as i32) {
            // Exit by break
            if k.wrapping_add((3 as i32) as u32) >= N {
                '__slate_break_1297: while k < N
                    && (unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(jsonIsOk.0) as *const i8 }.offset(
                                (((unsafe { *unsafe { z.offset(k as isize) } }) as u32) as i32)
                                    as isize,
                            )
                        }
                    }) != (0 as i8)
                {
                    let __v1647: u32 = k;
                    let __v1648: u32 = __v1647.wrapping_add((1 as i32) as u32);
                    k = __v1648;
                }
                break '__slate_break_1296;
            }
            if !((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(jsonIsOk.0) as *const i8 }.offset(
                        (((unsafe { *unsafe { z.offset(k as isize) } }) as u32) as i32) as isize,
                    )
                }
            }) != (0 as i8))
            {
                break '__slate_break_1296;
            }
            if !((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(jsonIsOk.0) as *const i8 }.offset(
                        (((unsafe {
                            *unsafe { z.offset(k.wrapping_add((1 as i32) as u32) as isize) }
                        }) as u32) as i32) as isize,
                    )
                }
            }) != (0 as i8))
            {
                let __v1649: u32 = k;
                let __v1650: u32 = __v1649.wrapping_add((1 as i32) as u32);
                k = __v1650;
                break '__slate_break_1296;
            }
            if !((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(jsonIsOk.0) as *const i8 }.offset(
                        (((unsafe {
                            *unsafe { z.offset(k.wrapping_add((2 as i32) as u32) as isize) }
                        }) as u32) as i32) as isize,
                    )
                }
            }) != (0 as i8))
            {
                let __v1651: u32 = k;
                let __v1652: u32 = __v1651.wrapping_add((2 as i32) as u32);
                k = __v1652;
                break '__slate_break_1296;
            }
            if !((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(jsonIsOk.0) as *const i8 }.offset(
                        (((unsafe {
                            *unsafe { z.offset(k.wrapping_add((3 as i32) as u32) as isize) }
                        }) as u32) as i32) as isize,
                    )
                }
            }) != (0 as i8))
            {
                let __v1653: u32 = k;
                let __v1654: u32 = __v1653.wrapping_add((3 as i32) as u32);
                k = __v1654;
                break '__slate_break_1296;
            } else {
                let __v1655: u32 = k;
                let __v1656: u32 = __v1655.wrapping_add((4 as i32) as u32);
                k = __v1656;
            }
        }
        if k >= N {
            if k > ((0 as i32) as u32) {
                unsafe {
                    memcpy(
                        (unsafe { unsafe { (*p).zBuf }.offset((unsafe { (*p).nUsed }) as isize) })
                            as *mut (),
                        z as *const (),
                        k as u64,
                    )
                };
                let __v1657: *mut JsonString = p;
                let __v1658: u64 = unsafe { (*__v1657).nUsed };
                let __v1659: u64 = __v1658.wrapping_add(k as u64);
                unsafe {
                    (*__v1657).nUsed = __v1659;
                }
            }
            break '__slate_break_1295;
        }
        if k > ((0 as i32) as u32) {
            unsafe {
                memcpy(
                    (unsafe { unsafe { (*p).zBuf }.offset((unsafe { (*p).nUsed }) as isize) })
                        as *mut (),
                    z as *const (),
                    k as u64,
                )
            };
            let __v1660: *mut JsonString = p;
            let __v1661: u64 = unsafe { (*__v1660).nUsed };
            let __v1662: u64 = __v1661.wrapping_add(k as u64);
            unsafe {
                (*__v1660).nUsed = __v1662;
            }
            let __v1663: *const u8 = z;
            let __v1664: *const u8 = unsafe { __v1663.offset(k as isize) };
            z = __v1664;
            let __v1665: u32 = N;
            let __v1666: u32 = __v1665.wrapping_sub(k);
            N = __v1666;
        }
        c = unsafe { *unsafe { z.offset((0 as i32) as isize) } };
        if ((c as u32) as i32) == (34 as i32) || ((c as u32) as i32) == (92 as i32) {
            let __v1667: bool;
            if unsafe { (*p).nUsed }
                .wrapping_add(N as u64)
                .wrapping_add(((3 as i32) as i64) as u64)
                > unsafe { (*p).nAlloc }
            {
                __v1667 = jsonStringGrow(p, N.wrapping_add((3 as i32) as u32)) != (0 as i32);
            } else {
                __v1667 = false as bool;
            }
            if __v1667 {
                return;
            }
            let __v1668: *mut JsonString = p;
            let __v1669: u64 = unsafe { (*__v1668).nUsed };
            let __v1670: u64 = __v1669.wrapping_add(((1 as i32) as i64) as u64);
            unsafe {
                (*__v1668).nUsed = __v1670;
            }
            unsafe {
                *unsafe { unsafe { (*p).zBuf }.offset(__v1669 as isize) } = (92 as i32) as i8;
            }
            let __v1671: *mut JsonString = p;
            let __v1672: u64 = unsafe { (*__v1671).nUsed };
            let __v1673: u64 = __v1672.wrapping_add(((1 as i32) as i64) as u64);
            unsafe {
                (*__v1671).nUsed = __v1673;
            }
            unsafe {
                *unsafe { unsafe { (*p).zBuf }.offset(__v1672 as isize) } = c as i8;
            }
        } else {
            if ((c as u32) as i32) == (39 as i32) {
                let __v1674: *mut JsonString = p;
                let __v1675: u64 = unsafe { (*__v1674).nUsed };
                let __v1676: u64 = __v1675.wrapping_add(((1 as i32) as i64) as u64);
                unsafe {
                    (*__v1674).nUsed = __v1676;
                }
                unsafe {
                    *unsafe { unsafe { (*p).zBuf }.offset(__v1675 as isize) } = c as i8;
                }
            } else {
                let __v1677: bool;
                if unsafe { (*p).nUsed }
                    .wrapping_add(N as u64)
                    .wrapping_add(((7 as i32) as i64) as u64)
                    > unsafe { (*p).nAlloc }
                {
                    __v1677 = jsonStringGrow(p, N.wrapping_add((7 as i32) as u32)) != (0 as i32);
                } else {
                    __v1677 = false as bool;
                }
                if __v1677 {
                    return;
                }
                jsonAppendControlChar(p, c);
            }
        }
        let __v1678: *const u8 = z;
        let __v1679: *const u8 = unsafe { __v1678.offset((1 as i32) as isize) };
        z = __v1679;
        let __v1680: u32 = N;
        let __v1681: u32 = __v1680.wrapping_sub((1 as i32) as u32);
        N = __v1681;
    }
    let __v1682: *mut JsonString = p;
    let __v1683: u64 = unsafe { (*__v1682).nUsed };
    let __v1684: u64 = __v1683.wrapping_add(((1 as i32) as i64) as u64);
    unsafe {
        (*__v1682).nUsed = __v1684;
    }
    unsafe {
        *unsafe { unsafe { (*p).zBuf }.offset(__v1683 as isize) } = (34 as i32) as i8;
    }
    0 as i32;
}

/// Append an sqlite3_value (such as a function parameter) to the JSON
/// string under construction in p.
///
/// # Arguments
///
/// * `p` - Append to this JSON string
/// * `pValue` - Value to append
fn jsonAppendSqlValue(mut p: *mut JsonString, mut pValue: *mut sqlite3_value) {
    '__slate_break_1298: {
        match unsafe { sqlite3_value_type(pValue) } {
            5 => {
                jsonAppendRawNZ(
                    p,
                    (b"null\0".as_ptr() as *mut i8) as *const i8,
                    (4 as i32) as u32,
                );
            }
            2 => {
                unsafe {
                    jsonPrintf(
                        100 as i32,
                        p,
                        (b"%!0.17g\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { sqlite3_value_double(pValue) },
                    )
                };
            }
            1 => {
                let mut z: *const i8 = (unsafe { sqlite3_value_text(pValue) }) as *const i8;
                let mut n: u32 = (unsafe { sqlite3_value_bytes(pValue) }) as u32;
                jsonAppendRaw(p, z, n);
            }
            3 => {
                let mut z: *const i8 = (unsafe { sqlite3_value_text(pValue) }) as *const i8;
                let mut n: u32 = (unsafe { sqlite3_value_bytes(pValue) }) as u32;
                if (unsafe { sqlite3_value_subtype(pValue) }) == ((74 as i32) as u32) {
                    jsonAppendRaw(p, z, n);
                } else {
                    jsonAppendString(p, z, n);
                }
            }
            _ => {
                let mut px: JsonParse = unsafe { std::mem::zeroed() };
                unsafe { memset(std::ptr::addr_of_mut!(px) as *mut (), 0 as i32, 72 as u64) };
                if jsonArgIsJsonb(pValue, std::ptr::addr_of_mut!(px)) != (0 as i32) {
                    jsonTranslateBlobToText(std::ptr::addr_of_mut!(px), (0 as i32) as u32, p);
                } else {
                    if (((unsafe { (*p).eErr }) as u32) as i32) == (0 as i32) {
                        unsafe {
                            sqlite3_result_error(
                                unsafe { (*p).pCtx },
                                (b"JSON cannot hold BLOB values\0".as_ptr() as *mut i8)
                                    as *const i8,
                                -(1 as i32),
                            )
                        };
                        unsafe {
                            (*p).eErr = ((8 as i32) as i8) as u8;
                        }
                        jsonStringReset(p);
                    }
                }
            }
        }
    }
}

/// Make the text in p (which is probably a generated JSON text string)
/// the result of the SQL function.
///
/// The JsonString is reset.
///
/// If pParse and ctx are both non-NULL, then the SQL string in p is
/// loaded into the zJson field of the pParse object as a RCStr and the
/// pParse is added to the cache.
///
/// # Arguments
///
/// * `p` - String to return
/// * `pParse` - JSONB source or NULL
/// * `ctx` - Where to cache
fn jsonReturnString(
    mut p: *mut JsonString,
    mut pParse: *mut JsonParse,
    mut ctx: *mut sqlite3_context,
) {
    0 as i32;
    0 as i32;
    jsonStringTerminate(p);
    if (((unsafe { (*p).eErr }) as u32) as i32) == (0 as i32) {
        let mut flags: i32 = ((unsafe { sqlite3_user_data(unsafe { (*p).pCtx }) }) as i64) as i32;
        if flags & (16 as i32) != (0 as i32) {
            jsonReturnStringAsBlob(p);
        } else {
            if (unsafe { (*p).bStatic }) != (0 as u8) {
                unsafe {
                    sqlite3_result_text64(
                        unsafe { (*p).pCtx },
                        (unsafe { (*p).zBuf }) as *const i8,
                        unsafe { (*p).nUsed },
                        unsafe {
                            std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                                -(1 as i32) as usize,
                            )
                        },
                        ((1 as i32) as i8) as u8,
                    )
                };
            } else {
                if pParse != std::ptr::null_mut::<JsonParse>()
                    && (((unsafe { (*pParse).bJsonIsRCStr }) as u32) as i32) == (0 as i32)
                    && (unsafe { (*pParse).nBlobAlloc }) > ((0 as i32) as u32)
                {
                    let mut rc: i32 = 0 as i32;
                    unsafe {
                        (*pParse).zJson = unsafe { sqlite3RCStrRef(unsafe { (*p).zBuf }) };
                    }
                    unsafe {
                        (*pParse).nJson = ((unsafe { (*p).nUsed }) as u32) as i32;
                    }
                    unsafe {
                        (*pParse).bJsonIsRCStr = ((1 as i32) as i8) as u8;
                    }
                    rc = jsonCacheInsert(ctx, pParse);
                    if rc == (7 as i32) {
                        unsafe { sqlite3_result_error_nomem(ctx) };
                        jsonStringReset(p);
                        return;
                    }
                }
                unsafe {
                    sqlite3_result_text64(
                        unsafe { (*p).pCtx },
                        (unsafe { sqlite3RCStrRef(unsafe { (*p).zBuf }) }) as *const i8,
                        unsafe { (*p).nUsed },
                        unsafe {
                            std::mem::transmute::<
                                *const (),
                                Option<unsafe extern "C-unwind" fn(*mut ())>,
                            >(sqlite3RCStrUnref as *const ())
                        },
                        ((1 as i32) as i8) as u8,
                    )
                };
            }
        }
    } else {
        if (((unsafe { (*p).eErr }) as u32) as i32) & (1 as i32) != (0 as i32) {
            unsafe { sqlite3_result_error_nomem(unsafe { (*p).pCtx }) };
        } else {
            if (((unsafe { (*p).eErr }) as u32) as i32) & (4 as i32) != (0 as i32) {
                // error already in p->pCtx
            } else {
                if (((unsafe { (*p).eErr }) as u32) as i32) & (2 as i32) != (0 as i32) {
                    unsafe {
                        sqlite3_result_error(
                            unsafe { (*p).pCtx },
                            (b"malformed JSON\0".as_ptr() as *mut i8) as *const i8,
                            -(1 as i32),
                        )
                    };
                }
            }
        }
    }
    jsonStringReset(p);
}

// Utility routines for dealing with JsonParse objects
/// Reclaim all memory allocated by a JsonParse object.  But do not
/// delete the JsonParse object itself.
fn jsonParseReset(mut pParse: *mut JsonParse) {
    0 as i32;
    if (unsafe { (*pParse).bJsonIsRCStr }) != (0 as u8) {
        unsafe { sqlite3RCStrUnref((unsafe { (*pParse).zJson }) as *mut ()) };
        unsafe {
            (*pParse).zJson = std::ptr::null_mut::<i8>();
        }
        unsafe {
            (*pParse).nJson = 0 as i32;
        }
        unsafe {
            (*pParse).bJsonIsRCStr = ((0 as i32) as i8) as u8;
        }
    }
    if (unsafe { (*pParse).nBlobAlloc }) != (0 as u32) {
        unsafe {
            sqlite3DbFree(
                unsafe { (*pParse).db },
                (unsafe { (*pParse).aBlob }) as *mut (),
            )
        };
        unsafe {
            (*pParse).aBlob = std::ptr::null_mut::<u8>();
        }
        unsafe {
            (*pParse).nBlob = (0 as i32) as u32;
        }
        unsafe {
            (*pParse).nBlobAlloc = (0 as i32) as u32;
        }
    }
}

/// Decrement the reference count on the JsonParse object.  When the
/// count reaches zero, free the object.
fn jsonParseFree(mut pParse: *mut JsonParse) {
    if pParse != std::ptr::null_mut::<JsonParse>() {
        if (unsafe { (*pParse).nJPRef }) > ((1 as i32) as u32) {
            let __v1588: *mut JsonParse = pParse;
            let __v1589: u32 = unsafe { (*__v1588).nJPRef };
            let __v1590: u32 = __v1589.wrapping_sub((1 as i32) as u32);
            unsafe {
                (*__v1588).nJPRef = __v1590;
            }
        } else {
            jsonParseReset(pParse);
            unsafe { sqlite3DbFree(unsafe { (*pParse).db }, pParse as *mut ()) };
        }
    }
}

// Utility routines for the JSON text parser
/// Translate a single byte of Hex into an integer.
/// This routine only gives a correct answer if h really is a valid hexadecimal
/// character:  0..9a..fA..F.  But unlike sqlite3HexToInt(), it does not
/// assert() if the digit is not hex.
fn jsonHexToInt(mut h: i32) -> u8 {
    let __v1685: i32 = h;
    let __v1686: i32 = __v1685 + (9 as i32) * ((1 as i32) & h >> (6 as i32));
    h = __v1686;
    return ((h & (15 as i32)) as i8) as u8;
}

/// Convert a 4-byte hex string into an integer
fn jsonHexToInt4(mut z: *const i8) -> u32 {
    let mut v: u32 = 0 as u32;
    v = ((((jsonHexToInt((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) as u32)
        as i32)
        << (12 as i32))
        + (((jsonHexToInt((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as i32) as u32)
            as i32)
            << (8 as i32))
        + (((jsonHexToInt((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as i32) as u32)
            as i32)
            << (4 as i32))
        + ((jsonHexToInt((unsafe { *unsafe { z.offset((3 as i32) as isize) } }) as i32) as u32)
            as i32)) as u32;
    return v;
}

/// Return true if z[] begins with 2 (or more) hexadecimal digits
fn jsonIs2Hex(mut z: *const i8) -> i32 {
    return ((((unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                ((((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u8) as u32) as i32)
                    as isize,
            )
        }
    }) as u32) as i32)
        & (8 as i32)
        != (0 as i32)
        && (((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                    ((((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u8) as u32) as i32)
                        as isize,
                )
            }
        }) as u32) as i32)
            & (8 as i32)
            != (0 as i32)) as i32;
}

/// Return true if z[] begins with 4 (or more) hexadecimal digits
fn jsonIs4Hex(mut z: *const i8) -> i32 {
    let __v1687: bool;
    if jsonIs2Hex(z) != (0 as i32) {
        __v1687 = jsonIs2Hex(unsafe { z.offset((2 as i32) as isize) }) != (0 as i32);
    } else {
        __v1687 = false as bool;
    }
    return __v1687 as i32;
}

/// Return the number of bytes of JSON5 whitespace at the beginning of
/// the input string z[].
///
/// JSON5 whitespace consists of any of the following characters:
///
///    Unicode  UTF-8         Name
///    U+0009   09            horizontal tab
///    U+000a   0a            line feed
///    U+000b   0b            vertical tab
///    U+000c   0c            form feed
///    U+000d   0d            carriage return
///    U+0020   20            space
///    U+00a0   c2 a0         non-breaking space
///    U+1680   e1 9a 80      ogham space mark
///    U+2000   e2 80 80      en quad
///    U+2001   e2 80 81      em quad
///    U+2002   e2 80 82      en space
///    U+2003   e2 80 83      em space
///    U+2004   e2 80 84      three-per-em space
///    U+2005   e2 80 85      four-per-em space
///    U+2006   e2 80 86      six-per-em space
///    U+2007   e2 80 87      figure space
///    U+2008   e2 80 88      punctuation space
///    U+2009   e2 80 89      thin space
///    U+200a   e2 80 8a      hair space
///    U+2028   e2 80 a8      line separator
///    U+2029   e2 80 a9      paragraph separator
///    U+202f   e2 80 af      narrow no-break space (NNBSP)
///    U+205f   e2 81 9f      medium mathematical space (MMSP)
///    U+3000   e3 80 80      ideographical space
///    U+FEFF   ef bb bf      byte order mark
///
/// In addition, comments between '/', '*' and '*', '/' and
/// from '/', '/' to end-of-line are also considered to be whitespace.
fn json5Whitespace(mut zIn: *const i8) -> i32 {
    let mut __slate_storage_1710: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1710: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1710) as *mut i32;
    let mut __slate_storage_1709: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1709: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1709) as *mut i32;
    let mut __slate_storage_1708: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1708: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1708) as *mut i32;
    let mut __slate_storage_1707: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1707: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1707) as *mut i32;
    let mut __slate_storage_1704: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1704: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1704) as *mut i32;
    let mut __slate_storage_1703: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1703: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1703) as *mut i32;
    let mut __slate_storage_495: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_495: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_495) as *mut u8;
    let mut __slate_storage_1706: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1706: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1706) as *mut i32;
    let mut __slate_storage_1705: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1705: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1705) as *mut i32;
    let mut __slate_storage_1702: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1702: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1702) as *mut i32;
    let mut __slate_storage_1701: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1701: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1701) as *mut i32;
    let mut __slate_storage_1700: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1700: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1700) as *mut i32;
    let mut __slate_storage_1699: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1699: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1699) as *mut i32;
    let mut __slate_storage_1691: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1691: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1691) as *mut i32;
    let mut __slate_storage_1690: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1690: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1690) as *mut i32;
    let mut __slate_storage_492: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_492: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_492) as *mut i32;
    let mut __slate_storage_1698: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1698: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1698) as *mut i32;
    let mut __slate_storage_1697: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1697: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1697) as *mut i32;
    let mut __slate_storage_1692: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1692: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_1692) as *mut i8;
    let mut __slate_storage_1694: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1694: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1694) as *mut i32;
    let mut __slate_storage_1693: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1693: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1693) as *mut i32;
    let mut __slate_storage_1696: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1696: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1696) as *mut i32;
    let mut __slate_storage_1695: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1695: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1695) as *mut i32;
    let mut __slate_storage_494: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_494: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_494) as *mut i8;
    let mut __slate_storage_493: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_493: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_493) as *mut i32;
    let mut __slate_storage_1689: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1689: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1689) as *mut i32;
    let mut __slate_storage_1688: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1688: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1688) as *mut i32;
    let mut __slate_storage_491: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_491: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_491) as *mut *const u8;
    let mut __slate_storage_490: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_490: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_490) as *mut i32;
    unsafe {
        std::ptr::write(__slate_slot_490, 0 as i32);
        std::ptr::write(__slate_slot_491, (zIn as *mut u8) as *const u8);
        '__loop_1: loop {
            if (1 as i32) != (0 as i32) {
                // exit by "goto whitespace_done"
                let __t0: i32 = ((unsafe {
                    *unsafe { (*__slate_slot_491).offset(*__slate_slot_490 as isize) }
                }) as u32) as i32;
                if __t0 == (9 as i32) {
                } else {
                    if __t0 == (10 as i32) {
                    } else {
                        if __t0 == (11 as i32) {
                        } else {
                            if __t0 == (12 as i32) {
                            } else {
                                if __t0 == (13 as i32) {
                                } else {
                                    if __t0 == (32 as i32) {
                                    } else {
                                        if __t0 == (47 as i32) {
                                            if (((unsafe {
                                                *unsafe {
                                                    (*__slate_slot_491).offset(
                                                        (*__slate_slot_490 + (1 as i32)) as isize,
                                                    )
                                                }
                                            })
                                                as u32)
                                                as i32)
                                                == (42 as i32)
                                                && (((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_491).offset(
                                                            (*__slate_slot_490 + (2 as i32))
                                                                as isize,
                                                        )
                                                    }
                                                })
                                                    as u32)
                                                    as i32)
                                                    != (0 as i32)
                                            {
                                                *__slate_slot_492 = *__slate_slot_490 + (3 as i32);
                                                loop {
                                                    if (((unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_491)
                                                                .offset(*__slate_slot_492 as isize)
                                                        }
                                                    })
                                                        as u32)
                                                        as i32)
                                                        != (47 as i32)
                                                        || (((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_491).offset(
                                                                    (*__slate_slot_492 - (1 as i32))
                                                                        as isize,
                                                                )
                                                            }
                                                        })
                                                            as u32)
                                                            as i32)
                                                            != (42 as i32)
                                                    {
                                                        if (((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_491).offset(
                                                                    *__slate_slot_492 as isize,
                                                                )
                                                            }
                                                        })
                                                            as u32)
                                                            as i32)
                                                            == (0 as i32)
                                                        {
                                                            break '__loop_1;
                                                        } else {
                                                            std::ptr::write(
                                                                __slate_slot_1690,
                                                                *__slate_slot_492,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1691,
                                                                *__slate_slot_1690 + (1 as i32),
                                                            );
                                                            *__slate_slot_492 = *__slate_slot_1691;
                                                        }
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                *__slate_slot_490 = *__slate_slot_492 + (1 as i32);
                                                continue '__loop_1;
                                            } else {
                                                if (((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_491).offset(
                                                            (*__slate_slot_490 + (1 as i32))
                                                                as isize,
                                                        )
                                                    }
                                                })
                                                    as u32)
                                                    as i32)
                                                    == (47 as i32)
                                                {
                                                    *__slate_slot_493 =
                                                        *__slate_slot_490 + (2 as i32);
                                                    '__join_22: {
                                                        loop {
                                                            std::ptr::write(
                                                                __slate_slot_1692,
                                                                (unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_491).offset(
                                                                            *__slate_slot_493
                                                                                as isize,
                                                                        )
                                                                    }
                                                                })
                                                                    as i8,
                                                            );
                                                            *__slate_slot_494 = *__slate_slot_1692;
                                                            if (*__slate_slot_1692 as i32)
                                                                != (0 as i32)
                                                            {
                                                                if (*__slate_slot_494 as i32)
                                                                    == (10 as i32)
                                                                    || (*__slate_slot_494 as i32)
                                                                        == (13 as i32)
                                                                {
                                                                    break '__join_22;
                                                                } else {
                                                                    if (226 as i32)
                                                                        == (((*__slate_slot_494
                                                                            as u8)
                                                                            as u32)
                                                                            as i32)
                                                                        && (128 as i32)
                                                                            == (((unsafe {
                                                                                *unsafe {
                                                                                    (*__slate_slot_491).offset((*__slate_slot_493 + (1 as i32)) as isize)
                                                                                }
                                                                            })
                                                                                as u32)
                                                                                as i32)
                                                                        && ((168 as i32)
                                                                            == (((unsafe {
                                                                                *unsafe {
                                                                                    (*__slate_slot_491).offset((*__slate_slot_493 + (2 as i32)) as isize)
                                                                                }
                                                                            })
                                                                                as u32)
                                                                                as i32)
                                                                            || (169 as i32)
                                                                                == (((unsafe {
                                                                                    *unsafe {
                                                                                        (*__slate_slot_491).offset((*__slate_slot_493 + (2 as i32)) as isize)
                                                                                    }
                                                                                })
                                                                                    as u32)
                                                                                    as i32))
                                                                    {
                                                                        break;
                                                                    } else {
                                                                        std::ptr::write(
                                                                            __slate_slot_1693,
                                                                            *__slate_slot_493,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_1694,
                                                                            *__slate_slot_1693
                                                                                + (1 as i32),
                                                                        );
                                                                        *__slate_slot_493 =
                                                                            *__slate_slot_1694;
                                                                    }
                                                                }
                                                            } else {
                                                                break '__join_22;
                                                            }
                                                        }
                                                        std::ptr::write(
                                                            __slate_slot_1695,
                                                            *__slate_slot_493,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1696,
                                                            *__slate_slot_1695 + (2 as i32),
                                                        );
                                                        *__slate_slot_493 = *__slate_slot_1696;
                                                    }
                                                    *__slate_slot_490 = *__slate_slot_493;
                                                    if (unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_491)
                                                                .offset(*__slate_slot_490 as isize)
                                                        }
                                                    }) != (0 as u8)
                                                    {
                                                        std::ptr::write(
                                                            __slate_slot_1697,
                                                            *__slate_slot_490,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1698,
                                                            *__slate_slot_1697 + (1 as i32),
                                                        );
                                                        *__slate_slot_490 = *__slate_slot_1698;
                                                        continue '__loop_1;
                                                    } else {
                                                        continue '__loop_1;
                                                    }
                                                } else {
                                                    break '__loop_1;
                                                }
                                            }
                                        } else {
                                            if __t0 == (194 as i32) {
                                                if (((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_491).offset(
                                                            (*__slate_slot_490 + (1 as i32))
                                                                as isize,
                                                        )
                                                    }
                                                })
                                                    as u32)
                                                    as i32)
                                                    == (160 as i32)
                                                {
                                                    std::ptr::write(
                                                        __slate_slot_1699,
                                                        *__slate_slot_490,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1700,
                                                        *__slate_slot_1699 + (2 as i32),
                                                    );
                                                    *__slate_slot_490 = *__slate_slot_1700;
                                                    continue '__loop_1;
                                                } else {
                                                    break '__loop_1;
                                                }
                                            } else {
                                                if __t0 == (225 as i32) {
                                                    if (((unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_491).offset(
                                                                (*__slate_slot_490 + (1 as i32))
                                                                    as isize,
                                                            )
                                                        }
                                                    })
                                                        as u32)
                                                        as i32)
                                                        == (154 as i32)
                                                        && (((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_491).offset(
                                                                    (*__slate_slot_490 + (2 as i32))
                                                                        as isize,
                                                                )
                                                            }
                                                        })
                                                            as u32)
                                                            as i32)
                                                            == (128 as i32)
                                                    {
                                                        std::ptr::write(
                                                            __slate_slot_1701,
                                                            *__slate_slot_490,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1702,
                                                            *__slate_slot_1701 + (3 as i32),
                                                        );
                                                        *__slate_slot_490 = *__slate_slot_1702;
                                                        continue '__loop_1;
                                                    } else {
                                                        break '__loop_1;
                                                    }
                                                } else {
                                                    if __t0 == (226 as i32) {
                                                        if (((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_491).offset(
                                                                    (*__slate_slot_490 + (1 as i32))
                                                                        as isize,
                                                                )
                                                            }
                                                        })
                                                            as u32)
                                                            as i32)
                                                            == (128 as i32)
                                                        {
                                                            std::ptr::write(
                                                                __slate_slot_495,
                                                                unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_491).offset(
                                                                            (*__slate_slot_490
                                                                                + (2 as i32))
                                                                                as isize,
                                                                        )
                                                                    }
                                                                },
                                                            );
                                                            if ((*__slate_slot_495 as u32) as i32)
                                                                < (128 as i32)
                                                            {
                                                                break '__loop_1;
                                                            } else {
                                                                if ((*__slate_slot_495 as u32)
                                                                    as i32)
                                                                    <= (138 as i32)
                                                                    || ((*__slate_slot_495 as u32)
                                                                        as i32)
                                                                        == (168 as i32)
                                                                    || ((*__slate_slot_495 as u32)
                                                                        as i32)
                                                                        == (169 as i32)
                                                                    || ((*__slate_slot_495 as u32)
                                                                        as i32)
                                                                        == (175 as i32)
                                                                {
                                                                    std::ptr::write(
                                                                        __slate_slot_1703,
                                                                        *__slate_slot_490,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_1704,
                                                                        *__slate_slot_1703
                                                                            + (3 as i32),
                                                                    );
                                                                    *__slate_slot_490 =
                                                                        *__slate_slot_1704;
                                                                    continue '__loop_1;
                                                                } else {
                                                                    break '__loop_1;
                                                                }
                                                            }
                                                        } else {
                                                            if (((unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_491).offset(
                                                                        (*__slate_slot_490
                                                                            + (1 as i32))
                                                                            as isize,
                                                                    )
                                                                }
                                                            })
                                                                as u32)
                                                                as i32)
                                                                == (129 as i32)
                                                                && (((unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_491).offset(
                                                                            (*__slate_slot_490
                                                                                + (2 as i32))
                                                                                as isize,
                                                                        )
                                                                    }
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    == (159 as i32)
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_1705,
                                                                    *__slate_slot_490,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1706,
                                                                    *__slate_slot_1705 + (3 as i32),
                                                                );
                                                                *__slate_slot_490 =
                                                                    *__slate_slot_1706;
                                                                continue '__loop_1;
                                                            } else {
                                                                break '__loop_1;
                                                            }
                                                        }
                                                    } else {
                                                        if __t0 == (227 as i32) {
                                                            if (((unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_491).offset(
                                                                        (*__slate_slot_490
                                                                            + (1 as i32))
                                                                            as isize,
                                                                    )
                                                                }
                                                            })
                                                                as u32)
                                                                as i32)
                                                                == (128 as i32)
                                                                && (((unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_491).offset(
                                                                            (*__slate_slot_490
                                                                                + (2 as i32))
                                                                                as isize,
                                                                        )
                                                                    }
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    == (128 as i32)
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_1707,
                                                                    *__slate_slot_490,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1708,
                                                                    *__slate_slot_1707 + (3 as i32),
                                                                );
                                                                *__slate_slot_490 =
                                                                    *__slate_slot_1708;
                                                                continue '__loop_1;
                                                            } else {
                                                                break '__loop_1;
                                                            }
                                                        } else {
                                                            if __t0 == (239 as i32) {
                                                                if (((unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_491).offset(
                                                                            (*__slate_slot_490
                                                                                + (1 as i32))
                                                                                as isize,
                                                                        )
                                                                    }
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    == (187 as i32)
                                                                    && (((unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_491)
                                                                                .offset(
                                                                                (*__slate_slot_490
                                                                                    + (2 as i32))
                                                                                    as isize,
                                                                            )
                                                                        }
                                                                    })
                                                                        as u32)
                                                                        as i32)
                                                                        == (191 as i32)
                                                                {
                                                                    std::ptr::write(
                                                                        __slate_slot_1709,
                                                                        *__slate_slot_490,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_1710,
                                                                        *__slate_slot_1709
                                                                            + (3 as i32),
                                                                    );
                                                                    *__slate_slot_490 =
                                                                        *__slate_slot_1710;
                                                                    continue '__loop_1;
                                                                } else {
                                                                    break '__loop_1;
                                                                }
                                                            } else {
                                                                break '__loop_1;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                std::ptr::write(__slate_slot_1688, *__slate_slot_490);
                std::ptr::write(__slate_slot_1689, *__slate_slot_1688 + (1 as i32));
                *__slate_slot_490 = *__slate_slot_1689;
            } else {
                break;
            }
        }
        return *__slate_slot_490;
    }
    return unsafe { std::mem::zeroed() };
}

/// Extra floating-point literals to allow in JSON.
#[repr(C)]
#[derive(Clone, Copy)]
struct NanInfName {
    c1: i8,
    c2: i8,
    n: i8,
    eType: i8,
    nRepl: i8,
    zMatch: *mut i8,
    zRepl: *mut i8,
}

static mut aNanInfName: __SlateAlign16<[NanInfName; 5]> = __SlateAlign16([
    NanInfName {
        c1: (105 as i32) as i8,
        c2: (73 as i32) as i8,
        n: (3 as i32) as i8,
        eType: (5 as i32) as i8,
        nRepl: (7 as i32) as i8,
        zMatch: b"inf\0".as_ptr() as *mut i8,
        zRepl: b"9.0e999\0".as_ptr() as *mut i8,
    },
    NanInfName {
        c1: (105 as i32) as i8,
        c2: (73 as i32) as i8,
        n: (8 as i32) as i8,
        eType: (5 as i32) as i8,
        nRepl: (7 as i32) as i8,
        zMatch: b"infinity\0".as_ptr() as *mut i8,
        zRepl: b"9.0e999\0".as_ptr() as *mut i8,
    },
    NanInfName {
        c1: (110 as i32) as i8,
        c2: (78 as i32) as i8,
        n: (3 as i32) as i8,
        eType: (0 as i32) as i8,
        nRepl: (4 as i32) as i8,
        zMatch: b"NaN\0".as_ptr() as *mut i8,
        zRepl: b"null\0".as_ptr() as *mut i8,
    },
    NanInfName {
        c1: (113 as i32) as i8,
        c2: (81 as i32) as i8,
        n: (4 as i32) as i8,
        eType: (0 as i32) as i8,
        nRepl: (4 as i32) as i8,
        zMatch: b"QNaN\0".as_ptr() as *mut i8,
        zRepl: b"null\0".as_ptr() as *mut i8,
    },
    NanInfName {
        c1: (115 as i32) as i8,
        c2: (83 as i32) as i8,
        n: (4 as i32) as i8,
        eType: (0 as i32) as i8,
        nRepl: (4 as i32) as i8,
        zMatch: b"SNaN\0".as_ptr() as *mut i8,
        zRepl: b"null\0".as_ptr() as *mut i8,
    },
]);

/// Report the wrong number of arguments for json_insert(), json_replace()
/// or json_set().
fn jsonWrongNumArgs(mut pCtx: *mut sqlite3_context, mut zFuncName: *const i8) {
    let mut zMsg: *mut i8 = unsafe {
        sqlite3_mprintf(
            (b"json_%s() needs an odd number of arguments\0".as_ptr() as *mut i8) as *const i8,
            zFuncName,
        )
    };
    unsafe { sqlite3_result_error(pCtx, zMsg as *const i8, -(1 as i32)) };
    unsafe { sqlite3_free(zMsg as *mut ()) };
}

// Utility routines for dealing with the binary BLOB representation of JSON
/// Expand pParse->aBlob so that it holds at least N bytes.
///
/// Return the number of errors.
fn jsonBlobExpand(mut pParse: *mut JsonParse, mut N: u64) -> i32 {
    let mut aNew: *mut u8 = unsafe { std::mem::zeroed() };
    let mut t: u64 = 0 as u64;
    0 as i32;
    if (unsafe { (*pParse).nBlobAlloc }) == ((0 as i32) as u32) {
        t = ((100 as i32) as i64) as u64;
    } else {
        t = unsafe { (*pParse).nBlobAlloc }.wrapping_mul((2 as i32) as u32) as u64;
    }
    if t < N {
        t = N.wrapping_add(((100 as i32) as i64) as u64);
    }
    aNew = (unsafe {
        sqlite3DbRealloc(
            unsafe { (*pParse).db },
            (unsafe { (*pParse).aBlob }) as *mut (),
            t,
        )
    }) as *mut u8;
    if aNew == std::ptr::null_mut::<u8>() {
        unsafe {
            (*pParse).oom = ((1 as i32) as i8) as u8;
        }
        return 1 as i32;
    }
    0 as i32;
    unsafe {
        (*pParse).aBlob = aNew;
    }
    unsafe {
        (*pParse).nBlobAlloc = t as u32;
    }
    return 0 as i32;
}

/// If pParse->aBlob is not previously editable (because it is taken
/// from sqlite3_value_blob(), as indicated by the fact that
/// pParse->nBlobAlloc==0 and pParse->nBlob>0) then make it editable
/// by making a copy into space obtained from malloc.
///
/// Return true on success.  Return false on OOM.
fn jsonBlobMakeEditable(mut pParse: *mut JsonParse, mut nExtra: u32) -> i32 {
    let mut aOld: *mut u8 = unsafe { std::mem::zeroed() };
    let mut nSize: u64 = 0 as u64;
    0 as i32;
    if (unsafe { (*pParse).oom }) != (0 as u8) {
        return 0 as i32;
    }
    if (unsafe { (*pParse).nBlobAlloc }) > ((0 as i32) as u32) {
        return 1 as i32;
    }
    aOld = unsafe { (*pParse).aBlob };
    nSize = unsafe { (*pParse).nBlob }.wrapping_add(nExtra) as u64;
    unsafe {
        (*pParse).aBlob = std::ptr::null_mut::<u8>();
    }
    if jsonBlobExpand(pParse, nSize) != (0 as i32) {
        return 0 as i32;
    }
    0 as i32;
    unsafe {
        memcpy(
            (unsafe { (*pParse).aBlob }) as *mut (),
            aOld as *const (),
            (unsafe { (*pParse).nBlob }) as u64,
        )
    };
    return 1 as i32;
}

/// Expand pParse->aBlob and append one bytes.
fn jsonBlobExpandAndAppendOneByte(mut pParse: *mut JsonParse, mut c: u8) {
    jsonBlobExpand(
        pParse,
        ((unsafe { (*pParse).nBlob }) as u64).wrapping_add(((1 as i32) as i64) as u64),
    );
    if (((unsafe { (*pParse).oom }) as u32) as i32) == (0 as i32) {
        0 as i32;
        let __v1711: *mut JsonParse = pParse;
        let __v1712: u32 = unsafe { (*__v1711).nBlob };
        let __v1713: u32 = __v1712.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v1711).nBlob = __v1713;
        }
        unsafe {
            *unsafe { unsafe { (*pParse).aBlob }.offset(__v1712 as isize) } = c;
        }
    }
}

/// Append a single character.
fn jsonBlobAppendOneByte(mut pParse: *mut JsonParse, mut c: u8) {
    if (unsafe { (*pParse).nBlob }) >= unsafe { (*pParse).nBlobAlloc } {
        jsonBlobExpandAndAppendOneByte(pParse, c);
    } else {
        let __v1714: *mut JsonParse = pParse;
        let __v1715: u32 = unsafe { (*__v1714).nBlob };
        let __v1716: u32 = __v1715.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v1714).nBlob = __v1716;
        }
        unsafe {
            *unsafe { unsafe { (*pParse).aBlob }.offset(__v1715 as isize) } = c;
        }
    }
}

fn jsonBlobExpandAndAppendNode(
    mut pParse: *mut JsonParse,
    mut eType: u8,
    mut szPayload: u64,
    mut aPayload: *const (),
) {
    if jsonBlobExpand(
        pParse,
        ((unsafe { (*pParse).nBlob }) as u64)
            .wrapping_add(szPayload)
            .wrapping_add(((9 as i32) as i64) as u64),
    ) != (0 as i32)
    {
        return;
    }
    jsonBlobAppendNode(pParse, eType, szPayload, aPayload);
}

/// Append a node type byte together with the payload size and
/// possibly also the payload.
///
/// If aPayload is not NULL, then it is a pointer to the payload which
/// is also appended.  If aPayload is NULL, the pParse->aBlob[] array
/// is resized (if necessary) so that it is big enough to hold the
/// payload, but the payload is not appended and pParse->nBlob is left
/// pointing to where the first byte of payload will eventually be.
///
/// # Arguments
///
/// * `pParse` - The JsonParse object under construction
/// * `eType` - Node type.  One of JSONB_*
/// * `szPayload` - Number of bytes of payload
/// * `aPayload` - The payload.  Might be NULL
fn jsonBlobAppendNode(
    mut pParse: *mut JsonParse,
    mut eType: u8,
    mut szPayload: u64,
    mut aPayload: *const (),
) {
    let mut a: *mut u8 = unsafe { std::mem::zeroed() };
    if ((unsafe { (*pParse).nBlob }) as u64)
        .wrapping_add(szPayload)
        .wrapping_add(((9 as i32) as i64) as u64)
        > ((unsafe { (*pParse).nBlobAlloc }) as u64)
    {
        jsonBlobExpandAndAppendNode(pParse, eType, szPayload, aPayload);
        return;
    }
    0 as i32;
    a = unsafe { unsafe { (*pParse).aBlob }.offset((unsafe { (*pParse).nBlob }) as isize) };
    if szPayload <= (((11 as i32) as i64) as u64) {
        unsafe {
            *unsafe { a.offset((0 as i32) as isize) } =
                (((((eType as u32) as i32) as i64) as u64) | szPayload << (4 as i32)) as u8;
        }
        let __v1717: *mut JsonParse = pParse;
        let __v1718: u32 = unsafe { (*__v1717).nBlob };
        let __v1719: u32 = __v1718.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v1717).nBlob = __v1719;
        }
    } else {
        if szPayload <= (((255 as i32) as i64) as u64) {
            unsafe {
                *unsafe { a.offset((0 as i32) as isize) } =
                    ((((eType as u32) as i32) | (192 as i32)) as i8) as u8;
            }
            unsafe {
                *unsafe { a.offset((1 as i32) as isize) } =
                    (szPayload & (((255 as i32) as i64) as u64)) as u8;
            }
            let __v1720: *mut JsonParse = pParse;
            let __v1721: u32 = unsafe { (*__v1720).nBlob };
            let __v1722: u32 = __v1721.wrapping_add((2 as i32) as u32);
            unsafe {
                (*__v1720).nBlob = __v1722;
            }
        } else {
            if szPayload <= (((65535 as i32) as i64) as u64) {
                unsafe {
                    *unsafe { a.offset((0 as i32) as isize) } =
                        ((((eType as u32) as i32) | (208 as i32)) as i8) as u8;
                }
                unsafe {
                    *unsafe { a.offset((1 as i32) as isize) } =
                        (szPayload >> (8 as i32) & (((255 as i32) as i64) as u64)) as u8;
                }
                unsafe {
                    *unsafe { a.offset((2 as i32) as isize) } =
                        (szPayload & (((255 as i32) as i64) as u64)) as u8;
                }
                let __v1723: *mut JsonParse = pParse;
                let __v1724: u32 = unsafe { (*__v1723).nBlob };
                let __v1725: u32 = __v1724.wrapping_add((3 as i32) as u32);
                unsafe {
                    (*__v1723).nBlob = __v1725;
                }
            } else {
                unsafe {
                    *unsafe { a.offset((0 as i32) as isize) } =
                        ((((eType as u32) as i32) | (224 as i32)) as i8) as u8;
                }
                unsafe {
                    *unsafe { a.offset((1 as i32) as isize) } =
                        (szPayload >> (24 as i32) & (((255 as i32) as i64) as u64)) as u8;
                }
                unsafe {
                    *unsafe { a.offset((2 as i32) as isize) } =
                        (szPayload >> (16 as i32) & (((255 as i32) as i64) as u64)) as u8;
                }
                unsafe {
                    *unsafe { a.offset((3 as i32) as isize) } =
                        (szPayload >> (8 as i32) & (((255 as i32) as i64) as u64)) as u8;
                }
                unsafe {
                    *unsafe { a.offset((4 as i32) as isize) } =
                        (szPayload & (((255 as i32) as i64) as u64)) as u8;
                }
                let __v1726: *mut JsonParse = pParse;
                let __v1727: u32 = unsafe { (*__v1726).nBlob };
                let __v1728: u32 = __v1727.wrapping_add((5 as i32) as u32);
                unsafe {
                    (*__v1726).nBlob = __v1728;
                }
            }
        }
    }
    if aPayload != std::ptr::null::<()>() {
        let __v1729: *mut JsonParse = pParse;
        let __v1730: u32 = unsafe { (*__v1729).nBlob };
        let __v1731: u32 = (__v1730 as u64).wrapping_add(szPayload) as u32;
        unsafe {
            (*__v1729).nBlob = __v1731;
        }
        unsafe {
            memcpy(
                (unsafe {
                    unsafe { (*pParse).aBlob }.offset(
                        ((unsafe { (*pParse).nBlob }) as u64).wrapping_sub(szPayload) as isize,
                    )
                }) as *mut (),
                aPayload,
                szPayload,
            )
        };
    }
}

/// Change the payload size for the node at index i to be szPayload.
fn jsonBlobChangePayloadSize(mut pParse: *mut JsonParse, mut i: u32, mut szPayload: u32) -> i32 {
    let mut a: *mut u8 = unsafe { std::mem::zeroed() };
    let mut szType: u8 = 0 as u8;
    let mut nExtra: u8 = 0 as u8;
    let mut nNeeded: u8 = 0 as u8;
    let mut delta: i32 = 0 as i32;
    if (unsafe { (*pParse).oom }) != (0 as u8) {
        return 0 as i32;
    }
    a = unsafe { unsafe { (*pParse).aBlob }.offset(i as isize) };
    szType = (((((unsafe { *unsafe { a.offset((0 as i32) as isize) } }) as u32) as i32)
        >> (4 as i32)) as i8) as u8;
    if ((szType as u32) as i32) <= (11 as i32) {
        nExtra = ((0 as i32) as i8) as u8;
    } else {
        if ((szType as u32) as i32) == (12 as i32) {
            nExtra = ((1 as i32) as i8) as u8;
        } else {
            if ((szType as u32) as i32) == (13 as i32) {
                nExtra = ((2 as i32) as i8) as u8;
            } else {
                if ((szType as u32) as i32) == (14 as i32) {
                    nExtra = ((4 as i32) as i8) as u8;
                } else {
                    nExtra = ((8 as i32) as i8) as u8;
                }
            }
        }
    }
    if szPayload <= ((11 as i32) as u32) {
        nNeeded = ((0 as i32) as i8) as u8;
    } else {
        if szPayload <= ((255 as i32) as u32) {
            nNeeded = ((1 as i32) as i8) as u8;
        } else {
            if szPayload <= ((65535 as i32) as u32) {
                nNeeded = ((2 as i32) as i8) as u8;
            } else {
                nNeeded = ((4 as i32) as i8) as u8;
            }
        }
    }
    delta = ((nNeeded as u32) as i32) - ((nExtra as u32) as i32);
    if delta != (0 as i32) {
        let mut newSize: u64 =
            ((unsafe { (*pParse).nBlob }) as u64).wrapping_add((delta as i64) as u64);
        if delta > (0 as i32) {
            let __v1732: bool;
            if newSize > ((unsafe { (*pParse).nBlobAlloc }) as u64) {
                __v1732 = jsonBlobExpand(pParse, newSize) != (0 as i32);
            } else {
                __v1732 = false as bool;
            }
            if __v1732 {
                return 0 as i32; // OOM error.  Error state recorded in pParse->oom.
            }
            a = unsafe { unsafe { (*pParse).aBlob }.offset(i as isize) };
            unsafe {
                memmove(
                    (unsafe { a.offset(((1 as i32) + delta) as isize) }) as *mut (),
                    (unsafe { a.offset((1 as i32) as isize) }) as *const (),
                    unsafe { (*pParse).nBlob }.wrapping_sub(i.wrapping_add((1 as i32) as u32))
                        as u64,
                )
            };
        } else {
            unsafe {
                memmove(
                    (unsafe { a.offset((1 as i32) as isize) }) as *mut (),
                    (unsafe { a.offset(((1 as i32) - delta) as isize) }) as *const (),
                    unsafe { (*pParse).nBlob }
                        .wrapping_sub(i.wrapping_add((1 as i32) as u32).wrapping_sub(delta as u32))
                        as u64,
                )
            };
        }
        unsafe {
            (*pParse).nBlob = newSize as u32;
        }
    }
    if ((nNeeded as u32) as i32) == (0 as i32) {
        unsafe {
            *unsafe { a.offset((0 as i32) as isize) } =
                ((((((unsafe { *unsafe { a.offset((0 as i32) as isize) } }) as u32) as i32)
                    & (15 as i32)) as u32)
                    | szPayload << (4 as i32)) as u8;
        }
    } else {
        if ((nNeeded as u32) as i32) == (1 as i32) {
            unsafe {
                *unsafe { a.offset((0 as i32) as isize) } =
                    (((((unsafe { *unsafe { a.offset((0 as i32) as isize) } }) as u32) as i32)
                        & (15 as i32)
                        | (192 as i32)) as i8) as u8;
            }
            unsafe {
                *unsafe { a.offset((1 as i32) as isize) } =
                    (szPayload & ((255 as i32) as u32)) as u8;
            }
        } else {
            if ((nNeeded as u32) as i32) == (2 as i32) {
                unsafe {
                    *unsafe { a.offset((0 as i32) as isize) } =
                        (((((unsafe { *unsafe { a.offset((0 as i32) as isize) } }) as u32) as i32)
                            & (15 as i32)
                            | (208 as i32)) as i8) as u8;
                }
                unsafe {
                    *unsafe { a.offset((1 as i32) as isize) } =
                        (szPayload >> (8 as i32) & ((255 as i32) as u32)) as u8;
                }
                unsafe {
                    *unsafe { a.offset((2 as i32) as isize) } =
                        (szPayload & ((255 as i32) as u32)) as u8;
                }
            } else {
                unsafe {
                    *unsafe { a.offset((0 as i32) as isize) } =
                        (((((unsafe { *unsafe { a.offset((0 as i32) as isize) } }) as u32) as i32)
                            & (15 as i32)
                            | (224 as i32)) as i8) as u8;
                }
                unsafe {
                    *unsafe { a.offset((1 as i32) as isize) } =
                        (szPayload >> (24 as i32) & ((255 as i32) as u32)) as u8;
                }
                unsafe {
                    *unsafe { a.offset((2 as i32) as isize) } =
                        (szPayload >> (16 as i32) & ((255 as i32) as u32)) as u8;
                }
                unsafe {
                    *unsafe { a.offset((3 as i32) as isize) } =
                        (szPayload >> (8 as i32) & ((255 as i32) as u32)) as u8;
                }
                unsafe {
                    *unsafe { a.offset((4 as i32) as isize) } =
                        (szPayload & ((255 as i32) as u32)) as u8;
                }
            }
        }
    }
    return delta;
}

/// If z[0] is 'u' and is followed by exactly 4 hexadecimal character,
/// then set *pOp to JSONB_TEXTJ and return true.  If not, do not make
/// any changes to *pOp and return false.
fn jsonIs4HexB(mut z: *const i8, mut pOp: *mut i32) -> i32 {
    if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) != (117 as i32) {
        return 0 as i32;
    }
    if !(jsonIs4Hex(unsafe { z.offset((1 as i32) as isize) }) != (0 as i32)) {
        return 0 as i32;
    }
    unsafe {
        *pOp = 8 as i32;
    }
    return 1 as i32;
}

/// Check a single element of the JSONB in pParse for validity.
///
/// The element to be checked starts at offset i and must end at on the
/// last byte before iEnd.
///
/// Return 0 if everything is correct.  Return the 1-based byte offset of the
/// error if a problem is detected.  (In other words, if the error is at offset
/// 0, return 1).
///
/// # Arguments
///
/// * `pParse` - Input JSONB.  Only aBlob and nBlob are used
/// * `i` - Start of element as pParse->aBlob[i]
/// * `iEnd` - One more than the last byte of the element
/// * `iDepth` - Current nesting depth
fn jsonbValidityCheck(
    mut pParse: *const JsonParse,
    mut i: u32,
    mut iEnd: u32,
    mut iDepth: u32,
) -> u32 {
    let mut n: u32 = 0 as u32;
    let mut sz: u32 = 0 as u32;
    let mut j: u32 = 0 as u32;
    let mut k: u32 = 0 as u32;
    let mut z: *const u8 = unsafe { std::mem::zeroed() };
    let mut x: u8 = 0 as u8;
    if iDepth > ((1000 as i32) as u32) {
        return i.wrapping_add((1 as i32) as u32);
    }
    sz = (0 as i32) as u32;
    n = jsonbPayloadSize(pParse, i, std::ptr::addr_of_mut!(sz));
    if n == ((0 as i32) as u32) {
        return i.wrapping_add((1 as i32) as u32);
    }
    // Checked by caller
    if i.wrapping_add(n).wrapping_add(sz) != iEnd {
        return i.wrapping_add((1 as i32) as u32);
    }
    // Checked by caller
    z = (unsafe { (*pParse).aBlob }) as *const u8;
    x = (((((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32) & (15 as i32)) as i8)
        as u8;
    match (x as u32) as i32 {
        0 | 1 | 2 => {
            return if n.wrapping_add(sz) == ((1 as i32) as u32) {
                (0 as i32) as u32
            } else {
                i.wrapping_add((1 as i32) as u32)
            };
        }
        3 => {
            if sz < ((1 as i32) as u32) {
                return i.wrapping_add((1 as i32) as u32);
            }
            j = i.wrapping_add(n);
            if (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32) == (45 as i32) {
                let __v1733: u32 = j;
                let __v1734: u32 = __v1733.wrapping_add((1 as i32) as u32);
                j = __v1734;
                if sz < ((2 as i32) as u32) {
                    return i.wrapping_add((1 as i32) as u32);
                }
            }
            k = i.wrapping_add(n).wrapping_add(sz);
            '__slate_break_1323: while j < k {
                if (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                            (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32)
                                as isize,
                        )
                    }
                }) as u32) as i32)
                    & (4 as i32)
                    != (0 as i32)
                {
                    let __v1735: u32 = j;
                    let __v1736: u32 = __v1735.wrapping_add((1 as i32) as u32);
                    j = __v1736;
                } else {
                    return j.wrapping_add((1 as i32) as u32);
                }
            }
            return (0 as i32) as u32;
        }
        4 => {
            if sz < ((3 as i32) as u32) {
                return i.wrapping_add((1 as i32) as u32);
            }
            j = i.wrapping_add(n);
            if (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32) == (45 as i32) {
                if sz < ((4 as i32) as u32) {
                    return i.wrapping_add((1 as i32) as u32);
                }
                let __v1737: u32 = j;
                let __v1738: u32 = __v1737.wrapping_add((1 as i32) as u32);
                j = __v1738;
            }
            if (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32) != (48 as i32) {
                return i.wrapping_add((1 as i32) as u32);
            }
            if (((unsafe { *unsafe { z.offset(j.wrapping_add((1 as i32) as u32) as isize) } })
                as u32) as i32)
                != (120 as i32)
                && (((unsafe { *unsafe { z.offset(j.wrapping_add((1 as i32) as u32) as isize) } })
                    as u32) as i32)
                    != (88 as i32)
            {
                return j.wrapping_add((2 as i32) as u32);
            }
            let __v1739: u32 = j;
            let __v1740: u32 = __v1739.wrapping_add((2 as i32) as u32);
            j = __v1740;
            k = i.wrapping_add(n).wrapping_add(sz);
            '__slate_break_1324: while j < k {
                if (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                            (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32)
                                as isize,
                        )
                    }
                }) as u32) as i32)
                    & (8 as i32)
                    != (0 as i32)
                {
                    let __v1741: u32 = j;
                    let __v1742: u32 = __v1741.wrapping_add((1 as i32) as u32);
                    j = __v1742;
                } else {
                    return j.wrapping_add((1 as i32) as u32);
                }
            }
            return (0 as i32) as u32;
        }
        5 | 6 => {
            let mut seen: u8 = ((0 as i32) as i8) as u8; // 0: initial.  1: '.' seen  2: 'e' seen
            if sz < ((2 as i32) as u32) {
                return i.wrapping_add((1 as i32) as u32);
            }
            j = i.wrapping_add(n);
            k = j.wrapping_add(sz);
            if (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32) == (45 as i32) {
                let __v1743: u32 = j;
                let __v1744: u32 = __v1743.wrapping_add((1 as i32) as u32);
                j = __v1744;
                if sz < ((3 as i32) as u32) {
                    return i.wrapping_add((1 as i32) as u32);
                }
            }
            if (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32) == (46 as i32) {
                if ((x as u32) as i32) == (5 as i32) {
                    return j.wrapping_add((1 as i32) as u32);
                }
                if !((((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                            (((unsafe {
                                *unsafe { z.offset(j.wrapping_add((1 as i32) as u32) as isize) }
                            }) as u32) as i32) as isize,
                        )
                    }
                }) as u32) as i32)
                    & (4 as i32)
                    != (0 as i32))
                {
                    return j.wrapping_add((1 as i32) as u32);
                }
                let __v1745: u32 = j;
                let __v1746: u32 = __v1745.wrapping_add((2 as i32) as u32);
                j = __v1746;
                seen = ((1 as i32) as i8) as u8;
            } else {
                if (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32) == (48 as i32)
                    && ((x as u32) as i32) == (5 as i32)
                {
                    if j.wrapping_add((3 as i32) as u32) > k {
                        return j.wrapping_add((1 as i32) as u32);
                    }
                    if (((unsafe {
                        *unsafe { z.offset(j.wrapping_add((1 as i32) as u32) as isize) }
                    }) as u32) as i32)
                        != (46 as i32)
                        && (((unsafe {
                            *unsafe { z.offset(j.wrapping_add((1 as i32) as u32) as isize) }
                        }) as u32) as i32)
                            != (101 as i32)
                        && (((unsafe {
                            *unsafe { z.offset(j.wrapping_add((1 as i32) as u32) as isize) }
                        }) as u32) as i32)
                            != (69 as i32)
                    {
                        return j.wrapping_add((1 as i32) as u32);
                    }
                    let __v1747: u32 = j;
                    let __v1748: u32 = __v1747.wrapping_add((1 as i32) as u32);
                    j = __v1748;
                }
            }
            '__slate_break_1325: while j < k {
                if (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                            (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32)
                                as isize,
                        )
                    }
                }) as u32) as i32)
                    & (4 as i32)
                    != (0 as i32)
                {
                } else {
                    if (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32)
                        == (46 as i32)
                    {
                        if ((seen as u32) as i32) > (0 as i32) {
                            return j.wrapping_add((1 as i32) as u32);
                        }
                        if ((x as u32) as i32) == (5 as i32)
                            && (j == k.wrapping_sub((1 as i32) as u32)
                                || !((((unsafe {
                                    *unsafe {
                                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                            .offset(
                                                (((unsafe {
                                                    *unsafe {
                                                        z.offset(j.wrapping_add((1 as i32) as u32)
                                                            as isize)
                                                    }
                                                })
                                                    as u32)
                                                    as i32)
                                                    as isize,
                                            )
                                    }
                                }) as u32) as i32)
                                    & (4 as i32)
                                    != (0 as i32)))
                        {
                            return j.wrapping_add((1 as i32) as u32);
                        }
                        seen = ((1 as i32) as i8) as u8;
                    } else {
                        if (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32)
                            == (101 as i32)
                            || (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32)
                                == (69 as i32)
                        {
                            if ((seen as u32) as i32) == (2 as i32) {
                                return j.wrapping_add((1 as i32) as u32);
                            }
                            if j == k.wrapping_sub((1 as i32) as u32) {
                                return j.wrapping_add((1 as i32) as u32);
                            }
                            if (((unsafe {
                                *unsafe { z.offset(j.wrapping_add((1 as i32) as u32) as isize) }
                            }) as u32) as i32)
                                == (43 as i32)
                                || (((unsafe {
                                    *unsafe { z.offset(j.wrapping_add((1 as i32) as u32) as isize) }
                                }) as u32) as i32)
                                    == (45 as i32)
                            {
                                let __v1751: u32 = j;
                                let __v1752: u32 = __v1751.wrapping_add((1 as i32) as u32);
                                j = __v1752;
                                if j == k.wrapping_sub((1 as i32) as u32) {
                                    return j.wrapping_add((1 as i32) as u32);
                                }
                            }
                            seen = ((2 as i32) as i8) as u8;
                        } else {
                            return j.wrapping_add((1 as i32) as u32);
                        }
                    }
                }
                let __v1749: u32 = j;
                let __v1750: u32 = __v1749.wrapping_add((1 as i32) as u32);
                j = __v1750;
            }
            if ((seen as u32) as i32) == (0 as i32) {
                return i.wrapping_add((1 as i32) as u32);
            }
            return (0 as i32) as u32;
        }
        7 => {
            j = i.wrapping_add(n);
            k = j.wrapping_add(sz);
            '__slate_break_1326: while j < k {
                if !((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(jsonIsOk.0) as *const i8 }.offset(
                            (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32)
                                as isize,
                        )
                    }
                }) != (0 as i8))
                    && (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32)
                        != (39 as i32)
                {
                    return j.wrapping_add((1 as i32) as u32);
                }
                let __v1753: u32 = j;
                let __v1754: u32 = __v1753.wrapping_add((1 as i32) as u32);
                j = __v1754;
            }
            return (0 as i32) as u32;
        }
        8 | 9 => {
            j = i.wrapping_add(n);
            k = j.wrapping_add(sz);
            '__slate_break_1327: while j < k {
                if !((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(jsonIsOk.0) as *const i8 }.offset(
                            (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32)
                                as isize,
                        )
                    }
                }) != (0 as i8))
                    && (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32)
                        != (39 as i32)
                {
                    if (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32)
                        == (34 as i32)
                    {
                        if ((x as u32) as i32) == (8 as i32) {
                            return j.wrapping_add((1 as i32) as u32);
                        }
                    } else {
                        if (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32)
                            <= (31 as i32)
                        {
                            // Control characters in JSON5 string literals are ok
                            if ((x as u32) as i32) == (8 as i32) {
                                return j.wrapping_add((1 as i32) as u32);
                            }
                        } else {
                            if (((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32)
                                != (92 as i32)
                                || j.wrapping_add((1 as i32) as u32) >= k
                            {
                                return j.wrapping_add((1 as i32) as u32);
                            } else {
                                if (unsafe {
                                    strchr(
                                        (b"\"\\/bfnrt\0".as_ptr() as *mut i8) as *const i8,
                                        ((unsafe {
                                            *unsafe {
                                                z.offset(j.wrapping_add((1 as i32) as u32) as isize)
                                            }
                                        }) as u32) as i32,
                                    )
                                }) != std::ptr::null_mut::<i8>()
                                {
                                    let __v1755: u32 = j;
                                    let __v1756: u32 = __v1755.wrapping_add((1 as i32) as u32);
                                    j = __v1756;
                                } else {
                                    if (((unsafe {
                                        *unsafe {
                                            z.offset(j.wrapping_add((1 as i32) as u32) as isize)
                                        }
                                    }) as u32) as i32)
                                        == (117 as i32)
                                    {
                                        if j.wrapping_add((5 as i32) as u32) >= k {
                                            return j.wrapping_add((1 as i32) as u32);
                                        }
                                        if !(jsonIs4Hex(
                                            (unsafe {
                                                z.offset(j.wrapping_add((2 as i32) as u32) as isize)
                                            })
                                                as *const i8,
                                        ) != (0 as i32))
                                        {
                                            return j.wrapping_add((1 as i32) as u32);
                                        }
                                        let __v1757: u32 = j;
                                        let __v1758: u32 = __v1757.wrapping_add((1 as i32) as u32);
                                        j = __v1758;
                                    } else {
                                        if ((x as u32) as i32) != (9 as i32) {
                                            return j.wrapping_add((1 as i32) as u32);
                                        } else {
                                            let mut c: u32 = (0 as i32) as u32;
                                            let mut szC: u32 = jsonUnescapeOneChar(
                                                (unsafe { z.offset(j as isize) }) as *const i8,
                                                k.wrapping_sub(j),
                                                std::ptr::addr_of_mut!(c),
                                            );
                                            if c == ((629145 as i32) as u32) {
                                                return j.wrapping_add((1 as i32) as u32);
                                            }
                                            let __v1759: u32 = j;
                                            let __v1760: u32 = __v1759
                                                .wrapping_add(szC.wrapping_sub((1 as i32) as u32));
                                            j = __v1760;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                let __v1761: u32 = j;
                let __v1762: u32 = __v1761.wrapping_add((1 as i32) as u32);
                j = __v1762;
            }
            return (0 as i32) as u32;
        }
        10 => {
            return (0 as i32) as u32;
        }
        11 => {
            let mut sub: u32 = 0 as u32;
            j = i.wrapping_add(n);
            k = j.wrapping_add(sz);
            '__slate_break_1329: while j < k {
                sz = (0 as i32) as u32;
                n = jsonbPayloadSize(pParse, j, std::ptr::addr_of_mut!(sz));
                if n == ((0 as i32) as u32) {
                    return j.wrapping_add((1 as i32) as u32);
                }
                if j.wrapping_add(n).wrapping_add(sz) > k {
                    return j.wrapping_add((1 as i32) as u32);
                }
                sub = jsonbValidityCheck(
                    pParse,
                    j,
                    j.wrapping_add(n).wrapping_add(sz),
                    iDepth.wrapping_add((1 as i32) as u32),
                );
                if sub != (0 as u32) {
                    return sub;
                }
                let __v1763: u32 = j;
                let __v1764: u32 = __v1763.wrapping_add(n.wrapping_add(sz));
                j = __v1764;
            }
            0 as i32;
            return (0 as i32) as u32;
        }
        12 => {
            let mut cnt: u32 = (0 as i32) as u32;
            let mut sub: u32 = 0 as u32;
            j = i.wrapping_add(n);
            k = j.wrapping_add(sz);
            '__slate_break_1330: while j < k {
                sz = (0 as i32) as u32;
                n = jsonbPayloadSize(pParse, j, std::ptr::addr_of_mut!(sz));
                if n == ((0 as i32) as u32) {
                    return j.wrapping_add((1 as i32) as u32);
                }
                if j.wrapping_add(n).wrapping_add(sz) > k {
                    return j.wrapping_add((1 as i32) as u32);
                }
                if cnt & ((1 as i32) as u32) == ((0 as i32) as u32) {
                    x = (((((unsafe { *unsafe { z.offset(j as isize) } }) as u32) as i32)
                        & (15 as i32)) as i8) as u8;
                    if ((x as u32) as i32) < (7 as i32) || ((x as u32) as i32) > (10 as i32) {
                        return j.wrapping_add((1 as i32) as u32);
                    }
                }
                sub = jsonbValidityCheck(
                    pParse,
                    j,
                    j.wrapping_add(n).wrapping_add(sz),
                    iDepth.wrapping_add((1 as i32) as u32),
                );
                if sub != (0 as u32) {
                    return sub;
                }
                let __v1765: u32 = cnt;
                let __v1766: u32 = __v1765.wrapping_add((1 as i32) as u32);
                cnt = __v1766;
                let __v1767: u32 = j;
                let __v1768: u32 = __v1767.wrapping_add(n.wrapping_add(sz));
                j = __v1768;
            }
            0 as i32;
            if cnt & ((1 as i32) as u32) != ((0 as i32) as u32) {
                return j.wrapping_add((1 as i32) as u32);
            }
            return (0 as i32) as u32;
        }
        _ => {
            return i.wrapping_add((1 as i32) as u32);
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Translate a single element of JSON text at pParse->zJson[i] into
/// its equivalent binary JSONB representation.  Append the translation into
/// pParse->aBlob[] beginning at pParse->nBlob.  The size of
/// pParse->aBlob[] is increased as necessary.
///
/// Return the index of the first character past the end of the element parsed,
/// or one of the following special result codes:
///
///      0    End of input
///     -1    Syntax error or OOM
///     -2    '}' seen   **     -3    ']' seen    \___  For these returns, pParse->iErr is set to
///     -4    ',' seen    /     the index in zJson[] of the seen character
///     -5    ':' seen   /
fn jsonTranslateTextToBlob(mut pParse: *mut JsonParse, mut i: u32) -> i32 {
    let mut __slate_storage_1848: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1848: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1848) as *mut u32;
    let mut __slate_storage_1847: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1847: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1847) as *mut u32;
    let mut __slate_storage_582: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_582: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_582) as *mut i32;
    let mut __slate_storage_581: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_581: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_581) as *mut u32;
    let mut __slate_storage_1846: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1846: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1846) as *mut u32;
    let mut __slate_storage_1845: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1845: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1845) as *mut u32;
    let mut __slate_storage_1844: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1844: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1844) as *mut u32;
    let mut __slate_storage_1843: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1843: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1843) as *mut u32;
    let mut __slate_storage_1842: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1842: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1842) as *mut u32;
    let mut __slate_storage_1841: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1841: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1841) as *mut u32;
    let mut __slate_storage_1840: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1840: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1840) as *mut u8;
    let mut __slate_storage_1839: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1839: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1839) as *mut u8;
    let mut __slate_storage_1830: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1830: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1830) as *mut u32;
    let mut __slate_storage_1829: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1829: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1829) as *mut u32;
    let mut __slate_storage_1838: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1838: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1838) as *mut u32;
    let mut __slate_storage_1837: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1837: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1837) as *mut u32;
    let mut __slate_storage_1836: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1836: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1836) as *mut u8;
    let mut __slate_storage_1835: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1835: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1835) as *mut u8;
    let mut __slate_storage_1834: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1834: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1834) as *mut u8;
    let mut __slate_storage_1833: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1833: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1833) as *mut u8;
    let mut __slate_storage_1832: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1832: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1832) as *mut u8;
    let mut __slate_storage_1831: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1831: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1831) as *mut u8;
    let mut __slate_storage_1821: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1821: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1821) as *mut u32;
    let mut __slate_storage_1820: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1820: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1820) as *mut u32;
    let mut __slate_storage_1828: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1828: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1828) as *mut u32;
    let mut __slate_storage_1827: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1827: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1827) as *mut u32;
    let mut __slate_storage_1826: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1826: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1826) as *mut u8;
    let mut __slate_storage_1825: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1825: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1825) as *mut u8;
    let mut __slate_storage_1824: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1824: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1824) as *mut u8;
    let mut __slate_storage_1823: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1823: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1823) as *mut u8;
    // JSON5 allows for "+Infinity" and "-Infinity" using exactly
    // that case.  SQLite also allows these in any case and it allows
    // "+inf" and "-inf".
    let mut __slate_storage_1822: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1822: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1822) as *mut bool;
    let mut __slate_storage_580: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_580: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_580) as *mut u8;
    let mut __slate_storage_1819: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1819: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1819) as *mut u32;
    let mut __slate_storage_1818: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1818: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1818) as *mut u32;
    let mut __slate_storage_1817: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1817: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1817) as *mut u32;
    let mut __slate_storage_1816: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1816: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1816) as *mut u32;
    let mut __slate_storage_1815: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1815: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1815) as *mut bool;
    let mut __slate_storage_1814: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1814: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1814) as *mut bool;
    let mut __slate_storage_1813: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1813: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1813) as *mut bool;
    let mut __slate_storage_1812: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1812: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1812) as *mut bool;
    let mut __slate_storage_1811: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1811: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1811) as *mut u32;
    let mut __slate_storage_1810: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1810: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1810) as *mut u32;
    let mut __slate_storage_1805: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1805: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1805) as *mut u32;
    let mut __slate_storage_1804: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1804: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1804) as *mut u32;
    let mut __slate_storage_1807: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1807: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1807) as *mut u32;
    let mut __slate_storage_1806: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1806: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1806) as *mut u32;
    let mut __slate_storage_1809: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1809: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1809) as *mut u32;
    let mut __slate_storage_1808: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1808: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1808) as *mut u32;
    let mut __slate_storage_579: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_579: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_579) as *mut i8;
    let mut __slate_storage_578: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_578: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_578) as *mut u8;
    let mut __slate_storage_1803: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1803: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1803) as *mut u16;
    let mut __slate_storage_1802: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1802: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1802) as *mut u16;
    let mut __slate_storage_1801: std::mem::MaybeUninit<*mut JsonParse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1801: *mut *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_1801) as *mut *mut JsonParse;
    let mut __slate_storage_1798: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1798: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1798) as *mut u32;
    let mut __slate_storage_1797: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1797: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1797) as *mut u32;
    let mut __slate_storage_1800: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1800: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1800) as *mut u32;
    let mut __slate_storage_1799: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1799: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1799) as *mut u32;
    let mut __slate_storage_1796: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1796: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1796) as *mut u16;
    let mut __slate_storage_1795: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1795: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1795) as *mut u16;
    let mut __slate_storage_1794: std::mem::MaybeUninit<*mut JsonParse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1794: *mut *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_1794) as *mut *mut JsonParse;
    let mut __slate_storage_1793: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1793: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1793) as *mut u16;
    let mut __slate_storage_1792: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1792: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1792) as *mut u16;
    let mut __slate_storage_1791: std::mem::MaybeUninit<*mut JsonParse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1791: *mut *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_1791) as *mut *mut JsonParse;
    let mut __slate_storage_1773: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1773: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1773) as *mut u32;
    let mut __slate_storage_1772: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1772: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1772) as *mut u32;
    let mut __slate_storage_1790: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1790: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1790) as *mut u32;
    let mut __slate_storage_1789: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1789: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1789) as *mut u32;
    let mut __slate_storage_1784: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1784: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1784) as *mut u32;
    let mut __slate_storage_1783: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1783: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1783) as *mut u32;
    let mut __slate_storage_1788: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1788: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1788) as *mut u32;
    let mut __slate_storage_1787: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1787: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1787) as *mut u32;
    let mut __slate_storage_1786: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1786: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1786) as *mut u32;
    let mut __slate_storage_1785: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1785: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1785) as *mut u32;
    let mut __slate_storage_1780: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1780: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1780) as *mut bool;
    let mut __slate_storage_1779: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1779: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1779) as *mut bool;
    let mut __slate_storage_1778: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1778: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1778) as *mut bool;
    let mut __slate_storage_1782: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1782: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1782) as *mut i32;
    let mut __slate_storage_1781: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1781: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1781) as *mut i32;
    let mut __slate_storage_577: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_577: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_577) as *mut i32;
    let mut __slate_storage_1777: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1777: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1777) as *mut bool;
    let mut __slate_storage_1776: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1776: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1776) as *mut bool;
    let mut __slate_storage_1775: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1775: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1775) as *mut u32;
    let mut __slate_storage_1774: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1774: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1774) as *mut u32;
    let mut __slate_storage_576: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_576: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_576) as *mut i32;
    let mut __slate_storage_575: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_575: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_575) as *mut u32;
    let mut __slate_storage_1771: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1771: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1771) as *mut u16;
    let mut __slate_storage_1770: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1770: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1770) as *mut u16;
    let mut __slate_storage_1769: std::mem::MaybeUninit<*mut JsonParse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1769: *mut *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_1769) as *mut *mut JsonParse;
    let mut __slate_storage_574: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_574: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_574) as *mut *const i8;
    let mut __slate_storage_573: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_573: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_573) as *mut u8;
    let mut __slate_storage_572: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_572: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_572) as *mut i32;
    let mut __slate_storage_571: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_571: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_571) as *mut u32;
    let mut __slate_storage_570: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_570: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_570) as *mut u32;
    let mut __slate_storage_569: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_569: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_569) as *mut u32;
    let mut __slate_storage_568: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_568: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_568) as *mut i8;
    unsafe {
        std::ptr::write(__slate_slot_574, (unsafe { (*pParse).zJson }) as *const i8);
        '__join_214: {
            '__join_152: {
                '__join_126: {
                    '__join_128: {
                        '__join_127: {
                            '__join_85: {
                                '__join_82: {
                                    '__join_25: {
                                        '__join_47: {
                                            '__join_74: {
                                                '__join_79: {
                                                    '__join_78: {
                                                        '__join_75: {
                                                            '__join_10: {
                                                                '__loop_215: loop {
                                                                    '__join_17: {
                                                                        '__join_16: {
                                                                            let __t0: i32 = (((unsafe {
                                                                                *unsafe {
                                                                                    (*__slate_slot_574).offset(i as isize)
                                                                                }
                                                                            })
                                                                                as u8)
                                                                                as u32)
                                                                                as i32;
                                                                            if __t0 == (123 as i32)
                                                                            {
                                                                                break '__join_214;
                                                                            } else {
                                                                                if __t0
                                                                                    == (91 as i32)
                                                                                {
                                                                                    break '__join_152;
                                                                                } else {
                                                                                    if __t0
                                                                                        == (39
                                                                                            as i32)
                                                                                    {
                                                                                        break '__join_128;
                                                                                    } else {
                                                                                        if __t0 == (34 as i32) {
break '__join_127;
} else {
if __t0 == (116 as i32) {
break '__join_85;
} else {
if __t0 == (102 as i32) {
break '__join_82;
} else {
if __t0 == (43 as i32) {
break '__join_79;
} else {
if __t0 == (46 as i32) {
break '__join_78;
} else {
if __t0 == (45 as i32) {
break '__join_75;
} else {
if __t0 == (48 as i32) {
break '__join_75;
} else {
if __t0 == (49 as i32) {
break '__join_75;
} else {
if __t0 == (50 as i32) {
break '__join_75;
} else {
if __t0 == (51 as i32) {
break '__join_75;
} else {
if __t0 == (52 as i32) {
break '__join_75;
} else {
if __t0 == (53 as i32) {
break '__join_75;
} else {
if __t0 == (54 as i32) {
break '__join_75;
} else {
if __t0 == (55 as i32) {
break '__join_75;
} else {
if __t0 == (56 as i32) {
break '__join_75;
} else {
if __t0 == (57 as i32) {
break '__join_75;
} else {
if __t0 == (125 as i32) {
unsafe {
(*pParse).iErr = i;
}
return -(2 as i32);
} else {
if __t0 == (93 as i32) {
unsafe {
(*pParse).iErr = i;
}
return -(3 as i32);
} else {
if __t0 == (44 as i32) {
unsafe {
(*pParse).iErr = i;
}
return -(4 as i32);
} else {
if __t0 == (58 as i32) {
unsafe {
(*pParse).iErr = i;
}
return -(5 as i32);
} else {
if __t0 == (0 as i32) {
return 0 as i32;
} else {
if __t0 == (9 as i32) {
break '__join_17;
} else {
if __t0 == (10 as i32) {
break '__join_17;
} else {
if __t0 == (13 as i32) {
break '__join_17;
} else {
if __t0 == (32 as i32) {
break '__join_17;
} else {
if __t0 == (11 as i32) {
} else {
if __t0 == (12 as i32) {
} else {
if __t0 == (47 as i32) {
} else {
if __t0 == (194 as i32) {
} else {
if __t0 == (225 as i32) {
} else {
if __t0 == (226 as i32) {
} else {
if __t0 == (227 as i32) {
} else {
if __t0 == (239 as i32) {
} else {
if __t0 == (110 as i32) {
break '__loop_215;
} else {
break '__join_10;
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        *__slate_slot_569 =
                                                                            json5Whitespace(
                                                                                unsafe {
                                                                                    (*__slate_slot_574).offset(i as isize)
                                                                                },
                                                                            )
                                                                                as u32;
                                                                        if *__slate_slot_569
                                                                            > ((0 as i32) as u32)
                                                                        {
                                                                            std::ptr::write(
                                                                                __slate_slot_1845,
                                                                                i,
                                                                            );
                                                                            std::ptr::write(__slate_slot_1846, (*__slate_slot_1845).wrapping_add(*__slate_slot_569));
                                                                            i = *__slate_slot_1846;
                                                                            unsafe {
                                                                                (*pParse)
                                                                                    .hasNonstd = ((1
                                                                                    as i32)
                                                                                    as i8)
                                                                                    as u8;
                                                                            }
                                                                            continue '__loop_215;
                                                                        } else {
                                                                            unsafe {
                                                                                (*pParse).iErr = i;
                                                                            }
                                                                            return -(1 as i32);
                                                                        }
                                                                    }
                                                                    std::ptr::write(
                                                                        __slate_slot_1843,
                                                                        i,
                                                                    );
                                                                    std::ptr::write(__slate_slot_1844, (*__slate_slot_1843).wrapping_add(((1 as i32) as u32).wrapping_add((unsafe { strspn(unsafe { (*__slate_slot_574).offset(i.wrapping_add((1 as i32) as u32) as isize) }, unsafe { std::ptr::addr_of!(jsonSpaces) as *const i8 }) }) as u32)));
                                                                    i = *__slate_slot_1844;
                                                                }
                                                                if (unsafe {
                                                                    strncmp(
                                                                        unsafe {
                                                                            (*__slate_slot_574)
                                                                                .offset(i as isize)
                                                                        },
                                                                        (b"null\0".as_ptr()
                                                                            as *mut i8)
                                                                            as *const i8,
                                                                        ((4 as i32) as i64) as u64,
                                                                    )
                                                                }) == (0 as i32)
                                                                    && !((((unsafe {
                                                                        *unsafe {
                                                                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(((((unsafe { *unsafe { (*__slate_slot_574).offset(i.wrapping_add((4 as i32) as u32) as isize) } }) as u8) as u32) as i32) as isize)
                                                                        }
                                                                    })
                                                                        as u32)
                                                                        as i32)
                                                                        & (6 as i32)
                                                                        != (0 as i32))
                                                                {
                                                                    jsonBlobAppendOneByte(
                                                                        pParse,
                                                                        ((0 as i32) as i8) as u8,
                                                                    );
                                                                    return i.wrapping_add(
                                                                        (4 as i32) as u32,
                                                                    )
                                                                        as i32;
                                                                } else {
                                                                    // fall-through into the default case that checks for NaN
                                                                    //
                                                                    // no break
                                                                    {}
                                                                }
                                                            }
                                                            *__slate_slot_568 = unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_574)
                                                                        .offset(i as isize)
                                                                }
                                                            };
                                                            *__slate_slot_581 = (0 as i32) as u32;
                                                            '__join_6: {
                                                                loop {
                                                                    if (*__slate_slot_581 as u64)
                                                                        < (120 as u64) / (24 as u64)
                                                                    {
                                                                        if (*__slate_slot_568
                                                                            as i32)
                                                                            != ((unsafe {
                                                                                (*unsafe { unsafe { std::ptr::addr_of!(aNanInfName.0) as *const NanInfName }.offset(*__slate_slot_581 as isize) }).c1
                                                                            })
                                                                                as i32)
                                                                            && (*__slate_slot_568
                                                                                as i32)
                                                                                != ((unsafe {
                                                                                    (*unsafe { unsafe { std::ptr::addr_of!(aNanInfName.0) as *const NanInfName }.offset(*__slate_slot_581 as isize) }).c2
                                                                                })
                                                                                    as i32)
                                                                        {
                                                                        } else {
                                                                            *__slate_slot_582 = (unsafe {
                                                                                (*unsafe { unsafe { std::ptr::addr_of!(aNanInfName.0) as *const NanInfName }.offset(*__slate_slot_581 as isize) }).n
                                                                            })
                                                                                as i32;
                                                                            if (unsafe {
                                                                                sqlite3_strnicmp(unsafe { (*__slate_slot_574).offset(i as isize) }, (unsafe { (*unsafe { unsafe { std::ptr::addr_of!(aNanInfName.0) as *const NanInfName }.offset(*__slate_slot_581 as isize) }).zMatch }) as *const i8, *__slate_slot_582)
                                                                            }) != (0 as i32)
                                                                            {
                                                                            } else {
                                                                                if (((unsafe {
                                                                                    *unsafe {
                                                                                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(((((unsafe { *unsafe { (*__slate_slot_574).offset(i.wrapping_add(*__slate_slot_582 as u32) as isize) } }) as u8) as u32) as i32) as isize)
                                                                                    }
                                                                                })
                                                                                    as u32)
                                                                                    as i32)
                                                                                    & (6 as i32)
                                                                                    != (0 as i32)
                                                                                {
                                                                                } else {
                                                                                    break '__join_6;
                                                                                }
                                                                            }
                                                                        }
                                                                        std::ptr::write(
                                                                            __slate_slot_1847,
                                                                            *__slate_slot_581,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_1848,
                                                                            (*__slate_slot_1847)
                                                                                .wrapping_add(
                                                                                    (1 as i32)
                                                                                        as u32,
                                                                                ),
                                                                        );
                                                                        *__slate_slot_581 =
                                                                            *__slate_slot_1848;
                                                                    } else {
                                                                        break;
                                                                    }
                                                                }
                                                                unsafe {
                                                                    (*pParse).iErr = i;
                                                                }
                                                                return -(1 as i32);
                                                            }
                                                            if ((unsafe {
                                                                (*unsafe {
                                                                    unsafe {
                                                                        std::ptr::addr_of!(
                                                                            aNanInfName.0
                                                                        )
                                                                            as *const NanInfName
                                                                    }
                                                                    .offset(
                                                                        *__slate_slot_581 as isize,
                                                                    )
                                                                })
                                                                .eType
                                                            })
                                                                as i32)
                                                                == (5 as i32)
                                                            {
                                                                jsonBlobAppendNode(
                                                                    pParse,
                                                                    ((5 as i32) as i8) as u8,
                                                                    ((5 as i32) as i64) as u64,
                                                                    (b"9e999\0".as_ptr() as *mut i8)
                                                                        as *const (),
                                                                );
                                                            } else {
                                                                jsonBlobAppendOneByte(
                                                                    pParse,
                                                                    ((0 as i32) as i8) as u8,
                                                                );
                                                            }
                                                            unsafe {
                                                                (*pParse).hasNonstd =
                                                                    ((1 as i32) as i8) as u8;
                                                            }
                                                            return i.wrapping_add(
                                                                *__slate_slot_582 as u32,
                                                            )
                                                                as i32;
                                                        }
                                                        *__slate_slot_573 =
                                                            ((0 as i32) as i8) as u8; // Parse number
                                                        // Bit 0x01:  JSON5.   Bit 0x02:  FLOAT
                                                        break '__join_74;
                                                    }
                                                    if (((unsafe {
                                                        *unsafe {
                                                            unsafe {
                                                                std::ptr::addr_of!(sqlite3CtypeMap)
                                                                    as *const u8
                                                            }
                                                            .offset(
                                                                ((((unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_574).offset(
                                                                            i.wrapping_add(
                                                                                (1 as i32) as u32,
                                                                            )
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
                                                    })
                                                        as u32)
                                                        as i32)
                                                        & (4 as i32)
                                                        != (0 as i32)
                                                    {
                                                        unsafe {
                                                            (*pParse).hasNonstd =
                                                                ((1 as i32) as i8) as u8;
                                                        }
                                                        *__slate_slot_573 =
                                                            ((3 as i32) as i8) as u8; // Bit 0x01:  JSON5.   Bit 0x02:  FLOAT
                                                        *__slate_slot_580 =
                                                            ((0 as i32) as i8) as u8;
                                                        break '__join_47;
                                                    } else {
                                                        unsafe {
                                                            (*pParse).iErr = i;
                                                        }
                                                        return -(1 as i32);
                                                    }
                                                }
                                                unsafe {
                                                    (*pParse).hasNonstd = ((1 as i32) as i8) as u8;
                                                }
                                                *__slate_slot_573 = ((0 as i32) as i8) as u8; // Bit 0x01:  JSON5.   Bit 0x02:  FLOAT
                                            }
                                            *__slate_slot_580 = ((0 as i32) as i8) as u8;
                                            0 as i32;
                                            0 as i32;
                                            0 as i32;
                                            *__slate_slot_568 = unsafe {
                                                *unsafe { (*__slate_slot_574).offset(i as isize) }
                                            };
                                            if (*__slate_slot_568 as i32) <= (48 as i32) {
                                                if (*__slate_slot_568 as i32) == (48 as i32) {
                                                    if (((unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_574).offset(
                                                                i.wrapping_add((1 as i32) as u32)
                                                                    as isize,
                                                            )
                                                        }
                                                    })
                                                        as i32)
                                                        == (120 as i32)
                                                        || ((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_574).offset(
                                                                    i.wrapping_add(
                                                                        (1 as i32) as u32,
                                                                    )
                                                                        as isize,
                                                                )
                                                            }
                                                        })
                                                            as i32)
                                                            == (88 as i32))
                                                        && (((unsafe {
                                                            *unsafe {
                                                                unsafe {
                                                                    std::ptr::addr_of!(
                                                                        sqlite3CtypeMap
                                                                    )
                                                                        as *const u8
                                                                }
                                                                .offset(
                                                                    ((((unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_574)
                                                                                .offset(
                                                                                    i.wrapping_add(
                                                                                        (2 as i32)
                                                                                            as u32,
                                                                                    )
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
                                                        })
                                                            as u32)
                                                            as i32)
                                                            & (8 as i32)
                                                            != (0 as i32)
                                                    {
                                                        0 as i32;
                                                        unsafe {
                                                            (*pParse).hasNonstd =
                                                                ((1 as i32) as i8) as u8;
                                                        }
                                                        *__slate_slot_573 =
                                                            ((1 as i32) as i8) as u8;
                                                        *__slate_slot_569 =
                                                            i.wrapping_add((3 as i32) as u32);
                                                        loop {
                                                            if (((unsafe {
                                                                *unsafe {
                                                                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(((((unsafe { *unsafe { (*__slate_slot_574).offset(*__slate_slot_569 as isize) } }) as u8) as u32) as i32) as isize)
                                                                }
                                                            })
                                                                as u32)
                                                                as i32)
                                                                & (8 as i32)
                                                                != (0 as i32)
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_1820,
                                                                    *__slate_slot_569,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1821,
                                                                    (*__slate_slot_1820)
                                                                        .wrapping_add(
                                                                            (1 as i32) as u32,
                                                                        ),
                                                                );
                                                                *__slate_slot_569 =
                                                                    *__slate_slot_1821;
                                                            } else {
                                                                break '__join_25;
                                                            }
                                                        }
                                                    } else {
                                                        if (((unsafe {
                                                            *unsafe {
                                                                unsafe {
                                                                    std::ptr::addr_of!(
                                                                        sqlite3CtypeMap
                                                                    )
                                                                        as *const u8
                                                                }
                                                                .offset(
                                                                    ((((unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_574)
                                                                                .offset(
                                                                                    i.wrapping_add(
                                                                                        (1 as i32)
                                                                                            as u32,
                                                                                    )
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
                                                        })
                                                            as u32)
                                                            as i32)
                                                            & (4 as i32)
                                                            != (0 as i32)
                                                        {
                                                            unsafe {
                                                                (*pParse).iErr = i.wrapping_add(
                                                                    (1 as i32) as u32,
                                                                );
                                                            }
                                                            return -(1 as i32);
                                                        }
                                                    }
                                                } else {
                                                    if !((((unsafe {
                                                        *unsafe {
                                                            unsafe {
                                                                std::ptr::addr_of!(sqlite3CtypeMap)
                                                                    as *const u8
                                                            }
                                                            .offset(
                                                                ((((unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_574).offset(
                                                                            i.wrapping_add(
                                                                                (1 as i32) as u32,
                                                                            )
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
                                                    })
                                                        as u32)
                                                        as i32)
                                                        & (4 as i32)
                                                        != (0 as i32))
                                                    {
                                                        if ((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_574).offset(
                                                                    i.wrapping_add(
                                                                        (1 as i32) as u32,
                                                                    )
                                                                        as isize,
                                                                )
                                                            }
                                                        })
                                                            as i32)
                                                            == (73 as i32)
                                                            || ((unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_574).offset(
                                                                        i.wrapping_add(
                                                                            (1 as i32) as u32,
                                                                        )
                                                                            as isize,
                                                                    )
                                                                }
                                                            })
                                                                as i32)
                                                                == (105 as i32)
                                                        {
                                                            *__slate_slot_1822 = (unsafe {
                                                                sqlite3_strnicmp(
                                                                    unsafe {
                                                                        (*__slate_slot_574).offset(
                                                                            i.wrapping_add(
                                                                                (1 as i32) as u32,
                                                                            )
                                                                                as isize,
                                                                        )
                                                                    },
                                                                    (b"inf\0".as_ptr() as *mut i8)
                                                                        as *const i8,
                                                                    3 as i32,
                                                                )
                                                            }) == (0 as i32);
                                                        } else {
                                                            *__slate_slot_1822 = false as bool;
                                                        }
                                                        if *__slate_slot_1822 {
                                                            unsafe {
                                                                (*pParse).hasNonstd =
                                                                    ((1 as i32) as i8) as u8;
                                                            }
                                                            if ((unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_574)
                                                                        .offset(i as isize)
                                                                }
                                                            })
                                                                as i32)
                                                                == (45 as i32)
                                                            {
                                                                jsonBlobAppendNode(
                                                                    pParse,
                                                                    ((5 as i32) as i8) as u8,
                                                                    ((6 as i32) as i64) as u64,
                                                                    (b"-9e999\0".as_ptr()
                                                                        as *mut i8)
                                                                        as *const (),
                                                                );
                                                            } else {
                                                                jsonBlobAppendNode(
                                                                    pParse,
                                                                    ((5 as i32) as i8) as u8,
                                                                    ((5 as i32) as i64) as u64,
                                                                    (b"9e999\0".as_ptr() as *mut i8)
                                                                        as *const (),
                                                                );
                                                            }
                                                            return i.wrapping_add(
                                                                (if (unsafe {
                                                                    sqlite3_strnicmp(
                                                                        unsafe {
                                                                            (*__slate_slot_574)
                                                                                .offset(
                                                                                    i.wrapping_add(
                                                                                        (4 as i32)
                                                                                            as u32,
                                                                                    )
                                                                                        as isize,
                                                                                )
                                                                        },
                                                                        (b"inity\0".as_ptr()
                                                                            as *mut i8)
                                                                            as *const i8,
                                                                        5 as i32,
                                                                    )
                                                                }) == (0 as i32)
                                                                {
                                                                    9 as i32
                                                                } else {
                                                                    4 as i32
                                                                })
                                                                    as u32,
                                                            )
                                                                as i32;
                                                        } else {
                                                            if ((unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_574).offset(
                                                                        i.wrapping_add(
                                                                            (1 as i32) as u32,
                                                                        )
                                                                            as isize,
                                                                    )
                                                                }
                                                            })
                                                                as i32)
                                                                == (46 as i32)
                                                            {
                                                                unsafe {
                                                                    (*pParse).hasNonstd =
                                                                        ((1 as i32) as i8) as u8;
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_1823,
                                                                    *__slate_slot_573,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1824,
                                                                    ((((*__slate_slot_1823 as u32)
                                                                        as i32)
                                                                        | (1 as i32))
                                                                        as i8)
                                                                        as u8,
                                                                );
                                                                *__slate_slot_573 =
                                                                    *__slate_slot_1824;
                                                            } else {
                                                                unsafe {
                                                                    (*pParse).iErr = i;
                                                                }
                                                                return -(1 as i32);
                                                            }
                                                        }
                                                    } else {
                                                        if ((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_574).offset(
                                                                    i.wrapping_add(
                                                                        (1 as i32) as u32,
                                                                    )
                                                                        as isize,
                                                                )
                                                            }
                                                        })
                                                            as i32)
                                                            == (48 as i32)
                                                        {
                                                            if (((unsafe {
                                                                *unsafe {
                                                                    unsafe {
                                                                        std::ptr::addr_of!(
                                                                            sqlite3CtypeMap
                                                                        )
                                                                            as *const u8
                                                                    }
                                                                    .offset(
                                                                        ((((unsafe {
                                                                            *unsafe {
                                                                                (*__slate_slot_574)
                                                                                    .offset(
                                                                                    i.wrapping_add(
                                                                                        (2 as i32)
                                                                                            as u32,
                                                                                    )
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
                                                            })
                                                                as u32)
                                                                as i32)
                                                                & (4 as i32)
                                                                != (0 as i32)
                                                            {
                                                                unsafe {
                                                                    (*pParse).iErr = i
                                                                        .wrapping_add(
                                                                            (1 as i32) as u32,
                                                                        );
                                                                }
                                                                return -(1 as i32);
                                                            } else {
                                                                if (((unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_574).offset(
                                                                            i.wrapping_add(
                                                                                (2 as i32) as u32,
                                                                            )
                                                                                as isize,
                                                                        )
                                                                    }
                                                                })
                                                                    as i32)
                                                                    == (120 as i32)
                                                                    || ((unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_574)
                                                                                .offset(
                                                                                    i.wrapping_add(
                                                                                        (2 as i32)
                                                                                            as u32,
                                                                                    )
                                                                                        as isize,
                                                                                )
                                                                        }
                                                                    })
                                                                        as i32)
                                                                        == (88 as i32))
                                                                    && (((unsafe {
                                                                        *unsafe {
                                                                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(((((unsafe { *unsafe { (*__slate_slot_574).offset(i.wrapping_add((3 as i32) as u32) as isize) } }) as u8) as u32) as i32) as isize)
                                                                        }
                                                                    })
                                                                        as u32)
                                                                        as i32)
                                                                        & (8 as i32)
                                                                        != (0 as i32)
                                                                {
                                                                    unsafe {
                                                                        (*pParse).hasNonstd =
                                                                            ((1 as i32) as i8)
                                                                                as u8;
                                                                    }
                                                                    std::ptr::write(
                                                                        __slate_slot_1825,
                                                                        *__slate_slot_573,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_1826,
                                                                        ((((*__slate_slot_1825
                                                                            as u32)
                                                                            as i32)
                                                                            | (1 as i32))
                                                                            as i8)
                                                                            as u8,
                                                                    );
                                                                    *__slate_slot_573 =
                                                                        *__slate_slot_1826;
                                                                    *__slate_slot_569 = i
                                                                        .wrapping_add(
                                                                            (4 as i32) as u32,
                                                                        );
                                                                    loop {
                                                                        if (((unsafe {
                                                                            *unsafe {
                                                                                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(((((unsafe { *unsafe { (*__slate_slot_574).offset(*__slate_slot_569 as isize) } }) as u8) as u32) as i32) as isize)
                                                                            }
                                                                        })
                                                                            as u32)
                                                                            as i32)
                                                                            & (8 as i32)
                                                                            != (0 as i32)
                                                                        {
                                                                            std::ptr::write(
                                                                                __slate_slot_1827,
                                                                                *__slate_slot_569,
                                                                            );
                                                                            std::ptr::write(__slate_slot_1828, (*__slate_slot_1827).wrapping_add((1 as i32) as u32));
                                                                            *__slate_slot_569 =
                                                                                *__slate_slot_1828;
                                                                        } else {
                                                                            break '__join_25;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        *__slate_slot_569 = i.wrapping_add((1 as i32) as u32);
                                        '__loop_46: loop {
                                            *__slate_slot_568 = unsafe {
                                                *unsafe {
                                                    (*__slate_slot_574)
                                                        .offset(*__slate_slot_569 as isize)
                                                }
                                            };
                                            if (((unsafe {
                                                *unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of!(sqlite3CtypeMap)
                                                            as *const u8
                                                    }
                                                    .offset(
                                                        (((*__slate_slot_568 as u8) as u32) as i32)
                                                            as isize,
                                                    )
                                                }
                                            })
                                                as u32)
                                                as i32)
                                                & (4 as i32)
                                                != (0 as i32)
                                            {
                                            } else {
                                                if (*__slate_slot_568 as i32) == (46 as i32) {
                                                    if ((*__slate_slot_573 as u32) as i32)
                                                        & (2 as i32)
                                                        != (0 as i32)
                                                    {
                                                        unsafe {
                                                            (*pParse).iErr = *__slate_slot_569;
                                                        }
                                                        return -(1 as i32);
                                                    } else {
                                                        std::ptr::write(
                                                            __slate_slot_1831,
                                                            *__slate_slot_573,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1832,
                                                            ((((*__slate_slot_1831 as u32) as i32)
                                                                | (2 as i32))
                                                                as i8)
                                                                as u8,
                                                        );
                                                        *__slate_slot_573 = *__slate_slot_1832;
                                                    }
                                                } else {
                                                    if (*__slate_slot_568 as i32) == (101 as i32)
                                                        || (*__slate_slot_568 as i32) == (69 as i32)
                                                    {
                                                        if ((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_574).offset(
                                                                    (*__slate_slot_569)
                                                                        .wrapping_sub(
                                                                            (1 as i32) as u32,
                                                                        )
                                                                        as isize,
                                                                )
                                                            }
                                                        })
                                                            as i32)
                                                            < (48 as i32)
                                                        {
                                                            if ((unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_574).offset(
                                                                        (*__slate_slot_569)
                                                                            .wrapping_sub(
                                                                                (1 as i32) as u32,
                                                                            )
                                                                            as isize,
                                                                    )
                                                                }
                                                            })
                                                                as i32)
                                                                == (46 as i32)
                                                                && (*__slate_slot_569)
                                                                    .wrapping_sub((2 as i32) as u32)
                                                                    >= i
                                                                && (((unsafe {
                                                                    *unsafe {
                                                                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(((((unsafe { *unsafe { (*__slate_slot_574).offset((*__slate_slot_569).wrapping_sub((2 as i32) as u32) as isize) } }) as u8) as u32) as i32) as isize)
                                                                    }
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    & (4 as i32)
                                                                    != (0 as i32)
                                                            {
                                                                unsafe {
                                                                    (*pParse).hasNonstd =
                                                                        ((1 as i32) as i8) as u8;
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_1833,
                                                                    *__slate_slot_573,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1834,
                                                                    ((((*__slate_slot_1833 as u32)
                                                                        as i32)
                                                                        | (1 as i32))
                                                                        as i8)
                                                                        as u8,
                                                                );
                                                                *__slate_slot_573 =
                                                                    *__slate_slot_1834;
                                                            } else {
                                                                unsafe {
                                                                    (*pParse).iErr =
                                                                        *__slate_slot_569;
                                                                }
                                                                return -(1 as i32);
                                                            }
                                                        }
                                                        if *__slate_slot_580 != (0 as u8) {
                                                            unsafe {
                                                                (*pParse).iErr = *__slate_slot_569;
                                                            }
                                                            return -(1 as i32);
                                                        } else {
                                                            std::ptr::write(
                                                                __slate_slot_1835,
                                                                *__slate_slot_573,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1836,
                                                                ((((*__slate_slot_1835 as u32)
                                                                    as i32)
                                                                    | (2 as i32))
                                                                    as i8)
                                                                    as u8,
                                                            );
                                                            *__slate_slot_573 = *__slate_slot_1836;
                                                            *__slate_slot_580 =
                                                                ((1 as i32) as i8) as u8;
                                                            *__slate_slot_568 = unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_574).offset(
                                                                        (*__slate_slot_569)
                                                                            .wrapping_add(
                                                                                (1 as i32) as u32,
                                                                            )
                                                                            as isize,
                                                                    )
                                                                }
                                                            };
                                                            if (*__slate_slot_568 as i32)
                                                                == (43 as i32)
                                                                || (*__slate_slot_568 as i32)
                                                                    == (45 as i32)
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_1837,
                                                                    *__slate_slot_569,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1838,
                                                                    (*__slate_slot_1837)
                                                                        .wrapping_add(
                                                                            (1 as i32) as u32,
                                                                        ),
                                                                );
                                                                *__slate_slot_569 =
                                                                    *__slate_slot_1838;
                                                                *__slate_slot_568 = unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_574).offset(
                                                                            (*__slate_slot_569)
                                                                                .wrapping_add(
                                                                                    (1 as i32)
                                                                                        as u32,
                                                                                )
                                                                                as isize,
                                                                        )
                                                                    }
                                                                };
                                                            }
                                                            if (*__slate_slot_568 as i32)
                                                                < (48 as i32)
                                                                || (*__slate_slot_568 as i32)
                                                                    > (57 as i32)
                                                            {
                                                                unsafe {
                                                                    (*pParse).iErr =
                                                                        *__slate_slot_569;
                                                                }
                                                                return -(1 as i32);
                                                            }
                                                        }
                                                    } else {
                                                        break '__loop_46;
                                                    }
                                                }
                                            }
                                            std::ptr::write(__slate_slot_1829, *__slate_slot_569);
                                            std::ptr::write(
                                                __slate_slot_1830,
                                                (*__slate_slot_1829)
                                                    .wrapping_add((1 as i32) as u32),
                                            );
                                            *__slate_slot_569 = *__slate_slot_1830;
                                        }
                                        if ((unsafe {
                                            *unsafe {
                                                (*__slate_slot_574).offset(
                                                    (*__slate_slot_569)
                                                        .wrapping_sub((1 as i32) as u32)
                                                        as isize,
                                                )
                                            }
                                        }) as i32)
                                            < (48 as i32)
                                        {
                                            if ((unsafe {
                                                *unsafe {
                                                    (*__slate_slot_574).offset(
                                                        (*__slate_slot_569)
                                                            .wrapping_sub((1 as i32) as u32)
                                                            as isize,
                                                    )
                                                }
                                            })
                                                as i32)
                                                == (46 as i32)
                                                && (*__slate_slot_569)
                                                    .wrapping_sub((2 as i32) as u32)
                                                    >= i
                                                && (((unsafe {
                                                    *unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of!(sqlite3CtypeMap)
                                                                as *const u8
                                                        }
                                                        .offset(
                                                            ((((unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_574).offset(
                                                                        (*__slate_slot_569)
                                                                            .wrapping_sub(
                                                                                (2 as i32) as u32,
                                                                            )
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
                                                })
                                                    as u32)
                                                    as i32)
                                                    & (4 as i32)
                                                    != (0 as i32)
                                            {
                                                unsafe {
                                                    (*pParse).hasNonstd = ((1 as i32) as i8) as u8;
                                                }
                                                std::ptr::write(
                                                    __slate_slot_1839,
                                                    *__slate_slot_573,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_1840,
                                                    ((((*__slate_slot_1839 as u32) as i32)
                                                        | (1 as i32))
                                                        as i8)
                                                        as u8,
                                                );
                                                *__slate_slot_573 = *__slate_slot_1840;
                                            } else {
                                                unsafe {
                                                    (*pParse).iErr = *__slate_slot_569;
                                                }
                                                return -(1 as i32);
                                            }
                                        }
                                    }
                                    0 as i32;
                                    0 as i32;
                                    0 as i32;
                                    if ((unsafe {
                                        *unsafe { (*__slate_slot_574).offset(i as isize) }
                                    }) as i32)
                                        == (43 as i32)
                                    {
                                        std::ptr::write(__slate_slot_1841, i);
                                        std::ptr::write(
                                            __slate_slot_1842,
                                            (*__slate_slot_1841).wrapping_add((1 as i32) as u32),
                                        );
                                        i = *__slate_slot_1842;
                                    }
                                    jsonBlobAppendNode(
                                        pParse,
                                        (((3 as i32) + ((*__slate_slot_573 as u32) as i32)) as i8)
                                            as u8,
                                        (*__slate_slot_569).wrapping_sub(i) as u64,
                                        (unsafe { (*__slate_slot_574).offset(i as isize) })
                                            as *const (),
                                    );
                                    return *__slate_slot_569 as i32;
                                }
                                if (unsafe {
                                    strncmp(
                                        unsafe { (*__slate_slot_574).offset(i as isize) },
                                        (b"false\0".as_ptr() as *mut i8) as *const i8,
                                        ((5 as i32) as i64) as u64,
                                    )
                                }) == (0 as i32)
                                    && !((((unsafe {
                                        *unsafe {
                                            unsafe {
                                                std::ptr::addr_of!(sqlite3CtypeMap) as *const u8
                                            }
                                            .offset(
                                                ((((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_574).offset(
                                                            i.wrapping_add((5 as i32) as u32)
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
                                        & (6 as i32)
                                        != (0 as i32))
                                {
                                    jsonBlobAppendOneByte(pParse, ((2 as i32) as i8) as u8);
                                    return i.wrapping_add((5 as i32) as u32) as i32;
                                } else {
                                    unsafe {
                                        (*pParse).iErr = i;
                                    }
                                    return -(1 as i32);
                                }
                            }
                            if (unsafe {
                                strncmp(
                                    unsafe { (*__slate_slot_574).offset(i as isize) },
                                    (b"true\0".as_ptr() as *mut i8) as *const i8,
                                    ((4 as i32) as i64) as u64,
                                )
                            }) == (0 as i32)
                                && !((((unsafe {
                                    *unsafe {
                                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                            .offset(
                                                ((((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_574).offset(
                                                            i.wrapping_add((4 as i32) as u32)
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
                                    & (6 as i32)
                                    != (0 as i32))
                            {
                                jsonBlobAppendOneByte(pParse, ((1 as i32) as i8) as u8);
                                return i.wrapping_add((4 as i32) as u32) as i32;
                            } else {
                                unsafe {
                                    (*pParse).iErr = i;
                                }
                                return -(1 as i32);
                            }
                        }
                        *__slate_slot_578 = ((7 as i32) as i8) as u8; // Parse string
                        break '__join_126;
                    }
                    unsafe {
                        (*pParse).hasNonstd = ((1 as i32) as i8) as u8;
                    }
                    *__slate_slot_578 = ((7 as i32) as i8) as u8;
                }
                *__slate_slot_579 = unsafe { *unsafe { (*__slate_slot_574).offset(i as isize) } };
                *__slate_slot_569 = i.wrapping_add((1 as i32) as u32);
                '__join_86: {
                    '__loop_87: loop {
                        if (1 as i32) != (0 as i32) {
                            // exit-by-break
                            if (unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(jsonIsOk.0) as *const i8 }.offset(
                                        ((((unsafe {
                                            *unsafe {
                                                (*__slate_slot_574)
                                                    .offset(*__slate_slot_569 as isize)
                                            }
                                        }) as u8) as u32)
                                            as i32)
                                            as isize,
                                    )
                                }
                            }) != (0 as i8)
                            {
                                if !((unsafe {
                                    *unsafe {
                                        unsafe { std::ptr::addr_of!(jsonIsOk.0) as *const i8 }
                                            .offset(
                                                ((((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_574).offset(
                                                            (*__slate_slot_569)
                                                                .wrapping_add((1 as i32) as u32)
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
                                }) != (0 as i8))
                                {
                                    std::ptr::write(__slate_slot_1804, *__slate_slot_569);
                                    std::ptr::write(
                                        __slate_slot_1805,
                                        (*__slate_slot_1804).wrapping_add((1 as i32) as u32),
                                    );
                                    *__slate_slot_569 = *__slate_slot_1805;
                                } else {
                                    if !((unsafe {
                                        *unsafe {
                                            unsafe { std::ptr::addr_of!(jsonIsOk.0) as *const i8 }
                                                .offset(
                                                    ((((unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_574).offset(
                                                                (*__slate_slot_569)
                                                                    .wrapping_add((2 as i32) as u32)
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
                                    }) != (0 as i8))
                                    {
                                        std::ptr::write(__slate_slot_1806, *__slate_slot_569);
                                        std::ptr::write(
                                            __slate_slot_1807,
                                            (*__slate_slot_1806).wrapping_add((2 as i32) as u32),
                                        );
                                        *__slate_slot_569 = *__slate_slot_1807;
                                    } else {
                                        std::ptr::write(__slate_slot_1808, *__slate_slot_569);
                                        std::ptr::write(
                                            __slate_slot_1809,
                                            (*__slate_slot_1808).wrapping_add((3 as i32) as u32),
                                        );
                                        *__slate_slot_569 = *__slate_slot_1809;
                                        continue '__loop_87;
                                    }
                                }
                            }
                            *__slate_slot_568 = unsafe {
                                *unsafe { (*__slate_slot_574).offset(*__slate_slot_569 as isize) }
                            };
                            if (*__slate_slot_568 as i32) == (*__slate_slot_579 as i32) {
                                break '__join_86;
                            } else {
                                if (*__slate_slot_568 as i32) == (92 as i32) {
                                    std::ptr::write(__slate_slot_1810, *__slate_slot_569);
                                    std::ptr::write(
                                        __slate_slot_1811,
                                        (*__slate_slot_1810).wrapping_add((1 as i32) as u32),
                                    );
                                    *__slate_slot_569 = *__slate_slot_1811;
                                    *__slate_slot_568 = unsafe {
                                        *unsafe {
                                            (*__slate_slot_574).offset(*__slate_slot_1811 as isize)
                                        }
                                    };
                                    if (*__slate_slot_568 as i32) == (34 as i32)
                                        || (*__slate_slot_568 as i32) == (92 as i32)
                                        || (*__slate_slot_568 as i32) == (47 as i32)
                                        || (*__slate_slot_568 as i32) == (98 as i32)
                                        || (*__slate_slot_568 as i32) == (102 as i32)
                                        || (*__slate_slot_568 as i32) == (110 as i32)
                                        || (*__slate_slot_568 as i32) == (114 as i32)
                                        || (*__slate_slot_568 as i32) == (116 as i32)
                                    {
                                        *__slate_slot_1812 = true as bool;
                                    } else {
                                        if (*__slate_slot_568 as i32) == (117 as i32) {
                                            *__slate_slot_1813 = jsonIs4Hex(unsafe {
                                                (*__slate_slot_574).offset(
                                                    (*__slate_slot_569)
                                                        .wrapping_add((1 as i32) as u32)
                                                        as isize,
                                                )
                                            }) != (0 as i32);
                                        } else {
                                            *__slate_slot_1813 = false as bool;
                                        }
                                        *__slate_slot_1812 = *__slate_slot_1813;
                                    }
                                    if *__slate_slot_1812 {
                                        if ((*__slate_slot_578 as u32) as i32) == (7 as i32) {
                                            *__slate_slot_578 = ((8 as i32) as i8) as u8;
                                        }
                                    } else {
                                        if (*__slate_slot_568 as i32) == (39 as i32)
                                            || (*__slate_slot_568 as i32) == (118 as i32)
                                            || (*__slate_slot_568 as i32) == (10 as i32)
                                            || (*__slate_slot_568 as i32) == (48 as i32)
                                                && !((((unsafe {
                                                    *unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of!(sqlite3CtypeMap)
                                                                as *const u8
                                                        }
                                                        .offset(
                                                            ((((unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_574).offset(
                                                                        (*__slate_slot_569)
                                                                            .wrapping_add(
                                                                                (1 as i32) as u32,
                                                                            )
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
                                                })
                                                    as u32)
                                                    as i32)
                                                    & (4 as i32)
                                                    != (0 as i32))
                                            || (226 as i32)
                                                == (((*__slate_slot_568 as u8) as u32) as i32)
                                                && (128 as i32)
                                                    == ((((unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_574).offset(
                                                                (*__slate_slot_569)
                                                                    .wrapping_add((1 as i32) as u32)
                                                                    as isize,
                                                            )
                                                        }
                                                    })
                                                        as u8)
                                                        as u32)
                                                        as i32)
                                                && ((168 as i32)
                                                    == ((((unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_574).offset(
                                                                (*__slate_slot_569)
                                                                    .wrapping_add((2 as i32) as u32)
                                                                    as isize,
                                                            )
                                                        }
                                                    })
                                                        as u8)
                                                        as u32)
                                                        as i32)
                                                    || (169 as i32)
                                                        == ((((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_574).offset(
                                                                    (*__slate_slot_569)
                                                                        .wrapping_add(
                                                                            (2 as i32) as u32,
                                                                        )
                                                                        as isize,
                                                                )
                                                            }
                                                        })
                                                            as u8)
                                                            as u32)
                                                            as i32))
                                        {
                                            *__slate_slot_1814 = true as bool;
                                        } else {
                                            if (*__slate_slot_568 as i32) == (120 as i32) {
                                                *__slate_slot_1815 = jsonIs2Hex(unsafe {
                                                    (*__slate_slot_574).offset(
                                                        (*__slate_slot_569)
                                                            .wrapping_add((1 as i32) as u32)
                                                            as isize,
                                                    )
                                                }) != (0 as i32);
                                            } else {
                                                *__slate_slot_1815 = false as bool;
                                            }
                                            *__slate_slot_1814 = *__slate_slot_1815;
                                        }
                                        if *__slate_slot_1814 {
                                            *__slate_slot_578 = ((9 as i32) as i8) as u8;
                                            unsafe {
                                                (*pParse).hasNonstd = ((1 as i32) as i8) as u8;
                                            }
                                        } else {
                                            if (*__slate_slot_568 as i32) == (13 as i32) {
                                                if ((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_574).offset(
                                                            (*__slate_slot_569)
                                                                .wrapping_add((1 as i32) as u32)
                                                                as isize,
                                                        )
                                                    }
                                                })
                                                    as i32)
                                                    == (10 as i32)
                                                {
                                                    std::ptr::write(
                                                        __slate_slot_1816,
                                                        *__slate_slot_569,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1817,
                                                        (*__slate_slot_1816)
                                                            .wrapping_add((1 as i32) as u32),
                                                    );
                                                    *__slate_slot_569 = *__slate_slot_1817;
                                                }
                                                *__slate_slot_578 = ((9 as i32) as i8) as u8;
                                                unsafe {
                                                    (*pParse).hasNonstd = ((1 as i32) as i8) as u8;
                                                }
                                            } else {
                                                unsafe {
                                                    (*pParse).iErr = *__slate_slot_569;
                                                }
                                                return -(1 as i32);
                                            }
                                        }
                                    }
                                // Correct implementation
                                } else {
                                    if (*__slate_slot_568 as i32) <= (31 as i32) {
                                        if (*__slate_slot_568 as i32) == (0 as i32) {
                                            break '__loop_87;
                                        } else {
                                            // Control characters are not allowed in canonical JSON string
                                            // literals, but are allowed in JSON5 string literals.
                                            *__slate_slot_578 = ((9 as i32) as i8) as u8;
                                            unsafe {
                                                (*pParse).hasNonstd = ((1 as i32) as i8) as u8;
                                            }
                                        }
                                    } else {
                                        if (*__slate_slot_568 as i32) == (34 as i32) {
                                            *__slate_slot_578 = ((9 as i32) as i8) as u8;
                                        }
                                    }
                                }
                                std::ptr::write(__slate_slot_1818, *__slate_slot_569);
                                std::ptr::write(
                                    __slate_slot_1819,
                                    (*__slate_slot_1818).wrapping_add((1 as i32) as u32),
                                );
                                *__slate_slot_569 = *__slate_slot_1819;
                            }
                        } else {
                            break '__join_86;
                        }
                    }
                    unsafe {
                        (*pParse).iErr = *__slate_slot_569;
                    }
                    return -(1 as i32);
                }
                jsonBlobAppendNode(
                    pParse,
                    *__slate_slot_578,
                    (*__slate_slot_569)
                        .wrapping_sub((1 as i32) as u32)
                        .wrapping_sub(i) as u64,
                    (unsafe {
                        (*__slate_slot_574).offset(i.wrapping_add((1 as i32) as u32) as isize)
                    }) as *const (),
                );
                return (*__slate_slot_569).wrapping_add((1 as i32) as u32) as i32;
            }
            // Parse array
            *__slate_slot_570 = unsafe { (*pParse).nBlob };
            0 as i32;
            jsonBlobAppendNode(
                pParse,
                ((11 as i32) as i8) as u8,
                ((unsafe { (*pParse).nJson }) as u32).wrapping_sub(i) as u64,
                std::ptr::null::<()>(),
            );
            *__slate_slot_571 = unsafe { (*pParse).nBlob };
            if (unsafe { (*pParse).oom }) != (0 as u8) {
                return -(1 as i32);
            } else {
                std::ptr::write(__slate_slot_1794, pParse);
                std::ptr::write(__slate_slot_1795, unsafe { (*(*__slate_slot_1794)).iDepth });
                std::ptr::write(
                    __slate_slot_1796,
                    ((((*__slate_slot_1795 as u32) as i32) + (1 as i32)) as i16) as u16,
                );
                unsafe {
                    (*(*__slate_slot_1794)).iDepth = *__slate_slot_1796;
                }
                if ((*__slate_slot_1796 as u32) as i32) > (1000 as i32) {
                    unsafe {
                        (*pParse).iErr = i;
                    }
                    return -(1 as i32);
                } else {
                    *__slate_slot_569 = i.wrapping_add((1 as i32) as u32);
                    '__join_129: {
                        '__join_146: {
                            '__loop_147: loop {
                                *__slate_slot_572 =
                                    jsonTranslateTextToBlob(pParse, *__slate_slot_569);
                                if *__slate_slot_572 <= (0 as i32) {
                                    break '__join_146;
                                } else {
                                    '__join_130: {
                                        *__slate_slot_569 = *__slate_slot_572 as u32;
                                        if ((unsafe {
                                            *unsafe {
                                                (*__slate_slot_574)
                                                    .offset(*__slate_slot_569 as isize)
                                            }
                                        }) as i32)
                                            == (44 as i32)
                                        {
                                        } else {
                                            if ((unsafe {
                                                *unsafe {
                                                    (*__slate_slot_574)
                                                        .offset(*__slate_slot_569 as isize)
                                                }
                                            })
                                                as i32)
                                                == (93 as i32)
                                            {
                                                break '__join_129;
                                            } else {
                                                if (unsafe {
                                                    *unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of!(jsonIsSpace.0)
                                                                as *const i8
                                                        }
                                                        .offset(
                                                            ((((unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_574).offset(
                                                                        *__slate_slot_569 as isize,
                                                                    )
                                                                }
                                                            })
                                                                as u8)
                                                                as u32)
                                                                as i32)
                                                                as isize,
                                                        )
                                                    }
                                                }) != (0 as i8)
                                                {
                                                    std::ptr::write(
                                                        __slate_slot_1799,
                                                        *__slate_slot_569,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1800,
                                                        (*__slate_slot_1799).wrapping_add(
                                                            ((1 as i32) as u32).wrapping_add(
                                                                (unsafe {
                                                                    strspn(
                                                                        unsafe {
                                                                            (*__slate_slot_574)
                                                                                .offset(
                                                                                (*__slate_slot_569)
                                                                                    .wrapping_add(
                                                                                        (1 as i32)
                                                                                            as u32,
                                                                                    )
                                                                                    as isize,
                                                                            )
                                                                        },
                                                                        unsafe {
                                                                            std::ptr::addr_of!(
                                                                                jsonSpaces
                                                                            )
                                                                                as *const i8
                                                                        },
                                                                    )
                                                                })
                                                                    as u32,
                                                            ),
                                                        ),
                                                    );
                                                    *__slate_slot_569 = *__slate_slot_1800;
                                                    if ((unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_574)
                                                                .offset(*__slate_slot_569 as isize)
                                                        }
                                                    })
                                                        as i32)
                                                        == (44 as i32)
                                                    {
                                                        break '__join_130;
                                                    } else {
                                                        if ((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_574).offset(
                                                                    *__slate_slot_569 as isize,
                                                                )
                                                            }
                                                        })
                                                            as i32)
                                                            == (93 as i32)
                                                        {
                                                            break '__join_129;
                                                        }
                                                    }
                                                }
                                                *__slate_slot_572 = jsonTranslateTextToBlob(
                                                    pParse,
                                                    *__slate_slot_569,
                                                );
                                                if *__slate_slot_572 == -(4 as i32) {
                                                    *__slate_slot_569 = unsafe { (*pParse).iErr };
                                                } else {
                                                    break '__loop_147;
                                                }
                                            }
                                        }
                                    }
                                    std::ptr::write(__slate_slot_1797, *__slate_slot_569);
                                    std::ptr::write(
                                        __slate_slot_1798,
                                        (*__slate_slot_1797).wrapping_add((1 as i32) as u32),
                                    );
                                    *__slate_slot_569 = *__slate_slot_1798;
                                }
                            }
                            if *__slate_slot_572 == -(3 as i32) {
                                *__slate_slot_569 = unsafe { (*pParse).iErr };
                                break '__join_129;
                            } else {
                                unsafe {
                                    (*pParse).iErr = *__slate_slot_569;
                                }
                                return -(1 as i32);
                            }
                        }
                        if *__slate_slot_572 == -(3 as i32) {
                            *__slate_slot_569 = unsafe { (*pParse).iErr };
                            if (unsafe { (*pParse).nBlob }) != *__slate_slot_571 {
                                unsafe {
                                    (*pParse).hasNonstd = ((1 as i32) as i8) as u8;
                                }
                            }
                        } else {
                            if *__slate_slot_572 != -(1 as i32) {
                                unsafe {
                                    (*pParse).iErr = *__slate_slot_569;
                                }
                            }
                            return -(1 as i32);
                        }
                    }
                    jsonBlobChangePayloadSize(
                        pParse,
                        *__slate_slot_570,
                        unsafe { (*pParse).nBlob }.wrapping_sub(*__slate_slot_571),
                    );
                    std::ptr::write(__slate_slot_1801, pParse);
                    std::ptr::write(__slate_slot_1802, unsafe { (*(*__slate_slot_1801)).iDepth });
                    std::ptr::write(
                        __slate_slot_1803,
                        ((((*__slate_slot_1802 as u32) as i32) - (1 as i32)) as i16) as u16,
                    );
                    unsafe {
                        (*(*__slate_slot_1801)).iDepth = *__slate_slot_1803;
                    }
                    return (*__slate_slot_569).wrapping_add((1 as i32) as u32) as i32;
                }
            }
        }
        // Parse object
        *__slate_slot_570 = unsafe { (*pParse).nBlob };
        jsonBlobAppendNode(
            pParse,
            ((12 as i32) as i8) as u8,
            ((unsafe { (*pParse).nJson }) as u32).wrapping_sub(i) as u64,
            std::ptr::null::<()>(),
        );
        std::ptr::write(__slate_slot_1769, pParse);
        std::ptr::write(__slate_slot_1770, unsafe { (*(*__slate_slot_1769)).iDepth });
        std::ptr::write(
            __slate_slot_1771,
            ((((*__slate_slot_1770 as u32) as i32) + (1 as i32)) as i16) as u16,
        );
        unsafe {
            (*(*__slate_slot_1769)).iDepth = *__slate_slot_1771;
        }
        if ((*__slate_slot_1771 as u32) as i32) > (1000 as i32) {
            unsafe {
                (*pParse).iErr = i;
            }
            return -(1 as i32);
        } else {
            *__slate_slot_571 = unsafe { (*pParse).nBlob };
            *__slate_slot_569 = i.wrapping_add((1 as i32) as u32);
            '__join_153: {
                '__join_209: {
                    '__join_167: {
                        '__join_157: {
                            '__join_173: {
                                '__loop_211: loop {
                                    std::ptr::write(__slate_slot_575, unsafe { (*pParse).nBlob });
                                    *__slate_slot_572 =
                                        jsonTranslateTextToBlob(pParse, *__slate_slot_569);
                                    if *__slate_slot_572 <= (0 as i32) {
                                        if *__slate_slot_572 == -(2 as i32) {
                                            break '__join_209;
                                        } else {
                                            std::ptr::write(__slate_slot_1774, *__slate_slot_569);
                                            std::ptr::write(
                                                __slate_slot_1775,
                                                (*__slate_slot_1774).wrapping_add(json5Whitespace(
                                                    unsafe {
                                                        (*__slate_slot_574)
                                                            .offset(*__slate_slot_569 as isize)
                                                    },
                                                )
                                                    as u32),
                                            );
                                            *__slate_slot_569 = *__slate_slot_1775;
                                            *__slate_slot_576 = 7 as i32;
                                            if (((unsafe {
                                                *unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of!(sqlite3CtypeMap)
                                                            as *const u8
                                                    }
                                                    .offset(
                                                        ((((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_574).offset(
                                                                    *__slate_slot_569 as isize,
                                                                )
                                                            }
                                                        })
                                                            as u8)
                                                            as u32)
                                                            as i32)
                                                            as isize,
                                                    )
                                                }
                                            })
                                                as u32)
                                                as i32)
                                                & (66 as i32)
                                                != (0 as i32)
                                            {
                                                *__slate_slot_1776 = true as bool;
                                            } else {
                                                if ((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_574)
                                                            .offset(*__slate_slot_569 as isize)
                                                    }
                                                })
                                                    as i32)
                                                    == (92 as i32)
                                                {
                                                    *__slate_slot_1777 = jsonIs4HexB(
                                                        unsafe {
                                                            (*__slate_slot_574).offset(
                                                                (*__slate_slot_569)
                                                                    .wrapping_add((1 as i32) as u32)
                                                                    as isize,
                                                            )
                                                        },
                                                        std::ptr::addr_of_mut!(*__slate_slot_576),
                                                    ) != (0 as i32);
                                                } else {
                                                    *__slate_slot_1777 = false as bool;
                                                }
                                                *__slate_slot_1776 = *__slate_slot_1777;
                                            }
                                            if *__slate_slot_1776 {
                                                std::ptr::write(
                                                    __slate_slot_577,
                                                    (*__slate_slot_569)
                                                        .wrapping_add((1 as i32) as u32)
                                                        as i32,
                                                );
                                                loop {
                                                    '__join_192: {
                                                        if (((unsafe {
                                                            *unsafe {
                                                                unsafe {
                                                                    std::ptr::addr_of!(
                                                                        sqlite3CtypeMap
                                                                    )
                                                                        as *const u8
                                                                }
                                                                .offset(
                                                                    ((((unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_574)
                                                                                .offset(
                                                                                *__slate_slot_577
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
                                                        })
                                                            as u32)
                                                            as i32)
                                                            & (70 as i32)
                                                            != (0 as i32)
                                                        {
                                                            *__slate_slot_1778 =
                                                                json5Whitespace(unsafe {
                                                                    (*__slate_slot_574).offset(
                                                                        *__slate_slot_577 as isize,
                                                                    )
                                                                }) == (0 as i32);
                                                        } else {
                                                            *__slate_slot_1778 = false as bool;
                                                        }
                                                    }
                                                    '__join_186: {
                                                        if *__slate_slot_1778 {
                                                            *__slate_slot_1779 = true as bool;
                                                        } else {
                                                            if ((unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_574).offset(
                                                                        *__slate_slot_577 as isize,
                                                                    )
                                                                }
                                                            })
                                                                as i32)
                                                                == (92 as i32)
                                                            {
                                                                *__slate_slot_1780 = jsonIs4HexB(
                                                                    unsafe {
                                                                        (*__slate_slot_574).offset(
                                                                            (*__slate_slot_577
                                                                                + (1 as i32))
                                                                                as isize,
                                                                        )
                                                                    },
                                                                    std::ptr::addr_of_mut!(
                                                                        *__slate_slot_576
                                                                    ),
                                                                ) != (0
                                                                    as i32);
                                                            } else {
                                                                *__slate_slot_1780 = false as bool;
                                                            }
                                                            *__slate_slot_1779 = *__slate_slot_1780;
                                                        }
                                                    }
                                                    if *__slate_slot_1779 {
                                                        std::ptr::write(
                                                            __slate_slot_1781,
                                                            *__slate_slot_577,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1782,
                                                            *__slate_slot_1781 + (1 as i32),
                                                        );
                                                        *__slate_slot_577 = *__slate_slot_1782;
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                0 as i32;
                                                jsonBlobAppendNode(
                                                    pParse,
                                                    (*__slate_slot_576 as i8) as u8,
                                                    (*__slate_slot_577 as u32)
                                                        .wrapping_sub(*__slate_slot_569)
                                                        as u64,
                                                    (unsafe {
                                                        (*__slate_slot_574)
                                                            .offset(*__slate_slot_569 as isize)
                                                    })
                                                        as *const (),
                                                );
                                                unsafe {
                                                    (*pParse).hasNonstd = ((1 as i32) as i8) as u8;
                                                }
                                                *__slate_slot_572 = *__slate_slot_577;
                                            } else {
                                                break '__loop_211;
                                            }
                                        }
                                    }
                                    if (unsafe { (*pParse).oom }) != (0 as u8) {
                                        return -(1 as i32);
                                    } else {
                                        *__slate_slot_573 = (((((unsafe {
                                            *unsafe {
                                                unsafe { (*pParse).aBlob }
                                                    .offset(*__slate_slot_575 as isize)
                                            }
                                        })
                                            as u32)
                                            as i32)
                                            & (15 as i32))
                                            as i8)
                                            as u8;
                                        if ((*__slate_slot_573 as u32) as i32) < (7 as i32)
                                            || ((*__slate_slot_573 as u32) as i32) > (10 as i32)
                                        {
                                            unsafe {
                                                (*pParse).iErr = *__slate_slot_569;
                                            }
                                            return -(1 as i32);
                                        } else {
                                            '__join_168: {
                                                *__slate_slot_569 = *__slate_slot_572 as u32;
                                                if ((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_574)
                                                            .offset(*__slate_slot_569 as isize)
                                                    }
                                                })
                                                    as i32)
                                                    == (58 as i32)
                                                {
                                                    std::ptr::write(
                                                        __slate_slot_1783,
                                                        *__slate_slot_569,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1784,
                                                        (*__slate_slot_1783)
                                                            .wrapping_add((1 as i32) as u32),
                                                    );
                                                    *__slate_slot_569 = *__slate_slot_1784;
                                                } else {
                                                    if (unsafe {
                                                        *unsafe {
                                                            unsafe {
                                                                std::ptr::addr_of!(jsonIsSpace.0)
                                                                    as *const i8
                                                            }
                                                            .offset(
                                                                ((((unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_574).offset(
                                                                            *__slate_slot_569
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
                                                    }) != (0 as i8)
                                                    {
                                                        // strspn() is not helpful here
                                                        loop {
                                                            std::ptr::write(
                                                                __slate_slot_1785,
                                                                *__slate_slot_569,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1786,
                                                                (*__slate_slot_1785).wrapping_add(
                                                                    (1 as i32) as u32,
                                                                ),
                                                            );
                                                            *__slate_slot_569 = *__slate_slot_1786;
                                                            if !((unsafe {
                                                                *unsafe {
                                                                    unsafe { std::ptr::addr_of!(jsonIsSpace.0) as *const i8 }.offset(((((unsafe { *unsafe { (*__slate_slot_574).offset(*__slate_slot_569 as isize) } }) as u8) as u32) as i32) as isize)
                                                                }
                                                            }) != (0 as i8))
                                                            {
                                                                break;
                                                            }
                                                        }
                                                        if ((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_574).offset(
                                                                    *__slate_slot_569 as isize,
                                                                )
                                                            }
                                                        })
                                                            as i32)
                                                            == (58 as i32)
                                                        {
                                                            std::ptr::write(
                                                                __slate_slot_1787,
                                                                *__slate_slot_569,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1788,
                                                                (*__slate_slot_1787).wrapping_add(
                                                                    (1 as i32) as u32,
                                                                ),
                                                            );
                                                            *__slate_slot_569 = *__slate_slot_1788;
                                                            break '__join_168;
                                                        }
                                                    }
                                                    *__slate_slot_572 = jsonTranslateTextToBlob(
                                                        pParse,
                                                        *__slate_slot_569,
                                                    );
                                                    if *__slate_slot_572 != -(5 as i32) {
                                                        break '__join_173;
                                                    } else {
                                                        *__slate_slot_569 =
                                                            unsafe { (*pParse).iErr }
                                                                .wrapping_add((1 as i32) as u32);
                                                    }
                                                }
                                            }
                                            *__slate_slot_572 =
                                                jsonTranslateTextToBlob(pParse, *__slate_slot_569);
                                            if *__slate_slot_572 <= (0 as i32) {
                                                break '__join_167;
                                            } else {
                                                '__join_154: {
                                                    *__slate_slot_569 = *__slate_slot_572 as u32;
                                                    if ((unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_574)
                                                                .offset(*__slate_slot_569 as isize)
                                                        }
                                                    })
                                                        as i32)
                                                        == (44 as i32)
                                                    {
                                                    } else {
                                                        if ((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_574).offset(
                                                                    *__slate_slot_569 as isize,
                                                                )
                                                            }
                                                        })
                                                            as i32)
                                                            == (125 as i32)
                                                        {
                                                            break '__join_153;
                                                        } else {
                                                            if (unsafe {
                                                                *unsafe {
                                                                    unsafe { std::ptr::addr_of!(jsonIsSpace.0) as *const i8 }.offset(((((unsafe { *unsafe { (*__slate_slot_574).offset(*__slate_slot_569 as isize) } }) as u8) as u32) as i32) as isize)
                                                                }
                                                            }) != (0 as i8)
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_1789,
                                                                    *__slate_slot_569,
                                                                );
                                                                std::ptr::write(__slate_slot_1790, (*__slate_slot_1789).wrapping_add(((1 as i32) as u32).wrapping_add((unsafe { strspn(unsafe { (*__slate_slot_574).offset((*__slate_slot_569).wrapping_add((1 as i32) as u32) as isize) }, unsafe { std::ptr::addr_of!(jsonSpaces) as *const i8 }) }) as u32)));
                                                                *__slate_slot_569 =
                                                                    *__slate_slot_1790;
                                                                if ((unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_574).offset(
                                                                            *__slate_slot_569
                                                                                as isize,
                                                                        )
                                                                    }
                                                                })
                                                                    as i32)
                                                                    == (44 as i32)
                                                                {
                                                                    break '__join_154;
                                                                } else {
                                                                    if ((unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_574)
                                                                                .offset(
                                                                                *__slate_slot_569
                                                                                    as isize,
                                                                            )
                                                                        }
                                                                    })
                                                                        as i32)
                                                                        == (125 as i32)
                                                                    {
                                                                        break '__join_153;
                                                                    }
                                                                }
                                                            }
                                                            *__slate_slot_572 =
                                                                jsonTranslateTextToBlob(
                                                                    pParse,
                                                                    *__slate_slot_569,
                                                                );
                                                            if *__slate_slot_572 == -(4 as i32) {
                                                                *__slate_slot_569 =
                                                                    unsafe { (*pParse).iErr };
                                                            } else {
                                                                break '__join_157;
                                                            }
                                                        }
                                                    }
                                                }
                                                std::ptr::write(
                                                    __slate_slot_1772,
                                                    *__slate_slot_569,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_1773,
                                                    (*__slate_slot_1772)
                                                        .wrapping_add((1 as i32) as u32),
                                                );
                                                *__slate_slot_569 = *__slate_slot_1773;
                                            }
                                        }
                                    }
                                }
                                if *__slate_slot_572 != -(1 as i32) {
                                    unsafe {
                                        (*pParse).iErr = *__slate_slot_569;
                                    }
                                }
                                return -(1 as i32);
                            }
                            if *__slate_slot_572 != -(1 as i32) {
                                unsafe {
                                    (*pParse).iErr = *__slate_slot_569;
                                }
                            }
                            return -(1 as i32);
                        }
                        if *__slate_slot_572 == -(2 as i32) {
                            *__slate_slot_569 = unsafe { (*pParse).iErr };
                            break '__join_153;
                        } else {
                            unsafe {
                                (*pParse).iErr = *__slate_slot_569;
                            }
                            return -(1 as i32);
                        }
                    }
                    if *__slate_slot_572 != -(1 as i32) {
                        unsafe {
                            (*pParse).iErr = *__slate_slot_569;
                        }
                    }
                    return -(1 as i32);
                }
                *__slate_slot_569 = unsafe { (*pParse).iErr };
                if (unsafe { (*pParse).nBlob }) != *__slate_slot_571 {
                    unsafe {
                        (*pParse).hasNonstd = ((1 as i32) as i8) as u8;
                    }
                }
            }
            jsonBlobChangePayloadSize(
                pParse,
                *__slate_slot_570,
                unsafe { (*pParse).nBlob }.wrapping_sub(*__slate_slot_571),
            );
            std::ptr::write(__slate_slot_1791, pParse);
            std::ptr::write(__slate_slot_1792, unsafe { (*(*__slate_slot_1791)).iDepth });
            std::ptr::write(
                __slate_slot_1793,
                ((((*__slate_slot_1792 as u32) as i32) - (1 as i32)) as i16) as u16,
            );
            unsafe {
                (*(*__slate_slot_1791)).iDepth = *__slate_slot_1793;
            }
            return (*__slate_slot_569).wrapping_add((1 as i32) as u32) as i32;
        }
    }
    // End of {...}
    // End of [...]
    // List separator
    // Object label/value separator
    // End of file
    // Syntax error
    // End switch(z[i])
    return unsafe { std::mem::zeroed() };
}

/// Parse a complete JSON string.  Return 0 on success or non-zero if there
/// are any errors.  If an error occurs, free all memory held by pParse,
/// but not pParse itself.
///
/// pParse must be initialized to an empty parse object prior to calling
/// this routine.
///
/// # Arguments
///
/// * `pParse` - Initialize and fill this JsonParse object
/// * `pCtx` - Report errors here
fn jsonConvertTextToBlob(mut pParse: *mut JsonParse, mut pCtx: *mut sqlite3_context) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut zJson: *const i8 = (unsafe { (*pParse).zJson }) as *const i8;
    i = jsonTranslateTextToBlob(pParse, (0 as i32) as u32);
    if (unsafe { (*pParse).oom }) != (0 as u8) {
        i = -(1 as i32);
    }
    if i > (0 as i32) {
        '__slate_break_1349: while (unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(jsonIsSpace.0) as *const i8 }.offset(
                    ((((unsafe { *unsafe { zJson.offset(i as isize) } }) as u8) as u32) as i32)
                        as isize,
                )
            }
        }) != (0 as i8)
        {
            let __v1849: i32 = i;
            let __v1850: i32 = __v1849 + (1 as i32);
            i = __v1850;
        }
        if (unsafe { *unsafe { zJson.offset(i as isize) } }) != (0 as i8) {
            let __v1851: i32 = i;
            let __v1852: i32 = __v1851 + json5Whitespace(unsafe { zJson.offset(i as isize) });
            i = __v1852;
            if (unsafe { *unsafe { zJson.offset(i as isize) } }) != (0 as i8) {
                if pCtx != std::ptr::null_mut::<sqlite3_context>() {
                    unsafe {
                        sqlite3_result_error(
                            pCtx,
                            (b"malformed JSON\0".as_ptr() as *mut i8) as *const i8,
                            -(1 as i32),
                        )
                    };
                }
                jsonParseReset(pParse);
                return 1 as i32;
            }
            unsafe {
                (*pParse).hasNonstd = ((1 as i32) as i8) as u8;
            }
        }
    }
    if i <= (0 as i32) {
        if pCtx != std::ptr::null_mut::<sqlite3_context>() {
            if (unsafe { (*pParse).oom }) != (0 as u8) {
                unsafe { sqlite3_result_error_nomem(pCtx) };
            } else {
                unsafe {
                    sqlite3_result_error(
                        pCtx,
                        (b"malformed JSON\0".as_ptr() as *mut i8) as *const i8,
                        -(1 as i32),
                    )
                };
            }
        }
        jsonParseReset(pParse);
        return 1 as i32;
    }
    return 0 as i32;
}

/// The input string pStr is a well-formed JSON text string.  Convert
/// this into the JSONB format and make it the return value of the
/// SQL function.
fn jsonReturnStringAsBlob(mut pStr: *mut JsonString) {
    let mut px: JsonParse = unsafe { std::mem::zeroed() };
    0 as i32;
    unsafe { memset(std::ptr::addr_of_mut!(px) as *mut (), 0 as i32, 72 as u64) };
    px.zJson = unsafe { (*pStr).zBuf };
    px.nJson = ((unsafe { (*pStr).nUsed }) as u32) as i32;
    px.db = unsafe { sqlite3_context_db_handle(unsafe { (*pStr).pCtx }) };
    jsonTranslateTextToBlob(std::ptr::addr_of_mut!(px), (0 as i32) as u32);
    if px.oom != (0 as u8) {
        unsafe { sqlite3DbFree(px.db, px.aBlob as *mut ()) };
        unsafe { sqlite3_result_error_nomem(unsafe { (*pStr).pCtx }) };
    } else {
        0 as i32;
        0 as i32;
        unsafe {
            sqlite3_result_blob(
                unsafe { (*pStr).pCtx },
                px.aBlob as *const (),
                px.nBlob as i32,
                unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        sqlite3RowSetClear as *const (),
                    )
                },
            )
        };
    }
}

/// This a helper routine for jsonbPayloadSize() and
/// jsonbPayloadSizeSemiInline().  This routine is called using tail
/// recursion to handle the (relatively uncommon) cases where the
/// the payload size is a 2, 4, or 8 byte integer.
///
/// # Arguments
///
/// * `pParse` - JSON parsing context
/// * `i` - Index of the node type-code
/// * `pSz` - Write payload size here
/// * `x` - pParse->aBlob[i]>>4
fn jsonbPayloadSizeWide(
    mut pParse: *const JsonParse,
    mut i: u32,
    mut pSz: *mut u32,
    mut x: u8,
) -> u32 {
    let mut sz: u32 = 0 as u32;
    let mut n: u32 = 0 as u32;
    0 as i32;
    0 as i32;
    0 as i32;
    if ((x as u32) as i32) == (13 as i32) {
        if i.wrapping_add((2 as i32) as u32) >= unsafe { (*pParse).nBlob } {
            unsafe {
                *pSz = (0 as i32) as u32;
            }
            return (0 as i32) as u32;
        }
        sz = (((((unsafe {
            *unsafe {
                unsafe { (*pParse).aBlob }.offset(i.wrapping_add((1 as i32) as u32) as isize)
            }
        }) as u32) as i32)
            << (8 as i32))
            + (((unsafe {
                *unsafe {
                    unsafe { (*pParse).aBlob }.offset(i.wrapping_add((2 as i32) as u32) as isize)
                }
            }) as u32) as i32)) as u32;
        n = (3 as i32) as u32;
    } else {
        if ((x as u32) as i32) == (14 as i32) {
            if i.wrapping_add((4 as i32) as u32) >= unsafe { (*pParse).nBlob } {
                unsafe {
                    *pSz = (0 as i32) as u32;
                }
                return (0 as i32) as u32;
            }
            sz = (((unsafe {
                *unsafe {
                    unsafe { (*pParse).aBlob }.offset(i.wrapping_add((1 as i32) as u32) as isize)
                }
            }) as u32)
                << (24 as i32))
                .wrapping_add(
                    ((((unsafe {
                        *unsafe {
                            unsafe { (*pParse).aBlob }
                                .offset(i.wrapping_add((2 as i32) as u32) as isize)
                        }
                    }) as u32) as i32)
                        << (16 as i32)) as u32,
                )
                .wrapping_add(
                    ((((unsafe {
                        *unsafe {
                            unsafe { (*pParse).aBlob }
                                .offset(i.wrapping_add((3 as i32) as u32) as isize)
                        }
                    }) as u32) as i32)
                        << (8 as i32)) as u32,
                )
                .wrapping_add(
                    (((unsafe {
                        *unsafe {
                            unsafe { (*pParse).aBlob }
                                .offset(i.wrapping_add((4 as i32) as u32) as isize)
                        }
                    }) as u32) as i32) as u32,
                );
            n = (5 as i32) as u32;
        } else {
            if i.wrapping_add((8 as i32) as u32) >= unsafe { (*pParse).nBlob }
                || (((unsafe {
                    *unsafe {
                        unsafe { (*pParse).aBlob }
                            .offset(i.wrapping_add((1 as i32) as u32) as isize)
                    }
                }) as u32) as i32)
                    != (0 as i32)
                || (((unsafe {
                    *unsafe {
                        unsafe { (*pParse).aBlob }
                            .offset(i.wrapping_add((2 as i32) as u32) as isize)
                    }
                }) as u32) as i32)
                    != (0 as i32)
                || (((unsafe {
                    *unsafe {
                        unsafe { (*pParse).aBlob }
                            .offset(i.wrapping_add((3 as i32) as u32) as isize)
                    }
                }) as u32) as i32)
                    != (0 as i32)
                || (((unsafe {
                    *unsafe {
                        unsafe { (*pParse).aBlob }
                            .offset(i.wrapping_add((4 as i32) as u32) as isize)
                    }
                }) as u32) as i32)
                    != (0 as i32)
            {
                unsafe {
                    *pSz = (0 as i32) as u32;
                }
                return (0 as i32) as u32;
            }
            sz = (((unsafe {
                *unsafe {
                    unsafe { (*pParse).aBlob }.offset(i.wrapping_add((5 as i32) as u32) as isize)
                }
            }) as u32)
                << (24 as i32))
                .wrapping_add(
                    ((((unsafe {
                        *unsafe {
                            unsafe { (*pParse).aBlob }
                                .offset(i.wrapping_add((6 as i32) as u32) as isize)
                        }
                    }) as u32) as i32)
                        << (16 as i32)) as u32,
                )
                .wrapping_add(
                    ((((unsafe {
                        *unsafe {
                            unsafe { (*pParse).aBlob }
                                .offset(i.wrapping_add((7 as i32) as u32) as isize)
                        }
                    }) as u32) as i32)
                        << (8 as i32)) as u32,
                )
                .wrapping_add(
                    (((unsafe {
                        *unsafe {
                            unsafe { (*pParse).aBlob }
                                .offset(i.wrapping_add((8 as i32) as u32) as isize)
                        }
                    }) as u32) as i32) as u32,
                );
            n = (9 as i32) as u32;
        }
    }
    {}
    {}
    {}
    {}
    // Quirks with -Os and gcov cause the NO_TEST line below to show up as
    // false-positive coverage miss.  The testcases() macros above are
    // sufficient to prove that the branch is in fact covered
    if ((i as u64) as i64) + ((sz as u64) as i64) + ((n as u64) as i64)
        > (((unsafe { (*pParse).nBlob }) as u64) as i64)
        && ((i as u64) as i64) + ((sz as u64) as i64) + ((n as u64) as i64)
            > ((unsafe { (*pParse).nBlob }.wrapping_sub((unsafe { (*pParse).delta }) as u32) as u64)
                as i64)
    {
        unsafe {
            *pSz = (0 as i32) as u32;
        }
        return (0 as i32) as u32;
    }
    unsafe {
        *pSz = sz;
    }
    return n;
}

/// This is the main routine for determining the size of a node in JSONB.
/// The jsonbPayloadSizeWide() above is a helper.  The two routines
/// jsonbPayloadSizeInline() and jsonbPayloadSizeSemiInline() below are
/// optional optimizations.  It is important to keep all these routines in
/// sync.  Agents reading this code:  Help us humans to remember that!
///
/// The byte at index i is a node type-code.  This routine
/// determines the payload size for that node and writes that
/// payload size in to *pSz.  It returns the offset from i to the
/// beginning of the payload.  Return 0 on error.
///
/// # Arguments
///
/// * `pParse` - JSON parsing context
/// * `i` - Index of the node type-code
/// * `pSz` - Write payload size here
fn jsonbPayloadSize(mut pParse: *const JsonParse, mut i: u32, mut pSz: *mut u32) -> u32 {
    let mut x: u8 = 0 as u8;
    let mut sz: u32 = 0 as u32;
    let mut n: u32 = 0 as u32;
    if i >= unsafe { (*pParse).nBlob } {
        unsafe {
            *pSz = (0 as i32) as u32;
        }
        return (0 as i32) as u32;
    } else {
        let __v1591: u8 = (((((unsafe { *unsafe { unsafe { (*pParse).aBlob }.offset(i as isize) } })
            as u32) as i32)
            >> (4 as i32)) as i8) as u8;
        x = __v1591;
        if ((__v1591 as u32) as i32) <= (11 as i32) {
            sz = x as u32;
            n = (1 as i32) as u32;
        } else {
            if ((x as u32) as i32) == (12 as i32) {
                if i.wrapping_add((1 as i32) as u32) >= unsafe { (*pParse).nBlob } {
                    unsafe {
                        *pSz = (0 as i32) as u32;
                    }
                    return (0 as i32) as u32;
                }
                sz = (unsafe {
                    *unsafe {
                        unsafe { (*pParse).aBlob }
                            .offset(i.wrapping_add((1 as i32) as u32) as isize)
                    }
                }) as u32;
                n = (2 as i32) as u32;
            } else {
                return jsonbPayloadSizeWide(pParse, i, pSz, x);
            }
        }
    }
    if ((i as u64) as i64) + ((sz as u64) as i64) + ((n as u64) as i64)
        > (((unsafe { (*pParse).nBlob }) as u64) as i64)
        && ((i as u64) as i64) + ((sz as u64) as i64) + ((n as u64) as i64)
            > ((unsafe { (*pParse).nBlob }.wrapping_sub((unsafe { (*pParse).delta }) as u32) as u64)
                as i64)
    {
        unsafe {
            *pSz = (0 as i32) as u32;
        }
        return (0 as i32) as u32;
    }
    unsafe {
        *pSz = sz;
    }
    return n;
}

/// A separate inline version of jsonbPayloadSize(), used in one particulary
/// performance-critical place.
///
/// # Arguments
///
/// * `pParse` - JSON parsing context
/// * `i` - Index of the node type-code
/// * `pSz` - Write payload size here
fn jsonbPayloadSizeInline(mut pParse: *const JsonParse, mut i: u32, mut pSz: *mut u32) -> u32 {
    let mut x: u8 = 0 as u8;
    let mut sz: u32 = 0 as u32;
    let mut n: u32 = 0 as u32;
    if i >= unsafe { (*pParse).nBlob } {
        unsafe {
            *pSz = (0 as i32) as u32;
        }
        return (0 as i32) as u32;
    } else {
        let __v1853: u8 = (((((unsafe { *unsafe { unsafe { (*pParse).aBlob }.offset(i as isize) } })
            as u32) as i32)
            >> (4 as i32)) as i8) as u8;
        x = __v1853;
        if ((__v1853 as u32) as i32) <= (11 as i32) {
            sz = x as u32;
            n = (1 as i32) as u32;
        } else {
            if ((x as u32) as i32) == (12 as i32) {
                if i.wrapping_add((1 as i32) as u32) >= unsafe { (*pParse).nBlob } {
                    unsafe {
                        *pSz = (0 as i32) as u32;
                    }
                    return (0 as i32) as u32;
                }
                sz = (unsafe {
                    *unsafe {
                        unsafe { (*pParse).aBlob }
                            .offset(i.wrapping_add((1 as i32) as u32) as isize)
                    }
                }) as u32;
                n = (2 as i32) as u32;
            } else {
                if ((x as u32) as i32) == (13 as i32) {
                    if i.wrapping_add((2 as i32) as u32) >= unsafe { (*pParse).nBlob } {
                        unsafe {
                            *pSz = (0 as i32) as u32;
                        }
                        return (0 as i32) as u32;
                    }
                    sz = (((((unsafe {
                        *unsafe {
                            unsafe { (*pParse).aBlob }
                                .offset(i.wrapping_add((1 as i32) as u32) as isize)
                        }
                    }) as u32) as i32)
                        << (8 as i32))
                        + (((unsafe {
                            *unsafe {
                                unsafe { (*pParse).aBlob }
                                    .offset(i.wrapping_add((2 as i32) as u32) as isize)
                            }
                        }) as u32) as i32)) as u32;
                    n = (3 as i32) as u32;
                } else {
                    if ((x as u32) as i32) == (14 as i32) {
                        if i.wrapping_add((4 as i32) as u32) >= unsafe { (*pParse).nBlob } {
                            unsafe {
                                *pSz = (0 as i32) as u32;
                            }
                            return (0 as i32) as u32;
                        }
                        sz = (((unsafe {
                            *unsafe {
                                unsafe { (*pParse).aBlob }
                                    .offset(i.wrapping_add((1 as i32) as u32) as isize)
                            }
                        }) as u32)
                            << (24 as i32))
                            .wrapping_add(
                                ((((unsafe {
                                    *unsafe {
                                        unsafe { (*pParse).aBlob }
                                            .offset(i.wrapping_add((2 as i32) as u32) as isize)
                                    }
                                }) as u32) as i32)
                                    << (16 as i32)) as u32,
                            )
                            .wrapping_add(
                                ((((unsafe {
                                    *unsafe {
                                        unsafe { (*pParse).aBlob }
                                            .offset(i.wrapping_add((3 as i32) as u32) as isize)
                                    }
                                }) as u32) as i32)
                                    << (8 as i32)) as u32,
                            )
                            .wrapping_add(
                                (((unsafe {
                                    *unsafe {
                                        unsafe { (*pParse).aBlob }
                                            .offset(i.wrapping_add((4 as i32) as u32) as isize)
                                    }
                                }) as u32) as i32) as u32,
                            );
                        n = (5 as i32) as u32;
                    } else {
                        if i.wrapping_add((8 as i32) as u32) >= unsafe { (*pParse).nBlob }
                            || (((unsafe {
                                *unsafe {
                                    unsafe { (*pParse).aBlob }
                                        .offset(i.wrapping_add((1 as i32) as u32) as isize)
                                }
                            }) as u32) as i32)
                                != (0 as i32)
                            || (((unsafe {
                                *unsafe {
                                    unsafe { (*pParse).aBlob }
                                        .offset(i.wrapping_add((2 as i32) as u32) as isize)
                                }
                            }) as u32) as i32)
                                != (0 as i32)
                            || (((unsafe {
                                *unsafe {
                                    unsafe { (*pParse).aBlob }
                                        .offset(i.wrapping_add((3 as i32) as u32) as isize)
                                }
                            }) as u32) as i32)
                                != (0 as i32)
                            || (((unsafe {
                                *unsafe {
                                    unsafe { (*pParse).aBlob }
                                        .offset(i.wrapping_add((4 as i32) as u32) as isize)
                                }
                            }) as u32) as i32)
                                != (0 as i32)
                        {
                            unsafe {
                                *pSz = (0 as i32) as u32;
                            }
                            return (0 as i32) as u32;
                        }
                        sz = (((unsafe {
                            *unsafe {
                                unsafe { (*pParse).aBlob }
                                    .offset(i.wrapping_add((5 as i32) as u32) as isize)
                            }
                        }) as u32)
                            << (24 as i32))
                            .wrapping_add(
                                ((((unsafe {
                                    *unsafe {
                                        unsafe { (*pParse).aBlob }
                                            .offset(i.wrapping_add((6 as i32) as u32) as isize)
                                    }
                                }) as u32) as i32)
                                    << (16 as i32)) as u32,
                            )
                            .wrapping_add(
                                ((((unsafe {
                                    *unsafe {
                                        unsafe { (*pParse).aBlob }
                                            .offset(i.wrapping_add((7 as i32) as u32) as isize)
                                    }
                                }) as u32) as i32)
                                    << (8 as i32)) as u32,
                            )
                            .wrapping_add(
                                (((unsafe {
                                    *unsafe {
                                        unsafe { (*pParse).aBlob }
                                            .offset(i.wrapping_add((8 as i32) as u32) as isize)
                                    }
                                }) as u32) as i32) as u32,
                            );
                        n = (9 as i32) as u32;
                    }
                }
            }
        }
    }
    if ((i as u64) as i64) + ((sz as u64) as i64) + ((n as u64) as i64)
        > (((unsafe { (*pParse).nBlob }) as u64) as i64)
        && ((i as u64) as i64) + ((sz as u64) as i64) + ((n as u64) as i64)
            > ((unsafe { (*pParse).nBlob }.wrapping_sub((unsafe { (*pParse).delta }) as u32) as u64)
                as i64)
    {
        unsafe {
            *pSz = (0 as i32) as u32;
        }
        return (0 as i32) as u32;
    }
    unsafe {
        *pSz = sz;
    }
    return n;
}

/// A separate inline version of jsonbPayloadSize() that implements the more
/// common paths inline but then calls out to a subroutine for the uncommon
/// paths.
///
///    *   It is guaranteed that the i parameter is a valid index for
///        pParse->aBlob[].  There is no possibility of running of the end
///        of the allocation.
///
///    *   Large sizes (greater than 255) are uncommon and are handled
///        by a subroutine call to the jsonbPayloadSizeWide().
///
/// # Arguments
///
/// * `pParse` - JSON parsing context
/// * `i` - Index of the node type-code
/// * `pSz` - Write payload size here
fn jsonbPayloadSizeSemiInline(mut pParse: *const JsonParse, mut i: u32, mut pSz: *mut u32) -> u32 {
    let mut x: u8 = 0 as u8;
    let mut sz: u32 = 0 as u32;
    let mut n: u32 = 0 as u32;
    0 as i32;
    let __v1854: u8 = (((((unsafe { *unsafe { unsafe { (*pParse).aBlob }.offset(i as isize) } })
        as u32) as i32)
        >> (4 as i32)) as i8) as u8;
    x = __v1854;
    if ((__v1854 as u32) as i32) <= (11 as i32) {
        sz = x as u32;
        n = (1 as i32) as u32;
    } else {
        if ((x as u32) as i32) == (12 as i32) {
            if i.wrapping_add((1 as i32) as u32) >= unsafe { (*pParse).nBlob } {
                unsafe {
                    *pSz = (0 as i32) as u32;
                }
                return (0 as i32) as u32;
            }
            sz = (unsafe {
                *unsafe {
                    unsafe { (*pParse).aBlob }.offset(i.wrapping_add((1 as i32) as u32) as isize)
                }
            }) as u32;
            n = (2 as i32) as u32;
        } else {
            return jsonbPayloadSizeWide(pParse, i, pSz, x);
        }
    }
    if ((i as u64) as i64) + ((sz as u64) as i64) + ((n as u64) as i64)
        > (((unsafe { (*pParse).nBlob }) as u64) as i64)
        && ((i as u64) as i64) + ((sz as u64) as i64) + ((n as u64) as i64)
            > ((unsafe { (*pParse).nBlob }.wrapping_sub((unsafe { (*pParse).delta }) as u32) as u64)
                as i64)
    {
        unsafe {
            *pSz = (0 as i32) as u32;
        }
        return (0 as i32) as u32;
    }
    unsafe {
        *pSz = sz;
    }
    return n;
}

/// Translate the binary JSONB representation of JSON beginning at
/// pParse->aBlob[i] into a JSON text string.  Append the JSON
/// text onto the end of pOut.  Return the index in pParse->aBlob[]
/// of the first byte past the end of the element that is translated.
///
/// If an error is detected in the BLOB input, the pOut->eErr flag
/// might get set to JSTRING_MALFORMED.  But not all BLOB input errors
/// are detected.  So a malformed JSONB input might either result
/// in an error, or in incorrect JSON.
///
/// The pOut->eErr JSTRING_OOM flag is set on a OOM.
///
/// # Arguments
///
/// * `pParse` - the complete parse of the JSON
/// * `i` - Start rendering at this index
/// * `pOut` - Write JSON here
fn jsonTranslateBlobToText(
    mut pParse: *mut JsonParse,
    mut i: u32,
    mut pOut: *mut JsonString,
) -> u32 {
    let mut __slate_storage_1582: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1582: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1582) as *mut u8;
    let mut __slate_storage_1581: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1581: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1581) as *mut u8;
    let mut __slate_storage_1580: std::mem::MaybeUninit<*mut JsonString> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1580: *mut *mut JsonString =
        std::ptr::addr_of_mut!(__slate_storage_1580) as *mut *mut JsonString;
    let mut __slate_storage_1579: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1579: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1579) as *mut u8;
    let mut __slate_storage_1578: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1578: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1578) as *mut u8;
    let mut __slate_storage_1577: std::mem::MaybeUninit<*mut JsonString> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1577: *mut *mut JsonString =
        std::ptr::addr_of_mut!(__slate_storage_1577) as *mut *mut JsonString;
    let mut __slate_storage_1576: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1576: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1576) as *mut u16;
    let mut __slate_storage_1575: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1575: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1575) as *mut u16;
    let mut __slate_storage_1574: std::mem::MaybeUninit<*mut JsonParse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1574: *mut *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_1574) as *mut *mut JsonParse;
    let mut __slate_storage_1573: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1573: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1573) as *mut i32;
    let mut __slate_storage_1572: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1572: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1572) as *mut i32;
    let mut __slate_storage_1571: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1571: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1571) as *mut u16;
    let mut __slate_storage_1570: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1570: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1570) as *mut u16;
    let mut __slate_storage_1569: std::mem::MaybeUninit<*mut JsonParse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1569: *mut *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_1569) as *mut *mut JsonParse;
    let mut __slate_storage_634: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_634: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_634) as *mut i32;
    let mut __slate_storage_1568: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1568: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1568) as *mut u8;
    let mut __slate_storage_1567: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1567: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1567) as *mut u8;
    let mut __slate_storage_1566: std::mem::MaybeUninit<*mut JsonString> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1566: *mut *mut JsonString =
        std::ptr::addr_of_mut!(__slate_storage_1566) as *mut *mut JsonString;
    let mut __slate_storage_1565: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1565: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1565) as *mut u16;
    let mut __slate_storage_1564: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1564: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1564) as *mut u16;
    let mut __slate_storage_1563: std::mem::MaybeUninit<*mut JsonParse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1563: *mut *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_1563) as *mut *mut JsonParse;
    let mut __slate_storage_1562: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1562: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1562) as *mut u16;
    let mut __slate_storage_1561: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1561: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1561) as *mut u16;
    let mut __slate_storage_1560: std::mem::MaybeUninit<*mut JsonParse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1560: *mut *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_1560) as *mut *mut JsonParse;
    let mut __slate_storage_1559: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1559: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1559) as *mut u32;
    let mut __slate_storage_1558: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1558: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1558) as *mut u32;
    let mut __slate_storage_1557: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1557: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1557) as *mut *const i8;
    let mut __slate_storage_1556: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1556: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1556) as *mut *const i8;
    let mut __slate_storage_1555: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1555: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1555) as *mut u32;
    let mut __slate_storage_1554: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1554: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1554) as *mut u32;
    let mut __slate_storage_1553: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1553: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1553) as *mut *const i8;
    let mut __slate_storage_1552: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1552: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1552) as *mut *const i8;
    let mut __slate_storage_1551: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1551: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1551) as *mut u8;
    let mut __slate_storage_1550: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1550: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1550) as *mut u8;
    let mut __slate_storage_1549: std::mem::MaybeUninit<*mut JsonString> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1549: *mut *mut JsonString =
        std::ptr::addr_of_mut!(__slate_storage_1549) as *mut *mut JsonString;
    let mut __slate_storage_1548: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1548: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1548) as *mut u32;
    let mut __slate_storage_1547: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1547: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1547) as *mut u32;
    let mut __slate_storage_1546: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1546: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1546) as *mut *const i8;
    let mut __slate_storage_1545: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1545: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1545) as *mut *const i8;
    let mut __slate_storage_1544: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1544: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1544) as *mut u32;
    let mut __slate_storage_1543: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1543: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1543) as *mut u32;
    let mut __slate_storage_1542: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1542: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1542) as *mut *const i8;
    let mut __slate_storage_1541: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1541: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1541) as *mut *const i8;
    let mut __slate_storage_1540: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1540: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1540) as *mut u8;
    let mut __slate_storage_1539: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1539: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1539) as *mut u8;
    let mut __slate_storage_1538: std::mem::MaybeUninit<*mut JsonString> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1538: *mut *mut JsonString =
        std::ptr::addr_of_mut!(__slate_storage_1538) as *mut *mut JsonString;
    let mut __slate_storage_1537: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1537: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1537) as *mut u8;
    let mut __slate_storage_1536: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1536: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1536) as *mut u8;
    let mut __slate_storage_1535: std::mem::MaybeUninit<*mut JsonString> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1535: *mut *mut JsonString =
        std::ptr::addr_of_mut!(__slate_storage_1535) as *mut *mut JsonString;
    let mut __slate_storage_1534: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1534: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1534) as *mut u32;
    let mut __slate_storage_1533: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1533: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1533) as *mut u32;
    let mut __slate_storage_1532: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1532: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1532) as *mut *const i8;
    let mut __slate_storage_1531: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1531: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1531) as *mut *const i8;
    let mut __slate_storage_1530: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1530: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1530) as *mut bool;
    let mut __slate_storage_1529: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1529: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1529) as *mut u32;
    let mut __slate_storage_1528: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1528: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1528) as *mut u32;
    let mut __slate_storage_1527: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1527: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1527) as *mut *const i8;
    let mut __slate_storage_1526: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1526: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1526) as *mut *const i8;
    let mut __slate_storage_1525: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1525: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1525) as *mut u32;
    let mut __slate_storage_1524: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1524: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1524) as *mut u32;
    let mut __slate_storage_1523: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1523: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1523) as *mut *const i8;
    let mut __slate_storage_1522: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1522: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1522) as *mut *const i8;
    let mut __slate_storage_1521: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1521: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1521) as *mut u32;
    let mut __slate_storage_1520: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1520: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1520) as *mut u32;
    let mut __slate_storage_633: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_633: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_633) as *mut u32;
    let mut __slate_storage_632: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_632: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_632) as *mut u32;
    let mut __slate_storage_631: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_631: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_631) as *mut *const i8;
    let mut __slate_storage_1519: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1519: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1519) as *mut u64;
    let mut __slate_storage_1518: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1518: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1518) as *mut u64;
    let mut __slate_storage_1517: std::mem::MaybeUninit<*mut JsonString> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1517: *mut *mut JsonString =
        std::ptr::addr_of_mut!(__slate_storage_1517) as *mut *mut JsonString;
    let mut __slate_storage_1516: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1516: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1516) as *mut bool;
    let mut __slate_storage_1515: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1515: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1515) as *mut u32;
    let mut __slate_storage_1514: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1514: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1514) as *mut u32;
    let mut __slate_storage_630: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_630: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_630) as *mut *const i8;
    let mut __slate_storage_629: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_629: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_629) as *mut u32;
    let mut __slate_storage_1510: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1510: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1510) as *mut u32;
    let mut __slate_storage_1509: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1509: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1509) as *mut u32;
    let mut __slate_storage_1513: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1513: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1513) as *mut u8;
    let mut __slate_storage_1512: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1512: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1512) as *mut u8;
    let mut __slate_storage_1511: std::mem::MaybeUninit<*mut JsonString> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1511: *mut *mut JsonString =
        std::ptr::addr_of_mut!(__slate_storage_1511) as *mut *mut JsonString;
    let mut __slate_storage_1506: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1506: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1506) as *mut u32;
    let mut __slate_storage_1505: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1505: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1505) as *mut u32;
    let mut __slate_storage_1508: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1508: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1508) as *mut u32;
    let mut __slate_storage_1507: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1507: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1507) as *mut u32;
    let mut __slate_storage_628: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_628: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_628) as *mut i32;
    let mut __slate_storage_627: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_627: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_627) as *mut *const i8;
    let mut __slate_storage_626: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_626: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_626) as *mut u64;
    let mut __slate_storage_625: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_625: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_625) as *mut u32;
    let mut __slate_storage_1504: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1504: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1504) as *mut u8;
    let mut __slate_storage_1503: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1503: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1503) as *mut u8;
    let mut __slate_storage_1502: std::mem::MaybeUninit<*mut JsonString> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1502: *mut *mut JsonString =
        std::ptr::addr_of_mut!(__slate_storage_1502) as *mut *mut JsonString;
    let mut __slate_storage_624: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_624: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_624) as *mut u32;
    let mut __slate_storage_623: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_623: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_623) as *mut u32;
    let mut __slate_storage_622: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_622: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_622) as *mut u32;
    let mut __slate_storage_621: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_621: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_621) as *mut u32;
    unsafe {
        *__slate_slot_622 = jsonbPayloadSizeInline(
            pParse as *const JsonParse,
            i,
            std::ptr::addr_of_mut!(*__slate_slot_621),
        );
        if *__slate_slot_622 == ((0 as i32) as u32) {
            std::ptr::write(__slate_slot_1502, pOut);
            std::ptr::write(__slate_slot_1503, unsafe { (*(*__slate_slot_1502)).eErr });
            std::ptr::write(
                __slate_slot_1504,
                ((((*__slate_slot_1503 as u32) as i32) | (2 as i32)) as i8) as u8,
            );
            unsafe {
                (*(*__slate_slot_1502)).eErr = *__slate_slot_1504;
            }
            return unsafe { (*pParse).nBlob }.wrapping_add((1 as i32) as u32);
        } else {
            '__join_0: {
                '__join_1: {
                    '__join_83: {
                        let __t1: i32 = (((unsafe {
                            *unsafe { unsafe { (*pParse).aBlob }.offset(i as isize) }
                        }) as u32) as i32)
                            & (15 as i32);
                        if __t1 == (0 as i32) {
                            jsonAppendRawNZ(
                                pOut,
                                (b"null\0".as_ptr() as *mut i8) as *const i8,
                                (4 as i32) as u32,
                            );
                            return i.wrapping_add((1 as i32) as u32);
                        } else {
                            if __t1 == (1 as i32) {
                                jsonAppendRawNZ(
                                    pOut,
                                    (b"true\0".as_ptr() as *mut i8) as *const i8,
                                    (4 as i32) as u32,
                                );
                                return i.wrapping_add((1 as i32) as u32);
                            } else {
                                if __t1 == (2 as i32) {
                                    jsonAppendRawNZ(
                                        pOut,
                                        (b"false\0".as_ptr() as *mut i8) as *const i8,
                                        (5 as i32) as u32,
                                    );
                                    return i.wrapping_add((1 as i32) as u32);
                                } else {
                                    if __t1 == (3 as i32) {
                                        break '__join_83;
                                    } else {
                                        if __t1 == (5 as i32) {
                                            break '__join_83;
                                        } else {
                                            if __t1 == (4 as i32) {
                                                // Integer literal in hexadecimal notation
                                                std::ptr::write(
                                                    __slate_slot_625,
                                                    (2 as i32) as u32,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_626,
                                                    ((0 as i32) as i64) as u64,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_627,
                                                    (unsafe {
                                                        unsafe { (*pParse).aBlob }.offset(
                                                            i.wrapping_add(*__slate_slot_622)
                                                                as isize,
                                                        )
                                                    })
                                                        as *const i8,
                                                );
                                                std::ptr::write(__slate_slot_628, 0 as i32);
                                                if *__slate_slot_621 == ((0 as i32) as u32) {
                                                    break '__join_1;
                                                } else {
                                                    if ((unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_627)
                                                                .offset((0 as i32) as isize)
                                                        }
                                                    })
                                                        as i32)
                                                        == (45 as i32)
                                                    {
                                                        jsonAppendChar(pOut, (45 as i32) as i8);
                                                        std::ptr::write(
                                                            __slate_slot_1505,
                                                            *__slate_slot_625,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1506,
                                                            (*__slate_slot_1505)
                                                                .wrapping_add((1 as i32) as u32),
                                                        );
                                                        *__slate_slot_625 = *__slate_slot_1506;
                                                    } else {
                                                        if ((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_627)
                                                                    .offset((0 as i32) as isize)
                                                            }
                                                        })
                                                            as i32)
                                                            == (43 as i32)
                                                        {
                                                            std::ptr::write(
                                                                __slate_slot_1507,
                                                                *__slate_slot_625,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1508,
                                                                (*__slate_slot_1507).wrapping_add(
                                                                    (1 as i32) as u32,
                                                                ),
                                                            );
                                                            *__slate_slot_625 = *__slate_slot_1508;
                                                        }
                                                    }
                                                    '__join_69: {
                                                        loop {
                                                            if *__slate_slot_625 < *__slate_slot_621
                                                            {
                                                                if !((((unsafe {
                                                                    *unsafe {
                                                                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(((((unsafe { *unsafe { (*__slate_slot_627).offset(*__slate_slot_625 as isize) } }) as u8) as u32) as i32) as isize)
                                                                    }
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    & (8 as i32)
                                                                    != (0 as i32))
                                                                {
                                                                    break;
                                                                } else {
                                                                    if *__slate_slot_626
                                                                        >> (60 as i32)
                                                                        != (((0 as i32) as i64)
                                                                            as u64)
                                                                    {
                                                                        *__slate_slot_628 =
                                                                            1 as i32;
                                                                    } else {
                                                                        *__slate_slot_626 = (*__slate_slot_626).wrapping_mul(((16 as i32) as i64) as u64).wrapping_add(((((unsafe { sqlite3HexToInt((unsafe { *unsafe { (*__slate_slot_627).offset(*__slate_slot_625 as isize) } }) as i32) }) as u32) as i32) as i64) as u64);
                                                                    }
                                                                    std::ptr::write(
                                                                        __slate_slot_1509,
                                                                        *__slate_slot_625,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_1510,
                                                                        (*__slate_slot_1509)
                                                                            .wrapping_add(
                                                                                (1 as i32) as u32,
                                                                            ),
                                                                    );
                                                                    *__slate_slot_625 =
                                                                        *__slate_slot_1510;
                                                                }
                                                            } else {
                                                                break '__join_69;
                                                            }
                                                        }
                                                        std::ptr::write(__slate_slot_1511, pOut);
                                                        std::ptr::write(
                                                            __slate_slot_1512,
                                                            unsafe { (*(*__slate_slot_1511)).eErr },
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1513,
                                                            ((((*__slate_slot_1512 as u32) as i32)
                                                                | (2 as i32))
                                                                as i8)
                                                                as u8,
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_1511)).eErr =
                                                                *__slate_slot_1513;
                                                        }
                                                    }
                                                    unsafe {
                                                        jsonPrintf(
                                                            100 as i32,
                                                            pOut,
                                                            (if *__slate_slot_628 != (0 as i32) {
                                                                b"9.0e999\0".as_ptr() as *mut i8
                                                            } else {
                                                                b"%llu\0".as_ptr() as *mut i8
                                                            })
                                                                as *const i8,
                                                            *__slate_slot_626,
                                                        )
                                                    };
                                                    break '__join_0;
                                                }
                                            } else {
                                                if __t1 == (6 as i32) {
                                                    // Float literal missing digits beside "."
                                                    std::ptr::write(
                                                        __slate_slot_629,
                                                        (0 as i32) as u32,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_630,
                                                        (unsafe {
                                                            unsafe { (*pParse).aBlob }.offset(
                                                                i.wrapping_add(*__slate_slot_622)
                                                                    as isize,
                                                            )
                                                        })
                                                            as *const i8,
                                                    );
                                                    if *__slate_slot_621 == ((0 as i32) as u32) {
                                                        break '__join_1;
                                                    } else {
                                                        if ((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_630)
                                                                    .offset((0 as i32) as isize)
                                                            }
                                                        })
                                                            as i32)
                                                            == (45 as i32)
                                                        {
                                                            jsonAppendChar(pOut, (45 as i32) as i8);
                                                            if *__slate_slot_621
                                                                <= ((1 as i32) as u32)
                                                            {
                                                                break '__join_1;
                                                            } else {
                                                                *__slate_slot_629 =
                                                                    (1 as i32) as u32;
                                                            }
                                                        }
                                                        if ((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_630).offset(
                                                                    *__slate_slot_629 as isize,
                                                                )
                                                            }
                                                        })
                                                            as i32)
                                                            == (46 as i32)
                                                        {
                                                            jsonAppendChar(pOut, (48 as i32) as i8);
                                                        }
                                                        loop {
                                                            if *__slate_slot_629 < *__slate_slot_621
                                                            {
                                                                jsonAppendChar(pOut, unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_630).offset(
                                                                            *__slate_slot_629
                                                                                as isize,
                                                                        )
                                                                    }
                                                                });
                                                                if ((unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_630).offset(
                                                                            *__slate_slot_629
                                                                                as isize,
                                                                        )
                                                                    }
                                                                })
                                                                    as i32)
                                                                    == (46 as i32)
                                                                    && ((*__slate_slot_629)
                                                                        .wrapping_add(
                                                                            (1 as i32) as u32,
                                                                        )
                                                                        == *__slate_slot_621
                                                                        || !((((unsafe {
                                                                            *unsafe {
                                                                                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(((((unsafe { *unsafe { (*__slate_slot_630).offset((*__slate_slot_629).wrapping_add((1 as i32) as u32) as isize) } }) as u8) as u32) as i32) as isize)
                                                                            }
                                                                        })
                                                                            as u32)
                                                                            as i32)
                                                                            & (4 as i32)
                                                                            != (0 as i32)))
                                                                {
                                                                    jsonAppendChar(
                                                                        pOut,
                                                                        (48 as i32) as i8,
                                                                    );
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_1514,
                                                                    *__slate_slot_629,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1515,
                                                                    (*__slate_slot_1514)
                                                                        .wrapping_add(
                                                                            (1 as i32) as u32,
                                                                        ),
                                                                );
                                                                *__slate_slot_629 =
                                                                    *__slate_slot_1515;
                                                            } else {
                                                                break '__join_0;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if __t1 == (7 as i32) {
                                                    } else {
                                                        if __t1 == (8 as i32) {
                                                        } else {
                                                            if __t1 == (9 as i32) {
                                                                std::ptr::write(
                                                                    __slate_slot_633,
                                                                    *__slate_slot_621,
                                                                );
                                                                *__slate_slot_631 = (unsafe {
                                                                    unsafe { (*pParse).aBlob }
                                                                        .offset(i.wrapping_add(
                                                                            *__slate_slot_622,
                                                                        )
                                                                            as isize)
                                                                })
                                                                    as *const i8;
                                                                jsonAppendChar(
                                                                    pOut,
                                                                    (34 as i32) as i8,
                                                                );
                                                                '__join_21: {
                                                                    loop {
                                                                        if *__slate_slot_633
                                                                            > ((0 as i32) as u32)
                                                                        {
                                                                            *__slate_slot_632 =
                                                                                (0 as i32) as u32;
                                                                            loop {
                                                                                if *__slate_slot_632 < *__slate_slot_633 && ((unsafe { *unsafe { unsafe { std::ptr::addr_of!(jsonIsOk.0) as *const i8 }.offset(((((unsafe { *unsafe { (*__slate_slot_631).offset(*__slate_slot_632 as isize) } }) as u8) as u32) as i32) as isize) } }) != (0 as i8) || ((unsafe { *unsafe { (*__slate_slot_631).offset(*__slate_slot_632 as isize) } }) as i32) == (39 as i32)) {
std::ptr::write(__slate_slot_1520, *__slate_slot_632);
std::ptr::write(__slate_slot_1521, (*__slate_slot_1520).wrapping_add((1 as i32) as u32));
*__slate_slot_632 = *__slate_slot_1521;
} else {
break;
}
                                                                            }
                                                                            if *__slate_slot_632
                                                                                > ((0 as i32)
                                                                                    as u32)
                                                                            {
                                                                                jsonAppendRawNZ(pOut, *__slate_slot_631, *__slate_slot_632);
                                                                                if *__slate_slot_632 >= *__slate_slot_633 {
break '__join_21;
} else {
std::ptr::write(__slate_slot_1522, *__slate_slot_631);
std::ptr::write(__slate_slot_1523, unsafe { (*__slate_slot_1522).offset(*__slate_slot_632 as isize) });
*__slate_slot_631 = *__slate_slot_1523;
std::ptr::write(__slate_slot_1524, *__slate_slot_633);
std::ptr::write(__slate_slot_1525, (*__slate_slot_1524).wrapping_sub(*__slate_slot_632));
*__slate_slot_633 = *__slate_slot_1525;
}
                                                                            }
                                                                            if ((unsafe {
                                                                                *unsafe {
                                                                                    (*__slate_slot_631).offset((0 as i32) as isize)
                                                                                }
                                                                            })
                                                                                as i32)
                                                                                == (34 as i32)
                                                                            {
                                                                                jsonAppendRawNZ(pOut, (b"\\\"\0".as_ptr() as *mut i8) as *const i8, (2 as i32) as u32);
                                                                                std::ptr::write(__slate_slot_1526, *__slate_slot_631);
                                                                                std::ptr::write(__slate_slot_1527, unsafe { (*__slate_slot_1526).offset((1 as i32) as isize) });
                                                                                *__slate_slot_631 = *__slate_slot_1527;
                                                                                std::ptr::write(__slate_slot_1528, *__slate_slot_633);
                                                                                std::ptr::write(__slate_slot_1529, (*__slate_slot_1528).wrapping_sub((1 as i32) as u32));
                                                                                *__slate_slot_633 = *__slate_slot_1529;
                                                                            } else {
                                                                                if ((unsafe {
                                                                                    *unsafe {
                                                                                        (*__slate_slot_631).offset((0 as i32) as isize)
                                                                                    }
                                                                                })
                                                                                    as i32)
                                                                                    <= (31 as i32)
                                                                                {
                                                                                    if unsafe {
                                                                                        (*pOut)
                                                                                            .nUsed
                                                                                    }
                                                                                    .wrapping_add(
                                                                                        ((7 as i32)
                                                                                            as i64)
                                                                                            as u64,
                                                                                    ) > unsafe {
                                                                                        (*pOut)
                                                                                            .nAlloc
                                                                                    } {
                                                                                        *__slate_slot_1530 = jsonStringGrow(pOut, (7 as i32) as u32) != (0 as i32);
                                                                                    } else {
                                                                                        *__slate_slot_1530 = false as bool;
                                                                                    }
                                                                                    if *__slate_slot_1530 {
break '__join_21;
} else {
jsonAppendControlChar(pOut, (unsafe { *unsafe { (*__slate_slot_631).offset((0 as i32) as isize) } }) as u8);
std::ptr::write(__slate_slot_1531, *__slate_slot_631);
std::ptr::write(__slate_slot_1532, unsafe { (*__slate_slot_1531).offset((1 as i32) as isize) });
*__slate_slot_631 = *__slate_slot_1532;
std::ptr::write(__slate_slot_1533, *__slate_slot_633);
std::ptr::write(__slate_slot_1534, (*__slate_slot_1533).wrapping_sub((1 as i32) as u32));
*__slate_slot_633 = *__slate_slot_1534;
}
                                                                                } else {
                                                                                    0 as i32;
                                                                                    0 as i32;
                                                                                    if *__slate_slot_633 < ((2 as i32) as u32) {
break;
} else {
let __t0: i32 = (((unsafe { *unsafe { (*__slate_slot_631).offset((1 as i32) as isize) } }) as u8) as u32) as i32;
if __t0 == (39 as i32) {
jsonAppendChar(pOut, (39 as i32) as i8);
} else {
if __t0 == (118 as i32) {
jsonAppendRawNZ(pOut, (b"\\u000b\0".as_ptr() as *mut i8) as *const i8, (6 as i32) as u32);
} else {
if __t0 == (120 as i32) {
if *__slate_slot_633 < ((4 as i32) as u32) {
std::ptr::write(__slate_slot_1538, pOut);
std::ptr::write(__slate_slot_1539, unsafe { (*(*__slate_slot_1538)).eErr });
std::ptr::write(__slate_slot_1540, ((((*__slate_slot_1539 as u32) as i32) | (2 as i32)) as i8) as u8);
unsafe {
(*(*__slate_slot_1538)).eErr = *__slate_slot_1540;
}
*__slate_slot_633 = (2 as i32) as u32;
} else {
jsonAppendRawNZ(pOut, (b"\\u00\0".as_ptr() as *mut i8) as *const i8, (4 as i32) as u32);
jsonAppendRawNZ(pOut, unsafe { (*__slate_slot_631).offset((2 as i32) as isize) }, (2 as i32) as u32);
std::ptr::write(__slate_slot_1541, *__slate_slot_631);
std::ptr::write(__slate_slot_1542, unsafe { (*__slate_slot_1541).offset((2 as i32) as isize) });
*__slate_slot_631 = *__slate_slot_1542;
std::ptr::write(__slate_slot_1543, *__slate_slot_633);
std::ptr::write(__slate_slot_1544, (*__slate_slot_1543).wrapping_sub((2 as i32) as u32));
*__slate_slot_633 = *__slate_slot_1544;
}
} else {
if __t0 == (48 as i32) {
jsonAppendRawNZ(pOut, (b"\\u0000\0".as_ptr() as *mut i8) as *const i8, (6 as i32) as u32);
} else {
if __t0 == (13 as i32) {
if *__slate_slot_633 > ((2 as i32) as u32) && ((unsafe { *unsafe { (*__slate_slot_631).offset((2 as i32) as isize) } }) as i32) == (10 as i32) {
std::ptr::write(__slate_slot_1545, *__slate_slot_631);
std::ptr::write(__slate_slot_1546, unsafe { (*__slate_slot_1545).offset((1 as i32) as isize) });
*__slate_slot_631 = *__slate_slot_1546;
std::ptr::write(__slate_slot_1547, *__slate_slot_633);
std::ptr::write(__slate_slot_1548, (*__slate_slot_1547).wrapping_sub((1 as i32) as u32));
*__slate_slot_633 = *__slate_slot_1548;
}
} else {
if __t0 == (10 as i32) {
} else {
if __t0 == (226 as i32) {
if *__slate_slot_633 < ((4 as i32) as u32) || (128 as i32) != ((((unsafe { *unsafe { (*__slate_slot_631).offset((2 as i32) as isize) } }) as u8) as u32) as i32) || (168 as i32) != ((((unsafe { *unsafe { (*__slate_slot_631).offset((3 as i32) as isize) } }) as u8) as u32) as i32) && (169 as i32) != ((((unsafe { *unsafe { (*__slate_slot_631).offset((3 as i32) as isize) } }) as u8) as u32) as i32) {
std::ptr::write(__slate_slot_1549, pOut);
std::ptr::write(__slate_slot_1550, unsafe { (*(*__slate_slot_1549)).eErr });
std::ptr::write(__slate_slot_1551, ((((*__slate_slot_1550 as u32) as i32) | (2 as i32)) as i8) as u8);
unsafe {
(*(*__slate_slot_1549)).eErr = *__slate_slot_1551;
}
*__slate_slot_633 = (2 as i32) as u32;
} else {
// '\' followed by either U+2028 or U+2029 is ignored as
// whitespace.  Note that in UTF8, U+2028 is 0xe2 0x80 0x29.
// U+2029 is the same except for the last byte
std::ptr::write(__slate_slot_1552, *__slate_slot_631);
std::ptr::write(__slate_slot_1553, unsafe { (*__slate_slot_1552).offset((2 as i32) as isize) });
*__slate_slot_631 = *__slate_slot_1553;
std::ptr::write(__slate_slot_1554, *__slate_slot_633);
std::ptr::write(__slate_slot_1555, (*__slate_slot_1554).wrapping_sub((2 as i32) as u32));
*__slate_slot_633 = *__slate_slot_1555;
}
} else {
jsonAppendRawNZ(pOut, *__slate_slot_631, (2 as i32) as u32);
}
}
}
}
}
}
}
0 as i32;
std::ptr::write(__slate_slot_1556, *__slate_slot_631);
std::ptr::write(__slate_slot_1557, unsafe { (*__slate_slot_1556).offset((2 as i32) as isize) });
*__slate_slot_631 = *__slate_slot_1557;
std::ptr::write(__slate_slot_1558, *__slate_slot_633);
std::ptr::write(__slate_slot_1559, (*__slate_slot_1558).wrapping_sub((2 as i32) as u32));
*__slate_slot_633 = *__slate_slot_1559;
}
                                                                                }
                                                                            }
                                                                        } else {
                                                                            break '__join_21;
                                                                        }
                                                                    }
                                                                    std::ptr::write(
                                                                        __slate_slot_1535,
                                                                        pOut,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_1536,
                                                                        unsafe {
                                                                            (*(*__slate_slot_1535))
                                                                                .eErr
                                                                        },
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_1537,
                                                                        ((((*__slate_slot_1536
                                                                            as u32)
                                                                            as i32)
                                                                            | (2 as i32))
                                                                            as i8)
                                                                            as u8,
                                                                    );
                                                                    unsafe {
                                                                        (*(*__slate_slot_1535))
                                                                            .eErr =
                                                                            *__slate_slot_1537;
                                                                    }
                                                                }
                                                                jsonAppendChar(
                                                                    pOut,
                                                                    (34 as i32) as i8,
                                                                );
                                                                break '__join_0;
                                                            } else {
                                                                if __t1 == (10 as i32) {
                                                                    jsonAppendString(
                                                                        pOut,
                                                                        (unsafe {
                                                                            unsafe {
                                                                                (*pParse).aBlob
                                                                            }
                                                                            .offset(i.wrapping_add(
                                                                                *__slate_slot_622,
                                                                            )
                                                                                as isize)
                                                                        })
                                                                            as *const i8,
                                                                        *__slate_slot_621,
                                                                    );
                                                                    break '__join_0;
                                                                } else {
                                                                    if __t1 == (11 as i32) {
                                                                        jsonAppendChar(
                                                                            pOut,
                                                                            (91 as i32) as i8,
                                                                        );
                                                                        *__slate_slot_623 = i
                                                                            .wrapping_add(
                                                                                *__slate_slot_622,
                                                                            );
                                                                        *__slate_slot_624 =
                                                                            (*__slate_slot_623)
                                                                                .wrapping_add(
                                                                                *__slate_slot_621,
                                                                            );
                                                                        std::ptr::write(
                                                                            __slate_slot_1560,
                                                                            pParse,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_1561,
                                                                            unsafe {
                                                                                (*(*__slate_slot_1560)).iDepth
                                                                            },
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_1562,
                                                                            ((((*__slate_slot_1561
                                                                                as u32)
                                                                                as i32)
                                                                                + (1 as i32))
                                                                                as i16)
                                                                                as u16,
                                                                        );
                                                                        unsafe {
                                                                            (*(*__slate_slot_1560)).iDepth = *__slate_slot_1562;
                                                                        }
                                                                        if ((*__slate_slot_1562
                                                                            as u32)
                                                                            as i32)
                                                                            > (1000 as i32)
                                                                        {
                                                                            jsonStringTooDeep(pOut);
                                                                        }
                                                                        loop {
                                                                            if *__slate_slot_623
                                                                                < *__slate_slot_624
                                                                                && (((unsafe {
                                                                                    (*pOut).eErr
                                                                                })
                                                                                    as u32)
                                                                                    as i32)
                                                                                    == (0 as i32)
                                                                            {
                                                                                *__slate_slot_623 = jsonTranslateBlobToText(pParse, *__slate_slot_623, pOut);
                                                                                jsonAppendChar(
                                                                                    pOut,
                                                                                    (44 as i32)
                                                                                        as i8,
                                                                                );
                                                                            } else {
                                                                                break;
                                                                            }
                                                                        }
                                                                        std::ptr::write(
                                                                            __slate_slot_1563,
                                                                            pParse,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_1564,
                                                                            unsafe {
                                                                                (*(*__slate_slot_1563)).iDepth
                                                                            },
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_1565,
                                                                            ((((*__slate_slot_1564
                                                                                as u32)
                                                                                as i32)
                                                                                - (1 as i32))
                                                                                as i16)
                                                                                as u16,
                                                                        );
                                                                        unsafe {
                                                                            (*(*__slate_slot_1563)).iDepth = *__slate_slot_1565;
                                                                        }
                                                                        if *__slate_slot_623
                                                                            > *__slate_slot_624
                                                                        {
                                                                            std::ptr::write(
                                                                                __slate_slot_1566,
                                                                                pOut,
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_1567,
                                                                                unsafe {
                                                                                    (*(*__slate_slot_1566)).eErr
                                                                                },
                                                                            );
                                                                            std::ptr::write(__slate_slot_1568, ((((*__slate_slot_1567 as u32) as i32) | (2 as i32)) as i8) as u8);
                                                                            unsafe {
                                                                                (*(*__slate_slot_1566)).eErr = *__slate_slot_1568;
                                                                            }
                                                                        }
                                                                        if *__slate_slot_621
                                                                            > ((0 as i32) as u32)
                                                                        {
                                                                            jsonStringTrimOneChar(
                                                                                pOut,
                                                                            );
                                                                        }
                                                                        jsonAppendChar(
                                                                            pOut,
                                                                            (93 as i32) as i8,
                                                                        );
                                                                        break '__join_0;
                                                                    } else {
                                                                        if __t1 == (12 as i32) {
                                                                            std::ptr::write(
                                                                                __slate_slot_634,
                                                                                0 as i32,
                                                                            );
                                                                            jsonAppendChar(
                                                                                pOut,
                                                                                (123 as i32) as i8,
                                                                            );
                                                                            *__slate_slot_623 = i
                                                                                .wrapping_add(
                                                                                *__slate_slot_622,
                                                                            );
                                                                            *__slate_slot_624 = (*__slate_slot_623).wrapping_add(*__slate_slot_621);
                                                                            std::ptr::write(
                                                                                __slate_slot_1569,
                                                                                pParse,
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_1570,
                                                                                unsafe {
                                                                                    (*(*__slate_slot_1569)).iDepth
                                                                                },
                                                                            );
                                                                            std::ptr::write(__slate_slot_1571, ((((*__slate_slot_1570 as u32) as i32) + (1 as i32)) as i16) as u16);
                                                                            unsafe {
                                                                                (*(*__slate_slot_1569)).iDepth = *__slate_slot_1571;
                                                                            }
                                                                            if ((*__slate_slot_1571
                                                                                as u32)
                                                                                as i32)
                                                                                > (1000 as i32)
                                                                            {
                                                                                jsonStringTooDeep(
                                                                                    pOut,
                                                                                );
                                                                            }
                                                                            loop {
                                                                                if *__slate_slot_623 < *__slate_slot_624 && (((unsafe { (*pOut).eErr }) as u32) as i32) == (0 as i32) {
*__slate_slot_623 = jsonTranslateBlobToText(pParse, *__slate_slot_623, pOut);
std::ptr::write(__slate_slot_1572, *__slate_slot_634);
std::ptr::write(__slate_slot_1573, *__slate_slot_1572 + (1 as i32));
*__slate_slot_634 = *__slate_slot_1573;
jsonAppendChar(pOut, (if *__slate_slot_1572 & (1 as i32) != (0 as i32) { 44 as i32 } else { 58 as i32 }) as i8);
} else {
break;
}
                                                                            }
                                                                            std::ptr::write(
                                                                                __slate_slot_1574,
                                                                                pParse,
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_1575,
                                                                                unsafe {
                                                                                    (*(*__slate_slot_1574)).iDepth
                                                                                },
                                                                            );
                                                                            std::ptr::write(__slate_slot_1576, ((((*__slate_slot_1575 as u32) as i32) - (1 as i32)) as i16) as u16);
                                                                            unsafe {
                                                                                (*(*__slate_slot_1574)).iDepth = *__slate_slot_1576;
                                                                            }
                                                                            if *__slate_slot_634 & (1 as i32) != (0 as i32) || *__slate_slot_623 > *__slate_slot_624 {
std::ptr::write(__slate_slot_1577, pOut);
std::ptr::write(__slate_slot_1578, unsafe { (*(*__slate_slot_1577)).eErr });
std::ptr::write(__slate_slot_1579, ((((*__slate_slot_1578 as u32) as i32) | (2 as i32)) as i8) as u8);
unsafe {
(*(*__slate_slot_1577)).eErr = *__slate_slot_1579;
}
}
                                                                            if *__slate_slot_621
                                                                                > ((0 as i32)
                                                                                    as u32)
                                                                            {
                                                                                jsonStringTrimOneChar(pOut);
                                                                            }
                                                                            jsonAppendChar(
                                                                                pOut,
                                                                                (125 as i32) as i8,
                                                                            );
                                                                            break '__join_0;
                                                                        } else {
                                                                            break '__join_1;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if unsafe { (*pOut).nUsed }
                            .wrapping_add(*__slate_slot_621 as u64)
                            .wrapping_add(((2 as i32) as i64) as u64)
                            <= unsafe { (*pOut).nAlloc }
                        {
                            *__slate_slot_1516 = true as bool;
                        } else {
                            *__slate_slot_1516 = jsonStringGrow(
                                pOut,
                                (*__slate_slot_621).wrapping_add((2 as i32) as u32),
                            ) == (0 as i32);
                        }
                        if *__slate_slot_1516 {
                            unsafe {
                                *unsafe {
                                    unsafe { (*pOut).zBuf }
                                        .offset((unsafe { (*pOut).nUsed }) as isize)
                                } = (34 as i32) as i8;
                            }
                            unsafe {
                                memcpy(
                                    (unsafe {
                                        unsafe {
                                            unsafe { (*pOut).zBuf }
                                                .offset((unsafe { (*pOut).nUsed }) as isize)
                                        }
                                        .offset((1 as i32) as isize)
                                    }) as *mut (),
                                    ((unsafe {
                                        unsafe { (*pParse).aBlob }
                                            .offset(i.wrapping_add(*__slate_slot_622) as isize)
                                    }) as *const i8)
                                        as *const (),
                                    *__slate_slot_621 as u64,
                                )
                            };
                            unsafe {
                                *unsafe {
                                    unsafe { (*pOut).zBuf }.offset(
                                        unsafe { (*pOut).nUsed }
                                            .wrapping_add(*__slate_slot_621 as u64)
                                            .wrapping_add(((1 as i32) as i64) as u64)
                                            as isize,
                                    )
                                } = (34 as i32) as i8;
                            }
                            std::ptr::write(__slate_slot_1517, pOut);
                            std::ptr::write(__slate_slot_1518, unsafe {
                                (*(*__slate_slot_1517)).nUsed
                            });
                            std::ptr::write(
                                __slate_slot_1519,
                                (*__slate_slot_1518).wrapping_add(
                                    (*__slate_slot_621).wrapping_add((2 as i32) as u32) as u64,
                                ),
                            );
                            unsafe {
                                (*(*__slate_slot_1517)).nUsed = *__slate_slot_1519;
                            }
                            break '__join_0;
                        } else {
                            break '__join_0;
                        }
                    }
                    if *__slate_slot_621 == ((0 as i32) as u32) {
                    } else {
                        jsonAppendRaw(
                            pOut,
                            (unsafe {
                                unsafe { (*pParse).aBlob }
                                    .offset(i.wrapping_add(*__slate_slot_622) as isize)
                            }) as *const i8,
                            *__slate_slot_621,
                        );
                        break '__join_0;
                    }
                }
                std::ptr::write(__slate_slot_1580, pOut);
                std::ptr::write(__slate_slot_1581, unsafe { (*(*__slate_slot_1580)).eErr });
                std::ptr::write(
                    __slate_slot_1582,
                    ((((*__slate_slot_1581 as u32) as i32) | (2 as i32)) as i8) as u8,
                );
                unsafe {
                    (*(*__slate_slot_1580)).eErr = *__slate_slot_1582;
                }
            }
            return i
                .wrapping_add(*__slate_slot_622)
                .wrapping_add(*__slate_slot_621);
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Context for recursion of json_pretty()
#[repr(C)]
#[derive(Clone, Copy)]
struct JsonPretty {
    /// The BLOB being rendered
    pParse: *mut JsonParse,
    /// Generate pretty output into this string
    pOut: *mut JsonString,
    /// Use this text for indentation
    zIndent: *const i8,
    /// Bytes in zIndent[]
    szIndent: u32,
    /// Current level of indentation
    nIndent: u32,
}

/// Append indentation to the pretty JSON under construction
fn jsonPrettyIndent(mut pPretty: *mut JsonPretty) {
    let mut jj: u32 = 0 as u32;
    jj = (0 as i32) as u32;
    '__slate_break_1369: while jj < unsafe { (*pPretty).nIndent } {
        jsonAppendRaw(
            unsafe { (*pPretty).pOut },
            unsafe { (*pPretty).zIndent },
            unsafe { (*pPretty).szIndent },
        );
        let __v1855: u32 = jj;
        let __v1856: u32 = __v1855.wrapping_add((1 as i32) as u32);
        jj = __v1856;
    }
}

/// Translate the binary JSONB representation of JSON beginning at
/// pParse->aBlob[i] into a JSON text string.  Append the JSON
/// text onto the end of pOut.  Return the index in pParse->aBlob[]
/// of the first byte past the end of the element that is translated.
///
/// This is a variant of jsonTranslateBlobToText() that "pretty-prints"
/// the output.  Extra whitespace is inserted to make the JSON easier
/// for humans to read.
///
/// If an error is detected in the BLOB input, the pOut->eErr flag
/// might get set to JSTRING_MALFORMED.  But not all BLOB input errors
/// are detected.  So a malformed JSONB input might either result
/// in an error, or in incorrect JSON.
///
/// The pOut->eErr JSTRING_OOM flag is set on a OOM.
///
/// # Arguments
///
/// * `pPretty` - Pretty-printing context
/// * `i` - Start rendering at this index
fn jsonTranslateBlobToPrettyText(mut pPretty: *mut JsonPretty, mut i: u32) -> u32 {
    let mut sz: u32 = 0 as u32;
    let mut n: u32 = 0 as u32;
    let mut j: u32 = 0 as u32;
    let mut iEnd: u32 = 0 as u32;
    let mut pParse: *mut JsonParse = unsafe { (*pPretty).pParse };
    let mut pOut: *mut JsonString = unsafe { (*pPretty).pOut };
    n = jsonbPayloadSize(pParse as *const JsonParse, i, std::ptr::addr_of_mut!(sz));
    if n == ((0 as i32) as u32) {
        let __v1857: *mut JsonString = pOut;
        let __v1858: u8 = unsafe { (*__v1857).eErr };
        let __v1859: u8 = ((((__v1858 as u32) as i32) | (2 as i32)) as i8) as u8;
        unsafe {
            (*__v1857).eErr = __v1859;
        }
        return unsafe { (*pParse).nBlob }.wrapping_add((1 as i32) as u32);
    }
    '__slate_break_1370: {
        match (((unsafe { *unsafe { unsafe { (*pParse).aBlob }.offset(i as isize) } }) as u32)
            as i32)
            & (15 as i32)
        {
            11 => {
                j = i.wrapping_add(n);
                iEnd = j.wrapping_add(sz);
                jsonAppendChar(pOut, (91 as i32) as i8);
                if j < iEnd {
                    jsonAppendChar(pOut, (10 as i32) as i8);
                    let __v1860: *mut JsonPretty = pPretty;
                    let __v1861: u32 = unsafe { (*__v1860).nIndent };
                    let __v1862: u32 = __v1861.wrapping_add((1 as i32) as u32);
                    unsafe {
                        (*__v1860).nIndent = __v1862;
                    }
                    if (unsafe { (*pPretty).nIndent }) >= ((1000 as i32) as u32) {
                        jsonStringTooDeep(pOut);
                    }
                    '__slate_break_1371: while (((unsafe { (*pOut).eErr }) as u32) as i32)
                        == (0 as i32)
                    {
                        jsonPrettyIndent(pPretty);
                        j = jsonTranslateBlobToPrettyText(pPretty, j);
                        if j >= iEnd {
                            break '__slate_break_1371;
                        }
                        jsonAppendRawNZ(
                            pOut,
                            (b",\n\0".as_ptr() as *mut i8) as *const i8,
                            (2 as i32) as u32,
                        );
                    }
                    jsonAppendChar(pOut, (10 as i32) as i8);
                    let __v1863: *mut JsonPretty = pPretty;
                    let __v1864: u32 = unsafe { (*__v1863).nIndent };
                    let __v1865: u32 = __v1864.wrapping_sub((1 as i32) as u32);
                    unsafe {
                        (*__v1863).nIndent = __v1865;
                    }
                    jsonPrettyIndent(pPretty);
                }
                jsonAppendChar(pOut, (93 as i32) as i8);
                i = iEnd;
            }
            12 => {
                j = i.wrapping_add(n);
                iEnd = j.wrapping_add(sz);
                jsonAppendChar(pOut, (123 as i32) as i8);
                if j < iEnd {
                    jsonAppendChar(pOut, (10 as i32) as i8);
                    let __v1866: *mut JsonPretty = pPretty;
                    let __v1867: u32 = unsafe { (*__v1866).nIndent };
                    let __v1868: u32 = __v1867.wrapping_add((1 as i32) as u32);
                    unsafe {
                        (*__v1866).nIndent = __v1868;
                    }
                    if (unsafe { (*pPretty).nIndent }) >= ((1000 as i32) as u32) {
                        jsonStringTooDeep(pOut);
                    }
                    unsafe {
                        (*pParse).iDepth = (unsafe { (*pPretty).nIndent }) as u16;
                    }
                    '__slate_break_1373: while (((unsafe { (*pOut).eErr }) as u32) as i32)
                        == (0 as i32)
                    {
                        jsonPrettyIndent(pPretty);
                        j = jsonTranslateBlobToText(pParse, j, pOut);
                        if j > iEnd {
                            let __v1869: *mut JsonString = pOut;
                            let __v1870: u8 = unsafe { (*__v1869).eErr };
                            let __v1871: u8 =
                                ((((__v1870 as u32) as i32) | (2 as i32)) as i8) as u8;
                            unsafe {
                                (*__v1869).eErr = __v1871;
                            }
                            break '__slate_break_1373;
                        }
                        jsonAppendRawNZ(
                            pOut,
                            (b": \0".as_ptr() as *mut i8) as *const i8,
                            (2 as i32) as u32,
                        );
                        j = jsonTranslateBlobToPrettyText(pPretty, j);
                        if j >= iEnd {
                            break '__slate_break_1373;
                        }
                        jsonAppendRawNZ(
                            pOut,
                            (b",\n\0".as_ptr() as *mut i8) as *const i8,
                            (2 as i32) as u32,
                        );
                    }
                    jsonAppendChar(pOut, (10 as i32) as i8);
                    let __v1872: *mut JsonPretty = pPretty;
                    let __v1873: u32 = unsafe { (*__v1872).nIndent };
                    let __v1874: u32 = __v1873.wrapping_sub((1 as i32) as u32);
                    unsafe {
                        (*__v1872).nIndent = __v1874;
                    }
                    jsonPrettyIndent(pPretty);
                }
                jsonAppendChar(pOut, (125 as i32) as i8);
                i = iEnd;
            }
            _ => {
                i = jsonTranslateBlobToText(pParse, i, pOut);
            }
        }
    }
    return i;
}

/// Given that a JSONB_ARRAY object starts at offset i, return
/// the number of entries in that array.
fn jsonbArrayCount(mut pParse: *mut JsonParse, mut iRoot: u32) -> u32 {
    let mut n: u32 = 0 as u32;
    let mut sz: u32 = 0 as u32;
    let mut i: u32 = 0 as u32;
    let mut iEnd: u32 = 0 as u32;
    let mut k: u32 = (0 as i32) as u32;
    n = jsonbPayloadSize(
        pParse as *const JsonParse,
        iRoot,
        std::ptr::addr_of_mut!(sz),
    );
    iEnd = iRoot.wrapping_add(n).wrapping_add(sz);
    i = iRoot.wrapping_add(n);
    '__slate_break_1376: while n > ((0 as i32) as u32) && i < iEnd {
        n = jsonbPayloadSizeSemiInline(pParse as *const JsonParse, i, std::ptr::addr_of_mut!(sz));
        let __v1875: u32 = i;
        let __v1876: u32 = __v1875.wrapping_add(sz.wrapping_add(n));
        i = __v1876;
        let __v1877: u32 = k;
        let __v1878: u32 = __v1877.wrapping_add((1 as i32) as u32);
        k = __v1878;
    }
    return k;
}

/// Edit the payload size of the element at iRoot by the amount in
/// pParse->delta.
fn jsonAfterEditSizeAdjust(mut pParse: *mut JsonParse, mut iRoot: u32) {
    let mut sz: u32 = (0 as i32) as u32;
    let mut nBlob: u32 = 0 as u32;
    0 as i32;
    0 as i32;
    nBlob = unsafe { (*pParse).nBlob };
    unsafe {
        (*pParse).nBlob = unsafe { (*pParse).nBlobAlloc };
    }
    jsonbPayloadSize(
        pParse as *const JsonParse,
        iRoot,
        std::ptr::addr_of_mut!(sz),
    );
    unsafe {
        (*pParse).nBlob = nBlob;
    }
    let __v1879: u32 = sz;
    let __v1880: u32 = __v1879.wrapping_add((unsafe { (*pParse).delta }) as u32);
    sz = __v1880;
    let __v1881: *mut JsonParse = pParse;
    let __v1882: i32 = unsafe { (*__v1881).delta };
    let __v1883: i32 = __v1882 + jsonBlobChangePayloadSize(pParse, iRoot, sz);
    unsafe {
        (*__v1881).delta = __v1883;
    }
}

/// If the JSONB at aIns[0..nIns-1] can be expanded (by denormalizing the
/// size field) by d bytes, then write the expansion into aOut[] and
/// return true.  In this way, an overwrite happens without changing the
/// size of the JSONB, which reduces memcpy() operations and also make it
/// faster and easier to update the B-Tree entry that contains the JSONB
/// in the database.
///
/// If the expansion of aIns[] by d bytes cannot be (easily) accomplished
/// then return false.
///
/// The d parameter is guaranteed to be between 1 and 8.
///
/// This routine is an optimization.  A correct answer is obtained if it
/// always leaves the output unchanged and returns false.
///
/// # Arguments
///
/// * `aOut` - Overwrite here
/// * `aIns` - New content
/// * `nIns` - Bytes of new content
/// * `d` - Need to expand new content by this much
fn jsonBlobOverwrite(mut aOut: *mut u8, mut aIns: *const u8, mut nIns: u32, mut d: u32) -> i32 {
    let mut szPayload: u32 = 0 as u32; // Bytes of payload
    let mut i: u32 = 0 as u32; // New header size, after expansion & a loop counter
    let mut szHdr: u8 = 0 as u8; // Size of header before expansion
    // Lookup table for finding the upper 4 bits of the first byte of the
    // expanded aIns[], based on the size of the expanded aIns[] header:
    //
    //                             2     3  4     5  6  7  8     9
    if (((unsafe { *unsafe { aIns.offset((0 as i32) as isize) } }) as u32) as i32) & (15 as i32)
        <= (2 as i32)
    {
        return 0 as i32;
    }
    // Cannot enlarge NULL, true, false
    '__slate_break_1377: {
        match (((unsafe { *unsafe { aIns.offset((0 as i32) as isize) } }) as u32) as i32)
            >> (4 as i32)
        {
            12 => {
                // aIns[] header size is 2
                if (1 as i32) << d & (138 as i32) == (0 as i32) {
                    return 0 as i32;
                }
                // d must be 1, 3, or 7
                i = d.wrapping_add((2 as i32) as u32); // New hdr sz: 2, 5, or 9
                szHdr = ((2 as i32) as i8) as u8;
            }
            13 => {
                // aIns[] header size is 3
                if d != ((2 as i32) as u32) && d != ((6 as i32) as u32) {
                    return 0 as i32;
                }
                // d must be 2 or 6
                i = d.wrapping_add((3 as i32) as u32); // New hdr sz: 5 or 9
                szHdr = ((3 as i32) as i8) as u8;
            }
            14 => {
                // aIns[] header size is 5
                if d != ((4 as i32) as u32) {
                    return 0 as i32;
                }
                // d must be 4
                i = (9 as i32) as u32; // New hdr sz: 9
                szHdr = ((5 as i32) as i8) as u8;
            }
            15 => {
                // aIns[] header size is 9
                return 0 as i32; // No solution
            }
            _ => {
                // aIns[] header size 1
                if (1 as i32) << d & (278 as i32) == (0 as i32) {
                    return 0 as i32;
                }
                // d must be 1, 2, 4, or 8
                i = d.wrapping_add((1 as i32) as u32); // New hdr sz: 2, 3, 5, or 9
                szHdr = ((1 as i32) as i8) as u8;
            }
        }
    }
    0 as i32;
    unsafe {
        *unsafe { aOut.offset((0 as i32) as isize) } =
            (((((unsafe { *unsafe { aIns.offset((0 as i32) as isize) } }) as u32) as i32)
                & (15 as i32)
                | (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(aType) as *const u8 }
                            .offset(i.wrapping_sub((2 as i32) as u32) as isize)
                    }
                }) as u32) as i32)) as i8) as u8;
    }
    unsafe {
        memcpy(
            (unsafe { aOut.offset(i as isize) }) as *mut (),
            (unsafe { aIns.offset(((szHdr as u32) as i32) as isize) }) as *const (),
            nIns.wrapping_sub(((szHdr as u32) as i32) as u32) as u64,
        )
    };
    szPayload = nIns.wrapping_sub(((szHdr as u32) as i32) as u32);
    '__slate_break_1378: while (1 as i32) != (0 as i32) {
        // edit-by-break
        let __v1884: u32 = i;
        let __v1885: u32 = __v1884.wrapping_sub((1 as i32) as u32);
        i = __v1885;
        unsafe {
            *unsafe { aOut.offset(i as isize) } = (szPayload & ((255 as i32) as u32)) as u8;
        }
        if i == ((1 as i32) as u32) {
            break '__slate_break_1378;
        }
        let __v1886: u32 = szPayload;
        let __v1887: u32 = __v1886 >> (8 as i32);
        szPayload = __v1887;
    }
    0 as i32;
    return 1 as i32;
}

static mut aType: [u8; 8] = [
    ((192 as i32) as i8) as u8,
    ((208 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((224 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((240 as i32) as i8) as u8,
];

/// Modify the JSONB blob at pParse->aBlob by removing nDel bytes of
/// content beginning at iDel, and replacing them with nIns bytes of
/// content given by aIns.
///
/// nDel may be zero, in which case no bytes are removed.  But iDel is
/// still important as new bytes will be insert beginning at iDel.
///
/// aIns may be zero, in which case space is created to hold nIns bytes
/// beginning at iDel, but that space is uninitialized.
///
/// Set pParse->oom if an OOM occurs.
///
/// # Arguments
///
/// * `pParse` - The JSONB to be modified is in pParse->aBlob
/// * `iDel` - First byte to be removed
/// * `nDel` - Number of bytes to remove
/// * `aIns` - Content to insert
/// * `nIns` - Bytes of content to insert
fn jsonBlobEdit(
    mut pParse: *mut JsonParse,
    mut iDel: u32,
    mut nDel: u32,
    mut aIns: *const u8,
    mut nIns: u32,
) {
    let mut d: i64 = ((nIns as u64) as i64) - ((nDel as u64) as i64);
    0 as i32;
    let __v1888: bool;
    if d < ((0 as i32) as i64) && d >= (-(8 as i32) as i64) && aIns != std::ptr::null::<u8>() {
        __v1888 = jsonBlobOverwrite(
            unsafe { unsafe { (*pParse).aBlob }.offset(iDel as isize) },
            aIns,
            nIns,
            (-d as i32) as u32,
        ) != (0 as i32);
    } else {
        __v1888 = false as bool;
    }
    if __v1888 {
        return;
    }
    if d != ((0 as i32) as i64) {
        if (((unsafe { (*pParse).nBlob }) as u64) as i64) + d
            > (((unsafe { (*pParse).nBlobAlloc }) as u64) as i64)
        {
            jsonBlobExpand(
                pParse,
                ((unsafe { (*pParse).nBlob }) as u64).wrapping_add(d as u64),
            );
            if (unsafe { (*pParse).oom }) != (0 as u8) {
                return;
            }
        }
        unsafe {
            memmove(
                (unsafe { unsafe { (*pParse).aBlob }.offset(iDel.wrapping_add(nIns) as isize) })
                    as *mut (),
                (unsafe { unsafe { (*pParse).aBlob }.offset(iDel.wrapping_add(nDel) as isize) })
                    as *const (),
                unsafe { (*pParse).nBlob }.wrapping_sub(iDel.wrapping_add(nDel)) as u64,
            )
        };
        let __v1889: *mut JsonParse = pParse;
        let __v1890: u32 = unsafe { (*__v1889).nBlob };
        let __v1891: u32 = ((((__v1890 as u64) as i64) + d) as i32) as u32;
        unsafe {
            (*__v1889).nBlob = __v1891;
        }
        let __v1892: *mut JsonParse = pParse;
        let __v1893: i32 = unsafe { (*__v1892).delta };
        let __v1894: i32 = ((__v1893 as i64) + d) as i32;
        unsafe {
            (*__v1892).delta = __v1894;
        }
    }
    if nIns != (0 as u32) && aIns != std::ptr::null::<u8>() {
        unsafe {
            memcpy(
                (unsafe { unsafe { (*pParse).aBlob }.offset(iDel as isize) }) as *mut (),
                aIns as *const (),
                nIns as u64,
            )
        };
    }
}

/// Return the number of escaped newlines to be ignored.
/// An escaped newline is a one of the following byte sequences:
///
///    0x5c 0x0a
///    0x5c 0x0d
///    0x5c 0x0d 0x0a
///    0x5c 0xe2 0x80 0xa8
///    0x5c 0xe2 0x80 0xa9
fn jsonBytesToBypass(mut z: *const i8, mut n: u32) -> u32 {
    let mut i: u32 = (0 as i32) as u32;
    '__slate_break_1379: while i.wrapping_add((1 as i32) as u32) < n {
        if ((unsafe { *unsafe { z.offset(i as isize) } }) as i32) != (92 as i32) {
            return i;
        }
        if ((unsafe { *unsafe { z.offset(i.wrapping_add((1 as i32) as u32) as isize) } }) as i32)
            == (10 as i32)
        {
            let __v1895: u32 = i;
            let __v1896: u32 = __v1895.wrapping_add((2 as i32) as u32);
            i = __v1896;
        } else {
            if ((unsafe { *unsafe { z.offset(i.wrapping_add((1 as i32) as u32) as isize) } })
                as i32)
                == (13 as i32)
            {
                if i.wrapping_add((2 as i32) as u32) < n
                    && ((unsafe {
                        *unsafe { z.offset(i.wrapping_add((2 as i32) as u32) as isize) }
                    }) as i32)
                        == (10 as i32)
                {
                    let __v1897: u32 = i;
                    let __v1898: u32 = __v1897.wrapping_add((3 as i32) as u32);
                    i = __v1898;
                } else {
                    let __v1899: u32 = i;
                    let __v1900: u32 = __v1899.wrapping_add((2 as i32) as u32);
                    i = __v1900;
                }
            } else {
                if (226 as i32)
                    == ((((unsafe {
                        *unsafe { z.offset(i.wrapping_add((1 as i32) as u32) as isize) }
                    }) as u8) as u32) as i32)
                    && i.wrapping_add((3 as i32) as u32) < n
                    && (128 as i32)
                        == ((((unsafe {
                            *unsafe { z.offset(i.wrapping_add((2 as i32) as u32) as isize) }
                        }) as u8) as u32) as i32)
                    && ((168 as i32)
                        == ((((unsafe {
                            *unsafe { z.offset(i.wrapping_add((3 as i32) as u32) as isize) }
                        }) as u8) as u32) as i32)
                        || (169 as i32)
                            == ((((unsafe {
                                *unsafe { z.offset(i.wrapping_add((3 as i32) as u32) as isize) }
                            }) as u8) as u32) as i32))
                {
                    let __v1901: u32 = i;
                    let __v1902: u32 = __v1901.wrapping_add((4 as i32) as u32);
                    i = __v1902;
                } else {
                    break '__slate_break_1379;
                }
            }
        }
    }
    return i;
}

/// Input z[0..n] defines JSON escape sequence including the leading '\\'.
/// Decode that escape sequence into a single character.  Write that
/// character into *piOut.  Return the number of bytes in the escape sequence.
///
/// If there is a syntax error of some kind (for example too few characters
/// after the '\\' to complete the encoding) then *piOut is set to
/// JSON_INVALID_CHAR.
fn jsonUnescapeOneChar(mut z: *const i8, mut n: u32, mut piOut: *mut u32) -> u32 {
    0 as i32;
    0 as i32;
    if n < ((2 as i32) as u32) {
        unsafe {
            *piOut = (629145 as i32) as u32;
        }
        return n;
    }
    // JSON5 requires that the \0 escape not be followed by a digit.
    // But SQLite did not enforce this restriction in versions 3.42.0
    // through 3.49.2.  That was a bug.  But some applications might have
    // come to depend on that bug.  Use the SQLITE_BUG_COMPATIBLE_20250510
    // option to restore the old buggy behavior.
    //
    // Correct behavior
    match (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u8) as u32) as i32 {
        117 => {
            let mut v: u32 = 0 as u32;
            let mut vlo: u32 = 0 as u32;
            if n < ((6 as i32) as u32) {
                unsafe {
                    *piOut = (629145 as i32) as u32;
                }
                return n;
            }
            v = jsonHexToInt4(unsafe { z.offset((2 as i32) as isize) });
            let __v1592: bool;
            if v & ((64512 as i32) as u32) == ((55296 as i32) as u32)
                && n >= ((12 as i32) as u32)
                && ((unsafe { *unsafe { z.offset((6 as i32) as isize) } }) as i32) == (92 as i32)
                && ((unsafe { *unsafe { z.offset((7 as i32) as isize) } }) as i32) == (117 as i32)
            {
                let __v1593: u32 = jsonHexToInt4(unsafe { z.offset((8 as i32) as isize) });
                vlo = __v1593;
                __v1592 = __v1593 & ((64512 as i32) as u32) == ((56320 as i32) as u32);
            } else {
                __v1592 = false as bool;
            }
            if __v1592 {
                unsafe {
                    *piOut = ((v & ((1023 as i32) as u32)) << (10 as i32))
                        .wrapping_add(vlo & ((1023 as i32) as u32))
                        .wrapping_add((65536 as i32) as u32);
                }
                return (12 as i32) as u32;
            } else {
                unsafe {
                    *piOut = v;
                }
                return (6 as i32) as u32;
            }
            unsafe {
                *piOut = (8 as i32) as u32;
            }
            return (2 as i32) as u32;
        }
        98 => {
            unsafe {
                *piOut = (8 as i32) as u32;
            }
            return (2 as i32) as u32;
        }
        102 => {
            unsafe {
                *piOut = (12 as i32) as u32;
            }
            return (2 as i32) as u32;
        }
        110 => {
            unsafe {
                *piOut = (10 as i32) as u32;
            }
            return (2 as i32) as u32;
        }
        114 => {
            unsafe {
                *piOut = (13 as i32) as u32;
            }
            return (2 as i32) as u32;
        }
        116 => {
            unsafe {
                *piOut = (9 as i32) as u32;
            }
            return (2 as i32) as u32;
        }
        118 => {
            unsafe {
                *piOut = (11 as i32) as u32;
            }
            return (2 as i32) as u32;
        }
        48 => {
            // JSON5 requires that the \0 escape not be followed by a digit.
            // But SQLite did not enforce this restriction in versions 3.42.0
            // through 3.49.2.  That was a bug.  But some applications might have
            // come to depend on that bug.  Use the SQLITE_BUG_COMPATIBLE_20250510
            // option to restore the old buggy behavior.
            //
            // Correct behavior
            unsafe {
                *piOut = (if n > ((2 as i32) as u32)
                    && (((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                ((((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as u8)
                                    as u32) as i32) as isize,
                            )
                        }
                    }) as u32) as i32)
                        & (4 as i32)
                        != (0 as i32)
                {
                    629145 as i32
                } else {
                    0 as i32
                }) as u32;
            }
            return (2 as i32) as u32;
        }
        39 | 34 | 47 | 92 => {
            unsafe {
                *piOut = ((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as i32) as u32;
            }
            return (2 as i32) as u32;
        }
        120 => {
            if n < ((4 as i32) as u32) {
                unsafe {
                    *piOut = (629145 as i32) as u32;
                }
                return n;
            }
            unsafe {
                *piOut =
                    (((jsonHexToInt((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as i32)
                        as u32) as i32)
                        << (4 as i32)
                        | ((jsonHexToInt(
                            (unsafe { *unsafe { z.offset((3 as i32) as isize) } }) as i32,
                        ) as u32) as i32)) as u32;
            }
            return (4 as i32) as u32;
        }
        226 | 13 | 10 => {
            let mut nSkip: u32 = jsonBytesToBypass(z, n);
            if nSkip == ((0 as i32) as u32) {
                unsafe {
                    *piOut = (629145 as i32) as u32;
                }
                return n;
            } else {
                if nSkip == n {
                    unsafe {
                        *piOut = (0 as i32) as u32;
                    }
                    return n;
                } else {
                    if ((unsafe { *unsafe { z.offset(nSkip as isize) } }) as i32) == (92 as i32) {
                        return nSkip.wrapping_add(jsonUnescapeOneChar(
                            unsafe { z.offset(nSkip as isize) },
                            n.wrapping_sub(nSkip),
                            piOut,
                        ));
                    } else {
                        let mut sz: i32 = unsafe {
                            sqlite3Utf8ReadLimited(
                                ((unsafe { z.offset(nSkip as isize) }) as *mut u8) as *const u8,
                                n.wrapping_sub(nSkip) as i32,
                                piOut,
                            )
                        };
                        return nSkip.wrapping_add(sz as u32);
                    }
                }
            }
            unsafe {
                *piOut = (629145 as i32) as u32;
            }
            return (2 as i32) as u32;
        }
        _ => {
            unsafe {
                *piOut = (629145 as i32) as u32;
            }
            return (2 as i32) as u32;
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Compare two object labels.  Return 1 if they are equal and
/// 0 if they differ.
///
/// In this version, we know that one or the other or both of the
/// two comparands contains an escape sequence.
///
/// # Arguments
///
/// * `zLeft` - The left label
/// * `nLeft` - Size of the left label in bytes
/// * `rawLeft` - True if zLeft contains no escapes
/// * `zRight` - The right label
/// * `nRight` - Size of the right label in bytes
/// * `rawRight` - True if zRight is escape-free
fn jsonLabelCompareEscaped(
    mut zLeft: *const i8,
    mut nLeft: u32,
    mut rawLeft: i32,
    mut zRight: *const i8,
    mut nRight: u32,
    mut rawRight: i32,
) -> i32 {
    let mut cLeft: u32 = 0 as u32;
    let mut cRight: u32 = 0 as u32;
    0 as i32;
    '__slate_break_1381: while (1 as i32) != (0 as i32) {
        // exit-by-return
        if nLeft == ((0 as i32) as u32) {
            cLeft = (0 as i32) as u32;
        } else {
            if rawLeft != (0 as i32)
                || ((unsafe { *unsafe { zLeft.offset((0 as i32) as isize) } }) as i32)
                    != (92 as i32)
            {
                cLeft =
                    (unsafe { *unsafe { (zLeft as *mut u8).offset((0 as i32) as isize) } }) as u32;
                if cLeft >= ((192 as i32) as u32) {
                    let mut sz: i32 = unsafe {
                        sqlite3Utf8ReadLimited(
                            (zLeft as *mut u8) as *const u8,
                            nLeft as i32,
                            std::ptr::addr_of_mut!(cLeft),
                        )
                    };
                    let __v1903: *const i8 = zLeft;
                    let __v1904: *const i8 = unsafe { __v1903.offset(sz as isize) };
                    zLeft = __v1904;
                    let __v1905: u32 = nLeft;
                    let __v1906: u32 = __v1905.wrapping_sub(sz as u32);
                    nLeft = __v1906;
                } else {
                    let __v1907: *const i8 = zLeft;
                    let __v1908: *const i8 = unsafe { __v1907.offset((1 as i32) as isize) };
                    zLeft = __v1908;
                    let __v1909: u32 = nLeft;
                    let __v1910: u32 = __v1909.wrapping_sub((1 as i32) as u32);
                    nLeft = __v1910;
                }
            } else {
                let mut n: u32 = jsonUnescapeOneChar(zLeft, nLeft, std::ptr::addr_of_mut!(cLeft));
                let __v1911: *const i8 = zLeft;
                let __v1912: *const i8 = unsafe { __v1911.offset(n as isize) };
                zLeft = __v1912;
                0 as i32;
                let __v1913: u32 = nLeft;
                let __v1914: u32 = __v1913.wrapping_sub(n);
                nLeft = __v1914;
            }
        }
        if nRight == ((0 as i32) as u32) {
            cRight = (0 as i32) as u32;
        } else {
            if rawRight != (0 as i32)
                || ((unsafe { *unsafe { zRight.offset((0 as i32) as isize) } }) as i32)
                    != (92 as i32)
            {
                cRight =
                    (unsafe { *unsafe { (zRight as *mut u8).offset((0 as i32) as isize) } }) as u32;
                if cRight >= ((192 as i32) as u32) {
                    let mut sz: i32 = unsafe {
                        sqlite3Utf8ReadLimited(
                            (zRight as *mut u8) as *const u8,
                            nRight as i32,
                            std::ptr::addr_of_mut!(cRight),
                        )
                    };
                    let __v1915: *const i8 = zRight;
                    let __v1916: *const i8 = unsafe { __v1915.offset(sz as isize) };
                    zRight = __v1916;
                    let __v1917: u32 = nRight;
                    let __v1918: u32 = __v1917.wrapping_sub(sz as u32);
                    nRight = __v1918;
                } else {
                    let __v1919: *const i8 = zRight;
                    let __v1920: *const i8 = unsafe { __v1919.offset((1 as i32) as isize) };
                    zRight = __v1920;
                    let __v1921: u32 = nRight;
                    let __v1922: u32 = __v1921.wrapping_sub((1 as i32) as u32);
                    nRight = __v1922;
                }
            } else {
                let mut n: u32 =
                    jsonUnescapeOneChar(zRight, nRight, std::ptr::addr_of_mut!(cRight));
                let __v1923: *const i8 = zRight;
                let __v1924: *const i8 = unsafe { __v1923.offset(n as isize) };
                zRight = __v1924;
                0 as i32;
                let __v1925: u32 = nRight;
                let __v1926: u32 = __v1925.wrapping_sub(n);
                nRight = __v1926;
            }
        }
        if cLeft != cRight {
            return 0 as i32;
        }
        if cLeft == ((0 as i32) as u32) {
            return 1 as i32;
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Compare two object labels.  Return 1 if they are equal and
/// 0 if they differ.  Return -1 if an OOM occurs.
///
/// # Arguments
///
/// * `zLeft` - The left label
/// * `nLeft` - Size of the left label in bytes
/// * `rawLeft` - True if zLeft contains no escapes
/// * `zRight` - The right label
/// * `nRight` - Size of the right label in bytes
/// * `rawRight` - True if zRight is escape-free
fn jsonLabelCompare(
    mut zLeft: *const i8,
    mut nLeft: u32,
    mut rawLeft: i32,
    mut zRight: *const i8,
    mut nRight: u32,
    mut rawRight: i32,
) -> i32 {
    if rawLeft != (0 as i32) && rawRight != (0 as i32) {
        // Simpliest case:  Neither label contains escapes.  A simple
        // memcmp() is sufficient.
        if nLeft != nRight {
            return 0 as i32;
        }
        return ((unsafe { memcmp(zLeft as *const (), zRight as *const (), nLeft as u64) })
            == (0 as i32)) as i32;
    } else {
        return jsonLabelCompareEscaped(zLeft, nLeft, rawLeft, zRight, nRight, rawRight);
    }
    return unsafe { std::mem::zeroed() };
}

// Error returns from jsonLookupStep()
/// This helper routine for jsonLookupStep() populates pIns with
/// binary data that is to be inserted into pParse.
///
/// In the common case, pIns just points to pParse->aIns and pParse->nIns.
/// But if the zPath of the original edit operation includes path elements
/// that go deeper, additional substructure must be created.
///
/// For example:
///
///     json_insert('{}', '$.a.b.c', 123);
///
/// The search stops at '$.a'  But additional substructure must be
/// created for the ".b.c" part of the patch so that the final result
/// is:  {"a":{"b":{"c"::123}}}.  This routine populates pIns with
/// the binary equivalent of {"b":{"c":123}} so that it can be inserted.
///
/// The caller is responsible for resetting pIns when it has finished
/// using the substructure.
///
/// # Arguments
///
/// * `pParse` - The original JSONB that is being edited
/// * `pIns` - Populate this with the blob data to insert
/// * `zTail` - Tail of the path that determines substructure
fn jsonCreateEditSubstructure(
    mut pParse: *mut JsonParse,
    mut pIns: *mut JsonParse,
    mut zTail: *const i8,
) -> u32 {
    let mut rc: i32 = 0 as i32;
    unsafe { memset(pIns as *mut (), 0 as i32, 72 as u64) };
    unsafe {
        (*pIns).db = unsafe { (*pParse).db };
    }
    if ((unsafe { *unsafe { zTail.offset((0 as i32) as isize) } }) as i32) == (0 as i32) {
        // No substructure.  Just insert what is given in pParse.
        unsafe {
            (*pIns).aBlob = unsafe { (*pParse).aIns };
        }
        unsafe {
            (*pIns).nBlob = unsafe { (*pParse).nIns };
        }
        rc = 0 as i32;
    } else {
        // Construct the binary substructure
        unsafe {
            (*pIns).nBlob = (1 as i32) as u32;
        }
        unsafe {
            (*pIns).aBlob = (unsafe {
                unsafe { std::ptr::addr_of!(emptyObject_714) as *const u8 }.offset(
                    ((((unsafe { *unsafe { zTail.offset((0 as i32) as isize) } }) as i32)
                        == (46 as i32)) as i32) as isize,
                )
            }) as *mut u8;
        }
        unsafe {
            (*pIns).eEdit = unsafe { (*pParse).eEdit };
        }
        unsafe {
            (*pIns).nIns = unsafe { (*pParse).nIns };
        }
        unsafe {
            (*pIns).aIns = unsafe { (*pParse).aIns };
        }
        unsafe {
            (*pIns).iDepth =
                (((((unsafe { (*pParse).iDepth }) as u32) as i32) + (1 as i32)) as i16) as u16;
        }
        if (((unsafe { (*pIns).iDepth }) as u32) as i32) >= (1000 as i32) {
            return 4294967292 as u32;
        }
        rc = jsonLookupStep(pIns, (0 as i32) as u32, zTail, (0 as i32) as u32) as i32;
        let __v1974: *mut JsonParse = pParse;
        let __v1975: u16 = unsafe { (*__v1974).iDepth };
        let __v1976: u16 = ((((__v1975 as u32) as i32) - (1 as i32)) as i16) as u16;
        unsafe {
            (*__v1974).iDepth = __v1976;
        }
        let __v1977: *mut JsonParse = pParse;
        let __v1978: u8 = unsafe { (*__v1977).oom };
        let __v1979: u8 =
            ((((__v1978 as u32) as i32) | (((unsafe { (*pIns).oom }) as u32) as i32)) as i8) as u8;
        unsafe {
            (*__v1977).oom = __v1979;
        }
    }
    return rc as u32; // Error code only
}

static mut emptyObject_714: [u8; 2] = [((11 as i32) as i8) as u8, ((12 as i32) as i8) as u8];

/// Search along zPath to find the Json element specified.  Return an
/// index into pParse->aBlob[] for the start of that element's value.
///
/// If the value found by this routine is the value half of label/value pair
/// within an object, then set pPath->iLabel to the start of the corresponding
/// label, before returning.
///
/// Return one of the JSON_LOOKUP error codes if problems are seen.
///
/// This routine will also modify the blob.  If pParse->eEdit is one of
/// JEDIT_DEL, JEDIT_REPL, JEDIT_INS, JEDIT_SET, or JEDIT_AINS, then changes
/// might be made to the selected value. If an edit is performed, then the
/// return value does not necessarily point to the select element. If an edit
/// is performed, the return value is only useful for detecting error
/// conditions.
///
/// # Arguments
///
/// * `pParse` - The JSON to search
/// * `iRoot` - Begin the search at this element of aBlob[]
/// * `zPath` - The path to search
/// * `iLabel` - Label if iRoot is a value of in an object
fn jsonLookupStep(
    mut pParse: *mut JsonParse,
    mut iRoot: u32,
    mut zPath: *const i8,
    mut iLabel: u32,
) -> u32 {
    let mut i: u32 = 0 as u32;
    let mut j: u32 = 0 as u32;
    let mut k: u32 = 0 as u32;
    let mut nKey: u32 = 0 as u32;
    let mut sz: u32 = 0 as u32;
    let mut n: u32 = 0 as u32;
    let mut iEnd: u32 = 0 as u32;
    let mut rc: u32 = 0 as u32;
    let mut zKey: *const i8 = unsafe { std::mem::zeroed() };
    let mut x: u8 = 0 as u8;
    if ((unsafe { *unsafe { zPath.offset((0 as i32) as isize) } }) as i32) == (0 as i32) {
        let __v1927: bool;
        if (unsafe { (*pParse).eEdit }) != (0 as u8) {
            __v1927 = jsonBlobMakeEditable(pParse, unsafe { (*pParse).nIns }) != (0 as i32);
        } else {
            __v1927 = false as bool;
        }
        if __v1927 {
            n = jsonbPayloadSize(
                pParse as *const JsonParse,
                iRoot,
                std::ptr::addr_of_mut!(sz),
            );
            let __v1928: u32 = sz;
            let __v1929: u32 = __v1928.wrapping_add(n);
            sz = __v1929;
            if (((unsafe { (*pParse).eEdit }) as u32) as i32) == (1 as i32) {
                if iLabel > ((0 as i32) as u32) {
                    let __v1930: u32 = sz;
                    let __v1931: u32 = __v1930.wrapping_add(iRoot.wrapping_sub(iLabel));
                    sz = __v1931;
                    iRoot = iLabel;
                }
                jsonBlobEdit(pParse, iRoot, sz, std::ptr::null::<u8>(), (0 as i32) as u32);
            } else {
                if (((unsafe { (*pParse).eEdit }) as u32) as i32) == (3 as i32) {
                    // Already exists, so json_insert() is a no-op
                } else {
                    if (((unsafe { (*pParse).eEdit }) as u32) as i32) == (5 as i32) {
                        // json_array_insert()
                        if ((unsafe { *unsafe { zPath.offset(-(1 as i32) as isize) } }) as i32)
                            != (93 as i32)
                        {
                            return 4294967293 as u32;
                        } else {
                            jsonBlobEdit(
                                pParse,
                                iRoot,
                                (0 as i32) as u32,
                                (unsafe { (*pParse).aIns }) as *const u8,
                                unsafe { (*pParse).nIns },
                            );
                        }
                    } else {
                        // json_set() or json_replace()
                        jsonBlobEdit(
                            pParse,
                            iRoot,
                            sz,
                            (unsafe { (*pParse).aIns }) as *const u8,
                            unsafe { (*pParse).nIns },
                        );
                    }
                }
            }
        }
        unsafe {
            (*pParse).iLabel = iLabel;
        }
        return iRoot;
    }
    if ((unsafe { *unsafe { zPath.offset((0 as i32) as isize) } }) as i32) == (46 as i32) {
        let mut rawKey: i32 = 1 as i32;
        x = unsafe { *unsafe { unsafe { (*pParse).aBlob }.offset(iRoot as isize) } };
        let __v1932: *const i8 = zPath;
        let __v1933: *const i8 = unsafe { __v1932.offset((1 as i32) as isize) };
        zPath = __v1933;
        if ((unsafe { *unsafe { zPath.offset((0 as i32) as isize) } }) as i32) == (34 as i32) {
            zKey = unsafe { zPath.offset((1 as i32) as isize) };
            i = (1 as i32) as u32;
            '__slate_break_1386: while (unsafe { *unsafe { zPath.offset(i as isize) } })
                != (0 as i8)
                && ((unsafe { *unsafe { zPath.offset(i as isize) } }) as i32) != (34 as i32)
            {
                if ((unsafe { *unsafe { zPath.offset(i as isize) } }) as i32) == (92 as i32)
                    && ((unsafe {
                        *unsafe { zPath.offset(i.wrapping_add((1 as i32) as u32) as isize) }
                    }) as i32)
                        != (0 as i32)
                {
                    let __v1936: u32 = i;
                    let __v1937: u32 = __v1936.wrapping_add((1 as i32) as u32);
                    i = __v1937;
                }
                let __v1934: u32 = i;
                let __v1935: u32 = __v1934.wrapping_add((1 as i32) as u32);
                i = __v1935;
            }
            nKey = i.wrapping_sub((1 as i32) as u32);
            if (unsafe { *unsafe { zPath.offset(i as isize) } }) != (0 as i8) {
                let __v1938: u32 = i;
                let __v1939: u32 = __v1938.wrapping_add((1 as i32) as u32);
                i = __v1939;
            } else {
                return 4294967291 as u32;
            }
            {}
            rawKey = (((unsafe { memchr(zKey as *const (), 92 as i32, nKey as u64) }) as *const ())
                == std::ptr::null::<()>()) as i32;
        } else {
            zKey = zPath;
            i = (0 as i32) as u32;
            '__slate_break_1387: while (unsafe { *unsafe { zPath.offset(i as isize) } })
                != (0 as i8)
                && ((unsafe { *unsafe { zPath.offset(i as isize) } }) as i32) != (46 as i32)
                && ((unsafe { *unsafe { zPath.offset(i as isize) } }) as i32) != (91 as i32)
            {
                let __v1940: u32 = i;
                let __v1941: u32 = __v1940.wrapping_add((1 as i32) as u32);
                i = __v1941;
            }
            nKey = i;
            if nKey == ((0 as i32) as u32) {
                return 4294967291 as u32;
            }
        }
        if ((x as u32) as i32) & (15 as i32) != (12 as i32) {
            return 4294967294 as u32;
        }
        n = jsonbPayloadSize(
            pParse as *const JsonParse,
            iRoot,
            std::ptr::addr_of_mut!(sz),
        );
        j = iRoot.wrapping_add(n); // j is the index of a label
        iEnd = j.wrapping_add(sz);
        '__slate_break_1388: while j < iEnd {
            let mut rawLabel: i32 = 0 as i32;
            let mut zLabel: *const i8 = unsafe { std::mem::zeroed() };
            x = (((((unsafe { *unsafe { unsafe { (*pParse).aBlob }.offset(j as isize) } }) as u32)
                as i32)
                & (15 as i32)) as i8) as u8;
            if ((x as u32) as i32) < (7 as i32) || ((x as u32) as i32) > (10 as i32) {
                return 4294967295 as u32;
            }
            n = jsonbPayloadSizeSemiInline(
                pParse as *const JsonParse,
                j,
                std::ptr::addr_of_mut!(sz),
            );
            if n == ((0 as i32) as u32) {
                return 4294967295 as u32;
            }
            k = j.wrapping_add(n); // k is the index of the label text
            if k.wrapping_add(sz) >= iEnd {
                return 4294967295 as u32;
            }
            zLabel = (unsafe { unsafe { (*pParse).aBlob }.offset(k as isize) }) as *const i8;
            rawLabel =
                (((x as u32) as i32) == (7 as i32) || ((x as u32) as i32) == (10 as i32)) as i32;
            if jsonLabelCompare(zKey, nKey, rawKey, zLabel, sz, rawLabel) != (0 as i32) {
                let mut v: u32 = k.wrapping_add(sz); // v is the index of the value
                if (((unsafe { *unsafe { unsafe { (*pParse).aBlob }.offset(v as isize) } }) as u32)
                    as i32)
                    & (15 as i32)
                    > (12 as i32)
                {
                    return 4294967295 as u32;
                }
                n = jsonbPayloadSize(pParse as *const JsonParse, v, std::ptr::addr_of_mut!(sz));
                if n == ((0 as i32) as u32) || v.wrapping_add(n).wrapping_add(sz) > iEnd {
                    return 4294967295 as u32;
                }
                0 as i32;
                let __v1942: *mut JsonParse = pParse;
                let __v1943: u16 = unsafe { (*__v1942).iDepth };
                let __v1944: u16 = ((((__v1943 as u32) as i32) + (1 as i32)) as i16) as u16;
                unsafe {
                    (*__v1942).iDepth = __v1944;
                }
                if ((__v1944 as u32) as i32) >= (1000 as i32) {
                    return 4294967292 as u32;
                }
                rc = jsonLookupStep(pParse, v, unsafe { zPath.offset(i as isize) }, j);
                let __v1945: *mut JsonParse = pParse;
                let __v1946: u16 = unsafe { (*__v1945).iDepth };
                let __v1947: u16 = ((((__v1946 as u32) as i32) - (1 as i32)) as i16) as u16;
                unsafe {
                    (*__v1945).iDepth = __v1947;
                }
                if (unsafe { (*pParse).delta }) != (0 as i32) {
                    jsonAfterEditSizeAdjust(pParse, iRoot);
                }
                return rc;
            }
            j = k.wrapping_add(sz);
            if (((unsafe { *unsafe { unsafe { (*pParse).aBlob }.offset(j as isize) } }) as u32)
                as i32)
                & (15 as i32)
                > (12 as i32)
            {
                return 4294967295 as u32;
            }
            n = jsonbPayloadSizeSemiInline(
                pParse as *const JsonParse,
                j,
                std::ptr::addr_of_mut!(sz),
            );
            if n == ((0 as i32) as u32) {
                return 4294967295 as u32;
            }
            let __v1948: u32 = j;
            let __v1949: u32 = __v1948.wrapping_add(n.wrapping_add(sz));
            j = __v1949;
        }
        if j > iEnd {
            return 4294967295 as u32;
        }
        if (((unsafe { (*pParse).eEdit }) as u32) as i32) >= (3 as i32) {
            let mut nIns: u32 = 0 as u32; // Total bytes to insert (label+value)
            let mut v: JsonParse = unsafe { std::mem::zeroed() }; // BLOB encoding of the value to be inserted
            let mut ix: JsonParse = unsafe { std::mem::zeroed() }; // Header of the label to be inserted
            {}
            {}
            {}
            let __v1950: bool;
            if (((unsafe { (*pParse).eEdit }) as u32) as i32) == (5 as i32) {
                __v1950 = (unsafe {
                    sqlite3_strglob((b"*]\0".as_ptr() as *mut i8) as *const i8, unsafe {
                        zPath.offset(i as isize)
                    })
                }) != (0 as i32);
            } else {
                __v1950 = false as bool;
            }
            if __v1950 {
                return 4294967293 as u32;
            }
            unsafe { memset(std::ptr::addr_of_mut!(ix) as *mut (), 0 as i32, 72 as u64) };
            ix.db = unsafe { (*pParse).db };
            jsonBlobAppendNode(
                std::ptr::addr_of_mut!(ix),
                ((if rawKey != (0 as i32) {
                    10 as i32
                } else {
                    9 as i32
                }) as i8) as u8,
                nKey as u64,
                std::ptr::null::<()>(),
            );
            let __v1951: *mut JsonParse = pParse;
            let __v1952: u8 = unsafe { (*__v1951).oom };
            let __v1953: u8 = ((((__v1952 as u32) as i32) | ((ix.oom as u32) as i32)) as i8) as u8;
            unsafe {
                (*__v1951).oom = __v1953;
            }
            rc = jsonCreateEditSubstructure(pParse, std::ptr::addr_of_mut!(v), unsafe {
                zPath.offset(i as isize)
            });
            let __v1954: bool;
            if !(rc >= (4294967291 as u32)) {
                __v1954 =
                    jsonBlobMakeEditable(pParse, ix.nBlob.wrapping_add(nKey).wrapping_add(v.nBlob))
                        != (0 as i32);
            } else {
                __v1954 = false as bool;
            }
            if __v1954 {
                0 as i32;
                nIns = ix.nBlob.wrapping_add(nKey).wrapping_add(v.nBlob);
                jsonBlobEdit(pParse, j, (0 as i32) as u32, std::ptr::null::<u8>(), nIns);
                if !((unsafe { (*pParse).oom }) != (0 as u8)) {
                    0 as i32; // Because pParse->oom!=0
                    0 as i32; // Because pPasre->oom!=0
                    unsafe {
                        memcpy(
                            (unsafe { unsafe { (*pParse).aBlob }.offset(j as isize) }) as *mut (),
                            ix.aBlob as *const (),
                            ix.nBlob as u64,
                        )
                    };
                    k = j.wrapping_add(ix.nBlob);
                    unsafe {
                        memcpy(
                            (unsafe { unsafe { (*pParse).aBlob }.offset(k as isize) }) as *mut (),
                            zKey as *const (),
                            nKey as u64,
                        )
                    };
                    let __v1955: u32 = k;
                    let __v1956: u32 = __v1955.wrapping_add(nKey);
                    k = __v1956;
                    unsafe {
                        memcpy(
                            (unsafe { unsafe { (*pParse).aBlob }.offset(k as isize) }) as *mut (),
                            v.aBlob as *const (),
                            v.nBlob as u64,
                        )
                    };
                    if (unsafe { (*pParse).delta }) != (0 as i32) {
                        jsonAfterEditSizeAdjust(pParse, iRoot);
                    }
                }
            }
            jsonParseReset(std::ptr::addr_of_mut!(v));
            jsonParseReset(std::ptr::addr_of_mut!(ix));
            return rc;
        }
    } else {
        if ((unsafe { *unsafe { zPath.offset((0 as i32) as isize) } }) as i32) == (91 as i32) {
            let mut kk: u64 = ((0 as i32) as i64) as u64;
            x = (((((unsafe { *unsafe { unsafe { (*pParse).aBlob }.offset(iRoot as isize) } })
                as u32) as i32)
                & (15 as i32)) as i8) as u8;
            if ((x as u32) as i32) != (11 as i32) {
                return 4294967294 as u32;
            }
            n = jsonbPayloadSize(
                pParse as *const JsonParse,
                iRoot,
                std::ptr::addr_of_mut!(sz),
            );
            i = (1 as i32) as u32;
            '__slate_break_1390: while (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                        ((((unsafe { *unsafe { zPath.offset(i as isize) } }) as u8) as u32) as i32)
                            as isize,
                    )
                }
            }) as u32) as i32)
                & (4 as i32)
                != (0 as i32)
            {
                if kk < ((4294967295 as u32) as u64) {
                    kk = kk
                        .wrapping_mul(((10 as i32) as i64) as u64)
                        .wrapping_add(
                            (((unsafe { *unsafe { zPath.offset(i as isize) } }) as i32) as i64)
                                as u64,
                        )
                        .wrapping_sub(((48 as i32) as i64) as u64);
                }
                //     ^^^^^^^^^^--- Allow kk to be bigger than any JSON array so that
                // we get NOTFOUND instead of PATHERROR, without overflowing kk.
                let __v1957: u32 = i;
                let __v1958: u32 = __v1957.wrapping_add((1 as i32) as u32);
                i = __v1958;
            }
            if i < ((2 as i32) as u32)
                || ((unsafe { *unsafe { zPath.offset(i as isize) } }) as i32) != (93 as i32)
            {
                if ((unsafe { *unsafe { zPath.offset((1 as i32) as isize) } }) as i32)
                    == (35 as i32)
                {
                    kk = jsonbArrayCount(pParse, iRoot) as u64;
                    i = (2 as i32) as u32;
                    if ((unsafe { *unsafe { zPath.offset((2 as i32) as isize) } }) as i32)
                        == (45 as i32)
                        && (((unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                    ((((unsafe { *unsafe { zPath.offset((3 as i32) as isize) } })
                                        as u8) as u32) as i32)
                                        as isize,
                                )
                            }
                        }) as u32) as i32)
                            & (4 as i32)
                            != (0 as i32)
                    {
                        let mut nn: u64 = ((0 as i32) as i64) as u64;
                        i = (3 as i32) as u32;
                        '__slate_break_1391: loop {
                            if nn < ((4294967295 as u32) as u64) {
                                nn = nn
                                    .wrapping_mul(((10 as i32) as i64) as u64)
                                    .wrapping_add(
                                        (((unsafe { *unsafe { zPath.offset(i as isize) } }) as i32)
                                            as i64) as u64,
                                    )
                                    .wrapping_sub(((48 as i32) as i64) as u64);
                            }
                            //     ^^^^^^^^^^--- Allow nn to be bigger than any JSON array to
                            // get NOTFOUND instead of PATHERROR, without overflowing nn.
                            let __v1959: u32 = i;
                            let __v1960: u32 = __v1959.wrapping_add((1 as i32) as u32);
                            i = __v1960;
                            if !((((unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                        .offset(
                                            ((((unsafe { *unsafe { zPath.offset(i as isize) } })
                                                as u8)
                                                as u32)
                                                as i32)
                                                as isize,
                                        )
                                }
                            }) as u32) as i32)
                                & (4 as i32)
                                != (0 as i32))
                            {
                                break;
                            }
                        }
                        if nn > kk {
                            return 4294967294 as u32;
                        }
                        let __v1961: u64 = kk;
                        let __v1962: u64 = __v1961.wrapping_sub(nn);
                        kk = __v1962;
                    }
                    if ((unsafe { *unsafe { zPath.offset(i as isize) } }) as i32) != (93 as i32) {
                        return 4294967291 as u32;
                    }
                } else {
                    return 4294967291 as u32;
                }
            }
            j = iRoot.wrapping_add(n);
            iEnd = j.wrapping_add(sz);
            '__slate_break_1392: while j < iEnd {
                if kk == (((0 as i32) as i64) as u64) {
                    let __v1963: *mut JsonParse = pParse;
                    let __v1964: u16 = unsafe { (*__v1963).iDepth };
                    let __v1965: u16 = ((((__v1964 as u32) as i32) + (1 as i32)) as i16) as u16;
                    unsafe {
                        (*__v1963).iDepth = __v1965;
                    }
                    if ((__v1965 as u32) as i32) >= (1000 as i32) {
                        return 4294967292 as u32;
                    }
                    rc = jsonLookupStep(
                        pParse,
                        j,
                        unsafe { zPath.offset(i.wrapping_add((1 as i32) as u32) as isize) },
                        (0 as i32) as u32,
                    );
                    let __v1966: *mut JsonParse = pParse;
                    let __v1967: u16 = unsafe { (*__v1966).iDepth };
                    let __v1968: u16 = ((((__v1967 as u32) as i32) - (1 as i32)) as i16) as u16;
                    unsafe {
                        (*__v1966).iDepth = __v1968;
                    }
                    if (unsafe { (*pParse).delta }) != (0 as i32) {
                        jsonAfterEditSizeAdjust(pParse, iRoot);
                    }
                    return rc;
                }
                let __v1969: u64 = kk;
                let __v1970: u64 = __v1969.wrapping_sub(((1 as i32) as i64) as u64);
                kk = __v1970;
                n = jsonbPayloadSizeSemiInline(
                    pParse as *const JsonParse,
                    j,
                    std::ptr::addr_of_mut!(sz),
                );
                if n == ((0 as i32) as u32) {
                    return 4294967295 as u32;
                }
                let __v1971: u32 = j;
                let __v1972: u32 = __v1971.wrapping_add(n.wrapping_add(sz));
                j = __v1972;
            }
            if j > iEnd {
                return 4294967295 as u32;
            }
            if kk > (((0 as i32) as i64) as u64) {
                return 4294967294 as u32;
            }
            if (((unsafe { (*pParse).eEdit }) as u32) as i32) >= (3 as i32) {
                let mut v: JsonParse = unsafe { std::mem::zeroed() };
                {}
                {}
                {}
                rc = jsonCreateEditSubstructure(pParse, std::ptr::addr_of_mut!(v), unsafe {
                    zPath.offset(i.wrapping_add((1 as i32) as u32) as isize)
                });
                let __v1973: bool;
                if !(rc >= (4294967291 as u32)) {
                    __v1973 = jsonBlobMakeEditable(pParse, v.nBlob) != (0 as i32);
                } else {
                    __v1973 = false as bool;
                }
                if __v1973 {
                    0 as i32;
                    jsonBlobEdit(pParse, j, (0 as i32) as u32, v.aBlob as *const u8, v.nBlob);
                }
                jsonParseReset(std::ptr::addr_of_mut!(v));
                if (unsafe { (*pParse).delta }) != (0 as i32) {
                    jsonAfterEditSizeAdjust(pParse, iRoot);
                }
                return rc;
            }
        } else {
            return 4294967291 as u32;
        }
    }
    return 4294967294 as u32;
}

/// Convert a JSON BLOB into text and make that text the return value
/// of an SQL function.
fn jsonReturnTextJsonFromBlob(mut ctx: *mut sqlite3_context, mut aBlob: *const u8, mut nBlob: u32) {
    let mut x: JsonParse = unsafe { std::mem::zeroed() };
    let mut s: JsonString = unsafe { std::mem::zeroed() };
    if aBlob == std::ptr::null::<u8>() {
        return;
    }
    unsafe { memset(std::ptr::addr_of_mut!(x) as *mut (), 0 as i32, 72 as u64) };
    x.aBlob = aBlob as *mut u8;
    x.nBlob = nBlob;
    jsonStringInit(std::ptr::addr_of_mut!(s), ctx);
    jsonTranslateBlobToText(
        std::ptr::addr_of_mut!(x),
        (0 as i32) as u32,
        std::ptr::addr_of_mut!(s),
    );
    jsonReturnString(
        std::ptr::addr_of_mut!(s),
        std::ptr::null_mut::<JsonParse>(),
        std::ptr::null_mut::<sqlite3_context>(),
    );
}

/// Return the value of the BLOB node at index i.
///
/// If the value is a primitive, return it as an SQL value.
/// If the value is an array or object, return it as either
/// JSON text or the BLOB encoding, depending on the eMode flag
/// as follows:
///
///     eMode==0     JSONB if the JSON_B flag is set in userdata or
///                  text if the JSON_B flag is omitted from userdata.
///
///     eMode==1     Text
///
///     eMode==2     JSONB
///
/// # Arguments
///
/// * `pParse` - Complete JSON parse tree
/// * `i` - Index of the node
/// * `pCtx` - Return value for this function
/// * `eMode` - Format of return: text of JSONB
fn jsonReturnFromBlob(
    mut pParse: *mut JsonParse,
    mut i: u32,
    mut pCtx: *mut sqlite3_context,
    mut eMode: i32,
) {
    let mut __slate_storage_1989: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1989: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1989) as *mut u32;
    let mut __slate_storage_1988: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1988: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1988) as *mut u32;
    let mut __slate_storage_2011: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2011: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2011) as *mut u32;
    let mut __slate_storage_2010: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2010: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2010) as *mut u32;
    let mut __slate_storage_1991: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1991: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1991) as *mut u32;
    let mut __slate_storage_1990: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1990: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1990) as *mut u32;
    let mut __slate_storage_1995: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1995: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1995) as *mut u32;
    let mut __slate_storage_1994: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1994: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1994) as *mut u32;
    let mut __slate_storage_1993: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1993: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1993) as *mut u32;
    let mut __slate_storage_1992: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1992: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1992) as *mut u32;
    let mut __slate_storage_2001: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2001: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2001) as *mut u32;
    let mut __slate_storage_2000: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2000: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2000) as *mut u32;
    let mut __slate_storage_1999: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1999: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1999) as *mut u32;
    let mut __slate_storage_1998: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1998: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1998) as *mut u32;
    let mut __slate_storage_1997: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1997: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1997) as *mut u32;
    let mut __slate_storage_1996: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1996: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1996) as *mut u32;
    let mut __slate_storage_2009: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2009: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2009) as *mut u32;
    let mut __slate_storage_2008: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2008: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2008) as *mut u32;
    let mut __slate_storage_2007: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2007: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2007) as *mut u32;
    let mut __slate_storage_2006: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2006: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2006) as *mut u32;
    let mut __slate_storage_2005: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2005: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2005) as *mut u32;
    let mut __slate_storage_2004: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2004: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2004) as *mut u32;
    let mut __slate_storage_2003: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2003: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2003) as *mut u32;
    let mut __slate_storage_2002: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2002: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2002) as *mut u32;
    let mut __slate_storage_772: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_772: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_772) as *mut u32;
    let mut __slate_storage_771: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_771: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_771) as *mut u32;
    let mut __slate_storage_2013: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2013: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2013) as *mut u32;
    let mut __slate_storage_2012: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2012: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2012) as *mut u32;
    let mut __slate_storage_770: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_770: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_770) as *mut i8;
    let mut __slate_storage_769: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_769: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_769) as *mut u32;
    let mut __slate_storage_768: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_768: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_768) as *mut *mut i8;
    let mut __slate_storage_767: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_767: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_767) as *mut *const i8;
    let mut __slate_storage_766: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_766: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_766) as *mut u32;
    // Translate JSON formatted string into raw text
    let mut __slate_storage_765: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_765: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_765) as *mut u32;
    let mut __slate_storage_764: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_764: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_764) as *mut *mut i8;
    let mut __slate_storage_763: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_763: *mut f64 = std::ptr::addr_of_mut!(__slate_storage_763) as *mut f64;
    // A hexadecimal literal with 16 significant digits and with the
    // high-order bit set is a negative integer in SQLite (and hence
    // iRes comes back as negative) but should be interpreted as a
    // positive value if it occurs within JSON.  The value is too
    // large to appear as an SQLite integer so it must be converted
    // into floating point.
    let mut __slate_storage_762: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_762: *mut f64 = std::ptr::addr_of_mut!(__slate_storage_762) as *mut f64;
    let mut __slate_storage_1987: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1987: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1987) as *mut u32;
    let mut __slate_storage_1986: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1986: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1986) as *mut u32;
    let mut __slate_storage_1985: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1985: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1985) as *mut u32;
    let mut __slate_storage_1984: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1984: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1984) as *mut u32;
    let mut __slate_storage_1983: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1983: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1983) as *mut u32;
    let mut __slate_storage_1982: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1982: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1982) as *mut u32;
    let mut __slate_storage_1981: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1981: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1981) as *mut u32;
    let mut __slate_storage_1980: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1980: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1980) as *mut u32;
    let mut __slate_storage_761: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_761: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_761) as *mut i8;
    let mut __slate_storage_760: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_760: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_760) as *mut i32;
    let mut __slate_storage_759: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_759: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_759) as *mut *mut i8;
    let mut __slate_storage_758: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_758: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_758) as *mut i64;
    let mut __slate_storage_757: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_757: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_757) as *mut *mut sqlite3;
    let mut __slate_storage_756: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_756: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_756) as *mut i32;
    let mut __slate_storage_755: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_755: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_755) as *mut u32;
    let mut __slate_storage_754: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_754: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_754) as *mut u32;
    unsafe {
        std::ptr::write(__slate_slot_757, unsafe { sqlite3_context_db_handle(pCtx) });
        0 as i32;
        *__slate_slot_754 = jsonbPayloadSize(
            pParse as *const JsonParse,
            i,
            std::ptr::addr_of_mut!(*__slate_slot_755),
        );
        if *__slate_slot_754 == ((0 as i32) as u32) {
            unsafe {
                sqlite3_result_error(
                    pCtx,
                    (b"malformed JSON\0".as_ptr() as *mut i8) as *const i8,
                    -(1 as i32),
                )
            };
            return;
        } else {
            '__join_0: {
                '__join_2: {
                    '__join_1: {
                        '__join_30: {
                            '__join_45: {
                                '__join_31: {
                                    '__join_27: {
                                        '__join_26: {
                                            '__join_9: {
                                                let __t0: i32 = (((unsafe {
                                                    *unsafe {
                                                        unsafe { (*pParse).aBlob }
                                                            .offset(i as isize)
                                                    }
                                                })
                                                    as u32)
                                                    as i32)
                                                    & (15 as i32);
                                                if __t0 == (0 as i32) {
                                                    if *__slate_slot_755 != (0 as u32) {
                                                        break '__join_0;
                                                    } else {
                                                        unsafe { sqlite3_result_null(pCtx) };
                                                        break '__join_2;
                                                    }
                                                } else {
                                                    if __t0 == (1 as i32) {
                                                        if *__slate_slot_755 != (0 as u32) {
                                                            break '__join_0;
                                                        } else {
                                                            unsafe {
                                                                sqlite3_result_int(pCtx, 1 as i32)
                                                            };
                                                            break '__join_2;
                                                        }
                                                    } else {
                                                        if __t0 == (2 as i32) {
                                                            if *__slate_slot_755 != (0 as u32) {
                                                                break '__join_0;
                                                            } else {
                                                                unsafe {
                                                                    sqlite3_result_int(
                                                                        pCtx, 0 as i32,
                                                                    )
                                                                };
                                                                break '__join_2;
                                                            }
                                                        } else {
                                                            if __t0 == (4 as i32) {
                                                                break '__join_45;
                                                            } else {
                                                                if __t0 == (3 as i32) {
                                                                    break '__join_45;
                                                                } else {
                                                                    if __t0 == (6 as i32) {
                                                                        break '__join_31;
                                                                    } else {
                                                                        if __t0 == (5 as i32) {
                                                                            break '__join_31;
                                                                        } else {
                                                                            if __t0 == (10 as i32) {
                                                                                break '__join_27;
                                                                            } else {
                                                                                if __t0
                                                                                    == (7 as i32)
                                                                                {
                                                                                    break '__join_27;
                                                                                } else {
                                                                                    if __t0
                                                                                        == (9
                                                                                            as i32)
                                                                                    {
                                                                                        break '__join_26;
                                                                                    } else {
                                                                                        if __t0 == (8 as i32) {
break '__join_26;
} else {
if __t0 == (11 as i32) {
} else {
if __t0 == (12 as i32) {
} else {
break '__join_0;
}
}
}
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            if eMode == (0 as i32) {
                                                if (((unsafe { sqlite3_user_data(pCtx) }) as i64)
                                                    as i32)
                                                    & (16 as i32)
                                                    != (0 as i32)
                                                {
                                                    eMode = 2 as i32;
                                                } else {
                                                    eMode = 1 as i32;
                                                }
                                            }
                                            if eMode == (2 as i32) {
                                                unsafe {
                                                    sqlite3_result_blob(
                                                        pCtx,
                                                        (unsafe {
                                                            unsafe { (*pParse).aBlob }
                                                                .offset(i as isize)
                                                        })
                                                            as *const (),
                                                        (*__slate_slot_755)
                                                            .wrapping_add(*__slate_slot_754)
                                                            as i32,
                                                        unsafe {
                                                            std::mem::transmute::<
                                                                usize,
                                                                Option<
                                                                    unsafe extern "C-unwind" fn(
                                                                        *mut (),
                                                                    ),
                                                                >,
                                                            >(
                                                                -(1 as i32) as usize
                                                            )
                                                        },
                                                    )
                                                };
                                                break '__join_2;
                                            } else {
                                                jsonReturnTextJsonFromBlob(
                                                    pCtx,
                                                    (unsafe {
                                                        unsafe { (*pParse).aBlob }
                                                            .offset(i as isize)
                                                    })
                                                        as *const u8,
                                                    (*__slate_slot_755)
                                                        .wrapping_add(*__slate_slot_754),
                                                );
                                                break '__join_2;
                                            }
                                        }
                                        std::ptr::write(__slate_slot_769, *__slate_slot_755);
                                        *__slate_slot_767 = (unsafe {
                                            unsafe { (*pParse).aBlob }
                                                .offset(i.wrapping_add(*__slate_slot_754) as isize)
                                        })
                                            as *const i8;
                                        *__slate_slot_768 = (unsafe {
                                            sqlite3DbMallocRaw(
                                                *__slate_slot_757,
                                                (*__slate_slot_769 as u64)
                                                    .wrapping_add(((1 as i32) as i64) as u64),
                                            )
                                        })
                                            as *mut i8;
                                        if *__slate_slot_768 == std::ptr::null_mut::<i8>() {
                                            break '__join_1;
                                        } else {
                                            *__slate_slot_766 = (0 as i32) as u32;
                                            *__slate_slot_765 = (0 as i32) as u32;
                                            loop {
                                                if *__slate_slot_765 < *__slate_slot_755 {
                                                    std::ptr::write(__slate_slot_770, unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_767)
                                                                .offset(*__slate_slot_765 as isize)
                                                        }
                                                    });
                                                    if (*__slate_slot_770 as i32) == (92 as i32) {
                                                        std::ptr::write(
                                                            __slate_slot_772,
                                                            jsonUnescapeOneChar(
                                                                unsafe {
                                                                    (*__slate_slot_767).offset(
                                                                        *__slate_slot_765 as isize,
                                                                    )
                                                                },
                                                                (*__slate_slot_755).wrapping_sub(
                                                                    *__slate_slot_765,
                                                                ),
                                                                std::ptr::addr_of_mut!(
                                                                    *__slate_slot_771
                                                                ),
                                                            ),
                                                        );
                                                        if *__slate_slot_771
                                                            <= ((127 as i32) as u32)
                                                        {
                                                            std::ptr::write(
                                                                __slate_slot_1990,
                                                                *__slate_slot_766,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1991,
                                                                (*__slate_slot_1990).wrapping_add(
                                                                    (1 as i32) as u32,
                                                                ),
                                                            );
                                                            *__slate_slot_766 = *__slate_slot_1991;
                                                            unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_768).offset(
                                                                        *__slate_slot_1990 as isize,
                                                                    )
                                                                } = (*__slate_slot_771 as u8) as i8;
                                                            }
                                                        } else {
                                                            if *__slate_slot_771
                                                                <= ((2047 as i32) as u32)
                                                            {
                                                                0 as i32;
                                                                std::ptr::write(
                                                                    __slate_slot_1992,
                                                                    *__slate_slot_766,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1993,
                                                                    (*__slate_slot_1992)
                                                                        .wrapping_add(
                                                                            (1 as i32) as u32,
                                                                        ),
                                                                );
                                                                *__slate_slot_766 =
                                                                    *__slate_slot_1993;
                                                                unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_768).offset(
                                                                            *__slate_slot_1992
                                                                                as isize,
                                                                        )
                                                                    } = ((((192 as i32) as u32)
                                                                        | *__slate_slot_771
                                                                            >> (6 as i32))
                                                                        as u8)
                                                                        as i8;
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_1994,
                                                                    *__slate_slot_766,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1995,
                                                                    (*__slate_slot_1994)
                                                                        .wrapping_add(
                                                                            (1 as i32) as u32,
                                                                        ),
                                                                );
                                                                *__slate_slot_766 =
                                                                    *__slate_slot_1995;
                                                                unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_768).offset(
                                                                            *__slate_slot_1994
                                                                                as isize,
                                                                        )
                                                                    } = ((((128 as i32) as u32)
                                                                        | *__slate_slot_771
                                                                            & ((63 as i32) as u32))
                                                                        as u8)
                                                                        as i8;
                                                                }
                                                            } else {
                                                                if *__slate_slot_771
                                                                    < ((65536 as i32) as u32)
                                                                {
                                                                    0 as i32;
                                                                    std::ptr::write(
                                                                        __slate_slot_1996,
                                                                        *__slate_slot_766,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_1997,
                                                                        (*__slate_slot_1996)
                                                                            .wrapping_add(
                                                                                (1 as i32) as u32,
                                                                            ),
                                                                    );
                                                                    *__slate_slot_766 =
                                                                        *__slate_slot_1997;
                                                                    unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_768)
                                                                                .offset(
                                                                                *__slate_slot_1996
                                                                                    as isize,
                                                                            )
                                                                        } = ((((224 as i32) as u32)
                                                                            | *__slate_slot_771
                                                                                >> (12 as i32))
                                                                            as u8)
                                                                            as i8;
                                                                    }
                                                                    std::ptr::write(
                                                                        __slate_slot_1998,
                                                                        *__slate_slot_766,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_1999,
                                                                        (*__slate_slot_1998)
                                                                            .wrapping_add(
                                                                                (1 as i32) as u32,
                                                                            ),
                                                                    );
                                                                    *__slate_slot_766 =
                                                                        *__slate_slot_1999;
                                                                    unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_768)
                                                                                .offset(
                                                                                *__slate_slot_1998
                                                                                    as isize,
                                                                            )
                                                                        } = ((((128 as i32) as u32)
                                                                            | *__slate_slot_771
                                                                                >> (6 as i32)
                                                                                & ((63 as i32)
                                                                                    as u32))
                                                                            as u8)
                                                                            as i8;
                                                                    }
                                                                    std::ptr::write(
                                                                        __slate_slot_2000,
                                                                        *__slate_slot_766,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_2001,
                                                                        (*__slate_slot_2000)
                                                                            .wrapping_add(
                                                                                (1 as i32) as u32,
                                                                            ),
                                                                    );
                                                                    *__slate_slot_766 =
                                                                        *__slate_slot_2001;
                                                                    unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_768)
                                                                                .offset(
                                                                                *__slate_slot_2000
                                                                                    as isize,
                                                                            )
                                                                        } = ((((128 as i32) as u32)
                                                                            | *__slate_slot_771
                                                                                & ((63 as i32)
                                                                                    as u32))
                                                                            as u8)
                                                                            as i8;
                                                                    }
                                                                } else {
                                                                    if *__slate_slot_771
                                                                        == ((629145 as i32) as u32)
                                                                    {
                                                                        // Silently ignore illegal unicode
                                                                    } else {
                                                                        0 as i32;
                                                                        std::ptr::write(
                                                                            __slate_slot_2002,
                                                                            *__slate_slot_766,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_2003,
                                                                            (*__slate_slot_2002)
                                                                                .wrapping_add(
                                                                                    (1 as i32)
                                                                                        as u32,
                                                                                ),
                                                                        );
                                                                        *__slate_slot_766 =
                                                                            *__slate_slot_2003;
                                                                        unsafe {
                                                                            *unsafe {
                                                                                (*__slate_slot_768).offset(*__slate_slot_2002 as isize)
                                                                            } = ((((240 as i32)
                                                                                as u32)
                                                                                | *__slate_slot_771
                                                                                    >> (18 as i32))
                                                                                as u8)
                                                                                as i8;
                                                                        }
                                                                        std::ptr::write(
                                                                            __slate_slot_2004,
                                                                            *__slate_slot_766,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_2005,
                                                                            (*__slate_slot_2004)
                                                                                .wrapping_add(
                                                                                    (1 as i32)
                                                                                        as u32,
                                                                                ),
                                                                        );
                                                                        *__slate_slot_766 =
                                                                            *__slate_slot_2005;
                                                                        unsafe {
                                                                            *unsafe {
                                                                                (*__slate_slot_768).offset(*__slate_slot_2004 as isize)
                                                                            } = ((((128 as i32)
                                                                                as u32)
                                                                                | *__slate_slot_771
                                                                                    >> (12 as i32)
                                                                                    & ((63 as i32)
                                                                                        as u32))
                                                                                as u8)
                                                                                as i8;
                                                                        }
                                                                        std::ptr::write(
                                                                            __slate_slot_2006,
                                                                            *__slate_slot_766,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_2007,
                                                                            (*__slate_slot_2006)
                                                                                .wrapping_add(
                                                                                    (1 as i32)
                                                                                        as u32,
                                                                                ),
                                                                        );
                                                                        *__slate_slot_766 =
                                                                            *__slate_slot_2007;
                                                                        unsafe {
                                                                            *unsafe {
                                                                                (*__slate_slot_768).offset(*__slate_slot_2006 as isize)
                                                                            } = ((((128 as i32)
                                                                                as u32)
                                                                                | *__slate_slot_771
                                                                                    >> (6 as i32)
                                                                                    & ((63 as i32)
                                                                                        as u32))
                                                                                as u8)
                                                                                as i8;
                                                                        }
                                                                        std::ptr::write(
                                                                            __slate_slot_2008,
                                                                            *__slate_slot_766,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_2009,
                                                                            (*__slate_slot_2008)
                                                                                .wrapping_add(
                                                                                    (1 as i32)
                                                                                        as u32,
                                                                                ),
                                                                        );
                                                                        *__slate_slot_766 =
                                                                            *__slate_slot_2009;
                                                                        unsafe {
                                                                            *unsafe {
                                                                                (*__slate_slot_768).offset(*__slate_slot_2008 as isize)
                                                                            } = ((((128 as i32)
                                                                                as u32)
                                                                                | *__slate_slot_771
                                                                                    & ((63 as i32)
                                                                                        as u32))
                                                                                as u8)
                                                                                as i8;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        std::ptr::write(
                                                            __slate_slot_2010,
                                                            *__slate_slot_765,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_2011,
                                                            (*__slate_slot_2010).wrapping_add(
                                                                (*__slate_slot_772).wrapping_sub(
                                                                    (1 as i32) as u32,
                                                                ),
                                                            ),
                                                        );
                                                        *__slate_slot_765 = *__slate_slot_2011;
                                                    } else {
                                                        std::ptr::write(
                                                            __slate_slot_2012,
                                                            *__slate_slot_766,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_2013,
                                                            (*__slate_slot_2012)
                                                                .wrapping_add((1 as i32) as u32),
                                                        );
                                                        *__slate_slot_766 = *__slate_slot_2013;
                                                        unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_768).offset(
                                                                    *__slate_slot_2012 as isize,
                                                                )
                                                            } = *__slate_slot_770;
                                                        }
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_1988,
                                                        *__slate_slot_765,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1989,
                                                        (*__slate_slot_1988)
                                                            .wrapping_add((1 as i32) as u32),
                                                    );
                                                    *__slate_slot_765 = *__slate_slot_1989;
                                                } else {
                                                    break;
                                                }
                                            }
                                            // end for()
                                            0 as i32;
                                            unsafe {
                                                *unsafe {
                                                    (*__slate_slot_768)
                                                        .offset(*__slate_slot_766 as isize)
                                                } = (0 as i32) as i8;
                                            }
                                            unsafe {
                                                sqlite3_result_text(
                                                    pCtx,
                                                    *__slate_slot_768 as *const i8,
                                                    *__slate_slot_766 as i32,
                                                    unsafe {
                                                        std::mem::transmute::<
                                                            *const (),
                                                            Option<
                                                                unsafe extern "C-unwind" fn(
                                                                    *mut (),
                                                                ),
                                                            >,
                                                        >(
                                                            sqlite3RowSetClear as *const ()
                                                        )
                                                    },
                                                )
                                            };
                                            break '__join_2;
                                        }
                                    }
                                    unsafe {
                                        sqlite3_result_text(
                                            pCtx,
                                            ((unsafe {
                                                unsafe { (*pParse).aBlob }.offset(
                                                    i.wrapping_add(*__slate_slot_754) as isize,
                                                )
                                            })
                                                as *mut i8)
                                                as *const i8,
                                            *__slate_slot_755 as i32,
                                            unsafe {
                                                std::mem::transmute::<
                                                    usize,
                                                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                                                >(
                                                    -(1 as i32) as usize
                                                )
                                            },
                                        )
                                    };
                                    break '__join_2;
                                }
                                if *__slate_slot_755 == ((0 as i32) as u32) {
                                    break '__join_0;
                                } else {
                                    break '__join_30;
                                }
                            }
                            std::ptr::write(__slate_slot_758, (0 as i32) as i64);
                            std::ptr::write(__slate_slot_760, 0 as i32);
                            if *__slate_slot_755 == ((0 as i32) as u32) {
                                break '__join_0;
                            } else {
                                *__slate_slot_761 = (unsafe {
                                    *unsafe {
                                        unsafe { (*pParse).aBlob }
                                            .offset(i.wrapping_add(*__slate_slot_754) as isize)
                                    }
                                }) as i8;
                                if (*__slate_slot_761 as i32) == (45 as i32) {
                                    if *__slate_slot_755 < ((2 as i32) as u32) {
                                        break '__join_0;
                                    } else {
                                        std::ptr::write(__slate_slot_1980, *__slate_slot_754);
                                        std::ptr::write(
                                            __slate_slot_1981,
                                            (*__slate_slot_1980).wrapping_add((1 as i32) as u32),
                                        );
                                        *__slate_slot_754 = *__slate_slot_1981;
                                        std::ptr::write(__slate_slot_1982, *__slate_slot_755);
                                        std::ptr::write(
                                            __slate_slot_1983,
                                            (*__slate_slot_1982).wrapping_sub((1 as i32) as u32),
                                        );
                                        *__slate_slot_755 = *__slate_slot_1983;
                                        *__slate_slot_760 = 1 as i32;
                                    }
                                }
                                *__slate_slot_759 = unsafe {
                                    sqlite3DbStrNDup(
                                        *__slate_slot_757,
                                        (unsafe {
                                            unsafe { (*pParse).aBlob }
                                                .offset(i.wrapping_add(*__slate_slot_754) as isize)
                                        }) as *const i8,
                                        ((*__slate_slot_755 as i32) as i64) as u64,
                                    )
                                };
                                if *__slate_slot_759 == std::ptr::null_mut::<i8>() {
                                    break '__join_1;
                                } else {
                                    *__slate_slot_756 = unsafe {
                                        sqlite3DecOrHexToI64(
                                            *__slate_slot_759 as *const i8,
                                            std::ptr::addr_of_mut!(*__slate_slot_758),
                                        )
                                    };
                                    unsafe {
                                        sqlite3DbFree(
                                            *__slate_slot_757,
                                            *__slate_slot_759 as *mut (),
                                        )
                                    };
                                    if *__slate_slot_756 == (0 as i32) {
                                        if *__slate_slot_758 < ((0 as i32) as i64) {
                                            *__slate_slot_762 = (unsafe {
                                                *(std::ptr::addr_of_mut!(*__slate_slot_758)
                                                    as *mut u64)
                                            })
                                                as f64;
                                            unsafe {
                                                sqlite3_result_double(
                                                    pCtx,
                                                    if *__slate_slot_760 != (0 as i32) {
                                                        -(*__slate_slot_762)
                                                    } else {
                                                        *__slate_slot_762
                                                    },
                                                )
                                            };
                                            break '__join_2;
                                        } else {
                                            unsafe {
                                                sqlite3_result_int64(
                                                    pCtx,
                                                    if *__slate_slot_760 != (0 as i32) {
                                                        -(*__slate_slot_758)
                                                    } else {
                                                        *__slate_slot_758
                                                    },
                                                )
                                            };
                                            break '__join_2;
                                        }
                                    } else {
                                        if *__slate_slot_756 == (3 as i32)
                                            && *__slate_slot_760 != (0 as i32)
                                        {
                                            unsafe {
                                                sqlite3_result_int64(
                                                    pCtx,
                                                    (-(1 as i32) as i64)
                                                        - ((((4294967295 as u32) as u64) as i64)
                                                            | ((2147483647 as i32) as i64)
                                                                << (32 as i32)),
                                                )
                                            };
                                            break '__join_2;
                                        } else {
                                            if *__slate_slot_756 == (1 as i32) {
                                                break '__join_0;
                                            } else {
                                                if *__slate_slot_760 != (0 as i32) {
                                                    std::ptr::write(
                                                        __slate_slot_1984,
                                                        *__slate_slot_754,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1985,
                                                        (*__slate_slot_1984)
                                                            .wrapping_sub((1 as i32) as u32),
                                                    );
                                                    *__slate_slot_754 = *__slate_slot_1985;
                                                    std::ptr::write(
                                                        __slate_slot_1986,
                                                        *__slate_slot_755,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1987,
                                                        (*__slate_slot_1986)
                                                            .wrapping_add((1 as i32) as u32),
                                                    );
                                                    *__slate_slot_755 = *__slate_slot_1987;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        *__slate_slot_764 = unsafe {
                            sqlite3DbStrNDup(
                                *__slate_slot_757,
                                (unsafe {
                                    unsafe { (*pParse).aBlob }
                                        .offset(i.wrapping_add(*__slate_slot_754) as isize)
                                }) as *const i8,
                                ((*__slate_slot_755 as i32) as i64) as u64,
                            )
                        };
                        if *__slate_slot_764 == std::ptr::null_mut::<i8>() {
                        } else {
                            *__slate_slot_756 = unsafe {
                                sqlite3AtoF(
                                    *__slate_slot_764 as *const i8,
                                    std::ptr::addr_of_mut!(*__slate_slot_763),
                                )
                            };
                            unsafe {
                                sqlite3DbFree(*__slate_slot_757, *__slate_slot_764 as *mut ())
                            };
                            if *__slate_slot_756 <= (0 as i32) {
                                break '__join_0;
                            } else {
                                unsafe { sqlite3_result_double(pCtx, *__slate_slot_763) };
                                break '__join_2;
                            }
                        }
                    }
                    unsafe { sqlite3_result_error_nomem(pCtx) };
                    return;
                }
                return;
            }
            unsafe {
                sqlite3_result_error(
                    pCtx,
                    (b"malformed JSON\0".as_ptr() as *mut i8) as *const i8,
                    -(1 as i32),
                )
            };
            return;
        }
    }
}

/// pArg is a function argument that might be an SQL value or a JSON
/// value.  Figure out what it is and encode it as a JSONB blob.
/// Return the results in pParse.
///
/// pParse is uninitialized upon entry.  This routine will handle the
/// initialization of pParse.  The result will be contained in
/// pParse->aBlob and pParse->nBlob.  pParse->aBlob might be dynamically
/// allocated (if pParse->nBlobAlloc is greater than zero) in which case
/// the caller is responsible for freeing the space allocated to pParse->aBlob
/// when it has finished with it.  Or pParse->aBlob might be a static string
/// or a value obtained from sqlite3_value_blob(pArg).
///
/// If the argument is a BLOB that is clearly not a JSONB, then this
/// function might set an error message in ctx and return non-zero.
/// It might also set an error message and return non-zero on an OOM error.
fn jsonFunctionArgToBlob(
    mut ctx: *mut sqlite3_context,
    mut pArg: *mut sqlite3_value,
    mut pParse: *mut JsonParse,
) -> i32 {
    let mut eType: i32 = unsafe { sqlite3_value_type(pArg) };
    unsafe { memset(pParse as *mut (), 0 as i32, 72 as u64) };
    unsafe {
        (*pParse).db = unsafe { sqlite3_context_db_handle(ctx) };
    }
    match eType {
        4 => {
            if !(jsonArgIsJsonb(pArg, pParse) != (0 as i32)) {
                unsafe {
                    sqlite3_result_error(
                        ctx,
                        (b"JSON cannot hold BLOB values\0".as_ptr() as *mut i8) as *const i8,
                        -(1 as i32),
                    )
                };
                return 1 as i32;
            }
        }
        3 => {
            let mut zJson: *const i8 = (unsafe { sqlite3_value_text(pArg) }) as *const i8;
            let mut nJson: i32 = unsafe { sqlite3_value_bytes(pArg) };
            if zJson == std::ptr::null::<i8>() {
                return 1 as i32;
            }
            if (unsafe { sqlite3_value_subtype(pArg) }) == ((74 as i32) as u32) {
                unsafe {
                    (*pParse).zJson = zJson as *mut i8;
                }
                unsafe {
                    (*pParse).nJson = nJson;
                }
                if jsonConvertTextToBlob(pParse, ctx) != (0 as i32) {
                    unsafe {
                        sqlite3_result_error(
                            ctx,
                            (b"malformed JSON\0".as_ptr() as *mut i8) as *const i8,
                            -(1 as i32),
                        )
                    };
                    unsafe {
                        sqlite3DbFree(
                            unsafe { (*pParse).db },
                            (unsafe { (*pParse).aBlob }) as *mut (),
                        )
                    };
                    unsafe { memset(pParse as *mut (), 0 as i32, 72 as u64) };
                    return 1 as i32;
                }
            } else {
                jsonBlobAppendNode(
                    pParse,
                    ((10 as i32) as i8) as u8,
                    (nJson as i64) as u64,
                    zJson as *const (),
                );
            }
        }
        2 => {
            if (unsafe { sqlite3IsNaN(unsafe { sqlite3_value_double(pArg) }) }) != (0 as i32) {
                jsonBlobAppendNode(
                    pParse,
                    ((0 as i32) as i8) as u8,
                    ((0 as i32) as i64) as u64,
                    std::ptr::null::<()>(),
                );
            } else {
                let mut n: i32 = unsafe { sqlite3_value_bytes(pArg) };
                let mut z: *const i8 = (unsafe { sqlite3_value_text(pArg) }) as *const i8;
                if z == std::ptr::null::<i8>() {
                    return 1 as i32;
                }
                if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (73 as i32) {
                    jsonBlobAppendNode(
                        pParse,
                        ((5 as i32) as i8) as u8,
                        ((5 as i32) as i64) as u64,
                        (b"9e999\0".as_ptr() as *mut i8) as *const (),
                    );
                } else {
                    if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32)
                        == (45 as i32)
                        && ((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as i32)
                            == (73 as i32)
                    {
                        jsonBlobAppendNode(
                            pParse,
                            ((5 as i32) as i8) as u8,
                            ((6 as i32) as i64) as u64,
                            (b"-9e999\0".as_ptr() as *mut i8) as *const (),
                        );
                    } else {
                        jsonBlobAppendNode(
                            pParse,
                            ((5 as i32) as i8) as u8,
                            (n as i64) as u64,
                            z as *const (),
                        );
                    }
                }
            }
        }
        1 => {
            let mut n: i32 = unsafe { sqlite3_value_bytes(pArg) };
            let mut z: *const i8 = (unsafe { sqlite3_value_text(pArg) }) as *const i8;
            if z == std::ptr::null::<i8>() {
                return 1 as i32;
            }
            jsonBlobAppendNode(
                pParse,
                ((3 as i32) as i8) as u8,
                (n as i64) as u64,
                z as *const (),
            );
        }
        _ => {
            unsafe {
                (*pParse).aBlob = unsafe { std::ptr::addr_of_mut!(aNull) as *mut u8 };
            }
            unsafe {
                (*pParse).nBlob = (1 as i32) as u32;
            }
            return 0 as i32;
        }
    }
    if (unsafe { (*pParse).oom }) != (0 as u8) {
        unsafe { sqlite3_result_error_nomem(ctx) };
        return 1 as i32;
    } else {
        return 0 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

static mut aNull: [u8; 1] = [((0 as i32) as i8) as u8];

/// Generate a path error.
///
/// The specifics of the error are determined by the rc argument.
///
///          rc                        error
///  -----------------       ----------------------
///  JSON_LOOKUP_ARRAY       "not an array"
///  JSON_LOOKUP_TOODEEP     "JSON nested too deep"
///  JSON_LOOKUP_ERROR       "malformed JSON"
///  otherwise...            "bad JSON path"
///
/// If ctx is not NULL then push the error message into ctx and return NULL.
/// If ctx is NULL, then return the text of the error message.
///
/// # Arguments
///
/// * `ctx` - The function call containing the error
/// * `zPath` - The path with the problem
/// * `rc` - Maybe JSON_LOOKUP_NOTARRAY
fn jsonBadPathError(mut ctx: *mut sqlite3_context, mut zPath: *const i8, mut rc: i32) -> *mut i8 {
    let mut zMsg: *mut i8 = unsafe { std::mem::zeroed() };
    if rc == ((4294967293 as u32) as i32) {
        zMsg = unsafe {
            sqlite3_mprintf(
                (b"not an array element: %Q\0".as_ptr() as *mut i8) as *const i8,
                zPath,
            )
        };
    } else {
        if rc == ((4294967295 as u32) as i32) {
            zMsg =
                unsafe { sqlite3_mprintf((b"malformed JSON\0".as_ptr() as *mut i8) as *const i8) };
        } else {
            if rc == ((4294967292 as u32) as i32) {
                zMsg = unsafe {
                    sqlite3_mprintf((b"JSON path too deep\0".as_ptr() as *mut i8) as *const i8)
                };
            } else {
                zMsg = unsafe {
                    sqlite3_mprintf(
                        (b"bad JSON path: %Q\0".as_ptr() as *mut i8) as *const i8,
                        zPath,
                    )
                };
            }
        }
    }
    if ctx == std::ptr::null_mut::<sqlite3_context>() {
        return zMsg;
    }
    if zMsg != std::ptr::null_mut::<i8>() {
        unsafe { sqlite3_result_error(ctx, zMsg as *const i8, -(1 as i32)) };
        unsafe { sqlite3_free(zMsg as *mut ()) };
    } else {
        unsafe { sqlite3_result_error_nomem(ctx) };
    }
    return std::ptr::null_mut::<i8>();
}

/// argv[0] is a BLOB that seems likely to be a JSONB.  Subsequent
/// arguments come in pairs where each pair contains a JSON path and
/// content to insert or set at that patch.  Do the updates
/// and return the result.
///
/// The specific operation is determined by eEdit, which can be one
/// of JEDIT_INS, JEDIT_REPL, JEDIT_SET, or JEDIT_AINS.
///
/// # Arguments
///
/// * `eEdit` - JEDIT_INS, JEDIT_REPL, JEDIT_SET, JEDIT_AINS
fn jsonInsertIntoBlob(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
    mut eEdit: i32,
) {
    let mut __slate_storage_2015: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2015: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2015) as *mut i32;
    let mut __slate_storage_2014: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2014: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2014) as *mut i32;
    let mut __slate_storage_801: std::mem::MaybeUninit<JsonParse> = std::mem::MaybeUninit::uninit();
    let __slate_slot_801: *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_801) as *mut JsonParse;
    let mut __slate_storage_800: std::mem::MaybeUninit<*mut JsonParse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_800: *mut *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_800) as *mut *mut JsonParse;
    let mut __slate_storage_799: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_799: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_799) as *mut i32;
    let mut __slate_storage_798: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_798: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_798) as *mut *const i8;
    let mut __slate_storage_797: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_797: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_797) as *mut u32;
    let mut __slate_storage_796: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_796: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_796) as *mut i32;
    unsafe {
        std::ptr::write(__slate_slot_797, (0 as i32) as u32);
        std::ptr::write(__slate_slot_798, std::ptr::null::<i8>());
        0 as i32;
        *__slate_slot_799 = if argc == (1 as i32) {
            0 as i32
        } else {
            1 as i32
        };
        *__slate_slot_800 = jsonParseFuncArg(
            ctx,
            unsafe { *unsafe { argv.offset((0 as i32) as isize) } },
            *__slate_slot_799 as u32,
        );
        if *__slate_slot_800 == std::ptr::null_mut::<JsonParse>() {
            return;
        } else {
            *__slate_slot_796 = 1 as i32;
            '__join_0: {
                loop {
                    if *__slate_slot_796 < argc - (1 as i32) {
                        if (unsafe {
                            sqlite3_value_type(unsafe {
                                *unsafe { argv.offset(*__slate_slot_796 as isize) }
                            })
                        }) == (5 as i32)
                        {
                        } else {
                            *__slate_slot_798 = (unsafe {
                                sqlite3_value_text(unsafe {
                                    *unsafe { argv.offset(*__slate_slot_796 as isize) }
                                })
                            }) as *const i8;
                            if *__slate_slot_798 == std::ptr::null::<i8>() {
                                unsafe { sqlite3_result_error_nomem(ctx) };
                                jsonParseFree(*__slate_slot_800);
                                return;
                            } else {
                                if ((unsafe {
                                    *unsafe { (*__slate_slot_798).offset((0 as i32) as isize) }
                                }) as i32)
                                    != (36 as i32)
                                {
                                    break '__join_0;
                                } else {
                                    if jsonFunctionArgToBlob(
                                        ctx,
                                        unsafe {
                                            *unsafe {
                                                argv.offset(
                                                    (*__slate_slot_796 + (1 as i32)) as isize,
                                                )
                                            }
                                        },
                                        std::ptr::addr_of_mut!(*__slate_slot_801),
                                    ) != (0 as i32)
                                    {
                                        jsonParseReset(std::ptr::addr_of_mut!(*__slate_slot_801));
                                        jsonParseFree(*__slate_slot_800);
                                        return;
                                    } else {
                                        if ((unsafe {
                                            *unsafe {
                                                (*__slate_slot_798).offset((1 as i32) as isize)
                                            }
                                        }) as i32)
                                            == (0 as i32)
                                        {
                                            if eEdit == (2 as i32) || eEdit == (4 as i32) {
                                                jsonBlobEdit(
                                                    *__slate_slot_800,
                                                    (0 as i32) as u32,
                                                    unsafe { (*(*__slate_slot_800)).nBlob },
                                                    (*__slate_slot_801).aBlob as *const u8,
                                                    (*__slate_slot_801).nBlob,
                                                );
                                            }
                                            *__slate_slot_797 = (0 as i32) as u32;
                                        } else {
                                            unsafe {
                                                (*(*__slate_slot_800)).eEdit = (eEdit as i8) as u8;
                                            }
                                            unsafe {
                                                (*(*__slate_slot_800)).nIns =
                                                    (*__slate_slot_801).nBlob;
                                            }
                                            unsafe {
                                                (*(*__slate_slot_800)).aIns =
                                                    (*__slate_slot_801).aBlob;
                                            }
                                            unsafe {
                                                (*(*__slate_slot_800)).delta = 0 as i32;
                                            }
                                            unsafe {
                                                (*(*__slate_slot_800)).iDepth =
                                                    ((0 as i32) as i16) as u16;
                                            }
                                            *__slate_slot_797 = jsonLookupStep(
                                                *__slate_slot_800,
                                                (0 as i32) as u32,
                                                unsafe {
                                                    (*__slate_slot_798).offset((1 as i32) as isize)
                                                },
                                                (0 as i32) as u32,
                                            );
                                        }
                                        jsonParseReset(std::ptr::addr_of_mut!(*__slate_slot_801));
                                        if *__slate_slot_797 == (4294967294 as u32) {
                                        } else {
                                            if *__slate_slot_797 >= (4294967291 as u32) {
                                                break '__join_0;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        std::ptr::write(__slate_slot_2014, *__slate_slot_796);
                        std::ptr::write(__slate_slot_2015, *__slate_slot_2014 + (2 as i32));
                        *__slate_slot_796 = *__slate_slot_2015;
                    } else {
                        break;
                    }
                }
                jsonReturnParse(ctx, *__slate_slot_800);
                jsonParseFree(*__slate_slot_800);
                return;
            }
            jsonParseFree(*__slate_slot_800);
            jsonBadPathError(ctx, *__slate_slot_798, *__slate_slot_797 as i32);
            return;
        }
    }
}

/// If pArg is a blob that seems like a JSONB blob, then initialize
/// p to point to that JSONB and return TRUE.  If pArg does not seem like
/// a JSONB blob, then return FALSE.
///
/// For small BLOBs (having no more than 7 bytes of payload) a full
/// validity check is done.  So for small BLOBs this routine only returns
/// true if the value is guaranteed to be a valid JSONB.  For larger BLOBs
/// (8 byte or more of payload) only the size of the outermost element is
/// checked to verify that the BLOB is superficially valid JSONB.
///
/// A full JSONB validation is done on smaller BLOBs because those BLOBs might
/// also be text JSON that has been incorrectly cast into a BLOB.
/// (See tag-20240123-a and https://sqlite.org/forum/forumpost/012136abd5)
/// If the BLOB is 9 bytes are larger, then it is not possible for the
/// superficial size check done here to pass if the input is really text
/// JSON so we do not need to look deeper in that case.
///
/// Why we only need to do full JSONB validation for smaller BLOBs:
///
/// The first byte of valid JSON text must be one of: '{', '[', '"', ' ', '\n',
/// '\r', '\t', '-', or a digit '0' through '9'.  Of these, only a subset
/// can also be the first byte of JSONB:  '{', '[', and digits '3'
/// through '9'.  In every one of those cases, the payload size is 7 bytes
/// or less.  So if we do full JSONB validation for every BLOB where the
/// payload is less than 7 bytes, we will never get a false positive for
/// JSONB on an input that is really text JSON.
fn jsonArgIsJsonb(mut pArg: *mut sqlite3_value, mut p: *mut JsonParse) -> i32 {
    let mut n: u32 = 0 as u32;
    let mut sz: u32 = (0 as i32) as u32;
    let mut c: u8 = 0 as u8;
    if (unsafe { sqlite3_value_type(pArg) }) != (4 as i32) {
        return 0 as i32;
    }
    unsafe {
        (*p).aBlob = (unsafe { sqlite3_value_blob(pArg) }) as *mut u8;
    }
    unsafe {
        (*p).nBlob = (unsafe { sqlite3_value_bytes(pArg) }) as u32;
    }
    let __v1496: bool;
    if (unsafe { (*p).nBlob }) > ((0 as i32) as u32)
        && (unsafe { (*p).aBlob }) != std::ptr::null_mut::<u8>()
    {
        let __v1497: u8 = unsafe { *unsafe { unsafe { (*p).aBlob }.offset((0 as i32) as isize) } };
        c = __v1497;
        __v1496 = ((__v1497 as u32) as i32) & (15 as i32) <= (12 as i32);
    } else {
        __v1496 = false as bool;
    }
    let __v1498: bool;
    if __v1496 {
        let __v1499: u32 = jsonbPayloadSize(
            p as *const JsonParse,
            (0 as i32) as u32,
            std::ptr::addr_of_mut!(sz),
        );
        n = __v1499;
        __v1498 = __v1499 > ((0 as i32) as u32);
    } else {
        __v1498 = false as bool;
    }
    let __v1500: bool;
    if __v1498
        && sz.wrapping_add(n) == unsafe { (*p).nBlob }
        && (((c as u32) as i32) & (15 as i32) > (2 as i32) || sz == ((0 as i32) as u32))
    {
        let __v1501: bool;
        if sz > ((7 as i32) as u32)
            || ((c as u32) as i32) != (123 as i32)
                && ((c as u32) as i32) != (91 as i32)
                && !((((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                            .offset(((c as u32) as i32) as isize)
                    }
                }) as u32) as i32)
                    & (4 as i32)
                    != (0 as i32))
        {
            __v1501 = true as bool;
        } else {
            __v1501 = jsonbValidityCheck(
                p as *const JsonParse,
                (0 as i32) as u32,
                unsafe { (*p).nBlob },
                (1 as i32) as u32,
            ) == ((0 as i32) as u32);
        }
        __v1500 = __v1501;
    } else {
        __v1500 = false as bool;
    }
    if __v1500 {
        return 1 as i32;
    }
    unsafe {
        (*p).aBlob = std::ptr::null_mut::<u8>();
    }
    unsafe {
        (*p).nBlob = (0 as i32) as u32;
    }
    return 0 as i32;
}

/// Generate a JsonParse object, containing valid JSONB in aBlob and nBlob,
/// from the SQL function argument pArg.  Return a pointer to the new
/// JsonParse object.
///
/// Ownership of the new JsonParse object is passed to the caller.  The
/// caller should invoke jsonParseFree() on the return value when it
/// has finished using it.
///
/// If any errors are detected, an appropriate error messages is set
/// using sqlite3_result_error() or the equivalent and this routine
/// returns NULL.  This routine also returns NULL if the pArg argument
/// is an SQL NULL value, but no error message is set in that case.  This
/// is so that SQL functions that are given NULL arguments will return
/// a NULL value.
fn jsonParseFuncArg(
    mut ctx: *mut sqlite3_context,
    mut pArg: *mut sqlite3_value,
    mut flgs: u32,
) -> *mut JsonParse {
    let mut __slate_storage_820: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_820: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_820) as *mut *mut i8;
    let mut __slate_storage_819: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_819: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_819) as *mut i32;
    let mut __slate_storage_818: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_818: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_818) as *mut i32;
    let mut __slate_storage_1587: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1587: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1587) as *mut bool;
    let mut __slate_storage_1586: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1586: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1586) as *mut u32;
    let mut __slate_storage_817: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_817: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_817) as *mut u32;
    let mut __slate_storage_1585: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1585: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1585) as *mut u32;
    let mut __slate_storage_1584: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1584: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1584) as *mut u32;
    let mut __slate_storage_1583: std::mem::MaybeUninit<*mut JsonParse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1583: *mut *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_1583) as *mut *mut JsonParse; // The database connection
    let mut __slate_storage_816: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_816: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_816) as *mut *mut sqlite3; // Value taken from cache
    let mut __slate_storage_815: std::mem::MaybeUninit<*mut JsonParse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_815: *mut *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_815) as *mut *mut JsonParse; // Value to be returned
    let mut __slate_storage_814: std::mem::MaybeUninit<*mut JsonParse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_814: *mut *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_814) as *mut *mut JsonParse; // Datatype of pArg
    let mut __slate_storage_813: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_813: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_813) as *mut i32;
    unsafe {
        std::ptr::write(__slate_slot_814, std::ptr::null_mut::<JsonParse>());
        std::ptr::write(__slate_slot_815, std::ptr::null_mut::<JsonParse>());
        0 as i32;
        *__slate_slot_813 = unsafe { sqlite3_value_type(pArg) };
        if *__slate_slot_813 == (5 as i32) {
            return std::ptr::null_mut::<JsonParse>();
        } else {
            *__slate_slot_815 = jsonCacheSearch(ctx, pArg);
            if *__slate_slot_815 != std::ptr::null_mut::<JsonParse>() {
                std::ptr::write(__slate_slot_1583, *__slate_slot_815);
                std::ptr::write(__slate_slot_1584, unsafe { (*(*__slate_slot_1583)).nJPRef });
                std::ptr::write(
                    __slate_slot_1585,
                    (*__slate_slot_1584).wrapping_add((1 as i32) as u32),
                );
                unsafe {
                    (*(*__slate_slot_1583)).nJPRef = *__slate_slot_1585;
                }
                if flgs & ((1 as i32) as u32) == ((0 as i32) as u32) {
                    return *__slate_slot_815;
                }
            }
            *__slate_slot_816 = unsafe { sqlite3_context_db_handle(ctx) };
            '__join_0: {
                '__join_27: {
                    '__join_23: {
                        '__join_3: {
                            '__join_7: {
                                loop {
                                    *__slate_slot_814 = (unsafe {
                                        sqlite3DbMallocZero(*__slate_slot_816, 72 as u64)
                                    })
                                        as *mut JsonParse;
                                    if *__slate_slot_814 == std::ptr::null_mut::<JsonParse>() {
                                        break '__join_0;
                                    } else {
                                        unsafe {
                                            memset(
                                                *__slate_slot_814 as *mut (),
                                                0 as i32,
                                                72 as u64,
                                            )
                                        };
                                        unsafe {
                                            (*(*__slate_slot_814)).db = *__slate_slot_816;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_814)).nJPRef = (1 as i32) as u32;
                                        }
                                        if *__slate_slot_815 != std::ptr::null_mut::<JsonParse>() {
                                            break '__join_27;
                                        } else {
                                            if *__slate_slot_813 == (4 as i32) {
                                                if jsonArgIsJsonb(pArg, *__slate_slot_814)
                                                    != (0 as i32)
                                                {
                                                    break '__join_23;
                                                } else {
                                                    // If the blob is not valid JSONB, fall through into trying to cast
                                                    // the blob into text which is then interpreted as JSON.  (tag-20240123-a)
                                                    //
                                                    // This goes against all historical documentation about how the SQLite
                                                    // JSON functions were suppose to work.  From the beginning, blob was
                                                    // reserved for expansion and a blob value should have raised an error.
                                                    // But it did not, due to a bug.  And many applications came to depend
                                                    // upon this buggy behavior, especially when using the CLI and reading
                                                    // JSON text using readfile(), which returns a blob.  For this reason
                                                    // we will continue to support the bug moving forward.
                                                    // See for example https://sqlite.org/forum/forumpost/012136abd5292b8d
                                                }
                                            }
                                            unsafe {
                                                (*(*__slate_slot_814)).zJson =
                                                    (unsafe { sqlite3_value_text(pArg) })
                                                        as *mut i8;
                                            }
                                            unsafe {
                                                (*(*__slate_slot_814)).nJson =
                                                    unsafe { sqlite3_value_bytes(pArg) };
                                            }
                                            if (unsafe { (*(*__slate_slot_816)).mallocFailed })
                                                != (0 as u8)
                                            {
                                                break '__join_0;
                                            } else {
                                                if (unsafe { (*(*__slate_slot_814)).nJson })
                                                    == (0 as i32)
                                                {
                                                    break '__join_3;
                                                } else {
                                                    0 as i32;
                                                    if jsonConvertTextToBlob(
                                                        *__slate_slot_814,
                                                        if flgs & ((2 as i32) as u32) != (0 as u32)
                                                        {
                                                            std::ptr::null_mut::<sqlite3_context>()
                                                        } else {
                                                            ctx
                                                        },
                                                    ) != (0 as i32)
                                                    {
                                                        break '__join_7;
                                                    } else {
                                                        std::ptr::write(__slate_slot_818, unsafe {
                                                            sqlite3ValueIsOfClass(
                                                                pArg as *const sqlite3_value,
                                                                unsafe {
                                                                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(sqlite3RCStrUnref as *const ())
                                                                },
                                                            )
                                                        });
                                                        if !(*__slate_slot_818 != (0 as i32)) {
                                                            std::ptr::write(
                                                                __slate_slot_820,
                                                                unsafe {
                                                                    sqlite3RCStrNew(
                                                                        ((unsafe {
                                                                            (*(*__slate_slot_814))
                                                                                .nJson
                                                                        })
                                                                            as i64)
                                                                            as u64,
                                                                    )
                                                                },
                                                            );
                                                            if *__slate_slot_820
                                                                == std::ptr::null_mut::<i8>()
                                                            {
                                                                break '__join_0;
                                                            } else {
                                                                unsafe {
                                                                    memcpy(
                                                                        *__slate_slot_820
                                                                            as *mut (),
                                                                        (unsafe {
                                                                            (*(*__slate_slot_814))
                                                                                .zJson
                                                                        })
                                                                            as *const (),
                                                                        ((unsafe {
                                                                            (*(*__slate_slot_814))
                                                                                .nJson
                                                                        })
                                                                            as i64)
                                                                            as u64,
                                                                    )
                                                                };
                                                                unsafe {
                                                                    (*(*__slate_slot_814)).zJson =
                                                                        *__slate_slot_820;
                                                                }
                                                                unsafe {
                                                                    *unsafe {
                                                                        unsafe { (*(*__slate_slot_814)).zJson }.offset((unsafe { (*(*__slate_slot_814)).nJson }) as isize)
                                                                    } = (0 as i32) as i8;
                                                                }
                                                            }
                                                        } else {
                                                            unsafe {
                                                                sqlite3RCStrRef(unsafe {
                                                                    (*(*__slate_slot_814)).zJson
                                                                })
                                                            };
                                                        }
                                                        unsafe {
                                                            (*(*__slate_slot_814)).bJsonIsRCStr =
                                                                ((1 as i32) as i8) as u8;
                                                        }
                                                        *__slate_slot_819 =
                                                            jsonCacheInsert(ctx, *__slate_slot_814);
                                                        if *__slate_slot_819 == (7 as i32) {
                                                            break '__join_0;
                                                        } else {
                                                            if flgs & ((1 as i32) as u32)
                                                                != (0 as u32)
                                                            {
                                                                *__slate_slot_815 =
                                                                    *__slate_slot_814;
                                                                *__slate_slot_814 =
                                                                    std::ptr::null_mut::<JsonParse>(
                                                                    );
                                                            } else {
                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                return *__slate_slot_814;
                            }
                            if flgs & ((2 as i32) as u32) != (0 as u32) {
                                unsafe {
                                    (*(*__slate_slot_814)).nErr = ((1 as i32) as i8) as u8;
                                }
                                return *__slate_slot_814;
                            } else {
                                jsonParseFree(*__slate_slot_814);
                                return std::ptr::null_mut::<JsonParse>();
                            }
                        }
                        if flgs & ((2 as i32) as u32) != (0 as u32) {
                            unsafe {
                                (*(*__slate_slot_814)).nErr = ((1 as i32) as i8) as u8;
                            }
                            return *__slate_slot_814;
                        } else {
                            jsonParseFree(*__slate_slot_814);
                            unsafe {
                                sqlite3_result_error(
                                    ctx,
                                    (b"malformed JSON\0".as_ptr() as *mut i8) as *const i8,
                                    -(1 as i32),
                                )
                            };
                            return std::ptr::null_mut::<JsonParse>();
                        }
                    }
                    if flgs & ((1 as i32) as u32) != ((0 as i32) as u32) {
                        *__slate_slot_1587 =
                            jsonBlobMakeEditable(*__slate_slot_814, (0 as i32) as u32)
                                == (0 as i32);
                    } else {
                        *__slate_slot_1587 = false as bool;
                    }
                    if *__slate_slot_1587 {
                        break '__join_0;
                    } else {
                        return *__slate_slot_814;
                    }
                }
                std::ptr::write(__slate_slot_817, unsafe { (*(*__slate_slot_815)).nBlob });
                unsafe {
                    (*(*__slate_slot_814)).aBlob = (unsafe {
                        sqlite3DbMallocRaw(*__slate_slot_816, *__slate_slot_817 as u64)
                    }) as *mut u8;
                }
                if (unsafe { (*(*__slate_slot_814)).aBlob }) == std::ptr::null_mut::<u8>() {
                } else {
                    unsafe {
                        memcpy(
                            (unsafe { (*(*__slate_slot_814)).aBlob }) as *mut (),
                            (unsafe { (*(*__slate_slot_815)).aBlob }) as *const (),
                            *__slate_slot_817 as u64,
                        )
                    };
                    std::ptr::write(__slate_slot_1586, *__slate_slot_817);
                    unsafe {
                        (*(*__slate_slot_814)).nBlob = *__slate_slot_1586;
                    }
                    unsafe {
                        (*(*__slate_slot_814)).nBlobAlloc = *__slate_slot_1586;
                    }
                    unsafe {
                        (*(*__slate_slot_814)).hasNonstd =
                            unsafe { (*(*__slate_slot_815)).hasNonstd };
                    }
                    jsonParseFree(*__slate_slot_815);
                    return *__slate_slot_814;
                }
            }
            jsonParseFree(*__slate_slot_815);
            jsonParseFree(*__slate_slot_814);
            unsafe { sqlite3_result_error_nomem(ctx) };
            return std::ptr::null_mut::<JsonParse>();
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Make the return value of a JSON function either the raw JSONB blob
/// or make it JSON text, depending on whether the JSON_BLOB flag is
/// set on the function.
fn jsonReturnParse(mut ctx: *mut sqlite3_context, mut p: *mut JsonParse) {
    let mut flgs: i32 = 0 as i32;
    if (unsafe { (*p).oom }) != (0 as u8) {
        unsafe { sqlite3_result_error_nomem(ctx) };
        return;
    }
    flgs = ((unsafe { sqlite3_user_data(ctx) }) as i64) as i32;
    if flgs & (16 as i32) != (0 as i32) {
        if (unsafe { (*p).nBlobAlloc }) > ((0 as i32) as u32)
            && !((unsafe { (*p).bReadOnly }) != (0 as u8))
        {
            unsafe {
                sqlite3_result_blob(
                    ctx,
                    (unsafe { (*p).aBlob }) as *const (),
                    (unsafe { (*p).nBlob }) as i32,
                    unsafe {
                        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                            sqlite3RowSetClear as *const (),
                        )
                    },
                )
            };
            unsafe {
                (*p).nBlobAlloc = (0 as i32) as u32;
            }
        } else {
            unsafe {
                sqlite3_result_blob(
                    ctx,
                    (unsafe { (*p).aBlob }) as *const (),
                    (unsafe { (*p).nBlob }) as i32,
                    unsafe {
                        std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                            -(1 as i32) as usize,
                        )
                    },
                )
            };
        }
    } else {
        let mut s: JsonString = unsafe { std::mem::zeroed() };
        jsonStringInit(std::ptr::addr_of_mut!(s), ctx);
        unsafe {
            (*p).delta = 0 as i32;
        }
        jsonTranslateBlobToText(p, (0 as i32) as u32, std::ptr::addr_of_mut!(s));
        jsonReturnString(std::ptr::addr_of_mut!(s), p, ctx);
        unsafe { sqlite3_result_subtype(ctx, (74 as i32) as u32) };
    }
}

// SQL functions used for testing and debugging
// Scalar SQL function implementations
/// Implementation of the json_quote(VALUE) function.  Return a JSON value
/// corresponding to the SQL value input.  Mostly this means putting
/// double-quotes around strings and returning the unquoted string "null"
/// when given a NULL input.
#[unsafe(link_section = ".text.slate_distinct.json.jsonQuoteFunc")]
extern "C-unwind" fn jsonQuoteFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut jx: JsonString = unsafe { std::mem::zeroed() };
    argc;
    jsonStringInit(std::ptr::addr_of_mut!(jx), ctx);
    jsonAppendSqlValue(std::ptr::addr_of_mut!(jx), unsafe {
        *unsafe { argv.offset((0 as i32) as isize) }
    });
    jsonReturnString(
        std::ptr::addr_of_mut!(jx),
        std::ptr::null_mut::<JsonParse>(),
        std::ptr::null_mut::<sqlite3_context>(),
    );
    unsafe { sqlite3_result_subtype(ctx, (74 as i32) as u32) };
}

/// Implementation of the json_array(VALUE,...) function.  Return a JSON
/// array that contains all values given in arguments.  Or if any argument
/// is a BLOB, throw an error.
#[unsafe(link_section = ".text.slate_distinct.json.jsonArrayFunc")]
extern "C-unwind" fn jsonArrayFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut i: i32 = 0 as i32;
    let mut jx: JsonString = unsafe { std::mem::zeroed() };
    jsonStringInit(std::ptr::addr_of_mut!(jx), ctx);
    jsonAppendChar(std::ptr::addr_of_mut!(jx), (91 as i32) as i8);
    i = 0 as i32;
    '__slate_break_1408: loop {
        if !(i < argc) {
            break;
        }
        jsonAppendSeparator(std::ptr::addr_of_mut!(jx));
        jsonAppendSqlValue(std::ptr::addr_of_mut!(jx), unsafe {
            *unsafe { argv.offset(i as isize) }
        });
        let __v2016: i32 = i;
        let __v2017: i32 = __v2016 + (1 as i32);
        i = __v2017;
    }
    jsonAppendChar(std::ptr::addr_of_mut!(jx), (93 as i32) as i8);
    jsonReturnString(
        std::ptr::addr_of_mut!(jx),
        std::ptr::null_mut::<JsonParse>(),
        std::ptr::null_mut::<sqlite3_context>(),
    );
    unsafe { sqlite3_result_subtype(ctx, (74 as i32) as u32) };
}

/// json_array_length(JSON)
/// json_array_length(JSON, PATH)
///
/// Return the number of elements in the top-level JSON array.
/// Return 0 if the input is not a well-formed JSON array.
#[unsafe(link_section = ".text.slate_distinct.json.jsonArrayLengthFunc")]
extern "C-unwind" fn jsonArrayLengthFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut p: *mut JsonParse = unsafe { std::mem::zeroed() }; // The parse
    let mut cnt: i64 = (0 as i32) as i64;
    let mut i: u32 = 0 as u32;
    let mut eErr: u8 = ((0 as i32) as i8) as u8;
    p = jsonParseFuncArg(
        ctx,
        unsafe { *unsafe { argv.offset((0 as i32) as isize) } },
        (0 as i32) as u32,
    );
    if p == std::ptr::null_mut::<JsonParse>() {
        return;
    }
    if argc == (2 as i32) {
        let mut zPath: *const i8 = (unsafe {
            sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
        }) as *const i8;
        if zPath == std::ptr::null::<i8>() {
            jsonParseFree(p);
            return;
        }
        i = jsonLookupStep(
            p,
            (0 as i32) as u32,
            if ((unsafe { *unsafe { zPath.offset((0 as i32) as isize) } }) as i32) == (36 as i32) {
                unsafe { zPath.offset((1 as i32) as isize) }
            } else {
                (b"@\0".as_ptr() as *mut i8) as *const i8
            },
            (0 as i32) as u32,
        );
        if i >= (4294967291 as u32) {
            if i == (4294967294 as u32) {
                // no-op
            } else {
                jsonBadPathError(ctx, zPath, i as i32);
            }
            eErr = ((1 as i32) as i8) as u8;
            i = (0 as i32) as u32;
        }
    } else {
        i = (0 as i32) as u32;
    }
    if (((unsafe { *unsafe { unsafe { (*p).aBlob }.offset(i as isize) } }) as u32) as i32)
        & (15 as i32)
        == (11 as i32)
    {
        cnt = (jsonbArrayCount(p, i) as u64) as i64;
    }
    if !(eErr != (0 as u8)) {
        unsafe { sqlite3_result_int64(ctx, cnt) };
    }
    jsonParseFree(p);
}

/// True if the string is all alphanumerics and underscores
fn jsonAllAlphanum(mut z: *const i8, mut n: i32) -> i32 {
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_1410: loop {
        if !(i < n
            && ((((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                        ((((unsafe { *unsafe { z.offset(i as isize) } }) as u8) as u32) as i32)
                            as isize,
                    )
                }
            }) as u32) as i32)
                & (6 as i32)
                != (0 as i32)
                || ((unsafe { *unsafe { z.offset(i as isize) } }) as i32) == (95 as i32)))
        {
            break;
        }
        let __v2018: i32 = i;
        let __v2019: i32 = __v2018 + (1 as i32);
        i = __v2019;
    }
    return (i == n) as i32;
}

/// json_extract(JSON, PATH, ...)
/// "->"(JSON,PATH)
/// "->>"(JSON,PATH)
///
/// Return the element described by PATH.  Return NULL if that PATH element
/// is not found.
///
/// If JSON_JSON is set or if more that one PATH argument is supplied then
/// always return a JSON representation of the result.  If JSON_SQL is set,
/// then always return an SQL representation of the result.  If neither flag
/// is present and argc==2, then return JSON for objects and arrays and SQL
/// for all other values.
///
/// When multiple PATH arguments are supplied, the result is a JSON array
/// containing the result of each PATH.
///
/// Abbreviated JSON path expressions are allows if JSON_ABPATH, for
/// compatibility with PG.
#[unsafe(link_section = ".text.slate_distinct.json.jsonExtractFunc")]
extern "C-unwind" fn jsonExtractFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut __slate_storage_2021: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2021: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2021) as *mut i32;
    let mut __slate_storage_2020: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2020: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2020) as *mut i32;
    let mut __slate_storage_860: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_860: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_860) as *mut u32;
    let mut __slate_storage_859: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_859: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_859) as *mut i32;
    // With a single PATH argument
    let mut __slate_storage_858: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_858: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_858) as *mut *const i8; // String for array result
    let mut __slate_storage_857: std::mem::MaybeUninit<JsonString> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_857: *mut JsonString =
        std::ptr::addr_of_mut!(__slate_storage_857) as *mut JsonString; // Loop counter
    let mut __slate_storage_856: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_856: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_856) as *mut i32; // Flags associated with the function
    let mut __slate_storage_855: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_855: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_855) as *mut i32; // The parse
    let mut __slate_storage_854: std::mem::MaybeUninit<*mut JsonParse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_854: *mut *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_854) as *mut *mut JsonParse;
    unsafe {
        std::ptr::write(__slate_slot_854, std::ptr::null_mut::<JsonParse>());
        if argc < (2 as i32) {
            return;
        } else {
            *__slate_slot_854 = jsonParseFuncArg(
                ctx,
                unsafe { *unsafe { argv.offset((0 as i32) as isize) } },
                (0 as i32) as u32,
            );
            if *__slate_slot_854 == std::ptr::null_mut::<JsonParse>() {
                return;
            } else {
                *__slate_slot_855 = ((unsafe { sqlite3_user_data(ctx) }) as i64) as i32;
                jsonStringInit(std::ptr::addr_of_mut!(*__slate_slot_857), ctx);
                if argc > (2 as i32) {
                    jsonAppendChar(std::ptr::addr_of_mut!(*__slate_slot_857), (91 as i32) as i8);
                }
                *__slate_slot_856 = 1 as i32;
                '__join_0: {
                    '__join_14: {
                        '__join_28: {
                            loop {
                                if *__slate_slot_856 < argc {
                                    std::ptr::write(
                                        __slate_slot_858,
                                        (unsafe {
                                            sqlite3_value_text(unsafe {
                                                *unsafe { argv.offset(*__slate_slot_856 as isize) }
                                            })
                                        }) as *const i8,
                                    );
                                    if *__slate_slot_858 == std::ptr::null::<i8>() {
                                        break '__join_0;
                                    } else {
                                        *__slate_slot_859 =
                                            unsafe { sqlite3Strlen30(*__slate_slot_858) };
                                        if ((unsafe {
                                            *unsafe {
                                                (*__slate_slot_858).offset((0 as i32) as isize)
                                            }
                                        }) as i32)
                                            == (36 as i32)
                                        {
                                            *__slate_slot_860 = jsonLookupStep(
                                                *__slate_slot_854,
                                                (0 as i32) as u32,
                                                unsafe {
                                                    (*__slate_slot_858).offset((1 as i32) as isize)
                                                },
                                                (0 as i32) as u32,
                                            );
                                        } else {
                                            if *__slate_slot_855 & (3 as i32) != (0 as i32) {
                                                // The -> and ->> operators accept abbreviated PATH arguments.  This
                                                // is mostly for compatibility with PostgreSQL, but also for
                                                // convenience.
                                                //
                                                //     NUMBER   ==>  $[NUMBER]     // PG compatible
                                                //     LABEL    ==>  $.LABEL       // PG compatible
                                                //     [NUMBER] ==>  $[NUMBER]     // Not PG.  Purely for convenience
                                                //
                                                // Updated 2024-05-27:  If the NUMBER is negative, then PG counts from
                                                // the right of the array.  Hence for negative NUMBER:
                                                //
                                                //     NUMBER   ==>  $[#NUMBER]    // PG compatible
                                                jsonStringInit(
                                                    std::ptr::addr_of_mut!(*__slate_slot_857),
                                                    ctx,
                                                );
                                                if (unsafe {
                                                    sqlite3_value_type(unsafe {
                                                        *unsafe {
                                                            argv.offset(*__slate_slot_856 as isize)
                                                        }
                                                    })
                                                }) == (1 as i32)
                                                {
                                                    jsonAppendRawNZ(
                                                        std::ptr::addr_of_mut!(*__slate_slot_857),
                                                        (b"[\0".as_ptr() as *mut i8) as *const i8,
                                                        (1 as i32) as u32,
                                                    );
                                                    if ((unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_858)
                                                                .offset((0 as i32) as isize)
                                                        }
                                                    })
                                                        as i32)
                                                        == (45 as i32)
                                                    {
                                                        jsonAppendRawNZ(
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_857
                                                            ),
                                                            (b"#\0".as_ptr() as *mut i8)
                                                                as *const i8,
                                                            (1 as i32) as u32,
                                                        );
                                                    }
                                                    jsonAppendRaw(
                                                        std::ptr::addr_of_mut!(*__slate_slot_857),
                                                        *__slate_slot_858,
                                                        *__slate_slot_859 as u32,
                                                    );
                                                    jsonAppendRawNZ(
                                                        std::ptr::addr_of_mut!(*__slate_slot_857),
                                                        (b"]\0".as_ptr() as *mut i8) as *const i8,
                                                        (2 as i32) as u32,
                                                    );
                                                } else {
                                                    if jsonAllAlphanum(
                                                        *__slate_slot_858,
                                                        *__slate_slot_859,
                                                    ) != (0 as i32)
                                                    {
                                                        jsonAppendRawNZ(
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_857
                                                            ),
                                                            (b".\0".as_ptr() as *mut i8)
                                                                as *const i8,
                                                            (1 as i32) as u32,
                                                        );
                                                        jsonAppendRaw(
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_857
                                                            ),
                                                            *__slate_slot_858,
                                                            *__slate_slot_859 as u32,
                                                        );
                                                    } else {
                                                        if ((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_858)
                                                                    .offset((0 as i32) as isize)
                                                            }
                                                        })
                                                            as i32)
                                                            == (91 as i32)
                                                            && *__slate_slot_859 >= (3 as i32)
                                                            && ((unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_858).offset(
                                                                        (*__slate_slot_859
                                                                            - (1 as i32))
                                                                            as isize,
                                                                    )
                                                                }
                                                            })
                                                                as i32)
                                                                == (93 as i32)
                                                        {
                                                            jsonAppendRaw(
                                                                std::ptr::addr_of_mut!(
                                                                    *__slate_slot_857
                                                                ),
                                                                *__slate_slot_858,
                                                                *__slate_slot_859 as u32,
                                                            );
                                                        } else {
                                                            jsonAppendRawNZ(
                                                                std::ptr::addr_of_mut!(
                                                                    *__slate_slot_857
                                                                ),
                                                                (b".\"\0".as_ptr() as *mut i8)
                                                                    as *const i8,
                                                                (2 as i32) as u32,
                                                            );
                                                            jsonAppendRaw(
                                                                std::ptr::addr_of_mut!(
                                                                    *__slate_slot_857
                                                                ),
                                                                *__slate_slot_858,
                                                                *__slate_slot_859 as u32,
                                                            );
                                                            jsonAppendRawNZ(
                                                                std::ptr::addr_of_mut!(
                                                                    *__slate_slot_857
                                                                ),
                                                                (b"\"\0".as_ptr() as *mut i8)
                                                                    as *const i8,
                                                                (1 as i32) as u32,
                                                            );
                                                        }
                                                    }
                                                }
                                                jsonStringTerminate(std::ptr::addr_of_mut!(
                                                    *__slate_slot_857
                                                ));
                                                *__slate_slot_860 = jsonLookupStep(
                                                    *__slate_slot_854,
                                                    (0 as i32) as u32,
                                                    (*__slate_slot_857).zBuf as *const i8,
                                                    (0 as i32) as u32,
                                                );
                                                jsonStringReset(std::ptr::addr_of_mut!(
                                                    *__slate_slot_857
                                                ));
                                            } else {
                                                break '__join_28;
                                            }
                                        }
                                        if *__slate_slot_860
                                            < unsafe { (*(*__slate_slot_854)).nBlob }
                                        {
                                            if argc == (2 as i32) {
                                                if *__slate_slot_855 & (1 as i32) != (0 as i32) {
                                                    jsonStringInit(
                                                        std::ptr::addr_of_mut!(*__slate_slot_857),
                                                        ctx,
                                                    );
                                                    jsonTranslateBlobToText(
                                                        *__slate_slot_854,
                                                        *__slate_slot_860,
                                                        std::ptr::addr_of_mut!(*__slate_slot_857),
                                                    );
                                                    jsonReturnString(
                                                        std::ptr::addr_of_mut!(*__slate_slot_857),
                                                        std::ptr::null_mut::<JsonParse>(),
                                                        std::ptr::null_mut::<sqlite3_context>(),
                                                    );
                                                    jsonStringReset(std::ptr::addr_of_mut!(
                                                        *__slate_slot_857
                                                    ));
                                                    0 as i32;
                                                    unsafe {
                                                        sqlite3_result_subtype(
                                                            ctx,
                                                            (74 as i32) as u32,
                                                        )
                                                    };
                                                } else {
                                                    jsonReturnFromBlob(
                                                        *__slate_slot_854,
                                                        *__slate_slot_860,
                                                        ctx,
                                                        0 as i32,
                                                    );
                                                    if *__slate_slot_855
                                                        & ((2 as i32) | (16 as i32))
                                                        == (0 as i32)
                                                        && (((unsafe {
                                                            *unsafe {
                                                                unsafe {
                                                                    (*(*__slate_slot_854)).aBlob
                                                                }
                                                                .offset(*__slate_slot_860 as isize)
                                                            }
                                                        })
                                                            as u32)
                                                            as i32)
                                                            & (15 as i32)
                                                            >= (11 as i32)
                                                    {
                                                        unsafe {
                                                            sqlite3_result_subtype(
                                                                ctx,
                                                                (74 as i32) as u32,
                                                            )
                                                        };
                                                    }
                                                }
                                            } else {
                                                jsonAppendSeparator(std::ptr::addr_of_mut!(
                                                    *__slate_slot_857
                                                ));
                                                jsonTranslateBlobToText(
                                                    *__slate_slot_854,
                                                    *__slate_slot_860,
                                                    std::ptr::addr_of_mut!(*__slate_slot_857),
                                                );
                                            }
                                        } else {
                                            if *__slate_slot_860 == (4294967294 as u32) {
                                                if argc == (2 as i32) {
                                                    break '__join_0;
                                                } else {
                                                    jsonAppendSeparator(std::ptr::addr_of_mut!(
                                                        *__slate_slot_857
                                                    ));
                                                    jsonAppendRawNZ(
                                                        std::ptr::addr_of_mut!(*__slate_slot_857),
                                                        (b"null\0".as_ptr() as *mut i8)
                                                            as *const i8,
                                                        (4 as i32) as u32,
                                                    );
                                                }
                                            } else {
                                                break '__join_14;
                                            }
                                        }
                                        std::ptr::write(__slate_slot_2020, *__slate_slot_856);
                                        std::ptr::write(
                                            __slate_slot_2021,
                                            *__slate_slot_2020 + (1 as i32),
                                        );
                                        *__slate_slot_856 = *__slate_slot_2021;
                                    }
                                } else {
                                    break;
                                }
                            }
                            if argc > (2 as i32) {
                                jsonAppendChar(
                                    std::ptr::addr_of_mut!(*__slate_slot_857),
                                    (93 as i32) as i8,
                                );
                                jsonReturnString(
                                    std::ptr::addr_of_mut!(*__slate_slot_857),
                                    std::ptr::null_mut::<JsonParse>(),
                                    std::ptr::null_mut::<sqlite3_context>(),
                                );
                                if *__slate_slot_855 & (16 as i32) == (0 as i32) {
                                    unsafe { sqlite3_result_subtype(ctx, (74 as i32) as u32) };
                                    break '__join_0;
                                } else {
                                    break '__join_0;
                                }
                            } else {
                                break '__join_0;
                            }
                        }
                        jsonBadPathError(ctx, *__slate_slot_858, 0 as i32);
                        break '__join_0;
                    }
                    jsonBadPathError(ctx, *__slate_slot_858, *__slate_slot_860 as i32);
                }
                jsonStringReset(std::ptr::addr_of_mut!(*__slate_slot_857));
                jsonParseFree(*__slate_slot_854);
                return;
            }
        }
    }
    // Return NULL if not found
}

// Return codes for jsonMergePatch()
// Success
// Malformed TARGET blob
// Malformed PATCH blob
// Out-of-memory condition
// Nested too deep
/// RFC-7396 MergePatch for two JSONB blobs.
///
/// pTarget is the target. pPatch is the patch.  The target is updated
/// in place.  The patch is read-only.
///
/// The original RFC-7396 algorithm is this:
///
///   define MergePatch(Target, Patch):
///     if Patch is an Object:
///       if Target is not an Object:
///         Target = {} # Ignore the contents and set it to an empty Object
///     for each Name/Value pair in Patch:
///         if Value is null:
///           if Name exists in Target:
///             remove the Name/Value pair from Target
///         else:
///           Target[Name] = MergePatch(Target[Name], Value)
///       return Target
///     else:
///       return Patch
///
/// Here is an equivalent algorithm restructured to show the actual
/// implementation:
///
/// 01   define MergePatch(Target, Patch):
/// 02      if Patch is not an Object:
/// 03         return Patch
/// 04      else: // if Patch is an Object
/// 05         if Target is not an Object:
/// 06            Target = {}
/// 07      for each Name/Value pair in Patch:
/// 08         if Name exists in Target:
/// 09            if Value is null:
/// 10               remove the Name/Value pair from Target
/// 11            else
/// 12               Target[name] = MergePatch(Target[Name], Value)
/// 13         else if Value is not NULL:
/// 14            if Value is not an Object:
/// 15               Target[name] = Value
/// 16            else:
/// 17               Target[name] = MergePatch('{}',value)
/// 18      return Target
///  |
///  ^---- Line numbers referenced in comments in the implementation
///
/// # Arguments
///
/// * `pTarget` - The JSON parser that contains the TARGET
/// * `iTarget` - Index of TARGET in pTarget->aBlob[]
/// * `pPatch` - The PATCH
/// * `iPatch` - Index of PATCH in pPatch->aBlob[]
/// * `iDepth` - Nesting depth
fn jsonMergePatch(
    mut pTarget: *mut JsonParse,
    mut iTarget: u32,
    mut pPatch: *const JsonParse,
    mut iPatch: u32,
    mut iDepth: u32,
) -> i32 {
    let mut x: u8 = 0 as u8; // Type of a single node
    let mut n: u32 = 0 as u32;
    let mut sz: u32 = (0 as i32) as u32; // Return values from jsonbPayloadSize()
    let mut iTCursor: u32 = 0 as u32; // Cursor position while scanning the target object
    let mut iTStart: u32 = 0 as u32; // First label in the target object
    let mut iTEndBE: u32 = 0 as u32; // Original first byte past end of target, before edit
    let mut iTEnd: u32 = 0 as u32; // Current first byte past end of target
    let mut eTLabel: u8 = 0 as u8; // Node type of the target label
    let mut iTLabel: u32 = (0 as i32) as u32; // Index of the label
    let mut nTLabel: u32 = (0 as i32) as u32; // Header size in bytes for the target label
    let mut szTLabel: u32 = (0 as i32) as u32; // Size of the target label payload
    let mut iTValue: u32 = (0 as i32) as u32; // Index of the target value
    let mut nTValue: u32 = (0 as i32) as u32; // Header size of the target value
    let mut szTValue: u32 = (0 as i32) as u32; // Payload size for the target value
    let mut iPCursor: u32 = 0 as u32; // Cursor position while scanning the patch
    let mut iPEnd: u32 = 0 as u32; // First byte past the end of the patch
    let mut ePLabel: u8 = 0 as u8; // Node type of the patch label
    let mut iPLabel: u32 = 0 as u32; // Start of patch label
    let mut nPLabel: u32 = 0 as u32; // Size of header on the patch label
    let mut szPLabel: u32 = 0 as u32; // Payload size of the patch label
    let mut iPValue: u32 = 0 as u32; // Start of patch value
    let mut nPValue: u32 = 0 as u32; // Header size for the patch value
    let mut szPValue: u32 = 0 as u32; // Payload size of the patch value
    0 as i32;
    0 as i32;
    x = (((((unsafe { *unsafe { unsafe { (*pPatch).aBlob }.offset(iPatch as isize) } }) as u32)
        as i32)
        & (15 as i32)) as i8) as u8;
    if ((x as u32) as i32) != (12 as i32) {
        // Algorithm line 02
        let mut szPatch: u32 = 0 as u32; // Total size of the patch, header+payload
        let mut szTarget: u32 = 0 as u32; // Total size of the target, header+payload
        n = jsonbPayloadSize(pPatch, iPatch, std::ptr::addr_of_mut!(sz));
        szPatch = n.wrapping_add(sz);
        sz = (0 as i32) as u32;
        n = jsonbPayloadSize(
            pTarget as *const JsonParse,
            iTarget,
            std::ptr::addr_of_mut!(sz),
        );
        szTarget = n.wrapping_add(sz);
        jsonBlobEdit(
            pTarget,
            iTarget,
            szTarget,
            (unsafe { unsafe { (*pPatch).aBlob }.offset(iPatch as isize) }) as *const u8,
            szPatch,
        );
        return if (unsafe { (*pTarget).oom }) != (0 as u8) {
            3 as i32
        } else {
            0 as i32
        }; // Line 03
    }
    x = (((((unsafe { *unsafe { unsafe { (*pTarget).aBlob }.offset(iTarget as isize) } }) as u32)
        as i32)
        & (15 as i32)) as i8) as u8;
    if ((x as u32) as i32) != (12 as i32) {
        // Algorithm line 05
        n = jsonbPayloadSize(
            pTarget as *const JsonParse,
            iTarget,
            std::ptr::addr_of_mut!(sz),
        );
        jsonBlobEdit(
            pTarget,
            iTarget.wrapping_add(n),
            sz,
            std::ptr::null::<u8>(),
            (0 as i32) as u32,
        );
        x = unsafe { *unsafe { unsafe { (*pTarget).aBlob }.offset(iTarget as isize) } };
        unsafe {
            *unsafe { unsafe { (*pTarget).aBlob }.offset(iTarget as isize) } =
                ((((x as u32) as i32) & (240 as i32) | (12 as i32)) as i8) as u8;
        }
    }
    n = jsonbPayloadSize(pPatch, iPatch, std::ptr::addr_of_mut!(sz));
    if n == ((0 as i32) as u32) {
        return 2 as i32;
    }
    iPCursor = iPatch.wrapping_add(n);
    iPEnd = iPCursor.wrapping_add(sz);
    n = jsonbPayloadSize(
        pTarget as *const JsonParse,
        iTarget,
        std::ptr::addr_of_mut!(sz),
    );
    if n == ((0 as i32) as u32) {
        return 1 as i32;
    }
    iTStart = iTarget.wrapping_add(n);
    iTEndBE = iTStart.wrapping_add(sz);
    '__slate_break_1419: while iPCursor < iPEnd {
        // Algorithm line 07
        iPLabel = iPCursor;
        ePLabel = (((((unsafe { *unsafe { unsafe { (*pPatch).aBlob }.offset(iPCursor as isize) } })
            as u32) as i32)
            & (15 as i32)) as i8) as u8;
        if ((ePLabel as u32) as i32) < (7 as i32) || ((ePLabel as u32) as i32) > (10 as i32) {
            return 2 as i32;
        }
        nPLabel = jsonbPayloadSize(pPatch, iPCursor, std::ptr::addr_of_mut!(szPLabel));
        if nPLabel == ((0 as i32) as u32) {
            return 2 as i32;
        }
        iPValue = iPCursor.wrapping_add(nPLabel).wrapping_add(szPLabel);
        if iPValue >= iPEnd {
            return 2 as i32;
        }
        nPValue = jsonbPayloadSize(pPatch, iPValue, std::ptr::addr_of_mut!(szPValue));
        if nPValue == ((0 as i32) as u32) {
            return 2 as i32;
        }
        iPCursor = iPValue.wrapping_add(nPValue).wrapping_add(szPValue);
        if iPCursor > iPEnd {
            return 2 as i32;
        }
        iTCursor = iTStart;
        iTEnd = iTEndBE.wrapping_add((unsafe { (*pTarget).delta }) as u32);
        '__slate_break_1420: while iTCursor < iTEnd {
            let mut isEqual: i32 = 0 as i32; // true if the patch and target labels match
            iTLabel = iTCursor;
            eTLabel =
                (((((unsafe { *unsafe { unsafe { (*pTarget).aBlob }.offset(iTCursor as isize) } })
                    as u32) as i32)
                    & (15 as i32)) as i8) as u8;
            if ((eTLabel as u32) as i32) < (7 as i32) || ((eTLabel as u32) as i32) > (10 as i32) {
                return 1 as i32;
            }
            nTLabel = jsonbPayloadSize(
                pTarget as *const JsonParse,
                iTCursor,
                std::ptr::addr_of_mut!(szTLabel),
            );
            if nTLabel == ((0 as i32) as u32) {
                return 1 as i32;
            }
            iTValue = iTLabel.wrapping_add(nTLabel).wrapping_add(szTLabel);
            if iTValue >= iTEnd {
                return 1 as i32;
            }
            nTValue = jsonbPayloadSize(
                pTarget as *const JsonParse,
                iTValue,
                std::ptr::addr_of_mut!(szTValue),
            );
            if nTValue == ((0 as i32) as u32) {
                return 1 as i32;
            }
            if iTValue.wrapping_add(nTValue).wrapping_add(szTValue) > iTEnd {
                return 1 as i32;
            }
            isEqual = jsonLabelCompare(
                (unsafe {
                    unsafe { (*pPatch).aBlob }.offset(iPLabel.wrapping_add(nPLabel) as isize)
                }) as *const i8,
                szPLabel,
                (((ePLabel as u32) as i32) == (7 as i32)
                    || ((ePLabel as u32) as i32) == (10 as i32)) as i32,
                (unsafe {
                    unsafe { (*pTarget).aBlob }.offset(iTLabel.wrapping_add(nTLabel) as isize)
                }) as *const i8,
                szTLabel,
                (((eTLabel as u32) as i32) == (7 as i32)
                    || ((eTLabel as u32) as i32) == (10 as i32)) as i32,
            );
            if isEqual != (0 as i32) {
                break '__slate_break_1420;
            }
            iTCursor = iTValue.wrapping_add(nTValue).wrapping_add(szTValue);
        }
        x = (((((unsafe { *unsafe { unsafe { (*pPatch).aBlob }.offset(iPValue as isize) } }) as u32)
            as i32)
            & (15 as i32)) as i8) as u8;
        if iTCursor < iTEnd {
            // A match was found.  Algorithm line 08
            if ((x as u32) as i32) == (0 as i32) {
                // Patch value is NULL.  Algorithm line 09
                jsonBlobEdit(
                    pTarget,
                    iTLabel,
                    nTLabel
                        .wrapping_add(szTLabel)
                        .wrapping_add(nTValue)
                        .wrapping_add(szTValue),
                    std::ptr::null::<u8>(),
                    (0 as i32) as u32,
                );
                //  vvvvvv----- No OOM on a delete-only edit
                if (unsafe { (*pTarget).oom }) != (0 as u8) {
                    return 3 as i32;
                }
            } else {
                // Algorithm line 12
                let mut rc: i32 = 0 as i32;
                let mut savedDelta: i32 = unsafe { (*pTarget).delta };
                unsafe {
                    (*pTarget).delta = 0 as i32;
                }
                if iDepth >= ((1000 as i32) as u32) {
                    return 4 as i32;
                }
                rc = jsonMergePatch(
                    pTarget,
                    iTValue,
                    pPatch,
                    iPValue,
                    iDepth.wrapping_add((1 as i32) as u32),
                );
                if rc != (0 as i32) {
                    return rc;
                }
                let __v2022: *mut JsonParse = pTarget;
                let __v2023: i32 = unsafe { (*__v2022).delta };
                let __v2024: i32 = __v2023 + savedDelta;
                unsafe {
                    (*__v2022).delta = __v2024;
                }
            }
        } else {
            if ((x as u32) as i32) > (0 as i32) {
                // Algorithm line 13
                // No match and patch value is not NULL
                let mut szNew: u32 = szPLabel.wrapping_add(nPLabel);
                if (((unsafe { *unsafe { unsafe { (*pPatch).aBlob }.offset(iPValue as isize) } })
                    as u32) as i32)
                    & (15 as i32)
                    != (12 as i32)
                {
                    // Line 14
                    jsonBlobEdit(
                        pTarget,
                        iTEnd,
                        (0 as i32) as u32,
                        std::ptr::null::<u8>(),
                        szPValue.wrapping_add(nPValue).wrapping_add(szNew),
                    );
                    if (unsafe { (*pTarget).oom }) != (0 as u8) {
                        return 3 as i32;
                    }
                    unsafe {
                        memcpy(
                            (unsafe { unsafe { (*pTarget).aBlob }.offset(iTEnd as isize) })
                                as *mut (),
                            (unsafe { unsafe { (*pPatch).aBlob }.offset(iPLabel as isize) })
                                as *const (),
                            szNew as u64,
                        )
                    };
                    unsafe {
                        memcpy(
                            (unsafe {
                                unsafe { (*pTarget).aBlob }
                                    .offset(iTEnd.wrapping_add(szNew) as isize)
                            }) as *mut (),
                            (unsafe { unsafe { (*pPatch).aBlob }.offset(iPValue as isize) })
                                as *const (),
                            szPValue.wrapping_add(nPValue) as u64,
                        )
                    };
                } else {
                    let mut rc: i32 = 0 as i32;
                    let mut savedDelta: i32 = 0 as i32;
                    jsonBlobEdit(
                        pTarget,
                        iTEnd,
                        (0 as i32) as u32,
                        std::ptr::null::<u8>(),
                        szNew.wrapping_add((1 as i32) as u32),
                    );
                    if (unsafe { (*pTarget).oom }) != (0 as u8) {
                        return 3 as i32;
                    }
                    unsafe {
                        memcpy(
                            (unsafe { unsafe { (*pTarget).aBlob }.offset(iTEnd as isize) })
                                as *mut (),
                            (unsafe { unsafe { (*pPatch).aBlob }.offset(iPLabel as isize) })
                                as *const (),
                            szNew as u64,
                        )
                    };
                    unsafe {
                        *unsafe {
                            unsafe { (*pTarget).aBlob }.offset(iTEnd.wrapping_add(szNew) as isize)
                        } = ((0 as i32) as i8) as u8;
                    }
                    savedDelta = unsafe { (*pTarget).delta };
                    unsafe {
                        (*pTarget).delta = 0 as i32;
                    }
                    if iDepth >= ((1000 as i32) as u32) {
                        return 4 as i32;
                    }
                    rc = jsonMergePatch(
                        pTarget,
                        iTEnd.wrapping_add(szNew),
                        pPatch,
                        iPValue,
                        iDepth.wrapping_add((1 as i32) as u32),
                    );
                    if rc != (0 as i32) {
                        return rc;
                    }
                    let __v2025: *mut JsonParse = pTarget;
                    let __v2026: i32 = unsafe { (*__v2025).delta };
                    let __v2027: i32 = __v2026 + savedDelta;
                    unsafe {
                        (*__v2025).delta = __v2027;
                    }
                }
            }
        }
    }
    if (unsafe { (*pTarget).delta }) != (0 as i32) {
        jsonAfterEditSizeAdjust(pTarget, iTarget);
    }
    return if (unsafe { (*pTarget).oom }) != (0 as u8) {
        3 as i32
    } else {
        0 as i32
    };
}

/// Implementation of the json_mergepatch(JSON1,JSON2) function.  Return a JSON
/// object that is the result of running the RFC 7396 MergePatch() algorithm
/// on the two arguments.
#[unsafe(link_section = ".text.slate_distinct.json.jsonPatchFunc")]
extern "C-unwind" fn jsonPatchFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut pTarget: *mut JsonParse = unsafe { std::mem::zeroed() }; // The TARGET
    let mut pPatch: *mut JsonParse = unsafe { std::mem::zeroed() }; // The PATCH
    let mut rc: i32 = 0 as i32; // Result code
    argc;
    0 as i32;
    pTarget = jsonParseFuncArg(
        ctx,
        unsafe { *unsafe { argv.offset((0 as i32) as isize) } },
        (1 as i32) as u32,
    );
    if pTarget == std::ptr::null_mut::<JsonParse>() {
        return;
    }
    pPatch = jsonParseFuncArg(
        ctx,
        unsafe { *unsafe { argv.offset((1 as i32) as isize) } },
        (0 as i32) as u32,
    );
    if pPatch != std::ptr::null_mut::<JsonParse>() {
        rc = jsonMergePatch(
            pTarget,
            (0 as i32) as u32,
            pPatch as *const JsonParse,
            (0 as i32) as u32,
            (0 as i32) as u32,
        );
        if rc == (0 as i32) {
            jsonReturnParse(ctx, pTarget);
        } else {
            if rc == (3 as i32) {
                unsafe { sqlite3_result_error_nomem(ctx) };
            } else {
                if rc == (4 as i32) {
                    unsafe {
                        sqlite3_result_error(
                            ctx,
                            (b"JSON nested too deep\0".as_ptr() as *mut i8) as *const i8,
                            -(1 as i32),
                        )
                    };
                } else {
                    unsafe {
                        sqlite3_result_error(
                            ctx,
                            (b"malformed JSON\0".as_ptr() as *mut i8) as *const i8,
                            -(1 as i32),
                        )
                    };
                }
            }
        }
        jsonParseFree(pPatch);
    }
    jsonParseFree(pTarget);
}

/// Implementation of the json_object(NAME,VALUE,...) function.  Return a JSON
/// object that contains all name/value given in arguments.  Or if any name
/// is not a string or if any value is a BLOB, throw an error.
#[unsafe(link_section = ".text.slate_distinct.json.jsonObjectFunc")]
extern "C-unwind" fn jsonObjectFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut i: i32 = 0 as i32;
    let mut jx: JsonString = unsafe { std::mem::zeroed() };
    let mut z: *const i8 = unsafe { std::mem::zeroed() };
    let mut n: u32 = 0 as u32;
    if argc & (1 as i32) != (0 as i32) {
        unsafe {
            sqlite3_result_error(
                ctx,
                (b"json_object() requires an even number of arguments\0".as_ptr() as *mut i8)
                    as *const i8,
                -(1 as i32),
            )
        };
        return;
    }
    jsonStringInit(std::ptr::addr_of_mut!(jx), ctx);
    jsonAppendChar(std::ptr::addr_of_mut!(jx), (123 as i32) as i8);
    i = 0 as i32;
    '__slate_break_1424: loop {
        if !(i < argc) {
            break;
        }
        if (unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset(i as isize) } }) })
            != (3 as i32)
        {
            unsafe {
                sqlite3_result_error(
                    ctx,
                    (b"json_object() labels must be TEXT\0".as_ptr() as *mut i8) as *const i8,
                    -(1 as i32),
                )
            };
            jsonStringReset(std::ptr::addr_of_mut!(jx));
            return;
        }
        jsonAppendSeparator(std::ptr::addr_of_mut!(jx));
        z = (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset(i as isize) } }) })
            as *const i8;
        n = (unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset(i as isize) } }) }) as u32;
        jsonAppendString(std::ptr::addr_of_mut!(jx), z, n);
        jsonAppendChar(std::ptr::addr_of_mut!(jx), (58 as i32) as i8);
        jsonAppendSqlValue(std::ptr::addr_of_mut!(jx), unsafe {
            *unsafe { argv.offset((i + (1 as i32)) as isize) }
        });
        let __v2028: i32 = i;
        let __v2029: i32 = __v2028 + (2 as i32);
        i = __v2029;
    }
    jsonAppendChar(std::ptr::addr_of_mut!(jx), (125 as i32) as i8);
    jsonReturnString(
        std::ptr::addr_of_mut!(jx),
        std::ptr::null_mut::<JsonParse>(),
        std::ptr::null_mut::<sqlite3_context>(),
    );
    unsafe { sqlite3_result_subtype(ctx, (74 as i32) as u32) };
}

/// json_remove(JSON, PATH, ...)
///
/// Remove the named elements from JSON and return the result.  malformed
/// JSON or PATH arguments result in an error.
#[unsafe(link_section = ".text.slate_distinct.json.jsonRemoveFunc")]
extern "C-unwind" fn jsonRemoveFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut __slate_storage_2031: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2031: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2031) as *mut i32;
    let mut __slate_storage_2030: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2030: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2030) as *mut i32; // Subroutine return code
    let mut __slate_storage_922: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_922: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_922) as *mut u32; // Loop counter
    let mut __slate_storage_921: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_921: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_921) as *mut i32; // Path of element to be removed
    let mut __slate_storage_920: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_920: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_920) as *mut *const i8; // The parse
    let mut __slate_storage_919: std::mem::MaybeUninit<*mut JsonParse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_919: *mut *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_919) as *mut *mut JsonParse;
    unsafe {
        std::ptr::write(__slate_slot_920, std::ptr::null::<i8>());
        if argc < (1 as i32) {
            return;
        } else {
            *__slate_slot_919 = jsonParseFuncArg(
                ctx,
                unsafe { *unsafe { argv.offset((0 as i32) as isize) } },
                (if argc > (1 as i32) {
                    1 as i32
                } else {
                    0 as i32
                }) as u32,
            );
            if *__slate_slot_919 == std::ptr::null_mut::<JsonParse>() {
                return;
            } else {
                *__slate_slot_921 = 1 as i32;
                '__join_0: {
                    '__join_1: {
                        '__join_8: {
                            '__join_5: {
                                loop {
                                    if *__slate_slot_921 < argc {
                                        *__slate_slot_920 = (unsafe {
                                            sqlite3_value_text(unsafe {
                                                *unsafe { argv.offset(*__slate_slot_921 as isize) }
                                            })
                                        })
                                            as *const i8;
                                        if *__slate_slot_920 == std::ptr::null::<i8>() {
                                            break '__join_0;
                                        } else {
                                            if ((unsafe {
                                                *unsafe {
                                                    (*__slate_slot_920).offset((0 as i32) as isize)
                                                }
                                            })
                                                as i32)
                                                != (36 as i32)
                                            {
                                                break '__join_1;
                                            } else {
                                                if ((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_920)
                                                            .offset((1 as i32) as isize)
                                                    }
                                                })
                                                    as i32)
                                                    == (0 as i32)
                                                {
                                                    break '__join_8;
                                                } else {
                                                    unsafe {
                                                        (*(*__slate_slot_919)).eEdit =
                                                            ((1 as i32) as i8) as u8;
                                                    }
                                                    unsafe {
                                                        (*(*__slate_slot_919)).delta = 0 as i32;
                                                    }
                                                    *__slate_slot_922 = jsonLookupStep(
                                                        *__slate_slot_919,
                                                        (0 as i32) as u32,
                                                        unsafe {
                                                            (*__slate_slot_920)
                                                                .offset((1 as i32) as isize)
                                                        },
                                                        (0 as i32) as u32,
                                                    );
                                                    if *__slate_slot_922 >= (4294967291 as u32) {
                                                        if *__slate_slot_922 == (4294967294 as u32)
                                                        {
                                                        } else {
                                                            break '__join_5;
                                                        }
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_2030,
                                                        *__slate_slot_921,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_2031,
                                                        *__slate_slot_2030 + (1 as i32),
                                                    );
                                                    *__slate_slot_921 = *__slate_slot_2031;
                                                }
                                            }
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                jsonReturnParse(ctx, *__slate_slot_919);
                                jsonParseFree(*__slate_slot_919);
                                return;
                            }
                            jsonBadPathError(ctx, *__slate_slot_920, *__slate_slot_922 as i32);
                            break '__join_0;
                        }
                        // json_remove(j,'$') returns NULL
                        break '__join_0;
                    }
                    jsonBadPathError(ctx, *__slate_slot_920, 0 as i32);
                }
                jsonParseFree(*__slate_slot_919);
                return;
            }
        }
    }
    // No-op
}

/// json_replace(JSON, PATH, VALUE, ...)
///
/// Replace the value at PATH with VALUE.  If PATH does not already exist,
/// this routine is a no-op.  If JSON or PATH is malformed, throw an error.
#[unsafe(link_section = ".text.slate_distinct.json.jsonReplaceFunc")]
extern "C-unwind" fn jsonReplaceFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    if argc < (1 as i32) {
        return;
    }
    if argc & (1 as i32) == (0 as i32) {
        jsonWrongNumArgs(ctx, (b"replace\0".as_ptr() as *mut i8) as *const i8);
        return;
    }
    jsonInsertIntoBlob(ctx, argc, argv, 2 as i32);
}

/// json_set(JSON, PATH, VALUE, ...)
///
/// Set the value at PATH to VALUE.  Create the PATH if it does not already
/// exist.  Overwrite existing values that do exist.
/// If JSON or PATH is malformed, throw an error.
///
/// json_insert(JSON, PATH, VALUE, ...)
///
/// Create PATH and initialize it to VALUE.  If PATH already exists, this
/// routine is a no-op.  If JSON or PATH is malformed, throw an error.
#[unsafe(link_section = ".text.slate_distinct.json.jsonSetFunc")]
extern "C-unwind" fn jsonSetFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut flags: i32 = ((unsafe { sqlite3_user_data(ctx) }) as i64) as i32;
    let mut eInsType: i32 = (flags & (12 as i32)) >> (2 as i32);
    if argc < (1 as i32) {
        return;
    }
    0 as i32;
    if argc & (1 as i32) == (0 as i32) {
        jsonWrongNumArgs(ctx, unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!(azInsType.0) as *mut *const i8 }
                    .offset(eInsType as isize)
            }
        });
        return;
    }
    jsonInsertIntoBlob(
        ctx,
        argc,
        argv,
        ((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(aEditType) as *const u8 }.offset(eInsType as isize)
            }
        }) as u32) as i32,
    );
}

static mut azInsType: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"insert\0".as_ptr() as *mut i8) as *const i8,
    (b"set\0".as_ptr() as *mut i8) as *const i8,
    (b"array_insert\0".as_ptr() as *mut i8) as *const i8,
]);

static mut aEditType: [u8; 3] = [
    ((3 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
];

/// json_type(JSON)
/// json_type(JSON, PATH)
///
/// Return the top-level "type" of a JSON string.  json_type() raises an
/// error if either the JSON or PATH inputs are not well-formed.
#[unsafe(link_section = ".text.slate_distinct.json.jsonTypeFunc")]
extern "C-unwind" fn jsonTypeFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut __slate_storage_942: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_942: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_942) as *mut u32;
    let mut __slate_storage_941: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_941: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_941) as *mut *const i8; // The parse
    let mut __slate_storage_940: std::mem::MaybeUninit<*mut JsonParse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_940: *mut *mut JsonParse =
        std::ptr::addr_of_mut!(__slate_storage_940) as *mut *mut JsonParse;
    unsafe {
        std::ptr::write(__slate_slot_941, std::ptr::null::<i8>());
        *__slate_slot_940 = jsonParseFuncArg(
            ctx,
            unsafe { *unsafe { argv.offset((0 as i32) as isize) } },
            (0 as i32) as u32,
        );
        if *__slate_slot_940 == std::ptr::null_mut::<JsonParse>() {
            return;
        } else {
            '__join_0: {
                if argc == (2 as i32) {
                    *__slate_slot_941 = (unsafe {
                        sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
                    }) as *const i8;
                    if *__slate_slot_941 == std::ptr::null::<i8>() {
                        break '__join_0;
                    } else {
                        if ((unsafe { *unsafe { (*__slate_slot_941).offset((0 as i32) as isize) } })
                            as i32)
                            != (36 as i32)
                        {
                            jsonBadPathError(ctx, *__slate_slot_941, 0 as i32);
                            break '__join_0;
                        } else {
                            *__slate_slot_942 = jsonLookupStep(
                                *__slate_slot_940,
                                (0 as i32) as u32,
                                unsafe { (*__slate_slot_941).offset((1 as i32) as isize) },
                                (0 as i32) as u32,
                            );
                            if *__slate_slot_942 >= (4294967291 as u32) {
                                if *__slate_slot_942 == (4294967294 as u32) {
                                    // no-op
                                    break '__join_0;
                                } else {
                                    jsonBadPathError(
                                        ctx,
                                        *__slate_slot_941,
                                        *__slate_slot_942 as i32,
                                    );
                                    break '__join_0;
                                }
                            }
                        }
                    }
                } else {
                    *__slate_slot_942 = (0 as i32) as u32;
                }
                unsafe {
                    sqlite3_result_text(
                        ctx,
                        unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(jsonbType.0) as *const *const i8 }
                                    .offset(
                                        ((((unsafe {
                                            *unsafe {
                                                unsafe { (*(*__slate_slot_940)).aBlob }
                                                    .offset(*__slate_slot_942 as isize)
                                            }
                                        }) as u32)
                                            as i32)
                                            & (15 as i32))
                                            as isize,
                                    )
                            }
                        },
                        -(1 as i32),
                        None,
                    )
                };
            }
            jsonParseFree(*__slate_slot_940);
        }
    }
}

/// json_pretty(JSON)
/// json_pretty(JSON, INDENT)
///
/// Return text that is a pretty-printed rendering of the input JSON.
/// If the argument is not valid JSON, return NULL.
///
/// The INDENT argument is text that is used for indentation.  If omitted,
/// it defaults to four spaces (the same as PostgreSQL).
#[unsafe(link_section = ".text.slate_distinct.json.jsonPrettyFunc")]
extern "C-unwind" fn jsonPrettyFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut s: JsonString = unsafe { std::mem::zeroed() }; // The output string
    let mut x: JsonPretty = unsafe { std::mem::zeroed() }; // Pretty printing context
    unsafe { memset(std::ptr::addr_of_mut!(x) as *mut (), 0 as i32, 32 as u64) };
    x.pParse = jsonParseFuncArg(
        ctx,
        unsafe { *unsafe { argv.offset((0 as i32) as isize) } },
        (0 as i32) as u32,
    );
    if x.pParse == std::ptr::null_mut::<JsonParse>() {
        return;
    }
    x.pOut = std::ptr::addr_of_mut!(s);
    jsonStringInit(std::ptr::addr_of_mut!(s), ctx);
    let __v2032: bool;
    if argc == (1 as i32) {
        __v2032 = true as bool;
    } else {
        let __v2033: *const i8 = (unsafe {
            sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
        }) as *const i8;
        x.zIndent = __v2033;
        __v2032 = __v2033 == std::ptr::null::<i8>();
    }
    if __v2032 {
        x.zIndent = (b"    \0".as_ptr() as *mut i8) as *const i8;
        x.szIndent = (4 as i32) as u32;
    } else {
        x.szIndent = (unsafe { strlen(x.zIndent) }) as u32;
    }
    jsonTranslateBlobToPrettyText(std::ptr::addr_of_mut!(x), (0 as i32) as u32);
    jsonReturnString(
        std::ptr::addr_of_mut!(s),
        std::ptr::null_mut::<JsonParse>(),
        std::ptr::null_mut::<sqlite3_context>(),
    );
    jsonParseFree(x.pParse);
}

/// json_valid(JSON)
/// json_valid(JSON, FLAGS)
///
/// Check the JSON argument to see if it is well-formed.  The FLAGS argument
/// encodes the various constraints on what is meant by "well-formed":
///
///     0x01      Canonical RFC-8259 JSON text
///     0x02      JSON text with optional JSON-5 extensions
///     0x04      Superficially appears to be JSONB
///     0x08      Strictly well-formed JSONB
///
/// If the FLAGS argument is omitted, it defaults to 1.  Useful values for
/// FLAGS include:
///
///    1          Strict canonical JSON text
///    2          JSON text perhaps with JSON-5 extensions
///    4          Superficially appears to be JSONB
///    5          Canonical JSON text or superficial JSONB
///    6          JSON-5 text or superficial JSONB
///    8          Strict JSONB
///    9          Canonical JSON text or strict JSONB
///    10         JSON-5 text or strict JSONB
///
/// Other flag combinations are redundant.  For example, every canonical
/// JSON text is also well-formed JSON-5 text, so FLAG values 2 and 3
/// are the same.  Similarly, any input that passes a strict JSONB validation
/// will also pass the superficial validation so 12 through 15 are the same
/// as 8 through 11 respectively.
///
/// This routine runs in linear time to validate text and when doing strict
/// JSONB validation.  Superficial JSONB validation is constant time,
/// assuming the BLOB is already in memory.  The performance advantage
/// of superficial JSONB validation is why that option is provided.
/// Application developers can choose to do fast superficial validation or
/// slower strict validation, according to their specific needs.
///
/// Only the lower four bits of the FLAGS argument are currently used.
/// Higher bits are reserved for future expansion.   To facilitate
/// compatibility, the current implementation raises an error if any bit
/// in FLAGS is set other than the lower four bits.
///
/// The original circa 2015 implementation of the JSON routines in
/// SQLite only supported canonical RFC-8259 JSON text and the json_valid()
/// function only accepted one argument.  That is why the default value
/// for the FLAGS argument is 1, since FLAGS=1 causes this routine to only
/// recognize canonical RFC-8259 JSON text as valid.  The extra FLAGS
/// argument was added when the JSON routines were extended to support
/// JSON5-like extensions and binary JSONB stored in BLOBs.
///
/// Return Values:
///
///   *   Raise an error if FLAGS is outside the range of 1 to 15.
///   *   Return NULL if the input is NULL
///   *   Return 1 if the input is well-formed.
///   *   Return 0 if the input is not well-formed.
#[unsafe(link_section = ".text.slate_distinct.json.jsonValidFunc")]
extern "C-unwind" fn jsonValidFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut p: *mut JsonParse = unsafe { std::mem::zeroed() }; // The parse
    let mut flags: u8 = ((1 as i32) as i8) as u8;
    let mut res: u8 = ((0 as i32) as i8) as u8;
    if argc == (2 as i32) {
        let mut f: i64 =
            unsafe { sqlite3_value_int64(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) };
        if f < ((1 as i32) as i64) || f > ((15 as i32) as i64) {
            unsafe {
                sqlite3_result_error(
                    ctx,
                    (b"FLAGS parameter to json_valid() must be between 1 and 15\0".as_ptr()
                        as *mut i8) as *const i8,
                    -(1 as i32),
                )
            };
            return;
        }
        flags = ((f & ((15 as i32) as i64)) as i8) as u8;
    }
    // Superficial checking only - accomplished by the
    // jsonArgIsJsonb() call above.
    // Strict checking.  Check by translating BLOB->TEXT->BLOB.  If
    // no errors occur, call that a "strict check".
    // Fall through into interpreting the input as text.  See note
    // above at tag-20240123-a.
    //
    // no break
    // no-op
    '__slate_break_1433: {
        match unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) }
        {
            5 => {
                return;
            }
            4 => {
                let mut py: JsonParse = unsafe { std::mem::zeroed() };
                unsafe { memset(std::ptr::addr_of_mut!(py) as *mut (), 0 as i32, 72 as u64) };
                if jsonArgIsJsonb(
                    unsafe { *unsafe { argv.offset((0 as i32) as isize) } },
                    std::ptr::addr_of_mut!(py),
                ) != (0 as i32)
                {
                    if ((flags as u32) as i32) & (4 as i32) != (0 as i32) {
                        // Superficial checking only - accomplished by the
                        // jsonArgIsJsonb() call above.
                        res = ((1 as i32) as i8) as u8;
                    } else {
                        if ((flags as u32) as i32) & (8 as i32) != (0 as i32) {
                            // Strict checking.  Check by translating BLOB->TEXT->BLOB.  If
                            // no errors occur, call that a "strict check".
                            res = (((0 as i32) as u32)
                                == jsonbValidityCheck(
                                    std::ptr::addr_of_mut!(py) as *const JsonParse,
                                    (0 as i32) as u32,
                                    py.nBlob,
                                    (1 as i32) as u32,
                                )) as u8;
                        }
                    }
                } else {
                    // Fall through into interpreting the input as text.  See note
                    // above at tag-20240123-a.
                    //
                    // no break
                    {}
                    let mut px: JsonParse = unsafe { std::mem::zeroed() };
                    if ((flags as u32) as i32) & (3 as i32) == (0 as i32) {
                    } else {
                        unsafe {
                            memset(std::ptr::addr_of_mut!(px) as *mut (), 0 as i32, 72 as u64)
                        };
                        p = jsonParseFuncArg(
                            ctx,
                            unsafe { *unsafe { argv.offset((0 as i32) as isize) } },
                            (2 as i32) as u32,
                        );
                        if p != std::ptr::null_mut::<JsonParse>() {
                            if (unsafe { (*p).oom }) != (0 as u8) {
                                unsafe { sqlite3_result_error_nomem(ctx) };
                            } else {
                                if (unsafe { (*p).nErr }) != (0 as u8) {
                                    // no-op
                                } else {
                                    if ((flags as u32) as i32) & (2 as i32) != (0 as i32)
                                        || (((unsafe { (*p).hasNonstd }) as u32) as i32)
                                            == (0 as i32)
                                    {
                                        res = ((1 as i32) as i8) as u8;
                                    }
                                }
                            }
                            jsonParseFree(p);
                        } else {
                            unsafe { sqlite3_result_error_nomem(ctx) };
                        }
                    }
                }
            }
            _ => {
                let mut px: JsonParse = unsafe { std::mem::zeroed() };
                if ((flags as u32) as i32) & (3 as i32) == (0 as i32) {
                } else {
                    unsafe { memset(std::ptr::addr_of_mut!(px) as *mut (), 0 as i32, 72 as u64) };
                    p = jsonParseFuncArg(
                        ctx,
                        unsafe { *unsafe { argv.offset((0 as i32) as isize) } },
                        (2 as i32) as u32,
                    );
                    if p != std::ptr::null_mut::<JsonParse>() {
                        if (unsafe { (*p).oom }) != (0 as u8) {
                            unsafe { sqlite3_result_error_nomem(ctx) };
                        } else {
                            if (unsafe { (*p).nErr }) != (0 as u8) {
                                // no-op
                            } else {
                                if ((flags as u32) as i32) & (2 as i32) != (0 as i32)
                                    || (((unsafe { (*p).hasNonstd }) as u32) as i32) == (0 as i32)
                                {
                                    res = ((1 as i32) as i8) as u8;
                                }
                            }
                        }
                        jsonParseFree(p);
                    } else {
                        unsafe { sqlite3_result_error_nomem(ctx) };
                    }
                }
            }
        }
    }
    unsafe { sqlite3_result_int(ctx, (res as u32) as i32) };
}

/// json_error_position(JSON)
///
/// If the argument is NULL, return NULL
///
/// If the argument is BLOB, do a full validity check and return non-zero
/// if the check fails.  The return value is the approximate 1-based offset
/// to the byte of the element that contains the first error.
///
/// Otherwise interpret the argument is TEXT (even if it is numeric) and
/// return the 1-based character position for where the parser first recognized
/// that the input was not valid JSON, or return 0 if the input text looks
/// ok.  JSON-5 extensions are accepted.
#[unsafe(link_section = ".text.slate_distinct.json.jsonErrorFunc")]
extern "C-unwind" fn jsonErrorFunc(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut iErrPos: i64 = (0 as i32) as i64; // Error position to be returned
    let mut s: JsonParse = unsafe { std::mem::zeroed() };
    0 as i32;
    argc;
    unsafe { memset(std::ptr::addr_of_mut!(s) as *mut (), 0 as i32, 72 as u64) };
    s.db = unsafe { sqlite3_context_db_handle(ctx) };
    if jsonArgIsJsonb(
        unsafe { *unsafe { argv.offset((0 as i32) as isize) } },
        std::ptr::addr_of_mut!(s),
    ) != (0 as i32)
    {
        iErrPos = (jsonbValidityCheck(
            std::ptr::addr_of_mut!(s) as *const JsonParse,
            (0 as i32) as u32,
            s.nBlob,
            (1 as i32) as u32,
        ) as u64) as i64;
    } else {
        s.zJson = (unsafe {
            sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
        }) as *mut i8;
        if s.zJson == std::ptr::null_mut::<i8>() {
            return;
        }
        // NULL input or OOM
        s.nJson =
            unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
        if jsonConvertTextToBlob(
            std::ptr::addr_of_mut!(s),
            std::ptr::null_mut::<sqlite3_context>(),
        ) != (0 as i32)
        {
            if s.oom != (0 as u8) {
                iErrPos = -(1 as i32) as i64;
            } else {
                // Convert byte-offset s.iErr into a character offset
                let mut k: u32 = 0 as u32;
                0 as i32; // Because s.oom is false
                k = (0 as i32) as u32;
                '__slate_break_1434: while k < s.iErr
                    && (unsafe { *unsafe { s.zJson.offset(k as isize) } }) != (0 as i8)
                {
                    if ((unsafe { *unsafe { s.zJson.offset(k as isize) } }) as i32) & (192 as i32)
                        != (128 as i32)
                    {
                        let __v2036: i64 = iErrPos;
                        let __v2037: i64 = __v2036 + ((1 as i32) as i64);
                        iErrPos = __v2037;
                    }
                    let __v2034: u32 = k;
                    let __v2035: u32 = __v2034.wrapping_add((1 as i32) as u32);
                    k = __v2035;
                }
                let __v2038: i64 = iErrPos;
                let __v2039: i64 = __v2038 + ((1 as i32) as i64);
                iErrPos = __v2039;
            }
        }
    }
    jsonParseReset(std::ptr::addr_of_mut!(s));
    if iErrPos < ((0 as i32) as i64) {
        unsafe { sqlite3_result_error_nomem(ctx) };
    } else {
        unsafe { sqlite3_result_int64(ctx, iErrPos) };
    }
}

/// Aggregate SQL function implementations
///
/// json_group_array(VALUE)
///
/// Return a JSON array composed of all values in the aggregate.
#[unsafe(link_section = ".text.slate_distinct.json.jsonArrayStep")]
extern "C-unwind" fn jsonArrayStep(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut pStr: *mut JsonString = unsafe { std::mem::zeroed() };
    argc;
    pStr = (unsafe { sqlite3_aggregate_context(ctx, ((136 as u64) as u32) as i32) })
        as *mut JsonString;
    if pStr != std::ptr::null_mut::<JsonString>() {
        if (unsafe { (*pStr).zBuf }) == std::ptr::null_mut::<i8>() {
            jsonStringInit(pStr, ctx);
            jsonAppendChar(pStr, (91 as i32) as i8);
        } else {
            if (unsafe { (*pStr).nUsed }) > (((1 as i32) as i64) as u64) {
                jsonAppendChar(pStr, (44 as i32) as i8);
            }
        }
        unsafe {
            (*pStr).pCtx = ctx;
        }
        jsonAppendSqlValue(pStr, unsafe {
            *unsafe { argv.offset((0 as i32) as isize) }
        });
    }
}

fn jsonArrayCompute(mut ctx: *mut sqlite3_context, mut isFinal: i32) {
    let mut pStr: *mut JsonString = unsafe { std::mem::zeroed() };
    let mut flags: i32 = ((unsafe { sqlite3_user_data(ctx) }) as i64) as i32;
    pStr = (unsafe { sqlite3_aggregate_context(ctx, 0 as i32) }) as *mut JsonString;
    if pStr != std::ptr::null_mut::<JsonString>() {
        unsafe {
            (*pStr).pCtx = ctx;
        }
        jsonAppendRawNZ(
            pStr,
            (b"]\0".as_ptr() as *mut i8) as *const i8,
            (2 as i32) as u32,
        );
        jsonStringTrimOneChar(pStr);
        if (unsafe { (*pStr).eErr }) != (0 as u8) {
            jsonReturnString(
                pStr,
                std::ptr::null_mut::<JsonParse>(),
                std::ptr::null_mut::<sqlite3_context>(),
            );
            return;
        } else {
            if flags & (16 as i32) != (0 as i32) {
                jsonReturnStringAsBlob(pStr);
                if isFinal != (0 as i32) {
                    if !((unsafe { (*pStr).bStatic }) != (0 as u8)) {
                        unsafe { sqlite3RCStrUnref((unsafe { (*pStr).zBuf }) as *mut ()) };
                    }
                } else {
                    jsonStringTrimOneChar(pStr);
                }
                return;
            } else {
                if isFinal != (0 as i32) {
                    unsafe {
                        sqlite3_result_text(
                            ctx,
                            (unsafe { (*pStr).zBuf }) as *const i8,
                            ((unsafe { (*pStr).nUsed }) as u32) as i32,
                            {
                                let __t0: Option<unsafe extern "C-unwind" fn(*mut ())> =
                                    if (unsafe { (*pStr).bStatic }) != (0 as u8) {
                                        unsafe {
                                            std::mem::transmute::<
                                                usize,
                                                Option<unsafe extern "C-unwind" fn(*mut ())>,
                                            >(
                                                -(1 as i32) as usize
                                            )
                                        }
                                    } else {
                                        unsafe {
                                            std::mem::transmute::<
                                                *const (),
                                                Option<unsafe extern "C-unwind" fn(*mut ())>,
                                            >(
                                                sqlite3RCStrUnref as *const ()
                                            )
                                        }
                                    };
                                __t0
                            },
                        )
                    };
                    unsafe {
                        (*pStr).bStatic = ((1 as i32) as i8) as u8;
                    }
                } else {
                    unsafe {
                        sqlite3_result_text(
                            ctx,
                            (unsafe { (*pStr).zBuf }) as *const i8,
                            ((unsafe { (*pStr).nUsed }) as u32) as i32,
                            unsafe {
                                std::mem::transmute::<
                                    usize,
                                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                                >(-(1 as i32) as usize)
                            },
                        )
                    };
                    jsonStringTrimOneChar(pStr);
                }
            }
        }
    } else {
        if flags & (16 as i32) != (0 as i32) {
            unsafe {
                sqlite3_result_blob(
                    ctx,
                    (unsafe { std::ptr::addr_of!(emptyArray) }) as *const (),
                    1 as i32,
                    None,
                )
            };
        } else {
            unsafe {
                sqlite3_result_text(
                    ctx,
                    (b"[]\0".as_ptr() as *mut i8) as *const i8,
                    2 as i32,
                    None,
                )
            };
        }
    }
    unsafe { sqlite3_result_subtype(ctx, (74 as i32) as u32) };
}

static mut emptyArray: u8 = ((11 as i32) as i8) as u8;

#[unsafe(link_section = ".text.slate_distinct.json.jsonArrayValue")]
extern "C-unwind" fn jsonArrayValue(mut ctx: *mut sqlite3_context) {
    jsonArrayCompute(ctx, 0 as i32);
}

#[unsafe(link_section = ".text.slate_distinct.json.jsonArrayFinal")]
extern "C-unwind" fn jsonArrayFinal(mut ctx: *mut sqlite3_context) {
    jsonArrayCompute(ctx, 1 as i32);
}

/// This method works for both json_group_array() and json_group_object().
/// It works by removing the first element of the group by searching forward
/// to the first comma (",") that is not within a string and deleting all
/// text through that comma.
#[unsafe(link_section = ".text.slate_distinct.json.jsonGroupInverse")]
extern "C-unwind" fn jsonGroupInverse(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut i: u32 = 0 as u32;
    let mut inStr: i32 = 0 as i32;
    let mut nNest: i32 = 0 as i32;
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    let mut c: i8 = 0 as i8;
    let mut pStr: *mut JsonString = unsafe { std::mem::zeroed() };
    argc;
    argv;
    pStr = (unsafe { sqlite3_aggregate_context(ctx, 0 as i32) }) as *mut JsonString;
    // pStr is always non-NULL since jsonArrayStep() or jsonObjectStep() will
    // always have been called to initialize it
    if !(pStr != std::ptr::null_mut::<JsonString>()) {
        return;
    }
    z = unsafe { (*pStr).zBuf };
    i = (1 as i32) as u32;
    '__slate_break_1437: loop {
        let __v2040: bool;
        if (i as u64) < unsafe { (*pStr).nUsed } {
            let __v2041: i8 = unsafe { *unsafe { z.offset(i as isize) } };
            c = __v2041;
            __v2040 = (__v2041 as i32) != (44 as i32) || inStr != (0 as i32) || nNest != (0 as i32);
        } else {
            __v2040 = false as bool;
        }
        if !__v2040 {
            break;
        }
        if (c as i32) == (34 as i32) {
            inStr = !(inStr != (0 as i32)) as i32;
        } else {
            if (c as i32) == (92 as i32) {
                let __v2044: u32 = i;
                let __v2045: u32 = __v2044.wrapping_add((1 as i32) as u32);
                i = __v2045;
            } else {
                if !(inStr != (0 as i32)) {
                    if (c as i32) == (123 as i32) || (c as i32) == (91 as i32) {
                        let __v2046: i32 = nNest;
                        let __v2047: i32 = __v2046 + (1 as i32);
                        nNest = __v2047;
                    }
                    if (c as i32) == (125 as i32) || (c as i32) == (93 as i32) {
                        let __v2048: i32 = nNest;
                        let __v2049: i32 = __v2048 - (1 as i32);
                        nNest = __v2049;
                    }
                }
            }
        }
        let __v2042: u32 = i;
        let __v2043: u32 = __v2042.wrapping_add((1 as i32) as u32);
        i = __v2043;
    }
    if (i as u64) < unsafe { (*pStr).nUsed } {
        let __v2050: *mut JsonString = pStr;
        let __v2051: u64 = unsafe { (*__v2050).nUsed };
        let __v2052: u64 = __v2051.wrapping_sub(i as u64);
        unsafe {
            (*__v2050).nUsed = __v2052;
        }
        unsafe {
            memmove(
                (unsafe { z.offset((1 as i32) as isize) }) as *mut (),
                (unsafe { z.offset(i.wrapping_add((1 as i32) as u32) as isize) }) as *const (),
                unsafe { (*pStr).nUsed }.wrapping_sub(((1 as i32) as i64) as u64),
            )
        };
        unsafe {
            *unsafe { z.offset((unsafe { (*pStr).nUsed }) as isize) } = (0 as i32) as i8;
        }
    } else {
        unsafe {
            (*pStr).nUsed = ((1 as i32) as i64) as u64;
        }
    }
}

/// json_group_obj(NAME,VALUE)
///
/// Return a JSON object composed of all names and values in the aggregate.
///
/// Rows for which NAME is NULL do not result in a new entry.  However, we
/// do initially insert a "@" entry into the growing string for each null entry
/// and change the first character of the string to "@" to signal that the
/// string contains null entries.  The "@" markers are needed in order to
/// correctly process xInverse() requests.  The initial "@" is converted
/// back into "{" and the "@" null values are removed by jsonObjectCompute().
#[unsafe(link_section = ".text.slate_distinct.json.jsonObjectStep")]
extern "C-unwind" fn jsonObjectStep(
    mut ctx: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut pStr: *mut JsonString = unsafe { std::mem::zeroed() };
    let mut z: *const i8 = unsafe { std::mem::zeroed() };
    let mut n: u32 = 0 as u32;
    argc;
    pStr = (unsafe { sqlite3_aggregate_context(ctx, ((136 as u64) as u32) as i32) })
        as *mut JsonString;
    if pStr != std::ptr::null_mut::<JsonString>() {
        z = (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
            as *const i8;
        n = (unsafe { sqlite3Strlen30(z) }) as u32;
        if (unsafe { (*pStr).zBuf }) == std::ptr::null_mut::<i8>() {
            jsonStringInit(pStr, ctx);
            jsonAppendChar(pStr, (123 as i32) as i8);
        } else {
            if (unsafe { (*pStr).nUsed }) > (((1 as i32) as i64) as u64) {
                jsonAppendChar(pStr, (44 as i32) as i8);
            }
        }
        unsafe {
            (*pStr).pCtx = ctx;
        }
        if z != std::ptr::null::<i8>() {
            jsonAppendString(pStr, z, n);
            jsonAppendChar(pStr, (58 as i32) as i8);
            jsonAppendSqlValue(pStr, unsafe {
                *unsafe { argv.offset((1 as i32) as isize) }
            });
        } else {
            unsafe {
                *unsafe { unsafe { (*pStr).zBuf }.offset((0 as i32) as isize) } = (64 as i32) as i8;
            }
            jsonAppendRawNZ(
                pStr,
                (b"@\0".as_ptr() as *mut i8) as *const i8,
                (1 as i32) as u32,
            );
        }
    }
}

fn jsonObjectCompute(mut ctx: *mut sqlite3_context, mut isFinal: i32) {
    let mut pStr: *mut JsonString = unsafe { std::mem::zeroed() };
    let mut flags: i32 = ((unsafe { sqlite3_user_data(ctx) }) as i64) as i32;
    pStr = (unsafe { sqlite3_aggregate_context(ctx, 0 as i32) }) as *mut JsonString;
    if pStr != std::ptr::null_mut::<JsonString>() {
        let mut pOgStr: *mut JsonString = pStr;
        let mut tmpStr: JsonString = unsafe { std::mem::zeroed() };
        jsonAppendRawNZ(
            pOgStr,
            (b"}\0".as_ptr() as *mut i8) as *const i8,
            (2 as i32) as u32,
        ); // Ensure it is zero-terminated
        jsonStringTrimOneChar(pOgStr); // Remove the zero terminator
        unsafe {
            (*pStr).pCtx = ctx;
        }
        if (unsafe { (*pStr).eErr }) != (0 as u8) {
            jsonReturnString(
                pStr,
                std::ptr::null_mut::<JsonParse>(),
                std::ptr::null_mut::<sqlite3_context>(),
            );
            return;
        }
        if ((unsafe { *unsafe { unsafe { (*pStr).zBuf }.offset((0 as i32) as isize) } }) as i32)
            != (123 as i32)
        {
            // The string contains null entries that need to be removed
            let mut i: u64 = 0 as u64;
            let mut j: u64 = 0 as u64;
            let mut inStr: i32 = 0 as i32;
            if !(isFinal != (0 as i32)) {
                // Work with a temporary copy of the string if this is not the
                // final result
                jsonStringInit(std::ptr::addr_of_mut!(tmpStr), ctx);
                jsonAppendRawNZ(
                    std::ptr::addr_of_mut!(tmpStr),
                    (unsafe { (*pStr).zBuf }) as *const i8,
                    unsafe { (*pStr).nUsed }.wrapping_add(((1 as i32) as i64) as u64) as u32,
                );
                pStr = std::ptr::addr_of_mut!(tmpStr);
                if (unsafe { (*pStr).eErr }) != (0 as u8) {
                    jsonReturnString(
                        pStr,
                        std::ptr::null_mut::<JsonParse>(),
                        std::ptr::null_mut::<sqlite3_context>(),
                    );
                    return;
                }
                jsonStringTrimOneChar(pStr); // Remove zero terminator
            }
            // Fix up the string by changing the initial "@" flag back to
            // to "{" and removing all subsequence "@" entries, with their
            // associated comma delimeters.
            unsafe {
                *unsafe { unsafe { (*pStr).zBuf }.offset((0 as i32) as isize) } =
                    (123 as i32) as i8;
            }
            j = ((1 as i32) as i64) as u64;
            i = ((1 as i32) as i64) as u64;
            '__slate_break_1440: while i < unsafe { (*pStr).nUsed } {
                let mut c: i8 = unsafe { *unsafe { unsafe { (*pStr).zBuf }.offset(i as isize) } };
                if (c as i32) == (34 as i32) {
                    inStr = !(inStr != (0 as i32)) as i32;
                    let __v2055: u64 = j;
                    let __v2056: u64 = __v2055.wrapping_add(((1 as i32) as i64) as u64);
                    j = __v2056;
                    unsafe {
                        *unsafe { unsafe { (*pStr).zBuf }.offset(__v2055 as isize) } =
                            (34 as i32) as i8;
                    }
                } else {
                    if (c as i32) == (92 as i32) {
                        let __v2057: u64 = j;
                        let __v2058: u64 = __v2057.wrapping_add(((1 as i32) as i64) as u64);
                        j = __v2058;
                        unsafe {
                            *unsafe { unsafe { (*pStr).zBuf }.offset(__v2057 as isize) } =
                                (92 as i32) as i8;
                        }
                        let __v2059: u64 = i;
                        let __v2060: u64 = __v2059.wrapping_add(((1 as i32) as i64) as u64);
                        i = __v2060;
                        let __v2061: u64 = j;
                        let __v2062: u64 = __v2061.wrapping_add(((1 as i32) as i64) as u64);
                        j = __v2062;
                        unsafe {
                            *unsafe { unsafe { (*pStr).zBuf }.offset(__v2061 as isize) } = unsafe {
                                *unsafe { unsafe { (*pStr).zBuf }.offset(__v2060 as isize) }
                            };
                        }
                    } else {
                        if (c as i32) == (64 as i32) && !(inStr != (0 as i32)) {
                            0 as i32;
                            if ((unsafe {
                                *unsafe {
                                    unsafe { (*pStr).zBuf }
                                        .offset(i.wrapping_add(((1 as i32) as i64) as u64) as isize)
                                }
                            }) as i32)
                                == (44 as i32)
                            {
                                let __v2063: u64 = i;
                                let __v2064: u64 = __v2063.wrapping_add(((1 as i32) as i64) as u64);
                                i = __v2064;
                            } else {
                                if ((unsafe {
                                    *unsafe {
                                        unsafe { (*pStr).zBuf }.offset(
                                            j.wrapping_sub(((1 as i32) as i64) as u64) as isize,
                                        )
                                    }
                                }) as i32)
                                    == (44 as i32)
                                {
                                    let __v2065: u64 = j;
                                    let __v2066: u64 =
                                        __v2065.wrapping_sub(((1 as i32) as i64) as u64);
                                    j = __v2066;
                                }
                            }
                        } else {
                            let __v2067: u64 = j;
                            let __v2068: u64 = __v2067.wrapping_add(((1 as i32) as i64) as u64);
                            j = __v2068;
                            unsafe {
                                *unsafe { unsafe { (*pStr).zBuf }.offset(__v2067 as isize) } = c;
                            }
                        }
                    }
                }
                let __v2053: u64 = i;
                let __v2054: u64 = __v2053.wrapping_add(((1 as i32) as i64) as u64);
                i = __v2054;
            }
            unsafe {
                *unsafe { unsafe { (*pStr).zBuf }.offset(j as isize) } = (0 as i32) as i8;
            }
            // Restore zero terminator
            unsafe {
                (*pStr).nUsed = j;
            }
            // Truncate the string
        }
        if flags & (16 as i32) != (0 as i32) {
            jsonReturnStringAsBlob(pStr);
            if isFinal != (0 as i32) {
                if !((unsafe { (*pStr).bStatic }) != (0 as u8)) {
                    unsafe { sqlite3RCStrUnref((unsafe { (*pStr).zBuf }) as *mut ()) };
                }
            } else {
                jsonStringTrimOneChar(pOgStr);
            }
        } else {
            if isFinal != (0 as i32) {
                unsafe {
                    sqlite3_result_text(
                        ctx,
                        (unsafe { (*pStr).zBuf }) as *const i8,
                        ((unsafe { (*pStr).nUsed }) as u32) as i32,
                        {
                            let __t0: Option<unsafe extern "C-unwind" fn(*mut ())> =
                                if (unsafe { (*pStr).bStatic }) != (0 as u8) {
                                    unsafe {
                                        std::mem::transmute::<
                                            usize,
                                            Option<unsafe extern "C-unwind" fn(*mut ())>,
                                        >(
                                            -(1 as i32) as usize
                                        )
                                    }
                                } else {
                                    unsafe {
                                        std::mem::transmute::<
                                            *const (),
                                            Option<unsafe extern "C-unwind" fn(*mut ())>,
                                        >(
                                            sqlite3RCStrUnref as *const ()
                                        )
                                    }
                                };
                            __t0
                        },
                    )
                };
                unsafe {
                    (*pStr).bStatic = ((1 as i32) as i8) as u8;
                }
            } else {
                unsafe {
                    sqlite3_result_text(
                        ctx,
                        (unsafe { (*pStr).zBuf }) as *const i8,
                        ((unsafe { (*pStr).nUsed }) as u32) as i32,
                        unsafe {
                            std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                                -(1 as i32) as usize,
                            )
                        },
                    )
                };
                jsonStringTrimOneChar(pOgStr);
            }
        }
        if pStr != pOgStr {
            jsonStringReset(pStr);
        }
    } else {
        if flags & (16 as i32) != (0 as i32) {
            unsafe {
                sqlite3_result_blob(
                    ctx,
                    (unsafe { std::ptr::addr_of!(emptyObject_1009) }) as *const (),
                    1 as i32,
                    None,
                )
            };
        } else {
            unsafe {
                sqlite3_result_text(
                    ctx,
                    (b"{}\0".as_ptr() as *mut i8) as *const i8,
                    2 as i32,
                    None,
                )
            };
        }
    }
    unsafe { sqlite3_result_subtype(ctx, (74 as i32) as u32) };
}

static mut emptyObject_1009: u8 = ((12 as i32) as i8) as u8;

#[unsafe(link_section = ".text.slate_distinct.json.jsonObjectValue")]
extern "C-unwind" fn jsonObjectValue(mut ctx: *mut sqlite3_context) {
    jsonObjectCompute(ctx, 0 as i32);
}

#[unsafe(link_section = ".text.slate_distinct.json.jsonObjectFinal")]
extern "C-unwind" fn jsonObjectFinal(mut ctx: *mut sqlite3_context) {
    jsonObjectCompute(ctx, 1 as i32);
}

/// The json_each virtual table
#[repr(C)]
#[derive(Clone, Copy)]
struct JsonParent {
    /// Start of object or array
    iHead: u32,
    /// Start of the value
    iValue: u32,
    /// First byte past the end
    iEnd: u32,
    /// Length of path
    nPath: u32,
    /// Key for JSONB_ARRAY
    iKey: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct JsonEachCursor {
    /// Base class - must be first
    base: sqlite3_vtab_cursor,
    /// The rowid
    iRowid: u32,
    /// Index in sParse.aBlob[] of current row
    i: u32,
    /// EOF when i equals or exceeds this value
    iEnd: u32,
    /// Size of the root path in bytes
    nRoot: u32,
    /// Type of the container for element i
    eType: u8,
    /// True for json_tree().  False for json_each()
    bRecursive: u8,
    /// 1 for json_each().  2 for jsonb_each()
    eMode: u8,
    /// Current nesting depth
    nParent: u32,
    /// Space allocated for aParent[]
    nParentAlloc: u32,
    /// Parent elements of i
    aParent: *mut JsonParent,
    /// Database connection
    db: *mut sqlite3,
    /// Current path
    path: JsonString,
    /// Parse of the input JSON
    sParse: JsonParse,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct JsonEachConnection {
    /// Base class - must be first
    base: sqlite3_vtab,
    /// Database connection
    db: *mut sqlite3,
    /// 1 for json_each().  2 for jsonb_each()
    eMode: u8,
    /// True for json_tree().  False for json_each()
    bRecursive: u8,
}

/// Constructor for the json_each virtual table
#[unsafe(link_section = ".text.slate_distinct.json.jsonEachConnect")]
extern "C-unwind" fn jsonEachConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut (),
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut pNew: *mut JsonEachConnection = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    // Column numbers
    // The xBestIndex method assumes that the JSON and ROOT columns are
    // the last two columns in the table.  Should this ever changes, be
    // sure to update the xBestIndex method.
    pzErr;
    argv;
    argc;
    pAux;
    rc = unsafe {
        sqlite3_declare_vtab(
            db,
            (b"CREATE TABLE x(key,value,type,atom,id,parent,fullkey,path,json HIDDEN,root HIDDEN)\0"
                .as_ptr() as *mut i8) as *const i8,
        )
    };
    if rc == (0 as i32) {
        pNew = (unsafe { sqlite3DbMallocZero(db, 40 as u64) }) as *mut JsonEachConnection;
        unsafe {
            *ppVtab = pNew as *mut sqlite3_vtab;
        }
        if pNew == std::ptr::null_mut::<JsonEachConnection>() {
            return 7 as i32;
        }
        unsafe { sqlite3_vtab_config(db, 2 as i32) };
        unsafe {
            (*pNew).db = db;
        }
        unsafe {
            (*pNew).eMode = ((if ((unsafe {
                *unsafe {
                    unsafe { *unsafe { argv.offset((0 as i32) as isize) } }
                        .offset((4 as i32) as isize)
                }
            }) as i32)
                == (98 as i32)
            {
                2 as i32
            } else {
                1 as i32
            }) as i8) as u8;
        }
        unsafe {
            (*pNew).bRecursive = (((unsafe {
                *unsafe {
                    unsafe { *unsafe { argv.offset((0 as i32) as isize) } }.offset(
                        ((4 as i32) + (((unsafe { (*pNew).eMode }) as u32) as i32)) as isize,
                    )
                }
            }) as i32)
                == (116 as i32)) as u8;
        }
    }
    return rc;
}

/// destructor for json_each virtual table
#[unsafe(link_section = ".text.slate_distinct.json.jsonEachDisconnect")]
extern "C-unwind" fn jsonEachDisconnect(mut pVtab: *mut sqlite3_vtab) -> i32 {
    let mut p: *mut JsonEachConnection = pVtab as *mut JsonEachConnection;
    unsafe { sqlite3DbFree(unsafe { (*p).db }, pVtab as *mut ()) };
    return 0 as i32;
}

/// constructor for a JsonEachCursor object for json_each()/json_tree().
#[unsafe(link_section = ".text.slate_distinct.json.jsonEachOpen")]
extern "C-unwind" fn jsonEachOpen(
    mut p: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> i32 {
    let mut pVtab: *mut JsonEachConnection = p as *mut JsonEachConnection;
    let mut pCur: *mut JsonEachCursor = unsafe { std::mem::zeroed() };
    p;
    pCur =
        (unsafe { sqlite3DbMallocZero(unsafe { (*pVtab).db }, 264 as u64) }) as *mut JsonEachCursor;
    if pCur == std::ptr::null_mut::<JsonEachCursor>() {
        return 7 as i32;
    }
    unsafe {
        (*pCur).db = unsafe { (*pVtab).db };
    }
    unsafe {
        (*pCur).eMode = unsafe { (*pVtab).eMode };
    }
    unsafe {
        (*pCur).bRecursive = unsafe { (*pVtab).bRecursive };
    }
    jsonStringZero(unsafe { std::ptr::addr_of_mut!((*pCur).path) });
    unsafe {
        *ppCursor = unsafe { std::ptr::addr_of_mut!((*pCur).base) };
    }
    return 0 as i32;
}

/// Reset a JsonEachCursor back to its original state.  Free any memory
/// held.
fn jsonEachCursorReset(mut p: *mut JsonEachCursor) {
    jsonParseReset(unsafe { std::ptr::addr_of_mut!((*p).sParse) });
    jsonStringReset(unsafe { std::ptr::addr_of_mut!((*p).path) });
    unsafe { sqlite3DbFree(unsafe { (*p).db }, (unsafe { (*p).aParent }) as *mut ()) };
    unsafe {
        (*p).iRowid = (0 as i32) as u32;
    }
    unsafe {
        (*p).i = (0 as i32) as u32;
    }
    unsafe {
        (*p).aParent = std::ptr::null_mut::<JsonParent>();
    }
    unsafe {
        (*p).nParent = (0 as i32) as u32;
    }
    unsafe {
        (*p).nParentAlloc = (0 as i32) as u32;
    }
    unsafe {
        (*p).iEnd = (0 as i32) as u32;
    }
    unsafe {
        (*p).eType = ((0 as i32) as i8) as u8;
    }
}

/// Destructor for a jsonEachCursor object
#[unsafe(link_section = ".text.slate_distinct.json.jsonEachClose")]
extern "C-unwind" fn jsonEachClose(mut cur: *mut sqlite3_vtab_cursor) -> i32 {
    let mut p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
    jsonEachCursorReset(p);
    unsafe { sqlite3DbFree(unsafe { (*p).db }, cur as *mut ()) };
    return 0 as i32;
}

/// Return TRUE if the jsonEachCursor object has been advanced off the end
/// of the JSON object
#[unsafe(link_section = ".text.slate_distinct.json.jsonEachEof")]
extern "C-unwind" fn jsonEachEof(mut cur: *mut sqlite3_vtab_cursor) -> i32 {
    let mut p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
    return ((unsafe { (*p).i }) >= unsafe { (*p).iEnd }) as i32;
}

/// If the cursor is currently pointing at the label of a object entry,
/// then return the index of the value.  For all other cases, return the
/// current pointer position, which is the value.
fn jsonSkipLabel(mut p: *mut JsonEachCursor) -> i32 {
    if (((unsafe { (*p).eType }) as u32) as i32) == (12 as i32) {
        let mut sz: u32 = (0 as i32) as u32;
        let mut n: u32 = jsonbPayloadSize(
            (unsafe { std::ptr::addr_of_mut!((*p).sParse) }) as *const JsonParse,
            unsafe { (*p).i },
            std::ptr::addr_of_mut!(sz),
        );
        let __v2069: u32 = sz;
        let __v2070: u32 = __v2069.wrapping_add(unsafe { (*p).i }.wrapping_add(n));
        sz = __v2070;
        if sz >= unsafe { (*p).sParse.nBlob } {
            sz = unsafe { (*p).i };
        }
        return sz as i32;
    } else {
        return (unsafe { (*p).i }) as i32;
    }
    return unsafe { std::mem::zeroed() };
}

/// Append the path name for the current element.
fn jsonAppendPathName(mut p: *mut JsonEachCursor) {
    0 as i32;
    0 as i32;
    if (((unsafe { (*p).eType }) as u32) as i32) == (11 as i32) {
        unsafe {
            jsonPrintf(
                30 as i32,
                unsafe { std::ptr::addr_of_mut!((*p).path) },
                (b"[%lld]\0".as_ptr() as *mut i8) as *const i8,
                unsafe {
                    (*unsafe { unsafe { (*p).aParent }.offset(unsafe { (*p).nParent }.wrapping_sub((1 as i32) as u32) as isize) }).iKey
                },
            )
        };
    } else {
        let mut n: u32 = 0 as u32;
        let mut sz: u32 = (0 as i32) as u32;
        let mut k: u32 = 0 as u32;
        let mut i: u32 = 0 as u32;
        let mut z: *const i8 = unsafe { std::mem::zeroed() };
        let mut needQuote: i32 = 0 as i32;
        n = jsonbPayloadSize(
            (unsafe { std::ptr::addr_of_mut!((*p).sParse) }) as *const JsonParse,
            unsafe { (*p).i },
            std::ptr::addr_of_mut!(sz),
        );
        k = unsafe { (*p).i }.wrapping_add(n);
        z = (unsafe { unsafe { (*p).sParse.aBlob }.offset(k as isize) }) as *const i8;
        if sz == ((0 as i32) as u32)
            || !((((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                        ((((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u8) as u32)
                            as i32) as isize,
                    )
                }
            }) as u32) as i32)
                & (2 as i32)
                != (0 as i32))
        {
            needQuote = 1 as i32;
        } else {
            i = (0 as i32) as u32;
            '__slate_break_1444: while i < sz {
                if !((((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                            ((((unsafe { *unsafe { z.offset(i as isize) } }) as u8) as u32) as i32)
                                as isize,
                        )
                    }
                }) as u32) as i32)
                    & (6 as i32)
                    != (0 as i32))
                {
                    needQuote = 1 as i32;
                    break '__slate_break_1444;
                }
                let __v2071: u32 = i;
                let __v2072: u32 = __v2071.wrapping_add((1 as i32) as u32);
                i = __v2072;
            }
        }
        if needQuote != (0 as i32) {
            unsafe {
                jsonPrintf(
                    sz.wrapping_add((4 as i32) as u32) as i32,
                    unsafe { std::ptr::addr_of_mut!((*p).path) },
                    (b".\"%.*s\"\0".as_ptr() as *mut i8) as *const i8,
                    sz,
                    z,
                )
            };
        } else {
            unsafe {
                jsonPrintf(
                    sz.wrapping_add((2 as i32) as u32) as i32,
                    unsafe { std::ptr::addr_of_mut!((*p).path) },
                    (b".%.*s\0".as_ptr() as *mut i8) as *const i8,
                    sz,
                    z,
                )
            };
        }
    }
}

/// Report a "malformed JSON" or OOM error against the cursor.
fn jsonEachMalformedInput(mut cur: *mut sqlite3_vtab_cursor) -> i32 {
    unsafe { sqlite3_free((unsafe { (*unsafe { (*cur).pVtab }).zErrMsg }) as *mut ()) };
    unsafe {
        (*unsafe { (*cur).pVtab }).zErrMsg =
            unsafe { sqlite3_mprintf((b"malformed JSON\0".as_ptr() as *mut i8) as *const i8) };
    }
    jsonEachCursorReset(cur as *mut JsonEachCursor);
    return if (unsafe { (*unsafe { (*cur).pVtab }).zErrMsg }) != std::ptr::null_mut::<i8>() {
        1 as i32
    } else {
        7 as i32
    };
}

/// Advance the cursor to the next element for json_tree()
#[unsafe(link_section = ".text.slate_distinct.json.jsonEachNext")]
extern "C-unwind" fn jsonEachNext(mut cur: *mut sqlite3_vtab_cursor) -> i32 {
    let mut p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*p).bRecursive }) != (0 as u8) {
        let mut x: u8 = 0 as u8;
        let mut levelChange: u8 = ((0 as i32) as i8) as u8;
        let mut n: u32 = 0 as u32;
        let mut sz: u32 = (0 as i32) as u32;
        let mut i: u32 = jsonSkipLabel(p) as u32;
        x = (((((unsafe { *unsafe { unsafe { (*p).sParse.aBlob }.offset(i as isize) } }) as u32)
            as i32)
            & (15 as i32)) as i8) as u8;
        n = jsonbPayloadSize(
            (unsafe { std::ptr::addr_of_mut!((*p).sParse) }) as *const JsonParse,
            i,
            std::ptr::addr_of_mut!(sz),
        );
        if n == ((0 as i32) as u32) {
            return jsonEachMalformedInput(cur);
        }
        if ((x as u32) as i32) == (12 as i32) || ((x as u32) as i32) == (11 as i32) {
            let mut pParent: *mut JsonParent = unsafe { std::mem::zeroed() };
            if (unsafe { (*p).nParent }) >= unsafe { (*p).nParentAlloc } {
                let mut pNew: *mut JsonParent = unsafe { std::mem::zeroed() };
                let mut nNew: u64 = 0 as u64;
                nNew = unsafe { (*p).nParentAlloc }
                    .wrapping_mul((2 as i32) as u32)
                    .wrapping_add((3 as i32) as u32) as u64;
                pNew = (unsafe {
                    sqlite3DbRealloc(
                        unsafe { (*p).db },
                        (unsafe { (*p).aParent }) as *mut (),
                        (24 as u64).wrapping_mul(nNew),
                    )
                }) as *mut JsonParent;
                if pNew == std::ptr::null_mut::<JsonParent>() {
                    return 7 as i32;
                }
                unsafe {
                    (*p).nParentAlloc = nNew as u32;
                }
                unsafe {
                    (*p).aParent = pNew;
                }
            }
            levelChange = ((1 as i32) as i8) as u8;
            pParent = unsafe { unsafe { (*p).aParent }.offset((unsafe { (*p).nParent }) as isize) };
            unsafe {
                (*pParent).iHead = unsafe { (*p).i };
            }
            unsafe {
                (*pParent).iValue = i;
            }
            unsafe {
                (*pParent).iEnd = i.wrapping_add(n).wrapping_add(sz);
            }
            unsafe {
                (*pParent).iKey = -(1 as i32) as i64;
            }
            unsafe {
                (*pParent).nPath = (unsafe { (*p).path.nUsed }) as u32;
            }
            if (unsafe { (*p).eType }) != (0 as u8) && (unsafe { (*p).nParent }) != (0 as u32) {
                jsonAppendPathName(p);
                if (unsafe { (*p).path.eErr }) != (0 as u8) {
                    rc = 7 as i32;
                }
            }
            let __v2073: *mut JsonEachCursor = p;
            let __v2074: u32 = unsafe { (*__v2073).nParent };
            let __v2075: u32 = __v2074.wrapping_add((1 as i32) as u32);
            unsafe {
                (*__v2073).nParent = __v2075;
            }
            unsafe {
                (*p).i = i.wrapping_add(n);
            }
        } else {
            unsafe {
                (*p).i = i.wrapping_add(n).wrapping_add(sz);
            }
        }
        '__slate_break_1448: while (unsafe { (*p).nParent }) > ((0 as i32) as u32)
            && (unsafe { (*p).i })
                >= unsafe {
                    (*unsafe { unsafe { (*p).aParent }.offset(unsafe { (*p).nParent }.wrapping_sub((1 as i32) as u32) as isize) }).iEnd
                }
        {
            let __v2076: *mut JsonEachCursor = p;
            let __v2077: u32 = unsafe { (*__v2076).nParent };
            let __v2078: u32 = __v2077.wrapping_sub((1 as i32) as u32);
            unsafe {
                (*__v2076).nParent = __v2078;
            }
            unsafe {
                (*p).path.nUsed = (unsafe {
                    (*unsafe { unsafe { (*p).aParent }.offset((unsafe { (*p).nParent }) as isize) })
                        .nPath
                }) as u64;
            }
            levelChange = ((1 as i32) as i8) as u8;
        }
        if levelChange != (0 as u8) {
            if (unsafe { (*p).nParent }) > ((0 as i32) as u32) {
                let mut pParent: *mut JsonParent = unsafe {
                    unsafe { (*p).aParent }
                        .offset(unsafe { (*p).nParent }.wrapping_sub((1 as i32) as u32) as isize)
                };
                let mut iVal: u32 = unsafe { (*pParent).iValue };
                unsafe {
                    (*p).eType = (((((unsafe {
                        *unsafe { unsafe { (*p).sParse.aBlob }.offset(iVal as isize) }
                    }) as u32) as i32)
                        & (15 as i32)) as i8) as u8;
                }
            } else {
                unsafe {
                    (*p).eType = ((0 as i32) as i8) as u8;
                }
            }
        }
    } else {
        let mut n: u32 = 0 as u32;
        let mut sz: u32 = (0 as i32) as u32;
        let mut i: u32 = jsonSkipLabel(p) as u32;
        n = jsonbPayloadSize(
            (unsafe { std::ptr::addr_of_mut!((*p).sParse) }) as *const JsonParse,
            i,
            std::ptr::addr_of_mut!(sz),
        );
        if n == ((0 as i32) as u32) {
            return jsonEachMalformedInput(cur);
        }
        unsafe {
            (*p).i = i.wrapping_add(n).wrapping_add(sz);
        }
    }
    if (((unsafe { (*p).eType }) as u32) as i32) == (11 as i32)
        && (unsafe { (*p).nParent }) != (0 as u32)
    {
        let __v2079: *mut JsonParent = unsafe {
            unsafe { (*p).aParent }
                .offset(unsafe { (*p).nParent }.wrapping_sub((1 as i32) as u32) as isize)
        };
        let __v2080: i64 = unsafe { (*__v2079).iKey };
        let __v2081: i64 = __v2080 + ((1 as i32) as i64);
        unsafe {
            (*__v2079).iKey = __v2081;
        }
    }
    let __v2082: *mut JsonEachCursor = p;
    let __v2083: u32 = unsafe { (*__v2082).iRowid };
    let __v2084: u32 = __v2083.wrapping_add((1 as i32) as u32);
    unsafe {
        (*__v2082).iRowid = __v2084;
    }
    return rc;
}

/// Length of the path for rowid==0 in bRecursive mode.
fn jsonEachPathLength(mut p: *mut JsonEachCursor) -> i32 {
    let mut n: u32 = (unsafe { (*p).path.nUsed }) as u32;
    let mut z: *mut i8 = unsafe { (*p).path.zBuf };
    if (unsafe { (*p).iRowid }) == ((0 as i32) as u32)
        && (unsafe { (*p).bRecursive }) != (0 as u8)
        && n >= ((2 as i32) as u32)
    {
        '__slate_break_1449: while n > ((1 as i32) as u32) {
            let __v2085: u32 = n;
            let __v2086: u32 = __v2085.wrapping_sub((1 as i32) as u32);
            n = __v2086;
            if ((unsafe { *unsafe { z.offset(n as isize) } }) as i32) == (91 as i32)
                || ((unsafe { *unsafe { z.offset(n as isize) } }) as i32) == (46 as i32)
            {
                let mut x: u32 = 0 as u32;
                let mut sz: u32 = (0 as i32) as u32;
                let mut cSaved: i8 = unsafe { *unsafe { z.offset(n as isize) } };
                unsafe {
                    *unsafe { z.offset(n as isize) } = (0 as i32) as i8;
                }
                0 as i32;
                x = jsonLookupStep(
                    unsafe { std::ptr::addr_of_mut!((*p).sParse) },
                    (0 as i32) as u32,
                    (unsafe { z.offset((1 as i32) as isize) }) as *const i8,
                    (0 as i32) as u32,
                );
                unsafe {
                    *unsafe { z.offset(n as isize) } = cSaved;
                }
                if x >= (4294967291 as u32) {
                } else {
                    if x.wrapping_add(jsonbPayloadSize(
                        (unsafe { std::ptr::addr_of_mut!((*p).sParse) }) as *const JsonParse,
                        x,
                        std::ptr::addr_of_mut!(sz),
                    )) == unsafe { (*p).i }
                    {
                        break '__slate_break_1449;
                    }
                }
            }
        }
    }
    return n as i32;
}

/// Return the value of a column
///
/// # Arguments
///
/// * `cur` - The cursor
/// * `ctx` - First argument to sqlite3_result_...()
/// * `iColumn` - Which column to return
#[unsafe(link_section = ".text.slate_distinct.json.jsonEachColumn")]
extern "C-unwind" fn jsonEachColumn(
    mut cur: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut iColumn: i32,
) -> i32 {
    let mut p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
    '__slate_break_1450: {
        match iColumn {
            0 => {
                if (unsafe { (*p).nParent }) == ((0 as i32) as u32) {
                    let mut n: u32 = 0 as u32;
                    let mut j: u32 = 0 as u32;
                    if (unsafe { (*p).nRoot }) == ((1 as i32) as u32) {
                    } else {
                        j = jsonEachPathLength(p) as u32;
                        n = unsafe { (*p).nRoot }.wrapping_sub(j);
                        if n == ((0 as i32) as u32) {
                        } else {
                            if ((unsafe {
                                *unsafe { unsafe { (*p).path.zBuf }.offset(j as isize) }
                            }) as i32)
                                == (91 as i32)
                            {
                                let mut x: i64 = 0 as i64;
                                unsafe {
                                    sqlite3Atoi64(
                                        (unsafe {
                                            unsafe { (*p).path.zBuf }
                                                .offset(j.wrapping_add((1 as i32) as u32) as isize)
                                        }) as *const i8,
                                        std::ptr::addr_of_mut!(x),
                                        n.wrapping_sub((1 as i32) as u32) as i32,
                                        ((1 as i32) as i8) as u8,
                                    )
                                };
                                unsafe { sqlite3_result_int64(ctx, x) };
                            } else {
                                if ((unsafe {
                                    *unsafe {
                                        unsafe { (*p).path.zBuf }
                                            .offset(j.wrapping_add((1 as i32) as u32) as isize)
                                    }
                                }) as i32)
                                    == (34 as i32)
                                {
                                    unsafe {
                                        sqlite3_result_text(
                                            ctx,
                                            (unsafe {
                                                unsafe { (*p).path.zBuf }.offset(
                                                    j.wrapping_add((2 as i32) as u32) as isize,
                                                )
                                            })
                                                as *const i8,
                                            n.wrapping_sub((3 as i32) as u32) as i32,
                                            unsafe {
                                                std::mem::transmute::<
                                                    usize,
                                                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                                                >(
                                                    -(1 as i32) as usize
                                                )
                                            },
                                        )
                                    };
                                } else {
                                    unsafe {
                                        sqlite3_result_text(
                                            ctx,
                                            (unsafe {
                                                unsafe { (*p).path.zBuf }.offset(
                                                    j.wrapping_add((1 as i32) as u32) as isize,
                                                )
                                            })
                                                as *const i8,
                                            n.wrapping_sub((1 as i32) as u32) as i32,
                                            unsafe {
                                                std::mem::transmute::<
                                                    usize,
                                                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                                                >(
                                                    -(1 as i32) as usize
                                                )
                                            },
                                        )
                                    };
                                }
                            }
                        }
                    }
                } else {
                    if (((unsafe { (*p).eType }) as u32) as i32) == (12 as i32) {
                        jsonReturnFromBlob(
                            unsafe { std::ptr::addr_of_mut!((*p).sParse) },
                            unsafe { (*p).i },
                            ctx,
                            1 as i32,
                        );
                    } else {
                        0 as i32;
                        unsafe {
                            sqlite3_result_int64(ctx, unsafe {
                                (*unsafe {
                                    unsafe { (*p).aParent }.offset(
                                        unsafe { (*p).nParent }.wrapping_sub((1 as i32) as u32)
                                            as isize,
                                    )
                                })
                                .iKey
                            })
                        };
                    }
                }
            }
            1 => {
                let mut i: u32 = jsonSkipLabel(p) as u32;
                jsonReturnFromBlob(
                    unsafe { std::ptr::addr_of_mut!((*p).sParse) },
                    i,
                    ctx,
                    ((unsafe { (*p).eMode }) as u32) as i32,
                );
                if (((unsafe { *unsafe { unsafe { (*p).sParse.aBlob }.offset(i as isize) } })
                    as u32) as i32)
                    & (15 as i32)
                    >= (11 as i32)
                {
                    unsafe { sqlite3_result_subtype(ctx, (74 as i32) as u32) };
                }
            }
            2 => {
                let mut i: u32 = jsonSkipLabel(p) as u32;
                let mut eType: u8 =
                    (((((unsafe { *unsafe { unsafe { (*p).sParse.aBlob }.offset(i as isize) } })
                        as u32) as i32)
                        & (15 as i32)) as i8) as u8;
                unsafe {
                    sqlite3_result_text(
                        ctx,
                        unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(jsonbType.0) as *const *const i8 }
                                    .offset(((eType as u32) as i32) as isize)
                            }
                        },
                        -(1 as i32),
                        None,
                    )
                };
            }
            3 => {
                let mut i: u32 = jsonSkipLabel(p) as u32;
                if (((unsafe { *unsafe { unsafe { (*p).sParse.aBlob }.offset(i as isize) } })
                    as u32) as i32)
                    & (15 as i32)
                    < (11 as i32)
                {
                    jsonReturnFromBlob(
                        unsafe { std::ptr::addr_of_mut!((*p).sParse) },
                        i,
                        ctx,
                        1 as i32,
                    );
                }
            }
            4 => {
                unsafe { sqlite3_result_int64(ctx, ((unsafe { (*p).i }) as u64) as i64) };
            }
            5 => {
                if (unsafe { (*p).nParent }) > ((0 as i32) as u32)
                    && (unsafe { (*p).bRecursive }) != (0 as u8)
                {
                    unsafe {
                        sqlite3_result_int64(
                            ctx,
                            ((unsafe {
                                (*unsafe {
                                    unsafe { (*p).aParent }.offset(
                                        unsafe { (*p).nParent }.wrapping_sub((1 as i32) as u32)
                                            as isize,
                                    )
                                })
                                .iHead
                            }) as u64) as i64,
                        )
                    };
                }
            }
            6 => {
                let mut nBase: u64 = unsafe { (*p).path.nUsed };
                if (unsafe { (*p).nParent }) != (0 as u32) {
                    jsonAppendPathName(p);
                }
                unsafe {
                    sqlite3_result_text64(
                        ctx,
                        (unsafe { (*p).path.zBuf }) as *const i8,
                        unsafe { (*p).path.nUsed },
                        unsafe {
                            std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                                -(1 as i32) as usize,
                            )
                        },
                        ((1 as i32) as i8) as u8,
                    )
                };
                unsafe {
                    (*p).path.nUsed = nBase;
                }
            }
            7 => {
                let mut n: u32 = jsonEachPathLength(p) as u32;
                unsafe {
                    sqlite3_result_text64(
                        ctx,
                        (unsafe { (*p).path.zBuf }) as *const i8,
                        n as u64,
                        unsafe {
                            std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                                -(1 as i32) as usize,
                            )
                        },
                        ((1 as i32) as i8) as u8,
                    )
                };
            }
            8 => {
                if (unsafe { (*p).sParse.zJson }) == std::ptr::null_mut::<i8>() {
                    unsafe {
                        sqlite3_result_blob(
                            ctx,
                            (unsafe { (*p).sParse.aBlob }) as *const (),
                            (unsafe { (*p).sParse.nBlob }) as i32,
                            unsafe {
                                std::mem::transmute::<
                                    usize,
                                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                                >(-(1 as i32) as usize)
                            },
                        )
                    };
                } else {
                    unsafe {
                        sqlite3_result_text(
                            ctx,
                            (unsafe { (*p).sParse.zJson }) as *const i8,
                            -(1 as i32),
                            unsafe {
                                std::mem::transmute::<
                                    usize,
                                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                                >(-(1 as i32) as usize)
                            },
                        )
                    };
                }
            }
            _ => {
                unsafe {
                    sqlite3_result_text(
                        ctx,
                        (unsafe { (*p).path.zBuf }) as *const i8,
                        (unsafe { (*p).nRoot }) as i32,
                        None,
                    )
                };
            }
        }
    }
    return 0 as i32;
}

/// Return the current rowid value
#[unsafe(link_section = ".text.slate_distinct.json.jsonEachRowid")]
extern "C-unwind" fn jsonEachRowid(mut cur: *mut sqlite3_vtab_cursor, mut pRowid: *mut i64) -> i32 {
    let mut p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
    unsafe {
        *pRowid = ((unsafe { (*p).iRowid }) as u64) as i64;
    }
    return 0 as i32;
}

/// The query strategy is to look for an equality constraint on the json
/// column.  Without such a constraint, the table cannot operate.  idxNum is
/// 1 if the constraint is found, 3 if the constraint and zRoot are found,
/// and 0 otherwise.
#[unsafe(link_section = ".text.slate_distinct.json.jsonEachBestIndex")]
extern "C-unwind" fn jsonEachBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> i32 {
    let mut i: i32 = 0 as i32; // Loop counter or computed array index
    let mut aIdx: [i32; 2] = [0 as i32; 2]; // Index of constraints for JSON and ROOT
    let mut unusableMask: i32 = 0 as i32; // Mask of unusable JSON and ROOT constraints
    let mut idxMask: i32 = 0 as i32; // Mask of usable == constraints JSON and ROOT
    let mut pConstraint: *const sqlite3_index_constraint = unsafe { std::mem::zeroed() };
    // This implementation assumes that JSON and ROOT are the last two
    // columns in the table
    0 as i32;
    tab;
    let __v2087: i32 = -(1 as i32);
    unsafe {
        *unsafe { (aIdx.as_mut_ptr() as *mut i32).offset((1 as i32) as isize) } = __v2087;
    }
    unsafe {
        *unsafe { (aIdx.as_mut_ptr() as *mut i32).offset((0 as i32) as isize) } = __v2087;
    }
    pConstraint = (unsafe { (*pIdxInfo).aConstraint }) as *const sqlite3_index_constraint;
    i = 0 as i32;
    '__slate_break_1451: while i < unsafe { (*pIdxInfo).nConstraint } {
        let mut iCol: i32 = 0 as i32;
        let mut iMask: i32 = 0 as i32;
        if (unsafe { (*pConstraint).iColumn }) < (8 as i32) {
        } else {
            iCol = (unsafe { (*pConstraint).iColumn }) - (8 as i32);
            0 as i32;
            {}
            iMask = (1 as i32) << iCol;
            if (((unsafe { (*pConstraint).usable }) as u32) as i32) == (0 as i32) {
                let __v2092: i32 = unusableMask;
                let __v2093: i32 = __v2092 | iMask;
                unusableMask = __v2093;
            } else {
                if (((unsafe { (*pConstraint).op }) as u32) as i32) == (2 as i32) {
                    unsafe {
                        *unsafe { (aIdx.as_mut_ptr() as *mut i32).offset(iCol as isize) } = i;
                    }
                    let __v2094: i32 = idxMask;
                    let __v2095: i32 = __v2094 | iMask;
                    idxMask = __v2095;
                }
            }
        }
        let __v2088: i32 = i;
        let __v2089: i32 = __v2088 + (1 as i32);
        i = __v2089;
        let __v2090: *const sqlite3_index_constraint = pConstraint;
        let __v2091: *const sqlite3_index_constraint =
            unsafe { __v2090.offset((1 as i32) as isize) };
        pConstraint = __v2091;
    }
    if (unsafe { (*pIdxInfo).nOrderBy }) > (0 as i32)
        && (unsafe {
            (*unsafe { unsafe { (*pIdxInfo).aOrderBy }.offset((0 as i32) as isize) }).iColumn
        }) < (0 as i32)
        && (((unsafe {
            (*unsafe { unsafe { (*pIdxInfo).aOrderBy }.offset((0 as i32) as isize) }).desc
        }) as u32) as i32)
            == (0 as i32)
    {
        unsafe {
            (*pIdxInfo).orderByConsumed = 1 as i32;
        }
    }
    if unusableMask & !idxMask != (0 as i32) {
        // If there are any unusable constraints on JSON or ROOT, then reject
        // this entire plan
        return 19 as i32;
    }
    if (unsafe { *unsafe { (aIdx.as_mut_ptr() as *mut i32).offset((0 as i32) as isize) } })
        < (0 as i32)
    {
        // No JSON input.  Leave estimatedCost at the huge value that it was
        // initialized to to discourage the query planner from selecting this
        // plan.
        unsafe {
            (*pIdxInfo).idxNum = 0 as i32;
        }
    } else {
        unsafe {
            (*pIdxInfo).estimatedCost = 1.0f64;
        }
        i = unsafe { *unsafe { (aIdx.as_mut_ptr() as *mut i32).offset((0 as i32) as isize) } };
        unsafe {
            (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(i as isize) }).argvIndex =
                1 as i32;
        }
        unsafe {
            (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(i as isize) }).omit =
                ((1 as i32) as i8) as u8;
        }
        if (unsafe { *unsafe { (aIdx.as_mut_ptr() as *mut i32).offset((1 as i32) as isize) } })
            < (0 as i32)
        {
            unsafe {
                (*pIdxInfo).idxNum = 1 as i32;
            }
        // Only JSON supplied.  Plan 1
        } else {
            i = unsafe { *unsafe { (aIdx.as_mut_ptr() as *mut i32).offset((1 as i32) as isize) } };
            unsafe {
                (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(i as isize) })
                    .argvIndex = 2 as i32;
            }
            unsafe {
                (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(i as isize) }).omit =
                    ((1 as i32) as i8) as u8;
            }
            unsafe {
                (*pIdxInfo).idxNum = 3 as i32;
            }
            // Both JSON and ROOT are supplied.  Plan 3
        }
    }
    return 0 as i32;
}

/// Start a search on a new JSON string
#[unsafe(link_section = ".text.slate_distinct.json.jsonEachFilter")]
extern "C-unwind" fn jsonEachFilter(
    mut cur: *mut sqlite3_vtab_cursor,
    mut idxNum: i32,
    mut idxStr: *const i8,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) -> i32 {
    let mut p: *mut JsonEachCursor = cur as *mut JsonEachCursor;
    let mut zRoot: *const i8 = std::ptr::null::<i8>();
    let mut i: u32 = 0 as u32;
    let mut n: u32 = 0 as u32;
    let mut sz: u32 = 0 as u32;
    idxStr;
    argc;
    jsonEachCursorReset(p);
    if idxNum == (0 as i32) {
        return 0 as i32;
    }
    unsafe {
        memset(
            (unsafe { std::ptr::addr_of_mut!((*p).sParse) }) as *mut (),
            0 as i32,
            72 as u64,
        )
    };
    unsafe {
        (*p).sParse.nJPRef = (1 as i32) as u32;
    }
    unsafe {
        (*p).sParse.db = unsafe { (*p).db };
    }
    if jsonArgIsJsonb(
        unsafe { *unsafe { argv.offset((0 as i32) as isize) } },
        unsafe { std::ptr::addr_of_mut!((*p).sParse) },
    ) != (0 as i32)
    {
        // We have JSONB
    } else {
        unsafe {
            (*p).sParse.zJson = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
            }) as *mut i8;
        }
        unsafe {
            (*p).sParse.nJson = unsafe {
                sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
            };
        }
        if (unsafe { (*p).sParse.zJson }) == std::ptr::null_mut::<i8>() {
            unsafe {
                (*p).iEnd = (0 as i32) as u32;
            }
            unsafe {
                (*p).i = (0 as i32) as u32;
            }
            return 0 as i32;
        }
        if jsonConvertTextToBlob(
            unsafe { std::ptr::addr_of_mut!((*p).sParse) },
            std::ptr::null_mut::<sqlite3_context>(),
        ) != (0 as i32)
        {
            if (unsafe { (*p).sParse.oom }) != (0 as u8) {
                return 7 as i32;
            }
            return jsonEachMalformedInput(cur);
        }
    }
    if idxNum == (3 as i32) {
        zRoot = (unsafe {
            sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
        }) as *const i8;
        if zRoot == std::ptr::null::<i8>() {
            return 0 as i32;
        }
        if ((unsafe { *unsafe { zRoot.offset((0 as i32) as isize) } }) as i32) != (36 as i32) {
            unsafe { sqlite3_free((unsafe { (*unsafe { (*cur).pVtab }).zErrMsg }) as *mut ()) };
            unsafe {
                (*unsafe { (*cur).pVtab }).zErrMsg =
                    jsonBadPathError(std::ptr::null_mut::<sqlite3_context>(), zRoot, 0 as i32);
            }
            jsonEachCursorReset(p);
            return if (unsafe { (*unsafe { (*cur).pVtab }).zErrMsg }) != std::ptr::null_mut::<i8>()
            {
                1 as i32
            } else {
                7 as i32
            };
        }
        unsafe {
            (*p).nRoot = (unsafe { sqlite3Strlen30(zRoot) }) as u32;
        }
        if ((unsafe { *unsafe { zRoot.offset((1 as i32) as isize) } }) as i32) == (0 as i32) {
            unsafe {
                (*p).i = (0 as i32) as u32;
            }
            i = (0 as i32) as u32;
            unsafe {
                (*p).eType = ((0 as i32) as i8) as u8;
            }
        } else {
            i = jsonLookupStep(
                unsafe { std::ptr::addr_of_mut!((*p).sParse) },
                (0 as i32) as u32,
                unsafe { zRoot.offset((1 as i32) as isize) },
                (0 as i32) as u32,
            );
            if i >= (4294967291 as u32) {
                if i == (4294967294 as u32) {
                    unsafe {
                        (*p).i = (0 as i32) as u32;
                    }
                    unsafe {
                        (*p).eType = ((0 as i32) as i8) as u8;
                    }
                    unsafe {
                        (*p).iEnd = (0 as i32) as u32;
                    }
                    return 0 as i32;
                }
                unsafe { sqlite3_free((unsafe { (*unsafe { (*cur).pVtab }).zErrMsg }) as *mut ()) };
                unsafe {
                    (*unsafe { (*cur).pVtab }).zErrMsg =
                        jsonBadPathError(std::ptr::null_mut::<sqlite3_context>(), zRoot, 0 as i32);
                }
                jsonEachCursorReset(p);
                return if (unsafe { (*unsafe { (*cur).pVtab }).zErrMsg })
                    != std::ptr::null_mut::<i8>()
                {
                    1 as i32
                } else {
                    7 as i32
                };
            }
            if (unsafe { (*p).sParse.iLabel }) != (0 as u32) {
                unsafe {
                    (*p).i = unsafe { (*p).sParse.iLabel };
                }
                unsafe {
                    (*p).eType = ((12 as i32) as i8) as u8;
                }
            } else {
                unsafe {
                    (*p).i = i;
                }
                unsafe {
                    (*p).eType = ((11 as i32) as i8) as u8;
                }
            }
        }
        jsonAppendRaw(
            unsafe { std::ptr::addr_of_mut!((*p).path) },
            zRoot,
            unsafe { (*p).nRoot },
        );
    } else {
        unsafe {
            (*p).i = (0 as i32) as u32;
        }
        i = (0 as i32) as u32;
        unsafe {
            (*p).eType = ((0 as i32) as i8) as u8;
        }
        unsafe {
            (*p).nRoot = (1 as i32) as u32;
        }
        jsonAppendRaw(
            unsafe { std::ptr::addr_of_mut!((*p).path) },
            (b"$\0".as_ptr() as *mut i8) as *const i8,
            (1 as i32) as u32,
        );
    }
    unsafe {
        (*p).nParent = (0 as i32) as u32;
    }
    n = jsonbPayloadSize(
        (unsafe { std::ptr::addr_of_mut!((*p).sParse) }) as *const JsonParse,
        i,
        std::ptr::addr_of_mut!(sz),
    );
    unsafe {
        (*p).iEnd = i.wrapping_add(n).wrapping_add(sz);
    }
    if (((unsafe { *unsafe { unsafe { (*p).sParse.aBlob }.offset(i as isize) } }) as u32) as i32)
        & (15 as i32)
        >= (11 as i32)
        && !((unsafe { (*p).bRecursive }) != (0 as u8))
    {
        unsafe {
            (*p).i = i.wrapping_add(n);
        }
        unsafe {
            (*p).eType = (((((unsafe {
                *unsafe { unsafe { (*p).sParse.aBlob }.offset(i as isize) }
            }) as u32) as i32)
                & (15 as i32)) as i8) as u8;
        }
        unsafe {
            (*p).aParent =
                (unsafe { sqlite3DbMallocZero(unsafe { (*p).db }, 24 as u64) }) as *mut JsonParent;
        }
        if (unsafe { (*p).aParent }) == std::ptr::null_mut::<JsonParent>() {
            return 7 as i32;
        }
        unsafe {
            (*p).nParent = (1 as i32) as u32;
        }
        unsafe {
            (*p).nParentAlloc = (1 as i32) as u32;
        }
        unsafe {
            (*unsafe { unsafe { (*p).aParent }.offset((0 as i32) as isize) }).iKey =
                (0 as i32) as i64;
        }
        unsafe {
            (*unsafe { unsafe { (*p).aParent }.offset((0 as i32) as isize) }).iEnd =
                unsafe { (*p).iEnd };
        }
        unsafe {
            (*unsafe { unsafe { (*p).aParent }.offset((0 as i32) as isize) }).iHead =
                unsafe { (*p).i };
        }
        unsafe {
            (*unsafe { unsafe { (*p).aParent }.offset((0 as i32) as isize) }).iValue = i;
        }
    }
    return 0 as i32;
}

/// The methods of the json_each virtual table
/// iVersion
/// xCreate
/// xConnect
/// xBestIndex
/// xDisconnect
/// xDestroy
/// xOpen - open a cursor
/// xClose - close a cursor
/// xFilter - configure scan constraints
/// xNext - advance a cursor
/// xEof - check for end of scan
/// xColumn - read data
/// xRowid - read data
/// xUpdate
/// xBegin
/// xSync
/// xCommit
/// xRollback
/// xFindMethod
/// xRename
/// xSavepoint
/// xRelease
/// xRollbackTo
/// xShadowName
/// xIntegrity
static mut jsonEachModule: sqlite3_module = sqlite3_module {
    iVersion: 0 as i32,
    xCreate: None,
    xConnect: Some(jsonEachConnect),
    xBestIndex: Some(jsonEachBestIndex),
    xDisconnect: Some(jsonEachDisconnect),
    xDestroy: None,
    xOpen: Some(jsonEachOpen),
    xClose: Some(jsonEachClose),
    xFilter: Some(jsonEachFilter),
    xNext: Some(jsonEachNext),
    xEof: Some(jsonEachEof),
    xColumn: Some(jsonEachColumn),
    xRowid: Some(jsonEachRowid),
    xUpdate: None,
    xBegin: None,
    xSync: None,
    xCommit: None,
    xRollback: None,
    xFindFunction: None,
    xRename: None,
    xSavepoint: None,
    xRelease: None,
    xRollbackTo: None,
    xShadowName: None,
    xIntegrity: None,
};

/// Register JSON functions.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RegisterJsonFunctions() {
    //   sqlite3_result_subtype() ----,  ,--- sqlite3_value_subtype()
    //
    //                                |  |
    //
    //             Uses cache ------, |  | ,---- Returns JSONB
    //
    //                              | |  | |
    //
    //     Number of arguments ---, | |  | | ,--- Flags
    //
    //                            | | |  | | |
    unsafe {
        sqlite3InsertBuiltinFuncs(
            unsafe { std::ptr::addr_of_mut!(aJsonFunc.0) as *mut FuncDef },
            (((2592 as u64) / (72 as u64)) as u32) as i32,
        )
    };
}

static mut aJsonFunc: __SlateAlign16<[FuncDef; 36]> = __SlateAlign16([
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (1 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonRemoveFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t0: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t0.pHash = std::ptr::null_mut::<FuncDef>();
            __t0
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: (((0 as i32) | (1 as i32) * (16 as i32)) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonRemoveFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"jsonb\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t1: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t1.pHash = std::ptr::null_mut::<FuncDef>();
            __t1
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (0 as i32) * (32768 as i32)
            | (1 as i32) * (1048576 as i32)
            | (1 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonArrayFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_array\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t2: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t2.pHash = std::ptr::null_mut::<FuncDef>();
            __t2
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (0 as i32) * (32768 as i32)
            | (1 as i32) * (1048576 as i32)
            | (1 as i32) * (16777216 as i32)) as u32,
        pUserData: (((0 as i32) | (1 as i32) * (16 as i32)) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonArrayFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"jsonb_array\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t3: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t3.pHash = std::ptr::null_mut::<FuncDef>();
            __t3
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (1 as i32) * (1048576 as i32)
            | (1 as i32) * (16777216 as i32)) as u32,
        pUserData: (((8 as i32) | (0 as i32) * (16 as i32)) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonSetFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_array_insert\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t4: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t4.pHash = std::ptr::null_mut::<FuncDef>();
            __t4
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (1 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: (((8 as i32) | (1 as i32) * (16 as i32)) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonSetFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"jsonb_array_insert\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t5: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t5.pHash = std::ptr::null_mut::<FuncDef>();
            __t5
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonArrayLengthFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_array_length\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t6: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t6.pHash = std::ptr::null_mut::<FuncDef>();
            __t6
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonArrayLengthFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_array_length\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t7: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t7.pHash = std::ptr::null_mut::<FuncDef>();
            __t7
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonErrorFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_error_position\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t8: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t8.pHash = std::ptr::null_mut::<FuncDef>();
            __t8
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (1 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonExtractFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_extract\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t9: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t9.pHash = std::ptr::null_mut::<FuncDef>();
            __t9
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: (((0 as i32) | (1 as i32) * (16 as i32)) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonExtractFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"jsonb_extract\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t10: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t10.pHash = std::ptr::null_mut::<FuncDef>();
            __t10
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (1 as i32) * (16777216 as i32)) as u32,
        pUserData: (((1 as i32) | (0 as i32) * (16 as i32)) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonExtractFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"->\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t11: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t11.pHash = std::ptr::null_mut::<FuncDef>();
            __t11
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: (((2 as i32) | (0 as i32) * (16 as i32)) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonExtractFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"->>\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t12: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t12.pHash = std::ptr::null_mut::<FuncDef>();
            __t12
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (1 as i32) * (1048576 as i32)
            | (1 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonSetFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_insert\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t13: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t13.pHash = std::ptr::null_mut::<FuncDef>();
            __t13
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (1 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: (((0 as i32) | (1 as i32) * (16 as i32)) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonSetFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"jsonb_insert\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t14: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t14.pHash = std::ptr::null_mut::<FuncDef>();
            __t14
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (0 as i32) * (32768 as i32)
            | (1 as i32) * (1048576 as i32)
            | (1 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonObjectFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_object\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t15: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t15.pHash = std::ptr::null_mut::<FuncDef>();
            __t15
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (0 as i32) * (32768 as i32)
            | (1 as i32) * (1048576 as i32)
            | (1 as i32) * (16777216 as i32)) as u32,
        pUserData: (((0 as i32) | (1 as i32) * (16 as i32)) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonObjectFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"jsonb_object\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t16: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t16.pHash = std::ptr::null_mut::<FuncDef>();
            __t16
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (1 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonPatchFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_patch\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t17: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t17.pHash = std::ptr::null_mut::<FuncDef>();
            __t17
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: (((0 as i32) | (1 as i32) * (16 as i32)) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonPatchFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"jsonb_patch\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t18: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t18.pHash = std::ptr::null_mut::<FuncDef>();
            __t18
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonPrettyFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_pretty\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t19: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t19.pHash = std::ptr::null_mut::<FuncDef>();
            __t19
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonPrettyFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_pretty\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t20: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t20.pHash = std::ptr::null_mut::<FuncDef>();
            __t20
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (0 as i32) * (32768 as i32)
            | (1 as i32) * (1048576 as i32)
            | (1 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonQuoteFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_quote\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t21: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t21.pHash = std::ptr::null_mut::<FuncDef>();
            __t21
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (1 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonRemoveFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_remove\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t22: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t22.pHash = std::ptr::null_mut::<FuncDef>();
            __t22
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: (((0 as i32) | (1 as i32) * (16 as i32)) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonRemoveFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"jsonb_remove\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t23: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t23.pHash = std::ptr::null_mut::<FuncDef>();
            __t23
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (1 as i32) * (1048576 as i32)
            | (1 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonReplaceFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_replace\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t24: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t24.pHash = std::ptr::null_mut::<FuncDef>();
            __t24
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (1 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: (((0 as i32) | (1 as i32) * (16 as i32)) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonReplaceFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"jsonb_replace\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t25: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t25.pHash = std::ptr::null_mut::<FuncDef>();
            __t25
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (1 as i32) * (1048576 as i32)
            | (1 as i32) * (16777216 as i32)) as u32,
        pUserData: (((4 as i32) | (0 as i32) * (16 as i32)) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonSetFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_set\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t26: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t26.pHash = std::ptr::null_mut::<FuncDef>();
            __t26
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (1 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: (((4 as i32) | (1 as i32) * (16 as i32)) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonSetFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"jsonb_set\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t27: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t27.pHash = std::ptr::null_mut::<FuncDef>();
            __t27
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonTypeFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_type\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t28: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t28.pHash = std::ptr::null_mut::<FuncDef>();
            __t28
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonTypeFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_type\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t29: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t29.pHash = std::ptr::null_mut::<FuncDef>();
            __t29
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonValidFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_valid\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t30: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t30.pHash = std::ptr::null_mut::<FuncDef>();
            __t30
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (2048 as i32)
            | (2048 as i32)
            | (1 as i32)
            | (1 as i32) * (32768 as i32)
            | (0 as i32) * (1048576 as i32)
            | (0 as i32) * (16777216 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonValidFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"json_valid\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t31: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t31.pHash = std::ptr::null_mut::<FuncDef>();
            __t31
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (0 as i32) * (32 as i32)
            | (1048576 as i32)
            | (16777216 as i32)
            | (1 as i32)
            | (2048 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonArrayStep),
        xFinalize: Some(jsonArrayFinal),
        xValue: Some(jsonArrayValue),
        xInverse: Some(jsonGroupInverse),
        zName: (b"json_group_array\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t32: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t32.pHash = std::ptr::null_mut::<FuncDef>();
            __t32
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (0 as i32) * (32 as i32)
            | (1048576 as i32)
            | (16777216 as i32)
            | (1 as i32)
            | (2048 as i32)) as u32,
        pUserData: ((16 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonArrayStep),
        xFinalize: Some(jsonArrayFinal),
        xValue: Some(jsonArrayValue),
        xInverse: Some(jsonGroupInverse),
        zName: (b"jsonb_group_array\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t33: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t33.pHash = std::ptr::null_mut::<FuncDef>();
            __t33
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (0 as i32) * (32 as i32)
            | (1048576 as i32)
            | (16777216 as i32)
            | (1 as i32)
            | (2048 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonObjectStep),
        xFinalize: Some(jsonObjectFinal),
        xValue: Some(jsonObjectValue),
        xInverse: Some(jsonGroupInverse),
        zName: (b"json_group_object\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t34: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t34.pHash = std::ptr::null_mut::<FuncDef>();
            __t34
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32)
            | (1 as i32)
            | (0 as i32) * (32 as i32)
            | (1048576 as i32)
            | (16777216 as i32)
            | (1 as i32)
            | (2048 as i32)) as u32,
        pUserData: ((16 as i32) as i64) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(jsonObjectStep),
        xFinalize: Some(jsonObjectFinal),
        xValue: Some(jsonObjectValue),
        xInverse: Some(jsonGroupInverse),
        zName: (b"jsonb_group_object\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t35: __SlateRecord161 = unsafe { std::mem::zeroed() };
            __t35.pHash = std::ptr::null_mut::<FuncDef>();
            __t35
        },
    },
]);

/// Register the JSON table-valued function named zName and return a
/// pointer to its Module object.  Return NULL if something goes wrong.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3JsonVtabRegister(
    mut db: *mut sqlite3,
    mut zName: *const i8,
) -> *mut Module {
    let mut i: u32 = 0 as u32;
    0 as i32;
    i = (0 as i32) as u32;
    '__slate_break_1493: while (i as u64) < (32 as u64) / (8 as u64) {
        if (unsafe {
            sqlite3StrICmp(
                unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of_mut!(azModule.0) as *mut *const i8 }
                            .offset(i as isize)
                    }
                },
                zName,
            )
        }) == (0 as i32)
        {
            return unsafe {
                sqlite3VtabCreateModule(
                    db,
                    unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of_mut!(azModule.0) as *mut *const i8 }
                                .offset(i as isize)
                        }
                    },
                    (unsafe { std::ptr::addr_of_mut!(jsonEachModule) }) as *const sqlite3_module,
                    std::ptr::null_mut::<()>(),
                    None,
                )
            };
        }
        let __v1494: u32 = i;
        let __v1495: u32 = __v1494.wrapping_add((1 as i32) as u32);
        i = __v1495;
    }
    return std::ptr::null_mut::<Module>();
}

static mut azModule: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"json_each\0".as_ptr() as *mut i8) as *const i8,
    (b"json_tree\0".as_ptr() as *mut i8) as *const i8,
    (b"jsonb_each\0".as_ptr() as *mut i8) as *const i8,
    (b"jsonb_tree\0".as_ptr() as *mut i8) as *const i8,
]);
