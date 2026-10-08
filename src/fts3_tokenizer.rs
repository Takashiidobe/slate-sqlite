//! 2007 June 22
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
//! This is part of an SQLite module implementing full-text search.
//! This particular file implements the generic tokenizer interface.
unsafe extern "C" {
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3_db_config(__v121: *mut sqlite3, op: i32, ...) -> i32;
    fn sqlite3_mprintf(__v123: *const i8, ...) -> *mut i8;
    fn sqlite3_realloc64(__v124: *mut (), __v125: u64) -> *mut ();
    fn sqlite3_free(__v126: *mut ());
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
    fn sqlite3_value_blob(__v135: *mut sqlite3_value) -> *const ();
    fn sqlite3_value_text(__v136: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_bytes(__v137: *mut sqlite3_value) -> i32;
    fn sqlite3_value_frombind(__v138: *mut sqlite3_value) -> i32;
    fn sqlite3_user_data(__v139: *mut sqlite3_context) -> *mut ();
    fn sqlite3_context_db_handle(__v140: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_blob(
        __v141: *mut sqlite3_context,
        __v142: *const (),
        __v143: i32,
        __v144: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_error(__v145: *mut sqlite3_context, __v146: *const i8, __v147: i32);
    fn sqlite3Fts3HashInsert(
        __v148: *mut Fts3Hash,
        pKey: *const (),
        nKey: i32,
        pData: *mut (),
    ) -> *mut ();
    fn sqlite3Fts3HashFind(__v152: *const Fts3Hash, pKey: *const (), nKey: i32) -> *mut ();
    fn sqlite3Fts3ErrMsg(__v155: *mut *mut i8, __v156: *const i8, ...);
    fn sqlite3Fts3Dequote(__v157: *mut i8);
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

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

// The code in this file is only compiled if:
//
//     * The FTS3 module is being built as an extension
//       (in which case SQLITE_CORE is not defined), or
//
//     * The FTS3 module is being built into the core of
//       SQLite (in which case SQLITE_ENABLE_FTS3 is defined).
/// Return true if the two-argument version of fts3_tokenizer()
/// has been activated via a prior call to sqlite3_db_config(db,
/// SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER, 1, 0);
fn fts3TokenizerEnabled(mut context: *mut sqlite3_context) -> i32 {
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    let mut isEnabled: i32 = 0 as i32;
    unsafe {
        sqlite3_db_config(
            db,
            1004 as i32,
            -(1 as i32),
            std::ptr::addr_of_mut!(isEnabled),
        )
    };
    return isEnabled;
}

/// Implementation of the SQL scalar function for accessing the underlying
/// hash table. This function may be called as follows:
///
///   SELECT <function-name>(<key-name>);
///   SELECT <function-name>(<key-name>, <pointer>);
///
/// where <function-name> is the name passed as the second argument
/// to the sqlite3Fts3InitHashTable() function (e.g. 'fts3_tokenizer').
///
/// If the <pointer> argument is specified, it must be a blob value
/// containing a pointer to be stored as the hash data corresponding
/// to the string <key-name>. If <pointer> is not specified, then
/// the string <key-name> must already exist in the has table. Otherwise,
/// an error is returned.
///
/// Whether or not the <pointer> argument is specified, the value returned
/// is a blob containing the pointer stored as the hash data corresponding
/// to string <key-name> (after the hash-table is updated, if applicable).
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenizer.fts3TokenizerFunc")]
extern "C-unwind" fn fts3TokenizerFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut pHash: *mut Fts3Hash = unsafe { std::mem::zeroed() };
    let mut pPtr: *mut () = std::ptr::null_mut::<()>();
    let mut zName: *const u8 = unsafe { std::mem::zeroed() };
    let mut nName: i32 = 0 as i32;
    0 as i32;
    pHash = (unsafe { sqlite3_user_data(context) }) as *mut Fts3Hash;
    zName = unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    nName =
        (unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
            + (1 as i32);
    if argc == (2 as i32) {
        let __v199: bool;
        if fts3TokenizerEnabled(context) != (0 as i32) {
            __v199 = true as bool;
        } else {
            __v199 = (unsafe {
                sqlite3_value_frombind(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
            }) != (0 as i32);
        }
        if __v199 {
            let mut pOld: *mut () = unsafe { std::mem::zeroed() };
            let mut n: i32 = unsafe {
                sqlite3_value_bytes(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
            };
            if zName == std::ptr::null::<u8>() || ((n as i64) as u64) != (8 as u64) {
                unsafe {
                    sqlite3_result_error(
                        context,
                        (b"argument type mismatch\0".as_ptr() as *mut i8) as *const i8,
                        -(1 as i32),
                    )
                };
                return;
            }
            pPtr = unsafe {
                *((unsafe {
                    sqlite3_value_blob(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
                }) as *mut *mut ())
            };
            pOld = unsafe {
                sqlite3Fts3HashInsert(pHash, (zName as *mut ()) as *const (), nName, pPtr)
            };
            if pOld == pPtr {
                unsafe {
                    sqlite3_result_error(
                        context,
                        (b"out of memory\0".as_ptr() as *mut i8) as *const i8,
                        -(1 as i32),
                    )
                };
            }
        } else {
            unsafe {
                sqlite3_result_error(
                    context,
                    (b"fts3tokenize disabled\0".as_ptr() as *mut i8) as *const i8,
                    -(1 as i32),
                )
            };
            return;
        }
    } else {
        if zName != std::ptr::null::<u8>() {
            pPtr =
                unsafe { sqlite3Fts3HashFind(pHash as *const Fts3Hash, zName as *const (), nName) };
        }
        if !(pPtr != std::ptr::null_mut::<()>()) {
            let mut zErr: *mut i8 = unsafe {
                sqlite3_mprintf(
                    (b"unknown tokenizer: %s\0".as_ptr() as *mut i8) as *const i8,
                    zName,
                )
            };
            unsafe { sqlite3_result_error(context, zErr as *const i8, -(1 as i32)) };
            unsafe { sqlite3_free(zErr as *mut ()) };
            return;
        }
    }
    let __v200: bool;
    if fts3TokenizerEnabled(context) != (0 as i32) {
        __v200 = true as bool;
    } else {
        __v200 = (unsafe {
            sqlite3_value_frombind(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
        }) != (0 as i32);
    }
    if __v200 {
        unsafe {
            sqlite3_result_blob(
                context,
                (std::ptr::addr_of_mut!(pPtr) as *mut ()) as *const (),
                ((8 as u64) as u32) as i32,
                unsafe {
                    std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        -(1 as i32) as usize,
                    )
                },
            )
        };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3IsIdChar(mut c: i8) -> i32 {
    // 0x
    // 1x
    // 2x
    // 3x
    // 4x
    // 5x
    // 6x
    // 7x
    return ((c as i32) & (128 as i32) != (0 as i32)
        || (unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(isFtsIdChar.0) as *const i8 }
                    .offset((c as i32) as isize)
            }
        }) != (0 as i8)) as i32;
}

static mut isFtsIdChar: __SlateAlign16<[i8; 128]> = __SlateAlign16([
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
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
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
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
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
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3NextToken(mut zStr: *const i8, mut pn: *mut i32) -> *const i8 {
    let mut z1: *const i8 = unsafe { std::mem::zeroed() };
    let mut z2: *const i8 = std::ptr::null::<i8>();
    // Find the start of the next token.
    z1 = zStr;
    '__slate_break_172: while z2 == std::ptr::null::<i8>() {
        let mut c: i8 = unsafe { *z1 };
        match c as i32 {
            0 => {
                return std::ptr::null::<i8>(); // No more tokens here
            }
            39 | 34 | 96 => {
                z2 = z1;
                '__slate_break_174: loop {
                    let __v181: *const i8 = z2;
                    let __v182: *const i8 = unsafe { __v181.offset((1 as i32) as isize) };
                    z2 = __v182;
                    let __v183: bool;
                    if (unsafe { *__v182 }) != (0 as i8) {
                        let __v184: bool;
                        if ((unsafe { *z2 }) as i32) != (c as i32) {
                            __v184 = true as bool;
                        } else {
                            let __v185: *const i8 = z2;
                            let __v186: *const i8 = unsafe { __v185.offset((1 as i32) as isize) };
                            z2 = __v186;
                            __v184 = ((unsafe { *__v186 }) as i32) == (c as i32);
                        }
                        __v183 = __v184;
                    } else {
                        __v183 = false as bool;
                    }
                    if !__v183 {
                        break;
                    }
                    {}
                }
            }
            91 => {
                z2 = unsafe { z1.offset((1 as i32) as isize) };
                '__slate_break_175: while (unsafe { *z2 }) != (0 as i8)
                    && ((unsafe { *unsafe { z2.offset((0 as i32) as isize) } }) as i32)
                        != (93 as i32)
                {
                    let __v187: *const i8 = z2;
                    let __v188: *const i8 = unsafe { __v187.offset((1 as i32) as isize) };
                    z2 = __v188;
                }
                if (unsafe { *z2 }) != (0 as i8) {
                    let __v189: *const i8 = z2;
                    let __v190: *const i8 = unsafe { __v189.offset((1 as i32) as isize) };
                    z2 = __v190;
                }
            }
            _ => {
                if sqlite3Fts3IsIdChar(unsafe { *z1 }) != (0 as i32) {
                    z2 = unsafe { z1.offset((1 as i32) as isize) };
                    '__slate_break_176: while sqlite3Fts3IsIdChar(unsafe { *z2 }) != (0 as i32) {
                        let __v191: *const i8 = z2;
                        let __v192: *const i8 = unsafe { __v191.offset((1 as i32) as isize) };
                        z2 = __v192;
                    }
                } else {
                    let __v193: *const i8 = z1;
                    let __v194: *const i8 = unsafe { __v193.offset((1 as i32) as isize) };
                    z1 = __v194;
                }
            }
        }
    }
    unsafe {
        *pn = ((unsafe { z2.offset_from(z1 as *const i8) }) as i64) as i32;
    }
    return z1;
}

/// # Arguments
///
/// * `pHash` - Tokenizer hash table
/// * `zArg` - Tokenizer name
/// * `ppTok` - OUT: Tokenizer (if applicable)
/// * `pzErr` - OUT: Set to malloced error message
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3InitTokenizer(
    mut pHash: *mut Fts3Hash,
    mut zArg: *const i8,
    mut ppTok: *mut *mut sqlite3_tokenizer,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut z: *mut i8 = zArg as *mut i8;
    let mut n: i32 = 0 as i32;
    let mut zCopy: *mut i8 = unsafe { std::mem::zeroed() };
    let mut zEnd: *mut i8 = unsafe { std::mem::zeroed() }; // Pointer to nul-term of zCopy
    let mut m: *mut sqlite3_tokenizer_module = unsafe { std::mem::zeroed() };
    zCopy = unsafe { sqlite3_mprintf((b"%s\0".as_ptr() as *mut i8) as *const i8, zArg) };
    if !(zCopy != std::ptr::null_mut::<i8>()) {
        return 7 as i32;
    }
    zEnd = unsafe { zCopy.offset((unsafe { strlen(zCopy as *const i8) }) as isize) };
    z = sqlite3Fts3NextToken(zCopy as *const i8, std::ptr::addr_of_mut!(n)) as *mut i8;
    if z == std::ptr::null_mut::<i8>() {
        0 as i32;
        z = zCopy;
    }
    unsafe {
        *unsafe { z.offset(n as isize) } = (0 as i32) as i8;
    }
    unsafe { sqlite3Fts3Dequote(z) };
    m = (unsafe {
        sqlite3Fts3HashFind(
            pHash as *const Fts3Hash,
            z as *const (),
            (((unsafe { strlen(z as *const i8) }) as u32) as i32) + (1 as i32),
        )
    }) as *mut sqlite3_tokenizer_module;
    if !(m != std::ptr::null_mut::<sqlite3_tokenizer_module>()) {
        unsafe {
            sqlite3Fts3ErrMsg(
                pzErr,
                (b"unknown tokenizer: %s\0".as_ptr() as *mut i8) as *const i8,
                z,
            )
        };
        rc = 1 as i32;
    } else {
        let mut aArg: *mut *const i8 = std::ptr::null_mut::<*const i8>();
        let mut iArg: i32 = 0 as i32;
        z = unsafe { z.offset((n + (1 as i32)) as isize) };
        '__slate_break_179: loop {
            let __v195: bool;
            if z < zEnd {
                let __v196: *mut i8 =
                    sqlite3Fts3NextToken(z as *const i8, std::ptr::addr_of_mut!(n)) as *mut i8;
                z = __v196;
                __v195 = std::ptr::null_mut::<()>() != (__v196 as *mut ());
            } else {
                __v195 = false as bool;
            }
            if !__v195 {
                break;
            }
            let mut nNew: i64 = (8 as u64).wrapping_mul(((iArg + (1 as i32)) as i64) as u64) as i64;
            let mut aNew: *mut *const i8 =
                (unsafe { sqlite3_realloc64(aArg as *mut (), nNew as u64) }) as *mut *const i8;
            if !(aNew != std::ptr::null_mut::<*const i8>()) {
                unsafe { sqlite3_free(zCopy as *mut ()) };
                unsafe { sqlite3_free(aArg as *mut ()) };
                return 7 as i32;
            }
            aArg = aNew;
            let __v197: i32 = iArg;
            let __v198: i32 = __v197 + (1 as i32);
            iArg = __v198;
            unsafe {
                *unsafe { aArg.offset(__v197 as isize) } = z as *const i8;
            }
            unsafe {
                *unsafe { z.offset(n as isize) } = (0 as i32) as i8;
            }
            unsafe { sqlite3Fts3Dequote(z) };
            z = unsafe { z.offset((n + (1 as i32)) as isize) };
        }
        rc = unsafe { unsafe { (*m).xCreate }.unwrap()(iArg, aArg as *const *const i8, ppTok) };
        0 as i32;
        if rc != (0 as i32) {
            unsafe {
                sqlite3Fts3ErrMsg(
                    pzErr,
                    (b"unknown tokenizer\0".as_ptr() as *mut i8) as *const i8,
                )
            };
        } else {
            unsafe {
                (*unsafe { *ppTok }).pModule = m as *const sqlite3_tokenizer_module;
            }
        }
        unsafe { sqlite3_free(aArg as *mut ()) };
    }
    unsafe { sqlite3_free(zCopy as *mut ()) };
    return rc;
}

/// Set up SQL objects in database db used to access the contents of
/// the hash table pointed to by argument pHash. The hash table must
/// been initialized to use string keys, and to take a private copy
/// of the key when a value is inserted. i.e. by a call similar to:
///
///    sqlite3Fts3HashInit(pHash, FTS3_HASH_STRING, 1);
///
/// This function adds a scalar function (see header comment above
/// fts3TokenizerFunc() in this file for details) and, if ENABLE_TABLE is
/// defined at compilation time, a temporary virtual table (see header
/// comment above struct HashTableVtab) to the database schema. Both
/// provide read/write access to the contents of *pHash.
///
/// The third argument to this function, zName, is used as the name
/// of both the scalar and, if created, the virtual table.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3InitHashTable(
    mut db: *mut sqlite3,
    mut pHash: *mut Fts3Hash,
    mut zName: *const i8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut p: *mut () = pHash as *mut ();
    let mut any: i32 = (1 as i32) | (524288 as i32);
    if (0 as i32) == rc {
        rc = unsafe {
            sqlite3_create_function(
                db,
                zName,
                1 as i32,
                any,
                p,
                Some(fts3TokenizerFunc),
                None,
                None,
            )
        };
    }
    if (0 as i32) == rc {
        rc = unsafe {
            sqlite3_create_function(
                db,
                zName,
                2 as i32,
                any,
                p,
                Some(fts3TokenizerFunc),
                None,
                None,
            )
        };
    }
    return rc;
}
