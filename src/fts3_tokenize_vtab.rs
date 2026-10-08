//! 2013 Apr 22
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
//! This file contains code for the "fts3tokenize" virtual table module.
//! An fts3tokenize virtual table is created as follows:
//!
//!   CREATE VIRTUAL TABLE <tbl> USING fts3tokenize(
//!       <tokenizer-name>, <arg-1>, ...
//!   );
//!
//! The table created has the following schema:
//!
//!   CREATE TABLE <tbl>(input, token, start, end, position)
//!
//! When queried, the query must include a WHERE clause of type:
//!
//!   input = <string>
//!
//! The virtual table module tokenizes this <string>, using the FTS3
//! tokenizer specified by the arguments to the CREATE VIRTUAL TABLE
//! statement and returns one row for each token in the result. With
//! fields set as follows:
//!
//!   input:   Always set to a copy of <string>
//!   token:   A token from the input.
//!   start:   Byte offset of the token within the input <string>.
//!   end:     Byte offset of the byte immediately following the end of the
//!            token within the input string.
//!   pos:     Token offset of token within input.
unsafe extern "C" {
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3_malloc(__v213: i32) -> *mut ();
    fn sqlite3_malloc64(__v214: u64) -> *mut ();
    fn sqlite3_free(__v215: *mut ());
    fn sqlite3_value_text(__v216: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_bytes(__v217: *mut sqlite3_value) -> i32;
    fn sqlite3_result_int(__v218: *mut sqlite3_context, __v219: i32);
    fn sqlite3_result_text(
        __v220: *mut sqlite3_context,
        __v221: *const i8,
        __v222: i32,
        __v223: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_create_module_v2(
        db: *mut sqlite3,
        zName: *const i8,
        p: *const sqlite3_module,
        pClientData: *mut (),
        xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_declare_vtab(__v229: *mut sqlite3, zSQL: *const i8) -> i32;
    fn sqlite3Fts3HashFind(__v231: *const Fts3Hash, pKey: *const (), nKey: i32) -> *mut ();
    fn sqlite3Fts3ErrMsg(__v234: *mut *mut i8, __v235: *const i8, ...);
    fn sqlite3Fts3Dequote(__v236: *mut i8);
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3 {}

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

/// Virtual table structure.
#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3tokTable {
    /// Base class used by SQLite core
    base: sqlite3_vtab,
    pMod: *const sqlite3_tokenizer_module,
    pTok: *mut sqlite3_tokenizer,
}

/// Virtual table cursor structure.
#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3tokCursor {
    /// Base class used by SQLite core
    base: sqlite3_vtab_cursor,
    /// Input string
    zInput: *mut i8,
    /// Cursor to iterate through zInput
    pCsr: *mut sqlite3_tokenizer_cursor,
    /// Current 'rowid' value
    iRowid: i32,
    /// Current 'token' value
    zToken: *const i8,
    /// Size of zToken in bytes
    nToken: i32,
    /// Current 'start' value
    iStart: i32,
    /// Current 'end' value
    iEnd: i32,
    /// Current 'pos' value
    iPos: i32,
}

/// Query FTS for the tokenizer implementation named zName.
fn fts3tokQueryTokenizer(
    mut pHash: *mut Fts3Hash,
    mut zName: *const i8,
    mut pp: *mut *const sqlite3_tokenizer_module,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut p: *mut sqlite3_tokenizer_module = unsafe { std::mem::zeroed() };
    let mut nName: i32 = ((unsafe { strlen(zName) }) as u32) as i32;
    p = (unsafe {
        sqlite3Fts3HashFind(
            pHash as *const Fts3Hash,
            zName as *const (),
            nName + (1 as i32),
        )
    }) as *mut sqlite3_tokenizer_module;
    if !(p != std::ptr::null_mut::<sqlite3_tokenizer_module>()) {
        unsafe {
            sqlite3Fts3ErrMsg(
                pzErr,
                (b"unknown tokenizer: %s\0".as_ptr() as *mut i8) as *const i8,
                zName,
            )
        };
        return 1 as i32;
    }
    unsafe {
        *pp = p as *const sqlite3_tokenizer_module;
    }
    return 0 as i32;
}

/// The second argument, argv[], is an array of pointers to nul-terminated
/// strings. This function makes a copy of the array and strings into a
/// single block of memory. It then dequotes any of the strings that appear
/// to be quoted.
///
/// If successful, output parameter *pazDequote is set to point at the
/// array of dequoted strings and SQLITE_OK is returned. The caller is
/// responsible for eventually calling sqlite3_free() to free the array
/// in this case. Or, if an error occurs, an SQLite error code is returned.
/// The final value of *pazDequote is undefined in this case.
///
/// # Arguments
///
/// * `argc` - Number of elements in argv[]
/// * `argv` - Input array
/// * `pazDequote` - Output array
fn fts3tokDequoteArray(
    mut argc: i32,
    mut argv: *const *const i8,
    mut pazDequote: *mut *mut *mut i8,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    if argc == (0 as i32) {
        unsafe {
            *pazDequote = std::ptr::null_mut::<*mut i8>();
        }
    } else {
        let mut i: i32 = 0 as i32;
        let mut nByte: i32 = 0 as i32;
        let mut azDequote: *mut *mut i8 = unsafe { std::mem::zeroed() };
        i = 0 as i32;
        '__slate_break_241: loop {
            if !(i < argc) {
                break;
            }
            let __v250: i32 = nByte;
            let __v251: i32 = __v250
                + ((unsafe { strlen(unsafe { *unsafe { argv.offset(i as isize) } }) }
                    .wrapping_add(((1 as i32) as i64) as u64) as u32) as i32);
            nByte = __v251;
            let __v248: i32 = i;
            let __v249: i32 = __v248 + (1 as i32);
            i = __v249;
        }
        let __v252: *mut *mut i8 = (unsafe {
            sqlite3_malloc64(
                (8 as u64)
                    .wrapping_mul((argc as i64) as u64)
                    .wrapping_add((nByte as i64) as u64),
            )
        }) as *mut *mut i8;
        azDequote = __v252;
        unsafe {
            *pazDequote = __v252;
        }
        if azDequote == std::ptr::null_mut::<*mut i8>() {
            rc = 7 as i32;
        } else {
            let mut pSpace: *mut i8 = (unsafe { azDequote.offset(argc as isize) }) as *mut i8;
            i = 0 as i32;
            '__slate_break_242: loop {
                if !(i < argc) {
                    break;
                }
                let mut n: i32 =
                    ((unsafe { strlen(unsafe { *unsafe { argv.offset(i as isize) } }) }) as u32)
                        as i32;
                unsafe {
                    *unsafe { azDequote.offset(i as isize) } = pSpace;
                }
                unsafe {
                    memcpy(
                        pSpace as *mut (),
                        (unsafe { *unsafe { argv.offset(i as isize) } }) as *const (),
                        ((n + (1 as i32)) as i64) as u64,
                    )
                };
                unsafe { sqlite3Fts3Dequote(pSpace) };
                let __v255: *mut i8 = pSpace;
                let __v256: *mut i8 = unsafe { __v255.offset((n + (1 as i32)) as isize) };
                pSpace = __v256;
                let __v253: i32 = i;
                let __v254: i32 = __v253 + (1 as i32);
                i = __v254;
            }
        }
    }
    return rc;
}

// Schema of the tokenizer table.
/// This function does all the work for both the xConnect and xCreate methods.
/// These tables have no persistent representation of their own, so xConnect
/// and xCreate are identical operations.
///
///   argv[0]: module name
///   argv[1]: database name
///   argv[2]: table name
///   argv[3]: first argument (tokenizer name)
///
/// # Arguments
///
/// * `db` - Database connection
/// * `pHash` - Hash table of tokenizers
/// * `argc` - Number of elements in argv array
/// * `argv` - xCreate/xConnect argument array
/// * `ppVtab` - OUT: New sqlite3_vtab object
/// * `pzErr` - OUT: sqlite3_malloc'd error message
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenize_vtab.fts3tokConnectMethod")]
extern "C-unwind" fn fts3tokConnectMethod(
    mut db: *mut sqlite3,
    mut pHash: *mut (),
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut pTab: *mut Fts3tokTable = std::ptr::null_mut::<Fts3tokTable>();
    let mut pMod: *const sqlite3_tokenizer_module = std::ptr::null::<sqlite3_tokenizer_module>();
    let mut pTok: *mut sqlite3_tokenizer = std::ptr::null_mut::<sqlite3_tokenizer>();
    let mut rc: i32 = 0 as i32;
    let mut azDequote: *mut *mut i8 = std::ptr::null_mut::<*mut i8>();
    let mut nDequote: i32 = 0 as i32;
    rc = unsafe {
        sqlite3_declare_vtab(
            db,
            (b"CREATE TABLE x(input, token, start, end, position)\0".as_ptr() as *mut i8)
                as *const i8,
        )
    };
    if rc != (0 as i32) {
        return rc;
    }
    nDequote = argc - (3 as i32);
    rc = fts3tokDequoteArray(
        nDequote,
        unsafe { argv.offset((3 as i32) as isize) },
        std::ptr::addr_of_mut!(azDequote),
    );
    if rc == (0 as i32) {
        let mut zModule: *const i8 = unsafe { std::mem::zeroed() };
        if nDequote < (1 as i32) {
            zModule = (b"simple\0".as_ptr() as *mut i8) as *const i8;
        } else {
            zModule = (unsafe { *unsafe { azDequote.offset((0 as i32) as isize) } }) as *const i8;
        }
        rc = fts3tokQueryTokenizer(
            pHash as *mut Fts3Hash,
            zModule,
            std::ptr::addr_of_mut!(pMod),
            pzErr,
        );
    }
    0 as i32;
    if rc == (0 as i32) {
        let mut azArg: *const *const i8 = std::ptr::null::<*const i8>();
        if nDequote > (1 as i32) {
            azArg = (unsafe { azDequote.offset((1 as i32) as isize) }) as *const *const i8;
        }
        rc = unsafe {
            unsafe { (*pMod).xCreate }.unwrap()(
                if nDequote > (1 as i32) {
                    nDequote - (1 as i32)
                } else {
                    0 as i32
                },
                azArg,
                std::ptr::addr_of_mut!(pTok),
            )
        };
    }
    if rc == (0 as i32) {
        pTab = (unsafe { sqlite3_malloc(((40 as u64) as u32) as i32) }) as *mut Fts3tokTable;
        if pTab == std::ptr::null_mut::<Fts3tokTable>() {
            rc = 7 as i32;
        }
    }
    if rc == (0 as i32) {
        unsafe { memset(pTab as *mut (), 0 as i32, 40 as u64) };
        unsafe {
            (*pTab).pMod = pMod;
        }
        unsafe {
            (*pTab).pTok = pTok;
        }
        unsafe {
            *ppVtab = unsafe { std::ptr::addr_of_mut!((*pTab).base) };
        }
    } else {
        if pTok != std::ptr::null_mut::<sqlite3_tokenizer>() {
            unsafe { unsafe { (*pMod).xDestroy }.unwrap()(pTok) };
        }
    }
    unsafe { sqlite3_free(azDequote as *mut ()) };
    return rc;
}

/// This function does the work for both the xDisconnect and xDestroy methods.
/// These tables have no persistent representation of their own, so xDisconnect
/// and xDestroy are identical operations.
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenize_vtab.fts3tokDisconnectMethod")]
extern "C-unwind" fn fts3tokDisconnectMethod(mut pVtab: *mut sqlite3_vtab) -> i32 {
    let mut pTab: *mut Fts3tokTable = pVtab as *mut Fts3tokTable;
    unsafe { unsafe { (*unsafe { (*pTab).pMod }).xDestroy }.unwrap()(unsafe { (*pTab).pTok }) };
    unsafe { sqlite3_free(pTab as *mut ()) };
    return 0 as i32;
}

/// xBestIndex - Analyze a WHERE and ORDER BY clause.
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenize_vtab.fts3tokBestIndexMethod")]
extern "C-unwind" fn fts3tokBestIndexMethod(
    mut pVTab: *mut sqlite3_vtab,
    mut pInfo: *mut sqlite3_index_info,
) -> i32 {
    let mut i: i32 = 0 as i32;
    pVTab;
    i = 0 as i32;
    '__slate_break_245: loop {
        if !(i < unsafe { (*pInfo).nConstraint }) {
            break;
        }
        if (unsafe { (*unsafe { unsafe { (*pInfo).aConstraint }.offset(i as isize) }).usable })
            != (0 as u8)
            && (unsafe { (*unsafe { unsafe { (*pInfo).aConstraint }.offset(i as isize) }).iColumn })
                == (0 as i32)
            && (((unsafe { (*unsafe { unsafe { (*pInfo).aConstraint }.offset(i as isize) }).op })
                as u32) as i32)
                == (2 as i32)
        {
            unsafe {
                (*pInfo).idxNum = 1 as i32;
            }
            unsafe {
                (*unsafe { unsafe { (*pInfo).aConstraintUsage }.offset(i as isize) }).argvIndex =
                    1 as i32;
            }
            unsafe {
                (*unsafe { unsafe { (*pInfo).aConstraintUsage }.offset(i as isize) }).omit =
                    ((1 as i32) as i8) as u8;
            }
            unsafe {
                (*pInfo).estimatedCost = (1 as i32) as f64;
            }
            return 0 as i32;
        }
        let __v257: i32 = i;
        let __v258: i32 = __v257 + (1 as i32);
        i = __v258;
    }
    unsafe {
        (*pInfo).idxNum = 0 as i32;
    }
    0 as i32;
    return 0 as i32;
}

/// xOpen - Open a cursor.
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenize_vtab.fts3tokOpenMethod")]
extern "C-unwind" fn fts3tokOpenMethod(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCsr: *mut *mut sqlite3_vtab_cursor,
) -> i32 {
    let mut pCsr: *mut Fts3tokCursor = unsafe { std::mem::zeroed() };
    pVTab;
    pCsr = (unsafe { sqlite3_malloc(((56 as u64) as u32) as i32) }) as *mut Fts3tokCursor;
    if pCsr == std::ptr::null_mut::<Fts3tokCursor>() {
        return 7 as i32;
    }
    unsafe { memset(pCsr as *mut (), 0 as i32, 56 as u64) };
    unsafe {
        *ppCsr = pCsr as *mut sqlite3_vtab_cursor;
    }
    return 0 as i32;
}

/// Reset the tokenizer cursor passed as the only argument. As if it had
/// just been returned by fts3tokOpenMethod().
fn fts3tokResetCursor(mut pCsr: *mut Fts3tokCursor) {
    if (unsafe { (*pCsr).pCsr }) != std::ptr::null_mut::<sqlite3_tokenizer_cursor>() {
        let mut pTab: *mut Fts3tokTable = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3tokTable;
        unsafe { unsafe { (*unsafe { (*pTab).pMod }).xClose }.unwrap()(unsafe { (*pCsr).pCsr }) };
        unsafe {
            (*pCsr).pCsr = std::ptr::null_mut::<sqlite3_tokenizer_cursor>();
        }
    }
    unsafe { sqlite3_free((unsafe { (*pCsr).zInput }) as *mut ()) };
    unsafe {
        (*pCsr).zInput = std::ptr::null_mut::<i8>();
    }
    unsafe {
        (*pCsr).zToken = std::ptr::null::<i8>();
    }
    unsafe {
        (*pCsr).nToken = 0 as i32;
    }
    unsafe {
        (*pCsr).iStart = 0 as i32;
    }
    unsafe {
        (*pCsr).iEnd = 0 as i32;
    }
    unsafe {
        (*pCsr).iPos = 0 as i32;
    }
    unsafe {
        (*pCsr).iRowid = 0 as i32;
    }
}

/// xClose - Close a cursor.
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenize_vtab.fts3tokCloseMethod")]
extern "C-unwind" fn fts3tokCloseMethod(mut pCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    fts3tokResetCursor(pCsr);
    unsafe { sqlite3_free(pCsr as *mut ()) };
    return 0 as i32;
}

/// xNext - Advance the cursor to the next row, if any.
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenize_vtab.fts3tokNextMethod")]
extern "C-unwind" fn fts3tokNextMethod(mut pCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    let mut pTab: *mut Fts3tokTable = (unsafe { (*pCursor).pVtab }) as *mut Fts3tokTable;
    let mut rc: i32 = 0 as i32; // Return code
    let __v259: *mut Fts3tokCursor = pCsr;
    let __v260: i32 = unsafe { (*__v259).iRowid };
    let __v261: i32 = __v260 + (1 as i32);
    unsafe {
        (*__v259).iRowid = __v261;
    }
    rc = unsafe {
        unsafe { (*unsafe { (*pTab).pMod }).xNext }.unwrap()(
            unsafe { (*pCsr).pCsr },
            unsafe { std::ptr::addr_of_mut!((*pCsr).zToken) },
            unsafe { std::ptr::addr_of_mut!((*pCsr).nToken) },
            unsafe { std::ptr::addr_of_mut!((*pCsr).iStart) },
            unsafe { std::ptr::addr_of_mut!((*pCsr).iEnd) },
            unsafe { std::ptr::addr_of_mut!((*pCsr).iPos) },
        )
    };
    if rc != (0 as i32) {
        fts3tokResetCursor(pCsr);
        if rc == (101 as i32) {
            rc = 0 as i32;
        }
    }
    return rc;
}

/// xFilter - Initialize a cursor to point at the start of its data.
///
/// # Arguments
///
/// * `pCursor` - The cursor used for this query
/// * `idxNum` - Strategy index
/// * `idxStr` - Unused
/// * `nVal` - Number of elements in apVal
/// * `apVal` - Arguments for the indexing scheme
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenize_vtab.fts3tokFilterMethod")]
extern "C-unwind" fn fts3tokFilterMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: i32,
    mut idxStr: *const i8,
    mut nVal: i32,
    mut apVal: *mut *mut sqlite3_value,
) -> i32 {
    let mut rc: i32 = 1 as i32;
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    let mut pTab: *mut Fts3tokTable = (unsafe { (*pCursor).pVtab }) as *mut Fts3tokTable;
    idxStr;
    nVal;
    fts3tokResetCursor(pCsr);
    if idxNum == (1 as i32) {
        let mut zByte: *const i8 = (unsafe {
            sqlite3_value_text(unsafe { *unsafe { apVal.offset((0 as i32) as isize) } })
        }) as *const i8;
        let mut nByte: i64 = (unsafe {
            sqlite3_value_bytes(unsafe { *unsafe { apVal.offset((0 as i32) as isize) } })
        }) as i64;
        unsafe {
            (*pCsr).zInput =
                (unsafe { sqlite3_malloc64((nByte + ((1 as i32) as i64)) as u64) }) as *mut i8;
        }
        if (unsafe { (*pCsr).zInput }) == std::ptr::null_mut::<i8>() {
            rc = 7 as i32;
        } else {
            if nByte > ((0 as i32) as i64) {
                unsafe {
                    memcpy(
                        (unsafe { (*pCsr).zInput }) as *mut (),
                        zByte as *const (),
                        nByte as u64,
                    )
                };
            }
            unsafe {
                *unsafe { unsafe { (*pCsr).zInput }.offset(nByte as isize) } = (0 as i32) as i8;
            }
            rc = unsafe {
                unsafe { (*unsafe { (*pTab).pMod }).xOpen }.unwrap()(
                    unsafe { (*pTab).pTok },
                    (unsafe { (*pCsr).zInput }) as *const i8,
                    nByte as i32,
                    unsafe { std::ptr::addr_of_mut!((*pCsr).pCsr) },
                )
            };
            if rc == (0 as i32) {
                unsafe {
                    (*unsafe { (*pCsr).pCsr }).pTokenizer = unsafe { (*pTab).pTok };
                }
            }
        }
    }
    if rc != (0 as i32) {
        return rc;
    }
    return fts3tokNextMethod(pCursor);
}

/// xEof - Return true if the cursor is at EOF, or false otherwise.
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenize_vtab.fts3tokEofMethod")]
extern "C-unwind" fn fts3tokEofMethod(mut pCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    return ((unsafe { (*pCsr).zToken }) == std::ptr::null::<i8>()) as i32;
}

/// xColumn - Return a column value.
///
/// # Arguments
///
/// * `pCursor` - Cursor to retrieve value from
/// * `pCtx` - Context for sqlite3_result_xxx() calls
/// * `iCol` - Index of column to read value from
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenize_vtab.fts3tokColumnMethod")]
extern "C-unwind" fn fts3tokColumnMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pCtx: *mut sqlite3_context,
    mut iCol: i32,
) -> i32 {
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    // CREATE TABLE x(input, token, start, end, position)
    '__slate_break_246: {
        match iCol {
            0 => {
                unsafe {
                    sqlite3_result_text(
                        pCtx,
                        (unsafe { (*pCsr).zInput }) as *const i8,
                        -(1 as i32),
                        unsafe {
                            std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                                -(1 as i32) as usize,
                            )
                        },
                    )
                };
            }
            1 => {
                unsafe {
                    sqlite3_result_text(
                        pCtx,
                        unsafe { (*pCsr).zToken },
                        unsafe { (*pCsr).nToken },
                        unsafe {
                            std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                                -(1 as i32) as usize,
                            )
                        },
                    )
                };
            }
            2 => {
                unsafe { sqlite3_result_int(pCtx, unsafe { (*pCsr).iStart }) };
            }
            3 => {
                unsafe { sqlite3_result_int(pCtx, unsafe { (*pCsr).iEnd }) };
            }
            _ => {
                0 as i32;
                unsafe { sqlite3_result_int(pCtx, unsafe { (*pCsr).iPos }) };
            }
        }
    }
    return 0 as i32;
}

/// xRowid - Return the current rowid for the cursor.
///
/// # Arguments
///
/// * `pCursor` - Cursor to retrieve value from
/// * `pRowid` - OUT: Rowid value
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenize_vtab.fts3tokRowidMethod")]
extern "C-unwind" fn fts3tokRowidMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut i64,
) -> i32 {
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    unsafe {
        *pRowid = (unsafe { (*pCsr).iRowid }) as i64;
    }
    return 0 as i32;
}

/// Register the fts3tok module with database connection db. Return SQLITE_OK
/// if successful or an error code if sqlite3_create_module() fails.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3InitTok(
    mut db: *mut sqlite3,
    mut pHash: *mut Fts3Hash,
    mut xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    // iVersion
    // xCreate
    // xConnect
    // xBestIndex
    // xDisconnect
    // xDestroy
    // xOpen
    // xClose
    // xFilter
    // xNext
    // xEof
    // xColumn
    // xRowid
    // xUpdate
    // xBegin
    // xSync
    // xCommit
    // xRollback
    // xFindFunction
    // xRename
    // xSavepoint
    // xRelease
    // xRollbackTo
    // xShadowName
    // xIntegrity
    let mut rc: i32 = 0 as i32; // Return code
    rc = unsafe {
        sqlite3_create_module_v2(
            db,
            (b"fts3tokenize\0".as_ptr() as *mut i8) as *const i8,
            unsafe { std::ptr::addr_of!(fts3tok_module) },
            pHash as *mut (),
            xDestroy,
        )
    };
    return rc;
}

static mut fts3tok_module: sqlite3_module = sqlite3_module {
    iVersion: 0 as i32,
    xCreate: Some(fts3tokConnectMethod),
    xConnect: Some(fts3tokConnectMethod),
    xBestIndex: Some(fts3tokBestIndexMethod),
    xDisconnect: Some(fts3tokDisconnectMethod),
    xDestroy: Some(fts3tokDisconnectMethod),
    xOpen: Some(fts3tokOpenMethod),
    xClose: Some(fts3tokCloseMethod),
    xFilter: Some(fts3tokFilterMethod),
    xNext: Some(fts3tokNextMethod),
    xEof: Some(fts3tokEofMethod),
    xColumn: Some(fts3tokColumnMethod),
    xRowid: Some(fts3tokRowidMethod),
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
