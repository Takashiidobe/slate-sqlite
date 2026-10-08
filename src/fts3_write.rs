//! 2009 Oct 23
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
//! This file is part of the SQLite FTS3 extension module. Specifically,
//! this file contains code to insert, update and delete rows from FTS3
//! tables. It also contains code to merge FTS3 b-tree segments. Some
//! of the sub-routines used to merge segments are also used by the query
//! code in fts3.c.
unsafe extern "C" {
    fn qsort(
        __base: *mut (),
        __nmemb: u64,
        __size: u64,
        __compar: Option<unsafe extern "C-unwind" fn(*const (), *const ()) -> i32>,
    );
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn sqlite3_exec(
        __v1266: *mut sqlite3,
        sql: *const i8,
        callback: Option<
            unsafe extern "C-unwind" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32,
        >,
        __v1269: *mut (),
        errmsg: *mut *mut i8,
    ) -> i32;
    fn sqlite3_last_insert_rowid(__v1271: *mut sqlite3) -> i64;
    fn sqlite3_mprintf(__v1272: *const i8, ...) -> *mut i8;
    fn sqlite3_malloc64(__v1273: u64) -> *mut ();
    fn sqlite3_realloc64(__v1274: *mut (), __v1275: u64) -> *mut ();
    fn sqlite3_free(__v1276: *mut ());
    fn sqlite3_prepare_v3(
        db: *mut sqlite3,
        zSql: *const i8,
        nByte: i32,
        prepFlags: u32,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const i8,
    ) -> i32;
    fn sqlite3_bind_blob(
        __v1283: *mut sqlite3_stmt,
        __v1284: i32,
        __v1285: *const (),
        n: i32,
        __v1287: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_bind_int(__v1288: *mut sqlite3_stmt, __v1289: i32, __v1290: i32) -> i32;
    fn sqlite3_bind_int64(__v1291: *mut sqlite3_stmt, __v1292: i32, __v1293: i64) -> i32;
    fn sqlite3_bind_null(__v1294: *mut sqlite3_stmt, __v1295: i32) -> i32;
    fn sqlite3_bind_text(
        __v1296: *mut sqlite3_stmt,
        __v1297: i32,
        __v1298: *const i8,
        __v1299: i32,
        __v1300: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_bind_value(
        __v1301: *mut sqlite3_stmt,
        __v1302: i32,
        __v1303: *const sqlite3_value,
    ) -> i32;
    fn sqlite3_bind_parameter_count(__v1304: *mut sqlite3_stmt) -> i32;
    fn sqlite3_step(__v1305: *mut sqlite3_stmt) -> i32;
    fn sqlite3_column_blob(__v1306: *mut sqlite3_stmt, iCol: i32) -> *const ();
    fn sqlite3_column_int(__v1308: *mut sqlite3_stmt, iCol: i32) -> i32;
    fn sqlite3_column_int64(__v1310: *mut sqlite3_stmt, iCol: i32) -> i64;
    fn sqlite3_column_text(__v1312: *mut sqlite3_stmt, iCol: i32) -> *const u8;
    fn sqlite3_column_bytes(__v1314: *mut sqlite3_stmt, iCol: i32) -> i32;
    fn sqlite3_column_type(__v1316: *mut sqlite3_stmt, iCol: i32) -> i32;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_value_int(__v1320: *mut sqlite3_value) -> i32;
    fn sqlite3_value_int64(__v1321: *mut sqlite3_value) -> i64;
    fn sqlite3_value_text(__v1322: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_bytes(__v1323: *mut sqlite3_value) -> i32;
    fn sqlite3_value_type(__v1324: *mut sqlite3_value) -> i32;
    fn sqlite3_blob_open(
        __v1325: *mut sqlite3,
        zDb: *const i8,
        zTable: *const i8,
        zColumn: *const i8,
        iRow: i64,
        flags: i32,
        ppBlob: *mut *mut sqlite3_blob,
    ) -> i32;
    fn sqlite3_blob_reopen(__v1332: *mut sqlite3_blob, __v1333: i64) -> i32;
    fn sqlite3_blob_close(__v1334: *mut sqlite3_blob) -> i32;
    fn sqlite3_blob_bytes(__v1335: *mut sqlite3_blob) -> i32;
    fn sqlite3_blob_read(__v1336: *mut sqlite3_blob, Z: *mut (), N: i32, iOffset: i32) -> i32;
    fn sqlite3_strnicmp(__v1340: *const i8, __v1341: *const i8, __v1342: i32) -> i32;
    fn sqlite3_vtab_on_conflict(__v1343: *mut sqlite3) -> i32;
    fn sqlite3Fts3HashInsert(
        __v1344: *mut Fts3Hash,
        pKey: *const (),
        nKey: i32,
        pData: *mut (),
    ) -> *mut ();
    fn sqlite3Fts3HashFind(__v1348: *const Fts3Hash, pKey: *const (), nKey: i32) -> *mut ();
    fn sqlite3Fts3HashClear(__v1351: *mut Fts3Hash);
    fn sqlite3Fts3HashFindElem(
        __v1352: *const Fts3Hash,
        __v1353: *const (),
        __v1354: i32,
    ) -> *mut Fts3HashElem;
    fn sqlite3Fts3SegReaderCursor(
        __v1410: *mut Fts3Table,
        __v1411: i32,
        __v1412: i32,
        __v1413: i32,
        __v1414: *const i8,
        __v1415: i32,
        __v1416: i32,
        __v1417: i32,
        __v1418: *mut Fts3MultiSegReader,
    ) -> i32;
    fn sqlite3Fts3PutVarint(__v1427: *mut i8, __v1428: i64) -> i32;
    fn sqlite3Fts3GetVarint(__v1429: *const i8, __v1430: *mut i64) -> i32;
    fn sqlite3Fts3GetVarintU(__v1431: *const i8, __v1432: *mut u64) -> i32;
    fn sqlite3Fts3GetVarint32(__v1433: *const i8, __v1434: *mut i32) -> i32;
    fn sqlite3Fts3VarintLen(__v1435: u64) -> i32;
    fn sqlite3Fts3DoclistPrev(
        __v1436: i32,
        __v1437: *mut i8,
        __v1438: i32,
        __v1439: *mut *mut i8,
        __v1440: *mut i64,
        __v1441: *mut i32,
        __v1442: *mut u8,
    );
    fn sqlite3Fts3FirstFilter(
        __v1443: i64,
        __v1444: *mut i8,
        __v1445: i32,
        __v1446: *mut i8,
    ) -> i32;
    fn sqlite3Fts3CreateStatTable(__v1447: *mut i32, __v1448: *mut Fts3Table);
    fn sqlite3Fts3OpenTokenizer(
        __v1449: *mut sqlite3_tokenizer,
        __v1450: i32,
        __v1451: *const i8,
        __v1452: i32,
        __v1453: *mut *mut sqlite3_tokenizer_cursor,
    ) -> i32;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3 {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_stmt {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_value {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_context {}

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
struct sqlite3_blob {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_tokenizer_module {
    iVersion: i32,
    xCreate: Option<
        unsafe extern "C-unwind" fn(i32, *const *const i8, *mut *mut sqlite3_tokenizer) -> i32,
    >,
    xDestroy: Option<unsafe extern "C-unwind" fn(*mut sqlite3_tokenizer) -> i32>,
    xOpen: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_tokenizer,
            *const i8,
            i32,
            *mut *mut sqlite3_tokenizer_cursor,
        ) -> i32,
    >,
    xClose: Option<unsafe extern "C-unwind" fn(*mut sqlite3_tokenizer_cursor) -> i32>,
    xNext: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_tokenizer_cursor,
            *mut *const i8,
            *mut i32,
            *mut i32,
            *mut i32,
            *mut i32,
        ) -> i32,
    >,
    xLanguageid: Option<unsafe extern "C-unwind" fn(*mut sqlite3_tokenizer_cursor, i32) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_tokenizer {
    pModule: *const sqlite3_tokenizer_module,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_tokenizer_cursor {
    pTokenizer: *mut sqlite3_tokenizer,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Hash {
    keyClass: i8,
    copyKey: i8,
    count: i32,
    first: *mut Fts3HashElem,
    htsize: i32,
    ht: *mut _fts3ht,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3HashElem {
    next: *mut Fts3HashElem,
    prev: *mut Fts3HashElem,
    data: *mut (),
    pKey: *mut (),
    nKey: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _fts3ht {
    count: i32,
    chain: *mut Fts3HashElem,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Table {
    base: sqlite3_vtab,
    db: *mut sqlite3,
    zDb: *const i8,
    zName: *const i8,
    nColumn: i32,
    azColumn: *mut *mut i8,
    abNotindexed: *mut u8,
    pTokenizer: *mut sqlite3_tokenizer,
    zContentTbl: *mut i8,
    zLanguageid: *mut i8,
    nAutoincrmerge: i32,
    nLeafAdd: u32,
    bLock: i32,
    aStmt: [*mut sqlite3_stmt; 40],
    pSeekStmt: *mut sqlite3_stmt,
    zReadExprlist: *mut i8,
    zWriteExprlist: *mut i8,
    nNodeSize: i32,
    bFts4: u8,
    bHasStat: u8,
    bHasDocsize: u8,
    bDescIdx: u8,
    bIgnoreSavepoint: u8,
    nPgsz: i32,
    zSegmentsTbl: *mut i8,
    pSegments: *mut sqlite3_blob,
    iSavepoint: i32,
    nIndex: i32,
    aIndex: *mut Fts3Index,
    nMaxPendingData: i32,
    nPendingData: i32,
    iPrevDocid: i64,
    iPrevLangid: i32,
    bPrevDelete: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Cursor {
    base: sqlite3_vtab_cursor,
    eSearch: i16,
    isEof: u8,
    isRequireSeek: u8,
    bSeekStmt: u8,
    pStmt: *mut sqlite3_stmt,
    pExpr: *mut Fts3Expr,
    iLangid: i32,
    nPhrase: i32,
    pDeferred: *mut Fts3DeferredToken,
    iPrevId: i64,
    pNextId: *mut i8,
    aDoclist: *mut i8,
    nDoclist: i32,
    bDesc: u8,
    eEvalmode: i32,
    nRowAvg: i32,
    nDoc: i64,
    iMinDocid: i64,
    iMaxDocid: i64,
    isMatchinfoNeeded: i32,
    pMIBuffer: *mut MatchinfoBuffer,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Doclist {
    aAll: *mut i8,
    nAll: i32,
    pNextDocid: *mut i8,
    iDocid: i64,
    bFreeList: i32,
    pList: *mut i8,
    nList: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3PhraseToken {
    z: *mut i8,
    n: i32,
    isPrefix: i32,
    bFirst: i32,
    pDeferred: *mut Fts3DeferredToken,
    pSegcsr: *mut Fts3MultiSegReader,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Phrase {
    doclist: Fts3Doclist,
    bIncr: i32,
    iDoclistToken: i32,
    pOrPoslist: *mut i8,
    iOrDocid: i64,
    nToken: i32,
    iColumn: i32,
    aToken: [Fts3PhraseToken; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Expr {
    eType: i32,
    nNear: i32,
    pParent: *mut Fts3Expr,
    pLeft: *mut Fts3Expr,
    pRight: *mut Fts3Expr,
    pPhrase: *mut Fts3Phrase,
    iDocid: i64,
    bEof: u8,
    bStart: u8,
    bDeferred: u8,
    iPhrase: i32,
    aMI: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3SegFilter {
    zTerm: *const i8,
    nTerm: i32,
    iCol: i32,
    flags: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3MultiSegReader {
    apSegment: *mut *mut Fts3SegReader,
    nSegment: i32,
    nAdvance: i32,
    pFilter: *mut Fts3SegFilter,
    aBuffer: *mut i8,
    nBuffer: i64,
    iColFilter: i32,
    bRestart: i32,
    nCost: i32,
    bLookup: i32,
    zTerm: *mut i8,
    nTerm: i32,
    aDoclist: *mut i8,
    nDoclist: i32,
}

// When full-text index nodes are loaded from disk, the buffer that they
// are loaded into has the following number of bytes of padding at the end
// of it. i.e. if a full-text index node is 900 bytes in size, then a buffer
// of 920 bytes is allocated for it.
//
// This means that if we have a pointer into a buffer containing node data,
// it is always safe to read up to two varints from it without risking an
// overread, even if the node data is corrupted.
// The values that may be meaningfully bound to the :1 parameter in
// statements SQL_REPLACE_STAT and SQL_SELECT_STAT.
/// An instance of the following data structure is used to build doclists
/// incrementally. See function fts3PendingListAppend() for details.
#[repr(C)]
#[derive(Clone, Copy)]
struct PendingList {
    nData: i64,
    aData: *mut i8,
    nSpace: i64,
    iLastDocid: i64,
    iLastCol: i64,
    iLastPos: i64,
}

/// Each cursor has a (possibly empty) linked list of the following objects.
#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3DeferredToken {
    /// Pointer to corresponding expr token
    pToken: *mut Fts3PhraseToken,
    /// Column token must occur in
    iCol: i32,
    /// Next in list of deferred tokens
    pNext: *mut Fts3DeferredToken,
    /// Doclist is assembled here
    pList: *mut PendingList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct MatchinfoBuffer {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Index {
    nPrefix: i32,
    hPending: Fts3Hash,
}

/// An instance of this structure is used to iterate through the terms on
/// a contiguous set of segment b-tree leaf nodes. Although the details of
/// this structure are only manipulated by code in this file, opaque handles
/// of type Fts3SegReader* are also used by code in fts3.c to iterate through
/// terms when querying the full-text index. See functions:
///
///   sqlite3Fts3SegReaderNew()
///   sqlite3Fts3SegReaderFree()
///   sqlite3Fts3SegReaderIterate()
///
/// Methods used to manipulate Fts3SegReader structures:
///
///   fts3SegReaderNext()
///   fts3SegReaderFirstDocid()
///   fts3SegReaderNextDocid()
#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3SegReader {
    /// Index within level, or 0x7FFFFFFF for PT
    iIdx: i32,
    /// True for a lookup only
    bLookup: u8,
    /// True for a root-only reader
    rootOnly: u8,
    /// Rowid of first leaf block to traverse
    iStartBlock: i64,
    /// Rowid of final leaf block to traverse
    iLeafEndBlock: i64,
    /// Rowid of final block in segment (or 0)
    iEndBlock: i64,
    /// Current leaf block (or 0)
    iCurrentBlock: i64,
    /// Pointer to node data (or NULL)
    aNode: *mut i8,
    /// Size of buffer at aNode (or 0)
    nNode: i32,
    /// If >0, bytes of buffer aNode[] loaded
    nPopulate: i32,
    /// If not NULL, blob handle to read node
    pBlob: *mut sqlite3_blob,
    ppNextElem: *mut *mut Fts3HashElem,
    /// Variables set by fts3SegReaderNext(). These may be read directly
    /// by the caller. They are valid from the time SegmentReaderNew() returns
    /// until SegmentReaderNext() returns something other than SQLITE_OK
    /// (i.e. SQLITE_DONE).
    /// Number of bytes in current term
    nTerm: i32,
    /// Pointer to current term
    zTerm: *mut i8,
    /// Allocated size of zTerm buffer
    nTermAlloc: i32,
    /// Pointer to doclist of current entry
    aDoclist: *mut i8,
    /// Size of doclist in current entry
    nDoclist: i32,
    /// The following variables are used by fts3SegReaderNextDocid() to iterate
    /// through the current doclist (aDoclist/nDoclist).
    pOffsetList: *mut i8,
    /// For descending pending seg-readers only
    nOffsetList: i32,
    iDocid: i64,
}

/// An instance of this structure is used to create a segment b-tree in the
/// database. The internal details of this type are only accessed by the
/// following functions:
///
///   fts3SegWriterAdd()
///   fts3SegWriterFlush()
///   fts3SegWriterFree()
#[repr(C)]
#[derive(Clone, Copy)]
struct SegmentWriter {
    /// Pointer to interior tree structure
    pTree: *mut SegmentNode,
    /// First slot in %_segments written
    iFirst: i64,
    /// Next free slot in %_segments
    iFree: i64,
    /// Pointer to previous term buffer
    zTerm: *mut i8,
    /// Number of bytes in zTerm
    nTerm: i32,
    /// Size of malloc'd buffer at zMalloc
    nMalloc: i32,
    /// Malloc'd space (possibly) used for zTerm
    zMalloc: *mut i8,
    /// Size of allocation at aData
    nSize: i32,
    /// Bytes of data in aData
    nData: i32,
    /// Pointer to block from malloc()
    aData: *mut i8,
    /// Number of bytes of leaf data written
    nLeafData: i64,
}

/// Type SegmentNode is used by the following three functions to create
/// the interior part of the segment b+-tree structures (everything except
/// the leaf nodes). These functions and type are only ever used by code
/// within the fts3SegWriterXXX() family of functions described above.
///
///   fts3NodeAddTerm()
///   fts3NodeWrite()
///   fts3NodeFree()
///
/// When a b+tree is written to the database (either as a result of a merge
/// or the pending-terms table being flushed), leaves are written into the
/// database file as soon as they are completely populated. The interior of
/// the tree is assembled in memory and written out only once all leaves have
/// been populated and stored. This is Ok, as the b+-tree fanout is usually
/// very large, meaning that the interior of the tree consumes relatively
/// little memory.
#[repr(C)]
#[derive(Clone, Copy)]
struct SegmentNode {
    /// Parent node (or NULL for root node)
    pParent: *mut SegmentNode,
    /// Pointer to right-sibling
    pRight: *mut SegmentNode,
    /// Pointer to left-most node of this depth
    pLeftmost: *mut SegmentNode,
    /// Number of terms written to node so far
    nEntry: i32,
    /// Pointer to previous term buffer
    zTerm: *mut i8,
    /// Number of bytes in zTerm
    nTerm: i32,
    /// Size of malloc'd buffer at zMalloc
    nMalloc: i32,
    /// Malloc'd space (possibly) used for zTerm
    zMalloc: *mut i8,
    /// Bytes of valid data so far
    nData: i32,
    /// Node data
    aData: *mut i8,
}

// Valid values for the second argument to fts3SqlStmt().
/// Wrapper around sqlite3_prepare_v3() to ensure that SQLITE_PREPARE_FROM_DDL
/// is always set.
///
/// # Arguments
///
/// * `p` - Prepare for this connection
/// * `zSql` - SQL to prepare
/// * `bPersist` - True to set SQLITE_PREPARE_PERSISTENT
/// * `bAllowVtab` - True to omit SQLITE_PREPARE_NO_VTAB
/// * `pp` - OUT: Prepared statement
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3PrepareStmt(
    mut p: *mut Fts3Table,
    mut zSql: *const i8,
    mut bPersist: i32,
    mut bAllowVtab: i32,
    mut pp: *mut *mut sqlite3_stmt,
) -> i32 {
    let mut f: i32 = (32 as i32)
        | if bAllowVtab == (0 as i32) {
            4 as i32
        } else {
            0 as i32
        }
        | if bPersist != (0 as i32) {
            1 as i32
        } else {
            0 as i32
        };
    return unsafe {
        sqlite3_prepare_v3(
            unsafe { (*p).db },
            zSql,
            -(1 as i32),
            f as u32,
            pp,
            std::ptr::null_mut::<*const i8>(),
        )
    };
}

/// This function is used to obtain an SQLite prepared statement handle
/// for the statement identified by the second argument. If successful,
/// *pp is set to the requested statement handle and SQLITE_OK returned.
/// Otherwise, an SQLite error code is returned and *pp is set to 0.
///
/// If argument apVal is not NULL, then it must point to an array with
/// at least as many entries as the requested statement has bound
/// parameters. The values are bound to the statements parameters before
/// returning.
///
/// # Arguments
///
/// * `p` - Virtual table handle
/// * `eStmt` - One of the SQL_XXX constants above
/// * `pp` - OUT: Statement handle
/// * `apVal` - Values to bind to statement
fn fts3SqlStmt(
    mut p: *mut Fts3Table,
    mut eStmt: i32,
    mut pp: *mut *mut sqlite3_stmt,
    mut apVal: *mut *mut sqlite3_value,
) -> i32 {
    let mut azSql: __SlateAlign16<[*const i8; 40]> = __SlateAlign16([(b"DELETE FROM %Q.'%q_content' WHERE rowid = ?\0".as_ptr() as *mut i8) as *const i8, (b"SELECT NOT EXISTS(SELECT docid FROM %Q.'%q_content' WHERE rowid!=?)\0".as_ptr() as *mut i8) as *const i8, (b"DELETE FROM %Q.'%q_content'\0".as_ptr() as *mut i8) as *const i8, (b"DELETE FROM %Q.'%q_segments'\0".as_ptr() as *mut i8) as *const i8, (b"DELETE FROM %Q.'%q_segdir'\0".as_ptr() as *mut i8) as *const i8, (b"DELETE FROM %Q.'%q_docsize'\0".as_ptr() as *mut i8) as *const i8, (b"DELETE FROM %Q.'%q_stat'\0".as_ptr() as *mut i8) as *const i8, (b"SELECT %s WHERE rowid=?\0".as_ptr() as *mut i8) as *const i8, (b"SELECT (SELECT max(idx) FROM %Q.'%q_segdir' WHERE level = ?) + 1\0".as_ptr() as *mut i8) as *const i8, (b"REPLACE INTO %Q.'%q_segments'(blockid, block) VALUES(?, ?)\0".as_ptr() as *mut i8) as *const i8, (b"SELECT coalesce((SELECT max(blockid) FROM %Q.'%q_segments') + 1, 1)\0".as_ptr() as *mut i8) as *const i8, (b"REPLACE INTO %Q.'%q_segdir' VALUES(?,?,?,?,?,?)\0".as_ptr() as *mut i8) as *const i8, (b"SELECT idx, start_block, leaves_end_block, end_block, root FROM %Q.'%q_segdir' WHERE level = ? ORDER BY idx ASC\0".as_ptr() as *mut i8) as *const i8, (b"SELECT idx, start_block, leaves_end_block, end_block, root FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ?ORDER BY level DESC, idx ASC\0".as_ptr() as *mut i8) as *const i8, (b"SELECT count(*) FROM %Q.'%q_segdir' WHERE level = ?\0".as_ptr() as *mut i8) as *const i8, (b"SELECT max(level) FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ?\0".as_ptr() as *mut i8) as *const i8, (b"DELETE FROM %Q.'%q_segdir' WHERE level = ?\0".as_ptr() as *mut i8) as *const i8, (b"DELETE FROM %Q.'%q_segments' WHERE blockid BETWEEN ? AND ?\0".as_ptr() as *mut i8) as *const i8, (b"INSERT INTO %Q.'%q_content' VALUES(%s)\0".as_ptr() as *mut i8) as *const i8, (b"DELETE FROM %Q.'%q_docsize' WHERE docid = ?\0".as_ptr() as *mut i8) as *const i8, (b"REPLACE INTO %Q.'%q_docsize' VALUES(?,?)\0".as_ptr() as *mut i8) as *const i8, (b"SELECT size FROM %Q.'%q_docsize' WHERE docid=?\0".as_ptr() as *mut i8) as *const i8, (b"SELECT value FROM %Q.'%q_stat' WHERE id=?\0".as_ptr() as *mut i8) as *const i8, (b"REPLACE INTO %Q.'%q_stat' VALUES(?,?)\0".as_ptr() as *mut i8) as *const i8, (b"\0".as_ptr() as *mut i8) as *const i8, (b"\0".as_ptr() as *mut i8) as *const i8, (b"DELETE FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ?\0".as_ptr() as *mut i8) as *const i8, (b"SELECT ? UNION SELECT level / (1024 * ?) FROM %Q.'%q_segdir'\0".as_ptr() as *mut i8) as *const i8, (b"SELECT level, count(*) AS cnt FROM %Q.'%q_segdir'   GROUP BY level HAVING cnt>=?  ORDER BY (level %% 1024) ASC, 2 DESC LIMIT 1\0".as_ptr() as *mut i8) as *const i8, (b"SELECT 2 * total(1 + leaves_end_block - start_block)   FROM (SELECT * FROM %Q.'%q_segdir'         WHERE level = ? ORDER BY idx ASC LIMIT ?  )\0".as_ptr() as *mut i8) as *const i8, (b"DELETE FROM %Q.'%q_segdir' WHERE level = ? AND idx = ?\0".as_ptr() as *mut i8) as *const i8, (b"UPDATE %Q.'%q_segdir' SET idx = ? WHERE level=? AND idx=?\0".as_ptr() as *mut i8) as *const i8, (b"SELECT idx, start_block, leaves_end_block, end_block, root FROM %Q.'%q_segdir' WHERE level = ? AND idx = ?\0".as_ptr() as *mut i8) as *const i8, (b"UPDATE %Q.'%q_segdir' SET start_block = ?, root = ?WHERE level = ? AND idx = ?\0".as_ptr() as *mut i8) as *const i8, (b"SELECT 1 FROM %Q.'%q_segments' WHERE blockid=? AND block IS NULL\0".as_ptr() as *mut i8) as *const i8, (b"SELECT idx FROM %Q.'%q_segdir' WHERE level=? ORDER BY 1 ASC\0".as_ptr() as *mut i8) as *const i8, (b"SELECT max( level %% 1024 ) FROM %Q.'%q_segdir'\0".as_ptr() as *mut i8) as *const i8, (b"SELECT level, idx, end_block FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ? ORDER BY level DESC, idx ASC\0".as_ptr() as *mut i8) as *const i8, (b"UPDATE OR FAIL %Q.'%q_segdir' SET level=-1,idx=? WHERE level=? AND idx=?\0".as_ptr() as *mut i8) as *const i8, (b"UPDATE OR FAIL %Q.'%q_segdir' SET level=? WHERE level=-1\0".as_ptr() as *mut i8) as *const i8]); // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
    // 8
    // 9
    // 10
    // 11
    // Return segments in order from oldest to newest.
    //
    // 12
    // 13
    // 14
    // 15
    // 16
    // 17
    // 18
    // 19
    // 20
    // 21
    // 22
    // 23
    // 24
    // 25
    // 26
    // 27
    // This statement is used to determine which level to read the input from
    // when performing an incremental merge. It returns the absolute level number
    // of the oldest level in the db that contains at least ? segments. Or,
    // if no level in the FTS index contains more than ? segments, the statement
    // returns zero rows.
    //
    // 28
    // Estimate the upper limit on the number of leaf nodes in a new segment
    // created by merging the oldest :2 segments from absolute level :1. See
    // function sqlite3Fts3Incrmerge() for details.
    //
    // 29
    // SQL_DELETE_SEGDIR_ENTRY
    // Delete the %_segdir entry on absolute level :1 with index :2.
    //
    // 30
    // SQL_SHIFT_SEGDIR_ENTRY
    // Modify the idx value for the segment with idx=:3 on absolute level :2
    // to :1.
    //
    // 31
    // SQL_SELECT_SEGDIR
    // Read a single entry from the %_segdir table. The entry from absolute
    // level :1 with index value :2.
    //
    // 32
    // SQL_CHOMP_SEGDIR
    // Update the start_block (:1) and root (:2) fields of the %_segdir
    // entry located on absolute level :3 with index :4.
    //
    // 33
    // SQL_SEGMENT_IS_APPENDABLE
    // Return a single row if the segment with end_block=? is appendable. Or
    // no rows otherwise.
    //
    // 34
    // SQL_SELECT_INDEXES
    // Return the list of valid segment indexes for absolute level ?
    //
    // 35
    // SQL_SELECT_MXLEVEL
    // Return the largest relative level in the FTS index or indexes.
    //
    // 36
    // Return segments in order from oldest to newest.
    //
    // 37
    // Update statements used while promoting segments
    //
    // 38
    // 39
    let mut rc: i32 = 0 as i32;
    let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    pStmt = unsafe {
        *unsafe {
            unsafe { (*p).aStmt.as_mut_ptr() as *mut *mut sqlite3_stmt }.offset(eStmt as isize)
        }
    };
    if !(pStmt != std::ptr::null_mut::<sqlite3_stmt>()) {
        let mut bAllowVtab: i32 = 0 as i32;
        let mut zSql: *mut i8 = unsafe { std::mem::zeroed() };
        if eStmt == (18 as i32) {
            zSql = unsafe {
                sqlite3_mprintf(
                    unsafe {
                        *unsafe { (azSql.0.as_mut_ptr() as *mut *const i8).offset(eStmt as isize) }
                    },
                    unsafe { (*p).zDb },
                    unsafe { (*p).zName },
                    unsafe { (*p).zWriteExprlist },
                )
            };
        } else {
            if eStmt == (7 as i32) {
                bAllowVtab = 1 as i32;
                zSql = unsafe {
                    sqlite3_mprintf(
                        unsafe {
                            *unsafe {
                                (azSql.0.as_mut_ptr() as *mut *const i8).offset(eStmt as isize)
                            }
                        },
                        unsafe { (*p).zReadExprlist },
                    )
                };
            } else {
                zSql = unsafe {
                    sqlite3_mprintf(
                        unsafe {
                            *unsafe {
                                (azSql.0.as_mut_ptr() as *mut *const i8).offset(eStmt as isize)
                            }
                        },
                        unsafe { (*p).zDb },
                        unsafe { (*p).zName },
                    )
                };
            }
        }
        if !(zSql != std::ptr::null_mut::<i8>()) {
            rc = 7 as i32;
        } else {
            rc = sqlite3Fts3PrepareStmt(
                p,
                zSql as *const i8,
                1 as i32,
                bAllowVtab,
                std::ptr::addr_of_mut!(pStmt),
            );
            unsafe { sqlite3_free(zSql as *mut ()) };
            0 as i32;
            unsafe {
                *unsafe {
                    unsafe { (*p).aStmt.as_mut_ptr() as *mut *mut sqlite3_stmt }
                        .offset(eStmt as isize)
                } = pStmt;
            }
        }
    }
    if apVal != std::ptr::null_mut::<*mut sqlite3_value>() {
        let mut i: i32 = 0 as i32;
        let mut nParam: i32 = unsafe { sqlite3_bind_parameter_count(pStmt) };
        i = 0 as i32;
        '__slate_break_1510: loop {
            if !(rc == (0 as i32) && i < nParam) {
                break;
            }
            rc = unsafe {
                sqlite3_bind_value(
                    pStmt,
                    i + (1 as i32),
                    (unsafe { *unsafe { apVal.offset(i as isize) } }) as *const sqlite3_value,
                )
            };
            let __v1682: i32 = i;
            let __v1683: i32 = __v1682 + (1 as i32);
            i = __v1683;
        }
    }
    unsafe {
        *pp = pStmt;
    }
    return rc;
}

/// # Arguments
///
/// * `pTab` - FTS3 table handle
/// * `iDocid` - Docid to bind for SQL_SELECT_DOCSIZE
/// * `ppStmt` - OUT: Statement handle
fn fts3SelectDocsize(
    mut pTab: *mut Fts3Table,
    mut iDocid: i64,
    mut ppStmt: *mut *mut sqlite3_stmt,
) -> i32 {
    let mut pStmt: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // Statement requested from fts3SqlStmt()
    let mut rc: i32 = 0 as i32; // Return code
    rc = fts3SqlStmt(
        pTab,
        21 as i32,
        std::ptr::addr_of_mut!(pStmt),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        unsafe { sqlite3_bind_int64(pStmt, 1 as i32, iDocid) };
        rc = unsafe { sqlite3_step(pStmt) };
        let __v1684: bool;
        if rc != (100 as i32) {
            __v1684 = true as bool;
        } else {
            __v1684 = (unsafe { sqlite3_column_type(pStmt, 0 as i32) }) != (4 as i32);
        }
        if __v1684 {
            rc = unsafe { sqlite3_reset(pStmt) };
            if rc == (0 as i32) {
                rc = (11 as i32) | (1 as i32) << (8 as i32);
            }
            pStmt = std::ptr::null_mut::<sqlite3_stmt>();
        } else {
            rc = 0 as i32;
        }
    }
    unsafe {
        *ppStmt = pStmt;
    }
    return rc;
}

/// # Arguments
///
/// * `pTab` - Fts3 table handle
/// * `ppStmt` - OUT: Statement handle
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3SelectDoctotal(
    mut pTab: *mut Fts3Table,
    mut ppStmt: *mut *mut sqlite3_stmt,
) -> i32 {
    let mut pStmt: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: i32 = 0 as i32;
    rc = fts3SqlStmt(
        pTab,
        22 as i32,
        std::ptr::addr_of_mut!(pStmt),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        unsafe { sqlite3_bind_int(pStmt, 1 as i32, 0 as i32) };
        let __v1631: bool;
        if (unsafe { sqlite3_step(pStmt) }) != (100 as i32) {
            __v1631 = true as bool;
        } else {
            __v1631 = (unsafe { sqlite3_column_type(pStmt, 0 as i32) }) != (4 as i32);
        }
        if __v1631 {
            rc = unsafe { sqlite3_reset(pStmt) };
            if rc == (0 as i32) {
                rc = (11 as i32) | (1 as i32) << (8 as i32);
            }
            pStmt = std::ptr::null_mut::<sqlite3_stmt>();
        }
    }
    unsafe {
        *ppStmt = pStmt;
    }
    return rc;
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

/// # Arguments
///
/// * `pTab` - Fts3 table handle
/// * `iDocid` - Docid to read size data for
/// * `ppStmt` - OUT: Statement handle
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3SelectDocsize(
    mut pTab: *mut Fts3Table,
    mut iDocid: i64,
    mut ppStmt: *mut *mut sqlite3_stmt,
) -> i32 {
    return fts3SelectDocsize(pTab, iDocid, ppStmt);
}

/// Similar to fts3SqlStmt(). Except, after binding the parameters in
/// array apVal[] to the SQL statement identified by eStmt, the statement
/// is executed.
///
/// Returns SQLITE_OK if the statement is successfully executed, or an
/// SQLite error code otherwise.
///
/// # Arguments
///
/// * `pRC` - Result code
/// * `p` - The FTS3 table
/// * `eStmt` - Index of statement to evaluate
/// * `apVal` - Parameters to bind
fn fts3SqlExec(
    mut pRC: *mut i32,
    mut p: *mut Fts3Table,
    mut eStmt: i32,
    mut apVal: *mut *mut sqlite3_value,
) {
    let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    if (unsafe { *pRC }) != (0 as i32) {
        return;
    }
    rc = fts3SqlStmt(p, eStmt, std::ptr::addr_of_mut!(pStmt), apVal);
    if rc == (0 as i32) {
        unsafe { sqlite3_step(pStmt) };
        rc = unsafe { sqlite3_reset(pStmt) };
    }
    unsafe {
        *pRC = rc;
    }
}

/// This function ensures that the caller has obtained an exclusive
/// shared-cache table-lock on the %_segdir table. This is required before
/// writing data to the fts3 table. If this lock is not acquired first, then
/// the caller may end up attempting to take this lock as part of committing
/// a transaction, causing SQLite to return SQLITE_LOCKED or
/// LOCKED_SHAREDCACHEto a COMMIT command.
///
/// It is best to avoid this because if FTS3 returns any error when
/// committing a transaction, the whole transaction will be rolled back.
/// And this is not what users expect when they get SQLITE_LOCKED_SHAREDCACHE.
/// It can still happen if the user locks the underlying tables directly
/// instead of accessing them via FTS.
fn fts3Writelock(mut p: *mut Fts3Table) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*p).nPendingData }) == (0 as i32) {
        let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
        rc = fts3SqlStmt(
            p,
            16 as i32,
            std::ptr::addr_of_mut!(pStmt),
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == (0 as i32) {
            unsafe { sqlite3_bind_null(pStmt, 1 as i32) };
            unsafe { sqlite3_step(pStmt) };
            rc = unsafe { sqlite3_reset(pStmt) };
        }
    }
    return rc;
}

/// FTS maintains a separate indexes for each language-id (a 32-bit integer).
/// Within each language id, a separate index is maintained to store the
/// document terms, and each configured prefix size (configured the FTS
/// "prefix=" option). And each index consists of multiple levels ("relative
/// levels").
///
/// All three of these values (the language id, the specific index and the
/// level within the index) are encoded in 64-bit integer values stored
/// in the %_segdir table on disk. This function is used to convert three
/// separate component values into the single 64-bit integer value that
/// can be used to query the %_segdir table.
///
/// Specifically, each language-id/index combination is allocated 1024
/// 64-bit integer level values ("absolute levels"). The main terms index
/// for language-id 0 is allocate values 0-1023. The first prefix index
/// (if any) for language-id 0 is allocated values 1024-2047. And so on.
/// Language 1 indexes are allocated immediately following language 0.
///
/// So, for a system with nPrefix prefix indexes configured, the block of
/// absolute levels that corresponds to language-id iLangid and index
/// iIndex starts at absolute level ((iLangid * (nPrefix+1) + iIndex) * 1024).
///
/// # Arguments
///
/// * `p` - FTS3 table handle
/// * `iLangid` - Language id
/// * `iIndex` - Index in p->aIndex[]
/// * `iLevel` - Level of segments
fn getAbsoluteLevel(
    mut p: *mut Fts3Table,
    mut iLangid: i32,
    mut iIndex: i32,
    mut iLevel: i32,
) -> i64 {
    let mut iBase: i64 = 0 as i64; // First absolute level for iLangid/iIndex
    0 as i32;
    0 as i32;
    0 as i32;
    iBase = ((iLangid as i64) * ((unsafe { (*p).nIndex }) as i64) + (iIndex as i64))
        * ((1024 as i32) as i64);
    return iBase + (iLevel as i64);
}

/// Set *ppStmt to a statement handle that may be used to iterate through
/// all rows in the %_segdir table, from oldest to newest. If successful,
/// return SQLITE_OK. If an error occurs while preparing the statement,
/// return an SQLite error code.
///
/// There is only ever one instance of this SQL statement compiled for
/// each FTS3 table.
///
/// The statement returns the following columns from the %_segdir table:
///
///   0: idx
///   1: start_block
///   2: leaves_end_block
///   3: end_block
///   4: root
///
/// # Arguments
///
/// * `p` - FTS3 table
/// * `iLangid` - Language being queried
/// * `iIndex` - Index for p->aIndex[]
/// * `iLevel` - Level to select (relative level)
/// * `ppStmt` - OUT: Compiled statement
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3AllSegdirs(
    mut p: *mut Fts3Table,
    mut iLangid: i32,
    mut iIndex: i32,
    mut iLevel: i32,
    mut ppStmt: *mut *mut sqlite3_stmt,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pStmt: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
    0 as i32;
    0 as i32;
    0 as i32;
    if iLevel < (0 as i32) {
        // "SELECT * FROM %_segdir WHERE level BETWEEN ? AND ? ORDER BY ..."
        rc = fts3SqlStmt(
            p,
            13 as i32,
            std::ptr::addr_of_mut!(pStmt),
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == (0 as i32) {
            unsafe {
                sqlite3_bind_int64(
                    pStmt,
                    1 as i32,
                    getAbsoluteLevel(p, iLangid, iIndex, 0 as i32),
                )
            };
            unsafe {
                sqlite3_bind_int64(
                    pStmt,
                    2 as i32,
                    getAbsoluteLevel(p, iLangid, iIndex, (1024 as i32) - (1 as i32)),
                )
            };
        }
    } else {
        // "SELECT * FROM %_segdir WHERE level = ? ORDER BY ..."
        rc = fts3SqlStmt(
            p,
            12 as i32,
            std::ptr::addr_of_mut!(pStmt),
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == (0 as i32) {
            unsafe {
                sqlite3_bind_int64(
                    pStmt,
                    1 as i32,
                    getAbsoluteLevel(p, iLangid, iIndex, iLevel),
                )
            };
        }
    }
    unsafe {
        *ppStmt = pStmt;
    }
    return rc;
}

/// Append a single varint to a PendingList buffer. SQLITE_OK is returned
/// if successful, or an SQLite error code otherwise.
///
/// This function also serves to allocate the PendingList structure itself.
/// For example, to create a new PendingList structure containing two
/// varints:
///
///   PendingList *p = 0;
///   fts3PendingListAppendVarint(&p, 1);
///   fts3PendingListAppendVarint(&p, 2);
///
/// # Arguments
///
/// * `pp` - IN/OUT: Pointer to PendingList struct
/// * `i` - Value to append to data
fn fts3PendingListAppendVarint(mut pp: *mut *mut PendingList, mut i: i64) -> i32 {
    let mut p: *mut PendingList = unsafe { *pp };
    // Allocate or grow the PendingList as required.
    if !(p != std::ptr::null_mut::<PendingList>()) {
        p = (unsafe { sqlite3_malloc64((48 as u64).wrapping_add(((100 as i32) as i64) as u64)) })
            as *mut PendingList;
        if !(p != std::ptr::null_mut::<PendingList>()) {
            return 7 as i32;
        }
        unsafe {
            (*p).nSpace = (100 as i32) as i64;
        }
        unsafe {
            (*p).aData = (unsafe { p.offset((1 as i32) as isize) }) as *mut i8;
        }
        unsafe {
            (*p).nData = (0 as i32) as i64;
        }
    } else {
        if (unsafe { (*p).nData }) + ((10 as i32) as i64) + ((1 as i32) as i64)
            > unsafe { (*p).nSpace }
        {
            let mut nNew: i64 = (unsafe { (*p).nSpace }) * ((2 as i32) as i64);
            p = (unsafe { sqlite3_realloc64(p as *mut (), (48 as u64).wrapping_add(nNew as u64)) })
                as *mut PendingList;
            if !(p != std::ptr::null_mut::<PendingList>()) {
                unsafe { sqlite3_free((unsafe { *pp }) as *mut ()) };
                unsafe {
                    *pp = std::ptr::null_mut::<PendingList>();
                }
                return 7 as i32;
            }
            unsafe {
                (*p).nSpace = (nNew as i32) as i64;
            }
            unsafe {
                (*p).aData = (unsafe { p.offset((1 as i32) as isize) }) as *mut i8;
            }
        }
    }
    // Append the new serialized varint to the end of the list.
    let __v1685: *mut PendingList = p;
    let __v1686: i64 = unsafe { (*__v1685).nData };
    let __v1687: i64 = __v1686
        + ((unsafe {
            sqlite3Fts3PutVarint(
                unsafe { unsafe { (*p).aData }.offset((unsafe { (*p).nData }) as isize) },
                i,
            )
        }) as i64);
    unsafe {
        (*__v1685).nData = __v1687;
    }
    unsafe {
        *unsafe { unsafe { (*p).aData }.offset((unsafe { (*p).nData }) as isize) } =
            (0 as i32) as i8;
    }
    unsafe {
        *pp = p;
    }
    return 0 as i32;
}

/// Add a docid/column/position entry to a PendingList structure. Non-zero
/// is returned if the structure is sqlite3_realloced as part of adding
/// the entry. Otherwise, zero.
///
/// If an OOM error occurs, *pRc is set to SQLITE_NOMEM before returning.
/// Zero is always returned in this case. Otherwise, if no OOM error occurs,
/// it is set to SQLITE_OK.
///
/// # Arguments
///
/// * `pp` - IN/OUT: PendingList structure
/// * `iDocid` - Docid for entry to add
/// * `iCol` - Column for entry to add
/// * `iPos` - Position of term for entry to add
/// * `pRc` - OUT: Return code
fn fts3PendingListAppend(
    mut pp: *mut *mut PendingList,
    mut iDocid: i64,
    mut iCol: i64,
    mut iPos: i64,
    mut pRc: *mut i32,
) -> i32 {
    let mut __slate_storage_1694: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1694: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1694) as *mut i32;
    let mut __slate_storage_1693: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1693: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1693) as *mut bool;
    let mut __slate_storage_1692: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1692: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1692) as *mut i32;
    let mut __slate_storage_1691: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1691: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1691) as *mut i32;
    let mut __slate_storage_1690: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1690: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1690) as *mut i64;
    let mut __slate_storage_1689: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1689: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1689) as *mut i64;
    let mut __slate_storage_1688: std::mem::MaybeUninit<*mut PendingList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1688: *mut *mut PendingList =
        std::ptr::addr_of_mut!(__slate_storage_1688) as *mut *mut PendingList;
    let mut __slate_storage_330: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_330: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_330) as *mut u64;
    let mut __slate_storage_329: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_329: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_329) as *mut i32;
    let mut __slate_storage_328: std::mem::MaybeUninit<*mut PendingList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_328: *mut *mut PendingList =
        std::ptr::addr_of_mut!(__slate_storage_328) as *mut *mut PendingList;
    unsafe {
        '__join_2: {
            std::ptr::write(__slate_slot_328, unsafe { *pp });
            std::ptr::write(__slate_slot_329, 0 as i32);
            0 as i32;
            if !(*__slate_slot_328 != std::ptr::null_mut::<PendingList>())
                || (unsafe { (*(*__slate_slot_328)).iLastDocid }) != iDocid
            {
                std::ptr::write(
                    __slate_slot_330,
                    (iDocid as u64).wrapping_sub(
                        (if *__slate_slot_328 != std::ptr::null_mut::<PendingList>() {
                            unsafe { (*(*__slate_slot_328)).iLastDocid }
                        } else {
                            (0 as i32) as i64
                        }) as u64,
                    ),
                );
                if *__slate_slot_328 != std::ptr::null_mut::<PendingList>() {
                    0 as i32;
                    0 as i32;
                    std::ptr::write(__slate_slot_1688, *__slate_slot_328);
                    std::ptr::write(__slate_slot_1689, unsafe { (*(*__slate_slot_1688)).nData });
                    std::ptr::write(__slate_slot_1690, *__slate_slot_1689 + ((1 as i32) as i64));
                    unsafe {
                        (*(*__slate_slot_1688)).nData = *__slate_slot_1690;
                    }
                }
                std::ptr::write(
                    __slate_slot_1691,
                    fts3PendingListAppendVarint(
                        std::ptr::addr_of_mut!(*__slate_slot_328),
                        *__slate_slot_330 as i64,
                    ),
                );
                *__slate_slot_329 = *__slate_slot_1691;
                if (0 as i32) != *__slate_slot_1691 {
                    break '__join_2;
                } else {
                    unsafe {
                        (*(*__slate_slot_328)).iLastCol = -(1 as i32) as i64;
                    }
                    unsafe {
                        (*(*__slate_slot_328)).iLastPos = (0 as i32) as i64;
                    }
                    unsafe {
                        (*(*__slate_slot_328)).iLastDocid = iDocid;
                    }
                }
            }
            if iCol > ((0 as i32) as i64) && (unsafe { (*(*__slate_slot_328)).iLastCol }) != iCol {
                std::ptr::write(
                    __slate_slot_1692,
                    fts3PendingListAppendVarint(
                        std::ptr::addr_of_mut!(*__slate_slot_328),
                        (1 as i32) as i64,
                    ),
                );
                *__slate_slot_329 = *__slate_slot_1692;
                if (0 as i32) != *__slate_slot_1692 {
                    *__slate_slot_1693 = true as bool;
                } else {
                    std::ptr::write(
                        __slate_slot_1694,
                        fts3PendingListAppendVarint(
                            std::ptr::addr_of_mut!(*__slate_slot_328),
                            iCol,
                        ),
                    );
                    *__slate_slot_329 = *__slate_slot_1694;
                    *__slate_slot_1693 = (0 as i32) != *__slate_slot_1694;
                }
                if *__slate_slot_1693 {
                    break '__join_2;
                } else {
                    unsafe {
                        (*(*__slate_slot_328)).iLastCol = iCol;
                    }
                    unsafe {
                        (*(*__slate_slot_328)).iLastPos = (0 as i32) as i64;
                    }
                }
            }
            if iCol >= ((0 as i32) as i64) {
                0 as i32;
                *__slate_slot_329 = fts3PendingListAppendVarint(
                    std::ptr::addr_of_mut!(*__slate_slot_328),
                    ((2 as i32) as i64) + iPos - unsafe { (*(*__slate_slot_328)).iLastPos },
                );
                if *__slate_slot_329 == (0 as i32) {
                    unsafe {
                        (*(*__slate_slot_328)).iLastPos = iPos;
                    }
                }
            }
        }
        unsafe {
            *pRc = *__slate_slot_329;
        }
        if *__slate_slot_328 != unsafe { *pp } {
            unsafe {
                *pp = *__slate_slot_328;
            }
            return 1 as i32;
        } else {
            return 0 as i32;
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Free a PendingList object allocated by fts3PendingListAppend().
fn fts3PendingListDelete(mut pList: *mut PendingList) {
    unsafe { sqlite3_free(pList as *mut ()) };
}

/// Add an entry to one of the pending-terms hash tables.
///
/// # Arguments
///
/// * `pHash` - Pending terms hash table to add entry to
fn fts3PendingTermsAddOne(
    mut p: *mut Fts3Table,
    mut iCol: i32,
    mut iPos: i32,
    mut pHash: *mut Fts3Hash,
    mut zToken: *const i8,
    mut nToken: i32,
) -> i32 {
    let mut pList: *mut PendingList = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    pList = (unsafe { sqlite3Fts3HashFind(pHash as *const Fts3Hash, zToken as *const (), nToken) })
        as *mut PendingList;
    if pList != std::ptr::null_mut::<PendingList>() {
        0 as i32;
        let __v1695: *mut Fts3Table = p;
        let __v1696: i32 = unsafe { (*__v1695).nPendingData };
        let __v1697: i32 = __v1696
            - (((((unsafe { (*pList).nData }) + (nToken as i64)) as u64).wrapping_add(40 as u64)
                as u32) as i32);
        unsafe {
            (*__v1695).nPendingData = __v1697;
        }
    }
    if fts3PendingListAppend(
        std::ptr::addr_of_mut!(pList),
        unsafe { (*p).iPrevDocid },
        iCol as i64,
        iPos as i64,
        std::ptr::addr_of_mut!(rc),
    ) != (0 as i32)
    {
        if pList
            == ((unsafe {
                sqlite3Fts3HashInsert(pHash, zToken as *const (), nToken, pList as *mut ())
            }) as *mut PendingList)
        {
            // Malloc failed while inserting the new entry. This can only
            // happen if there was no previous entry for this token.
            0 as i32;
            unsafe { sqlite3_free(pList as *mut ()) };
            rc = 7 as i32;
        }
    }
    if rc == (0 as i32) {
        0 as i32;
        let __v1698: *mut Fts3Table = p;
        let __v1699: i32 = unsafe { (*__v1698).nPendingData };
        let __v1700: i32 = __v1699
            + (((((unsafe { (*pList).nData }) + (nToken as i64)) as u64).wrapping_add(40 as u64)
                as u32) as i32);
        unsafe {
            (*__v1698).nPendingData = __v1700;
        }
    }
    return rc;
}

/// Tokenize the nul-terminated string zText and add all tokens to the
/// pending-terms hash-table. The docid used is that currently stored in
/// p->iPrevDocid, and the column is specified by argument iCol.
///
/// If successful, SQLITE_OK is returned. Otherwise, an SQLite error code.
///
/// # Arguments
///
/// * `p` - Table into which text will be inserted
/// * `iLangid` - Language id to use
/// * `zText` - Text of document to be inserted
/// * `iCol` - Column into which text is being inserted
/// * `pnWord` - IN/OUT: Incr. by number tokens inserted
fn fts3PendingTermsAdd(
    mut p: *mut Fts3Table,
    mut iLangid: i32,
    mut zText: *const i8,
    mut iCol: i32,
    mut pnWord: *mut u32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut iStart: i32 = 0 as i32;
    let mut iEnd: i32 = 0 as i32;
    let mut iPos: i32 = 0 as i32;
    let mut nWord: i32 = 0 as i32;
    let mut zToken: *const i8 = unsafe { std::mem::zeroed() };
    let mut nToken: i32 = 0 as i32;
    let mut pTokenizer: *mut sqlite3_tokenizer = unsafe { (*p).pTokenizer };
    let mut pModule: *const sqlite3_tokenizer_module = unsafe { (*pTokenizer).pModule };
    let mut pCsr: *mut sqlite3_tokenizer_cursor = unsafe { std::mem::zeroed() };
    let mut xNext: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_tokenizer_cursor,
            *mut *const i8,
            *mut i32,
            *mut i32,
            *mut i32,
            *mut i32,
        ) -> i32,
    > = unsafe { std::mem::zeroed() };
    0 as i32;
    // If the user has inserted a NULL value, this function may be called with
    // zText==0. In this case, add zero token entries to the hash table and
    // return early.
    if zText == std::ptr::null::<i8>() {
        unsafe {
            *pnWord = (0 as i32) as u32;
        }
        return 0 as i32;
    }
    rc = unsafe {
        sqlite3Fts3OpenTokenizer(
            pTokenizer,
            iLangid,
            zText,
            -(1 as i32),
            std::ptr::addr_of_mut!(pCsr),
        )
    };
    if rc != (0 as i32) {
        return rc;
    }
    xNext = unsafe { (*pModule).xNext };
    '__slate_break_1511: loop {
        let __v1701: bool;
        if (0 as i32) == rc {
            let __v1702: i32 = unsafe {
                xNext.unwrap()(
                    pCsr,
                    std::ptr::addr_of_mut!(zToken),
                    std::ptr::addr_of_mut!(nToken),
                    std::ptr::addr_of_mut!(iStart),
                    std::ptr::addr_of_mut!(iEnd),
                    std::ptr::addr_of_mut!(iPos),
                )
            };
            rc = __v1702;
            __v1701 = (0 as i32) == __v1702;
        } else {
            __v1701 = false as bool;
        }
        if !__v1701 {
            break;
        }
        let mut i: i32 = 0 as i32;
        if iPos >= nWord {
            nWord = iPos + (1 as i32);
        }
        // Positions cannot be negative; we use -1 as a terminator internally.
        // Tokens must have a non-zero length.
        if iPos < (0 as i32) || !(zToken != std::ptr::null::<i8>()) || nToken <= (0 as i32) {
            rc = 1 as i32;
            break '__slate_break_1511;
        }
        // Add the term to the terms index
        rc = fts3PendingTermsAddOne(
            p,
            iCol,
            iPos,
            unsafe {
                std::ptr::addr_of_mut!(
                    (*unsafe { unsafe { (*p).aIndex }.offset((0 as i32) as isize) }).hPending
                )
            },
            zToken,
            nToken,
        );
        // Add the term to each of the prefix indexes that it is not too
        // short for.
        i = 1 as i32;
        '__slate_break_1512: loop {
            if !(rc == (0 as i32) && i < unsafe { (*p).nIndex }) {
                break;
            }
            let mut pIndex: *mut Fts3Index = unsafe { unsafe { (*p).aIndex }.offset(i as isize) };
            if nToken < unsafe { (*pIndex).nPrefix } {
            } else {
                rc = fts3PendingTermsAddOne(
                    p,
                    iCol,
                    iPos,
                    unsafe { std::ptr::addr_of_mut!((*pIndex).hPending) },
                    zToken,
                    unsafe { (*pIndex).nPrefix },
                );
            }
            let __v1703: i32 = i;
            let __v1704: i32 = __v1703 + (1 as i32);
            i = __v1704;
        }
    }
    unsafe { unsafe { (*pModule).xClose }.unwrap()(pCsr) };
    let __v1705: *mut u32 = pnWord;
    let __v1706: u32 = unsafe { *__v1705 };
    let __v1707: u32 = __v1706.wrapping_add(nWord as u32);
    unsafe {
        *__v1705 = __v1707;
    }
    return if rc == (101 as i32) { 0 as i32 } else { rc };
}

/// Calling this function indicates that subsequent calls to
/// fts3PendingTermsAdd() are to add term/position-list pairs for the
/// contents of the document with docid iDocid.
///
/// # Arguments
///
/// * `p` - Full-text table handle
/// * `bDelete` - True if this op is a delete
/// * `iLangid` - Language id of row being written
/// * `iDocid` - Docid of row being written
fn fts3PendingTermsDocid(
    mut p: *mut Fts3Table,
    mut bDelete: i32,
    mut iLangid: i32,
    mut iDocid: i64,
) -> i32 {
    0 as i32;
    0 as i32;
    // TODO(shess) Explore whether partially flushing the buffer on
    // forced-flush would provide better performance.  I suspect that if
    // we ordered the doclists by size and flushed the largest until the
    // buffer was half empty, that would let the less frequent terms
    // generate longer doclists.
    if iDocid < unsafe { (*p).iPrevDocid }
        || iDocid == unsafe { (*p).iPrevDocid } && (unsafe { (*p).bPrevDelete }) == (0 as i32)
        || (unsafe { (*p).iPrevLangid }) != iLangid
        || (unsafe { (*p).nPendingData }) > unsafe { (*p).nMaxPendingData }
    {
        let mut rc: i32 = sqlite3Fts3PendingTermsFlush(p);
        if rc != (0 as i32) {
            return rc;
        }
    }
    unsafe {
        (*p).iPrevDocid = iDocid;
    }
    unsafe {
        (*p).iPrevLangid = iLangid;
    }
    unsafe {
        (*p).bPrevDelete = bDelete;
    }
    return 0 as i32;
}

/// Discard the contents of the pending-terms hash tables.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3PendingTermsClear(mut p: *mut Fts3Table) {
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_1513: loop {
        if !(i < unsafe { (*p).nIndex }) {
            break;
        }
        let mut pElem: *mut Fts3HashElem = unsafe { std::mem::zeroed() };
        let mut pHash: *mut Fts3Hash = unsafe {
            std::ptr::addr_of_mut!((*unsafe { unsafe { (*p).aIndex }.offset(i as isize) }).hPending)
        };
        pElem = unsafe { (*pHash).first };
        '__slate_break_1514: while pElem != std::ptr::null_mut::<Fts3HashElem>() {
            let mut pList: *mut PendingList = (unsafe { (*pElem).data }) as *mut PendingList;
            fts3PendingListDelete(pList);
            pElem = unsafe { (*pElem).next };
        }
        unsafe { sqlite3Fts3HashClear(pHash) };
        let __v1625: i32 = i;
        let __v1626: i32 = __v1625 + (1 as i32);
        i = __v1626;
    }
    unsafe {
        (*p).nPendingData = 0 as i32;
    }
}

/// This function is called by the xUpdate() method as part of an INSERT
/// operation. It adds entries for each term in the new record to the
/// pendingTerms hash table.
///
/// Argument apVal is the same as the similarly named argument passed to
/// fts3InsertData(). Parameter iDocid is the docid of the new row.
fn fts3InsertTerms(
    mut p: *mut Fts3Table,
    mut iLangid: i32,
    mut apVal: *mut *mut sqlite3_value,
    mut aSz: *mut u32,
) -> i32 {
    let mut i: i32 = 0 as i32; // Iterator variable
    i = 2 as i32;
    '__slate_break_1515: loop {
        if !(i < (unsafe { (*p).nColumn }) + (2 as i32)) {
            break;
        }
        let mut iCol: i32 = i - (2 as i32);
        if (((unsafe { *unsafe { unsafe { (*p).abNotindexed }.offset(iCol as isize) } }) as u32)
            as i32)
            == (0 as i32)
        {
            let mut zText: *const i8 =
                (unsafe { sqlite3_value_text(unsafe { *unsafe { apVal.offset(i as isize) } }) })
                    as *const i8;
            let mut rc: i32 = fts3PendingTermsAdd(p, iLangid, zText, iCol, unsafe {
                aSz.offset(iCol as isize)
            });
            if rc != (0 as i32) {
                return rc;
            }
            let __v1710: *mut u32 = unsafe { aSz.offset((unsafe { (*p).nColumn }) as isize) };
            let __v1711: u32 = unsafe { *__v1710 };
            let __v1712: u32 = __v1711.wrapping_add(
                (unsafe { sqlite3_value_bytes(unsafe { *unsafe { apVal.offset(i as isize) } }) })
                    as u32,
            );
            unsafe {
                *__v1710 = __v1712;
            }
        }
        let __v1708: i32 = i;
        let __v1709: i32 = __v1708 + (1 as i32);
        i = __v1709;
    }
    return 0 as i32;
}

/// This function is called by the xUpdate() method for an INSERT operation.
/// The apVal parameter is passed a copy of the apVal argument passed by
/// SQLite to the xUpdate() method. i.e:
///
///   apVal[0]                Not used for INSERT.
///   apVal[1]                rowid
///   apVal[2]                Left-most user-defined column
///   ...
///   apVal[p->nColumn+1]     Right-most user-defined column
///   apVal[p->nColumn+2]     Hidden column with same name as table
///   apVal[p->nColumn+3]     Hidden "docid" column (alias for rowid)
///   apVal[p->nColumn+4]     Hidden languageid column
///
/// # Arguments
///
/// * `p` - Full-text table
/// * `apVal` - Array of values to insert
/// * `piDocid` - OUT: Docid for row just inserted
fn fts3InsertData(
    mut p: *mut Fts3Table,
    mut apVal: *mut *mut sqlite3_value,
    mut piDocid: *mut i64,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut pContentInsert: *mut sqlite3_stmt = unsafe { std::mem::zeroed() }; // INSERT INTO %_content VALUES(...)
    if (unsafe { (*p).zContentTbl }) != std::ptr::null_mut::<i8>() {
        let mut pRowid: *mut sqlite3_value =
            unsafe { *unsafe { apVal.offset(((unsafe { (*p).nColumn }) + (3 as i32)) as isize) } };
        if (unsafe { sqlite3_value_type(pRowid) }) == (5 as i32) {
            pRowid = unsafe { *unsafe { apVal.offset((1 as i32) as isize) } };
        }
        if (unsafe { sqlite3_value_type(pRowid) }) != (1 as i32) {
            return 19 as i32;
        }
        unsafe {
            *piDocid = unsafe { sqlite3_value_int64(pRowid) };
        }
        return 0 as i32;
    }
    // Locate the statement handle used to insert data into the %_content
    // table. The SQL for this statement is:
    //
    //   INSERT INTO %_content VALUES(?, ?, ?, ...)
    //
    // The statement features N '?' variables, where N is the number of user
    // defined columns in the FTS3 table, plus one for the docid field.
    rc = fts3SqlStmt(
        p,
        18 as i32,
        std::ptr::addr_of_mut!(pContentInsert),
        unsafe { apVal.offset((1 as i32) as isize) },
    );
    if rc == (0 as i32) && (unsafe { (*p).zLanguageid }) != std::ptr::null_mut::<i8>() {
        rc = unsafe {
            sqlite3_bind_int(
                pContentInsert,
                (unsafe { (*p).nColumn }) + (2 as i32),
                unsafe {
                    sqlite3_value_int(unsafe {
                        *unsafe { apVal.offset(((unsafe { (*p).nColumn }) + (4 as i32)) as isize) }
                    })
                },
            )
        };
    }
    if rc != (0 as i32) {
        return rc;
    }
    // There is a quirk here. The users INSERT statement may have specified
    // a value for the "rowid" field, for the "docid" field, or for both.
    // Which is a problem, since "rowid" and "docid" are aliases for the
    // same value. For example:
    //
    //   INSERT INTO fts3tbl(rowid, docid) VALUES(1, 2);
    //
    // In FTS3, this is an error. It is an error to specify non-NULL values
    // for both docid and some other rowid alias.
    if (5 as i32)
        != unsafe {
            sqlite3_value_type(unsafe {
                *unsafe { apVal.offset(((3 as i32) + unsafe { (*p).nColumn }) as isize) }
            })
        }
    {
        let __v1713: bool;
        if (5 as i32)
            == unsafe {
                sqlite3_value_type(unsafe { *unsafe { apVal.offset((0 as i32) as isize) } })
            }
        {
            __v1713 = (5 as i32)
                != unsafe {
                    sqlite3_value_type(unsafe { *unsafe { apVal.offset((1 as i32) as isize) } })
                };
        } else {
            __v1713 = false as bool;
        }
        if __v1713 {
            // A rowid/docid conflict.
            return 1 as i32;
        }
        rc = unsafe {
            sqlite3_bind_value(
                pContentInsert,
                1 as i32,
                (unsafe {
                    *unsafe { apVal.offset(((3 as i32) + unsafe { (*p).nColumn }) as isize) }
                }) as *const sqlite3_value,
            )
        };
        if rc != (0 as i32) {
            return rc;
        }
    }
    // Execute the statement to insert the record. Set *piDocid to the
    // new docid value.
    unsafe { sqlite3_step(pContentInsert) };
    rc = unsafe { sqlite3_reset(pContentInsert) };
    unsafe {
        *piDocid = unsafe { sqlite3_last_insert_rowid(unsafe { (*p).db }) };
    }
    return rc;
}

/// Remove all data from the FTS3 table. Clear the hash table containing
/// pending terms.
fn fts3DeleteAll(mut p: *mut Fts3Table, mut bContent: i32) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    // Discard the contents of the pending-terms hash table.
    sqlite3Fts3PendingTermsClear(p);
    // Delete everything from the shadow tables. Except, leave %_content as
    // is if bContent is false.
    0 as i32;
    if bContent != (0 as i32) {
        fts3SqlExec(
            std::ptr::addr_of_mut!(rc),
            p,
            2 as i32,
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
    }
    fts3SqlExec(
        std::ptr::addr_of_mut!(rc),
        p,
        3 as i32,
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    fts3SqlExec(
        std::ptr::addr_of_mut!(rc),
        p,
        4 as i32,
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if (unsafe { (*p).bHasDocsize }) != (0 as u8) {
        fts3SqlExec(
            std::ptr::addr_of_mut!(rc),
            p,
            5 as i32,
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
    }
    if (unsafe { (*p).bHasStat }) != (0 as u8) {
        fts3SqlExec(
            std::ptr::addr_of_mut!(rc),
            p,
            6 as i32,
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
    }
    return rc;
}

fn langidFromSelect(mut p: *mut Fts3Table, mut pSelect: *mut sqlite3_stmt) -> i32 {
    let mut iLangid: i32 = 0 as i32;
    if (unsafe { (*p).zLanguageid }) != std::ptr::null_mut::<i8>() {
        iLangid = unsafe { sqlite3_column_int(pSelect, (unsafe { (*p).nColumn }) + (1 as i32)) };
    }
    return iLangid;
}

/// The first element in the apVal[] array is assumed to contain the docid
/// (an integer) of a row about to be deleted. Remove all terms from the
/// full-text index.
///
/// # Arguments
///
/// * `pRC` - Result code
/// * `p` - The FTS table to delete from
/// * `pRowid` - The docid to be deleted
/// * `aSz` - Sizes of deleted document written here
/// * `pbFound` - OUT: Set to true if row really does exist
fn fts3DeleteTerms(
    mut pRC: *mut i32,
    mut p: *mut Fts3Table,
    mut pRowid: *mut sqlite3_value,
    mut aSz: *mut u32,
    mut pbFound: *mut i32,
) {
    let mut rc: i32 = 0 as i32;
    let mut pSelect: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    0 as i32;
    if (unsafe { *pRC }) != (0 as i32) {
        return;
    }
    rc = fts3SqlStmt(
        p,
        7 as i32,
        std::ptr::addr_of_mut!(pSelect),
        std::ptr::addr_of_mut!(pRowid),
    );
    if rc == (0 as i32) {
        if (100 as i32) == unsafe { sqlite3_step(pSelect) } {
            let mut i: i32 = 0 as i32;
            let mut iLangid: i32 = langidFromSelect(p, pSelect);
            let mut iDocid: i64 = unsafe { sqlite3_column_int64(pSelect, 0 as i32) };
            rc = fts3PendingTermsDocid(p, 1 as i32, iLangid, iDocid);
            i = 1 as i32;
            '__slate_break_1516: loop {
                if !(rc == (0 as i32) && i <= unsafe { (*p).nColumn }) {
                    break;
                }
                let mut iCol: i32 = i - (1 as i32);
                if (((unsafe { *unsafe { unsafe { (*p).abNotindexed }.offset(iCol as isize) } })
                    as u32) as i32)
                    == (0 as i32)
                {
                    let mut zText: *const i8 =
                        (unsafe { sqlite3_column_text(pSelect, i) }) as *const i8;
                    rc = fts3PendingTermsAdd(p, iLangid, zText, -(1 as i32), unsafe {
                        aSz.offset(iCol as isize)
                    });
                    let __v1716: *mut u32 =
                        unsafe { aSz.offset((unsafe { (*p).nColumn }) as isize) };
                    let __v1717: u32 = unsafe { *__v1716 };
                    let __v1718: u32 =
                        __v1717.wrapping_add((unsafe { sqlite3_column_bytes(pSelect, i) }) as u32);
                    unsafe {
                        *__v1716 = __v1718;
                    }
                }
                let __v1714: i32 = i;
                let __v1715: i32 = __v1714 + (1 as i32);
                i = __v1715;
            }
            if rc != (0 as i32) {
                unsafe { sqlite3_reset(pSelect) };
                unsafe {
                    *pRC = rc;
                }
                return;
            }
            unsafe {
                *pbFound = 1 as i32;
            }
        }
        rc = unsafe { sqlite3_reset(pSelect) };
    } else {
        unsafe { sqlite3_reset(pSelect) };
    }
    unsafe {
        *pRC = rc;
    }
}

/// This function allocates a new level iLevel index in the segdir table.
/// Usually, indexes are allocated within a level sequentially starting
/// with 0, so the allocated index is one greater than the value returned
/// by:
///
///   SELECT max(idx) FROM %_segdir WHERE level = :iLevel
///
/// However, if there are already FTS3_MERGE_COUNT indexes at the requested
/// level, they are merged into a single level (iLevel+1) segment and the
/// allocated index is 0.
///
/// If successful, *piIdx is set to the allocated index slot and SQLITE_OK
/// returned. Otherwise, an SQLite error code is returned.
///
/// # Arguments
///
/// * `iLangid` - Language id
/// * `iIndex` - Index for p->aIndex
fn fts3AllocateSegdirIdx(
    mut p: *mut Fts3Table,
    mut iLangid: i32,
    mut iIndex: i32,
    mut iLevel: i32,
    mut piIdx: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return Code
    let mut pNextIdx: *mut sqlite3_stmt = unsafe { std::mem::zeroed() }; // Query for next idx at level iLevel
    let mut iNext: i32 = 0 as i32; // Result of query pNextIdx
    0 as i32;
    0 as i32;
    // Set variable iNext to the next available segdir index at level iLevel.
    rc = fts3SqlStmt(
        p,
        8 as i32,
        std::ptr::addr_of_mut!(pNextIdx),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        unsafe {
            sqlite3_bind_int64(
                pNextIdx,
                1 as i32,
                getAbsoluteLevel(p, iLangid, iIndex, iLevel),
            )
        };
        if (100 as i32) == unsafe { sqlite3_step(pNextIdx) } {
            iNext = unsafe { sqlite3_column_int(pNextIdx, 0 as i32) };
        }
        rc = unsafe { sqlite3_reset(pNextIdx) };
    }
    if rc == (0 as i32) {
        // If iNext is FTS3_MERGE_COUNT, indicating that level iLevel is already
        // full, merge all segments in level iLevel into a single iLevel+1
        // segment and allocate (newly freed) index 0 at level iLevel. Otherwise,
        // if iNext is less than FTS3_MERGE_COUNT, allocate index iNext.
        if iNext >= (16 as i32) {
            {}
            rc = fts3SegmentMerge(p, iLangid, iIndex, iLevel);
            unsafe {
                *piIdx = 0 as i32;
            }
        } else {
            unsafe {
                *piIdx = iNext;
            }
        }
    }
    return rc;
}

/// The %_segments table is declared as follows:
///
///   CREATE TABLE %_segments(blockid INTEGER PRIMARY KEY, block BLOB)
///
/// This function reads data from a single row of the %_segments table. The
/// specific row is identified by the iBlockid parameter. If paBlob is not
/// NULL, then a buffer is allocated using sqlite3_malloc() and populated
/// with the contents of the blob stored in the "block" column of the
/// identified table row is. Whether or not paBlob is NULL, *pnBlob is set
/// to the size of the blob in bytes before returning.
///
/// If an error occurs, or the table does not contain the specified row,
/// an SQLite error code is returned. Otherwise, SQLITE_OK is returned. If
/// paBlob is non-NULL, then it is the responsibility of the caller to
/// eventually free the returned buffer.
///
/// This function may leave an open sqlite3_blob* handle in the
/// Fts3Table.pSegments variable. This handle is reused by subsequent calls
/// to this function. The handle may be closed by calling the
/// sqlite3Fts3SegmentsClose() function. Reusing a blob handle is a handy
/// performance improvement, but the blob handle should always be closed
/// before control is returned to the user (to prevent a lock being held
/// on the database file for longer than necessary). Thus, any virtual table
/// method (xFilter etc.) that may directly or indirectly call this function
/// must call sqlite3Fts3SegmentsClose() before returning.
///
/// # Arguments
///
/// * `p` - FTS3 table handle
/// * `iBlockid` - Access the row with blockid=$iBlockid
/// * `paBlob` - OUT: Blob data in malloc'd buffer
/// * `pnBlob` - OUT: Size of blob data
/// * `pnLoad` - OUT: Bytes actually loaded
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3ReadBlock(
    mut p: *mut Fts3Table,
    mut iBlockid: i64,
    mut paBlob: *mut *mut i8,
    mut pnBlob: *mut i32,
    mut pnLoad: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    // pnBlob must be non-NULL. paBlob may be NULL or non-NULL.
    0 as i32;
    if (unsafe { (*p).pSegments }) != std::ptr::null_mut::<sqlite3_blob>() {
        rc = unsafe { sqlite3_blob_reopen(unsafe { (*p).pSegments }, iBlockid) };
    } else {
        if std::ptr::null_mut::<i8>() == unsafe { (*p).zSegmentsTbl } {
            unsafe {
                (*p).zSegmentsTbl = unsafe {
                    sqlite3_mprintf(
                        (b"%s_segments\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { (*p).zName },
                    )
                };
            }
            if std::ptr::null_mut::<i8>() == unsafe { (*p).zSegmentsTbl } {
                return 7 as i32;
            }
        }
        rc = unsafe {
            sqlite3_blob_open(
                unsafe { (*p).db },
                unsafe { (*p).zDb },
                (unsafe { (*p).zSegmentsTbl }) as *const i8,
                (b"block\0".as_ptr() as *mut i8) as *const i8,
                iBlockid,
                0 as i32,
                unsafe { std::ptr::addr_of_mut!((*p).pSegments) },
            )
        };
    }
    if rc == (0 as i32) {
        let mut nByte: i32 = unsafe { sqlite3_blob_bytes(unsafe { (*p).pSegments }) };
        unsafe {
            *pnBlob = nByte;
        }
        if paBlob != std::ptr::null_mut::<*mut i8>() {
            let mut aByte: *mut i8 = (unsafe {
                sqlite3_malloc64(((nByte as i64) + (((10 as i32) * (2 as i32)) as i64)) as u64)
            }) as *mut i8;
            if !(aByte != std::ptr::null_mut::<i8>()) {
                rc = 7 as i32;
            } else {
                if pnLoad != std::ptr::null_mut::<i32>()
                    && nByte > (4 as i32) * (1024 as i32) * (4 as i32)
                {
                    nByte = (4 as i32) * (1024 as i32);
                    unsafe {
                        *pnLoad = nByte;
                    }
                }
                rc = unsafe {
                    sqlite3_blob_read(unsafe { (*p).pSegments }, aByte as *mut (), nByte, 0 as i32)
                };
                unsafe {
                    memset(
                        (unsafe { aByte.offset(nByte as isize) }) as *mut (),
                        0 as i32,
                        (((10 as i32) * (2 as i32)) as i64) as u64,
                    )
                };
                if rc != (0 as i32) {
                    unsafe { sqlite3_free(aByte as *mut ()) };
                    aByte = std::ptr::null_mut::<i8>();
                }
            }
            unsafe {
                *paBlob = aByte;
            }
        }
    } else {
        if rc == (1 as i32) {
            rc = (11 as i32) | (1 as i32) << (8 as i32);
        }
    }
    return rc;
}

/// Close the blob handle at p->pSegments, if it is open. See comments above
/// the sqlite3Fts3ReadBlock() function for details.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3SegmentsClose(mut p: *mut Fts3Table) {
    unsafe { sqlite3_blob_close(unsafe { (*p).pSegments }) };
    unsafe {
        (*p).pSegments = std::ptr::null_mut::<sqlite3_blob>();
    }
}

fn fts3SegReaderIncrRead(mut pReader: *mut Fts3SegReader) -> i32 {
    let mut nRead: i32 = 0 as i32; // Number of bytes to read
    let mut rc: i32 = 0 as i32; // Return code
    nRead = if (unsafe { (*pReader).nNode }) - unsafe { (*pReader).nPopulate }
        < (4 as i32) * (1024 as i32)
    {
        (unsafe { (*pReader).nNode }) - unsafe { (*pReader).nPopulate }
    } else {
        (4 as i32) * (1024 as i32)
    };
    rc = unsafe {
        sqlite3_blob_read(
            unsafe { (*pReader).pBlob },
            (unsafe {
                unsafe { (*pReader).aNode }.offset((unsafe { (*pReader).nPopulate }) as isize)
            }) as *mut (),
            nRead,
            unsafe { (*pReader).nPopulate },
        )
    };
    if rc == (0 as i32) {
        let __v1721: *mut Fts3SegReader = pReader;
        let __v1722: i32 = unsafe { (*__v1721).nPopulate };
        let __v1723: i32 = __v1722 + nRead;
        unsafe {
            (*__v1721).nPopulate = __v1723;
        }
        unsafe {
            memset(
                (unsafe {
                    unsafe { (*pReader).aNode }.offset((unsafe { (*pReader).nPopulate }) as isize)
                }) as *mut (),
                0 as i32,
                (((10 as i32) * (2 as i32)) as i64) as u64,
            )
        };
        if (unsafe { (*pReader).nPopulate }) == unsafe { (*pReader).nNode } {
            unsafe { sqlite3_blob_close(unsafe { (*pReader).pBlob }) };
            unsafe {
                (*pReader).pBlob = std::ptr::null_mut::<sqlite3_blob>();
            }
            unsafe {
                (*pReader).nPopulate = 0 as i32;
            }
        }
    }
    return rc;
}

fn fts3SegReaderRequire(
    mut pReader: *mut Fts3SegReader,
    mut pFrom: *mut i8,
    mut nByte: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    '__slate_break_1523: while (unsafe { (*pReader).pBlob }) != std::ptr::null_mut::<sqlite3_blob>()
        && rc == (0 as i32)
        && ((unsafe { pFrom.offset_from((unsafe { (*pReader).aNode }) as *mut i8) }) as i64)
            + (nByte as i64)
            > ((unsafe { (*pReader).nPopulate }) as i64)
    {
        rc = fts3SegReaderIncrRead(pReader);
    }
    return rc;
}

/// Set an Fts3SegReader cursor to point at EOF.
fn fts3SegReaderSetEof(mut pSeg: *mut Fts3SegReader) {
    if !((((unsafe { (*pSeg).rootOnly }) as u32) as i32) != (0 as i32)) {
        unsafe { sqlite3_free((unsafe { (*pSeg).aNode }) as *mut ()) };
        unsafe { sqlite3_blob_close(unsafe { (*pSeg).pBlob }) };
        unsafe {
            (*pSeg).pBlob = std::ptr::null_mut::<sqlite3_blob>();
        }
    }
    unsafe {
        (*pSeg).aNode = std::ptr::null_mut::<i8>();
    }
}

/// Move the iterator passed as the first argument to the next term in the
/// segment. If successful, SQLITE_OK is returned. If there is no next term,
/// SQLITE_DONE. Otherwise, an SQLite error code.
fn fts3SegReaderNext(
    mut p: *mut Fts3Table,
    mut pReader: *mut Fts3SegReader,
    mut bIncr: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code of various sub-routines
    let mut pNext: *mut i8 = unsafe { std::mem::zeroed() }; // Cursor variable
    let mut nPrefix: i32 = 0 as i32; // Number of bytes in term prefix
    let mut nSuffix: i32 = 0 as i32; // Number of bytes in term suffix
    if !((unsafe { (*pReader).aDoclist }) != std::ptr::null_mut::<i8>()) {
        pNext = unsafe { (*pReader).aNode };
    } else {
        pNext = unsafe {
            unsafe { (*pReader).aDoclist }.offset((unsafe { (*pReader).nDoclist }) as isize)
        };
    }
    if !(pNext != std::ptr::null_mut::<i8>())
        || pNext
            >= unsafe { unsafe { (*pReader).aNode }.offset((unsafe { (*pReader).nNode }) as isize) }
    {
        if (unsafe { (*pReader).ppNextElem }) != std::ptr::null_mut::<*mut Fts3HashElem>() {
            let mut pElem: *mut Fts3HashElem = unsafe { *unsafe { (*pReader).ppNextElem } };
            unsafe { sqlite3_free((unsafe { (*pReader).aNode }) as *mut ()) };
            unsafe {
                (*pReader).aNode = std::ptr::null_mut::<i8>();
            }
            if pElem != std::ptr::null_mut::<Fts3HashElem>() {
                let mut aCopy: *mut i8 = unsafe { std::mem::zeroed() };
                let mut pList: *mut PendingList = (unsafe { (*pElem).data }) as *mut PendingList;
                let mut nCopy: i32 = ((unsafe { (*pList).nData }) + ((1 as i32) as i64)) as i32;
                let mut nTerm: i32 = unsafe { (*pElem).nKey };
                if nTerm + (1 as i32) > unsafe { (*pReader).nTermAlloc } {
                    unsafe { sqlite3_free((unsafe { (*pReader).zTerm }) as *mut ()) };
                    unsafe {
                        (*pReader).zTerm = (unsafe {
                            sqlite3_malloc64(
                                (((nTerm as i64) + ((1 as i32) as i64)) * ((2 as i32) as i64))
                                    as u64,
                            )
                        }) as *mut i8;
                    }
                    if !((unsafe { (*pReader).zTerm }) != std::ptr::null_mut::<i8>()) {
                        return 7 as i32;
                    }
                    unsafe {
                        (*pReader).nTermAlloc = (nTerm + (1 as i32)) * (2 as i32);
                    }
                }
                unsafe {
                    memcpy(
                        (unsafe { (*pReader).zTerm }) as *mut (),
                        (unsafe { (*pElem).pKey }) as *const (),
                        (nTerm as i64) as u64,
                    )
                };
                unsafe {
                    *unsafe { unsafe { (*pReader).zTerm }.offset(nTerm as isize) } =
                        (0 as i32) as i8;
                }
                unsafe {
                    (*pReader).nTerm = nTerm;
                }
                aCopy = (unsafe { sqlite3_malloc64((nCopy as i64) as u64) }) as *mut i8;
                if !(aCopy != std::ptr::null_mut::<i8>()) {
                    return 7 as i32;
                }
                unsafe {
                    memcpy(
                        aCopy as *mut (),
                        (unsafe { (*pList).aData }) as *const (),
                        (nCopy as i64) as u64,
                    )
                };
                let __v1724: i32 = nCopy;
                unsafe {
                    (*pReader).nDoclist = __v1724;
                }
                unsafe {
                    (*pReader).nNode = __v1724;
                }
                let __v1725: *mut i8 = aCopy;
                unsafe {
                    (*pReader).aDoclist = __v1725;
                }
                unsafe {
                    (*pReader).aNode = __v1725;
                }
                let __v1726: *mut Fts3SegReader = pReader;
                let __v1727: *mut *mut Fts3HashElem = unsafe { (*__v1726).ppNextElem };
                let __v1728: *mut *mut Fts3HashElem =
                    unsafe { __v1727.offset((1 as i32) as isize) };
                unsafe {
                    (*__v1726).ppNextElem = __v1728;
                }
                0 as i32;
            }
            return 0 as i32;
        }
        fts3SegReaderSetEof(pReader);
        // If iCurrentBlock>=iLeafEndBlock, this is an EOF condition. All leaf
        // blocks have already been traversed.
        if (unsafe { (*pReader).iCurrentBlock }) >= unsafe { (*pReader).iLeafEndBlock } {
            return 0 as i32;
        }
        let __v1729: *mut Fts3SegReader = pReader;
        let __v1730: i64 = unsafe { (*__v1729).iCurrentBlock };
        let __v1731: i64 = __v1730 + ((1 as i32) as i64);
        unsafe {
            (*__v1729).iCurrentBlock = __v1731;
        }
        rc = sqlite3Fts3ReadBlock(
            p,
            __v1731,
            unsafe { std::ptr::addr_of_mut!((*pReader).aNode) },
            unsafe { std::ptr::addr_of_mut!((*pReader).nNode) },
            if bIncr != (0 as i32) {
                unsafe { std::ptr::addr_of_mut!((*pReader).nPopulate) }
            } else {
                std::ptr::null_mut::<i32>()
            },
        );
        if rc != (0 as i32) {
            return rc;
        }
        0 as i32;
        if bIncr != (0 as i32) && (unsafe { (*pReader).nPopulate }) < unsafe { (*pReader).nNode } {
            unsafe {
                (*pReader).pBlob = unsafe { (*p).pSegments };
            }
            unsafe {
                (*p).pSegments = std::ptr::null_mut::<sqlite3_blob>();
            }
        }
        pNext = unsafe { (*pReader).aNode };
    }
    0 as i32;
    rc = fts3SegReaderRequire(pReader, pNext, (10 as i32) * (2 as i32));
    if rc != (0 as i32) {
        return rc;
    }
    // Because of the FTS3_NODE_PADDING bytes of padding, the following is
    // safe (no risk of overread) even if the node data is corrupted.
    let __v1732: *mut i8 = pNext;
    let __v1733: i32;
    if (((unsafe { *(pNext as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
        __v1733 =
            unsafe { sqlite3Fts3GetVarint32(pNext as *const i8, std::ptr::addr_of_mut!(nPrefix)) };
    } else {
        unsafe {
            *std::ptr::addr_of_mut!(nPrefix) = ((unsafe { *(pNext as *mut u8) }) as u32) as i32;
        }
        __v1733 = 1 as i32;
    }
    let __v1734: *mut i8 = unsafe { __v1732.offset(__v1733 as isize) };
    pNext = __v1734;
    let __v1735: *mut i8 = pNext;
    let __v1736: i32;
    if (((unsafe { *(pNext as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
        __v1736 =
            unsafe { sqlite3Fts3GetVarint32(pNext as *const i8, std::ptr::addr_of_mut!(nSuffix)) };
    } else {
        unsafe {
            *std::ptr::addr_of_mut!(nSuffix) = ((unsafe { *(pNext as *mut u8) }) as u32) as i32;
        }
        __v1736 = 1 as i32;
    }
    let __v1737: *mut i8 = unsafe { __v1735.offset(__v1736 as isize) };
    pNext = __v1737;
    if nSuffix <= (0 as i32)
        || ((unsafe {
            unsafe { unsafe { (*pReader).aNode }.offset((unsafe { (*pReader).nNode }) as isize) }
                .offset_from(pNext as *mut i8)
        }) as i64)
            < (nSuffix as i64)
        || nPrefix > unsafe { (*pReader).nTerm }
    {
        return (11 as i32) | (1 as i32) << (8 as i32);
    }
    // Both nPrefix and nSuffix were read by fts3GetVarint32() and so are
    // between 0 and 0x7FFFFFFF. But the sum of the two may cause integer
    // overflow - hence the (i64) casts.
    if (nPrefix as i64) + (nSuffix as i64) > ((unsafe { (*pReader).nTermAlloc }) as i64) {
        let mut nNew: i64 = ((nPrefix as i64) + (nSuffix as i64)) * ((2 as i32) as i64);
        let mut zNew: *mut i8 =
            (unsafe { sqlite3_realloc64((unsafe { (*pReader).zTerm }) as *mut (), nNew as u64) })
                as *mut i8;
        if !(zNew != std::ptr::null_mut::<i8>()) {
            return 7 as i32;
        }
        unsafe {
            (*pReader).zTerm = zNew;
        }
        unsafe {
            (*pReader).nTermAlloc = nNew as i32;
        }
    }
    rc = fts3SegReaderRequire(pReader, pNext, nSuffix + (10 as i32));
    if rc != (0 as i32) {
        return rc;
    }
    unsafe {
        memcpy(
            (unsafe { unsafe { (*pReader).zTerm }.offset(nPrefix as isize) }) as *mut (),
            pNext as *const (),
            (nSuffix as i64) as u64,
        )
    };
    unsafe {
        (*pReader).nTerm = nPrefix + nSuffix;
    }
    let __v1738: *mut i8 = pNext;
    let __v1739: *mut i8 = unsafe { __v1738.offset(nSuffix as isize) };
    pNext = __v1739;
    let __v1740: *mut i8 = pNext;
    let __v1741: i32;
    if (((unsafe { *(pNext as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
        __v1741 = unsafe {
            sqlite3Fts3GetVarint32(pNext as *const i8, unsafe {
                std::ptr::addr_of_mut!((*pReader).nDoclist)
            })
        };
    } else {
        unsafe {
            *unsafe { std::ptr::addr_of_mut!((*pReader).nDoclist) } =
                ((unsafe { *(pNext as *mut u8) }) as u32) as i32;
        }
        __v1741 = 1 as i32;
    }
    let __v1742: *mut i8 = unsafe { __v1740.offset(__v1741 as isize) };
    pNext = __v1742;
    unsafe {
        (*pReader).aDoclist = pNext;
    }
    unsafe {
        (*pReader).pOffsetList = std::ptr::null_mut::<i8>();
    }
    // Check that the doclist does not appear to extend past the end of the
    // b-tree node. And that the final byte of the doclist is 0x00. If either
    // of these statements is untrue, then the data structure is corrupt.
    if ((unsafe { (*pReader).nDoclist }) as i64)
        > ((unsafe { (*pReader).nNode }) as i64)
            - ((unsafe {
                unsafe { (*pReader).aDoclist }.offset_from((unsafe { (*pReader).aNode }) as *mut i8)
            }) as i64)
        || (unsafe { (*pReader).nPopulate }) == (0 as i32)
            && (unsafe {
                *unsafe {
                    unsafe { (*pReader).aDoclist }
                        .offset(((unsafe { (*pReader).nDoclist }) - (1 as i32)) as isize)
                }
            }) != (0 as i8)
        || (unsafe { (*pReader).nDoclist }) == (0 as i32)
    {
        return (11 as i32) | (1 as i32) << (8 as i32);
    }
    return 0 as i32;
}

/// Set the SegReader to point to the first docid in the doclist associated
/// with the current term.
fn fts3SegReaderFirstDocid(mut pTab: *mut Fts3Table, mut pReader: *mut Fts3SegReader) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe { (*pTab).bDescIdx }) != (0 as u8)
        && (unsafe { (*pReader).ppNextElem }) != std::ptr::null_mut::<*mut Fts3HashElem>()
    {
        let mut bEof: u8 = ((0 as i32) as i8) as u8;
        unsafe {
            (*pReader).iDocid = (0 as i32) as i64;
        }
        unsafe {
            (*pReader).nOffsetList = 0 as i32;
        }
        unsafe {
            sqlite3Fts3DoclistPrev(
                0 as i32,
                unsafe { (*pReader).aDoclist },
                unsafe { (*pReader).nDoclist },
                unsafe { std::ptr::addr_of_mut!((*pReader).pOffsetList) },
                unsafe { std::ptr::addr_of_mut!((*pReader).iDocid) },
                unsafe { std::ptr::addr_of_mut!((*pReader).nOffsetList) },
                std::ptr::addr_of_mut!(bEof),
            )
        };
    } else {
        rc = fts3SegReaderRequire(pReader, unsafe { (*pReader).aDoclist }, 10 as i32);
        if rc == (0 as i32) {
            let mut n: i32 = unsafe {
                sqlite3Fts3GetVarint((unsafe { (*pReader).aDoclist }) as *const i8, unsafe {
                    std::ptr::addr_of_mut!((*pReader).iDocid)
                })
            };
            unsafe {
                (*pReader).pOffsetList =
                    unsafe { unsafe { (*pReader).aDoclist }.offset(n as isize) };
            }
        }
    }
    return rc;
}

/// Advance the SegReader to point to the next docid in the doclist
/// associated with the current term.
///
/// If arguments ppOffsetList and pnOffsetList are not NULL, then
/// *ppOffsetList is set to point to the first column-offset list
/// in the doclist entry (i.e. immediately past the docid varint).
/// *pnOffsetList is set to the length of the set of column-offset
/// lists, not including the nul-terminator byte. For example:
///
/// # Arguments
///
/// * `pReader` - Reader to advance to next docid
/// * `ppOffsetList` - OUT: Pointer to current position-list
/// * `pnOffsetList` - OUT: Length of *ppOffsetList in bytes
fn fts3SegReaderNextDocid(
    mut pTab: *mut Fts3Table,
    mut pReader: *mut Fts3SegReader,
    mut ppOffsetList: *mut *mut i8,
    mut pnOffsetList: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut p: *mut i8 = unsafe { (*pReader).pOffsetList };
    let mut c: i8 = (0 as i32) as i8;
    0 as i32;
    if (unsafe { (*pTab).bDescIdx }) != (0 as u8)
        && (unsafe { (*pReader).ppNextElem }) != std::ptr::null_mut::<*mut Fts3HashElem>()
    {
        // A pending-terms seg-reader for an FTS4 table that uses order=desc.
        // Pending-terms doclists are always built up in ascending order, so
        // we have to iterate through them backwards here.
        let mut bEof: u8 = ((0 as i32) as i8) as u8;
        if ppOffsetList != std::ptr::null_mut::<*mut i8>() {
            unsafe {
                *ppOffsetList = unsafe { (*pReader).pOffsetList };
            }
            unsafe {
                *pnOffsetList = (unsafe { (*pReader).nOffsetList }) - (1 as i32);
            }
        }
        unsafe {
            sqlite3Fts3DoclistPrev(
                0 as i32,
                unsafe { (*pReader).aDoclist },
                unsafe { (*pReader).nDoclist },
                std::ptr::addr_of_mut!(p),
                unsafe { std::ptr::addr_of_mut!((*pReader).iDocid) },
                unsafe { std::ptr::addr_of_mut!((*pReader).nOffsetList) },
                std::ptr::addr_of_mut!(bEof),
            )
        };
        if bEof != (0 as u8) {
            unsafe {
                (*pReader).pOffsetList = std::ptr::null_mut::<i8>();
            }
        } else {
            unsafe {
                (*pReader).pOffsetList = p;
            }
        }
    } else {
        let mut pEnd: *mut i8 = unsafe {
            unsafe { (*pReader).aDoclist }.offset((unsafe { (*pReader).nDoclist }) as isize)
        };
        // Pointer p currently points at the first byte of an offset list. The
        // following block advances it to point one byte past the end of
        // the same offset list.
        '__slate_break_1524: while (1 as i32) != (0 as i32) {
            // The following line of code (and the "p++" below the while() loop) is
            // normally all that is required to move pointer p to the desired
            // position. The exception is if this node is being loaded from disk
            // incrementally and pointer "p" now points to the first byte past
            // the populated part of pReader->aNode[].
            '__slate_break_1525: while ((unsafe { *p }) as i32) | (c as i32) != (0 as i32) {
                let __v1743: *mut i8 = p;
                let __v1744: *mut i8 = unsafe { __v1743.offset((1 as i32) as isize) };
                p = __v1744;
                c = (((unsafe { *__v1743 }) as i32) & (128 as i32)) as i8;
            }
            0 as i32;
            if (unsafe { (*pReader).pBlob }) == std::ptr::null_mut::<sqlite3_blob>()
                || p < unsafe {
                    unsafe { (*pReader).aNode }.offset((unsafe { (*pReader).nPopulate }) as isize)
                }
            {
                break '__slate_break_1524;
            }
            rc = fts3SegReaderIncrRead(pReader);
            if rc != (0 as i32) {
                return rc;
            }
        }
        let __v1745: *mut i8 = p;
        let __v1746: *mut i8 = unsafe { __v1745.offset((1 as i32) as isize) };
        p = __v1746;
        // If required, populate the output variables with a pointer to and the
        // size of the previous offset-list.
        if ppOffsetList != std::ptr::null_mut::<*mut i8>() {
            unsafe {
                *ppOffsetList = unsafe { (*pReader).pOffsetList };
            }
            unsafe {
                *pnOffsetList =
                    (((unsafe { p.offset_from((unsafe { (*pReader).pOffsetList }) as *mut i8) })
                        as i64)
                        - ((1 as i32) as i64)) as i32;
            }
        }
        // List may have been edited in place by fts3EvalNearTrim()
        '__slate_break_1526: while p < pEnd && ((unsafe { *p }) as i32) == (0 as i32) {
            let __v1747: *mut i8 = p;
            let __v1748: *mut i8 = unsafe { __v1747.offset((1 as i32) as isize) };
            p = __v1748;
        }
        // If there are no more entries in the doclist, set pOffsetList to
        // NULL. Otherwise, set Fts3SegReader.iDocid to the next docid and
        // Fts3SegReader.pOffsetList to point to the next offset list before
        // returning.
        if p >= pEnd {
            unsafe {
                (*pReader).pOffsetList = std::ptr::null_mut::<i8>();
            }
        } else {
            rc = fts3SegReaderRequire(pReader, p, 10 as i32);
            if rc == (0 as i32) {
                let mut iDelta: u64 = 0 as u64;
                unsafe {
                    (*pReader).pOffsetList = unsafe {
                        p.offset(
                            (unsafe {
                                sqlite3Fts3GetVarintU(
                                    p as *const i8,
                                    std::ptr::addr_of_mut!(iDelta),
                                )
                            }) as isize,
                        )
                    };
                }
                if (unsafe { (*pTab).bDescIdx }) != (0 as u8) {
                    unsafe {
                        (*pReader).iDocid =
                            ((unsafe { (*pReader).iDocid }) as u64).wrapping_sub(iDelta) as i64;
                    }
                } else {
                    unsafe {
                        (*pReader).iDocid =
                            ((unsafe { (*pReader).iDocid }) as u64).wrapping_add(iDelta) as i64;
                    }
                }
            }
        }
    }
    return rc;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3MsrOvfl(
    mut pCsr: *mut Fts3Cursor,
    mut pMsr: *mut Fts3MultiSegReader,
    mut pnOvfl: *mut i32,
) -> i32 {
    let mut p: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
    let mut nOvfl: i32 = 0 as i32;
    let mut ii: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut pgsz: i32 = unsafe { (*p).nPgsz };
    0 as i32;
    0 as i32;
    ii = 0 as i32;
    '__slate_break_1527: loop {
        if !(rc == (0 as i32) && ii < unsafe { (*pMsr).nSegment }) {
            break;
        }
        let mut pReader: *mut Fts3SegReader =
            unsafe { *unsafe { unsafe { (*pMsr).apSegment }.offset(ii as isize) } };
        if !((unsafe { (*pReader).ppNextElem }) != std::ptr::null_mut::<*mut Fts3HashElem>())
            && !((((unsafe { (*pReader).rootOnly }) as u32) as i32) != (0 as i32))
        {
            let mut jj: i64 = 0 as i64;
            jj = unsafe { (*pReader).iStartBlock };
            '__slate_break_1528: loop {
                if !(jj <= unsafe { (*pReader).iLeafEndBlock }) {
                    break;
                }
                let mut nBlob: i32 = 0 as i32;
                rc = sqlite3Fts3ReadBlock(
                    p,
                    jj,
                    std::ptr::null_mut::<*mut i8>(),
                    std::ptr::addr_of_mut!(nBlob),
                    std::ptr::null_mut::<i32>(),
                );
                if rc != (0 as i32) {
                    break '__slate_break_1528;
                }
                if nBlob + (35 as i32) > pgsz {
                    let __v1670: i32 = nOvfl;
                    let __v1671: i32 = __v1670 + (nBlob + (34 as i32)) / pgsz;
                    nOvfl = __v1671;
                }
                let __v1668: i64 = jj;
                let __v1669: i64 = __v1668 + ((1 as i32) as i64);
                jj = __v1669;
            }
        }
        let __v1666: i32 = ii;
        let __v1667: i32 = __v1666 + (1 as i32);
        ii = __v1667;
    }
    unsafe {
        *pnOvfl = nOvfl;
    }
    return rc;
}

/// Free all allocations associated with the iterator passed as the
/// second argument.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3SegReaderFree(mut pReader: *mut Fts3SegReader) {
    if pReader != std::ptr::null_mut::<Fts3SegReader>() {
        unsafe { sqlite3_free((unsafe { (*pReader).zTerm }) as *mut ()) };
        if !((((unsafe { (*pReader).rootOnly }) as u32) as i32) != (0 as i32)) {
            unsafe { sqlite3_free((unsafe { (*pReader).aNode }) as *mut ()) };
        }
        unsafe { sqlite3_blob_close(unsafe { (*pReader).pBlob }) };
    }
    unsafe { sqlite3_free(pReader as *mut ()) };
}

/// Allocate a new SegReader object.
///
/// # Arguments
///
/// * `iAge` - Segment "age".
/// * `bLookup` - True for a lookup only
/// * `iStartLeaf` - First leaf to traverse
/// * `iEndLeaf` - Final leaf to traverse
/// * `iEndBlock` - Final block of segment
/// * `zRoot` - Buffer containing root node
/// * `nRoot` - Size of buffer containing root node
/// * `ppReader` - OUT: Allocated Fts3SegReader
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3SegReaderNew(
    mut iAge: i32,
    mut bLookup: i32,
    mut iStartLeaf: i64,
    mut iEndLeaf: i64,
    mut iEndBlock: i64,
    mut zRoot: *const i8,
    mut nRoot: i32,
    mut ppReader: *mut *mut Fts3SegReader,
) -> i32 {
    let mut pReader: *mut Fts3SegReader = unsafe { std::mem::zeroed() }; // Newly allocated SegReader object
    let mut nExtra: i32 = 0 as i32; // Bytes to allocate segment root node
    0 as i32;
    if iStartLeaf == ((0 as i32) as i64) {
        if iEndLeaf != ((0 as i32) as i64) {
            return (11 as i32) | (1 as i32) << (8 as i32);
        }
        nExtra = nRoot + (10 as i32) * (2 as i32);
    }
    pReader = (unsafe { sqlite3_malloc64((136 as u64).wrapping_add((nExtra as i64) as u64)) })
        as *mut Fts3SegReader;
    if !(pReader != std::ptr::null_mut::<Fts3SegReader>()) {
        return 7 as i32;
    }
    unsafe { memset(pReader as *mut (), 0 as i32, 136 as u64) };
    unsafe {
        (*pReader).iIdx = iAge;
    }
    unsafe {
        (*pReader).bLookup = (bLookup != (0 as i32)) as u8;
    }
    unsafe {
        (*pReader).iStartBlock = iStartLeaf;
    }
    unsafe {
        (*pReader).iLeafEndBlock = iEndLeaf;
    }
    unsafe {
        (*pReader).iEndBlock = iEndBlock;
    }
    if nExtra != (0 as i32) {
        // The entire segment is stored in the root node.
        unsafe {
            (*pReader).aNode = (unsafe { pReader.offset((1 as i32) as isize) }) as *mut i8;
        }
        unsafe {
            (*pReader).rootOnly = ((1 as i32) as i8) as u8;
        }
        unsafe {
            (*pReader).nNode = nRoot;
        }
        if nRoot != (0 as i32) {
            unsafe {
                memcpy(
                    (unsafe { (*pReader).aNode }) as *mut (),
                    zRoot as *const (),
                    (nRoot as i64) as u64,
                )
            };
        }
        unsafe {
            memset(
                (unsafe { unsafe { (*pReader).aNode }.offset(nRoot as isize) }) as *mut (),
                0 as i32,
                (((10 as i32) * (2 as i32)) as i64) as u64,
            )
        };
    } else {
        unsafe {
            (*pReader).iCurrentBlock = iStartLeaf - ((1 as i32) as i64);
        }
    }
    unsafe {
        *ppReader = pReader;
    }
    return 0 as i32;
}

/// This is a comparison function used as a qsort() callback when sorting
/// an array of pending terms by term. This occurs as part of flushing
/// the contents of the pending-terms hash table to the database.
#[unsafe(link_section = ".text.slate_distinct.fts3_write.fts3CompareElemByTerm")]
extern "C-unwind" fn fts3CompareElemByTerm(mut lhs: *const (), mut rhs: *const ()) -> i32 {
    let mut z1: *mut i8 =
        (unsafe { (*unsafe { *(lhs as *mut *mut Fts3HashElem) }).pKey }) as *mut i8;
    let mut z2: *mut i8 =
        (unsafe { (*unsafe { *(rhs as *mut *mut Fts3HashElem) }).pKey }) as *mut i8;
    let mut n1: i32 = unsafe { (*unsafe { *(lhs as *mut *mut Fts3HashElem) }).nKey };
    let mut n2: i32 = unsafe { (*unsafe { *(rhs as *mut *mut Fts3HashElem) }).nKey };
    let mut n: i32 = if n1 < n2 { n1 } else { n2 };
    let mut c: i32 = unsafe { memcmp(z1 as *const (), z2 as *const (), (n as i64) as u64) };
    if c == (0 as i32) {
        c = n1 - n2;
    }
    return c;
}

/// This function is used to allocate an Fts3SegReader that iterates through
/// a subset of the terms stored in the Fts3Table.pendingTerms array.
///
/// If the isPrefixIter parameter is zero, then the returned SegReader iterates
/// through each term in the pending-terms table. Or, if isPrefixIter is
/// non-zero, it iterates through each term and its prefixes. For example, if
/// the pending terms hash table contains the terms "sqlite", "mysql" and
/// "firebird", then the iterator visits the following 'terms' (in the order
/// shown):
///
///   f fi fir fire fireb firebi firebir firebird
///   m my mys mysq mysql
///   s sq sql sqli sqlit sqlite
///
/// Whereas if isPrefixIter is zero, the terms visited are:
///
///   firebird mysql sqlite
///
/// # Arguments
///
/// * `p` - Virtual table handle
/// * `iIndex` - Index for p->aIndex
/// * `zTerm` - Term to search for
/// * `nTerm` - Size of buffer zTerm
/// * `bPrefix` - True for a prefix iterator
/// * `ppReader` - OUT: SegReader for pending-terms
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3SegReaderPending(
    mut p: *mut Fts3Table,
    mut iIndex: i32,
    mut zTerm: *const i8,
    mut nTerm: i32,
    mut bPrefix: i32,
    mut ppReader: *mut *mut Fts3SegReader,
) -> i32 {
    let mut pReader: *mut Fts3SegReader = std::ptr::null_mut::<Fts3SegReader>(); // Fts3SegReader object to return
    let mut pE: *mut Fts3HashElem = unsafe { std::mem::zeroed() }; // Iterator variable
    let mut aElem: *mut *mut Fts3HashElem = std::ptr::null_mut::<*mut Fts3HashElem>(); // Array of term hash entries to scan
    let mut nElem: i32 = 0 as i32; // Size of array at aElem
    let mut rc: i32 = 0 as i32; // Return Code
    let mut pHash: *mut Fts3Hash = unsafe { std::mem::zeroed() };
    pHash = unsafe {
        std::ptr::addr_of_mut!(
            (*unsafe { unsafe { (*p).aIndex }.offset(iIndex as isize) }).hPending
        )
    };
    if bPrefix != (0 as i32) {
        let mut nAlloc: i32 = 0 as i32; // Size of allocated array at aElem
        pE = unsafe { (*pHash).first };
        '__slate_break_1529: while pE != std::ptr::null_mut::<Fts3HashElem>() {
            let mut zKey: *mut i8 = (unsafe { (*pE).pKey }) as *mut i8;
            let mut nKey: i32 = unsafe { (*pE).nKey };
            if nTerm == (0 as i32)
                || nKey >= nTerm
                    && (0 as i32)
                        == unsafe {
                            memcmp(zKey as *const (), zTerm as *const (), (nTerm as i64) as u64)
                        }
            {
                if nElem == nAlloc {
                    let mut aElem2: *mut *mut Fts3HashElem = unsafe { std::mem::zeroed() };
                    let __v1627: i32 = nAlloc;
                    let __v1628: i32 = __v1627 + (16 as i32);
                    nAlloc = __v1628;
                    aElem2 = (unsafe {
                        sqlite3_realloc64(
                            aElem as *mut (),
                            ((nAlloc as i64) as u64).wrapping_mul(8 as u64),
                        )
                    }) as *mut *mut Fts3HashElem;
                    if !(aElem2 != std::ptr::null_mut::<*mut Fts3HashElem>()) {
                        rc = 7 as i32;
                        nElem = 0 as i32;
                        break '__slate_break_1529;
                    }
                    aElem = aElem2;
                }
                let __v1629: i32 = nElem;
                let __v1630: i32 = __v1629 + (1 as i32);
                nElem = __v1630;
                unsafe {
                    *unsafe { aElem.offset(__v1629 as isize) } = pE;
                }
            }
            pE = unsafe { (*pE).next };
        }
        // If more than one term matches the prefix, sort the Fts3HashElem
        // objects in term order using qsort(). This uses the same comparison
        // callback as is used when flushing terms to disk.
        if nElem > (1 as i32) {
            unsafe {
                qsort(
                    aElem as *mut (),
                    (nElem as i64) as u64,
                    8 as u64,
                    Some(fts3CompareElemByTerm),
                )
            };
        }
    } else {
        // The query is a simple term lookup that matches at most one term in
        // the index. All that is required is a straight hash-lookup.
        //
        // Because the stack address of pE may be accessed via the aElem pointer
        // below, the "Fts3HashElem *pE" must be declared so that it is valid
        // within this entire function, not just this "else{...}" block.
        pE =
            unsafe { sqlite3Fts3HashFindElem(pHash as *const Fts3Hash, zTerm as *const (), nTerm) };
        if pE != std::ptr::null_mut::<Fts3HashElem>() {
            aElem = std::ptr::addr_of_mut!(pE);
            nElem = 1 as i32;
        }
    }
    if nElem > (0 as i32) {
        let mut nByte: i64 = 0 as i64;
        nByte = (136 as u64)
            .wrapping_add((((nElem + (1 as i32)) as i64) as u64).wrapping_mul(8 as u64))
            as i64;
        pReader = (unsafe { sqlite3_malloc64(nByte as u64) }) as *mut Fts3SegReader;
        if !(pReader != std::ptr::null_mut::<Fts3SegReader>()) {
            rc = 7 as i32;
        } else {
            unsafe { memset(pReader as *mut (), 0 as i32, nByte as u64) };
            unsafe {
                (*pReader).iIdx = 2147483647 as i32;
            }
            unsafe {
                (*pReader).ppNextElem =
                    (unsafe { pReader.offset((1 as i32) as isize) }) as *mut *mut Fts3HashElem;
            }
            unsafe {
                memcpy(
                    (unsafe { (*pReader).ppNextElem }) as *mut (),
                    aElem as *const (),
                    ((nElem as i64) as u64).wrapping_mul(8 as u64),
                )
            };
        }
    }
    if bPrefix != (0 as i32) {
        unsafe { sqlite3_free(aElem as *mut ()) };
    }
    unsafe {
        *ppReader = pReader;
    }
    return rc;
}

/// Compare the entries pointed to by two Fts3SegReader structures.
/// Comparison is as follows:
///
///   1) EOF is greater than not EOF.
///
///   2) The current terms (if any) are compared using memcmp(). If one
///      term is a prefix of another, the longer term is considered the
///      larger.
///
///   3) By segment age. An older segment is considered larger.
#[unsafe(link_section = ".text.slate_distinct.fts3_write.fts3SegReaderCmp")]
extern "C-unwind" fn fts3SegReaderCmp(
    mut pLhs: *mut Fts3SegReader,
    mut pRhs: *mut Fts3SegReader,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pLhs).aNode }) != std::ptr::null_mut::<i8>()
        && (unsafe { (*pRhs).aNode }) != std::ptr::null_mut::<i8>()
    {
        let mut rc2: i32 = (unsafe { (*pLhs).nTerm }) - unsafe { (*pRhs).nTerm };
        if rc2 < (0 as i32) {
            rc = unsafe {
                memcmp(
                    (unsafe { (*pLhs).zTerm }) as *const (),
                    (unsafe { (*pRhs).zTerm }) as *const (),
                    ((unsafe { (*pLhs).nTerm }) as i64) as u64,
                )
            };
        } else {
            rc = unsafe {
                memcmp(
                    (unsafe { (*pLhs).zTerm }) as *const (),
                    (unsafe { (*pRhs).zTerm }) as *const (),
                    ((unsafe { (*pRhs).nTerm }) as i64) as u64,
                )
            };
        }
        if rc == (0 as i32) {
            rc = rc2;
        }
    } else {
        rc = (((unsafe { (*pLhs).aNode }) == std::ptr::null_mut::<i8>()) as i32)
            - (((unsafe { (*pRhs).aNode }) == std::ptr::null_mut::<i8>()) as i32);
    }
    if rc == (0 as i32) {
        rc = (unsafe { (*pRhs).iIdx }) - unsafe { (*pLhs).iIdx };
    }
    0 as i32;
    return rc;
}

/// A different comparison function for SegReader structures. In this
/// version, it is assumed that each SegReader points to an entry in
/// a doclist for identical terms. Comparison is made as follows:
///
///   1) EOF (end of doclist in this case) is greater than not EOF.
///
///   2) By current docid.
///
///   3) By segment age. An older segment is considered larger.
#[unsafe(link_section = ".text.slate_distinct.fts3_write.fts3SegReaderDoclistCmp")]
extern "C-unwind" fn fts3SegReaderDoclistCmp(
    mut pLhs: *mut Fts3SegReader,
    mut pRhs: *mut Fts3SegReader,
) -> i32 {
    let mut rc: i32 = (((unsafe { (*pLhs).pOffsetList }) == std::ptr::null_mut::<i8>()) as i32)
        - (((unsafe { (*pRhs).pOffsetList }) == std::ptr::null_mut::<i8>()) as i32);
    if rc == (0 as i32) {
        if (unsafe { (*pLhs).iDocid }) == unsafe { (*pRhs).iDocid } {
            rc = (unsafe { (*pRhs).iIdx }) - unsafe { (*pLhs).iIdx };
        } else {
            rc = if (unsafe { (*pLhs).iDocid }) > unsafe { (*pRhs).iDocid } {
                1 as i32
            } else {
                -(1 as i32)
            };
        }
    }
    0 as i32;
    return rc;
}

#[unsafe(link_section = ".text.slate_distinct.fts3_write.fts3SegReaderDoclistCmpRev")]
extern "C-unwind" fn fts3SegReaderDoclistCmpRev(
    mut pLhs: *mut Fts3SegReader,
    mut pRhs: *mut Fts3SegReader,
) -> i32 {
    let mut rc: i32 = (((unsafe { (*pLhs).pOffsetList }) == std::ptr::null_mut::<i8>()) as i32)
        - (((unsafe { (*pRhs).pOffsetList }) == std::ptr::null_mut::<i8>()) as i32);
    if rc == (0 as i32) {
        if (unsafe { (*pLhs).iDocid }) == unsafe { (*pRhs).iDocid } {
            rc = (unsafe { (*pRhs).iIdx }) - unsafe { (*pLhs).iIdx };
        } else {
            rc = if (unsafe { (*pLhs).iDocid }) < unsafe { (*pRhs).iDocid } {
                1 as i32
            } else {
                -(1 as i32)
            };
        }
    }
    0 as i32;
    return rc;
}

/// Compare the term that the Fts3SegReader object passed as the first argument
/// points to with the term specified by arguments zTerm and nTerm.
///
/// If the pSeg iterator is already at EOF, return 0. Otherwise, return
/// -ve if the pSeg term is less than zTerm/nTerm, 0 if the two terms are
/// equal, or +ve if the pSeg term is greater than zTerm/nTerm.
///
/// # Arguments
///
/// * `pSeg` - Segment reader object
/// * `zTerm` - Term to compare to
/// * `nTerm` - Size of term zTerm in bytes
fn fts3SegReaderTermCmp(mut pSeg: *mut Fts3SegReader, mut zTerm: *const i8, mut nTerm: i32) -> i32 {
    let mut res: i32 = 0 as i32;
    if (unsafe { (*pSeg).aNode }) != std::ptr::null_mut::<i8>() {
        if (unsafe { (*pSeg).nTerm }) > nTerm {
            res = unsafe {
                memcmp(
                    (unsafe { (*pSeg).zTerm }) as *const (),
                    zTerm as *const (),
                    (nTerm as i64) as u64,
                )
            };
        } else {
            res = unsafe {
                memcmp(
                    (unsafe { (*pSeg).zTerm }) as *const (),
                    zTerm as *const (),
                    ((unsafe { (*pSeg).nTerm }) as i64) as u64,
                )
            };
        }
        if res == (0 as i32) {
            res = (unsafe { (*pSeg).nTerm }) - nTerm;
        }
    }
    return res;
}

/// Argument apSegment is an array of nSegment elements. It is known that
/// the final (nSegment-nSuspect) members are already in sorted order
/// (according to the comparison function provided). This function shuffles
/// the array around until all entries are in sorted order.
///
/// # Arguments
///
/// * `apSegment` - Array to sort entries of
/// * `nSegment` - Size of apSegment array
/// * `nSuspect` - Unsorted entry count
/// * `xCmp` - Comparison function
fn fts3SegReaderSort(
    mut apSegment: *mut *mut Fts3SegReader,
    mut nSegment: i32,
    mut nSuspect: i32,
    mut xCmp: Option<unsafe extern "C-unwind" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> i32>,
) {
    let mut i: i32 = 0 as i32; // Iterator variable
    0 as i32;
    if nSuspect == nSegment {
        let __v1749: i32 = nSuspect;
        let __v1750: i32 = __v1749 - (1 as i32);
        nSuspect = __v1750;
    }
    i = nSuspect - (1 as i32);
    '__slate_break_1530: loop {
        if !(i >= (0 as i32)) {
            break;
        }
        let mut j: i32 = 0 as i32;
        j = i;
        '__slate_break_1531: loop {
            if !(j < nSegment - (1 as i32)) {
                break;
            }
            let mut pTmp: *mut Fts3SegReader = unsafe { std::mem::zeroed() };
            if (unsafe {
                xCmp.unwrap()(
                    unsafe { *unsafe { apSegment.offset(j as isize) } },
                    unsafe { *unsafe { apSegment.offset((j + (1 as i32)) as isize) } },
                )
            }) < (0 as i32)
            {
                break '__slate_break_1531;
            }
            pTmp = unsafe { *unsafe { apSegment.offset((j + (1 as i32)) as isize) } };
            unsafe {
                *unsafe { apSegment.offset((j + (1 as i32)) as isize) } =
                    unsafe { *unsafe { apSegment.offset(j as isize) } };
            }
            unsafe {
                *unsafe { apSegment.offset(j as isize) } = pTmp;
            }
            let __v1753: i32 = j;
            let __v1754: i32 = __v1753 + (1 as i32);
            j = __v1754;
        }
        let __v1751: i32 = i;
        let __v1752: i32 = __v1751 - (1 as i32);
        i = __v1752;
    }
}

/// Insert a record into the %_segments table.
///
/// # Arguments
///
/// * `p` - Virtual table handle
/// * `iBlock` - Block id for new block
/// * `z` - Pointer to buffer containing block data
/// * `n` - Size of buffer z in bytes
fn fts3WriteSegment(mut p: *mut Fts3Table, mut iBlock: i64, mut z: *mut i8, mut n: i32) -> i32 {
    let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    let mut rc: i32 = fts3SqlStmt(
        p,
        9 as i32,
        std::ptr::addr_of_mut!(pStmt),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        unsafe { sqlite3_bind_int64(pStmt, 1 as i32, iBlock) };
        unsafe { sqlite3_bind_blob(pStmt, 2 as i32, z as *const (), n, None) };
        unsafe { sqlite3_step(pStmt) };
        rc = unsafe { sqlite3_reset(pStmt) };
        unsafe { sqlite3_bind_null(pStmt, 2 as i32) };
    }
    return rc;
}

/// Find the largest relative level number in the table. If successful, set
/// *pnMax to this value and return SQLITE_OK. Otherwise, if an error occurs,
/// set *pnMax to zero and return an SQLite error code.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3MaxLevel(mut p: *mut Fts3Table, mut pnMax: *mut i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut mxLevel: i32 = 0 as i32;
    let mut pStmt: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
    rc = fts3SqlStmt(
        p,
        36 as i32,
        std::ptr::addr_of_mut!(pStmt),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        if (100 as i32) == unsafe { sqlite3_step(pStmt) } {
            mxLevel = unsafe { sqlite3_column_int(pStmt, 0 as i32) };
        }
        rc = unsafe { sqlite3_reset(pStmt) };
    }
    unsafe {
        *pnMax = mxLevel;
    }
    return rc;
}

/// Insert a record into the %_segdir table.
///
/// # Arguments
///
/// * `p` - Virtual table handle
/// * `iLevel` - Value for "level" field (absolute level)
/// * `iIdx` - Value for "idx" field
/// * `iStartBlock` - Value for "start_block" field
/// * `iLeafEndBlock` - Value for "leaves_end_block" field
/// * `iEndBlock` - Value for "end_block" field
/// * `nLeafData` - Bytes of leaf data in segment
/// * `zRoot` - Blob value for "root" field
/// * `nRoot` - Number of bytes in buffer zRoot
fn fts3WriteSegdir(
    mut p: *mut Fts3Table,
    mut iLevel: i64,
    mut iIdx: i32,
    mut iStartBlock: i64,
    mut iLeafEndBlock: i64,
    mut iEndBlock: i64,
    mut nLeafData: i64,
    mut zRoot: *mut i8,
    mut nRoot: i32,
) -> i32 {
    let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    let mut rc: i32 = fts3SqlStmt(
        p,
        11 as i32,
        std::ptr::addr_of_mut!(pStmt),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        unsafe { sqlite3_bind_int64(pStmt, 1 as i32, iLevel) };
        unsafe { sqlite3_bind_int(pStmt, 2 as i32, iIdx) };
        unsafe { sqlite3_bind_int64(pStmt, 3 as i32, iStartBlock) };
        unsafe { sqlite3_bind_int64(pStmt, 4 as i32, iLeafEndBlock) };
        if nLeafData == ((0 as i32) as i64) {
            unsafe { sqlite3_bind_int64(pStmt, 5 as i32, iEndBlock) };
        } else {
            let mut zEnd: *mut i8 = unsafe {
                sqlite3_mprintf(
                    (b"%lld %lld\0".as_ptr() as *mut i8) as *const i8,
                    iEndBlock,
                    nLeafData,
                )
            };
            if !(zEnd != std::ptr::null_mut::<i8>()) {
                return 7 as i32;
            }
            unsafe {
                sqlite3_bind_text(pStmt, 5 as i32, zEnd as *const i8, -(1 as i32), unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        sqlite3_free as *const (),
                    )
                })
            };
        }
        unsafe { sqlite3_bind_blob(pStmt, 6 as i32, zRoot as *const (), nRoot, None) };
        unsafe { sqlite3_step(pStmt) };
        rc = unsafe { sqlite3_reset(pStmt) };
        unsafe { sqlite3_bind_null(pStmt, 6 as i32) };
    }
    return rc;
}

/// Return the size of the common prefix (if any) shared by zPrev and
/// zNext, in bytes. For example,
///
///   fts3PrefixCompress("abc", 3, "abcdef", 6)   // returns 3
///   fts3PrefixCompress("abX", 3, "abcdef", 6)   // returns 2
///   fts3PrefixCompress("abX", 3, "Xbcdef", 6)   // returns 0
///
/// # Arguments
///
/// * `zPrev` - Buffer containing previous term
/// * `nPrev` - Size of buffer zPrev in bytes
/// * `zNext` - Buffer containing next term
/// * `nNext` - Size of buffer zNext in bytes
fn fts3PrefixCompress(
    mut zPrev: *const i8,
    mut nPrev: i32,
    mut zNext: *const i8,
    mut nNext: i32,
) -> i32 {
    let mut n: i32 = 0 as i32;
    n = 0 as i32;
    '__slate_break_1533: loop {
        if !(n < nPrev
            && n < nNext
            && ((unsafe { *unsafe { zPrev.offset(n as isize) } }) as i32)
                == ((unsafe { *unsafe { zNext.offset(n as isize) } }) as i32))
        {
            break;
        }
        {}
        let __v1755: i32 = n;
        let __v1756: i32 = __v1755 + (1 as i32);
        n = __v1756;
    }
    0 as i32;
    return n;
}

/// Add term zTerm to the SegmentNode. It is guaranteed that zTerm is larger
/// (according to memcmp) than the previous term.
///
/// # Arguments
///
/// * `p` - Virtual table handle
/// * `ppTree` - IN/OUT: SegmentNode handle
/// * `isCopyTerm` - True if zTerm/nTerm is transient
/// * `zTerm` - Pointer to buffer containing term
/// * `nTerm` - Size of term in bytes
fn fts3NodeAddTerm(
    mut p: *mut Fts3Table,
    mut ppTree: *mut *mut SegmentNode,
    mut isCopyTerm: i32,
    mut zTerm: *const i8,
    mut nTerm: i32,
) -> i32 {
    let mut pTree: *mut SegmentNode = unsafe { *ppTree };
    let mut rc: i32 = 0 as i32;
    let mut pNew: *mut SegmentNode = unsafe { std::mem::zeroed() };
    // First try to append the term to the current node. Return early if
    // this is possible.
    if pTree != std::ptr::null_mut::<SegmentNode>() {
        let mut nData: i32 = unsafe { (*pTree).nData }; // Current size of node in bytes
        let mut nReq: i32 = nData; // Required space after adding zTerm
        let mut nPrefix: i32 = 0 as i32; // Number of bytes of prefix compression
        let mut nSuffix: i32 = 0 as i32; // Suffix length
        nPrefix = fts3PrefixCompress(
            (unsafe { (*pTree).zTerm }) as *const i8,
            unsafe { (*pTree).nTerm },
            zTerm,
            nTerm,
        );
        nSuffix = nTerm - nPrefix;
        // If nSuffix is zero or less, then zTerm/nTerm must be a prefix of
        // pWriter->zTerm/pWriter->nTerm. i.e. must be equal to or less than when
        // compared with BINARY collation. This indicates corruption.
        if nSuffix <= (0 as i32) {
            return (11 as i32) | (1 as i32) << (8 as i32);
        }
        let __v1757: i32 = nReq;
        let __v1758: i32 = __v1757
            + ((unsafe { sqlite3Fts3VarintLen((nPrefix as i64) as u64) })
                + unsafe { sqlite3Fts3VarintLen((nSuffix as i64) as u64) }
                + nSuffix);
        nReq = __v1758;
        if nReq <= unsafe { (*p).nNodeSize }
            || !((unsafe { (*pTree).zTerm }) != std::ptr::null_mut::<i8>())
        {
            if nReq > unsafe { (*p).nNodeSize } {
                // An unusual case: this is the first term to be added to the node
                // and the static node buffer (p->nNodeSize bytes) is not large
                // enough. Use a separately malloced buffer instead This wastes
                // p->nNodeSize bytes, but since this scenario only comes about when
                // the database contain two terms that share a prefix of almost 2KB,
                // this is not expected to be a serious problem.
                0 as i32;
                unsafe {
                    (*pTree).aData = (unsafe { sqlite3_malloc64((nReq as i64) as u64) }) as *mut i8;
                }
                if !((unsafe { (*pTree).aData }) != std::ptr::null_mut::<i8>()) {
                    return 7 as i32;
                }
            }
            if (unsafe { (*pTree).zTerm }) != std::ptr::null_mut::<i8>() {
                // There is no prefix-length field for first term in a node
                let __v1759: i32 = nData;
                let __v1760: i32 = __v1759
                    + unsafe {
                        sqlite3Fts3PutVarint(
                            unsafe { unsafe { (*pTree).aData }.offset(nData as isize) },
                            nPrefix as i64,
                        )
                    };
                nData = __v1760;
            }
            let __v1761: i32 = nData;
            let __v1762: i32 = __v1761
                + unsafe {
                    sqlite3Fts3PutVarint(
                        unsafe { unsafe { (*pTree).aData }.offset(nData as isize) },
                        nSuffix as i64,
                    )
                };
            nData = __v1762;
            unsafe {
                memcpy(
                    (unsafe { unsafe { (*pTree).aData }.offset(nData as isize) }) as *mut (),
                    (unsafe { zTerm.offset(nPrefix as isize) }) as *const (),
                    (nSuffix as i64) as u64,
                )
            };
            unsafe {
                (*pTree).nData = nData + nSuffix;
            }
            let __v1763: *mut SegmentNode = pTree;
            let __v1764: i32 = unsafe { (*__v1763).nEntry };
            let __v1765: i32 = __v1764 + (1 as i32);
            unsafe {
                (*__v1763).nEntry = __v1765;
            }
            if isCopyTerm != (0 as i32) {
                if (unsafe { (*pTree).nMalloc }) < nTerm {
                    let mut zNew: *mut i8 = (unsafe {
                        sqlite3_realloc64(
                            (unsafe { (*pTree).zMalloc }) as *mut (),
                            ((nTerm as i64) * ((2 as i32) as i64)) as u64,
                        )
                    }) as *mut i8;
                    if !(zNew != std::ptr::null_mut::<i8>()) {
                        return 7 as i32;
                    }
                    unsafe {
                        (*pTree).nMalloc = nTerm * (2 as i32);
                    }
                    unsafe {
                        (*pTree).zMalloc = zNew;
                    }
                }
                unsafe {
                    (*pTree).zTerm = unsafe { (*pTree).zMalloc };
                }
                unsafe {
                    memcpy(
                        (unsafe { (*pTree).zTerm }) as *mut (),
                        zTerm as *const (),
                        (nTerm as i64) as u64,
                    )
                };
                unsafe {
                    (*pTree).nTerm = nTerm;
                }
            } else {
                unsafe {
                    (*pTree).zTerm = zTerm as *mut i8;
                }
                unsafe {
                    (*pTree).nTerm = nTerm;
                }
            }
            return 0 as i32;
        }
    }
    // If control flows to here, it was not possible to append zTerm to the
    // current node. Create a new node (a right-sibling of the current node).
    // If this is the first node in the tree, the term is added to it.
    //
    // Otherwise, the term is not added to the new node, it is left empty for
    // now. Instead, the term is inserted into the parent of pTree. If pTree
    // has no parent, one is created here.
    pNew = (unsafe {
        sqlite3_malloc64((72 as u64).wrapping_add(((unsafe { (*p).nNodeSize }) as i64) as u64))
    }) as *mut SegmentNode;
    if !(pNew != std::ptr::null_mut::<SegmentNode>()) {
        return 7 as i32;
    }
    unsafe { memset(pNew as *mut (), 0 as i32, 72 as u64) };
    unsafe {
        (*pNew).nData = (1 as i32) + (10 as i32);
    }
    unsafe {
        (*pNew).aData = (unsafe { pNew.offset((1 as i32) as isize) }) as *mut i8;
    }
    if pTree != std::ptr::null_mut::<SegmentNode>() {
        let mut pParent: *mut SegmentNode = unsafe { (*pTree).pParent };
        rc = fts3NodeAddTerm(p, std::ptr::addr_of_mut!(pParent), isCopyTerm, zTerm, nTerm);
        if (unsafe { (*pTree).pParent }) == std::ptr::null_mut::<SegmentNode>() {
            unsafe {
                (*pTree).pParent = pParent;
            }
        }
        unsafe {
            (*pTree).pRight = pNew;
        }
        unsafe {
            (*pNew).pLeftmost = unsafe { (*pTree).pLeftmost };
        }
        unsafe {
            (*pNew).pParent = pParent;
        }
        unsafe {
            (*pNew).zMalloc = unsafe { (*pTree).zMalloc };
        }
        unsafe {
            (*pNew).nMalloc = unsafe { (*pTree).nMalloc };
        }
        unsafe {
            (*pTree).zMalloc = std::ptr::null_mut::<i8>();
        }
    } else {
        unsafe {
            (*pNew).pLeftmost = pNew;
        }
        rc = fts3NodeAddTerm(p, std::ptr::addr_of_mut!(pNew), isCopyTerm, zTerm, nTerm);
    }
    unsafe {
        *ppTree = pNew;
    }
    return rc;
}

/// Helper function for fts3NodeWrite().
fn fts3TreeFinishNode(mut pTree: *mut SegmentNode, mut iHeight: i32, mut iLeftChild: i64) -> i32 {
    let mut nStart: i32 = 0 as i32;
    0 as i32;
    nStart = (10 as i32) - unsafe { sqlite3Fts3VarintLen(iLeftChild as u64) };
    unsafe {
        *unsafe { unsafe { (*pTree).aData }.offset(nStart as isize) } = iHeight as i8;
    }
    unsafe {
        sqlite3Fts3PutVarint(
            unsafe { unsafe { (*pTree).aData }.offset((nStart + (1 as i32)) as isize) },
            iLeftChild,
        )
    };
    return nStart;
}

/// Write the buffer for the segment node pTree and all of its peers to the
/// database. Then call this function recursively to write the parent of
/// pTree and its peers to the database.
///
/// Except, if pTree is a root node, do not write it to the database. Instead,
/// set output variables *paRoot and *pnRoot to contain the root node.
///
/// If successful, SQLITE_OK is returned and output variable *piLast is
/// set to the largest blockid written to the database (or zero if no
/// blocks were written to the db). Otherwise, an SQLite error code is
/// returned.
///
/// # Arguments
///
/// * `p` - Virtual table handle
/// * `pTree` - SegmentNode handle
/// * `iHeight` - Height of this node in tree
/// * `iLeaf` - Block id of first leaf node
/// * `iFree` - Block id of next free slot in %_segments
/// * `piLast` - OUT: Block id of last entry written
/// * `paRoot` - OUT: Data for root node
/// * `pnRoot` - OUT: Size of root node in bytes
fn fts3NodeWrite(
    mut p: *mut Fts3Table,
    mut pTree: *mut SegmentNode,
    mut iHeight: i32,
    mut iLeaf: i64,
    mut iFree: i64,
    mut piLast: *mut i64,
    mut paRoot: *mut *mut i8,
    mut pnRoot: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    if !((unsafe { (*pTree).pParent }) != std::ptr::null_mut::<SegmentNode>()) {
        // Root node of the tree.
        let mut nStart: i32 = fts3TreeFinishNode(pTree, iHeight, iLeaf);
        unsafe {
            *piLast = iFree - ((1 as i32) as i64);
        }
        unsafe {
            *pnRoot = (unsafe { (*pTree).nData }) - nStart;
        }
        unsafe {
            *paRoot = unsafe { unsafe { (*pTree).aData }.offset(nStart as isize) };
        }
    } else {
        let mut pIter: *mut SegmentNode = unsafe { std::mem::zeroed() };
        let mut iNextFree: i64 = iFree;
        let mut iNextLeaf: i64 = iLeaf;
        pIter = unsafe { (*pTree).pLeftmost };
        '__slate_break_1534: while pIter != std::ptr::null_mut::<SegmentNode>() && rc == (0 as i32)
        {
            let mut nStart: i32 = fts3TreeFinishNode(pIter, iHeight, iNextLeaf);
            let mut nWrite: i32 = (unsafe { (*pIter).nData }) - nStart;
            rc = fts3WriteSegment(
                p,
                iNextFree,
                unsafe { unsafe { (*pIter).aData }.offset(nStart as isize) },
                nWrite,
            );
            let __v1766: i64 = iNextFree;
            let __v1767: i64 = __v1766 + ((1 as i32) as i64);
            iNextFree = __v1767;
            let __v1768: i64 = iNextLeaf;
            let __v1769: i64 = __v1768 + (((unsafe { (*pIter).nEntry }) + (1 as i32)) as i64);
            iNextLeaf = __v1769;
            pIter = unsafe { (*pIter).pRight };
        }
        if rc == (0 as i32) {
            0 as i32;
            rc = fts3NodeWrite(
                p,
                unsafe { (*pTree).pParent },
                iHeight + (1 as i32),
                iFree,
                iNextFree,
                piLast,
                paRoot,
                pnRoot,
            );
        }
    }
    return rc;
}

/// Free all memory allocations associated with the tree pTree.
fn fts3NodeFree(mut pTree: *mut SegmentNode) {
    if pTree != std::ptr::null_mut::<SegmentNode>() {
        let mut p: *mut SegmentNode = unsafe { (*pTree).pLeftmost };
        fts3NodeFree(unsafe { (*p).pParent });
        '__slate_break_1535: while p != std::ptr::null_mut::<SegmentNode>() {
            let mut pRight: *mut SegmentNode = unsafe { (*p).pRight };
            if (unsafe { (*p).aData }) != ((unsafe { p.offset((1 as i32) as isize) }) as *mut i8) {
                unsafe { sqlite3_free((unsafe { (*p).aData }) as *mut ()) };
            }
            0 as i32;
            unsafe { sqlite3_free((unsafe { (*p).zMalloc }) as *mut ()) };
            unsafe { sqlite3_free(p as *mut ()) };
            p = pRight;
        }
    }
}

/// Add a term to the segment being constructed by the SegmentWriter object
/// *ppWriter. When adding the first term to a segment, *ppWriter should
/// be passed NULL. This function will allocate a new SegmentWriter object
/// and return it via the input/output variable *ppWriter in this case.
///
/// If successful, SQLITE_OK is returned. Otherwise, an SQLite error code.
///
/// # Arguments
///
/// * `p` - Virtual table handle
/// * `ppWriter` - IN/OUT: SegmentWriter handle
/// * `isCopyTerm` - True if buffer zTerm must be copied
/// * `zTerm` - Pointer to buffer containing term
/// * `nTerm` - Size of term in bytes
/// * `aDoclist` - Pointer to buffer containing doclist
/// * `nDoclist` - Size of doclist in bytes
fn fts3SegWriterAdd(
    mut p: *mut Fts3Table,
    mut ppWriter: *mut *mut SegmentWriter,
    mut isCopyTerm: i32,
    mut zTerm: *const i8,
    mut nTerm: i32,
    mut aDoclist: *const i8,
    mut nDoclist: i32,
) -> i32 {
    let mut nPrefix: i32 = 0 as i32; // Size of term prefix in bytes
    let mut nSuffix: i32 = 0 as i32; // Size of term suffix in bytes
    let mut nReq: i64 = 0 as i64; // Number of bytes required on leaf page
    let mut nData: i32 = 0 as i32;
    let mut pWriter: *mut SegmentWriter = unsafe { *ppWriter };
    if !(pWriter != std::ptr::null_mut::<SegmentWriter>()) {
        let mut rc: i32 = 0 as i32;
        let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
        // Allocate the SegmentWriter structure
        pWriter = (unsafe { sqlite3_malloc64(72 as u64) }) as *mut SegmentWriter;
        if !(pWriter != std::ptr::null_mut::<SegmentWriter>()) {
            return 7 as i32;
        }
        unsafe { memset(pWriter as *mut (), 0 as i32, 72 as u64) };
        unsafe {
            *ppWriter = pWriter;
        }
        // Allocate a buffer in which to accumulate data
        unsafe {
            (*pWriter).aData =
                (unsafe { sqlite3_malloc64(((unsafe { (*p).nNodeSize }) as i64) as u64) })
                    as *mut i8;
        }
        if !((unsafe { (*pWriter).aData }) != std::ptr::null_mut::<i8>()) {
            return 7 as i32;
        }
        unsafe {
            (*pWriter).nSize = unsafe { (*p).nNodeSize };
        }
        // Find the next free blockid in the %_segments table
        rc = fts3SqlStmt(
            p,
            10 as i32,
            std::ptr::addr_of_mut!(pStmt),
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc != (0 as i32) {
            return rc;
        }
        if (100 as i32) == unsafe { sqlite3_step(pStmt) } {
            unsafe {
                (*pWriter).iFree = unsafe { sqlite3_column_int64(pStmt, 0 as i32) };
            }
            unsafe {
                (*pWriter).iFirst = unsafe { (*pWriter).iFree };
            }
        }
        rc = unsafe { sqlite3_reset(pStmt) };
        if rc != (0 as i32) {
            return rc;
        }
    }
    nData = unsafe { (*pWriter).nData };
    nPrefix = fts3PrefixCompress(
        (unsafe { (*pWriter).zTerm }) as *const i8,
        unsafe { (*pWriter).nTerm },
        zTerm,
        nTerm,
    );
    nSuffix = nTerm - nPrefix;
    // If nSuffix is zero or less, then zTerm/nTerm must be a prefix of
    // pWriter->zTerm/pWriter->nTerm. i.e. must be equal to or less than when
    // compared with BINARY collation. This indicates corruption.
    if nSuffix <= (0 as i32) {
        return (11 as i32) | (1 as i32) << (8 as i32);
    }
    // Figure out how many bytes are required by this new entry
    nReq = ((unsafe { sqlite3Fts3VarintLen((nPrefix as i64) as u64) })
        + unsafe { sqlite3Fts3VarintLen((nSuffix as i64) as u64) }
        + nSuffix
        + unsafe { sqlite3Fts3VarintLen((nDoclist as i64) as u64) }
        + nDoclist) as i64; // varint containing prefix size
    // varint containing suffix size
    // Term suffix
    // Size of doclist
    // Doclist data
    if nData > (0 as i32) && (nData as i64) + nReq > ((unsafe { (*p).nNodeSize }) as i64) {
        let mut rc: i32 = 0 as i32;
        // The current leaf node is full. Write it out to the database.
        if (unsafe { (*pWriter).iFree })
            == (((4294967295 as u32) as u64) as i64) | ((2147483647 as i32) as i64) << (32 as i32)
        {
            return (11 as i32) | (1 as i32) << (8 as i32);
        }
        let __v1770: *mut SegmentWriter = pWriter;
        let __v1771: i64 = unsafe { (*__v1770).iFree };
        let __v1772: i64 = __v1771 + ((1 as i32) as i64);
        unsafe {
            (*__v1770).iFree = __v1772;
        }
        rc = fts3WriteSegment(p, __v1771, unsafe { (*pWriter).aData }, nData);
        if rc != (0 as i32) {
            return rc;
        }
        let __v1773: *mut Fts3Table = p;
        let __v1774: u32 = unsafe { (*__v1773).nLeafAdd };
        let __v1775: u32 = __v1774.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v1773).nLeafAdd = __v1775;
        }
        // Add the current term to the interior node tree. The term added to
        // the interior tree must:
        //
        //   a) be greater than the largest term on the leaf node just written
        //      to the database (still available in pWriter->zTerm), and
        //
        //   b) be less than or equal to the term about to be added to the new
        //      leaf node (zTerm/nTerm).
        //
        // In other words, it must be the prefix of zTerm 1 byte longer than
        // the common prefix (if any) of zTerm and pWriter->zTerm.
        0 as i32;
        rc = fts3NodeAddTerm(
            p,
            unsafe { std::ptr::addr_of_mut!((*pWriter).pTree) },
            isCopyTerm,
            zTerm,
            nPrefix + (1 as i32),
        );
        if rc != (0 as i32) {
            return rc;
        }
        nData = 0 as i32;
        unsafe {
            (*pWriter).nTerm = 0 as i32;
        }
        nPrefix = 0 as i32;
        nSuffix = nTerm;
        nReq = ((1 as i32)
            + unsafe { sqlite3Fts3VarintLen((nTerm as i64) as u64) }
            + nTerm
            + unsafe { sqlite3Fts3VarintLen((nDoclist as i64) as u64) }
            + nDoclist) as i64; // varint containing prefix size
        // varint containing suffix size
        // Term suffix
        // Size of doclist
        // Doclist data
    }
    // Increase the total number of bytes written to account for the new entry.
    let __v1776: *mut SegmentWriter = pWriter;
    let __v1777: i64 = unsafe { (*__v1776).nLeafData };
    let __v1778: i64 = __v1777 + nReq;
    unsafe {
        (*__v1776).nLeafData = __v1778;
    }
    // If the buffer currently allocated is too small for this entry, realloc
    // the buffer to make it large enough.
    if nReq > ((unsafe { (*pWriter).nSize }) as i64) {
        let mut aNew: *mut i8 =
            (unsafe { sqlite3_realloc64((unsafe { (*pWriter).aData }) as *mut (), nReq as u64) })
                as *mut i8;
        if !(aNew != std::ptr::null_mut::<i8>()) {
            return 7 as i32;
        }
        unsafe {
            (*pWriter).aData = aNew;
        }
        unsafe {
            (*pWriter).nSize = nReq as i32;
        }
    }
    0 as i32;
    // Append the prefix-compressed term and doclist to the buffer.
    let __v1779: i32 = nData;
    let __v1780: i32 = __v1779
        + unsafe {
            sqlite3Fts3PutVarint(
                unsafe { unsafe { (*pWriter).aData }.offset(nData as isize) },
                nPrefix as i64,
            )
        };
    nData = __v1780;
    let __v1781: i32 = nData;
    let __v1782: i32 = __v1781
        + unsafe {
            sqlite3Fts3PutVarint(
                unsafe { unsafe { (*pWriter).aData }.offset(nData as isize) },
                nSuffix as i64,
            )
        };
    nData = __v1782;
    0 as i32;
    unsafe {
        memcpy(
            (unsafe { unsafe { (*pWriter).aData }.offset(nData as isize) }) as *mut (),
            (unsafe { zTerm.offset(nPrefix as isize) }) as *const (),
            (nSuffix as i64) as u64,
        )
    };
    let __v1783: i32 = nData;
    let __v1784: i32 = __v1783 + nSuffix;
    nData = __v1784;
    let __v1785: i32 = nData;
    let __v1786: i32 = __v1785
        + unsafe {
            sqlite3Fts3PutVarint(
                unsafe { unsafe { (*pWriter).aData }.offset(nData as isize) },
                nDoclist as i64,
            )
        };
    nData = __v1786;
    0 as i32;
    unsafe {
        memcpy(
            (unsafe { unsafe { (*pWriter).aData }.offset(nData as isize) }) as *mut (),
            aDoclist as *const (),
            (nDoclist as i64) as u64,
        )
    };
    unsafe {
        (*pWriter).nData = nData + nDoclist;
    }
    // Save the current term so that it can be used to prefix-compress the next.
    // If the isCopyTerm parameter is true, then the buffer pointed to by
    // zTerm is transient, so take a copy of the term data. Otherwise, just
    // store a copy of the pointer.
    if isCopyTerm != (0 as i32) {
        if nTerm > unsafe { (*pWriter).nMalloc } {
            let mut zNew: *mut i8 = (unsafe {
                sqlite3_realloc64(
                    (unsafe { (*pWriter).zMalloc }) as *mut (),
                    ((nTerm as i64) * ((2 as i32) as i64)) as u64,
                )
            }) as *mut i8;
            if !(zNew != std::ptr::null_mut::<i8>()) {
                return 7 as i32;
            }
            unsafe {
                (*pWriter).nMalloc = nTerm * (2 as i32);
            }
            unsafe {
                (*pWriter).zMalloc = zNew;
            }
            unsafe {
                (*pWriter).zTerm = zNew;
            }
        }
        0 as i32;
        0 as i32;
        unsafe {
            memcpy(
                (unsafe { (*pWriter).zTerm }) as *mut (),
                zTerm as *const (),
                (nTerm as i64) as u64,
            )
        };
    } else {
        unsafe {
            (*pWriter).zTerm = zTerm as *mut i8;
        }
    }
    unsafe {
        (*pWriter).nTerm = nTerm;
    }
    return 0 as i32;
}

/// Flush all data associated with the SegmentWriter object pWriter to the
/// database. This function must be called after all terms have been added
/// to the segment using fts3SegWriterAdd(). If successful, SQLITE_OK is
/// returned. Otherwise, an SQLite error code.
///
/// # Arguments
///
/// * `p` - Virtual table handle
/// * `pWriter` - SegmentWriter to flush to the db
/// * `iLevel` - Value for 'level' column of %_segdir
/// * `iIdx` - Value for 'idx' column of %_segdir
fn fts3SegWriterFlush(
    mut p: *mut Fts3Table,
    mut pWriter: *mut SegmentWriter,
    mut iLevel: i64,
    mut iIdx: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    if (unsafe { (*pWriter).pTree }) != std::ptr::null_mut::<SegmentNode>() {
        let mut iLast: i64 = (0 as i32) as i64; // Largest block id written to database
        let mut iLastLeaf: i64 = 0 as i64; // Largest leaf block id written to db
        let mut zRoot: *mut i8 = std::ptr::null_mut::<i8>(); // Pointer to buffer containing root node
        let mut nRoot: i32 = 0 as i32; // Size of buffer zRoot
        iLastLeaf = unsafe { (*pWriter).iFree };
        let __v1787: *mut SegmentWriter = pWriter;
        let __v1788: i64 = unsafe { (*__v1787).iFree };
        let __v1789: i64 = __v1788 + ((1 as i32) as i64);
        unsafe {
            (*__v1787).iFree = __v1789;
        }
        rc = fts3WriteSegment(p, __v1788, unsafe { (*pWriter).aData }, unsafe {
            (*pWriter).nData
        });
        if rc == (0 as i32) {
            rc = fts3NodeWrite(
                p,
                unsafe { (*pWriter).pTree },
                1 as i32,
                unsafe { (*pWriter).iFirst },
                unsafe { (*pWriter).iFree },
                std::ptr::addr_of_mut!(iLast),
                std::ptr::addr_of_mut!(zRoot),
                std::ptr::addr_of_mut!(nRoot),
            );
        }
        if rc == (0 as i32) {
            rc = fts3WriteSegdir(
                p,
                iLevel,
                iIdx,
                unsafe { (*pWriter).iFirst },
                iLastLeaf,
                iLast,
                unsafe { (*pWriter).nLeafData },
                zRoot,
                nRoot,
            );
        }
    } else {
        // The entire tree fits on the root node. Write it to the segdir table.
        rc = fts3WriteSegdir(
            p,
            iLevel,
            iIdx,
            (0 as i32) as i64,
            (0 as i32) as i64,
            (0 as i32) as i64,
            unsafe { (*pWriter).nLeafData },
            unsafe { (*pWriter).aData },
            unsafe { (*pWriter).nData },
        );
    }
    let __v1790: *mut Fts3Table = p;
    let __v1791: u32 = unsafe { (*__v1790).nLeafAdd };
    let __v1792: u32 = __v1791.wrapping_add((1 as i32) as u32);
    unsafe {
        (*__v1790).nLeafAdd = __v1792;
    }
    return rc;
}

/// Release all memory held by the SegmentWriter object passed as the
/// first argument.
fn fts3SegWriterFree(mut pWriter: *mut SegmentWriter) {
    if pWriter != std::ptr::null_mut::<SegmentWriter>() {
        unsafe { sqlite3_free((unsafe { (*pWriter).aData }) as *mut ()) };
        unsafe { sqlite3_free((unsafe { (*pWriter).zMalloc }) as *mut ()) };
        fts3NodeFree(unsafe { (*pWriter).pTree });
        unsafe { sqlite3_free(pWriter as *mut ()) };
    }
}

/// The first value in the apVal[] array is assumed to contain an integer.
/// This function tests if there exist any documents with docid values that
/// are different from that integer. i.e. if deleting the document with docid
/// pRowid would mean the FTS3 table were empty.
///
/// If successful, *pisEmpty is set to true if the table is empty except for
/// document pRowid, or false otherwise, and SQLITE_OK is returned. If an
/// error occurs, an SQLite error code is returned.
fn fts3IsEmpty(
    mut p: *mut Fts3Table,
    mut pRowid: *mut sqlite3_value,
    mut pisEmpty: *mut i32,
) -> i32 {
    let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*p).zContentTbl }) != std::ptr::null_mut::<i8>() {
        // If using the content=xxx option, assume the table is never empty
        unsafe {
            *pisEmpty = 0 as i32;
        }
        rc = 0 as i32;
    } else {
        rc = fts3SqlStmt(
            p,
            1 as i32,
            std::ptr::addr_of_mut!(pStmt),
            std::ptr::addr_of_mut!(pRowid),
        );
        if rc == (0 as i32) {
            if (100 as i32) == unsafe { sqlite3_step(pStmt) } {
                unsafe {
                    *pisEmpty = unsafe { sqlite3_column_int(pStmt, 0 as i32) };
                }
            }
            rc = unsafe { sqlite3_reset(pStmt) };
        }
    }
    return rc;
}

/// Set *pnMax to the largest segment level in the database for the index
/// iIndex.
///
/// Segment levels are stored in the 'level' column of the %_segdir table.
///
/// Return SQLITE_OK if successful, or an SQLite error code if not.
fn fts3SegmentMaxLevel(
    mut p: *mut Fts3Table,
    mut iLangid: i32,
    mut iIndex: i32,
    mut pnMax: *mut i64,
) -> i32 {
    let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    0 as i32;
    // Set pStmt to the compiled version of:
    //
    //   SELECT max(level) FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ?
    //
    // (1024 is actually the value of macro FTS3_SEGDIR_PREFIXLEVEL_STR).
    rc = fts3SqlStmt(
        p,
        15 as i32,
        std::ptr::addr_of_mut!(pStmt),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc != (0 as i32) {
        return rc;
    }
    unsafe {
        sqlite3_bind_int64(
            pStmt,
            1 as i32,
            getAbsoluteLevel(p, iLangid, iIndex, 0 as i32),
        )
    };
    unsafe {
        sqlite3_bind_int64(
            pStmt,
            2 as i32,
            getAbsoluteLevel(p, iLangid, iIndex, (1024 as i32) - (1 as i32)),
        )
    };
    if (100 as i32) == unsafe { sqlite3_step(pStmt) } {
        unsafe {
            *pnMax = unsafe { sqlite3_column_int64(pStmt, 0 as i32) };
        }
    }
    return unsafe { sqlite3_reset(pStmt) };
}

/// iAbsLevel is an absolute level that may be assumed to exist within
/// the database. This function checks if it is the largest level number
/// within its index. Assuming no error occurs, *pbMax is set to 1 if
/// iAbsLevel is indeed the largest level, or 0 otherwise, and SQLITE_OK
/// is returned. If an error occurs, an error code is returned and the
/// final value of *pbMax is undefined.
fn fts3SegmentIsMaxLevel(mut p: *mut Fts3Table, mut iAbsLevel: i64, mut pbMax: *mut i32) -> i32 {
    // Set pStmt to the compiled version of:
    //
    //   SELECT max(level) FROM %Q.'%q_segdir' WHERE level BETWEEN ? AND ?
    //
    // (1024 is actually the value of macro FTS3_SEGDIR_PREFIXLEVEL_STR).
    let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    let mut rc: i32 = fts3SqlStmt(
        p,
        15 as i32,
        std::ptr::addr_of_mut!(pStmt),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc != (0 as i32) {
        return rc;
    }
    unsafe { sqlite3_bind_int64(pStmt, 1 as i32, iAbsLevel + ((1 as i32) as i64)) };
    unsafe {
        sqlite3_bind_int64(
            pStmt,
            2 as i32,
            ((iAbsLevel as u64) / (((1024 as i32) as i64) as u64))
                .wrapping_add(((1 as i32) as i64) as u64)
                .wrapping_mul(((1024 as i32) as i64) as u64) as i64,
        )
    };
    unsafe {
        *pbMax = 0 as i32;
    }
    if (100 as i32) == unsafe { sqlite3_step(pStmt) } {
        unsafe {
            *pbMax = ((unsafe { sqlite3_column_type(pStmt, 0 as i32) }) == (5 as i32)) as i32;
        }
    }
    return unsafe { sqlite3_reset(pStmt) };
}

/// Delete all entries in the %_segments table associated with the segment
/// opened with seg-reader pSeg. This function does not affect the contents
/// of the %_segdir table.
///
/// # Arguments
///
/// * `p` - FTS table handle
/// * `pSeg` - Segment to delete
fn fts3DeleteSegment(mut p: *mut Fts3Table, mut pSeg: *mut Fts3SegReader) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    if (unsafe { (*pSeg).iStartBlock }) != (0 as i64) {
        let mut pDelete: *mut sqlite3_stmt = unsafe { std::mem::zeroed() }; // SQL statement to delete rows
        rc = fts3SqlStmt(
            p,
            17 as i32,
            std::ptr::addr_of_mut!(pDelete),
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == (0 as i32) {
            unsafe { sqlite3_bind_int64(pDelete, 1 as i32, unsafe { (*pSeg).iStartBlock }) };
            unsafe { sqlite3_bind_int64(pDelete, 2 as i32, unsafe { (*pSeg).iEndBlock }) };
            unsafe { sqlite3_step(pDelete) };
            rc = unsafe { sqlite3_reset(pDelete) };
        }
    }
    return rc;
}

/// This function is used after merging multiple segments into a single large
/// segment to delete the old, now redundant, segment b-trees. Specifically,
/// it:
///
///   1) Deletes all %_segments entries for the segments associated with
///      each of the SegReader objects in the array passed as the third
///      argument, and
///
///   2) deletes all %_segdir entries with level iLevel, or all %_segdir
///      entries regardless of level if (iLevel<0).
///
/// SQLITE_OK is returned if successful, otherwise an SQLite error code.
///
/// # Arguments
///
/// * `p` - Virtual table handle
/// * `iLangid` - Language id
/// * `iIndex` - Index for p->aIndex
/// * `iLevel` - Level of %_segdir entries to delete
/// * `apSegment` - Array of SegReader objects
/// * `nReader` - Size of array apSegment
fn fts3DeleteSegdir(
    mut p: *mut Fts3Table,
    mut iLangid: i32,
    mut iIndex: i32,
    mut iLevel: i32,
    mut apSegment: *mut *mut Fts3SegReader,
    mut nReader: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return Code
    let mut i: i32 = 0 as i32; // Iterator variable
    let mut pDelete: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // SQL statement to delete rows
    i = 0 as i32;
    '__slate_break_1536: loop {
        if !(rc == (0 as i32) && i < nReader) {
            break;
        }
        rc = fts3DeleteSegment(p, unsafe { *unsafe { apSegment.offset(i as isize) } });
        let __v1793: i32 = i;
        let __v1794: i32 = __v1793 + (1 as i32);
        i = __v1794;
    }
    if rc != (0 as i32) {
        return rc;
    }
    0 as i32;
    if iLevel == -(2 as i32) {
        rc = fts3SqlStmt(
            p,
            26 as i32,
            std::ptr::addr_of_mut!(pDelete),
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == (0 as i32) {
            unsafe {
                sqlite3_bind_int64(
                    pDelete,
                    1 as i32,
                    getAbsoluteLevel(p, iLangid, iIndex, 0 as i32),
                )
            };
            unsafe {
                sqlite3_bind_int64(
                    pDelete,
                    2 as i32,
                    getAbsoluteLevel(p, iLangid, iIndex, (1024 as i32) - (1 as i32)),
                )
            };
        }
    } else {
        rc = fts3SqlStmt(
            p,
            16 as i32,
            std::ptr::addr_of_mut!(pDelete),
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == (0 as i32) {
            unsafe {
                sqlite3_bind_int64(
                    pDelete,
                    1 as i32,
                    getAbsoluteLevel(p, iLangid, iIndex, iLevel),
                )
            };
        }
    }
    if rc == (0 as i32) {
        unsafe { sqlite3_step(pDelete) };
        rc = unsafe { sqlite3_reset(pDelete) };
    }
    return rc;
}

/// When this function is called, buffer *ppList (size *pnList bytes) contains
/// a position list that may (or may not) feature multiple columns. This
/// function adjusts the pointer *ppList and the length *pnList so that they
/// identify the subset of the position list that corresponds to column iCol.
///
/// If there are no entries in the input position list for column iCol, then
/// *pnList is set to zero before returning.
///
/// If parameter bZero is non-zero, then any part of the input list following
/// the end of the output list is zeroed before returning.
///
/// # Arguments
///
/// * `iCol` - Column to filter on
/// * `bZero` - Zero out anything following *ppList
/// * `ppList` - IN/OUT: Pointer to position list
/// * `pnList` - IN/OUT: Size of buffer *ppList in bytes
fn fts3ColumnFilter(mut iCol: i32, mut bZero: i32, mut ppList: *mut *mut i8, mut pnList: *mut i32) {
    let mut pList: *mut i8 = unsafe { *ppList };
    let mut nList: i32 = unsafe { *pnList };
    let mut pEnd: *mut i8 = unsafe { pList.offset(nList as isize) };
    let mut iCurrent: i32 = 0 as i32;
    let mut p: *mut i8 = pList;
    0 as i32;
    '__slate_break_1537: while (1 as i32) != (0 as i32) {
        let mut c: i8 = (0 as i32) as i8;
        '__slate_break_1538: while p < pEnd
            && ((c as i32) | ((unsafe { *p }) as i32)) & (254 as i32) != (0 as i32)
        {
            let __v1795: *mut i8 = p;
            let __v1796: *mut i8 = unsafe { __v1795.offset((1 as i32) as isize) };
            p = __v1796;
            c = (((unsafe { *__v1795 }) as i32) & (128 as i32)) as i8;
        }
        if iCol == iCurrent {
            nList = ((unsafe { p.offset_from(pList as *mut i8) }) as i64) as i32;
            break '__slate_break_1537;
        }
        let __v1797: i32 = nList;
        let __v1798: i32 = __v1797 - (((unsafe { p.offset_from(pList as *mut i8) }) as i64) as i32);
        nList = __v1798;
        pList = p;
        if nList <= (0 as i32) {
            break '__slate_break_1537;
        }
        p = unsafe { pList.offset((1 as i32) as isize) };
        let __v1799: *mut i8 = p;
        let __v1800: i32;
        if (((unsafe { *(p as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
            __v1800 =
                unsafe { sqlite3Fts3GetVarint32(p as *const i8, std::ptr::addr_of_mut!(iCurrent)) };
        } else {
            unsafe {
                *std::ptr::addr_of_mut!(iCurrent) = ((unsafe { *(p as *mut u8) }) as u32) as i32;
            }
            __v1800 = 1 as i32;
        }
        let __v1801: *mut i8 = unsafe { __v1799.offset(__v1800 as isize) };
        p = __v1801;
    }
    if bZero != (0 as i32)
        && ((unsafe { pEnd.offset_from((unsafe { pList.offset(nList as isize) }) as *mut i8) })
            as i64)
            > ((0 as i32) as i64)
    {
        unsafe {
            memset(
                (unsafe { pList.offset(nList as isize) }) as *mut (),
                0 as i32,
                ((unsafe { pEnd.offset_from((unsafe { pList.offset(nList as isize) }) as *mut i8) })
                    as i64) as u64,
            )
        };
    }
    unsafe {
        *ppList = pList;
    }
    unsafe {
        *pnList = nList;
    }
}

/// Cache data in the Fts3MultiSegReader.aBuffer[] buffer (overwriting any
/// existing data). Grow the buffer if required.
///
/// If successful, return SQLITE_OK. Otherwise, if an OOM error is encountered
/// trying to resize the buffer, return SQLITE_NOMEM.
///
/// # Arguments
///
/// * `pMsr` - Multi-segment-reader handle
fn fts3MsrBufferData(mut pMsr: *mut Fts3MultiSegReader, mut pList: *mut i8, mut nList: i64) -> i32 {
    if nList + (((10 as i32) * (2 as i32)) as i64) > unsafe { (*pMsr).nBuffer } {
        let mut pNew: *mut i8 = unsafe { std::mem::zeroed() };
        let mut nNew: i32 =
            (nList * ((2 as i32) as i64) + (((10 as i32) * (2 as i32)) as i64)) as i32;
        pNew = (unsafe {
            sqlite3_realloc64(
                (unsafe { (*pMsr).aBuffer }) as *mut (),
                (nNew as i64) as u64,
            )
        }) as *mut i8;
        if !(pNew != std::ptr::null_mut::<i8>()) {
            return 7 as i32;
        }
        unsafe {
            (*pMsr).aBuffer = pNew;
        }
        unsafe {
            (*pMsr).nBuffer = nNew as i64;
        }
    }
    0 as i32;
    unsafe {
        memcpy(
            (unsafe { (*pMsr).aBuffer }) as *mut (),
            pList as *const (),
            nList as u64,
        )
    };
    unsafe {
        memset(
            (unsafe { unsafe { (*pMsr).aBuffer }.offset(nList as isize) }) as *mut (),
            0 as i32,
            (((10 as i32) * (2 as i32)) as i64) as u64,
        )
    };
    return 0 as i32;
}

/// # Arguments
///
/// * `p` - Virtual table handle
/// * `pMsr` - Multi-segment-reader handle
/// * `piDocid` - OUT: Docid value
/// * `paPoslist` - OUT: Pointer to position list
/// * `pnPoslist` - OUT: Size of position list in bytes
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3MsrIncrNext(
    mut p: *mut Fts3Table,
    mut pMsr: *mut Fts3MultiSegReader,
    mut piDocid: *mut i64,
    mut paPoslist: *mut *mut i8,
    mut pnPoslist: *mut i32,
) -> i32 {
    let mut nMerge: i32 = unsafe { (*pMsr).nAdvance };
    let mut apSegment: *mut *mut Fts3SegReader = unsafe { (*pMsr).apSegment };
    let mut xCmp: Option<
        unsafe extern "C-unwind" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> i32,
    > = {
        let __t0: Option<
            unsafe extern "C-unwind" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> i32,
        > = if (unsafe { (*p).bDescIdx }) != (0 as u8) {
            Some(fts3SegReaderDoclistCmpRev)
        } else {
            Some(fts3SegReaderDoclistCmp)
        };
        __t0
    };
    if nMerge == (0 as i32) {
        unsafe {
            *paPoslist = std::ptr::null_mut::<i8>();
        }
        return 0 as i32;
    }
    '__slate_break_1539: while (1 as i32) != (0 as i32) {
        let mut pSeg: *mut Fts3SegReader = unsafe { std::mem::zeroed() };
        pSeg = unsafe { *unsafe { unsafe { (*pMsr).apSegment }.offset((0 as i32) as isize) } };
        if (unsafe { (*pSeg).pOffsetList }) == std::ptr::null_mut::<i8>() {
            unsafe {
                *paPoslist = std::ptr::null_mut::<i8>();
            }
            break '__slate_break_1539;
        } else {
            let mut rc: i32 = 0 as i32;
            let mut pList: *mut i8 = unsafe { std::mem::zeroed() };
            let mut nList: i32 = 0 as i32;
            let mut j: i32 = 0 as i32;
            let mut iDocid: i64 =
                unsafe { (*unsafe { *unsafe { apSegment.offset((0 as i32) as isize) } }).iDocid };
            rc = fts3SegReaderNextDocid(
                p,
                unsafe { *unsafe { apSegment.offset((0 as i32) as isize) } },
                std::ptr::addr_of_mut!(pList),
                std::ptr::addr_of_mut!(nList),
            );
            j = 1 as i32;
            '__slate_break_1540: while rc == (0 as i32)
                && j < nMerge
                && (unsafe { (*unsafe { *unsafe { apSegment.offset(j as isize) } }).pOffsetList })
                    != std::ptr::null_mut::<i8>()
                && (unsafe { (*unsafe { *unsafe { apSegment.offset(j as isize) } }).iDocid })
                    == iDocid
            {
                rc = fts3SegReaderNextDocid(
                    p,
                    unsafe { *unsafe { apSegment.offset(j as isize) } },
                    std::ptr::null_mut::<*mut i8>(),
                    std::ptr::null_mut::<i32>(),
                );
                let __v1664: i32 = j;
                let __v1665: i32 = __v1664 + (1 as i32);
                j = __v1665;
            }
            if rc != (0 as i32) {
                return rc;
            }
            fts3SegReaderSort(unsafe { (*pMsr).apSegment }, nMerge, j, xCmp);
            if nList > (0 as i32)
                && (unsafe {
                    (*unsafe { *unsafe { apSegment.offset((0 as i32) as isize) } }).ppNextElem
                }) != std::ptr::null_mut::<*mut Fts3HashElem>()
            {
                rc = fts3MsrBufferData(pMsr, pList, (nList as i64) + ((1 as i32) as i64));
                if rc != (0 as i32) {
                    return rc;
                }
                0 as i32;
                pList = unsafe { (*pMsr).aBuffer };
            }
            if (unsafe { (*pMsr).iColFilter }) >= (0 as i32) {
                fts3ColumnFilter(
                    unsafe { (*pMsr).iColFilter },
                    1 as i32,
                    std::ptr::addr_of_mut!(pList),
                    std::ptr::addr_of_mut!(nList),
                );
            }
            if nList > (0 as i32) {
                unsafe {
                    *paPoslist = pList;
                }
                unsafe {
                    *piDocid = iDocid;
                }
                unsafe {
                    *pnPoslist = nList;
                }
                break '__slate_break_1539;
            }
        }
    }
    return 0 as i32;
}

/// # Arguments
///
/// * `p` - Virtual table handle
/// * `pCsr` - Cursor object
/// * `zTerm` - Term searched for (or NULL)
/// * `nTerm` - Length of zTerm in bytes
fn fts3SegReaderStart(
    mut p: *mut Fts3Table,
    mut pCsr: *mut Fts3MultiSegReader,
    mut zTerm: *const i8,
    mut nTerm: i32,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut nSeg: i32 = unsafe { (*pCsr).nSegment };
    // If the Fts3SegFilter defines a specific term (or term prefix) to search
    // for, then advance each segment iterator until it points to a term of
    // equal or greater value than the specified term. This prevents many
    // unnecessary merge/sort operations for the case where single segment
    // b-tree leaf nodes contain more than one term.
    i = 0 as i32;
    '__slate_break_1541: loop {
        if !((unsafe { (*pCsr).bRestart }) == (0 as i32) && i < unsafe { (*pCsr).nSegment }) {
            break;
        }
        let mut res: i32 = 0 as i32;
        let mut pSeg: *mut Fts3SegReader =
            unsafe { *unsafe { unsafe { (*pCsr).apSegment }.offset(i as isize) } };
        '__slate_break_1542: loop {
            let mut rc: i32 = fts3SegReaderNext(p, pSeg, 0 as i32);
            if rc != (0 as i32) {
                return rc;
            }
            let __v1804: bool;
            if zTerm != std::ptr::null::<i8>() {
                let __v1805: i32 = fts3SegReaderTermCmp(pSeg, zTerm, nTerm);
                res = __v1805;
                __v1804 = __v1805 < (0 as i32);
            } else {
                __v1804 = false as bool;
            }
            if !__v1804 {
                break;
            }
        }
        if (unsafe { (*pSeg).bLookup }) != (0 as u8) && res != (0 as i32) {
            fts3SegReaderSetEof(pSeg);
        }
        let __v1802: i32 = i;
        let __v1803: i32 = __v1802 + (1 as i32);
        i = __v1803;
    }
    fts3SegReaderSort(
        unsafe { (*pCsr).apSegment },
        nSeg,
        nSeg,
        Some(fts3SegReaderCmp),
    );
    return 0 as i32;
}

/// # Arguments
///
/// * `p` - Virtual table handle
/// * `pCsr` - Cursor object
/// * `pFilter` - Restrictions on range of iteration
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3SegReaderStart(
    mut p: *mut Fts3Table,
    mut pCsr: *mut Fts3MultiSegReader,
    mut pFilter: *mut Fts3SegFilter,
) -> i32 {
    unsafe {
        (*pCsr).pFilter = pFilter;
    }
    return fts3SegReaderStart(p, pCsr, unsafe { (*pFilter).zTerm }, unsafe {
        (*pFilter).nTerm
    });
}

/// # Arguments
///
/// * `p` - Virtual table handle
/// * `pCsr` - Cursor object
/// * `iCol` - Column to match on.
/// * `zTerm` - Term to iterate through a doclist for
/// * `nTerm` - Number of bytes in zTerm
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3MsrIncrStart(
    mut p: *mut Fts3Table,
    mut pCsr: *mut Fts3MultiSegReader,
    mut iCol: i32,
    mut zTerm: *const i8,
    mut nTerm: i32,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut nSegment: i32 = unsafe { (*pCsr).nSegment };
    let mut xCmp: Option<
        unsafe extern "C-unwind" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> i32,
    > = {
        let __t0: Option<
            unsafe extern "C-unwind" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> i32,
        > = if (unsafe { (*p).bDescIdx }) != (0 as u8) {
            Some(fts3SegReaderDoclistCmpRev)
        } else {
            Some(fts3SegReaderDoclistCmp)
        };
        __t0
    };
    0 as i32;
    0 as i32;
    // Advance each segment iterator until it points to the term zTerm/nTerm.
    rc = fts3SegReaderStart(p, pCsr, zTerm, nTerm);
    if rc != (0 as i32) {
        return rc;
    }
    // Determine how many of the segments actually point to zTerm/nTerm.
    i = 0 as i32;
    '__slate_break_1543: loop {
        if !(i < nSegment) {
            break;
        }
        let mut pSeg: *mut Fts3SegReader =
            unsafe { *unsafe { unsafe { (*pCsr).apSegment }.offset(i as isize) } };
        let __v1661: bool;
        if !((unsafe { (*pSeg).aNode }) != std::ptr::null_mut::<i8>()) {
            __v1661 = true as bool;
        } else {
            __v1661 = fts3SegReaderTermCmp(pSeg, zTerm, nTerm) != (0 as i32);
        }
        if __v1661 {
            break '__slate_break_1543;
        }
        let __v1659: i32 = i;
        let __v1660: i32 = __v1659 + (1 as i32);
        i = __v1660;
    }
    unsafe {
        (*pCsr).nAdvance = i;
    }
    // Advance each of the segments to point to the first docid.
    i = 0 as i32;
    '__slate_break_1544: loop {
        if !(i < unsafe { (*pCsr).nAdvance }) {
            break;
        }
        rc = fts3SegReaderFirstDocid(p, unsafe {
            *unsafe { unsafe { (*pCsr).apSegment }.offset(i as isize) }
        });
        if rc != (0 as i32) {
            return rc;
        }
        let __v1662: i32 = i;
        let __v1663: i32 = __v1662 + (1 as i32);
        i = __v1663;
    }
    fts3SegReaderSort(unsafe { (*pCsr).apSegment }, i, i, xCmp);
    0 as i32;
    unsafe {
        (*pCsr).iColFilter = iCol;
    }
    return 0 as i32;
}

/// This function is called on a MultiSegReader that has been started using
/// sqlite3Fts3MsrIncrStart(). One or more calls to MsrIncrNext() may also
/// have been made. Calling this function puts the MultiSegReader in such
/// a state that if the next two calls are:
///
///   sqlite3Fts3SegReaderStart()
///   sqlite3Fts3SegReaderStep()
///
/// then the entire doclist for the term is available in
/// MultiSegReader.aDoclist/nDoclist.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3MsrIncrRestart(mut pCsr: *mut Fts3MultiSegReader) -> i32 {
    let mut i: i32 = 0 as i32; // Used to iterate through segment-readers
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe {
        (*pCsr).nAdvance = 0 as i32;
    }
    unsafe {
        (*pCsr).bRestart = 1 as i32;
    }
    i = 0 as i32;
    '__slate_break_1545: loop {
        if !(i < unsafe { (*pCsr).nSegment }) {
            break;
        }
        unsafe {
            (*unsafe { *unsafe { unsafe { (*pCsr).apSegment }.offset(i as isize) } }).pOffsetList =
                std::ptr::null_mut::<i8>();
        }
        unsafe {
            (*unsafe { *unsafe { unsafe { (*pCsr).apSegment }.offset(i as isize) } }).nOffsetList =
                0 as i32;
        }
        unsafe {
            (*unsafe { *unsafe { unsafe { (*pCsr).apSegment }.offset(i as isize) } }).iDocid =
                (0 as i32) as i64;
        }
        let __v1672: i32 = i;
        let __v1673: i32 = __v1672 + (1 as i32);
        i = __v1673;
    }
    return 0 as i32;
}

fn fts3GrowSegReaderBuffer(mut pCsr: *mut Fts3MultiSegReader, mut nReq: i64) -> i32 {
    if nReq > unsafe { (*pCsr).nBuffer } {
        let mut aNew: *mut i8 = unsafe { std::mem::zeroed() };
        unsafe {
            (*pCsr).nBuffer = nReq * ((2 as i32) as i64);
        }
        aNew = (unsafe {
            sqlite3_realloc64(
                (unsafe { (*pCsr).aBuffer }) as *mut (),
                (unsafe { (*pCsr).nBuffer }) as u64,
            )
        }) as *mut i8;
        if !(aNew != std::ptr::null_mut::<i8>()) {
            return 7 as i32;
        }
        unsafe {
            (*pCsr).aBuffer = aNew;
        }
    }
    return 0 as i32;
}

/// # Arguments
///
/// * `p` - Virtual table handle
/// * `pCsr` - Cursor object
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3SegReaderStep(
    mut p: *mut Fts3Table,
    mut pCsr: *mut Fts3MultiSegReader,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut isIgnoreEmpty: i32 = (unsafe { (*unsafe { (*pCsr).pFilter }).flags }) & (2 as i32);
    let mut isRequirePos: i32 = (unsafe { (*unsafe { (*pCsr).pFilter }).flags }) & (1 as i32);
    let mut isColFilter: i32 = (unsafe { (*unsafe { (*pCsr).pFilter }).flags }) & (4 as i32);
    let mut isPrefix: i32 = (unsafe { (*unsafe { (*pCsr).pFilter }).flags }) & (8 as i32);
    let mut isScan: i32 = (unsafe { (*unsafe { (*pCsr).pFilter }).flags }) & (16 as i32);
    let mut isFirst: i32 = (unsafe { (*unsafe { (*pCsr).pFilter }).flags }) & (32 as i32);
    let mut apSegment: *mut *mut Fts3SegReader = unsafe { (*pCsr).apSegment };
    let mut nSegment: i32 = unsafe { (*pCsr).nSegment };
    let mut pFilter: *mut Fts3SegFilter = unsafe { (*pCsr).pFilter };
    let mut xCmp: Option<
        unsafe extern "C-unwind" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> i32,
    > = {
        let __t0: Option<
            unsafe extern "C-unwind" fn(*mut Fts3SegReader, *mut Fts3SegReader) -> i32,
        > = if (unsafe { (*p).bDescIdx }) != (0 as u8) {
            Some(fts3SegReaderDoclistCmpRev)
        } else {
            Some(fts3SegReaderDoclistCmp)
        };
        __t0
    };
    if (unsafe { (*pCsr).nSegment }) == (0 as i32) {
        return 0 as i32;
    }
    '__slate_break_1546: loop {
        let mut nMerge: i32 = 0 as i32;
        let mut i: i32 = 0 as i32;
        // Advance the first pCsr->nAdvance entries in the apSegment[] array
        // forward. Then sort the list in order of current term again.
        i = 0 as i32;
        '__slate_break_1547: loop {
            if !(i < unsafe { (*pCsr).nAdvance }) {
                break;
            }
            let mut pSeg: *mut Fts3SegReader = unsafe { *unsafe { apSegment.offset(i as isize) } };
            if (unsafe { (*pSeg).bLookup }) != (0 as u8) {
                fts3SegReaderSetEof(pSeg);
            } else {
                rc = fts3SegReaderNext(p, pSeg, 0 as i32);
            }
            if rc != (0 as i32) {
                return rc;
            }
            let __v1634: i32 = i;
            let __v1635: i32 = __v1634 + (1 as i32);
            i = __v1635;
        }
        fts3SegReaderSort(
            apSegment,
            nSegment,
            unsafe { (*pCsr).nAdvance },
            Some(fts3SegReaderCmp),
        );
        unsafe {
            (*pCsr).nAdvance = 0 as i32;
        }
        // If all the seg-readers are at EOF, we're finished. return SQLITE_OK.
        0 as i32;
        if (unsafe { (*unsafe { *unsafe { apSegment.offset((0 as i32) as isize) } }).aNode })
            == std::ptr::null_mut::<i8>()
        {
            break '__slate_break_1546;
        }
        unsafe {
            (*pCsr).nTerm =
                unsafe { (*unsafe { *unsafe { apSegment.offset((0 as i32) as isize) } }).nTerm };
        }
        unsafe {
            (*pCsr).zTerm =
                unsafe { (*unsafe { *unsafe { apSegment.offset((0 as i32) as isize) } }).zTerm };
        }
        // If this is a prefix-search, and if the term that apSegment[0] points
        // to does not share a suffix with pFilter->zTerm/nTerm, then all
        // required callbacks have been made. In this case exit early.
        //
        // Similarly, if this is a search for an exact match, and the first term
        // of segment apSegment[0] is not a match, exit early.
        if (unsafe { (*pFilter).zTerm }) != std::ptr::null::<i8>() && !(isScan != (0 as i32)) {
            if (unsafe { (*pCsr).nTerm }) < unsafe { (*pFilter).nTerm }
                || !(isPrefix != (0 as i32))
                    && (unsafe { (*pCsr).nTerm }) > unsafe { (*pFilter).nTerm }
                || (unsafe {
                    memcmp(
                        (unsafe { (*pCsr).zTerm }) as *const (),
                        (unsafe { (*pFilter).zTerm }) as *const (),
                        ((unsafe { (*pFilter).nTerm }) as i64) as u64,
                    )
                }) != (0 as i32)
            {
                break '__slate_break_1546;
            }
        }
        nMerge = 1 as i32;
        '__slate_break_1548: while nMerge < nSegment
            && (unsafe { (*unsafe { *unsafe { apSegment.offset(nMerge as isize) } }).aNode })
                != std::ptr::null_mut::<i8>()
            && (unsafe { (*unsafe { *unsafe { apSegment.offset(nMerge as isize) } }).nTerm })
                == unsafe { (*pCsr).nTerm }
            && (0 as i32)
                == unsafe {
                    memcmp(
                        (unsafe { (*pCsr).zTerm }) as *const (),
                        (unsafe {
                            (*unsafe { *unsafe { apSegment.offset(nMerge as isize) } }).zTerm
                        }) as *const (),
                        ((unsafe { (*pCsr).nTerm }) as i64) as u64,
                    )
                }
        {
            let __v1636: i32 = nMerge;
            let __v1637: i32 = __v1636 + (1 as i32);
            nMerge = __v1637;
        }
        0 as i32;
        if nMerge == (1 as i32)
            && !(isIgnoreEmpty != (0 as i32))
            && !(isFirst != (0 as i32))
            && ((((unsafe { (*p).bDescIdx }) as u32) as i32) == (0 as i32)
                || (((unsafe {
                    (*unsafe { *unsafe { apSegment.offset((0 as i32) as isize) } }).ppNextElem
                }) != std::ptr::null_mut::<*mut Fts3HashElem>()) as i32)
                    == (0 as i32))
        {
            unsafe {
                (*pCsr).nDoclist = unsafe {
                    (*unsafe { *unsafe { apSegment.offset((0 as i32) as isize) } }).nDoclist
                };
            }
            if (unsafe {
                (*unsafe { *unsafe { apSegment.offset((0 as i32) as isize) } }).ppNextElem
            }) != std::ptr::null_mut::<*mut Fts3HashElem>()
            {
                rc = fts3MsrBufferData(
                    pCsr,
                    unsafe {
                        (*unsafe { *unsafe { apSegment.offset((0 as i32) as isize) } }).aDoclist
                    },
                    (unsafe { (*pCsr).nDoclist }) as i64,
                );
                unsafe {
                    (*pCsr).aDoclist = unsafe { (*pCsr).aBuffer };
                }
            } else {
                unsafe {
                    (*pCsr).aDoclist = unsafe {
                        (*unsafe { *unsafe { apSegment.offset((0 as i32) as isize) } }).aDoclist
                    };
                }
            }
            if rc == (0 as i32) {
                rc = 100 as i32;
            }
        } else {
            let mut nDoclist: i32 = 0 as i32; // Size of doclist
            let mut iPrev: i64 = (0 as i32) as i64; // Previous docid stored in doclist
            // The current term of the first nMerge entries in the array
            // of Fts3SegReader objects is the same. The doclists must be merged
            // and a single term returned with the merged doclist.
            i = 0 as i32;
            '__slate_break_1549: loop {
                if !(i < nMerge) {
                    break;
                }
                fts3SegReaderFirstDocid(p, unsafe { *unsafe { apSegment.offset(i as isize) } });
                let __v1638: i32 = i;
                let __v1639: i32 = __v1638 + (1 as i32);
                i = __v1639;
            }
            fts3SegReaderSort(apSegment, nMerge, nMerge, xCmp);
            '__slate_break_1550: while (unsafe {
                (*unsafe { *unsafe { apSegment.offset((0 as i32) as isize) } }).pOffsetList
            }) != std::ptr::null_mut::<i8>()
            {
                let mut j: i32 = 0 as i32; // Number of segments that share a docid
                let mut pList: *mut i8 = std::ptr::null_mut::<i8>();
                let mut nList: i32 = 0 as i32;
                let mut nByte: i32 = 0 as i32;
                let mut iDocid: i64 = unsafe {
                    (*unsafe { *unsafe { apSegment.offset((0 as i32) as isize) } }).iDocid
                };
                fts3SegReaderNextDocid(
                    p,
                    unsafe { *unsafe { apSegment.offset((0 as i32) as isize) } },
                    std::ptr::addr_of_mut!(pList),
                    std::ptr::addr_of_mut!(nList),
                );
                j = 1 as i32;
                '__slate_break_1551: while j < nMerge
                    && (unsafe {
                        (*unsafe { *unsafe { apSegment.offset(j as isize) } }).pOffsetList
                    }) != std::ptr::null_mut::<i8>()
                    && (unsafe { (*unsafe { *unsafe { apSegment.offset(j as isize) } }).iDocid })
                        == iDocid
                {
                    fts3SegReaderNextDocid(
                        p,
                        unsafe { *unsafe { apSegment.offset(j as isize) } },
                        std::ptr::null_mut::<*mut i8>(),
                        std::ptr::null_mut::<i32>(),
                    );
                    let __v1640: i32 = j;
                    let __v1641: i32 = __v1640 + (1 as i32);
                    j = __v1641;
                }
                if isColFilter != (0 as i32) {
                    fts3ColumnFilter(
                        unsafe { (*pFilter).iCol },
                        0 as i32,
                        std::ptr::addr_of_mut!(pList),
                        std::ptr::addr_of_mut!(nList),
                    );
                }
                if !(isIgnoreEmpty != (0 as i32)) || nList > (0 as i32) {
                    // Calculate the 'docid' delta value to write into the merged
                    // doclist.
                    let mut iDelta: i64 = 0 as i64;
                    if (unsafe { (*p).bDescIdx }) != (0 as u8) && nDoclist > (0 as i32) {
                        if iPrev <= iDocid {
                            return (11 as i32) | (1 as i32) << (8 as i32);
                        }
                        iDelta = (iPrev as u64).wrapping_sub(iDocid as u64) as i64;
                    } else {
                        if nDoclist > (0 as i32) && iPrev >= iDocid {
                            return (11 as i32) | (1 as i32) << (8 as i32);
                        }
                        iDelta = (iDocid as u64).wrapping_sub(iPrev as u64) as i64;
                    }
                    nByte = (unsafe { sqlite3Fts3VarintLen(iDelta as u64) })
                        + if isRequirePos != (0 as i32) {
                            nList + (1 as i32)
                        } else {
                            0 as i32
                        };
                    rc = fts3GrowSegReaderBuffer(
                        pCsr,
                        (nByte as i64) + (nDoclist as i64) + (((10 as i32) * (2 as i32)) as i64),
                    );
                    if rc != (0 as i32) {
                        return rc;
                    }
                    if isFirst != (0 as i32) {
                        let mut a: *mut i8 =
                            unsafe { unsafe { (*pCsr).aBuffer }.offset(nDoclist as isize) };
                        let mut nWrite: i32 = 0 as i32;
                        nWrite = unsafe { sqlite3Fts3FirstFilter(iDelta, pList, nList, a) };
                        if nWrite != (0 as i32) {
                            iPrev = iDocid;
                            let __v1642: i32 = nDoclist;
                            let __v1643: i32 = __v1642 + nWrite;
                            nDoclist = __v1643;
                        }
                    } else {
                        let __v1644: i32 = nDoclist;
                        let __v1645: i32 = __v1644
                            + unsafe {
                                sqlite3Fts3PutVarint(
                                    unsafe { unsafe { (*pCsr).aBuffer }.offset(nDoclist as isize) },
                                    iDelta,
                                )
                            };
                        nDoclist = __v1645;
                        iPrev = iDocid;
                        if isRequirePos != (0 as i32) {
                            unsafe {
                                memcpy(
                                    (unsafe {
                                        unsafe { (*pCsr).aBuffer }.offset(nDoclist as isize)
                                    }) as *mut (),
                                    pList as *const (),
                                    (nList as i64) as u64,
                                )
                            };
                            let __v1646: i32 = nDoclist;
                            let __v1647: i32 = __v1646 + nList;
                            nDoclist = __v1647;
                            let __v1648: i32 = nDoclist;
                            let __v1649: i32 = __v1648 + (1 as i32);
                            nDoclist = __v1649;
                            unsafe {
                                *unsafe { unsafe { (*pCsr).aBuffer }.offset(__v1648 as isize) } =
                                    (0 as i32) as i8;
                            }
                        }
                    }
                }
                fts3SegReaderSort(apSegment, nMerge, j, xCmp);
            }
            if nDoclist > (0 as i32) {
                rc = fts3GrowSegReaderBuffer(
                    pCsr,
                    (nDoclist as i64) + (((10 as i32) * (2 as i32)) as i64),
                );
                if rc != (0 as i32) {
                    return rc;
                }
                unsafe {
                    memset(
                        (unsafe { unsafe { (*pCsr).aBuffer }.offset(nDoclist as isize) })
                            as *mut (),
                        0 as i32,
                        (((10 as i32) * (2 as i32)) as i64) as u64,
                    )
                };
                unsafe {
                    (*pCsr).aDoclist = unsafe { (*pCsr).aBuffer };
                }
                unsafe {
                    (*pCsr).nDoclist = nDoclist;
                }
                rc = 100 as i32;
            }
        }
        unsafe {
            (*pCsr).nAdvance = nMerge;
        }
        if !(rc == (0 as i32)) {
            break;
        }
    }
    return rc;
}

/// # Arguments
///
/// * `pCsr` - Cursor object
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3SegReaderFinish(mut pCsr: *mut Fts3MultiSegReader) {
    if pCsr != std::ptr::null_mut::<Fts3MultiSegReader>() {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_1552: loop {
            if !(i < unsafe { (*pCsr).nSegment }) {
                break;
            }
            sqlite3Fts3SegReaderFree(unsafe {
                *unsafe { unsafe { (*pCsr).apSegment }.offset(i as isize) }
            });
            let __v1650: i32 = i;
            let __v1651: i32 = __v1650 + (1 as i32);
            i = __v1651;
        }
        unsafe { sqlite3_free((unsafe { (*pCsr).apSegment }) as *mut ()) };
        unsafe { sqlite3_free((unsafe { (*pCsr).aBuffer }) as *mut ()) };
        unsafe {
            (*pCsr).nSegment = 0 as i32;
        }
        unsafe {
            (*pCsr).apSegment = std::ptr::null_mut::<*mut Fts3SegReader>();
        }
        unsafe {
            (*pCsr).aBuffer = std::ptr::null_mut::<i8>();
        }
    }
}

/// Decode the "end_block" field, selected by column iCol of the SELECT
/// statement passed as the first argument.
///
/// The "end_block" field may contain either an integer, or a text field
/// containing the text representation of two non-negative integers separated
/// by one or more space (0x20) characters. In the first case, set *piEndBlock
/// to the integer value and *pnByte to zero before returning. In the second,
/// set *piEndBlock to the first value and *pnByte to the second.
fn fts3ReadEndBlockField(
    mut pStmt: *mut sqlite3_stmt,
    mut iCol: i32,
    mut piEndBlock: *mut i64,
    mut pnByte: *mut i64,
) {
    let mut zText: *const u8 = unsafe { sqlite3_column_text(pStmt, iCol) };
    if zText != std::ptr::null::<u8>() {
        let mut i: i32 = 0 as i32;
        let mut iMul: i32 = 1 as i32;
        let mut iVal: u64 = ((0 as i32) as i64) as u64;
        i = 0 as i32;
        '__slate_break_1553: loop {
            if !((((unsafe { *unsafe { zText.offset(i as isize) } }) as u32) as i32) >= (48 as i32)
                && (((unsafe { *unsafe { zText.offset(i as isize) } }) as u32) as i32)
                    <= (57 as i32))
            {
                break;
            }
            iVal = iVal.wrapping_mul(((10 as i32) as i64) as u64).wrapping_add(
                (((((unsafe { *unsafe { zText.offset(i as isize) } }) as u32) as i32) - (48 as i32))
                    as i64) as u64,
            );
            let __v1806: i32 = i;
            let __v1807: i32 = __v1806 + (1 as i32);
            i = __v1807;
        }
        unsafe {
            *piEndBlock = iVal as i64;
        }
        '__slate_break_1554: while (((unsafe { *unsafe { zText.offset(i as isize) } }) as u32)
            as i32)
            == (32 as i32)
        {
            let __v1808: i32 = i;
            let __v1809: i32 = __v1808 + (1 as i32);
            i = __v1809;
        }
        iVal = ((0 as i32) as i64) as u64;
        if (((unsafe { *unsafe { zText.offset(i as isize) } }) as u32) as i32) == (45 as i32) {
            let __v1810: i32 = i;
            let __v1811: i32 = __v1810 + (1 as i32);
            i = __v1811;
            iMul = -(1 as i32);
        }
        '__slate_break_1555: loop {
            if !((((unsafe { *unsafe { zText.offset(i as isize) } }) as u32) as i32) >= (48 as i32)
                && (((unsafe { *unsafe { zText.offset(i as isize) } }) as u32) as i32)
                    <= (57 as i32))
            {
                break;
            }
            // no-op
            iVal = iVal.wrapping_mul(((10 as i32) as i64) as u64).wrapping_add(
                (((((unsafe { *unsafe { zText.offset(i as isize) } }) as u32) as i32) - (48 as i32))
                    as i64) as u64,
            );
            let __v1812: i32 = i;
            let __v1813: i32 = __v1812 + (1 as i32);
            i = __v1813;
        }
        // This if() clause is just to avoid an integer overflow. The record is
        // corrupt in this case.
        if (iVal as i64)
            == (-(1 as i32) as i64)
                - ((((4294967295 as u32) as u64) as i64)
                    | ((2147483647 as i32) as i64) << (32 as i32))
        {
            iMul = 1 as i32;
        }
        unsafe {
            *pnByte = (iVal as i64) * (iMul as i64);
        }
    }
}

/// A segment of size nByte bytes has just been written to absolute level
/// iAbsLevel. Promote any segments that should be promoted as a result.
///
/// # Arguments
///
/// * `p` - FTS table handle
/// * `iAbsLevel` - Absolute level just updated
/// * `nByte` - Size of new segment at iAbsLevel
fn fts3PromoteSegments(mut p: *mut Fts3Table, mut iAbsLevel: i64, mut nByte: i64) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pRange: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    rc = fts3SqlStmt(
        p,
        37 as i32,
        std::ptr::addr_of_mut!(pRange),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        let mut bOk: i32 = 0 as i32;
        let mut iLast: i64 = (iAbsLevel / ((1024 as i32) as i64) + ((1 as i32) as i64))
            * ((1024 as i32) as i64)
            - ((1 as i32) as i64);
        let mut nLimit: i64 = nByte * ((3 as i32) as i64) / ((2 as i32) as i64);
        // Loop through all entries in the %_segdir table corresponding to
        // segments in this index on levels greater than iAbsLevel. If there is
        // at least one such segment, and it is possible to determine that all
        // such segments are smaller than nLimit bytes in size, they will be
        // promoted to level iAbsLevel.
        unsafe { sqlite3_bind_int64(pRange, 1 as i32, iAbsLevel + ((1 as i32) as i64)) };
        unsafe { sqlite3_bind_int64(pRange, 2 as i32, iLast) };
        '__slate_break_1556: while (100 as i32) == unsafe { sqlite3_step(pRange) } {
            let mut nSize: i64 = (0 as i32) as i64;
            let mut dummy: i64 = 0 as i64;
            fts3ReadEndBlockField(
                pRange,
                2 as i32,
                std::ptr::addr_of_mut!(dummy),
                std::ptr::addr_of_mut!(nSize),
            );
            if nSize <= ((0 as i32) as i64) || nSize > nLimit {
                // If nSize==0, then the %_segdir.end_block field does not not
                // contain a size value. This happens if it was written by an
                // old version of FTS. In this case it is not possible to determine
                // the size of the segment, and so segment promotion does not
                // take place.
                bOk = 0 as i32;
                break '__slate_break_1556;
            }
            bOk = 1 as i32;
        }
        rc = unsafe { sqlite3_reset(pRange) };
        if bOk != (0 as i32) {
            let mut iIdx: i32 = 0 as i32;
            let mut pUpdate1: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
            let mut pUpdate2: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
            if rc == (0 as i32) {
                rc = fts3SqlStmt(
                    p,
                    38 as i32,
                    std::ptr::addr_of_mut!(pUpdate1),
                    std::ptr::null_mut::<*mut sqlite3_value>(),
                );
            }
            if rc == (0 as i32) {
                rc = fts3SqlStmt(
                    p,
                    39 as i32,
                    std::ptr::addr_of_mut!(pUpdate2),
                    std::ptr::null_mut::<*mut sqlite3_value>(),
                );
            }
            if rc == (0 as i32) {
                // Loop through all %_segdir entries for segments in this index with
                // levels equal to or greater than iAbsLevel. As each entry is visited,
                // updated it to set (level = -1) and (idx = N), where N is 0 for the
                // oldest segment in the range, 1 for the next oldest, and so on.
                //
                // In other words, move all segments being promoted to level -1,
                // setting the "idx" fields as appropriate to keep them in the same
                // order. The contents of level -1 (which is never used, except
                // transiently here), will be moved back to level iAbsLevel below.
                unsafe { sqlite3_bind_int64(pRange, 1 as i32, iAbsLevel) };
                '__slate_break_1557: while (100 as i32) == unsafe { sqlite3_step(pRange) } {
                    let __v1814: i32 = iIdx;
                    let __v1815: i32 = __v1814 + (1 as i32);
                    iIdx = __v1815;
                    unsafe { sqlite3_bind_int(pUpdate1, 1 as i32, __v1814) };
                    unsafe {
                        sqlite3_bind_int(pUpdate1, 2 as i32, unsafe {
                            sqlite3_column_int(pRange, 0 as i32)
                        })
                    };
                    unsafe {
                        sqlite3_bind_int(pUpdate1, 3 as i32, unsafe {
                            sqlite3_column_int(pRange, 1 as i32)
                        })
                    };
                    unsafe { sqlite3_step(pUpdate1) };
                    rc = unsafe { sqlite3_reset(pUpdate1) };
                    if rc != (0 as i32) {
                        unsafe { sqlite3_reset(pRange) };
                        break '__slate_break_1557;
                    }
                }
            }
            if rc == (0 as i32) {
                rc = unsafe { sqlite3_reset(pRange) };
            }
            // Move level -1 to level iAbsLevel
            if rc == (0 as i32) {
                unsafe { sqlite3_bind_int64(pUpdate2, 1 as i32, iAbsLevel) };
                unsafe { sqlite3_step(pUpdate2) };
                rc = unsafe { sqlite3_reset(pUpdate2) };
            }
        }
    }
    return rc;
}

/// Merge all level iLevel segments in the database into a single
/// iLevel+1 segment. Or, if iLevel<0, merge all segments into a
/// single segment with a level equal to the numerically largest level
/// currently present in the database.
///
/// If this function is called with iLevel<0, but there is only one
/// segment in the database, SQLITE_DONE is returned immediately.
/// Otherwise, if successful, SQLITE_OK is returned. If an error occurs,
/// an SQLite error code is returned.
///
/// # Arguments
///
/// * `iLangid` - Language id to merge
/// * `iIndex` - Index in p->aIndex[] to merge
/// * `iLevel` - Level to merge
fn fts3SegmentMerge(
    mut p: *mut Fts3Table,
    mut iLangid: i32,
    mut iIndex: i32,
    mut iLevel: i32,
) -> i32 {
    let mut __slate_storage_1720: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1720: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1720) as *mut i32;
    let mut __slate_storage_1719: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1719: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1719) as *mut i32; // Max level number for this index/langid
    let mut __slate_storage_804: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_804: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_804) as *mut i64; // True to ignore empty segments
    let mut __slate_storage_803: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_803: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_803) as *mut i32; // Cursor to iterate through level(s)
    let mut __slate_storage_802: std::mem::MaybeUninit<Fts3MultiSegReader> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_802: *mut Fts3MultiSegReader =
        std::ptr::addr_of_mut!(__slate_storage_802) as *mut Fts3MultiSegReader; // Segment term filter condition
    let mut __slate_storage_801: std::mem::MaybeUninit<Fts3SegFilter> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_801: *mut Fts3SegFilter =
        std::ptr::addr_of_mut!(__slate_storage_801) as *mut Fts3SegFilter; // Used to write the new, merged, segment
    let mut __slate_storage_800: std::mem::MaybeUninit<*mut SegmentWriter> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_800: *mut *mut SegmentWriter =
        std::ptr::addr_of_mut!(__slate_storage_800) as *mut *mut SegmentWriter; // Level/index to create new segment at
    let mut __slate_storage_799: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_799: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_799) as *mut i64; // Index of new segment
    let mut __slate_storage_798: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_798: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_798) as *mut i32; // Return code
    let mut __slate_storage_797: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_797: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_797) as *mut i32;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_798, 0 as i32);
            std::ptr::write(__slate_slot_799, (0 as i32) as i64);
            std::ptr::write(__slate_slot_800, std::ptr::null_mut::<SegmentWriter>());
            std::ptr::write(__slate_slot_803, 0 as i32);
            std::ptr::write(__slate_slot_804, (0 as i32) as i64);
            0 as i32;
            0 as i32;
            0 as i32;
            *__slate_slot_797 = unsafe {
                sqlite3Fts3SegReaderCursor(
                    p,
                    iLangid,
                    iIndex,
                    iLevel,
                    std::ptr::null::<i8>(),
                    0 as i32,
                    1 as i32,
                    0 as i32,
                    std::ptr::addr_of_mut!(*__slate_slot_802),
                )
            };
            if *__slate_slot_797 != (0 as i32) || (*__slate_slot_802).nSegment == (0 as i32) {
            } else {
                if iLevel != -(1 as i32) {
                    *__slate_slot_797 = fts3SegmentMaxLevel(
                        p,
                        iLangid,
                        iIndex,
                        std::ptr::addr_of_mut!(*__slate_slot_804),
                    );
                    if *__slate_slot_797 != (0 as i32) {
                        break '__join_0;
                    }
                }
                if iLevel == -(2 as i32) {
                    // This call is to merge all segments in the database to a single
                    // segment. The level of the new segment is equal to the numerically
                    // greatest segment level currently present in the database for this
                    // index. The idx of the new segment is always 0.
                    if (*__slate_slot_802).nSegment == (1 as i32)
                        && (0 as i32)
                            == (((unsafe {
                                (*unsafe {
                                    *unsafe {
                                        (*__slate_slot_802).apSegment.offset((0 as i32) as isize)
                                    }
                                })
                                .ppNextElem
                            }) != std::ptr::null_mut::<*mut Fts3HashElem>())
                                as i32)
                    {
                        *__slate_slot_797 = 101 as i32;
                        break '__join_0;
                    } else {
                        *__slate_slot_799 = *__slate_slot_804;
                        *__slate_slot_803 = 1 as i32;
                    }
                } else {
                    // This call is to merge all segments at level iLevel. find the next
                    // available segment index at level iLevel+1. The call to
                    // fts3AllocateSegdirIdx() will merge the segments at level iLevel+1 to
                    // a single iLevel+2 segment if necessary.
                    0 as i32;
                    *__slate_slot_799 = getAbsoluteLevel(p, iLangid, iIndex, iLevel + (1 as i32));
                    *__slate_slot_797 = fts3AllocateSegdirIdx(
                        p,
                        iLangid,
                        iIndex,
                        iLevel + (1 as i32),
                        std::ptr::addr_of_mut!(*__slate_slot_798),
                    );
                    *__slate_slot_803 =
                        (iLevel != -(1 as i32) && *__slate_slot_799 > *__slate_slot_804) as i32;
                }
                if *__slate_slot_797 != (0 as i32) {
                } else {
                    0 as i32;
                    0 as i32;
                    0 as i32;
                    unsafe {
                        memset(
                            std::ptr::addr_of_mut!(*__slate_slot_801) as *mut (),
                            0 as i32,
                            24 as u64,
                        )
                    };
                    (*__slate_slot_801).flags = 1 as i32;
                    std::ptr::write(__slate_slot_1719, (*__slate_slot_801).flags);
                    std::ptr::write(
                        __slate_slot_1720,
                        *__slate_slot_1719
                            | if *__slate_slot_803 != (0 as i32) {
                                2 as i32
                            } else {
                                0 as i32
                            },
                    );
                    (*__slate_slot_801).flags = *__slate_slot_1720;
                    *__slate_slot_797 = sqlite3Fts3SegReaderStart(
                        p,
                        std::ptr::addr_of_mut!(*__slate_slot_802),
                        std::ptr::addr_of_mut!(*__slate_slot_801),
                    );
                    loop {
                        if (0 as i32) == *__slate_slot_797 {
                            *__slate_slot_797 = sqlite3Fts3SegReaderStep(
                                p,
                                std::ptr::addr_of_mut!(*__slate_slot_802),
                            );
                            if *__slate_slot_797 != (100 as i32) {
                                break;
                            } else {
                                *__slate_slot_797 = fts3SegWriterAdd(
                                    p,
                                    std::ptr::addr_of_mut!(*__slate_slot_800),
                                    1 as i32,
                                    (*__slate_slot_802).zTerm as *const i8,
                                    (*__slate_slot_802).nTerm,
                                    (*__slate_slot_802).aDoclist as *const i8,
                                    (*__slate_slot_802).nDoclist,
                                );
                            }
                        } else {
                            break;
                        }
                    }
                    if *__slate_slot_797 != (0 as i32) {
                    } else {
                        0 as i32;
                        if iLevel != -(1 as i32) {
                            *__slate_slot_797 = fts3DeleteSegdir(
                                p,
                                iLangid,
                                iIndex,
                                iLevel,
                                (*__slate_slot_802).apSegment,
                                (*__slate_slot_802).nSegment,
                            );
                            if *__slate_slot_797 != (0 as i32) {
                                break '__join_0;
                            }
                        }
                        if *__slate_slot_800 != std::ptr::null_mut::<SegmentWriter>() {
                            *__slate_slot_797 = fts3SegWriterFlush(
                                p,
                                *__slate_slot_800,
                                *__slate_slot_799,
                                *__slate_slot_798,
                            );
                            if *__slate_slot_797 == (0 as i32) {
                                if iLevel == -(1 as i32) || *__slate_slot_799 < *__slate_slot_804 {
                                    *__slate_slot_797 =
                                        fts3PromoteSegments(p, *__slate_slot_799, unsafe {
                                            (*(*__slate_slot_800)).nLeafData
                                        });
                                }
                            }
                        }
                    }
                }
            }
        }
        fts3SegWriterFree(*__slate_slot_800);
        sqlite3Fts3SegReaderFinish(std::ptr::addr_of_mut!(*__slate_slot_802));
        return *__slate_slot_797;
    }
    return unsafe { std::mem::zeroed() };
}

/// Flush the contents of pendingTerms to level 0 segments.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3PendingTermsFlush(mut p: *mut Fts3Table) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_1559: loop {
        if !(rc == (0 as i32) && i < unsafe { (*p).nIndex }) {
            break;
        }
        rc = fts3SegmentMerge(p, unsafe { (*p).iPrevLangid }, i, -(1 as i32));
        if rc == (101 as i32) {
            rc = 0 as i32;
        }
        let __v1623: i32 = i;
        let __v1624: i32 = __v1623 + (1 as i32);
        i = __v1624;
    }
    // Determine the auto-incr-merge setting if unknown.  If enabled,
    // estimate the number of leaf blocks of content to be written
    if rc == (0 as i32)
        && (unsafe { (*p).bHasStat }) != (0 as u8)
        && (unsafe { (*p).nAutoincrmerge }) == (255 as i32)
        && (unsafe { (*p).nLeafAdd }) > ((0 as i32) as u32)
    {
        let mut pStmt: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
        rc = fts3SqlStmt(
            p,
            22 as i32,
            std::ptr::addr_of_mut!(pStmt),
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == (0 as i32) {
            unsafe { sqlite3_bind_int(pStmt, 1 as i32, 2 as i32) };
            rc = unsafe { sqlite3_step(pStmt) };
            if rc == (100 as i32) {
                unsafe {
                    (*p).nAutoincrmerge = unsafe { sqlite3_column_int(pStmt, 0 as i32) };
                }
                if (unsafe { (*p).nAutoincrmerge }) == (1 as i32) {
                    unsafe {
                        (*p).nAutoincrmerge = 8 as i32;
                    }
                }
            } else {
                if rc == (101 as i32) {
                    unsafe {
                        (*p).nAutoincrmerge = 0 as i32;
                    }
                }
            }
            rc = unsafe { sqlite3_reset(pStmt) };
        }
    }
    if rc == (0 as i32) {
        sqlite3Fts3PendingTermsClear(p);
    }
    return rc;
}

/// Encode N integers as varints into a blob.
///
/// # Arguments
///
/// * `N` - The number of integers to encode
/// * `a` - The integer values
/// * `zBuf` - Write the BLOB here
/// * `pNBuf` - Write number of bytes if zBuf[] used here
fn fts3EncodeIntArray(mut N: i32, mut a: *mut u32, mut zBuf: *mut i8, mut pNBuf: *mut i32) {
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    j = 0 as i32;
    i = 0 as i32;
    '__slate_break_1560: loop {
        if !(i < N) {
            break;
        }
        let __v1818: i32 = j;
        let __v1819: i32 = __v1818
            + unsafe {
                sqlite3Fts3PutVarint(
                    unsafe { zBuf.offset(j as isize) },
                    ((unsafe { *unsafe { a.offset(i as isize) } }) as u64) as i64,
                )
            };
        j = __v1819;
        let __v1816: i32 = i;
        let __v1817: i32 = __v1816 + (1 as i32);
        i = __v1817;
    }
    unsafe {
        *pNBuf = j;
    }
}

/// Decode a blob of varints into N integers
///
/// # Arguments
///
/// * `N` - The number of integers to decode
/// * `a` - Write the integer values
/// * `zBuf` - The BLOB containing the varints
/// * `nBuf` - size of the BLOB
fn fts3DecodeIntArray(mut N: i32, mut a: *mut u32, mut zBuf: *const i8, mut nBuf: i32) {
    let mut i: i32 = 0 as i32;
    if nBuf != (0 as i32)
        && ((unsafe { *unsafe { zBuf.offset((nBuf - (1 as i32)) as isize) } }) as i32)
            & (128 as i32)
            == (0 as i32)
    {
        let mut j: i32 = 0 as i32;
        j = 0 as i32;
        i = 0 as i32;
        '__slate_break_1561: loop {
            if !(i < N && j < nBuf) {
                break;
            }
            let mut x: i64 = 0 as i64;
            let __v1822: i32 = j;
            let __v1823: i32 = __v1822
                + unsafe {
                    sqlite3Fts3GetVarint(
                        unsafe { zBuf.offset(j as isize) },
                        std::ptr::addr_of_mut!(x),
                    )
                };
            j = __v1823;
            unsafe {
                *unsafe { a.offset(i as isize) } =
                    ((x & (((4294967295 as u32) as u64) as i64)) as i32) as u32;
            }
            let __v1820: i32 = i;
            let __v1821: i32 = __v1820 + (1 as i32);
            i = __v1821;
        }
    }
    '__slate_break_1562: while i < N {
        let __v1824: i32 = i;
        let __v1825: i32 = __v1824 + (1 as i32);
        i = __v1825;
        unsafe {
            *unsafe { a.offset(__v1824 as isize) } = (0 as i32) as u32;
        }
    }
}

/// Insert the sizes (in tokens) for each column of the document
/// with docid equal to p->iPrevDocid.  The sizes are encoded as
/// a blob of varints.
///
/// # Arguments
///
/// * `pRC` - Result code
/// * `p` - Table into which to insert
/// * `aSz` - Sizes of each column, in tokens
fn fts3InsertDocsize(mut pRC: *mut i32, mut p: *mut Fts3Table, mut aSz: *mut u32) {
    let mut pBlob: *mut i8 = unsafe { std::mem::zeroed() }; // The BLOB encoding of the document size
    let mut nBlob: i32 = 0 as i32; // Number of bytes in the BLOB
    let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() }; // Statement used to insert the encoding
    let mut rc: i32 = 0 as i32; // Result code from subfunctions
    if (unsafe { *pRC }) != (0 as i32) {
        return;
    }
    pBlob = (unsafe {
        sqlite3_malloc64((((10 as i32) as i64) * ((unsafe { (*p).nColumn }) as i64)) as u64)
    }) as *mut i8;
    if pBlob == std::ptr::null_mut::<i8>() {
        unsafe {
            *pRC = 7 as i32;
        }
        return;
    }
    fts3EncodeIntArray(
        unsafe { (*p).nColumn },
        aSz,
        pBlob,
        std::ptr::addr_of_mut!(nBlob),
    );
    rc = fts3SqlStmt(
        p,
        20 as i32,
        std::ptr::addr_of_mut!(pStmt),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc != (0 as i32) {
        unsafe { sqlite3_free(pBlob as *mut ()) };
        unsafe {
            *pRC = rc;
        }
        return;
    }
    unsafe { sqlite3_bind_int64(pStmt, 1 as i32, unsafe { (*p).iPrevDocid }) };
    unsafe {
        sqlite3_bind_blob(pStmt, 2 as i32, pBlob as *const (), nBlob, unsafe {
            std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                sqlite3_free as *const (),
            )
        })
    };
    unsafe { sqlite3_step(pStmt) };
    unsafe {
        *pRC = unsafe { sqlite3_reset(pStmt) };
    }
}

/// Record 0 of the %_stat table contains a blob consisting of N varints,
/// where N is the number of user defined columns in the fts3 table plus
/// two. If nCol is the number of user defined columns, then values of the
/// varints are set as follows:
///
///   Varint 0:       Total number of rows in the table.
///
///   Varint 1..nCol: For each column, the total number of tokens stored in
///                   the column for all rows of the table.
///
///   Varint 1+nCol:  The total size, in bytes, of all text values in all
///                   columns of all rows of the table.
///
/// # Arguments
///
/// * `pRC` - The result code
/// * `p` - Table being updated
/// * `aSzIns` - Size increases
/// * `aSzDel` - Size decreases
/// * `nChng` - Change in the number of documents
fn fts3UpdateDocTotals(
    mut pRC: *mut i32,
    mut p: *mut Fts3Table,
    mut aSzIns: *mut u32,
    mut aSzDel: *mut u32,
    mut nChng: i32,
) {
    let mut pBlob: *mut i8 = unsafe { std::mem::zeroed() }; // Storage for BLOB written into %_stat
    let mut nBlob: i32 = 0 as i32; // Size of BLOB written into %_stat
    let mut a: *mut u32 = unsafe { std::mem::zeroed() }; // Array of integers that becomes the BLOB
    let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() }; // Statement for reading and writing
    let mut i: i32 = 0 as i32; // Loop counter
    let mut rc: i32 = 0 as i32; // Result code from subfunctions
    let mut nStat: i32 = (unsafe { (*p).nColumn }) + (2 as i32);
    if (unsafe { *pRC }) != (0 as i32) {
        return;
    }
    a = (unsafe {
        sqlite3_malloc64(
            (4 as u64)
                .wrapping_add(((10 as i32) as i64) as u64)
                .wrapping_mul((nStat as i64) as u64),
        )
    }) as *mut u32;
    if a == std::ptr::null_mut::<u32>() {
        unsafe {
            *pRC = 7 as i32;
        }
        return;
    }
    pBlob = (unsafe { a.offset(nStat as isize) }) as *mut i8;
    rc = fts3SqlStmt(
        p,
        22 as i32,
        std::ptr::addr_of_mut!(pStmt),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc != (0 as i32) {
        unsafe { sqlite3_free(a as *mut ()) };
        unsafe {
            *pRC = rc;
        }
        return;
    }
    unsafe { sqlite3_bind_int(pStmt, 1 as i32, 0 as i32) };
    if (unsafe { sqlite3_step(pStmt) }) == (100 as i32) {
        fts3DecodeIntArray(
            nStat,
            a,
            (unsafe { sqlite3_column_blob(pStmt, 0 as i32) }) as *const i8,
            unsafe { sqlite3_column_bytes(pStmt, 0 as i32) },
        );
    } else {
        unsafe {
            memset(
                a as *mut (),
                0 as i32,
                (4 as u64).wrapping_mul((nStat as i64) as u64),
            )
        };
    }
    rc = unsafe { sqlite3_reset(pStmt) };
    if rc != (0 as i32) {
        unsafe { sqlite3_free(a as *mut ()) };
        unsafe {
            *pRC = rc;
        }
        return;
    }
    if nChng < (0 as i32)
        && (unsafe { *unsafe { a.offset((0 as i32) as isize) } }) < (-nChng as u32)
    {
        unsafe {
            *unsafe { a.offset((0 as i32) as isize) } = (0 as i32) as u32;
        }
    } else {
        let __v1826: *mut u32 = unsafe { a.offset((0 as i32) as isize) };
        let __v1827: u32 = unsafe { *__v1826 };
        let __v1828: u32 = __v1827.wrapping_add(nChng as u32);
        unsafe {
            *__v1826 = __v1828;
        }
    }
    i = 0 as i32;
    '__slate_break_1563: loop {
        if !(i < (unsafe { (*p).nColumn }) + (1 as i32)) {
            break;
        }
        let mut x: u32 = unsafe { *unsafe { a.offset((i + (1 as i32)) as isize) } };
        if x.wrapping_add(unsafe { *unsafe { aSzIns.offset(i as isize) } })
            < unsafe { *unsafe { aSzDel.offset(i as isize) } }
        {
            x = (0 as i32) as u32;
        } else {
            x = x
                .wrapping_add(unsafe { *unsafe { aSzIns.offset(i as isize) } })
                .wrapping_sub(unsafe { *unsafe { aSzDel.offset(i as isize) } });
        }
        unsafe {
            *unsafe { a.offset((i + (1 as i32)) as isize) } = x;
        }
        let __v1829: i32 = i;
        let __v1830: i32 = __v1829 + (1 as i32);
        i = __v1830;
    }
    fts3EncodeIntArray(nStat, a, pBlob, std::ptr::addr_of_mut!(nBlob));
    rc = fts3SqlStmt(
        p,
        23 as i32,
        std::ptr::addr_of_mut!(pStmt),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc != (0 as i32) {
        unsafe { sqlite3_free(a as *mut ()) };
        unsafe {
            *pRC = rc;
        }
        return;
    }
    unsafe { sqlite3_bind_int(pStmt, 1 as i32, 0 as i32) };
    unsafe { sqlite3_bind_blob(pStmt, 2 as i32, pBlob as *const (), nBlob, None) };
    unsafe { sqlite3_step(pStmt) };
    unsafe {
        *pRC = unsafe { sqlite3_reset(pStmt) };
    }
    unsafe { sqlite3_bind_null(pStmt, 2 as i32) };
    unsafe { sqlite3_free(a as *mut ()) };
}

/// Merge the entire database so that there is one segment for each
/// iIndex/iLangid combination.
fn fts3DoOptimize(mut p: *mut Fts3Table, mut bReturnDone: i32) -> i32 {
    let mut bSeenDone: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut pAllLangid: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
    rc = sqlite3Fts3PendingTermsFlush(p);
    if rc == (0 as i32) {
        rc = fts3SqlStmt(
            p,
            27 as i32,
            std::ptr::addr_of_mut!(pAllLangid),
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
    }
    if rc == (0 as i32) {
        let mut rc2: i32 = 0 as i32;
        unsafe { sqlite3_bind_int(pAllLangid, 1 as i32, unsafe { (*p).iPrevLangid }) };
        unsafe { sqlite3_bind_int(pAllLangid, 2 as i32, unsafe { (*p).nIndex }) };
        '__slate_break_1564: while (unsafe { sqlite3_step(pAllLangid) }) == (100 as i32) {
            let mut i: i32 = 0 as i32;
            let mut iLangid: i32 = unsafe { sqlite3_column_int(pAllLangid, 0 as i32) };
            i = 0 as i32;
            '__slate_break_1565: loop {
                if !(rc == (0 as i32) && i < unsafe { (*p).nIndex }) {
                    break;
                }
                rc = fts3SegmentMerge(p, iLangid, i, -(2 as i32));
                if rc == (101 as i32) {
                    bSeenDone = 1 as i32;
                    rc = 0 as i32;
                }
                let __v1831: i32 = i;
                let __v1832: i32 = __v1831 + (1 as i32);
                i = __v1832;
            }
        }
        rc2 = unsafe { sqlite3_reset(pAllLangid) };
        if rc == (0 as i32) {
            rc = rc2;
        }
    }
    sqlite3Fts3SegmentsClose(p);
    return if rc == (0 as i32) && bReturnDone != (0 as i32) && bSeenDone != (0 as i32) {
        101 as i32
    } else {
        rc
    };
}

/// This function is called when the user executes the following statement:
///
///     INSERT INTO <tbl>(<tbl>) VALUES('rebuild');
///
/// The entire FTS index is discarded and rebuilt. If the table is one
/// created using the content=xxx option, then the new index is based on
/// the current contents of the xxx table. Otherwise, it is rebuilt based
/// on the contents of the %_content table.
fn fts3DoRebuild(mut p: *mut Fts3Table) -> i32 {
    let mut rc: i32 = 0 as i32; // Return Code
    rc = fts3DeleteAll(p, 0 as i32);
    if rc == (0 as i32) {
        let mut aSz: *mut u32 = std::ptr::null_mut::<u32>();
        let mut aSzIns: *mut u32 = std::ptr::null_mut::<u32>();
        let mut aSzDel: *mut u32 = std::ptr::null_mut::<u32>();
        let mut pStmt: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
        let mut nEntry: i32 = 0 as i32;
        // Compose and prepare an SQL statement to loop through the content table
        let mut zSql: *mut i8 = unsafe {
            sqlite3_mprintf((b"SELECT %s\0".as_ptr() as *mut i8) as *const i8, unsafe {
                (*p).zReadExprlist
            })
        };
        if !(zSql != std::ptr::null_mut::<i8>()) {
            rc = 7 as i32;
        } else {
            rc = sqlite3Fts3PrepareStmt(
                p,
                zSql as *const i8,
                0 as i32,
                1 as i32,
                std::ptr::addr_of_mut!(pStmt),
            );
            unsafe { sqlite3_free(zSql as *mut ()) };
        }
        if rc == (0 as i32) {
            let mut nByte: i64 = (4 as u64)
                .wrapping_mul((((unsafe { (*p).nColumn }) as i64) + ((1 as i32) as i64)) as u64)
                .wrapping_mul(((3 as i32) as i64) as u64) as i64;
            aSz = (unsafe { sqlite3_malloc64(nByte as u64) }) as *mut u32;
            if aSz == std::ptr::null_mut::<u32>() {
                rc = 7 as i32;
            } else {
                unsafe { memset(aSz as *mut (), 0 as i32, nByte as u64) };
                aSzIns = unsafe { aSz.offset(((unsafe { (*p).nColumn }) + (1 as i32)) as isize) };
                aSzDel =
                    unsafe { aSzIns.offset(((unsafe { (*p).nColumn }) + (1 as i32)) as isize) };
            }
        }
        '__slate_break_1567: loop {
            let __v1833: bool;
            if rc == (0 as i32) {
                __v1833 = (100 as i32) == unsafe { sqlite3_step(pStmt) };
            } else {
                __v1833 = false as bool;
            }
            if !__v1833 {
                break;
            }
            let mut iCol: i32 = 0 as i32;
            let mut iLangid: i32 = langidFromSelect(p, pStmt);
            rc = fts3PendingTermsDocid(p, 0 as i32, iLangid, unsafe {
                sqlite3_column_int64(pStmt, 0 as i32)
            });
            unsafe {
                memset(
                    aSz as *mut (),
                    0 as i32,
                    (4 as u64)
                        .wrapping_mul((((unsafe { (*p).nColumn }) + (1 as i32)) as i64) as u64),
                )
            };
            iCol = 0 as i32;
            '__slate_break_1568: loop {
                if !(rc == (0 as i32) && iCol < unsafe { (*p).nColumn }) {
                    break;
                }
                if (((unsafe { *unsafe { unsafe { (*p).abNotindexed }.offset(iCol as isize) } })
                    as u32) as i32)
                    == (0 as i32)
                {
                    let mut z: *const i8 =
                        (unsafe { sqlite3_column_text(pStmt, iCol + (1 as i32)) }) as *const i8;
                    rc = fts3PendingTermsAdd(p, iLangid, z, iCol, unsafe {
                        aSz.offset(iCol as isize)
                    });
                    let __v1836: *mut u32 =
                        unsafe { aSz.offset((unsafe { (*p).nColumn }) as isize) };
                    let __v1837: u32 = unsafe { *__v1836 };
                    let __v1838: u32 = __v1837.wrapping_add(
                        (unsafe { sqlite3_column_bytes(pStmt, iCol + (1 as i32)) }) as u32,
                    );
                    unsafe {
                        *__v1836 = __v1838;
                    }
                }
                let __v1834: i32 = iCol;
                let __v1835: i32 = __v1834 + (1 as i32);
                iCol = __v1835;
            }
            if (unsafe { (*p).bHasDocsize }) != (0 as u8) {
                fts3InsertDocsize(std::ptr::addr_of_mut!(rc), p, aSz);
            }
            if rc != (0 as i32) {
                unsafe { sqlite3_finalize(pStmt) };
                pStmt = std::ptr::null_mut::<sqlite3_stmt>();
            } else {
                let __v1839: i32 = nEntry;
                let __v1840: i32 = __v1839 + (1 as i32);
                nEntry = __v1840;
                iCol = 0 as i32;
                '__slate_break_1569: loop {
                    if !(iCol <= unsafe { (*p).nColumn }) {
                        break;
                    }
                    let __v1843: *mut u32 = unsafe { aSzIns.offset(iCol as isize) };
                    let __v1844: u32 = unsafe { *__v1843 };
                    let __v1845: u32 =
                        __v1844.wrapping_add(unsafe { *unsafe { aSz.offset(iCol as isize) } });
                    unsafe {
                        *__v1843 = __v1845;
                    }
                    let __v1841: i32 = iCol;
                    let __v1842: i32 = __v1841 + (1 as i32);
                    iCol = __v1842;
                }
            }
        }
        if (unsafe { (*p).bFts4 }) != (0 as u8) {
            fts3UpdateDocTotals(std::ptr::addr_of_mut!(rc), p, aSzIns, aSzDel, nEntry);
        }
        unsafe { sqlite3_free(aSz as *mut ()) };
        if pStmt != std::ptr::null_mut::<sqlite3_stmt>() {
            let mut rc2: i32 = unsafe { sqlite3_finalize(pStmt) };
            if rc == (0 as i32) {
                rc = rc2;
            }
        }
    }
    return rc;
}

/// This function opens a cursor used to read the input data for an
/// incremental merge operation. Specifically, it opens a cursor to scan
/// the oldest nSeg segments (idx=0 through idx=(nSeg-1)) in absolute
/// level iAbsLevel.
///
/// # Arguments
///
/// * `p` - FTS3 table handle
/// * `iAbsLevel` - Absolute level to open
/// * `nSeg` - Number of segments to merge
/// * `pCsr` - Cursor object to populate
fn fts3IncrmergeCsr(
    mut p: *mut Fts3Table,
    mut iAbsLevel: i64,
    mut nSeg: i32,
    mut pCsr: *mut Fts3MultiSegReader,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return Code
    let mut pStmt: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // Statement used to read %_segdir entry
    let mut nByte: i64 = 0 as i64; // Bytes allocated at pCsr->apSegment[]
    // Allocate space for the Fts3MultiSegReader.aCsr[] array
    unsafe { memset(pCsr as *mut (), 0 as i32, 88 as u64) };
    nByte = (8 as u64).wrapping_mul((nSeg as i64) as u64) as i64;
    unsafe {
        (*pCsr).apSegment = (unsafe { sqlite3_malloc64(nByte as u64) }) as *mut *mut Fts3SegReader;
    }
    if (unsafe { (*pCsr).apSegment }) == std::ptr::null_mut::<*mut Fts3SegReader>() {
        rc = 7 as i32;
    } else {
        unsafe {
            memset(
                (unsafe { (*pCsr).apSegment }) as *mut (),
                0 as i32,
                nByte as u64,
            )
        };
        rc = fts3SqlStmt(
            p,
            12 as i32,
            std::ptr::addr_of_mut!(pStmt),
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
    }
    if rc == (0 as i32) {
        let mut i: i32 = 0 as i32;
        let mut rc2: i32 = 0 as i32;
        unsafe { sqlite3_bind_int64(pStmt, 1 as i32, iAbsLevel) };
        0 as i32;
        i = 0 as i32;
        '__slate_break_1570: loop {
            let __v1846: bool;
            if rc == (0 as i32) {
                __v1846 = (unsafe { sqlite3_step(pStmt) }) == (100 as i32);
            } else {
                __v1846 = false as bool;
            }
            if !(__v1846 && i < nSeg) {
                break;
            }
            rc = sqlite3Fts3SegReaderNew(
                i,
                0 as i32,
                unsafe { sqlite3_column_int64(pStmt, 1 as i32) },
                unsafe { sqlite3_column_int64(pStmt, 2 as i32) },
                unsafe { sqlite3_column_int64(pStmt, 3 as i32) },
                (unsafe { sqlite3_column_blob(pStmt, 4 as i32) }) as *const i8,
                unsafe { sqlite3_column_bytes(pStmt, 4 as i32) },
                unsafe { unsafe { (*pCsr).apSegment }.offset(i as isize) },
            ); // segdir.start_block
            // segdir.leaves_end_block
            // segdir.end_block
            // segdir.root
            // segdir.root
            let __v1849: *mut Fts3MultiSegReader = pCsr;
            let __v1850: i32 = unsafe { (*__v1849).nSegment };
            let __v1851: i32 = __v1850 + (1 as i32);
            unsafe {
                (*__v1849).nSegment = __v1851;
            }
            let __v1847: i32 = i;
            let __v1848: i32 = __v1847 + (1 as i32);
            i = __v1848;
        }
        rc2 = unsafe { sqlite3_reset(pStmt) };
        if rc == (0 as i32) {
            rc = rc2;
        }
    }
    return rc;
}

/// An instance of the following structure is used as a dynamic buffer
/// to build up nodes or other blobs of data in.
///
/// The function blobGrowBuffer() is used to extend the allocation.
#[repr(C)]
#[derive(Clone, Copy)]
struct Blob {
    /// Pointer to allocation
    a: *mut i8,
    /// Number of valid bytes of data in a[]
    n: i32,
    /// Allocated size of a[] (nAlloc>=n)
    nAlloc: i32,
}

/// This structure is used to build up buffers containing segment b-tree
/// nodes (blocks).
#[repr(C)]
#[derive(Clone, Copy)]
struct NodeWriter {
    /// Current block id
    iBlock: i64,
    /// Last key written to the current block
    key: Blob,
    /// Current block image
    block: Blob,
}

/// An object of this type contains the state required to create or append
/// to an appendable b-tree segment.
#[repr(C)]
#[derive(Clone, Copy)]
struct IncrmergeWriter {
    /// Space allocated for leaf blocks
    nLeafEst: i64,
    /// Number of leaf pages flushed
    nWork: i64,
    /// Absolute level of input segments
    iAbsLevel: i64,
    /// Index of *output* segment in iAbsLevel+1
    iIdx: i32,
    /// Block number of first allocated block
    iStart: i64,
    /// Block number of last allocated block
    iEnd: i64,
    /// Bytes of leaf page data so far
    nLeafData: i64,
    /// If true, store 0 for segment size
    bNoLeafData: u8,
    aNodeWriter: [NodeWriter; 16],
}

/// An object of the following type is used to read data from a single
/// FTS segment node. See the following functions:
///
///     nodeReaderInit()
///     nodeReaderNext()
///     nodeReaderRelease()
#[repr(C)]
#[derive(Clone, Copy)]
struct NodeReader {
    aNode: *const i8,
    nNode: i32,
    /// Current offset within aNode[]
    iOff: i32,
    /// Output variables. Containing the current node entry.
    /// Pointer to child node
    iChild: i64,
    /// Current term
    term: Blob,
    /// Pointer to doclist
    aDoclist: *const i8,
    /// Size of doclist in bytes
    nDoclist: i32,
}

/// If *pRc is not SQLITE_OK when this function is called, it is a no-op.
/// Otherwise, if the allocation at pBlob->a is not already at least nMin
/// bytes in size, extend (realloc) it to be so.
///
/// If an OOM error occurs, set *pRc to SQLITE_NOMEM and leave pBlob->a
/// unmodified. Otherwise, if the allocation succeeds, update pBlob->nAlloc
/// to reflect the new size of the pBlob->a[] buffer.
fn blobGrowBuffer(mut pBlob: *mut Blob, mut nMin: i32, mut pRc: *mut i32) {
    if (unsafe { *pRc }) == (0 as i32) && nMin > unsafe { (*pBlob).nAlloc } {
        let mut nAlloc: i32 = nMin;
        let mut a: *mut i8 = (unsafe {
            sqlite3_realloc64((unsafe { (*pBlob).a }) as *mut (), (nAlloc as i64) as u64)
        }) as *mut i8;
        if a != std::ptr::null_mut::<i8>() {
            unsafe {
                (*pBlob).nAlloc = nAlloc;
            }
            unsafe {
                (*pBlob).a = a;
            }
        } else {
            unsafe {
                *pRc = 7 as i32;
            }
        }
    }
}

/// Attempt to advance the node-reader object passed as the first argument to
/// the next entry on the node.
///
/// Return an error code if an error occurs (SQLITE_NOMEM is possible).
/// Otherwise return SQLITE_OK. If there is no next entry on the node
/// (e.g. because the current entry is the last) set NodeReader->aNode to
/// NULL to indicate EOF. Otherwise, populate the NodeReader structure output
/// variables for the new entry.
fn nodeReaderNext(mut p: *mut NodeReader) -> i32 {
    let mut bFirst: i32 = ((unsafe { (*p).term.n }) == (0 as i32)) as i32; // True for first term on the node
    let mut nPrefix: i32 = 0 as i32; // Bytes to copy from previous term
    let mut nSuffix: i32 = 0 as i32; // Bytes to append to the prefix
    let mut rc: i32 = 0 as i32; // Return code
    0 as i32;
    if (unsafe { (*p).iChild }) != (0 as i64) && bFirst == (0 as i32) {
        let __v1852: *mut NodeReader = p;
        let __v1853: i64 = unsafe { (*__v1852).iChild };
        let __v1854: i64 = __v1853 + ((1 as i32) as i64);
        unsafe {
            (*__v1852).iChild = __v1854;
        }
    }
    if (unsafe { (*p).iOff }) >= unsafe { (*p).nNode } {
        // EOF
        unsafe {
            (*p).aNode = std::ptr::null::<i8>();
        }
    } else {
        if bFirst == (0 as i32) {
            let __v1855: *mut NodeReader = p;
            let __v1856: i32 = unsafe { (*__v1855).iOff };
            let __v1857: i32;
            if (((unsafe {
                *((unsafe { unsafe { (*p).aNode }.offset((unsafe { (*p).iOff }) as isize) })
                    as *mut u8)
            }) as u32) as i32)
                & (128 as i32)
                != (0 as i32)
            {
                __v1857 = unsafe {
                    sqlite3Fts3GetVarint32(
                        unsafe { unsafe { (*p).aNode }.offset((unsafe { (*p).iOff }) as isize) },
                        std::ptr::addr_of_mut!(nPrefix),
                    )
                };
            } else {
                unsafe {
                    *std::ptr::addr_of_mut!(nPrefix) = ((unsafe {
                        *((unsafe { unsafe { (*p).aNode }.offset((unsafe { (*p).iOff }) as isize) })
                            as *mut u8)
                    }) as u32) as i32;
                }
                __v1857 = 1 as i32;
            }
            let __v1858: i32 = __v1856 + __v1857;
            unsafe {
                (*__v1855).iOff = __v1858;
            }
        }
        let __v1859: *mut NodeReader = p;
        let __v1860: i32 = unsafe { (*__v1859).iOff };
        let __v1861: i32;
        if (((unsafe {
            *((unsafe { unsafe { (*p).aNode }.offset((unsafe { (*p).iOff }) as isize) }) as *mut u8)
        }) as u32) as i32)
            & (128 as i32)
            != (0 as i32)
        {
            __v1861 = unsafe {
                sqlite3Fts3GetVarint32(
                    unsafe { unsafe { (*p).aNode }.offset((unsafe { (*p).iOff }) as isize) },
                    std::ptr::addr_of_mut!(nSuffix),
                )
            };
        } else {
            unsafe {
                *std::ptr::addr_of_mut!(nSuffix) = ((unsafe {
                    *((unsafe { unsafe { (*p).aNode }.offset((unsafe { (*p).iOff }) as isize) })
                        as *mut u8)
                }) as u32) as i32;
            }
            __v1861 = 1 as i32;
        }
        let __v1862: i32 = __v1860 + __v1861;
        unsafe {
            (*__v1859).iOff = __v1862;
        }
        if nPrefix > unsafe { (*p).term.n }
            || nSuffix > (unsafe { (*p).nNode }) - unsafe { (*p).iOff }
            || nSuffix == (0 as i32)
        {
            return (11 as i32) | (1 as i32) << (8 as i32);
        }
        blobGrowBuffer(
            unsafe { std::ptr::addr_of_mut!((*p).term) },
            nPrefix + nSuffix,
            std::ptr::addr_of_mut!(rc),
        );
        if rc == (0 as i32) && (unsafe { (*p).term.a }) != std::ptr::null_mut::<i8>() {
            unsafe {
                memcpy(
                    (unsafe { unsafe { (*p).term.a }.offset(nPrefix as isize) }) as *mut (),
                    (unsafe { unsafe { (*p).aNode }.offset((unsafe { (*p).iOff }) as isize) })
                        as *const (),
                    (nSuffix as i64) as u64,
                )
            };
            unsafe {
                (*p).term.n = nPrefix + nSuffix;
            }
            let __v1863: *mut NodeReader = p;
            let __v1864: i32 = unsafe { (*__v1863).iOff };
            let __v1865: i32 = __v1864 + nSuffix;
            unsafe {
                (*__v1863).iOff = __v1865;
            }
            if (unsafe { (*p).iChild }) == ((0 as i32) as i64) {
                let __v1866: *mut NodeReader = p;
                let __v1867: i32 = unsafe { (*__v1866).iOff };
                let __v1868: i32;
                if (((unsafe {
                    *((unsafe { unsafe { (*p).aNode }.offset((unsafe { (*p).iOff }) as isize) })
                        as *mut u8)
                }) as u32) as i32)
                    & (128 as i32)
                    != (0 as i32)
                {
                    __v1868 = unsafe {
                        sqlite3Fts3GetVarint32(
                            unsafe {
                                unsafe { (*p).aNode }.offset((unsafe { (*p).iOff }) as isize)
                            },
                            unsafe { std::ptr::addr_of_mut!((*p).nDoclist) },
                        )
                    };
                } else {
                    unsafe {
                        *unsafe { std::ptr::addr_of_mut!((*p).nDoclist) } =
                            ((unsafe {
                                *((unsafe {
                                    unsafe { (*p).aNode }.offset((unsafe { (*p).iOff }) as isize)
                                }) as *mut u8)
                            }) as u32) as i32;
                    }
                    __v1868 = 1 as i32;
                }
                let __v1869: i32 = __v1867 + __v1868;
                unsafe {
                    (*__v1866).iOff = __v1869;
                }
                if (unsafe { (*p).nNode }) - unsafe { (*p).iOff } < unsafe { (*p).nDoclist } {
                    return (11 as i32) | (1 as i32) << (8 as i32);
                }
                unsafe {
                    (*p).aDoclist =
                        unsafe { unsafe { (*p).aNode }.offset((unsafe { (*p).iOff }) as isize) };
                }
                let __v1870: *mut NodeReader = p;
                let __v1871: i32 = unsafe { (*__v1870).iOff };
                let __v1872: i32 = __v1871 + unsafe { (*p).nDoclist };
                unsafe {
                    (*__v1870).iOff = __v1872;
                }
            }
        }
    }
    0 as i32;
    return rc;
}

/// Release all dynamic resources held by node-reader object *p.
fn nodeReaderRelease(mut p: *mut NodeReader) {
    unsafe { sqlite3_free((unsafe { (*p).term.a }) as *mut ()) };
}

/// Initialize a node-reader object to read the node in buffer aNode/nNode.
///
/// If successful, SQLITE_OK is returned and the NodeReader object set to
/// point to the first entry on the node (if any). Otherwise, an SQLite
/// error code is returned.
fn nodeReaderInit(mut p: *mut NodeReader, mut aNode: *const i8, mut nNode: i32) -> i32 {
    unsafe { memset(p as *mut (), 0 as i32, 56 as u64) };
    unsafe {
        (*p).aNode = aNode;
    }
    unsafe {
        (*p).nNode = nNode;
    }
    // Figure out if this is a leaf or an internal node.
    if aNode != std::ptr::null::<i8>()
        && (unsafe { *unsafe { aNode.offset((0 as i32) as isize) } }) != (0 as i8)
    {
        // An internal node.
        unsafe {
            (*p).iOff = (1 as i32)
                + unsafe {
                    sqlite3Fts3GetVarint(
                        unsafe { unsafe { (*p).aNode }.offset((1 as i32) as isize) },
                        unsafe { std::ptr::addr_of_mut!((*p).iChild) },
                    )
                };
        }
    } else {
        unsafe {
            (*p).iOff = 1 as i32;
        }
    }
    let __v1873: i32;
    if aNode != std::ptr::null::<i8>() {
        __v1873 = nodeReaderNext(p);
    } else {
        __v1873 = 0 as i32;
    }
    return __v1873;
}

/// This function is called while writing an FTS segment each time a leaf o
/// node is finished and written to disk. The key (zTerm/nTerm) is guaranteed
/// to be greater than the largest key on the node just written, but smaller
/// than or equal to the first key that will be written to the next leaf
/// node.
///
/// The block id of the leaf node just written to disk may be found in
/// (pWriter->aNodeWriter[0].iBlock) when this function is called.
///
/// # Arguments
///
/// * `p` - Fts3 table handle
/// * `pWriter` - Writer object
/// * `zTerm` - Term to write to internal node
/// * `nTerm` - Bytes at zTerm
fn fts3IncrmergePush(
    mut p: *mut Fts3Table,
    mut pWriter: *mut IncrmergeWriter,
    mut zTerm: *const i8,
    mut nTerm: i32,
) -> i32 {
    let mut iPtr: i64 = unsafe {
        (*unsafe {
            unsafe { (*pWriter).aNodeWriter.as_mut_ptr() as *mut NodeWriter }
                .offset((0 as i32) as isize)
        })
        .iBlock
    };
    let mut iLayer: i32 = 0 as i32;
    0 as i32;
    iLayer = 1 as i32;
    '__slate_break_1571: loop {
        if !(iLayer < (16 as i32)) {
            break;
        }
        let mut iNextPtr: i64 = (0 as i32) as i64;
        let mut pNode: *mut NodeWriter = unsafe {
            unsafe { (*pWriter).aNodeWriter.as_mut_ptr() as *mut NodeWriter }
                .offset(iLayer as isize)
        };
        let mut rc: i32 = 0 as i32;
        let mut nPrefix: i32 = 0 as i32;
        let mut nSuffix: i32 = 0 as i32;
        let mut nSpace: i32 = 0 as i32;
        // Figure out how much space the key will consume if it is written to
        // the current node of layer iLayer. Due to the prefix compression,
        // the space required changes depending on which node the key is to
        // be added to.
        nPrefix = fts3PrefixCompress(
            (unsafe { (*pNode).key.a }) as *const i8,
            unsafe { (*pNode).key.n },
            zTerm,
            nTerm,
        );
        nSuffix = nTerm - nPrefix;
        if nSuffix <= (0 as i32) {
            return (11 as i32) | (1 as i32) << (8 as i32);
        }
        nSpace = unsafe { sqlite3Fts3VarintLen((nPrefix as i64) as u64) };
        let __v1876: i32 = nSpace;
        let __v1877: i32 =
            __v1876 + ((unsafe { sqlite3Fts3VarintLen((nSuffix as i64) as u64) }) + nSuffix);
        nSpace = __v1877;
        if (unsafe { (*pNode).key.n }) == (0 as i32)
            || (unsafe { (*pNode).block.n }) + nSpace <= unsafe { (*p).nNodeSize }
        {
            // If the current node of layer iLayer contains zero keys, or if adding
            // the key to it will not cause it to grow to larger than nNodeSize
            // bytes in size, write the key here.
            let mut pBlk: *mut Blob = unsafe { std::ptr::addr_of_mut!((*pNode).block) };
            if (unsafe { (*pBlk).n }) == (0 as i32) {
                blobGrowBuffer(pBlk, unsafe { (*p).nNodeSize }, std::ptr::addr_of_mut!(rc));
                if rc == (0 as i32) {
                    unsafe {
                        *unsafe { unsafe { (*pBlk).a }.offset((0 as i32) as isize) } = iLayer as i8;
                    }
                    unsafe {
                        (*pBlk).n = (1 as i32)
                            + unsafe {
                                sqlite3Fts3PutVarint(
                                    unsafe { unsafe { (*pBlk).a }.offset((1 as i32) as isize) },
                                    iPtr,
                                )
                            };
                    }
                }
            }
            blobGrowBuffer(
                pBlk,
                (unsafe { (*pBlk).n }) + nSpace,
                std::ptr::addr_of_mut!(rc),
            );
            blobGrowBuffer(
                unsafe { std::ptr::addr_of_mut!((*pNode).key) },
                nTerm,
                std::ptr::addr_of_mut!(rc),
            );
            if rc == (0 as i32) {
                if (unsafe { (*pNode).key.n }) != (0 as i32) {
                    let __v1878: *mut Blob = pBlk;
                    let __v1879: i32 = unsafe { (*__v1878).n };
                    let __v1880: i32 = __v1879
                        + unsafe {
                            sqlite3Fts3PutVarint(
                                unsafe {
                                    unsafe { (*pBlk).a }.offset((unsafe { (*pBlk).n }) as isize)
                                },
                                nPrefix as i64,
                            )
                        };
                    unsafe {
                        (*__v1878).n = __v1880;
                    }
                }
                let __v1881: *mut Blob = pBlk;
                let __v1882: i32 = unsafe { (*__v1881).n };
                let __v1883: i32 = __v1882
                    + unsafe {
                        sqlite3Fts3PutVarint(
                            unsafe { unsafe { (*pBlk).a }.offset((unsafe { (*pBlk).n }) as isize) },
                            nSuffix as i64,
                        )
                    };
                unsafe {
                    (*__v1881).n = __v1883;
                }
                0 as i32;
                0 as i32;
                unsafe {
                    memcpy(
                        (unsafe { unsafe { (*pBlk).a }.offset((unsafe { (*pBlk).n }) as isize) })
                            as *mut (),
                        (unsafe { zTerm.offset(nPrefix as isize) }) as *const (),
                        (nSuffix as i64) as u64,
                    )
                };
                let __v1884: *mut Blob = pBlk;
                let __v1885: i32 = unsafe { (*__v1884).n };
                let __v1886: i32 = __v1885 + nSuffix;
                unsafe {
                    (*__v1884).n = __v1886;
                }
                unsafe {
                    memcpy(
                        (unsafe { (*pNode).key.a }) as *mut (),
                        zTerm as *const (),
                        (nTerm as i64) as u64,
                    )
                };
                unsafe {
                    (*pNode).key.n = nTerm;
                }
            }
        } else {
            // Otherwise, flush the current node of layer iLayer to disk.
            // Then allocate a new, empty sibling node. The key will be written
            // into the parent of this node.
            rc = fts3WriteSegment(
                p,
                unsafe { (*pNode).iBlock },
                unsafe { (*pNode).block.a },
                unsafe { (*pNode).block.n },
            );
            0 as i32;
            unsafe {
                *unsafe { unsafe { (*pNode).block.a }.offset((0 as i32) as isize) } = iLayer as i8;
            }
            unsafe {
                (*pNode).block.n = (1 as i32)
                    + unsafe {
                        sqlite3Fts3PutVarint(
                            unsafe { unsafe { (*pNode).block.a }.offset((1 as i32) as isize) },
                            iPtr + ((1 as i32) as i64),
                        )
                    };
            }
            iNextPtr = unsafe { (*pNode).iBlock };
            let __v1887: *mut NodeWriter = pNode;
            let __v1888: i64 = unsafe { (*__v1887).iBlock };
            let __v1889: i64 = __v1888 + ((1 as i32) as i64);
            unsafe {
                (*__v1887).iBlock = __v1889;
            }
            unsafe {
                (*pNode).key.n = 0 as i32;
            }
        }
        if rc != (0 as i32) || iNextPtr == ((0 as i32) as i64) {
            return rc;
        }
        iPtr = iNextPtr;
        let __v1874: i32 = iLayer;
        let __v1875: i32 = __v1874 + (1 as i32);
        iLayer = __v1875;
    }
    0 as i32;
    return 0 as i32;
}

/// Append a term and (optionally) doclist to the FTS segment node currently
/// stored in blob *pNode. The node need not contain any terms, but the
/// header must be written before this function is called.
///
/// A node header is a single 0x00 byte for a leaf node, or a height varint
/// followed by the left-hand-child varint for an internal node.
///
/// The term to be appended is passed via arguments zTerm/nTerm. For a
/// leaf node, the doclist is passed as aDoclist/nDoclist. For an internal
/// node, both aDoclist and nDoclist must be passed 0.
///
/// If the size of the value in blob pPrev is zero, then this is the first
/// term written to the node. Otherwise, pPrev contains a copy of the
/// previous term. Before this function returns, it is updated to contain a
/// copy of zTerm/nTerm.
///
/// It is assumed that the buffer associated with pNode is already large
/// enough to accommodate the new entry. The buffer associated with pPrev
/// is extended by this function if required.
///
/// If an error (i.e. OOM condition) occurs, an SQLite error code is
/// returned. Otherwise, SQLITE_OK.
///
/// # Arguments
///
/// * `pNode` - Current node image to append to
/// * `pPrev` - Buffer containing previous term written
/// * `zTerm` - New term to write
/// * `nTerm` - Size of zTerm in bytes
/// * `aDoclist` - Doclist (or NULL) to write
/// * `nDoclist` - Size of aDoclist in bytes
fn fts3AppendToNode(
    mut pNode: *mut Blob,
    mut pPrev: *mut Blob,
    mut zTerm: *const i8,
    mut nTerm: i32,
    mut aDoclist: *const i8,
    mut nDoclist: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut bFirst: i32 = ((unsafe { (*pPrev).n }) == (0 as i32)) as i32; // True if this is the first term written
    let mut nPrefix: i32 = 0 as i32; // Size of term prefix in bytes
    let mut nSuffix: i32 = 0 as i32; // Size of term suffix in bytes
    // Node must have already been started. There must be a doclist for a
    // leaf node, and there must not be a doclist for an internal node.
    0 as i32;
    0 as i32;
    blobGrowBuffer(pPrev, nTerm, std::ptr::addr_of_mut!(rc));
    if rc != (0 as i32) {
        return rc;
    }
    0 as i32;
    0 as i32;
    nPrefix = fts3PrefixCompress(
        (unsafe { (*pPrev).a }) as *const i8,
        unsafe { (*pPrev).n },
        zTerm,
        nTerm,
    );
    nSuffix = nTerm - nPrefix;
    if nSuffix <= (0 as i32) {
        return (11 as i32) | (1 as i32) << (8 as i32);
    }
    unsafe {
        memcpy(
            (unsafe { (*pPrev).a }) as *mut (),
            zTerm as *const (),
            (nTerm as i64) as u64,
        )
    };
    unsafe {
        (*pPrev).n = nTerm;
    }
    if bFirst == (0 as i32) {
        let __v1890: *mut Blob = pNode;
        let __v1891: i32 = unsafe { (*__v1890).n };
        let __v1892: i32 = __v1891
            + unsafe {
                sqlite3Fts3PutVarint(
                    unsafe { unsafe { (*pNode).a }.offset((unsafe { (*pNode).n }) as isize) },
                    nPrefix as i64,
                )
            };
        unsafe {
            (*__v1890).n = __v1892;
        }
    }
    let __v1893: *mut Blob = pNode;
    let __v1894: i32 = unsafe { (*__v1893).n };
    let __v1895: i32 = __v1894
        + unsafe {
            sqlite3Fts3PutVarint(
                unsafe { unsafe { (*pNode).a }.offset((unsafe { (*pNode).n }) as isize) },
                nSuffix as i64,
            )
        };
    unsafe {
        (*__v1893).n = __v1895;
    }
    unsafe {
        memcpy(
            (unsafe { unsafe { (*pNode).a }.offset((unsafe { (*pNode).n }) as isize) }) as *mut (),
            (unsafe { zTerm.offset(nPrefix as isize) }) as *const (),
            (nSuffix as i64) as u64,
        )
    };
    let __v1896: *mut Blob = pNode;
    let __v1897: i32 = unsafe { (*__v1896).n };
    let __v1898: i32 = __v1897 + nSuffix;
    unsafe {
        (*__v1896).n = __v1898;
    }
    if aDoclist != std::ptr::null::<i8>() {
        let __v1899: *mut Blob = pNode;
        let __v1900: i32 = unsafe { (*__v1899).n };
        let __v1901: i32 = __v1900
            + unsafe {
                sqlite3Fts3PutVarint(
                    unsafe { unsafe { (*pNode).a }.offset((unsafe { (*pNode).n }) as isize) },
                    nDoclist as i64,
                )
            };
        unsafe {
            (*__v1899).n = __v1901;
        }
        unsafe {
            memcpy(
                (unsafe { unsafe { (*pNode).a }.offset((unsafe { (*pNode).n }) as isize) })
                    as *mut (),
                aDoclist as *const (),
                (nDoclist as i64) as u64,
            )
        };
        let __v1902: *mut Blob = pNode;
        let __v1903: i32 = unsafe { (*__v1902).n };
        let __v1904: i32 = __v1903 + nDoclist;
        unsafe {
            (*__v1902).n = __v1904;
        }
    }
    0 as i32;
    return 0 as i32;
}

/// Append the current term and doclist pointed to by cursor pCsr to the
/// appendable b-tree segment opened for writing by pWriter.
///
/// Return SQLITE_OK if successful, or an SQLite error code otherwise.
///
/// # Arguments
///
/// * `p` - Fts3 table handle
/// * `pWriter` - Writer object
/// * `pCsr` - Cursor containing term and doclist
fn fts3IncrmergeAppend(
    mut p: *mut Fts3Table,
    mut pWriter: *mut IncrmergeWriter,
    mut pCsr: *mut Fts3MultiSegReader,
) -> i32 {
    let mut zTerm: *const i8 = (unsafe { (*pCsr).zTerm }) as *const i8;
    let mut nTerm: i32 = unsafe { (*pCsr).nTerm };
    let mut aDoclist: *const i8 = (unsafe { (*pCsr).aDoclist }) as *const i8;
    let mut nDoclist: i32 = unsafe { (*pCsr).nDoclist };
    let mut rc: i32 = 0 as i32; // Return code
    let mut nSpace: i32 = 0 as i32; // Total space in bytes required on leaf
    let mut nPrefix: i32 = 0 as i32; // Size of prefix shared with previous term
    let mut nSuffix: i32 = 0 as i32; // Size of suffix (nTerm - nPrefix)
    let mut pLeaf: *mut NodeWriter = unsafe { std::mem::zeroed() }; // Object used to write leaf nodes
    pLeaf = unsafe {
        unsafe { (*pWriter).aNodeWriter.as_mut_ptr() as *mut NodeWriter }
            .offset((0 as i32) as isize)
    };
    nPrefix = fts3PrefixCompress(
        (unsafe { (*pLeaf).key.a }) as *const i8,
        unsafe { (*pLeaf).key.n },
        zTerm,
        nTerm,
    );
    nSuffix = nTerm - nPrefix;
    if nSuffix <= (0 as i32) {
        return (11 as i32) | (1 as i32) << (8 as i32);
    }
    nSpace = unsafe { sqlite3Fts3VarintLen((nPrefix as i64) as u64) };
    let __v1905: i32 = nSpace;
    let __v1906: i32 =
        __v1905 + ((unsafe { sqlite3Fts3VarintLen((nSuffix as i64) as u64) }) + nSuffix);
    nSpace = __v1906;
    let __v1907: i32 = nSpace;
    let __v1908: i32 =
        __v1907 + ((unsafe { sqlite3Fts3VarintLen((nDoclist as i64) as u64) }) + nDoclist);
    nSpace = __v1908;
    // If the current block is not empty, and if adding this term/doclist
    // to the current block would make it larger than Fts3Table.nNodeSize bytes,
    // and if there is still room for another leaf page, write this block out to
    // the database.
    if (unsafe { (*pLeaf).block.n }) > (0 as i32)
        && (unsafe { (*pLeaf).block.n }) + nSpace > unsafe { (*p).nNodeSize }
        && (unsafe { (*pLeaf).iBlock })
            < (unsafe { (*pWriter).iStart }) + unsafe { (*pWriter).nLeafEst }
    {
        rc = fts3WriteSegment(
            p,
            unsafe { (*pLeaf).iBlock },
            unsafe { (*pLeaf).block.a },
            unsafe { (*pLeaf).block.n },
        );
        let __v1909: *mut IncrmergeWriter = pWriter;
        let __v1910: i64 = unsafe { (*__v1909).nWork };
        let __v1911: i64 = __v1910 + ((1 as i32) as i64);
        unsafe {
            (*__v1909).nWork = __v1911;
        }
        // Add the current term to the parent node. The term added to the
        // parent must:
        //
        //   a) be greater than the largest term on the leaf node just written
        //      to the database (still available in pLeaf->key), and
        //
        //   b) be less than or equal to the term about to be added to the new
        //      leaf node (zTerm/nTerm).
        //
        // In other words, it must be the prefix of zTerm 1 byte longer than
        // the common prefix (if any) of zTerm and pWriter->zTerm.
        if rc == (0 as i32) {
            rc = fts3IncrmergePush(p, pWriter, zTerm, nPrefix + (1 as i32));
        }
        // Advance to the next output block
        let __v1912: *mut NodeWriter = pLeaf;
        let __v1913: i64 = unsafe { (*__v1912).iBlock };
        let __v1914: i64 = __v1913 + ((1 as i32) as i64);
        unsafe {
            (*__v1912).iBlock = __v1914;
        }
        unsafe {
            (*pLeaf).key.n = 0 as i32;
        }
        unsafe {
            (*pLeaf).block.n = 0 as i32;
        }
        nSuffix = nTerm;
        nSpace = 1 as i32;
        let __v1915: i32 = nSpace;
        let __v1916: i32 =
            __v1915 + ((unsafe { sqlite3Fts3VarintLen((nSuffix as i64) as u64) }) + nSuffix);
        nSpace = __v1916;
        let __v1917: i32 = nSpace;
        let __v1918: i32 =
            __v1917 + ((unsafe { sqlite3Fts3VarintLen((nDoclist as i64) as u64) }) + nDoclist);
        nSpace = __v1918;
    }
    let __v1919: *mut IncrmergeWriter = pWriter;
    let __v1920: i64 = unsafe { (*__v1919).nLeafData };
    let __v1921: i64 = __v1920 + (nSpace as i64);
    unsafe {
        (*__v1919).nLeafData = __v1921;
    }
    blobGrowBuffer(
        unsafe { std::ptr::addr_of_mut!((*pLeaf).block) },
        (unsafe { (*pLeaf).block.n }) + nSpace,
        std::ptr::addr_of_mut!(rc),
    );
    if rc == (0 as i32) {
        if (unsafe { (*pLeaf).block.n }) == (0 as i32) {
            unsafe {
                (*pLeaf).block.n = 1 as i32;
            }
            unsafe {
                *unsafe { unsafe { (*pLeaf).block.a }.offset((0 as i32) as isize) } =
                    (0 as i32) as i8;
            }
        }
        rc = fts3AppendToNode(
            unsafe { std::ptr::addr_of_mut!((*pLeaf).block) },
            unsafe { std::ptr::addr_of_mut!((*pLeaf).key) },
            zTerm,
            nTerm,
            aDoclist,
            nDoclist,
        );
    }
    return rc;
}

/// This function is called to release all dynamic resources held by the
/// merge-writer object pWriter, and if no error has occurred, to flush
/// all outstanding node buffers held by pWriter to disk.
///
/// If *pRc is not SQLITE_OK when this function is called, then no attempt
/// is made to write any data to disk. Instead, this function serves only
/// to release outstanding resources.
///
/// Otherwise, if *pRc is initially SQLITE_OK and an error occurs while
/// flushing buffers to disk, *pRc is set to an SQLite error code before
/// returning.
///
/// # Arguments
///
/// * `p` - FTS3 table handle
/// * `pWriter` - Merge-writer object
/// * `pRc` - IN/OUT: Error code
fn fts3IncrmergeRelease(
    mut p: *mut Fts3Table,
    mut pWriter: *mut IncrmergeWriter,
    mut pRc: *mut i32,
) {
    let mut i: i32 = 0 as i32; // Used to iterate through non-root layers
    let mut iRoot: i32 = 0 as i32; // Index of root in pWriter->aNodeWriter
    let mut pRoot: *mut NodeWriter = unsafe { std::mem::zeroed() }; // NodeWriter for root node
    let mut rc: i32 = unsafe { *pRc }; // Error code
    // Set iRoot to the index in pWriter->aNodeWriter[] of the output segment
    // root node. If the segment fits entirely on a single leaf node, iRoot
    // will be set to 0. If the root node is the parent of the leaves, iRoot
    // will be 1. And so on.
    iRoot = (16 as i32) - (1 as i32);
    '__slate_break_1572: loop {
        if !(iRoot >= (0 as i32)) {
            break;
        }
        let mut pNode: *mut NodeWriter = unsafe {
            unsafe { (*pWriter).aNodeWriter.as_mut_ptr() as *mut NodeWriter }.offset(iRoot as isize)
        };
        if (unsafe { (*pNode).block.n }) > (0 as i32) {
            break '__slate_break_1572;
        }
        0 as i32;
        0 as i32;
        unsafe { sqlite3_free((unsafe { (*pNode).block.a }) as *mut ()) };
        unsafe { sqlite3_free((unsafe { (*pNode).key.a }) as *mut ()) };
        let __v1922: i32 = iRoot;
        let __v1923: i32 = __v1922 - (1 as i32);
        iRoot = __v1923;
    }
    // Empty output segment. This is a no-op.
    if iRoot < (0 as i32) {
        return;
    }
    // The entire output segment fits on a single node. Normally, this means
    // the node would be stored as a blob in the "root" column of the %_segdir
    // table. However, this is not permitted in this case. The problem is that
    // space has already been reserved in the %_segments table, and so the
    // start_block and end_block fields of the %_segdir table must be populated.
    // And, by design or by accident, released versions of FTS cannot handle
    // segments that fit entirely on the root node with start_block!=0.
    //
    // Instead, create a synthetic root node that contains nothing but a
    // pointer to the single content node. So that the segment consists of a
    // single leaf and a single interior (root) node.
    //
    // Todo: Better might be to defer allocating space in the %_segments
    // table until we are sure it is needed.
    if iRoot == (0 as i32) {
        let mut pBlock: *mut Blob = unsafe {
            std::ptr::addr_of_mut!(
                (*unsafe {
                    unsafe { (*pWriter).aNodeWriter.as_mut_ptr() as *mut NodeWriter }
                        .offset((1 as i32) as isize)
                })
                .block
            )
        };
        blobGrowBuffer(pBlock, (1 as i32) + (10 as i32), std::ptr::addr_of_mut!(rc));
        if rc == (0 as i32) {
            unsafe {
                *unsafe { unsafe { (*pBlock).a }.offset((0 as i32) as isize) } = (1 as i32) as i8;
            }
            unsafe {
                (*pBlock).n = (1 as i32)
                    + unsafe {
                        sqlite3Fts3PutVarint(
                            unsafe { unsafe { (*pBlock).a }.offset((1 as i32) as isize) },
                            unsafe {
                                (*unsafe {
                                    unsafe {
                                        (*pWriter).aNodeWriter.as_mut_ptr() as *mut NodeWriter
                                    }
                                    .offset((0 as i32) as isize)
                                })
                                .iBlock
                            },
                        )
                    };
            }
        }
        iRoot = 1 as i32;
    }
    pRoot = unsafe {
        unsafe { (*pWriter).aNodeWriter.as_mut_ptr() as *mut NodeWriter }.offset(iRoot as isize)
    };
    // Flush all currently outstanding nodes to disk.
    i = 0 as i32;
    '__slate_break_1573: loop {
        if !(i < iRoot) {
            break;
        }
        let mut pNode: *mut NodeWriter = unsafe {
            unsafe { (*pWriter).aNodeWriter.as_mut_ptr() as *mut NodeWriter }.offset(i as isize)
        };
        if (unsafe { (*pNode).block.n }) > (0 as i32) && rc == (0 as i32) {
            rc = fts3WriteSegment(
                p,
                unsafe { (*pNode).iBlock },
                unsafe { (*pNode).block.a },
                unsafe { (*pNode).block.n },
            );
        }
        unsafe { sqlite3_free((unsafe { (*pNode).block.a }) as *mut ()) };
        unsafe { sqlite3_free((unsafe { (*pNode).key.a }) as *mut ()) };
        let __v1924: i32 = i;
        let __v1925: i32 = __v1924 + (1 as i32);
        i = __v1925;
    }
    // Write the %_segdir record.
    if rc == (0 as i32) {
        rc = fts3WriteSegdir(
            p,
            (unsafe { (*pWriter).iAbsLevel }) + ((1 as i32) as i64),
            unsafe { (*pWriter).iIdx },
            unsafe { (*pWriter).iStart },
            unsafe {
                (*unsafe {
                    unsafe { (*pWriter).aNodeWriter.as_mut_ptr() as *mut NodeWriter }
                        .offset((0 as i32) as isize)
                })
                .iBlock
            },
            unsafe { (*pWriter).iEnd },
            if (((unsafe { (*pWriter).bNoLeafData }) as u32) as i32) == (0 as i32) {
                unsafe { (*pWriter).nLeafData }
            } else {
                (0 as i32) as i64
            },
            unsafe { (*pRoot).block.a },
            unsafe { (*pRoot).block.n },
        ); // level
        // idx
        // start_block
        // leaves_end_block
        // end_block
        // end_block
        // root
    }
    unsafe { sqlite3_free((unsafe { (*pRoot).block.a }) as *mut ()) };
    unsafe { sqlite3_free((unsafe { (*pRoot).key.a }) as *mut ()) };
    unsafe {
        *pRc = rc;
    }
}

/// Compare the term in buffer zLhs (size in bytes nLhs) with that in
/// zRhs (size in bytes nRhs) using memcmp. If one term is a prefix of
/// the other, it is considered to be smaller than the other.
///
/// Return -ve if zLhs is smaller than zRhs, 0 if it is equal, or +ve
/// if it is greater.
///
/// # Arguments
///
/// * `nLhs` - LHS of comparison
/// * `nRhs` - RHS of comparison
fn fts3TermCmp(mut zLhs: *const i8, mut nLhs: i32, mut zRhs: *const i8, mut nRhs: i32) -> i32 {
    let mut nCmp: i32 = if nLhs < nRhs { nLhs } else { nRhs };
    let mut res: i32 = 0 as i32;
    if nCmp != (0 as i32) && zLhs != std::ptr::null::<i8>() && zRhs != std::ptr::null::<i8>() {
        res = unsafe { memcmp(zLhs as *const (), zRhs as *const (), (nCmp as i64) as u64) };
    } else {
        res = 0 as i32;
    }
    if res == (0 as i32) {
        res = nLhs - nRhs;
    }
    return res;
}

/// Query to see if the entry in the %_segments table with blockid iEnd is
/// NULL. If no error occurs and the entry is NULL, set *pbRes 1 before
/// returning. Otherwise, set *pbRes to 0.
///
/// Or, if an error occurs while querying the database, return an SQLite
/// error code. The final value of *pbRes is undefined in this case.
///
/// This is used to test if a segment is an "appendable" segment. If it
/// is, then a NULL entry has been inserted into the %_segments table
/// with blockid %_segdir.end_block.
fn fts3IsAppendable(mut p: *mut Fts3Table, mut iEnd: i64, mut pbRes: *mut i32) -> i32 {
    let mut bRes: i32 = 0 as i32; // Result to set *pbRes to
    let mut pCheck: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // Statement to query database with
    let mut rc: i32 = 0 as i32; // Return code
    rc = fts3SqlStmt(
        p,
        34 as i32,
        std::ptr::addr_of_mut!(pCheck),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        unsafe { sqlite3_bind_int64(pCheck, 1 as i32, iEnd) };
        if (100 as i32) == unsafe { sqlite3_step(pCheck) } {
            bRes = 1 as i32;
        }
        rc = unsafe { sqlite3_reset(pCheck) };
    }
    unsafe {
        *pbRes = bRes;
    }
    return rc;
}

/// This function is called when initializing an incremental-merge operation.
/// It checks if the existing segment with index value iIdx at absolute level
/// (iAbsLevel+1) can be appended to by the incremental merge. If it can, the
/// merge-writer object *pWriter is initialized to write to it.
///
/// An existing segment can be appended to by an incremental merge if:
///
///   * It was initially created as an appendable segment (with all required
///     space pre-allocated), and
///
///   * The first key read from the input (arguments zKey and nKey) is
///     greater than the largest key currently stored in the potential
///     output segment.
///
/// # Arguments
///
/// * `p` - Fts3 table handle
/// * `iAbsLevel` - Absolute level of input segments
/// * `iIdx` - Index of candidate output segment
/// * `zKey` - First key to write
/// * `nKey` - Number of bytes in nKey
/// * `pWriter` - Populate this object
fn fts3IncrmergeLoad(
    mut p: *mut Fts3Table,
    mut iAbsLevel: i64,
    mut iIdx: i32,
    mut zKey: *const i8,
    mut nKey: i32,
    mut pWriter: *mut IncrmergeWriter,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut pSelect: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // SELECT to read %_segdir entry
    rc = fts3SqlStmt(
        p,
        32 as i32,
        std::ptr::addr_of_mut!(pSelect),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        let mut iStart: i64 = (0 as i32) as i64; // Value of %_segdir.start_block
        let mut iLeafEnd: i64 = (0 as i32) as i64; // Value of %_segdir.leaves_end_block
        let mut iEnd: i64 = (0 as i32) as i64; // Value of %_segdir.end_block
        let mut aRoot: *const i8 = std::ptr::null::<i8>(); // Pointer to %_segdir.root buffer
        let mut nRoot: i32 = 0 as i32; // Size of aRoot[] in bytes
        let mut rc2: i32 = 0 as i32; // Return code from sqlite3_reset()
        let mut bAppendable: i32 = 0 as i32; // Set to true if segment is appendable
        // Read the %_segdir entry for index iIdx absolute level (iAbsLevel+1)
        unsafe { sqlite3_bind_int64(pSelect, 1 as i32, iAbsLevel + ((1 as i32) as i64)) };
        unsafe { sqlite3_bind_int(pSelect, 2 as i32, iIdx) };
        if (unsafe { sqlite3_step(pSelect) }) == (100 as i32) {
            iStart = unsafe { sqlite3_column_int64(pSelect, 1 as i32) };
            iLeafEnd = unsafe { sqlite3_column_int64(pSelect, 2 as i32) };
            fts3ReadEndBlockField(pSelect, 3 as i32, std::ptr::addr_of_mut!(iEnd), unsafe {
                std::ptr::addr_of_mut!((*pWriter).nLeafData)
            });
            if (unsafe { (*pWriter).nLeafData }) < ((0 as i32) as i64) {
                unsafe {
                    (*pWriter).nLeafData = (unsafe { (*pWriter).nLeafData }) * (-(1 as i32) as i64);
                }
            }
            unsafe {
                (*pWriter).bNoLeafData =
                    ((unsafe { (*pWriter).nLeafData }) == ((0 as i32) as i64)) as u8;
            }
            nRoot = unsafe { sqlite3_column_bytes(pSelect, 4 as i32) };
            aRoot = (unsafe { sqlite3_column_blob(pSelect, 4 as i32) }) as *const i8;
            if aRoot == std::ptr::null::<i8>() {
                unsafe { sqlite3_reset(pSelect) };
                return if nRoot != (0 as i32) {
                    7 as i32
                } else {
                    (11 as i32) | (1 as i32) << (8 as i32)
                };
            }
        } else {
            return unsafe { sqlite3_reset(pSelect) };
        }
        // Check for the zero-length marker in the %_segments table
        rc = fts3IsAppendable(p, iEnd, std::ptr::addr_of_mut!(bAppendable));
        // Check that zKey/nKey is larger than the largest key the candidate
        if rc == (0 as i32) && bAppendable != (0 as i32) {
            let mut aLeaf: *mut i8 = std::ptr::null_mut::<i8>();
            let mut nLeaf: i32 = 0 as i32;
            rc = sqlite3Fts3ReadBlock(
                p,
                iLeafEnd,
                std::ptr::addr_of_mut!(aLeaf),
                std::ptr::addr_of_mut!(nLeaf),
                std::ptr::null_mut::<i32>(),
            );
            if rc == (0 as i32) {
                let mut reader: NodeReader = unsafe { std::mem::zeroed() };
                rc = nodeReaderInit(std::ptr::addr_of_mut!(reader), aLeaf as *const i8, nLeaf);
                '__slate_break_1574: while rc == (0 as i32)
                    && reader.aNode != std::ptr::null::<i8>()
                {
                    0 as i32;
                    rc = nodeReaderNext(std::ptr::addr_of_mut!(reader));
                }
                if fts3TermCmp(zKey, nKey, reader.term.a as *const i8, reader.term.n) <= (0 as i32)
                {
                    bAppendable = 0 as i32;
                }
                nodeReaderRelease(std::ptr::addr_of_mut!(reader));
            }
            unsafe { sqlite3_free(aLeaf as *mut ()) };
        }
        if rc == (0 as i32) && bAppendable != (0 as i32) {
            // It is possible to append to this segment. Set up the IncrmergeWriter
            // object to do so.
            let mut i: i32 = 0 as i32;
            let mut nHeight: i32 =
                (unsafe { *unsafe { aRoot.offset((0 as i32) as isize) } }) as i32;
            let mut pNode: *mut NodeWriter = unsafe { std::mem::zeroed() };
            if nHeight < (1 as i32) || nHeight >= (16 as i32) {
                unsafe { sqlite3_reset(pSelect) };
                return (11 as i32) | (1 as i32) << (8 as i32);
            }
            unsafe {
                (*pWriter).nLeafEst =
                    (((iEnd - iStart + ((1 as i32) as i64)) / ((16 as i32) as i64)) as i32) as i64;
            }
            unsafe {
                (*pWriter).iStart = iStart;
            }
            unsafe {
                (*pWriter).iEnd = iEnd;
            }
            unsafe {
                (*pWriter).iAbsLevel = iAbsLevel;
            }
            unsafe {
                (*pWriter).iIdx = iIdx;
            }
            i = nHeight + (1 as i32);
            '__slate_break_1575: loop {
                if !(i < (16 as i32)) {
                    break;
                }
                unsafe {
                    (*unsafe {
                        unsafe { (*pWriter).aNodeWriter.as_mut_ptr() as *mut NodeWriter }
                            .offset(i as isize)
                    })
                    .iBlock = (unsafe { (*pWriter).iStart })
                        + (i as i64) * unsafe { (*pWriter).nLeafEst };
                }
                let __v1926: i32 = i;
                let __v1927: i32 = __v1926 + (1 as i32);
                i = __v1927;
            }
            pNode = unsafe {
                unsafe { (*pWriter).aNodeWriter.as_mut_ptr() as *mut NodeWriter }
                    .offset(nHeight as isize)
            };
            unsafe {
                (*pNode).iBlock = (unsafe { (*pWriter).iStart })
                    + (unsafe { (*pWriter).nLeafEst }) * (nHeight as i64);
            }
            blobGrowBuffer(
                unsafe { std::ptr::addr_of_mut!((*pNode).block) },
                (if nRoot > unsafe { (*p).nNodeSize } {
                    nRoot
                } else {
                    unsafe { (*p).nNodeSize }
                }) + (10 as i32) * (2 as i32),
                std::ptr::addr_of_mut!(rc),
            );
            if rc == (0 as i32) {
                unsafe {
                    memcpy(
                        (unsafe { (*pNode).block.a }) as *mut (),
                        aRoot as *const (),
                        (nRoot as i64) as u64,
                    )
                };
                unsafe {
                    (*pNode).block.n = nRoot;
                }
                unsafe {
                    memset(
                        (unsafe { unsafe { (*pNode).block.a }.offset(nRoot as isize) }) as *mut (),
                        0 as i32,
                        (((10 as i32) * (2 as i32)) as i64) as u64,
                    )
                };
            }
            i = nHeight;
            '__slate_break_1576: loop {
                if !(i >= (0 as i32) && rc == (0 as i32)) {
                    break;
                }
                let mut reader: NodeReader = unsafe { std::mem::zeroed() };
                unsafe {
                    memset(
                        std::ptr::addr_of_mut!(reader) as *mut (),
                        0 as i32,
                        56 as u64,
                    )
                };
                pNode = unsafe {
                    unsafe { (*pWriter).aNodeWriter.as_mut_ptr() as *mut NodeWriter }
                        .offset(i as isize)
                };
                if (unsafe { (*pNode).block.a }) != std::ptr::null_mut::<i8>() {
                    rc = nodeReaderInit(
                        std::ptr::addr_of_mut!(reader),
                        (unsafe { (*pNode).block.a }) as *const i8,
                        unsafe { (*pNode).block.n },
                    );
                    '__slate_break_1577: while reader.aNode != std::ptr::null::<i8>()
                        && rc == (0 as i32)
                    {
                        rc = nodeReaderNext(std::ptr::addr_of_mut!(reader));
                    }
                    blobGrowBuffer(
                        unsafe { std::ptr::addr_of_mut!((*pNode).key) },
                        reader.term.n,
                        std::ptr::addr_of_mut!(rc),
                    );
                    if rc == (0 as i32) {
                        0 as i32;
                        if reader.term.n > (0 as i32) {
                            unsafe {
                                memcpy(
                                    (unsafe { (*pNode).key.a }) as *mut (),
                                    reader.term.a as *const (),
                                    (reader.term.n as i64) as u64,
                                )
                            };
                        }
                        unsafe {
                            (*pNode).key.n = reader.term.n;
                        }
                        if i > (0 as i32) {
                            let mut aBlock: *mut i8 = std::ptr::null_mut::<i8>();
                            let mut nBlock: i32 = 0 as i32;
                            pNode = unsafe {
                                unsafe { (*pWriter).aNodeWriter.as_mut_ptr() as *mut NodeWriter }
                                    .offset((i - (1 as i32)) as isize)
                            };
                            unsafe {
                                (*pNode).iBlock = reader.iChild;
                            }
                            rc = sqlite3Fts3ReadBlock(
                                p,
                                reader.iChild,
                                std::ptr::addr_of_mut!(aBlock),
                                std::ptr::addr_of_mut!(nBlock),
                                std::ptr::null_mut::<i32>(),
                            );
                            blobGrowBuffer(
                                unsafe { std::ptr::addr_of_mut!((*pNode).block) },
                                (if nBlock > unsafe { (*p).nNodeSize } {
                                    nBlock
                                } else {
                                    unsafe { (*p).nNodeSize }
                                }) + (10 as i32) * (2 as i32),
                                std::ptr::addr_of_mut!(rc),
                            );
                            if rc == (0 as i32) {
                                unsafe {
                                    memcpy(
                                        (unsafe { (*pNode).block.a }) as *mut (),
                                        aBlock as *const (),
                                        (nBlock as i64) as u64,
                                    )
                                };
                                unsafe {
                                    (*pNode).block.n = nBlock;
                                }
                                unsafe {
                                    memset(
                                        (unsafe {
                                            unsafe { (*pNode).block.a }.offset(nBlock as isize)
                                        }) as *mut (),
                                        0 as i32,
                                        (((10 as i32) * (2 as i32)) as i64) as u64,
                                    )
                                };
                            }
                            unsafe { sqlite3_free(aBlock as *mut ()) };
                        }
                    }
                }
                nodeReaderRelease(std::ptr::addr_of_mut!(reader));
                let __v1928: i32 = i;
                let __v1929: i32 = __v1928 - (1 as i32);
                i = __v1929;
            }
        }
        rc2 = unsafe { sqlite3_reset(pSelect) };
        if rc == (0 as i32) {
            rc = rc2;
        }
    }
    return rc;
}

/// Determine the largest segment index value that exists within absolute
/// level iAbsLevel+1. If no error occurs, set *piIdx to this value plus
/// one before returning SQLITE_OK. Or, if there are no segments at all
/// within level iAbsLevel, set *piIdx to zero.
///
/// If an error occurs, return an SQLite error code. The final value of
/// *piIdx is undefined in this case.
///
/// # Arguments
///
/// * `p` - FTS Table handle
/// * `iAbsLevel` - Absolute index of input segments
/// * `piIdx` - OUT: Next free index at iAbsLevel+1
fn fts3IncrmergeOutputIdx(mut p: *mut Fts3Table, mut iAbsLevel: i64, mut piIdx: *mut i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pOutputIdx: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // SQL used to find output index
    rc = fts3SqlStmt(
        p,
        8 as i32,
        std::ptr::addr_of_mut!(pOutputIdx),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        unsafe { sqlite3_bind_int64(pOutputIdx, 1 as i32, iAbsLevel + ((1 as i32) as i64)) };
        unsafe { sqlite3_step(pOutputIdx) };
        unsafe {
            *piIdx = unsafe { sqlite3_column_int(pOutputIdx, 0 as i32) };
        }
        rc = unsafe { sqlite3_reset(pOutputIdx) };
    }
    return rc;
}

/// Allocate an appendable output segment on absolute level iAbsLevel+1
/// with idx value iIdx.
///
/// In the %_segdir table, a segment is defined by the values in three
/// columns:
///
///     start_block
///     leaves_end_block
///     end_block
///
/// When an appendable segment is allocated, it is estimated that the
/// maximum number of leaf blocks that may be required is the sum of the
/// number of leaf blocks consumed by the input segments, plus the number
/// of input segments, multiplied by two. This value is stored in stack
/// variable nLeafEst.
///
/// A total of 16*nLeafEst blocks are allocated when an appendable segment
/// is created ((1 + end_block - start_block)==16*nLeafEst). The contiguous
/// array of leaf nodes starts at the first block allocated. The array
/// of interior nodes that are parents of the leaf nodes start at block
/// (start_block + (1 + end_block - start_block) / 16). And so on.
///
/// In the actual code below, the value "16" is replaced with the
/// pre-processor macro FTS_MAX_APPENDABLE_HEIGHT.
///
/// # Arguments
///
/// * `p` - Fts3 table handle
/// * `iAbsLevel` - Absolute level of input segments
/// * `iIdx` - Index of new output segment
/// * `pCsr` - Cursor that data will be read from
/// * `pWriter` - Populate this object
fn fts3IncrmergeWriter(
    mut p: *mut Fts3Table,
    mut iAbsLevel: i64,
    mut iIdx: i32,
    mut pCsr: *mut Fts3MultiSegReader,
    mut pWriter: *mut IncrmergeWriter,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return Code
    let mut i: i32 = 0 as i32; // Iterator variable
    let mut nLeafEst: i64 = (0 as i32) as i64; // Blocks allocated for leaf nodes
    let mut pLeafEst: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // SQL used to determine nLeafEst
    let mut pFirstBlock: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // SQL used to determine first block
    // Calculate nLeafEst.
    rc = fts3SqlStmt(
        p,
        29 as i32,
        std::ptr::addr_of_mut!(pLeafEst),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        unsafe { sqlite3_bind_int64(pLeafEst, 1 as i32, iAbsLevel) };
        unsafe { sqlite3_bind_int64(pLeafEst, 2 as i32, (unsafe { (*pCsr).nSegment }) as i64) };
        if (100 as i32) == unsafe { sqlite3_step(pLeafEst) } {
            nLeafEst = unsafe { sqlite3_column_int64(pLeafEst, 0 as i32) };
        }
        rc = unsafe { sqlite3_reset(pLeafEst) };
    }
    if rc != (0 as i32) {
        return rc;
    }
    // Calculate the first block to use in the output segment
    rc = fts3SqlStmt(
        p,
        10 as i32,
        std::ptr::addr_of_mut!(pFirstBlock),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        if (100 as i32) == unsafe { sqlite3_step(pFirstBlock) } {
            unsafe {
                (*pWriter).iStart = unsafe { sqlite3_column_int64(pFirstBlock, 0 as i32) };
            }
            unsafe {
                (*pWriter).iEnd = (unsafe { (*pWriter).iStart }) - ((1 as i32) as i64);
            }
            let __v1930: *mut IncrmergeWriter = pWriter;
            let __v1931: i64 = unsafe { (*__v1930).iEnd };
            let __v1932: i64 = __v1931 + nLeafEst * ((16 as i32) as i64);
            unsafe {
                (*__v1930).iEnd = __v1932;
            }
        }
        rc = unsafe { sqlite3_reset(pFirstBlock) };
    }
    if rc != (0 as i32) {
        return rc;
    }
    // Insert the marker in the %_segments table to make sure nobody tries
    // to steal the space just allocated. This is also used to identify
    // appendable segments.
    rc = fts3WriteSegment(
        p,
        unsafe { (*pWriter).iEnd },
        std::ptr::null_mut::<i8>(),
        0 as i32,
    );
    if rc != (0 as i32) {
        return rc;
    }
    unsafe {
        (*pWriter).iAbsLevel = iAbsLevel;
    }
    unsafe {
        (*pWriter).nLeafEst = nLeafEst;
    }
    unsafe {
        (*pWriter).iIdx = iIdx;
    }
    // Set up the array of NodeWriter objects
    i = 0 as i32;
    '__slate_break_1578: loop {
        if !(i < (16 as i32)) {
            break;
        }
        unsafe {
            (*unsafe {
                unsafe { (*pWriter).aNodeWriter.as_mut_ptr() as *mut NodeWriter }.offset(i as isize)
            })
            .iBlock = (unsafe { (*pWriter).iStart }) + (i as i64) * unsafe { (*pWriter).nLeafEst };
        }
        let __v1933: i32 = i;
        let __v1934: i32 = __v1933 + (1 as i32);
        i = __v1934;
    }
    return 0 as i32;
}

/// Remove an entry from the %_segdir table. This involves running the
/// following two statements:
///
///   DELETE FROM %_segdir WHERE level = :iAbsLevel AND idx = :iIdx
///   UPDATE %_segdir SET idx = idx - 1 WHERE level = :iAbsLevel AND idx > :iIdx
///
/// The DELETE statement removes the specific %_segdir level. The UPDATE
/// statement ensures that the remaining segments have contiguously allocated
/// idx values.
///
/// # Arguments
///
/// * `p` - FTS3 table handle
/// * `iAbsLevel` - Absolute level to delete from
/// * `iIdx` - Index of %_segdir entry to delete
fn fts3RemoveSegdirEntry(mut p: *mut Fts3Table, mut iAbsLevel: i64, mut iIdx: i32) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut pDelete: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // DELETE statement
    rc = fts3SqlStmt(
        p,
        30 as i32,
        std::ptr::addr_of_mut!(pDelete),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        unsafe { sqlite3_bind_int64(pDelete, 1 as i32, iAbsLevel) };
        unsafe { sqlite3_bind_int(pDelete, 2 as i32, iIdx) };
        unsafe { sqlite3_step(pDelete) };
        rc = unsafe { sqlite3_reset(pDelete) };
    }
    return rc;
}

/// One or more segments have just been removed from absolute level iAbsLevel.
/// Update the 'idx' values of the remaining segments in the level so that
/// the idx values are a contiguous sequence starting from 0.
///
/// # Arguments
///
/// * `p` - FTS3 table handle
/// * `iAbsLevel` - Absolute level to repack
fn fts3RepackSegdirLevel(mut p: *mut Fts3Table, mut iAbsLevel: i64) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut aIdx: *mut i32 = std::ptr::null_mut::<i32>(); // Array of remaining idx values
    let mut nIdx: i32 = 0 as i32; // Valid entries in aIdx[]
    let mut nAlloc: i32 = 0 as i32; // Allocated size of aIdx[]
    let mut i: i32 = 0 as i32; // Iterator variable
    let mut pSelect: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // Select statement to read idx values
    let mut pUpdate: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // Update statement to modify idx values
    rc = fts3SqlStmt(
        p,
        35 as i32,
        std::ptr::addr_of_mut!(pSelect),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        let mut rc2: i32 = 0 as i32;
        unsafe { sqlite3_bind_int64(pSelect, 1 as i32, iAbsLevel) };
        '__slate_break_1579: while (100 as i32) == unsafe { sqlite3_step(pSelect) } {
            if nIdx >= nAlloc {
                let mut aNew: *mut i32 = unsafe { std::mem::zeroed() };
                let __v1935: i32 = nAlloc;
                let __v1936: i32 = __v1935 + (16 as i32);
                nAlloc = __v1936;
                aNew = (unsafe {
                    sqlite3_realloc64(
                        aIdx as *mut (),
                        ((nAlloc as i64) as u64).wrapping_mul(4 as u64),
                    )
                }) as *mut i32;
                if !(aNew != std::ptr::null_mut::<i32>()) {
                    rc = 7 as i32;
                    break '__slate_break_1579;
                }
                aIdx = aNew;
            }
            let __v1937: i32 = nIdx;
            let __v1938: i32 = __v1937 + (1 as i32);
            nIdx = __v1938;
            unsafe {
                *unsafe { aIdx.offset(__v1937 as isize) } =
                    unsafe { sqlite3_column_int(pSelect, 0 as i32) };
            }
        }
        rc2 = unsafe { sqlite3_reset(pSelect) };
        if rc == (0 as i32) {
            rc = rc2;
        }
    }
    if rc == (0 as i32) {
        rc = fts3SqlStmt(
            p,
            31 as i32,
            std::ptr::addr_of_mut!(pUpdate),
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
    }
    if rc == (0 as i32) {
        unsafe { sqlite3_bind_int64(pUpdate, 2 as i32, iAbsLevel) };
    }
    0 as i32;
    unsafe {
        (*p).bIgnoreSavepoint = ((1 as i32) as i8) as u8;
    }
    i = 0 as i32;
    '__slate_break_1580: loop {
        if !(rc == (0 as i32) && i < nIdx) {
            break;
        }
        if (unsafe { *unsafe { aIdx.offset(i as isize) } }) != i {
            unsafe {
                sqlite3_bind_int(pUpdate, 3 as i32, unsafe {
                    *unsafe { aIdx.offset(i as isize) }
                })
            };
            unsafe { sqlite3_bind_int(pUpdate, 1 as i32, i) };
            unsafe { sqlite3_step(pUpdate) };
            rc = unsafe { sqlite3_reset(pUpdate) };
        }
        let __v1939: i32 = i;
        let __v1940: i32 = __v1939 + (1 as i32);
        i = __v1940;
    }
    unsafe {
        (*p).bIgnoreSavepoint = ((0 as i32) as i8) as u8;
    }
    unsafe { sqlite3_free(aIdx as *mut ()) };
    return rc;
}

fn fts3StartNode(mut pNode: *mut Blob, mut iHeight: i32, mut iChild: i64) {
    unsafe {
        *unsafe { unsafe { (*pNode).a }.offset((0 as i32) as isize) } = iHeight as i8;
    }
    if iChild != (0 as i64) {
        0 as i32;
        unsafe {
            (*pNode).n = (1 as i32)
                + unsafe {
                    sqlite3Fts3PutVarint(
                        unsafe { unsafe { (*pNode).a }.offset((1 as i32) as isize) },
                        iChild,
                    )
                };
        }
    } else {
        0 as i32;
        unsafe {
            (*pNode).n = 1 as i32;
        }
    }
}

/// The first two arguments are a pointer to and the size of a segment b-tree
/// node. The node may be a leaf or an internal node.
///
/// This function creates a new node image in blob object *pNew by copying
/// all terms that are greater than or equal to zTerm/nTerm (for leaf nodes)
/// or greater than zTerm/nTerm (for internal nodes) from aNode/nNode.
///
/// # Arguments
///
/// * `aNode` - Current node image
/// * `nNode` - Size of aNode in bytes
/// * `pNew` - OUT: Write new node image here
/// * `zTerm` - Omit all terms smaller than this
/// * `nTerm` - Size of zTerm in bytes
/// * `piBlock` - OUT: Block number in next layer down
fn fts3TruncateNode(
    mut aNode: *const i8,
    mut nNode: i32,
    mut pNew: *mut Blob,
    mut zTerm: *const i8,
    mut nTerm: i32,
    mut piBlock: *mut i64,
) -> i32 {
    let mut reader: NodeReader = unsafe { std::mem::zeroed() }; // Reader object
    let mut prev: Blob = Blob {
        a: std::ptr::null_mut::<i8>(),
        n: 0 as i32,
        nAlloc: 0 as i32,
    }; // Previous term written to new node
    let mut rc: i32 = 0 as i32; // Return code
    let mut bLeaf: i32 = 0 as i32; // True for a leaf node
    if nNode < (1 as i32) {
        return (11 as i32) | (1 as i32) << (8 as i32);
    }
    bLeaf =
        (((unsafe { *unsafe { aNode.offset((0 as i32) as isize) } }) as i32) == (0 as i32)) as i32;
    // Allocate required output space
    blobGrowBuffer(pNew, nNode, std::ptr::addr_of_mut!(rc));
    if rc != (0 as i32) {
        return rc;
    }
    unsafe {
        (*pNew).n = 0 as i32;
    }
    // Populate new node buffer
    rc = nodeReaderInit(std::ptr::addr_of_mut!(reader), aNode, nNode);
    '__slate_break_1581: while rc == (0 as i32) && reader.aNode != std::ptr::null::<i8>() {
        '__slate_continue_1581: {
            if (unsafe { (*pNew).n }) == (0 as i32) {
                let mut res: i32 =
                    fts3TermCmp(reader.term.a as *const i8, reader.term.n, zTerm, nTerm);
                if res < (0 as i32) || bLeaf == (0 as i32) && res == (0 as i32) {
                    break '__slate_continue_1581;
                }
                fts3StartNode(
                    pNew,
                    (unsafe { *unsafe { aNode.offset((0 as i32) as isize) } }) as i32,
                    reader.iChild,
                );
                unsafe {
                    *piBlock = reader.iChild;
                }
            }
            rc = fts3AppendToNode(
                pNew,
                std::ptr::addr_of_mut!(prev),
                reader.term.a as *const i8,
                reader.term.n,
                reader.aDoclist,
                reader.nDoclist,
            );
            if rc != (0 as i32) {
                break '__slate_break_1581;
            }
        }
        rc = nodeReaderNext(std::ptr::addr_of_mut!(reader));
    }
    if (unsafe { (*pNew).n }) == (0 as i32) {
        fts3StartNode(
            pNew,
            (unsafe { *unsafe { aNode.offset((0 as i32) as isize) } }) as i32,
            reader.iChild,
        );
        unsafe {
            *piBlock = reader.iChild;
        }
    }
    0 as i32;
    nodeReaderRelease(std::ptr::addr_of_mut!(reader));
    unsafe { sqlite3_free(prev.a as *mut ()) };
    return rc;
}

/// Remove all terms smaller than zTerm/nTerm from segment iIdx in absolute
/// level iAbsLevel. This may involve deleting entries from the %_segments
/// table, and modifying existing entries in both the %_segments and %_segdir
/// tables.
///
/// SQLITE_OK is returned if the segment is updated successfully. Or an
/// SQLite error code otherwise.
///
/// # Arguments
///
/// * `p` - FTS3 table handle
/// * `iAbsLevel` - Absolute level of segment to modify
/// * `iIdx` - Index within level of segment to modify
/// * `zTerm` - Remove terms smaller than this
/// * `nTerm` - Number of bytes in buffer zTerm
fn fts3TruncateSegment(
    mut p: *mut Fts3Table,
    mut iAbsLevel: i64,
    mut iIdx: i32,
    mut zTerm: *const i8,
    mut nTerm: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut root: Blob = Blob {
        a: std::ptr::null_mut::<i8>(),
        n: 0 as i32,
        nAlloc: 0 as i32,
    }; // New root page image
    let mut block: Blob = Blob {
        a: std::ptr::null_mut::<i8>(),
        n: 0 as i32,
        nAlloc: 0 as i32,
    }; // Buffer used for any other block
    let mut iBlock: i64 = (0 as i32) as i64; // Block id
    let mut iNewStart: i64 = (0 as i32) as i64; // New value for iStartBlock
    let mut iOldStart: i64 = (0 as i32) as i64; // Old value for iStartBlock
    let mut pFetch: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // Statement used to fetch segdir
    rc = fts3SqlStmt(
        p,
        32 as i32,
        std::ptr::addr_of_mut!(pFetch),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        let mut rc2: i32 = 0 as i32; // sqlite3_reset() return code
        unsafe { sqlite3_bind_int64(pFetch, 1 as i32, iAbsLevel) };
        unsafe { sqlite3_bind_int(pFetch, 2 as i32, iIdx) };
        if (100 as i32) == unsafe { sqlite3_step(pFetch) } {
            let mut aRoot: *const i8 =
                (unsafe { sqlite3_column_blob(pFetch, 4 as i32) }) as *const i8;
            let mut nRoot: i32 = unsafe { sqlite3_column_bytes(pFetch, 4 as i32) };
            iOldStart = unsafe { sqlite3_column_int64(pFetch, 1 as i32) };
            rc = fts3TruncateNode(
                aRoot,
                nRoot,
                std::ptr::addr_of_mut!(root),
                zTerm,
                nTerm,
                std::ptr::addr_of_mut!(iBlock),
            );
        }
        rc2 = unsafe { sqlite3_reset(pFetch) };
        if rc == (0 as i32) {
            rc = rc2;
        }
    }
    '__slate_break_1582: while rc == (0 as i32) && iBlock != (0 as i64) {
        let mut aBlock: *mut i8 = std::ptr::null_mut::<i8>();
        let mut nBlock: i32 = 0 as i32;
        iNewStart = iBlock;
        rc = sqlite3Fts3ReadBlock(
            p,
            iBlock,
            std::ptr::addr_of_mut!(aBlock),
            std::ptr::addr_of_mut!(nBlock),
            std::ptr::null_mut::<i32>(),
        );
        if rc == (0 as i32) {
            rc = fts3TruncateNode(
                aBlock as *const i8,
                nBlock,
                std::ptr::addr_of_mut!(block),
                zTerm,
                nTerm,
                std::ptr::addr_of_mut!(iBlock),
            );
        }
        if rc == (0 as i32) {
            rc = fts3WriteSegment(p, iNewStart, block.a, block.n);
        }
        unsafe { sqlite3_free(aBlock as *mut ()) };
    }
    // Variable iNewStart now contains the first valid leaf node.
    if rc == (0 as i32) && iNewStart != (0 as i64) {
        let mut pDel: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
        rc = fts3SqlStmt(
            p,
            17 as i32,
            std::ptr::addr_of_mut!(pDel),
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == (0 as i32) {
            unsafe { sqlite3_bind_int64(pDel, 1 as i32, iOldStart) };
            unsafe { sqlite3_bind_int64(pDel, 2 as i32, iNewStart - ((1 as i32) as i64)) };
            unsafe { sqlite3_step(pDel) };
            rc = unsafe { sqlite3_reset(pDel) };
        }
    }
    if rc == (0 as i32) {
        let mut pChomp: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
        rc = fts3SqlStmt(
            p,
            33 as i32,
            std::ptr::addr_of_mut!(pChomp),
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
        if rc == (0 as i32) {
            unsafe { sqlite3_bind_int64(pChomp, 1 as i32, iNewStart) };
            unsafe { sqlite3_bind_blob(pChomp, 2 as i32, root.a as *const (), root.n, None) };
            unsafe { sqlite3_bind_int64(pChomp, 3 as i32, iAbsLevel) };
            unsafe { sqlite3_bind_int(pChomp, 4 as i32, iIdx) };
            unsafe { sqlite3_step(pChomp) };
            rc = unsafe { sqlite3_reset(pChomp) };
            unsafe { sqlite3_bind_null(pChomp, 2 as i32) };
        }
    }
    unsafe { sqlite3_free(root.a as *mut ()) };
    unsafe { sqlite3_free(block.a as *mut ()) };
    return rc;
}

/// This function is called after an incrmental-merge operation has run to
/// merge (or partially merge) two or more segments from absolute level
/// iAbsLevel.
///
/// Each input segment is either removed from the db completely (if all of
/// its data was copied to the output segment by the incrmerge operation)
/// or modified in place so that it no longer contains those entries that
/// have been duplicated in the output segment.
///
/// # Arguments
///
/// * `p` - FTS table handle
/// * `iAbsLevel` - Absolute level containing segments
/// * `pCsr` - Chomp all segments opened by this cursor
/// * `pnRem` - Number of segments not deleted
fn fts3IncrmergeChomp(
    mut p: *mut Fts3Table,
    mut iAbsLevel: i64,
    mut pCsr: *mut Fts3MultiSegReader,
    mut pnRem: *mut i32,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut nRem: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    i = (unsafe { (*pCsr).nSegment }) - (1 as i32);
    '__slate_break_1583: loop {
        if !(i >= (0 as i32) && rc == (0 as i32)) {
            break;
        }
        let mut pSeg: *mut Fts3SegReader = std::ptr::null_mut::<Fts3SegReader>();
        let mut j: i32 = 0 as i32;
        // Find the Fts3SegReader object with Fts3SegReader.iIdx==i. It is hiding
        // somewhere in the pCsr->apSegment[] array.
        j = 0 as i32;
        '__slate_break_1584: loop {
            if !(j < unsafe { (*pCsr).nSegment }) {
                break;
            }
            pSeg = unsafe { *unsafe { unsafe { (*pCsr).apSegment }.offset(j as isize) } };
            if (unsafe { (*pSeg).iIdx }) == i {
                break '__slate_break_1584;
            }
            let __v1943: i32 = j;
            let __v1944: i32 = __v1943 + (1 as i32);
            j = __v1944;
        }
        0 as i32;
        if (unsafe { (*pSeg).aNode }) == std::ptr::null_mut::<i8>() {
            // Seg-reader is at EOF. Remove the entire input segment.
            rc = fts3DeleteSegment(p, pSeg);
            if rc == (0 as i32) {
                rc = fts3RemoveSegdirEntry(p, iAbsLevel, unsafe { (*pSeg).iIdx });
            }
            unsafe {
                *pnRem = 0 as i32;
            }
        } else {
            // The incremental merge did not copy all the data from this
            // segment to the upper level. The segment is modified in place
            // so that it contains no keys smaller than zTerm/nTerm.
            let mut zTerm: *const i8 = (unsafe { (*pSeg).zTerm }) as *const i8;
            let mut nTerm: i32 = unsafe { (*pSeg).nTerm };
            rc = fts3TruncateSegment(p, iAbsLevel, unsafe { (*pSeg).iIdx }, zTerm, nTerm);
            let __v1945: i32 = nRem;
            let __v1946: i32 = __v1945 + (1 as i32);
            nRem = __v1946;
        }
        let __v1941: i32 = i;
        let __v1942: i32 = __v1941 - (1 as i32);
        i = __v1942;
    }
    if rc == (0 as i32) && nRem != unsafe { (*pCsr).nSegment } {
        rc = fts3RepackSegdirLevel(p, iAbsLevel);
    }
    unsafe {
        *pnRem = nRem;
    }
    return rc;
}

/// Store an incr-merge hint in the database.
fn fts3IncrmergeHintStore(mut p: *mut Fts3Table, mut pHint: *mut Blob) -> i32 {
    let mut pReplace: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: i32 = 0 as i32; // Return code
    rc = fts3SqlStmt(
        p,
        23 as i32,
        std::ptr::addr_of_mut!(pReplace),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        unsafe { sqlite3_bind_int(pReplace, 1 as i32, 1 as i32) };
        unsafe {
            sqlite3_bind_blob(
                pReplace,
                2 as i32,
                (unsafe { (*pHint).a }) as *const (),
                unsafe { (*pHint).n },
                None,
            )
        };
        unsafe { sqlite3_step(pReplace) };
        rc = unsafe { sqlite3_reset(pReplace) };
        unsafe { sqlite3_bind_null(pReplace, 2 as i32) };
    }
    return rc;
}

/// Load an incr-merge hint from the database. The incr-merge hint, if one
/// exists, is stored in the rowid==1 row of the %_stat table.
///
/// If successful, populate blob *pHint with the value read from the %_stat
/// table and return SQLITE_OK. Otherwise, if an error occurs, return an
/// SQLite error code.
fn fts3IncrmergeHintLoad(mut p: *mut Fts3Table, mut pHint: *mut Blob) -> i32 {
    let mut pSelect: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: i32 = 0 as i32;
    unsafe {
        (*pHint).n = 0 as i32;
    }
    rc = fts3SqlStmt(
        p,
        22 as i32,
        std::ptr::addr_of_mut!(pSelect),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        let mut rc2: i32 = 0 as i32;
        unsafe { sqlite3_bind_int(pSelect, 1 as i32, 1 as i32) };
        if (100 as i32) == unsafe { sqlite3_step(pSelect) } {
            let mut aHint: *const i8 =
                (unsafe { sqlite3_column_blob(pSelect, 0 as i32) }) as *const i8;
            let mut nHint: i32 = unsafe { sqlite3_column_bytes(pSelect, 0 as i32) };
            if aHint != std::ptr::null::<i8>() {
                blobGrowBuffer(pHint, nHint, std::ptr::addr_of_mut!(rc));
                if rc == (0 as i32) {
                    if (unsafe { (*pHint).a }) != std::ptr::null_mut::<i8>() {
                        unsafe {
                            memcpy(
                                (unsafe { (*pHint).a }) as *mut (),
                                aHint as *const (),
                                (nHint as i64) as u64,
                            )
                        };
                    }
                    unsafe {
                        (*pHint).n = nHint;
                    }
                }
            }
        }
        rc2 = unsafe { sqlite3_reset(pSelect) };
        if rc == (0 as i32) {
            rc = rc2;
        }
    }
    return rc;
}

/// If *pRc is not SQLITE_OK when this function is called, it is a no-op.
/// Otherwise, append an entry to the hint stored in blob *pHint. Each entry
/// consists of two varints, the absolute level number of the input segments
/// and the number of input segments.
///
/// If successful, leave *pRc set to SQLITE_OK and return. If an error occurs,
/// set *pRc to an SQLite error code before returning.
///
/// # Arguments
///
/// * `pHint` - Hint blob to append to
/// * `iAbsLevel` - First varint to store in hint
/// * `nInput` - Second varint to store in hint
/// * `pRc` - IN/OUT: Error code
fn fts3IncrmergeHintPush(
    mut pHint: *mut Blob,
    mut iAbsLevel: i64,
    mut nInput: i32,
    mut pRc: *mut i32,
) {
    blobGrowBuffer(
        pHint,
        (unsafe { (*pHint).n }) + (2 as i32) * (10 as i32),
        pRc,
    );
    if (unsafe { *pRc }) == (0 as i32) {
        let __v1947: *mut Blob = pHint;
        let __v1948: i32 = unsafe { (*__v1947).n };
        let __v1949: i32 = __v1948
            + unsafe {
                sqlite3Fts3PutVarint(
                    unsafe { unsafe { (*pHint).a }.offset((unsafe { (*pHint).n }) as isize) },
                    iAbsLevel,
                )
            };
        unsafe {
            (*__v1947).n = __v1949;
        }
        let __v1950: *mut Blob = pHint;
        let __v1951: i32 = unsafe { (*__v1950).n };
        let __v1952: i32 = __v1951
            + unsafe {
                sqlite3Fts3PutVarint(
                    unsafe { unsafe { (*pHint).a }.offset((unsafe { (*pHint).n }) as isize) },
                    nInput as i64,
                )
            };
        unsafe {
            (*__v1950).n = __v1952;
        }
    }
}

/// Read the last entry (most recently pushed) from the hint blob *pHint
/// and then remove the entry. Write the two values read to *piAbsLevel and
/// *pnInput before returning.
///
/// If no error occurs, return SQLITE_OK. If the hint blob in *pHint does
/// not contain at least two valid varints, return SQLITE_CORRUPT_VTAB.
fn fts3IncrmergeHintPop(
    mut pHint: *mut Blob,
    mut piAbsLevel: *mut i64,
    mut pnInput: *mut i32,
) -> i32 {
    let mut nHint: i32 = unsafe { (*pHint).n };
    let mut i: i32 = 0 as i32;
    i = (unsafe { (*pHint).n }) - (1 as i32);
    if ((unsafe { *unsafe { unsafe { (*pHint).a }.offset(i as isize) } }) as i32) & (128 as i32)
        != (0 as i32)
    {
        return (11 as i32) | (1 as i32) << (8 as i32);
    }
    '__slate_break_1585: while i > (0 as i32)
        && ((unsafe { *unsafe { unsafe { (*pHint).a }.offset((i - (1 as i32)) as isize) } }) as i32)
            & (128 as i32)
            != (0 as i32)
    {
        let __v1953: i32 = i;
        let __v1954: i32 = __v1953 - (1 as i32);
        i = __v1954;
    }
    if i == (0 as i32) {
        return (11 as i32) | (1 as i32) << (8 as i32);
    }
    let __v1955: i32 = i;
    let __v1956: i32 = __v1955 - (1 as i32);
    i = __v1956;
    '__slate_break_1586: while i > (0 as i32)
        && ((unsafe { *unsafe { unsafe { (*pHint).a }.offset((i - (1 as i32)) as isize) } }) as i32)
            & (128 as i32)
            != (0 as i32)
    {
        let __v1957: i32 = i;
        let __v1958: i32 = __v1957 - (1 as i32);
        i = __v1958;
    }
    unsafe {
        (*pHint).n = i;
    }
    let __v1959: i32 = i;
    let __v1960: i32 = __v1959
        + unsafe {
            sqlite3Fts3GetVarint(
                (unsafe { unsafe { (*pHint).a }.offset(i as isize) }) as *const i8,
                piAbsLevel,
            )
        };
    i = __v1960;
    let __v1961: i32 = i;
    let __v1962: i32;
    if (((unsafe { *((unsafe { unsafe { (*pHint).a }.offset(i as isize) }) as *mut u8) }) as u32)
        as i32)
        & (128 as i32)
        != (0 as i32)
    {
        __v1962 = unsafe {
            sqlite3Fts3GetVarint32(
                (unsafe { unsafe { (*pHint).a }.offset(i as isize) }) as *const i8,
                pnInput,
            )
        };
    } else {
        unsafe {
            *pnInput =
                ((unsafe { *((unsafe { unsafe { (*pHint).a }.offset(i as isize) }) as *mut u8) })
                    as u32) as i32;
        }
        __v1962 = 1 as i32;
    }
    let __v1963: i32 = __v1961 + __v1962;
    i = __v1963;
    0 as i32;
    if i != nHint {
        return (11 as i32) | (1 as i32) << (8 as i32);
    }
    return 0 as i32;
}

/// Attempt an incremental merge that writes nMerge leaf blocks.
///
/// Incremental merges happen nMin segments at a time. The segments
/// to be merged are the nMin oldest segments (the ones with the smallest
/// values for the _segdir.idx field) in the highest level that contains
/// at least nMin segments. Multiple merges might occur in an attempt to
/// write the quota of nMerge leaf blocks.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3Incrmerge(
    mut p: *mut Fts3Table,
    mut nMerge: i32,
    mut nMin: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut nRem: i32 = nMerge; // Number of leaf pages yet to  be written
    let mut pCsr: *mut Fts3MultiSegReader = unsafe { std::mem::zeroed() }; // Cursor used to read input data
    let mut pFilter: *mut Fts3SegFilter = unsafe { std::mem::zeroed() }; // Filter used with cursor pCsr
    let mut pWriter: *mut IncrmergeWriter = unsafe { std::mem::zeroed() }; // Writer object
    let mut nSeg: i32 = 0 as i32; // Number of input segments
    let mut iAbsLevel: i64 = (0 as i32) as i64; // Absolute level number to work on
    let mut hint: Blob = Blob {
        a: std::ptr::null_mut::<i8>(),
        n: 0 as i32,
        nAlloc: 0 as i32,
    }; // Hint read from %_stat table
    let mut bDirtyHint: i32 = 0 as i32; // True if blob 'hint' has been modified
    // Allocate space for the cursor, filter and writer objects
    let mut nAlloc: i32 =
        ((88 as u64).wrapping_add(24 as u64).wrapping_add(704 as u64) as u32) as i32;
    pWriter = (unsafe { sqlite3_malloc64((nAlloc as i64) as u64) }) as *mut IncrmergeWriter;
    if !(pWriter != std::ptr::null_mut::<IncrmergeWriter>()) {
        return 7 as i32;
    }
    pFilter = (unsafe { pWriter.offset((1 as i32) as isize) }) as *mut Fts3SegFilter;
    pCsr = (unsafe { pFilter.offset((1 as i32) as isize) }) as *mut Fts3MultiSegReader;
    rc = fts3IncrmergeHintLoad(p, std::ptr::addr_of_mut!(hint));
    '__slate_break_1587: while rc == (0 as i32) && nRem > (0 as i32) {
        let mut nMod: i64 = ((1024 as i32) * unsafe { (*p).nIndex }) as i64;
        let mut pFindLevel: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // SQL used to determine iAbsLevel
        let mut bUseHint: i32 = 0 as i32; // True if attempting to append
        let mut iIdx: i32 = 0 as i32; // Largest idx in level (iAbsLevel+1)
        // Search the %_segdir table for the absolute level with the smallest
        // relative level number that contains at least nMin segments, if any.
        // If one is found, set iAbsLevel to the absolute level number and
        // nSeg to nMin. If no level with at least nMin segments can be found,
        // set nSeg to -1.
        rc = fts3SqlStmt(
            p,
            28 as i32,
            std::ptr::addr_of_mut!(pFindLevel),
            std::ptr::null_mut::<*mut sqlite3_value>(),
        );
        unsafe {
            sqlite3_bind_int(
                pFindLevel,
                1 as i32,
                if (2 as i32) > nMin { 2 as i32 } else { nMin },
            )
        };
        if (unsafe { sqlite3_step(pFindLevel) }) == (100 as i32) {
            iAbsLevel = unsafe { sqlite3_column_int64(pFindLevel, 0 as i32) };
            nSeg = unsafe { sqlite3_column_int(pFindLevel, 1 as i32) };
            0 as i32;
        } else {
            nSeg = -(1 as i32);
        }
        rc = unsafe { sqlite3_reset(pFindLevel) };
        // If the hint read from the %_stat table is not empty, check if the
        // last entry in it specifies a relative level smaller than or equal
        // to the level identified by the block above (if any). If so, this
        // iteration of the loop will work on merging at the hinted level.
        if rc == (0 as i32) && hint.n != (0 as i32) {
            let mut nHint: i32 = hint.n;
            let mut iHintAbsLevel: i64 = (0 as i32) as i64; // Hint level
            let mut nHintSeg: i32 = 0 as i32; // Hint number of segments
            rc = fts3IncrmergeHintPop(
                std::ptr::addr_of_mut!(hint),
                std::ptr::addr_of_mut!(iHintAbsLevel),
                std::ptr::addr_of_mut!(nHintSeg),
            );
            if nSeg < (0 as i32) || iAbsLevel % nMod >= iHintAbsLevel % nMod {
                // Based on the scan in the block above, it is known that there
                // are no levels with a relative level smaller than that of
                // iAbsLevel with more than nSeg segments, or if nSeg is -1,
                // no levels with more than nMin segments. Use this to limit the
                // value of nHintSeg to avoid a large memory allocation in case the
                // merge-hint is corrupt
                iAbsLevel = iHintAbsLevel;
                nSeg = if (if nMin > nSeg { nMin } else { nSeg }) < nHintSeg {
                    if nMin > nSeg { nMin } else { nSeg }
                } else {
                    nHintSeg
                };
                bUseHint = 1 as i32;
                bDirtyHint = 1 as i32;
            } else {
                // This undoes the effect of the HintPop() above - so that no entry
                // is removed from the hint blob.
                hint.n = nHint;
            }
        }
        // If nSeg is less that zero, then there is no level with at least
        // nMin segments and no hint in the %_stat table. No work to do.
        // Exit early in this case.
        if nSeg <= (0 as i32) {
            break '__slate_break_1587;
        }
        0 as i32;
        if iAbsLevel < ((0 as i32) as i64) || iAbsLevel > nMod << (32 as i32) {
            rc = (11 as i32) | (1 as i32) << (8 as i32);
            break '__slate_break_1587;
        }
        // Open a cursor to iterate through the contents of the oldest nSeg
        // indexes of absolute level iAbsLevel. If this cursor is opened using
        // the 'hint' parameters, it is possible that there are less than nSeg
        // segments available in level iAbsLevel. In this case, no work is
        // done on iAbsLevel - fall through to the next iteration of the loop
        // to start work on some other level.
        unsafe { memset(pWriter as *mut (), 0 as i32, (nAlloc as i64) as u64) };
        unsafe {
            (*pFilter).flags = 1 as i32;
        }
        if rc == (0 as i32) {
            rc = fts3IncrmergeOutputIdx(p, iAbsLevel, std::ptr::addr_of_mut!(iIdx));
            0 as i32;
            if iIdx == (0 as i32) || bUseHint != (0 as i32) && iIdx == (1 as i32) {
                let mut bIgnore: i32 = 0 as i32;
                rc = fts3SegmentIsMaxLevel(
                    p,
                    iAbsLevel + ((1 as i32) as i64),
                    std::ptr::addr_of_mut!(bIgnore),
                );
                if bIgnore != (0 as i32) {
                    let __v1652: *mut Fts3SegFilter = pFilter;
                    let __v1653: i32 = unsafe { (*__v1652).flags };
                    let __v1654: i32 = __v1653 | (2 as i32);
                    unsafe {
                        (*__v1652).flags = __v1654;
                    }
                }
            }
        }
        if rc == (0 as i32) {
            rc = fts3IncrmergeCsr(p, iAbsLevel, nSeg, pCsr);
        }
        let __v1655: bool;
        if (0 as i32) == rc && (unsafe { (*pCsr).nSegment }) == nSeg {
            let __v1656: i32 = sqlite3Fts3SegReaderStart(p, pCsr, pFilter);
            rc = __v1656;
            __v1655 = (0 as i32) == __v1656;
        } else {
            __v1655 = false as bool;
        }
        if __v1655 {
            let mut bEmpty: i32 = 0 as i32;
            rc = sqlite3Fts3SegReaderStep(p, pCsr);
            if rc == (0 as i32) {
                bEmpty = 1 as i32;
            } else {
                if rc != (100 as i32) {
                    sqlite3Fts3SegReaderFinish(pCsr);
                    break '__slate_break_1587;
                }
            }
            if bUseHint != (0 as i32) && iIdx > (0 as i32) {
                let mut zKey: *const i8 = (unsafe { (*pCsr).zTerm }) as *const i8;
                let mut nKey: i32 = unsafe { (*pCsr).nTerm };
                rc = fts3IncrmergeLoad(p, iAbsLevel, iIdx - (1 as i32), zKey, nKey, pWriter);
            } else {
                rc = fts3IncrmergeWriter(p, iAbsLevel, iIdx, pCsr, pWriter);
            }
            if rc == (0 as i32) && (unsafe { (*pWriter).nLeafEst }) != (0 as i64) {
                {}
                if bEmpty == (0 as i32) {
                    '__slate_break_1588: loop {
                        rc = fts3IncrmergeAppend(p, pWriter, pCsr);
                        if rc == (0 as i32) {
                            rc = sqlite3Fts3SegReaderStep(p, pCsr);
                        }
                        if (unsafe { (*pWriter).nWork }) >= (nRem as i64) && rc == (100 as i32) {
                            rc = 0 as i32;
                        }
                        if !(rc == (100 as i32)) {
                            break;
                        }
                    }
                }
                // Update or delete the input segments
                if rc == (0 as i32) {
                    let __v1657: i32 = nRem;
                    let __v1658: i32 = ((__v1657 as i64)
                        - (((1 as i32) as i64) + unsafe { (*pWriter).nWork }))
                        as i32;
                    nRem = __v1658;
                    rc = fts3IncrmergeChomp(p, iAbsLevel, pCsr, std::ptr::addr_of_mut!(nSeg));
                    if nSeg != (0 as i32) {
                        bDirtyHint = 1 as i32;
                        fts3IncrmergeHintPush(
                            std::ptr::addr_of_mut!(hint),
                            iAbsLevel,
                            nSeg,
                            std::ptr::addr_of_mut!(rc),
                        );
                    }
                }
            }
            if nSeg != (0 as i32) {
                unsafe {
                    (*pWriter).nLeafData = (unsafe { (*pWriter).nLeafData }) * (-(1 as i32) as i64);
                }
            }
            fts3IncrmergeRelease(p, pWriter, std::ptr::addr_of_mut!(rc));
            if nSeg == (0 as i32)
                && (((unsafe { (*pWriter).bNoLeafData }) as u32) as i32) == (0 as i32)
            {
                fts3PromoteSegments(p, iAbsLevel + ((1 as i32) as i64), unsafe {
                    (*pWriter).nLeafData
                });
            }
        }
        sqlite3Fts3SegReaderFinish(pCsr);
    }
    // Write the hint values into the %_stat table for the next incr-merger
    if bDirtyHint != (0 as i32) && rc == (0 as i32) {
        rc = fts3IncrmergeHintStore(p, std::ptr::addr_of_mut!(hint));
    }
    unsafe { sqlite3_free(pWriter as *mut ()) };
    unsafe { sqlite3_free(hint.a as *mut ()) };
    return rc;
}

/// Convert the text beginning at *pz into an integer and return
/// its value.  Advance *pz to point to the first character past
/// the integer.
///
/// This function used for parameters to merge= and incrmerge=
/// commands.
fn fts3Getint(mut pz: *mut *const i8) -> i32 {
    let mut z: *const i8 = unsafe { *pz };
    let mut i: i32 = 0 as i32;
    '__slate_break_1589: while ((unsafe { *z }) as i32) >= (48 as i32)
        && ((unsafe { *z }) as i32) <= (57 as i32)
        && i < (214748363 as i32)
    {
        let __v1964: *const i8 = z;
        let __v1965: *const i8 = unsafe { __v1964.offset((1 as i32) as isize) };
        z = __v1965;
        i = (10 as i32) * i + ((unsafe { *__v1964 }) as i32) - (48 as i32);
    }
    unsafe {
        *pz = z;
    }
    return i;
}

/// Process statements of the form:
///
///    INSERT INTO table(table) VALUES('merge=A,B');
///
/// A and B are integers that decode to be the number of leaf pages
/// written for the merge, and the minimum number of segments on a level
/// before it will be selected for a merge, respectively.
///
/// # Arguments
///
/// * `p` - FTS3 table handle
/// * `zParam` - Nul-terminated string containing "A,B"
fn fts3DoIncrmerge(mut p: *mut Fts3Table, mut zParam: *const i8) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut nMin: i32 = (16 as i32) / (2 as i32);
    let mut nMerge: i32 = 0 as i32;
    let mut z: *const i8 = zParam;
    // Read the first integer value
    nMerge = fts3Getint(std::ptr::addr_of_mut!(z));
    // If the first integer value is followed by a ',',  read the second
    // integer value.
    if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (44 as i32)
        && ((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as i32) != (0 as i32)
    {
        let __v1966: *const i8 = z;
        let __v1967: *const i8 = unsafe { __v1966.offset((1 as i32) as isize) };
        z = __v1967;
        nMin = fts3Getint(std::ptr::addr_of_mut!(z));
    }
    if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) != (0 as i32)
        || nMin < (2 as i32)
    {
        rc = 1 as i32;
    } else {
        rc = 0 as i32;
        if !((unsafe { (*p).bHasStat }) != (0 as u8)) {
            0 as i32;
            unsafe { sqlite3Fts3CreateStatTable(std::ptr::addr_of_mut!(rc), p) };
        }
        if rc == (0 as i32) {
            rc = sqlite3Fts3Incrmerge(p, nMerge, nMin);
        }
        sqlite3Fts3SegmentsClose(p);
    }
    return rc;
}

/// Process statements of the form:
///
///    INSERT INTO table(table) VALUES('automerge=X');
///
/// where X is an integer.  X==0 means to turn automerge off.  X!=0 means
/// turn it on.  The setting is persistent.
///
/// # Arguments
///
/// * `p` - FTS3 table handle
/// * `zParam` - Nul-terminated string containing boolean
fn fts3DoAutoincrmerge(mut p: *mut Fts3Table, mut zParam: *const i8) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pStmt: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
    unsafe {
        (*p).nAutoincrmerge = fts3Getint(std::ptr::addr_of_mut!(zParam));
    }
    if (unsafe { (*p).nAutoincrmerge }) == (1 as i32)
        || (unsafe { (*p).nAutoincrmerge }) > (16 as i32)
    {
        unsafe {
            (*p).nAutoincrmerge = 8 as i32;
        }
    }
    if !((unsafe { (*p).bHasStat }) != (0 as u8)) {
        0 as i32;
        unsafe { sqlite3Fts3CreateStatTable(std::ptr::addr_of_mut!(rc), p) };
        if rc != (0 as i32) {
            return rc;
        }
    }
    rc = fts3SqlStmt(
        p,
        23 as i32,
        std::ptr::addr_of_mut!(pStmt),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc != (0 as i32) {
        return rc;
    }
    unsafe { sqlite3_bind_int(pStmt, 1 as i32, 2 as i32) };
    unsafe { sqlite3_bind_int(pStmt, 2 as i32, unsafe { (*p).nAutoincrmerge }) };
    unsafe { sqlite3_step(pStmt) };
    rc = unsafe { sqlite3_reset(pStmt) };
    return rc;
}

/// Return a 64-bit checksum for the FTS index entry specified by the
/// arguments to this function.
///
/// # Arguments
///
/// * `zTerm` - Pointer to buffer containing term
/// * `nTerm` - Size of zTerm in bytes
/// * `iLangid` - Language id for current row
/// * `iIndex` - Index (0..Fts3Table.nIndex-1)
/// * `iDocid` - Docid for current row.
/// * `iCol` - Column number
/// * `iPos` - Position
fn fts3ChecksumEntry(
    mut zTerm: *const i8,
    mut nTerm: i32,
    mut iLangid: i32,
    mut iIndex: i32,
    mut iDocid: i64,
    mut iCol: i32,
    mut iPos: i32,
) -> u64 {
    let mut i: i32 = 0 as i32;
    let mut ret: u64 = iDocid as u64;
    let __v1968: u64 = ret;
    let __v1969: u64 =
        __v1968.wrapping_add((ret << (3 as i32)).wrapping_add((iLangid as i64) as u64));
    ret = __v1969;
    let __v1970: u64 = ret;
    let __v1971: u64 =
        __v1970.wrapping_add((ret << (3 as i32)).wrapping_add((iIndex as i64) as u64));
    ret = __v1971;
    let __v1972: u64 = ret;
    let __v1973: u64 = __v1972.wrapping_add((ret << (3 as i32)).wrapping_add((iCol as i64) as u64));
    ret = __v1973;
    let __v1974: u64 = ret;
    let __v1975: u64 = __v1974.wrapping_add((ret << (3 as i32)).wrapping_add((iPos as i64) as u64));
    ret = __v1975;
    i = 0 as i32;
    '__slate_break_1590: loop {
        if !(i < nTerm) {
            break;
        }
        let __v1978: u64 = ret;
        let __v1979: u64 = __v1978.wrapping_add((ret << (3 as i32)).wrapping_add(
            (((unsafe { *unsafe { zTerm.offset(i as isize) } }) as i32) as i64) as u64,
        ));
        ret = __v1979;
        let __v1976: i32 = i;
        let __v1977: i32 = __v1976 + (1 as i32);
        i = __v1977;
    }
    return ret;
}

/// Return a checksum of all entries in the FTS index that correspond to
/// language id iLangid. The checksum is calculated by XORing the checksums
/// of each individual entry (see fts3ChecksumEntry()) together.
///
/// If successful, the checksum value is returned and *pRc set to SQLITE_OK.
/// Otherwise, if an error occurs, *pRc is set to an SQLite error code. The
/// return value is undefined in this case.
///
/// # Arguments
///
/// * `p` - FTS3 table handle
/// * `iLangid` - Language id to return cksum for
/// * `iIndex` - Index to cksum (0..p->nIndex-1)
/// * `pRc` - OUT: Return code
fn fts3ChecksumIndex(
    mut p: *mut Fts3Table,
    mut iLangid: i32,
    mut iIndex: i32,
    mut pRc: *mut i32,
) -> u64 {
    let mut filter: Fts3SegFilter = unsafe { std::mem::zeroed() };
    let mut csr: Fts3MultiSegReader = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    let mut cksum: u64 = ((0 as i32) as i64) as u64;
    if (unsafe { *pRc }) != (0 as i32) {
        return ((0 as i32) as i64) as u64;
    }
    unsafe {
        memset(
            std::ptr::addr_of_mut!(filter) as *mut (),
            0 as i32,
            24 as u64,
        )
    };
    unsafe { memset(std::ptr::addr_of_mut!(csr) as *mut (), 0 as i32, 88 as u64) };
    filter.flags = (1 as i32) | (2 as i32);
    let __v1980: i32 = filter.flags;
    let __v1981: i32 = __v1980 | (16 as i32);
    filter.flags = __v1981;
    rc = unsafe {
        sqlite3Fts3SegReaderCursor(
            p,
            iLangid,
            iIndex,
            -(2 as i32),
            std::ptr::null::<i8>(),
            0 as i32,
            0 as i32,
            1 as i32,
            std::ptr::addr_of_mut!(csr),
        )
    };
    if rc == (0 as i32) {
        rc = sqlite3Fts3SegReaderStart(
            p,
            std::ptr::addr_of_mut!(csr),
            std::ptr::addr_of_mut!(filter),
        );
    }
    if rc == (0 as i32) {
        '__slate_break_1591: loop {
            let __v1982: i32 = sqlite3Fts3SegReaderStep(p, std::ptr::addr_of_mut!(csr));
            rc = __v1982;
            if !((100 as i32) == __v1982) {
                break;
            }
            let mut pCsr: *mut i8 = csr.aDoclist;
            let mut pEnd: *mut i8 = unsafe { pCsr.offset(csr.nDoclist as isize) };
            let mut iDocid: i64 = (0 as i32) as i64;
            let mut iCol: i64 = (0 as i32) as i64;
            let mut iPos: u64 = ((0 as i32) as i64) as u64;
            let __v1983: *mut i8 = pCsr;
            let __v1984: *mut i8 = unsafe {
                __v1983.offset(
                    (unsafe {
                        sqlite3Fts3GetVarint(pCsr as *const i8, std::ptr::addr_of_mut!(iDocid))
                    }) as isize,
                )
            };
            pCsr = __v1984;
            '__slate_break_1592: while pCsr < pEnd {
                let mut iVal: u64 = ((0 as i32) as i64) as u64;
                let __v1985: *mut i8 = pCsr;
                let __v1986: *mut i8 = unsafe {
                    __v1985.offset(
                        (unsafe {
                            sqlite3Fts3GetVarintU(pCsr as *const i8, std::ptr::addr_of_mut!(iVal))
                        }) as isize,
                    )
                };
                pCsr = __v1986;
                if pCsr < pEnd {
                    if iVal == (((0 as i32) as i64) as u64) || iVal == (((1 as i32) as i64) as u64)
                    {
                        iCol = (0 as i32) as i64;
                        iPos = ((0 as i32) as i64) as u64;
                        if iVal != (0 as u64) {
                            let __v1987: *mut i8 = pCsr;
                            let __v1988: *mut i8 = unsafe {
                                __v1987.offset(
                                    (unsafe {
                                        sqlite3Fts3GetVarint(
                                            pCsr as *const i8,
                                            std::ptr::addr_of_mut!(iCol),
                                        )
                                    }) as isize,
                                )
                            };
                            pCsr = __v1988;
                        } else {
                            let __v1989: *mut i8 = pCsr;
                            let __v1990: *mut i8 = unsafe {
                                __v1989.offset(
                                    (unsafe {
                                        sqlite3Fts3GetVarintU(
                                            pCsr as *const i8,
                                            std::ptr::addr_of_mut!(iVal),
                                        )
                                    }) as isize,
                                )
                            };
                            pCsr = __v1990;
                            if (unsafe { (*p).bDescIdx }) != (0 as u8) {
                                iDocid = (iDocid as u64).wrapping_sub(iVal) as i64;
                            } else {
                                iDocid = (iDocid as u64).wrapping_add(iVal) as i64;
                            }
                        }
                    } else {
                        let __v1991: u64 = iPos;
                        let __v1992: u64 =
                            __v1991.wrapping_add(iVal.wrapping_sub(((2 as i32) as i64) as u64));
                        iPos = __v1992;
                        cksum = cksum
                            ^ fts3ChecksumEntry(
                                csr.zTerm as *const i8,
                                csr.nTerm,
                                iLangid,
                                iIndex,
                                iDocid,
                                iCol as i32,
                                (iPos as u32) as i32,
                            );
                    }
                }
            }
        }
    }
    sqlite3Fts3SegReaderFinish(std::ptr::addr_of_mut!(csr));
    unsafe {
        *pRc = rc;
    }
    return cksum;
}

/// Check if the contents of the FTS index match the current contents of the
/// content table. If no error occurs and the contents do match, set *pbOk
/// to true and return SQLITE_OK. Or if the contents do not match, set *pbOk
/// to false before returning.
///
/// If an error occurs (e.g. an OOM or IO error), return an SQLite error
/// code. The final value of *pbOk is undefined in this case.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3IntegrityCheck(mut p: *mut Fts3Table, mut pbOk: *mut i32) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut cksum1: u64 = ((0 as i32) as i64) as u64; // Checksum based on FTS index contents
    let mut cksum2: u64 = ((0 as i32) as i64) as u64; // Checksum based on %_content contents
    let mut pAllLangid: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // Statement to return all language-ids
    // This block calculates the checksum according to the FTS index.
    rc = fts3SqlStmt(
        p,
        27 as i32,
        std::ptr::addr_of_mut!(pAllLangid),
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
    if rc == (0 as i32) {
        let mut rc2: i32 = 0 as i32;
        unsafe { sqlite3_bind_int(pAllLangid, 1 as i32, unsafe { (*p).iPrevLangid }) };
        unsafe { sqlite3_bind_int(pAllLangid, 2 as i32, unsafe { (*p).nIndex }) };
        '__slate_break_1593: loop {
            let __v1674: bool;
            if rc == (0 as i32) {
                __v1674 = (unsafe { sqlite3_step(pAllLangid) }) == (100 as i32);
            } else {
                __v1674 = false as bool;
            }
            if !__v1674 {
                break;
            }
            let mut iLangid: i32 = unsafe { sqlite3_column_int(pAllLangid, 0 as i32) };
            let mut i: i32 = 0 as i32;
            i = 0 as i32;
            '__slate_break_1594: loop {
                if !(i < unsafe { (*p).nIndex }) {
                    break;
                }
                cksum1 = cksum1 ^ fts3ChecksumIndex(p, iLangid, i, std::ptr::addr_of_mut!(rc));
                let __v1675: i32 = i;
                let __v1676: i32 = __v1675 + (1 as i32);
                i = __v1676;
            }
        }
        rc2 = unsafe { sqlite3_reset(pAllLangid) };
        if rc == (0 as i32) {
            rc = rc2;
        }
    }
    // This block calculates the checksum according to the %_content table
    if rc == (0 as i32) {
        let mut pModule: *const sqlite3_tokenizer_module =
            unsafe { (*unsafe { (*p).pTokenizer }).pModule };
        let mut pStmt: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
        let mut zSql: *mut i8 = unsafe { std::mem::zeroed() };
        zSql = unsafe {
            sqlite3_mprintf((b"SELECT %s\0".as_ptr() as *mut i8) as *const i8, unsafe {
                (*p).zReadExprlist
            })
        };
        if !(zSql != std::ptr::null_mut::<i8>()) {
            rc = 7 as i32;
        } else {
            rc = sqlite3Fts3PrepareStmt(
                p,
                zSql as *const i8,
                0 as i32,
                1 as i32,
                std::ptr::addr_of_mut!(pStmt),
            );
            unsafe { sqlite3_free(zSql as *mut ()) };
        }
        '__slate_break_1596: loop {
            let __v1677: bool;
            if rc == (0 as i32) {
                __v1677 = (100 as i32) == unsafe { sqlite3_step(pStmt) };
            } else {
                __v1677 = false as bool;
            }
            if !__v1677 {
                break;
            }
            let mut iDocid: i64 = unsafe { sqlite3_column_int64(pStmt, 0 as i32) };
            let mut iLang: i32 = langidFromSelect(p, pStmt);
            let mut iCol: i32 = 0 as i32;
            iCol = 0 as i32;
            '__slate_break_1597: loop {
                if !(rc == (0 as i32) && iCol < unsafe { (*p).nColumn }) {
                    break;
                }
                if (((unsafe { *unsafe { unsafe { (*p).abNotindexed }.offset(iCol as isize) } })
                    as u32) as i32)
                    == (0 as i32)
                {
                    let mut zText: *const i8 =
                        (unsafe { sqlite3_column_text(pStmt, iCol + (1 as i32)) }) as *const i8;
                    let mut pT: *mut sqlite3_tokenizer_cursor =
                        std::ptr::null_mut::<sqlite3_tokenizer_cursor>();
                    rc = unsafe {
                        sqlite3Fts3OpenTokenizer(
                            unsafe { (*p).pTokenizer },
                            iLang,
                            zText,
                            -(1 as i32),
                            std::ptr::addr_of_mut!(pT),
                        )
                    };
                    '__slate_break_1598: while rc == (0 as i32) {
                        let mut zToken: *const i8 = unsafe { std::mem::zeroed() }; // Buffer containing token
                        let mut nToken: i32 = 0 as i32; // Number of bytes in token
                        let mut iDum1: i32 = 0 as i32;
                        let mut iDum2: i32 = 0 as i32; // Dummy variables
                        let mut iPos: i32 = 0 as i32; // Position of token in zText
                        rc = unsafe {
                            unsafe { (*pModule).xNext }.unwrap()(
                                pT,
                                std::ptr::addr_of_mut!(zToken),
                                std::ptr::addr_of_mut!(nToken),
                                std::ptr::addr_of_mut!(iDum1),
                                std::ptr::addr_of_mut!(iDum2),
                                std::ptr::addr_of_mut!(iPos),
                            )
                        };
                        if rc == (0 as i32) {
                            let mut i: i32 = 0 as i32;
                            cksum2 = cksum2
                                ^ fts3ChecksumEntry(
                                    zToken, nToken, iLang, 0 as i32, iDocid, iCol, iPos,
                                );
                            i = 1 as i32;
                            '__slate_break_1599: loop {
                                if !(i < unsafe { (*p).nIndex }) {
                                    break;
                                }
                                if (unsafe {
                                    (*unsafe { unsafe { (*p).aIndex }.offset(i as isize) }).nPrefix
                                }) <= nToken
                                {
                                    cksum2 = cksum2
                                        ^ fts3ChecksumEntry(
                                            zToken,
                                            unsafe {
                                                (*unsafe {
                                                    unsafe { (*p).aIndex }.offset(i as isize)
                                                })
                                                .nPrefix
                                            },
                                            iLang,
                                            i,
                                            iDocid,
                                            iCol,
                                            iPos,
                                        );
                                }
                                let __v1680: i32 = i;
                                let __v1681: i32 = __v1680 + (1 as i32);
                                i = __v1681;
                            }
                        }
                    }
                    if pT != std::ptr::null_mut::<sqlite3_tokenizer_cursor>() {
                        unsafe { unsafe { (*pModule).xClose }.unwrap()(pT) };
                    }
                    if rc == (101 as i32) {
                        rc = 0 as i32;
                    }
                }
                let __v1678: i32 = iCol;
                let __v1679: i32 = __v1678 + (1 as i32);
                iCol = __v1679;
            }
        }
        unsafe { sqlite3_finalize(pStmt) };
    }
    if rc == (11 as i32) | (1 as i32) << (8 as i32) {
        rc = 0 as i32;
        unsafe {
            *pbOk = 0 as i32;
        }
    } else {
        unsafe {
            *pbOk = (rc == (0 as i32) && cksum1 == cksum2) as i32;
        }
    }
    return rc;
}

/// Run the integrity-check. If no error occurs and the current contents of
/// the FTS index are correct, return SQLITE_OK. Or, if the contents of the
/// FTS index are incorrect, return SQLITE_CORRUPT_VTAB.
///
/// Or, if an error (e.g. an OOM or IO error) occurs, return an SQLite
/// error code.
///
/// The integrity-check works as follows. For each token and indexed token
/// prefix in the document set, a 64-bit checksum is calculated (by code
/// in fts3ChecksumEntry()) based on the following:
///
///     + The index number (0 for the main index, 1 for the first prefix
///       index etc.),
///     + The token (or token prefix) text itself,
///     + The language-id of the row it appears in,
///     + The docid of the row it appears in,
///     + The column it appears in, and
///     + The tokens position within that column.
///
/// The checksums for all entries in the index are XORed together to create
/// a single checksum for the entire index.
///
/// The integrity-check code calculates the same checksum in two ways:
///
///     1. By scanning the contents of the FTS index, and
///     2. By scanning and tokenizing the content table.
///
/// If the two checksums are identical, the integrity-check is deemed to have
/// passed.
///
/// # Arguments
///
/// * `p` - FTS3 table handle
fn fts3DoIntegrityCheck(mut p: *mut Fts3Table) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut bOk: i32 = 0 as i32;
    rc = sqlite3Fts3IntegrityCheck(p, std::ptr::addr_of_mut!(bOk));
    if rc == (0 as i32) && bOk == (0 as i32) {
        rc = (11 as i32) | (1 as i32) << (8 as i32);
    }
    return rc;
}

/// Handle a 'special' INSERT of the form:
///
///   "INSERT INTO tbl(tbl) VALUES(<expr>)"
///
/// Argument pVal contains the result of <expr>. Currently the only
/// meaningful value to insert is the text 'optimize'.
fn fts3SpecialInsert(mut p: *mut Fts3Table, mut pVal: *mut sqlite3_value) -> i32 {
    let mut rc: i32 = 1 as i32; // Return Code
    let mut zVal: *const i8 = (unsafe { sqlite3_value_text(pVal) }) as *const i8;
    let mut nVal: i32 = unsafe { sqlite3_value_bytes(pVal) };
    if !(zVal != std::ptr::null::<i8>()) {
        return 7 as i32;
    } else {
        let __v1993: bool;
        if nVal == (8 as i32) {
            __v1993 = (0 as i32)
                == unsafe {
                    sqlite3_strnicmp(
                        zVal,
                        (b"optimize\0".as_ptr() as *mut i8) as *const i8,
                        8 as i32,
                    )
                };
        } else {
            __v1993 = false as bool;
        }
        if __v1993 {
            rc = fts3DoOptimize(p, 0 as i32);
        } else {
            let __v1994: bool;
            if nVal == (7 as i32) {
                __v1994 = (0 as i32)
                    == unsafe {
                        sqlite3_strnicmp(
                            zVal,
                            (b"rebuild\0".as_ptr() as *mut i8) as *const i8,
                            7 as i32,
                        )
                    };
            } else {
                __v1994 = false as bool;
            }
            if __v1994 {
                rc = fts3DoRebuild(p);
            } else {
                let __v1995: bool;
                if nVal == (15 as i32) {
                    __v1995 = (0 as i32)
                        == unsafe {
                            sqlite3_strnicmp(
                                zVal,
                                (b"integrity-check\0".as_ptr() as *mut i8) as *const i8,
                                15 as i32,
                            )
                        };
                } else {
                    __v1995 = false as bool;
                }
                if __v1995 {
                    rc = fts3DoIntegrityCheck(p);
                } else {
                    let __v1996: bool;
                    if nVal > (6 as i32) {
                        __v1996 = (0 as i32)
                            == unsafe {
                                sqlite3_strnicmp(
                                    zVal,
                                    (b"merge=\0".as_ptr() as *mut i8) as *const i8,
                                    6 as i32,
                                )
                            };
                    } else {
                        __v1996 = false as bool;
                    }
                    if __v1996 {
                        rc = fts3DoIncrmerge(p, unsafe { zVal.offset((6 as i32) as isize) });
                    } else {
                        let __v1997: bool;
                        if nVal > (10 as i32) {
                            __v1997 = (0 as i32)
                                == unsafe {
                                    sqlite3_strnicmp(
                                        zVal,
                                        (b"automerge=\0".as_ptr() as *mut i8) as *const i8,
                                        10 as i32,
                                    )
                                };
                        } else {
                            __v1997 = false as bool;
                        }
                        if __v1997 {
                            rc = fts3DoAutoincrmerge(p, unsafe {
                                zVal.offset((10 as i32) as isize)
                            });
                        } else {
                            let __v1998: bool;
                            if nVal == (5 as i32) {
                                __v1998 = (0 as i32)
                                    == unsafe {
                                        sqlite3_strnicmp(
                                            zVal,
                                            (b"flush\0".as_ptr() as *mut i8) as *const i8,
                                            5 as i32,
                                        )
                                    };
                            } else {
                                __v1998 = false as bool;
                            }
                            if __v1998 {
                                rc = sqlite3Fts3PendingTermsFlush(p);
                            }
                        }
                    }
                }
            }
        }
    }
    return rc;
}

/// Delete all cached deferred doclists. Deferred doclists are cached
/// (allocated) by the sqlite3Fts3CacheDeferredDoclists() function.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3FreeDeferredDoclists(mut pCsr: *mut Fts3Cursor) {
    let mut pDef: *mut Fts3DeferredToken = unsafe { std::mem::zeroed() };
    pDef = unsafe { (*pCsr).pDeferred };
    '__slate_break_1606: while pDef != std::ptr::null_mut::<Fts3DeferredToken>() {
        fts3PendingListDelete(unsafe { (*pDef).pList });
        unsafe {
            (*pDef).pList = std::ptr::null_mut::<PendingList>();
        }
        pDef = unsafe { (*pDef).pNext };
    }
}

/// Free all entries in the pCsr->pDeffered list. Entries are added to
/// this list using sqlite3Fts3DeferToken().
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3FreeDeferredTokens(mut pCsr: *mut Fts3Cursor) {
    let mut pDef: *mut Fts3DeferredToken = unsafe { std::mem::zeroed() };
    let mut pNext: *mut Fts3DeferredToken = unsafe { std::mem::zeroed() };
    pDef = unsafe { (*pCsr).pDeferred };
    '__slate_break_1607: while pDef != std::ptr::null_mut::<Fts3DeferredToken>() {
        pNext = unsafe { (*pDef).pNext };
        fts3PendingListDelete(unsafe { (*pDef).pList });
        unsafe { sqlite3_free(pDef as *mut ()) };
        pDef = pNext;
    }
    unsafe {
        (*pCsr).pDeferred = std::ptr::null_mut::<Fts3DeferredToken>();
    }
}

/// Generate deferred-doclists for all tokens in the pCsr->pDeferred list
/// based on the row that pCsr currently points to.
///
/// A deferred-doclist is like any other doclist with position information
/// included, except that it only contains entries for a single row of the
/// table, not for all rows.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3CacheDeferredDoclists(mut pCsr: *mut Fts3Cursor) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    if (unsafe { (*pCsr).pDeferred }) != std::ptr::null_mut::<Fts3DeferredToken>() {
        let mut i: i32 = 0 as i32; // Used to iterate through table columns
        let mut iDocid: i64 = 0 as i64; // Docid of the row pCsr points to
        let mut pDef: *mut Fts3DeferredToken = unsafe { std::mem::zeroed() }; // Used to iterate through deferred tokens
        let mut p: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
        let mut pT: *mut sqlite3_tokenizer = unsafe { (*p).pTokenizer };
        let mut pModule: *const sqlite3_tokenizer_module = unsafe { (*pT).pModule };
        0 as i32;
        iDocid = unsafe { sqlite3_column_int64(unsafe { (*pCsr).pStmt }, 0 as i32) };
        i = 0 as i32;
        '__slate_break_1608: loop {
            if !(i < unsafe { (*p).nColumn } && rc == (0 as i32)) {
                break;
            }
            if (((unsafe { *unsafe { unsafe { (*p).abNotindexed }.offset(i as isize) } }) as u32)
                as i32)
                == (0 as i32)
            {
                let mut zText: *const i8 =
                    (unsafe { sqlite3_column_text(unsafe { (*pCsr).pStmt }, i + (1 as i32)) })
                        as *const i8;
                let mut pTC: *mut sqlite3_tokenizer_cursor =
                    std::ptr::null_mut::<sqlite3_tokenizer_cursor>();
                rc = unsafe {
                    sqlite3Fts3OpenTokenizer(
                        pT,
                        unsafe { (*pCsr).iLangid },
                        zText,
                        -(1 as i32),
                        std::ptr::addr_of_mut!(pTC),
                    )
                };
                '__slate_break_1609: while rc == (0 as i32) {
                    let mut zToken: *const i8 = unsafe { std::mem::zeroed() }; // Buffer containing token
                    let mut nToken: i32 = 0 as i32; // Number of bytes in token
                    let mut iDum1: i32 = 0 as i32;
                    let mut iDum2: i32 = 0 as i32; // Dummy variables
                    let mut iPos: i32 = 0 as i32; // Position of token in zText
                    rc = unsafe {
                        unsafe { (*pModule).xNext }.unwrap()(
                            pTC,
                            std::ptr::addr_of_mut!(zToken),
                            std::ptr::addr_of_mut!(nToken),
                            std::ptr::addr_of_mut!(iDum1),
                            std::ptr::addr_of_mut!(iDum2),
                            std::ptr::addr_of_mut!(iPos),
                        )
                    };
                    pDef = unsafe { (*pCsr).pDeferred };
                    '__slate_break_1610: while pDef != std::ptr::null_mut::<Fts3DeferredToken>()
                        && rc == (0 as i32)
                    {
                        let mut pPT: *mut Fts3PhraseToken = unsafe { (*pDef).pToken };
                        if ((unsafe { (*pDef).iCol }) >= unsafe { (*p).nColumn }
                            || (unsafe { (*pDef).iCol }) == i)
                            && ((unsafe { (*pPT).bFirst }) == (0 as i32) || iPos == (0 as i32))
                            && ((unsafe { (*pPT).n }) == nToken
                                || (unsafe { (*pPT).isPrefix }) != (0 as i32)
                                    && (unsafe { (*pPT).n }) < nToken)
                            && (0 as i32)
                                == unsafe {
                                    memcmp(
                                        zToken as *const (),
                                        (unsafe { (*pPT).z }) as *const (),
                                        ((unsafe { (*pPT).n }) as i64) as u64,
                                    )
                                }
                        {
                            fts3PendingListAppend(
                                unsafe { std::ptr::addr_of_mut!((*pDef).pList) },
                                iDocid,
                                i as i64,
                                iPos as i64,
                                std::ptr::addr_of_mut!(rc),
                            );
                        }
                        pDef = unsafe { (*pDef).pNext };
                    }
                }
                if pTC != std::ptr::null_mut::<sqlite3_tokenizer_cursor>() {
                    unsafe { unsafe { (*pModule).xClose }.unwrap()(pTC) };
                }
                if rc == (101 as i32) {
                    rc = 0 as i32;
                }
            }
            let __v1632: i32 = i;
            let __v1633: i32 = __v1632 + (1 as i32);
            i = __v1633;
        }
        pDef = unsafe { (*pCsr).pDeferred };
        '__slate_break_1611: while pDef != std::ptr::null_mut::<Fts3DeferredToken>()
            && rc == (0 as i32)
        {
            if (unsafe { (*pDef).pList }) != std::ptr::null_mut::<PendingList>() {
                rc = fts3PendingListAppendVarint(
                    unsafe { std::ptr::addr_of_mut!((*pDef).pList) },
                    (0 as i32) as i64,
                );
            }
            pDef = unsafe { (*pDef).pNext };
        }
    }
    return rc;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3DeferredTokenList(
    mut p: *mut Fts3DeferredToken,
    mut ppData: *mut *mut i8,
    mut pnData: *mut i32,
) -> i32 {
    let mut pRet: *mut i8 = unsafe { std::mem::zeroed() };
    let mut nSkip: i32 = 0 as i32;
    let mut dummy: i64 = 0 as i64;
    unsafe {
        *ppData = std::ptr::null_mut::<i8>();
    }
    unsafe {
        *pnData = 0 as i32;
    }
    if (unsafe { (*p).pList }) == std::ptr::null_mut::<PendingList>() {
        return 0 as i32;
    }
    pRet = (unsafe { sqlite3_malloc64((unsafe { (*unsafe { (*p).pList }).nData }) as u64) })
        as *mut i8;
    if !(pRet != std::ptr::null_mut::<i8>()) {
        return 7 as i32;
    }
    nSkip = unsafe {
        sqlite3Fts3GetVarint(
            (unsafe { (*unsafe { (*p).pList }).aData }) as *const i8,
            std::ptr::addr_of_mut!(dummy),
        )
    };
    unsafe {
        *pnData = ((unsafe { (*unsafe { (*p).pList }).nData }) - (nSkip as i64)) as i32;
    }
    unsafe {
        *ppData = pRet;
    }
    unsafe {
        memcpy(
            pRet as *mut (),
            (unsafe { unsafe { (*unsafe { (*p).pList }).aData }.offset(nSkip as isize) })
                as *const (),
            ((unsafe { *pnData }) as i64) as u64,
        )
    };
    return 0 as i32;
}

/// Add an entry for token pToken to the pCsr->pDeferred list.
///
/// # Arguments
///
/// * `pCsr` - Fts3 table cursor
/// * `pToken` - Token to defer
/// * `iCol` - Column that token must appear in (or -1)
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3DeferToken(
    mut pCsr: *mut Fts3Cursor,
    mut pToken: *mut Fts3PhraseToken,
    mut iCol: i32,
) -> i32 {
    let mut pDeferred: *mut Fts3DeferredToken = unsafe { std::mem::zeroed() };
    pDeferred = (unsafe { sqlite3_malloc64(32 as u64) }) as *mut Fts3DeferredToken;
    if !(pDeferred != std::ptr::null_mut::<Fts3DeferredToken>()) {
        return 7 as i32;
    }
    unsafe { memset(pDeferred as *mut (), 0 as i32, 32 as u64) };
    unsafe {
        (*pDeferred).pToken = pToken;
    }
    unsafe {
        (*pDeferred).pNext = unsafe { (*pCsr).pDeferred };
    }
    unsafe {
        (*pDeferred).iCol = iCol;
    }
    unsafe {
        (*pCsr).pDeferred = pDeferred;
    }
    0 as i32;
    unsafe {
        (*pToken).pDeferred = pDeferred;
    }
    return 0 as i32;
}

/// SQLite value pRowid contains the rowid of a row that may or may not be
/// present in the FTS3 table. If it is, delete it and adjust the contents
/// of subsidiary data structures accordingly.
///
/// # Arguments
///
/// * `pnChng` - IN/OUT: Decrement if row is deleted
fn fts3DeleteByRowid(
    mut p: *mut Fts3Table,
    mut pRowid: *mut sqlite3_value,
    mut pnChng: *mut i32,
    mut aSzDel: *mut u32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut bFound: i32 = 0 as i32; // True if *pRowid really is in the table
    fts3DeleteTerms(
        std::ptr::addr_of_mut!(rc),
        p,
        pRowid,
        aSzDel,
        std::ptr::addr_of_mut!(bFound),
    );
    if bFound != (0 as i32) && rc == (0 as i32) {
        let mut isEmpty: i32 = 0 as i32; // Deleting *pRowid leaves the table empty
        rc = fts3IsEmpty(p, pRowid, std::ptr::addr_of_mut!(isEmpty));
        if rc == (0 as i32) {
            if isEmpty != (0 as i32) {
                // Deleting this row means the whole table is empty. In this case
                // delete the contents of all three tables and throw away any
                // data in the pendingTerms hash table.
                rc = fts3DeleteAll(p, 1 as i32);
                unsafe {
                    *pnChng = 0 as i32;
                }
                unsafe {
                    memset(
                        aSzDel as *mut (),
                        0 as i32,
                        (4 as u64)
                            .wrapping_mul((((unsafe { (*p).nColumn }) + (1 as i32)) as i64) as u64)
                            .wrapping_mul(((2 as i32) as i64) as u64),
                    )
                };
            } else {
                unsafe {
                    *pnChng = (unsafe { *pnChng }) - (1 as i32);
                }
                if (unsafe { (*p).zContentTbl }) == std::ptr::null_mut::<i8>() {
                    fts3SqlExec(
                        std::ptr::addr_of_mut!(rc),
                        p,
                        0 as i32,
                        std::ptr::addr_of_mut!(pRowid),
                    );
                }
                if (unsafe { (*p).bHasDocsize }) != (0 as u8) {
                    fts3SqlExec(
                        std::ptr::addr_of_mut!(rc),
                        p,
                        19 as i32,
                        std::ptr::addr_of_mut!(pRowid),
                    );
                }
            }
        }
    }
    return rc;
}

/// This function does the work for the xUpdate method of FTS3 virtual
/// tables. The schema of the virtual table being:
///
///     CREATE TABLE <table name>(
///       <user columns>,
///       <table name> HIDDEN,
///       docid HIDDEN,
///       <langid> HIDDEN
///     );
///
/// # Arguments
///
/// * `pVtab` - FTS3 vtab object
/// * `nArg` - Size of argument array
/// * `apVal` - Array of arguments
/// * `pRowid` - OUT: The affected (or effected) rowid
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3UpdateMethod(
    mut pVtab: *mut sqlite3_vtab,
    mut nArg: i32,
    mut apVal: *mut *mut sqlite3_value,
    mut pRowid: *mut i64,
) -> i32 {
    let mut __slate_storage_1622: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1622: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1622) as *mut i32;
    let mut __slate_storage_1621: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1621: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1621) as *mut i32;
    let mut __slate_storage_1249: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1249: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1249) as *mut i32;
    let mut __slate_storage_1620: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1620: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1620) as *mut bool;
    let mut __slate_storage_1619: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1619: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1619) as *mut bool;
    // Find the value object that holds the new rowid value.
    let mut __slate_storage_1248: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1248: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1248) as *mut *mut sqlite3_value;
    let mut __slate_storage_1618: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1618: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1618) as *mut bool;
    let mut __slate_storage_1617: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1617: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1617) as *mut bool;
    // Check for a "special" INSERT operation. One of the form:
    //
    // INSERT INTO xyz(xyz) VALUES('command');
    let mut __slate_storage_1616: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1616: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1616) as *mut bool;
    let mut __slate_storage_1247: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1247: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1247) as *mut i32; // Net change in number of documents
    let mut __slate_storage_1246: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1246: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1246) as *mut i32; // Sizes of deleted documents
    let mut __slate_storage_1245: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1245: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_1245) as *mut *mut u32; // Sizes of inserted documents
    let mut __slate_storage_1244: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1244: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_1244) as *mut *mut u32; // Return Code
    let mut __slate_storage_1243: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1243: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1243) as *mut i32;
    let mut __slate_storage_1242: std::mem::MaybeUninit<*mut Fts3Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1242: *mut *mut Fts3Table =
        std::ptr::addr_of_mut!(__slate_storage_1242) as *mut *mut Fts3Table;
    unsafe {
        '__join_42: {
            std::ptr::write(__slate_slot_1242, pVtab as *mut Fts3Table);
            std::ptr::write(__slate_slot_1243, 0 as i32);
            std::ptr::write(__slate_slot_1244, std::ptr::null_mut::<u32>());
            std::ptr::write(__slate_slot_1245, std::ptr::null_mut::<u32>());
            std::ptr::write(__slate_slot_1246, 0 as i32);
            std::ptr::write(__slate_slot_1247, 0 as i32);
            // At this point it must be known if the %_stat table exists or not.
            // So bHasStat may not be 2.
            0 as i32;
            0 as i32;
            0 as i32; // DELETE operations
            // INSERT or UPDATE operations
            if nArg > (1 as i32) {
                *__slate_slot_1616 = (unsafe {
                    sqlite3_value_type(unsafe { *unsafe { apVal.offset((0 as i32) as isize) } })
                }) == (5 as i32);
            } else {
                *__slate_slot_1616 = false as bool;
            }
        }
        if *__slate_slot_1616 {
            *__slate_slot_1617 = (unsafe {
                sqlite3_value_type(unsafe {
                    *unsafe {
                        apVal.offset(
                            ((unsafe { (*(*__slate_slot_1242)).nColumn }) + (2 as i32)) as isize,
                        )
                    }
                })
            }) != (5 as i32);
        } else {
            *__slate_slot_1617 = false as bool;
        }
        if *__slate_slot_1617 {
            *__slate_slot_1243 = fts3SpecialInsert(*__slate_slot_1242, unsafe {
                *unsafe {
                    apVal.offset(
                        ((unsafe { (*(*__slate_slot_1242)).nColumn }) + (2 as i32)) as isize,
                    )
                }
            });
        } else {
            if nArg > (1 as i32) {
                *__slate_slot_1618 = (unsafe {
                    sqlite3_value_int(unsafe {
                        *unsafe {
                            apVal.offset(
                                ((2 as i32)
                                    + unsafe { (*(*__slate_slot_1242)).nColumn }
                                    + (2 as i32)) as isize,
                            )
                        }
                    })
                }) < (0 as i32);
            } else {
                *__slate_slot_1618 = false as bool;
            }
            if *__slate_slot_1618 {
                *__slate_slot_1243 = 19 as i32;
            } else {
                // Allocate space to hold the change in document sizes
                *__slate_slot_1245 = (unsafe {
                    sqlite3_malloc64(
                        (4 as u64)
                            .wrapping_mul(
                                (((unsafe { (*(*__slate_slot_1242)).nColumn }) as i64)
                                    + ((1 as i32) as i64)) as u64,
                            )
                            .wrapping_mul(((2 as i32) as i64) as u64),
                    )
                }) as *mut u32;
                if *__slate_slot_1245 == std::ptr::null_mut::<u32>() {
                    *__slate_slot_1243 = 7 as i32;
                } else {
                    *__slate_slot_1244 = unsafe {
                        (*__slate_slot_1245).offset(
                            ((unsafe { (*(*__slate_slot_1242)).nColumn }) + (1 as i32)) as isize,
                        )
                    };
                    unsafe {
                        memset(
                            *__slate_slot_1245 as *mut (),
                            0 as i32,
                            (4 as u64)
                                .wrapping_mul(
                                    (((unsafe { (*(*__slate_slot_1242)).nColumn }) + (1 as i32))
                                        as i64) as u64,
                                )
                                .wrapping_mul(((2 as i32) as i64) as u64),
                        )
                    };
                    *__slate_slot_1243 = fts3Writelock(*__slate_slot_1242);
                    if *__slate_slot_1243 != (0 as i32) {
                    } else {
                        // If this is an INSERT operation, or an UPDATE that modifies the rowid
                        // value, then this operation requires constraint handling.
                        //
                        // If the on-conflict mode is REPLACE, this means that the existing row
                        // should be deleted from the database before inserting the new row. Or,
                        // if the on-conflict mode is other than REPLACE, then this method must
                        // detect the conflict and return SQLITE_CONSTRAINT before beginning to
                        // modify the database file.
                        if nArg > (1 as i32)
                            && (unsafe { (*(*__slate_slot_1242)).zContentTbl })
                                == std::ptr::null_mut::<i8>()
                        {
                            std::ptr::write(__slate_slot_1248, unsafe {
                                *unsafe {
                                    apVal.offset(
                                        ((3 as i32) + unsafe { (*(*__slate_slot_1242)).nColumn })
                                            as isize,
                                    )
                                }
                            });
                            if (unsafe { sqlite3_value_type(*__slate_slot_1248) }) == (5 as i32) {
                                *__slate_slot_1248 =
                                    unsafe { *unsafe { apVal.offset((1 as i32) as isize) } };
                            }
                            if (unsafe { sqlite3_value_type(*__slate_slot_1248) }) != (5 as i32) {
                                if (unsafe {
                                    sqlite3_value_type(unsafe {
                                        *unsafe { apVal.offset((0 as i32) as isize) }
                                    })
                                }) == (5 as i32)
                                {
                                    *__slate_slot_1620 = true as bool;
                                } else {
                                    *__slate_slot_1620 =
                                        (unsafe {
                                            sqlite3_value_int64(unsafe {
                                                *unsafe { apVal.offset((0 as i32) as isize) }
                                            })
                                        }) != unsafe { sqlite3_value_int64(*__slate_slot_1248) };
                                }
                                *__slate_slot_1619 = *__slate_slot_1620;
                            } else {
                                *__slate_slot_1619 = false as bool;
                            }
                            if *__slate_slot_1619 {
                                // The new rowid is not NULL (in this case the rowid will be
                                // automatically assigned and there is no chance of a conflict), and
                                // the statement is either an INSERT or an UPDATE that modifies the
                                // rowid column. So if the conflict mode is REPLACE, then delete any
                                // existing row with rowid=pNewRowid.
                                //
                                // Or, if the conflict mode is not REPLACE, insert the new record into
                                // the %_content table. If we hit the duplicate rowid constraint (or any
                                // other error) while doing so, return immediately.
                                //
                                // This branch may also run if pNewRowid contains a value that cannot
                                // be losslessly converted to an integer. In this case, the eventual
                                // call to fts3InsertData() (either just below or further on in this
                                // function) will return SQLITE_MISMATCH. If fts3DeleteByRowid is
                                // invoked, it will delete zero rows (since no row will have
                                // docid=$pNewRowid if $pNewRowid is not an integer value).
                                if (unsafe {
                                    sqlite3_vtab_on_conflict(unsafe { (*(*__slate_slot_1242)).db })
                                }) == (5 as i32)
                                {
                                    *__slate_slot_1243 = fts3DeleteByRowid(
                                        *__slate_slot_1242,
                                        *__slate_slot_1248,
                                        std::ptr::addr_of_mut!(*__slate_slot_1246),
                                        *__slate_slot_1245,
                                    );
                                } else {
                                    *__slate_slot_1243 =
                                        fts3InsertData(*__slate_slot_1242, apVal, pRowid);
                                    *__slate_slot_1247 = 1 as i32;
                                }
                            }
                        }
                        if *__slate_slot_1243 != (0 as i32) {
                        } else {
                            // If this is a DELETE or UPDATE operation, remove the old record.
                            if (unsafe {
                                sqlite3_value_type(unsafe {
                                    *unsafe { apVal.offset((0 as i32) as isize) }
                                })
                            }) != (5 as i32)
                            {
                                0 as i32;
                                *__slate_slot_1243 = fts3DeleteByRowid(
                                    *__slate_slot_1242,
                                    unsafe { *unsafe { apVal.offset((0 as i32) as isize) } },
                                    std::ptr::addr_of_mut!(*__slate_slot_1246),
                                    *__slate_slot_1245,
                                );
                            }
                            // If this is an INSERT or UPDATE operation, insert the new record.
                            if nArg > (1 as i32) && *__slate_slot_1243 == (0 as i32) {
                                std::ptr::write(__slate_slot_1249, unsafe {
                                    sqlite3_value_int(unsafe {
                                        *unsafe {
                                            apVal.offset(
                                                ((2 as i32)
                                                    + unsafe { (*(*__slate_slot_1242)).nColumn }
                                                    + (2 as i32))
                                                    as isize,
                                            )
                                        }
                                    })
                                });
                                if *__slate_slot_1247 == (0 as i32) {
                                    *__slate_slot_1243 =
                                        fts3InsertData(*__slate_slot_1242, apVal, pRowid);
                                    if *__slate_slot_1243 == (19 as i32)
                                        && (unsafe { (*(*__slate_slot_1242)).zContentTbl })
                                            == std::ptr::null_mut::<i8>()
                                    {
                                        *__slate_slot_1243 = (11 as i32) | (1 as i32) << (8 as i32);
                                    }
                                }
                                if *__slate_slot_1243 == (0 as i32) {
                                    *__slate_slot_1243 = fts3PendingTermsDocid(
                                        *__slate_slot_1242,
                                        0 as i32,
                                        *__slate_slot_1249,
                                        unsafe { *pRowid },
                                    );
                                }
                                if *__slate_slot_1243 == (0 as i32) {
                                    0 as i32;
                                    *__slate_slot_1243 = fts3InsertTerms(
                                        *__slate_slot_1242,
                                        *__slate_slot_1249,
                                        apVal,
                                        *__slate_slot_1244,
                                    );
                                }
                                if (unsafe { (*(*__slate_slot_1242)).bHasDocsize }) != (0 as u8) {
                                    fts3InsertDocsize(
                                        std::ptr::addr_of_mut!(*__slate_slot_1243),
                                        *__slate_slot_1242,
                                        *__slate_slot_1244,
                                    );
                                }
                                std::ptr::write(__slate_slot_1621, *__slate_slot_1246);
                                std::ptr::write(__slate_slot_1622, *__slate_slot_1621 + (1 as i32));
                                *__slate_slot_1246 = *__slate_slot_1622;
                            }
                            if (unsafe { (*(*__slate_slot_1242)).bFts4 }) != (0 as u8) {
                                fts3UpdateDocTotals(
                                    std::ptr::addr_of_mut!(*__slate_slot_1243),
                                    *__slate_slot_1242,
                                    *__slate_slot_1244,
                                    *__slate_slot_1245,
                                    *__slate_slot_1246,
                                );
                            }
                        }
                    }
                }
            }
        }
        unsafe { sqlite3_free(*__slate_slot_1245 as *mut ()) };
        sqlite3Fts3SegmentsClose(*__slate_slot_1242);
        return *__slate_slot_1243;
    }
    return unsafe { std::mem::zeroed() };
}

/// Flush any data in the pending-terms hash table to disk. If successful,
/// merge all segments in the database (including the new segment, if
/// there was any data to flush) into a single segment.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3Optimize(mut p: *mut Fts3Table) -> i32 {
    let mut rc: i32 = 0 as i32;
    rc = unsafe {
        sqlite3_exec(
            unsafe { (*p).db },
            (b"SAVEPOINT fts3\0".as_ptr() as *mut i8) as *const i8,
            None,
            std::ptr::null_mut::<()>(),
            std::ptr::null_mut::<*mut i8>(),
        )
    };
    if rc == (0 as i32) {
        rc = fts3DoOptimize(p, 1 as i32);
        if rc == (0 as i32) || rc == (101 as i32) {
            let mut rc2: i32 = unsafe {
                sqlite3_exec(
                    unsafe { (*p).db },
                    (b"RELEASE fts3\0".as_ptr() as *mut i8) as *const i8,
                    None,
                    std::ptr::null_mut::<()>(),
                    std::ptr::null_mut::<*mut i8>(),
                )
            };
            if rc2 != (0 as i32) {
                rc = rc2;
            }
        } else {
            unsafe {
                sqlite3_exec(
                    unsafe { (*p).db },
                    (b"ROLLBACK TO fts3\0".as_ptr() as *mut i8) as *const i8,
                    None,
                    std::ptr::null_mut::<()>(),
                    std::ptr::null_mut::<*mut i8>(),
                )
            };
            unsafe {
                sqlite3_exec(
                    unsafe { (*p).db },
                    (b"RELEASE fts3\0".as_ptr() as *mut i8) as *const i8,
                    None,
                    std::ptr::null_mut::<()>(),
                    std::ptr::null_mut::<*mut i8>(),
                )
            };
        }
    }
    sqlite3Fts3SegmentsClose(p);
    return rc;
}
