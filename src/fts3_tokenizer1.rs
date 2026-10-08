//! 2006 Oct 10
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
//! Implementation of the "simple" full-text-search tokenizer.
unsafe extern "C" {
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3_malloc(__v86: i32) -> *mut ();
    fn sqlite3_realloc64(__v87: *mut (), __v88: u64) -> *mut ();
    fn sqlite3_free(__v89: *mut ());
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

// The code in this file is only compiled if:
//
//     * The FTS3 module is being built as an extension
//       (in which case SQLITE_CORE is not defined), or
//
//     * The FTS3 module is being built into the core of
//       SQLite (in which case SQLITE_ENABLE_FTS3 is defined).
#[repr(C)]
#[derive(Clone, Copy)]
struct simple_tokenizer {
    base: sqlite3_tokenizer,
    /// flag ASCII delimiters
    delim: [i8; 128],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct simple_tokenizer_cursor {
    base: sqlite3_tokenizer_cursor,
    /// input we are tokenizing
    pInput: *const i8,
    /// size of the input
    nBytes: i32,
    /// current position in pInput
    iOffset: i32,
    /// index of next token to be returned
    iToken: i32,
    /// storage for current token
    pToken: *mut i8,
    /// space allocated to zToken buffer
    nTokenAllocated: i32,
}

fn simpleDelim(mut t: *mut simple_tokenizer, mut c: u8) -> i32 {
    return (((c as u32) as i32) < (128 as i32)
        && (unsafe {
            *unsafe {
                unsafe { (*t).delim.as_mut_ptr() as *mut i8 }.offset(((c as u32) as i32) as isize)
            }
        }) != (0 as i8)) as i32;
}

fn fts3_isalnum(mut x: i32) -> i32 {
    return (x >= (48 as i32) && x <= (57 as i32)
        || x >= (65 as i32) && x <= (90 as i32)
        || x >= (97 as i32) && x <= (122 as i32)) as i32;
}

/// Create a new tokenizer instance.
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenizer1.simpleCreate")]
extern "C-unwind" fn simpleCreate(
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppTokenizer: *mut *mut sqlite3_tokenizer,
) -> i32 {
    let mut t: *mut simple_tokenizer = unsafe { std::mem::zeroed() };
    t = (unsafe { sqlite3_malloc(((136 as u64) as u32) as i32) }) as *mut simple_tokenizer;
    if t == std::ptr::null_mut::<simple_tokenizer>() {
        return 7 as i32;
    }
    unsafe { memset(t as *mut (), 0 as i32, 136 as u64) };
    // TODO(shess) Delimiters need to remain the same from run to run,
    // else we need to reindex.  One solution would be a meta-table to
    // track such information in the database, then we'd only want this
    // information on the initial create.
    if argc > (1 as i32) {
        let mut i: i32 = 0 as i32;
        let mut n: i32 =
            ((unsafe { strlen(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) }) as u32)
                as i32;
        i = 0 as i32;
        '__slate_break_90: loop {
            if !(i < n) {
                break;
            }
            let mut ch: u8 = (unsafe {
                *unsafe {
                    unsafe { *unsafe { argv.offset((1 as i32) as isize) } }.offset(i as isize)
                }
            }) as u8;
            // We explicitly don't support UTF-8 delimiters for now.
            if ((ch as u32) as i32) >= (128 as i32) {
                unsafe { sqlite3_free(t as *mut ()) };
                return 1 as i32;
            }
            unsafe {
                *unsafe {
                    unsafe { (*t).delim.as_mut_ptr() as *mut i8 }
                        .offset(((ch as u32) as i32) as isize)
                } = (1 as i32) as i8;
            }
            let __v96: i32 = i;
            let __v97: i32 = __v96 + (1 as i32);
            i = __v97;
        }
    } else {
        // Mark non-alphanumeric ASCII characters as delimiters
        let mut i: i32 = 0 as i32;
        i = 1 as i32;
        '__slate_break_91: loop {
            if !(i < (128 as i32)) {
                break;
            }
            unsafe {
                *unsafe { unsafe { (*t).delim.as_mut_ptr() as *mut i8 }.offset(i as isize) } =
                    (if !(fts3_isalnum(i) != (0 as i32)) {
                        -(1 as i32)
                    } else {
                        0 as i32
                    }) as i8;
            }
            let __v98: i32 = i;
            let __v99: i32 = __v98 + (1 as i32);
            i = __v99;
        }
    }
    unsafe {
        *ppTokenizer = unsafe { std::ptr::addr_of_mut!((*t).base) };
    }
    return 0 as i32;
}

/// Destroy a tokenizer
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenizer1.simpleDestroy")]
extern "C-unwind" fn simpleDestroy(mut pTokenizer: *mut sqlite3_tokenizer) -> i32 {
    unsafe { sqlite3_free(pTokenizer as *mut ()) };
    return 0 as i32;
}

/// Prepare to begin tokenizing a particular string.  The input
/// string to be tokenized is pInput[0..nBytes-1].  A cursor
/// used to incrementally tokenize this string is returned in
/// *ppCursor.
///
/// # Arguments
///
/// * `pTokenizer` - The tokenizer
/// * `nBytes` - String to be tokenized
/// * `ppCursor` - OUT: Tokenization cursor
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenizer1.simpleOpen")]
extern "C-unwind" fn simpleOpen(
    mut pTokenizer: *mut sqlite3_tokenizer,
    mut pInput: *const i8,
    mut nBytes: i32,
    mut ppCursor: *mut *mut sqlite3_tokenizer_cursor,
) -> i32 {
    let mut c: *mut simple_tokenizer_cursor = unsafe { std::mem::zeroed() };
    pTokenizer;
    c = (unsafe { sqlite3_malloc(((48 as u64) as u32) as i32) }) as *mut simple_tokenizer_cursor;
    if c == std::ptr::null_mut::<simple_tokenizer_cursor>() {
        return 7 as i32;
    }
    unsafe {
        (*c).pInput = pInput;
    }
    if pInput == std::ptr::null::<i8>() {
        unsafe {
            (*c).nBytes = 0 as i32;
        }
    } else {
        if nBytes < (0 as i32) {
            unsafe {
                (*c).nBytes = ((unsafe { strlen(pInput) }) as u32) as i32;
            }
        } else {
            unsafe {
                (*c).nBytes = nBytes;
            }
        }
    }
    unsafe {
        (*c).iOffset = 0 as i32;
    }
    // start tokenizing at the beginning
    unsafe {
        (*c).iToken = 0 as i32;
    }
    unsafe {
        (*c).pToken = std::ptr::null_mut::<i8>();
    }
    // no space allocated, yet.
    unsafe {
        (*c).nTokenAllocated = 0 as i32;
    }
    unsafe {
        *ppCursor = unsafe { std::ptr::addr_of_mut!((*c).base) };
    }
    return 0 as i32;
}

/// Close a tokenization cursor previously opened by a call to
/// simpleOpen() above.
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenizer1.simpleClose")]
extern "C-unwind" fn simpleClose(mut pCursor: *mut sqlite3_tokenizer_cursor) -> i32 {
    let mut c: *mut simple_tokenizer_cursor = pCursor as *mut simple_tokenizer_cursor;
    unsafe { sqlite3_free((unsafe { (*c).pToken }) as *mut ()) };
    unsafe { sqlite3_free(c as *mut ()) };
    return 0 as i32;
}

/// Extract the next token from a tokenization cursor.  The cursor must
/// have been opened by a prior call to simpleOpen().
///
/// # Arguments
///
/// * `pCursor` - Cursor returned by simpleOpen
/// * `ppToken` - OUT: *ppToken is the token text
/// * `pnBytes` - OUT: Number of bytes in token
/// * `piStartOffset` - OUT: Starting offset of token
/// * `piEndOffset` - OUT: Ending offset of token
/// * `piPosition` - OUT: Position integer of token
#[unsafe(link_section = ".text.slate_distinct.fts3_tokenizer1.simpleNext")]
extern "C-unwind" fn simpleNext(
    mut pCursor: *mut sqlite3_tokenizer_cursor,
    mut ppToken: *mut *const i8,
    mut pnBytes: *mut i32,
    mut piStartOffset: *mut i32,
    mut piEndOffset: *mut i32,
    mut piPosition: *mut i32,
) -> i32 {
    let mut c: *mut simple_tokenizer_cursor = pCursor as *mut simple_tokenizer_cursor;
    let mut t: *mut simple_tokenizer = (unsafe { (*pCursor).pTokenizer }) as *mut simple_tokenizer;
    let mut p: *mut u8 = (unsafe { (*c).pInput }) as *mut u8;
    '__slate_break_92: while (unsafe { (*c).iOffset }) < unsafe { (*c).nBytes } {
        let mut iStartOffset: i32 = 0 as i32;
        // Scan past delimiter characters
        '__slate_break_93: loop {
            let __v100: bool;
            if (unsafe { (*c).iOffset }) < unsafe { (*c).nBytes } {
                __v100 = simpleDelim(t, unsafe {
                    *unsafe { p.offset((unsafe { (*c).iOffset }) as isize) }
                }) != (0 as i32);
            } else {
                __v100 = false as bool;
            }
            if !__v100 {
                break;
            }
            let __v101: *mut simple_tokenizer_cursor = c;
            let __v102: i32 = unsafe { (*__v101).iOffset };
            let __v103: i32 = __v102 + (1 as i32);
            unsafe {
                (*__v101).iOffset = __v103;
            }
        }
        // Count non-delimiter characters.
        iStartOffset = unsafe { (*c).iOffset };
        '__slate_break_94: loop {
            let __v104: bool;
            if (unsafe { (*c).iOffset }) < unsafe { (*c).nBytes } {
                __v104 = !(simpleDelim(t, unsafe {
                    *unsafe { p.offset((unsafe { (*c).iOffset }) as isize) }
                }) != (0 as i32));
            } else {
                __v104 = false as bool;
            }
            if !__v104 {
                break;
            }
            let __v105: *mut simple_tokenizer_cursor = c;
            let __v106: i32 = unsafe { (*__v105).iOffset };
            let __v107: i32 = __v106 + (1 as i32);
            unsafe {
                (*__v105).iOffset = __v107;
            }
        }
        if (unsafe { (*c).iOffset }) > iStartOffset {
            let mut i: i32 = 0 as i32;
            let mut n: i32 = (unsafe { (*c).iOffset }) - iStartOffset;
            if n > unsafe { (*c).nTokenAllocated } {
                let mut pNew: *mut i8 = unsafe { std::mem::zeroed() };
                unsafe {
                    (*c).nTokenAllocated = n + (20 as i32);
                }
                pNew = (unsafe {
                    sqlite3_realloc64(
                        (unsafe { (*c).pToken }) as *mut (),
                        ((unsafe { (*c).nTokenAllocated }) as i64) as u64,
                    )
                }) as *mut i8;
                if !(pNew != std::ptr::null_mut::<i8>()) {
                    return 7 as i32;
                }
                unsafe {
                    (*c).pToken = pNew;
                }
            }
            i = 0 as i32;
            '__slate_break_95: loop {
                if !(i < n) {
                    break;
                }
                // TODO(shess) This needs expansion to handle UTF-8
                // case-insensitivity.
                let mut ch: u8 = unsafe { *unsafe { p.offset((iStartOffset + i) as isize) } };
                unsafe {
                    *unsafe { unsafe { (*c).pToken }.offset(i as isize) } =
                        (if ((ch as u32) as i32) >= (65 as i32)
                            && ((ch as u32) as i32) <= (90 as i32)
                        {
                            ((ch as u32) as i32) - (65 as i32) + (97 as i32)
                        } else {
                            (ch as u32) as i32
                        }) as i8;
                }
                let __v108: i32 = i;
                let __v109: i32 = __v108 + (1 as i32);
                i = __v109;
            }
            unsafe {
                *ppToken = (unsafe { (*c).pToken }) as *const i8;
            }
            unsafe {
                *pnBytes = n;
            }
            unsafe {
                *piStartOffset = iStartOffset;
            }
            unsafe {
                *piEndOffset = unsafe { (*c).iOffset };
            }
            let __v110: *mut simple_tokenizer_cursor = c;
            let __v111: i32 = unsafe { (*__v110).iToken };
            let __v112: i32 = __v111 + (1 as i32);
            unsafe {
                (*__v110).iToken = __v112;
            }
            unsafe {
                *piPosition = __v111;
            }
            return 0 as i32;
        }
    }
    return 101 as i32;
}

/// The set of routines that implement the simple tokenizer
static mut simpleTokenizerModule: sqlite3_tokenizer_module = sqlite3_tokenizer_module {
    iVersion: 0 as i32,
    xCreate: Some(simpleCreate),
    xDestroy: Some(simpleDestroy),
    xOpen: Some(simpleOpen),
    xClose: Some(simpleClose),
    xNext: Some(simpleNext),
    xLanguageid: None,
};

/// Allocate a new simple tokenizer.  Return a pointer to the new
/// tokenizer in *ppModule
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3SimpleTokenizerModule(
    mut ppModule: *mut *const sqlite3_tokenizer_module,
) {
    unsafe {
        *ppModule = unsafe { std::ptr::addr_of!(simpleTokenizerModule) };
    }
}
