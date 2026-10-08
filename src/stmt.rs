//! 2017-05-31
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
//! This file demonstrates an eponymous virtual table that returns information
//! about all prepared statements for the database connection.
//!
//! Usage example:
//!
//!     .load ./stmt
//!     .mode line
//!     .header on
//!     SELECT * FROM stmt;
unsafe extern "C" {
    fn sqlite3_malloc64(__v160: u64) -> *mut ();
    fn sqlite3_free(__v161: *mut ());
    fn sqlite3_sql(pStmt: *mut sqlite3_stmt) -> *const i8;
    fn sqlite3_stmt_readonly(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_stmt_busy(__v164: *mut sqlite3_stmt) -> i32;
    fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_result_int(__v166: *mut sqlite3_context, __v167: i32);
    fn sqlite3_result_text(
        __v168: *mut sqlite3_context,
        __v169: *const i8,
        __v170: i32,
        __v171: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_next_stmt(pDb: *mut sqlite3, pStmt: *mut sqlite3_stmt) -> *mut sqlite3_stmt;
    fn sqlite3_create_module(
        db: *mut sqlite3,
        zName: *const i8,
        p: *const sqlite3_module,
        pClientData: *mut (),
    ) -> i32;
    fn sqlite3_declare_vtab(__v178: *mut sqlite3, zSQL: *const i8) -> i32;
    fn sqlite3_stmt_status(__v180: *mut sqlite3_stmt, op: i32, resetFlg: i32) -> i32;
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strlen(__s: *const i8) -> u64;
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
struct StmtRow {
    /// Rowid value
    iRowid: i64,
    /// column "sql"
    zSql: *mut i8,
    /// all other column values
    aCol: [i32; 11],
    /// Next row to return
    pNext: *mut StmtRow,
}

/// stmt_vtab is a subclass of sqlite3_vtab which will
/// serve as the underlying representation of a stmt virtual table
#[repr(C)]
#[derive(Clone, Copy)]
struct stmt_vtab {
    /// Base class - must be first
    base: sqlite3_vtab,
    /// Database connection for this stmt vtab
    db: *mut sqlite3,
}

/// stmt_cursor is a subclass of sqlite3_vtab_cursor which will
/// serve as the underlying representation of a cursor that scans
/// over rows of the result
#[repr(C)]
#[derive(Clone, Copy)]
struct stmt_cursor {
    /// Base class - must be first
    base: sqlite3_vtab_cursor,
    /// Database connection for this cursor
    db: *mut sqlite3,
    /// Current row
    pRow: *mut StmtRow,
}

/// The stmtConnect() method is invoked to create a new
/// stmt_vtab that describes the stmt virtual table.
///
/// Think of this routine as the constructor for stmt_vtab objects.
///
/// All this routine needs to do is:
///
///    (1) Allocate the stmt_vtab object and initialize all fields.
///
///    (2) Tell SQLite (via the sqlite3_declare_vtab() interface) what the
///        result set of queries against stmt will look like.
#[unsafe(link_section = ".text.slate_distinct.stmt.stmtConnect")]
extern "C-unwind" fn stmtConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut (),
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut pNew: *mut stmt_vtab = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    // Column numbers
    // SQL for the statement
    // Number of result columns
    // True if read-only
    // True if currently busy
    // SQLITE_STMTSTATUS_FULLSCAN_STEP
    // SQLITE_STMTSTATUS_SORT
    // SQLITE_STMTSTATUS_AUTOINDEX
    // SQLITE_STMTSTATUS_VM_STEP
    // SQLITE_STMTSTATUS_REPREPARE
    // SQLITE_STMTSTATUS_RUN
    // SQLITE_STMTSTATUS_MEMUSED
    pAux;
    argc;
    argv;
    pzErr;
    rc = unsafe {
        sqlite3_declare_vtab(
            db,
            (b"CREATE TABLE x(sql,ncol,ro,busy,nscan,nsort,naidx,nstep,reprep,run,mem)\0".as_ptr()
                as *mut i8) as *const i8,
        )
    };
    if rc == (0 as i32) {
        pNew = (unsafe { sqlite3_malloc64(32 as u64) }) as *mut stmt_vtab;
        unsafe {
            *ppVtab = pNew as *mut sqlite3_vtab;
        }
        if pNew == std::ptr::null_mut::<stmt_vtab>() {
            return 7 as i32;
        }
        unsafe { memset(pNew as *mut (), 0 as i32, 32 as u64) };
        unsafe {
            (*pNew).db = db;
        }
    }
    return rc;
}

/// This method is the destructor for stmt_cursor objects.
#[unsafe(link_section = ".text.slate_distinct.stmt.stmtDisconnect")]
extern "C-unwind" fn stmtDisconnect(mut pVtab: *mut sqlite3_vtab) -> i32 {
    unsafe { sqlite3_free(pVtab as *mut ()) };
    return 0 as i32;
}

/// Constructor for a new stmt_cursor object.
#[unsafe(link_section = ".text.slate_distinct.stmt.stmtOpen")]
extern "C-unwind" fn stmtOpen(
    mut p: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> i32 {
    let mut pCur: *mut stmt_cursor = unsafe { std::mem::zeroed() };
    pCur = (unsafe { sqlite3_malloc64(24 as u64) }) as *mut stmt_cursor;
    if pCur == std::ptr::null_mut::<stmt_cursor>() {
        return 7 as i32;
    }
    unsafe { memset(pCur as *mut (), 0 as i32, 24 as u64) };
    unsafe {
        (*pCur).db = unsafe { (*(p as *mut stmt_vtab)).db };
    }
    unsafe {
        *ppCursor = unsafe { std::ptr::addr_of_mut!((*pCur).base) };
    }
    return 0 as i32;
}

fn stmtCsrReset(mut pCur: *mut stmt_cursor) {
    let mut pRow: *mut StmtRow = std::ptr::null_mut::<StmtRow>();
    let mut pNext: *mut StmtRow = std::ptr::null_mut::<StmtRow>();
    pRow = unsafe { (*pCur).pRow };
    '__slate_break_191: while pRow != std::ptr::null_mut::<StmtRow>() {
        pNext = unsafe { (*pRow).pNext };
        unsafe { sqlite3_free(pRow as *mut ()) };
        pRow = pNext;
    }
    unsafe {
        (*pCur).pRow = std::ptr::null_mut::<StmtRow>();
    }
}

/// Destructor for a stmt_cursor.
#[unsafe(link_section = ".text.slate_distinct.stmt.stmtClose")]
extern "C-unwind" fn stmtClose(mut cur: *mut sqlite3_vtab_cursor) -> i32 {
    stmtCsrReset(cur as *mut stmt_cursor);
    unsafe { sqlite3_free(cur as *mut ()) };
    return 0 as i32;
}

/// Advance a stmt_cursor to its next row of output.
#[unsafe(link_section = ".text.slate_distinct.stmt.stmtNext")]
extern "C-unwind" fn stmtNext(mut cur: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCur: *mut stmt_cursor = cur as *mut stmt_cursor;
    let mut pNext: *mut StmtRow = unsafe { (*unsafe { (*pCur).pRow }).pNext };
    unsafe { sqlite3_free((unsafe { (*pCur).pRow }) as *mut ()) };
    unsafe {
        (*pCur).pRow = pNext;
    }
    return 0 as i32;
}

/// Return values of columns for the row at which the stmt_cursor
/// is currently pointing.
///
/// # Arguments
///
/// * `cur` - The cursor
/// * `ctx` - First argument to sqlite3_result_...()
/// * `i` - Which column to return
#[unsafe(link_section = ".text.slate_distinct.stmt.stmtColumn")]
extern "C-unwind" fn stmtColumn(
    mut cur: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: i32,
) -> i32 {
    let mut pCur: *mut stmt_cursor = cur as *mut stmt_cursor;
    let mut pRow: *mut StmtRow = unsafe { (*pCur).pRow };
    if i == (0 as i32) {
        unsafe {
            sqlite3_result_text(
                ctx,
                (unsafe { (*pRow).zSql }) as *const i8,
                -(1 as i32),
                unsafe {
                    std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        -(1 as i32) as usize,
                    )
                },
            )
        };
    } else {
        unsafe {
            sqlite3_result_int(ctx, unsafe {
                *unsafe { unsafe { (*pRow).aCol.as_mut_ptr() as *mut i32 }.offset(i as isize) }
            })
        };
    }
    return 0 as i32;
}

/// Return the rowid for the current row.  In this implementation, the
/// rowid is the same as the output value.
#[unsafe(link_section = ".text.slate_distinct.stmt.stmtRowid")]
extern "C-unwind" fn stmtRowid(mut cur: *mut sqlite3_vtab_cursor, mut pRowid: *mut i64) -> i32 {
    let mut pCur: *mut stmt_cursor = cur as *mut stmt_cursor;
    unsafe {
        *pRowid = unsafe { (*unsafe { (*pCur).pRow }).iRowid };
    }
    return 0 as i32;
}

/// Return TRUE if the cursor has been moved off of the last
/// row of output.
#[unsafe(link_section = ".text.slate_distinct.stmt.stmtEof")]
extern "C-unwind" fn stmtEof(mut cur: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCur: *mut stmt_cursor = cur as *mut stmt_cursor;
    return ((unsafe { (*pCur).pRow }) == std::ptr::null_mut::<StmtRow>()) as i32;
}

/// This method is called to "rewind" the stmt_cursor object back
/// to the first row of output.  This method is always called at least
/// once prior to any call to stmtColumn() or stmtRowid() or
/// stmtEof().
#[unsafe(link_section = ".text.slate_distinct.stmt.stmtFilter")]
extern "C-unwind" fn stmtFilter(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: i32,
    mut idxStr: *const i8,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) -> i32 {
    let mut pCur: *mut stmt_cursor = pVtabCursor as *mut stmt_cursor;
    let mut p: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
    let mut iRowid: i64 = (1 as i32) as i64;
    let mut ppRow: *mut *mut StmtRow = std::ptr::null_mut::<*mut StmtRow>();
    idxNum;
    idxStr;
    argc;
    argv;
    stmtCsrReset(pCur);
    ppRow = unsafe { std::ptr::addr_of_mut!((*pCur).pRow) };
    p = unsafe { sqlite3_next_stmt(unsafe { (*pCur).db }, std::ptr::null_mut::<sqlite3_stmt>()) };
    '__slate_break_192: while p != std::ptr::null_mut::<sqlite3_stmt>() {
        let mut zSql: *const i8 = unsafe { sqlite3_sql(p) };
        let mut nSql: i64 = (if zSql != std::ptr::null::<i8>() {
            unsafe { strlen(zSql) }.wrapping_add(((1 as i32) as i64) as u64)
        } else {
            ((0 as i32) as i64) as u64
        }) as i64;
        let mut pNew: *mut StmtRow =
            (unsafe { sqlite3_malloc64((72 as u64).wrapping_add(nSql as u64)) }) as *mut StmtRow;
        if pNew == std::ptr::null_mut::<StmtRow>() {
            return 7 as i32;
        }
        unsafe { memset(pNew as *mut (), 0 as i32, 72 as u64) };
        if zSql != std::ptr::null::<i8>() {
            unsafe {
                (*pNew).zSql = (unsafe { pNew.offset((1 as i32) as isize) }) as *mut i8;
            }
            unsafe {
                memcpy(
                    (unsafe { (*pNew).zSql }) as *mut (),
                    zSql as *const (),
                    nSql as u64,
                )
            };
        }
        unsafe {
            *unsafe {
                unsafe { (*pNew).aCol.as_mut_ptr() as *mut i32 }.offset((1 as i32) as isize)
            } = unsafe { sqlite3_column_count(p) };
        }
        unsafe {
            *unsafe {
                unsafe { (*pNew).aCol.as_mut_ptr() as *mut i32 }.offset((2 as i32) as isize)
            } = unsafe { sqlite3_stmt_readonly(p) };
        }
        unsafe {
            *unsafe {
                unsafe { (*pNew).aCol.as_mut_ptr() as *mut i32 }.offset((3 as i32) as isize)
            } = unsafe { sqlite3_stmt_busy(p) };
        }
        unsafe {
            *unsafe {
                unsafe { (*pNew).aCol.as_mut_ptr() as *mut i32 }.offset((4 as i32) as isize)
            } = unsafe { sqlite3_stmt_status(p, 1 as i32, 0 as i32) };
        }
        unsafe {
            *unsafe {
                unsafe { (*pNew).aCol.as_mut_ptr() as *mut i32 }.offset((5 as i32) as isize)
            } = unsafe { sqlite3_stmt_status(p, 2 as i32, 0 as i32) };
        }
        unsafe {
            *unsafe {
                unsafe { (*pNew).aCol.as_mut_ptr() as *mut i32 }.offset((6 as i32) as isize)
            } = unsafe { sqlite3_stmt_status(p, 3 as i32, 0 as i32) };
        }
        unsafe {
            *unsafe {
                unsafe { (*pNew).aCol.as_mut_ptr() as *mut i32 }.offset((7 as i32) as isize)
            } = unsafe { sqlite3_stmt_status(p, 4 as i32, 0 as i32) };
        }
        unsafe {
            *unsafe {
                unsafe { (*pNew).aCol.as_mut_ptr() as *mut i32 }.offset((8 as i32) as isize)
            } = unsafe { sqlite3_stmt_status(p, 5 as i32, 0 as i32) };
        }
        unsafe {
            *unsafe {
                unsafe { (*pNew).aCol.as_mut_ptr() as *mut i32 }.offset((9 as i32) as isize)
            } = unsafe { sqlite3_stmt_status(p, 6 as i32, 0 as i32) };
        }
        unsafe {
            *unsafe {
                unsafe { (*pNew).aCol.as_mut_ptr() as *mut i32 }.offset((10 as i32) as isize)
            } = unsafe { sqlite3_stmt_status(p, 99 as i32, 0 as i32) };
        }
        let __v194: i64 = iRowid;
        let __v195: i64 = __v194 + ((1 as i32) as i64);
        iRowid = __v195;
        unsafe {
            (*pNew).iRowid = __v194;
        }
        unsafe {
            *ppRow = pNew;
        }
        ppRow = unsafe { std::ptr::addr_of_mut!((*pNew).pNext) };
        p = unsafe { sqlite3_next_stmt(unsafe { (*pCur).db }, p) };
    }
    return 0 as i32;
}

/// SQLite will invoke this method one or more times while planning a query
/// that uses the stmt virtual table.  This routine needs to create
/// a query plan for each invocation and compute an estimated cost for that
/// plan.
#[unsafe(link_section = ".text.slate_distinct.stmt.stmtBestIndex")]
extern "C-unwind" fn stmtBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> i32 {
    tab;
    unsafe {
        (*pIdxInfo).estimatedCost = (500 as i32) as f64;
    }
    unsafe {
        (*pIdxInfo).estimatedRows = (500 as i32) as i64;
    }
    return 0 as i32;
}

/// This following structure defines all the methods for the
/// stmt virtual table.
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
static mut stmtModule: sqlite3_module = sqlite3_module {
    iVersion: 0 as i32,
    xCreate: None,
    xConnect: Some(stmtConnect),
    xBestIndex: Some(stmtBestIndex),
    xDisconnect: Some(stmtDisconnect),
    xDestroy: None,
    xOpen: Some(stmtOpen),
    xClose: Some(stmtClose),
    xFilter: Some(stmtFilter),
    xNext: Some(stmtNext),
    xEof: Some(stmtEof),
    xColumn: Some(stmtColumn),
    xRowid: Some(stmtRowid),
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

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.stmt.sqlite3StmtVtabInit")]
extern "C-unwind" fn sqlite3StmtVtabInit(mut db: *mut sqlite3) -> i32 {
    let mut rc: i32 = 0 as i32;
    rc = unsafe {
        sqlite3_create_module(
            db,
            (b"sqlite_stmt\0".as_ptr() as *mut i8) as *const i8,
            (unsafe { std::ptr::addr_of_mut!(stmtModule) }) as *const sqlite3_module,
            std::ptr::null_mut::<()>(),
        )
    };
    return rc;
}
