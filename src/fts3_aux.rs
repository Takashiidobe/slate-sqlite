//! 2011 Jan 27
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
unsafe extern "C" {
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3_mprintf(__v257: *const i8, ...) -> *mut i8;
    fn sqlite3_malloc(__v258: i32) -> *mut ();
    fn sqlite3_malloc64(__v259: u64) -> *mut ();
    fn sqlite3_realloc64(__v260: *mut (), __v261: u64) -> *mut ();
    fn sqlite3_free(__v262: *mut ());
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_value_int(__v264: *mut sqlite3_value) -> i32;
    fn sqlite3_value_text(__v265: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_type(__v266: *mut sqlite3_value) -> i32;
    fn sqlite3_result_int(__v267: *mut sqlite3_context, __v268: i32);
    fn sqlite3_result_int64(__v269: *mut sqlite3_context, __v270: i64);
    fn sqlite3_result_text(
        __v271: *mut sqlite3_context,
        __v272: *const i8,
        __v273: i32,
        __v274: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_create_module(
        db: *mut sqlite3,
        zName: *const i8,
        p: *const sqlite3_module,
        pClientData: *mut (),
    ) -> i32;
    fn sqlite3_declare_vtab(__v279: *mut sqlite3, zSQL: *const i8) -> i32;
    fn sqlite3_stricmp(__v281: *const i8, __v282: *const i8) -> i32;
    fn sqlite3_strnicmp(__v283: *const i8, __v284: *const i8, __v285: i32) -> i32;
    fn sqlite3_vtab_collation(__v286: *mut sqlite3_index_info, __v287: i32) -> *const i8;
    fn sqlite3Fts3SegmentsClose(__v288: *mut Fts3Table);
    fn sqlite3Fts3SegReaderStart(
        __v289: *mut Fts3Table,
        __v290: *mut Fts3MultiSegReader,
        __v291: *mut Fts3SegFilter,
    ) -> i32;
    fn sqlite3Fts3SegReaderStep(__v292: *mut Fts3Table, __v293: *mut Fts3MultiSegReader) -> i32;
    fn sqlite3Fts3SegReaderFinish(__v294: *mut Fts3MultiSegReader);
    fn sqlite3Fts3SegReaderCursor(
        __v295: *mut Fts3Table,
        __v296: i32,
        __v297: i32,
        __v298: i32,
        __v299: *const i8,
        __v300: i32,
        __v301: i32,
        __v302: i32,
        __v303: *mut Fts3MultiSegReader,
    ) -> i32;
    fn sqlite3Fts3ErrMsg(__v304: *mut *mut i8, __v305: *const i8, ...);
    fn sqlite3Fts3GetVarint(__v306: *const i8, __v307: *mut i64) -> i32;
    fn sqlite3Fts3Dequote(__v308: *mut i8);
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
struct Fts3SegFilter {
    zTerm: *const i8,
    nTerm: i32,
    iCol: i32,
    flags: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3SegReader {}

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

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Index {
    nPrefix: i32,
    hPending: Fts3Hash,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3auxTable {
    /// Base class used by SQLite core
    base: sqlite3_vtab,
    pFts3Tab: *mut Fts3Table,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3auxCursor {
    /// Base class used by SQLite core
    base: sqlite3_vtab_cursor,
    /// Must be right after "base"
    csr: Fts3MultiSegReader,
    filter: Fts3SegFilter,
    zStop: *mut i8,
    /// Byte-length of string zStop
    nStop: i32,
    /// Language id to query
    iLangid: i32,
    /// True if cursor is at EOF
    isEof: i32,
    /// Current rowid
    iRowid: i64,
    /// Current value of 'col' column
    iCol: i32,
    /// Size of aStat[] array
    nStat: i32,
    aStat: *mut Fts3auxColstats,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3auxColstats {
    /// 'documents' values for current csr row
    nDoc: i64,
    /// 'occurrences' values for current csr row
    nOcc: i64,
}

// Schema of the terms table.
/// This function does all the work for both the xConnect and xCreate methods.
/// These tables have no persistent representation of their own, so xConnect
/// and xCreate are identical operations.
///
/// # Arguments
///
/// * `db` - Database connection
/// * `pUnused` - Unused
/// * `argc` - Number of elements in argv array
/// * `argv` - xCreate/xConnect argument array
/// * `ppVtab` - OUT: New sqlite3_vtab object
/// * `pzErr` - OUT: sqlite3_malloc'd error message
#[unsafe(link_section = ".text.slate_distinct.fts3_aux.fts3auxConnectMethod")]
extern "C-unwind" fn fts3auxConnectMethod(
    mut db: *mut sqlite3,
    mut pUnused: *mut (),
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut __slate_storage_324: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_324: *mut bool = std::ptr::addr_of_mut!(__slate_storage_324) as *mut bool; // Virtual table object to return
    let mut __slate_storage_170: std::mem::MaybeUninit<*mut Fts3auxTable> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_170: *mut *mut Fts3auxTable =
        std::ptr::addr_of_mut!(__slate_storage_170) as *mut *mut Fts3auxTable; // value returned by declare_vtab()
    let mut __slate_storage_169: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_169: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_169) as *mut i32; // Bytes of space to allocate here
    let mut __slate_storage_168: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_168: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_168) as *mut i64; // Result of strlen(zFts3)
    let mut __slate_storage_167: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_167: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_167) as *mut i32; // Result of strlen(zDb)
    let mut __slate_storage_166: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_166: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_166) as *mut i32; // Name of fts3 table
    let mut __slate_storage_165: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_165: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_165) as *mut *const i8; // Name of database (e.g. "main")
    let mut __slate_storage_164: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_164: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_164) as *mut *const i8;
    unsafe {
        '__join_0: {
            pUnused;
            // The user should invoke this in one of two forms:
            //
            // CREATE VIRTUAL TABLE xxx USING fts4aux(fts4-table);
            // CREATE VIRTUAL TABLE xxx USING fts4aux(fts4-table-db, fts4-table);
            if argc != (4 as i32) && argc != (5 as i32) {
            } else {
                *__slate_slot_164 = unsafe { *unsafe { argv.offset((1 as i32) as isize) } };
                *__slate_slot_166 = ((unsafe { strlen(*__slate_slot_164) }) as u32) as i32;
                if argc == (5 as i32) {
                    '__join_7: {
                        if *__slate_slot_166 == (4 as i32) {
                            *__slate_slot_324 = (0 as i32)
                                == unsafe {
                                    sqlite3_strnicmp(
                                        (b"temp\0".as_ptr() as *mut i8) as *const i8,
                                        *__slate_slot_164,
                                        4 as i32,
                                    )
                                };
                        } else {
                            *__slate_slot_324 = false as bool;
                        }
                    }
                    if *__slate_slot_324 {
                        *__slate_slot_164 = unsafe { *unsafe { argv.offset((3 as i32) as isize) } };
                        *__slate_slot_166 = ((unsafe { strlen(*__slate_slot_164) }) as u32) as i32;
                        *__slate_slot_165 = unsafe { *unsafe { argv.offset((4 as i32) as isize) } };
                    } else {
                        break '__join_0;
                    }
                } else {
                    *__slate_slot_165 = unsafe { *unsafe { argv.offset((3 as i32) as isize) } };
                }
                *__slate_slot_167 = ((unsafe { strlen(*__slate_slot_165) }) as u32) as i32;
                *__slate_slot_169 = unsafe {
                    sqlite3_declare_vtab(
                        db,
                        (b"CREATE TABLE x(term, col, documents, occurrences, languageid HIDDEN)\0"
                            .as_ptr() as *mut i8) as *const i8,
                    )
                };
                if *__slate_slot_169 != (0 as i32) {
                    return *__slate_slot_169;
                } else {
                    *__slate_slot_168 = (32 as u64)
                        .wrapping_add(528 as u64)
                        .wrapping_add((*__slate_slot_166 as i64) as u64)
                        .wrapping_add((*__slate_slot_167 as i64) as u64)
                        .wrapping_add(((2 as i32) as i64) as u64)
                        as i64;
                    *__slate_slot_170 = (unsafe { sqlite3_malloc64(*__slate_slot_168 as u64) })
                        as *mut Fts3auxTable;
                    if !(*__slate_slot_170 != std::ptr::null_mut::<Fts3auxTable>()) {
                        return 7 as i32;
                    } else {
                        unsafe {
                            memset(
                                *__slate_slot_170 as *mut (),
                                0 as i32,
                                *__slate_slot_168 as u64,
                            )
                        };
                        unsafe {
                            (*(*__slate_slot_170)).pFts3Tab =
                                (unsafe { (*__slate_slot_170).offset((1 as i32) as isize) })
                                    as *mut Fts3Table;
                        }
                        unsafe {
                            (*unsafe { (*(*__slate_slot_170)).pFts3Tab }).zDb =
                                ((unsafe {
                                    unsafe { (*(*__slate_slot_170)).pFts3Tab }
                                        .offset((1 as i32) as isize)
                                }) as *mut i8) as *const i8;
                        }
                        unsafe {
                            (*unsafe { (*(*__slate_slot_170)).pFts3Tab }).zName = unsafe {
                                unsafe { (*unsafe { (*(*__slate_slot_170)).pFts3Tab }).zDb }
                                    .offset((*__slate_slot_166 + (1 as i32)) as isize)
                            };
                        }
                        unsafe {
                            (*unsafe { (*(*__slate_slot_170)).pFts3Tab }).db = db;
                        }
                        unsafe {
                            (*unsafe { (*(*__slate_slot_170)).pFts3Tab }).nIndex = 1 as i32;
                        }
                        unsafe {
                            memcpy(
                                ((unsafe { (*unsafe { (*(*__slate_slot_170)).pFts3Tab }).zDb })
                                    as *mut i8) as *mut (),
                                *__slate_slot_164 as *const (),
                                (*__slate_slot_166 as i64) as u64,
                            )
                        };
                        unsafe {
                            memcpy(
                                ((unsafe { (*unsafe { (*(*__slate_slot_170)).pFts3Tab }).zName })
                                    as *mut i8) as *mut (),
                                *__slate_slot_165 as *const (),
                                (*__slate_slot_167 as i64) as u64,
                            )
                        };
                        unsafe {
                            sqlite3Fts3Dequote(
                                (unsafe { (*unsafe { (*(*__slate_slot_170)).pFts3Tab }).zName })
                                    as *mut i8,
                            )
                        };
                        unsafe {
                            *ppVtab = *__slate_slot_170 as *mut sqlite3_vtab;
                        }
                        return 0 as i32;
                    }
                }
            }
        }
        unsafe {
            sqlite3Fts3ErrMsg(
                pzErr,
                (b"invalid arguments to fts4aux constructor\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        return 1 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

/// This function does the work for both the xDisconnect and xDestroy methods.
/// These tables have no persistent representation of their own, so xDisconnect
/// and xDestroy are identical operations.
#[unsafe(link_section = ".text.slate_distinct.fts3_aux.fts3auxDisconnectMethod")]
extern "C-unwind" fn fts3auxDisconnectMethod(mut pVtab: *mut sqlite3_vtab) -> i32 {
    let mut p: *mut Fts3auxTable = pVtab as *mut Fts3auxTable;
    let mut pFts3: *mut Fts3Table = unsafe { (*p).pFts3Tab };
    let mut i: i32 = 0 as i32;
    // Free any prepared statements held
    i = 0 as i32;
    '__slate_break_313: loop {
        if !(i < ((((320 as u64) / (8 as u64)) as u32) as i32)) {
            break;
        }
        unsafe {
            sqlite3_finalize(unsafe {
                *unsafe {
                    unsafe { (*pFts3).aStmt.as_mut_ptr() as *mut *mut sqlite3_stmt }
                        .offset(i as isize)
                }
            })
        };
        let __v325: i32 = i;
        let __v326: i32 = __v325 + (1 as i32);
        i = __v326;
    }
    unsafe { sqlite3_free((unsafe { (*pFts3).zSegmentsTbl }) as *mut ()) };
    unsafe { sqlite3_free(p as *mut ()) };
    return 0 as i32;
}

/// Return true if constraint iCons of pInfo uses the "binary" collation
/// sequence. Or false it it uses anything else.
fn fts3auxIsBinary(mut pInfo: *mut sqlite3_index_info, mut iCons: i32) -> i32 {
    return ((0 as i32)
        == unsafe {
            sqlite3_stricmp((b"binary\0".as_ptr() as *mut i8) as *const i8, unsafe {
                sqlite3_vtab_collation(pInfo, iCons)
            })
        }) as i32;
}

/// xBestIndex - Analyze a WHERE and ORDER BY clause.
#[unsafe(link_section = ".text.slate_distinct.fts3_aux.fts3auxBestIndexMethod")]
extern "C-unwind" fn fts3auxBestIndexMethod(
    mut pVTab: *mut sqlite3_vtab,
    mut pInfo: *mut sqlite3_index_info,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut iEq: i32 = -(1 as i32);
    let mut iGe: i32 = -(1 as i32);
    let mut iLe: i32 = -(1 as i32);
    let mut iLangid: i32 = -(1 as i32);
    let mut iNext: i32 = 1 as i32; // Next free argvIndex value
    pVTab;
    // This vtab delivers always results in "ORDER BY term ASC" order.
    if (unsafe { (*pInfo).nOrderBy }) == (1 as i32)
        && (unsafe {
            (*unsafe { unsafe { (*pInfo).aOrderBy }.offset((0 as i32) as isize) }).iColumn
        }) == (0 as i32)
        && (((unsafe {
            (*unsafe { unsafe { (*pInfo).aOrderBy }.offset((0 as i32) as isize) }).desc
        }) as u32) as i32)
            == (0 as i32)
    {
        unsafe {
            (*pInfo).orderByConsumed = 1 as i32;
        }
    }
    // Search for equality and range constraints on the "term" column.
    // And equality constraints on the hidden "languageid" column.
    i = 0 as i32;
    '__slate_break_315: loop {
        if !(i < unsafe { (*pInfo).nConstraint }) {
            break;
        }
        let __v329: bool;
        if (unsafe { (*unsafe { unsafe { (*pInfo).aConstraint }.offset(i as isize) }).usable })
            != (0 as u8)
        {
            __v329 = fts3auxIsBinary(pInfo, i) != (0 as i32);
        } else {
            __v329 = false as bool;
        }
        if __v329 {
            let mut op: i32 =
                ((unsafe { (*unsafe { unsafe { (*pInfo).aConstraint }.offset(i as isize) }).op })
                    as u32) as i32;
            let mut iCol: i32 =
                unsafe { (*unsafe { unsafe { (*pInfo).aConstraint }.offset(i as isize) }).iColumn };
            if iCol == (0 as i32) {
                if op == (2 as i32) {
                    iEq = i;
                }
                if op == (16 as i32) {
                    iLe = i;
                }
                if op == (8 as i32) {
                    iLe = i;
                }
                if op == (4 as i32) {
                    iGe = i;
                }
                if op == (32 as i32) {
                    iGe = i;
                }
            }
            if iCol == (4 as i32) {
                if op == (2 as i32) {
                    iLangid = i;
                }
            }
        }
        let __v327: i32 = i;
        let __v328: i32 = __v327 + (1 as i32);
        i = __v328;
    }
    if iEq >= (0 as i32) {
        unsafe {
            (*pInfo).idxNum = 1 as i32;
        }
        let __v330: i32 = iNext;
        let __v331: i32 = __v330 + (1 as i32);
        iNext = __v331;
        unsafe {
            (*unsafe { unsafe { (*pInfo).aConstraintUsage }.offset(iEq as isize) }).argvIndex =
                __v330;
        }
        unsafe {
            (*pInfo).estimatedCost = (5 as i32) as f64;
        }
    } else {
        unsafe {
            (*pInfo).idxNum = 0 as i32;
        }
        unsafe {
            (*pInfo).estimatedCost = (20000 as i32) as f64;
        }
        if iGe >= (0 as i32) {
            let __v332: *mut sqlite3_index_info = pInfo;
            let __v333: i32 = unsafe { (*__v332).idxNum };
            let __v334: i32 = __v333 + (2 as i32);
            unsafe {
                (*__v332).idxNum = __v334;
            }
            let __v335: i32 = iNext;
            let __v336: i32 = __v335 + (1 as i32);
            iNext = __v336;
            unsafe {
                (*unsafe { unsafe { (*pInfo).aConstraintUsage }.offset(iGe as isize) }).argvIndex =
                    __v335;
            }
            let __v337: *mut sqlite3_index_info = pInfo;
            let __v338: f64 = unsafe { (*__v337).estimatedCost };
            let __v339: f64 = __v338 / ((2 as i32) as f64);
            unsafe {
                (*__v337).estimatedCost = __v339;
            }
        }
        if iLe >= (0 as i32) {
            let __v340: *mut sqlite3_index_info = pInfo;
            let __v341: i32 = unsafe { (*__v340).idxNum };
            let __v342: i32 = __v341 + (4 as i32);
            unsafe {
                (*__v340).idxNum = __v342;
            }
            let __v343: i32 = iNext;
            let __v344: i32 = __v343 + (1 as i32);
            iNext = __v344;
            unsafe {
                (*unsafe { unsafe { (*pInfo).aConstraintUsage }.offset(iLe as isize) }).argvIndex =
                    __v343;
            }
            let __v345: *mut sqlite3_index_info = pInfo;
            let __v346: f64 = unsafe { (*__v345).estimatedCost };
            let __v347: f64 = __v346 / ((2 as i32) as f64);
            unsafe {
                (*__v345).estimatedCost = __v347;
            }
        }
    }
    if iLangid >= (0 as i32) {
        let __v348: i32 = iNext;
        let __v349: i32 = __v348 + (1 as i32);
        iNext = __v349;
        unsafe {
            (*unsafe { unsafe { (*pInfo).aConstraintUsage }.offset(iLangid as isize) }).argvIndex =
                __v348;
        }
        let __v350: *mut sqlite3_index_info = pInfo;
        let __v351: f64 = unsafe { (*__v350).estimatedCost };
        let __v352: f64 = __v351 - ((1 as i32) as f64);
        unsafe {
            (*__v350).estimatedCost = __v352;
        }
    }
    return 0 as i32;
}

/// xOpen - Open a cursor.
#[unsafe(link_section = ".text.slate_distinct.fts3_aux.fts3auxOpenMethod")]
extern "C-unwind" fn fts3auxOpenMethod(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCsr: *mut *mut sqlite3_vtab_cursor,
) -> i32 {
    let mut pCsr: *mut Fts3auxCursor = unsafe { std::mem::zeroed() }; // Pointer to cursor object to return
    pVTab;
    pCsr = (unsafe { sqlite3_malloc(((168 as u64) as u32) as i32) }) as *mut Fts3auxCursor;
    if !(pCsr != std::ptr::null_mut::<Fts3auxCursor>()) {
        return 7 as i32;
    }
    unsafe { memset(pCsr as *mut (), 0 as i32, 168 as u64) };
    unsafe {
        *ppCsr = pCsr as *mut sqlite3_vtab_cursor;
    }
    return 0 as i32;
}

/// xClose - Close a cursor.
#[unsafe(link_section = ".text.slate_distinct.fts3_aux.fts3auxCloseMethod")]
extern "C-unwind" fn fts3auxCloseMethod(mut pCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pFts3: *mut Fts3Table =
        unsafe { (*((unsafe { (*pCursor).pVtab }) as *mut Fts3auxTable)).pFts3Tab };
    let mut pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    unsafe { sqlite3Fts3SegmentsClose(pFts3) };
    unsafe { sqlite3Fts3SegReaderFinish(unsafe { std::ptr::addr_of_mut!((*pCsr).csr) }) };
    unsafe { sqlite3_free((unsafe { (*pCsr).filter.zTerm }) as *mut ()) };
    unsafe { sqlite3_free((unsafe { (*pCsr).zStop }) as *mut ()) };
    unsafe { sqlite3_free((unsafe { (*pCsr).aStat }) as *mut ()) };
    unsafe { sqlite3_free(pCsr as *mut ()) };
    return 0 as i32;
}

fn fts3auxGrowStatArray(mut pCsr: *mut Fts3auxCursor, mut nSize: i32) -> i32 {
    if nSize > unsafe { (*pCsr).nStat } {
        let mut aNew: *mut Fts3auxColstats = unsafe { std::mem::zeroed() };
        aNew = (unsafe {
            sqlite3_realloc64(
                (unsafe { (*pCsr).aStat }) as *mut (),
                (16 as u64).wrapping_mul((nSize as i64) as u64),
            )
        }) as *mut Fts3auxColstats;
        if aNew == std::ptr::null_mut::<Fts3auxColstats>() {
            return 7 as i32;
        }
        unsafe {
            memset(
                (unsafe { aNew.offset((unsafe { (*pCsr).nStat }) as isize) }) as *mut (),
                0 as i32,
                (16 as u64).wrapping_mul(((nSize - unsafe { (*pCsr).nStat }) as i64) as u64),
            )
        };
        unsafe {
            (*pCsr).aStat = aNew;
        }
        unsafe {
            (*pCsr).nStat = nSize;
        }
    }
    return 0 as i32;
}

/// xNext - Advance the cursor to the next row, if any.
#[unsafe(link_section = ".text.slate_distinct.fts3_aux.fts3auxNextMethod")]
extern "C-unwind" fn fts3auxNextMethod(mut pCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    let mut pFts3: *mut Fts3Table =
        unsafe { (*((unsafe { (*pCursor).pVtab }) as *mut Fts3auxTable)).pFts3Tab };
    let mut rc: i32 = 0 as i32;
    // Increment our pretend rowid value.
    let __v353: *mut Fts3auxCursor = pCsr;
    let __v354: i64 = unsafe { (*__v353).iRowid };
    let __v355: i64 = __v354 + ((1 as i32) as i64);
    unsafe {
        (*__v353).iRowid = __v355;
    }
    let __v356: *mut Fts3auxCursor = pCsr;
    let __v357: i32 = unsafe { (*__v356).iCol };
    let __v358: i32 = __v357 + (1 as i32);
    unsafe {
        (*__v356).iCol = __v358;
    }
    '__slate_break_316: while (unsafe { (*pCsr).iCol }) < unsafe { (*pCsr).nStat } {
        if (unsafe {
            (*unsafe { unsafe { (*pCsr).aStat }.offset((unsafe { (*pCsr).iCol }) as isize) }).nDoc
        }) > ((0 as i32) as i64)
        {
            return 0 as i32;
        }
        let __v359: *mut Fts3auxCursor = pCsr;
        let __v360: i32 = unsafe { (*__v359).iCol };
        let __v361: i32 = __v360 + (1 as i32);
        unsafe {
            (*__v359).iCol = __v361;
        }
    }
    rc = unsafe { sqlite3Fts3SegReaderStep(pFts3, unsafe { std::ptr::addr_of_mut!((*pCsr).csr) }) };
    if rc == (100 as i32) {
        let mut i: i32 = 0 as i32;
        let mut nDoclist: i32 = unsafe { (*pCsr).csr.nDoclist };
        let mut aDoclist: *mut i8 = unsafe { (*pCsr).csr.aDoclist };
        let mut iCol: i32 = 0 as i32;
        let mut eState: i32 = 0 as i32;
        if (unsafe { (*pCsr).zStop }) != std::ptr::null_mut::<i8>() {
            let mut n: i32 = if (unsafe { (*pCsr).nStop }) < unsafe { (*pCsr).csr.nTerm } {
                unsafe { (*pCsr).nStop }
            } else {
                unsafe { (*pCsr).csr.nTerm }
            };
            let mut mc: i32 = unsafe {
                memcmp(
                    (unsafe { (*pCsr).zStop }) as *const (),
                    (unsafe { (*pCsr).csr.zTerm }) as *const (),
                    (n as i64) as u64,
                )
            };
            if mc < (0 as i32)
                || mc == (0 as i32) && (unsafe { (*pCsr).csr.nTerm }) > unsafe { (*pCsr).nStop }
            {
                unsafe {
                    (*pCsr).isEof = 1 as i32;
                }
                return 0 as i32;
            }
        }
        if fts3auxGrowStatArray(pCsr, 2 as i32) != (0 as i32) {
            return 7 as i32;
        }
        unsafe {
            memset(
                (unsafe { (*pCsr).aStat }) as *mut (),
                0 as i32,
                (16 as u64).wrapping_mul(((unsafe { (*pCsr).nStat }) as i64) as u64),
            )
        };
        iCol = 0 as i32;
        rc = 0 as i32;
        '__slate_break_317: while i < nDoclist {
            let mut v: i64 = (0 as i32) as i64;
            let __v362: i32 = i;
            let __v363: i32 = __v362
                + unsafe {
                    sqlite3Fts3GetVarint(
                        (unsafe { aDoclist.offset(i as isize) }) as *const i8,
                        std::ptr::addr_of_mut!(v),
                    )
                };
            i = __v363;
            // State 0. In this state the integer just read was a docid.
            // State 1. In this state we are expecting either a 1, indicating
            // that the following integer will be a column number, or the
            // start of a position list for column 0.
            //
            // The only difference between state 1 and state 2 is that if the
            // integer encountered in state 1 is not 0 or 1, then we need to
            // increment the column 0 "nDoc" count for this term.
            // no break
            // 0x00. Next integer will be a docid.
            // 0x01. Next integer will be a column number.
            // 2 or greater. A position.
            // State 3. The integer just read is a column number.
            '__slate_break_318: {
                match eState {
                    0 => {
                        // State 0. In this state the integer just read was a docid.
                        let __v364: *mut Fts3auxColstats =
                            unsafe { unsafe { (*pCsr).aStat }.offset((0 as i32) as isize) };
                        let __v365: i64 = unsafe { (*__v364).nDoc };
                        let __v366: i64 = __v365 + ((1 as i32) as i64);
                        unsafe {
                            (*__v364).nDoc = __v366;
                        }
                        eState = 1 as i32;
                        iCol = 0 as i32;
                        break '__slate_break_318;
                        // State 1. In this state we are expecting either a 1, indicating
                        // that the following integer will be a column number, or the
                        // start of a position list for column 0.
                        //
                        // The only difference between state 1 and state 2 is that if the
                        // integer encountered in state 1 is not 0 or 1, then we need to
                        // increment the column 0 "nDoc" count for this term.
                    }
                    1 => {
                        0 as i32;
                        if v > ((1 as i32) as i64) {
                            let __v367: *mut Fts3auxColstats =
                                unsafe { unsafe { (*pCsr).aStat }.offset((1 as i32) as isize) };
                            let __v368: i64 = unsafe { (*__v367).nDoc };
                            let __v369: i64 = __v368 + ((1 as i32) as i64);
                            unsafe {
                                (*__v367).nDoc = __v369;
                            }
                        }
                        eState = 2 as i32;
                        // no break
                        {}
                        if v == ((0 as i32) as i64) {
                            // 0x00. Next integer will be a docid.
                            eState = 0 as i32;
                        } else {
                            if v == ((1 as i32) as i64) {
                                // 0x01. Next integer will be a column number.
                                eState = 3 as i32;
                            } else {
                                // 2 or greater. A position.
                                let _v379: *mut Fts3auxColstats = unsafe {
                                    unsafe { (*pCsr).aStat }.offset((iCol + (1 as i32)) as isize)
                                };
                                let _v380: i64 = unsafe { (*_v379).nOcc };
                                let _v381: i64 = _v380 + ((1 as i32) as i64);
                                unsafe {
                                    (*_v379).nOcc = _v381;
                                }
                                let _v382: *mut Fts3auxColstats =
                                    unsafe { unsafe { (*pCsr).aStat }.offset((0 as i32) as isize) };
                                let _v383: i64 = unsafe { (*_v382).nOcc };
                                let _v384: i64 = _v383 + ((1 as i32) as i64);
                                unsafe {
                                    (*_v382).nOcc = _v384;
                                }
                            }
                        }
                        break '__slate_break_318;
                        // State 3. The integer just read is a column number.
                    }
                    2 => {
                        if v == ((0 as i32) as i64) {
                            // 0x00. Next integer will be a docid.
                            eState = 0 as i32;
                        } else {
                            if v == ((1 as i32) as i64) {
                                // 0x01. Next integer will be a column number.
                                eState = 3 as i32;
                            } else {
                                // 2 or greater. A position.
                                let __v370: *mut Fts3auxColstats = unsafe {
                                    unsafe { (*pCsr).aStat }.offset((iCol + (1 as i32)) as isize)
                                };
                                let __v371: i64 = unsafe { (*__v370).nOcc };
                                let __v372: i64 = __v371 + ((1 as i32) as i64);
                                unsafe {
                                    (*__v370).nOcc = __v372;
                                }
                                let __v373: *mut Fts3auxColstats =
                                    unsafe { unsafe { (*pCsr).aStat }.offset((0 as i32) as isize) };
                                let __v374: i64 = unsafe { (*__v373).nOcc };
                                let __v375: i64 = __v374 + ((1 as i32) as i64);
                                unsafe {
                                    (*__v373).nOcc = __v375;
                                }
                            }
                        }
                        break '__slate_break_318;
                        // State 3. The integer just read is a column number.
                    }
                    _ => {
                        0 as i32;
                        iCol = v as i32;
                        if iCol < (1 as i32) || iCol > (32767 as i32) {
                            rc = (11 as i32) | (1 as i32) << (8 as i32);
                        } else {
                            if fts3auxGrowStatArray(pCsr, iCol + (2 as i32)) != (0 as i32) {
                                return 7 as i32;
                            }
                            let __v376: *mut Fts3auxColstats = unsafe {
                                unsafe { (*pCsr).aStat }.offset((iCol + (1 as i32)) as isize)
                            };
                            let __v377: i64 = unsafe { (*__v376).nDoc };
                            let __v378: i64 = __v377 + ((1 as i32) as i64);
                            unsafe {
                                (*__v376).nDoc = __v378;
                            }
                            eState = 2 as i32;
                        }
                    }
                }
            }
        }
        unsafe {
            (*pCsr).iCol = 0 as i32;
        }
    } else {
        unsafe {
            (*pCsr).isEof = 1 as i32;
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
#[unsafe(link_section = ".text.slate_distinct.fts3_aux.fts3auxFilterMethod")]
extern "C-unwind" fn fts3auxFilterMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: i32,
    mut idxStr: *const i8,
    mut nVal: i32,
    mut apVal: *mut *mut sqlite3_value,
) -> i32 {
    let mut pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    let mut pFts3: *mut Fts3Table =
        unsafe { (*((unsafe { (*pCursor).pVtab }) as *mut Fts3auxTable)).pFts3Tab };
    let mut rc: i32 = 0 as i32;
    let mut isScan: i32 = 0 as i32;
    let mut iLangVal: i32 = 0 as i32; // Language id to query
    let mut iEq: i32 = -(1 as i32); // Index of term=? value in apVal
    let mut iGe: i32 = -(1 as i32); // Index of term>=? value in apVal
    let mut iLe: i32 = -(1 as i32); // Index of term<=? value in apVal
    let mut iLangid: i32 = -(1 as i32); // Index of languageid=? value in apVal
    let mut iNext: i32 = 0 as i32;
    nVal;
    idxStr;
    0 as i32;
    0 as i32;
    if idxNum == (1 as i32) {
        let __v379: i32 = iNext;
        let __v380: i32 = __v379 + (1 as i32);
        iNext = __v380;
        iEq = __v379;
    } else {
        isScan = 1 as i32;
        if idxNum & (2 as i32) != (0 as i32) {
            let __v381: i32 = iNext;
            let __v382: i32 = __v381 + (1 as i32);
            iNext = __v382;
            iGe = __v381;
        }
        if idxNum & (4 as i32) != (0 as i32) {
            let __v383: i32 = iNext;
            let __v384: i32 = __v383 + (1 as i32);
            iNext = __v384;
            iLe = __v383;
        }
    }
    if iNext < nVal {
        let __v385: i32 = iNext;
        let __v386: i32 = __v385 + (1 as i32);
        iNext = __v386;
        iLangid = __v385;
    }
    // In case this cursor is being reused, close and zero it.
    {}
    unsafe { sqlite3Fts3SegReaderFinish(unsafe { std::ptr::addr_of_mut!((*pCsr).csr) }) };
    unsafe { sqlite3_free((unsafe { (*pCsr).filter.zTerm }) as *mut ()) };
    unsafe { sqlite3_free((unsafe { (*pCsr).aStat }) as *mut ()) };
    unsafe { sqlite3_free((unsafe { (*pCsr).zStop }) as *mut ()) };
    unsafe {
        memset(
            (unsafe { std::ptr::addr_of_mut!((*pCsr).csr) }) as *mut (),
            0 as i32,
            ((unsafe {
                ((unsafe { pCsr.offset((1 as i32) as isize) }) as *mut u8).offset_from(
                    ((unsafe { std::ptr::addr_of_mut!((*pCsr).csr) }) as *mut u8) as *mut u8,
                )
            }) as i64) as u64,
        )
    };
    unsafe {
        (*pCsr).filter.flags = (1 as i32) | (2 as i32);
    }
    if isScan != (0 as i32) {
        let __v387: *mut Fts3auxCursor = pCsr;
        let __v388: i32 = unsafe { (*__v387).filter.flags };
        let __v389: i32 = __v388 | (16 as i32);
        unsafe {
            (*__v387).filter.flags = __v389;
        }
    }
    let __v390: bool;
    if iEq >= (0 as i32) || iGe >= (0 as i32) {
        __v390 = (unsafe {
            sqlite3_value_type(unsafe { *unsafe { apVal.offset((0 as i32) as isize) } })
        }) == (3 as i32);
    } else {
        __v390 = false as bool;
    }
    if __v390 {
        let mut zStr: *const u8 =
            unsafe { sqlite3_value_text(unsafe { *unsafe { apVal.offset((0 as i32) as isize) } }) };
        0 as i32;
        if zStr != std::ptr::null::<u8>() {
            unsafe {
                (*pCsr).filter.zTerm =
                    (unsafe { sqlite3_mprintf((b"%s\0".as_ptr() as *mut i8) as *const i8, zStr) })
                        as *const i8;
            }
            if (unsafe { (*pCsr).filter.zTerm }) == std::ptr::null::<i8>() {
                return 7 as i32;
            }
            unsafe {
                (*pCsr).filter.nTerm =
                    ((unsafe { strlen(unsafe { (*pCsr).filter.zTerm }) }) as u32) as i32;
            }
        }
    }
    let __v391: bool;
    if iLe >= (0 as i32) {
        __v391 = (unsafe {
            sqlite3_value_type(unsafe { *unsafe { apVal.offset((0 as i32) as isize) } })
        }) == (3 as i32);
    } else {
        __v391 = false as bool;
    }
    if __v391 {
        unsafe {
            (*pCsr).zStop = unsafe {
                sqlite3_mprintf((b"%s\0".as_ptr() as *mut i8) as *const i8, unsafe {
                    sqlite3_value_text(unsafe { *unsafe { apVal.offset(iLe as isize) } })
                })
            };
        }
        if (unsafe { (*pCsr).zStop }) == std::ptr::null_mut::<i8>() {
            return 7 as i32;
        }
        unsafe {
            (*pCsr).nStop =
                ((unsafe { strlen((unsafe { (*pCsr).zStop }) as *const i8) }) as u32) as i32;
        }
    }
    if iLangid >= (0 as i32) {
        iLangVal =
            unsafe { sqlite3_value_int(unsafe { *unsafe { apVal.offset(iLangid as isize) } }) };
        // If the user specified a negative value for the languageid, use zero
        // instead. This works, as the "languageid=?" constraint will also
        // be tested by the VDBE layer. The test will always be false (since
        // this module will not return a row with a negative languageid), and
        // so the overall query will return zero rows.
        if iLangVal < (0 as i32) {
            iLangVal = 0 as i32;
        }
    }
    unsafe {
        (*pCsr).iLangid = iLangVal;
    }
    rc = unsafe {
        sqlite3Fts3SegReaderCursor(
            pFts3,
            iLangVal,
            0 as i32,
            -(2 as i32),
            unsafe { (*pCsr).filter.zTerm },
            unsafe { (*pCsr).filter.nTerm },
            0 as i32,
            isScan,
            unsafe { std::ptr::addr_of_mut!((*pCsr).csr) },
        )
    };
    if rc == (0 as i32) {
        rc = unsafe {
            sqlite3Fts3SegReaderStart(
                pFts3,
                unsafe { std::ptr::addr_of_mut!((*pCsr).csr) },
                unsafe { std::ptr::addr_of_mut!((*pCsr).filter) },
            )
        };
    }
    if rc == (0 as i32) {
        rc = fts3auxNextMethod(pCursor);
    }
    return rc;
}

/// xEof - Return true if the cursor is at EOF, or false otherwise.
#[unsafe(link_section = ".text.slate_distinct.fts3_aux.fts3auxEofMethod")]
extern "C-unwind" fn fts3auxEofMethod(mut pCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    return unsafe { (*pCsr).isEof };
}

/// xColumn - Return a column value.
///
/// # Arguments
///
/// * `pCursor` - Cursor to retrieve value from
/// * `pCtx` - Context for sqlite3_result_xxx() calls
/// * `iCol` - Index of column to read value from
#[unsafe(link_section = ".text.slate_distinct.fts3_aux.fts3auxColumnMethod")]
extern "C-unwind" fn fts3auxColumnMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pCtx: *mut sqlite3_context,
    mut iCol: i32,
) -> i32 {
    let mut p: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    0 as i32;
    '__slate_break_321: {
        match iCol {
            0 => {
                unsafe {
                    sqlite3_result_text(
                        pCtx,
                        (unsafe { (*p).csr.zTerm }) as *const i8,
                        unsafe { (*p).csr.nTerm },
                        unsafe {
                            std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                                -(1 as i32) as usize,
                            )
                        },
                    )
                }; // term
            }
            1 => {
                if (unsafe { (*p).iCol }) != (0 as i32) {
                    unsafe { sqlite3_result_int(pCtx, (unsafe { (*p).iCol }) - (1 as i32)) };
                } else {
                    unsafe {
                        sqlite3_result_text(
                            pCtx,
                            (b"*\0".as_ptr() as *mut i8) as *const i8,
                            -(1 as i32),
                            None,
                        )
                    };
                }
                // col
            }
            2 => {
                unsafe {
                    sqlite3_result_int64(pCtx, unsafe {
                        (*unsafe { unsafe { (*p).aStat }.offset((unsafe { (*p).iCol }) as isize) })
                            .nDoc
                    })
                }; // documents
            }
            3 => {
                unsafe {
                    sqlite3_result_int64(pCtx, unsafe {
                        (*unsafe { unsafe { (*p).aStat }.offset((unsafe { (*p).iCol }) as isize) })
                            .nOcc
                    })
                }; // occurrences
            }
            _ => {
                0 as i32; // languageid
                unsafe { sqlite3_result_int(pCtx, unsafe { (*p).iLangid }) };
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
#[unsafe(link_section = ".text.slate_distinct.fts3_aux.fts3auxRowidMethod")]
extern "C-unwind" fn fts3auxRowidMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut i64,
) -> i32 {
    let mut pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    unsafe {
        *pRowid = unsafe { (*pCsr).iRowid };
    }
    return 0 as i32;
}

/// Register the fts3aux module with database connection db. Return SQLITE_OK
/// if successful or an error code if sqlite3_create_module() fails.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3InitAux(mut db: *mut sqlite3) -> i32 {
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
        sqlite3_create_module(
            db,
            (b"fts4aux\0".as_ptr() as *mut i8) as *const i8,
            unsafe { std::ptr::addr_of!(fts3aux_module) },
            std::ptr::null_mut::<()>(),
        )
    };
    return rc;
}

static mut fts3aux_module: sqlite3_module = sqlite3_module {
    iVersion: 0 as i32,
    xCreate: Some(fts3auxConnectMethod),
    xConnect: Some(fts3auxConnectMethod),
    xBestIndex: Some(fts3auxBestIndexMethod),
    xDisconnect: Some(fts3auxDisconnectMethod),
    xDestroy: Some(fts3auxDisconnectMethod),
    xOpen: Some(fts3auxOpenMethod),
    xClose: Some(fts3auxCloseMethod),
    xFilter: Some(fts3auxFilterMethod),
    xNext: Some(fts3auxNextMethod),
    xEof: Some(fts3auxEofMethod),
    xColumn: Some(fts3auxColumnMethod),
    xRowid: Some(fts3auxRowidMethod),
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
