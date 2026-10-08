//! 2009 Oct 23
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
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3_snprintf(__v552: i32, __v553: *mut i8, __v554: *const i8, ...) -> *mut i8;
    fn sqlite3_malloc64(__v555: u64) -> *mut ();
    fn sqlite3_realloc64(__v556: *mut (), __v557: u64) -> *mut ();
    fn sqlite3_free(__v558: *mut ());
    fn sqlite3_column_blob(__v559: *mut sqlite3_stmt, iCol: i32) -> *const ();
    fn sqlite3_column_text(__v561: *mut sqlite3_stmt, iCol: i32) -> *const u8;
    fn sqlite3_column_bytes(__v563: *mut sqlite3_stmt, iCol: i32) -> i32;
    fn sqlite3_column_type(__v565: *mut sqlite3_stmt, iCol: i32) -> i32;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_result_blob(
        __v568: *mut sqlite3_context,
        __v569: *const (),
        __v570: i32,
        __v571: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_error(__v572: *mut sqlite3_context, __v573: *const i8, __v574: i32);
    fn sqlite3_result_error_code(__v575: *mut sqlite3_context, __v576: i32);
    fn sqlite3_result_text(
        __v577: *mut sqlite3_context,
        __v578: *const i8,
        __v579: i32,
        __v580: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3Fts3SelectDoctotal(__v581: *mut Fts3Table, __v582: *mut *mut sqlite3_stmt) -> i32;
    fn sqlite3Fts3SelectDocsize(
        __v583: *mut Fts3Table,
        __v584: i64,
        __v585: *mut *mut sqlite3_stmt,
    ) -> i32;
    fn sqlite3Fts3SegmentsClose(__v586: *mut Fts3Table);
    fn sqlite3Fts3ErrMsg(__v587: *mut *mut i8, __v588: *const i8, ...);
    fn sqlite3Fts3GetVarint(__v589: *const i8, __v590: *mut i64) -> i32;
    fn sqlite3Fts3GetVarintBounded(__v591: *const i8, __v592: *const i8, __v593: *mut i64) -> i32;
    fn sqlite3Fts3GetVarint32(__v594: *const i8, __v595: *mut i32) -> i32;
    fn sqlite3Fts3EvalPhraseStats(
        __v596: *mut Fts3Cursor,
        __v597: *mut Fts3Expr,
        __v598: *mut u32,
    ) -> i32;
    fn sqlite3Fts3EvalTestDeferred(pCsr: *mut Fts3Cursor, pRc: *mut i32) -> i32;
    fn sqlite3Fts3MallocZero(nByte: i64) -> *mut ();
    fn sqlite3Fts3OpenTokenizer(
        __v615: *mut sqlite3_tokenizer,
        __v616: i32,
        __v617: *const i8,
        __v618: i32,
        __v619: *mut *mut sqlite3_tokenizer_cursor,
    ) -> i32;
    fn sqlite3Fts3EvalPhrasePoslist(
        __v620: *mut Fts3Cursor,
        __v621: *mut Fts3Expr,
        iCol: i32,
        __v623: *mut *mut i8,
    ) -> i32;
    fn sqlite3Fts3MsrCancel(__v624: *mut Fts3Cursor, __v625: *mut Fts3Expr) -> i32;
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

// Characters that may appear in the second argument to matchinfo().
// 1 value
// 1 value
// 1 value
// nCol values
// nCol values
// nCol values
// 3*nCol*nPhrase values
// nCol*nPhrase values
// nCol*nPhrase values
// The default value for the second argument to matchinfo().
/// Used as an sqlite3Fts3ExprIterate() context when loading phrase doclists to
/// Fts3Expr.aDoclist[]/nDoclist.
#[repr(C)]
#[derive(Clone, Copy)]
struct LoadDoclistCtx {
    /// FTS3 Cursor
    pCsr: *mut Fts3Cursor,
    /// Number of phrases seen so far
    nPhrase: i32,
    /// Number of tokens seen so far
    nToken: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Index {
    nPrefix: i32,
    hPending: Fts3Hash,
}

/// The following types are used as part of the implementation of the
/// fts3BestSnippet() routine.
#[repr(C)]
#[derive(Clone, Copy)]
struct SnippetIter {
    /// Cursor snippet is being generated from
    pCsr: *mut Fts3Cursor,
    /// Extract snippet from this column
    iCol: i32,
    /// Requested snippet length (in tokens)
    nSnippet: i32,
    /// Number of phrases in query
    nPhrase: i32,
    /// Array of size nPhrase
    aPhrase: *mut SnippetPhrase,
    /// First token of current snippet
    iCurrent: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SnippetPhrase {
    /// Number of tokens in phrase
    nToken: i32,
    /// Pointer to start of phrase position list
    pList: *mut i8,
    /// Next value in position list
    iHead: i64,
    /// Position list data following iHead
    pHead: *mut i8,
    /// Next value in trailing position list
    iTail: i64,
    /// Position list data following iTail
    pTail: *mut i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SnippetFragment {
    /// Column snippet is extracted from
    iCol: i32,
    /// Index of first token in snippet
    iPos: i32,
    /// Mask of query phrases covered
    covered: u64,
    /// Mask of snippet terms to highlight
    hlmask: u64,
}

/// This type is used as an sqlite3Fts3ExprIterate() context object while
/// accumulating the data returned by the matchinfo() function.
#[repr(C)]
#[derive(Clone, Copy)]
struct MatchInfo {
    /// FTS3 Cursor
    pCursor: *mut Fts3Cursor,
    /// Number of columns in table
    nCol: i32,
    /// Number of matchable phrases in query
    nPhrase: i32,
    /// Number of docs in database
    nDoc: i64,
    flag: i8,
    /// Pre-allocated buffer
    aMatchinfo: *mut u32,
}

/// An instance of this structure is used to manage a pair of buffers, each
/// (nElem * sizeof(u32)) bytes in size. See the MatchinfoBuffer code below
/// for details.
#[repr(C)]
#[derive(Clone, Copy)]
struct MatchinfoBuffer {
    aRef: [u8; 3],
    nElem: i32,
    /// Set if global data is loaded
    bGlobal: i32,
    zMatchinfo: *mut i8,
    aMI: [u32; 0],
}

// Size (in bytes) of a MatchinfoBuffer sufficient for N elements
/// The snippet() and offsets() functions both return text values. An instance
/// of the following structure is used to accumulate those values while the
/// functions are running. See fts3StringAppend() for details.
#[repr(C)]
#[derive(Clone, Copy)]
struct StrBuffer {
    /// Pointer to buffer containing string
    z: *mut i8,
    /// Length of z in bytes (excl. nul-term)
    n: i32,
    /// Allocated size of buffer z in bytes
    nAlloc: i32,
}

// Start of MatchinfoBuffer code.
/// Allocate a two-slot MatchinfoBuffer object.
fn fts3MIBufferNew(mut nElem: i64, mut zMatchinfo: *const i8) -> *mut MatchinfoBuffer {
    let mut pRet: *mut MatchinfoBuffer = unsafe { std::mem::zeroed() };
    let mut nByte: i64 = (4 as u64)
        .wrapping_mul((((2 as i32) as i64) * nElem + ((1 as i32) as i64)) as u64)
        .wrapping_add((24 as u64).wrapping_add(
            (((((1 as i32) + (1 as i32)) / (2 as i32)) as i64) as u64).wrapping_mul(8 as u64),
        )) as i64;
    let mut nStr: i64 = (unsafe { strlen(zMatchinfo) }) as i64;
    pRet = (unsafe { sqlite3Fts3MallocZero(nByte + nStr + ((1 as i32) as i64)) })
        as *mut MatchinfoBuffer;
    if pRet != std::ptr::null_mut::<MatchinfoBuffer>() {
        unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!((*pRet).aMI) as *mut u32 }
                    .offset((0 as i32) as isize)
            } = (((unsafe {
                ((unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pRet).aMI) as *mut u32 }
                        .offset((1 as i32) as isize)
                }) as *mut u8)
                    .offset_from((pRet as *mut u8) as *mut u8)
            }) as i64) as i32) as u32;
        }
        unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!((*pRet).aMI) as *mut u32 }
                    .offset((((1 as i32) as i64) + nElem) as isize)
            } = ((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pRet).aMI) as *mut u32 }
                        .offset((0 as i32) as isize)
                }
            }) as u64)
                .wrapping_add(
                    (4 as u64).wrapping_mul((((nElem as i32) + (1 as i32)) as i64) as u64),
                ) as u32;
        }
        unsafe {
            (*pRet).nElem = nElem as i32;
        }
        unsafe {
            (*pRet).zMatchinfo = unsafe { (pRet as *mut i8).offset(nByte as isize) };
        }
        unsafe {
            memcpy(
                (unsafe { (*pRet).zMatchinfo }) as *mut (),
                zMatchinfo as *const (),
                (nStr + ((1 as i32) as i64)) as u64,
            )
        };
        unsafe {
            *unsafe {
                unsafe { (*pRet).aRef.as_mut_ptr() as *mut u8 }.offset((0 as i32) as isize)
            } = ((1 as i32) as i8) as u8;
        }
    }
    return pRet;
}

#[unsafe(link_section = ".text.slate_distinct.fts3_snippet.fts3MIBufferFree")]
extern "C-unwind" fn fts3MIBufferFree(mut p: *mut ()) {
    let mut pBuf: *mut MatchinfoBuffer = (unsafe {
        (p as *mut u8).offset(
            -((unsafe { *unsafe { (p as *mut u32).offset(-(1 as i32) as isize) } }) as isize),
        )
    }) as *mut MatchinfoBuffer;
    0 as i32;
    if (p as *mut u32)
        == unsafe {
            unsafe { std::ptr::addr_of_mut!((*pBuf).aMI) as *mut u32 }.offset((1 as i32) as isize)
        }
    {
        unsafe {
            *unsafe {
                unsafe { (*pBuf).aRef.as_mut_ptr() as *mut u8 }.offset((1 as i32) as isize)
            } = ((0 as i32) as i8) as u8;
        }
    } else {
        unsafe {
            *unsafe {
                unsafe { (*pBuf).aRef.as_mut_ptr() as *mut u8 }.offset((2 as i32) as isize)
            } = ((0 as i32) as i8) as u8;
        }
    }
    if (((unsafe {
        *unsafe { unsafe { (*pBuf).aRef.as_mut_ptr() as *mut u8 }.offset((0 as i32) as isize) }
    }) as u32) as i32)
        == (0 as i32)
        && (((unsafe {
            *unsafe { unsafe { (*pBuf).aRef.as_mut_ptr() as *mut u8 }.offset((1 as i32) as isize) }
        }) as u32) as i32)
            == (0 as i32)
        && (((unsafe {
            *unsafe { unsafe { (*pBuf).aRef.as_mut_ptr() as *mut u8 }.offset((2 as i32) as isize) }
        }) as u32) as i32)
            == (0 as i32)
    {
        unsafe { sqlite3_free(pBuf as *mut ()) };
    }
}

fn fts3MIBufferAlloc(
    mut p: *mut MatchinfoBuffer,
    mut paOut: *mut *mut u32,
) -> Option<unsafe extern "C-unwind" fn(*mut ())> {
    let mut xRet: Option<unsafe extern "C-unwind" fn(*mut ())> = None;
    let mut aOut: *mut u32 = std::ptr::null_mut::<u32>();
    if (((unsafe {
        *unsafe { unsafe { (*p).aRef.as_mut_ptr() as *mut u8 }.offset((1 as i32) as isize) }
    }) as u32) as i32)
        == (0 as i32)
    {
        unsafe {
            *unsafe { unsafe { (*p).aRef.as_mut_ptr() as *mut u8 }.offset((1 as i32) as isize) } =
                ((1 as i32) as i8) as u8;
        }
        aOut = unsafe {
            unsafe { std::ptr::addr_of_mut!((*p).aMI) as *mut u32 }.offset((1 as i32) as isize)
        };
        xRet = Some(fts3MIBufferFree);
    } else {
        if (((unsafe {
            *unsafe { unsafe { (*p).aRef.as_mut_ptr() as *mut u8 }.offset((2 as i32) as isize) }
        }) as u32) as i32)
            == (0 as i32)
        {
            unsafe {
                *unsafe {
                    unsafe { (*p).aRef.as_mut_ptr() as *mut u8 }.offset((2 as i32) as isize)
                } = ((1 as i32) as i8) as u8;
            }
            aOut = unsafe {
                unsafe { std::ptr::addr_of_mut!((*p).aMI) as *mut u32 }
                    .offset(((unsafe { (*p).nElem }) + (2 as i32)) as isize)
            };
            xRet = Some(fts3MIBufferFree);
        } else {
            aOut = (unsafe {
                sqlite3_malloc64((((unsafe { (*p).nElem }) as i64) as u64).wrapping_mul(4 as u64))
            }) as *mut u32;
            if aOut != std::ptr::null_mut::<u32>() {
                xRet = unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        sqlite3_free as *const (),
                    )
                };
                if (unsafe { (*p).bGlobal }) != (0 as i32) {
                    unsafe {
                        memcpy(
                            aOut as *mut (),
                            (unsafe {
                                unsafe { std::ptr::addr_of_mut!((*p).aMI) as *mut u32 }
                                    .offset((1 as i32) as isize)
                            }) as *const (),
                            (((unsafe { (*p).nElem }) as i64) as u64).wrapping_mul(4 as u64),
                        )
                    };
                }
            }
        }
    }
    unsafe {
        *paOut = aOut;
    }
    return xRet;
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

fn fts3MIBufferSetGlobal(mut p: *mut MatchinfoBuffer) {
    unsafe {
        (*p).bGlobal = 1 as i32;
    }
    unsafe {
        memcpy(
            (unsafe {
                unsafe { std::ptr::addr_of_mut!((*p).aMI) as *mut u32 }
                    .offset(((2 as i32) + unsafe { (*p).nElem }) as isize)
            }) as *mut (),
            (unsafe {
                unsafe { std::ptr::addr_of_mut!((*p).aMI) as *mut u32 }.offset((1 as i32) as isize)
            }) as *const (),
            (((unsafe { (*p).nElem }) as i64) as u64).wrapping_mul(4 as u64),
        )
    };
}

/// Free a MatchinfoBuffer object allocated using fts3MIBufferNew()
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3MIBufferFree(mut p: *mut MatchinfoBuffer) {
    if p != std::ptr::null_mut::<MatchinfoBuffer>() {
        0 as i32;
        unsafe {
            *unsafe { unsafe { (*p).aRef.as_mut_ptr() as *mut u8 }.offset((0 as i32) as isize) } =
                ((0 as i32) as i8) as u8;
        }
        if (((unsafe {
            *unsafe { unsafe { (*p).aRef.as_mut_ptr() as *mut u8 }.offset((0 as i32) as isize) }
        }) as u32) as i32)
            == (0 as i32)
            && (((unsafe {
                *unsafe { unsafe { (*p).aRef.as_mut_ptr() as *mut u8 }.offset((1 as i32) as isize) }
            }) as u32) as i32)
                == (0 as i32)
            && (((unsafe {
                *unsafe { unsafe { (*p).aRef.as_mut_ptr() as *mut u8 }.offset((2 as i32) as isize) }
            }) as u32) as i32)
                == (0 as i32)
        {
            unsafe { sqlite3_free(p as *mut ()) };
        }
    }
}

// End of MatchinfoBuffer code.
/// This function is used to help iterate through a position-list. A position
/// list is a list of unique integers, sorted from smallest to largest. Each
/// element of the list is represented by an FTS3 varint that takes the value
/// of the difference between the current element and the previous one plus
/// two. For example, to store the position-list:
///
///     4 9 113
///
/// the three varints:
///
///     6 7 106
///
/// are encoded.
///
/// When this function is called, *pp points to the start of an element of
/// the list. *piPos contains the value of the previous entry in the list.
/// After it returns, *piPos contains the value of the next element of the
/// list and *pp is advanced to the following varint.
fn fts3GetDeltaPosition(mut pp: *mut *mut i8, mut piPos: *mut i64) {
    let mut iVal: i32 = 0 as i32;
    let __v685: *mut *mut i8 = pp;
    let __v686: *mut i8 = unsafe { *__v685 };
    let __v687: i32;
    if (((unsafe { *((unsafe { *pp }) as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
        __v687 = unsafe {
            sqlite3Fts3GetVarint32((unsafe { *pp }) as *const i8, std::ptr::addr_of_mut!(iVal))
        };
    } else {
        unsafe {
            *std::ptr::addr_of_mut!(iVal) =
                ((unsafe { *((unsafe { *pp }) as *mut u8) }) as u32) as i32;
        }
        __v687 = 1 as i32;
    }
    let __v688: *mut i8 = unsafe { __v686.offset(__v687 as isize) };
    unsafe {
        *__v685 = __v688;
    }
    let __v689: *mut i64 = piPos;
    let __v690: i64 = unsafe { *__v689 };
    let __v691: i64 = __v690 + ((iVal - (2 as i32)) as i64);
    unsafe {
        *__v689 = __v691;
    }
}

/// Helper function for sqlite3Fts3ExprIterate() (see below).
///
/// # Arguments
///
/// * `pExpr` - Expression to iterate phrases of
/// * `piPhrase` - Pointer to phrase counter
/// * `x` - Callback function to invoke for phrases
/// * `pCtx` - Second argument to pass to callback
fn fts3ExprIterate2(
    mut pExpr: *mut Fts3Expr,
    mut piPhrase: *mut i32,
    mut x: Option<unsafe extern "C-unwind" fn(*mut Fts3Expr, i32, *mut ()) -> i32>,
    mut pCtx: *mut (),
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut eType: i32 = unsafe { (*pExpr).eType }; // Type of expression node pExpr
    if eType != (5 as i32) {
        0 as i32;
        rc = fts3ExprIterate2(unsafe { (*pExpr).pLeft }, piPhrase, x, pCtx);
        if rc == (0 as i32) && eType != (2 as i32) {
            rc = fts3ExprIterate2(unsafe { (*pExpr).pRight }, piPhrase, x, pCtx);
        }
    } else {
        rc = unsafe { x.unwrap()(pExpr, unsafe { *piPhrase }, pCtx) };
        let __v692: *mut i32 = piPhrase;
        let __v693: i32 = unsafe { *__v692 };
        let __v694: i32 = __v693 + (1 as i32);
        unsafe {
            *__v692 = __v694;
        }
    }
    return rc;
}

/// Iterate through all phrase nodes in an FTS3 query, except those that
/// are part of a sub-tree that is the right-hand-side of a NOT operator.
/// For each phrase node found, the supplied callback function is invoked.
///
/// If the callback function returns anything other than SQLITE_OK,
/// the iteration is abandoned and the error code returned immediately.
/// Otherwise, SQLITE_OK is returned after a callback has been made for
/// all eligible phrase nodes.
///
/// # Arguments
///
/// * `pExpr` - Expression to iterate phrases of
/// * `x` - Callback function to invoke for phrases
/// * `pCtx` - Second argument to pass to callback
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3ExprIterate(
    mut pExpr: *mut Fts3Expr,
    mut x: Option<unsafe extern "C-unwind" fn(*mut Fts3Expr, i32, *mut ()) -> i32>,
    mut pCtx: *mut (),
) -> i32 {
    let mut iPhrase: i32 = 0 as i32; // Variable used as the phrase counter
    return fts3ExprIterate2(pExpr, std::ptr::addr_of_mut!(iPhrase), x, pCtx);
}

/// This is an sqlite3Fts3ExprIterate() callback used while loading the
/// doclists for each phrase into Fts3Expr.aDoclist[]/nDoclist. See also
/// fts3ExprLoadDoclists().
#[unsafe(link_section = ".text.slate_distinct.fts3_snippet.fts3ExprLoadDoclistsCb")]
extern "C-unwind" fn fts3ExprLoadDoclistsCb(
    mut pExpr: *mut Fts3Expr,
    mut iPhrase: i32,
    mut ctx: *mut (),
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pPhrase: *mut Fts3Phrase = unsafe { (*pExpr).pPhrase };
    let mut p: *mut LoadDoclistCtx = ctx as *mut LoadDoclistCtx;
    iPhrase;
    let __v695: *mut LoadDoclistCtx = p;
    let __v696: i32 = unsafe { (*__v695).nPhrase };
    let __v697: i32 = __v696 + (1 as i32);
    unsafe {
        (*__v695).nPhrase = __v697;
    }
    let __v698: *mut LoadDoclistCtx = p;
    let __v699: i32 = unsafe { (*__v698).nToken };
    let __v700: i32 = __v699 + unsafe { (*pPhrase).nToken };
    unsafe {
        (*__v698).nToken = __v700;
    }
    return rc;
}

/// Load the doclists for each phrase in the query associated with FTS3 cursor
/// pCsr.
///
/// If pnPhrase is not NULL, then *pnPhrase is set to the number of matchable
/// phrases in the expression (all phrases except those directly or
/// indirectly descended from the right-hand-side of a NOT operator). If
/// pnToken is not NULL, then it is set to the number of tokens in all
/// matchable phrases of the expression.
///
/// # Arguments
///
/// * `pCsr` - Fts3 cursor for current query
/// * `pnPhrase` - OUT: Number of phrases in query
/// * `pnToken` - OUT: Number of tokens in query
fn fts3ExprLoadDoclists(
    mut pCsr: *mut Fts3Cursor,
    mut pnPhrase: *mut i32,
    mut pnToken: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return Code
    let mut sCtx: LoadDoclistCtx = LoadDoclistCtx {
        pCsr: std::ptr::null_mut::<Fts3Cursor>(),
        nPhrase: 0 as i32,
        nToken: 0 as i32,
    }; // Context for sqlite3Fts3ExprIterate()
    sCtx.pCsr = pCsr;
    rc = sqlite3Fts3ExprIterate(
        unsafe { (*pCsr).pExpr },
        Some(fts3ExprLoadDoclistsCb),
        std::ptr::addr_of_mut!(sCtx) as *mut (),
    );
    if pnPhrase != std::ptr::null_mut::<i32>() {
        unsafe {
            *pnPhrase = sCtx.nPhrase;
        }
    }
    if pnToken != std::ptr::null_mut::<i32>() {
        unsafe {
            *pnToken = sCtx.nToken;
        }
    }
    return rc;
}

#[unsafe(link_section = ".text.slate_distinct.fts3_snippet.fts3ExprPhraseCountCb")]
extern "C-unwind" fn fts3ExprPhraseCountCb(
    mut pExpr: *mut Fts3Expr,
    mut iPhrase: i32,
    mut ctx: *mut (),
) -> i32 {
    let __v701: *mut i32 = ctx as *mut i32;
    let __v702: i32 = unsafe { *__v701 };
    let __v703: i32 = __v702 + (1 as i32);
    unsafe {
        *__v701 = __v703;
    }
    unsafe {
        (*pExpr).iPhrase = iPhrase;
    }
    return 0 as i32;
}

fn fts3ExprPhraseCount(mut pExpr: *mut Fts3Expr) -> i32 {
    let mut nPhrase: i32 = 0 as i32;
    sqlite3Fts3ExprIterate(
        pExpr,
        Some(fts3ExprPhraseCountCb),
        std::ptr::addr_of_mut!(nPhrase) as *mut (),
    );
    return nPhrase;
}

/// Advance the position list iterator specified by the first two
/// arguments so that it points to the first element with a value greater
/// than or equal to parameter iNext.
fn fts3SnippetAdvance(mut ppIter: *mut *mut i8, mut piIter: *mut i64, mut iNext: i32) {
    let mut pIter: *mut i8 = unsafe { *ppIter };
    if pIter != std::ptr::null_mut::<i8>() {
        let mut iIter: i64 = unsafe { *piIter };
        '__slate_break_629: while iIter < (iNext as i64) {
            if (0 as i32) == ((unsafe { *pIter }) as i32) & (254 as i32) {
                iIter = -(1 as i32) as i64;
                pIter = std::ptr::null_mut::<i8>();
                break '__slate_break_629;
            }
            fts3GetDeltaPosition(std::ptr::addr_of_mut!(pIter), std::ptr::addr_of_mut!(iIter));
        }
        unsafe {
            *piIter = iIter;
        }
        unsafe {
            *ppIter = pIter;
        }
    }
}

/// Advance the snippet iterator to the next candidate snippet.
fn fts3SnippetNextCandidate(mut pIter: *mut SnippetIter) -> i32 {
    let mut i: i32 = 0 as i32; // Loop counter
    if (unsafe { (*pIter).iCurrent }) < (0 as i32) {
        // The SnippetIter object has just been initialized. The first snippet
        // candidate always starts at offset 0 (even if this candidate has a
        // score of 0.0).
        unsafe {
            (*pIter).iCurrent = 0 as i32;
        }
        // Advance the 'head' iterator of each phrase to the first offset that
        // is greater than or equal to (iNext+nSnippet).
        i = 0 as i32;
        '__slate_break_630: loop {
            if !(i < unsafe { (*pIter).nPhrase }) {
                break;
            }
            let mut pPhrase: *mut SnippetPhrase =
                unsafe { unsafe { (*pIter).aPhrase }.offset(i as isize) };
            fts3SnippetAdvance(
                unsafe { std::ptr::addr_of_mut!((*pPhrase).pHead) },
                unsafe { std::ptr::addr_of_mut!((*pPhrase).iHead) },
                unsafe { (*pIter).nSnippet },
            );
            let __v704: i32 = i;
            let __v705: i32 = __v704 + (1 as i32);
            i = __v705;
        }
    } else {
        let mut iStart: i32 = 0 as i32;
        let mut iEnd: i32 = 2147483647 as i32;
        i = 0 as i32;
        '__slate_break_631: loop {
            if !(i < unsafe { (*pIter).nPhrase }) {
                break;
            }
            let mut pPhrase: *mut SnippetPhrase =
                unsafe { unsafe { (*pIter).aPhrase }.offset(i as isize) };
            if (unsafe { (*pPhrase).pHead }) != std::ptr::null_mut::<i8>()
                && (unsafe { (*pPhrase).iHead }) < (iEnd as i64)
            {
                iEnd = (unsafe { (*pPhrase).iHead }) as i32;
            }
            let __v706: i32 = i;
            let __v707: i32 = __v706 + (1 as i32);
            i = __v707;
        }
        if iEnd == (2147483647 as i32) {
            return 1 as i32;
        }
        0 as i32;
        let __v708: i32 = iEnd - unsafe { (*pIter).nSnippet } + (1 as i32);
        iStart = __v708;
        unsafe {
            (*pIter).iCurrent = __v708;
        }
        i = 0 as i32;
        '__slate_break_632: loop {
            if !(i < unsafe { (*pIter).nPhrase }) {
                break;
            }
            let mut pPhrase: *mut SnippetPhrase =
                unsafe { unsafe { (*pIter).aPhrase }.offset(i as isize) };
            fts3SnippetAdvance(
                unsafe { std::ptr::addr_of_mut!((*pPhrase).pHead) },
                unsafe { std::ptr::addr_of_mut!((*pPhrase).iHead) },
                iEnd + (1 as i32),
            );
            fts3SnippetAdvance(
                unsafe { std::ptr::addr_of_mut!((*pPhrase).pTail) },
                unsafe { std::ptr::addr_of_mut!((*pPhrase).iTail) },
                iStart,
            );
            let __v709: i32 = i;
            let __v710: i32 = __v709 + (1 as i32);
            i = __v710;
        }
    }
    return 0 as i32;
}

/// Retrieve information about the current candidate snippet of snippet
/// iterator pIter.
///
/// # Arguments
///
/// * `pIter` - Snippet iterator
/// * `mCovered` - Bitmask of phrases already covered
/// * `piToken` - OUT: First token of proposed snippet
/// * `piScore` - OUT: "Score" for this snippet
/// * `pmCover` - OUT: Bitmask of phrases covered
/// * `pmHighlight` - OUT: Bitmask of terms to highlight
fn fts3SnippetDetails(
    mut pIter: *mut SnippetIter,
    mut mCovered: u64,
    mut piToken: *mut i32,
    mut piScore: *mut i32,
    mut pmCover: *mut u64,
    mut pmHighlight: *mut u64,
) {
    let mut iStart: i32 = unsafe { (*pIter).iCurrent }; // First token of snippet
    let mut iScore: i32 = 0 as i32; // Score of this snippet
    let mut i: i32 = 0 as i32; // Loop counter
    let mut mCover: u64 = ((0 as i32) as i64) as u64; // Mask of phrases covered by this snippet
    let mut mHighlight: u64 = ((0 as i32) as i64) as u64; // Mask of tokens to highlight in snippet
    i = 0 as i32;
    '__slate_break_633: loop {
        if !(i < unsafe { (*pIter).nPhrase }) {
            break;
        }
        let mut pPhrase: *mut SnippetPhrase =
            unsafe { unsafe { (*pIter).aPhrase }.offset(i as isize) };
        if (unsafe { (*pPhrase).pTail }) != std::ptr::null_mut::<i8>() {
            let mut pCsr: *mut i8 = unsafe { (*pPhrase).pTail };
            let mut iCsr: i64 = unsafe { (*pPhrase).iTail };
            '__slate_break_634: while iCsr < ((iStart + unsafe { (*pIter).nSnippet }) as i64)
                && iCsr >= (iStart as i64)
            {
                let mut j: i32 = 0 as i32;
                let mut mPhrase: u64 = (((1 as i32) as i64) as u64) << i % (64 as i32);
                let mut mPos: u64 = (((1 as i32) as i64) as u64) << iCsr - (iStart as i64);
                0 as i32;
                0 as i32;
                if (mCover | mCovered) & mPhrase != (0 as u64) {
                    let __v713: i32 = iScore;
                    let __v714: i32 = __v713 + (1 as i32);
                    iScore = __v714;
                } else {
                    let __v715: i32 = iScore;
                    let __v716: i32 = __v715 + (1000 as i32);
                    iScore = __v716;
                }
                let __v717: u64 = mCover;
                let __v718: u64 = __v717 | mPhrase;
                mCover = __v718;
                j = 0 as i32;
                '__slate_break_635: loop {
                    if !(j < unsafe { (*pPhrase).nToken } && j < unsafe { (*pIter).nSnippet }) {
                        break;
                    }
                    let __v721: u64 = mHighlight;
                    let __v722: u64 = __v721 | mPos >> j;
                    mHighlight = __v722;
                    let __v719: i32 = j;
                    let __v720: i32 = __v719 + (1 as i32);
                    j = __v720;
                }
                if (0 as i32) == ((unsafe { *pCsr }) as i32) & (254 as i32) {
                    break '__slate_break_634;
                }
                fts3GetDeltaPosition(std::ptr::addr_of_mut!(pCsr), std::ptr::addr_of_mut!(iCsr));
            }
        }
        let __v711: i32 = i;
        let __v712: i32 = __v711 + (1 as i32);
        i = __v712;
    }
    // Set the output variables before returning.
    unsafe {
        *piToken = iStart;
    }
    unsafe {
        *piScore = iScore;
    }
    unsafe {
        *pmCover = mCover;
    }
    unsafe {
        *pmHighlight = mHighlight;
    }
}

/// This function is an sqlite3Fts3ExprIterate() callback used by
/// fts3BestSnippet().  Each invocation populates an element of the
/// SnippetIter.aPhrase[] array.
#[unsafe(link_section = ".text.slate_distinct.fts3_snippet.fts3SnippetFindPositions")]
extern "C-unwind" fn fts3SnippetFindPositions(
    mut pExpr: *mut Fts3Expr,
    mut iPhrase: i32,
    mut ctx: *mut (),
) -> i32 {
    let mut p: *mut SnippetIter = ctx as *mut SnippetIter;
    let mut pPhrase: *mut SnippetPhrase =
        unsafe { unsafe { (*p).aPhrase }.offset(iPhrase as isize) };
    let mut pCsr: *mut i8 = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    unsafe {
        (*pPhrase).nToken = unsafe { (*unsafe { (*pExpr).pPhrase }).nToken };
    }
    rc = unsafe {
        sqlite3Fts3EvalPhrasePoslist(
            unsafe { (*p).pCsr },
            pExpr,
            unsafe { (*p).iCol },
            std::ptr::addr_of_mut!(pCsr),
        )
    };
    0 as i32;
    if pCsr != std::ptr::null_mut::<i8>() {
        let mut iFirst: i64 = (0 as i32) as i64;
        unsafe {
            (*pPhrase).pList = pCsr;
        }
        fts3GetDeltaPosition(std::ptr::addr_of_mut!(pCsr), std::ptr::addr_of_mut!(iFirst));
        if iFirst < ((0 as i32) as i64) {
            rc = (11 as i32) | (1 as i32) << (8 as i32);
        } else {
            unsafe {
                (*pPhrase).pHead = pCsr;
            }
            unsafe {
                (*pPhrase).pTail = pCsr;
            }
            unsafe {
                (*pPhrase).iHead = iFirst;
            }
            unsafe {
                (*pPhrase).iTail = iFirst;
            }
        }
    } else {
        0 as i32;
    }
    return rc;
}

/// Select the fragment of text consisting of nFragment contiguous tokens
/// from column iCol that represent the "best" snippet. The best snippet
/// is the snippet with the highest score, where scores are calculated
/// by adding:
///
///   (a) +1 point for each occurrence of a matchable phrase in the snippet.
///
///   (b) +1000 points for the first occurrence of each matchable phrase in
///       the snippet for which the corresponding mCovered bit is not set.
///
/// The selected snippet parameters are stored in structure *pFragment before
/// returning. The score of the selected snippet is stored in *piScore
/// before returning.
///
/// # Arguments
///
/// * `nSnippet` - Desired snippet length
/// * `pCsr` - Cursor to create snippet for
/// * `iCol` - Index of column to create snippet from
/// * `mCovered` - Mask of phrases already covered
/// * `pmSeen` - IN/OUT: Mask of phrases seen
/// * `pFragment` - OUT: Best snippet found
/// * `piScore` - OUT: Score of snippet pFragment
fn fts3BestSnippet(
    mut nSnippet: i32,
    mut pCsr: *mut Fts3Cursor,
    mut iCol: i32,
    mut mCovered: u64,
    mut pmSeen: *mut u64,
    mut pFragment: *mut SnippetFragment,
    mut piScore: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return Code
    let mut nList: i32 = 0 as i32; // Number of phrases in expression
    let mut sIter: SnippetIter = unsafe { std::mem::zeroed() }; // Iterates through snippet candidates
    let mut nByte: i64 = 0 as i64; // Number of bytes of space to allocate
    let mut iBestScore: i32 = -(1 as i32); // Best snippet score found so far
    let mut i: i32 = 0 as i32; // Loop counter
    let mut pExpr: *mut Fts3Expr = unsafe { (*pCsr).pExpr };
    unsafe {
        memset(
            std::ptr::addr_of_mut!(sIter) as *mut (),
            0 as i32,
            40 as u64,
        )
    };
    // Iterate through the phrases in the expression to count them. The same
    // callback makes sure the doclists are loaded for each phrase.
    rc = fts3ExprLoadDoclists(
        pCsr,
        std::ptr::addr_of_mut!(nList),
        std::ptr::null_mut::<i32>(),
    );
    if rc != (0 as i32) {
        return rc;
    }
    // Now that it is known how many phrases there are, allocate and zero
    // the required space using malloc().
    nByte = (48 as u64).wrapping_mul((nList as i64) as u64) as i64;
    sIter.aPhrase = (unsafe { sqlite3Fts3MallocZero(nByte) }) as *mut SnippetPhrase;
    if !(sIter.aPhrase != std::ptr::null_mut::<SnippetPhrase>()) {
        return 7 as i32;
    }
    // Initialize the contents of the SnippetIter object. Then iterate through
    // the set of phrases in the expression to populate the aPhrase[] array.
    sIter.pCsr = pCsr;
    sIter.iCol = iCol;
    sIter.nSnippet = nSnippet;
    sIter.nPhrase = nList;
    sIter.iCurrent = -(1 as i32);
    rc = sqlite3Fts3ExprIterate(
        pExpr,
        Some(fts3SnippetFindPositions),
        std::ptr::addr_of_mut!(sIter) as *mut (),
    );
    if rc == (0 as i32) {
        // Iterate through the expression twice, in case pointers garnered during
        // the first iteration are invalidated by a call to fts5EvalRestart().
        rc = sqlite3Fts3ExprIterate(
            pExpr,
            Some(fts3SnippetFindPositions),
            std::ptr::addr_of_mut!(sIter) as *mut (),
        );
    }
    if rc == (0 as i32) {
        // Set the *pmSeen output variable.
        i = 0 as i32;
        '__slate_break_636: loop {
            if !(i < nList) {
                break;
            }
            if (unsafe { (*unsafe { sIter.aPhrase.offset(i as isize) }).pHead })
                != std::ptr::null_mut::<i8>()
            {
                let __v725: *mut u64 = pmSeen;
                let __v726: u64 = unsafe { *__v725 };
                let __v727: u64 = __v726 | (((1 as i32) as i64) as u64) << i % (64 as i32);
                unsafe {
                    *__v725 = __v727;
                }
            }
            let __v723: i32 = i;
            let __v724: i32 = __v723 + (1 as i32);
            i = __v724;
        }
        // Loop through all candidate snippets. Store the best snippet in
        // *pFragment. Store its associated 'score' in iBestScore.
        unsafe {
            (*pFragment).iCol = iCol;
        }
        '__slate_break_637: while !(fts3SnippetNextCandidate(std::ptr::addr_of_mut!(sIter))
            != (0 as i32))
        {
            let mut iPos: i32 = 0 as i32;
            let mut iScore: i32 = 0 as i32;
            let mut mCover: u64 = 0 as u64;
            let mut mHighlite: u64 = 0 as u64;
            fts3SnippetDetails(
                std::ptr::addr_of_mut!(sIter),
                mCovered,
                std::ptr::addr_of_mut!(iPos),
                std::ptr::addr_of_mut!(iScore),
                std::ptr::addr_of_mut!(mCover),
                std::ptr::addr_of_mut!(mHighlite),
            );
            0 as i32;
            if iScore > iBestScore {
                unsafe {
                    (*pFragment).iPos = iPos;
                }
                unsafe {
                    (*pFragment).hlmask = mHighlite;
                }
                unsafe {
                    (*pFragment).covered = mCover;
                }
                iBestScore = iScore;
            }
        }
        unsafe {
            *piScore = iBestScore;
        }
    }
    unsafe { sqlite3_free(sIter.aPhrase as *mut ()) };
    return rc;
}

/// Append a string to the string-buffer passed as the first argument.
///
/// If nAppend is negative, then the length of the string zAppend is
/// determined using strlen().
///
/// # Arguments
///
/// * `pStr` - Buffer to append to
/// * `zAppend` - Pointer to data to append to buffer
/// * `nAppend` - Size of zAppend in bytes (or -1)
fn fts3StringAppend(mut pStr: *mut StrBuffer, mut zAppend: *const i8, mut nAppend: i32) -> i32 {
    if nAppend < (0 as i32) {
        nAppend = ((unsafe { strlen(zAppend) }) as u32) as i32;
    }
    // If there is insufficient space allocated at StrBuffer.z, use realloc()
    // to grow the buffer until so that it is big enough to accommodate the
    // appended data.
    if ((unsafe { (*pStr).n }) as i64) + (nAppend as i64) + ((1 as i32) as i64)
        >= ((unsafe { (*pStr).nAlloc }) as i64)
    {
        let mut nAlloc: i64 =
            ((unsafe { (*pStr).nAlloc }) as i64) + (nAppend as i64) + ((100 as i32) as i64);
        let mut zNew: *mut i8 =
            (unsafe { sqlite3_realloc64((unsafe { (*pStr).z }) as *mut (), nAlloc as u64) })
                as *mut i8;
        if !(zNew != std::ptr::null_mut::<i8>()) {
            return 7 as i32;
        }
        unsafe {
            (*pStr).z = zNew;
        }
        unsafe {
            (*pStr).nAlloc = nAlloc as i32;
        }
    }
    0 as i32;
    // Append the data to the string buffer.
    unsafe {
        memcpy(
            (unsafe { unsafe { (*pStr).z }.offset((unsafe { (*pStr).n }) as isize) }) as *mut (),
            zAppend as *const (),
            (nAppend as i64) as u64,
        )
    };
    let __v728: *mut StrBuffer = pStr;
    let __v729: i32 = unsafe { (*__v728).n };
    let __v730: i32 = __v729 + nAppend;
    unsafe {
        (*__v728).n = __v730;
    }
    unsafe {
        *unsafe { unsafe { (*pStr).z }.offset((unsafe { (*pStr).n }) as isize) } = (0 as i32) as i8;
    }
    return 0 as i32;
}

/// The fts3BestSnippet() function often selects snippets that end with a
/// query term. That is, the final term of the snippet is always a term
/// that requires highlighting. For example, if 'X' is a highlighted term
/// and '.' is a non-highlighted term, BestSnippet() may select:
///
///     ........X.....X
///
/// This function "shifts" the beginning of the snippet forward in the
/// document so that there are approximately the same number of
/// non-highlighted terms to the right of the final highlighted term as there
/// are to the left of the first highlighted term. For example, to this:
///
///     ....X.....X....
///
/// This is done as part of extracting the snippet text, not when selecting
/// the snippet. Snippet selection is done based on doclists only, so there
/// is no way for fts3BestSnippet() to know whether or not the document
/// actually contains terms that follow the final highlighted term.
///
/// # Arguments
///
/// * `pTab` - FTS3 table snippet comes from
/// * `iLangid` - Language id to use in tokenizing
/// * `nSnippet` - Number of tokens desired for snippet
/// * `zDoc` - Document text to extract snippet from
/// * `nDoc` - Size of buffer zDoc in bytes
/// * `piPos` - IN/OUT: First token of snippet
/// * `pHlmask` - IN/OUT: Mask of tokens to highlight
fn fts3SnippetShift(
    mut pTab: *mut Fts3Table,
    mut iLangid: i32,
    mut nSnippet: i32,
    mut zDoc: *const i8,
    mut nDoc: i32,
    mut piPos: *mut i32,
    mut pHlmask: *mut u64,
) -> i32 {
    let mut hlmask: u64 = unsafe { *pHlmask }; // Local copy of initial highlight-mask
    if hlmask != (0 as u64) {
        let mut nLeft: i32 = 0 as i32; // Tokens to the left of first highlight
        let mut nRight: i32 = 0 as i32; // Tokens to the right of last highlight
        let mut nDesired: i32 = 0 as i32; // Ideal number of tokens to shift forward
        nLeft = 0 as i32;
        '__slate_break_638: loop {
            if !(!(hlmask & (((1 as i32) as i64) as u64) << nLeft != (0 as u64))) {
                break;
            }
            {}
            let __v731: i32 = nLeft;
            let __v732: i32 = __v731 + (1 as i32);
            nLeft = __v732;
        }
        nRight = 0 as i32;
        '__slate_break_639: loop {
            if !(!(hlmask & (((1 as i32) as i64) as u64) << nSnippet - (1 as i32) - nRight
                != (0 as u64)))
            {
                break;
            }
            {}
            let __v733: i32 = nRight;
            let __v734: i32 = __v733 + (1 as i32);
            nRight = __v734;
        }
        0 as i32;
        nDesired = (nLeft - nRight) / (2 as i32);
        // Ideally, the start of the snippet should be pushed forward in the
        // document nDesired tokens. This block checks if there are actually
        // nDesired tokens to the right of the snippet. If so, *piPos and
        // *pHlMask are updated to shift the snippet nDesired tokens to the
        // right. Otherwise, the snippet is shifted by the number of tokens
        // available.
        if nDesired > (0 as i32) {
            let mut nShift: i32 = 0 as i32; // Number of tokens to shift snippet by
            let mut iCurrent: i32 = 0 as i32; // Token counter
            let mut rc: i32 = 0 as i32; // Return Code
            let mut pMod: *mut sqlite3_tokenizer_module = unsafe { std::mem::zeroed() };
            let mut pC: *mut sqlite3_tokenizer_cursor = unsafe { std::mem::zeroed() };
            pMod = (unsafe { (*unsafe { (*pTab).pTokenizer }).pModule })
                as *mut sqlite3_tokenizer_module;
            // Open a cursor on zDoc/nDoc. Check if there are (nSnippet+nDesired)
            // or more tokens in zDoc/nDoc.
            rc = unsafe {
                sqlite3Fts3OpenTokenizer(
                    unsafe { (*pTab).pTokenizer },
                    iLangid,
                    zDoc,
                    nDoc,
                    std::ptr::addr_of_mut!(pC),
                )
            };
            if rc != (0 as i32) {
                return rc;
            }
            '__slate_break_640: while rc == (0 as i32) && iCurrent < nSnippet + nDesired {
                let mut ZDUMMY: *const i8 = unsafe { std::mem::zeroed() };
                let mut DUMMY1: i32 = 0 as i32;
                let mut DUMMY2: i32 = 0 as i32;
                let mut DUMMY3: i32 = 0 as i32;
                rc = unsafe {
                    unsafe { (*pMod).xNext }.unwrap()(
                        pC,
                        std::ptr::addr_of_mut!(ZDUMMY),
                        std::ptr::addr_of_mut!(DUMMY1),
                        std::ptr::addr_of_mut!(DUMMY2),
                        std::ptr::addr_of_mut!(DUMMY3),
                        std::ptr::addr_of_mut!(iCurrent),
                    )
                };
            }
            unsafe { unsafe { (*pMod).xClose }.unwrap()(pC) };
            if rc != (0 as i32) && rc != (101 as i32) {
                return rc;
            }
            nShift = ((rc == (101 as i32)) as i32) + iCurrent - nSnippet;
            0 as i32;
            if nShift > (0 as i32) {
                let __v735: *mut i32 = piPos;
                let __v736: i32 = unsafe { *__v735 };
                let __v737: i32 = __v736 + nShift;
                unsafe {
                    *__v735 = __v737;
                }
                unsafe {
                    *pHlmask = hlmask >> nShift;
                }
            }
        }
    }
    return 0 as i32;
}

/// Extract the snippet text for fragment pFragment from cursor pCsr and
/// append it to string buffer pOut.
///
/// # Arguments
///
/// * `pCsr` - FTS3 Cursor
/// * `pFragment` - Snippet to extract
/// * `iFragment` - Fragment number
/// * `isLast` - True for final fragment in snippet
/// * `nSnippet` - Number of tokens in extracted snippet
/// * `zOpen` - String inserted before highlighted term
/// * `zClose` - String inserted after highlighted term
/// * `zEllipsis` - String inserted between snippets
/// * `pOut` - Write output here
fn fts3SnippetText(
    mut pCsr: *mut Fts3Cursor,
    mut pFragment: *mut SnippetFragment,
    mut iFragment: i32,
    mut isLast: i32,
    mut nSnippet: i32,
    mut zOpen: *const i8,
    mut zClose: *const i8,
    mut zEllipsis: *const i8,
    mut pOut: *mut StrBuffer,
) -> i32 {
    let mut pTab: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
    let mut rc: i32 = 0 as i32; // Return code
    let mut zDoc: *const i8 = unsafe { std::mem::zeroed() }; // Document text to extract snippet from
    let mut nDoc: i32 = 0 as i32; // Size of zDoc in bytes
    let mut iCurrent: i32 = 0 as i32; // Current token number of document
    let mut iEnd: i32 = 0 as i32; // Byte offset of end of current token
    let mut isShiftDone: i32 = 0 as i32; // True after snippet is shifted
    let mut iPos: i32 = unsafe { (*pFragment).iPos }; // First token of snippet
    let mut hlmask: u64 = unsafe { (*pFragment).hlmask }; // Highlight-mask for snippet
    let mut iCol: i32 = (unsafe { (*pFragment).iCol }) + (1 as i32); // Query column to extract text from
    let mut pMod: *mut sqlite3_tokenizer_module = unsafe { std::mem::zeroed() }; // Tokenizer module methods object
    let mut pC: *mut sqlite3_tokenizer_cursor = unsafe { std::mem::zeroed() }; // Tokenizer cursor open on zDoc/nDoc
    zDoc = (unsafe { sqlite3_column_text(unsafe { (*pCsr).pStmt }, iCol) }) as *const i8;
    if zDoc == std::ptr::null::<i8>() {
        if (unsafe { sqlite3_column_type(unsafe { (*pCsr).pStmt }, iCol) }) != (5 as i32) {
            return 7 as i32;
        }
        return 0 as i32;
    }
    nDoc = unsafe { sqlite3_column_bytes(unsafe { (*pCsr).pStmt }, iCol) };
    // Open a token cursor on the document.
    pMod = (unsafe { (*unsafe { (*pTab).pTokenizer }).pModule }) as *mut sqlite3_tokenizer_module;
    rc = unsafe {
        sqlite3Fts3OpenTokenizer(
            unsafe { (*pTab).pTokenizer },
            unsafe { (*pCsr).iLangid },
            zDoc,
            nDoc,
            std::ptr::addr_of_mut!(pC),
        )
    };
    if rc != (0 as i32) {
        return rc;
    }
    '__slate_break_641: while rc == (0 as i32) {
        '__slate_continue_641: {
            let mut ZDUMMY: *const i8 = unsafe { std::mem::zeroed() }; // Dummy argument used with tokenizer
            let mut DUMMY1: i32 = -(1 as i32); // Dummy argument used with tokenizer
            let mut iBegin: i32 = 0 as i32; // Offset in zDoc of start of token
            let mut iFin: i32 = 0 as i32; // Offset in zDoc of end of token
            let mut isHighlight: i32 = 0 as i32; // True for highlighted terms
            // Variable DUMMY1 is initialized to a negative value above. Elsewhere
            // in the FTS code the variable that the third argument to xNext points to
            // is initialized to zero before the first (*but not necessarily
            // subsequent*) call to xNext(). This is done for a particular application
            // that needs to know whether or not the tokenizer is being used for
            // snippet generation or for some other purpose.
            //
            // Extreme care is required when writing code to depend on this
            // initialization. It is not a documented part of the tokenizer interface.
            // If a tokenizer is used directly by any code outside of FTS, this
            // convention might not be respected.
            rc = unsafe {
                unsafe { (*pMod).xNext }.unwrap()(
                    pC,
                    std::ptr::addr_of_mut!(ZDUMMY),
                    std::ptr::addr_of_mut!(DUMMY1),
                    std::ptr::addr_of_mut!(iBegin),
                    std::ptr::addr_of_mut!(iFin),
                    std::ptr::addr_of_mut!(iCurrent),
                )
            };
            if rc != (0 as i32) {
                if rc == (101 as i32) {
                    // Special case - the last token of the snippet is also the last token
                    // of the column. Append any punctuation that occurred between the end
                    // of the previous token and the end of the document to the output.
                    // Then break out of the loop.
                    rc = fts3StringAppend(pOut, unsafe { zDoc.offset(iEnd as isize) }, -(1 as i32));
                }
                break '__slate_break_641;
            }
            if iCurrent < iPos {
            } else {
                if !(isShiftDone != (0 as i32)) {
                    let mut n: i32 = nDoc - iBegin;
                    rc = fts3SnippetShift(
                        pTab,
                        unsafe { (*pCsr).iLangid },
                        nSnippet,
                        unsafe { zDoc.offset(iBegin as isize) },
                        n,
                        std::ptr::addr_of_mut!(iPos),
                        std::ptr::addr_of_mut!(hlmask),
                    );
                    isShiftDone = 1 as i32;
                    // Now that the shift has been done, check if the initial "..." are
                    // required. They are required if (a) this is not the first fragment,
                    // or (b) this fragment does not begin at position 0 of its column.
                    if rc == (0 as i32) {
                        if iPos > (0 as i32) || iFragment > (0 as i32) {
                            rc = fts3StringAppend(pOut, zEllipsis, -(1 as i32));
                        } else {
                            if iBegin != (0 as i32) {
                                rc = fts3StringAppend(pOut, zDoc, iBegin);
                            }
                        }
                    }
                    if rc != (0 as i32) || iCurrent < iPos {
                        break '__slate_continue_641;
                    }
                }
                if iCurrent >= iPos + nSnippet {
                    if isLast != (0 as i32) {
                        rc = fts3StringAppend(pOut, zEllipsis, -(1 as i32));
                    }
                    break '__slate_break_641;
                }
                // Set isHighlight to true if this term should be highlighted.
                isHighlight = (hlmask & (((1 as i32) as i64) as u64) << iCurrent - iPos
                    != (((0 as i32) as i64) as u64)) as i32;
                if iCurrent > iPos {
                    rc = fts3StringAppend(
                        pOut,
                        unsafe { zDoc.offset(iEnd as isize) },
                        iBegin - iEnd,
                    );
                }
                if rc == (0 as i32) && isHighlight != (0 as i32) {
                    rc = fts3StringAppend(pOut, zOpen, -(1 as i32));
                }
                if rc == (0 as i32) {
                    rc = fts3StringAppend(
                        pOut,
                        unsafe { zDoc.offset(iBegin as isize) },
                        iFin - iBegin,
                    );
                }
                if rc == (0 as i32) && isHighlight != (0 as i32) {
                    rc = fts3StringAppend(pOut, zClose, -(1 as i32));
                }
                iEnd = iFin;
            }
        }
    }
    unsafe { unsafe { (*pMod).xClose }.unwrap()(pC) };
    return rc;
}

/// This function is used to count the entries in a column-list (a
/// delta-encoded list of term offsets within a single column of a single
/// row). When this function is called, *ppCollist should point to the
/// beginning of the first varint in the column-list (the varint that
/// contains the position of the first matching term in the column data).
/// Before returning, *ppCollist is set to point to the first byte after
/// the last varint in the column-list (either the 0x00 signifying the end
/// of the position-list, or the 0x01 that precedes the column number of
/// the next column in the position-list).
///
/// The number of elements in the column-list is returned.
fn fts3ColumnlistCount(mut ppCollist: *mut *mut i8) -> i32 {
    let mut pEnd: *mut i8 = unsafe { *ppCollist };
    let mut c: i8 = (0 as i32) as i8;
    let mut nEntry: i32 = 0 as i32;
    // A column-list is terminated by either a 0x01 or 0x00.
    '__slate_break_642: while (254 as i32) & (((unsafe { *pEnd }) as i32) | (c as i32))
        != (0 as i32)
    {
        let __v738: *mut i8 = pEnd;
        let __v739: *mut i8 = unsafe { __v738.offset((1 as i32) as isize) };
        pEnd = __v739;
        c = (((unsafe { *__v738 }) as i32) & (128 as i32)) as i8;
        if !(c != (0 as i8)) {
            let __v740: i32 = nEntry;
            let __v741: i32 = __v740 + (1 as i32);
            nEntry = __v741;
        }
    }
    unsafe {
        *ppCollist = pEnd;
    }
    return nEntry;
}

/// This function gathers 'y' or 'b' data for a single phrase.
///
/// # Arguments
///
/// * `pExpr` - Phrase expression node
/// * `p` - Matchinfo context
fn fts3ExprLHits(mut pExpr: *mut Fts3Expr, mut p: *mut MatchInfo) -> i32 {
    let mut pTab: *mut Fts3Table =
        (unsafe { (*unsafe { (*p).pCursor }).base.pVtab }) as *mut Fts3Table;
    let mut iStart: i32 = 0 as i32;
    let mut pPhrase: *mut Fts3Phrase = unsafe { (*pExpr).pPhrase };
    let mut pIter: *mut i8 = unsafe { (*pPhrase).doclist.pList };
    let mut iCol: i32 = 0 as i32;
    0 as i32;
    if ((unsafe { (*p).flag }) as i32) == (121 as i32) {
        iStart = (unsafe { (*pExpr).iPhrase }) * unsafe { (*p).nCol };
    } else {
        iStart =
            (unsafe { (*pExpr).iPhrase }) * (((unsafe { (*p).nCol }) + (31 as i32)) / (32 as i32));
    }
    if pIter != std::ptr::null_mut::<i8>() {
        '__slate_break_643: while (1 as i32) != (0 as i32) {
            let mut nHit: i32 = fts3ColumnlistCount(std::ptr::addr_of_mut!(pIter));
            if (unsafe { (*pPhrase).iColumn }) >= unsafe { (*pTab).nColumn }
                || (unsafe { (*pPhrase).iColumn }) == iCol
            {
                if ((unsafe { (*p).flag }) as i32) == (121 as i32) {
                    unsafe {
                        *unsafe { unsafe { (*p).aMatchinfo }.offset((iStart + iCol) as isize) } =
                            nHit as u32;
                    }
                } else {
                    if nHit != (0 as i32) {
                        let __v742: *mut u32 = unsafe {
                            unsafe { (*p).aMatchinfo }
                                .offset((iStart + iCol / (32 as i32)) as isize)
                        };
                        let __v743: u32 = unsafe { *__v742 };
                        let __v744: u32 = __v743 | (1 as u32) << (iCol & (31 as i32));
                        unsafe {
                            *__v742 = __v744;
                        }
                    }
                }
            }
            0 as i32;
            if ((unsafe { *pIter }) as i32) != (1 as i32) {
                break '__slate_break_643;
            }
            let __v745: *mut i8 = pIter;
            let __v746: *mut i8 = unsafe { __v745.offset((1 as i32) as isize) };
            pIter = __v746;
            let __v747: *mut i8 = pIter;
            let __v748: i32;
            if (((unsafe { *(pIter as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
                __v748 = unsafe {
                    sqlite3Fts3GetVarint32(pIter as *const i8, std::ptr::addr_of_mut!(iCol))
                };
            } else {
                unsafe {
                    *std::ptr::addr_of_mut!(iCol) =
                        ((unsafe { *(pIter as *mut u8) }) as u32) as i32;
                }
                __v748 = 1 as i32;
            }
            let __v749: *mut i8 = unsafe { __v747.offset(__v748 as isize) };
            pIter = __v749;
            if iCol >= unsafe { (*p).nCol } {
                return (11 as i32) | (1 as i32) << (8 as i32);
            }
        }
    }
    return 0 as i32;
}

/// Gather the results for matchinfo directives 'y' and 'b'.
fn fts3ExprLHitGather(mut pExpr: *mut Fts3Expr, mut p: *mut MatchInfo) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    if (((unsafe { (*pExpr).bEof }) as u32) as i32) == (0 as i32)
        && (unsafe { (*pExpr).iDocid }) == unsafe { (*unsafe { (*p).pCursor }).iPrevId }
    {
        if (unsafe { (*pExpr).pLeft }) != std::ptr::null_mut::<Fts3Expr>() {
            rc = fts3ExprLHitGather(unsafe { (*pExpr).pLeft }, p);
            if rc == (0 as i32) {
                rc = fts3ExprLHitGather(unsafe { (*pExpr).pRight }, p);
            }
        } else {
            rc = fts3ExprLHits(pExpr, p);
        }
    }
    return rc;
}

/// sqlite3Fts3ExprIterate() callback used to collect the "global" matchinfo
/// stats for a single query.
///
/// sqlite3Fts3ExprIterate() callback to load the 'global' elements of a
/// FTS3_MATCHINFO_HITS matchinfo array. The global stats are those elements
/// of the matchinfo array that are constant for all rows returned by the
/// current query.
///
/// Argument pCtx is actually a pointer to a struct of type MatchInfo. This
/// function populates Matchinfo.aMatchinfo[] as follows:
///
///   for(iCol=0; iCol<nCol; iCol++){
///     aMatchinfo[3*iPhrase*nCol + 3*iCol + 1] = X;
///     aMatchinfo[3*iPhrase*nCol + 3*iCol + 2] = Y;
///   }
///
/// where X is the number of matches for phrase iPhrase is column iCol of all
/// rows of the table. Y is the number of rows for which column iCol contains
/// at least one instance of phrase iPhrase.
///
/// If the phrase pExpr consists entirely of deferred tokens, then all X and
/// Y values are set to nDoc, where nDoc is the number of documents in the
/// file system. This is done because the full-text index doclist is required
/// to calculate these values properly, and the full-text index doclist is
/// not available for deferred tokens.
///
/// # Arguments
///
/// * `pExpr` - Phrase expression node
/// * `iPhrase` - Phrase number (numbered from zero)
/// * `pCtx` - Pointer to MatchInfo structure
#[unsafe(link_section = ".text.slate_distinct.fts3_snippet.fts3ExprGlobalHitsCb")]
extern "C-unwind" fn fts3ExprGlobalHitsCb(
    mut pExpr: *mut Fts3Expr,
    mut iPhrase: i32,
    mut pCtx: *mut (),
) -> i32 {
    let mut p: *mut MatchInfo = pCtx as *mut MatchInfo;
    return unsafe {
        sqlite3Fts3EvalPhraseStats(unsafe { (*p).pCursor }, pExpr, unsafe {
            unsafe { (*p).aMatchinfo }
                .offset(((3 as i32) * iPhrase * unsafe { (*p).nCol }) as isize)
        })
    };
}

/// sqlite3Fts3ExprIterate() callback used to collect the "local" part of the
/// FTS3_MATCHINFO_HITS array. The local stats are those elements of the
/// array that are different for each row returned by the query.
///
/// # Arguments
///
/// * `pExpr` - Phrase expression node
/// * `iPhrase` - Phrase number
/// * `pCtx` - Pointer to MatchInfo structure
#[unsafe(link_section = ".text.slate_distinct.fts3_snippet.fts3ExprLocalHitsCb")]
extern "C-unwind" fn fts3ExprLocalHitsCb(
    mut pExpr: *mut Fts3Expr,
    mut iPhrase: i32,
    mut pCtx: *mut (),
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut p: *mut MatchInfo = pCtx as *mut MatchInfo;
    let mut iStart: i32 = iPhrase * unsafe { (*p).nCol } * (3 as i32);
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_644: loop {
        if !(i < unsafe { (*p).nCol } && rc == (0 as i32)) {
            break;
        }
        let mut pCsr: *mut i8 = unsafe { std::mem::zeroed() };
        rc = unsafe {
            sqlite3Fts3EvalPhrasePoslist(
                unsafe { (*p).pCursor },
                pExpr,
                i,
                std::ptr::addr_of_mut!(pCsr),
            )
        };
        if pCsr != std::ptr::null_mut::<i8>() {
            unsafe {
                *unsafe { unsafe { (*p).aMatchinfo }.offset((iStart + i * (3 as i32)) as isize) } =
                    fts3ColumnlistCount(std::ptr::addr_of_mut!(pCsr)) as u32;
            }
        } else {
            unsafe {
                *unsafe { unsafe { (*p).aMatchinfo }.offset((iStart + i * (3 as i32)) as isize) } =
                    (0 as i32) as u32;
            }
        }
        let __v750: i32 = i;
        let __v751: i32 = __v750 + (1 as i32);
        i = __v751;
    }
    return rc;
}

fn fts3MatchinfoCheck(mut pTab: *mut Fts3Table, mut cArg: i8, mut pzErr: *mut *mut i8) -> i32 {
    if (cArg as i32) == (112 as i32)
        || (cArg as i32) == (99 as i32)
        || (cArg as i32) == (110 as i32) && (unsafe { (*pTab).bFts4 }) != (0 as u8)
        || (cArg as i32) == (97 as i32) && (unsafe { (*pTab).bFts4 }) != (0 as u8)
        || (cArg as i32) == (108 as i32) && (unsafe { (*pTab).bHasDocsize }) != (0 as u8)
        || (cArg as i32) == (115 as i32)
        || (cArg as i32) == (120 as i32)
        || (cArg as i32) == (121 as i32)
        || (cArg as i32) == (98 as i32)
    {
        return 0 as i32;
    }
    unsafe {
        sqlite3Fts3ErrMsg(
            pzErr,
            (b"unrecognized matchinfo request: %c\0".as_ptr() as *mut i8) as *const i8,
            cArg as i32,
        )
    };
    return 1 as i32;
}

fn fts3MatchinfoSize(mut pInfo: *mut MatchInfo, mut cArg: i8) -> i64 {
    let mut nVal: i64 = 0 as i64; // Number of integers output by cArg
    '__slate_break_646: {
        match cArg as i32 {
            110 | 112 | 99 => {
                nVal = (1 as i32) as i64;
            }
            97 | 108 | 115 => {
                nVal = (unsafe { (*pInfo).nCol }) as i64;
            }
            121 => {
                nVal = ((unsafe { (*pInfo).nCol }) as i64) * ((unsafe { (*pInfo).nPhrase }) as i64);
            }
            98 => {
                nVal = ((unsafe { (*pInfo).nPhrase }) as i64)
                    * ((((unsafe { (*pInfo).nCol }) + (31 as i32)) / (32 as i32)) as i64);
            }
            _ => {
                0 as i32;
                nVal = ((unsafe { (*pInfo).nCol }) as i64)
                    * ((unsafe { (*pInfo).nPhrase }) as i64)
                    * ((3 as i32) as i64);
            }
        }
    }
    return nVal;
}

fn fts3MatchinfoSelectDoctotal(
    mut pTab: *mut Fts3Table,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pnDoc: *mut i64,
    mut paLen: *mut *const i8,
    mut ppEnd: *mut *const i8,
) -> i32 {
    let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    let mut a: *const i8 = unsafe { std::mem::zeroed() };
    let mut pEnd: *const i8 = unsafe { std::mem::zeroed() };
    let mut nDoc: i64 = 0 as i64;
    let mut n: i32 = 0 as i32;
    if !((unsafe { *ppStmt }) != std::ptr::null_mut::<sqlite3_stmt>()) {
        let mut rc: i32 = unsafe { sqlite3Fts3SelectDoctotal(pTab, ppStmt) };
        if rc != (0 as i32) {
            return rc;
        }
    }
    pStmt = unsafe { *ppStmt };
    0 as i32;
    n = unsafe { sqlite3_column_bytes(pStmt, 0 as i32) };
    a = (unsafe { sqlite3_column_blob(pStmt, 0 as i32) }) as *const i8;
    if a == std::ptr::null::<i8>() {
        return (11 as i32) | (1 as i32) << (8 as i32);
    }
    pEnd = unsafe { a.offset(n as isize) };
    let __v752: *const i8 = a;
    let __v753: *const i8 = unsafe {
        __v752.offset(
            (unsafe { sqlite3Fts3GetVarintBounded(a, pEnd, std::ptr::addr_of_mut!(nDoc)) })
                as isize,
        )
    };
    a = __v753;
    if nDoc <= ((0 as i32) as i64) || a > pEnd {
        return (11 as i32) | (1 as i32) << (8 as i32);
    }
    unsafe {
        *pnDoc = nDoc;
    }
    if paLen != std::ptr::null_mut::<*const i8>() {
        unsafe {
            *paLen = a;
        }
    }
    if ppEnd != std::ptr::null_mut::<*const i8>() {
        unsafe {
            *ppEnd = pEnd;
        }
    }
    return 0 as i32;
}

/// An instance of the following structure is used to store state while
/// iterating through a multi-column position-list corresponding to the
/// hits for a single phrase on a single row in order to calculate the
/// values for a matchinfo() FTS3_MATCHINFO_LCS request.
#[repr(C)]
#[derive(Clone, Copy)]
struct LcsIterator {
    /// Pointer to phrase expression
    pExpr: *mut Fts3Expr,
    /// Tokens count up to end of this phrase
    iPosOffset: i32,
    /// Cursor used to iterate through aDoclist
    pRead: *mut i8,
    /// Current position
    iPos: i32,
}

// If LcsIterator.iCol is set to the following value, the iterator has
// finished iterating through all offsets for all columns.
/// # Arguments
///
/// * `pExpr` - Phrase expression node
/// * `iPhrase` - Phrase number (numbered from zero)
/// * `pCtx` - Pointer to MatchInfo structure
#[unsafe(link_section = ".text.slate_distinct.fts3_snippet.fts3MatchinfoLcsCb")]
extern "C-unwind" fn fts3MatchinfoLcsCb(
    mut pExpr: *mut Fts3Expr,
    mut iPhrase: i32,
    mut pCtx: *mut (),
) -> i32 {
    let mut aIter: *mut LcsIterator = pCtx as *mut LcsIterator;
    unsafe {
        (*unsafe { aIter.offset(iPhrase as isize) }).pExpr = pExpr;
    }
    return 0 as i32;
}

/// Advance the iterator passed as an argument to the next position. Return
/// 1 if the iterator is at EOF or if it now points to the start of the
/// position list for the next column.
fn fts3LcsIteratorAdvance(mut pIter: *mut LcsIterator) -> i32 {
    let mut pRead: *mut i8 = unsafe { std::mem::zeroed() };
    let mut iRead: i64 = 0 as i64;
    let mut rc: i32 = 0 as i32;
    if pIter == std::ptr::null_mut::<LcsIterator>() {
        return 1 as i32;
    }
    pRead = unsafe { (*pIter).pRead };
    let __v754: *mut i8 = pRead;
    let __v755: *mut i8 = unsafe {
        __v754.offset(
            (unsafe { sqlite3Fts3GetVarint(pRead as *const i8, std::ptr::addr_of_mut!(iRead)) })
                as isize,
        )
    };
    pRead = __v755;
    if iRead == ((0 as i32) as i64) || iRead == ((1 as i32) as i64) {
        pRead = std::ptr::null_mut::<i8>();
        rc = 1 as i32;
    } else {
        let __v756: *mut LcsIterator = pIter;
        let __v757: i32 = unsafe { (*__v756).iPos };
        let __v758: i32 = __v757 + ((iRead - ((2 as i32) as i64)) as i32);
        unsafe {
            (*__v756).iPos = __v758;
        }
    }
    unsafe {
        (*pIter).pRead = pRead;
    }
    return rc;
}

/// This function implements the FTS3_MATCHINFO_LCS matchinfo() flag.
///
/// If the call is successful, the longest-common-substring lengths for each
/// column are written into the first nCol elements of the pInfo->aMatchinfo[]
/// array before returning. SQLITE_OK is returned in this case.
///
/// Otherwise, if an error occurs, an SQLite error code is returned and the
/// data written to the first nCol elements of pInfo->aMatchinfo[] is
/// undefined.
fn fts3MatchinfoLcs(mut pCsr: *mut Fts3Cursor, mut pInfo: *mut MatchInfo) -> i32 {
    let mut __slate_storage_764: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_764: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_764) as *mut i32;
    let mut __slate_storage_763: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_763: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_763) as *mut i32;
    let mut __slate_storage_774: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_774: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_774) as *mut i32;
    let mut __slate_storage_773: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_773: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_773) as *mut i32;
    let mut __slate_storage_770: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_770: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_770) as *mut i32;
    let mut __slate_storage_769: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_769: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_769) as *mut i32;
    let mut __slate_storage_772: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_772: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_772) as *mut i32;
    let mut __slate_storage_771: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_771: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_771) as *mut i32;
    let mut __slate_storage_433: std::mem::MaybeUninit<*mut LcsIterator> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_433: *mut *mut LcsIterator =
        std::ptr::addr_of_mut!(__slate_storage_433) as *mut *mut LcsIterator; // LCS for the current iterator positions
    let mut __slate_storage_432: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_432: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_432) as *mut i32; // The iterator to advance by one position
    let mut __slate_storage_431: std::mem::MaybeUninit<*mut LcsIterator> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_431: *mut *mut LcsIterator =
        std::ptr::addr_of_mut!(__slate_storage_431) as *mut *mut LcsIterator;
    let mut __slate_storage_766: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_766: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_766) as *mut i32;
    let mut __slate_storage_765: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_765: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_765) as *mut i32;
    let mut __slate_storage_768: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_768: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_768) as *mut i32;
    let mut __slate_storage_767: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_767: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_767) as *mut i32;
    let mut __slate_storage_430: std::mem::MaybeUninit<*mut LcsIterator> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_430: *mut *mut LcsIterator =
        std::ptr::addr_of_mut!(__slate_storage_430) as *mut *mut LcsIterator; // Number of iterators in aIter not at EOF
    let mut __slate_storage_429: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_429: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_429) as *mut i32; // LCS value for this column
    let mut __slate_storage_428: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_428: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_428) as *mut i32;
    let mut __slate_storage_760: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_760: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_760) as *mut i32;
    let mut __slate_storage_759: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_759: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_759) as *mut i32;
    let mut __slate_storage_762: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_762: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_762) as *mut i32;
    let mut __slate_storage_761: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_761: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_761) as *mut i32;
    let mut __slate_storage_427: std::mem::MaybeUninit<*mut LcsIterator> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_427: *mut *mut LcsIterator =
        std::ptr::addr_of_mut!(__slate_storage_427) as *mut *mut LcsIterator;
    let mut __slate_storage_426: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_426: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_426) as *mut i32;
    let mut __slate_storage_425: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_425: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_425) as *mut i32;
    let mut __slate_storage_424: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_424: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_424) as *mut i32;
    let mut __slate_storage_423: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_423: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_423) as *mut i32;
    let mut __slate_storage_422: std::mem::MaybeUninit<*mut LcsIterator> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_422: *mut *mut LcsIterator =
        std::ptr::addr_of_mut!(__slate_storage_422) as *mut *mut LcsIterator;
    unsafe {
        std::ptr::write(__slate_slot_425, 0 as i32);
        std::ptr::write(__slate_slot_426, 0 as i32);
        // Allocate and populate the array of LcsIterator objects. The array
        // contains one element for each matchable phrase in the query.
        *__slate_slot_422 = (unsafe {
            sqlite3Fts3MallocZero(
                (32 as u64).wrapping_mul(((unsafe { (*pCsr).nPhrase }) as i64) as u64) as i64,
            )
        }) as *mut LcsIterator;
        if !(*__slate_slot_422 != std::ptr::null_mut::<LcsIterator>()) {
            return 7 as i32;
        } else {
            sqlite3Fts3ExprIterate(
                unsafe { (*pCsr).pExpr },
                Some(fts3MatchinfoLcsCb),
                *__slate_slot_422 as *mut (),
            );
            *__slate_slot_423 = 0 as i32;
            loop {
                if *__slate_slot_423 < unsafe { (*pInfo).nPhrase } {
                    std::ptr::write(__slate_slot_427, unsafe {
                        (*__slate_slot_422).offset(*__slate_slot_423 as isize)
                    });
                    std::ptr::write(__slate_slot_761, *__slate_slot_425);
                    std::ptr::write(
                        __slate_slot_762,
                        *__slate_slot_761
                            - unsafe {
                                (*unsafe { (*unsafe { (*(*__slate_slot_427)).pExpr }).pPhrase })
                                    .nToken
                            },
                    );
                    *__slate_slot_425 = *__slate_slot_762;
                    unsafe {
                        (*(*__slate_slot_427)).iPosOffset = *__slate_slot_425;
                    }
                    std::ptr::write(__slate_slot_759, *__slate_slot_423);
                    std::ptr::write(__slate_slot_760, *__slate_slot_759 + (1 as i32));
                    *__slate_slot_423 = *__slate_slot_760;
                } else {
                    break;
                }
            }
            *__slate_slot_424 = 0 as i32;
            '__join_0: {
                '__loop_1: loop {
                    if *__slate_slot_424 < unsafe { (*pInfo).nCol } {
                        std::ptr::write(__slate_slot_428, 0 as i32);
                        std::ptr::write(__slate_slot_429, 0 as i32);
                        *__slate_slot_423 = 0 as i32;
                        loop {
                            if *__slate_slot_423 < unsafe { (*pInfo).nPhrase } {
                                std::ptr::write(__slate_slot_430, unsafe {
                                    (*__slate_slot_422).offset(*__slate_slot_423 as isize)
                                });
                                *__slate_slot_426 = unsafe {
                                    sqlite3Fts3EvalPhrasePoslist(
                                        pCsr,
                                        unsafe { (*(*__slate_slot_430)).pExpr },
                                        *__slate_slot_424,
                                        unsafe {
                                            std::ptr::addr_of_mut!((*(*__slate_slot_430)).pRead)
                                        },
                                    )
                                };
                                if *__slate_slot_426 != (0 as i32) {
                                    break '__join_0;
                                } else {
                                    if (unsafe { (*(*__slate_slot_430)).pRead })
                                        != std::ptr::null_mut::<i8>()
                                    {
                                        unsafe {
                                            (*(*__slate_slot_430)).iPos =
                                                unsafe { (*(*__slate_slot_430)).iPosOffset };
                                        }
                                        fts3LcsIteratorAdvance(*__slate_slot_430);
                                        if (unsafe { (*(*__slate_slot_430)).pRead })
                                            == std::ptr::null_mut::<i8>()
                                        {
                                            break '__loop_1;
                                        } else {
                                            std::ptr::write(__slate_slot_767, *__slate_slot_429);
                                            std::ptr::write(
                                                __slate_slot_768,
                                                *__slate_slot_767 + (1 as i32),
                                            );
                                            *__slate_slot_429 = *__slate_slot_768;
                                        }
                                    }
                                    std::ptr::write(__slate_slot_765, *__slate_slot_423);
                                    std::ptr::write(
                                        __slate_slot_766,
                                        *__slate_slot_765 + (1 as i32),
                                    );
                                    *__slate_slot_423 = *__slate_slot_766;
                                }
                            } else {
                                break;
                            }
                        }
                        loop {
                            if *__slate_slot_429 > (0 as i32) {
                                std::ptr::write(
                                    __slate_slot_431,
                                    std::ptr::null_mut::<LcsIterator>(),
                                );
                                std::ptr::write(__slate_slot_432, 0 as i32);
                                *__slate_slot_423 = 0 as i32;
                                loop {
                                    if *__slate_slot_423 < unsafe { (*pInfo).nPhrase } {
                                        std::ptr::write(__slate_slot_433, unsafe {
                                            (*__slate_slot_422).offset(*__slate_slot_423 as isize)
                                        });
                                        if (unsafe { (*(*__slate_slot_433)).pRead })
                                            == std::ptr::null_mut::<i8>()
                                        {
                                            // This iterator is already at EOF for this column.
                                            *__slate_slot_432 = 0 as i32;
                                        } else {
                                            if *__slate_slot_431
                                                == std::ptr::null_mut::<LcsIterator>()
                                                || (unsafe { (*(*__slate_slot_433)).iPos })
                                                    < unsafe { (*(*__slate_slot_431)).iPos }
                                            {
                                                *__slate_slot_431 = *__slate_slot_433;
                                            }
                                            if *__slate_slot_432 == (0 as i32)
                                                || (unsafe { (*(*__slate_slot_433)).iPos })
                                                    == unsafe {
                                                        (*unsafe {
                                                            (*__slate_slot_433)
                                                                .offset(-(1 as i32) as isize)
                                                        })
                                                        .iPos
                                                    }
                                            {
                                                std::ptr::write(
                                                    __slate_slot_771,
                                                    *__slate_slot_432,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_772,
                                                    *__slate_slot_771 + (1 as i32),
                                                );
                                                *__slate_slot_432 = *__slate_slot_772;
                                            } else {
                                                *__slate_slot_432 = 1 as i32;
                                            }
                                            if *__slate_slot_432 > *__slate_slot_428 {
                                                *__slate_slot_428 = *__slate_slot_432;
                                            }
                                        }
                                        std::ptr::write(__slate_slot_769, *__slate_slot_423);
                                        std::ptr::write(
                                            __slate_slot_770,
                                            *__slate_slot_769 + (1 as i32),
                                        );
                                        *__slate_slot_423 = *__slate_slot_770;
                                    } else {
                                        break;
                                    }
                                }
                                if fts3LcsIteratorAdvance(*__slate_slot_431) != (0 as i32) {
                                    std::ptr::write(__slate_slot_773, *__slate_slot_429);
                                    std::ptr::write(
                                        __slate_slot_774,
                                        *__slate_slot_773 - (1 as i32),
                                    );
                                    *__slate_slot_429 = *__slate_slot_774;
                                }
                            } else {
                                break;
                            }
                        }
                        unsafe {
                            *unsafe {
                                unsafe { (*pInfo).aMatchinfo }.offset(*__slate_slot_424 as isize)
                            } = *__slate_slot_428 as u32;
                        }
                        std::ptr::write(__slate_slot_763, *__slate_slot_424);
                        std::ptr::write(__slate_slot_764, *__slate_slot_763 + (1 as i32));
                        *__slate_slot_424 = *__slate_slot_764;
                    } else {
                        break '__join_0;
                    }
                }
                *__slate_slot_426 = (11 as i32) | (1 as i32) << (8 as i32);
            }
            unsafe { sqlite3_free(*__slate_slot_422 as *mut ()) };
            return *__slate_slot_426;
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Populate the buffer pInfo->aMatchinfo[] with an array of integers to
/// be returned by the matchinfo() function. Argument zArg contains the
/// format string passed as the second argument to matchinfo (or the
/// default value "pcx" if no second argument was specified). The format
/// string has already been validated and the pInfo->aMatchinfo[] array
/// is guaranteed to be large enough for the output.
///
/// If bGlobal is true, then populate all fields of the matchinfo() output.
/// If it is false, then assume that those fields that do not change between
/// rows (i.e. FTS3_MATCHINFO_NPHRASE, NCOL, NDOC, AVGLENGTH and part of HITS)
/// have already been populated.
///
/// Return SQLITE_OK if successful, or an SQLite error code if an error
/// occurs. If a value other than SQLITE_OK is returned, the state the
/// pInfo->aMatchinfo[] buffer is left in is undefined.
///
/// # Arguments
///
/// * `pCsr` - FTS3 cursor object
/// * `bGlobal` - True to grab the global stats
/// * `pInfo` - Matchinfo context object
/// * `zArg` - Matchinfo format string
fn fts3MatchinfoValues(
    mut pCsr: *mut Fts3Cursor,
    mut bGlobal: i32,
    mut pInfo: *mut MatchInfo,
    mut zArg: *const i8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    let mut pTab: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
    let mut pSelect: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>();
    i = 0 as i32;
    '__slate_break_652: loop {
        if !(rc == (0 as i32) && (unsafe { *unsafe { zArg.offset(i as isize) } }) != (0 as i8)) {
            break;
        }
        unsafe {
            (*pInfo).flag = unsafe { *unsafe { zArg.offset(i as isize) } };
        }
        '__slate_break_653: {
            match (unsafe { *unsafe { zArg.offset(i as isize) } }) as i32 {
                112 => {
                    if bGlobal != (0 as i32) {
                        unsafe {
                            *unsafe {
                                unsafe { (*pInfo).aMatchinfo }.offset((0 as i32) as isize)
                            } = (unsafe { (*pInfo).nPhrase }) as u32;
                        }
                    }
                }
                99 => {
                    if bGlobal != (0 as i32) {
                        unsafe {
                            *unsafe {
                                unsafe { (*pInfo).aMatchinfo }.offset((0 as i32) as isize)
                            } = (unsafe { (*pInfo).nCol }) as u32;
                        }
                    }
                }
                110 => {
                    if bGlobal != (0 as i32) {
                        let mut nDoc: i64 = (0 as i32) as i64;
                        rc = fts3MatchinfoSelectDoctotal(
                            pTab,
                            std::ptr::addr_of_mut!(pSelect),
                            std::ptr::addr_of_mut!(nDoc),
                            std::ptr::null_mut::<*const i8>(),
                            std::ptr::null_mut::<*const i8>(),
                        );
                        unsafe {
                            *unsafe {
                                unsafe { (*pInfo).aMatchinfo }.offset((0 as i32) as isize)
                            } = (nDoc as i32) as u32;
                        }
                    }
                }
                97 => {
                    if bGlobal != (0 as i32) {
                        let mut nDoc: i64 = 0 as i64; // Number of rows in table
                        let mut a: *const i8 = unsafe { std::mem::zeroed() }; // Aggregate column length array
                        let mut pEnd: *const i8 = unsafe { std::mem::zeroed() }; // First byte past end of length array
                        rc = fts3MatchinfoSelectDoctotal(
                            pTab,
                            std::ptr::addr_of_mut!(pSelect),
                            std::ptr::addr_of_mut!(nDoc),
                            std::ptr::addr_of_mut!(a),
                            std::ptr::addr_of_mut!(pEnd),
                        );
                        if rc == (0 as i32) {
                            let mut iCol: i32 = 0 as i32;
                            iCol = 0 as i32;
                            '__slate_break_654: loop {
                                if !(iCol < unsafe { (*pInfo).nCol }) {
                                    break;
                                }
                                let mut iVal: u32 = 0 as u32;
                                let mut nToken: i64 = 0 as i64;
                                let __v779: *const i8 = a;
                                let __v780: *const i8 = unsafe {
                                    __v779.offset(
                                        (unsafe {
                                            sqlite3Fts3GetVarint(a, std::ptr::addr_of_mut!(nToken))
                                        }) as isize,
                                    )
                                };
                                a = __v780;
                                if a > pEnd {
                                    rc = (11 as i32) | (1 as i32) << (8 as i32);
                                    break '__slate_break_654;
                                }
                                iVal = ((((((((nToken & (((4294967295 as u32) as u64) as i64))
                                    as i32) as u32)
                                    as u64) as i64)
                                    + nDoc / ((2 as i32) as i64))
                                    / nDoc) as i32) as u32;
                                unsafe {
                                    *unsafe {
                                        unsafe { (*pInfo).aMatchinfo }.offset(iCol as isize)
                                    } = iVal;
                                }
                                let __v777: i32 = iCol;
                                let __v778: i32 = __v777 + (1 as i32);
                                iCol = __v778;
                            }
                        }
                    }
                }
                108 => {
                    let mut pSelectDocsize: *mut sqlite3_stmt =
                        std::ptr::null_mut::<sqlite3_stmt>();
                    rc = unsafe {
                        sqlite3Fts3SelectDocsize(
                            pTab,
                            unsafe { (*pCsr).iPrevId },
                            std::ptr::addr_of_mut!(pSelectDocsize),
                        )
                    };
                    if rc == (0 as i32) {
                        let mut iCol: i32 = 0 as i32;
                        let mut a: *const i8 =
                            (unsafe { sqlite3_column_blob(pSelectDocsize, 0 as i32) }) as *const i8;
                        let mut pEnd: *const i8 = unsafe {
                            a.offset(
                                (unsafe { sqlite3_column_bytes(pSelectDocsize, 0 as i32) })
                                    as isize,
                            )
                        };
                        iCol = 0 as i32;
                        '__slate_break_655: loop {
                            if !(iCol < unsafe { (*pInfo).nCol }) {
                                break;
                            }
                            let mut nToken: i64 = 0 as i64;
                            let __v783: *const i8 = a;
                            let __v784: *const i8 = unsafe {
                                __v783.offset(
                                    (unsafe {
                                        sqlite3Fts3GetVarintBounded(
                                            a,
                                            pEnd,
                                            std::ptr::addr_of_mut!(nToken),
                                        )
                                    }) as isize,
                                )
                            };
                            a = __v784;
                            if a > pEnd {
                                rc = (11 as i32) | (1 as i32) << (8 as i32);
                                break '__slate_break_655;
                            }
                            unsafe {
                                *unsafe { unsafe { (*pInfo).aMatchinfo }.offset(iCol as isize) } =
                                    (nToken as i32) as u32;
                            }
                            let __v781: i32 = iCol;
                            let __v782: i32 = __v781 + (1 as i32);
                            iCol = __v782;
                        }
                    }
                    unsafe { sqlite3_reset(pSelectDocsize) };
                }
                115 => {
                    rc = fts3ExprLoadDoclists(
                        pCsr,
                        std::ptr::null_mut::<i32>(),
                        std::ptr::null_mut::<i32>(),
                    );
                    if rc == (0 as i32) {
                        rc = fts3MatchinfoLcs(pCsr, pInfo);
                    }
                }
                98 | 121 => {
                    let mut nZero: i64 =
                        (fts3MatchinfoSize(pInfo, unsafe { *unsafe { zArg.offset(i as isize) } })
                            as u64)
                            .wrapping_mul(4 as u64) as i64;
                    unsafe {
                        memset(
                            (unsafe { (*pInfo).aMatchinfo }) as *mut (),
                            0 as i32,
                            nZero as u64,
                        )
                    };
                    rc = fts3ExprLHitGather(unsafe { (*pCsr).pExpr }, pInfo);
                }
                _ => {
                    let mut pExpr: *mut Fts3Expr = unsafe { std::mem::zeroed() };
                    0 as i32;
                    pExpr = unsafe { (*pCsr).pExpr };
                    rc = fts3ExprLoadDoclists(
                        pCsr,
                        std::ptr::null_mut::<i32>(),
                        std::ptr::null_mut::<i32>(),
                    );
                    if rc != (0 as i32) {
                    } else {
                        if bGlobal != (0 as i32) {
                            if (unsafe { (*pCsr).pDeferred })
                                != std::ptr::null_mut::<Fts3DeferredToken>()
                            {
                                rc = fts3MatchinfoSelectDoctotal(
                                    pTab,
                                    std::ptr::addr_of_mut!(pSelect),
                                    unsafe { std::ptr::addr_of_mut!((*pInfo).nDoc) },
                                    std::ptr::null_mut::<*const i8>(),
                                    std::ptr::null_mut::<*const i8>(),
                                );
                                if rc != (0 as i32) {
                                    break '__slate_break_653;
                                }
                            }
                            rc = sqlite3Fts3ExprIterate(
                                pExpr,
                                Some(fts3ExprGlobalHitsCb),
                                pInfo as *mut (),
                            );
                            unsafe {
                                sqlite3Fts3EvalTestDeferred(pCsr, std::ptr::addr_of_mut!(rc))
                            };
                            if rc != (0 as i32) {
                                break '__slate_break_653;
                            }
                        }
                        sqlite3Fts3ExprIterate(pExpr, Some(fts3ExprLocalHitsCb), pInfo as *mut ());
                    }
                }
            }
        }
        let __v785: *mut MatchInfo = pInfo;
        let __v786: *mut u32 = unsafe { (*__v785).aMatchinfo };
        let __v787: *mut u32 = unsafe {
            __v786.offset(
                fts3MatchinfoSize(pInfo, unsafe { *unsafe { zArg.offset(i as isize) } }) as isize,
            )
        };
        unsafe {
            (*__v785).aMatchinfo = __v787;
        }
        let __v775: i32 = i;
        let __v776: i32 = __v775 + (1 as i32);
        i = __v776;
    }
    unsafe { sqlite3_reset(pSelect) };
    return rc;
}

/// Populate pCsr->aMatchinfo[] with data for the current row. The
/// 'matchinfo' data is an array of 32-bit unsigned integers (C type u32).
///
/// # Arguments
///
/// * `pCtx` - Return results here
/// * `pCsr` - FTS3 Cursor object
/// * `zArg` - Second argument to matchinfo() function
fn fts3GetMatchinfo(
    mut pCtx: *mut sqlite3_context,
    mut pCsr: *mut Fts3Cursor,
    mut zArg: *const i8,
) {
    let mut sInfo: MatchInfo = unsafe { std::mem::zeroed() };
    let mut pTab: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
    let mut rc: i32 = 0 as i32;
    let mut bGlobal: i32 = 0 as i32; // Collect 'global' stats as well as local
    let mut aOut: *mut u32 = std::ptr::null_mut::<u32>();
    let mut xDestroyOut: Option<unsafe extern "C-unwind" fn(*mut ())> = None;
    unsafe {
        memset(
            std::ptr::addr_of_mut!(sInfo) as *mut (),
            0 as i32,
            40 as u64,
        )
    };
    sInfo.pCursor = pCsr;
    sInfo.nCol = unsafe { (*pTab).nColumn };
    // If there is cached matchinfo() data, but the format string for the
    // cache does not match the format string for this request, discard
    // the cached data.
    if (unsafe { (*pCsr).pMIBuffer }) != std::ptr::null_mut::<MatchinfoBuffer>()
        && (unsafe {
            strcmp(
                (unsafe { (*unsafe { (*pCsr).pMIBuffer }).zMatchinfo }) as *const i8,
                zArg,
            )
        }) != (0 as i32)
    {
        sqlite3Fts3MIBufferFree(unsafe { (*pCsr).pMIBuffer });
        unsafe {
            (*pCsr).pMIBuffer = std::ptr::null_mut::<MatchinfoBuffer>();
        }
    }
    // If Fts3Cursor.pMIBuffer is NULL, then this is the first time the
    // matchinfo function has been called for this query. In this case
    // allocate the array used to accumulate the matchinfo data and
    // initialize those elements that are constant for every row.
    if (unsafe { (*pCsr).pMIBuffer }) == std::ptr::null_mut::<MatchinfoBuffer>() {
        let mut nMatchinfo: i64 = (0 as i32) as i64; // Number of u32 elements in match-info
        let mut i: i32 = 0 as i32; // Used to iterate through zArg
        // Determine the number of phrases in the query
        unsafe {
            (*pCsr).nPhrase = fts3ExprPhraseCount(unsafe { (*pCsr).pExpr });
        }
        sInfo.nPhrase = unsafe { (*pCsr).nPhrase };
        // Determine the number of integers in the buffer returned by this call.
        i = 0 as i32;
        '__slate_break_656: loop {
            if !((unsafe { *unsafe { zArg.offset(i as isize) } }) != (0 as i8)) {
                break;
            }
            let mut zErr: *mut i8 = std::ptr::null_mut::<i8>();
            if fts3MatchinfoCheck(
                pTab,
                unsafe { *unsafe { zArg.offset(i as isize) } },
                std::ptr::addr_of_mut!(zErr),
            ) != (0 as i32)
            {
                unsafe { sqlite3_result_error(pCtx, zErr as *const i8, -(1 as i32)) };
                unsafe { sqlite3_free(zErr as *mut ()) };
                return;
            }
            let __v790: i64 = nMatchinfo;
            let __v791: i64 = __v790
                + fts3MatchinfoSize(std::ptr::addr_of_mut!(sInfo), unsafe {
                    *unsafe { zArg.offset(i as isize) }
                });
            nMatchinfo = __v791;
            let __v788: i32 = i;
            let __v789: i32 = __v788 + (1 as i32);
            i = __v789;
        }
        // Allocate space for Fts3Cursor.aMatchinfo[] and Fts3Cursor.zMatchinfo.
        unsafe {
            (*pCsr).pMIBuffer = fts3MIBufferNew(nMatchinfo, zArg);
        }
        if !((unsafe { (*pCsr).pMIBuffer }) != std::ptr::null_mut::<MatchinfoBuffer>()) {
            rc = 7 as i32;
        }
        unsafe {
            (*pCsr).isMatchinfoNeeded = 1 as i32;
        }
        bGlobal = 1 as i32;
    }
    if rc == (0 as i32) {
        xDestroyOut = fts3MIBufferAlloc(unsafe { (*pCsr).pMIBuffer }, std::ptr::addr_of_mut!(aOut));
        if xDestroyOut == None {
            rc = 7 as i32;
        }
    }
    if rc == (0 as i32) {
        sInfo.aMatchinfo = aOut;
        sInfo.nPhrase = unsafe { (*pCsr).nPhrase };
        rc = fts3MatchinfoValues(pCsr, bGlobal, std::ptr::addr_of_mut!(sInfo), zArg);
        if bGlobal != (0 as i32) {
            fts3MIBufferSetGlobal(unsafe { (*pCsr).pMIBuffer });
        }
    }
    if rc != (0 as i32) {
        unsafe { sqlite3_result_error_code(pCtx, rc) };
        if xDestroyOut != None {
            unsafe { xDestroyOut.unwrap()(aOut as *mut ()) };
        }
    } else {
        let mut n: i32 = ((((unsafe { (*unsafe { (*pCsr).pMIBuffer }).nElem }) as i64) as u64)
            .wrapping_mul(4 as u64) as u32) as i32;
        unsafe { sqlite3_result_blob(pCtx, aOut as *const (), n, xDestroyOut) };
    }
}

/// Implementation of snippet() function.
///
/// # Arguments
///
/// * `pCtx` - SQLite function call context
/// * `pCsr` - Cursor object
/// * `zStart` - Snippet start text - "<b>"
/// * `zEnd` - Snippet end text - "</b>"
/// * `zEllipsis` - Snippet ellipsis text - "<b>...</b>"
/// * `iCol` - Extract snippet from this column
/// * `nToken` - Approximate number of tokens in snippet
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3Snippet(
    mut pCtx: *mut sqlite3_context,
    mut pCsr: *mut Fts3Cursor,
    mut zStart: *const i8,
    mut zEnd: *const i8,
    mut zEllipsis: *const i8,
    mut iCol: i32,
    mut nToken: i32,
) {
    let mut __slate_storage_684: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_684: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_684) as *mut i32;
    let mut __slate_storage_683: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_683: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_683) as *mut i32;
    let mut __slate_storage_676: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_676: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_676) as *mut i32;
    let mut __slate_storage_675: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_675: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_675) as *mut i32;
    let mut __slate_storage_678: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_678: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_678) as *mut i32;
    let mut __slate_storage_677: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_677: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_677) as *mut i32;
    let mut __slate_storage_682: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_682: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_682) as *mut u64;
    let mut __slate_storage_681: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_681: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_681) as *mut u64;
    let mut __slate_storage_680: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_680: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_680) as *mut i32;
    let mut __slate_storage_679: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_679: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_679) as *mut i32;
    let mut __slate_storage_493: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_493: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_493) as *mut i32;
    let mut __slate_storage_492: std::mem::MaybeUninit<SnippetFragment> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_492: *mut SnippetFragment =
        std::ptr::addr_of_mut!(__slate_storage_492) as *mut SnippetFragment;
    let mut __slate_storage_491: std::mem::MaybeUninit<*mut SnippetFragment> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_491: *mut *mut SnippetFragment =
        std::ptr::addr_of_mut!(__slate_storage_491) as *mut *mut SnippetFragment; // Used to iterate through columns
    let mut __slate_storage_490: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_490: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_490) as *mut i32; // Best score of columns checked so far
    let mut __slate_storage_489: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_489: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_489) as *mut i32; // Bitmask of phrases seen by BestSnippet()
    let mut __slate_storage_488: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_488: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_488) as *mut u64; // Bitmask of phrases covered by snippet
    let mut __slate_storage_487: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_487: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_487) as *mut u64; // Loop counter 0..nSnippet-1
    let mut __slate_storage_486: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_486: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_486) as *mut i32; // Number of tokens in each fragment
    let mut __slate_storage_485: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_485: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_485) as *mut i32; // Maximum of 4 fragments per snippet
    let mut __slate_storage_484: std::mem::MaybeUninit<__SlateAlign16<[SnippetFragment; 4]>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_484: *mut [SnippetFragment; 4] =
        std::ptr::addr_of_mut!(__slate_storage_484) as *mut [SnippetFragment; 4];
    // The returned text includes up to four fragments of text extracted from
    // the data in the current row. The first iteration of the for(...) loop
    // below attempts to locate a single fragment of text nToken tokens in
    // size that contains at least one instance of all phrases in the query
    // expression that appear in the current row. If such a fragment of text
    // cannot be found, the second iteration of the loop attempts to locate
    // a pair of fragments, and so on.
    // Number of fragments in this snippet
    let mut __slate_storage_483: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_483: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_483) as *mut i32;
    let mut __slate_storage_482: std::mem::MaybeUninit<StrBuffer> = std::mem::MaybeUninit::uninit();
    let __slate_slot_482: *mut StrBuffer =
        std::ptr::addr_of_mut!(__slate_storage_482) as *mut StrBuffer;
    let mut __slate_storage_481: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_481: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_481) as *mut i32;
    let mut __slate_storage_480: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_480: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_480) as *mut i32;
    let mut __slate_storage_479: std::mem::MaybeUninit<*mut Fts3Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_479: *mut *mut Fts3Table =
        std::ptr::addr_of_mut!(__slate_storage_479) as *mut *mut Fts3Table;
    unsafe {
        std::ptr::write(
            __slate_slot_479,
            (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table,
        );
        std::ptr::write(__slate_slot_480, 0 as i32);
        std::ptr::write(
            __slate_slot_482,
            StrBuffer {
                z: std::ptr::null_mut::<i8>(),
                n: 0 as i32,
                nAlloc: 0 as i32,
            },
        );
        std::ptr::write(__slate_slot_483, 0 as i32);
        std::ptr::write(__slate_slot_485, -(1 as i32));
        if !((unsafe { (*pCsr).pExpr }) != std::ptr::null_mut::<Fts3Expr>()) {
            unsafe {
                sqlite3_result_text(
                    pCtx,
                    (b"\0".as_ptr() as *mut i8) as *const i8,
                    0 as i32,
                    None,
                )
            };
            return;
        } else {
            // Limit the snippet length to 64 tokens.
            if nToken < -(64 as i32) {
                nToken = -(64 as i32);
            }
            if nToken > (64 as i32) {
                nToken = 64 as i32;
            }
            *__slate_slot_483 = 1 as i32;
            '__join_3: {
                loop {
                    if (1 as i32) != (0 as i32) {
                        std::ptr::write(__slate_slot_487, ((0 as i32) as i64) as u64);
                        std::ptr::write(__slate_slot_488, ((0 as i32) as i64) as u64);
                        if nToken >= (0 as i32) {
                            *__slate_slot_485 =
                                (nToken + *__slate_slot_483 - (1 as i32)) / *__slate_slot_483;
                        } else {
                            *__slate_slot_485 = -(1 as i32) * nToken;
                        }
                        *__slate_slot_486 = 0 as i32;
                        loop {
                            if *__slate_slot_486 < *__slate_slot_483 {
                                std::ptr::write(__slate_slot_489, -(1 as i32));
                                std::ptr::write(__slate_slot_491, unsafe {
                                    ((*__slate_slot_484).as_mut_ptr() as *mut SnippetFragment)
                                        .offset(*__slate_slot_486 as isize)
                                });
                                unsafe {
                                    memset(*__slate_slot_491 as *mut (), 0 as i32, 24 as u64)
                                };
                                // Loop through all columns of the table being considered for snippets.
                                // If the iCol argument to this function was negative, this means all
                                // columns of the FTS3 table. Otherwise, only column iCol is considered.
                                *__slate_slot_490 = 0 as i32;
                                loop {
                                    if *__slate_slot_490 < unsafe { (*(*__slate_slot_479)).nColumn }
                                    {
                                        std::ptr::write(
                                            __slate_slot_492,
                                            SnippetFragment {
                                                iCol: 0 as i32,
                                                iPos: 0 as i32,
                                                covered: ((0 as i32) as i64) as u64,
                                                hlmask: ((0 as i32) as i64) as u64,
                                            },
                                        );
                                        std::ptr::write(__slate_slot_493, 0 as i32);
                                        if iCol >= (0 as i32) && *__slate_slot_490 != iCol {
                                        } else {
                                            // Find the best snippet of nFToken tokens in column iRead.
                                            *__slate_slot_480 = fts3BestSnippet(
                                                *__slate_slot_485,
                                                pCsr,
                                                *__slate_slot_490,
                                                *__slate_slot_487,
                                                std::ptr::addr_of_mut!(*__slate_slot_488),
                                                std::ptr::addr_of_mut!(*__slate_slot_492),
                                                std::ptr::addr_of_mut!(*__slate_slot_493),
                                            );
                                            if *__slate_slot_480 != (0 as i32) {
                                                break '__join_3;
                                            } else {
                                                if *__slate_slot_493 > *__slate_slot_489 {
                                                    unsafe {
                                                        *(*__slate_slot_491) = *__slate_slot_492;
                                                    }
                                                    *__slate_slot_489 = *__slate_slot_493;
                                                }
                                            }
                                        }
                                        std::ptr::write(__slate_slot_679, *__slate_slot_490);
                                        std::ptr::write(
                                            __slate_slot_680,
                                            *__slate_slot_679 + (1 as i32),
                                        );
                                        *__slate_slot_490 = *__slate_slot_680;
                                    } else {
                                        break;
                                    }
                                }
                                std::ptr::write(__slate_slot_681, *__slate_slot_487);
                                std::ptr::write(
                                    __slate_slot_682,
                                    *__slate_slot_681 | unsafe { (*(*__slate_slot_491)).covered },
                                );
                                *__slate_slot_487 = *__slate_slot_682;
                                std::ptr::write(__slate_slot_677, *__slate_slot_486);
                                std::ptr::write(__slate_slot_678, *__slate_slot_677 + (1 as i32));
                                *__slate_slot_486 = *__slate_slot_678;
                            } else {
                                break;
                            }
                        }
                        // If all query phrases seen by fts3BestSnippet() are present in at least
                        // one of the nSnippet snippet fragments, break out of the loop.
                        0 as i32;
                        if *__slate_slot_488 == *__slate_slot_487
                            || *__slate_slot_483 == ((((96 as u64) / (24 as u64)) as u32) as i32)
                        {
                            break;
                        } else {
                            std::ptr::write(__slate_slot_675, *__slate_slot_483);
                            std::ptr::write(__slate_slot_676, *__slate_slot_675 + (1 as i32));
                            *__slate_slot_483 = *__slate_slot_676;
                        }
                    } else {
                        break;
                    }
                }
                0 as i32;
                *__slate_slot_481 = 0 as i32;
                loop {
                    if *__slate_slot_481 < *__slate_slot_483 && *__slate_slot_480 == (0 as i32) {
                        *__slate_slot_480 = fts3SnippetText(
                            pCsr,
                            unsafe {
                                ((*__slate_slot_484).as_mut_ptr() as *mut SnippetFragment)
                                    .offset(*__slate_slot_481 as isize)
                            },
                            *__slate_slot_481,
                            (*__slate_slot_481 == *__slate_slot_483 - (1 as i32)) as i32,
                            *__slate_slot_485,
                            zStart,
                            zEnd,
                            zEllipsis,
                            std::ptr::addr_of_mut!(*__slate_slot_482),
                        );
                        std::ptr::write(__slate_slot_683, *__slate_slot_481);
                        std::ptr::write(__slate_slot_684, *__slate_slot_683 + (1 as i32));
                        *__slate_slot_481 = *__slate_slot_684;
                    } else {
                        break '__join_3;
                    }
                }
            }
            unsafe { sqlite3Fts3SegmentsClose(*__slate_slot_479) };
            if *__slate_slot_480 != (0 as i32) {
                unsafe { sqlite3_result_error_code(pCtx, *__slate_slot_480) };
                unsafe { sqlite3_free((*__slate_slot_482).z as *mut ()) };
            } else {
                unsafe {
                    sqlite3_result_text(
                        pCtx,
                        (*__slate_slot_482).z as *const i8,
                        -(1 as i32),
                        unsafe {
                            std::mem::transmute::<
                                *const (),
                                Option<unsafe extern "C-unwind" fn(*mut ())>,
                            >(sqlite3_free as *const ())
                        },
                    )
                };
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TermOffset {
    /// Position-list
    pList: *mut i8,
    /// Position just read from pList
    iPos: i64,
    /// Offset of this term from read positions
    iOff: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TermOffsetCtx {
    pCsr: *mut Fts3Cursor,
    /// Column of table to populate aTerm for
    iCol: i32,
    iTerm: i32,
    iDocid: i64,
    aTerm: *mut TermOffset,
}

/// This function is an sqlite3Fts3ExprIterate() callback used by sqlite3Fts3Offsets().
#[unsafe(link_section = ".text.slate_distinct.fts3_snippet.fts3ExprTermOffsetInit")]
extern "C-unwind" fn fts3ExprTermOffsetInit(
    mut pExpr: *mut Fts3Expr,
    mut iPhrase: i32,
    mut ctx: *mut (),
) -> i32 {
    let mut p: *mut TermOffsetCtx = ctx as *mut TermOffsetCtx;
    let mut nTerm: i32 = 0 as i32; // Number of tokens in phrase
    let mut iTerm: i32 = 0 as i32; // For looping through nTerm phrase terms
    let mut pList: *mut i8 = unsafe { std::mem::zeroed() }; // Pointer to position list for phrase
    let mut iPos: i64 = (0 as i32) as i64; // First position in position-list
    let mut rc: i32 = 0 as i32;
    iPhrase;
    rc = unsafe {
        sqlite3Fts3EvalPhrasePoslist(
            unsafe { (*p).pCsr },
            pExpr,
            unsafe { (*p).iCol },
            std::ptr::addr_of_mut!(pList),
        )
    };
    nTerm = unsafe { (*unsafe { (*pExpr).pPhrase }).nToken };
    if pList != std::ptr::null_mut::<i8>() {
        fts3GetDeltaPosition(std::ptr::addr_of_mut!(pList), std::ptr::addr_of_mut!(iPos));
        0 as i32;
    }
    iTerm = 0 as i32;
    '__slate_break_662: loop {
        if !(iTerm < nTerm) {
            break;
        }
        let mut pT: *mut TermOffset = unsafe { std::mem::zeroed() };
        let __v794: *mut TermOffsetCtx = p;
        let __v795: i32 = unsafe { (*__v794).iTerm };
        let __v796: i32 = __v795 + (1 as i32);
        unsafe {
            (*__v794).iTerm = __v796;
        }
        pT = unsafe { unsafe { (*p).aTerm }.offset(__v795 as isize) };
        unsafe {
            (*pT).iOff = (nTerm - iTerm - (1 as i32)) as i64;
        }
        unsafe {
            (*pT).pList = pList;
        }
        unsafe {
            (*pT).iPos = iPos;
        }
        let __v792: i32 = iTerm;
        let __v793: i32 = __v792 + (1 as i32);
        iTerm = __v793;
    }
    return rc;
}

/// If expression pExpr is a phrase expression that uses an MSR query,
/// restart it as a regular, non-incremental query. Return SQLITE_OK
/// if successful, or an SQLite error code otherwise.
#[unsafe(link_section = ".text.slate_distinct.fts3_snippet.fts3ExprRestartIfCb")]
extern "C-unwind" fn fts3ExprRestartIfCb(
    mut pExpr: *mut Fts3Expr,
    mut iPhrase: i32,
    mut ctx: *mut (),
) -> i32 {
    let mut p: *mut TermOffsetCtx = ctx as *mut TermOffsetCtx;
    let mut rc: i32 = 0 as i32;
    iPhrase;
    if (unsafe { (*pExpr).pPhrase }) != std::ptr::null_mut::<Fts3Phrase>()
        && (unsafe { (*unsafe { (*pExpr).pPhrase }).bIncr }) != (0 as i32)
    {
        rc = unsafe { sqlite3Fts3MsrCancel(unsafe { (*p).pCsr }, pExpr) };
        unsafe {
            (*unsafe { (*pExpr).pPhrase }).bIncr = 0 as i32;
        }
    }
    return rc;
}

/// Implementation of offsets() function.
///
/// # Arguments
///
/// * `pCtx` - SQLite function call context
/// * `pCsr` - Cursor object
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3Offsets(mut pCtx: *mut sqlite3_context, mut pCsr: *mut Fts3Cursor) {
    let mut __slate_storage_672: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_672: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_672) as *mut i32;
    let mut __slate_storage_671: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_671: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_671) as *mut i32;
    let mut __slate_storage_537: std::mem::MaybeUninit<__SlateAlign16<[i8; 64]>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_537: *mut [i8; 64] =
        std::ptr::addr_of_mut!(__slate_storage_537) as *mut [i8; 64];
    let mut __slate_storage_674: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_674: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_674) as *mut i32;
    let mut __slate_storage_673: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_673: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_673) as *mut i32;
    let mut __slate_storage_536: std::mem::MaybeUninit<*mut TermOffset> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_536: *mut *mut TermOffset =
        std::ptr::addr_of_mut!(__slate_storage_536) as *mut *mut TermOffset; // TermOffset associated with next token
    let mut __slate_storage_535: std::mem::MaybeUninit<*mut TermOffset> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_535: *mut *mut TermOffset =
        std::ptr::addr_of_mut!(__slate_storage_535) as *mut *mut TermOffset; // Position of next token
    let mut __slate_storage_534: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_534: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_534) as *mut i32; // Used to loop through terms
    let mut __slate_storage_533: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_533: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_533) as *mut i32;
    let mut __slate_storage_532: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_532: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_532) as *mut i32;
    let mut __slate_storage_531: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_531: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_531) as *mut *const i8;
    let mut __slate_storage_530: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_530: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_530) as *mut i32;
    let mut __slate_storage_529: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_529: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_529) as *mut i32;
    let mut __slate_storage_528: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_528: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_528) as *mut i32; // Dummy argument used with xNext()
    let mut __slate_storage_527: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_527: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_527) as *mut i32; // Dummy argument used with xNext()
    let mut __slate_storage_526: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_526: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_526) as *mut *const i8; // Tokenizer cursor
    let mut __slate_storage_525: std::mem::MaybeUninit<*mut sqlite3_tokenizer_cursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_525: *mut *mut sqlite3_tokenizer_cursor =
        std::ptr::addr_of_mut!(__slate_storage_525) as *mut *mut sqlite3_tokenizer_cursor; // Context for fts3ExprTermOffsetInit()
    let mut __slate_storage_524: std::mem::MaybeUninit<TermOffsetCtx> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_524: *mut TermOffsetCtx =
        std::ptr::addr_of_mut!(__slate_storage_524) as *mut TermOffsetCtx; // Result string
    let mut __slate_storage_523: std::mem::MaybeUninit<StrBuffer> = std::mem::MaybeUninit::uninit();
    let __slate_slot_523: *mut StrBuffer =
        std::ptr::addr_of_mut!(__slate_storage_523) as *mut StrBuffer; // Column currently being processed
    let mut __slate_storage_522: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_522: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_522) as *mut i32; // Number of tokens in query
    let mut __slate_storage_521: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_521: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_521) as *mut i32; // Return Code
    let mut __slate_storage_520: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_520: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_520) as *mut i32;
    let mut __slate_storage_519: std::mem::MaybeUninit<*const sqlite3_tokenizer_module> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_519: *mut *const sqlite3_tokenizer_module =
        std::ptr::addr_of_mut!(__slate_storage_519) as *mut *const sqlite3_tokenizer_module;
    let mut __slate_storage_518: std::mem::MaybeUninit<*mut Fts3Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_518: *mut *mut Fts3Table =
        std::ptr::addr_of_mut!(__slate_storage_518) as *mut *mut Fts3Table;
    unsafe {
        std::ptr::write(
            __slate_slot_518,
            (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table,
        );
        std::ptr::write(__slate_slot_519, unsafe {
            (*unsafe { (*(*__slate_slot_518)).pTokenizer }).pModule
        });
        std::ptr::write(
            __slate_slot_523,
            StrBuffer {
                z: std::ptr::null_mut::<i8>(),
                n: 0 as i32,
                nAlloc: 0 as i32,
            },
        );
        if !((unsafe { (*pCsr).pExpr }) != std::ptr::null_mut::<Fts3Expr>()) {
            unsafe {
                sqlite3_result_text(
                    pCtx,
                    (b"\0".as_ptr() as *mut i8) as *const i8,
                    0 as i32,
                    None,
                )
            };
            return;
        } else {
            '__join_3: {
                unsafe {
                    memset(
                        std::ptr::addr_of_mut!(*__slate_slot_524) as *mut (),
                        0 as i32,
                        32 as u64,
                    )
                };
                0 as i32;
                // Count the number of terms in the query
                *__slate_slot_520 = fts3ExprLoadDoclists(
                    pCsr,
                    std::ptr::null_mut::<i32>(),
                    std::ptr::addr_of_mut!(*__slate_slot_521),
                );
                if *__slate_slot_520 != (0 as i32) {
                } else {
                    // Allocate the array of TermOffset iterators.
                    (*__slate_slot_524).aTerm = (unsafe {
                        sqlite3Fts3MallocZero(
                            (24 as u64).wrapping_mul((*__slate_slot_521 as i64) as u64) as i64,
                        )
                    }) as *mut TermOffset;
                    if std::ptr::null_mut::<TermOffset>() == (*__slate_slot_524).aTerm {
                        *__slate_slot_520 = 7 as i32;
                    } else {
                        (*__slate_slot_524).iDocid = unsafe { (*pCsr).iPrevId };
                        (*__slate_slot_524).pCsr = pCsr;
                        // If a query restart will be required, do it here, rather than later of
                        // after pointers to poslist buffers that may be invalidated by a restart
                        // have been saved.
                        *__slate_slot_520 = sqlite3Fts3ExprIterate(
                            unsafe { (*pCsr).pExpr },
                            Some(fts3ExprRestartIfCb),
                            std::ptr::addr_of_mut!(*__slate_slot_524) as *mut (),
                        );
                        if *__slate_slot_520 != (0 as i32) {
                        } else {
                            // Loop through the table columns, appending offset information to
                            // string-buffer res for each column.
                            *__slate_slot_522 = 0 as i32;
                            '__loop_4: loop {
                                if *__slate_slot_522 < unsafe { (*(*__slate_slot_518)).nColumn } {
                                    std::ptr::write(__slate_slot_527, 0 as i32);
                                    std::ptr::write(__slate_slot_528, 0 as i32);
                                    std::ptr::write(__slate_slot_529, 0 as i32);
                                    std::ptr::write(__slate_slot_530, 0 as i32);
                                    // Initialize the contents of sCtx.aTerm[] for column iCol. This
                                    // operation may fail if the database contains corrupt records.
                                    (*__slate_slot_524).iCol = *__slate_slot_522;
                                    (*__slate_slot_524).iTerm = 0 as i32;
                                    *__slate_slot_520 = sqlite3Fts3ExprIterate(
                                        unsafe { (*pCsr).pExpr },
                                        Some(fts3ExprTermOffsetInit),
                                        std::ptr::addr_of_mut!(*__slate_slot_524) as *mut (),
                                    );
                                    if *__slate_slot_520 != (0 as i32) {
                                        break '__join_3;
                                    } else {
                                        // Retreive the text stored in column iCol. If an SQL NULL is stored
                                        // in column iCol, jump immediately to the next iteration of the loop.
                                        // If an OOM occurs while retrieving the data (this can happen if SQLite
                                        // needs to transform the data from utf-16 to utf-8), return SQLITE_NOMEM
                                        // to the caller.
                                        *__slate_slot_531 = (unsafe {
                                            sqlite3_column_text(
                                                unsafe { (*pCsr).pStmt },
                                                *__slate_slot_522 + (1 as i32),
                                            )
                                        })
                                            as *const i8;
                                        *__slate_slot_532 = unsafe {
                                            sqlite3_column_bytes(
                                                unsafe { (*pCsr).pStmt },
                                                *__slate_slot_522 + (1 as i32),
                                            )
                                        };
                                        if *__slate_slot_531 == std::ptr::null::<i8>() {
                                            if (unsafe {
                                                sqlite3_column_type(
                                                    unsafe { (*pCsr).pStmt },
                                                    *__slate_slot_522 + (1 as i32),
                                                )
                                            }) == (5 as i32)
                                            {
                                            } else {
                                                break '__loop_4;
                                            }
                                        } else {
                                            // Initialize a tokenizer iterator to iterate through column iCol.
                                            *__slate_slot_520 = unsafe {
                                                sqlite3Fts3OpenTokenizer(
                                                    unsafe { (*(*__slate_slot_518)).pTokenizer },
                                                    unsafe { (*pCsr).iLangid },
                                                    *__slate_slot_531,
                                                    *__slate_slot_532,
                                                    std::ptr::addr_of_mut!(*__slate_slot_525),
                                                )
                                            };
                                            if *__slate_slot_520 != (0 as i32) {
                                                break '__join_3;
                                            } else {
                                                *__slate_slot_520 = unsafe {
                                                    unsafe { (*(*__slate_slot_519)).xNext }.unwrap()(
                                                        *__slate_slot_525,
                                                        std::ptr::addr_of_mut!(*__slate_slot_526),
                                                        std::ptr::addr_of_mut!(*__slate_slot_527),
                                                        std::ptr::addr_of_mut!(*__slate_slot_528),
                                                        std::ptr::addr_of_mut!(*__slate_slot_529),
                                                        std::ptr::addr_of_mut!(*__slate_slot_530),
                                                    )
                                                };
                                                loop {
                                                    if *__slate_slot_520 == (0 as i32) {
                                                        std::ptr::write(
                                                            __slate_slot_534,
                                                            2147483647 as i32,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_535,
                                                            std::ptr::null_mut::<TermOffset>(),
                                                        );
                                                        *__slate_slot_533 = 0 as i32;
                                                        loop {
                                                            if *__slate_slot_533 < *__slate_slot_521
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_536,
                                                                    unsafe {
                                                                        (*__slate_slot_524)
                                                                            .aTerm
                                                                            .offset(
                                                                                *__slate_slot_533
                                                                                    as isize,
                                                                            )
                                                                    },
                                                                );
                                                                if (unsafe {
                                                                    (*(*__slate_slot_536)).pList
                                                                }) != std::ptr::null_mut::<i8>()
                                                                    && (unsafe {
                                                                        (*(*__slate_slot_536)).iPos
                                                                    }) - unsafe {
                                                                        (*(*__slate_slot_536)).iOff
                                                                    } < (*__slate_slot_534
                                                                        as i64)
                                                                {
                                                                    *__slate_slot_534 = ((unsafe {
                                                                        (*(*__slate_slot_536)).iPos
                                                                    }) - unsafe {
                                                                        (*(*__slate_slot_536)).iOff
                                                                    })
                                                                        as i32;
                                                                    *__slate_slot_535 =
                                                                        *__slate_slot_536;
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_673,
                                                                    *__slate_slot_533,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_674,
                                                                    *__slate_slot_673 + (1 as i32),
                                                                );
                                                                *__slate_slot_533 =
                                                                    *__slate_slot_674;
                                                            } else {
                                                                break;
                                                            }
                                                        }
                                                        if !(*__slate_slot_535
                                                            != std::ptr::null_mut::<TermOffset>())
                                                        {
                                                            // All offsets for this column have been gathered.
                                                            *__slate_slot_520 = 101 as i32;
                                                        } else {
                                                            0 as i32;
                                                            if (0 as i32)
                                                                == (254 as i32)
                                                                    & ((unsafe {
                                                                        *unsafe {
                                                                            (*(*__slate_slot_535))
                                                                                .pList
                                                                        }
                                                                    })
                                                                        as i32)
                                                            {
                                                                unsafe {
                                                                    (*(*__slate_slot_535)).pList =
                                                                        std::ptr::null_mut::<i8>();
                                                                }
                                                            } else {
                                                                fts3GetDeltaPosition(
                                                                    unsafe {
                                                                        std::ptr::addr_of_mut!(
                                                                            (*(*__slate_slot_535))
                                                                                .pList
                                                                        )
                                                                    },
                                                                    unsafe {
                                                                        std::ptr::addr_of_mut!(
                                                                            (*(*__slate_slot_535))
                                                                                .iPos
                                                                        )
                                                                    },
                                                                );
                                                            }
                                                            loop {
                                                                if *__slate_slot_520 == (0 as i32)
                                                                    && *__slate_slot_530
                                                                        < *__slate_slot_534
                                                                {
                                                                    *__slate_slot_520 = unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_519))
                                                                                .xNext
                                                                        }
                                                                        .unwrap()(
                                                                            *__slate_slot_525,
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_526
                                                                            ),
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_527
                                                                            ),
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_528
                                                                            ),
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_529
                                                                            ),
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_530
                                                                            ),
                                                                        )
                                                                    };
                                                                } else {
                                                                    break;
                                                                }
                                                            }
                                                            if *__slate_slot_520 == (0 as i32) {
                                                                unsafe {
                                                                    sqlite3_snprintf(
                                                                        ((64 as u64) as u32) as i32,
                                                                        (*__slate_slot_537)
                                                                            .as_mut_ptr()
                                                                            as *mut i8,
                                                                        (b"%d %d %d %d \0".as_ptr()
                                                                            as *mut i8)
                                                                            as *const i8,
                                                                        *__slate_slot_522,
                                                                        (unsafe {
                                                                            (*__slate_slot_535).offset_from((*__slate_slot_524).aTerm as *mut TermOffset)
                                                                        })
                                                                            as i64,
                                                                        *__slate_slot_528,
                                                                        *__slate_slot_529
                                                                            - *__slate_slot_528,
                                                                    )
                                                                };
                                                                *__slate_slot_520 =
                                                                    fts3StringAppend(
                                                                        std::ptr::addr_of_mut!(
                                                                            *__slate_slot_523
                                                                        ),
                                                                        ((*__slate_slot_537)
                                                                            .as_mut_ptr()
                                                                            as *mut i8)
                                                                            as *const i8,
                                                                        -(1 as i32),
                                                                    );
                                                            } else {
                                                                if *__slate_slot_520 == (101 as i32)
                                                                    && (unsafe {
                                                                        (*(*__slate_slot_518))
                                                                            .zContentTbl
                                                                    }) == std::ptr::null_mut::<i8>(
                                                                    )
                                                                {
                                                                    *__slate_slot_520 = (11 as i32)
                                                                        | (1 as i32) << (8 as i32);
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                if *__slate_slot_520 == (101 as i32) {
                                                    *__slate_slot_520 = 0 as i32;
                                                }
                                                unsafe {
                                                    unsafe { (*(*__slate_slot_519)).xClose }
                                                        .unwrap()(
                                                        *__slate_slot_525
                                                    )
                                                };
                                                if *__slate_slot_520 != (0 as i32) {
                                                    break '__join_3;
                                                }
                                            }
                                        }
                                        std::ptr::write(__slate_slot_671, *__slate_slot_522);
                                        std::ptr::write(
                                            __slate_slot_672,
                                            *__slate_slot_671 + (1 as i32),
                                        );
                                        *__slate_slot_522 = *__slate_slot_672;
                                    }
                                } else {
                                    break '__join_3;
                                }
                            }
                            *__slate_slot_520 = 7 as i32;
                        }
                    }
                }
            }
            unsafe { sqlite3_free((*__slate_slot_524).aTerm as *mut ()) };
            0 as i32;
            unsafe { sqlite3Fts3SegmentsClose(*__slate_slot_518) };
            if *__slate_slot_520 != (0 as i32) {
                unsafe { sqlite3_result_error_code(pCtx, *__slate_slot_520) };
                unsafe { sqlite3_free((*__slate_slot_523).z as *mut ()) };
            } else {
                unsafe {
                    sqlite3_result_text(
                        pCtx,
                        (*__slate_slot_523).z as *const i8,
                        (*__slate_slot_523).n - (1 as i32),
                        unsafe {
                            std::mem::transmute::<
                                *const (),
                                Option<unsafe extern "C-unwind" fn(*mut ())>,
                            >(sqlite3_free as *const ())
                        },
                    )
                };
            }
            return;
        }
    }
}

/// Implementation of matchinfo() function.
///
/// # Arguments
///
/// * `pContext` - Function call context
/// * `pCsr` - FTS3 table cursor
/// * `zArg` - Second arg to matchinfo() function
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3Matchinfo(
    mut pContext: *mut sqlite3_context,
    mut pCsr: *mut Fts3Cursor,
    mut zArg: *const i8,
) {
    let mut pTab: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
    let mut zFormat: *const i8 = unsafe { std::mem::zeroed() };
    if zArg != std::ptr::null::<i8>() {
        zFormat = zArg;
    } else {
        zFormat = (b"pcx\0".as_ptr() as *mut i8) as *const i8;
    }
    if !((unsafe { (*pCsr).pExpr }) != std::ptr::null_mut::<Fts3Expr>()) {
        unsafe {
            sqlite3_result_blob(
                pContext,
                (b"\0".as_ptr() as *mut i8) as *const (),
                0 as i32,
                None,
            )
        };
        return;
    } else {
        // Retrieve matchinfo() data.
        fts3GetMatchinfo(pContext, pCsr, zFormat);
        unsafe { sqlite3Fts3SegmentsClose(pTab) };
    }
}
