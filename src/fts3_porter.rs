//! 2006 September 30
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! Implementation of the full-text-search tokenizer that implements
//! a Porter stemmer.
unsafe extern "C" {
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3_malloc(__v124: i32) -> *mut ();
    fn sqlite3_realloc64(__v125: *mut (), __v126: u64) -> *mut ();
    fn sqlite3_free(__v127: *mut ());
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
/// Class derived from sqlite3_tokenizer
#[repr(C)]
#[derive(Clone, Copy)]
struct porter_tokenizer {
    /// Base class
    base: sqlite3_tokenizer,
}

/// Class derived from sqlite3_tokenizer_cursor
#[repr(C)]
#[derive(Clone, Copy)]
struct porter_tokenizer_cursor {
    base: sqlite3_tokenizer_cursor,
    /// input we are tokenizing
    zInput: *const i8,
    /// size of the input
    nInput: i32,
    /// current position in zInput
    iOffset: i32,
    /// index of next token to be returned
    iToken: i32,
    /// storage for current token
    zToken: *mut i8,
    /// space allocated to zToken buffer
    nAllocated: i32,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

/// Create a new tokenizer instance.
#[unsafe(link_section = ".text.slate_distinct.fts3_porter.porterCreate")]
extern "C-unwind" fn porterCreate(
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppTokenizer: *mut *mut sqlite3_tokenizer,
) -> i32 {
    let mut t: *mut porter_tokenizer = unsafe { std::mem::zeroed() };
    argc;
    argv;
    t = (unsafe { sqlite3_malloc(((8 as u64) as u32) as i32) }) as *mut porter_tokenizer;
    if t == std::ptr::null_mut::<porter_tokenizer>() {
        return 7 as i32;
    }
    unsafe { memset(t as *mut (), 0 as i32, 8 as u64) };
    unsafe {
        *ppTokenizer = unsafe { std::ptr::addr_of_mut!((*t).base) };
    }
    return 0 as i32;
}

/// Destroy a tokenizer
#[unsafe(link_section = ".text.slate_distinct.fts3_porter.porterDestroy")]
extern "C-unwind" fn porterDestroy(mut pTokenizer: *mut sqlite3_tokenizer) -> i32 {
    unsafe { sqlite3_free(pTokenizer as *mut ()) };
    return 0 as i32;
}

/// Prepare to begin tokenizing a particular string.  The input
/// string to be tokenized is zInput[0..nInput-1].  A cursor
/// used to incrementally tokenize this string is returned in
/// *ppCursor.
///
/// # Arguments
///
/// * `pTokenizer` - The tokenizer
/// * `nInput` - String to be tokenized
/// * `ppCursor` - OUT: Tokenization cursor
#[unsafe(link_section = ".text.slate_distinct.fts3_porter.porterOpen")]
extern "C-unwind" fn porterOpen(
    mut pTokenizer: *mut sqlite3_tokenizer,
    mut zInput: *const i8,
    mut nInput: i32,
    mut ppCursor: *mut *mut sqlite3_tokenizer_cursor,
) -> i32 {
    let mut c: *mut porter_tokenizer_cursor = unsafe { std::mem::zeroed() };
    pTokenizer;
    c = (unsafe { sqlite3_malloc(((48 as u64) as u32) as i32) }) as *mut porter_tokenizer_cursor;
    if c == std::ptr::null_mut::<porter_tokenizer_cursor>() {
        return 7 as i32;
    }
    unsafe {
        (*c).zInput = zInput;
    }
    if zInput == std::ptr::null::<i8>() {
        unsafe {
            (*c).nInput = 0 as i32;
        }
    } else {
        if nInput < (0 as i32) {
            unsafe {
                (*c).nInput = ((unsafe { strlen(zInput) }) as u32) as i32;
            }
        } else {
            unsafe {
                (*c).nInput = nInput;
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
        (*c).zToken = std::ptr::null_mut::<i8>();
    }
    // no space allocated, yet.
    unsafe {
        (*c).nAllocated = 0 as i32;
    }
    unsafe {
        *ppCursor = unsafe { std::ptr::addr_of_mut!((*c).base) };
    }
    return 0 as i32;
}

/// Close a tokenization cursor previously opened by a call to
/// porterOpen() above.
#[unsafe(link_section = ".text.slate_distinct.fts3_porter.porterClose")]
extern "C-unwind" fn porterClose(mut pCursor: *mut sqlite3_tokenizer_cursor) -> i32 {
    let mut c: *mut porter_tokenizer_cursor = pCursor as *mut porter_tokenizer_cursor;
    unsafe { sqlite3_free((unsafe { (*c).zToken }) as *mut ()) };
    unsafe { sqlite3_free(c as *mut ()) };
    return 0 as i32;
}

/// Vowel or consonant
static mut cType: __SlateAlign16<[i8; 26]> = __SlateAlign16([
    (0 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (0 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (0 as i32) as i8,
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
    (0 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (2 as i32) as i8,
    (1 as i32) as i8,
]);

fn isConsonant(mut z: *const i8) -> i32 {
    let mut j: i32 = 0 as i32;
    let mut x: i8 = unsafe { *z };
    if (x as i32) == (0 as i32) {
        return 0 as i32;
    }
    0 as i32;
    j = (unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(cType.0) as *const i8 }
                .offset(((x as i32) - (97 as i32)) as isize)
        }
    }) as i32;
    if j < (2 as i32) {
        return j;
    }
    let __v238: bool;
    if ((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as i32) == (0 as i32) {
        __v238 = true as bool;
    } else {
        __v238 = isVowel(unsafe { z.offset((1 as i32) as isize) }) != (0 as i32);
    }
    return __v238 as i32;
}

/// isConsonant() and isVowel() determine if their first character in
/// the string they point to is a consonant or a vowel, according
/// to Porter ruls.
///
/// A consonate is any letter other than 'a', 'e', 'i', 'o', or 'u'.
/// 'Y' is a consonant unless it follows another consonant,
/// in which case it is a vowel.
///
/// In these routine, the letters are in reverse order.  So the 'y' rule
/// is that 'y' is a consonant unless it is followed by another
/// consonent.
fn isVowel(mut z: *const i8) -> i32 {
    let mut j: i32 = 0 as i32;
    let mut x: i8 = unsafe { *z };
    if (x as i32) == (0 as i32) {
        return 0 as i32;
    }
    0 as i32;
    j = (unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(cType.0) as *const i8 }
                .offset(((x as i32) - (97 as i32)) as isize)
        }
    }) as i32;
    if j < (2 as i32) {
        return (1 as i32) - j;
    }
    return isConsonant(unsafe { z.offset((1 as i32) as isize) });
}

/// Let any sequence of one or more vowels be represented by V and let
/// C be sequence of one or more consonants.  Then every word can be
/// represented as:
///
///           [C] (VC){m} [V]
///
/// In prose:  A word is an optional consonant followed by zero or
/// vowel-consonant pairs followed by an optional vowel.  "m" is the
/// number of vowel consonant pairs.  This routine computes the value
/// of m for the first i bytes of a word.
///
/// Return true if the m-value for z is 1 or more.  In other words,
/// return true if z contains at least one vowel that is followed
/// by a consonant.
///
/// In this routine z[] is in reverse order.  So we are really looking
/// for an instance of a consonant followed by a vowel.
#[unsafe(link_section = ".text.slate_distinct.fts3_porter.m_gt_0")]
extern "C-unwind" fn m_gt_0(mut z: *const i8) -> i32 {
    '__slate_break_129: while isVowel(z) != (0 as i32) {
        let __v239: *const i8 = z;
        let __v240: *const i8 = unsafe { __v239.offset((1 as i32) as isize) };
        z = __v240;
    }
    if ((unsafe { *z }) as i32) == (0 as i32) {
        return 0 as i32;
    }
    '__slate_break_130: while isConsonant(z) != (0 as i32) {
        let __v241: *const i8 = z;
        let __v242: *const i8 = unsafe { __v241.offset((1 as i32) as isize) };
        z = __v242;
    }
    return (((unsafe { *z }) as i32) != (0 as i32)) as i32;
}

/// Like mgt0 above except we are looking for a value of m which is
/// exactly 1
fn m_eq_1(mut z: *const i8) -> i32 {
    '__slate_break_131: while isVowel(z) != (0 as i32) {
        let __v243: *const i8 = z;
        let __v244: *const i8 = unsafe { __v243.offset((1 as i32) as isize) };
        z = __v244;
    }
    if ((unsafe { *z }) as i32) == (0 as i32) {
        return 0 as i32;
    }
    '__slate_break_132: while isConsonant(z) != (0 as i32) {
        let __v245: *const i8 = z;
        let __v246: *const i8 = unsafe { __v245.offset((1 as i32) as isize) };
        z = __v246;
    }
    if ((unsafe { *z }) as i32) == (0 as i32) {
        return 0 as i32;
    }
    '__slate_break_133: while isVowel(z) != (0 as i32) {
        let __v247: *const i8 = z;
        let __v248: *const i8 = unsafe { __v247.offset((1 as i32) as isize) };
        z = __v248;
    }
    if ((unsafe { *z }) as i32) == (0 as i32) {
        return 1 as i32;
    }
    '__slate_break_134: while isConsonant(z) != (0 as i32) {
        let __v249: *const i8 = z;
        let __v250: *const i8 = unsafe { __v249.offset((1 as i32) as isize) };
        z = __v250;
    }
    return (((unsafe { *z }) as i32) == (0 as i32)) as i32;
}

/// Like mgt0 above except we are looking for a value of m>1 instead
/// or m>0
#[unsafe(link_section = ".text.slate_distinct.fts3_porter.m_gt_1")]
extern "C-unwind" fn m_gt_1(mut z: *const i8) -> i32 {
    '__slate_break_135: while isVowel(z) != (0 as i32) {
        let __v251: *const i8 = z;
        let __v252: *const i8 = unsafe { __v251.offset((1 as i32) as isize) };
        z = __v252;
    }
    if ((unsafe { *z }) as i32) == (0 as i32) {
        return 0 as i32;
    }
    '__slate_break_136: while isConsonant(z) != (0 as i32) {
        let __v253: *const i8 = z;
        let __v254: *const i8 = unsafe { __v253.offset((1 as i32) as isize) };
        z = __v254;
    }
    if ((unsafe { *z }) as i32) == (0 as i32) {
        return 0 as i32;
    }
    '__slate_break_137: while isVowel(z) != (0 as i32) {
        let __v255: *const i8 = z;
        let __v256: *const i8 = unsafe { __v255.offset((1 as i32) as isize) };
        z = __v256;
    }
    if ((unsafe { *z }) as i32) == (0 as i32) {
        return 0 as i32;
    }
    '__slate_break_138: while isConsonant(z) != (0 as i32) {
        let __v257: *const i8 = z;
        let __v258: *const i8 = unsafe { __v257.offset((1 as i32) as isize) };
        z = __v258;
    }
    return (((unsafe { *z }) as i32) != (0 as i32)) as i32;
}

/// Return TRUE if there is a vowel anywhere within z[0..n-1]
#[unsafe(link_section = ".text.slate_distinct.fts3_porter.hasVowel")]
extern "C-unwind" fn hasVowel(mut z: *const i8) -> i32 {
    '__slate_break_139: while isConsonant(z) != (0 as i32) {
        let __v259: *const i8 = z;
        let __v260: *const i8 = unsafe { __v259.offset((1 as i32) as isize) };
        z = __v260;
    }
    return (((unsafe { *z }) as i32) != (0 as i32)) as i32;
}

/// Return TRUE if the word ends in a double consonant.
///
/// The text is reversed here. So we are really looking at
/// the first two characters of z[].
fn doubleConsonant(mut z: *const i8) -> i32 {
    return (isConsonant(z) != (0 as i32)
        && ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32)
            == ((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as i32)) as i32;
}

/// Return TRUE if the word ends with three letters which
/// are consonant-vowel-consonent and where the final consonant
/// is not 'w', 'x', or 'y'.
///
/// The word is reversed here.  So we are really checking the
/// first three letters and the first one cannot be in [wxy].
fn star_oh(mut z: *const i8) -> i32 {
    let __v261: bool;
    if isConsonant(z) != (0 as i32)
        && ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) != (119 as i32)
        && ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) != (120 as i32)
        && ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) != (121 as i32)
    {
        __v261 = isVowel(unsafe { z.offset((1 as i32) as isize) }) != (0 as i32);
    } else {
        __v261 = false as bool;
    }
    let __v262: bool;
    if __v261 {
        __v262 = isConsonant(unsafe { z.offset((2 as i32) as isize) }) != (0 as i32);
    } else {
        __v262 = false as bool;
    }
    return __v262 as i32;
}

/// If the word ends with zFrom and xCond() is true for the stem
/// of the word that precedes the zFrom ending, then change the
/// ending to zTo.
///
/// The input word *pz and zFrom are both in reverse order.  zTo
/// is in normal order.
///
/// Return TRUE if zFrom matches.  Return FALSE if zFrom does not
/// match.  Not that TRUE is returned even if xCond() fails and
/// no substitution occurs.
///
/// # Arguments
///
/// * `pz` - The word being stemmed (Reversed)
/// * `zFrom` - If the ending matches this... (Reversed)
/// * `zTo` - ... change the ending to this (not reversed)
/// * `xCond` - Condition that must be true
fn stem(
    mut pz: *mut *mut i8,
    mut zFrom: *const i8,
    mut zTo: *const i8,
    mut xCond: Option<unsafe extern "C-unwind" fn(*const i8) -> i32>,
) -> i32 {
    let mut z: *mut i8 = unsafe { *pz };
    '__slate_break_140: while (unsafe { *zFrom }) != (0 as i8)
        && ((unsafe { *zFrom }) as i32) == ((unsafe { *z }) as i32)
    {
        let __v263: *mut i8 = z;
        let __v264: *mut i8 = unsafe { __v263.offset((1 as i32) as isize) };
        z = __v264;
        let __v265: *const i8 = zFrom;
        let __v266: *const i8 = unsafe { __v265.offset((1 as i32) as isize) };
        zFrom = __v266;
    }
    if ((unsafe { *zFrom }) as i32) != (0 as i32) {
        return 0 as i32;
    }
    let __v267: bool;
    if xCond != None {
        __v267 = !((unsafe { xCond.unwrap()(z as *const i8) }) != (0 as i32));
    } else {
        __v267 = false as bool;
    }
    if __v267 {
        return 1 as i32;
    }
    '__slate_break_141: while (unsafe { *zTo }) != (0 as i8) {
        let __v268: *const i8 = zTo;
        let __v269: *const i8 = unsafe { __v268.offset((1 as i32) as isize) };
        zTo = __v269;
        let __v270: *mut i8 = z;
        let __v271: *mut i8 = unsafe { __v270.offset(-((1 as i32) as isize)) };
        z = __v271;
        unsafe {
            *__v271 = unsafe { *__v268 };
        }
    }
    unsafe {
        *pz = z;
    }
    return 1 as i32;
}

/// This is the fallback stemmer used when the porter stemmer is
/// inappropriate.  The input word is copied into the output with
/// US-ASCII case folding.  If the input word is too long (more
/// than 20 bytes if it contains no digits or more than 6 bytes if
/// it contains digits) then word is truncated to 20 or 6 bytes
/// by taking 10 or 3 bytes from the beginning and end.
fn copy_stemmer(mut zIn: *const i8, mut nIn: i32, mut zOut: *mut i8, mut pnOut: *mut i32) {
    let mut i: i32 = 0 as i32;
    let mut mx: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut hasDigit: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_142: loop {
        if !(i < nIn) {
            break;
        }
        let mut c: i8 = unsafe { *unsafe { zIn.offset(i as isize) } };
        if (c as i32) >= (65 as i32) && (c as i32) <= (90 as i32) {
            unsafe {
                *unsafe { zOut.offset(i as isize) } =
                    ((c as i32) - (65 as i32) + (97 as i32)) as i8;
            }
        } else {
            if (c as i32) >= (48 as i32) && (c as i32) <= (57 as i32) {
                hasDigit = 1 as i32;
            }
            unsafe {
                *unsafe { zOut.offset(i as isize) } = c;
            }
        }
        let __v272: i32 = i;
        let __v273: i32 = __v272 + (1 as i32);
        i = __v273;
    }
    mx = if hasDigit != (0 as i32) {
        3 as i32
    } else {
        10 as i32
    };
    if nIn > mx * (2 as i32) {
        j = mx;
        let __v274: i32 = nIn - mx;
        i = __v274;
        '__slate_break_143: loop {
            if !(i < nIn) {
                break;
            }
            unsafe {
                *unsafe { zOut.offset(j as isize) } =
                    unsafe { *unsafe { zOut.offset(i as isize) } };
            }
            let __v275: i32 = i;
            let __v276: i32 = __v275 + (1 as i32);
            i = __v276;
            let __v277: i32 = j;
            let __v278: i32 = __v277 + (1 as i32);
            j = __v278;
        }
        i = j;
    }
    unsafe {
        *unsafe { zOut.offset(i as isize) } = (0 as i32) as i8;
    }
    unsafe {
        *pnOut = i;
    }
}

/// Stem the input word zIn[0..nIn-1].  Store the output in zOut.
/// zOut is at least big enough to hold nIn bytes.  Write the actual
/// size of the output word (exclusive of the '\0' terminator) into *pnOut.
///
/// Any upper-case characters in the US-ASCII character set ([A-Z])
/// are converted to lower case.  Upper-case UTF characters are
/// unchanged.
///
/// Words that are longer than about 20 bytes are stemmed by retaining
/// a few bytes from the beginning and the end of the word.  If the
/// word contains digits, 3 bytes are taken from the beginning and
/// 3 bytes from the end.  For long words without digits, 10 bytes
/// are taken from each end.  US-ASCII case folding still applies.
///
/// If the input word contains not digits but does characters not
/// in [a-zA-Z] then no stemming is attempted and this routine just
/// copies the input into the input into the output with US-ASCII
/// case folding.
///
/// Stemming never increases the length of the word.  So there is
/// no chance of overflowing the zOut buffer.
fn porter_stemmer(mut zIn: *const i8, mut nIn: i32, mut zOut: *mut i8, mut pnOut: *mut i32) {
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut zReverse: __SlateAlign16<[i8; 28]> = __SlateAlign16([0 as i8; 28]);
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    let mut z2: *mut i8 = unsafe { std::mem::zeroed() };
    if nIn < (3 as i32) || nIn >= (((28 as u64) as u32) as i32) - (7 as i32) {
        // The word is too big or too small for the porter stemmer.
        // Fallback to the copy stemmer
        copy_stemmer(zIn, nIn, zOut, pnOut);
        return;
    }
    i = 0 as i32;
    let __v279: i32 = ((28 as u64).wrapping_sub(((6 as i32) as i64) as u64) as u32) as i32;
    j = __v279;
    '__slate_break_144: loop {
        if !(i < nIn) {
            break;
        }
        let mut c: i8 = unsafe { *unsafe { zIn.offset(i as isize) } };
        if (c as i32) >= (65 as i32) && (c as i32) <= (90 as i32) {
            unsafe {
                *unsafe { (zReverse.0.as_mut_ptr() as *mut i8).offset(j as isize) } =
                    ((c as i32) + (97 as i32) - (65 as i32)) as i8;
            }
        } else {
            if (c as i32) >= (97 as i32) && (c as i32) <= (122 as i32) {
                unsafe {
                    *unsafe { (zReverse.0.as_mut_ptr() as *mut i8).offset(j as isize) } = c;
                }
            } else {
                // The use of a character not in [a-zA-Z] means that we fallback
                // to the copy stemmer
                copy_stemmer(zIn, nIn, zOut, pnOut);
                return;
            }
        }
        let __v280: i32 = i;
        let __v281: i32 = __v280 + (1 as i32);
        i = __v281;
        let __v282: i32 = j;
        let __v283: i32 = __v282 - (1 as i32);
        j = __v283;
    }
    unsafe {
        memset(
            (unsafe {
                (zReverse.0.as_mut_ptr() as *mut i8)
                    .offset((28 as u64).wrapping_sub(((5 as i32) as i64) as u64) as isize)
            }) as *mut (),
            0 as i32,
            ((5 as i32) as i64) as u64,
        )
    };
    z = unsafe { (zReverse.0.as_mut_ptr() as *mut i8).offset((j + (1 as i32)) as isize) };
    // Step 1a
    if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (115 as i32) {
        let __v284: bool;
        if !(stem(
            std::ptr::addr_of_mut!(z),
            (b"sess\0".as_ptr() as *mut i8) as *const i8,
            (b"ss\0".as_ptr() as *mut i8) as *const i8,
            None,
        ) != (0 as i32))
        {
            __v284 = !(stem(
                std::ptr::addr_of_mut!(z),
                (b"sei\0".as_ptr() as *mut i8) as *const i8,
                (b"i\0".as_ptr() as *mut i8) as *const i8,
                None,
            ) != (0 as i32));
        } else {
            __v284 = false as bool;
        }
        let __v285: bool;
        if __v284 {
            __v285 = !(stem(
                std::ptr::addr_of_mut!(z),
                (b"ss\0".as_ptr() as *mut i8) as *const i8,
                (b"ss\0".as_ptr() as *mut i8) as *const i8,
                None,
            ) != (0 as i32));
        } else {
            __v285 = false as bool;
        }
        if __v285 {
            let __v286: *mut i8 = z;
            let __v287: *mut i8 = unsafe { __v286.offset((1 as i32) as isize) };
            z = __v287;
        }
    }
    // Step 1b
    z2 = z;
    if stem(
        std::ptr::addr_of_mut!(z),
        (b"dee\0".as_ptr() as *mut i8) as *const i8,
        (b"ee\0".as_ptr() as *mut i8) as *const i8,
        Some(m_gt_0),
    ) != (0 as i32)
    {
        // Do nothing.  The work was all in the test
    } else {
        let __v288: bool;
        if stem(
            std::ptr::addr_of_mut!(z),
            (b"gni\0".as_ptr() as *mut i8) as *const i8,
            (b"\0".as_ptr() as *mut i8) as *const i8,
            Some(hasVowel),
        ) != (0 as i32)
        {
            __v288 = true as bool;
        } else {
            __v288 = stem(
                std::ptr::addr_of_mut!(z),
                (b"de\0".as_ptr() as *mut i8) as *const i8,
                (b"\0".as_ptr() as *mut i8) as *const i8,
                Some(hasVowel),
            ) != (0 as i32);
        }
        if __v288 && z != z2 {
            let __v289: bool;
            if stem(
                std::ptr::addr_of_mut!(z),
                (b"ta\0".as_ptr() as *mut i8) as *const i8,
                (b"ate\0".as_ptr() as *mut i8) as *const i8,
                None,
            ) != (0 as i32)
            {
                __v289 = true as bool;
            } else {
                __v289 = stem(
                    std::ptr::addr_of_mut!(z),
                    (b"lb\0".as_ptr() as *mut i8) as *const i8,
                    (b"ble\0".as_ptr() as *mut i8) as *const i8,
                    None,
                ) != (0 as i32);
            }
            let __v290: bool;
            if __v289 {
                __v290 = true as bool;
            } else {
                __v290 = stem(
                    std::ptr::addr_of_mut!(z),
                    (b"zi\0".as_ptr() as *mut i8) as *const i8,
                    (b"ize\0".as_ptr() as *mut i8) as *const i8,
                    None,
                ) != (0 as i32);
            }
            if __v290 {
                // Do nothing.  The work was all in the test
            } else {
                if doubleConsonant(z as *const i8) != (0 as i32)
                    && (((unsafe { *z }) as i32) != (108 as i32)
                        && ((unsafe { *z }) as i32) != (115 as i32)
                        && ((unsafe { *z }) as i32) != (122 as i32))
                {
                    let __v291: *mut i8 = z;
                    let __v292: *mut i8 = unsafe { __v291.offset((1 as i32) as isize) };
                    z = __v292;
                } else {
                    let __v293: bool;
                    if m_eq_1(z as *const i8) != (0 as i32) {
                        __v293 = star_oh(z as *const i8) != (0 as i32);
                    } else {
                        __v293 = false as bool;
                    }
                    if __v293 {
                        let __v294: *mut i8 = z;
                        let __v295: *mut i8 = unsafe { __v294.offset(-((1 as i32) as isize)) };
                        z = __v295;
                        unsafe {
                            *__v295 = (101 as i32) as i8;
                        }
                    }
                }
            }
        }
    }
    // Step 1c
    let __v296: bool;
    if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (121 as i32) {
        __v296 = hasVowel((unsafe { z.offset((1 as i32) as isize) }) as *const i8) != (0 as i32);
    } else {
        __v296 = false as bool;
    }
    if __v296 {
        unsafe {
            *unsafe { z.offset((0 as i32) as isize) } = (105 as i32) as i8;
        }
    }
    // Step 2
    '__slate_break_163: {
        match (unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as i32 {
            97 => {
                if !(stem(
                    std::ptr::addr_of_mut!(z),
                    (b"lanoita\0".as_ptr() as *mut i8) as *const i8,
                    (b"ate\0".as_ptr() as *mut i8) as *const i8,
                    Some(m_gt_0),
                ) != (0 as i32))
                {
                    stem(
                        std::ptr::addr_of_mut!(z),
                        (b"lanoit\0".as_ptr() as *mut i8) as *const i8,
                        (b"tion\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    );
                }
            }
            99 => {
                if !(stem(
                    std::ptr::addr_of_mut!(z),
                    (b"icne\0".as_ptr() as *mut i8) as *const i8,
                    (b"ence\0".as_ptr() as *mut i8) as *const i8,
                    Some(m_gt_0),
                ) != (0 as i32))
                {
                    stem(
                        std::ptr::addr_of_mut!(z),
                        (b"icna\0".as_ptr() as *mut i8) as *const i8,
                        (b"ance\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    );
                }
            }
            101 => {
                stem(
                    std::ptr::addr_of_mut!(z),
                    (b"rezi\0".as_ptr() as *mut i8) as *const i8,
                    (b"ize\0".as_ptr() as *mut i8) as *const i8,
                    Some(m_gt_0),
                );
            }
            103 => {
                stem(
                    std::ptr::addr_of_mut!(z),
                    (b"igol\0".as_ptr() as *mut i8) as *const i8,
                    (b"log\0".as_ptr() as *mut i8) as *const i8,
                    Some(m_gt_0),
                );
            }
            108 => {
                let __v297: bool;
                if !(stem(
                    std::ptr::addr_of_mut!(z),
                    (b"ilb\0".as_ptr() as *mut i8) as *const i8,
                    (b"ble\0".as_ptr() as *mut i8) as *const i8,
                    Some(m_gt_0),
                ) != (0 as i32))
                {
                    __v297 = !(stem(
                        std::ptr::addr_of_mut!(z),
                        (b"illa\0".as_ptr() as *mut i8) as *const i8,
                        (b"al\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    ) != (0 as i32));
                } else {
                    __v297 = false as bool;
                }
                let __v298: bool;
                if __v297 {
                    __v298 = !(stem(
                        std::ptr::addr_of_mut!(z),
                        (b"iltne\0".as_ptr() as *mut i8) as *const i8,
                        (b"ent\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    ) != (0 as i32));
                } else {
                    __v298 = false as bool;
                }
                let __v299: bool;
                if __v298 {
                    __v299 = !(stem(
                        std::ptr::addr_of_mut!(z),
                        (b"ile\0".as_ptr() as *mut i8) as *const i8,
                        (b"e\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    ) != (0 as i32));
                } else {
                    __v299 = false as bool;
                }
                if __v299 {
                    stem(
                        std::ptr::addr_of_mut!(z),
                        (b"ilsuo\0".as_ptr() as *mut i8) as *const i8,
                        (b"ous\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    );
                }
            }
            111 => {
                let __v300: bool;
                if !(stem(
                    std::ptr::addr_of_mut!(z),
                    (b"noitazi\0".as_ptr() as *mut i8) as *const i8,
                    (b"ize\0".as_ptr() as *mut i8) as *const i8,
                    Some(m_gt_0),
                ) != (0 as i32))
                {
                    __v300 = !(stem(
                        std::ptr::addr_of_mut!(z),
                        (b"noita\0".as_ptr() as *mut i8) as *const i8,
                        (b"ate\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    ) != (0 as i32));
                } else {
                    __v300 = false as bool;
                }
                if __v300 {
                    stem(
                        std::ptr::addr_of_mut!(z),
                        (b"rota\0".as_ptr() as *mut i8) as *const i8,
                        (b"ate\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    );
                }
            }
            115 => {
                let __v301: bool;
                if !(stem(
                    std::ptr::addr_of_mut!(z),
                    (b"msila\0".as_ptr() as *mut i8) as *const i8,
                    (b"al\0".as_ptr() as *mut i8) as *const i8,
                    Some(m_gt_0),
                ) != (0 as i32))
                {
                    __v301 = !(stem(
                        std::ptr::addr_of_mut!(z),
                        (b"ssenevi\0".as_ptr() as *mut i8) as *const i8,
                        (b"ive\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    ) != (0 as i32));
                } else {
                    __v301 = false as bool;
                }
                let __v302: bool;
                if __v301 {
                    __v302 = !(stem(
                        std::ptr::addr_of_mut!(z),
                        (b"ssenluf\0".as_ptr() as *mut i8) as *const i8,
                        (b"ful\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    ) != (0 as i32));
                } else {
                    __v302 = false as bool;
                }
                if __v302 {
                    stem(
                        std::ptr::addr_of_mut!(z),
                        (b"ssensuo\0".as_ptr() as *mut i8) as *const i8,
                        (b"ous\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    );
                }
            }
            116 => {
                let __v303: bool;
                if !(stem(
                    std::ptr::addr_of_mut!(z),
                    (b"itila\0".as_ptr() as *mut i8) as *const i8,
                    (b"al\0".as_ptr() as *mut i8) as *const i8,
                    Some(m_gt_0),
                ) != (0 as i32))
                {
                    __v303 = !(stem(
                        std::ptr::addr_of_mut!(z),
                        (b"itivi\0".as_ptr() as *mut i8) as *const i8,
                        (b"ive\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    ) != (0 as i32));
                } else {
                    __v303 = false as bool;
                }
                if __v303 {
                    stem(
                        std::ptr::addr_of_mut!(z),
                        (b"itilib\0".as_ptr() as *mut i8) as *const i8,
                        (b"ble\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    );
                }
            }
            _ => {}
        }
    }
    // Step 3
    '__slate_break_206: {
        match (unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32 {
            101 => {
                let __v304: bool;
                if !(stem(
                    std::ptr::addr_of_mut!(z),
                    (b"etaci\0".as_ptr() as *mut i8) as *const i8,
                    (b"ic\0".as_ptr() as *mut i8) as *const i8,
                    Some(m_gt_0),
                ) != (0 as i32))
                {
                    __v304 = !(stem(
                        std::ptr::addr_of_mut!(z),
                        (b"evita\0".as_ptr() as *mut i8) as *const i8,
                        (b"\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    ) != (0 as i32));
                } else {
                    __v304 = false as bool;
                }
                if __v304 {
                    stem(
                        std::ptr::addr_of_mut!(z),
                        (b"ezila\0".as_ptr() as *mut i8) as *const i8,
                        (b"al\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    );
                }
            }
            105 => {
                stem(
                    std::ptr::addr_of_mut!(z),
                    (b"itici\0".as_ptr() as *mut i8) as *const i8,
                    (b"ic\0".as_ptr() as *mut i8) as *const i8,
                    Some(m_gt_0),
                );
            }
            108 => {
                if !(stem(
                    std::ptr::addr_of_mut!(z),
                    (b"laci\0".as_ptr() as *mut i8) as *const i8,
                    (b"ic\0".as_ptr() as *mut i8) as *const i8,
                    Some(m_gt_0),
                ) != (0 as i32))
                {
                    stem(
                        std::ptr::addr_of_mut!(z),
                        (b"luf\0".as_ptr() as *mut i8) as *const i8,
                        (b"\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_0),
                    );
                }
            }
            115 => {
                stem(
                    std::ptr::addr_of_mut!(z),
                    (b"ssen\0".as_ptr() as *mut i8) as *const i8,
                    (b"\0".as_ptr() as *mut i8) as *const i8,
                    Some(m_gt_0),
                );
            }
            _ => {}
        }
    }
    // Step 4
    '__slate_break_221: {
        match (unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as i32 {
            97 => {
                let __v305: bool;
                if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (108 as i32) {
                    __v305 = m_gt_1((unsafe { z.offset((2 as i32) as isize) }) as *const i8)
                        != (0 as i32);
                } else {
                    __v305 = false as bool;
                }
                if __v305 {
                    let __v306: *mut i8 = z;
                    let __v307: *mut i8 = unsafe { __v306.offset((2 as i32) as isize) };
                    z = __v307;
                }
            }
            99 => {
                let __v308: bool;
                if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (101 as i32)
                    && ((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as i32)
                        == (110 as i32)
                    && (((unsafe { *unsafe { z.offset((3 as i32) as isize) } }) as i32)
                        == (97 as i32)
                        || ((unsafe { *unsafe { z.offset((3 as i32) as isize) } }) as i32)
                            == (101 as i32))
                {
                    __v308 = m_gt_1((unsafe { z.offset((4 as i32) as isize) }) as *const i8)
                        != (0 as i32);
                } else {
                    __v308 = false as bool;
                }
                if __v308 {
                    let __v309: *mut i8 = z;
                    let __v310: *mut i8 = unsafe { __v309.offset((4 as i32) as isize) };
                    z = __v310;
                }
            }
            101 => {
                let __v311: bool;
                if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (114 as i32) {
                    __v311 = m_gt_1((unsafe { z.offset((2 as i32) as isize) }) as *const i8)
                        != (0 as i32);
                } else {
                    __v311 = false as bool;
                }
                if __v311 {
                    let __v312: *mut i8 = z;
                    let __v313: *mut i8 = unsafe { __v312.offset((2 as i32) as isize) };
                    z = __v313;
                }
            }
            105 => {
                let __v314: bool;
                if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (99 as i32) {
                    __v314 = m_gt_1((unsafe { z.offset((2 as i32) as isize) }) as *const i8)
                        != (0 as i32);
                } else {
                    __v314 = false as bool;
                }
                if __v314 {
                    let __v315: *mut i8 = z;
                    let __v316: *mut i8 = unsafe { __v315.offset((2 as i32) as isize) };
                    z = __v316;
                }
            }
            108 => {
                let __v317: bool;
                if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (101 as i32)
                    && ((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as i32)
                        == (98 as i32)
                    && (((unsafe { *unsafe { z.offset((3 as i32) as isize) } }) as i32)
                        == (97 as i32)
                        || ((unsafe { *unsafe { z.offset((3 as i32) as isize) } }) as i32)
                            == (105 as i32))
                {
                    __v317 = m_gt_1((unsafe { z.offset((4 as i32) as isize) }) as *const i8)
                        != (0 as i32);
                } else {
                    __v317 = false as bool;
                }
                if __v317 {
                    let __v318: *mut i8 = z;
                    let __v319: *mut i8 = unsafe { __v318.offset((4 as i32) as isize) };
                    z = __v319;
                }
            }
            110 => {
                if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (116 as i32) {
                    if ((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as i32)
                        == (97 as i32)
                    {
                        if m_gt_1((unsafe { z.offset((3 as i32) as isize) }) as *const i8)
                            != (0 as i32)
                        {
                            let __v320: *mut i8 = z;
                            let __v321: *mut i8 = unsafe { __v320.offset((3 as i32) as isize) };
                            z = __v321;
                        }
                    } else {
                        if ((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as i32)
                            == (101 as i32)
                        {
                            let __v322: bool;
                            if !(stem(
                                std::ptr::addr_of_mut!(z),
                                (b"tneme\0".as_ptr() as *mut i8) as *const i8,
                                (b"\0".as_ptr() as *mut i8) as *const i8,
                                Some(m_gt_1),
                            ) != (0 as i32))
                            {
                                __v322 = !(stem(
                                    std::ptr::addr_of_mut!(z),
                                    (b"tnem\0".as_ptr() as *mut i8) as *const i8,
                                    (b"\0".as_ptr() as *mut i8) as *const i8,
                                    Some(m_gt_1),
                                ) != (0 as i32));
                            } else {
                                __v322 = false as bool;
                            }
                            if __v322 {
                                stem(
                                    std::ptr::addr_of_mut!(z),
                                    (b"tne\0".as_ptr() as *mut i8) as *const i8,
                                    (b"\0".as_ptr() as *mut i8) as *const i8,
                                    Some(m_gt_1),
                                );
                            }
                        }
                    }
                }
            }
            111 => {
                if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (117 as i32) {
                    if m_gt_1((unsafe { z.offset((2 as i32) as isize) }) as *const i8) != (0 as i32)
                    {
                        let __v323: *mut i8 = z;
                        let __v324: *mut i8 = unsafe { __v323.offset((2 as i32) as isize) };
                        z = __v324;
                    }
                } else {
                    if ((unsafe { *unsafe { z.offset((3 as i32) as isize) } }) as i32)
                        == (115 as i32)
                        || ((unsafe { *unsafe { z.offset((3 as i32) as isize) } }) as i32)
                            == (116 as i32)
                    {
                        stem(
                            std::ptr::addr_of_mut!(z),
                            (b"noi\0".as_ptr() as *mut i8) as *const i8,
                            (b"\0".as_ptr() as *mut i8) as *const i8,
                            Some(m_gt_1),
                        );
                    }
                }
            }
            115 => {
                let __v325: bool;
                if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (109 as i32)
                    && ((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as i32)
                        == (105 as i32)
                {
                    __v325 = m_gt_1((unsafe { z.offset((3 as i32) as isize) }) as *const i8)
                        != (0 as i32);
                } else {
                    __v325 = false as bool;
                }
                if __v325 {
                    let __v326: *mut i8 = z;
                    let __v327: *mut i8 = unsafe { __v326.offset((3 as i32) as isize) };
                    z = __v327;
                }
            }
            116 => {
                if !(stem(
                    std::ptr::addr_of_mut!(z),
                    (b"eta\0".as_ptr() as *mut i8) as *const i8,
                    (b"\0".as_ptr() as *mut i8) as *const i8,
                    Some(m_gt_1),
                ) != (0 as i32))
                {
                    stem(
                        std::ptr::addr_of_mut!(z),
                        (b"iti\0".as_ptr() as *mut i8) as *const i8,
                        (b"\0".as_ptr() as *mut i8) as *const i8,
                        Some(m_gt_1),
                    );
                }
            }
            117 => {
                let __v328: bool;
                if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (115 as i32)
                    && ((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as i32)
                        == (111 as i32)
                {
                    __v328 = m_gt_1((unsafe { z.offset((3 as i32) as isize) }) as *const i8)
                        != (0 as i32);
                } else {
                    __v328 = false as bool;
                }
                if __v328 {
                    let __v329: *mut i8 = z;
                    let __v330: *mut i8 = unsafe { __v329.offset((3 as i32) as isize) };
                    z = __v330;
                }
            }
            118 | 122 => {
                let __v331: bool;
                if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (101 as i32)
                    && ((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as i32)
                        == (105 as i32)
                {
                    __v331 = m_gt_1((unsafe { z.offset((3 as i32) as isize) }) as *const i8)
                        != (0 as i32);
                } else {
                    __v331 = false as bool;
                }
                if __v331 {
                    let __v332: *mut i8 = z;
                    let __v333: *mut i8 = unsafe { __v332.offset((3 as i32) as isize) };
                    z = __v333;
                }
            }
            _ => {}
        }
    }
    // Step 5a
    if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (101 as i32) {
        if m_gt_1((unsafe { z.offset((1 as i32) as isize) }) as *const i8) != (0 as i32) {
            let __v334: *mut i8 = z;
            let __v335: *mut i8 = unsafe { __v334.offset((1 as i32) as isize) };
            z = __v335;
        } else {
            let __v336: bool;
            if m_eq_1((unsafe { z.offset((1 as i32) as isize) }) as *const i8) != (0 as i32) {
                __v336 = !(star_oh((unsafe { z.offset((1 as i32) as isize) }) as *const i8)
                    != (0 as i32));
            } else {
                __v336 = false as bool;
            }
            if __v336 {
                let __v337: *mut i8 = z;
                let __v338: *mut i8 = unsafe { __v337.offset((1 as i32) as isize) };
                z = __v338;
            }
        }
    }
    // Step 5b
    if m_gt_1(z as *const i8) != (0 as i32)
        && ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (108 as i32)
        && ((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as i32) == (108 as i32)
    {
        let __v339: *mut i8 = z;
        let __v340: *mut i8 = unsafe { __v339.offset((1 as i32) as isize) };
        z = __v340;
    }
    // z[] is now the stemmed word in reverse order.  Flip it back
    // around into forward order and return.
    let __v341: i32 = ((unsafe { strlen(z as *const i8) }) as u32) as i32;
    i = __v341;
    unsafe {
        *pnOut = __v341;
    }
    unsafe {
        *unsafe { zOut.offset(i as isize) } = (0 as i32) as i8;
    }
    '__slate_break_234: while (unsafe { *z }) != (0 as i8) {
        let __v342: *mut i8 = z;
        let __v343: *mut i8 = unsafe { __v342.offset((1 as i32) as isize) };
        z = __v343;
        let __v344: i32 = i;
        let __v345: i32 = __v344 - (1 as i32);
        i = __v345;
        unsafe {
            *unsafe { zOut.offset(__v345 as isize) } = unsafe { *__v342 };
        }
    }
}

/// Characters that can be part of a token.  We assume any character
/// whose value is greater than 0x80 (any UTF character) can be
/// part of a token.  In other words, delimiters all must have
/// values of 0x7f or lower.
/// x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 xA xB xC xD xE xF
/// 3x
/// 4x
/// 5x
/// 6x
/// 7x
static mut porterIdChar: __SlateAlign16<[i8; 80]> = __SlateAlign16([
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

/// Extract the next token from a tokenization cursor.  The cursor must
/// have been opened by a prior call to porterOpen().
///
/// # Arguments
///
/// * `pCursor` - Cursor returned by porterOpen
/// * `pzToken` - OUT: *pzToken is the token text
/// * `pnBytes` - OUT: Number of bytes in token
/// * `piStartOffset` - OUT: Starting offset of token
/// * `piEndOffset` - OUT: Ending offset of token
/// * `piPosition` - OUT: Position integer of token
#[unsafe(link_section = ".text.slate_distinct.fts3_porter.porterNext")]
extern "C-unwind" fn porterNext(
    mut pCursor: *mut sqlite3_tokenizer_cursor,
    mut pzToken: *mut *const i8,
    mut pnBytes: *mut i32,
    mut piStartOffset: *mut i32,
    mut piEndOffset: *mut i32,
    mut piPosition: *mut i32,
) -> i32 {
    let mut c: *mut porter_tokenizer_cursor = pCursor as *mut porter_tokenizer_cursor;
    let mut z: *const i8 = unsafe { (*c).zInput };
    '__slate_break_235: while (unsafe { (*c).iOffset }) < unsafe { (*c).nInput } {
        let mut iStartOffset: i32 = 0 as i32;
        let mut ch: i32 = 0 as i32;
        // Scan past delimiter characters
        '__slate_break_236: loop {
            let __v346: bool;
            if (unsafe { (*c).iOffset }) < unsafe { (*c).nInput } {
                let __v347: i32 =
                    (unsafe { *unsafe { z.offset((unsafe { (*c).iOffset }) as isize) } }) as i32;
                ch = __v347;
                __v346 = __v347 & (128 as i32) == (0 as i32)
                    && (ch < (48 as i32)
                        || !((unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(porterIdChar.0) as *const i8 }
                                    .offset((ch - (48 as i32)) as isize)
                            }
                        }) != (0 as i8)));
            } else {
                __v346 = false as bool;
            }
            if !__v346 {
                break;
            }
            let __v348: *mut porter_tokenizer_cursor = c;
            let __v349: i32 = unsafe { (*__v348).iOffset };
            let __v350: i32 = __v349 + (1 as i32);
            unsafe {
                (*__v348).iOffset = __v350;
            }
        }
        // Count non-delimiter characters.
        iStartOffset = unsafe { (*c).iOffset };
        '__slate_break_237: loop {
            let __v351: bool;
            if (unsafe { (*c).iOffset }) < unsafe { (*c).nInput } {
                let __v352: i32 =
                    (unsafe { *unsafe { z.offset((unsafe { (*c).iOffset }) as isize) } }) as i32;
                ch = __v352;
                __v351 = !(__v352 & (128 as i32) == (0 as i32)
                    && (ch < (48 as i32)
                        || !((unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(porterIdChar.0) as *const i8 }
                                    .offset((ch - (48 as i32)) as isize)
                            }
                        }) != (0 as i8))));
            } else {
                __v351 = false as bool;
            }
            if !__v351 {
                break;
            }
            let __v353: *mut porter_tokenizer_cursor = c;
            let __v354: i32 = unsafe { (*__v353).iOffset };
            let __v355: i32 = __v354 + (1 as i32);
            unsafe {
                (*__v353).iOffset = __v355;
            }
        }
        if (unsafe { (*c).iOffset }) > iStartOffset {
            let mut n: i32 = (unsafe { (*c).iOffset }) - iStartOffset;
            if n > unsafe { (*c).nAllocated } {
                let mut pNew: *mut i8 = unsafe { std::mem::zeroed() };
                unsafe {
                    (*c).nAllocated = n + (20 as i32);
                }
                pNew = (unsafe {
                    sqlite3_realloc64(
                        (unsafe { (*c).zToken }) as *mut (),
                        ((unsafe { (*c).nAllocated }) as i64) as u64,
                    )
                }) as *mut i8;
                if !(pNew != std::ptr::null_mut::<i8>()) {
                    return 7 as i32;
                }
                unsafe {
                    (*c).zToken = pNew;
                }
            }
            porter_stemmer(
                unsafe { z.offset(iStartOffset as isize) },
                n,
                unsafe { (*c).zToken },
                pnBytes,
            );
            unsafe {
                *pzToken = (unsafe { (*c).zToken }) as *const i8;
            }
            unsafe {
                *piStartOffset = iStartOffset;
            }
            unsafe {
                *piEndOffset = unsafe { (*c).iOffset };
            }
            let __v356: *mut porter_tokenizer_cursor = c;
            let __v357: i32 = unsafe { (*__v356).iToken };
            let __v358: i32 = __v357 + (1 as i32);
            unsafe {
                (*__v356).iToken = __v358;
            }
            unsafe {
                *piPosition = __v357;
            }
            return 0 as i32;
        }
    }
    return 101 as i32;
}

/// The set of routines that implement the porter-stemmer tokenizer
static mut porterTokenizerModule: sqlite3_tokenizer_module = sqlite3_tokenizer_module {
    iVersion: 0 as i32,
    xCreate: Some(porterCreate),
    xDestroy: Some(porterDestroy),
    xOpen: Some(porterOpen),
    xClose: Some(porterClose),
    xNext: Some(porterNext),
    xLanguageid: None,
};

/// Allocate a new porter tokenizer.  Return a pointer to the new
/// tokenizer in *ppModule
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3PorterTokenizerModule(
    mut ppModule: *mut *const sqlite3_tokenizer_module,
) {
    unsafe {
        *ppModule = unsafe { std::ptr::addr_of!(porterTokenizerModule) };
    }
}
