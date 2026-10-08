//! 2008 Nov 28
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
//! This module contains code that implements a parser for fts3 query strings
//! (the right-hand argument to the MATCH operator). Because the supported
//! syntax is relatively simple, the whole tokenizer/parser system is
//! hand-coded.
unsafe extern "C" {
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3_malloc64(__v251: u64) -> *mut ();
    fn sqlite3_realloc64(__v252: *mut (), __v253: u64) -> *mut ();
    fn sqlite3_free(__v254: *mut ());
    fn sqlite3_strnicmp(__v255: *const i8, __v256: *const i8, __v257: i32) -> i32;
    fn sqlite3Fts3ErrMsg(__v258: *mut *mut i8, __v259: *const i8, ...);
    fn sqlite3Fts3ReadInt(z: *const i8, pnOut: *mut i32) -> i32;
    fn sqlite3Fts3EvalPhraseCleanup(__v279: *mut Fts3Phrase);
}

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
struct Fts3DeferredToken {}

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

// By default, this module parses the legacy syntax that has been
// traditionally used by fts3. Or, if SQLITE_ENABLE_FTS3_PARENTHESIS
// is defined, then it uses the new syntax. The differences between
// the new and the old syntaxes are:
//
//  a) The new syntax supports parenthesis. The old does not.
//
//  b) The new syntax supports the AND and NOT operators. The old does not.
//
//  c) The old syntax supports the "-" token qualifier. This is not
//     supported by the new syntax (it is replaced by the NOT operator).
//
//  d) When using the old syntax, the OR operator has a greater precedence
//     than an implicit AND. When using the new, both implicity and explicit
//     AND operators have a higher precedence than OR.
//
// If compiled with SQLITE_TEST defined, then this module exports the
// symbol "int sqlite3_fts3_enable_parentheses". Setting this variable
// to zero causes the module to use the old syntax. If it is set to
// non-zero the new syntax is activated. This is so both syntaxes can
// be tested using a single build of testfixture.
//
// The following describes the syntax supported by the fts3 MATCH
// operator in a similar format to that used by the lemon parser
// generator. This module does not use actually lemon, it uses a
// custom parser.
//
//   query ::= andexpr (OR andexpr)*.
//
//   andexpr ::= notexpr (AND? notexpr)*.
//
//   notexpr ::= nearexpr (NOT nearexpr|-TOKEN)*.
//   notexpr ::= LP query RP.
//
//   nearexpr ::= phrase (NEAR distance_opt nearexpr)*.
//
//   distance_opt ::= .
//   distance_opt ::= / INTEGER.
//
//   phrase ::= TOKEN.
//   phrase ::= COLUMN:TOKEN.
//   phrase ::= "TOKEN TOKEN TOKEN...".
// Default span for NEAR operators.
/// isNot:
///   This variable is used by function getNextNode(). When getNextNode() is
///   called, it sets ParseContext.isNot to true if the 'next node' is a
///   FTSQUERY_PHRASE with a unary "-" attached to it. i.e. "mysql" in the
///   FTS3 query "sqlite -mysql". Otherwise, ParseContext.isNot is set to
///   zero.
#[repr(C)]
#[derive(Clone, Copy)]
struct ParseContext {
    /// Tokenizer module
    pTokenizer: *mut sqlite3_tokenizer,
    /// Language id used with tokenizer
    iLangid: i32,
    /// Array of column names for fts3 table
    azCol: *mut *const i8,
    /// True to allow FTS4-only syntax
    bFts4: i32,
    /// Number of entries in azCol[]
    nCol: i32,
    /// Default column to query
    iDefaultCol: i32,
    /// True if getNextNode() sees a unary -
    isNot: i32,
    /// Write error message here
    pCtx: *mut sqlite3_context,
    /// Number of nested brackets
    nNest: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Keyword {
    /// Keyword text
    z: *mut i8,
    /// Length of the keyword
    n: u8,
    /// Only valid in paren mode
    parenOnly: u8,
    /// Keyword code
    eType: u8,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

/// This function is equivalent to the standard isspace() function.
///
/// The standard isspace() can be awkward to use safely, because although it
/// is defined to accept an argument of type int, its behavior when passed
/// an integer that falls outside of the range of the unsigned char type
/// is undefined (and sometimes, "undefined" means segfault). This wrapper
/// is defined to accept an argument of type char, and always returns 0 for
/// any values that fall outside of the range of the unsigned char type (i.e.
/// negative values).
fn fts3isspace(mut c: i8) -> i32 {
    return ((c as i32) == (32 as i32)
        || (c as i32) == (9 as i32)
        || (c as i32) == (10 as i32)
        || (c as i32) == (13 as i32)
        || (c as i32) == (11 as i32)
        || (c as i32) == (12 as i32)) as i32;
}

/// Allocate nByte bytes of memory using sqlite3_malloc(). If successful,
/// zero the memory before returning a pointer to it. If unsuccessful,
/// return NULL.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3MallocZero(mut nByte: i64) -> *mut () {
    let mut pRet: *mut () = unsafe { sqlite3_malloc64(nByte as u64) };
    if pRet != std::ptr::null_mut::<()>() {
        unsafe { memset(pRet, 0 as i32, nByte as u64) };
    }
    return pRet;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3OpenTokenizer(
    mut pTokenizer: *mut sqlite3_tokenizer,
    mut iLangid: i32,
    mut z: *const i8,
    mut n: i32,
    mut ppCsr: *mut *mut sqlite3_tokenizer_cursor,
) -> i32 {
    let mut pModule: *const sqlite3_tokenizer_module = unsafe { (*pTokenizer).pModule };
    let mut pCsr: *mut sqlite3_tokenizer_cursor = std::ptr::null_mut::<sqlite3_tokenizer_cursor>();
    let mut rc: i32 = 0 as i32;
    rc = unsafe {
        unsafe { (*pModule).xOpen }.unwrap()(pTokenizer, z, n, std::ptr::addr_of_mut!(pCsr))
    };
    0 as i32;
    if rc == (0 as i32) {
        unsafe {
            (*pCsr).pTokenizer = pTokenizer;
        }
        if (unsafe { (*pModule).iVersion }) >= (1 as i32) {
            rc = unsafe { unsafe { (*pModule).xLanguageid }.unwrap()(pCsr, iLangid) };
            if rc != (0 as i32) {
                unsafe { unsafe { (*pModule).xClose }.unwrap()(pCsr) };
                pCsr = std::ptr::null_mut::<sqlite3_tokenizer_cursor>();
            }
        }
    }
    unsafe {
        *ppCsr = pCsr;
    }
    return rc;
}

/// Search buffer z[], size n, for a '"' character. Or, if enable_parenthesis
/// is defined, search for '(' and ')' as well. Return the index of the first
/// such character in the buffer. If there is no such character, return -1.
fn findBarredChar(mut z: *const i8, mut n: i32) -> i32 {
    let mut ii: i32 = 0 as i32;
    ii = 0 as i32;
    '__slate_break_285: loop {
        if !(ii < n) {
            break;
        }
        if ((unsafe { *unsafe { z.offset(ii as isize) } }) as i32) == (34 as i32)
            || (0 as i32) != (0 as i32)
                && (((unsafe { *unsafe { z.offset(ii as isize) } }) as i32) == (40 as i32)
                    || ((unsafe { *unsafe { z.offset(ii as isize) } }) as i32) == (41 as i32))
        {
            return ii;
        }
        let __v316: i32 = ii;
        let __v317: i32 = __v316 + (1 as i32);
        ii = __v317;
    }
    return -(1 as i32);
}

/// Extract the next token from buffer z (length n) using the tokenizer
/// and other information (column names etc.) in pParse. Create an Fts3Expr
/// structure of type FTSQUERY_PHRASE containing a phrase consisting of this
/// single token and set *ppExpr to point to it. If the end of the buffer is
/// reached before a token is found, set *ppExpr to zero. It is the
/// responsibility of the caller to eventually deallocate the allocated
/// Fts3Expr structure (if any) by passing it to sqlite3_free().
///
/// Return SQLITE_OK if successful, or SQLITE_NOMEM if a memory allocation
/// fails.
///
/// # Arguments
///
/// * `pParse` - fts3 query parse context
/// * `iCol` - Value for Fts3Phrase.iColumn
/// * `n` - Input string
/// * `ppExpr` - OUT: expression
/// * `pnConsumed` - OUT: Number of bytes consumed
fn getNextToken(
    mut pParse: *mut ParseContext,
    mut iCol: i32,
    mut z: *const i8,
    mut n: i32,
    mut ppExpr: *mut *mut Fts3Expr,
    mut pnConsumed: *mut i32,
) -> i32 {
    let mut pTokenizer: *mut sqlite3_tokenizer = unsafe { (*pParse).pTokenizer };
    let mut pModule: *const sqlite3_tokenizer_module = unsafe { (*pTokenizer).pModule };
    let mut rc: i32 = 0 as i32;
    let mut pCursor: *mut sqlite3_tokenizer_cursor = unsafe { std::mem::zeroed() };
    let mut pRet: *mut Fts3Expr = std::ptr::null_mut::<Fts3Expr>();
    unsafe {
        *pnConsumed = n;
    }
    rc = sqlite3Fts3OpenTokenizer(
        pTokenizer,
        unsafe { (*pParse).iLangid },
        z,
        n,
        std::ptr::addr_of_mut!(pCursor),
    );
    if rc == (0 as i32) {
        let mut zToken: *const i8 = unsafe { std::mem::zeroed() };
        let mut nToken: i32 = 0 as i32;
        let mut iStart: i32 = 0 as i32;
        let mut iEnd: i32 = 0 as i32;
        let mut iPosition: i32 = 0 as i32;
        let mut nByte: i64 = 0 as i64; // total space to allocate
        rc = unsafe {
            unsafe { (*pModule).xNext }.unwrap()(
                pCursor,
                std::ptr::addr_of_mut!(zToken),
                std::ptr::addr_of_mut!(nToken),
                std::ptr::addr_of_mut!(iStart),
                std::ptr::addr_of_mut!(iEnd),
                std::ptr::addr_of_mut!(iPosition),
            )
        };
        if rc == (0 as i32) {
            // Check that this tokenization did not gobble up any " characters. Or,
            // if enable_parenthesis is true, that it did not gobble up any
            // open or close parenthesis characters either. If it did, call
            // getNextToken() again, but pass only that part of the input buffer
            // up to the first such character.
            let mut iBarred: i32 = findBarredChar(z, iEnd);
            if iBarred >= (0 as i32) {
                unsafe { unsafe { (*pModule).xClose }.unwrap()(pCursor) };
                return getNextToken(pParse, iCol, z, iBarred, ppExpr, pnConsumed);
            }
            nByte = (64 as u64)
                .wrapping_add(
                    (88 as u64).wrapping_add((((1 as i32) as i64) as u64).wrapping_mul(40 as u64)),
                )
                .wrapping_add((nToken as i64) as u64) as i64;
            pRet = sqlite3Fts3MallocZero(nByte) as *mut Fts3Expr;
            if !(pRet != std::ptr::null_mut::<Fts3Expr>()) {
                rc = 7 as i32;
            } else {
                unsafe {
                    (*pRet).eType = 5 as i32;
                }
                unsafe {
                    (*pRet).pPhrase =
                        (unsafe { pRet.offset((1 as i32) as isize) }) as *mut Fts3Phrase;
                }
                unsafe {
                    (*unsafe { (*pRet).pPhrase }).nToken = 1 as i32;
                }
                unsafe {
                    (*unsafe { (*pRet).pPhrase }).iColumn = iCol;
                }
                unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*pRet).pPhrase }).aToken)
                                as *mut Fts3PhraseToken
                        }
                        .offset((0 as i32) as isize)
                    })
                    .n = nToken;
                }
                unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*pRet).pPhrase }).aToken)
                                as *mut Fts3PhraseToken
                        }
                        .offset((0 as i32) as isize)
                    })
                    .z = (unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*pRet).pPhrase }).aToken)
                                as *mut Fts3PhraseToken
                        }
                        .offset((1 as i32) as isize)
                    }) as *mut i8;
                }
                unsafe {
                    memcpy(
                        (unsafe {
                            (*unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*unsafe { (*pRet).pPhrase }).aToken)
                                        as *mut Fts3PhraseToken
                                }
                                .offset((0 as i32) as isize)
                            })
                            .z
                        }) as *mut (),
                        zToken as *const (),
                        (nToken as i64) as u64,
                    )
                };
                if iEnd < n
                    && ((unsafe { *unsafe { z.offset(iEnd as isize) } }) as i32) == (42 as i32)
                {
                    unsafe {
                        (*unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!((*unsafe { (*pRet).pPhrase }).aToken)
                                    as *mut Fts3PhraseToken
                            }
                            .offset((0 as i32) as isize)
                        })
                        .isPrefix = 1 as i32;
                    }
                    let __v318: i32 = iEnd;
                    let __v319: i32 = __v318 + (1 as i32);
                    iEnd = __v319;
                }
                '__slate_break_286: while (1 as i32) != (0 as i32) {
                    if !((0 as i32) != (0 as i32))
                        && iStart > (0 as i32)
                        && ((unsafe { *unsafe { z.offset((iStart - (1 as i32)) as isize) } })
                            as i32)
                            == (45 as i32)
                    {
                        unsafe {
                            (*pParse).isNot = 1 as i32;
                        }
                        let __v320: i32 = iStart;
                        let __v321: i32 = __v320 - (1 as i32);
                        iStart = __v321;
                    } else {
                        if (unsafe { (*pParse).bFts4 }) != (0 as i32)
                            && iStart > (0 as i32)
                            && ((unsafe { *unsafe { z.offset((iStart - (1 as i32)) as isize) } })
                                as i32)
                                == (94 as i32)
                        {
                            unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*unsafe { (*pRet).pPhrase }).aToken)
                                            as *mut Fts3PhraseToken
                                    }
                                    .offset((0 as i32) as isize)
                                })
                                .bFirst = 1 as i32;
                            }
                            let __v322: i32 = iStart;
                            let __v323: i32 = __v322 - (1 as i32);
                            iStart = __v323;
                        } else {
                            break '__slate_break_286;
                        }
                    }
                }
            }
            unsafe {
                *pnConsumed = iEnd;
            }
        } else {
            if n != (0 as i32) && rc == (101 as i32) {
                let mut iBarred: i32 = findBarredChar(z, n);
                if iBarred >= (0 as i32) {
                    unsafe {
                        *pnConsumed = iBarred;
                    }
                }
                rc = 0 as i32;
            }
        }
        unsafe { unsafe { (*pModule).xClose }.unwrap()(pCursor) };
    }
    unsafe {
        *ppExpr = pRet;
    }
    return rc;
}

/// Enlarge a memory allocation.  If an out-of-memory allocation occurs,
/// then free the old allocation.
fn fts3ReallocOrFree(mut pOrig: *mut (), mut nNew: i64) -> *mut () {
    let mut pRet: *mut () = unsafe { sqlite3_realloc64(pOrig, nNew as u64) };
    if !(pRet != std::ptr::null_mut::<()>()) {
        unsafe { sqlite3_free(pOrig) };
    }
    return pRet;
}

/// Buffer zInput, length nInput, contains the contents of a quoted string
/// that appeared as part of an fts3 query expression. Neither quote character
/// is included in the buffer. This function attempts to tokenize the entire
/// input buffer and create an Fts3Expr structure of type FTSQUERY_PHRASE
/// containing the results.
///
/// If successful, SQLITE_OK is returned and *ppExpr set to point at the
/// allocated Fts3Expr structure. Otherwise, either SQLITE_NOMEM (out of memory
/// error) or SQLITE_ERROR (tokenization error) is returned and *ppExpr set
/// to 0.
///
/// # Arguments
///
/// * `pParse` - fts3 query parse context
/// * `nInput` - Input string
/// * `ppExpr` - OUT: expression
fn getNextString(
    mut pParse: *mut ParseContext,
    mut zInput: *const i8,
    mut nInput: i32,
    mut ppExpr: *mut *mut Fts3Expr,
) -> i32 {
    let mut __slate_storage_329: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_329: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_329) as *mut i32;
    let mut __slate_storage_328: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_328: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_328) as *mut i32;
    let mut __slate_storage_331: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_331: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_331) as *mut *mut i8;
    let mut __slate_storage_330: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_330: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_330) as *mut *mut i8;
    let mut __slate_storage_143: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_143: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_143) as *mut *mut i8;
    let mut __slate_storage_142: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_142: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_142) as *mut i32;
    let mut __slate_storage_325: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_325: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_325) as *mut i32;
    let mut __slate_storage_324: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_324: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_324) as *mut i32;
    let mut __slate_storage_327: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_327: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_327) as *mut i64;
    let mut __slate_storage_326: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_326: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_326) as *mut i64;
    let mut __slate_storage_141: std::mem::MaybeUninit<*mut Fts3PhraseToken> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_141: *mut *mut Fts3PhraseToken =
        std::ptr::addr_of_mut!(__slate_storage_141) as *mut *mut Fts3PhraseToken;
    let mut __slate_storage_140: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_140: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_140) as *mut i32;
    let mut __slate_storage_139: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_139: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_139) as *mut i32;
    let mut __slate_storage_138: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_138: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_138) as *mut i32;
    let mut __slate_storage_137: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_137: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_137) as *mut i32;
    let mut __slate_storage_136: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_136: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_136) as *mut *const i8;
    let mut __slate_storage_135: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_135: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_135) as *mut i32;
    let mut __slate_storage_134: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_134: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_134) as *mut i32;
    let mut __slate_storage_133: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_133: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_133) as *mut i32;
    let mut __slate_storage_132: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_132: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_132) as *mut i64;
    let mut __slate_storage_131: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_131: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_131) as *mut *mut i8;
    let mut __slate_storage_130: std::mem::MaybeUninit<*mut sqlite3_tokenizer_cursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_130: *mut *mut sqlite3_tokenizer_cursor =
        std::ptr::addr_of_mut!(__slate_storage_130) as *mut *mut sqlite3_tokenizer_cursor;
    let mut __slate_storage_129: std::mem::MaybeUninit<*mut Fts3Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_129: *mut *mut Fts3Expr =
        std::ptr::addr_of_mut!(__slate_storage_129) as *mut *mut Fts3Expr;
    let mut __slate_storage_128: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_128: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_128) as *mut i32;
    let mut __slate_storage_127: std::mem::MaybeUninit<*const sqlite3_tokenizer_module> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_127: *mut *const sqlite3_tokenizer_module =
        std::ptr::addr_of_mut!(__slate_storage_127) as *mut *const sqlite3_tokenizer_module;
    let mut __slate_storage_126: std::mem::MaybeUninit<*mut sqlite3_tokenizer> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_126: *mut *mut sqlite3_tokenizer =
        std::ptr::addr_of_mut!(__slate_storage_126) as *mut *mut sqlite3_tokenizer;
    unsafe {
        '__join_4: {
            '__join_13: {
                std::ptr::write(__slate_slot_126, unsafe { (*pParse).pTokenizer });
                std::ptr::write(__slate_slot_127, unsafe { (*(*__slate_slot_126)).pModule });
                std::ptr::write(__slate_slot_129, std::ptr::null_mut::<Fts3Expr>());
                std::ptr::write(
                    __slate_slot_130,
                    std::ptr::null_mut::<sqlite3_tokenizer_cursor>(),
                );
                std::ptr::write(__slate_slot_131, std::ptr::null_mut::<i8>());
                std::ptr::write(__slate_slot_132, (0 as i32) as i64);
                std::ptr::write(
                    __slate_slot_133,
                    ((64 as u64).wrapping_add(
                        (88 as u64)
                            .wrapping_add((((1 as i32) as i64) as u64).wrapping_mul(40 as u64)),
                    ) as u32) as i32,
                );
                std::ptr::write(__slate_slot_134, 0 as i32);
                // The final Fts3Expr data structure, including the Fts3Phrase,
                // Fts3PhraseToken structures token buffers are all stored as a single
                // allocation so that the expression can be freed with a single call to
                // sqlite3_free(). Setting this up requires a two pass approach.
                //
                // The first pass, in the block below, uses a tokenizer cursor to iterate
                // through the tokens in the expression. This pass uses fts3ReallocOrFree()
                // to assemble data in two dynamic buffers:
                //
                //   Buffer p: Points to the Fts3Expr structure, followed by the Fts3Phrase
                //             structure, followed by the array of Fts3PhraseToken
                //             structures. This pass only populates the Fts3PhraseToken array.
                //
                //   Buffer zTemp: Contains copies of all tokens.
                //
                // The second pass, in the block that begins "if( rc==SQLITE_DONE )" below,
                // appends buffer zTemp to buffer p, and fills in the Fts3Expr and Fts3Phrase
                // structures.
                *__slate_slot_128 = sqlite3Fts3OpenTokenizer(
                    *__slate_slot_126,
                    unsafe { (*pParse).iLangid },
                    zInput,
                    nInput,
                    std::ptr::addr_of_mut!(*__slate_slot_130),
                );
                if *__slate_slot_128 == (0 as i32) {
                    *__slate_slot_135 = 0 as i32;
                    '__loop_14: loop {
                        if *__slate_slot_128 == (0 as i32) {
                            std::ptr::write(__slate_slot_137, 0 as i32);
                            std::ptr::write(__slate_slot_138, 0 as i32);
                            std::ptr::write(__slate_slot_139, 0 as i32);
                            std::ptr::write(__slate_slot_140, 0 as i32);
                            *__slate_slot_128 = unsafe {
                                unsafe { (*(*__slate_slot_127)).xNext }.unwrap()(
                                    *__slate_slot_130,
                                    std::ptr::addr_of_mut!(*__slate_slot_136),
                                    std::ptr::addr_of_mut!(*__slate_slot_137),
                                    std::ptr::addr_of_mut!(*__slate_slot_138),
                                    std::ptr::addr_of_mut!(*__slate_slot_139),
                                    std::ptr::addr_of_mut!(*__slate_slot_140),
                                )
                            };
                            if *__slate_slot_128 == (0 as i32) {
                                *__slate_slot_129 = fts3ReallocOrFree(
                                    *__slate_slot_129 as *mut (),
                                    ((*__slate_slot_133 as i64) as u64).wrapping_add(
                                        ((*__slate_slot_135 as i64) as u64).wrapping_mul(40 as u64),
                                    ) as i64,
                                )
                                    as *mut Fts3Expr;
                                *__slate_slot_131 = fts3ReallocOrFree(
                                    *__slate_slot_131 as *mut (),
                                    *__slate_slot_132 + (*__slate_slot_137 as i64),
                                ) as *mut i8;
                                if !(*__slate_slot_131 != std::ptr::null_mut::<i8>())
                                    || !(*__slate_slot_129 != std::ptr::null_mut::<Fts3Expr>())
                                {
                                    break '__loop_14;
                                } else {
                                    0 as i32;
                                    *__slate_slot_141 = unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!(
                                                (*((unsafe {
                                                    (*__slate_slot_129).offset((1 as i32) as isize)
                                                })
                                                    as *mut Fts3Phrase))
                                                    .aToken
                                            )
                                                as *mut Fts3PhraseToken
                                        }
                                        .offset(*__slate_slot_135 as isize)
                                    };
                                    unsafe {
                                        memset(*__slate_slot_141 as *mut (), 0 as i32, 40 as u64)
                                    };
                                    unsafe {
                                        memcpy(
                                            (unsafe {
                                                (*__slate_slot_131)
                                                    .offset(*__slate_slot_132 as isize)
                                            })
                                                as *mut (),
                                            *__slate_slot_136 as *const (),
                                            (*__slate_slot_137 as i64) as u64,
                                        )
                                    };
                                    std::ptr::write(__slate_slot_326, *__slate_slot_132);
                                    std::ptr::write(
                                        __slate_slot_327,
                                        *__slate_slot_326 + (*__slate_slot_137 as i64),
                                    );
                                    *__slate_slot_132 = *__slate_slot_327;
                                    unsafe {
                                        (*(*__slate_slot_141)).n = *__slate_slot_137;
                                    }
                                    unsafe {
                                        (*(*__slate_slot_141)).isPrefix = (*__slate_slot_139
                                            < nInput
                                            && ((unsafe {
                                                *unsafe {
                                                    zInput.offset(*__slate_slot_139 as isize)
                                                }
                                            })
                                                as i32)
                                                == (42 as i32))
                                            as i32;
                                    }
                                    unsafe {
                                        (*(*__slate_slot_141)).bFirst = (*__slate_slot_138
                                            > (0 as i32)
                                            && ((unsafe {
                                                *unsafe {
                                                    zInput.offset(
                                                        (*__slate_slot_138 - (1 as i32)) as isize,
                                                    )
                                                }
                                            })
                                                as i32)
                                                == (94 as i32))
                                            as i32;
                                    }
                                    *__slate_slot_134 = *__slate_slot_135 + (1 as i32);
                                }
                            }
                            std::ptr::write(__slate_slot_324, *__slate_slot_135);
                            std::ptr::write(__slate_slot_325, *__slate_slot_324 + (1 as i32));
                            *__slate_slot_135 = *__slate_slot_325;
                        } else {
                            break '__join_13;
                        }
                    }
                    *__slate_slot_128 = 7 as i32;
                    break '__join_4;
                }
            }
            if *__slate_slot_128 == (101 as i32) {
                std::ptr::write(__slate_slot_143, std::ptr::null_mut::<i8>());
                *__slate_slot_129 = fts3ReallocOrFree(
                    *__slate_slot_129 as *mut (),
                    ((*__slate_slot_133 as i64) as u64)
                        .wrapping_add(((*__slate_slot_134 as i64) as u64).wrapping_mul(40 as u64))
                        .wrapping_add(*__slate_slot_132 as u64) as i64,
                ) as *mut Fts3Expr;
                if !(*__slate_slot_129 != std::ptr::null_mut::<Fts3Expr>()) {
                    *__slate_slot_128 = 7 as i32;
                } else {
                    unsafe {
                        memset(
                            *__slate_slot_129 as *mut (),
                            0 as i32,
                            ((unsafe {
                                ((unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!(
                                            (*((unsafe {
                                                (*__slate_slot_129).offset((1 as i32) as isize)
                                            })
                                                as *mut Fts3Phrase))
                                                .aToken
                                        )
                                            as *mut Fts3PhraseToken
                                    }
                                    .offset((0 as i32) as isize)
                                }) as *mut i8)
                                    .offset_from((*__slate_slot_129 as *mut i8) as *mut i8)
                            }) as i64) as u64,
                        )
                    };
                    unsafe {
                        (*(*__slate_slot_129)).eType = 5 as i32;
                    }
                    unsafe {
                        (*(*__slate_slot_129)).pPhrase =
                            (unsafe { (*__slate_slot_129).offset((1 as i32) as isize) })
                                as *mut Fts3Phrase;
                    }
                    unsafe {
                        (*unsafe { (*(*__slate_slot_129)).pPhrase }).iColumn =
                            unsafe { (*pParse).iDefaultCol };
                    }
                    unsafe {
                        (*unsafe { (*(*__slate_slot_129)).pPhrase }).nToken = *__slate_slot_134;
                    }
                    *__slate_slot_143 = (unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!(
                                (*unsafe { (*(*__slate_slot_129)).pPhrase }).aToken
                            ) as *mut Fts3PhraseToken
                        }
                        .offset(*__slate_slot_134 as isize)
                    }) as *mut i8;
                    0 as i32;
                    if *__slate_slot_131 != std::ptr::null_mut::<i8>() {
                        unsafe {
                            memcpy(
                                *__slate_slot_143 as *mut (),
                                *__slate_slot_131 as *const (),
                                *__slate_slot_132 as u64,
                            )
                        };
                    }
                    *__slate_slot_142 = 0 as i32;
                    loop {
                        if *__slate_slot_142
                            < unsafe { (*unsafe { (*(*__slate_slot_129)).pPhrase }).nToken }
                        {
                            unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!(
                                            (*unsafe { (*(*__slate_slot_129)).pPhrase }).aToken
                                        )
                                            as *mut Fts3PhraseToken
                                    }
                                    .offset(*__slate_slot_142 as isize)
                                })
                                .z = *__slate_slot_143;
                            }
                            std::ptr::write(__slate_slot_330, *__slate_slot_143);
                            std::ptr::write(__slate_slot_331, unsafe {
                                (*__slate_slot_330).offset(
                                    (unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!(
                                                    (*unsafe { (*(*__slate_slot_129)).pPhrase })
                                                        .aToken
                                                )
                                                    as *mut Fts3PhraseToken
                                            }
                                            .offset(*__slate_slot_142 as isize)
                                        })
                                        .n
                                    }) as isize,
                                )
                            });
                            *__slate_slot_143 = *__slate_slot_331;
                            std::ptr::write(__slate_slot_328, *__slate_slot_142);
                            std::ptr::write(__slate_slot_329, *__slate_slot_328 + (1 as i32));
                            *__slate_slot_142 = *__slate_slot_329;
                        } else {
                            break;
                        }
                    }
                    *__slate_slot_128 = 0 as i32;
                }
            }
        }
        if *__slate_slot_130 != std::ptr::null_mut::<sqlite3_tokenizer_cursor>() {
            unsafe { unsafe { (*(*__slate_slot_127)).xClose }.unwrap()(*__slate_slot_130) };
        }
        unsafe { sqlite3_free(*__slate_slot_131 as *mut ()) };
        if *__slate_slot_128 != (0 as i32) {
            unsafe { sqlite3_free(*__slate_slot_129 as *mut ()) };
            *__slate_slot_129 = std::ptr::null_mut::<Fts3Expr>();
        }
        unsafe {
            *ppExpr = *__slate_slot_129;
        }
        return *__slate_slot_128;
    }
    return unsafe { std::mem::zeroed() };
}

/// The output variable *ppExpr is populated with an allocated Fts3Expr
/// structure, or set to 0 if the end of the input buffer is reached.
///
/// Returns an SQLite error code. SQLITE_OK if everything works, SQLITE_NOMEM
/// if a malloc failure occurs, or SQLITE_ERROR if a parse error is encountered.
/// If SQLITE_ERROR is returned, pContext is populated with an error message.
///
/// # Arguments
///
/// * `pParse` - fts3 query parse context
/// * `n` - Input string
/// * `ppExpr` - OUT: expression
/// * `pnConsumed` - OUT: Number of bytes consumed
fn getNextNode(
    mut pParse: *mut ParseContext,
    mut z: *const i8,
    mut n: i32,
    mut ppExpr: *mut *mut Fts3Expr,
    mut pnConsumed: *mut i32,
) -> i32 {
    let mut ii: i32 = 0 as i32;
    let mut iCol: i32 = 0 as i32;
    let mut iColLen: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut pRet: *mut Fts3Expr = std::ptr::null_mut::<Fts3Expr>();
    let mut zInput: *const i8 = z;
    let mut nInput: i32 = n;
    unsafe {
        (*pParse).isNot = 0 as i32;
    }
    // Skip over any whitespace before checking for a keyword, an open or
    // close bracket, or a quoted string.
    '__slate_break_293: loop {
        let __v332: bool;
        if nInput > (0 as i32) {
            __v332 = fts3isspace(unsafe { *zInput }) != (0 as i32);
        } else {
            __v332 = false as bool;
        }
        if !__v332 {
            break;
        }
        let __v333: i32 = nInput;
        let __v334: i32 = __v333 - (1 as i32);
        nInput = __v334;
        let __v335: *const i8 = zInput;
        let __v336: *const i8 = unsafe { __v335.offset((1 as i32) as isize) };
        zInput = __v336;
    }
    if nInput == (0 as i32) {
        return 101 as i32;
    }
    // See if we are dealing with a keyword.
    ii = 0 as i32;
    '__slate_break_294: loop {
        if !(ii < ((((64 as u64) / (16 as u64)) as u32) as i32)) {
            break;
        }
        let mut pKey: *const Fts3Keyword = unsafe {
            unsafe { std::ptr::addr_of!(aKeyword.0) as *const Fts3Keyword }.offset(ii as isize)
        };
        if (((unsafe { (*pKey).parenOnly }) as u32) as i32) & !(0 as i32) != (0 as i32) {
        } else {
            if nInput >= (((unsafe { (*pKey).n }) as u32) as i32)
                && (0 as i32)
                    == unsafe {
                        memcmp(
                            zInput as *const (),
                            (unsafe { (*pKey).z }) as *const (),
                            (unsafe { (*pKey).n }) as u64,
                        )
                    }
            {
                let mut nNear: i32 = 10 as i32;
                let mut nKey: i32 = ((unsafe { (*pKey).n }) as u32) as i32;
                let mut cNext: i8 = 0 as i8;
                // If this is a "NEAR" keyword, check for an explicit nearness.
                if (((unsafe { (*pKey).eType }) as u32) as i32) == (1 as i32) {
                    0 as i32;
                    if ((unsafe { *unsafe { zInput.offset((4 as i32) as isize) } }) as i32)
                        == (47 as i32)
                        && ((unsafe { *unsafe { zInput.offset((5 as i32) as isize) } }) as i32)
                            >= (48 as i32)
                        && ((unsafe { *unsafe { zInput.offset((5 as i32) as isize) } }) as i32)
                            <= (57 as i32)
                    {
                        let __v339: i32 = nKey;
                        let __v340: i32 = __v339
                            + ((1 as i32)
                                + unsafe {
                                    sqlite3Fts3ReadInt(
                                        unsafe { zInput.offset((nKey + (1 as i32)) as isize) },
                                        std::ptr::addr_of_mut!(nNear),
                                    )
                                });
                        nKey = __v340;
                        if nNear >= (1000000000 as i32) {
                            nNear = 1000000000 as i32;
                        }
                    }
                }
                // At this point this is probably a keyword. But for that to be true,
                // the next byte must contain either whitespace, an open or close
                // parenthesis, a quote character, or EOF.
                cNext = unsafe { *unsafe { zInput.offset(nKey as isize) } };
                if fts3isspace(cNext) != (0 as i32)
                    || (cNext as i32) == (34 as i32)
                    || (cNext as i32) == (40 as i32)
                    || (cNext as i32) == (41 as i32)
                    || (cNext as i32) == (0 as i32)
                {
                    pRet = sqlite3Fts3MallocZero((64 as u64) as i64) as *mut Fts3Expr;
                    if !(pRet != std::ptr::null_mut::<Fts3Expr>()) {
                        return 7 as i32;
                    }
                    unsafe {
                        (*pRet).eType = ((unsafe { (*pKey).eType }) as u32) as i32;
                    }
                    unsafe {
                        (*pRet).nNear = nNear;
                    }
                    unsafe {
                        *ppExpr = pRet;
                    }
                    unsafe {
                        *pnConsumed = (((unsafe { zInput.offset_from(z as *const i8) }) as i64)
                            + (nKey as i64)) as i32;
                    }
                    return 0 as i32;
                }
                // Turns out that wasn't a keyword after all. This happens if the
                // user has supplied a token such as "ORacle". Continue.
            }
        }
        let __v337: i32 = ii;
        let __v338: i32 = __v337 + (1 as i32);
        ii = __v338;
    }
    // See if we are dealing with a quoted phrase. If this is the case, then
    // search for the closing quote and pass the whole string to getNextString()
    // for processing. This is easy to do, as fts3 has no syntax for escaping
    // a quote character embedded in a string.
    if ((unsafe { *zInput }) as i32) == (34 as i32) {
        ii = 1 as i32;
        '__slate_break_295: loop {
            if !(ii < nInput
                && ((unsafe { *unsafe { zInput.offset(ii as isize) } }) as i32) != (34 as i32))
            {
                break;
            }
            {}
            let __v341: i32 = ii;
            let __v342: i32 = __v341 + (1 as i32);
            ii = __v342;
        }
        unsafe {
            *pnConsumed = (((unsafe { zInput.offset_from(z as *const i8) }) as i64)
                + (ii as i64)
                + ((1 as i32) as i64)) as i32;
        }
        if ii == nInput {
            return 1 as i32;
        }
        return getNextString(
            pParse,
            unsafe { zInput.offset((1 as i32) as isize) },
            ii - (1 as i32),
            ppExpr,
        );
    }
    if (0 as i32) != (0 as i32) {
        if ((unsafe { *zInput }) as i32) == (40 as i32) {
            let mut nConsumed: i32 = 0 as i32;
            let __v343: *mut ParseContext = pParse;
            let __v344: i32 = unsafe { (*__v343).nNest };
            let __v345: i32 = __v344 + (1 as i32);
            unsafe {
                (*__v343).nNest = __v345;
            }
            if (unsafe { (*pParse).nNest }) > (1000 as i32) {
                return 1 as i32;
            }
            rc = fts3ExprParse(
                pParse,
                unsafe { zInput.offset((1 as i32) as isize) },
                nInput - (1 as i32),
                ppExpr,
                std::ptr::addr_of_mut!(nConsumed),
            );
            unsafe {
                *pnConsumed = (((unsafe { zInput.offset_from(z as *const i8) }) as i64) as i32)
                    + (1 as i32)
                    + nConsumed;
            }
            return rc;
        } else {
            if ((unsafe { *zInput }) as i32) == (41 as i32) {
                let __v346: *mut ParseContext = pParse;
                let __v347: i32 = unsafe { (*__v346).nNest };
                let __v348: i32 = __v347 - (1 as i32);
                unsafe {
                    (*__v346).nNest = __v348;
                }
                unsafe {
                    *pnConsumed = (((unsafe { zInput.offset_from(z as *const i8) }) as i64)
                        + ((1 as i32) as i64)) as i32;
                }
                unsafe {
                    *ppExpr = std::ptr::null_mut::<Fts3Expr>();
                }
                return 101 as i32;
            }
        }
    }
    // If control flows to this point, this must be a regular token, or
    // the end of the input. Read a regular token using the sqlite3_tokenizer
    // interface. Before doing so, figure out if there is an explicit
    // column specifier for the token.
    //
    // TODO: Strangely, it is not possible to associate a column specifier
    // with a quoted phrase, only with a single token. Not sure if this was
    // an implementation artifact or an intentional decision when fts3 was
    // first implemented. Whichever it was, this module duplicates the
    // limitation.
    iCol = unsafe { (*pParse).iDefaultCol };
    iColLen = 0 as i32;
    ii = 0 as i32;
    '__slate_break_296: loop {
        if !(ii < unsafe { (*pParse).nCol }) {
            break;
        }
        let mut zStr: *const i8 =
            unsafe { *unsafe { unsafe { (*pParse).azCol }.offset(ii as isize) } };
        let mut nStr: i32 = if zStr != std::ptr::null::<i8>() {
            ((unsafe { strlen(zStr) }) as u32) as i32
        } else {
            0 as i32
        };
        let __v351: bool;
        if nInput > nStr
            && ((unsafe { *unsafe { zInput.offset(nStr as isize) } }) as i32) == (58 as i32)
        {
            __v351 = (unsafe { sqlite3_strnicmp(zStr, zInput, nStr) }) == (0 as i32);
        } else {
            __v351 = false as bool;
        }
        if __v351 {
            iCol = ii;
            iColLen = (((unsafe { zInput.offset_from(z as *const i8) }) as i64)
                + (nStr as i64)
                + ((1 as i32) as i64)) as i32;
            break '__slate_break_296;
        }
        let __v349: i32 = ii;
        let __v350: i32 = __v349 + (1 as i32);
        ii = __v350;
    }
    rc = getNextToken(
        pParse,
        iCol,
        unsafe { z.offset(iColLen as isize) },
        n - iColLen,
        ppExpr,
        pnConsumed,
    );
    let __v352: *mut i32 = pnConsumed;
    let __v353: i32 = unsafe { *__v352 };
    let __v354: i32 = __v353 + iColLen;
    unsafe {
        *__v352 = __v354;
    }
    return rc;
}

static mut aKeyword: __SlateAlign16<[Fts3Keyword; 4]> = __SlateAlign16([
    Fts3Keyword {
        z: b"OR\0".as_ptr() as *mut i8,
        n: ((2 as i32) as i8) as u8,
        parenOnly: ((0 as i32) as i8) as u8,
        eType: ((4 as i32) as i8) as u8,
    },
    Fts3Keyword {
        z: b"AND\0".as_ptr() as *mut i8,
        n: ((3 as i32) as i8) as u8,
        parenOnly: ((1 as i32) as i8) as u8,
        eType: ((3 as i32) as i8) as u8,
    },
    Fts3Keyword {
        z: b"NOT\0".as_ptr() as *mut i8,
        n: ((3 as i32) as i8) as u8,
        parenOnly: ((1 as i32) as i8) as u8,
        eType: ((2 as i32) as i8) as u8,
    },
    Fts3Keyword {
        z: b"NEAR\0".as_ptr() as *mut i8,
        n: ((4 as i32) as i8) as u8,
        parenOnly: ((0 as i32) as i8) as u8,
        eType: ((1 as i32) as i8) as u8,
    },
]);

/// The argument is an Fts3Expr structure for a binary operator (any type
/// except an FTSQUERY_PHRASE). Return an integer value representing the
/// precedence of the operator. Lower values have a higher precedence (i.e.
/// group more tightly). For example, in the C language, the == operator
/// groups more tightly than ||, and would therefore have a higher precedence.
///
/// When using the new fts3 query syntax (when SQLITE_ENABLE_FTS3_PARENTHESIS
/// is defined), the order of the operators in precedence from highest to
/// lowest is:
///
///   NEAR
///   NOT
///   AND (including implicit ANDs)
///   OR
///
/// Note that when using the old query syntax, the OR operator has a higher
/// precedence than the AND operator.
fn opPrecedence(mut p: *mut Fts3Expr) -> i32 {
    0 as i32;
    if (0 as i32) != (0 as i32) {
        return unsafe { (*p).eType };
    } else {
        if (unsafe { (*p).eType }) == (1 as i32) {
            return 1 as i32;
        } else {
            if (unsafe { (*p).eType }) == (4 as i32) {
                return 2 as i32;
            }
        }
    }
    0 as i32;
    return 3 as i32;
}

/// Argument ppHead contains a pointer to the current head of a query
/// expression tree being parsed. pPrev is the expression node most recently
/// inserted into the tree. This function adds pNew, which is always a binary
/// operator node, into the expression tree based on the relative precedence
/// of pNew and the existing nodes of the tree. This may result in the head
/// of the tree changing, in which case *ppHead is set to the new root node.
///
/// # Arguments
///
/// * `ppHead` - Pointer to the root node of a tree
/// * `pPrev` - Node most recently inserted into the tree
/// * `pNew` - New binary node to insert into expression tree
fn insertBinaryOperator(
    mut ppHead: *mut *mut Fts3Expr,
    mut pPrev: *mut Fts3Expr,
    mut pNew: *mut Fts3Expr,
) {
    let mut pSplit: *mut Fts3Expr = pPrev;
    '__slate_break_297: loop {
        let __v355: bool;
        if (unsafe { (*pSplit).pParent }) != std::ptr::null_mut::<Fts3Expr>() {
            __v355 = opPrecedence(unsafe { (*pSplit).pParent }) <= opPrecedence(pNew);
        } else {
            __v355 = false as bool;
        }
        if !__v355 {
            break;
        }
        pSplit = unsafe { (*pSplit).pParent };
    }
    if (unsafe { (*pSplit).pParent }) != std::ptr::null_mut::<Fts3Expr>() {
        0 as i32;
        unsafe {
            (*unsafe { (*pSplit).pParent }).pRight = pNew;
        }
        unsafe {
            (*pNew).pParent = unsafe { (*pSplit).pParent };
        }
    } else {
        unsafe {
            *ppHead = pNew;
        }
    }
    unsafe {
        (*pNew).pLeft = pSplit;
    }
    unsafe {
        (*pSplit).pParent = pNew;
    }
}

/// Parse the fts3 query expression found in buffer z, length n. This function
/// returns either when the end of the buffer is reached or an unmatched
/// closing bracket - ')' - is encountered.
///
/// If successful, SQLITE_OK is returned, *ppExpr is set to point to the
/// parsed form of the expression and *pnConsumed is set to the number of
/// bytes read from buffer z. Otherwise, *ppExpr is set to 0 and SQLITE_NOMEM
/// (out of memory error) or SQLITE_ERROR (parse error) is returned.
///
/// # Arguments
///
/// * `pParse` - fts3 query parse context
/// * `n` - Text of MATCH query
/// * `ppExpr` - OUT: Parsed query structure
/// * `pnConsumed` - OUT: Number of bytes consumed
fn fts3ExprParse(
    mut pParse: *mut ParseContext,
    mut z: *const i8,
    mut n: i32,
    mut ppExpr: *mut *mut Fts3Expr,
    mut pnConsumed: *mut i32,
) -> i32 {
    let mut __slate_storage_192: std::mem::MaybeUninit<*mut Fts3Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_192: *mut *mut Fts3Expr =
        std::ptr::addr_of_mut!(__slate_storage_192) as *mut *mut Fts3Expr;
    let mut __slate_storage_315: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_315: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_315) as *mut *const i8;
    let mut __slate_storage_314: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_314: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_314) as *mut *const i8;
    let mut __slate_storage_313: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_313: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_313) as *mut i32;
    let mut __slate_storage_312: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_312: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_312) as *mut i32;
    // Create an implicit NOT operator.
    let mut __slate_storage_189: std::mem::MaybeUninit<*mut Fts3Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_189: *mut *mut Fts3Expr =
        std::ptr::addr_of_mut!(__slate_storage_189) as *mut *mut Fts3Expr;
    // Insert an implicit AND operator.
    let mut __slate_storage_191: std::mem::MaybeUninit<*mut Fts3Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_191: *mut *mut Fts3Expr =
        std::ptr::addr_of_mut!(__slate_storage_191) as *mut *mut Fts3Expr;
    let mut __slate_storage_190: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_190: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_190) as *mut i32;
    let mut __slate_storage_188: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_188: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_188) as *mut i32;
    let mut __slate_storage_187: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_187: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_187) as *mut i32;
    let mut __slate_storage_186: std::mem::MaybeUninit<*mut Fts3Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_186: *mut *mut Fts3Expr =
        std::ptr::addr_of_mut!(__slate_storage_186) as *mut *mut Fts3Expr;
    let mut __slate_storage_185: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_185: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_185) as *mut i32;
    let mut __slate_storage_184: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_184: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_184) as *mut i32;
    let mut __slate_storage_183: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_183: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_183) as *mut *const i8;
    let mut __slate_storage_182: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_182: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_182) as *mut i32; // Only used in legacy parse mode
    let mut __slate_storage_181: std::mem::MaybeUninit<*mut Fts3Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_181: *mut *mut Fts3Expr =
        std::ptr::addr_of_mut!(__slate_storage_181) as *mut *mut Fts3Expr;
    let mut __slate_storage_180: std::mem::MaybeUninit<*mut Fts3Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_180: *mut *mut Fts3Expr =
        std::ptr::addr_of_mut!(__slate_storage_180) as *mut *mut Fts3Expr;
    let mut __slate_storage_179: std::mem::MaybeUninit<*mut Fts3Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_179: *mut *mut Fts3Expr =
        std::ptr::addr_of_mut!(__slate_storage_179) as *mut *mut Fts3Expr;
    unsafe {
        std::ptr::write(__slate_slot_179, std::ptr::null_mut::<Fts3Expr>());
        std::ptr::write(__slate_slot_180, std::ptr::null_mut::<Fts3Expr>());
        std::ptr::write(__slate_slot_181, std::ptr::null_mut::<Fts3Expr>());
        std::ptr::write(__slate_slot_182, n);
        std::ptr::write(__slate_slot_183, z);
        std::ptr::write(__slate_slot_184, 0 as i32);
        std::ptr::write(__slate_slot_185, 1 as i32);
        '__join_2: {
            '__join_21: {
                '__join_35: {
                    '__join_32: {
                        '__join_29: {
                            loop {
                                if *__slate_slot_184 == (0 as i32) {
                                    std::ptr::write(
                                        __slate_slot_186,
                                        std::ptr::null_mut::<Fts3Expr>(),
                                    );
                                    std::ptr::write(__slate_slot_187, 0 as i32);
                                    *__slate_slot_184 = getNextNode(
                                        pParse,
                                        *__slate_slot_183,
                                        *__slate_slot_182,
                                        std::ptr::addr_of_mut!(*__slate_slot_186),
                                        std::ptr::addr_of_mut!(*__slate_slot_187),
                                    );
                                    0 as i32;
                                    if *__slate_slot_184 == (0 as i32) {
                                        if *__slate_slot_186 != std::ptr::null_mut::<Fts3Expr>() {
                                            if !((0 as i32) != (0 as i32))
                                                && (unsafe { (*(*__slate_slot_186)).eType })
                                                    == (5 as i32)
                                                && (unsafe { (*pParse).isNot }) != (0 as i32)
                                            {
                                                std::ptr::write(
                                                    __slate_slot_189,
                                                    sqlite3Fts3MallocZero((64 as u64) as i64)
                                                        as *mut Fts3Expr,
                                                );
                                                if !(*__slate_slot_189
                                                    != std::ptr::null_mut::<Fts3Expr>())
                                                {
                                                    break '__join_21;
                                                } else {
                                                    unsafe {
                                                        (*(*__slate_slot_189)).eType = 2 as i32;
                                                    }
                                                    unsafe {
                                                        (*(*__slate_slot_189)).pRight =
                                                            *__slate_slot_186;
                                                    }
                                                    unsafe {
                                                        (*(*__slate_slot_186)).pParent =
                                                            *__slate_slot_189;
                                                    }
                                                    if *__slate_slot_181
                                                        != std::ptr::null_mut::<Fts3Expr>()
                                                    {
                                                        unsafe {
                                                            (*(*__slate_slot_189)).pLeft =
                                                                *__slate_slot_181;
                                                        }
                                                        unsafe {
                                                            (*(*__slate_slot_181)).pParent =
                                                                *__slate_slot_189;
                                                        }
                                                    }
                                                    *__slate_slot_181 = *__slate_slot_189;
                                                    *__slate_slot_186 = *__slate_slot_180;
                                                }
                                            } else {
                                                std::ptr::write(__slate_slot_190, unsafe {
                                                    (*(*__slate_slot_186)).eType
                                                });
                                                *__slate_slot_188 = (*__slate_slot_190
                                                    == (5 as i32)
                                                    || (unsafe { (*(*__slate_slot_186)).pLeft })
                                                        != std::ptr::null_mut::<Fts3Expr>())
                                                    as i32;
                                                // The isRequirePhrase variable is set to true if a phrase or
                                                // an expression contained in parenthesis is required. If a
                                                // binary operator (AND, OR, NOT or NEAR) is encountered when
                                                // isRequirePhrase is set, this is a syntax error.
                                                if !(*__slate_slot_188 != (0 as i32))
                                                    && *__slate_slot_185 != (0 as i32)
                                                {
                                                    break '__join_35;
                                                } else {
                                                    if *__slate_slot_188 != (0 as i32)
                                                        && !(*__slate_slot_185 != (0 as i32))
                                                    {
                                                        0 as i32;
                                                        *__slate_slot_191 = sqlite3Fts3MallocZero(
                                                            (64 as u64) as i64,
                                                        )
                                                            as *mut Fts3Expr;
                                                        if !(*__slate_slot_191
                                                            != std::ptr::null_mut::<Fts3Expr>())
                                                        {
                                                            break '__join_32;
                                                        } else {
                                                            unsafe {
                                                                (*(*__slate_slot_191)).eType =
                                                                    3 as i32;
                                                            }
                                                            insertBinaryOperator(
                                                                std::ptr::addr_of_mut!(
                                                                    *__slate_slot_179
                                                                ),
                                                                *__slate_slot_180,
                                                                *__slate_slot_191,
                                                            );
                                                            *__slate_slot_180 = *__slate_slot_191;
                                                        }
                                                    }
                                                    // This test catches attempts to make either operand of a NEAR
                                                    // operator something other than a phrase. For example, either of
                                                    // the following:
                                                    //
                                                    //    (bracketed expression) NEAR phrase
                                                    //    phrase NEAR (bracketed expression)
                                                    //
                                                    // Return an error in either case.
                                                    if *__slate_slot_180
                                                        != std::ptr::null_mut::<Fts3Expr>()
                                                        && (*__slate_slot_190 == (1 as i32)
                                                            && !(*__slate_slot_188 != (0 as i32))
                                                            && (unsafe {
                                                                (*(*__slate_slot_180)).eType
                                                            }) != (5 as i32)
                                                            || *__slate_slot_190 != (5 as i32)
                                                                && *__slate_slot_188 != (0 as i32)
                                                                && (unsafe {
                                                                    (*(*__slate_slot_180)).eType
                                                                }) == (1 as i32))
                                                    {
                                                        break '__join_29;
                                                    } else {
                                                        if *__slate_slot_188 != (0 as i32) {
                                                            if *__slate_slot_179
                                                                != std::ptr::null_mut::<Fts3Expr>()
                                                            {
                                                                0 as i32;
                                                                unsafe {
                                                                    (*(*__slate_slot_180)).pRight =
                                                                        *__slate_slot_186;
                                                                }
                                                                unsafe {
                                                                    (*(*__slate_slot_186))
                                                                        .pParent =
                                                                        *__slate_slot_180;
                                                                }
                                                            } else {
                                                                *__slate_slot_179 =
                                                                    *__slate_slot_186;
                                                            }
                                                        } else {
                                                            insertBinaryOperator(
                                                                std::ptr::addr_of_mut!(
                                                                    *__slate_slot_179
                                                                ),
                                                                *__slate_slot_180,
                                                                *__slate_slot_186,
                                                            );
                                                        }
                                                        *__slate_slot_185 = !(*__slate_slot_188
                                                            != (0 as i32))
                                                            as i32;
                                                    }
                                                }
                                            }
                                            *__slate_slot_180 = *__slate_slot_186;
                                        }
                                        0 as i32;
                                    }
                                    0 as i32;
                                    std::ptr::write(__slate_slot_312, *__slate_slot_182);
                                    std::ptr::write(
                                        __slate_slot_313,
                                        *__slate_slot_312 - *__slate_slot_187,
                                    );
                                    *__slate_slot_182 = *__slate_slot_313;
                                    std::ptr::write(__slate_slot_314, *__slate_slot_183);
                                    std::ptr::write(__slate_slot_315, unsafe {
                                        (*__slate_slot_314).offset(*__slate_slot_187 as isize)
                                    });
                                    *__slate_slot_183 = *__slate_slot_315;
                                } else {
                                    break;
                                }
                            }
                            if *__slate_slot_184 == (101 as i32)
                                && *__slate_slot_179 != std::ptr::null_mut::<Fts3Expr>()
                                && *__slate_slot_185 != (0 as i32)
                            {
                                *__slate_slot_184 = 1 as i32;
                            }
                            if *__slate_slot_184 == (101 as i32) {
                                *__slate_slot_184 = 0 as i32;
                                if !((0 as i32) != (0 as i32))
                                    && *__slate_slot_181 != std::ptr::null_mut::<Fts3Expr>()
                                {
                                    if !(*__slate_slot_179 != std::ptr::null_mut::<Fts3Expr>()) {
                                        *__slate_slot_184 = 1 as i32;
                                    } else {
                                        std::ptr::write(__slate_slot_192, *__slate_slot_181);
                                        loop {
                                            if (unsafe { (*(*__slate_slot_192)).pLeft })
                                                != std::ptr::null_mut::<Fts3Expr>()
                                            {
                                                *__slate_slot_192 =
                                                    unsafe { (*(*__slate_slot_192)).pLeft };
                                            } else {
                                                break;
                                            }
                                        }
                                        unsafe {
                                            (*(*__slate_slot_192)).pLeft = *__slate_slot_179;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_179)).pParent = *__slate_slot_192;
                                        }
                                        *__slate_slot_179 = *__slate_slot_181;
                                    }
                                }
                            }
                            unsafe {
                                *pnConsumed = n - *__slate_slot_182;
                            }
                            break '__join_2;
                        }
                        sqlite3Fts3ExprFree(*__slate_slot_186);
                        *__slate_slot_184 = 1 as i32;
                        break '__join_2;
                    }
                    sqlite3Fts3ExprFree(*__slate_slot_186);
                    *__slate_slot_184 = 7 as i32;
                    break '__join_2;
                }
                sqlite3Fts3ExprFree(*__slate_slot_186);
                *__slate_slot_184 = 1 as i32;
                break '__join_2;
            }
            sqlite3Fts3ExprFree(*__slate_slot_186);
            *__slate_slot_184 = 7 as i32;
        }
        if *__slate_slot_184 != (0 as i32) {
            sqlite3Fts3ExprFree(*__slate_slot_179);
            sqlite3Fts3ExprFree(*__slate_slot_181);
            *__slate_slot_179 = std::ptr::null_mut::<Fts3Expr>();
        }
        unsafe {
            *ppExpr = *__slate_slot_179;
        }
        return *__slate_slot_184;
    }
    return unsafe { std::mem::zeroed() };
}

/// Return SQLITE_ERROR if the maximum depth of the expression tree passed
/// as the only argument is more than nMaxDepth.
fn fts3ExprCheckDepth(mut p: *mut Fts3Expr, mut nMaxDepth: i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    if p != std::ptr::null_mut::<Fts3Expr>() {
        if nMaxDepth < (0 as i32) {
            rc = 18 as i32;
        } else {
            rc = fts3ExprCheckDepth(unsafe { (*p).pLeft }, nMaxDepth - (1 as i32));
            if rc == (0 as i32) {
                rc = fts3ExprCheckDepth(unsafe { (*p).pRight }, nMaxDepth - (1 as i32));
            }
        }
    }
    return rc;
}

/// This function attempts to transform the expression tree at (*pp) to
/// an equivalent but more balanced form. The tree is modified in place.
/// If successful, SQLITE_OK is returned and (*pp) set to point to the
/// new root expression node.
///
/// nMaxDepth is the maximum allowable depth of the balanced sub-tree.
///
/// Otherwise, if an error occurs, an SQLite error code is returned and
/// expression (*pp) freed.
fn fts3ExprBalance(mut pp: *mut *mut Fts3Expr, mut nMaxDepth: i32) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut pRoot: *mut Fts3Expr = unsafe { *pp }; // Initial root node
    let mut pFree: *mut Fts3Expr = std::ptr::null_mut::<Fts3Expr>(); // List of free nodes. Linked by pParent.
    let mut eType: i32 = unsafe { (*pRoot).eType }; // Type of node in this tree
    if nMaxDepth == (0 as i32) {
        rc = 1 as i32;
    }
    if rc == (0 as i32) {
        if eType == (3 as i32) || eType == (4 as i32) {
            let mut apLeaf: *mut *mut Fts3Expr = unsafe { std::mem::zeroed() };
            apLeaf =
                (unsafe { sqlite3_malloc64((8 as u64).wrapping_mul((nMaxDepth as i64) as u64)) })
                    as *mut *mut Fts3Expr;
            if std::ptr::null_mut::<*mut Fts3Expr>() == apLeaf {
                rc = 7 as i32;
            } else {
                unsafe {
                    memset(
                        apLeaf as *mut (),
                        0 as i32,
                        (8 as u64).wrapping_mul((nMaxDepth as i64) as u64),
                    )
                };
            }
            if rc == (0 as i32) {
                let mut i: i32 = 0 as i32;
                let mut p: *mut Fts3Expr = unsafe { std::mem::zeroed() };
                // Set $p to point to the left-most leaf in the tree of eType nodes.
                p = pRoot;
                '__slate_break_300: while (unsafe { (*p).eType }) == eType {
                    0 as i32;
                    0 as i32;
                    p = unsafe { (*p).pLeft };
                }
                // This loop runs once for each leaf in the tree of eType nodes.
                '__slate_break_301: while (1 as i32) != (0 as i32) {
                    let mut iLvl: i32 = 0 as i32;
                    let mut pParent: *mut Fts3Expr = unsafe { (*p).pParent }; // Current parent of p
                    0 as i32;
                    unsafe {
                        (*p).pParent = std::ptr::null_mut::<Fts3Expr>();
                    }
                    if pParent != std::ptr::null_mut::<Fts3Expr>() {
                        unsafe {
                            (*pParent).pLeft = std::ptr::null_mut::<Fts3Expr>();
                        }
                    } else {
                        pRoot = std::ptr::null_mut::<Fts3Expr>();
                    }
                    rc = fts3ExprBalance(std::ptr::addr_of_mut!(p), nMaxDepth - (1 as i32));
                    if rc != (0 as i32) {
                        break '__slate_break_301;
                    }
                    iLvl = 0 as i32;
                    '__slate_break_302: loop {
                        if !(p != std::ptr::null_mut::<Fts3Expr>() && iLvl < nMaxDepth) {
                            break;
                        }
                        if (unsafe { *unsafe { apLeaf.offset(iLvl as isize) } })
                            == std::ptr::null_mut::<Fts3Expr>()
                        {
                            unsafe {
                                *unsafe { apLeaf.offset(iLvl as isize) } = p;
                            }
                            p = std::ptr::null_mut::<Fts3Expr>();
                        } else {
                            0 as i32;
                            unsafe {
                                (*pFree).pLeft =
                                    unsafe { *unsafe { apLeaf.offset(iLvl as isize) } };
                            }
                            unsafe {
                                (*pFree).pRight = p;
                            }
                            unsafe {
                                (*unsafe { (*pFree).pLeft }).pParent = pFree;
                            }
                            unsafe {
                                (*unsafe { (*pFree).pRight }).pParent = pFree;
                            }
                            p = pFree;
                            pFree = unsafe { (*pFree).pParent };
                            unsafe {
                                (*p).pParent = std::ptr::null_mut::<Fts3Expr>();
                            }
                            unsafe {
                                *unsafe { apLeaf.offset(iLvl as isize) } =
                                    std::ptr::null_mut::<Fts3Expr>();
                            }
                        }
                        let __v356: i32 = iLvl;
                        let __v357: i32 = __v356 + (1 as i32);
                        iLvl = __v357;
                    }
                    if p != std::ptr::null_mut::<Fts3Expr>() {
                        sqlite3Fts3ExprFree(p);
                        rc = 18 as i32;
                        break '__slate_break_301;
                    }
                    // If that was the last leaf node, break out of the loop
                    if pParent == std::ptr::null_mut::<Fts3Expr>() {
                        break '__slate_break_301;
                    }
                    // Set $p to point to the next leaf in the tree of eType nodes
                    p = unsafe { (*pParent).pRight };
                    '__slate_break_303: while (unsafe { (*p).eType }) == eType {
                        {}
                        p = unsafe { (*p).pLeft };
                    }
                    // Remove pParent from the original tree.
                    0 as i32;
                    unsafe {
                        (*unsafe { (*pParent).pRight }).pParent = unsafe { (*pParent).pParent };
                    }
                    if (unsafe { (*pParent).pParent }) != std::ptr::null_mut::<Fts3Expr>() {
                        unsafe {
                            (*unsafe { (*pParent).pParent }).pLeft = unsafe { (*pParent).pRight };
                        }
                    } else {
                        0 as i32;
                        pRoot = unsafe { (*pParent).pRight };
                    }
                    // Link pParent into the free node list. It will be used as an
                    // internal node of the new tree.
                    unsafe {
                        (*pParent).pParent = pFree;
                    }
                    pFree = pParent;
                }
                if rc == (0 as i32) {
                    p = std::ptr::null_mut::<Fts3Expr>();
                    i = 0 as i32;
                    '__slate_break_304: loop {
                        if !(i < nMaxDepth) {
                            break;
                        }
                        if (unsafe { *unsafe { apLeaf.offset(i as isize) } })
                            != std::ptr::null_mut::<Fts3Expr>()
                        {
                            if p == std::ptr::null_mut::<Fts3Expr>() {
                                p = unsafe { *unsafe { apLeaf.offset(i as isize) } };
                                unsafe {
                                    (*p).pParent = std::ptr::null_mut::<Fts3Expr>();
                                }
                            } else {
                                0 as i32;
                                unsafe {
                                    (*pFree).pRight = p;
                                }
                                unsafe {
                                    (*pFree).pLeft =
                                        unsafe { *unsafe { apLeaf.offset(i as isize) } };
                                }
                                unsafe {
                                    (*unsafe { (*pFree).pLeft }).pParent = pFree;
                                }
                                unsafe {
                                    (*unsafe { (*pFree).pRight }).pParent = pFree;
                                }
                                p = pFree;
                                pFree = unsafe { (*pFree).pParent };
                                unsafe {
                                    (*p).pParent = std::ptr::null_mut::<Fts3Expr>();
                                }
                            }
                        }
                        let __v358: i32 = i;
                        let __v359: i32 = __v358 + (1 as i32);
                        i = __v359;
                    }
                    pRoot = p;
                } else {
                    // An error occurred. Delete the contents of the apLeaf[] array
                    // and pFree list. Everything else is cleaned up by the call to
                    // sqlite3Fts3ExprFree(pRoot) below.
                    let mut pDel: *mut Fts3Expr = unsafe { std::mem::zeroed() };
                    i = 0 as i32;
                    '__slate_break_305: loop {
                        if !(i < nMaxDepth) {
                            break;
                        }
                        sqlite3Fts3ExprFree(unsafe { *unsafe { apLeaf.offset(i as isize) } });
                        let __v360: i32 = i;
                        let __v361: i32 = __v360 + (1 as i32);
                        i = __v361;
                    }
                    '__slate_break_306: loop {
                        let __v362: *mut Fts3Expr = pFree;
                        pDel = __v362;
                        if !(__v362 != std::ptr::null_mut::<Fts3Expr>()) {
                            break;
                        }
                        pFree = unsafe { (*pDel).pParent };
                        unsafe { sqlite3_free(pDel as *mut ()) };
                    }
                }
                0 as i32;
                unsafe { sqlite3_free(apLeaf as *mut ()) };
            }
        } else {
            if eType == (2 as i32) {
                let mut pLeft: *mut Fts3Expr = unsafe { (*pRoot).pLeft };
                let mut pRight: *mut Fts3Expr = unsafe { (*pRoot).pRight };
                unsafe {
                    (*pRoot).pLeft = std::ptr::null_mut::<Fts3Expr>();
                }
                unsafe {
                    (*pRoot).pRight = std::ptr::null_mut::<Fts3Expr>();
                }
                unsafe {
                    (*pLeft).pParent = std::ptr::null_mut::<Fts3Expr>();
                }
                unsafe {
                    (*pRight).pParent = std::ptr::null_mut::<Fts3Expr>();
                }
                rc = fts3ExprBalance(std::ptr::addr_of_mut!(pLeft), nMaxDepth - (1 as i32));
                if rc == (0 as i32) {
                    rc = fts3ExprBalance(std::ptr::addr_of_mut!(pRight), nMaxDepth - (1 as i32));
                }
                if rc != (0 as i32) {
                    sqlite3Fts3ExprFree(pRight);
                    sqlite3Fts3ExprFree(pLeft);
                } else {
                    0 as i32;
                    unsafe {
                        (*pRoot).pLeft = pLeft;
                    }
                    unsafe {
                        (*pLeft).pParent = pRoot;
                    }
                    unsafe {
                        (*pRoot).pRight = pRight;
                    }
                    unsafe {
                        (*pRight).pParent = pRoot;
                    }
                }
            }
        }
    }
    if rc != (0 as i32) {
        sqlite3Fts3ExprFree(pRoot);
        pRoot = std::ptr::null_mut::<Fts3Expr>();
    }
    unsafe {
        *pp = pRoot;
    }
    return rc;
}

/// This function is similar to sqlite3Fts3ExprParse(), with the following
/// differences:
///
///   1. It does not do expression rebalancing.
///   2. It does not check that the expression does not exceed the
///      maximum allowable depth.
///   3. Even if it fails, *ppExpr may still be set to point to an
///      expression tree. It should be deleted using sqlite3Fts3ExprFree()
///      in this case.
///
/// # Arguments
///
/// * `pTokenizer` - Tokenizer module
/// * `iLangid` - Language id for tokenizer
/// * `azCol` - Array of column names for fts3 table
/// * `bFts4` - True to allow FTS4-only syntax
/// * `nCol` - Number of entries in azCol[]
/// * `iDefaultCol` - Default column to query
/// * `n` - Text of MATCH query
/// * `ppExpr` - OUT: Parsed query structure
fn fts3ExprParseUnbalanced(
    mut pTokenizer: *mut sqlite3_tokenizer,
    mut iLangid: i32,
    mut azCol: *mut *mut i8,
    mut bFts4: i32,
    mut nCol: i32,
    mut iDefaultCol: i32,
    mut z: *const i8,
    mut n: i32,
    mut ppExpr: *mut *mut Fts3Expr,
) -> i32 {
    let mut nParsed: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut sParse: ParseContext = unsafe { std::mem::zeroed() };
    unsafe {
        memset(
            std::ptr::addr_of_mut!(sParse) as *mut (),
            0 as i32,
            56 as u64,
        )
    };
    sParse.pTokenizer = pTokenizer;
    sParse.iLangid = iLangid;
    sParse.azCol = azCol as *mut *const i8;
    sParse.nCol = nCol;
    sParse.iDefaultCol = iDefaultCol;
    sParse.bFts4 = bFts4;
    if z == std::ptr::null::<i8>() {
        unsafe {
            *ppExpr = std::ptr::null_mut::<Fts3Expr>();
        }
        return 0 as i32;
    }
    if n < (0 as i32) {
        n = ((unsafe { strlen(z) }) as u32) as i32;
    }
    rc = fts3ExprParse(
        std::ptr::addr_of_mut!(sParse),
        z,
        n,
        ppExpr,
        std::ptr::addr_of_mut!(nParsed),
    );
    0 as i32;
    // Check for mismatched parenthesis
    if rc == (0 as i32) && sParse.nNest != (0 as i32) {
        rc = 1 as i32;
    }
    return rc;
}

/// Parameters z and n contain a pointer to and length of a buffer containing
/// an fts3 query expression, respectively. This function attempts to parse the
/// query expression and create a tree of Fts3Expr structures representing the
/// parsed expression. If successful, *ppExpr is set to point to the head
/// of the parsed expression tree and SQLITE_OK is returned. If an error
/// occurs, either SQLITE_NOMEM (out-of-memory error) or SQLITE_ERROR (parse
/// error) is returned and *ppExpr is set to 0.
///
/// If parameter n is a negative number, then z is assumed to point to a
/// nul-terminated string and the length is determined using strlen().
///
/// The first parameter, pTokenizer, is passed the fts3 tokenizer module to
/// use to normalize query tokens while parsing the expression. The azCol[]
/// array, which is assumed to contain nCol entries, should contain the names
/// of each column in the target fts3 table, in order from left to right.
/// Column names must be nul-terminated strings.
///
/// The iDefaultCol parameter should be passed the index of the table column
/// that appears on the left-hand-side of the MATCH operator (the default
/// column to match against for tokens for which a column name is not explicitly
/// specified as part of the query string), or -1 if tokens may by default
/// match any table column.
///
/// # Arguments
///
/// * `pTokenizer` - Tokenizer module
/// * `iLangid` - Language id for tokenizer
/// * `azCol` - Array of column names for fts3 table
/// * `bFts4` - True to allow FTS4-only syntax
/// * `nCol` - Number of entries in azCol[]
/// * `iDefaultCol` - Default column to query
/// * `n` - Text of MATCH query
/// * `ppExpr` - OUT: Parsed query structure
/// * `pzErr` - OUT: Error message (sqlite3_malloc)
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3ExprParse(
    mut pTokenizer: *mut sqlite3_tokenizer,
    mut iLangid: i32,
    mut azCol: *mut *mut i8,
    mut bFts4: i32,
    mut nCol: i32,
    mut iDefaultCol: i32,
    mut z: *const i8,
    mut n: i32,
    mut ppExpr: *mut *mut Fts3Expr,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut rc: i32 = fts3ExprParseUnbalanced(
        pTokenizer,
        iLangid,
        azCol,
        bFts4,
        nCol,
        iDefaultCol,
        z,
        n,
        ppExpr,
    );
    // Rebalance the expression. And check that its depth does not exceed
    // SQLITE_FTS3_MAX_EXPR_DEPTH.
    if rc == (0 as i32) && (unsafe { *ppExpr }) != std::ptr::null_mut::<Fts3Expr>() {
        rc = fts3ExprBalance(ppExpr, 12 as i32);
        if rc == (0 as i32) {
            rc = fts3ExprCheckDepth(unsafe { *ppExpr }, 12 as i32);
        }
    }
    if rc != (0 as i32) {
        sqlite3Fts3ExprFree(unsafe { *ppExpr });
        unsafe {
            *ppExpr = std::ptr::null_mut::<Fts3Expr>();
        }
        if rc == (18 as i32) {
            unsafe {
                sqlite3Fts3ErrMsg(
                    pzErr,
                    (b"FTS expression tree is too large (maximum depth %d)\0".as_ptr() as *mut i8)
                        as *const i8,
                    12 as i32,
                )
            };
            rc = 1 as i32;
        } else {
            if rc == (1 as i32) {
                unsafe {
                    sqlite3Fts3ErrMsg(
                        pzErr,
                        (b"malformed MATCH expression: [%s]\0".as_ptr() as *mut i8) as *const i8,
                        z,
                    )
                };
            }
        }
    }
    return rc;
}

/// Free a single node of an expression tree.
fn fts3FreeExprNode(mut p: *mut Fts3Expr) {
    0 as i32;
    unsafe { sqlite3Fts3EvalPhraseCleanup(unsafe { (*p).pPhrase }) };
    unsafe { sqlite3_free((unsafe { (*p).aMI }) as *mut ()) };
    unsafe { sqlite3_free(p as *mut ()) };
}

/// Free a parsed fts3 query expression allocated by sqlite3Fts3ExprParse().
///
/// This function would be simpler if it recursively called itself. But
/// that would mean passing a sufficiently large expression to ExprParse()
/// could cause a stack overflow.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3ExprFree(mut pDel: *mut Fts3Expr) {
    let mut p: *mut Fts3Expr = unsafe { std::mem::zeroed() };
    0 as i32;
    p = pDel;
    '__slate_break_309: while p != std::ptr::null_mut::<Fts3Expr>()
        && ((unsafe { (*p).pLeft }) != std::ptr::null_mut::<Fts3Expr>()
            || (unsafe { (*p).pRight }) != std::ptr::null_mut::<Fts3Expr>())
    {
        0 as i32;
        p = if (unsafe { (*p).pLeft }) != std::ptr::null_mut::<Fts3Expr>() {
            unsafe { (*p).pLeft }
        } else {
            unsafe { (*p).pRight }
        };
    }
    '__slate_break_310: while p != std::ptr::null_mut::<Fts3Expr>() {
        let mut pParent: *mut Fts3Expr = unsafe { (*p).pParent };
        fts3FreeExprNode(p);
        if pParent != std::ptr::null_mut::<Fts3Expr>()
            && p == unsafe { (*pParent).pLeft }
            && (unsafe { (*pParent).pRight }) != std::ptr::null_mut::<Fts3Expr>()
        {
            p = unsafe { (*pParent).pRight };
            '__slate_break_311: while p != std::ptr::null_mut::<Fts3Expr>()
                && ((unsafe { (*p).pLeft }) != std::ptr::null_mut::<Fts3Expr>()
                    || (unsafe { (*p).pRight }) != std::ptr::null_mut::<Fts3Expr>())
            {
                0 as i32;
                p = if (unsafe { (*p).pLeft }) != std::ptr::null_mut::<Fts3Expr>() {
                    unsafe { (*p).pLeft }
                } else {
                    unsafe { (*p).pRight }
                };
            }
        } else {
            p = pParent;
        }
    }
}

// Everything after this point is just test code.
