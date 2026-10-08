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
//! This file contains code for implementations of the r-tree and r*-tree
//! algorithms packaged as an SQLite virtual table module.
unsafe extern "C" {
    fn sqlite3_exec(
        __v1039: *mut sqlite3,
        sql: *const i8,
        callback: Option<
            unsafe extern "C-unwind" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32,
        >,
        __v1042: *mut (),
        errmsg: *mut *mut i8,
    ) -> i32;
    fn sqlite3_last_insert_rowid(__v1044: *mut sqlite3) -> i64;
    fn sqlite3_mprintf(__v1045: *const i8, ...) -> *mut i8;
    fn sqlite3_vmprintf(__v1046: *const i8, __v1047: core::ffi::VaList<'_>) -> *mut i8;
    fn sqlite3_malloc(__v1048: i32) -> *mut ();
    fn sqlite3_malloc64(__v1049: u64) -> *mut ();
    fn sqlite3_realloc64(__v1050: *mut (), __v1051: u64) -> *mut ();
    fn sqlite3_free(__v1052: *mut ());
    fn sqlite3_errmsg(__v1053: *mut sqlite3) -> *const i8;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const i8,
        nByte: i32,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const i8,
    ) -> i32;
    fn sqlite3_prepare_v3(
        db: *mut sqlite3,
        zSql: *const i8,
        nByte: i32,
        prepFlags: u32,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const i8,
    ) -> i32;
    fn sqlite3_bind_blob(
        __v1065: *mut sqlite3_stmt,
        __v1066: i32,
        __v1067: *const (),
        n: i32,
        __v1069: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_bind_int64(__v1070: *mut sqlite3_stmt, __v1071: i32, __v1072: i64) -> i32;
    fn sqlite3_bind_null(__v1073: *mut sqlite3_stmt, __v1074: i32) -> i32;
    fn sqlite3_bind_value(
        __v1075: *mut sqlite3_stmt,
        __v1076: i32,
        __v1077: *const sqlite3_value,
    ) -> i32;
    fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_column_name(__v1079: *mut sqlite3_stmt, N: i32) -> *const i8;
    fn sqlite3_step(__v1081: *mut sqlite3_stmt) -> i32;
    fn sqlite3_column_blob(__v1082: *mut sqlite3_stmt, iCol: i32) -> *const ();
    fn sqlite3_column_int(__v1084: *mut sqlite3_stmt, iCol: i32) -> i32;
    fn sqlite3_column_int64(__v1086: *mut sqlite3_stmt, iCol: i32) -> i64;
    fn sqlite3_column_value(__v1088: *mut sqlite3_stmt, iCol: i32) -> *mut sqlite3_value;
    fn sqlite3_column_bytes(__v1090: *mut sqlite3_stmt, iCol: i32) -> i32;
    fn sqlite3_column_type(__v1092: *mut sqlite3_stmt, iCol: i32) -> i32;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_create_function(
        db: *mut sqlite3,
        zFunctionName: *const i8,
        nArg: i32,
        eTextRep: i32,
        pApp: *mut (),
        xFunc: Option<
            unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
        >,
        xStep: Option<
            unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
        >,
        xFinal: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    ) -> i32;
    fn sqlite3_create_function_v2(
        db: *mut sqlite3,
        zFunctionName: *const i8,
        nArg: i32,
        eTextRep: i32,
        pApp: *mut (),
        xFunc: Option<
            unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
        >,
        xStep: Option<
            unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
        >,
        xFinal: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
        xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_value_blob(__v1113: *mut sqlite3_value) -> *const ();
    fn sqlite3_value_double(__v1114: *mut sqlite3_value) -> f64;
    fn sqlite3_value_int(__v1115: *mut sqlite3_value) -> i32;
    fn sqlite3_value_int64(__v1116: *mut sqlite3_value) -> i64;
    fn sqlite3_value_pointer(__v1117: *mut sqlite3_value, __v1118: *const i8) -> *mut ();
    fn sqlite3_value_text(__v1119: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_bytes(__v1120: *mut sqlite3_value) -> i32;
    fn sqlite3_value_type(__v1121: *mut sqlite3_value) -> i32;
    fn sqlite3_value_numeric_type(__v1122: *mut sqlite3_value) -> i32;
    fn sqlite3_value_dup(__v1123: *const sqlite3_value) -> *mut sqlite3_value;
    fn sqlite3_value_free(__v1124: *mut sqlite3_value);
    fn sqlite3_user_data(__v1125: *mut sqlite3_context) -> *mut ();
    fn sqlite3_context_db_handle(__v1126: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_double(__v1127: *mut sqlite3_context, __v1128: f64);
    fn sqlite3_result_error(__v1129: *mut sqlite3_context, __v1130: *const i8, __v1131: i32);
    fn sqlite3_result_error_nomem(__v1132: *mut sqlite3_context);
    fn sqlite3_result_error_code(__v1133: *mut sqlite3_context, __v1134: i32);
    fn sqlite3_result_int(__v1135: *mut sqlite3_context, __v1136: i32);
    fn sqlite3_result_int64(__v1137: *mut sqlite3_context, __v1138: i64);
    fn sqlite3_result_text(
        __v1139: *mut sqlite3_context,
        __v1140: *const i8,
        __v1141: i32,
        __v1142: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_value(__v1143: *mut sqlite3_context, __v1144: *mut sqlite3_value);
    fn sqlite3_result_pointer(
        __v1145: *mut sqlite3_context,
        __v1146: *mut (),
        __v1147: *const i8,
        __v1148: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_table_column_metadata(
        db: *mut sqlite3,
        zDbName: *const i8,
        zTableName: *const i8,
        zColumnName: *const i8,
        pzDataType: *mut *const i8,
        pzCollSeq: *mut *const i8,
        pNotNull: *mut i32,
        pPrimaryKey: *mut i32,
        pAutoinc: *mut i32,
    ) -> i32;
    fn sqlite3_create_module_v2(
        db: *mut sqlite3,
        zName: *const i8,
        p: *const sqlite3_module,
        pClientData: *mut (),
        xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_declare_vtab(__v1163: *mut sqlite3, zSQL: *const i8) -> i32;
    fn sqlite3_blob_open(
        __v1165: *mut sqlite3,
        zDb: *const i8,
        zTable: *const i8,
        zColumn: *const i8,
        iRow: i64,
        flags: i32,
        ppBlob: *mut *mut sqlite3_blob,
    ) -> i32;
    fn sqlite3_blob_reopen(__v1172: *mut sqlite3_blob, __v1173: i64) -> i32;
    fn sqlite3_blob_close(__v1174: *mut sqlite3_blob) -> i32;
    fn sqlite3_blob_bytes(__v1175: *mut sqlite3_blob) -> i32;
    fn sqlite3_blob_read(__v1176: *mut sqlite3_blob, Z: *mut (), N: i32, iOffset: i32) -> i32;
    fn sqlite3_str_new(__v1180: *mut sqlite3) -> *mut sqlite3_str;
    fn sqlite3_str_finish(__v1181: *mut sqlite3_str) -> *mut i8;
    fn sqlite3_str_appendf(__v1182: *mut sqlite3_str, zFormat: *const i8, ...);
    fn sqlite3_str_append(__v1184: *mut sqlite3_str, zIn: *const i8, N: i32);
    fn sqlite3_str_errcode(__v1187: *mut sqlite3_str) -> i32;
    fn sqlite3_stricmp(__v1188: *const i8, __v1189: *const i8) -> i32;
    fn sqlite3_vtab_config(__v1190: *mut sqlite3, op: i32, ...) -> i32;
    fn sqlite3_vtab_on_conflict(__v1192: *mut sqlite3) -> i32;
    fn sqlite3GetToken(__v1202: *const u8, __v1203: *mut i32) -> i64;
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memmove(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3IntFloatCompare(__v1243: i64, __v1244: f64) -> i32;
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
struct sqlite3_str {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_rtree_geometry {
    pContext: *mut (),
    nParam: i32,
    aParam: *mut f64,
    pUser: *mut (),
    xDelUser: Option<unsafe extern "C-unwind" fn(*mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_rtree_query_info {
    pContext: *mut (),
    nParam: i32,
    aParam: *mut f64,
    pUser: *mut (),
    xDelUser: Option<unsafe extern "C-unwind" fn(*mut ())>,
    aCoord: *mut f64,
    anQueue: *mut u32,
    nCoord: i32,
    iLevel: i32,
    mxLevel: i32,
    iRowid: i64,
    rParentScore: f64,
    eParentWithin: i32,
    eWithin: i32,
    rScore: f64,
    apSqlParam: *mut *mut sqlite3_value,
}

// Database Format of R-Tree Tables
//
// The data structure for a single virtual r-tree table is stored in three
// native SQLite tables declared as follows. In each case, the '%' character
// in the table name is replaced with the user-supplied name of the r-tree
// table.
//
//   CREATE TABLE %_node(nodeno INTEGER PRIMARY KEY, data BLOB)
//   CREATE TABLE %_parent(nodeno INTEGER PRIMARY KEY, parentnode INTEGER)
//   CREATE TABLE %_rowid(rowid INTEGER PRIMARY KEY, nodeno INTEGER, ...)
//
// The data for each node of the r-tree structure is stored in the %_node
// table. For each node that is not the root node of the r-tree, there is
// an entry in the %_parent table associating the node with its parent.
// And for each row of data in the table, there is an entry in the %_rowid
// table that maps from the entries rowid to the id of the node that it
// is stored on.  If the r-tree contains auxiliary columns, those are stored
// on the end of the %_rowid table.
//
// The root node of an r-tree always exists, even if the r-tree table is
// empty. The nodeno of the root node is always 1. All other nodes in the
// table must be the same size as the root node. The content of each node
// is formatted as follows:
//
//   1. If the node is the root node (node 1), then the first 2 bytes
//      of the node contain the tree depth as a big-endian integer.
//      For non-root nodes, the first 2 bytes are left unused.
//
//   2. The next 2 bytes contain the number of entries currently
//      stored in the node.
//
//   3. The remainder of the node contains the node entries. Each entry
//      consists of a single 8-byte integer followed by an even number
//      of 4-byte coordinates. For leaf nodes the integer is the rowid
//      of a record. For internal nodes it is the node number of a
//      child page.
// If building separately, we will need some setup that is normally
// found in sqliteInt.h
// Macro to check for 4-byte alignment.  Only used inside of assert()
//  The following macro is used to suppress compiler warnings.
// The rtree may have between 1 and RTREE_MAX_DIMENSIONS dimensions.
// Maximum number of auxiliary columns
// Size of hash table Rtree.aHash. This hash table is not expected to
// ever contain very many entries, so a fixed number of buckets is
// used.
// The xBestIndex method of this virtual table requires an estimate of
// the number of rows in the virtual table to calculate the costs of
// various strategies. If possible, this estimate is loaded from the
// sqlite_stat1 table (with RTREE_MIN_ROWEST as a hard-coded minimum).
// Otherwise, if no sqlite_stat1 entry is available, use
// RTREE_DEFAULT_ROWEST.
// In SQLite core
/// An rtree virtual-table object.
#[repr(C)]
#[derive(Clone, Copy)]
struct Rtree {
    /// Base class.  Must be first
    base: sqlite3_vtab,
    /// Host database connection
    db: *mut sqlite3,
    /// Size in bytes of each node in the node table
    iNodeSize: i32,
    /// Number of dimensions
    nDim: u8,
    /// Twice the number of dimensions
    nDim2: u8,
    /// RTREE_COORD_REAL32 or RTREE_COORD_INT32
    eCoordType: u8,
    /// Bytes consumed per cell
    nBytesPerCell: u8,
    /// True if inside write transaction
    inWrTrans: u8,
    /// # of auxiliary columns in %_rowid
    nAux: u16,
    /// Current depth of the r-tree structure
    iDepth: i32,
    /// Name of database containing r-tree table
    zDb: *mut i8,
    /// Name of r-tree table
    zName: *mut i8,
    /// Name of the %_node table
    zNodeName: *mut i8,
    /// Current number of users of this structure
    nBusy: u32,
    /// Estimated number of rows in this table
    nRowEst: i64,
    /// Number of open cursors
    nCursor: u32,
    /// Number RtreeNodes with positive nRef
    nNodeRef: u32,
    /// SQL for statement to read aux data
    zReadAuxSql: *mut i8,
    /// List of nodes removed during a CondenseTree operation. List is
    /// linked together via the pointer normally used for hash chains -
    /// RtreeNode.pNext. RtreeNode.iNode stores the depth of the sub-tree
    /// headed by the node (leaf nodes have RtreeNode.iNode==0).
    pDeleted: *mut RtreeNode,
    /// Blob I/O on xxx_node
    pNodeBlob: *mut sqlite3_blob,
    /// Statements to read/write/delete a record from xxx_node
    pWriteNode: *mut sqlite3_stmt,
    pDeleteNode: *mut sqlite3_stmt,
    /// Statements to read/write/delete a record from xxx_rowid
    pReadRowid: *mut sqlite3_stmt,
    pWriteRowid: *mut sqlite3_stmt,
    pDeleteRowid: *mut sqlite3_stmt,
    /// Statements to read/write/delete a record from xxx_parent
    pReadParent: *mut sqlite3_stmt,
    pWriteParent: *mut sqlite3_stmt,
    pDeleteParent: *mut sqlite3_stmt,
    /// Statement for writing to the "aux:" fields, if there are any
    pWriteAux: *mut sqlite3_stmt,
    /// Hash table of in-memory nodes.
    aHash: [*mut RtreeNode; 97],
}

// Possible values for Rtree.eCoordType:
// High accuracy coordinate
// Low accuracy coordinate
// Set the Rtree.bCorrupt flag
/// If SQLITE_RTREE_INT_ONLY is defined, then this virtual table will
/// only deal with integer coordinates.  No floating point operations
/// will be done.
/// When doing a search of an r-tree, instances of the following structure
/// record intermediate results from the tree walk.
///
/// The id is always a node-id.  For iLevel>=1 the id is the node-id of
/// the node that the RtreeSearchPoint represents.  When iLevel==0, however,
/// the id is of the parent node and the cell that RtreeSearchPoint
/// represents is the iCell-th entry in the parent node.
#[repr(C)]
#[derive(Clone, Copy)]
struct RtreeSearchPoint {
    /// The score for this node.  Smallest goes first.
    rScore: f64,
    /// Node ID
    id: i64,
    /// 0=entries.  1=leaf node.  2+ for higher
    iLevel: u8,
    /// PARTLY_WITHIN or FULLY_WITHIN
    eWithin: u8,
    /// Cell index within the node
    iCell: u8,
}

// The minimum number of cells allowed for a node is a third of the
// maximum. In Gutman's notation:
//
//     m = M/3
//
// If an R*-tree "Reinsert" operation is required, the same number of
// cells are removed from the overfull node and reinserted into the tree.
// The smallest possible node-size is (512-64)==448 bytes. And the largest
// supported cell size is 48 bytes (8 byte rowid + ten 4 byte coordinates).
// Therefore all non-root nodes must contain at least 3 entries. Since
// 3^40 is greater than 2^64, an r-tree structure always has a depth of
// 40 or less.
// Number of entries in the cursor RtreeNode cache.  The first entry is
// used to cache the RtreeNode for RtreeCursor.sPoint.  The remaining
// entries cache the RtreeNode for the first elements of the priority queue.
/// An rtree cursor object.
#[repr(C)]
#[derive(Clone, Copy)]
struct RtreeCursor {
    /// Base class.  Must be first
    base: sqlite3_vtab_cursor,
    /// True if at end of search
    atEOF: u8,
    /// True if sPoint is valid
    bPoint: u8,
    /// True if pReadAux is valid
    bAuxValid: u8,
    /// Copy of idxNum search parameter
    iStrategy: i32,
    /// Number of entries in aConstraint
    nConstraint: i32,
    /// Search constraints.
    aConstraint: *mut RtreeConstraint,
    /// Number of slots allocated for aPoint[]
    nPointAlloc: i32,
    /// Number of slots used in aPoint[]
    nPoint: i32,
    /// iLevel value for root of the tree
    mxLevel: i32,
    /// Priority queue for search points
    aPoint: *mut RtreeSearchPoint,
    /// Statement to read aux-data
    pReadAux: *mut sqlite3_stmt,
    /// Cached next search point
    sPoint: RtreeSearchPoint,
    /// Rtree node cache
    aNode: [*mut RtreeNode; 5],
    /// Number of queued entries by iLevel
    anQueue: [u32; 42],
}

// Return the Rtree of a RtreeCursor
/// A coordinate can be either a floating point number or a integer.  All
/// coordinates within a single R-Tree are always of the same time.
#[repr(C)]
#[derive(Clone, Copy)]
union RtreeCoord {
    /// Floating point value
    f: f32,
    /// Integer value
    i: i32,
    /// Unsigned for byte-order conversions
    u: u32,
}

// The argument is an RtreeCoord. Return the value stored within the RtreeCoord
// formatted as a RtreeDValue (double or int64). This macro assumes that local
// variable pRtree points to the Rtree structure associated with the
// RtreeCoord.
/// A search constraint.
#[repr(C)]
#[derive(Clone, Copy)]
struct RtreeConstraint {
    /// Index of constrained coordinate
    iCoord: i32,
    /// Constraining operation
    op: i32,
    u: __SlateRecord62,
    /// xGeom and xQueryFunc argument
    pInfo: *mut sqlite3_rtree_query_info,
}

// Possible values for RtreeConstraint.op
// A
// B
// C
// D
// E
// F: Old-style sqlite3_rtree_geometry_callback()
// G: New-style sqlite3_rtree_query_callback()
// Special operators available only on cursors.  Needs to be consecutive
// with the normal values above, but must be less than RTREE_MATCH.  These
// are used in the cursor for contraints such as x=NULL (RTREE_FALSE) or
// x<'xyz' (RTREE_TRUE)
// ?
// @
/// An rtree structure node.
#[repr(C)]
#[derive(Clone, Copy)]
struct RtreeNode {
    /// Parent node
    pParent: *mut RtreeNode,
    /// The node number
    iNode: i64,
    /// Number of references to this node
    nRef: i32,
    /// True if the node needs to be written to disk
    isDirty: i32,
    /// Content of the node, as should be on disk
    zData: *mut u8,
    /// Next node in this hash collision chain
    pNext: *mut RtreeNode,
}

// Return the number of cells in a node
/// A single cell from a node, deserialized
#[repr(C)]
#[derive(Clone, Copy)]
struct RtreeCell {
    /// Node or entry ID
    iRowid: i64,
    /// Bounding box coordinates
    aCoord: [RtreeCoord; 10],
}

/// This object becomes the sqlite3_user_data() for the SQL functions
/// that are created by sqlite3_rtree_geometry_callback() and
/// sqlite3_rtree_query_callback() and which appear on the right of MATCH
/// operators in order to constrain a search.
///
/// xGeom and xQueryFunc are the callback functions.  Exactly one of
/// xGeom and xQueryFunc fields is non-NULL, depending on whether the
/// SQL function was created using sqlite3_rtree_geometry_callback() or
/// sqlite3_rtree_query_callback().
///
/// This object is deleted automatically by the destructor mechanism in
/// sqlite3_create_function_v2().
#[repr(C)]
#[derive(Clone, Copy)]
struct RtreeGeomCallback {
    xGeom: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_rtree_geometry, i32, *mut f64, *mut i32) -> i32,
    >,
    xQueryFunc: Option<unsafe extern "C-unwind" fn(*mut sqlite3_rtree_query_info) -> i32>,
    xDestructor: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pContext: *mut (),
}

/// An instance of this structure (in the form of a BLOB) is returned by
/// the SQL functions that sqlite3_rtree_geometry_callback() and
/// sqlite3_rtree_query_callback() create, and is read as the right-hand
/// operand to the MATCH operator of an R-Tree.
#[repr(C)]
#[derive(Clone, Copy)]
struct RtreeMatchArg {
    /// Size of this object
    iSize: u32,
    /// Info about the callback functions
    cb: RtreeGeomCallback,
    /// Number of parameters to the SQL function
    nParam: i32,
    /// Original SQL parameter values
    apSqlParam: *mut *mut sqlite3_value,
    /// Values for parameters to the SQL function
    aParam: [f64; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord62 {
    /// Constraint value.
    rValue: f64,
    xGeom: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_rtree_geometry, i32, *mut f64, *mut i32) -> i32,
    >,
    xQueryFunc: Option<unsafe extern "C-unwind" fn(*mut sqlite3_rtree_query_info) -> i32>,
}

// Size of an RtreeMatchArg object with N parameters
// What version of GCC is being used.  0 means GCC is not being used .
// Note that the GCC_VERSION macro will also be set correctly when using
// clang, since clang works hard to be gcc compatible.  So the gcc
// optimizations will also work when compiling with clang.
// Make sure that the compiler intrinsics we desire are enabled when
// compiling with an appropriate version of MSVC unless prevented by
// the SQLITE_DISABLE_INTRINSIC define.
// Macros to determine whether the machine is big or little endian,
// and whether or not that determination is run-time or compile-time.
//
// For best performance, an attempt is made to guess at the byte-order
// using C-preprocessor macros.  If that is unsuccessful, or if
// -DSQLITE_BYTEORDER=0 is set, then byte-order is determined
// at run-time.
// What version of MSVC is being used.  0 means MSVC is not being used
/// The testcase() macro should already be defined in the amalgamation.  If
/// it is not, make it a no-op.
/// Functions to deserialize a 16 bit integer, 32 bit real number and
/// 64 bit integer. The deserialized value is returned.
fn readInt16(mut p: *mut u8) -> i32 {
    return ((((unsafe { *unsafe { p.offset((0 as i32) as isize) } }) as u32) as i32)
        << (8 as i32))
        + (((unsafe { *unsafe { p.offset((1 as i32) as isize) } }) as u32) as i32);
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

fn readCoord(mut p: *mut u8, mut pCoord: *mut RtreeCoord) {
    0 as i32;
    unsafe {
        (*pCoord).u = (((unsafe { *unsafe { p.offset((0 as i32) as isize) } }) as u32)
            << (24 as i32))
            .wrapping_add(
                ((unsafe { *unsafe { p.offset((1 as i32) as isize) } }) as u32) << (16 as i32),
            )
            .wrapping_add(
                ((unsafe { *unsafe { p.offset((2 as i32) as isize) } }) as u32) << (8 as i32),
            )
            .wrapping_add(
                ((unsafe { *unsafe { p.offset((3 as i32) as isize) } }) as u32) << (0 as i32),
            );
    }
}

fn readInt64(mut p: *mut u8) -> i64 {
    return (((unsafe { *unsafe { p.offset((0 as i32) as isize) } }) as u64) << (56 as i32))
        .wrapping_add(
            ((unsafe { *unsafe { p.offset((1 as i32) as isize) } }) as u64) << (48 as i32),
        )
        .wrapping_add(
            ((unsafe { *unsafe { p.offset((2 as i32) as isize) } }) as u64) << (40 as i32),
        )
        .wrapping_add(
            ((unsafe { *unsafe { p.offset((3 as i32) as isize) } }) as u64) << (32 as i32),
        )
        .wrapping_add(
            ((unsafe { *unsafe { p.offset((4 as i32) as isize) } }) as u64) << (24 as i32),
        )
        .wrapping_add(
            ((unsafe { *unsafe { p.offset((5 as i32) as isize) } }) as u64) << (16 as i32),
        )
        .wrapping_add(((unsafe { *unsafe { p.offset((6 as i32) as isize) } }) as u64) << (8 as i32))
        .wrapping_add(((unsafe { *unsafe { p.offset((7 as i32) as isize) } }) as u64) << (0 as i32))
        as i64;
}

/// Functions to serialize a 16 bit integer, 32 bit real number and
/// 64 bit integer. The value returned is the number of bytes written
/// to the argument buffer (always 2, 4 and 8 respectively).
fn writeInt16(mut p: *mut u8, mut i: i32) {
    unsafe {
        *unsafe { p.offset((0 as i32) as isize) } = ((i >> (8 as i32) & (255 as i32)) as i8) as u8;
    }
    unsafe {
        *unsafe { p.offset((1 as i32) as isize) } = ((i >> (0 as i32) & (255 as i32)) as i8) as u8;
    }
}

fn writeCoord(mut p: *mut u8, mut pCoord: *mut RtreeCoord) -> i32 {
    let mut i: u32 = 0 as u32;
    0 as i32;
    0 as i32;
    0 as i32;
    i = unsafe { (*pCoord).u };
    unsafe {
        *unsafe { p.offset((0 as i32) as isize) } =
            (i >> (24 as i32) & ((255 as i32) as u32)) as u8;
    }
    unsafe {
        *unsafe { p.offset((1 as i32) as isize) } =
            (i >> (16 as i32) & ((255 as i32) as u32)) as u8;
    }
    unsafe {
        *unsafe { p.offset((2 as i32) as isize) } = (i >> (8 as i32) & ((255 as i32) as u32)) as u8;
    }
    unsafe {
        *unsafe { p.offset((3 as i32) as isize) } = (i >> (0 as i32) & ((255 as i32) as u32)) as u8;
    }
    return 4 as i32;
}

fn writeInt64(mut p: *mut u8, mut i: i64) -> i32 {
    unsafe {
        *unsafe { p.offset((0 as i32) as isize) } =
            ((i >> (56 as i32) & ((255 as i32) as i64)) as i8) as u8;
    }
    unsafe {
        *unsafe { p.offset((1 as i32) as isize) } =
            ((i >> (48 as i32) & ((255 as i32) as i64)) as i8) as u8;
    }
    unsafe {
        *unsafe { p.offset((2 as i32) as isize) } =
            ((i >> (40 as i32) & ((255 as i32) as i64)) as i8) as u8;
    }
    unsafe {
        *unsafe { p.offset((3 as i32) as isize) } =
            ((i >> (32 as i32) & ((255 as i32) as i64)) as i8) as u8;
    }
    unsafe {
        *unsafe { p.offset((4 as i32) as isize) } =
            ((i >> (24 as i32) & ((255 as i32) as i64)) as i8) as u8;
    }
    unsafe {
        *unsafe { p.offset((5 as i32) as isize) } =
            ((i >> (16 as i32) & ((255 as i32) as i64)) as i8) as u8;
    }
    unsafe {
        *unsafe { p.offset((6 as i32) as isize) } =
            ((i >> (8 as i32) & ((255 as i32) as i64)) as i8) as u8;
    }
    unsafe {
        *unsafe { p.offset((7 as i32) as isize) } =
            ((i >> (0 as i32) & ((255 as i32) as i64)) as i8) as u8;
    }
    return 8 as i32;
}

/// Increment the reference count of node p.
fn nodeReference(mut p: *mut RtreeNode) {
    if p != std::ptr::null_mut::<RtreeNode>() {
        0 as i32;
        let __v1394: *mut RtreeNode = p;
        let __v1395: i32 = unsafe { (*__v1394).nRef };
        let __v1396: i32 = __v1395 + (1 as i32);
        unsafe {
            (*__v1394).nRef = __v1396;
        }
    }
}

/// Clear the content of node p (set all bytes to 0x00).
fn nodeZero(mut pRtree: *mut Rtree, mut p: *mut RtreeNode) {
    unsafe {
        memset(
            (unsafe { unsafe { (*p).zData }.offset((2 as i32) as isize) }) as *mut (),
            0 as i32,
            (((unsafe { (*pRtree).iNodeSize }) - (2 as i32)) as i64) as u64,
        )
    };
    unsafe {
        (*p).isDirty = 1 as i32;
    }
}

/// Given a node number iNode, return the corresponding key to use
/// in the Rtree.aHash table.
fn nodeHash(mut iNode: i64) -> u32 {
    return ((iNode as i32) as u32) % ((97 as i32) as u32);
}

/// Search the node hash table for node iNode. If found, return a pointer
/// to it. Otherwise, return 0.
fn nodeHashLookup(mut pRtree: *mut Rtree, mut iNode: i64) -> *mut RtreeNode {
    let mut p: *mut RtreeNode = unsafe { std::mem::zeroed() };
    p = unsafe {
        *unsafe {
            unsafe { (*pRtree).aHash.as_mut_ptr() as *mut *mut RtreeNode }
                .offset(nodeHash(iNode) as isize)
        }
    };
    '__slate_break_1214: while p != std::ptr::null_mut::<RtreeNode>()
        && (unsafe { (*p).iNode }) != iNode
    {
        {}
        p = unsafe { (*p).pNext };
    }
    return p;
}

/// Add node pNode to the node hash table.
fn nodeHashInsert(mut pRtree: *mut Rtree, mut pNode: *mut RtreeNode) {
    let mut iHash: i32 = 0 as i32;
    0 as i32;
    iHash = nodeHash(unsafe { (*pNode).iNode }) as i32;
    unsafe {
        (*pNode).pNext = unsafe {
            *unsafe {
                unsafe { (*pRtree).aHash.as_mut_ptr() as *mut *mut RtreeNode }
                    .offset(iHash as isize)
            }
        };
    }
    unsafe {
        *unsafe {
            unsafe { (*pRtree).aHash.as_mut_ptr() as *mut *mut RtreeNode }.offset(iHash as isize)
        } = pNode;
    }
}

/// Remove node pNode from the node hash table.
fn nodeHashDelete(mut pRtree: *mut Rtree, mut pNode: *mut RtreeNode) {
    let mut pp: *mut *mut RtreeNode = unsafe { std::mem::zeroed() };
    if (unsafe { (*pNode).iNode }) != ((0 as i32) as i64) {
        pp = unsafe {
            unsafe { (*pRtree).aHash.as_mut_ptr() as *mut *mut RtreeNode }
                .offset(nodeHash(unsafe { (*pNode).iNode }) as isize)
        };
        '__slate_break_1215: while (unsafe { *pp }) != pNode {
            0 as i32;
            pp = unsafe { std::ptr::addr_of_mut!((*unsafe { *pp }).pNext) };
        }
        unsafe {
            *pp = unsafe { (*pNode).pNext };
        }
        unsafe {
            (*pNode).pNext = std::ptr::null_mut::<RtreeNode>();
        }
    }
}

/// Allocate and return new r-tree node. Initially, (RtreeNode.iNode==0),
/// indicating that node has not yet been assigned a node number. It is
/// assigned a node number when nodeWrite() is called to write the
/// node contents out to the database.
fn nodeNew(mut pRtree: *mut Rtree, mut pParent: *mut RtreeNode) -> *mut RtreeNode {
    let mut pNode: *mut RtreeNode = unsafe { std::mem::zeroed() };
    pNode = (unsafe {
        sqlite3_malloc64((40 as u64).wrapping_add(((unsafe { (*pRtree).iNodeSize }) as i64) as u64))
    }) as *mut RtreeNode;
    if pNode != std::ptr::null_mut::<RtreeNode>() {
        unsafe {
            memset(
                pNode as *mut (),
                0 as i32,
                (40 as u64).wrapping_add(((unsafe { (*pRtree).iNodeSize }) as i64) as u64),
            )
        };
        unsafe {
            (*pNode).zData = (unsafe { pNode.offset((1 as i32) as isize) }) as *mut u8;
        }
        unsafe {
            (*pNode).nRef = 1 as i32;
        }
        let __v1397: *mut Rtree = pRtree;
        let __v1398: u32 = unsafe { (*__v1397).nNodeRef };
        let __v1399: u32 = __v1398.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v1397).nNodeRef = __v1399;
        }
        unsafe {
            (*pNode).pParent = pParent;
        }
        unsafe {
            (*pNode).isDirty = 1 as i32;
        }
        nodeReference(pParent);
    }
    return pNode;
}

/// Clear the Rtree.pNodeBlob object
fn nodeBlobReset(mut pRtree: *mut Rtree) {
    let mut pBlob: *mut sqlite3_blob = unsafe { (*pRtree).pNodeBlob };
    unsafe {
        (*pRtree).pNodeBlob = std::ptr::null_mut::<sqlite3_blob>();
    }
    unsafe { sqlite3_blob_close(pBlob) };
}

/// Obtain a reference to an r-tree node.
///
/// # Arguments
///
/// * `pRtree` - R-tree structure
/// * `iNode` - Node number to load
/// * `pParent` - Either the parent node or NULL
/// * `ppNode` - OUT: Acquired node
fn nodeAcquire(
    mut pRtree: *mut Rtree,
    mut iNode: i64,
    mut pParent: *mut RtreeNode,
    mut ppNode: *mut *mut RtreeNode,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pNode: *mut RtreeNode = std::ptr::null_mut::<RtreeNode>();
    // Check if the requested node is already in the hash table. If so,
    // increase its reference count and return it.
    let __v1400: *mut RtreeNode = nodeHashLookup(pRtree, iNode);
    pNode = __v1400;
    if __v1400 != std::ptr::null_mut::<RtreeNode>() {
        if pParent != std::ptr::null_mut::<RtreeNode>() && pParent != unsafe { (*pNode).pParent } {
            {}
            return (11 as i32) | (1 as i32) << (8 as i32);
        }
        let __v1401: *mut RtreeNode = pNode;
        let __v1402: i32 = unsafe { (*__v1401).nRef };
        let __v1403: i32 = __v1402 + (1 as i32);
        unsafe {
            (*__v1401).nRef = __v1403;
        }
        unsafe {
            *ppNode = pNode;
        }
        return 0 as i32;
    }
    if (unsafe { (*pRtree).pNodeBlob }) != std::ptr::null_mut::<sqlite3_blob>() {
        let mut pBlob: *mut sqlite3_blob = unsafe { (*pRtree).pNodeBlob };
        unsafe {
            (*pRtree).pNodeBlob = std::ptr::null_mut::<sqlite3_blob>();
        }
        rc = unsafe { sqlite3_blob_reopen(pBlob, iNode) };
        unsafe {
            (*pRtree).pNodeBlob = pBlob;
        }
        if rc != (0 as i32) {
            nodeBlobReset(pRtree);
            if rc == (7 as i32) {
                return 7 as i32;
            }
        }
    }
    if (unsafe { (*pRtree).pNodeBlob }) == std::ptr::null_mut::<sqlite3_blob>() {
        rc = unsafe {
            sqlite3_blob_open(
                unsafe { (*pRtree).db },
                (unsafe { (*pRtree).zDb }) as *const i8,
                (unsafe { (*pRtree).zNodeName }) as *const i8,
                (b"data\0".as_ptr() as *mut i8) as *const i8,
                iNode,
                0 as i32,
                unsafe { std::ptr::addr_of_mut!((*pRtree).pNodeBlob) },
            )
        };
    }
    if rc != (0 as i32) {
        unsafe {
            *ppNode = std::ptr::null_mut::<RtreeNode>();
        }
        // If unable to open an sqlite3_blob on the desired row, that can only
        // be because the shadow tables hold erroneous data.
        if rc == (1 as i32) {
            rc = (11 as i32) | (1 as i32) << (8 as i32);
            {}
        }
    } else {
        if iNode <= ((0 as i32) as i64) {
            {}
            rc = (11 as i32) | (1 as i32) << (8 as i32);
        } else {
            if (unsafe { (*pRtree).iNodeSize })
                == unsafe { sqlite3_blob_bytes(unsafe { (*pRtree).pNodeBlob }) }
            {
                pNode = (unsafe {
                    sqlite3_malloc64(
                        (40 as u64).wrapping_add(((unsafe { (*pRtree).iNodeSize }) as i64) as u64),
                    )
                }) as *mut RtreeNode;
                if !(pNode != std::ptr::null_mut::<RtreeNode>()) {
                    rc = 7 as i32;
                } else {
                    unsafe {
                        (*pNode).pParent = pParent;
                    }
                    unsafe {
                        (*pNode).zData = (unsafe { pNode.offset((1 as i32) as isize) }) as *mut u8;
                    }
                    unsafe {
                        (*pNode).nRef = 1 as i32;
                    }
                    let __v1404: *mut Rtree = pRtree;
                    let __v1405: u32 = unsafe { (*__v1404).nNodeRef };
                    let __v1406: u32 = __v1405.wrapping_add((1 as i32) as u32);
                    unsafe {
                        (*__v1404).nNodeRef = __v1406;
                    }
                    unsafe {
                        (*pNode).iNode = iNode;
                    }
                    unsafe {
                        (*pNode).isDirty = 0 as i32;
                    }
                    unsafe {
                        (*pNode).pNext = std::ptr::null_mut::<RtreeNode>();
                    }
                    rc = unsafe {
                        sqlite3_blob_read(
                            unsafe { (*pRtree).pNodeBlob },
                            (unsafe { (*pNode).zData }) as *mut (),
                            unsafe { (*pRtree).iNodeSize },
                            0 as i32,
                        )
                    };
                }
            }
        }
    }
    // If the root node was just loaded, set pRtree->iDepth to the height
    // of the r-tree structure. A height of zero means all data is stored on
    // the root node. A height of one means the children of the root node
    // are the leaves, and so on. If the depth as specified on the root node
    // is greater than RTREE_MAX_DEPTH, the r-tree structure must be corrupt.
    if rc == (0 as i32)
        && pNode != std::ptr::null_mut::<RtreeNode>()
        && iNode == ((1 as i32) as i64)
    {
        unsafe {
            (*pRtree).iDepth = readInt16(unsafe { (*pNode).zData });
        }
        if (unsafe { (*pRtree).iDepth }) >= (40 as i32) {
            rc = (11 as i32) | (1 as i32) << (8 as i32);
            {}
        }
    }
    // If no error has occurred so far, check if the "number of entries"
    // field on the node is too large. If so, set the return code to
    // SQLITE_CORRUPT_VTAB.
    if pNode != std::ptr::null_mut::<RtreeNode>() && rc == (0 as i32) {
        if readInt16(unsafe { unsafe { (*pNode).zData }.offset((2 as i32) as isize) })
            > ((unsafe { (*pRtree).iNodeSize }) - (4 as i32))
                / (((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32)
        {
            rc = (11 as i32) | (1 as i32) << (8 as i32);
            {}
        }
    }
    if rc == (0 as i32) {
        if pNode != std::ptr::null_mut::<RtreeNode>() {
            nodeReference(pParent);
            nodeHashInsert(pRtree, pNode);
        } else {
            rc = (11 as i32) | (1 as i32) << (8 as i32);
            {}
        }
        unsafe {
            *ppNode = pNode;
        }
    } else {
        nodeBlobReset(pRtree);
        if pNode != std::ptr::null_mut::<RtreeNode>() {
            let __v1407: *mut Rtree = pRtree;
            let __v1408: u32 = unsafe { (*__v1407).nNodeRef };
            let __v1409: u32 = __v1408.wrapping_sub((1 as i32) as u32);
            unsafe {
                (*__v1407).nNodeRef = __v1409;
            }
            unsafe { sqlite3_free(pNode as *mut ()) };
        }
        unsafe {
            *ppNode = std::ptr::null_mut::<RtreeNode>();
        }
    }
    return rc;
}

/// Overwrite cell iCell of node pNode with the contents of pCell.
///
/// # Arguments
///
/// * `pRtree` - The overall R-Tree
/// * `pNode` - The node into which the cell is to be written
/// * `pCell` - The cell to write
/// * `iCell` - Index into pNode into which pCell is written
fn nodeOverwriteCell(
    mut pRtree: *mut Rtree,
    mut pNode: *mut RtreeNode,
    mut pCell: *mut RtreeCell,
    mut iCell: i32,
) {
    let mut ii: i32 = 0 as i32;
    let mut p: *mut u8 = unsafe {
        unsafe { (*pNode).zData }.offset(
            ((4 as i32) + (((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32) * iCell) as isize,
        )
    };
    let __v1410: *mut u8 = p;
    let __v1411: *mut u8 =
        unsafe { __v1410.offset(writeInt64(p, unsafe { (*pCell).iRowid }) as isize) };
    p = __v1411;
    ii = 0 as i32;
    '__slate_break_1217: loop {
        if !(ii < (((unsafe { (*pRtree).nDim2 }) as u32) as i32)) {
            break;
        }
        let __v1414: *mut u8 = p;
        let __v1415: *mut u8 = unsafe {
            __v1414.offset(writeCoord(p, unsafe {
                unsafe { (*pCell).aCoord.as_mut_ptr() as *mut RtreeCoord }.offset(ii as isize)
            }) as isize)
        };
        p = __v1415;
        let __v1412: i32 = ii;
        let __v1413: i32 = __v1412 + (1 as i32);
        ii = __v1413;
    }
    unsafe {
        (*pNode).isDirty = 1 as i32;
    }
}

/// Remove the cell with index iCell from node pNode.
fn nodeDeleteCell(mut pRtree: *mut Rtree, mut pNode: *mut RtreeNode, mut iCell: i32) {
    let mut pDst: *mut u8 = unsafe {
        unsafe { (*pNode).zData }.offset(
            ((4 as i32) + (((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32) * iCell) as isize,
        )
    };
    let mut pSrc: *mut u8 =
        unsafe { pDst.offset((((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32) as isize) };
    let mut nByte: i32 =
        (readInt16(unsafe { unsafe { (*pNode).zData }.offset((2 as i32) as isize) })
            - iCell
            - (1 as i32))
            * (((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32);
    unsafe { memmove(pDst as *mut (), pSrc as *const (), (nByte as i64) as u64) };
    writeInt16(
        unsafe { unsafe { (*pNode).zData }.offset((2 as i32) as isize) },
        readInt16(unsafe { unsafe { (*pNode).zData }.offset((2 as i32) as isize) }) - (1 as i32),
    );
    unsafe {
        (*pNode).isDirty = 1 as i32;
    }
}

/// Insert the contents of cell pCell into node pNode. If the insert
/// is successful, return SQLITE_OK.
///
/// If there is not enough free space in pNode, return SQLITE_FULL.
///
/// # Arguments
///
/// * `pRtree` - The overall R-Tree
/// * `pNode` - Write new cell into this node
/// * `pCell` - The cell to be inserted
fn nodeInsertCell(
    mut pRtree: *mut Rtree,
    mut pNode: *mut RtreeNode,
    mut pCell: *mut RtreeCell,
) -> i32 {
    let mut nCell: i32 = 0 as i32; // Current number of cells in pNode
    let mut nMaxCell: i32 = 0 as i32; // Maximum number of cells for pNode
    nMaxCell = ((unsafe { (*pRtree).iNodeSize }) - (4 as i32))
        / (((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32);
    nCell = readInt16(unsafe { unsafe { (*pNode).zData }.offset((2 as i32) as isize) });
    0 as i32;
    if nCell < nMaxCell {
        nodeOverwriteCell(pRtree, pNode, pCell, nCell);
        writeInt16(
            unsafe { unsafe { (*pNode).zData }.offset((2 as i32) as isize) },
            nCell + (1 as i32),
        );
        unsafe {
            (*pNode).isDirty = 1 as i32;
        }
    }
    return (nCell == nMaxCell) as i32;
}

/// If the node is dirty, write it out to the database.
fn nodeWrite(mut pRtree: *mut Rtree, mut pNode: *mut RtreeNode) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pNode).isDirty }) != (0 as i32) {
        let mut p: *mut sqlite3_stmt = unsafe { (*pRtree).pWriteNode };
        if (unsafe { (*pNode).iNode }) != (0 as i64) {
            unsafe { sqlite3_bind_int64(p, 1 as i32, unsafe { (*pNode).iNode }) };
        } else {
            unsafe { sqlite3_bind_null(p, 1 as i32) };
        }
        unsafe {
            sqlite3_bind_blob(
                p,
                2 as i32,
                (unsafe { (*pNode).zData }) as *const (),
                unsafe { (*pRtree).iNodeSize },
                None,
            )
        };
        unsafe { sqlite3_step(p) };
        unsafe {
            (*pNode).isDirty = 0 as i32;
        }
        rc = unsafe { sqlite3_reset(p) };
        unsafe { sqlite3_bind_null(p, 2 as i32) };
        if (unsafe { (*pNode).iNode }) == ((0 as i32) as i64) && rc == (0 as i32) {
            unsafe {
                (*pNode).iNode = unsafe { sqlite3_last_insert_rowid(unsafe { (*pRtree).db }) };
            }
            nodeHashInsert(pRtree, pNode);
        }
    }
    return rc;
}

/// Release a reference to a node. If the node is dirty and the reference
/// count drops to zero, the node data is written to the database.
fn nodeRelease(mut pRtree: *mut Rtree, mut pNode: *mut RtreeNode) -> i32 {
    let mut rc: i32 = 0 as i32;
    if pNode != std::ptr::null_mut::<RtreeNode>() {
        0 as i32;
        0 as i32;
        let __v1416: *mut RtreeNode = pNode;
        let __v1417: i32 = unsafe { (*__v1416).nRef };
        let __v1418: i32 = __v1417 - (1 as i32);
        unsafe {
            (*__v1416).nRef = __v1418;
        }
        if (unsafe { (*pNode).nRef }) == (0 as i32) {
            let __v1419: *mut Rtree = pRtree;
            let __v1420: u32 = unsafe { (*__v1419).nNodeRef };
            let __v1421: u32 = __v1420.wrapping_sub((1 as i32) as u32);
            unsafe {
                (*__v1419).nNodeRef = __v1421;
            }
            if (unsafe { (*pNode).iNode }) == ((1 as i32) as i64) {
                unsafe {
                    (*pRtree).iDepth = -(1 as i32);
                }
            }
            if (unsafe { (*pNode).pParent }) != std::ptr::null_mut::<RtreeNode>() {
                rc = nodeRelease(pRtree, unsafe { (*pNode).pParent });
            }
            if rc == (0 as i32) {
                rc = nodeWrite(pRtree, pNode);
            }
            nodeHashDelete(pRtree, pNode);
            unsafe { sqlite3_free(pNode as *mut ()) };
        }
    }
    return rc;
}

/// Return the 64-bit integer value associated with cell iCell of
/// node pNode. If pNode is a leaf node, this is a rowid. If it is
/// an internal node, then the 64-bit integer is a child page number.
///
/// # Arguments
///
/// * `pRtree` - The overall R-Tree
/// * `pNode` - The node from which to extract the ID
/// * `iCell` - The cell index from which to extract the ID
fn nodeGetRowid(mut pRtree: *mut Rtree, mut pNode: *mut RtreeNode, mut iCell: i32) -> i64 {
    0 as i32;
    return readInt64(unsafe {
        unsafe { (*pNode).zData }.offset(
            ((4 as i32) + (((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32) * iCell) as isize,
        )
    });
}

/// Return coordinate iCoord from cell iCell in node pNode.
///
/// # Arguments
///
/// * `pRtree` - The overall R-Tree
/// * `pNode` - The node from which to extract a coordinate
/// * `iCell` - The index of the cell within the node
/// * `iCoord` - Which coordinate to extract
/// * `pCoord` - OUT: Space to write result to
fn nodeGetCoord(
    mut pRtree: *mut Rtree,
    mut pNode: *mut RtreeNode,
    mut iCell: i32,
    mut iCoord: i32,
    mut pCoord: *mut RtreeCoord,
) {
    0 as i32;
    readCoord(
        unsafe {
            unsafe { (*pNode).zData }.offset(
                ((12 as i32)
                    + (((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32) * iCell
                    + (4 as i32) * iCoord) as isize,
            )
        },
        pCoord,
    );
}

/// Deserialize cell iCell of node pNode. Populate the structure pointed
/// to by pCell with the results.
///
/// # Arguments
///
/// * `pRtree` - The overall R-Tree
/// * `pNode` - The node containing the cell to be read
/// * `iCell` - Index of the cell within the node
/// * `pCell` - OUT: Write the cell contents here
fn nodeGetCell(
    mut pRtree: *mut Rtree,
    mut pNode: *mut RtreeNode,
    mut iCell: i32,
    mut pCell: *mut RtreeCell,
) {
    let mut pData: *mut u8 = unsafe { std::mem::zeroed() };
    let mut pCoord: *mut RtreeCoord = unsafe { std::mem::zeroed() };
    let mut ii: i32 = 0 as i32;
    unsafe {
        (*pCell).iRowid = nodeGetRowid(pRtree, pNode, iCell);
    }
    pData = unsafe {
        unsafe { (*pNode).zData }.offset(
            ((12 as i32) + (((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32) * iCell) as isize,
        )
    };
    pCoord = unsafe { (*pCell).aCoord.as_mut_ptr() as *mut RtreeCoord };
    '__slate_break_1218: loop {
        readCoord(pData, unsafe { pCoord.offset(ii as isize) });
        readCoord(unsafe { pData.offset((4 as i32) as isize) }, unsafe {
            pCoord.offset((ii + (1 as i32)) as isize)
        });
        let __v1422: *mut u8 = pData;
        let __v1423: *mut u8 = unsafe { __v1422.offset((8 as i32) as isize) };
        pData = __v1423;
        let __v1424: i32 = ii;
        let __v1425: i32 = __v1424 + (2 as i32);
        ii = __v1425;
        if !(ii < (((unsafe { (*pRtree).nDim2 }) as u32) as i32)) {
            break;
        }
    }
}

/// Rtree virtual table module xCreate method.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeCreate")]
extern "C-unwind" fn rtreeCreate(
    mut db: *mut sqlite3,
    mut pAux: *mut (),
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut i8,
) -> i32 {
    return rtreeInit(db, pAux, argc, argv, ppVtab, pzErr, 1 as i32);
}

/// Rtree virtual table module xConnect method.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeConnect")]
extern "C-unwind" fn rtreeConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut (),
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut i8,
) -> i32 {
    return rtreeInit(db, pAux, argc, argv, ppVtab, pzErr, 0 as i32);
}

/// Increment the r-tree reference count.
fn rtreeReference(mut pRtree: *mut Rtree) {
    let __v1435: *mut Rtree = pRtree;
    let __v1436: u32 = unsafe { (*__v1435).nBusy };
    let __v1437: u32 = __v1436.wrapping_add((1 as i32) as u32);
    unsafe {
        (*__v1435).nBusy = __v1437;
    }
}

/// Decrement the r-tree reference count. When the reference count reaches
/// zero the structure is deleted.
fn rtreeRelease(mut pRtree: *mut Rtree) {
    let __v1438: *mut Rtree = pRtree;
    let __v1439: u32 = unsafe { (*__v1438).nBusy };
    let __v1440: u32 = __v1439.wrapping_sub((1 as i32) as u32);
    unsafe {
        (*__v1438).nBusy = __v1440;
    }
    if (unsafe { (*pRtree).nBusy }) == ((0 as i32) as u32) {
        unsafe {
            (*pRtree).inWrTrans = ((0 as i32) as i8) as u8;
        }
        0 as i32;
        nodeBlobReset(pRtree);
        if (unsafe { (*pRtree).nNodeRef }) != (0 as u32) {
            let mut i: i32 = 0 as i32;
            0 as i32;
            i = 0 as i32;
            '__slate_break_1226: loop {
                if !(i < (97 as i32)) {
                    break;
                }
                '__slate_break_1227: while (unsafe {
                    *unsafe {
                        unsafe { (*pRtree).aHash.as_mut_ptr() as *mut *mut RtreeNode }
                            .offset(i as isize)
                    }
                }) != std::ptr::null_mut::<RtreeNode>()
                {
                    let mut pNext: *mut RtreeNode = unsafe {
                        (*unsafe {
                            *unsafe {
                                unsafe { (*pRtree).aHash.as_mut_ptr() as *mut *mut RtreeNode }
                                    .offset(i as isize)
                            }
                        })
                        .pNext
                    };
                    unsafe {
                        sqlite3_free(
                            (unsafe {
                                *unsafe {
                                    unsafe { (*pRtree).aHash.as_mut_ptr() as *mut *mut RtreeNode }
                                        .offset(i as isize)
                                }
                            }) as *mut (),
                        )
                    };
                    unsafe {
                        *unsafe {
                            unsafe { (*pRtree).aHash.as_mut_ptr() as *mut *mut RtreeNode }
                                .offset(i as isize)
                        } = pNext;
                    }
                }
                let __v1441: i32 = i;
                let __v1442: i32 = __v1441 + (1 as i32);
                i = __v1442;
            }
        }
        unsafe { sqlite3_finalize(unsafe { (*pRtree).pWriteNode }) };
        unsafe { sqlite3_finalize(unsafe { (*pRtree).pDeleteNode }) };
        unsafe { sqlite3_finalize(unsafe { (*pRtree).pReadRowid }) };
        unsafe { sqlite3_finalize(unsafe { (*pRtree).pWriteRowid }) };
        unsafe { sqlite3_finalize(unsafe { (*pRtree).pDeleteRowid }) };
        unsafe { sqlite3_finalize(unsafe { (*pRtree).pReadParent }) };
        unsafe { sqlite3_finalize(unsafe { (*pRtree).pWriteParent }) };
        unsafe { sqlite3_finalize(unsafe { (*pRtree).pDeleteParent }) };
        unsafe { sqlite3_finalize(unsafe { (*pRtree).pWriteAux }) };
        unsafe { sqlite3_free((unsafe { (*pRtree).zReadAuxSql }) as *mut ()) };
        unsafe { sqlite3_free(pRtree as *mut ()) };
    }
}

/// Rtree virtual table module xDisconnect method.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeDisconnect")]
extern "C-unwind" fn rtreeDisconnect(mut pVtab: *mut sqlite3_vtab) -> i32 {
    rtreeRelease(pVtab as *mut Rtree);
    return 0 as i32;
}

/// Rtree virtual table module xDestroy method.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeDestroy")]
extern "C-unwind" fn rtreeDestroy(mut pVtab: *mut sqlite3_vtab) -> i32 {
    let mut pRtree: *mut Rtree = pVtab as *mut Rtree;
    let mut rc: i32 = 0 as i32;
    let mut zCreate: *mut i8 = unsafe {
        sqlite3_mprintf(
            (b"DROP TABLE '%q'.'%q_node';DROP TABLE '%q'.'%q_rowid';DROP TABLE '%q'.'%q_parent';\0"
                .as_ptr() as *mut i8) as *const i8,
            unsafe { (*pRtree).zDb },
            unsafe { (*pRtree).zName },
            unsafe { (*pRtree).zDb },
            unsafe { (*pRtree).zName },
            unsafe { (*pRtree).zDb },
            unsafe { (*pRtree).zName },
        )
    };
    if !(zCreate != std::ptr::null_mut::<i8>()) {
        rc = 7 as i32;
    } else {
        nodeBlobReset(pRtree);
        rc = unsafe {
            sqlite3_exec(
                unsafe { (*pRtree).db },
                zCreate as *const i8,
                None,
                std::ptr::null_mut::<()>(),
                std::ptr::null_mut::<*mut i8>(),
            )
        };
        unsafe { sqlite3_free(zCreate as *mut ()) };
    }
    if rc == (0 as i32) {
        rtreeRelease(pRtree);
    }
    return rc;
}

/// Rtree virtual table module xOpen method.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeOpen")]
extern "C-unwind" fn rtreeOpen(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> i32 {
    let mut rc: i32 = 7 as i32;
    let mut pRtree: *mut Rtree = pVTab as *mut Rtree;
    let mut pCsr: *mut RtreeCursor = unsafe { std::mem::zeroed() };
    pCsr = (unsafe { sqlite3_malloc64(296 as u64) }) as *mut RtreeCursor;
    if pCsr != std::ptr::null_mut::<RtreeCursor>() {
        unsafe { memset(pCsr as *mut (), 0 as i32, 296 as u64) };
        unsafe {
            (*pCsr).base.pVtab = pVTab;
        }
        rc = 0 as i32;
        let __v1443: *mut Rtree = pRtree;
        let __v1444: u32 = unsafe { (*__v1443).nCursor };
        let __v1445: u32 = __v1444.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v1443).nCursor = __v1445;
        }
    }
    unsafe {
        *ppCursor = pCsr as *mut sqlite3_vtab_cursor;
    }
    return rc;
}

/// Reset a cursor back to its initial state.
fn resetCursor(mut pCsr: *mut RtreeCursor) {
    let mut pRtree: *mut Rtree = (unsafe { (*pCsr).base.pVtab }) as *mut Rtree;
    let mut ii: i32 = 0 as i32;
    let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    if (unsafe { (*pCsr).aConstraint }) != std::ptr::null_mut::<RtreeConstraint>() {
        let mut i: i32 = 0 as i32; // Used to iterate through constraint array
        i = 0 as i32;
        '__slate_break_1229: loop {
            if !(i < unsafe { (*pCsr).nConstraint }) {
                break;
            }
            let mut pInfo: *mut sqlite3_rtree_query_info =
                unsafe { (*unsafe { unsafe { (*pCsr).aConstraint }.offset(i as isize) }).pInfo };
            if pInfo != std::ptr::null_mut::<sqlite3_rtree_query_info>() {
                if (unsafe { (*pInfo).xDelUser }) != None {
                    unsafe { unsafe { (*pInfo).xDelUser }.unwrap()(unsafe { (*pInfo).pUser }) };
                }
                unsafe { sqlite3_free(pInfo as *mut ()) };
            }
            let __v1446: i32 = i;
            let __v1447: i32 = __v1446 + (1 as i32);
            i = __v1447;
        }
        unsafe { sqlite3_free((unsafe { (*pCsr).aConstraint }) as *mut ()) };
        unsafe {
            (*pCsr).aConstraint = std::ptr::null_mut::<RtreeConstraint>();
        }
    }
    ii = 0 as i32;
    '__slate_break_1230: loop {
        if !(ii < (5 as i32)) {
            break;
        }
        nodeRelease(pRtree, unsafe {
            *unsafe {
                unsafe { (*pCsr).aNode.as_mut_ptr() as *mut *mut RtreeNode }.offset(ii as isize)
            }
        });
        let __v1448: i32 = ii;
        let __v1449: i32 = __v1448 + (1 as i32);
        ii = __v1449;
    }
    unsafe { sqlite3_free((unsafe { (*pCsr).aPoint }) as *mut ()) };
    pStmt = unsafe { (*pCsr).pReadAux };
    unsafe { memset(pCsr as *mut (), 0 as i32, 296 as u64) };
    unsafe {
        (*pCsr).base.pVtab = pRtree as *mut sqlite3_vtab;
    }
    unsafe {
        (*pCsr).pReadAux = pStmt;
    }
    // The following will only fail if the previous sqlite3_step() call failed,
    // in which case the error has already been caught. This statement never
    // encounters an error within an sqlite3_column_xxx() function, as it
    // calls sqlite3_column_value(), which does not use malloc(). So it is safe
    // to ignore the error code here.
    unsafe { sqlite3_reset(pStmt) };
}

/// Rtree virtual table module xClose method.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeClose")]
extern "C-unwind" fn rtreeClose(mut cur: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pRtree: *mut Rtree = (unsafe { (*cur).pVtab }) as *mut Rtree;
    let mut pCsr: *mut RtreeCursor = cur as *mut RtreeCursor;
    0 as i32;
    resetCursor(pCsr);
    unsafe { sqlite3_finalize(unsafe { (*pCsr).pReadAux }) };
    unsafe { sqlite3_free(pCsr as *mut ()) };
    let __v1450: *mut Rtree = pRtree;
    let __v1451: u32 = unsafe { (*__v1450).nCursor };
    let __v1452: u32 = __v1451.wrapping_sub((1 as i32) as u32);
    unsafe {
        (*__v1450).nCursor = __v1452;
    }
    if (unsafe { (*pRtree).nCursor }) == ((0 as i32) as u32)
        && (((unsafe { (*pRtree).inWrTrans }) as u32) as i32) == (0 as i32)
    {
        nodeBlobReset(pRtree);
    }
    return 0 as i32;
}

/// Rtree virtual table module xEof method.
///
/// Return non-zero if the cursor does not currently point to a valid
/// record (i.e if the scan has finished), or zero otherwise.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeEof")]
extern "C-unwind" fn rtreeEof(mut cur: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut RtreeCursor = cur as *mut RtreeCursor;
    return ((unsafe { (*pCsr).atEOF }) as u32) as i32;
}

// Convert raw bits from the on-disk RTree record into a coordinate value.
// The on-disk format is big-endian and needs to be converted for little-
// endian platforms.  The on-disk record stores integer coordinates if
// eInt is true and it stores 32-bit floating point records if eInt is
// false.  a[] is the four bytes of the on-disk record to be decoded.
// Store the results in "r".
//
// There are five versions of this macro.  The last one is generic.  The
// other four are various architectures-specific optimizations.
// Coordinate decoded
/// Check the RTree node or entry given by pCellData and p against the MATCH
/// constraint pConstraint.
///
/// # Arguments
///
/// * `pConstraint` - The constraint to test
/// * `eInt` - True if RTree holding integer coordinates
/// * `pCellData` - Raw cell content
/// * `pSearch` - Container of this cell
/// * `prScore` - OUT: score for the cell
/// * `peWithin` - OUT: visibility of the cell
fn rtreeCallbackConstraint(
    mut pConstraint: *mut RtreeConstraint,
    mut eInt: i32,
    mut pCellData: *mut u8,
    mut pSearch: *mut RtreeSearchPoint,
    mut prScore: *mut f64,
    mut peWithin: *mut i32,
) -> i32 {
    let mut pInfo: *mut sqlite3_rtree_query_info = unsafe { (*pConstraint).pInfo }; // Callback info
    let mut nCoord: i32 = unsafe { (*pInfo).nCoord }; // No. of coordinates
    let mut rc: i32 = 0 as i32; // Callback return code
    let mut c: RtreeCoord = unsafe { std::mem::zeroed() }; // Translator union
    let mut aCoord: __SlateAlign16<[f64; 10]> = __SlateAlign16([0 as f64; 10]); // Decoded coordinates
    0 as i32;
    0 as i32;
    if (unsafe { (*pConstraint).op }) == (71 as i32)
        && (((unsafe { (*pSearch).iLevel }) as u32) as i32) == (1 as i32)
    {
        unsafe {
            (*pInfo).iRowid = readInt64(pCellData);
        }
    }
    let __v1453: *mut u8 = pCellData;
    let __v1454: *mut u8 = unsafe { __v1453.offset((8 as i32) as isize) };
    pCellData = __v1454;
    if eInt == (0 as i32) {
        let mut __t0: i64 = match nCoord {
            10 => 0,
            8 => 1,
            6 => 2,
            4 => 3,
            _ => 4,
        };
        '__slate_break_1231: loop {
            match __t0 {
                0 => {
                    readCoord(
                        unsafe { pCellData.offset((36 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((9 as i32) as isize)
                        } = (unsafe { c.f }) as f64;
                    }
                    readCoord(
                        unsafe { pCellData.offset((32 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((8 as i32) as isize)
                        } = (unsafe { c.f }) as f64;
                    }
                    __t0 = 1;
                    continue '__slate_break_1231;
                }
                1 => {
                    readCoord(
                        unsafe { pCellData.offset((28 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((7 as i32) as isize)
                        } = (unsafe { c.f }) as f64;
                    }
                    readCoord(
                        unsafe { pCellData.offset((24 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((6 as i32) as isize)
                        } = (unsafe { c.f }) as f64;
                    }
                    __t0 = 2;
                    continue '__slate_break_1231;
                }
                2 => {
                    readCoord(
                        unsafe { pCellData.offset((20 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((5 as i32) as isize)
                        } = (unsafe { c.f }) as f64;
                    }
                    readCoord(
                        unsafe { pCellData.offset((16 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((4 as i32) as isize)
                        } = (unsafe { c.f }) as f64;
                    }
                    __t0 = 3;
                    continue '__slate_break_1231;
                }
                3 => {
                    readCoord(
                        unsafe { pCellData.offset((12 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((3 as i32) as isize)
                        } = (unsafe { c.f }) as f64;
                    }
                    readCoord(
                        unsafe { pCellData.offset((8 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((2 as i32) as isize)
                        } = (unsafe { c.f }) as f64;
                    }
                    __t0 = 4;
                    continue '__slate_break_1231;
                }
                4 => {
                    readCoord(
                        unsafe { pCellData.offset((4 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((1 as i32) as isize)
                        } = (unsafe { c.f }) as f64;
                    }
                    readCoord(pCellData, std::ptr::addr_of_mut!(c));
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((0 as i32) as isize)
                        } = (unsafe { c.f }) as f64;
                    }
                    break '__slate_break_1231;
                }
                _ => {
                    break '__slate_break_1231;
                }
            }
        }
    } else {
        let mut __t1: i64 = match nCoord {
            10 => 0,
            8 => 1,
            6 => 2,
            4 => 3,
            _ => 4,
        };
        '__slate_break_1232: loop {
            match __t1 {
                0 => {
                    readCoord(
                        unsafe { pCellData.offset((36 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((9 as i32) as isize)
                        } = (unsafe { c.i }) as f64;
                    }
                    readCoord(
                        unsafe { pCellData.offset((32 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((8 as i32) as isize)
                        } = (unsafe { c.i }) as f64;
                    }
                    __t1 = 1;
                    continue '__slate_break_1232;
                }
                1 => {
                    readCoord(
                        unsafe { pCellData.offset((28 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((7 as i32) as isize)
                        } = (unsafe { c.i }) as f64;
                    }
                    readCoord(
                        unsafe { pCellData.offset((24 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((6 as i32) as isize)
                        } = (unsafe { c.i }) as f64;
                    }
                    __t1 = 2;
                    continue '__slate_break_1232;
                }
                2 => {
                    readCoord(
                        unsafe { pCellData.offset((20 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((5 as i32) as isize)
                        } = (unsafe { c.i }) as f64;
                    }
                    readCoord(
                        unsafe { pCellData.offset((16 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((4 as i32) as isize)
                        } = (unsafe { c.i }) as f64;
                    }
                    __t1 = 3;
                    continue '__slate_break_1232;
                }
                3 => {
                    readCoord(
                        unsafe { pCellData.offset((12 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((3 as i32) as isize)
                        } = (unsafe { c.i }) as f64;
                    }
                    readCoord(
                        unsafe { pCellData.offset((8 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((2 as i32) as isize)
                        } = (unsafe { c.i }) as f64;
                    }
                    __t1 = 4;
                    continue '__slate_break_1232;
                }
                4 => {
                    readCoord(
                        unsafe { pCellData.offset((4 as i32) as isize) },
                        std::ptr::addr_of_mut!(c),
                    );
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((1 as i32) as isize)
                        } = (unsafe { c.i }) as f64;
                    }
                    readCoord(pCellData, std::ptr::addr_of_mut!(c));
                    unsafe {
                        *unsafe {
                            (aCoord.0.as_mut_ptr() as *mut f64).offset((0 as i32) as isize)
                        } = (unsafe { c.i }) as f64;
                    }
                    break '__slate_break_1232;
                }
                _ => {
                    break '__slate_break_1232;
                }
            }
        }
    }
    if (unsafe { (*pConstraint).op }) == (70 as i32) {
        let mut eWithin: i32 = 0 as i32;
        rc = unsafe {
            unsafe { (*pConstraint).u.xGeom }.unwrap()(
                pInfo as *mut sqlite3_rtree_geometry,
                nCoord,
                aCoord.0.as_mut_ptr() as *mut f64,
                std::ptr::addr_of_mut!(eWithin),
            )
        };
        if eWithin == (0 as i32) {
            unsafe {
                *peWithin = 0 as i32;
            }
        }
        unsafe {
            *prScore = 0.0f64;
        }
    } else {
        unsafe {
            (*pInfo).aCoord = aCoord.0.as_mut_ptr() as *mut f64;
        }
        unsafe {
            (*pInfo).iLevel = (((unsafe { (*pSearch).iLevel }) as u32) as i32) - (1 as i32);
        }
        let __v1455: f64 = unsafe { (*pSearch).rScore };
        unsafe {
            (*pInfo).rParentScore = __v1455;
        }
        unsafe {
            (*pInfo).rScore = __v1455;
        }
        let __v1456: i32 = ((unsafe { (*pSearch).eWithin }) as u32) as i32;
        unsafe {
            (*pInfo).eParentWithin = __v1456;
        }
        unsafe {
            (*pInfo).eWithin = __v1456;
        }
        rc = unsafe { unsafe { (*pConstraint).u.xQueryFunc }.unwrap()(pInfo) };
        if (unsafe { (*pInfo).eWithin }) < unsafe { *peWithin } {
            unsafe {
                *peWithin = unsafe { (*pInfo).eWithin };
            }
        }
        if (unsafe { (*pInfo).rScore }) < unsafe { *prScore } || (unsafe { *prScore }) < 0.0f64 {
            unsafe {
                *prScore = unsafe { (*pInfo).rScore };
            }
        }
    }
    return rc;
}

/// Check the internal RTree node given by pCellData against constraint p.
/// If this constraint cannot be satisfied by any child within the node,
/// set *peWithin to NOT_WITHIN.
///
/// # Arguments
///
/// * `p` - The constraint to test
/// * `eInt` - True if RTree holds integer coordinates
/// * `pCellData` - Raw cell content as appears on disk
/// * `peWithin` - Adjust downward, as appropriate
fn rtreeNonleafConstraint(
    mut p: *mut RtreeConstraint,
    mut eInt: i32,
    mut pCellData: *mut u8,
    mut peWithin: *mut i32,
) {
    let mut val: f64 = 0 as f64; // Coordinate value convert to a double
    // p->iCoord might point to either a lower or upper bound coordinate
    // in a coordinate pair.  But make pCellData point to the lower bound.
    let __v1457: *mut u8 = pCellData;
    let __v1458: *mut u8 = unsafe {
        __v1457
            .offset(((8 as i32) + (4 as i32) * ((unsafe { (*p).iCoord }) & (254 as i32))) as isize)
    };
    pCellData = __v1458;
    0 as i32;
    0 as i32;
    '__slate_break_1233: {
        match unsafe { (*p).op } {
            63 => {
                return; // Always satisfied
            }
            64 => {
                break '__slate_break_1233; // Never satisfied
            }
            65 => {
                {
                    let mut c: RtreeCoord = unsafe { std::mem::zeroed() };
                    unsafe {
                        memcpy(
                            (unsafe { std::ptr::addr_of_mut!(c.u) }) as *mut (),
                            pCellData as *const (),
                            ((4 as i32) as i64) as u64,
                        )
                    };
                    unsafe {
                        c.u = (unsafe { c.u }) >> (24 as i32) & ((255 as i32) as u32)
                            | (unsafe { c.u }) >> (8 as i32) & ((65280 as i32) as u32)
                            | ((unsafe { c.u }) & ((255 as i32) as u32)) << (24 as i32)
                            | ((unsafe { c.u }) & ((65280 as i32) as u32)) << (8 as i32);
                    }
                    val = if eInt != (0 as i32) {
                        (unsafe { c.i }) as f64
                    } else {
                        (unsafe { c.f }) as f64
                    };
                }
                {}
                // val now holds the lower bound of the coordinate pair
                if (unsafe { (*p).u.rValue }) >= val {
                    let __v1459: *mut u8 = pCellData;
                    let __v1460: *mut u8 = unsafe { __v1459.offset((4 as i32) as isize) };
                    pCellData = __v1460;
                    let mut c: RtreeCoord = unsafe { std::mem::zeroed() };
                    unsafe {
                        memcpy(
                            (unsafe { std::ptr::addr_of_mut!(c.u) }) as *mut (),
                            pCellData as *const (),
                            ((4 as i32) as i64) as u64,
                        )
                    };
                    unsafe {
                        c.u = (unsafe { c.u }) >> (24 as i32) & ((255 as i32) as u32)
                            | (unsafe { c.u }) >> (8 as i32) & ((65280 as i32) as u32)
                            | ((unsafe { c.u }) & ((255 as i32) as u32)) << (24 as i32)
                            | ((unsafe { c.u }) & ((65280 as i32) as u32)) << (8 as i32);
                    }
                    val = if eInt != (0 as i32) {
                        (unsafe { c.i }) as f64
                    } else {
                        (unsafe { c.f }) as f64
                    };
                    {}
                    // val now holds the upper bound of the coordinate pair
                    if (unsafe { (*p).u.rValue }) <= val {
                        return;
                    }
                }
            }
            66 | 67 => {
                let mut c: RtreeCoord = unsafe { std::mem::zeroed() };
                unsafe {
                    memcpy(
                        (unsafe { std::ptr::addr_of_mut!(c.u) }) as *mut (),
                        pCellData as *const (),
                        ((4 as i32) as i64) as u64,
                    )
                };
                unsafe {
                    c.u = (unsafe { c.u }) >> (24 as i32) & ((255 as i32) as u32)
                        | (unsafe { c.u }) >> (8 as i32) & ((65280 as i32) as u32)
                        | ((unsafe { c.u }) & ((255 as i32) as u32)) << (24 as i32)
                        | ((unsafe { c.u }) & ((65280 as i32) as u32)) << (8 as i32);
                }
                val = if eInt != (0 as i32) {
                    (unsafe { c.i }) as f64
                } else {
                    (unsafe { c.f }) as f64
                };
                {}
                // val now holds the lower bound of the coordinate pair
                if (unsafe { (*p).u.rValue }) >= val {
                    return;
                }
            }
            _ => {
                let __v1461: *mut u8 = pCellData;
                let __v1462: *mut u8 = unsafe { __v1461.offset((4 as i32) as isize) };
                pCellData = __v1462;
                let mut c: RtreeCoord = unsafe { std::mem::zeroed() };
                unsafe {
                    memcpy(
                        (unsafe { std::ptr::addr_of_mut!(c.u) }) as *mut (),
                        pCellData as *const (),
                        ((4 as i32) as i64) as u64,
                    )
                };
                unsafe {
                    c.u = (unsafe { c.u }) >> (24 as i32) & ((255 as i32) as u32)
                        | (unsafe { c.u }) >> (8 as i32) & ((65280 as i32) as u32)
                        | ((unsafe { c.u }) & ((255 as i32) as u32)) << (24 as i32)
                        | ((unsafe { c.u }) & ((65280 as i32) as u32)) << (8 as i32);
                }
                val = if eInt != (0 as i32) {
                    (unsafe { c.i }) as f64
                } else {
                    (unsafe { c.f }) as f64
                };
                {}
                // val now holds the upper bound of the coordinate pair
                if (unsafe { (*p).u.rValue }) <= val {
                    return;
                }
            }
        }
    }
    unsafe {
        *peWithin = 0 as i32;
    }
}

/// Check the leaf RTree cell given by pCellData against constraint p.
/// If this constraint is not satisfied, set *peWithin to NOT_WITHIN.
/// If the constraint is satisfied, leave *peWithin unchanged.
///
/// The constraint is of the form:  xN op $val
///
/// The op is given by p->op.  The xN is p->iCoord-th coordinate in
/// pCellData.  $val is given by p->u.rValue.
///
/// # Arguments
///
/// * `p` - The constraint to test
/// * `eInt` - True if RTree holds integer coordinates
/// * `pCellData` - Raw cell content as appears on disk
/// * `peWithin` - Adjust downward, as appropriate
fn rtreeLeafConstraint(
    mut p: *mut RtreeConstraint,
    mut eInt: i32,
    mut pCellData: *mut u8,
    mut peWithin: *mut i32,
) {
    let mut xN: f64 = 0 as f64; // Coordinate value converted to a double
    0 as i32;
    let __v1463: *mut u8 = pCellData;
    let __v1464: *mut u8 =
        unsafe { __v1463.offset(((8 as i32) + (unsafe { (*p).iCoord }) * (4 as i32)) as isize) };
    pCellData = __v1464;
    0 as i32;
    let mut c: RtreeCoord = unsafe { std::mem::zeroed() };
    unsafe {
        memcpy(
            (unsafe { std::ptr::addr_of_mut!(c.u) }) as *mut (),
            pCellData as *const (),
            ((4 as i32) as i64) as u64,
        )
    };
    unsafe {
        c.u = (unsafe { c.u }) >> (24 as i32) & ((255 as i32) as u32)
            | (unsafe { c.u }) >> (8 as i32) & ((65280 as i32) as u32)
            | ((unsafe { c.u }) & ((255 as i32) as u32)) << (24 as i32)
            | ((unsafe { c.u }) & ((65280 as i32) as u32)) << (8 as i32);
    }
    xN = if eInt != (0 as i32) {
        (unsafe { c.i }) as f64
    } else {
        (unsafe { c.f }) as f64
    };
    {}
    '__slate_break_1234: {
        match unsafe { (*p).op } {
            63 => {
                return; // Always satisfied
            }
            64 => {
                break '__slate_break_1234; // Never satisfied
            }
            66 => {
                if xN <= unsafe { (*p).u.rValue } {
                    return;
                }
            }
            67 => {
                if xN < unsafe { (*p).u.rValue } {
                    return;
                }
            }
            68 => {
                if xN >= unsafe { (*p).u.rValue } {
                    return;
                }
            }
            69 => {
                if xN > unsafe { (*p).u.rValue } {
                    return;
                }
            }
            _ => {
                if xN == unsafe { (*p).u.rValue } {
                    return;
                }
            }
        }
    }
    unsafe {
        *peWithin = 0 as i32;
    }
}

/// One of the cells in node pNode is guaranteed to have a 64-bit
/// integer value equal to iRowid. Return the index of this cell.
fn nodeRowidIndex(
    mut pRtree: *mut Rtree,
    mut pNode: *mut RtreeNode,
    mut iRowid: i64,
    mut piIndex: *mut i32,
) -> i32 {
    let mut ii: i32 = 0 as i32;
    let mut nCell: i32 =
        readInt16(unsafe { unsafe { (*pNode).zData }.offset((2 as i32) as isize) });
    0 as i32;
    ii = 0 as i32;
    '__slate_break_1235: loop {
        if !(ii < nCell) {
            break;
        }
        if nodeGetRowid(pRtree, pNode, ii) == iRowid {
            unsafe {
                *piIndex = ii;
            }
            return 0 as i32;
        }
        let __v1465: i32 = ii;
        let __v1466: i32 = __v1465 + (1 as i32);
        ii = __v1466;
    }
    {}
    return (11 as i32) | (1 as i32) << (8 as i32);
}

/// Return the index of the cell containing a pointer to node pNode
/// in its parent. If pNode is the root node, return -1.
fn nodeParentIndex(
    mut pRtree: *mut Rtree,
    mut pNode: *mut RtreeNode,
    mut piIndex: *mut i32,
) -> i32 {
    let mut pParent: *mut RtreeNode = unsafe { (*pNode).pParent };
    if pParent != std::ptr::null_mut::<RtreeNode>() {
        return nodeRowidIndex(pRtree, pParent, unsafe { (*pNode).iNode }, piIndex);
    } else {
        unsafe {
            *piIndex = -(1 as i32);
        }
        return 0 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

/// Compare two search points.  Return negative, zero, or positive if the first
/// is less than, equal to, or greater than the second.
///
/// The rScore is the primary key.  Smaller rScore values come first.
/// If the rScore is a tie, then use iLevel as the tie breaker with smaller
/// iLevel values coming first.  In this way, if rScore is the same for all
/// SearchPoints, then iLevel becomes the deciding factor and the result
/// is a depth-first search, which is the desired default behavior.
fn rtreeSearchPointCompare(
    mut pA: *const RtreeSearchPoint,
    mut pB: *const RtreeSearchPoint,
) -> i32 {
    if (unsafe { (*pA).rScore }) < unsafe { (*pB).rScore } {
        return -(1 as i32);
    }
    if (unsafe { (*pA).rScore }) > unsafe { (*pB).rScore } {
        return 1 as i32;
    }
    if (((unsafe { (*pA).iLevel }) as u32) as i32) < (((unsafe { (*pB).iLevel }) as u32) as i32) {
        return -(1 as i32);
    }
    if (((unsafe { (*pA).iLevel }) as u32) as i32) > (((unsafe { (*pB).iLevel }) as u32) as i32) {
        return 1 as i32;
    }
    return 0 as i32;
}

/// Interchange two search points in a cursor.
fn rtreeSearchPointSwap(mut p: *mut RtreeCursor, mut i: i32, mut j: i32) {
    let mut t: RtreeSearchPoint = unsafe { *unsafe { unsafe { (*p).aPoint }.offset(i as isize) } };
    0 as i32;
    unsafe {
        *unsafe { unsafe { (*p).aPoint }.offset(i as isize) } =
            unsafe { *unsafe { unsafe { (*p).aPoint }.offset(j as isize) } };
    }
    unsafe {
        *unsafe { unsafe { (*p).aPoint }.offset(j as isize) } = t;
    }
    let __v1467: i32 = i;
    let __v1468: i32 = __v1467 + (1 as i32);
    i = __v1468;
    let __v1469: i32 = j;
    let __v1470: i32 = __v1469 + (1 as i32);
    j = __v1470;
    if i < (5 as i32) {
        if j >= (5 as i32) {
            nodeRelease((unsafe { (*p).base.pVtab }) as *mut Rtree, unsafe {
                *unsafe {
                    unsafe { (*p).aNode.as_mut_ptr() as *mut *mut RtreeNode }.offset(i as isize)
                }
            });
            unsafe {
                *unsafe {
                    unsafe { (*p).aNode.as_mut_ptr() as *mut *mut RtreeNode }.offset(i as isize)
                } = std::ptr::null_mut::<RtreeNode>();
            }
        } else {
            let mut pTemp: *mut RtreeNode = unsafe {
                *unsafe {
                    unsafe { (*p).aNode.as_mut_ptr() as *mut *mut RtreeNode }.offset(i as isize)
                }
            };
            unsafe {
                *unsafe {
                    unsafe { (*p).aNode.as_mut_ptr() as *mut *mut RtreeNode }.offset(i as isize)
                } = unsafe {
                    *unsafe {
                        unsafe { (*p).aNode.as_mut_ptr() as *mut *mut RtreeNode }.offset(j as isize)
                    }
                };
            }
            unsafe {
                *unsafe {
                    unsafe { (*p).aNode.as_mut_ptr() as *mut *mut RtreeNode }.offset(j as isize)
                } = pTemp;
            }
        }
    }
}

/// Return the search point with the lowest current score.
fn rtreeSearchPointFirst(mut pCur: *mut RtreeCursor) -> *mut RtreeSearchPoint {
    return if (unsafe { (*pCur).bPoint }) != (0 as u8) {
        unsafe { std::ptr::addr_of_mut!((*pCur).sPoint) }
    } else {
        if (unsafe { (*pCur).nPoint }) != (0 as i32) {
            unsafe { (*pCur).aPoint }
        } else {
            std::ptr::null_mut::<RtreeSearchPoint>()
        }
    };
}

/// Get the RtreeNode for the search point with the lowest score.
fn rtreeNodeOfFirstSearchPoint(mut pCur: *mut RtreeCursor, mut pRC: *mut i32) -> *mut RtreeNode {
    let mut id: i64 = 0 as i64;
    let mut ii: i32 = (1 as i32) - (((unsafe { (*pCur).bPoint }) as u32) as i32);
    0 as i32;
    0 as i32;
    if (unsafe {
        *unsafe { unsafe { (*pCur).aNode.as_mut_ptr() as *mut *mut RtreeNode }.offset(ii as isize) }
    }) == std::ptr::null_mut::<RtreeNode>()
    {
        0 as i32;
        id = if ii != (0 as i32) {
            unsafe { (*unsafe { unsafe { (*pCur).aPoint }.offset((0 as i32) as isize) }).id }
        } else {
            unsafe { (*pCur).sPoint.id }
        };
        unsafe {
            *pRC = nodeAcquire(
                (unsafe { (*pCur).base.pVtab }) as *mut Rtree,
                id,
                std::ptr::null_mut::<RtreeNode>(),
                unsafe {
                    unsafe { (*pCur).aNode.as_mut_ptr() as *mut *mut RtreeNode }.offset(ii as isize)
                },
            );
        }
    }
    return unsafe {
        *unsafe { unsafe { (*pCur).aNode.as_mut_ptr() as *mut *mut RtreeNode }.offset(ii as isize) }
    };
}

/// Push a new element onto the priority queue
///
/// # Arguments
///
/// * `pCur` - The cursor
/// * `rScore` - Score for the new search point
/// * `iLevel` - Level for the new search point
fn rtreeEnqueue(
    mut pCur: *mut RtreeCursor,
    mut rScore: f64,
    mut iLevel: u8,
) -> *mut RtreeSearchPoint {
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut pNew: *mut RtreeSearchPoint = unsafe { std::mem::zeroed() };
    if (unsafe { (*pCur).nPoint }) >= unsafe { (*pCur).nPointAlloc } {
        let mut nNew: i32 = (unsafe { (*pCur).nPointAlloc }) * (2 as i32) + (8 as i32);
        pNew = (unsafe {
            sqlite3_realloc64(
                (unsafe { (*pCur).aPoint }) as *mut (),
                ((nNew as i64) as u64).wrapping_mul(24 as u64),
            )
        }) as *mut RtreeSearchPoint;
        if pNew == std::ptr::null_mut::<RtreeSearchPoint>() {
            return std::ptr::null_mut::<RtreeSearchPoint>();
        }
        unsafe {
            (*pCur).aPoint = pNew;
        }
        unsafe {
            (*pCur).nPointAlloc = nNew;
        }
    }
    let __v1471: *mut RtreeCursor = pCur;
    let __v1472: i32 = unsafe { (*__v1471).nPoint };
    let __v1473: i32 = __v1472 + (1 as i32);
    unsafe {
        (*__v1471).nPoint = __v1473;
    }
    i = __v1472;
    pNew = unsafe { unsafe { (*pCur).aPoint }.offset(i as isize) };
    unsafe {
        (*pNew).rScore = rScore;
    }
    unsafe {
        (*pNew).iLevel = iLevel;
    }
    0 as i32;
    '__slate_break_1236: while i > (0 as i32) {
        let mut pParent: *mut RtreeSearchPoint = unsafe { std::mem::zeroed() };
        j = (i - (1 as i32)) / (2 as i32);
        pParent = unsafe { unsafe { (*pCur).aPoint }.offset(j as isize) };
        if rtreeSearchPointCompare(
            pNew as *const RtreeSearchPoint,
            pParent as *const RtreeSearchPoint,
        ) >= (0 as i32)
        {
            break '__slate_break_1236;
        }
        rtreeSearchPointSwap(pCur, j, i);
        i = j;
        pNew = pParent;
    }
    return pNew;
}

/// Allocate a new RtreeSearchPoint and return a pointer to it.  Return
/// NULL if malloc fails.
///
/// # Arguments
///
/// * `pCur` - The cursor
/// * `rScore` - Score for the new search point
/// * `iLevel` - Level for the new search point
fn rtreeSearchPointNew(
    mut pCur: *mut RtreeCursor,
    mut rScore: f64,
    mut iLevel: u8,
) -> *mut RtreeSearchPoint {
    let mut pNew: *mut RtreeSearchPoint = unsafe { std::mem::zeroed() };
    let mut pFirst: *mut RtreeSearchPoint = unsafe { std::mem::zeroed() };
    pFirst = rtreeSearchPointFirst(pCur);
    let __v1474: *mut u32 = unsafe {
        unsafe { (*pCur).anQueue.as_mut_ptr() as *mut u32 }
            .offset(((iLevel as u32) as i32) as isize)
    };
    let __v1475: u32 = unsafe { *__v1474 };
    let __v1476: u32 = __v1475.wrapping_add((1 as i32) as u32);
    unsafe {
        *__v1474 = __v1476;
    }
    if pFirst == std::ptr::null_mut::<RtreeSearchPoint>()
        || (unsafe { (*pFirst).rScore }) > rScore
        || (unsafe { (*pFirst).rScore }) == rScore
            && (((unsafe { (*pFirst).iLevel }) as u32) as i32) > ((iLevel as u32) as i32)
    {
        if (unsafe { (*pCur).bPoint }) != (0 as u8) {
            let mut ii: i32 = 0 as i32;
            pNew = rtreeEnqueue(pCur, rScore, iLevel);
            if pNew == std::ptr::null_mut::<RtreeSearchPoint>() {
                return std::ptr::null_mut::<RtreeSearchPoint>();
            }
            ii = (((unsafe {
                pNew.offset_from((unsafe { (*pCur).aPoint }) as *mut RtreeSearchPoint)
            }) as i64) as i32)
                + (1 as i32);
            0 as i32;
            if ii < (5 as i32) {
                0 as i32;
                unsafe {
                    *unsafe {
                        unsafe { (*pCur).aNode.as_mut_ptr() as *mut *mut RtreeNode }
                            .offset(ii as isize)
                    } = unsafe {
                        *unsafe {
                            unsafe { (*pCur).aNode.as_mut_ptr() as *mut *mut RtreeNode }
                                .offset((0 as i32) as isize)
                        }
                    };
                }
            } else {
                nodeRelease((unsafe { (*pCur).base.pVtab }) as *mut Rtree, unsafe {
                    *unsafe {
                        unsafe { (*pCur).aNode.as_mut_ptr() as *mut *mut RtreeNode }
                            .offset((0 as i32) as isize)
                    }
                });
            }
            unsafe {
                *unsafe {
                    unsafe { (*pCur).aNode.as_mut_ptr() as *mut *mut RtreeNode }
                        .offset((0 as i32) as isize)
                } = std::ptr::null_mut::<RtreeNode>();
            }
            unsafe {
                *pNew = unsafe { (*pCur).sPoint };
            }
        }
        unsafe {
            (*pCur).sPoint.rScore = rScore;
        }
        unsafe {
            (*pCur).sPoint.iLevel = iLevel;
        }
        unsafe {
            (*pCur).bPoint = ((1 as i32) as i8) as u8;
        }
        return unsafe { std::ptr::addr_of_mut!((*pCur).sPoint) };
    } else {
        return rtreeEnqueue(pCur, rScore, iLevel);
    }
    return unsafe { std::mem::zeroed() };
}

// no-op
/// Remove the search point with the lowest current score.
fn rtreeSearchPointPop(mut p: *mut RtreeCursor) {
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut k: i32 = 0 as i32;
    let mut n: i32 = 0 as i32;
    i = (1 as i32) - (((unsafe { (*p).bPoint }) as u32) as i32);
    0 as i32;
    if (unsafe {
        *unsafe { unsafe { (*p).aNode.as_mut_ptr() as *mut *mut RtreeNode }.offset(i as isize) }
    }) != std::ptr::null_mut::<RtreeNode>()
    {
        nodeRelease((unsafe { (*p).base.pVtab }) as *mut Rtree, unsafe {
            *unsafe { unsafe { (*p).aNode.as_mut_ptr() as *mut *mut RtreeNode }.offset(i as isize) }
        });
        unsafe {
            *unsafe {
                unsafe { (*p).aNode.as_mut_ptr() as *mut *mut RtreeNode }.offset(i as isize)
            } = std::ptr::null_mut::<RtreeNode>();
        }
    }
    if (unsafe { (*p).bPoint }) != (0 as u8) {
        let __v1477: *mut u32 = unsafe {
            unsafe { (*p).anQueue.as_mut_ptr() as *mut u32 }
                .offset((((unsafe { (*p).sPoint.iLevel }) as u32) as i32) as isize)
        };
        let __v1478: u32 = unsafe { *__v1477 };
        let __v1479: u32 = __v1478.wrapping_sub((1 as i32) as u32);
        unsafe {
            *__v1477 = __v1479;
        }
        unsafe {
            (*p).bPoint = ((0 as i32) as i8) as u8;
        }
    } else {
        if (unsafe { (*p).nPoint }) != (0 as i32) {
            let __v1480: *mut u32 = unsafe {
                unsafe { (*p).anQueue.as_mut_ptr() as *mut u32 }.offset(
                    (((unsafe {
                        (*unsafe { unsafe { (*p).aPoint }.offset((0 as i32) as isize) }).iLevel
                    }) as u32) as i32) as isize,
                )
            };
            let __v1481: u32 = unsafe { *__v1480 };
            let __v1482: u32 = __v1481.wrapping_sub((1 as i32) as u32);
            unsafe {
                *__v1480 = __v1482;
            }
            let __v1483: *mut RtreeCursor = p;
            let __v1484: i32 = unsafe { (*__v1483).nPoint };
            let __v1485: i32 = __v1484 - (1 as i32);
            unsafe {
                (*__v1483).nPoint = __v1485;
            }
            n = __v1485;
            unsafe {
                *unsafe { unsafe { (*p).aPoint }.offset((0 as i32) as isize) } =
                    unsafe { *unsafe { unsafe { (*p).aPoint }.offset(n as isize) } };
            }
            if n < (5 as i32) - (1 as i32) {
                unsafe {
                    *unsafe {
                        unsafe { (*p).aNode.as_mut_ptr() as *mut *mut RtreeNode }
                            .offset((1 as i32) as isize)
                    } = unsafe {
                        *unsafe {
                            unsafe { (*p).aNode.as_mut_ptr() as *mut *mut RtreeNode }
                                .offset((n + (1 as i32)) as isize)
                        }
                    };
                }
                unsafe {
                    *unsafe {
                        unsafe { (*p).aNode.as_mut_ptr() as *mut *mut RtreeNode }
                            .offset((n + (1 as i32)) as isize)
                    } = std::ptr::null_mut::<RtreeNode>();
                }
            }
            i = 0 as i32;
            '__slate_break_1237: loop {
                let __v1486: i32 = i * (2 as i32) + (1 as i32);
                j = __v1486;
                if !(__v1486 < n) {
                    break;
                }
                k = j + (1 as i32);
                let __v1487: bool;
                if k < n {
                    __v1487 = rtreeSearchPointCompare(
                        (unsafe { unsafe { (*p).aPoint }.offset(k as isize) })
                            as *const RtreeSearchPoint,
                        (unsafe { unsafe { (*p).aPoint }.offset(j as isize) })
                            as *const RtreeSearchPoint,
                    ) < (0 as i32);
                } else {
                    __v1487 = false as bool;
                }
                if __v1487 {
                    if rtreeSearchPointCompare(
                        (unsafe { unsafe { (*p).aPoint }.offset(k as isize) })
                            as *const RtreeSearchPoint,
                        (unsafe { unsafe { (*p).aPoint }.offset(i as isize) })
                            as *const RtreeSearchPoint,
                    ) < (0 as i32)
                    {
                        rtreeSearchPointSwap(p, i, k);
                        i = k;
                    } else {
                        break '__slate_break_1237;
                    }
                } else {
                    if rtreeSearchPointCompare(
                        (unsafe { unsafe { (*p).aPoint }.offset(j as isize) })
                            as *const RtreeSearchPoint,
                        (unsafe { unsafe { (*p).aPoint }.offset(i as isize) })
                            as *const RtreeSearchPoint,
                    ) < (0 as i32)
                    {
                        rtreeSearchPointSwap(p, i, j);
                        i = j;
                    } else {
                        break '__slate_break_1237;
                    }
                }
            }
        }
    }
}

/// Continue the search on cursor pCur until the front of the queue
/// contains an entry suitable for returning as a result-set row,
/// or until the RtreeSearchPoint queue is empty, indicating that the
/// query has completed.
fn rtreeStepToLeaf(mut pCur: *mut RtreeCursor) -> i32 {
    let mut p: *mut RtreeSearchPoint = unsafe { std::mem::zeroed() };
    let mut pRtree: *mut Rtree = (unsafe { (*pCur).base.pVtab }) as *mut Rtree;
    let mut pNode: *mut RtreeNode = unsafe { std::mem::zeroed() };
    let mut eWithin: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut nCell: i32 = 0 as i32;
    let mut nConstraint: i32 = unsafe { (*pCur).nConstraint };
    let mut ii: i32 = 0 as i32;
    let mut eInt: i32 = 0 as i32;
    let mut x: RtreeSearchPoint = unsafe { std::mem::zeroed() };
    eInt = ((((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (1 as i32)) as i32;
    '__slate_break_1238: loop {
        let __v1488: *mut RtreeSearchPoint = rtreeSearchPointFirst(pCur);
        p = __v1488;
        if !(__v1488 != std::ptr::null_mut::<RtreeSearchPoint>()
            && (((unsafe { (*p).iLevel }) as u32) as i32) > (0 as i32))
        {
            break;
        }
        let mut pCellData: *mut u8 = unsafe { std::mem::zeroed() };
        pNode = rtreeNodeOfFirstSearchPoint(pCur, std::ptr::addr_of_mut!(rc));
        if rc != (0 as i32) {
            return rc;
        }
        nCell = readInt16(unsafe { unsafe { (*pNode).zData }.offset((2 as i32) as isize) });
        if nCell > (51 as i32) {
            {}
            return (11 as i32) | (1 as i32) << (8 as i32);
        }
        pCellData = unsafe {
            unsafe { (*pNode).zData }.offset(
                ((4 as i32)
                    + (((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32)
                        * (((unsafe { (*p).iCell }) as u32) as i32)) as isize,
            )
        };
        '__slate_break_1239: while (((unsafe { (*p).iCell }) as u32) as i32) < nCell {
            let mut rScore: f64 = -(1 as i32) as f64;
            eWithin = 2 as i32;
            ii = 0 as i32;
            '__slate_break_1240: loop {
                if !(ii < nConstraint) {
                    break;
                }
                let mut pConstraint: *mut RtreeConstraint =
                    unsafe { unsafe { (*pCur).aConstraint }.offset(ii as isize) };
                if (unsafe { (*pConstraint).op }) >= (70 as i32) {
                    rc = rtreeCallbackConstraint(
                        pConstraint,
                        eInt,
                        pCellData,
                        p,
                        std::ptr::addr_of_mut!(rScore),
                        std::ptr::addr_of_mut!(eWithin),
                    );
                    if rc != (0 as i32) {
                        return rc;
                    }
                } else {
                    if (((unsafe { (*p).iLevel }) as u32) as i32) == (1 as i32) {
                        rtreeLeafConstraint(
                            pConstraint,
                            eInt,
                            pCellData,
                            std::ptr::addr_of_mut!(eWithin),
                        );
                    } else {
                        rtreeNonleafConstraint(
                            pConstraint,
                            eInt,
                            pCellData,
                            std::ptr::addr_of_mut!(eWithin),
                        );
                    }
                }
                if eWithin == (0 as i32) {
                    let __v1491: *mut RtreeSearchPoint = p;
                    let __v1492: u8 = unsafe { (*__v1491).iCell };
                    let __v1493: u8 = ((((__v1492 as u32) as i32) + (1 as i32)) as i8) as u8;
                    unsafe {
                        (*__v1491).iCell = __v1493;
                    }
                    let __v1494: *mut u8 = pCellData;
                    let __v1495: *mut u8 = unsafe {
                        __v1494
                            .offset((((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32) as isize)
                    };
                    pCellData = __v1495;
                    break '__slate_break_1240;
                }
                let __v1489: i32 = ii;
                let __v1490: i32 = __v1489 + (1 as i32);
                ii = __v1490;
            }
            if eWithin == (0 as i32) {
            } else {
                let __v1496: *mut RtreeSearchPoint = p;
                let __v1497: u8 = unsafe { (*__v1496).iCell };
                let __v1498: u8 = ((((__v1497 as u32) as i32) + (1 as i32)) as i8) as u8;
                unsafe {
                    (*__v1496).iCell = __v1498;
                }
                x.iLevel = (((((unsafe { (*p).iLevel }) as u32) as i32) - (1 as i32)) as i8) as u8;
                if x.iLevel != (0 as u8) {
                    x.id = readInt64(pCellData);
                    ii = 0 as i32;
                    '__slate_break_1241: loop {
                        if !(ii < unsafe { (*pCur).nPoint }) {
                            break;
                        }
                        if (unsafe {
                            (*unsafe { unsafe { (*pCur).aPoint }.offset(ii as isize) }).id
                        }) == x.id
                        {
                            {}
                            return (11 as i32) | (1 as i32) << (8 as i32);
                        }
                        let __v1499: i32 = ii;
                        let __v1500: i32 = __v1499 + (1 as i32);
                        ii = __v1500;
                    }
                    x.iCell = ((0 as i32) as i8) as u8;
                } else {
                    x.id = unsafe { (*p).id };
                    x.iCell =
                        (((((unsafe { (*p).iCell }) as u32) as i32) - (1 as i32)) as i8) as u8;
                }
                if (((unsafe { (*p).iCell }) as u32) as i32) >= nCell {
                    {}
                    rtreeSearchPointPop(pCur);
                }
                if rScore < 0.0f64 {
                    rScore = 0.0f64;
                }
                p = rtreeSearchPointNew(pCur, rScore, x.iLevel);
                if p == std::ptr::null_mut::<RtreeSearchPoint>() {
                    return 7 as i32;
                }
                unsafe {
                    (*p).eWithin = (eWithin as i8) as u8;
                }
                unsafe {
                    (*p).id = x.id;
                }
                unsafe {
                    (*p).iCell = x.iCell;
                }
                {}
                break '__slate_break_1239;
            }
        }
        if (((unsafe { (*p).iCell }) as u32) as i32) >= nCell {
            {}
            rtreeSearchPointPop(pCur);
        }
    }
    unsafe {
        (*pCur).atEOF = (p == std::ptr::null_mut::<RtreeSearchPoint>()) as u8;
    }
    return 0 as i32;
}

/// Rtree virtual table module xNext method.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeNext")]
extern "C-unwind" fn rtreeNext(mut pVtabCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut RtreeCursor = pVtabCursor as *mut RtreeCursor;
    let mut rc: i32 = 0 as i32;
    // Move to the next entry that matches the configured constraints.
    {}
    if (unsafe { (*pCsr).bAuxValid }) != (0 as u8) {
        unsafe {
            (*pCsr).bAuxValid = ((0 as i32) as i8) as u8;
        }
        unsafe { sqlite3_reset(unsafe { (*pCsr).pReadAux }) };
    }
    rtreeSearchPointPop(pCsr);
    rc = rtreeStepToLeaf(pCsr);
    return rc;
}

/// Rtree virtual table module xRowid method.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeRowid")]
extern "C-unwind" fn rtreeRowid(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut i64,
) -> i32 {
    let mut pCsr: *mut RtreeCursor = pVtabCursor as *mut RtreeCursor;
    let mut p: *mut RtreeSearchPoint = rtreeSearchPointFirst(pCsr);
    let mut rc: i32 = 0 as i32;
    let mut pNode: *mut RtreeNode = rtreeNodeOfFirstSearchPoint(pCsr, std::ptr::addr_of_mut!(rc));
    if rc == (0 as i32) && p != std::ptr::null_mut::<RtreeSearchPoint>() {
        if (((unsafe { (*p).iCell }) as u32) as i32)
            >= readInt16(unsafe { unsafe { (*pNode).zData }.offset((2 as i32) as isize) })
        {
            rc = 4 as i32;
        } else {
            unsafe {
                *pRowid = nodeGetRowid(
                    (unsafe { (*pCsr).base.pVtab }) as *mut Rtree,
                    pNode,
                    ((unsafe { (*p).iCell }) as u32) as i32,
                );
            }
        }
    }
    return rc;
}

/// Rtree virtual table module xColumn method.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeColumn")]
extern "C-unwind" fn rtreeColumn(
    mut cur: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: i32,
) -> i32 {
    let mut pRtree: *mut Rtree = (unsafe { (*cur).pVtab }) as *mut Rtree;
    let mut pCsr: *mut RtreeCursor = cur as *mut RtreeCursor;
    let mut p: *mut RtreeSearchPoint = rtreeSearchPointFirst(pCsr);
    let mut c: RtreeCoord = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    let mut pNode: *mut RtreeNode = rtreeNodeOfFirstSearchPoint(pCsr, std::ptr::addr_of_mut!(rc));
    if rc != (0 as i32) {
        return rc;
    }
    if p == std::ptr::null_mut::<RtreeSearchPoint>() {
        return 0 as i32;
    }
    if (((unsafe { (*p).iCell }) as u32) as i32)
        >= readInt16(unsafe { unsafe { (*pNode).zData }.offset((2 as i32) as isize) })
    {
        return 4 as i32;
    }
    if i == (0 as i32) {
        unsafe {
            sqlite3_result_int64(
                ctx,
                nodeGetRowid(pRtree, pNode, ((unsafe { (*p).iCell }) as u32) as i32),
            )
        };
    } else {
        if i <= (((unsafe { (*pRtree).nDim2 }) as u32) as i32) {
            nodeGetCoord(
                pRtree,
                pNode,
                ((unsafe { (*p).iCell }) as u32) as i32,
                i - (1 as i32),
                std::ptr::addr_of_mut!(c),
            );
            if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (0 as i32) {
                unsafe { sqlite3_result_double(ctx, (unsafe { c.f }) as f64) };
            } else {
                0 as i32;
                unsafe { sqlite3_result_int(ctx, unsafe { c.i }) };
            }
        } else {
            if !((unsafe { (*pCsr).bAuxValid }) != (0 as u8)) {
                if (unsafe { (*pCsr).pReadAux }) == std::ptr::null_mut::<sqlite3_stmt>() {
                    rc = unsafe {
                        sqlite3_prepare_v3(
                            unsafe { (*pRtree).db },
                            (unsafe { (*pRtree).zReadAuxSql }) as *const i8,
                            -(1 as i32),
                            (0 as i32) as u32,
                            unsafe { std::ptr::addr_of_mut!((*pCsr).pReadAux) },
                            std::ptr::null_mut::<*const i8>(),
                        )
                    };
                    if rc != (0 as i32) {
                        return rc;
                    }
                }
                unsafe {
                    sqlite3_bind_int64(
                        unsafe { (*pCsr).pReadAux },
                        1 as i32,
                        nodeGetRowid(pRtree, pNode, ((unsafe { (*p).iCell }) as u32) as i32),
                    )
                };
                rc = unsafe { sqlite3_step(unsafe { (*pCsr).pReadAux }) };
                if rc == (100 as i32) {
                    unsafe {
                        (*pCsr).bAuxValid = ((1 as i32) as i8) as u8;
                    }
                } else {
                    unsafe { sqlite3_reset(unsafe { (*pCsr).pReadAux }) };
                    if rc == (101 as i32) {
                        rc = 0 as i32;
                    }
                    return rc;
                }
            }
            unsafe {
                sqlite3_result_value(ctx, unsafe {
                    sqlite3_column_value(
                        unsafe { (*pCsr).pReadAux },
                        i - (((unsafe { (*pRtree).nDim2 }) as u32) as i32) + (1 as i32),
                    )
                })
            };
        }
    }
    return 0 as i32;
}

/// Use nodeAcquire() to obtain the leaf node containing the record with
/// rowid iRowid. If successful, set *ppLeaf to point to the node and
/// return SQLITE_OK. If there is no such record in the table, set
/// *ppLeaf to 0 and return SQLITE_OK. If an error occurs, set *ppLeaf
/// to zero and return an SQLite error code.
///
/// # Arguments
///
/// * `pRtree` - RTree to search
/// * `iRowid` - The rowid searching for
/// * `ppLeaf` - Write the node here
/// * `piNode` - Write the node-id here
fn findLeafNode(
    mut pRtree: *mut Rtree,
    mut iRowid: i64,
    mut ppLeaf: *mut *mut RtreeNode,
    mut piNode: *mut i64,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    unsafe {
        *ppLeaf = std::ptr::null_mut::<RtreeNode>();
    }
    unsafe { sqlite3_bind_int64(unsafe { (*pRtree).pReadRowid }, 1 as i32, iRowid) };
    if (unsafe { sqlite3_step(unsafe { (*pRtree).pReadRowid }) }) == (100 as i32) {
        let mut iNode: i64 =
            unsafe { sqlite3_column_int64(unsafe { (*pRtree).pReadRowid }, 0 as i32) };
        if piNode != std::ptr::null_mut::<i64>() {
            unsafe {
                *piNode = iNode;
            }
        }
        rc = nodeAcquire(pRtree, iNode, std::ptr::null_mut::<RtreeNode>(), ppLeaf);
        unsafe { sqlite3_reset(unsafe { (*pRtree).pReadRowid }) };
    } else {
        rc = unsafe { sqlite3_reset(unsafe { (*pRtree).pReadRowid }) };
    }
    return rc;
}

/// This function is called to configure the RtreeConstraint object passed
/// as the second argument for a MATCH constraint. The value passed as the
/// first argument to this function is the right-hand operand to the MATCH
/// operator.
fn deserializeGeometry(mut pValue: *mut sqlite3_value, mut pCons: *mut RtreeConstraint) -> i32 {
    let mut pBlob: *mut RtreeMatchArg = unsafe { std::mem::zeroed() };
    let mut pSrc: *mut RtreeMatchArg = unsafe { std::mem::zeroed() }; // BLOB returned by geometry function
    let mut pInfo: *mut sqlite3_rtree_query_info = unsafe { std::mem::zeroed() }; // Callback information
    pSrc = (unsafe {
        sqlite3_value_pointer(
            pValue,
            (b"RtreeMatchArg\0".as_ptr() as *mut i8) as *const i8,
        )
    }) as *mut RtreeMatchArg;
    if pSrc == std::ptr::null_mut::<RtreeMatchArg>() {
        return 1 as i32;
    }
    pInfo =
        (unsafe { sqlite3_malloc64((112 as u64).wrapping_add((unsafe { (*pSrc).iSize }) as u64)) })
            as *mut sqlite3_rtree_query_info;
    if !(pInfo != std::ptr::null_mut::<sqlite3_rtree_query_info>()) {
        return 7 as i32;
    }
    unsafe { memset(pInfo as *mut (), 0 as i32, 112 as u64) };
    pBlob = (unsafe { pInfo.offset((1 as i32) as isize) }) as *mut RtreeMatchArg;
    unsafe {
        memcpy(
            pBlob as *mut (),
            pSrc as *const (),
            (unsafe { (*pSrc).iSize }) as u64,
        )
    };
    unsafe {
        (*pInfo).pContext = unsafe { (*pBlob).cb.pContext };
    }
    unsafe {
        (*pInfo).nParam = unsafe { (*pBlob).nParam };
    }
    unsafe {
        (*pInfo).aParam = unsafe { std::ptr::addr_of_mut!((*pBlob).aParam) as *mut f64 };
    }
    unsafe {
        (*pInfo).apSqlParam = unsafe { (*pBlob).apSqlParam };
    }
    if (unsafe { (*pBlob).cb.xGeom }) != None {
        unsafe {
            (*pCons).u.xGeom = unsafe { (*pBlob).cb.xGeom };
        }
    } else {
        unsafe {
            (*pCons).op = 71 as i32;
        }
        unsafe {
            (*pCons).u.xQueryFunc = unsafe { (*pBlob).cb.xQueryFunc };
        }
    }
    unsafe {
        (*pCons).pInfo = pInfo;
    }
    return 0 as i32;
}

/// Rtree virtual table module xFilter method.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeFilter")]
extern "C-unwind" fn rtreeFilter(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: i32,
    mut idxStr: *const i8,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) -> i32 {
    let mut pRtree: *mut Rtree = (unsafe { (*pVtabCursor).pVtab }) as *mut Rtree;
    let mut pCsr: *mut RtreeCursor = pVtabCursor as *mut RtreeCursor;
    let mut pRoot: *mut RtreeNode = std::ptr::null_mut::<RtreeNode>();
    let mut ii: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut iCell: i32 = 0 as i32;
    rtreeReference(pRtree);
    // Reset the cursor to the same state as rtreeOpen() leaves it in.
    resetCursor(pCsr);
    unsafe {
        (*pCsr).iStrategy = idxNum;
    }
    if idxNum == (1 as i32) {
        // Special case - lookup by rowid.
        let mut pLeaf: *mut RtreeNode = unsafe { std::mem::zeroed() }; // Leaf on which the required cell resides
        let mut p: *mut RtreeSearchPoint = unsafe { std::mem::zeroed() }; // Search point for the leaf
        let mut iRowid: i64 =
            unsafe { sqlite3_value_int64(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
        let mut iNode: i64 = (0 as i32) as i64;
        let mut eType: i32 = unsafe {
            sqlite3_value_numeric_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
        };
        let __v1501: bool;
        if eType == (1 as i32) {
            __v1501 = true as bool;
        } else {
            let __v1502: bool;
            if eType == (2 as i32) {
                __v1502 = (0 as i32)
                    == unsafe {
                        sqlite3IntFloatCompare(iRowid, unsafe {
                            sqlite3_value_double(unsafe {
                                *unsafe { argv.offset((0 as i32) as isize) }
                            })
                        })
                    };
            } else {
                __v1502 = false as bool;
            }
            __v1501 = __v1502;
        }
        if __v1501 {
            rc = findLeafNode(
                pRtree,
                iRowid,
                std::ptr::addr_of_mut!(pLeaf),
                std::ptr::addr_of_mut!(iNode),
            );
        } else {
            rc = 0 as i32;
            pLeaf = std::ptr::null_mut::<RtreeNode>();
        }
        if rc == (0 as i32) && pLeaf != std::ptr::null_mut::<RtreeNode>() {
            p = rtreeSearchPointNew(pCsr, 0.0f64, ((0 as i32) as i8) as u8);
            0 as i32; // Always returns pCsr->sPoint
            unsafe {
                *unsafe {
                    unsafe { (*pCsr).aNode.as_mut_ptr() as *mut *mut RtreeNode }
                        .offset((0 as i32) as isize)
                } = pLeaf;
            }
            unsafe {
                (*p).id = iNode;
            }
            unsafe {
                (*p).eWithin = ((1 as i32) as i8) as u8;
            }
            rc = nodeRowidIndex(pRtree, pLeaf, iRowid, std::ptr::addr_of_mut!(iCell));
            unsafe {
                (*p).iCell = (iCell as i8) as u8;
            }
            {}
        } else {
            unsafe {
                (*pCsr).atEOF = ((1 as i32) as i8) as u8;
            }
        }
    } else {
        // Normal case - r-tree scan. Set up the RtreeCursor.aConstraint array
        // with the configured constraints.
        rc = nodeAcquire(
            pRtree,
            (1 as i32) as i64,
            std::ptr::null_mut::<RtreeNode>(),
            std::ptr::addr_of_mut!(pRoot),
        );
        if rc == (0 as i32) && argc > (0 as i32) {
            unsafe {
                (*pCsr).aConstraint =
                    (unsafe { sqlite3_malloc64((24 as u64).wrapping_mul((argc as i64) as u64)) })
                        as *mut RtreeConstraint;
            }
            unsafe {
                (*pCsr).nConstraint = argc;
            }
            if !((unsafe { (*pCsr).aConstraint }) != std::ptr::null_mut::<RtreeConstraint>()) {
                rc = 7 as i32;
            } else {
                unsafe {
                    memset(
                        (unsafe { (*pCsr).aConstraint }) as *mut (),
                        0 as i32,
                        (24 as u64).wrapping_mul((argc as i64) as u64),
                    )
                };
                unsafe {
                    memset(
                        (unsafe { (*pCsr).anQueue.as_mut_ptr() as *mut u32 }) as *mut (),
                        0 as i32,
                        (4 as u64).wrapping_mul(
                            (((unsafe { (*pRtree).iDepth }) + (1 as i32)) as i64) as u64,
                        ),
                    )
                };
                0 as i32;
                ii = 0 as i32;
                '__slate_break_1245: loop {
                    if !(ii < argc) {
                        break;
                    }
                    let mut p: *mut RtreeConstraint =
                        unsafe { unsafe { (*pCsr).aConstraint }.offset(ii as isize) };
                    let mut eType: i32 = unsafe {
                        sqlite3_value_numeric_type(unsafe { *unsafe { argv.offset(ii as isize) } })
                    };
                    unsafe {
                        (*p).op = (unsafe { *unsafe { idxStr.offset((ii * (2 as i32)) as isize) } })
                            as i32;
                    }
                    unsafe {
                        (*p).iCoord = ((unsafe {
                            *unsafe { idxStr.offset((ii * (2 as i32) + (1 as i32)) as isize) }
                        }) as i32)
                            - (48 as i32);
                    }
                    if (unsafe { (*p).op }) >= (70 as i32) {
                        // A MATCH operator. The right-hand-side must be a blob that
                        // can be cast into an RtreeMatchArg object. One created using
                        // an sqlite3_rtree_geometry_callback() SQL user function.
                        rc =
                            deserializeGeometry(unsafe { *unsafe { argv.offset(ii as isize) } }, p);
                        if rc != (0 as i32) {
                            break '__slate_break_1245;
                        }
                        unsafe {
                            (*unsafe { (*p).pInfo }).nCoord =
                                ((unsafe { (*pRtree).nDim2 }) as u32) as i32;
                        }
                        unsafe {
                            (*unsafe { (*p).pInfo }).anQueue =
                                unsafe { (*pCsr).anQueue.as_mut_ptr() as *mut u32 };
                        }
                        unsafe {
                            (*unsafe { (*p).pInfo }).mxLevel =
                                (unsafe { (*pRtree).iDepth }) + (1 as i32);
                        }
                    } else {
                        if eType == (1 as i32) {
                            let mut iVal: i64 = unsafe {
                                sqlite3_value_int64(unsafe { *unsafe { argv.offset(ii as isize) } })
                            };
                            unsafe {
                                (*p).u.rValue = iVal as f64;
                            }
                            if iVal >= ((1 as i32) as i64) << (48 as i32)
                                || iVal <= -(((1 as i32) as i64) << (48 as i32))
                            {
                                if (unsafe { (*p).op }) == (67 as i32) {
                                    unsafe {
                                        (*p).op = 66 as i32;
                                    }
                                }
                                if (unsafe { (*p).op }) == (69 as i32) {
                                    unsafe {
                                        (*p).op = 68 as i32;
                                    }
                                }
                            }
                        } else {
                            if eType == (2 as i32) {
                                unsafe {
                                    (*p).u.rValue = unsafe {
                                        sqlite3_value_double(unsafe {
                                            *unsafe { argv.offset(ii as isize) }
                                        })
                                    };
                                }
                            } else {
                                unsafe {
                                    (*p).u.rValue = 0.0f64;
                                }
                                if eType == (5 as i32) {
                                    unsafe {
                                        (*p).op = 64 as i32;
                                    }
                                } else {
                                    if (unsafe { (*p).op }) == (67 as i32)
                                        || (unsafe { (*p).op }) == (66 as i32)
                                    {
                                        unsafe {
                                            (*p).op = 63 as i32;
                                        }
                                    } else {
                                        unsafe {
                                            (*p).op = 64 as i32;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    let __v1503: i32 = ii;
                    let __v1504: i32 = __v1503 + (1 as i32);
                    ii = __v1504;
                }
            }
        }
        if rc == (0 as i32) {
            let mut pNew: *mut RtreeSearchPoint = unsafe { std::mem::zeroed() };
            0 as i32; // Due to the resetCursor() call above
            pNew = rtreeSearchPointNew(
                pCsr,
                0.0f64,
                (((unsafe { (*pRtree).iDepth }) + (1 as i32)) as i8) as u8,
            );
            if pNew == std::ptr::null_mut::<RtreeSearchPoint>() {
                // Because pCsr->bPoint was FALSE
                return 7 as i32;
            }
            unsafe {
                (*pNew).id = (1 as i32) as i64;
            }
            unsafe {
                (*pNew).iCell = ((0 as i32) as i8) as u8;
            }
            unsafe {
                (*pNew).eWithin = ((1 as i32) as i8) as u8;
            }
            0 as i32;
            unsafe {
                *unsafe {
                    unsafe { (*pCsr).aNode.as_mut_ptr() as *mut *mut RtreeNode }
                        .offset((0 as i32) as isize)
                } = pRoot;
            }
            pRoot = std::ptr::null_mut::<RtreeNode>();
            {}
            rc = rtreeStepToLeaf(pCsr);
        }
    }
    nodeRelease(pRtree, pRoot);
    rtreeRelease(pRtree);
    return rc;
}

/// Rtree virtual table module xBestIndex method. There are three
/// table scan strategies to choose from (in order from most to
/// least desirable):
///
///   idxNum     idxStr        Strategy
///     1        Unused        Direct lookup by rowid.
///     2        See below     R-tree query or full-table scan.
///
/// If strategy 1 is used, then idxStr is not meaningful. If strategy
/// 2 is used, idxStr is formatted to contain 2 bytes for each
/// constraint used. The first two bytes of idxStr correspond to
/// the constraint in sqlite3_index_info.aConstraintUsage[] with
/// (argvIndex==1) etc.
///
/// The first of each pair of bytes in idxStr identifies the constraint
/// operator as follows:
///
///   Operator    Byte Value
///      =        0x41 ('A')
///     <=        0x42 ('B')
///      <        0x43 ('C')
///     >=        0x44 ('D')
///      >        0x45 ('E')
///   MATCH       0x46 ('F')
///
/// The second of each pair of bytes identifies the coordinate column
/// to which the constraint applies. The leftmost coordinate column
/// is 'a', the second from the left 'b' etc.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeBestIndex")]
extern "C-unwind" fn rtreeBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> i32 {
    let mut pRtree: *mut Rtree = tab as *mut Rtree;
    let mut rc: i32 = 0 as i32;
    let mut ii: i32 = 0 as i32;
    let mut bMatch: i32 = 0 as i32; // True if there exists a MATCH constraint
    let mut nRow: i64 = 0 as i64; // Estimated rows returned by this scan
    let mut iIdx: i32 = 0 as i32;
    let mut zIdxStr: __SlateAlign16<[i8; 41]> = __SlateAlign16([0 as i8; 41]);
    unsafe {
        memset(
            (zIdxStr.0.as_mut_ptr() as *mut i8) as *mut (),
            0 as i32,
            41 as u64,
        )
    };
    // Check if there exists a MATCH constraint - even an unusable one. If there
    // is, do not consider the lookup-by-rowid plan as using such a plan would
    // require the VDBE to evaluate the MATCH constraint, which is not currently
    // possible.
    ii = 0 as i32;
    '__slate_break_1246: loop {
        if !(ii < unsafe { (*pIdxInfo).nConstraint }) {
            break;
        }
        if (((unsafe { (*unsafe { unsafe { (*pIdxInfo).aConstraint }.offset(ii as isize) }).op })
            as u32) as i32)
            == (64 as i32)
        {
            bMatch = 1 as i32;
        }
        let __v1505: i32 = ii;
        let __v1506: i32 = __v1505 + (1 as i32);
        ii = __v1506;
    }
    0 as i32;
    ii = 0 as i32;
    '__slate_break_1247: loop {
        if !(ii < unsafe { (*pIdxInfo).nConstraint }
            && iIdx < (((41 as u64).wrapping_sub(((1 as i32) as i64) as u64) as u32) as i32))
        {
            break;
        }
        let mut p: *mut sqlite3_index_constraint =
            unsafe { unsafe { (*pIdxInfo).aConstraint }.offset(ii as isize) };
        if bMatch == (0 as i32)
            && (unsafe { (*p).usable }) != (0 as u8)
            && (unsafe { (*p).iColumn }) <= (0 as i32)
            && (((unsafe { (*p).op }) as u32) as i32) == (2 as i32)
        {
            // We have an equality constraint on the rowid. Use strategy 1.
            let mut jj: i32 = 0 as i32;
            jj = 0 as i32;
            '__slate_break_1248: loop {
                if !(jj < ii) {
                    break;
                }
                unsafe {
                    (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(jj as isize) })
                        .argvIndex = 0 as i32;
                }
                unsafe {
                    (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(jj as isize) })
                        .omit = ((0 as i32) as i8) as u8;
                }
                let __v1509: i32 = jj;
                let __v1510: i32 = __v1509 + (1 as i32);
                jj = __v1510;
            }
            unsafe {
                (*pIdxInfo).idxNum = 1 as i32;
            }
            unsafe {
                (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(ii as isize) })
                    .argvIndex = 1 as i32;
            }
            unsafe {
                (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(jj as isize) }).omit =
                    ((1 as i32) as i8) as u8;
            }
            // This strategy involves a two rowid lookups on an B-Tree structures
            // and then a linear search of an R-Tree node. This should be
            // considered almost as quick as a direct rowid lookup (for which
            // sqlite uses an internal cost of 0.0). It is expected to return
            // a single row.
            unsafe {
                (*pIdxInfo).estimatedCost = 30.0f64;
            }
            unsafe {
                (*pIdxInfo).estimatedRows = (1 as i32) as i64;
            }
            unsafe {
                (*pIdxInfo).idxFlags = 1 as i32;
            }
            return 0 as i32;
        }
        if (unsafe { (*p).usable }) != (0 as u8)
            && ((unsafe { (*p).iColumn }) > (0 as i32)
                && (unsafe { (*p).iColumn }) <= (((unsafe { (*pRtree).nDim2 }) as u32) as i32)
                || (((unsafe { (*p).op }) as u32) as i32) == (64 as i32))
        {
            let mut op: u8 = 0 as u8;
            let mut doOmit: u8 = ((1 as i32) as i8) as u8;
            '__slate_break_1249: {
                match ((unsafe { (*p).op }) as u32) as i32 {
                    2 => {
                        op = ((65 as i32) as i8) as u8;
                        doOmit = ((0 as i32) as i8) as u8;
                    }
                    4 => {
                        op = ((69 as i32) as i8) as u8;
                        doOmit = ((0 as i32) as i8) as u8;
                    }
                    8 => {
                        op = ((66 as i32) as i8) as u8;
                    }
                    16 => {
                        op = ((67 as i32) as i8) as u8;
                        doOmit = ((0 as i32) as i8) as u8;
                    }
                    32 => {
                        op = ((68 as i32) as i8) as u8;
                    }
                    64 => {
                        op = ((70 as i32) as i8) as u8;
                    }
                    _ => {
                        op = ((0 as i32) as i8) as u8;
                    }
                }
            }
            if op != (0 as u8) {
                let __v1511: i32 = iIdx;
                let __v1512: i32 = __v1511 + (1 as i32);
                iIdx = __v1512;
                unsafe {
                    *unsafe { (zIdxStr.0.as_mut_ptr() as *mut i8).offset(__v1511 as isize) } =
                        op as i8;
                }
                let __v1513: i32 = iIdx;
                let __v1514: i32 = __v1513 + (1 as i32);
                iIdx = __v1514;
                unsafe {
                    *unsafe { (zIdxStr.0.as_mut_ptr() as *mut i8).offset(__v1513 as isize) } =
                        ((unsafe { (*p).iColumn }) - (1 as i32) + (48 as i32)) as i8;
                }
                unsafe {
                    (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(ii as isize) })
                        .argvIndex = iIdx / (2 as i32);
                }
                unsafe {
                    (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(ii as isize) })
                        .omit = doOmit;
                }
            }
        }
        let __v1507: i32 = ii;
        let __v1508: i32 = __v1507 + (1 as i32);
        ii = __v1508;
    }
    unsafe {
        (*pIdxInfo).idxNum = 2 as i32;
    }
    unsafe {
        (*pIdxInfo).needToFreeIdxStr = 1 as i32;
    }
    if iIdx > (0 as i32) {
        unsafe {
            (*pIdxInfo).idxStr = (unsafe { sqlite3_malloc(iIdx + (1 as i32)) }) as *mut i8;
        }
        if (unsafe { (*pIdxInfo).idxStr }) == std::ptr::null_mut::<i8>() {
            return 7 as i32;
        }
        unsafe {
            memcpy(
                (unsafe { (*pIdxInfo).idxStr }) as *mut (),
                (zIdxStr.0.as_mut_ptr() as *mut i8) as *const (),
                ((iIdx + (1 as i32)) as i64) as u64,
            )
        };
    }
    nRow = (unsafe { (*pRtree).nRowEst }) >> iIdx / (2 as i32);
    unsafe {
        (*pIdxInfo).estimatedCost = 6.0f64 * (nRow as f64);
    }
    unsafe {
        (*pIdxInfo).estimatedRows = nRow;
    }
    return rc;
}

/// Return the N-dimensional volume of the cell stored in *p.
fn cellArea(mut pRtree: *mut Rtree, mut p: *mut RtreeCell) -> f64 {
    let mut area: f64 = (1 as i32) as f64;
    0 as i32;
    if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (0 as i32) {
        match ((unsafe { (*pRtree).nDim }) as u32) as i32 {
            5 => {
                area = ((unsafe {
                    (*unsafe {
                        unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                            .offset((9 as i32) as isize)
                    })
                    .f
                }) - unsafe {
                    (*unsafe {
                        unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                            .offset((8 as i32) as isize)
                    })
                    .f
                }) as f64;
                let _v1531: f64 = area;
                let _v1532: f64 = _v1531
                    * (((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((7 as i32) as isize)
                        })
                        .f
                    }) - unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((6 as i32) as isize)
                        })
                        .f
                    }) as f64);
                area = _v1532;
                let _v1533: f64 = area;
                let _v1534: f64 = _v1533
                    * (((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((5 as i32) as isize)
                        })
                        .f
                    }) - unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((4 as i32) as isize)
                        })
                        .f
                    }) as f64);
                area = _v1534;
                let _v1535: f64 = area;
                let _v1536: f64 = _v1535
                    * (((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((3 as i32) as isize)
                        })
                        .f
                    }) - unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((2 as i32) as isize)
                        })
                        .f
                    }) as f64);
                area = _v1536;
                let _v1537: f64 = area;
                let _v1538: f64 = _v1537
                    * (((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((1 as i32) as isize)
                        })
                        .f
                    }) - unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((0 as i32) as isize)
                        })
                        .f
                    }) as f64);
                area = _v1538;
            }
            4 => {
                let __v1515: f64 = area;
                let __v1516: f64 = __v1515
                    * (((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((7 as i32) as isize)
                        })
                        .f
                    }) - unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((6 as i32) as isize)
                        })
                        .f
                    }) as f64);
                area = __v1516;
                let _v1539: f64 = area;
                let _v1540: f64 = _v1539
                    * (((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((5 as i32) as isize)
                        })
                        .f
                    }) - unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((4 as i32) as isize)
                        })
                        .f
                    }) as f64);
                area = _v1540;
                let _v1541: f64 = area;
                let _v1542: f64 = _v1541
                    * (((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((3 as i32) as isize)
                        })
                        .f
                    }) - unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((2 as i32) as isize)
                        })
                        .f
                    }) as f64);
                area = _v1542;
                let _v1543: f64 = area;
                let _v1544: f64 = _v1543
                    * (((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((1 as i32) as isize)
                        })
                        .f
                    }) - unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((0 as i32) as isize)
                        })
                        .f
                    }) as f64);
                area = _v1544;
            }
            3 => {
                let __v1517: f64 = area;
                let __v1518: f64 = __v1517
                    * (((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((5 as i32) as isize)
                        })
                        .f
                    }) - unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((4 as i32) as isize)
                        })
                        .f
                    }) as f64);
                area = __v1518;
                let _v1545: f64 = area;
                let _v1546: f64 = _v1545
                    * (((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((3 as i32) as isize)
                        })
                        .f
                    }) - unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((2 as i32) as isize)
                        })
                        .f
                    }) as f64);
                area = _v1546;
                let _v1547: f64 = area;
                let _v1548: f64 = _v1547
                    * (((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((1 as i32) as isize)
                        })
                        .f
                    }) - unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((0 as i32) as isize)
                        })
                        .f
                    }) as f64);
                area = _v1548;
            }
            2 => {
                let __v1519: f64 = area;
                let __v1520: f64 = __v1519
                    * (((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((3 as i32) as isize)
                        })
                        .f
                    }) - unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((2 as i32) as isize)
                        })
                        .f
                    }) as f64);
                area = __v1520;
                let _v1549: f64 = area;
                let _v1550: f64 = _v1549
                    * (((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((1 as i32) as isize)
                        })
                        .f
                    }) - unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((0 as i32) as isize)
                        })
                        .f
                    }) as f64);
                area = _v1550;
            }
            _ => {
                let __v1521: f64 = area;
                let __v1522: f64 = __v1521
                    * (((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((1 as i32) as isize)
                        })
                        .f
                    }) - unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((0 as i32) as isize)
                        })
                        .f
                    }) as f64);
                area = __v1522;
            }
        }
    } else {
        match ((unsafe { (*pRtree).nDim }) as u32) as i32 {
            5 => {
                area = (((unsafe {
                    (*unsafe {
                        unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                            .offset((9 as i32) as isize)
                    })
                    .i
                }) as i64)
                    - ((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((8 as i32) as isize)
                        })
                        .i
                    }) as i64)) as f64;
                let _v1531: f64 = area;
                let _v1532: f64 = _v1531
                    * ((((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((7 as i32) as isize)
                        })
                        .i
                    }) as i64)
                        - ((unsafe {
                            (*unsafe {
                                unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                    .offset((6 as i32) as isize)
                            })
                            .i
                        }) as i64)) as f64);
                area = _v1532;
                let _v1533: f64 = area;
                let _v1534: f64 = _v1533
                    * ((((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((5 as i32) as isize)
                        })
                        .i
                    }) as i64)
                        - ((unsafe {
                            (*unsafe {
                                unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                    .offset((4 as i32) as isize)
                            })
                            .i
                        }) as i64)) as f64);
                area = _v1534;
                let _v1535: f64 = area;
                let _v1536: f64 = _v1535
                    * ((((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((3 as i32) as isize)
                        })
                        .i
                    }) as i64)
                        - ((unsafe {
                            (*unsafe {
                                unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                    .offset((2 as i32) as isize)
                            })
                            .i
                        }) as i64)) as f64);
                area = _v1536;
                let _v1537: f64 = area;
                let _v1538: f64 = _v1537
                    * ((((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((1 as i32) as isize)
                        })
                        .i
                    }) as i64)
                        - ((unsafe {
                            (*unsafe {
                                unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                    .offset((0 as i32) as isize)
                            })
                            .i
                        }) as i64)) as f64);
                area = _v1538;
            }
            4 => {
                let __v1523: f64 = area;
                let __v1524: f64 = __v1523
                    * ((((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((7 as i32) as isize)
                        })
                        .i
                    }) as i64)
                        - ((unsafe {
                            (*unsafe {
                                unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                    .offset((6 as i32) as isize)
                            })
                            .i
                        }) as i64)) as f64);
                area = __v1524;
                let _v1539: f64 = area;
                let _v1540: f64 = _v1539
                    * ((((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((5 as i32) as isize)
                        })
                        .i
                    }) as i64)
                        - ((unsafe {
                            (*unsafe {
                                unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                    .offset((4 as i32) as isize)
                            })
                            .i
                        }) as i64)) as f64);
                area = _v1540;
                let _v1541: f64 = area;
                let _v1542: f64 = _v1541
                    * ((((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((3 as i32) as isize)
                        })
                        .i
                    }) as i64)
                        - ((unsafe {
                            (*unsafe {
                                unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                    .offset((2 as i32) as isize)
                            })
                            .i
                        }) as i64)) as f64);
                area = _v1542;
                let _v1543: f64 = area;
                let _v1544: f64 = _v1543
                    * ((((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((1 as i32) as isize)
                        })
                        .i
                    }) as i64)
                        - ((unsafe {
                            (*unsafe {
                                unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                    .offset((0 as i32) as isize)
                            })
                            .i
                        }) as i64)) as f64);
                area = _v1544;
            }
            3 => {
                let __v1525: f64 = area;
                let __v1526: f64 = __v1525
                    * ((((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((5 as i32) as isize)
                        })
                        .i
                    }) as i64)
                        - ((unsafe {
                            (*unsafe {
                                unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                    .offset((4 as i32) as isize)
                            })
                            .i
                        }) as i64)) as f64);
                area = __v1526;
                let _v1545: f64 = area;
                let _v1546: f64 = _v1545
                    * ((((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((3 as i32) as isize)
                        })
                        .i
                    }) as i64)
                        - ((unsafe {
                            (*unsafe {
                                unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                    .offset((2 as i32) as isize)
                            })
                            .i
                        }) as i64)) as f64);
                area = _v1546;
                let _v1547: f64 = area;
                let _v1548: f64 = _v1547
                    * ((((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((1 as i32) as isize)
                        })
                        .i
                    }) as i64)
                        - ((unsafe {
                            (*unsafe {
                                unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                    .offset((0 as i32) as isize)
                            })
                            .i
                        }) as i64)) as f64);
                area = _v1548;
            }
            2 => {
                let __v1527: f64 = area;
                let __v1528: f64 = __v1527
                    * ((((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((3 as i32) as isize)
                        })
                        .i
                    }) as i64)
                        - ((unsafe {
                            (*unsafe {
                                unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                    .offset((2 as i32) as isize)
                            })
                            .i
                        }) as i64)) as f64);
                area = __v1528;
                let _v1549: f64 = area;
                let _v1550: f64 = _v1549
                    * ((((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((1 as i32) as isize)
                        })
                        .i
                    }) as i64)
                        - ((unsafe {
                            (*unsafe {
                                unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                    .offset((0 as i32) as isize)
                            })
                            .i
                        }) as i64)) as f64);
                area = _v1550;
            }
            _ => {
                let __v1529: f64 = area;
                let __v1530: f64 = __v1529
                    * ((((unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((1 as i32) as isize)
                        })
                        .i
                    }) as i64)
                        - ((unsafe {
                            (*unsafe {
                                unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                    .offset((0 as i32) as isize)
                            })
                            .i
                        }) as i64)) as f64);
                area = __v1530;
            }
        }
    }
    return area;
}

/// Return the margin length of cell p. The margin length is the sum
/// of the objects size in each dimension.
fn cellMargin(mut pRtree: *mut Rtree, mut p: *mut RtreeCell) -> f64 {
    let mut margin: f64 = (0 as i32) as f64;
    let mut ii: i32 = (((unsafe { (*pRtree).nDim2 }) as u32) as i32) - (2 as i32);
    '__slate_break_1252: loop {
        let __v1531: f64 = margin;
        let __v1532: f64 = __v1531
            + ((if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (0 as i32) {
                (unsafe {
                    (*unsafe {
                        unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                            .offset((ii + (1 as i32)) as isize)
                    })
                    .f
                }) as f64
            } else {
                (unsafe {
                    (*unsafe {
                        unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                            .offset((ii + (1 as i32)) as isize)
                    })
                    .i
                }) as f64
            }) - if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (0 as i32) {
                (unsafe {
                    (*unsafe {
                        unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }.offset(ii as isize)
                    })
                    .f
                }) as f64
            } else {
                (unsafe {
                    (*unsafe {
                        unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }.offset(ii as isize)
                    })
                    .i
                }) as f64
            });
        margin = __v1532;
        let __v1533: i32 = ii;
        let __v1534: i32 = __v1533 - (2 as i32);
        ii = __v1534;
        if !(ii >= (0 as i32)) {
            break;
        }
    }
    return margin;
}

/// Store the union of cells p1 and p2 in p1.
fn cellUnion(mut pRtree: *mut Rtree, mut p1: *mut RtreeCell, mut p2: *mut RtreeCell) {
    let mut ii: i32 = 0 as i32;
    if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (0 as i32) {
        '__slate_break_1253: loop {
            unsafe {
                (*unsafe {
                    unsafe { (*p1).aCoord.as_mut_ptr() as *mut RtreeCoord }.offset(ii as isize)
                })
                .f = if (unsafe {
                    (*unsafe {
                        unsafe { (*p1).aCoord.as_mut_ptr() as *mut RtreeCoord }.offset(ii as isize)
                    })
                    .f
                }) > unsafe {
                    (*unsafe {
                        unsafe { (*p2).aCoord.as_mut_ptr() as *mut RtreeCoord }.offset(ii as isize)
                    })
                    .f
                } {
                    unsafe {
                        (*unsafe {
                            unsafe { (*p2).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset(ii as isize)
                        })
                        .f
                    }
                } else {
                    unsafe {
                        (*unsafe {
                            unsafe { (*p1).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset(ii as isize)
                        })
                        .f
                    }
                };
            }
            unsafe {
                (*unsafe {
                    unsafe { (*p1).aCoord.as_mut_ptr() as *mut RtreeCoord }
                        .offset((ii + (1 as i32)) as isize)
                })
                .f = if (unsafe {
                    (*unsafe {
                        unsafe { (*p1).aCoord.as_mut_ptr() as *mut RtreeCoord }
                            .offset((ii + (1 as i32)) as isize)
                    })
                    .f
                }) < unsafe {
                    (*unsafe {
                        unsafe { (*p2).aCoord.as_mut_ptr() as *mut RtreeCoord }
                            .offset((ii + (1 as i32)) as isize)
                    })
                    .f
                } {
                    unsafe {
                        (*unsafe {
                            unsafe { (*p2).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((ii + (1 as i32)) as isize)
                        })
                        .f
                    }
                } else {
                    unsafe {
                        (*unsafe {
                            unsafe { (*p1).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((ii + (1 as i32)) as isize)
                        })
                        .f
                    }
                };
            }
            let __v1535: i32 = ii;
            let __v1536: i32 = __v1535 + (2 as i32);
            ii = __v1536;
            if !(ii < (((unsafe { (*pRtree).nDim2 }) as u32) as i32)) {
                break;
            }
        }
    } else {
        '__slate_break_1254: loop {
            unsafe {
                (*unsafe {
                    unsafe { (*p1).aCoord.as_mut_ptr() as *mut RtreeCoord }.offset(ii as isize)
                })
                .i = if (unsafe {
                    (*unsafe {
                        unsafe { (*p1).aCoord.as_mut_ptr() as *mut RtreeCoord }.offset(ii as isize)
                    })
                    .i
                }) > unsafe {
                    (*unsafe {
                        unsafe { (*p2).aCoord.as_mut_ptr() as *mut RtreeCoord }.offset(ii as isize)
                    })
                    .i
                } {
                    unsafe {
                        (*unsafe {
                            unsafe { (*p2).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset(ii as isize)
                        })
                        .i
                    }
                } else {
                    unsafe {
                        (*unsafe {
                            unsafe { (*p1).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset(ii as isize)
                        })
                        .i
                    }
                };
            }
            unsafe {
                (*unsafe {
                    unsafe { (*p1).aCoord.as_mut_ptr() as *mut RtreeCoord }
                        .offset((ii + (1 as i32)) as isize)
                })
                .i = if (unsafe {
                    (*unsafe {
                        unsafe { (*p1).aCoord.as_mut_ptr() as *mut RtreeCoord }
                            .offset((ii + (1 as i32)) as isize)
                    })
                    .i
                }) < unsafe {
                    (*unsafe {
                        unsafe { (*p2).aCoord.as_mut_ptr() as *mut RtreeCoord }
                            .offset((ii + (1 as i32)) as isize)
                    })
                    .i
                } {
                    unsafe {
                        (*unsafe {
                            unsafe { (*p2).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((ii + (1 as i32)) as isize)
                        })
                        .i
                    }
                } else {
                    unsafe {
                        (*unsafe {
                            unsafe { (*p1).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((ii + (1 as i32)) as isize)
                        })
                        .i
                    }
                };
            }
            let __v1537: i32 = ii;
            let __v1538: i32 = __v1537 + (2 as i32);
            ii = __v1538;
            if !(ii < (((unsafe { (*pRtree).nDim2 }) as u32) as i32)) {
                break;
            }
        }
    }
}

/// Return true if the area covered by p2 is a subset of the area covered
/// by p1. False otherwise.
fn cellContains(mut pRtree: *mut Rtree, mut p1: *mut RtreeCell, mut p2: *mut RtreeCell) -> i32 {
    let mut ii: i32 = 0 as i32;
    if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (1 as i32) {
        ii = 0 as i32;
        '__slate_break_1255: loop {
            if !(ii < (((unsafe { (*pRtree).nDim2 }) as u32) as i32)) {
                break;
            }
            let mut a1: *mut RtreeCoord = unsafe {
                unsafe { (*p1).aCoord.as_mut_ptr() as *mut RtreeCoord }.offset(ii as isize)
            };
            let mut a2: *mut RtreeCoord = unsafe {
                unsafe { (*p2).aCoord.as_mut_ptr() as *mut RtreeCoord }.offset(ii as isize)
            };
            if (unsafe { (*unsafe { a2.offset((0 as i32) as isize) }).i })
                < unsafe { (*unsafe { a1.offset((0 as i32) as isize) }).i }
                || (unsafe { (*unsafe { a2.offset((1 as i32) as isize) }).i })
                    > unsafe { (*unsafe { a1.offset((1 as i32) as isize) }).i }
            {
                return 0 as i32;
            }
            let __v1539: i32 = ii;
            let __v1540: i32 = __v1539 + (2 as i32);
            ii = __v1540;
        }
    } else {
        ii = 0 as i32;
        '__slate_break_1256: loop {
            if !(ii < (((unsafe { (*pRtree).nDim2 }) as u32) as i32)) {
                break;
            }
            let mut a1: *mut RtreeCoord = unsafe {
                unsafe { (*p1).aCoord.as_mut_ptr() as *mut RtreeCoord }.offset(ii as isize)
            };
            let mut a2: *mut RtreeCoord = unsafe {
                unsafe { (*p2).aCoord.as_mut_ptr() as *mut RtreeCoord }.offset(ii as isize)
            };
            if (unsafe { (*unsafe { a2.offset((0 as i32) as isize) }).f })
                < unsafe { (*unsafe { a1.offset((0 as i32) as isize) }).f }
                || (unsafe { (*unsafe { a2.offset((1 as i32) as isize) }).f })
                    > unsafe { (*unsafe { a1.offset((1 as i32) as isize) }).f }
            {
                return 0 as i32;
            }
            let __v1541: i32 = ii;
            let __v1542: i32 = __v1541 + (2 as i32);
            ii = __v1542;
        }
    }
    return 1 as i32;
}

fn cellOverlap(
    mut pRtree: *mut Rtree,
    mut p: *mut RtreeCell,
    mut aCell: *mut RtreeCell,
    mut nCell: i32,
) -> f64 {
    let mut ii: i32 = 0 as i32;
    let mut overlap: f64 = 0.0f64;
    ii = 0 as i32;
    '__slate_break_1257: loop {
        if !(ii < nCell) {
            break;
        }
        let mut jj: i32 = 0 as i32;
        let mut o: f64 = (1 as i32) as f64;
        jj = 0 as i32;
        '__slate_break_1258: loop {
            if !(jj < (((unsafe { (*pRtree).nDim2 }) as u32) as i32)) {
                break;
            }
            let mut x1: f64 = 0 as f64;
            let mut x2: f64 = 0 as f64;
            x1 = if (if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (0 as i32) {
                (unsafe {
                    (*unsafe {
                        unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }.offset(jj as isize)
                    })
                    .f
                }) as f64
            } else {
                (unsafe {
                    (*unsafe {
                        unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }.offset(jj as isize)
                    })
                    .i
                }) as f64
            }) < if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (0 as i32) {
                (unsafe {
                    (*unsafe {
                        unsafe {
                            (*unsafe { aCell.offset(ii as isize) }).aCoord.as_mut_ptr()
                                as *mut RtreeCoord
                        }
                        .offset(jj as isize)
                    })
                    .f
                }) as f64
            } else {
                (unsafe {
                    (*unsafe {
                        unsafe {
                            (*unsafe { aCell.offset(ii as isize) }).aCoord.as_mut_ptr()
                                as *mut RtreeCoord
                        }
                        .offset(jj as isize)
                    })
                    .i
                }) as f64
            } {
                if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (0 as i32) {
                    (unsafe {
                        (*unsafe {
                            unsafe {
                                (*unsafe { aCell.offset(ii as isize) }).aCoord.as_mut_ptr()
                                    as *mut RtreeCoord
                            }
                            .offset(jj as isize)
                        })
                        .f
                    }) as f64
                } else {
                    (unsafe {
                        (*unsafe {
                            unsafe {
                                (*unsafe { aCell.offset(ii as isize) }).aCoord.as_mut_ptr()
                                    as *mut RtreeCoord
                            }
                            .offset(jj as isize)
                        })
                        .i
                    }) as f64
                }
            } else {
                if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (0 as i32) {
                    (unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset(jj as isize)
                        })
                        .f
                    }) as f64
                } else {
                    (unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset(jj as isize)
                        })
                        .i
                    }) as f64
                }
            };
            x2 = if (if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (0 as i32) {
                (unsafe {
                    (*unsafe {
                        unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                            .offset((jj + (1 as i32)) as isize)
                    })
                    .f
                }) as f64
            } else {
                (unsafe {
                    (*unsafe {
                        unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                            .offset((jj + (1 as i32)) as isize)
                    })
                    .i
                }) as f64
            }) > if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (0 as i32) {
                (unsafe {
                    (*unsafe {
                        unsafe {
                            (*unsafe { aCell.offset(ii as isize) }).aCoord.as_mut_ptr()
                                as *mut RtreeCoord
                        }
                        .offset((jj + (1 as i32)) as isize)
                    })
                    .f
                }) as f64
            } else {
                (unsafe {
                    (*unsafe {
                        unsafe {
                            (*unsafe { aCell.offset(ii as isize) }).aCoord.as_mut_ptr()
                                as *mut RtreeCoord
                        }
                        .offset((jj + (1 as i32)) as isize)
                    })
                    .i
                }) as f64
            } {
                if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (0 as i32) {
                    (unsafe {
                        (*unsafe {
                            unsafe {
                                (*unsafe { aCell.offset(ii as isize) }).aCoord.as_mut_ptr()
                                    as *mut RtreeCoord
                            }
                            .offset((jj + (1 as i32)) as isize)
                        })
                        .f
                    }) as f64
                } else {
                    (unsafe {
                        (*unsafe {
                            unsafe {
                                (*unsafe { aCell.offset(ii as isize) }).aCoord.as_mut_ptr()
                                    as *mut RtreeCoord
                            }
                            .offset((jj + (1 as i32)) as isize)
                        })
                        .i
                    }) as f64
                }
            } else {
                if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (0 as i32) {
                    (unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((jj + (1 as i32)) as isize)
                        })
                        .f
                    }) as f64
                } else {
                    (unsafe {
                        (*unsafe {
                            unsafe { (*p).aCoord.as_mut_ptr() as *mut RtreeCoord }
                                .offset((jj + (1 as i32)) as isize)
                        })
                        .i
                    }) as f64
                }
            };
            if x2 < x1 {
                o = (0 as i32) as f64;
                break '__slate_break_1258;
            } else {
                o = o * (x2 - x1);
            }
            let __v1545: i32 = jj;
            let __v1546: i32 = __v1545 + (2 as i32);
            jj = __v1546;
        }
        let __v1547: f64 = overlap;
        let __v1548: f64 = __v1547 + o;
        overlap = __v1548;
        let __v1543: i32 = ii;
        let __v1544: i32 = __v1543 + (1 as i32);
        ii = __v1544;
    }
    return overlap;
}

/// This function implements the ChooseLeaf algorithm from Gutman[84].
/// ChooseSubTree in r*tree terminology.
///
/// # Arguments
///
/// * `pRtree` - Rtree table
/// * `pCell` - Cell to insert into rtree
/// * `iHeight` - Height of sub-tree rooted at pCell
/// * `ppLeaf` - OUT: Selected leaf page
fn ChooseLeaf(
    mut pRtree: *mut Rtree,
    mut pCell: *mut RtreeCell,
    mut iHeight: i32,
    mut ppLeaf: *mut *mut RtreeNode,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut ii: i32 = 0 as i32;
    let mut pNode: *mut RtreeNode = std::ptr::null_mut::<RtreeNode>();
    rc = nodeAcquire(
        pRtree,
        (1 as i32) as i64,
        std::ptr::null_mut::<RtreeNode>(),
        std::ptr::addr_of_mut!(pNode),
    );
    ii = 0 as i32;
    '__slate_break_1259: loop {
        if !(rc == (0 as i32) && ii < (unsafe { (*pRtree).iDepth }) - iHeight) {
            break;
        }
        let mut iCell: i32 = 0 as i32;
        let mut iBest: i64 = (0 as i32) as i64;
        let mut bFound: i32 = 0 as i32;
        let mut fMinGrowth: f64 = 0.0f64;
        let mut fMinArea: f64 = 0.0f64;
        let mut nCell: i32 =
            readInt16(unsafe { unsafe { (*pNode).zData }.offset((2 as i32) as isize) });
        let mut pChild: *mut RtreeNode = std::ptr::null_mut::<RtreeNode>();
        // First check to see if there is are any cells in pNode that completely
        // contains pCell.  If two or more cells in pNode completely contain pCell
        // then pick the smallest.
        iCell = 0 as i32;
        '__slate_break_1260: loop {
            if !(iCell < nCell) {
                break;
            }
            let mut cell: RtreeCell = unsafe { std::mem::zeroed() };
            nodeGetCell(pRtree, pNode, iCell, std::ptr::addr_of_mut!(cell));
            if cellContains(pRtree, std::ptr::addr_of_mut!(cell), pCell) != (0 as i32) {
                let mut area: f64 = cellArea(pRtree, std::ptr::addr_of_mut!(cell));
                if bFound == (0 as i32) || area < fMinArea {
                    iBest = cell.iRowid;
                    fMinArea = area;
                    bFound = 1 as i32;
                }
            }
            let __v1551: i32 = iCell;
            let __v1552: i32 = __v1551 + (1 as i32);
            iCell = __v1552;
        }
        if !(bFound != (0 as i32)) {
            // No cells of pNode will completely contain pCell.  So pick the
            // cell of pNode that grows by the least amount when pCell is added.
            // Break ties by selecting the smaller cell.
            iCell = 0 as i32;
            '__slate_break_1261: loop {
                if !(iCell < nCell) {
                    break;
                }
                let mut cell: RtreeCell = unsafe { std::mem::zeroed() };
                let mut growth: f64 = 0 as f64;
                let mut area: f64 = 0 as f64;
                nodeGetCell(pRtree, pNode, iCell, std::ptr::addr_of_mut!(cell));
                area = cellArea(pRtree, std::ptr::addr_of_mut!(cell));
                cellUnion(pRtree, std::ptr::addr_of_mut!(cell), pCell);
                growth = cellArea(pRtree, std::ptr::addr_of_mut!(cell)) - area;
                if iCell == (0 as i32)
                    || growth < fMinGrowth
                    || growth == fMinGrowth && area < fMinArea
                {
                    fMinGrowth = growth;
                    fMinArea = area;
                    iBest = cell.iRowid;
                }
                let __v1553: i32 = iCell;
                let __v1554: i32 = __v1553 + (1 as i32);
                iCell = __v1554;
            }
        }
        rc = nodeAcquire(pRtree, iBest, pNode, std::ptr::addr_of_mut!(pChild));
        nodeRelease(pRtree, pNode);
        pNode = pChild;
        let __v1549: i32 = ii;
        let __v1550: i32 = __v1549 + (1 as i32);
        ii = __v1550;
    }
    unsafe {
        *ppLeaf = pNode;
    }
    return rc;
}

/// A cell with the same content as pCell has just been inserted into
/// the node pNode. This function updates the bounding box cells in
/// all ancestor elements.
///
/// # Arguments
///
/// * `pRtree` - Rtree table
/// * `pNode` - Adjust ancestry of this node.
/// * `pCell` - This cell was just inserted
fn AdjustTree(mut pRtree: *mut Rtree, mut pNode: *mut RtreeNode, mut pCell: *mut RtreeCell) -> i32 {
    let mut p: *mut RtreeNode = pNode;
    let mut cnt: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    '__slate_break_1262: while (unsafe { (*p).pParent }) != std::ptr::null_mut::<RtreeNode>() {
        let mut pParent: *mut RtreeNode = unsafe { (*p).pParent };
        let mut cell: RtreeCell = unsafe { std::mem::zeroed() };
        let mut iCell: i32 = 0 as i32;
        let __v1555: i32 = cnt;
        let __v1556: i32 = __v1555 + (1 as i32);
        cnt = __v1556;
        if cnt > (100 as i32) {
            {}
            return (11 as i32) | (1 as i32) << (8 as i32);
        }
        rc = nodeParentIndex(pRtree, p, std::ptr::addr_of_mut!(iCell));
        if rc != (0 as i32) {
            {}
            return (11 as i32) | (1 as i32) << (8 as i32);
        }
        nodeGetCell(pRtree, pParent, iCell, std::ptr::addr_of_mut!(cell));
        if !(cellContains(pRtree, std::ptr::addr_of_mut!(cell), pCell) != (0 as i32)) {
            cellUnion(pRtree, std::ptr::addr_of_mut!(cell), pCell);
            nodeOverwriteCell(pRtree, pParent, std::ptr::addr_of_mut!(cell), iCell);
        }
        p = pParent;
    }
    return 0 as i32;
}

/// Write mapping (iRowid->iNode) to the <rtree>_rowid table.
#[unsafe(link_section = ".text.slate_distinct.rtree.rowidWrite")]
extern "C-unwind" fn rowidWrite(mut pRtree: *mut Rtree, mut iRowid: i64, mut iNode: i64) -> i32 {
    unsafe { sqlite3_bind_int64(unsafe { (*pRtree).pWriteRowid }, 1 as i32, iRowid) };
    unsafe { sqlite3_bind_int64(unsafe { (*pRtree).pWriteRowid }, 2 as i32, iNode) };
    unsafe { sqlite3_step(unsafe { (*pRtree).pWriteRowid }) };
    return unsafe { sqlite3_reset(unsafe { (*pRtree).pWriteRowid }) };
}

/// Write mapping (iNode->iPar) to the <rtree>_parent table.
#[unsafe(link_section = ".text.slate_distinct.rtree.parentWrite")]
extern "C-unwind" fn parentWrite(mut pRtree: *mut Rtree, mut iNode: i64, mut iPar: i64) -> i32 {
    unsafe { sqlite3_bind_int64(unsafe { (*pRtree).pWriteParent }, 1 as i32, iNode) };
    unsafe { sqlite3_bind_int64(unsafe { (*pRtree).pWriteParent }, 2 as i32, iPar) };
    unsafe { sqlite3_step(unsafe { (*pRtree).pWriteParent }) };
    return unsafe { sqlite3_reset(unsafe { (*pRtree).pWriteParent }) };
}

/// Arguments aIdx, aCell and aSpare all point to arrays of size
/// nIdx. The aIdx array contains the set of integers from 0 to
/// (nIdx-1) in no particular order. This function sorts the values
/// in aIdx according to dimension iDim of the cells in aCell. The
/// minimum value of dimension iDim is considered first, the
/// maximum used to break ties.
///
/// The aSpare array is used as temporary working space by the
/// sorting algorithm.
fn SortByDimension(
    mut pRtree: *mut Rtree,
    mut aIdx: *mut i32,
    mut nIdx: i32,
    mut iDim: i32,
    mut aCell: *mut RtreeCell,
    mut aSpare: *mut i32,
) {
    if nIdx > (1 as i32) {
        let mut iLeft: i32 = 0 as i32;
        let mut iRight: i32 = 0 as i32;
        let mut nLeft: i32 = nIdx / (2 as i32);
        let mut nRight: i32 = nIdx - nLeft;
        let mut aLeft: *mut i32 = aIdx;
        let mut aRight: *mut i32 = unsafe { aIdx.offset(nLeft as isize) };
        SortByDimension(pRtree, aLeft, nLeft, iDim, aCell, aSpare);
        SortByDimension(pRtree, aRight, nRight, iDim, aCell, aSpare);
        unsafe {
            memcpy(
                aSpare as *mut (),
                aLeft as *const (),
                (4 as u64).wrapping_mul((nLeft as i64) as u64),
            )
        };
        aLeft = aSpare;
        '__slate_break_1267: while iLeft < nLeft || iRight < nRight {
            let mut xleft1: f64 = if (((unsafe { (*pRtree).eCoordType }) as u32) as i32)
                == (0 as i32)
            {
                (unsafe {
                    (*unsafe {
                        unsafe {
                            (*unsafe {
                                aCell.offset(
                                    (unsafe { *unsafe { aLeft.offset(iLeft as isize) } }) as isize,
                                )
                            })
                            .aCoord
                            .as_mut_ptr() as *mut RtreeCoord
                        }
                        .offset((iDim * (2 as i32)) as isize)
                    })
                    .f
                }) as f64
            } else {
                (unsafe {
                    (*unsafe {
                        unsafe {
                            (*unsafe {
                                aCell.offset(
                                    (unsafe { *unsafe { aLeft.offset(iLeft as isize) } }) as isize,
                                )
                            })
                            .aCoord
                            .as_mut_ptr() as *mut RtreeCoord
                        }
                        .offset((iDim * (2 as i32)) as isize)
                    })
                    .i
                }) as f64
            };
            let mut xleft2: f64 = if (((unsafe { (*pRtree).eCoordType }) as u32) as i32)
                == (0 as i32)
            {
                (unsafe {
                    (*unsafe {
                        unsafe {
                            (*unsafe {
                                aCell.offset(
                                    (unsafe { *unsafe { aLeft.offset(iLeft as isize) } }) as isize,
                                )
                            })
                            .aCoord
                            .as_mut_ptr() as *mut RtreeCoord
                        }
                        .offset((iDim * (2 as i32) + (1 as i32)) as isize)
                    })
                    .f
                }) as f64
            } else {
                (unsafe {
                    (*unsafe {
                        unsafe {
                            (*unsafe {
                                aCell.offset(
                                    (unsafe { *unsafe { aLeft.offset(iLeft as isize) } }) as isize,
                                )
                            })
                            .aCoord
                            .as_mut_ptr() as *mut RtreeCoord
                        }
                        .offset((iDim * (2 as i32) + (1 as i32)) as isize)
                    })
                    .i
                }) as f64
            };
            let mut xright1: f64 =
                if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (0 as i32) {
                    (unsafe {
                        (*unsafe {
                            unsafe {
                                (*unsafe {
                                    aCell.offset(
                                        (unsafe { *unsafe { aRight.offset(iRight as isize) } })
                                            as isize,
                                    )
                                })
                                .aCoord
                                .as_mut_ptr() as *mut RtreeCoord
                            }
                            .offset((iDim * (2 as i32)) as isize)
                        })
                        .f
                    }) as f64
                } else {
                    (unsafe {
                        (*unsafe {
                            unsafe {
                                (*unsafe {
                                    aCell.offset(
                                        (unsafe { *unsafe { aRight.offset(iRight as isize) } })
                                            as isize,
                                    )
                                })
                                .aCoord
                                .as_mut_ptr() as *mut RtreeCoord
                            }
                            .offset((iDim * (2 as i32)) as isize)
                        })
                        .i
                    }) as f64
                };
            let mut xright2: f64 =
                if (((unsafe { (*pRtree).eCoordType }) as u32) as i32) == (0 as i32) {
                    (unsafe {
                        (*unsafe {
                            unsafe {
                                (*unsafe {
                                    aCell.offset(
                                        (unsafe { *unsafe { aRight.offset(iRight as isize) } })
                                            as isize,
                                    )
                                })
                                .aCoord
                                .as_mut_ptr() as *mut RtreeCoord
                            }
                            .offset((iDim * (2 as i32) + (1 as i32)) as isize)
                        })
                        .f
                    }) as f64
                } else {
                    (unsafe {
                        (*unsafe {
                            unsafe {
                                (*unsafe {
                                    aCell.offset(
                                        (unsafe { *unsafe { aRight.offset(iRight as isize) } })
                                            as isize,
                                    )
                                })
                                .aCoord
                                .as_mut_ptr() as *mut RtreeCoord
                            }
                            .offset((iDim * (2 as i32) + (1 as i32)) as isize)
                        })
                        .i
                    }) as f64
                };
            if iLeft != nLeft
                && (iRight == nRight || xleft1 < xright1 || xleft1 == xright1 && xleft2 < xright2)
            {
                unsafe {
                    *unsafe { aIdx.offset((iLeft + iRight) as isize) } =
                        unsafe { *unsafe { aLeft.offset(iLeft as isize) } };
                }
                let __v1557: i32 = iLeft;
                let __v1558: i32 = __v1557 + (1 as i32);
                iLeft = __v1558;
            } else {
                unsafe {
                    *unsafe { aIdx.offset((iLeft + iRight) as isize) } =
                        unsafe { *unsafe { aRight.offset(iRight as isize) } };
                }
                let __v1559: i32 = iRight;
                let __v1560: i32 = __v1559 + (1 as i32);
                iRight = __v1560;
            }
        }
    }
}

/// Implementation of the R*-tree variant of SplitNode from Beckman[1990].
fn splitNodeStartree(
    mut pRtree: *mut Rtree,
    mut aCell: *mut RtreeCell,
    mut nCell: i32,
    mut pLeft: *mut RtreeNode,
    mut pRight: *mut RtreeNode,
    mut pBboxLeft: *mut RtreeCell,
    mut pBboxRight: *mut RtreeCell,
) -> i32 {
    let mut aaSorted: *mut *mut i32 = unsafe { std::mem::zeroed() };
    let mut aSpare: *mut i32 = unsafe { std::mem::zeroed() };
    let mut ii: i32 = 0 as i32;
    let mut iBestDim: i32 = 0 as i32;
    let mut iBestSplit: i32 = 0 as i32;
    let mut fBestMargin: f64 = 0.0f64;
    let mut nByte: i64 = ((((((unsafe { (*pRtree).nDim }) as u32) as i32) + (1 as i32)) as i64)
        as u64)
        .wrapping_mul((8 as u64).wrapping_add(((nCell as i64) as u64).wrapping_mul(4 as u64)))
        as i64;
    aaSorted = (unsafe { sqlite3_malloc64(nByte as u64) }) as *mut *mut i32;
    if !(aaSorted != std::ptr::null_mut::<*mut i32>()) {
        return 7 as i32;
    }
    aSpare = unsafe {
        ((unsafe { aaSorted.offset((((unsafe { (*pRtree).nDim }) as u32) as i32) as isize) })
            as *mut i32)
            .offset(((((unsafe { (*pRtree).nDim }) as u32) as i32) * nCell) as isize)
    };
    unsafe { memset(aaSorted as *mut (), 0 as i32, nByte as u64) };
    ii = 0 as i32;
    '__slate_break_1268: loop {
        if !(ii < (((unsafe { (*pRtree).nDim }) as u32) as i32)) {
            break;
        }
        let mut jj: i32 = 0 as i32;
        unsafe {
            *unsafe { aaSorted.offset(ii as isize) } = unsafe {
                ((unsafe {
                    aaSorted.offset((((unsafe { (*pRtree).nDim }) as u32) as i32) as isize)
                }) as *mut i32)
                    .offset((ii * nCell) as isize)
            };
        }
        jj = 0 as i32;
        '__slate_break_1269: loop {
            if !(jj < nCell) {
                break;
            }
            unsafe {
                *unsafe {
                    unsafe { *unsafe { aaSorted.offset(ii as isize) } }.offset(jj as isize)
                } = jj;
            }
            let __v1563: i32 = jj;
            let __v1564: i32 = __v1563 + (1 as i32);
            jj = __v1564;
        }
        SortByDimension(
            pRtree,
            unsafe { *unsafe { aaSorted.offset(ii as isize) } },
            nCell,
            ii,
            aCell,
            aSpare,
        );
        let __v1561: i32 = ii;
        let __v1562: i32 = __v1561 + (1 as i32);
        ii = __v1562;
    }
    ii = 0 as i32;
    '__slate_break_1270: loop {
        if !(ii < (((unsafe { (*pRtree).nDim }) as u32) as i32)) {
            break;
        }
        let mut margin: f64 = 0.0f64;
        let mut fBestOverlap: f64 = 0.0f64;
        let mut fBestArea: f64 = 0.0f64;
        let mut iBestLeft: i32 = 0 as i32;
        let mut nLeft: i32 = 0 as i32;
        nLeft = ((unsafe { (*pRtree).iNodeSize }) - (4 as i32))
            / (((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32)
            / (3 as i32);
        '__slate_break_1271: loop {
            if !(nLeft
                <= nCell
                    - ((unsafe { (*pRtree).iNodeSize }) - (4 as i32))
                        / (((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32)
                        / (3 as i32))
            {
                break;
            }
            let mut left: RtreeCell = unsafe { std::mem::zeroed() };
            let mut right: RtreeCell = unsafe { std::mem::zeroed() };
            let mut kk: i32 = 0 as i32;
            let mut overlap: f64 = 0 as f64;
            let mut area: f64 = 0 as f64;
            unsafe {
                memcpy(
                    std::ptr::addr_of_mut!(left) as *mut (),
                    (unsafe {
                        aCell.offset(
                            (unsafe {
                                *unsafe {
                                    unsafe { *unsafe { aaSorted.offset(ii as isize) } }
                                        .offset((0 as i32) as isize)
                                }
                            }) as isize,
                        )
                    }) as *const (),
                    48 as u64,
                )
            };
            unsafe {
                memcpy(
                    std::ptr::addr_of_mut!(right) as *mut (),
                    (unsafe {
                        aCell.offset(
                            (unsafe {
                                *unsafe {
                                    unsafe { *unsafe { aaSorted.offset(ii as isize) } }
                                        .offset((nCell - (1 as i32)) as isize)
                                }
                            }) as isize,
                        )
                    }) as *const (),
                    48 as u64,
                )
            };
            kk = 1 as i32;
            '__slate_break_1272: loop {
                if !(kk < nCell - (1 as i32)) {
                    break;
                }
                if kk < nLeft {
                    cellUnion(pRtree, std::ptr::addr_of_mut!(left), unsafe {
                        aCell.offset(
                            (unsafe {
                                *unsafe {
                                    unsafe { *unsafe { aaSorted.offset(ii as isize) } }
                                        .offset(kk as isize)
                                }
                            }) as isize,
                        )
                    });
                } else {
                    cellUnion(pRtree, std::ptr::addr_of_mut!(right), unsafe {
                        aCell.offset(
                            (unsafe {
                                *unsafe {
                                    unsafe { *unsafe { aaSorted.offset(ii as isize) } }
                                        .offset(kk as isize)
                                }
                            }) as isize,
                        )
                    });
                }
                let __v1569: i32 = kk;
                let __v1570: i32 = __v1569 + (1 as i32);
                kk = __v1570;
            }
            let __v1571: f64 = margin;
            let __v1572: f64 = __v1571 + cellMargin(pRtree, std::ptr::addr_of_mut!(left));
            margin = __v1572;
            let __v1573: f64 = margin;
            let __v1574: f64 = __v1573 + cellMargin(pRtree, std::ptr::addr_of_mut!(right));
            margin = __v1574;
            overlap = cellOverlap(
                pRtree,
                std::ptr::addr_of_mut!(left),
                std::ptr::addr_of_mut!(right),
                1 as i32,
            );
            area = cellArea(pRtree, std::ptr::addr_of_mut!(left))
                + cellArea(pRtree, std::ptr::addr_of_mut!(right));
            if nLeft
                == ((unsafe { (*pRtree).iNodeSize }) - (4 as i32))
                    / (((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32)
                    / (3 as i32)
                || overlap < fBestOverlap
                || overlap == fBestOverlap && area < fBestArea
            {
                iBestLeft = nLeft;
                fBestOverlap = overlap;
                fBestArea = area;
            }
            let __v1567: i32 = nLeft;
            let __v1568: i32 = __v1567 + (1 as i32);
            nLeft = __v1568;
        }
        if ii == (0 as i32) || margin < fBestMargin {
            iBestDim = ii;
            fBestMargin = margin;
            iBestSplit = iBestLeft;
        }
        let __v1565: i32 = ii;
        let __v1566: i32 = __v1565 + (1 as i32);
        ii = __v1566;
    }
    unsafe {
        memcpy(
            pBboxLeft as *mut (),
            (unsafe {
                aCell.offset(
                    (unsafe {
                        *unsafe {
                            unsafe { *unsafe { aaSorted.offset(iBestDim as isize) } }
                                .offset((0 as i32) as isize)
                        }
                    }) as isize,
                )
            }) as *const (),
            48 as u64,
        )
    };
    unsafe {
        memcpy(
            pBboxRight as *mut (),
            (unsafe {
                aCell.offset(
                    (unsafe {
                        *unsafe {
                            unsafe { *unsafe { aaSorted.offset(iBestDim as isize) } }
                                .offset(iBestSplit as isize)
                        }
                    }) as isize,
                )
            }) as *const (),
            48 as u64,
        )
    };
    ii = 0 as i32;
    '__slate_break_1273: loop {
        if !(ii < nCell) {
            break;
        }
        let mut pTarget: *mut RtreeNode = if ii < iBestSplit { pLeft } else { pRight };
        let mut pBbox: *mut RtreeCell = if ii < iBestSplit {
            pBboxLeft
        } else {
            pBboxRight
        };
        let mut pCell: *mut RtreeCell = unsafe {
            aCell.offset(
                (unsafe {
                    *unsafe {
                        unsafe { *unsafe { aaSorted.offset(iBestDim as isize) } }
                            .offset(ii as isize)
                    }
                }) as isize,
            )
        };
        nodeInsertCell(pRtree, pTarget, pCell);
        cellUnion(pRtree, pBbox, pCell);
        let __v1575: i32 = ii;
        let __v1576: i32 = __v1575 + (1 as i32);
        ii = __v1576;
    }
    unsafe { sqlite3_free(aaSorted as *mut ()) };
    return 0 as i32;
}

fn updateMapping(
    mut pRtree: *mut Rtree,
    mut iRowid: i64,
    mut pNode: *mut RtreeNode,
    mut iHeight: i32,
) -> i32 {
    let mut xSetMapping: Option<unsafe extern "C-unwind" fn(*mut Rtree, i64, i64) -> i32> =
        unsafe { std::mem::zeroed() };
    xSetMapping = {
        let __t0: Option<unsafe extern "C-unwind" fn(*mut Rtree, i64, i64) -> i32> =
            if iHeight == (0 as i32) {
                Some(rowidWrite)
            } else {
                Some(parentWrite)
            };
        __t0
    };
    if iHeight > (0 as i32) {
        let mut pChild: *mut RtreeNode = nodeHashLookup(pRtree, iRowid);
        let mut p: *mut RtreeNode = unsafe { std::mem::zeroed() };
        p = pNode;
        '__slate_break_1274: while p != std::ptr::null_mut::<RtreeNode>() {
            if p == pChild {
                return (11 as i32) | (1 as i32) << (8 as i32);
            }
            p = unsafe { (*p).pParent };
        }
        if pChild != std::ptr::null_mut::<RtreeNode>() {
            nodeRelease(pRtree, unsafe { (*pChild).pParent });
            nodeReference(pNode);
            unsafe {
                (*pChild).pParent = pNode;
            }
        }
    }
    if pNode == std::ptr::null_mut::<RtreeNode>() {
        return 1 as i32;
    }
    return unsafe { xSetMapping.unwrap()(pRtree, iRowid, unsafe { (*pNode).iNode }) };
}

fn SplitNode(
    mut pRtree: *mut Rtree,
    mut pNode: *mut RtreeNode,
    mut pCell: *mut RtreeCell,
    mut iHeight: i32,
) -> i32 {
    let mut __slate_storage_1595: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1595: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1595) as *mut i32;
    let mut __slate_storage_1594: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1594: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1594) as *mut i32;
    let mut __slate_storage_718: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_718: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_718) as *mut i64;
    let mut __slate_storage_1593: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1593: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1593) as *mut i32;
    let mut __slate_storage_1592: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1592: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1592) as *mut i32;
    let mut __slate_storage_717: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_717: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_717) as *mut i64;
    let mut __slate_storage_1591: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1591: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1591) as *mut i32;
    let mut __slate_storage_716: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_716: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_716) as *mut i32;
    let mut __slate_storage_715: std::mem::MaybeUninit<*mut RtreeNode> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_715: *mut *mut RtreeNode =
        std::ptr::addr_of_mut!(__slate_storage_715) as *mut *mut RtreeNode;
    let mut __slate_storage_1590: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1590: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1590) as *mut i32;
    let mut __slate_storage_1589: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1589: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1589) as *mut bool;
    let mut __slate_storage_1588: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1588: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1588) as *mut bool;
    // Ensure both child nodes have node numbers assigned to them by calling
    // nodeWrite(). Node pRight always needs a node number, as it was created
    // by nodeNew() above. But node pLeft sometimes already has a node number.
    // In this case avoid the all to nodeWrite().
    let mut __slate_storage_1587: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1587: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1587) as *mut i32;
    let mut __slate_storage_1583: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1583: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1583) as *mut i32;
    let mut __slate_storage_1582: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1582: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1582) as *mut i32;
    let mut __slate_storage_1581: std::mem::MaybeUninit<*mut Rtree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1581: *mut *mut Rtree =
        std::ptr::addr_of_mut!(__slate_storage_1581) as *mut *mut Rtree;
    let mut __slate_storage_1586: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1586: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1586) as *mut i32;
    let mut __slate_storage_1585: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1585: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1585) as *mut i32;
    let mut __slate_storage_1584: std::mem::MaybeUninit<*mut RtreeNode> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1584: *mut *mut RtreeNode =
        std::ptr::addr_of_mut!(__slate_storage_1584) as *mut *mut RtreeNode;
    let mut __slate_storage_1580: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1580: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1580) as *mut i32;
    let mut __slate_storage_1579: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1579: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1579) as *mut i32;
    let mut __slate_storage_1578: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1578: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1578) as *mut i32;
    let mut __slate_storage_1577: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1577: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1577) as *mut i32;
    let mut __slate_storage_714: std::mem::MaybeUninit<RtreeCell> = std::mem::MaybeUninit::uninit();
    let __slate_slot_714: *mut RtreeCell =
        std::ptr::addr_of_mut!(__slate_storage_714) as *mut RtreeCell;
    let mut __slate_storage_713: std::mem::MaybeUninit<RtreeCell> = std::mem::MaybeUninit::uninit();
    let __slate_slot_713: *mut RtreeCell =
        std::ptr::addr_of_mut!(__slate_storage_713) as *mut RtreeCell;
    let mut __slate_storage_712: std::mem::MaybeUninit<*mut RtreeNode> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_712: *mut *mut RtreeNode =
        std::ptr::addr_of_mut!(__slate_storage_712) as *mut *mut RtreeNode;
    let mut __slate_storage_711: std::mem::MaybeUninit<*mut RtreeNode> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_711: *mut *mut RtreeNode =
        std::ptr::addr_of_mut!(__slate_storage_711) as *mut *mut RtreeNode;
    let mut __slate_storage_710: std::mem::MaybeUninit<*mut i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_710: *mut *mut i32 =
        std::ptr::addr_of_mut!(__slate_storage_710) as *mut *mut i32;
    let mut __slate_storage_709: std::mem::MaybeUninit<*mut RtreeCell> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_709: *mut *mut RtreeCell =
        std::ptr::addr_of_mut!(__slate_storage_709) as *mut *mut RtreeCell;
    let mut __slate_storage_708: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_708: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_708) as *mut i32;
    let mut __slate_storage_707: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_707: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_707) as *mut i32;
    let mut __slate_storage_706: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_706: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_706) as *mut i32;
    let mut __slate_storage_705: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_705: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_705) as *mut i32;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_706, 0 as i32);
            std::ptr::write(__slate_slot_707, 0 as i32);
            std::ptr::write(
                __slate_slot_708,
                readInt16(unsafe { unsafe { (*pNode).zData }.offset((2 as i32) as isize) }),
            );
            std::ptr::write(__slate_slot_711, std::ptr::null_mut::<RtreeNode>());
            std::ptr::write(__slate_slot_712, std::ptr::null_mut::<RtreeNode>());
            // Allocate an array and populate it with a copy of pCell and
            // all cells from node pLeft. Then zero the original node.
            *__slate_slot_709 = (unsafe {
                sqlite3_malloc64(
                    (48 as u64)
                        .wrapping_add(4 as u64)
                        .wrapping_mul(((*__slate_slot_708 + (1 as i32)) as i64) as u64),
                )
            }) as *mut RtreeCell;
            if !(*__slate_slot_709 != std::ptr::null_mut::<RtreeCell>()) {
                *__slate_slot_707 = 7 as i32;
            } else {
                *__slate_slot_710 = (unsafe {
                    (*__slate_slot_709).offset((*__slate_slot_708 + (1 as i32)) as isize)
                }) as *mut i32;
                unsafe {
                    memset(
                        *__slate_slot_710 as *mut (),
                        0 as i32,
                        (4 as u64).wrapping_mul(((*__slate_slot_708 + (1 as i32)) as i64) as u64),
                    )
                };
                *__slate_slot_705 = 0 as i32;
                loop {
                    if *__slate_slot_705 < *__slate_slot_708 {
                        nodeGetCell(pRtree, pNode, *__slate_slot_705, unsafe {
                            (*__slate_slot_709).offset(*__slate_slot_705 as isize)
                        });
                        std::ptr::write(__slate_slot_1577, *__slate_slot_705);
                        std::ptr::write(__slate_slot_1578, *__slate_slot_1577 + (1 as i32));
                        *__slate_slot_705 = *__slate_slot_1578;
                    } else {
                        break;
                    }
                }
                nodeZero(pRtree, pNode);
                unsafe {
                    memcpy(
                        (unsafe { (*__slate_slot_709).offset(*__slate_slot_708 as isize) })
                            as *mut (),
                        pCell as *const (),
                        48 as u64,
                    )
                };
                std::ptr::write(__slate_slot_1579, *__slate_slot_708);
                std::ptr::write(__slate_slot_1580, *__slate_slot_1579 + (1 as i32));
                *__slate_slot_708 = *__slate_slot_1580;
                if (unsafe { (*pNode).iNode }) == ((1 as i32) as i64) {
                    *__slate_slot_712 = nodeNew(pRtree, pNode);
                    *__slate_slot_711 = nodeNew(pRtree, pNode);
                    std::ptr::write(__slate_slot_1581, pRtree);
                    std::ptr::write(__slate_slot_1582, unsafe { (*(*__slate_slot_1581)).iDepth });
                    std::ptr::write(__slate_slot_1583, *__slate_slot_1582 + (1 as i32));
                    unsafe {
                        (*(*__slate_slot_1581)).iDepth = *__slate_slot_1583;
                    }
                    unsafe {
                        (*pNode).isDirty = 1 as i32;
                    }
                    writeInt16(unsafe { (*pNode).zData }, unsafe { (*pRtree).iDepth });
                } else {
                    *__slate_slot_711 = pNode;
                    *__slate_slot_712 = nodeNew(pRtree, unsafe { (*(*__slate_slot_711)).pParent });
                    std::ptr::write(__slate_slot_1584, *__slate_slot_711);
                    std::ptr::write(__slate_slot_1585, unsafe { (*(*__slate_slot_1584)).nRef });
                    std::ptr::write(__slate_slot_1586, *__slate_slot_1585 + (1 as i32));
                    unsafe {
                        (*(*__slate_slot_1584)).nRef = *__slate_slot_1586;
                    }
                }
                if !(*__slate_slot_711 != std::ptr::null_mut::<RtreeNode>())
                    || !(*__slate_slot_712 != std::ptr::null_mut::<RtreeNode>())
                {
                    *__slate_slot_707 = 7 as i32;
                } else {
                    unsafe {
                        memset(
                            (unsafe { (*(*__slate_slot_711)).zData }) as *mut (),
                            0 as i32,
                            ((unsafe { (*pRtree).iNodeSize }) as i64) as u64,
                        )
                    };
                    unsafe {
                        memset(
                            (unsafe { (*(*__slate_slot_712)).zData }) as *mut (),
                            0 as i32,
                            ((unsafe { (*pRtree).iNodeSize }) as i64) as u64,
                        )
                    };
                    *__slate_slot_707 = splitNodeStartree(
                        pRtree,
                        *__slate_slot_709,
                        *__slate_slot_708,
                        *__slate_slot_711,
                        *__slate_slot_712,
                        std::ptr::addr_of_mut!(*__slate_slot_713),
                        std::ptr::addr_of_mut!(*__slate_slot_714),
                    );
                    if *__slate_slot_707 != (0 as i32) {
                    } else {
                        std::ptr::write(__slate_slot_1587, nodeWrite(pRtree, *__slate_slot_712));
                        *__slate_slot_707 = *__slate_slot_1587;
                        if (0 as i32) != *__slate_slot_1587 {
                            *__slate_slot_1588 = true as bool;
                        } else {
                            if ((0 as i32) as i64) == unsafe { (*(*__slate_slot_711)).iNode } {
                                std::ptr::write(
                                    __slate_slot_1590,
                                    nodeWrite(pRtree, *__slate_slot_711),
                                );
                                *__slate_slot_707 = *__slate_slot_1590;
                                *__slate_slot_1589 = (0 as i32) != *__slate_slot_1590;
                            } else {
                                *__slate_slot_1589 = false as bool;
                            }
                            *__slate_slot_1588 = *__slate_slot_1589;
                        }
                        if *__slate_slot_1588 {
                        } else {
                            (*__slate_slot_714).iRowid = unsafe { (*(*__slate_slot_712)).iNode };
                            (*__slate_slot_713).iRowid = unsafe { (*(*__slate_slot_711)).iNode };
                            if (unsafe { (*pNode).iNode }) == ((1 as i32) as i64) {
                                *__slate_slot_707 = rtreeInsertCell(
                                    pRtree,
                                    unsafe { (*(*__slate_slot_711)).pParent },
                                    std::ptr::addr_of_mut!(*__slate_slot_713),
                                    iHeight + (1 as i32),
                                );
                                if *__slate_slot_707 != (0 as i32) {
                                    break '__join_0;
                                }
                            } else {
                                std::ptr::write(__slate_slot_715, unsafe {
                                    (*(*__slate_slot_711)).pParent
                                });
                                *__slate_slot_707 = nodeParentIndex(
                                    pRtree,
                                    *__slate_slot_711,
                                    std::ptr::addr_of_mut!(*__slate_slot_716),
                                );
                                if *__slate_slot_707 == (0 as i32) {
                                    nodeOverwriteCell(
                                        pRtree,
                                        *__slate_slot_715,
                                        std::ptr::addr_of_mut!(*__slate_slot_713),
                                        *__slate_slot_716,
                                    );
                                    *__slate_slot_707 = AdjustTree(
                                        pRtree,
                                        *__slate_slot_715,
                                        std::ptr::addr_of_mut!(*__slate_slot_713),
                                    );
                                    0 as i32;
                                }
                                if *__slate_slot_707 != (0 as i32) {
                                    break '__join_0;
                                }
                            }
                            std::ptr::write(
                                __slate_slot_1591,
                                rtreeInsertCell(
                                    pRtree,
                                    unsafe { (*(*__slate_slot_712)).pParent },
                                    std::ptr::addr_of_mut!(*__slate_slot_714),
                                    iHeight + (1 as i32),
                                ),
                            );
                            *__slate_slot_707 = *__slate_slot_1591;
                            if *__slate_slot_1591 != (0 as i32) {
                            } else {
                                *__slate_slot_705 = 0 as i32;
                                loop {
                                    if *__slate_slot_705
                                        < readInt16(unsafe {
                                            unsafe { (*(*__slate_slot_712)).zData }
                                                .offset((2 as i32) as isize)
                                        })
                                    {
                                        std::ptr::write(
                                            __slate_slot_717,
                                            nodeGetRowid(
                                                pRtree,
                                                *__slate_slot_712,
                                                *__slate_slot_705,
                                            ),
                                        );
                                        *__slate_slot_707 = updateMapping(
                                            pRtree,
                                            *__slate_slot_717,
                                            *__slate_slot_712,
                                            iHeight,
                                        );
                                        if *__slate_slot_717 == unsafe { (*pCell).iRowid } {
                                            *__slate_slot_706 = 1 as i32;
                                        }
                                        if *__slate_slot_707 != (0 as i32) {
                                            break '__join_0;
                                        } else {
                                            std::ptr::write(__slate_slot_1592, *__slate_slot_705);
                                            std::ptr::write(
                                                __slate_slot_1593,
                                                *__slate_slot_1592 + (1 as i32),
                                            );
                                            *__slate_slot_705 = *__slate_slot_1593;
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                if (unsafe { (*pNode).iNode }) == ((1 as i32) as i64) {
                                    *__slate_slot_705 = 0 as i32;
                                    loop {
                                        if *__slate_slot_705
                                            < readInt16(unsafe {
                                                unsafe { (*(*__slate_slot_711)).zData }
                                                    .offset((2 as i32) as isize)
                                            })
                                        {
                                            std::ptr::write(
                                                __slate_slot_718,
                                                nodeGetRowid(
                                                    pRtree,
                                                    *__slate_slot_711,
                                                    *__slate_slot_705,
                                                ),
                                            );
                                            *__slate_slot_707 = updateMapping(
                                                pRtree,
                                                *__slate_slot_718,
                                                *__slate_slot_711,
                                                iHeight,
                                            );
                                            if *__slate_slot_707 != (0 as i32) {
                                                break '__join_0;
                                            } else {
                                                std::ptr::write(
                                                    __slate_slot_1594,
                                                    *__slate_slot_705,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_1595,
                                                    *__slate_slot_1594 + (1 as i32),
                                                );
                                                *__slate_slot_705 = *__slate_slot_1595;
                                            }
                                        } else {
                                            break '__join_0;
                                        }
                                    }
                                } else {
                                    if *__slate_slot_706 == (0 as i32) {
                                        *__slate_slot_707 = updateMapping(
                                            pRtree,
                                            unsafe { (*pCell).iRowid },
                                            *__slate_slot_711,
                                            iHeight,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        nodeRelease(pRtree, *__slate_slot_712);
        nodeRelease(pRtree, *__slate_slot_711);
        unsafe { sqlite3_free(*__slate_slot_709 as *mut ()) };
        return *__slate_slot_707;
    }
    return unsafe { std::mem::zeroed() };
}

/// If node pLeaf is not the root of the r-tree and its pParent pointer is
/// still NULL, load all ancestor nodes of pLeaf into memory and populate
/// the pLeaf->pParent chain all the way up to the root node.
///
/// This operation is required when a row is deleted (or updated - an update
/// is implemented as a delete followed by an insert). SQLite provides the
/// rowid of the row to delete, which can be used to find the leaf on which
/// the entry resides (argument pLeaf). Once the leaf is located, this
/// function is called to determine its ancestry.
fn fixLeafParent(mut pRtree: *mut Rtree, mut pLeaf: *mut RtreeNode) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pChild: *mut RtreeNode = pLeaf;
    '__slate_break_1278: while rc == (0 as i32)
        && (unsafe { (*pChild).iNode }) != ((1 as i32) as i64)
        && (unsafe { (*pChild).pParent }) == std::ptr::null_mut::<RtreeNode>()
    {
        let mut rc2: i32 = 0 as i32; // sqlite3_reset() return code
        unsafe {
            sqlite3_bind_int64(unsafe { (*pRtree).pReadParent }, 1 as i32, unsafe {
                (*pChild).iNode
            })
        };
        rc = unsafe { sqlite3_step(unsafe { (*pRtree).pReadParent }) };
        if rc == (100 as i32) {
            let mut pTest: *mut RtreeNode = unsafe { std::mem::zeroed() }; // Used to test for reference loops
            let mut iNode: i64 = 0 as i64; // Node number of parent node
            // Before setting pChild->pParent, test that we are not creating a
            // loop of references (as we would if, say, pChild==pParent). We don't
            // want to do this as it leads to a memory leak when trying to delete
            // the referenced counted node structures.
            iNode = unsafe { sqlite3_column_int64(unsafe { (*pRtree).pReadParent }, 0 as i32) };
            pTest = pLeaf;
            '__slate_break_1279: while pTest != std::ptr::null_mut::<RtreeNode>()
                && (unsafe { (*pTest).iNode }) != iNode
            {
                {}
                pTest = unsafe { (*pTest).pParent };
            }
            if pTest == std::ptr::null_mut::<RtreeNode>() {
                rc2 = nodeAcquire(pRtree, iNode, std::ptr::null_mut::<RtreeNode>(), unsafe {
                    std::ptr::addr_of_mut!((*pChild).pParent)
                });
            }
        }
        rc = unsafe { sqlite3_reset(unsafe { (*pRtree).pReadParent }) };
        if rc == (0 as i32) {
            rc = rc2;
        }
        if rc == (0 as i32)
            && !((unsafe { (*pChild).pParent }) != std::ptr::null_mut::<RtreeNode>())
        {
            {}
            rc = (11 as i32) | (1 as i32) << (8 as i32);
        }
        pChild = unsafe { (*pChild).pParent };
    }
    return rc;
}

fn removeNode(mut pRtree: *mut Rtree, mut pNode: *mut RtreeNode, mut iHeight: i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut rc2: i32 = 0 as i32;
    let mut pParent: *mut RtreeNode = std::ptr::null_mut::<RtreeNode>();
    let mut iCell: i32 = 0 as i32;
    0 as i32;
    // Remove the entry in the parent cell.
    rc = nodeParentIndex(pRtree, pNode, std::ptr::addr_of_mut!(iCell));
    if rc == (0 as i32) {
        pParent = unsafe { (*pNode).pParent };
        unsafe {
            (*pNode).pParent = std::ptr::null_mut::<RtreeNode>();
        }
        rc = deleteCell(pRtree, pParent, iCell, iHeight + (1 as i32));
        {}
    }
    rc2 = nodeRelease(pRtree, pParent);
    if rc == (0 as i32) {
        rc = rc2;
    }
    if rc != (0 as i32) {
        return rc;
    }
    // Remove the xxx_node entry.
    unsafe {
        sqlite3_bind_int64(unsafe { (*pRtree).pDeleteNode }, 1 as i32, unsafe {
            (*pNode).iNode
        })
    };
    unsafe { sqlite3_step(unsafe { (*pRtree).pDeleteNode }) };
    let __v1597: i32 = unsafe { sqlite3_reset(unsafe { (*pRtree).pDeleteNode }) };
    rc = __v1597;
    if (0 as i32) != __v1597 {
        return rc;
    }
    // Remove the xxx_parent entry.
    unsafe {
        sqlite3_bind_int64(unsafe { (*pRtree).pDeleteParent }, 1 as i32, unsafe {
            (*pNode).iNode
        })
    };
    unsafe { sqlite3_step(unsafe { (*pRtree).pDeleteParent }) };
    let __v1598: i32 = unsafe { sqlite3_reset(unsafe { (*pRtree).pDeleteParent }) };
    rc = __v1598;
    if (0 as i32) != __v1598 {
        return rc;
    }
    // Remove the node from the in-memory hash table and link it into
    // the Rtree.pDeleted list. Its contents will be re-inserted later on.
    nodeHashDelete(pRtree, pNode);
    unsafe {
        (*pNode).iNode = iHeight as i64;
    }
    unsafe {
        (*pNode).pNext = unsafe { (*pRtree).pDeleted };
    }
    let __v1599: *mut RtreeNode = pNode;
    let __v1600: i32 = unsafe { (*__v1599).nRef };
    let __v1601: i32 = __v1600 + (1 as i32);
    unsafe {
        (*__v1599).nRef = __v1601;
    }
    unsafe {
        (*pRtree).pDeleted = pNode;
    }
    return 0 as i32;
}

fn fixBoundingBox(mut pRtree: *mut Rtree, mut pNode: *mut RtreeNode) -> i32 {
    let mut pParent: *mut RtreeNode = unsafe { (*pNode).pParent };
    let mut rc: i32 = 0 as i32;
    if pParent != std::ptr::null_mut::<RtreeNode>() {
        let mut ii: i32 = 0 as i32;
        let mut nCell: i32 =
            readInt16(unsafe { unsafe { (*pNode).zData }.offset((2 as i32) as isize) });
        let mut r#box: RtreeCell = unsafe { std::mem::zeroed() }; // Bounding box for pNode
        nodeGetCell(pRtree, pNode, 0 as i32, std::ptr::addr_of_mut!(r#box));
        ii = 1 as i32;
        '__slate_break_1284: loop {
            if !(ii < nCell) {
                break;
            }
            let mut cell: RtreeCell = unsafe { std::mem::zeroed() };
            nodeGetCell(pRtree, pNode, ii, std::ptr::addr_of_mut!(cell));
            cellUnion(
                pRtree,
                std::ptr::addr_of_mut!(r#box),
                std::ptr::addr_of_mut!(cell),
            );
            let __v1602: i32 = ii;
            let __v1603: i32 = __v1602 + (1 as i32);
            ii = __v1603;
        }
        r#box.iRowid = unsafe { (*pNode).iNode };
        rc = nodeParentIndex(pRtree, pNode, std::ptr::addr_of_mut!(ii));
        if rc == (0 as i32) {
            nodeOverwriteCell(pRtree, pParent, std::ptr::addr_of_mut!(r#box), ii);
            rc = fixBoundingBox(pRtree, pParent);
        }
    }
    return rc;
}

/// Delete the cell at index iCell of node pNode. After removing the
/// cell, adjust the r-tree data structure if required.
fn deleteCell(
    mut pRtree: *mut Rtree,
    mut pNode: *mut RtreeNode,
    mut iCell: i32,
    mut iHeight: i32,
) -> i32 {
    let mut pParent: *mut RtreeNode = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    let __v1596: i32 = fixLeafParent(pRtree, pNode);
    rc = __v1596;
    if (0 as i32) != __v1596 {
        return rc;
    }
    // Remove the cell from the node. This call just moves bytes around
    // the in-memory node image, so it cannot fail.
    nodeDeleteCell(pRtree, pNode, iCell);
    // If the node is not the tree root and now has less than the minimum
    // number of cells, remove it from the tree. Otherwise, update the
    // cell in the parent node so that it tightly contains the updated
    // node.
    pParent = unsafe { (*pNode).pParent };
    0 as i32;
    if pParent != std::ptr::null_mut::<RtreeNode>() {
        if readInt16(unsafe { unsafe { (*pNode).zData }.offset((2 as i32) as isize) })
            < ((unsafe { (*pRtree).iNodeSize }) - (4 as i32))
                / (((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32)
                / (3 as i32)
        {
            rc = removeNode(pRtree, pNode, iHeight);
        } else {
            rc = fixBoundingBox(pRtree, pNode);
        }
    }
    return rc;
}

/// Insert cell pCell into node pNode. Node pNode is the head of a
/// subtree iHeight high (leaf nodes have iHeight==0).
fn rtreeInsertCell(
    mut pRtree: *mut Rtree,
    mut pNode: *mut RtreeNode,
    mut pCell: *mut RtreeCell,
    mut iHeight: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    if iHeight > (0 as i32) {
        let mut pChild: *mut RtreeNode = nodeHashLookup(pRtree, unsafe { (*pCell).iRowid });
        if pChild != std::ptr::null_mut::<RtreeNode>() {
            nodeRelease(pRtree, unsafe { (*pChild).pParent });
            nodeReference(pNode);
            unsafe {
                (*pChild).pParent = pNode;
            }
        }
    }
    if nodeInsertCell(pRtree, pNode, pCell) != (0 as i32) {
        rc = SplitNode(pRtree, pNode, pCell, iHeight);
    } else {
        rc = AdjustTree(pRtree, pNode, pCell);
        if rc == (0 as i32) {
            if iHeight == (0 as i32) {
                rc = rowidWrite(pRtree, unsafe { (*pCell).iRowid }, unsafe {
                    (*pNode).iNode
                });
            } else {
                rc = parentWrite(pRtree, unsafe { (*pCell).iRowid }, unsafe {
                    (*pNode).iNode
                });
            }
        }
    }
    return rc;
}

fn reinsertNodeContent(mut pRtree: *mut Rtree, mut pNode: *mut RtreeNode) -> i32 {
    let mut ii: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut nCell: i32 =
        readInt16(unsafe { unsafe { (*pNode).zData }.offset((2 as i32) as isize) });
    ii = 0 as i32;
    '__slate_break_1285: loop {
        if !(rc == (0 as i32) && ii < nCell) {
            break;
        }
        let mut pInsert: *mut RtreeNode = unsafe { std::mem::zeroed() };
        let mut cell: RtreeCell = unsafe { std::mem::zeroed() };
        nodeGetCell(pRtree, pNode, ii, std::ptr::addr_of_mut!(cell));
        // Find a node to store this cell in. pNode->iNode currently contains
        // the height of the sub-tree headed by the cell.
        rc = ChooseLeaf(
            pRtree,
            std::ptr::addr_of_mut!(cell),
            (unsafe { (*pNode).iNode }) as i32,
            std::ptr::addr_of_mut!(pInsert),
        );
        if rc == (0 as i32) {
            let mut rc2: i32 = 0 as i32;
            rc = rtreeInsertCell(
                pRtree,
                pInsert,
                std::ptr::addr_of_mut!(cell),
                (unsafe { (*pNode).iNode }) as i32,
            );
            rc2 = nodeRelease(pRtree, pInsert);
            if rc == (0 as i32) {
                rc = rc2;
            }
        }
        let __v1604: i32 = ii;
        let __v1605: i32 = __v1604 + (1 as i32);
        ii = __v1605;
    }
    return rc;
}

/// Select a currently unused rowid for a new r-tree record.
fn rtreeNewRowid(mut pRtree: *mut Rtree, mut piRowid: *mut i64) -> i32 {
    let mut rc: i32 = 0 as i32;
    unsafe { sqlite3_bind_null(unsafe { (*pRtree).pWriteRowid }, 1 as i32) };
    unsafe { sqlite3_bind_null(unsafe { (*pRtree).pWriteRowid }, 2 as i32) };
    unsafe { sqlite3_step(unsafe { (*pRtree).pWriteRowid }) };
    rc = unsafe { sqlite3_reset(unsafe { (*pRtree).pWriteRowid }) };
    unsafe {
        *piRowid = unsafe { sqlite3_last_insert_rowid(unsafe { (*pRtree).db }) };
    }
    return rc;
}

/// Remove the entry with rowid=iDelete from the r-tree structure.
fn rtreeDeleteRowid(mut pRtree: *mut Rtree, mut iDelete: i64) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut pLeaf: *mut RtreeNode = std::ptr::null_mut::<RtreeNode>(); // Leaf node containing record iDelete
    let mut iCell: i32 = 0 as i32; // Index of iDelete cell in pLeaf
    let mut pRoot: *mut RtreeNode = std::ptr::null_mut::<RtreeNode>(); // Root node of rtree structure
    // Obtain a reference to the root node to initialize Rtree.iDepth
    rc = nodeAcquire(
        pRtree,
        (1 as i32) as i64,
        std::ptr::null_mut::<RtreeNode>(),
        std::ptr::addr_of_mut!(pRoot),
    );
    // Obtain a reference to the leaf node that contains the entry
    // about to be deleted.
    if rc == (0 as i32) {
        rc = findLeafNode(
            pRtree,
            iDelete,
            std::ptr::addr_of_mut!(pLeaf),
            std::ptr::null_mut::<i64>(),
        );
    }
    // Delete the cell in question from the leaf node.
    if rc == (0 as i32) && pLeaf != std::ptr::null_mut::<RtreeNode>() {
        let mut rc2: i32 = 0 as i32;
        rc = nodeRowidIndex(pRtree, pLeaf, iDelete, std::ptr::addr_of_mut!(iCell));
        if rc == (0 as i32) {
            rc = deleteCell(pRtree, pLeaf, iCell, 0 as i32);
        }
        rc2 = nodeRelease(pRtree, pLeaf);
        if rc == (0 as i32) {
            rc = rc2;
        }
    }
    // Delete the corresponding entry in the <rtree>_rowid table.
    if rc == (0 as i32) {
        unsafe { sqlite3_bind_int64(unsafe { (*pRtree).pDeleteRowid }, 1 as i32, iDelete) };
        unsafe { sqlite3_step(unsafe { (*pRtree).pDeleteRowid }) };
        rc = unsafe { sqlite3_reset(unsafe { (*pRtree).pDeleteRowid }) };
    }
    // Check if the root node now has exactly one child. If so, remove
    // it, schedule the contents of the child for reinsertion and
    // reduce the tree height by one.
    //
    // This is equivalent to copying the contents of the child into
    // the root node (the operation that Gutman's paper says to perform
    // in this scenario).
    let __v1606: bool;
    if rc == (0 as i32) && (unsafe { (*pRtree).iDepth }) > (0 as i32) {
        __v1606 = readInt16(unsafe { unsafe { (*pRoot).zData }.offset((2 as i32) as isize) })
            == (1 as i32);
    } else {
        __v1606 = false as bool;
    }
    if __v1606 {
        let mut rc2: i32 = 0 as i32;
        let mut pChild: *mut RtreeNode = std::ptr::null_mut::<RtreeNode>();
        let mut iChild: i64 = nodeGetRowid(pRtree, pRoot, 0 as i32);
        rc = nodeAcquire(pRtree, iChild, pRoot, std::ptr::addr_of_mut!(pChild)); // tag-20210916a
        if rc == (0 as i32) {
            rc = removeNode(pRtree, pChild, (unsafe { (*pRtree).iDepth }) - (1 as i32));
        }
        rc2 = nodeRelease(pRtree, pChild);
        if rc == (0 as i32) {
            rc = rc2;
        }
        if rc == (0 as i32) {
            let __v1607: *mut Rtree = pRtree;
            let __v1608: i32 = unsafe { (*__v1607).iDepth };
            let __v1609: i32 = __v1608 - (1 as i32);
            unsafe {
                (*__v1607).iDepth = __v1609;
            }
            writeInt16(unsafe { (*pRoot).zData }, unsafe { (*pRtree).iDepth });
            unsafe {
                (*pRoot).isDirty = 1 as i32;
            }
        }
    }
    // Re-insert the contents of any underfull nodes removed from the tree.
    pLeaf = unsafe { (*pRtree).pDeleted };
    '__slate_break_1286: while pLeaf != std::ptr::null_mut::<RtreeNode>() {
        if rc == (0 as i32) {
            rc = reinsertNodeContent(pRtree, pLeaf);
        }
        unsafe {
            (*pRtree).pDeleted = unsafe { (*pLeaf).pNext };
        }
        let __v1610: *mut Rtree = pRtree;
        let __v1611: u32 = unsafe { (*__v1610).nNodeRef };
        let __v1612: u32 = __v1611.wrapping_sub((1 as i32) as u32);
        unsafe {
            (*__v1610).nNodeRef = __v1612;
        }
        unsafe { sqlite3_free(pLeaf as *mut ()) };
        pLeaf = unsafe { (*pRtree).pDeleted };
    }
    // Release the reference to the root node.
    if rc == (0 as i32) {
        rc = nodeRelease(pRtree, pRoot);
    } else {
        nodeRelease(pRtree, pRoot);
    }
    return rc;
}

// Rounding constants for float->double conversion.
// Round towards zero
// Round away from zero
/// Convert an sqlite3_value into an RtreeValue (presumably a float)
/// while taking care to round toward negative or positive, respectively.
fn rtreeValueDown(mut v: *mut sqlite3_value) -> f32 {
    let mut d: f64 = unsafe { sqlite3_value_double(v) };
    let mut f: f32 = d as f32;
    if (f as f64) > d {
        f = (d * if d < ((0 as i32) as f64) {
            1.0f64 + 1.0f64 / 8388608.0f64
        } else {
            1.0f64 - 1.0f64 / 8388608.0f64
        }) as f32;
    }
    return f;
}

fn rtreeValueUp(mut v: *mut sqlite3_value) -> f32 {
    let mut d: f64 = unsafe { sqlite3_value_double(v) };
    let mut f: f32 = d as f32;
    if (f as f64) < d {
        f = (d * if d < ((0 as i32) as f64) {
            1.0f64 - 1.0f64 / 8388608.0f64
        } else {
            1.0f64 + 1.0f64 / 8388608.0f64
        }) as f32;
    }
    return f;
}

/// A constraint has failed while inserting a row into an rtree table.
/// Assuming no OOM error occurs, this function sets the error message
/// (at pRtree->base.zErrMsg) to an appropriate value and returns
/// SQLITE_CONSTRAINT.
///
/// Parameter iCol is the index of the leftmost column involved in the
/// constraint failure. If it is 0, then the constraint that failed is
/// the unique constraint on the id column. Otherwise, it is the rtree
/// (c1<=c2) constraint on columns iCol and iCol+1 that has failed.
///
/// If an OOM occurs, SQLITE_NOMEM is returned instead of SQLITE_CONSTRAINT.
fn rtreeConstraintError(mut pRtree: *mut Rtree, mut iCol: i32) -> i32 {
    let mut pStmt: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
    let mut zSql: *mut i8 = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    0 as i32;
    zSql = unsafe {
        sqlite3_mprintf(
            (b"SELECT * FROM %Q.%Q\0".as_ptr() as *mut i8) as *const i8,
            unsafe { (*pRtree).zDb },
            unsafe { (*pRtree).zName },
        )
    };
    if zSql != std::ptr::null_mut::<i8>() {
        rc = unsafe {
            sqlite3_prepare_v2(
                unsafe { (*pRtree).db },
                zSql as *const i8,
                -(1 as i32),
                std::ptr::addr_of_mut!(pStmt),
                std::ptr::null_mut::<*const i8>(),
            )
        };
    } else {
        rc = 7 as i32;
    }
    unsafe { sqlite3_free(zSql as *mut ()) };
    if rc == (0 as i32) {
        if iCol == (0 as i32) {
            let mut zCol: *const i8 = unsafe { sqlite3_column_name(pStmt, 0 as i32) };
            unsafe {
                (*pRtree).base.zErrMsg = unsafe {
                    sqlite3_mprintf(
                        (b"UNIQUE constraint failed: %s.%s\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { (*pRtree).zName },
                        zCol,
                    )
                };
            }
        } else {
            let mut zCol1: *const i8 = unsafe { sqlite3_column_name(pStmt, iCol) };
            let mut zCol2: *const i8 = unsafe { sqlite3_column_name(pStmt, iCol + (1 as i32)) };
            unsafe {
                (*pRtree).base.zErrMsg = unsafe {
                    sqlite3_mprintf(
                        (b"rtree constraint failed: %s.(%s<=%s)\0".as_ptr() as *mut i8)
                            as *const i8,
                        unsafe { (*pRtree).zName },
                        zCol1,
                        zCol2,
                    )
                };
            }
        }
    }
    unsafe { sqlite3_finalize(pStmt) };
    return if rc == (0 as i32) { 19 as i32 } else { rc };
}

/// The xUpdate method for rtree module virtual tables.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeUpdate")]
extern "C-unwind" fn rtreeUpdate(
    mut pVtab: *mut sqlite3_vtab,
    mut nData: i32,
    mut aData: *mut *mut sqlite3_value,
    mut pRowid: *mut i64,
) -> i32 {
    let mut __slate_storage_1619: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1619: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1619) as *mut i32;
    let mut __slate_storage_1618: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1618: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1618) as *mut i32;
    let mut __slate_storage_814: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_814: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_814) as *mut i32;
    let mut __slate_storage_813: std::mem::MaybeUninit<*mut sqlite3_stmt> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_813: *mut *mut sqlite3_stmt =
        std::ptr::addr_of_mut!(__slate_storage_813) as *mut *mut sqlite3_stmt;
    let mut __slate_storage_812: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_812: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_812) as *mut i32;
    // Insert the new record into the r-tree
    let mut __slate_storage_811: std::mem::MaybeUninit<*mut RtreeNode> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_811: *mut *mut RtreeNode =
        std::ptr::addr_of_mut!(__slate_storage_811) as *mut *mut RtreeNode;
    let mut __slate_storage_810: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_810: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_810) as *mut i32;
    let mut __slate_storage_1617: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1617: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1617) as *mut bool;
    let mut __slate_storage_1614: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1614: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1614) as *mut i32;
    let mut __slate_storage_1613: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1613: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1613) as *mut i32;
    let mut __slate_storage_1616: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1616: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1616) as *mut i32;
    let mut __slate_storage_1615: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1615: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1615) as *mut i32;
    let mut __slate_storage_809: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_809: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_809) as *mut i32;
    let mut __slate_storage_808: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_808: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_808) as *mut i32; // Set to 1 after new rowid is determined
    let mut __slate_storage_807: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_807: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_807) as *mut i32; // New cell to insert if nData>1
    let mut __slate_storage_806: std::mem::MaybeUninit<RtreeCell> = std::mem::MaybeUninit::uninit();
    let __slate_slot_806: *mut RtreeCell =
        std::ptr::addr_of_mut!(__slate_storage_806) as *mut RtreeCell;
    let mut __slate_storage_805: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_805: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_805) as *mut i32;
    let mut __slate_storage_804: std::mem::MaybeUninit<*mut Rtree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_804: *mut *mut Rtree =
        std::ptr::addr_of_mut!(__slate_storage_804) as *mut *mut Rtree;
    unsafe {
        std::ptr::write(__slate_slot_804, pVtab as *mut Rtree);
        std::ptr::write(__slate_slot_805, 0 as i32);
        std::ptr::write(__slate_slot_807, 0 as i32);
        if (unsafe { (*(*__slate_slot_804)).nNodeRef }) != (0 as u32) {
            // Unable to write to the btree while another cursor is reading from it,
            // since the write might do a rebalance which would disrupt the read
            // cursor.
            return (6 as i32) | (2 as i32) << (8 as i32);
        } else {
            '__join_0: {
                rtreeReference(*__slate_slot_804);
                0 as i32;
                unsafe {
                    memset(
                        std::ptr::addr_of_mut!(*__slate_slot_806) as *mut (),
                        0 as i32,
                        48 as u64,
                    )
                };
                // Constraint handling. A write operation on an r-tree table may return
                // SQLITE_CONSTRAINT for two reasons:
                //
                //   1. A duplicate rowid value, or
                //   2. The supplied data violates the "x2>=x1" constraint.
                //
                // In the first case, if the conflict-handling mode is REPLACE, then
                // the conflicting row can be removed before proceeding. In the second
                // case, SQLITE_CONSTRAINT must be returned regardless of the
                // conflict-handling mode specified by the user.
                if nData > (1 as i32) {
                    std::ptr::write(__slate_slot_809, nData - (4 as i32));
                    if *__slate_slot_809
                        > (((unsafe { (*(*__slate_slot_804)).nDim2 }) as u32) as i32)
                    {
                        *__slate_slot_809 =
                            ((unsafe { (*(*__slate_slot_804)).nDim2 }) as u32) as i32;
                    }
                    '__join_25: {
                        // Populate the cell.aCoord[] array. The first coordinate is aData[3].
                        //
                        // NB: nData can only be less than nDim*2+3 if the rtree is mis-declared
                        // with "column" that are interpreted as table constraints.
                        // Example:  CREATE VIRTUAL TABLE bad USING rtree(x,y,CHECK(y>5));
                        // This problem was discovered after years of use, so we silently ignore
                        // these kinds of misdeclared tables to avoid breaking any legacy.
                        if (((unsafe { (*(*__slate_slot_804)).eCoordType }) as u32) as i32)
                            == (0 as i32)
                        {
                            *__slate_slot_808 = 0 as i32;
                            loop {
                                if *__slate_slot_808 < *__slate_slot_809 {
                                    unsafe {
                                        (*unsafe {
                                            ((*__slate_slot_806).aCoord.as_mut_ptr()
                                                as *mut RtreeCoord)
                                                .offset(*__slate_slot_808 as isize)
                                        })
                                        .f = rtreeValueDown(unsafe {
                                            *unsafe {
                                                aData.offset(
                                                    (*__slate_slot_808 + (3 as i32)) as isize,
                                                )
                                            }
                                        });
                                    }
                                    unsafe {
                                        (*unsafe {
                                            ((*__slate_slot_806).aCoord.as_mut_ptr()
                                                as *mut RtreeCoord)
                                                .offset((*__slate_slot_808 + (1 as i32)) as isize)
                                        })
                                        .f = rtreeValueUp(unsafe {
                                            *unsafe {
                                                aData.offset(
                                                    (*__slate_slot_808 + (4 as i32)) as isize,
                                                )
                                            }
                                        });
                                    }
                                    if (unsafe {
                                        (*unsafe {
                                            ((*__slate_slot_806).aCoord.as_mut_ptr()
                                                as *mut RtreeCoord)
                                                .offset(*__slate_slot_808 as isize)
                                        })
                                        .f
                                    }) > unsafe {
                                        (*unsafe {
                                            ((*__slate_slot_806).aCoord.as_mut_ptr()
                                                as *mut RtreeCoord)
                                                .offset((*__slate_slot_808 + (1 as i32)) as isize)
                                        })
                                        .f
                                    } {
                                        break;
                                    } else {
                                        std::ptr::write(__slate_slot_1613, *__slate_slot_808);
                                        std::ptr::write(
                                            __slate_slot_1614,
                                            *__slate_slot_1613 + (2 as i32),
                                        );
                                        *__slate_slot_808 = *__slate_slot_1614;
                                    }
                                } else {
                                    break '__join_25;
                                }
                            }
                            *__slate_slot_805 = rtreeConstraintError(
                                *__slate_slot_804,
                                *__slate_slot_808 + (1 as i32),
                            );
                            break '__join_0;
                        } else {
                            *__slate_slot_808 = 0 as i32;
                            loop {
                                if *__slate_slot_808 < *__slate_slot_809 {
                                    unsafe {
                                        (*unsafe {
                                            ((*__slate_slot_806).aCoord.as_mut_ptr()
                                                as *mut RtreeCoord)
                                                .offset(*__slate_slot_808 as isize)
                                        })
                                        .i = unsafe {
                                            sqlite3_value_int(unsafe {
                                                *unsafe {
                                                    aData.offset(
                                                        (*__slate_slot_808 + (3 as i32)) as isize,
                                                    )
                                                }
                                            })
                                        };
                                    }
                                    unsafe {
                                        (*unsafe {
                                            ((*__slate_slot_806).aCoord.as_mut_ptr()
                                                as *mut RtreeCoord)
                                                .offset((*__slate_slot_808 + (1 as i32)) as isize)
                                        })
                                        .i = unsafe {
                                            sqlite3_value_int(unsafe {
                                                *unsafe {
                                                    aData.offset(
                                                        (*__slate_slot_808 + (4 as i32)) as isize,
                                                    )
                                                }
                                            })
                                        };
                                    }
                                    if (unsafe {
                                        (*unsafe {
                                            ((*__slate_slot_806).aCoord.as_mut_ptr()
                                                as *mut RtreeCoord)
                                                .offset(*__slate_slot_808 as isize)
                                        })
                                        .i
                                    }) > unsafe {
                                        (*unsafe {
                                            ((*__slate_slot_806).aCoord.as_mut_ptr()
                                                as *mut RtreeCoord)
                                                .offset((*__slate_slot_808 + (1 as i32)) as isize)
                                        })
                                        .i
                                    } {
                                        break;
                                    } else {
                                        std::ptr::write(__slate_slot_1615, *__slate_slot_808);
                                        std::ptr::write(
                                            __slate_slot_1616,
                                            *__slate_slot_1615 + (2 as i32),
                                        );
                                        *__slate_slot_808 = *__slate_slot_1616;
                                    }
                                } else {
                                    break '__join_25;
                                }
                            }
                            *__slate_slot_805 = rtreeConstraintError(
                                *__slate_slot_804,
                                *__slate_slot_808 + (1 as i32),
                            );
                            break '__join_0;
                        }
                    }
                    // If a rowid value was supplied, check if it is already present in
                    // the table. If so, the constraint has failed.
                    if (unsafe {
                        sqlite3_value_type(unsafe { *unsafe { aData.offset((2 as i32) as isize) } })
                    }) != (5 as i32)
                    {
                        (*__slate_slot_806).iRowid = unsafe {
                            sqlite3_value_int64(unsafe {
                                *unsafe { aData.offset((2 as i32) as isize) }
                            })
                        };
                        if (unsafe {
                            sqlite3_value_type(unsafe {
                                *unsafe { aData.offset((0 as i32) as isize) }
                            })
                        }) == (5 as i32)
                        {
                            *__slate_slot_1617 = true as bool;
                        } else {
                            *__slate_slot_1617 = (unsafe {
                                sqlite3_value_int64(unsafe {
                                    *unsafe { aData.offset((0 as i32) as isize) }
                                })
                            }) != (*__slate_slot_806).iRowid;
                        }
                        if *__slate_slot_1617 {
                            unsafe {
                                sqlite3_bind_int64(
                                    unsafe { (*(*__slate_slot_804)).pReadRowid },
                                    1 as i32,
                                    (*__slate_slot_806).iRowid,
                                )
                            };
                            *__slate_slot_810 = unsafe {
                                sqlite3_step(unsafe { (*(*__slate_slot_804)).pReadRowid })
                            };
                            *__slate_slot_805 = unsafe {
                                sqlite3_reset(unsafe { (*(*__slate_slot_804)).pReadRowid })
                            };
                            if (100 as i32) == *__slate_slot_810 {
                                if (unsafe {
                                    sqlite3_vtab_on_conflict(unsafe { (*(*__slate_slot_804)).db })
                                }) == (5 as i32)
                                {
                                    *__slate_slot_805 = rtreeDeleteRowid(
                                        *__slate_slot_804,
                                        (*__slate_slot_806).iRowid,
                                    );
                                } else {
                                    *__slate_slot_805 =
                                        rtreeConstraintError(*__slate_slot_804, 0 as i32);
                                    break '__join_0;
                                }
                            }
                        }
                        *__slate_slot_807 = 1 as i32;
                    }
                }
                // If aData[0] is not an SQL NULL value, it is the rowid of a
                // record to delete from the r-tree table. The following block does
                // just that.
                if (unsafe {
                    sqlite3_value_type(unsafe { *unsafe { aData.offset((0 as i32) as isize) } })
                }) != (5 as i32)
                {
                    *__slate_slot_805 = rtreeDeleteRowid(*__slate_slot_804, unsafe {
                        sqlite3_value_int64(unsafe {
                            *unsafe { aData.offset((0 as i32) as isize) }
                        })
                    });
                }
                // If the aData[] array contains more than one element, elements
                // (aData[2]..aData[argc-1]) contain a new record to insert into
                // the r-tree structure.
                if *__slate_slot_805 == (0 as i32) && nData > (1 as i32) {
                    std::ptr::write(__slate_slot_811, std::ptr::null_mut::<RtreeNode>());
                    // Figure out the rowid of the new row.
                    if *__slate_slot_807 == (0 as i32) {
                        *__slate_slot_805 = rtreeNewRowid(
                            *__slate_slot_804,
                            std::ptr::addr_of_mut!((*__slate_slot_806).iRowid),
                        );
                    }
                    unsafe {
                        *pRowid = (*__slate_slot_806).iRowid;
                    }
                    if *__slate_slot_805 == (0 as i32) {
                        *__slate_slot_805 = ChooseLeaf(
                            *__slate_slot_804,
                            std::ptr::addr_of_mut!(*__slate_slot_806),
                            0 as i32,
                            std::ptr::addr_of_mut!(*__slate_slot_811),
                        );
                    }
                    if *__slate_slot_805 == (0 as i32) {
                        *__slate_slot_805 = rtreeInsertCell(
                            *__slate_slot_804,
                            *__slate_slot_811,
                            std::ptr::addr_of_mut!(*__slate_slot_806),
                            0 as i32,
                        );
                        *__slate_slot_812 = nodeRelease(*__slate_slot_804, *__slate_slot_811);
                        if *__slate_slot_805 == (0 as i32) {
                            *__slate_slot_805 = *__slate_slot_812;
                        }
                    }
                    if *__slate_slot_805 == (0 as i32)
                        && (unsafe { (*(*__slate_slot_804)).nAux }) != (0 as u16)
                    {
                        std::ptr::write(__slate_slot_813, unsafe {
                            (*(*__slate_slot_804)).pWriteAux
                        });
                        unsafe {
                            sqlite3_bind_int64(*__slate_slot_813, 1 as i32, unsafe { *pRowid })
                        };
                        *__slate_slot_814 = 0 as i32;
                        loop {
                            if *__slate_slot_814
                                < (((unsafe { (*(*__slate_slot_804)).nAux }) as u32) as i32)
                            {
                                unsafe {
                                    sqlite3_bind_value(
                                        *__slate_slot_813,
                                        *__slate_slot_814 + (2 as i32),
                                        (unsafe {
                                            *unsafe {
                                                aData.offset(
                                                    ((((unsafe { (*(*__slate_slot_804)).nDim2 })
                                                        as u32)
                                                        as i32)
                                                        + (3 as i32)
                                                        + *__slate_slot_814)
                                                        as isize,
                                                )
                                            }
                                        })
                                            as *const sqlite3_value,
                                    )
                                };
                                std::ptr::write(__slate_slot_1618, *__slate_slot_814);
                                std::ptr::write(__slate_slot_1619, *__slate_slot_1618 + (1 as i32));
                                *__slate_slot_814 = *__slate_slot_1619;
                            } else {
                                break;
                            }
                        }
                        unsafe { sqlite3_step(*__slate_slot_813) };
                        *__slate_slot_805 = unsafe { sqlite3_reset(*__slate_slot_813) };
                    }
                }
            }
            rtreeRelease(*__slate_slot_804);
            return *__slate_slot_805;
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Called when a transaction starts.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeBeginTransaction")]
extern "C-unwind" fn rtreeBeginTransaction(mut pVtab: *mut sqlite3_vtab) -> i32 {
    let mut pRtree: *mut Rtree = pVtab as *mut Rtree;
    0 as i32;
    unsafe {
        (*pRtree).inWrTrans = ((1 as i32) as i8) as u8;
    }
    return 0 as i32;
}

/// Called when a transaction completes (either by COMMIT or ROLLBACK).
/// The sqlite3_blob object should be released at this point.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeEndTransaction")]
extern "C-unwind" fn rtreeEndTransaction(mut pVtab: *mut sqlite3_vtab) -> i32 {
    let mut pRtree: *mut Rtree = pVtab as *mut Rtree;
    unsafe {
        (*pRtree).inWrTrans = ((0 as i32) as i8) as u8;
    }
    nodeBlobReset(pRtree);
    return 0 as i32;
}

#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeRollback")]
extern "C-unwind" fn rtreeRollback(mut pVtab: *mut sqlite3_vtab) -> i32 {
    return rtreeEndTransaction(pVtab);
}

/// The xRename method for rtree module virtual tables.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeRename")]
extern "C-unwind" fn rtreeRename(mut pVtab: *mut sqlite3_vtab, mut zNewName: *const i8) -> i32 {
    let mut pRtree: *mut Rtree = pVtab as *mut Rtree;
    let mut rc: i32 = 7 as i32;
    let mut zSql: *mut i8 = unsafe {
        sqlite3_mprintf((b"ALTER TABLE %Q.'%q_node'   RENAME TO \"%w_node\";ALTER TABLE %Q.'%q_parent' RENAME TO \"%w_parent\";ALTER TABLE %Q.'%q_rowid'  RENAME TO \"%w_rowid\";\0".as_ptr() as *mut i8) as *const i8, unsafe { (*pRtree).zDb }, unsafe { (*pRtree).zName }, zNewName, unsafe { (*pRtree).zDb }, unsafe { (*pRtree).zName }, zNewName, unsafe { (*pRtree).zDb }, unsafe { (*pRtree).zName }, zNewName)
    };
    if zSql != std::ptr::null_mut::<i8>() {
        nodeBlobReset(pRtree);
        rc = unsafe {
            sqlite3_exec(
                unsafe { (*pRtree).db },
                zSql as *const i8,
                None,
                std::ptr::null_mut::<()>(),
                std::ptr::null_mut::<*mut i8>(),
            )
        };
        unsafe { sqlite3_free(zSql as *mut ()) };
    }
    return rc;
}

/// The xSavepoint method.
///
/// This module does not need to do anything to support savepoints. However,
/// it uses this hook to close any open blob handle. This is done because a
/// DROP TABLE command - which fortunately always opens a savepoint - cannot
/// succeed if there are any open blob handles. i.e. if the blob handle were
/// not closed here, the following would fail:
///
///   BEGIN;
///     INSERT INTO rtree...
///     DROP TABLE <tablename>;    -- Would fail with SQLITE_LOCKED
///   COMMIT;
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeSavepoint")]
extern "C-unwind" fn rtreeSavepoint(mut pVtab: *mut sqlite3_vtab, mut iSavepoint: i32) -> i32 {
    let mut pRtree: *mut Rtree = pVtab as *mut Rtree;
    let mut iwt: u8 = unsafe { (*pRtree).inWrTrans };
    iSavepoint;
    unsafe {
        (*pRtree).inWrTrans = ((0 as i32) as i8) as u8;
    }
    nodeBlobReset(pRtree);
    unsafe {
        (*pRtree).inWrTrans = iwt;
    }
    return 0 as i32;
}

/// This function populates the pRtree->nRowEst variable with an estimate
/// of the number of rows in the virtual table. If possible, this is based
/// on sqlite_stat1 data. Otherwise, use RTREE_DEFAULT_ROWEST.
fn rtreeQueryStat1(mut db: *mut sqlite3, mut pRtree: *mut Rtree) -> i32 {
    let mut zFmt: *const i8 = (b"SELECT stat FROM %Q.sqlite_stat1 WHERE tbl = '%q_rowid'\0".as_ptr()
        as *mut i8) as *const i8;
    let mut zSql: *mut i8 = unsafe { std::mem::zeroed() };
    let mut p: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    let mut nRow: i64 = (100 as i32) as i64;
    rc = unsafe {
        sqlite3_table_column_metadata(
            db,
            (unsafe { (*pRtree).zDb }) as *const i8,
            (b"sqlite_stat1\0".as_ptr() as *mut i8) as *const i8,
            std::ptr::null::<i8>(),
            std::ptr::null_mut::<*const i8>(),
            std::ptr::null_mut::<*const i8>(),
            std::ptr::null_mut::<i32>(),
            std::ptr::null_mut::<i32>(),
            std::ptr::null_mut::<i32>(),
        )
    };
    if rc != (0 as i32) {
        unsafe {
            (*pRtree).nRowEst = (1048576 as i32) as i64;
        }
        return if rc == (1 as i32) { 0 as i32 } else { rc };
    }
    zSql = unsafe { sqlite3_mprintf(zFmt, unsafe { (*pRtree).zDb }, unsafe { (*pRtree).zName }) };
    if zSql == std::ptr::null_mut::<i8>() {
        rc = 7 as i32;
    } else {
        rc = unsafe {
            sqlite3_prepare_v2(
                db,
                zSql as *const i8,
                -(1 as i32),
                std::ptr::addr_of_mut!(p),
                std::ptr::null_mut::<*const i8>(),
            )
        };
        if rc == (0 as i32) {
            if (unsafe { sqlite3_step(p) }) == (100 as i32) {
                nRow = unsafe { sqlite3_column_int64(p, 0 as i32) };
            }
            rc = unsafe { sqlite3_finalize(p) };
        }
        unsafe { sqlite3_free(zSql as *mut ()) };
    }
    unsafe {
        (*pRtree).nRowEst = if nRow < ((100 as i32) as i64) {
            (100 as i32) as i64
        } else {
            nRow
        };
    }
    return rc;
}

/// Return true if zName is the extension on one of the shadow tables used
/// by this module.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeShadowName")]
extern "C-unwind" fn rtreeShadowName(mut zName: *const i8) -> i32 {
    let mut i: u32 = 0 as u32;
    i = (0 as i32) as u32;
    '__slate_break_1299: while (i as u64) < (24 as u64) / (8 as u64) {
        if (unsafe {
            sqlite3_stricmp(zName, unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!(azName.0) as *mut *const i8 }.offset(i as isize)
                }
            })
        }) == (0 as i32)
        {
            return 1 as i32;
        }
        let __v1620: u32 = i;
        let __v1621: u32 = __v1620.wrapping_add((1 as i32) as u32);
        i = __v1621;
    }
    return 0 as i32;
}

static mut azName: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"node\0".as_ptr() as *mut i8) as *const i8,
    (b"parent\0".as_ptr() as *mut i8) as *const i8,
    (b"rowid\0".as_ptr() as *mut i8) as *const i8,
]);

/// iVersion
/// xCreate - create a table
/// xConnect - connect to an existing table
/// xBestIndex - Determine search strategy
/// xDisconnect - Disconnect from a table
/// xDestroy - Drop a table
/// xOpen - open a cursor
/// xClose - close a cursor
/// xFilter - configure scan constraints
/// xNext - advance a cursor
/// xEof
/// xColumn - read data
/// xRowid - read data
/// xUpdate - write data
/// xBegin - begin transaction
/// xSync - sync transaction
/// xCommit - commit transaction
/// xRollback - rollback transaction
/// xFindFunction - function overloading
/// xRename - rename the table
/// xSavepoint
/// xRelease
/// xRollbackTo
/// xShadowName
/// xIntegrity
static mut rtreeModule: sqlite3_module = sqlite3_module {
    iVersion: 4 as i32,
    xCreate: Some(rtreeCreate),
    xConnect: Some(rtreeConnect),
    xBestIndex: Some(rtreeBestIndex),
    xDisconnect: Some(rtreeDisconnect),
    xDestroy: Some(rtreeDestroy),
    xOpen: Some(rtreeOpen),
    xClose: Some(rtreeClose),
    xFilter: Some(rtreeFilter),
    xNext: Some(rtreeNext),
    xEof: Some(rtreeEof),
    xColumn: Some(rtreeColumn),
    xRowid: Some(rtreeRowid),
    xUpdate: Some(rtreeUpdate),
    xBegin: Some(rtreeBeginTransaction),
    xSync: Some(rtreeEndTransaction),
    xCommit: Some(rtreeEndTransaction),
    xRollback: Some(rtreeRollback),
    xFindFunction: None,
    xRename: Some(rtreeRename),
    xSavepoint: Some(rtreeSavepoint),
    xRelease: None,
    xRollbackTo: None,
    xShadowName: Some(rtreeShadowName),
    xIntegrity: Some(rtreeIntegrity),
};

fn rtreeSqlInit(
    mut pRtree: *mut Rtree,
    mut db: *mut sqlite3,
    mut zDb: *const i8,
    mut zPrefix: *const i8,
    mut isCreate: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Write the xxx_node table
    // Read and write the xxx_rowid table
    // Read and write the xxx_parent table
    let mut appStmt: __SlateAlign16<[*mut *mut sqlite3_stmt; 8]> =
        __SlateAlign16([0 as *mut *mut sqlite3_stmt; 8]);
    let mut i: i32 = 0 as i32;
    let mut f: i32 = (1 as i32) | (4 as i32);
    unsafe {
        (*pRtree).db = db;
    }
    if isCreate != (0 as i32) {
        let mut zCreate: *mut i8 = unsafe { std::mem::zeroed() };
        let mut p: *mut sqlite3_str = unsafe { sqlite3_str_new(db) };
        let mut ii: i32 = 0 as i32;
        unsafe {
            sqlite3_str_appendf(
                p,
                (b"CREATE TABLE \"%w\".\"%w_rowid\"(rowid INTEGER PRIMARY KEY,nodeno\0".as_ptr()
                    as *mut i8) as *const i8,
                zDb,
                zPrefix,
            )
        };
        ii = 0 as i32;
        '__slate_break_1314: loop {
            if !(ii < (((unsafe { (*pRtree).nAux }) as u32) as i32)) {
                break;
            }
            unsafe { sqlite3_str_appendf(p, (b",a%d\0".as_ptr() as *mut i8) as *const i8, ii) };
            let __v1622: i32 = ii;
            let __v1623: i32 = __v1622 + (1 as i32);
            ii = __v1623;
        }
        unsafe {
            sqlite3_str_appendf(
                p,
                (b");CREATE TABLE \"%w\".\"%w_node\"(nodeno INTEGER PRIMARY KEY,data);\0".as_ptr()
                    as *mut i8) as *const i8,
                zDb,
                zPrefix,
            )
        };
        unsafe {
            sqlite3_str_appendf(
                p,
                (b"CREATE TABLE \"%w\".\"%w_parent\"(nodeno INTEGER PRIMARY KEY,parentnode);\0"
                    .as_ptr() as *mut i8) as *const i8,
                zDb,
                zPrefix,
            )
        };
        unsafe {
            sqlite3_str_appendf(
                p,
                (b"INSERT INTO \"%w\".\"%w_node\"VALUES(1,zeroblob(%d))\0".as_ptr() as *mut i8)
                    as *const i8,
                zDb,
                zPrefix,
                unsafe { (*pRtree).iNodeSize },
            )
        };
        zCreate = unsafe { sqlite3_str_finish(p) };
        if !(zCreate != std::ptr::null_mut::<i8>()) {
            return 7 as i32;
        }
        rc = unsafe {
            sqlite3_exec(
                db,
                zCreate as *const i8,
                None,
                std::ptr::null_mut::<()>(),
                std::ptr::null_mut::<*mut i8>(),
            )
        };
        unsafe { sqlite3_free(zCreate as *mut ()) };
        if rc != (0 as i32) {
            return rc;
        }
    }
    unsafe {
        *unsafe {
            (appStmt.0.as_mut_ptr() as *mut *mut *mut sqlite3_stmt).offset((0 as i32) as isize)
        } = unsafe { std::ptr::addr_of_mut!((*pRtree).pWriteNode) };
    }
    unsafe {
        *unsafe {
            (appStmt.0.as_mut_ptr() as *mut *mut *mut sqlite3_stmt).offset((1 as i32) as isize)
        } = unsafe { std::ptr::addr_of_mut!((*pRtree).pDeleteNode) };
    }
    unsafe {
        *unsafe {
            (appStmt.0.as_mut_ptr() as *mut *mut *mut sqlite3_stmt).offset((2 as i32) as isize)
        } = unsafe { std::ptr::addr_of_mut!((*pRtree).pReadRowid) };
    }
    unsafe {
        *unsafe {
            (appStmt.0.as_mut_ptr() as *mut *mut *mut sqlite3_stmt).offset((3 as i32) as isize)
        } = unsafe { std::ptr::addr_of_mut!((*pRtree).pWriteRowid) };
    }
    unsafe {
        *unsafe {
            (appStmt.0.as_mut_ptr() as *mut *mut *mut sqlite3_stmt).offset((4 as i32) as isize)
        } = unsafe { std::ptr::addr_of_mut!((*pRtree).pDeleteRowid) };
    }
    unsafe {
        *unsafe {
            (appStmt.0.as_mut_ptr() as *mut *mut *mut sqlite3_stmt).offset((5 as i32) as isize)
        } = unsafe { std::ptr::addr_of_mut!((*pRtree).pReadParent) };
    }
    unsafe {
        *unsafe {
            (appStmt.0.as_mut_ptr() as *mut *mut *mut sqlite3_stmt).offset((6 as i32) as isize)
        } = unsafe { std::ptr::addr_of_mut!((*pRtree).pWriteParent) };
    }
    unsafe {
        *unsafe {
            (appStmt.0.as_mut_ptr() as *mut *mut *mut sqlite3_stmt).offset((7 as i32) as isize)
        } = unsafe { std::ptr::addr_of_mut!((*pRtree).pDeleteParent) };
    }
    rc = rtreeQueryStat1(db, pRtree);
    i = 0 as i32;
    '__slate_break_1319: loop {
        if !(i < (8 as i32) && rc == (0 as i32)) {
            break;
        }
        let mut zSql: *mut i8 = unsafe { std::mem::zeroed() };
        let mut zFormat: *const i8 = unsafe { std::mem::zeroed() };
        if i != (3 as i32) || (((unsafe { (*pRtree).nAux }) as u32) as i32) == (0 as i32) {
            zFormat = unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!(azSql.0) as *mut *const i8 }.offset(i as isize)
                }
            };
        } else {
            // An UPSERT is very slightly slower than REPLACE, but it is needed
            // if there are auxiliary columns
            zFormat = (b"INSERT INTO\"%w\".\"%w_rowid\"(rowid,nodeno)VALUES(?1,?2)ON CONFLICT(rowid)DO UPDATE SET nodeno=excluded.nodeno\0".as_ptr() as *mut i8) as *const i8;
        }
        zSql = unsafe { sqlite3_mprintf(zFormat, zDb, zPrefix) };
        if zSql != std::ptr::null_mut::<i8>() {
            rc = unsafe {
                sqlite3_prepare_v3(
                    db,
                    zSql as *const i8,
                    -(1 as i32),
                    f as u32,
                    unsafe {
                        *unsafe {
                            (appStmt.0.as_mut_ptr() as *mut *mut *mut sqlite3_stmt)
                                .offset(i as isize)
                        }
                    },
                    std::ptr::null_mut::<*const i8>(),
                )
            };
        } else {
            rc = 7 as i32;
        }
        unsafe { sqlite3_free(zSql as *mut ()) };
        let __v1624: i32 = i;
        let __v1625: i32 = __v1624 + (1 as i32);
        i = __v1625;
    }
    if (unsafe { (*pRtree).nAux }) != (0 as u16) && rc != (7 as i32) {
        unsafe {
            (*pRtree).zReadAuxSql = unsafe {
                sqlite3_mprintf(
                    (b"SELECT * FROM \"%w\".\"%w_rowid\" WHERE rowid=?1\0".as_ptr() as *mut i8)
                        as *const i8,
                    zDb,
                    zPrefix,
                )
            };
        }
        if (unsafe { (*pRtree).zReadAuxSql }) == std::ptr::null_mut::<i8>() {
            rc = 7 as i32;
        } else {
            let mut p: *mut sqlite3_str = unsafe { sqlite3_str_new(db) };
            let mut ii: i32 = 0 as i32;
            let mut zSql: *mut i8 = unsafe { std::mem::zeroed() };
            unsafe {
                sqlite3_str_appendf(
                    p,
                    (b"UPDATE \"%w\".\"%w_rowid\"SET \0".as_ptr() as *mut i8) as *const i8,
                    zDb,
                    zPrefix,
                )
            };
            ii = 0 as i32;
            '__slate_break_1323: loop {
                if !(ii < (((unsafe { (*pRtree).nAux }) as u32) as i32)) {
                    break;
                }
                if ii != (0 as i32) {
                    unsafe {
                        sqlite3_str_append(p, (b",\0".as_ptr() as *mut i8) as *const i8, 1 as i32)
                    };
                }
                unsafe {
                    sqlite3_str_appendf(
                        p,
                        (b"a%d=?%d\0".as_ptr() as *mut i8) as *const i8,
                        ii,
                        ii + (2 as i32),
                    )
                };
                let __v1626: i32 = ii;
                let __v1627: i32 = __v1626 + (1 as i32);
                ii = __v1627;
            }
            unsafe {
                sqlite3_str_appendf(p, (b" WHERE rowid=?1\0".as_ptr() as *mut i8) as *const i8)
            };
            zSql = unsafe { sqlite3_str_finish(p) };
            if zSql == std::ptr::null_mut::<i8>() {
                rc = 7 as i32;
            } else {
                rc = unsafe {
                    sqlite3_prepare_v3(
                        db,
                        zSql as *const i8,
                        -(1 as i32),
                        f as u32,
                        unsafe { std::ptr::addr_of_mut!((*pRtree).pWriteAux) },
                        std::ptr::null_mut::<*const i8>(),
                    )
                };
                unsafe { sqlite3_free(zSql as *mut ()) };
            }
        }
    }
    return rc;
}

static mut azSql: __SlateAlign16<[*const i8; 8]> = __SlateAlign16([
    (b"INSERT OR REPLACE INTO '%q'.'%q_node' VALUES(?1, ?2)\0".as_ptr() as *mut i8) as *const i8,
    (b"DELETE FROM '%q'.'%q_node' WHERE nodeno = ?1\0".as_ptr() as *mut i8) as *const i8,
    (b"SELECT nodeno FROM '%q'.'%q_rowid' WHERE rowid = ?1\0".as_ptr() as *mut i8) as *const i8,
    (b"INSERT OR REPLACE INTO '%q'.'%q_rowid' VALUES(?1, ?2)\0".as_ptr() as *mut i8) as *const i8,
    (b"DELETE FROM '%q'.'%q_rowid' WHERE rowid = ?1\0".as_ptr() as *mut i8) as *const i8,
    (b"SELECT parentnode FROM '%q'.'%q_parent' WHERE nodeno = ?1\0".as_ptr() as *mut i8)
        as *const i8,
    (b"INSERT OR REPLACE INTO '%q'.'%q_parent' VALUES(?1, ?2)\0".as_ptr() as *mut i8) as *const i8,
    (b"DELETE FROM '%q'.'%q_parent' WHERE nodeno = ?1\0".as_ptr() as *mut i8) as *const i8,
]);

/// The second argument to this function contains the text of an SQL statement
/// that returns a single integer value. The statement is compiled and executed
/// using database connection db. If successful, the integer value returned
/// is written to *piVal and SQLITE_OK returned. Otherwise, an SQLite error
/// code is returned and the value of *piVal after returning is not defined.
fn getIntFromStmt(mut db: *mut sqlite3, mut zSql: *const i8, mut piVal: *mut i32) -> i32 {
    let mut rc: i32 = 7 as i32;
    if zSql != std::ptr::null::<i8>() {
        let mut pStmt: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
        rc = unsafe {
            sqlite3_prepare_v2(
                db,
                zSql,
                -(1 as i32),
                std::ptr::addr_of_mut!(pStmt),
                std::ptr::null_mut::<*const i8>(),
            )
        };
        if rc == (0 as i32) {
            if (100 as i32) == unsafe { sqlite3_step(pStmt) } {
                unsafe {
                    *piVal = unsafe { sqlite3_column_int(pStmt, 0 as i32) };
                }
            }
            rc = unsafe { sqlite3_finalize(pStmt) };
        }
    }
    return rc;
}

/// This function is called from within the xConnect() or xCreate() method to
/// determine the node-size used by the rtree table being created or connected
/// to. If successful, pRtree->iNodeSize is populated and SQLITE_OK returned.
/// Otherwise, an SQLite error code is returned.
///
/// If this function is being called as part of an xConnect(), then the rtree
/// table already exists. In this case the node-size is determined by inspecting
/// the root node of the tree.
///
/// Otherwise, for an xCreate(), use 64 bytes less than the database page-size.
/// This ensures that each node is stored on a single database page. If the
/// database page-size is so large that more than RTREE_MAXCELLS entries
/// would fit in a single node, use a smaller node-size.
///
/// # Arguments
///
/// * `db` - Database handle
/// * `pRtree` - Rtree handle
/// * `isCreate` - True for xCreate, false for xConnect
/// * `pzErr` - OUT: Error message, if any
fn getNodeSize(
    mut db: *mut sqlite3,
    mut pRtree: *mut Rtree,
    mut isCreate: i32,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut zSql: *mut i8 = unsafe { std::mem::zeroed() };
    if isCreate != (0 as i32) {
        let mut iPageSize: i32 = 0 as i32;
        zSql = unsafe {
            sqlite3_mprintf(
                (b"PRAGMA %Q.page_size\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pRtree).zDb },
            )
        };
        rc = getIntFromStmt(db, zSql as *const i8, std::ptr::addr_of_mut!(iPageSize));
        if rc == (0 as i32) {
            unsafe {
                (*pRtree).iNodeSize = iPageSize - (64 as i32);
            }
            if (4 as i32) + (((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32) * (51 as i32)
                < unsafe { (*pRtree).iNodeSize }
            {
                unsafe {
                    (*pRtree).iNodeSize = (4 as i32)
                        + (((unsafe { (*pRtree).nBytesPerCell }) as u32) as i32) * (51 as i32);
                }
            }
        } else {
            unsafe {
                *pzErr = unsafe {
                    sqlite3_mprintf((b"%s\0".as_ptr() as *mut i8) as *const i8, unsafe {
                        sqlite3_errmsg(db)
                    })
                };
            }
        }
    } else {
        zSql = unsafe {
            sqlite3_mprintf(
                (b"SELECT length(data) FROM '%q'.'%q_node' WHERE nodeno = 1\0".as_ptr() as *mut i8)
                    as *const i8,
                unsafe { (*pRtree).zDb },
                unsafe { (*pRtree).zName },
            )
        };
        rc = getIntFromStmt(db, zSql as *const i8, unsafe {
            std::ptr::addr_of_mut!((*pRtree).iNodeSize)
        });
        if rc != (0 as i32) {
            unsafe {
                *pzErr = unsafe {
                    sqlite3_mprintf((b"%s\0".as_ptr() as *mut i8) as *const i8, unsafe {
                        sqlite3_errmsg(db)
                    })
                };
            }
        } else {
            if (unsafe { (*pRtree).iNodeSize }) < (512 as i32) - (64 as i32) {
                rc = (11 as i32) | (1 as i32) << (8 as i32);
                {}
                unsafe {
                    *pzErr = unsafe {
                        sqlite3_mprintf(
                            (b"undersize RTree blobs in \"%q_node\"\0".as_ptr() as *mut i8)
                                as *const i8,
                            unsafe { (*pRtree).zName },
                        )
                    };
                }
            }
        }
    }
    unsafe { sqlite3_free(zSql as *mut ()) };
    return rc;
}

/// Return the length of a token
fn rtreeTokenLength(mut z: *const i8) -> i32 {
    let mut dummy: i32 = 0 as i32;
    return (unsafe { sqlite3GetToken(z as *const u8, std::ptr::addr_of_mut!(dummy)) }) as i32;
}

/// This function is the implementation of both the xConnect and xCreate
/// methods of the r-tree virtual table.
///
///   argv[0]   -> module name
///   argv[1]   -> database name
///   argv[2]   -> table name
///   argv[...] -> column names...
///
/// # Arguments
///
/// * `db` - Database connection
/// * `pAux` - One of the RTREE_COORD_* constants
/// * `argv` - Parameters to CREATE TABLE statement
/// * `ppVtab` - OUT: New virtual table
/// * `pzErr` - OUT: Error message, if any
/// * `isCreate` - True for xCreate, false for xConnect
fn rtreeInit(
    mut db: *mut sqlite3,
    mut pAux: *mut (),
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut i8,
    mut isCreate: i32,
) -> i32 {
    let mut __slate_storage_1434: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1434: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1434) as *mut i32;
    let mut __slate_storage_1427: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1427: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1427) as *mut i32;
    let mut __slate_storage_1426: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1426: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1426) as *mut i32;
    let mut __slate_storage_1430: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1430: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1430) as *mut u16;
    let mut __slate_storage_1429: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1429: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1429) as *mut u16;
    let mut __slate_storage_1428: std::mem::MaybeUninit<*mut Rtree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1428: *mut *mut Rtree =
        std::ptr::addr_of_mut!(__slate_storage_1428) as *mut *mut Rtree;
    let mut __slate_storage_1433: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1433: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1433) as *mut u8;
    let mut __slate_storage_1432: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1432: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1432) as *mut u8;
    let mut __slate_storage_1431: std::mem::MaybeUninit<*mut Rtree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1431: *mut *mut Rtree =
        std::ptr::addr_of_mut!(__slate_storage_1431) as *mut *mut Rtree;
    let mut __slate_storage_902: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_902: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_902) as *mut *const i8; // 0
    // 1
    // 2
    // 3
    // 4
    let mut __slate_storage_901: std::mem::MaybeUninit<__SlateAlign16<[*const i8; 5]>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_901: *mut [*const i8; 5] =
        std::ptr::addr_of_mut!(__slate_storage_901) as *mut [*const i8; 5];
    let mut __slate_storage_900: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_900: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_900) as *mut i32;
    let mut __slate_storage_899: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_899: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_899) as *mut i32;
    let mut __slate_storage_898: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_898: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_898) as *mut *mut i8;
    let mut __slate_storage_897: std::mem::MaybeUninit<*mut sqlite3_str> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_897: *mut *mut sqlite3_str =
        std::ptr::addr_of_mut!(__slate_storage_897) as *mut *mut sqlite3_str;
    let mut __slate_storage_896: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_896: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_896) as *mut i32; // Length of string argv[2]
    let mut __slate_storage_895: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_895: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_895) as *mut i32; // Length of string argv[1]
    let mut __slate_storage_894: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_894: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_894) as *mut i32;
    let mut __slate_storage_893: std::mem::MaybeUninit<*mut Rtree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_893: *mut *mut Rtree =
        std::ptr::addr_of_mut!(__slate_storage_893) as *mut *mut Rtree;
    let mut __slate_storage_892: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_892: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_892) as *mut i32;
    unsafe {
        std::ptr::write(__slate_slot_892, 0 as i32);
        std::ptr::write(
            __slate_slot_896,
            if pAux != std::ptr::null_mut::<()>() {
                1 as i32
            } else {
                0 as i32
            },
        );
        std::ptr::write(__slate_slot_899, 4 as i32);
        std::ptr::write(
            __slate_slot_901,
            [
                std::ptr::null::<i8>(),
                (b"Wrong number of columns for an rtree table\0".as_ptr() as *mut i8) as *const i8,
                (b"Too few columns for an rtree table\0".as_ptr() as *mut i8) as *const i8,
                (b"Too many columns for an rtree table\0".as_ptr() as *mut i8) as *const i8,
                (b"Auxiliary rtree columns must be last\0".as_ptr() as *mut i8) as *const i8,
            ],
        );
        0 as i32;
        if argc < (6 as i32) || argc > (100 as i32) + (3 as i32) {
            unsafe {
                *pzErr = unsafe {
                    sqlite3_mprintf((b"%s\0".as_ptr() as *mut i8) as *const i8, unsafe {
                        *unsafe {
                            ((*__slate_slot_901).as_mut_ptr() as *mut *const i8)
                                .offset(((2 as i32) + ((argc >= (6 as i32)) as i32)) as isize)
                        }
                    })
                };
            }
            return 1 as i32;
        } else {
            unsafe { sqlite3_vtab_config(db, 1 as i32, 1 as i32) };
            unsafe { sqlite3_vtab_config(db, 2 as i32) };
            // Allocate the sqlite3_vtab structure
            *__slate_slot_894 =
                ((unsafe { strlen(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) })
                    as u32) as i32;
            *__slate_slot_895 =
                ((unsafe { strlen(unsafe { *unsafe { argv.offset((2 as i32) as isize) } }) })
                    as u32) as i32;
            *__slate_slot_893 = (unsafe {
                sqlite3_malloc64(
                    (968 as u64)
                        .wrapping_add((*__slate_slot_894 as i64) as u64)
                        .wrapping_add(((*__slate_slot_895 * (2 as i32)) as i64) as u64)
                        .wrapping_add(((8 as i32) as i64) as u64),
                )
            }) as *mut Rtree;
            if !(*__slate_slot_893 != std::ptr::null_mut::<Rtree>()) {
                return 7 as i32;
            } else {
                unsafe {
                    memset(
                        *__slate_slot_893 as *mut (),
                        0 as i32,
                        (968 as u64)
                            .wrapping_add((*__slate_slot_894 as i64) as u64)
                            .wrapping_add(((*__slate_slot_895 * (2 as i32)) as i64) as u64)
                            .wrapping_add(((8 as i32) as i64) as u64),
                    )
                };
                unsafe {
                    (*(*__slate_slot_893)).nBusy = (1 as i32) as u32;
                }
                unsafe {
                    (*(*__slate_slot_893)).base.pModule =
                        (unsafe { std::ptr::addr_of_mut!(rtreeModule) }) as *const sqlite3_module;
                }
                unsafe {
                    (*(*__slate_slot_893)).zDb =
                        (unsafe { (*__slate_slot_893).offset((1 as i32) as isize) }) as *mut i8;
                }
                unsafe {
                    (*(*__slate_slot_893)).zName = unsafe {
                        unsafe { (*(*__slate_slot_893)).zDb }
                            .offset((*__slate_slot_894 + (1 as i32)) as isize)
                    };
                }
                unsafe {
                    (*(*__slate_slot_893)).zNodeName = unsafe {
                        unsafe { (*(*__slate_slot_893)).zName }
                            .offset((*__slate_slot_895 + (1 as i32)) as isize)
                    };
                }
                unsafe {
                    (*(*__slate_slot_893)).eCoordType = (*__slate_slot_896 as i8) as u8;
                }
                unsafe {
                    memcpy(
                        (unsafe { (*(*__slate_slot_893)).zDb }) as *mut (),
                        (unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) as *const (),
                        (*__slate_slot_894 as i64) as u64,
                    )
                };
                unsafe {
                    memcpy(
                        (unsafe { (*(*__slate_slot_893)).zName }) as *mut (),
                        (unsafe { *unsafe { argv.offset((2 as i32) as isize) } }) as *const (),
                        (*__slate_slot_895 as i64) as u64,
                    )
                };
                unsafe {
                    memcpy(
                        (unsafe { (*(*__slate_slot_893)).zNodeName }) as *mut (),
                        (unsafe { *unsafe { argv.offset((2 as i32) as isize) } }) as *const (),
                        (*__slate_slot_895 as i64) as u64,
                    )
                };
                unsafe {
                    memcpy(
                        (unsafe {
                            unsafe { (*(*__slate_slot_893)).zNodeName }
                                .offset(*__slate_slot_895 as isize)
                        }) as *mut (),
                        (b"_node\0".as_ptr() as *mut i8) as *const (),
                        ((6 as i32) as i64) as u64,
                    )
                };
                // Create/Connect to the underlying relational database schema. If
                // that is successful, call sqlite3_declare_vtab() to configure
                // the r-tree table schema.
                *__slate_slot_897 = unsafe { sqlite3_str_new(db) };
                unsafe {
                    sqlite3_str_appendf(
                        *__slate_slot_897,
                        (b"CREATE TABLE x(%.*s INT\0".as_ptr() as *mut i8) as *const i8,
                        rtreeTokenLength(unsafe { *unsafe { argv.offset((3 as i32) as isize) } }),
                        unsafe { *unsafe { argv.offset((3 as i32) as isize) } },
                    )
                };
                *__slate_slot_899 = 4 as i32;
                '__loop_23: loop {
                    if *__slate_slot_899 < argc {
                        std::ptr::write(__slate_slot_902, unsafe {
                            *unsafe { argv.offset(*__slate_slot_899 as isize) }
                        });
                        if ((unsafe { *unsafe { (*__slate_slot_902).offset((0 as i32) as isize) } })
                            as i32)
                            == (43 as i32)
                        {
                            std::ptr::write(__slate_slot_1428, *__slate_slot_893);
                            std::ptr::write(__slate_slot_1429, unsafe {
                                (*(*__slate_slot_1428)).nAux
                            });
                            std::ptr::write(
                                __slate_slot_1430,
                                ((((*__slate_slot_1429 as u32) as i32) + (1 as i32)) as i16) as u16,
                            );
                            unsafe {
                                (*(*__slate_slot_1428)).nAux = *__slate_slot_1430;
                            }
                            unsafe {
                                sqlite3_str_appendf(
                                    *__slate_slot_897,
                                    (b",%.*s\0".as_ptr() as *mut i8) as *const i8,
                                    rtreeTokenLength(unsafe {
                                        (*__slate_slot_902).offset((1 as i32) as isize)
                                    }),
                                    unsafe { (*__slate_slot_902).offset((1 as i32) as isize) },
                                )
                            };
                        } else {
                            if (((unsafe { (*(*__slate_slot_893)).nAux }) as u32) as i32)
                                > (0 as i32)
                            {
                                break '__loop_23;
                            } else {
                                std::ptr::write(__slate_slot_1431, *__slate_slot_893);
                                std::ptr::write(__slate_slot_1432, unsafe {
                                    (*(*__slate_slot_1431)).nDim2
                                });
                                std::ptr::write(
                                    __slate_slot_1433,
                                    ((((*__slate_slot_1432 as u32) as i32) + (1 as i32)) as i8)
                                        as u8,
                                );
                                unsafe {
                                    (*(*__slate_slot_1431)).nDim2 = *__slate_slot_1433;
                                }
                                unsafe {
                                    sqlite3_str_appendf(
                                        *__slate_slot_897,
                                        unsafe {
                                            *unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!(azFormat.0)
                                                        as *mut *const i8
                                                }
                                                .offset(*__slate_slot_896 as isize)
                                            }
                                        },
                                        rtreeTokenLength(*__slate_slot_902),
                                        *__slate_slot_902,
                                    )
                                };
                            }
                        }
                        std::ptr::write(__slate_slot_1426, *__slate_slot_899);
                        std::ptr::write(__slate_slot_1427, *__slate_slot_1426 + (1 as i32));
                        *__slate_slot_899 = *__slate_slot_1427;
                    } else {
                        break;
                    }
                }
                unsafe {
                    sqlite3_str_appendf(
                        *__slate_slot_897,
                        (b");\0".as_ptr() as *mut i8) as *const i8,
                    )
                };
                *__slate_slot_898 = unsafe { sqlite3_str_finish(*__slate_slot_897) };
                if !(*__slate_slot_898 != std::ptr::null_mut::<i8>()) {
                    *__slate_slot_892 = 7 as i32;
                } else {
                    if *__slate_slot_899 < argc {
                        unsafe {
                            *pzErr = unsafe {
                                sqlite3_mprintf(
                                    (b"%s\0".as_ptr() as *mut i8) as *const i8,
                                    unsafe {
                                        *unsafe {
                                            ((*__slate_slot_901).as_mut_ptr() as *mut *const i8)
                                                .offset((4 as i32) as isize)
                                        }
                                    },
                                )
                            };
                        }
                        *__slate_slot_892 = 1 as i32;
                    } else {
                        std::ptr::write(__slate_slot_1434, unsafe {
                            sqlite3_declare_vtab(db, *__slate_slot_898 as *const i8)
                        });
                        *__slate_slot_892 = *__slate_slot_1434;
                        if (0 as i32) != *__slate_slot_1434 {
                            unsafe {
                                *pzErr = unsafe {
                                    sqlite3_mprintf(
                                        (b"%s\0".as_ptr() as *mut i8) as *const i8,
                                        unsafe { sqlite3_errmsg(db) },
                                    )
                                };
                            }
                        }
                    }
                }
                unsafe { sqlite3_free(*__slate_slot_898 as *mut ()) };
                if *__slate_slot_892 != (0 as i32) {
                } else {
                    unsafe {
                        (*(*__slate_slot_893)).nDim =
                            (((((unsafe { (*(*__slate_slot_893)).nDim2 }) as u32) as i32)
                                / (2 as i32)) as i8) as u8;
                    }
                    if (((unsafe { (*(*__slate_slot_893)).nDim }) as u32) as i32) < (1 as i32) {
                        *__slate_slot_900 = 2 as i32;
                    } else {
                        if (((unsafe { (*(*__slate_slot_893)).nDim2 }) as u32) as i32)
                            > (5 as i32) * (2 as i32)
                        {
                            *__slate_slot_900 = 3 as i32;
                        } else {
                            if (((unsafe { (*(*__slate_slot_893)).nDim2 }) as u32) as i32)
                                % (2 as i32)
                                != (0 as i32)
                            {
                                *__slate_slot_900 = 1 as i32;
                            } else {
                                *__slate_slot_900 = 0 as i32;
                            }
                        }
                    }
                    if *__slate_slot_900 != (0 as i32) {
                        unsafe {
                            *pzErr = unsafe {
                                sqlite3_mprintf(
                                    (b"%s\0".as_ptr() as *mut i8) as *const i8,
                                    unsafe {
                                        *unsafe {
                                            ((*__slate_slot_901).as_mut_ptr() as *mut *const i8)
                                                .offset(*__slate_slot_900 as isize)
                                        }
                                    },
                                )
                            };
                        }
                    } else {
                        unsafe {
                            (*(*__slate_slot_893)).nBytesPerCell =
                                (((8 as i32)
                                    + (((unsafe { (*(*__slate_slot_893)).nDim2 }) as u32) as i32)
                                        * (4 as i32)) as i8) as u8;
                        }
                        // Figure out the node size to use.
                        *__slate_slot_892 = getNodeSize(db, *__slate_slot_893, isCreate, pzErr);
                        if *__slate_slot_892 != (0 as i32) {
                        } else {
                            *__slate_slot_892 = rtreeSqlInit(
                                *__slate_slot_893,
                                db,
                                unsafe { *unsafe { argv.offset((1 as i32) as isize) } },
                                unsafe { *unsafe { argv.offset((2 as i32) as isize) } },
                                isCreate,
                            );
                            if *__slate_slot_892 != (0 as i32) {
                                unsafe {
                                    *pzErr = unsafe {
                                        sqlite3_mprintf(
                                            (b"%s\0".as_ptr() as *mut i8) as *const i8,
                                            unsafe { sqlite3_errmsg(db) },
                                        )
                                    };
                                }
                            } else {
                                unsafe {
                                    *ppVtab = *__slate_slot_893 as *mut sqlite3_vtab;
                                }
                                return 0 as i32;
                            }
                        }
                    }
                }
                if *__slate_slot_892 == (0 as i32) {
                    *__slate_slot_892 = 1 as i32;
                }
                0 as i32;
                0 as i32;
                rtreeRelease(*__slate_slot_893);
                return *__slate_slot_892;
            }
        }
    }
    return unsafe { std::mem::zeroed() };
}

static mut azFormat: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b",%.*s REAL\0".as_ptr() as *mut i8) as *const i8,
    (b",%.*s INT\0".as_ptr() as *mut i8) as *const i8,
]);

/// Implementation of a scalar function that decodes r-tree nodes to
/// human readable strings. This can be used for debugging and analysis.
///
/// The scalar function takes two arguments: (1) the number of dimensions
/// to the rtree (between 1 and 5, inclusive) and (2) a blob of data containing
/// an r-tree node.  For a two-dimensional r-tree structure called "rt", to
/// deserialize all nodes, a statement like:
///
///   SELECT rtreenode(2, data) FROM rt_node;
///
/// The human readable string takes the form of a Tcl list with one
/// entry for each cell in the r-tree node. Each entry is itself a
/// list, containing the 8-byte rowid/pageno followed by the
/// <num-dimension>*2 coordinates.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreenode")]
extern "C-unwind" fn rtreenode(
    mut ctx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    let mut node: RtreeNode = unsafe { std::mem::zeroed() };
    let mut tree: Rtree = unsafe { std::mem::zeroed() };
    let mut ii: i32 = 0 as i32;
    let mut nData: i32 = 0 as i32;
    let mut errCode: i32 = 0 as i32;
    let mut pOut: *mut sqlite3_str = unsafe { std::mem::zeroed() };
    nArg;
    unsafe { memset(std::ptr::addr_of_mut!(node) as *mut (), 0 as i32, 40 as u64) };
    unsafe {
        memset(
            std::ptr::addr_of_mut!(tree) as *mut (),
            0 as i32,
            968 as u64,
        )
    };
    tree.nDim =
        ((unsafe { sqlite3_value_int(unsafe { *unsafe { apArg.offset((0 as i32) as isize) } }) })
            as i8) as u8;
    if ((tree.nDim as u32) as i32) < (1 as i32) || ((tree.nDim as u32) as i32) > (5 as i32) {
        return;
    }
    tree.nDim2 = ((((tree.nDim as u32) as i32) * (2 as i32)) as i8) as u8;
    tree.nBytesPerCell = (((8 as i32) + (8 as i32) * ((tree.nDim as u32) as i32)) as i8) as u8;
    node.zData =
        (unsafe { sqlite3_value_blob(unsafe { *unsafe { apArg.offset((1 as i32) as isize) } }) })
            as *mut u8;
    if node.zData == std::ptr::null_mut::<u8>() {
        return;
    }
    nData =
        unsafe { sqlite3_value_bytes(unsafe { *unsafe { apArg.offset((1 as i32) as isize) } }) };
    if nData < (4 as i32) {
        return;
    }
    if nData
        < (4 as i32)
            + readInt16(unsafe {
                unsafe { (*std::ptr::addr_of_mut!(node)).zData }.offset((2 as i32) as isize)
            }) * ((tree.nBytesPerCell as u32) as i32)
    {
        return;
    }
    pOut = unsafe { sqlite3_str_new(std::ptr::null_mut::<sqlite3>()) };
    ii = 0 as i32;
    '__slate_break_1348: loop {
        if !(ii
            < readInt16(unsafe {
                unsafe { (*std::ptr::addr_of_mut!(node)).zData }.offset((2 as i32) as isize)
            }))
        {
            break;
        }
        let mut cell: RtreeCell = unsafe { std::mem::zeroed() };
        let mut jj: i32 = 0 as i32;
        nodeGetCell(
            std::ptr::addr_of_mut!(tree),
            std::ptr::addr_of_mut!(node),
            ii,
            std::ptr::addr_of_mut!(cell),
        );
        if ii > (0 as i32) {
            unsafe {
                sqlite3_str_append(pOut, (b" \0".as_ptr() as *mut i8) as *const i8, 1 as i32)
            };
        }
        unsafe {
            sqlite3_str_appendf(
                pOut,
                (b"{%lld\0".as_ptr() as *mut i8) as *const i8,
                cell.iRowid,
            )
        };
        jj = 0 as i32;
        '__slate_break_1351: loop {
            if !(jj < ((tree.nDim2 as u32) as i32)) {
                break;
            }
            unsafe {
                sqlite3_str_appendf(
                    pOut,
                    (b" %g\0".as_ptr() as *mut i8) as *const i8,
                    (unsafe {
                        (*unsafe {
                            (cell.aCoord.as_mut_ptr() as *mut RtreeCoord).offset(jj as isize)
                        })
                        .f
                    }) as f64,
                )
            };
            let __v1630: i32 = jj;
            let __v1631: i32 = __v1630 + (1 as i32);
            jj = __v1631;
        }
        unsafe { sqlite3_str_append(pOut, (b"}\0".as_ptr() as *mut i8) as *const i8, 1 as i32) };
        let __v1628: i32 = ii;
        let __v1629: i32 = __v1628 + (1 as i32);
        ii = __v1629;
    }
    errCode = unsafe { sqlite3_str_errcode(pOut) };
    unsafe { sqlite3_result_error_code(ctx, errCode) };
    unsafe {
        sqlite3_result_text(
            ctx,
            (unsafe { sqlite3_str_finish(pOut) }) as *const i8,
            -(1 as i32),
            unsafe {
                std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    sqlite3_free as *const (),
                )
            },
        )
    };
}

/// This routine implements an SQL function that returns the "depth" parameter
/// from the front of a blob that is an r-tree node.  For example:
///
///     SELECT rtreedepth(data) FROM rt_node WHERE nodeno=1;
///
/// The depth value is 0 for all nodes other than the root node, and the root
/// node always has nodeno=1, so the example above is the primary use for this
/// routine.  This routine is intended for testing and analysis only.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreedepth")]
extern "C-unwind" fn rtreedepth(
    mut ctx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    nArg;
    let __v1632: bool;
    if (unsafe { sqlite3_value_type(unsafe { *unsafe { apArg.offset((0 as i32) as isize) } }) })
        != (4 as i32)
    {
        __v1632 = true as bool;
    } else {
        __v1632 = (unsafe {
            sqlite3_value_bytes(unsafe { *unsafe { apArg.offset((0 as i32) as isize) } })
        }) < (2 as i32);
    }
    if __v1632 {
        unsafe {
            sqlite3_result_error(
                ctx,
                (b"Invalid argument to rtreedepth()\0".as_ptr() as *mut i8) as *const i8,
                -(1 as i32),
            )
        };
    } else {
        let mut zBlob: *mut u8 = (unsafe {
            sqlite3_value_blob(unsafe { *unsafe { apArg.offset((0 as i32) as isize) } })
        }) as *mut u8;
        if zBlob != std::ptr::null_mut::<u8>() {
            unsafe { sqlite3_result_int(ctx, readInt16(zBlob)) };
        } else {
            unsafe { sqlite3_result_error_nomem(ctx) };
        }
    }
}

/// Context object passed between the various routines that make up the
/// implementation of integrity-check function rtreecheck().
#[repr(C)]
#[derive(Clone, Copy)]
struct RtreeCheck {
    /// Database handle
    db: *mut sqlite3,
    /// Database containing rtree table
    zDb: *const i8,
    /// Name of rtree table
    zTab: *const i8,
    /// True for rtree_i32 table
    bInt: i32,
    /// Number of dimensions for this rtree tbl
    nDim: i32,
    /// Statement used to retrieve nodes
    pGetNode: *mut sqlite3_stmt,
    /// Statements to query %_parent/%_rowid
    aCheckMapping: [*mut sqlite3_stmt; 2],
    /// Number of leaf cells in table
    nLeaf: i32,
    /// Number of non-leaf cells in table
    nNonLeaf: i32,
    /// Return code
    rc: i32,
    /// Message to report
    zReport: *mut i8,
    /// Number of lines in zReport
    nErr: i32,
}

/// Reset SQL statement pStmt. If the sqlite3_reset() call returns an error,
/// and RtreeCheck.rc==SQLITE_OK, set RtreeCheck.rc to the error code.
fn rtreeCheckReset(mut pCheck: *mut RtreeCheck, mut pStmt: *mut sqlite3_stmt) {
    let mut rc: i32 = unsafe { sqlite3_reset(pStmt) };
    if (unsafe { (*pCheck).rc }) == (0 as i32) {
        unsafe {
            (*pCheck).rc = rc;
        }
    }
}

/// The second and subsequent arguments to this function are a format string
/// and printf style arguments. This function formats the string and attempts
/// to compile it as an SQL statement.
///
/// If successful, a pointer to the new SQL statement is returned. Otherwise,
/// NULL is returned and an error code left in RtreeCheck.rc.
/// Format string and trailing args
///
/// # Arguments
///
/// * `pCheck` - RtreeCheck object
unsafe extern "C-unwind" fn rtreeCheckPrepare(
    mut pCheck: *mut RtreeCheck,
    mut zFmt: *const i8,
    mut __va_args: ...
) -> *mut sqlite3_stmt {
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    let mut pRet: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
    ap = __va_args.clone();
    z = unsafe { sqlite3_vmprintf(zFmt, ap.clone()) };
    if (unsafe { (*pCheck).rc }) == (0 as i32) {
        if z == std::ptr::null_mut::<i8>() {
            unsafe {
                (*pCheck).rc = 7 as i32;
            }
        } else {
            unsafe {
                (*pCheck).rc = unsafe {
                    sqlite3_prepare_v2(
                        unsafe { (*pCheck).db },
                        z as *const i8,
                        -(1 as i32),
                        std::ptr::addr_of_mut!(pRet),
                        std::ptr::null_mut::<*const i8>(),
                    )
                };
            }
        }
    }
    unsafe { sqlite3_free(z as *mut ()) };
    {}
    return pRet;
}

/// The second and subsequent arguments to this function are a printf()
/// style format string and arguments. This function formats the string and
/// appends it to the report being accumulated in pCheck.
unsafe extern "C-unwind" fn rtreeCheckAppendMsg(
    mut pCheck: *mut RtreeCheck,
    mut zFmt: *const i8,
    mut __va_args: ...
) {
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    ap = __va_args.clone();
    if (unsafe { (*pCheck).rc }) == (0 as i32) && (unsafe { (*pCheck).nErr }) < (100 as i32) {
        let mut z: *mut i8 = unsafe { sqlite3_vmprintf(zFmt, ap.clone()) };
        if z == std::ptr::null_mut::<i8>() {
            unsafe {
                (*pCheck).rc = 7 as i32;
            }
        } else {
            unsafe {
                (*pCheck).zReport = unsafe {
                    sqlite3_mprintf(
                        (b"%z%s%z\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { (*pCheck).zReport },
                        if (unsafe { (*pCheck).zReport }) != std::ptr::null_mut::<i8>() {
                            b"\n\0".as_ptr() as *mut i8
                        } else {
                            b"\0".as_ptr() as *mut i8
                        },
                        z,
                    )
                };
            }
            if (unsafe { (*pCheck).zReport }) == std::ptr::null_mut::<i8>() {
                unsafe {
                    (*pCheck).rc = 7 as i32;
                }
            }
        }
        let __v1633: *mut RtreeCheck = pCheck;
        let __v1634: i32 = unsafe { (*__v1633).nErr };
        let __v1635: i32 = __v1634 + (1 as i32);
        unsafe {
            (*__v1633).nErr = __v1635;
        }
    }
    {}
}

/// This function is a no-op if there is already an error code stored
/// in the RtreeCheck object indicated by the first argument. NULL is
/// returned in this case.
///
/// Otherwise, the contents of rtree table node iNode are loaded from
/// the database and copied into a buffer obtained from sqlite3_malloc().
/// If no error occurs, a pointer to the buffer is returned and (*pnNode)
/// is set to the size of the buffer in bytes.
///
/// Or, if an error does occur, NULL is returned and an error code left
/// in the RtreeCheck object. The final value of *pnNode is undefined in
/// this case.
fn rtreeCheckGetNode(mut pCheck: *mut RtreeCheck, mut iNode: i64, mut pnNode: *mut i32) -> *mut u8 {
    let mut pRet: *mut u8 = std::ptr::null_mut::<u8>(); // Return value
    if (unsafe { (*pCheck).rc }) == (0 as i32)
        && (unsafe { (*pCheck).pGetNode }) == std::ptr::null_mut::<sqlite3_stmt>()
    {
        unsafe {
            (*pCheck).pGetNode = unsafe {
                rtreeCheckPrepare(
                    pCheck,
                    (b"SELECT data FROM %Q.'%q_node' WHERE nodeno=?\0".as_ptr() as *mut i8)
                        as *const i8,
                    unsafe { (*pCheck).zDb },
                    unsafe { (*pCheck).zTab },
                )
            };
        }
    }
    if (unsafe { (*pCheck).rc }) == (0 as i32) {
        unsafe { sqlite3_bind_int64(unsafe { (*pCheck).pGetNode }, 1 as i32, iNode) };
        if (unsafe { sqlite3_step(unsafe { (*pCheck).pGetNode }) }) == (100 as i32) {
            let mut nNode: i32 =
                unsafe { sqlite3_column_bytes(unsafe { (*pCheck).pGetNode }, 0 as i32) };
            let mut pNode: *const u8 =
                (unsafe { sqlite3_column_blob(unsafe { (*pCheck).pGetNode }, 0 as i32) })
                    as *const u8;
            pRet = (unsafe { sqlite3_malloc64((nNode as i64) as u64) }) as *mut u8;
            if pRet == std::ptr::null_mut::<u8>() {
                unsafe {
                    (*pCheck).rc = 7 as i32;
                }
            } else {
                unsafe { memcpy(pRet as *mut (), pNode as *const (), (nNode as i64) as u64) };
                unsafe {
                    *pnNode = nNode;
                }
            }
        }
        rtreeCheckReset(pCheck, unsafe { (*pCheck).pGetNode });
        if (unsafe { (*pCheck).rc }) == (0 as i32) && pRet == std::ptr::null_mut::<u8>() {
            unsafe {
                rtreeCheckAppendMsg(
                    pCheck,
                    (b"Node %lld missing from database\0".as_ptr() as *mut i8) as *const i8,
                    iNode,
                )
            };
        }
    }
    return pRet;
}

/// This function is used to check that the %_parent (if bLeaf==0) or %_rowid
/// (if bLeaf==1) table contains a specified entry. The schemas of the
/// two tables are:
///
///   CREATE TABLE %_parent(nodeno INTEGER PRIMARY KEY, parentnode INTEGER)
///   CREATE TABLE %_rowid(rowid INTEGER PRIMARY KEY, nodeno INTEGER, ...)
///
/// In both cases, this function checks that there exists an entry with
/// IPK value iKey and the second column set to iVal.
///
/// # Arguments
///
/// * `pCheck` - RtreeCheck object
/// * `bLeaf` - True for a leaf cell, false for interior
/// * `iKey` - Key for mapping
/// * `iVal` - Expected value for mapping
fn rtreeCheckMapping(mut pCheck: *mut RtreeCheck, mut bLeaf: i32, mut iKey: i64, mut iVal: i64) {
    let mut rc: i32 = 0 as i32;
    let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    let mut azSql_952: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
        (b"SELECT parentnode FROM %Q.'%q_parent' WHERE nodeno=?1\0".as_ptr() as *mut i8)
            as *const i8,
        (b"SELECT nodeno FROM %Q.'%q_rowid' WHERE rowid=?1\0".as_ptr() as *mut i8) as *const i8,
    ]);
    0 as i32;
    if (unsafe {
        *unsafe {
            unsafe { (*pCheck).aCheckMapping.as_mut_ptr() as *mut *mut sqlite3_stmt }
                .offset(bLeaf as isize)
        }
    }) == std::ptr::null_mut::<sqlite3_stmt>()
    {
        unsafe {
            *unsafe {
                unsafe { (*pCheck).aCheckMapping.as_mut_ptr() as *mut *mut sqlite3_stmt }
                    .offset(bLeaf as isize)
            } = unsafe {
                rtreeCheckPrepare(
                    pCheck,
                    unsafe {
                        *unsafe {
                            (azSql_952.0.as_mut_ptr() as *mut *const i8).offset(bLeaf as isize)
                        }
                    },
                    unsafe { (*pCheck).zDb },
                    unsafe { (*pCheck).zTab },
                )
            };
        }
    }
    if (unsafe { (*pCheck).rc }) != (0 as i32) {
        return;
    }
    pStmt = unsafe {
        *unsafe {
            unsafe { (*pCheck).aCheckMapping.as_mut_ptr() as *mut *mut sqlite3_stmt }
                .offset(bLeaf as isize)
        }
    };
    unsafe { sqlite3_bind_int64(pStmt, 1 as i32, iKey) };
    rc = unsafe { sqlite3_step(pStmt) };
    if rc == (101 as i32) {
        unsafe {
            rtreeCheckAppendMsg(
                pCheck,
                (b"Mapping (%lld -> %lld) missing from %s table\0".as_ptr() as *mut i8)
                    as *const i8,
                iKey,
                iVal,
                if bLeaf != (0 as i32) {
                    b"%_rowid\0".as_ptr() as *mut i8
                } else {
                    b"%_parent\0".as_ptr() as *mut i8
                },
            )
        };
    } else {
        if rc == (100 as i32) {
            let mut ii: i64 = unsafe { sqlite3_column_int64(pStmt, 0 as i32) };
            if ii != iVal {
                unsafe {
                    rtreeCheckAppendMsg(
                        pCheck,
                        (b"Found (%lld -> %lld) in %s table, expected (%lld -> %lld)\0".as_ptr()
                            as *mut i8) as *const i8,
                        iKey,
                        ii,
                        if bLeaf != (0 as i32) {
                            b"%_rowid\0".as_ptr() as *mut i8
                        } else {
                            b"%_parent\0".as_ptr() as *mut i8
                        },
                        iKey,
                        iVal,
                    )
                };
            }
        }
    }
    rtreeCheckReset(pCheck, pStmt);
}

/// Argument pCell points to an array of coordinates stored on an rtree page.
/// This function checks that the coordinates are internally consistent (no
/// x1>x2 conditions) and adds an error message to the RtreeCheck object
/// if they are not.
///
/// Additionally, if pParent is not NULL, then it is assumed to point to
/// the array of coordinates on the parent page that bound the page
/// containing pCell. In this case it is also verified that the two
/// sets of coordinates are mutually consistent and an error message added
/// to the RtreeCheck object if they are not.
///
/// # Arguments
///
/// * `iNode` - Node id to use in error messages
/// * `iCell` - Cell number to use in error messages
/// * `pCell` - Pointer to cell coordinates
/// * `pParent` - Pointer to parent coordinates
fn rtreeCheckCellCoord(
    mut pCheck: *mut RtreeCheck,
    mut iNode: i64,
    mut iCell: i32,
    mut pCell: *mut u8,
    mut pParent: *mut u8,
) {
    let mut c1: RtreeCoord = unsafe { std::mem::zeroed() };
    let mut c2: RtreeCoord = unsafe { std::mem::zeroed() };
    let mut p1: RtreeCoord = unsafe { std::mem::zeroed() };
    let mut p2: RtreeCoord = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_1368: loop {
        if !(i < unsafe { (*pCheck).nDim }) {
            break;
        }
        readCoord(
            unsafe { pCell.offset(((4 as i32) * (2 as i32) * i) as isize) },
            std::ptr::addr_of_mut!(c1),
        );
        readCoord(
            unsafe { pCell.offset(((4 as i32) * ((2 as i32) * i + (1 as i32))) as isize) },
            std::ptr::addr_of_mut!(c2),
        );
        // printf("%e, %e\n", c1.u.f, c2.u.f);
        if (if (unsafe { (*pCheck).bInt }) != (0 as i32) {
            ((unsafe { c1.i }) > unsafe { c2.i }) as i32
        } else {
            ((unsafe { c1.f }) > unsafe { c2.f }) as i32
        }) != (0 as i32)
        {
            unsafe {
                rtreeCheckAppendMsg(
                    pCheck,
                    (b"Dimension %d of cell %d on node %lld is corrupt\0".as_ptr() as *mut i8)
                        as *const i8,
                    i,
                    iCell,
                    iNode,
                )
            };
        }
        if pParent != std::ptr::null_mut::<u8>() {
            readCoord(
                unsafe { pParent.offset(((4 as i32) * (2 as i32) * i) as isize) },
                std::ptr::addr_of_mut!(p1),
            );
            readCoord(
                unsafe { pParent.offset(((4 as i32) * ((2 as i32) * i + (1 as i32))) as isize) },
                std::ptr::addr_of_mut!(p2),
            );
            if (if (unsafe { (*pCheck).bInt }) != (0 as i32) {
                ((unsafe { c1.i }) < unsafe { p1.i }) as i32
            } else {
                ((unsafe { c1.f }) < unsafe { p1.f }) as i32
            }) != (0 as i32)
                || (if (unsafe { (*pCheck).bInt }) != (0 as i32) {
                    ((unsafe { c2.i }) > unsafe { p2.i }) as i32
                } else {
                    ((unsafe { c2.f }) > unsafe { p2.f }) as i32
                }) != (0 as i32)
            {
                unsafe {
                    rtreeCheckAppendMsg(
                        pCheck,
                        (b"Dimension %d of cell %d on node %lld is corrupt relative to parent\0"
                            .as_ptr() as *mut i8) as *const i8,
                        i,
                        iCell,
                        iNode,
                    )
                };
            }
        }
        let __v1636: i32 = i;
        let __v1637: i32 = __v1636 + (1 as i32);
        i = __v1637;
    }
}

/// Run rtreecheck() checks on node iNode, which is at depth iDepth within
/// the r-tree structure. Argument aParent points to the array of coordinates
/// that bound node iNode on the parent node.
///
/// If any problems are discovered, an error message is appended to the
/// report accumulated in the RtreeCheck object.
///
/// # Arguments
///
/// * `iDepth` - Depth of iNode (0==leaf)
/// * `aParent` - Buffer containing parent coords
/// * `iNode` - Node to check
fn rtreeCheckNode(
    mut pCheck: *mut RtreeCheck,
    mut iDepth: i32,
    mut aParent: *mut u8,
    mut iNode: i64,
) {
    let mut aNode: *mut u8 = std::ptr::null_mut::<u8>();
    let mut nNode: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    aNode = rtreeCheckGetNode(pCheck, iNode, std::ptr::addr_of_mut!(nNode));
    if aNode != std::ptr::null_mut::<u8>() {
        if nNode < (4 as i32) {
            unsafe {
                rtreeCheckAppendMsg(
                    pCheck,
                    (b"Node %lld is too small (%d bytes)\0".as_ptr() as *mut i8) as *const i8,
                    iNode,
                    nNode,
                )
            };
        } else {
            let mut nCell: i32 = 0 as i32; // Number of cells on page
            let mut i: i32 = 0 as i32; // Used to iterate through cells
            if aParent == std::ptr::null_mut::<u8>() {
                iDepth = readInt16(aNode);
                if iDepth > (40 as i32) {
                    unsafe {
                        rtreeCheckAppendMsg(
                            pCheck,
                            (b"Rtree depth out of range (%d)\0".as_ptr() as *mut i8) as *const i8,
                            iDepth,
                        )
                    };
                    unsafe { sqlite3_free(aNode as *mut ()) };
                    return;
                }
            }
            nCell = readInt16(unsafe { aNode.offset((2 as i32) as isize) });
            if (4 as i32)
                + nCell * ((8 as i32) + (unsafe { (*pCheck).nDim }) * (2 as i32) * (4 as i32))
                > nNode
            {
                unsafe {
                    rtreeCheckAppendMsg(
                        pCheck,
                        (b"Node %lld is too small for cell count of %d (%d bytes)\0".as_ptr()
                            as *mut i8) as *const i8,
                        iNode,
                        nCell,
                        nNode,
                    )
                };
            } else {
                i = 0 as i32;
                '__slate_break_1374: loop {
                    if !(i < nCell) {
                        break;
                    }
                    let mut pCell: *mut u8 = unsafe {
                        aNode.offset(
                            ((4 as i32)
                                + i * ((8 as i32)
                                    + (unsafe { (*pCheck).nDim }) * (2 as i32) * (4 as i32)))
                                as isize,
                        )
                    };
                    let mut iVal: i64 = readInt64(pCell);
                    rtreeCheckCellCoord(
                        pCheck,
                        iNode,
                        i,
                        unsafe { pCell.offset((8 as i32) as isize) },
                        aParent,
                    );
                    if iDepth > (0 as i32) {
                        rtreeCheckMapping(pCheck, 0 as i32, iVal, iNode);
                        rtreeCheckNode(
                            pCheck,
                            iDepth - (1 as i32),
                            unsafe { pCell.offset((8 as i32) as isize) },
                            iVal,
                        );
                        let __v1640: *mut RtreeCheck = pCheck;
                        let __v1641: i32 = unsafe { (*__v1640).nNonLeaf };
                        let __v1642: i32 = __v1641 + (1 as i32);
                        unsafe {
                            (*__v1640).nNonLeaf = __v1642;
                        }
                    } else {
                        rtreeCheckMapping(pCheck, 1 as i32, iVal, iNode);
                        let __v1643: *mut RtreeCheck = pCheck;
                        let __v1644: i32 = unsafe { (*__v1643).nLeaf };
                        let __v1645: i32 = __v1644 + (1 as i32);
                        unsafe {
                            (*__v1643).nLeaf = __v1645;
                        }
                    }
                    let __v1638: i32 = i;
                    let __v1639: i32 = __v1638 + (1 as i32);
                    i = __v1639;
                }
            }
        }
        unsafe { sqlite3_free(aNode as *mut ()) };
    }
}

/// The second argument to this function must be either "_rowid" or
/// "_parent". This function checks that the number of entries in the
/// %_rowid or %_parent table is exactly nExpect. If not, it adds
/// an error message to the report in the RtreeCheck object indicated
/// by the first argument.
fn rtreeCheckCount(mut pCheck: *mut RtreeCheck, mut zTbl: *const i8, mut nExpect: i64) {
    if (unsafe { (*pCheck).rc }) == (0 as i32) {
        let mut pCount: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
        pCount = unsafe {
            rtreeCheckPrepare(
                pCheck,
                (b"SELECT count(*) FROM %Q.'%q%s'\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pCheck).zDb },
                unsafe { (*pCheck).zTab },
                zTbl,
            )
        };
        if pCount != std::ptr::null_mut::<sqlite3_stmt>() {
            if (unsafe { sqlite3_step(pCount) }) == (100 as i32) {
                let mut nActual: i64 = unsafe { sqlite3_column_int64(pCount, 0 as i32) };
                if nActual != nExpect {
                    unsafe {
                        rtreeCheckAppendMsg(
                            pCheck,
                            (b"Wrong number of entries in %%%s table - expected %lld, actual %lld\0"
                                .as_ptr() as *mut i8) as *const i8,
                            zTbl,
                            nExpect,
                            nActual,
                        )
                    };
                }
            }
            unsafe {
                (*pCheck).rc = unsafe { sqlite3_finalize(pCount) };
            }
        }
    }
}

/// This function does the bulk of the work for the rtree integrity-check.
/// It is called by rtreecheck(), which is the SQL function implementation.
///
/// # Arguments
///
/// * `db` - Database handle to access db through
/// * `zDb` - Name of db ("main", "temp" etc.)
/// * `zTab` - Name of rtree table to check
/// * `pzReport` - OUT: sqlite3_malloc'd report text
fn rtreeCheckTable(
    mut db: *mut sqlite3,
    mut zDb: *const i8,
    mut zTab: *const i8,
    mut pzReport: *mut *mut i8,
) -> i32 {
    let mut check: RtreeCheck = unsafe { std::mem::zeroed() }; // Common context for various routines
    let mut pStmt: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // Used to find column count of rtree table
    let mut nAux: i32 = 0 as i32; // Number of extra columns.
    // Initialize the context object
    unsafe {
        memset(
            std::ptr::addr_of_mut!(check) as *mut (),
            0 as i32,
            88 as u64,
        )
    };
    check.db = db;
    check.zDb = zDb;
    check.zTab = zTab;
    // Find the number of auxiliary columns
    pStmt = unsafe {
        rtreeCheckPrepare(
            std::ptr::addr_of_mut!(check),
            (b"SELECT * FROM %Q.'%q_rowid'\0".as_ptr() as *mut i8) as *const i8,
            zDb,
            zTab,
        )
    };
    if pStmt != std::ptr::null_mut::<sqlite3_stmt>() {
        nAux = (unsafe { sqlite3_column_count(pStmt) }) - (2 as i32);
        unsafe { sqlite3_finalize(pStmt) };
    } else {
        if check.rc != (7 as i32) {
            check.rc = 0 as i32;
        }
    }
    // Find number of dimensions in the rtree table.
    pStmt = unsafe {
        rtreeCheckPrepare(
            std::ptr::addr_of_mut!(check),
            (b"SELECT * FROM %Q.%Q\0".as_ptr() as *mut i8) as *const i8,
            zDb,
            zTab,
        )
    };
    if pStmt != std::ptr::null_mut::<sqlite3_stmt>() {
        let mut rc: i32 = 0 as i32;
        check.nDim = ((unsafe { sqlite3_column_count(pStmt) }) - (1 as i32) - nAux) / (2 as i32);
        if check.nDim < (1 as i32) {
            unsafe {
                rtreeCheckAppendMsg(
                    std::ptr::addr_of_mut!(check),
                    (b"Schema corrupt or not an rtree\0".as_ptr() as *mut i8) as *const i8,
                )
            };
        } else {
            if (100 as i32) == unsafe { sqlite3_step(pStmt) } {
                check.bInt =
                    ((unsafe { sqlite3_column_type(pStmt, 1 as i32) }) == (1 as i32)) as i32;
            }
        }
        rc = unsafe { sqlite3_finalize(pStmt) };
        if rc != (11 as i32) {
            check.rc = rc;
        }
    }
    // Do the actual integrity-check
    if check.nDim >= (1 as i32) {
        if check.rc == (0 as i32) {
            rtreeCheckNode(
                std::ptr::addr_of_mut!(check),
                0 as i32,
                std::ptr::null_mut::<u8>(),
                (1 as i32) as i64,
            );
        }
        rtreeCheckCount(
            std::ptr::addr_of_mut!(check),
            (b"_rowid\0".as_ptr() as *mut i8) as *const i8,
            check.nLeaf as i64,
        );
        rtreeCheckCount(
            std::ptr::addr_of_mut!(check),
            (b"_parent\0".as_ptr() as *mut i8) as *const i8,
            check.nNonLeaf as i64,
        );
    }
    // Finalize SQL statements used by the integrity-check
    unsafe { sqlite3_finalize(check.pGetNode) };
    unsafe {
        sqlite3_finalize(unsafe {
            *unsafe {
                (check.aCheckMapping.as_mut_ptr() as *mut *mut sqlite3_stmt)
                    .offset((0 as i32) as isize)
            }
        })
    };
    unsafe {
        sqlite3_finalize(unsafe {
            *unsafe {
                (check.aCheckMapping.as_mut_ptr() as *mut *mut sqlite3_stmt)
                    .offset((1 as i32) as isize)
            }
        })
    };
    unsafe {
        *pzReport = check.zReport;
    }
    return check.rc;
}

/// Implementation of the xIntegrity method for Rtree.
///
/// # Arguments
///
/// * `pVtab` - The virtual table to check
/// * `zSchema` - Schema in which the virtual table lives
/// * `zName` - Name of the virtual table
/// * `isQuick` - True for a quick_check
/// * `pzErr` - Write results here
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeIntegrity")]
extern "C-unwind" fn rtreeIntegrity(
    mut pVtab: *mut sqlite3_vtab,
    mut zSchema: *const i8,
    mut zName: *const i8,
    mut isQuick: i32,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut pRtree: *mut Rtree = pVtab as *mut Rtree;
    let mut rc: i32 = 0 as i32;
    0 as i32;
    zSchema;
    zName;
    isQuick;
    rc = rtreeCheckTable(
        unsafe { (*pRtree).db },
        (unsafe { (*pRtree).zDb }) as *const i8,
        (unsafe { (*pRtree).zName }) as *const i8,
        pzErr,
    );
    if rc == (0 as i32) && (unsafe { *pzErr }) != std::ptr::null_mut::<i8>() {
        unsafe {
            *pzErr = unsafe {
                sqlite3_mprintf(
                    (b"In RTree %s.%s:\n%z\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*pRtree).zDb },
                    unsafe { (*pRtree).zName },
                    unsafe { *pzErr },
                )
            };
        }
        if (unsafe { *pzErr }) == std::ptr::null_mut::<i8>() {
            rc = 7 as i32;
        }
    }
    return rc;
}

/// Usage:
///
///   rtreecheck(<rtree-table>);
///   rtreecheck(<database>, <rtree-table>);
///
/// Invoking this SQL function runs an integrity-check on the named rtree
/// table. The integrity-check verifies the following:
///
///   1. For each cell in the r-tree structure (%_node table), that:
///
///       a) for each dimension, (coord1 <= coord2).
///
///       b) unless the cell is on the root node, that the cell is bounded
///          by the parent cell on the parent node.
///
///       c) for leaf nodes, that there is an entry in the %_rowid
///          table corresponding to the cell's rowid value that
///          points to the correct node.
///
///       d) for cells on non-leaf nodes, that there is an entry in the
///          %_parent table mapping from the cell's child node to the
///          node that it resides on.
///
///   2. That there are the same number of entries in the %_rowid table
///      as there are leaf cells in the r-tree structure, and that there
///      is a leaf cell that corresponds to each entry in the %_rowid table.
///
///   3. That there are the same number of entries in the %_parent table
///      as there are non-leaf cells in the r-tree structure, and that
///      there is a non-leaf cell that corresponds to each entry in the
///      %_parent table.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreecheck")]
extern "C-unwind" fn rtreecheck(
    mut ctx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    if nArg != (1 as i32) && nArg != (2 as i32) {
        unsafe {
            sqlite3_result_error(
                ctx,
                (b"wrong number of arguments to function rtreecheck()\0".as_ptr() as *mut i8)
                    as *const i8,
                -(1 as i32),
            )
        };
    } else {
        let mut rc: i32 = 0 as i32;
        let mut zReport: *mut i8 = std::ptr::null_mut::<i8>();
        let mut zDb: *const i8 = (unsafe {
            sqlite3_value_text(unsafe { *unsafe { apArg.offset((0 as i32) as isize) } })
        }) as *const i8;
        let mut zTab: *const i8 = unsafe { std::mem::zeroed() };
        if nArg == (1 as i32) {
            zTab = zDb;
            zDb = (b"main\0".as_ptr() as *mut i8) as *const i8;
        } else {
            zTab = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { apArg.offset((1 as i32) as isize) } })
            }) as *const i8;
        }
        rc = rtreeCheckTable(
            unsafe { sqlite3_context_db_handle(ctx) },
            zDb,
            zTab,
            std::ptr::addr_of_mut!(zReport),
        );
        if rc == (0 as i32) {
            unsafe {
                sqlite3_result_text(
                    ctx,
                    (if zReport != std::ptr::null_mut::<i8>() {
                        zReport
                    } else {
                        b"ok\0".as_ptr() as *mut i8
                    }) as *const i8,
                    -(1 as i32),
                    unsafe {
                        std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                            -(1 as i32) as usize,
                        )
                    },
                )
            };
        } else {
            unsafe { sqlite3_result_error_code(ctx, rc) };
        }
        unsafe { sqlite3_free(zReport as *mut ()) };
    }
}

// Conditionally include the geopoly code
/// Register the r-tree module with database handle db. This creates the
/// virtual table module "rtree" and the debugging/analysis scalar
/// function "rtreenode".
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.rtree.sqlite3RtreeInit")]
extern "C-unwind" fn sqlite3RtreeInit(mut db: *mut sqlite3) -> i32 {
    let mut utf8: i32 = 1 as i32;
    let mut rc: i32 = 0 as i32;
    rc = unsafe {
        sqlite3_create_function(
            db,
            (b"rtreenode\0".as_ptr() as *mut i8) as *const i8,
            2 as i32,
            utf8,
            std::ptr::null_mut::<()>(),
            Some(rtreenode),
            None,
            None,
        )
    };
    if rc == (0 as i32) {
        rc = unsafe {
            sqlite3_create_function(
                db,
                (b"rtreedepth\0".as_ptr() as *mut i8) as *const i8,
                1 as i32,
                utf8,
                std::ptr::null_mut::<()>(),
                Some(rtreedepth),
                None,
                None,
            )
        };
    }
    if rc == (0 as i32) {
        rc = unsafe {
            sqlite3_create_function(
                db,
                (b"rtreecheck\0".as_ptr() as *mut i8) as *const i8,
                -(1 as i32),
                utf8,
                std::ptr::null_mut::<()>(),
                Some(rtreecheck),
                None,
                None,
            )
        };
    }
    if rc == (0 as i32) {
        let mut c: *mut () = std::ptr::null_mut::<()>();
        rc = unsafe {
            sqlite3_create_module_v2(
                db,
                (b"rtree\0".as_ptr() as *mut i8) as *const i8,
                (unsafe { std::ptr::addr_of_mut!(rtreeModule) }) as *const sqlite3_module,
                c,
                None,
            )
        };
    }
    if rc == (0 as i32) {
        let mut c: *mut () = (1 as i32) as *mut ();
        rc = unsafe {
            sqlite3_create_module_v2(
                db,
                (b"rtree_i32\0".as_ptr() as *mut i8) as *const i8,
                (unsafe { std::ptr::addr_of_mut!(rtreeModule) }) as *const sqlite3_module,
                c,
                None,
            )
        };
    }
    return rc;
}

/// This routine deletes the RtreeGeomCallback object that was attached
/// one of the SQL functions create by sqlite3_rtree_geometry_callback()
/// or sqlite3_rtree_query_callback().  In other words, this routine is the
/// destructor for an RtreeGeomCallback objecct.  This routine is called when
/// the corresponding SQL function is deleted.
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeFreeCallback")]
extern "C-unwind" fn rtreeFreeCallback(mut p: *mut ()) {
    let mut pInfo: *mut RtreeGeomCallback = p as *mut RtreeGeomCallback;
    if (unsafe { (*pInfo).xDestructor }) != None {
        unsafe { unsafe { (*pInfo).xDestructor }.unwrap()(unsafe { (*pInfo).pContext }) };
    }
    unsafe { sqlite3_free(p) };
}

/// This routine frees the BLOB that is returned by geomCallback().
#[unsafe(link_section = ".text.slate_distinct.rtree.rtreeMatchArgFree")]
extern "C-unwind" fn rtreeMatchArgFree(mut pArg: *mut ()) {
    let mut i: i32 = 0 as i32;
    let mut p: *mut RtreeMatchArg = pArg as *mut RtreeMatchArg;
    i = 0 as i32;
    '__slate_break_1391: loop {
        if !(i < unsafe { (*p).nParam }) {
            break;
        }
        unsafe {
            sqlite3_value_free(unsafe { *unsafe { unsafe { (*p).apSqlParam }.offset(i as isize) } })
        };
        let __v1646: i32 = i;
        let __v1647: i32 = __v1646 + (1 as i32);
        i = __v1647;
    }
    unsafe { sqlite3_free(p as *mut ()) };
}

/// Each call to sqlite3_rtree_geometry_callback() or
/// sqlite3_rtree_query_callback() creates an ordinary SQLite
/// scalar function that is implemented by this routine.
///
/// All this function does is construct an RtreeMatchArg object that
/// contains the geometry-checking callback routines and a list of
/// parameters to this function, then return that RtreeMatchArg object
/// as a BLOB.
///
/// The R-Tree MATCH operator will read the returned BLOB, deserialize
/// the RtreeMatchArg object, and use the RtreeMatchArg object to figure
/// out which elements of the R-Tree should be returned by the query.
#[unsafe(link_section = ".text.slate_distinct.rtree.geomCallback")]
extern "C-unwind" fn geomCallback(
    mut ctx: *mut sqlite3_context,
    mut nArg: i32,
    mut aArg: *mut *mut sqlite3_value,
) {
    let mut pGeomCtx: *mut RtreeGeomCallback =
        (unsafe { sqlite3_user_data(ctx) }) as *mut RtreeGeomCallback;
    let mut pBlob: *mut RtreeMatchArg = unsafe { std::mem::zeroed() };
    let mut nBlob: i64 = 0 as i64;
    let mut memErr: i32 = 0 as i32;
    nBlob = (56 as u64)
        .wrapping_add(((nArg as i64) as u64).wrapping_mul(8 as u64))
        .wrapping_add(((nArg as i64) as u64).wrapping_mul(8 as u64)) as i64;
    pBlob = (unsafe { sqlite3_malloc64(nBlob as u64) }) as *mut RtreeMatchArg;
    if !(pBlob != std::ptr::null_mut::<RtreeMatchArg>()) {
        unsafe { sqlite3_result_error_nomem(ctx) };
    } else {
        let mut i: i32 = 0 as i32;
        unsafe {
            (*pBlob).iSize = (nBlob as i32) as u32;
        }
        unsafe {
            (*pBlob).cb = unsafe { *unsafe { pGeomCtx.offset((0 as i32) as isize) } };
        }
        unsafe {
            (*pBlob).apSqlParam = (unsafe {
                unsafe { std::ptr::addr_of_mut!((*pBlob).aParam) as *mut f64 }.offset(nArg as isize)
            }) as *mut *mut sqlite3_value;
        }
        unsafe {
            (*pBlob).nParam = nArg;
        }
        i = 0 as i32;
        '__slate_break_1392: loop {
            if !(i < nArg) {
                break;
            }
            unsafe {
                *unsafe { unsafe { (*pBlob).apSqlParam }.offset(i as isize) } = unsafe {
                    sqlite3_value_dup(
                        (unsafe { *unsafe { aArg.offset(i as isize) } }) as *const sqlite3_value,
                    )
                };
            }
            if (unsafe { *unsafe { unsafe { (*pBlob).apSqlParam }.offset(i as isize) } })
                == std::ptr::null_mut::<sqlite3_value>()
            {
                memErr = 1 as i32;
            }
            unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pBlob).aParam) as *mut f64 }
                        .offset(i as isize)
                } = unsafe { sqlite3_value_double(unsafe { *unsafe { aArg.offset(i as isize) } }) };
            }
            let __v1648: i32 = i;
            let __v1649: i32 = __v1648 + (1 as i32);
            i = __v1649;
        }
        if memErr != (0 as i32) {
            unsafe { sqlite3_result_error_nomem(ctx) };
            rtreeMatchArgFree(pBlob as *mut ());
        } else {
            unsafe {
                sqlite3_result_pointer(
                    ctx,
                    pBlob as *mut (),
                    (b"RtreeMatchArg\0".as_ptr() as *mut i8) as *const i8,
                    Some(rtreeMatchArgFree),
                )
            };
        }
    }
}

/// Register a new geometry function for use with the r-tree MATCH operator.
///
/// # Arguments
///
/// * `db` - Register SQL function on this connection
/// * `zGeom` - Name of the new SQL function
/// * `xGeom` - Callback
/// * `pContext` - Extra data associated with the callback
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3_rtree_geometry_callback(
    mut db: *mut sqlite3,
    mut zGeom: *const i8,
    mut xGeom: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_rtree_geometry, i32, *mut f64, *mut i32) -> i32,
    >,
    mut pContext: *mut (),
) -> i32 {
    let mut pGeomCtx: *mut RtreeGeomCallback = unsafe { std::mem::zeroed() }; // Context object for new user-function
    // Allocate and populate the context object.
    pGeomCtx = (unsafe { sqlite3_malloc(((32 as u64) as u32) as i32) }) as *mut RtreeGeomCallback;
    if !(pGeomCtx != std::ptr::null_mut::<RtreeGeomCallback>()) {
        return 7 as i32;
    }
    unsafe {
        (*pGeomCtx).xGeom = xGeom;
    }
    unsafe {
        (*pGeomCtx).xQueryFunc = None;
    }
    unsafe {
        (*pGeomCtx).xDestructor = None;
    }
    unsafe {
        (*pGeomCtx).pContext = pContext;
    }
    return unsafe {
        sqlite3_create_function_v2(
            db,
            zGeom,
            -(1 as i32),
            5 as i32,
            pGeomCtx as *mut (),
            Some(geomCallback),
            None,
            None,
            Some(rtreeFreeCallback),
        )
    };
}

/// Register a new 2nd-generation geometry function for use with the
/// r-tree MATCH operator.
///
/// # Arguments
///
/// * `db` - Register SQL function on this connection
/// * `zQueryFunc` - Name of new SQL function
/// * `xQueryFunc` - Callback
/// * `pContext` - Extra data passed into the callback
/// * `xDestructor` - Destructor for the extra data
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3_rtree_query_callback(
    mut db: *mut sqlite3,
    mut zQueryFunc: *const i8,
    mut xQueryFunc: Option<unsafe extern "C-unwind" fn(*mut sqlite3_rtree_query_info) -> i32>,
    mut pContext: *mut (),
    mut xDestructor: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    let mut pGeomCtx: *mut RtreeGeomCallback = unsafe { std::mem::zeroed() }; // Context object for new user-function
    // Allocate and populate the context object.
    pGeomCtx = (unsafe { sqlite3_malloc(((32 as u64) as u32) as i32) }) as *mut RtreeGeomCallback;
    if !(pGeomCtx != std::ptr::null_mut::<RtreeGeomCallback>()) {
        if xDestructor != None {
            unsafe { xDestructor.unwrap()(pContext) };
        }
        return 7 as i32;
    }
    unsafe {
        (*pGeomCtx).xGeom = None;
    }
    unsafe {
        (*pGeomCtx).xQueryFunc = xQueryFunc;
    }
    unsafe {
        (*pGeomCtx).xDestructor = xDestructor;
    }
    unsafe {
        (*pGeomCtx).pContext = pContext;
    }
    return unsafe {
        sqlite3_create_function_v2(
            db,
            zQueryFunc,
            -(1 as i32),
            5 as i32,
            pGeomCtx as *mut (),
            Some(geomCallback),
            None,
            None,
            Some(rtreeFreeCallback),
        )
    };
}
