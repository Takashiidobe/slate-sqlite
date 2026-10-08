//! 2012 May 24
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
//! Implementation of the "unicode" full-text-search tokenizer.
unsafe extern "C" {
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3_malloc(__v119: i32) -> *mut ();
    fn sqlite3_realloc64(__v120: *mut (), __v121: u64) -> *mut ();
    fn sqlite3_free(__v122: *mut ());
    fn sqlite3FtsUnicodeFold(__v123: i32, __v124: i32) -> i32;
    fn sqlite3FtsUnicodeIsalnum(__v125: i32) -> i32;
    fn sqlite3FtsUnicodeIsdiacritic(__v126: i32) -> i32;
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

// The following two macros - READ_UTF8 and WRITE_UTF8 - have been copied
// from the sqlite3 source file utf.c. If this file is compiled as part
// of the amalgamation, they are not required.
static mut sqlite3Utf8Trans1: __SlateAlign16<[u8; 64]> = __SlateAlign16([
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((13 as i32) as i8) as u8,
    ((14 as i32) as i8) as u8,
    ((15 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((17 as i32) as i8) as u8,
    ((18 as i32) as i8) as u8,
    ((19 as i32) as i8) as u8,
    ((20 as i32) as i8) as u8,
    ((21 as i32) as i8) as u8,
    ((22 as i32) as i8) as u8,
    ((23 as i32) as i8) as u8,
    ((24 as i32) as i8) as u8,
    ((25 as i32) as i8) as u8,
    ((26 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((29 as i32) as i8) as u8,
    ((30 as i32) as i8) as u8,
    ((31 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((13 as i32) as i8) as u8,
    ((14 as i32) as i8) as u8,
    ((15 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
]);

#[repr(C)]
#[derive(Clone, Copy)]
struct unicode_tokenizer {
    base: sqlite3_tokenizer,
    eRemoveDiacritic: i32,
    nException: i32,
    aiException: *mut i32,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

#[repr(C)]
#[derive(Clone, Copy)]
struct unicode_cursor {
    base: sqlite3_tokenizer_cursor,
    /// Input text being tokenized
    aInput: *const u8,
    /// Size of aInput[] in bytes
    nInput: i32,
    /// Current offset within aInput[]
    iOff: i32,
    /// Index of next token to be returned
    iToken: i32,
    /// storage for current token
    zToken: *mut i8,
    /// space allocated at zToken
    nAlloc: i32,
}

/// Destroy a tokenizer allocated by unicodeCreate().
#[unsafe(link_section = ".text.slate_distinct.fts3_unicode.unicodeDestroy")]
extern "C-unwind" fn unicodeDestroy(mut pTokenizer: *mut sqlite3_tokenizer) -> i32 {
    if pTokenizer != std::ptr::null_mut::<sqlite3_tokenizer>() {
        let mut p: *mut unicode_tokenizer = pTokenizer as *mut unicode_tokenizer;
        unsafe { sqlite3_free((unsafe { (*p).aiException }) as *mut ()) };
        unsafe { sqlite3_free(p as *mut ()) };
    }
    return 0 as i32;
}

/// As part of a tokenchars= or separators= option, the CREATE VIRTUAL TABLE
/// statement has specified that the tokenizer for this table shall consider
/// all characters in string zIn/nIn to be separators (if bAlnum==0) or
/// token characters (if bAlnum==1).
///
/// For each codepoint in the zIn/nIn string, this function checks if the
/// sqlite3FtsUnicodeIsalnum() function already returns the desired result.
/// If so, no action is taken. Otherwise, the codepoint is added to the
/// unicode_tokenizer.aiException[] array. For the purposes of tokenization,
/// the return value of sqlite3FtsUnicodeIsalnum() is inverted for all
/// codepoints in the aiException[] array.
///
/// If a standalone diacritic mark (one that sqlite3FtsUnicodeIsdiacritic()
/// identifies as a diacritic) occurs in the zIn/nIn string it is ignored.
/// It is not possible to change the behavior of the tokenizer with respect
/// to these codepoints.
///
/// # Arguments
///
/// * `p` - Tokenizer to add exceptions to
/// * `bAlnum` - Replace Isalnum() return value with this
/// * `zIn` - Array of characters to make exceptions
/// * `nIn` - Length of z in bytes
fn unicodeAddExceptions(
    mut p: *mut unicode_tokenizer,
    mut bAlnum: i32,
    mut zIn: *const i8,
    mut nIn: i32,
) -> i32 {
    let mut z: *const u8 = zIn as *const u8;
    let mut zTerm: *const u8 = unsafe { z.offset(nIn as isize) };
    let mut iCode: u32 = 0 as u32;
    let mut nEntry: i32 = 0 as i32;
    0 as i32;
    '__slate_break_127: while z < zTerm {
        let __v145: *const u8 = z;
        let __v146: *const u8 = unsafe { __v145.offset((1 as i32) as isize) };
        z = __v146;
        iCode = (unsafe { *__v145 }) as u32;
        if iCode >= ((192 as i32) as u32) {
            iCode = (unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3Utf8Trans1.0) as *const u8 }
                        .offset(iCode.wrapping_sub((192 as i32) as u32) as isize)
                }
            }) as u32;
            '__slate_break_128: while z != zTerm
                && (((unsafe { *z }) as u32) as i32) & (192 as i32) == (128 as i32)
            {
                let __v147: *const u8 = z;
                let __v148: *const u8 = unsafe { __v147.offset((1 as i32) as isize) };
                z = __v148;
                iCode = (iCode << (6 as i32))
                    .wrapping_add(((63 as i32) & (((unsafe { *__v147 }) as u32) as i32)) as u32);
            }
            if iCode < ((128 as i32) as u32)
                || iCode & (4294965248 as u32) == ((55296 as i32) as u32)
                || iCode & (4294967294 as u32) == ((65534 as i32) as u32)
            {
                iCode = (65533 as i32) as u32;
            }
        }
        {}
        0 as i32;
        let __v149: bool;
        if (unsafe { sqlite3FtsUnicodeIsalnum(iCode as i32) }) != bAlnum {
            __v149 = (unsafe { sqlite3FtsUnicodeIsdiacritic(iCode as i32) }) == (0 as i32);
        } else {
            __v149 = false as bool;
        }
        if __v149 {
            let __v150: i32 = nEntry;
            let __v151: i32 = __v150 + (1 as i32);
            nEntry = __v151;
        }
    }
    if nEntry != (0 as i32) {
        let mut aNew: *mut i32 = unsafe { std::mem::zeroed() }; // New aiException[] array
        let mut nNew: i32 = 0 as i32; // Number of valid entries in array aNew[]
        aNew = (unsafe {
            sqlite3_realloc64(
                (unsafe { (*p).aiException }) as *mut (),
                ((((unsafe { (*p).nException }) + nEntry) as i64) as u64).wrapping_mul(4 as u64),
            )
        }) as *mut i32;
        if aNew == std::ptr::null_mut::<i32>() {
            return 7 as i32;
        }
        nNew = unsafe { (*p).nException };
        z = zIn as *const u8;
        '__slate_break_129: while z < zTerm {
            let __v152: *const u8 = z;
            let __v153: *const u8 = unsafe { __v152.offset((1 as i32) as isize) };
            z = __v153;
            iCode = (unsafe { *__v152 }) as u32;
            if iCode >= ((192 as i32) as u32) {
                iCode = (unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3Utf8Trans1.0) as *const u8 }
                            .offset(iCode.wrapping_sub((192 as i32) as u32) as isize)
                    }
                }) as u32;
                '__slate_break_130: while z != zTerm
                    && (((unsafe { *z }) as u32) as i32) & (192 as i32) == (128 as i32)
                {
                    let __v154: *const u8 = z;
                    let __v155: *const u8 = unsafe { __v154.offset((1 as i32) as isize) };
                    z = __v155;
                    iCode = (iCode << (6 as i32)).wrapping_add(
                        ((63 as i32) & (((unsafe { *__v154 }) as u32) as i32)) as u32,
                    );
                }
                if iCode < ((128 as i32) as u32)
                    || iCode & (4294965248 as u32) == ((55296 as i32) as u32)
                    || iCode & (4294967294 as u32) == ((65534 as i32) as u32)
                {
                    iCode = (65533 as i32) as u32;
                }
            }
            {}
            let __v156: bool;
            if (unsafe { sqlite3FtsUnicodeIsalnum(iCode as i32) }) != bAlnum {
                __v156 = (unsafe { sqlite3FtsUnicodeIsdiacritic(iCode as i32) }) == (0 as i32);
            } else {
                __v156 = false as bool;
            }
            if __v156 {
                let mut i: i32 = 0 as i32;
                let mut j: i32 = 0 as i32;
                i = 0 as i32;
                '__slate_break_131: loop {
                    if !(i < nNew
                        && (unsafe { *unsafe { aNew.offset(i as isize) } }) < (iCode as i32))
                    {
                        break;
                    }
                    {}
                    let __v157: i32 = i;
                    let __v158: i32 = __v157 + (1 as i32);
                    i = __v158;
                }
                j = nNew;
                '__slate_break_132: loop {
                    if !(j > i) {
                        break;
                    }
                    unsafe {
                        *unsafe { aNew.offset(j as isize) } =
                            unsafe { *unsafe { aNew.offset((j - (1 as i32)) as isize) } };
                    }
                    let __v159: i32 = j;
                    let __v160: i32 = __v159 - (1 as i32);
                    j = __v160;
                }
                unsafe {
                    *unsafe { aNew.offset(i as isize) } = iCode as i32;
                }
                let __v161: i32 = nNew;
                let __v162: i32 = __v161 + (1 as i32);
                nNew = __v162;
            }
        }
        unsafe {
            (*p).aiException = aNew;
        }
        unsafe {
            (*p).nException = nNew;
        }
    }
    return 0 as i32;
}

/// Return true if the p->aiException[] array contains the value iCode.
fn unicodeIsException(mut p: *mut unicode_tokenizer, mut iCode: i32) -> i32 {
    if (unsafe { (*p).nException }) > (0 as i32) {
        let mut a: *mut i32 = unsafe { (*p).aiException };
        let mut iLo: i32 = 0 as i32;
        let mut iHi: i32 = (unsafe { (*p).nException }) - (1 as i32);
        '__slate_break_133: while iHi >= iLo {
            let mut iTest: i32 = (iHi + iLo) / (2 as i32);
            if iCode == unsafe { *unsafe { a.offset(iTest as isize) } } {
                return 1 as i32;
            } else {
                if iCode > unsafe { *unsafe { a.offset(iTest as isize) } } {
                    iLo = iTest + (1 as i32);
                } else {
                    iHi = iTest - (1 as i32);
                }
            }
        }
    }
    return 0 as i32;
}

/// Return true if, for the purposes of tokenization, codepoint iCode is
/// considered a token character (not a separator).
fn unicodeIsAlnum(mut p: *mut unicode_tokenizer, mut iCode: i32) -> i32 {
    0 as i32;
    return (unsafe { sqlite3FtsUnicodeIsalnum(iCode) }) ^ unicodeIsException(p, iCode);
}

/// Create a new tokenizer instance.
///
/// # Arguments
///
/// * `nArg` - Size of array argv[]
/// * `azArg` - Tokenizer creation arguments
/// * `pp` - OUT: New tokenizer handle
#[unsafe(link_section = ".text.slate_distinct.fts3_unicode.unicodeCreate")]
extern "C-unwind" fn unicodeCreate(
    mut nArg: i32,
    mut azArg: *const *const i8,
    mut pp: *mut *mut sqlite3_tokenizer,
) -> i32 {
    let mut pNew: *mut unicode_tokenizer = unsafe { std::mem::zeroed() }; // New tokenizer object
    let mut i: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    pNew = (unsafe { sqlite3_malloc(((24 as u64) as u32) as i32) }) as *mut unicode_tokenizer;
    if pNew == std::ptr::null_mut::<unicode_tokenizer>() {
        return 7 as i32;
    }
    unsafe { memset(pNew as *mut (), 0 as i32, 24 as u64) };
    unsafe {
        (*pNew).eRemoveDiacritic = 1 as i32;
    }
    i = 0 as i32;
    '__slate_break_134: loop {
        if !(rc == (0 as i32) && i < nArg) {
            break;
        }
        let mut z: *const i8 = unsafe { *unsafe { azArg.offset(i as isize) } };
        let mut n: i32 = ((unsafe { strlen(z) }) as u32) as i32;
        if n == (19 as i32)
            && (unsafe {
                memcmp(
                    (b"remove_diacritics=1\0".as_ptr() as *mut i8) as *const (),
                    z as *const (),
                    ((19 as i32) as i64) as u64,
                )
            }) == (0 as i32)
        {
            unsafe {
                (*pNew).eRemoveDiacritic = 1 as i32;
            }
        } else {
            if n == (19 as i32)
                && (unsafe {
                    memcmp(
                        (b"remove_diacritics=0\0".as_ptr() as *mut i8) as *const (),
                        z as *const (),
                        ((19 as i32) as i64) as u64,
                    )
                }) == (0 as i32)
            {
                unsafe {
                    (*pNew).eRemoveDiacritic = 0 as i32;
                }
            } else {
                if n == (19 as i32)
                    && (unsafe {
                        memcmp(
                            (b"remove_diacritics=2\0".as_ptr() as *mut i8) as *const (),
                            z as *const (),
                            ((19 as i32) as i64) as u64,
                        )
                    }) == (0 as i32)
                {
                    unsafe {
                        (*pNew).eRemoveDiacritic = 2 as i32;
                    }
                } else {
                    if n >= (11 as i32)
                        && (unsafe {
                            memcmp(
                                (b"tokenchars=\0".as_ptr() as *mut i8) as *const (),
                                z as *const (),
                                ((11 as i32) as i64) as u64,
                            )
                        }) == (0 as i32)
                    {
                        rc = unicodeAddExceptions(
                            pNew,
                            1 as i32,
                            unsafe { z.offset((11 as i32) as isize) },
                            n - (11 as i32),
                        );
                    } else {
                        if n >= (11 as i32)
                            && (unsafe {
                                memcmp(
                                    (b"separators=\0".as_ptr() as *mut i8) as *const (),
                                    z as *const (),
                                    ((11 as i32) as i64) as u64,
                                )
                            }) == (0 as i32)
                        {
                            rc = unicodeAddExceptions(
                                pNew,
                                0 as i32,
                                unsafe { z.offset((11 as i32) as isize) },
                                n - (11 as i32),
                            );
                        } else {
                            // Unrecognized argument
                            rc = 1 as i32;
                        }
                    }
                }
            }
        }
        let __v163: i32 = i;
        let __v164: i32 = __v163 + (1 as i32);
        i = __v164;
    }
    if rc != (0 as i32) {
        unicodeDestroy(pNew as *mut sqlite3_tokenizer);
        pNew = std::ptr::null_mut::<unicode_tokenizer>();
    }
    unsafe {
        *pp = pNew as *mut sqlite3_tokenizer;
    }
    return rc;
}

/// Prepare to begin tokenizing a particular string.  The input
/// string to be tokenized is pInput[0..nBytes-1].  A cursor
/// used to incrementally tokenize this string is returned in
/// *ppCursor.
///
/// # Arguments
///
/// * `p` - The tokenizer
/// * `aInput` - Input string
/// * `nInput` - Size of string aInput in bytes
/// * `pp` - OUT: New cursor object
#[unsafe(link_section = ".text.slate_distinct.fts3_unicode.unicodeOpen")]
extern "C-unwind" fn unicodeOpen(
    mut p: *mut sqlite3_tokenizer,
    mut aInput: *const i8,
    mut nInput: i32,
    mut pp: *mut *mut sqlite3_tokenizer_cursor,
) -> i32 {
    let mut pCsr: *mut unicode_cursor = unsafe { std::mem::zeroed() };
    pCsr = (unsafe { sqlite3_malloc(((48 as u64) as u32) as i32) }) as *mut unicode_cursor;
    if pCsr == std::ptr::null_mut::<unicode_cursor>() {
        return 7 as i32;
    }
    unsafe { memset(pCsr as *mut (), 0 as i32, 48 as u64) };
    unsafe {
        (*pCsr).aInput = aInput as *const u8;
    }
    if aInput == std::ptr::null::<i8>() {
        unsafe {
            (*pCsr).nInput = 0 as i32;
        }
        unsafe {
            (*pCsr).aInput = (b"\0".as_ptr() as *mut i8) as *const u8;
        }
    } else {
        if nInput < (0 as i32) {
            unsafe {
                (*pCsr).nInput = ((unsafe { strlen(aInput) }) as u32) as i32;
            }
        } else {
            unsafe {
                (*pCsr).nInput = nInput;
            }
        }
    }
    unsafe {
        *pp = unsafe { std::ptr::addr_of_mut!((*pCsr).base) };
    }
    p;
    return 0 as i32;
}

/// Close a tokenization cursor previously opened by a call to
/// simpleOpen() above.
#[unsafe(link_section = ".text.slate_distinct.fts3_unicode.unicodeClose")]
extern "C-unwind" fn unicodeClose(mut pCursor: *mut sqlite3_tokenizer_cursor) -> i32 {
    let mut pCsr: *mut unicode_cursor = pCursor as *mut unicode_cursor;
    unsafe { sqlite3_free((unsafe { (*pCsr).zToken }) as *mut ()) };
    unsafe { sqlite3_free(pCsr as *mut ()) };
    return 0 as i32;
}

/// Extract the next token from a tokenization cursor.  The cursor must
/// have been opened by a prior call to simpleOpen().
///
/// # Arguments
///
/// * `pC` - Cursor returned by simpleOpen
/// * `paToken` - OUT: Token text
/// * `pnToken` - OUT: Number of bytes at *paToken
/// * `piStart` - OUT: Starting offset of token
/// * `piEnd` - OUT: Ending offset of token
/// * `piPos` - OUT: Position integer of token
#[unsafe(link_section = ".text.slate_distinct.fts3_unicode.unicodeNext")]
extern "C-unwind" fn unicodeNext(
    mut pC: *mut sqlite3_tokenizer_cursor,
    mut paToken: *mut *const i8,
    mut pnToken: *mut i32,
    mut piStart: *mut i32,
    mut piEnd: *mut i32,
    mut piPos: *mut i32,
) -> i32 {
    let mut pCsr: *mut unicode_cursor = pC as *mut unicode_cursor;
    let mut p: *mut unicode_tokenizer =
        (unsafe { (*pCsr).base.pTokenizer }) as *mut unicode_tokenizer;
    let mut iCode: u32 = (0 as i32) as u32;
    let mut zOut: *mut i8 = unsafe { std::mem::zeroed() };
    let mut z: *const u8 =
        unsafe { unsafe { (*pCsr).aInput }.offset((unsafe { (*pCsr).iOff }) as isize) };
    let mut zStart: *const u8 = z;
    let mut zEnd: *const u8 = unsafe { std::mem::zeroed() };
    let mut zTerm: *const u8 =
        unsafe { unsafe { (*pCsr).aInput }.offset((unsafe { (*pCsr).nInput }) as isize) };
    // Scan past any delimiter characters before the start of the next token.
    // Return SQLITE_DONE early if this takes us all the way to the end of
    // the input.
    '__slate_break_141: while z < zTerm {
        let __v165: *const u8 = z;
        let __v166: *const u8 = unsafe { __v165.offset((1 as i32) as isize) };
        z = __v166;
        iCode = (unsafe { *__v165 }) as u32;
        if iCode >= ((192 as i32) as u32) {
            iCode = (unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3Utf8Trans1.0) as *const u8 }
                        .offset(iCode.wrapping_sub((192 as i32) as u32) as isize)
                }
            }) as u32;
            '__slate_break_142: while z != zTerm
                && (((unsafe { *z }) as u32) as i32) & (192 as i32) == (128 as i32)
            {
                let __v167: *const u8 = z;
                let __v168: *const u8 = unsafe { __v167.offset((1 as i32) as isize) };
                z = __v168;
                iCode = (iCode << (6 as i32))
                    .wrapping_add(((63 as i32) & (((unsafe { *__v167 }) as u32) as i32)) as u32);
            }
            if iCode < ((128 as i32) as u32)
                || iCode & (4294965248 as u32) == ((55296 as i32) as u32)
                || iCode & (4294967294 as u32) == ((65534 as i32) as u32)
            {
                iCode = (65533 as i32) as u32;
            }
        }
        {}
        if unicodeIsAlnum(p, iCode as i32) != (0 as i32) {
            break '__slate_break_141;
        }
        zStart = z;
    }
    if zStart >= zTerm {
        return 101 as i32;
    }
    zOut = unsafe { (*pCsr).zToken };
    '__slate_break_143: loop {
        let mut iOut: i32 = 0 as i32;
        // Grow the output buffer if required.
        if ((unsafe { zOut.offset_from((unsafe { (*pCsr).zToken }) as *mut i8) }) as i64)
            >= (((unsafe { (*pCsr).nAlloc }) - (4 as i32)) as i64)
        {
            let mut zNew: *mut i8 = (unsafe {
                sqlite3_realloc64(
                    (unsafe { (*pCsr).zToken }) as *mut (),
                    (((unsafe { (*pCsr).nAlloc }) + (64 as i32)) as i64) as u64,
                )
            }) as *mut i8;
            if !(zNew != std::ptr::null_mut::<i8>()) {
                return 7 as i32;
            }
            zOut = unsafe {
                zNew.offset(
                    ((unsafe { zOut.offset_from((unsafe { (*pCsr).zToken }) as *mut i8) }) as i64)
                        as isize,
                )
            };
            unsafe {
                (*pCsr).zToken = zNew;
            }
            let __v169: *mut unicode_cursor = pCsr;
            let __v170: i32 = unsafe { (*__v169).nAlloc };
            let __v171: i32 = __v170 + (64 as i32);
            unsafe {
                (*__v169).nAlloc = __v171;
            }
        }
        // Write the folded case of the last character read to the output
        zEnd = z;
        iOut = unsafe { sqlite3FtsUnicodeFold(iCode as i32, unsafe { (*p).eRemoveDiacritic }) };
        if iOut != (0 as i32) {
            if iOut < (128 as i32) {
                let __v172: *mut i8 = zOut;
                let __v173: *mut i8 = unsafe { __v172.offset((1 as i32) as isize) };
                zOut = __v173;
                unsafe {
                    *__v172 = (((iOut & (255 as i32)) as i8) as u8) as i8;
                }
            } else {
                if iOut < (2048 as i32) {
                    let __v174: *mut i8 = zOut;
                    let __v175: *mut i8 = unsafe { __v174.offset((1 as i32) as isize) };
                    zOut = __v175;
                    unsafe {
                        *__v174 = ((192 as i32)
                            + (((((iOut >> (6 as i32) & (31 as i32)) as i8) as u8) as u32) as i32))
                            as i8;
                    }
                    let __v176: *mut i8 = zOut;
                    let __v177: *mut i8 = unsafe { __v176.offset((1 as i32) as isize) };
                    zOut = __v177;
                    unsafe {
                        *__v176 = ((128 as i32)
                            + (((((iOut & (63 as i32)) as i8) as u8) as u32) as i32))
                            as i8;
                    }
                } else {
                    if iOut < (65536 as i32) {
                        let __v178: *mut i8 = zOut;
                        let __v179: *mut i8 = unsafe { __v178.offset((1 as i32) as isize) };
                        zOut = __v179;
                        unsafe {
                            *__v178 = ((224 as i32)
                                + (((((iOut >> (12 as i32) & (15 as i32)) as i8) as u8) as u32)
                                    as i32)) as i8;
                        }
                        let __v180: *mut i8 = zOut;
                        let __v181: *mut i8 = unsafe { __v180.offset((1 as i32) as isize) };
                        zOut = __v181;
                        unsafe {
                            *__v180 = ((128 as i32)
                                + (((((iOut >> (6 as i32) & (63 as i32)) as i8) as u8) as u32)
                                    as i32)) as i8;
                        }
                        let __v182: *mut i8 = zOut;
                        let __v183: *mut i8 = unsafe { __v182.offset((1 as i32) as isize) };
                        zOut = __v183;
                        unsafe {
                            *__v182 = ((128 as i32)
                                + (((((iOut & (63 as i32)) as i8) as u8) as u32) as i32))
                                as i8;
                        }
                    } else {
                        let __v184: *mut i8 = zOut;
                        let __v185: *mut i8 = unsafe { __v184.offset((1 as i32) as isize) };
                        zOut = __v185;
                        unsafe {
                            *__v184 = ((240 as i32)
                                + (((((iOut >> (18 as i32) & (7 as i32)) as i8) as u8) as u32)
                                    as i32)) as i8;
                        }
                        let __v186: *mut i8 = zOut;
                        let __v187: *mut i8 = unsafe { __v186.offset((1 as i32) as isize) };
                        zOut = __v187;
                        unsafe {
                            *__v186 = ((128 as i32)
                                + (((((iOut >> (12 as i32) & (63 as i32)) as i8) as u8) as u32)
                                    as i32)) as i8;
                        }
                        let __v188: *mut i8 = zOut;
                        let __v189: *mut i8 = unsafe { __v188.offset((1 as i32) as isize) };
                        zOut = __v189;
                        unsafe {
                            *__v188 = ((128 as i32)
                                + (((((iOut >> (6 as i32) & (63 as i32)) as i8) as u8) as u32)
                                    as i32)) as i8;
                        }
                        let __v190: *mut i8 = zOut;
                        let __v191: *mut i8 = unsafe { __v190.offset((1 as i32) as isize) };
                        zOut = __v191;
                        unsafe {
                            *__v190 = ((128 as i32)
                                + (((((iOut & (63 as i32)) as i8) as u8) as u32) as i32))
                                as i8;
                        }
                    }
                }
            }
            {}
        }
        // If the cursor is not at EOF, read the next character
        if z >= zTerm {
            break '__slate_break_143;
        }
        let __v192: *const u8 = z;
        let __v193: *const u8 = unsafe { __v192.offset((1 as i32) as isize) };
        z = __v193;
        iCode = (unsafe { *__v192 }) as u32;
        if iCode >= ((192 as i32) as u32) {
            iCode = (unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3Utf8Trans1.0) as *const u8 }
                        .offset(iCode.wrapping_sub((192 as i32) as u32) as isize)
                }
            }) as u32;
            '__slate_break_144: while z != zTerm
                && (((unsafe { *z }) as u32) as i32) & (192 as i32) == (128 as i32)
            {
                let __v194: *const u8 = z;
                let __v195: *const u8 = unsafe { __v194.offset((1 as i32) as isize) };
                z = __v195;
                iCode = (iCode << (6 as i32))
                    .wrapping_add(((63 as i32) & (((unsafe { *__v194 }) as u32) as i32)) as u32);
            }
            if iCode < ((128 as i32) as u32)
                || iCode & (4294965248 as u32) == ((55296 as i32) as u32)
                || iCode & (4294967294 as u32) == ((65534 as i32) as u32)
            {
                iCode = (65533 as i32) as u32;
            }
        }
        {}
        let __v196: bool;
        if unicodeIsAlnum(p, iCode as i32) != (0 as i32) {
            __v196 = true as bool;
        } else {
            __v196 = (unsafe { sqlite3FtsUnicodeIsdiacritic(iCode as i32) }) != (0 as i32);
        }
        if !__v196 {
            break;
        }
    }
    // Set the output variables and return.
    unsafe {
        (*pCsr).iOff =
            ((unsafe { z.offset_from((unsafe { (*pCsr).aInput }) as *const u8) }) as i64) as i32;
    }
    unsafe {
        *paToken = (unsafe { (*pCsr).zToken }) as *const i8;
    }
    unsafe {
        *pnToken =
            ((unsafe { zOut.offset_from((unsafe { (*pCsr).zToken }) as *mut i8) }) as i64) as i32;
    }
    unsafe {
        *piStart = ((unsafe { zStart.offset_from((unsafe { (*pCsr).aInput }) as *const u8) })
            as i64) as i32;
    }
    unsafe {
        *piEnd =
            ((unsafe { zEnd.offset_from((unsafe { (*pCsr).aInput }) as *const u8) }) as i64) as i32;
    }
    let __v197: *mut unicode_cursor = pCsr;
    let __v198: i32 = unsafe { (*__v197).iToken };
    let __v199: i32 = __v198 + (1 as i32);
    unsafe {
        (*__v197).iToken = __v199;
    }
    unsafe {
        *piPos = __v198;
    }
    return 0 as i32;
}

/// Set *ppModule to a pointer to the sqlite3_tokenizer_module
/// structure for the unicode tokenizer.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3UnicodeTokenizer(
    mut ppModule: *mut *const sqlite3_tokenizer_module,
) {
    unsafe {
        *ppModule = unsafe { std::ptr::addr_of!(module) };
    }
}

static mut module: sqlite3_tokenizer_module = sqlite3_tokenizer_module {
    iVersion: 0 as i32,
    xCreate: Some(unicodeCreate),
    xDestroy: Some(unicodeDestroy),
    xOpen: Some(unicodeOpen),
    xClose: Some(unicodeClose),
    xNext: Some(unicodeNext),
    xLanguageid: None,
};
