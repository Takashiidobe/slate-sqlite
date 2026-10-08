//! 2008 August 05
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This file implements that page cache.
unsafe extern "C" {
    static mut sqlite3Config: Sqlite3Config;
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3PCacheSetDefault();
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_mutex {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_mem_methods {
    xMalloc: Option<unsafe extern "C-unwind" fn(i32) -> *mut ()>,
    xFree: Option<unsafe extern "C-unwind" fn(*mut ())>,
    xRealloc: Option<unsafe extern "C-unwind" fn(*mut (), i32) -> *mut ()>,
    xSize: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    xRoundup: Option<unsafe extern "C-unwind" fn(i32) -> i32>,
    xInit: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    xShutdown: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pAppData: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_mutex_methods {
    xMutexInit: Option<unsafe extern "C-unwind" fn() -> i32>,
    xMutexEnd: Option<unsafe extern "C-unwind" fn() -> i32>,
    xMutexAlloc: Option<unsafe extern "C-unwind" fn(i32) -> *mut sqlite3_mutex>,
    xMutexFree: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex)>,
    xMutexEnter: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex)>,
    xMutexTry: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex) -> i32>,
    xMutexLeave: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex)>,
    xMutexHeld: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex) -> i32>,
    xMutexNotheld: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_pcache {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_pcache_page {
    pBuf: *mut (),
    pExtra: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_pcache_methods2 {
    iVersion: i32,
    pArg: *mut (),
    xInit: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    xShutdown: Option<unsafe extern "C-unwind" fn(*mut ())>,
    xCreate: Option<unsafe extern "C-unwind" fn(i32, i32, i32) -> *mut sqlite3_pcache>,
    xCachesize: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache, i32)>,
    xPagecount: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache) -> i32>,
    xFetch: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_pcache, u32, i32) -> *mut sqlite3_pcache_page,
    >,
    xUnpin: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache, *mut sqlite3_pcache_page, i32)>,
    xRekey: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_pcache, *mut sqlite3_pcache_page, u32, u32),
    >,
    xTruncate: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache, u32)>,
    xDestroy: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache)>,
    xShrink: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache)>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Pager {}

#[repr(C)]
#[derive(Clone, Copy)]
struct PgHdr {
    pPage: *mut sqlite3_pcache_page,
    pData: *mut (),
    pExtra: *mut (),
    pCache: *mut PCache,
    pDirty: *mut PgHdr,
    pPager: *mut Pager,
    pgno: u32,
    flags: u16,
    nRef: i64,
    pDirtyNext: *mut PgHdr,
    pDirtyPrev: *mut PgHdr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Sqlite3Config {
    bMemstat: i32,
    bCoreMutex: u8,
    bFullMutex: u8,
    bOpenUri: u8,
    bUseCis: u8,
    bSmallMalloc: u8,
    bExtraSchemaChecks: u8,
    mxStrlen: i32,
    neverCorrupt: i32,
    szLookaside: i32,
    nLookaside: i32,
    nStmtSpill: i32,
    m: sqlite3_mem_methods,
    mutex: sqlite3_mutex_methods,
    pcache2: sqlite3_pcache_methods2,
    pHeap: *mut (),
    nHeap: i32,
    mnReq: i32,
    mxReq: i32,
    szMmap: i64,
    mxMmap: i64,
    pPage: *mut (),
    szPage: i32,
    nPage: i32,
    mxParserStack: i32,
    sharedCacheEnabled: i32,
    szPma: u32,
    isInit: i32,
    inProgress: i32,
    isMutexInit: i32,
    isMallocInit: i32,
    isPCacheInit: i32,
    nRefInitMutex: i32,
    pInitMutex: *mut sqlite3_mutex,
    xLog: Option<unsafe extern "C-unwind" fn(*mut (), i32, *const i8)>,
    pLogArg: *mut (),
    mxMemdbSize: i64,
    xTestCallback: Option<unsafe extern "C-unwind" fn(i32) -> i32>,
    bLocaltimeFault: i32,
    xAltLocaltime: Option<unsafe extern "C-unwind" fn(*const (), *mut ()) -> i32>,
    iOnceResetThreshold: i32,
    szSorterRef: u32,
    iPrngSeed: u32,
}

/// A complete page cache is an instance of this structure.  Every
/// entry in the cache holds a single page of the database file.  The
/// btree layer only operates on the cached copy of the database pages.
///
/// A page cache entry is "clean" if it exactly matches what is currently
/// on disk.  A page is "dirty" if it has been modified and needs to be
/// persisted to disk.
///
/// pDirty, pDirtyTail, pSynced:
///   All dirty pages are linked into the doubly linked list using
///   PgHdr.pDirtyNext and pDirtyPrev. The list is maintained in LRU order
///   such that p was added to the list more recently than p->pDirtyNext.
///   PCache.pDirty points to the first (newest) element in the list and
///   pDirtyTail to the last (oldest).
///
///   The PCache.pSynced variable is used to optimize searching for a dirty
///   page to eject from the cache mid-transaction. It is better to eject
///   a page that does not require a journal sync than one that does.
///   Therefore, pSynced is maintained so that it *almost* always points
///   to either the oldest page in the pDirty/pDirtyTail list that has a
///   clear PGHDR_NEED_SYNC flag or to a page that is older than this one
///   (so that the right page to eject can be found by following pDirtyPrev
///   pointers).
#[repr(C)]
#[derive(Clone, Copy)]
struct PCache {
    pDirty: *mut PgHdr,
    /// List of dirty pages in LRU order
    pDirtyTail: *mut PgHdr,
    /// Last synced page in dirty page list
    pSynced: *mut PgHdr,
    /// Sum of ref counts over all pages
    nRefSum: i64,
    /// Configured cache size
    szCache: i32,
    /// Size before spilling occurs
    szSpill: i32,
    /// Size of every page in this cache
    szPage: i32,
    /// Size of extra space for each page
    szExtra: i32,
    /// True if pages are on backing store
    bPurgeable: u8,
    /// eCreate value for for xFetch()
    eCreate: u8,
    /// Call to try make a page clean
    xStress: Option<unsafe extern "C-unwind" fn(*mut (), *mut PgHdr) -> i32>,
    /// Argument to xStress
    pStress: *mut (),
    /// Pluggable cache module
    pCache: *mut sqlite3_pcache,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

// Linked List Management
// Allowed values for second argument to pcacheManageDirtyList()
// Remove pPage from dirty list
// Add pPage to the dirty list
// Move pPage to the front of the list
/// Test and Debug Logic
///
/// Debug tracing macros.  Enable by by changing the "0" to "1" and
/// recompiling.
///
/// When sqlite3PcacheTrace is 1, single line trace messages are issued.
/// When sqlite3PcacheTrace is 2, a dump of the pcache showing all cache entries
/// is displayed for many operations, resulting in a lot of output.
/// Return 1 if pPg is on the dirty list for pCache.  Return 0 if not.
/// This routine runs inside of assert() statements only.
/// Check invariants on a PgHdr entry.  Return true if everything is OK.
/// Return false if any invariant is violated.
///
/// This routine is for use inside of assert() statements only.  For
/// example:
///
///          assert( sqlite3PcachePageSanity(pPg) );
/// Manage pPage's participation on the dirty list.  Bits of the addRemove
/// argument determines what operation to do.  The 0x01 bit means first
/// remove pPage from the dirty list.  The 0x02 means add pPage back to
/// the dirty list.  Doing both moves pPage to the front of the dirty list.
fn pcacheManageDirtyList(mut pPage: *mut PgHdr, mut addRemove: u8) {
    let mut p: *mut PCache = unsafe { (*pPage).pCache };
    {}
    if ((addRemove as u32) as i32) & (1 as i32) != (0 as i32) {
        0 as i32;
        0 as i32;
        // Update the PCache1.pSynced variable if necessary.
        if (unsafe { (*p).pSynced }) == pPage {
            unsafe {
                (*p).pSynced = unsafe { (*pPage).pDirtyPrev };
            }
        }
        if (unsafe { (*pPage).pDirtyNext }) != std::ptr::null_mut::<PgHdr>() {
            unsafe {
                (*unsafe { (*pPage).pDirtyNext }).pDirtyPrev = unsafe { (*pPage).pDirtyPrev };
            }
        } else {
            0 as i32;
            unsafe {
                (*p).pDirtyTail = unsafe { (*pPage).pDirtyPrev };
            }
        }
        if (unsafe { (*pPage).pDirtyPrev }) != std::ptr::null_mut::<PgHdr>() {
            unsafe {
                (*unsafe { (*pPage).pDirtyPrev }).pDirtyNext = unsafe { (*pPage).pDirtyNext };
            }
        } else {
            // If there are now no dirty pages in the cache, set eCreate to 2.
            // This is an optimization that allows sqlite3PcacheFetch() to skip
            // searching for a dirty page to eject from the cache when it might
            // otherwise have to.
            0 as i32;
            unsafe {
                (*p).pDirty = unsafe { (*pPage).pDirtyNext };
            }
            0 as i32;
            if (unsafe { (*p).pDirty }) == std::ptr::null_mut::<PgHdr>() {
                0 as i32;
                unsafe {
                    (*p).eCreate = ((2 as i32) as i8) as u8;
                }
            }
        }
    }
    if ((addRemove as u32) as i32) & (2 as i32) != (0 as i32) {
        unsafe {
            (*pPage).pDirtyPrev = std::ptr::null_mut::<PgHdr>();
        }
        unsafe {
            (*pPage).pDirtyNext = unsafe { (*p).pDirty };
        }
        if (unsafe { (*pPage).pDirtyNext }) != std::ptr::null_mut::<PgHdr>() {
            0 as i32;
            unsafe {
                (*unsafe { (*pPage).pDirtyNext }).pDirtyPrev = pPage;
            }
        } else {
            unsafe {
                (*p).pDirtyTail = pPage;
            }
            if (unsafe { (*p).bPurgeable }) != (0 as u8) {
                0 as i32;
                unsafe {
                    (*p).eCreate = ((1 as i32) as i8) as u8;
                }
            }
        }
        unsafe {
            (*p).pDirty = pPage;
        }
        // If pSynced is NULL and this page has a clear NEED_SYNC flag, set
        // pSynced to point to it. Checking the NEED_SYNC flag is an
        // optimization, as if pSynced points to a page with the NEED_SYNC
        // flag set sqlite3PcacheFetchStress() searches through all newer
        // entries of the dirty-list for a page with NEED_SYNC clear anyway.
        if !((unsafe { (*p).pSynced }) != std::ptr::null_mut::<PgHdr>())
            && (0 as i32) == (((unsafe { (*pPage).flags }) as u32) as i32) & (8 as i32)
        {
            unsafe {
                (*p).pSynced = pPage;
            }
        }
    }
    {}
}

/// Wrapper around the pluggable caches xUnpin method. If the cache is
/// being used for an in-memory database, this function is a no-op.
fn pcacheUnpin(mut p: *mut PgHdr) {
    if (unsafe { (*unsafe { (*p).pCache }).bPurgeable }) != (0 as u8) {
        {}
        unsafe {
            unsafe { sqlite3Config.pcache2.xUnpin }.unwrap()(
                unsafe { (*unsafe { (*p).pCache }).pCache },
                unsafe { (*p).pPage },
                0 as i32,
            )
        };
        {}
    }
}

/// Compute the number of pages of cache requested.   p->szCache is the
/// cache size requested by the "PRAGMA cache_size" statement.
fn numberOfCachePages(mut p: *mut PCache) -> i32 {
    if (unsafe { (*p).szCache }) >= (0 as i32) {
        // IMPLEMENTATION-OF: R-42059-47211 If the argument N is positive then the
        // suggested cache size is set to N.
        return unsafe { (*p).szCache };
    } else {
        let mut n: i64 = 0 as i64;
        // IMPLEMENTATION-OF: R-59858-46238 If the argument N is negative, then the
        // number of cache pages is adjusted to be a number of pages that would
        // use approximately abs(N*1024) bytes of memory based on the current
        // page size.
        n = (-(1024 as i32) as i64) * ((unsafe { (*p).szCache }) as i64)
            / (((unsafe { (*p).szPage }) + unsafe { (*p).szExtra }) as i64);
        if n > ((1000000000 as i32) as i64) {
            n = (1000000000 as i32) as i64;
        }
        return n as i32;
    }
    return unsafe { std::mem::zeroed() };
}

/// General Interfaces ******
///
/// Initialize and shutdown the page cache subsystem. Neither of these
/// functions are threadsafe.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheInitialize() -> i32 {
    if (unsafe { sqlite3Config.pcache2.xInit }) == None {
        // IMPLEMENTATION-OF: R-26801-64137 If the xInit() method is NULL, then the
        // built-in default page cache is used instead of the application defined
        // page cache.
        unsafe { sqlite3PCacheSetDefault() };
        0 as i32;
    }
    return unsafe {
        unsafe { sqlite3Config.pcache2.xInit }.unwrap()(unsafe { sqlite3Config.pcache2.pArg })
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheShutdown() {
    if (unsafe { sqlite3Config.pcache2.xShutdown }) != None {
        // IMPLEMENTATION-OF: R-26000-56589 The xShutdown() method may be NULL.
        unsafe {
            unsafe { sqlite3Config.pcache2.xShutdown }.unwrap()(unsafe {
                sqlite3Config.pcache2.pArg
            })
        };
    }
}

/// Return the size in bytes of a PCache object.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheSize() -> i32 {
    return ((80 as u64) as u32) as i32;
}

/// Create a new PCache object. Storage space to hold the object
/// has already been allocated and is passed in as the p pointer.
/// The caller discovers how much space needs to be allocated by
/// calling sqlite3PcacheSize().
///
/// szExtra is some extra space allocated for each page.  The first
/// 8 bytes of the extra space will be zeroed as the page is allocated,
/// but remaining content will be uninitialized.  Though it is opaque
/// to this module, the extra space really ends up being the MemPage
/// structure in the pager.
///
/// # Arguments
///
/// * `szPage` - Size of every page
/// * `szExtra` - Extra space associated with each page
/// * `bPurgeable` - True if pages are on backing store
/// * `xStress` - Call to try to make pages clean
/// * `pStress` - Argument to xStress
/// * `p` - Preallocated space for the PCache
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheOpen(
    mut szPage: i32,
    mut szExtra: i32,
    mut bPurgeable: i32,
    mut xStress: Option<unsafe extern "C-unwind" fn(*mut (), *mut PgHdr) -> i32>,
    mut pStress: *mut (),
    mut p: *mut PCache,
) -> i32 {
    unsafe { memset(p as *mut (), 0 as i32, 80 as u64) };
    unsafe {
        (*p).szPage = 1 as i32;
    }
    unsafe {
        (*p).szExtra = szExtra;
    }
    0 as i32; // First 8 bytes will be zeroed
    unsafe {
        (*p).bPurgeable = (bPurgeable as i8) as u8;
    }
    unsafe {
        (*p).eCreate = ((2 as i32) as i8) as u8;
    }
    unsafe {
        (*p).xStress = xStress;
    }
    unsafe {
        (*p).pStress = pStress;
    }
    unsafe {
        (*p).szCache = 100 as i32;
    }
    unsafe {
        (*p).szSpill = 1 as i32;
    }
    {}
    return sqlite3PcacheSetPageSize(p, szPage);
}

/// Change the page size for PCache object. The caller must ensure that there
/// are no outstanding page references when this function is called.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheSetPageSize(mut pCache: *mut PCache, mut szPage: i32) -> i32 {
    0 as i32;
    if (unsafe { (*pCache).szPage }) != (0 as i32) {
        let mut pNew: *mut sqlite3_pcache = unsafe { std::mem::zeroed() };
        pNew = unsafe {
            unsafe { sqlite3Config.pcache2.xCreate }.unwrap()(
                szPage,
                ((((unsafe { (*pCache).szExtra }) as i64) as u64).wrapping_add(
                    (80 as u64).wrapping_add(((7 as i32) as i64) as u64)
                        & ((!(7 as i32) as i64) as u64),
                ) as u32) as i32,
                ((unsafe { (*pCache).bPurgeable }) as u32) as i32,
            )
        };
        if pNew == std::ptr::null_mut::<sqlite3_pcache>() {
            return 7 as i32;
        }
        unsafe {
            unsafe { sqlite3Config.pcache2.xCachesize }.unwrap()(pNew, numberOfCachePages(pCache))
        };
        if (unsafe { (*pCache).pCache }) != std::ptr::null_mut::<sqlite3_pcache>() {
            unsafe {
                unsafe { sqlite3Config.pcache2.xDestroy }.unwrap()(unsafe { (*pCache).pCache })
            };
        }
        unsafe {
            (*pCache).pCache = pNew;
        }
        unsafe {
            (*pCache).szPage = szPage;
        }
        {}
    }
    return 0 as i32;
}

/// Try to obtain a page from the cache.
///
/// This routine returns a pointer to an sqlite3_pcache_page object if
/// such an object is already in cache, or if a new one is created.
/// This routine returns a NULL pointer if the object was not in cache
/// and could not be created.
///
/// The createFlags should be 0 to check for existing pages and should
/// be 3 (not 1, but 3) to try to create a new page.
///
/// If the createFlag is 0, then NULL is always returned if the page
/// is not already in the cache.  If createFlag is 1, then a new page
/// is created only if that can be done without spilling dirty pages
/// and without exceeding the cache size limit.
///
/// The caller needs to invoke sqlite3PcacheFetchFinish() to properly
/// initialize the sqlite3_pcache_page object and convert it into a
/// PgHdr object.  The sqlite3PcacheFetch() and sqlite3PcacheFetchFinish()
/// routines are split this way for performance reasons. When separated
/// they can both (usually) operate without having to push values to
/// the stack on entry and pop them back off on exit, which saves a
/// lot of pushing and popping.
///
/// # Arguments
///
/// * `pCache` - Obtain the page from this cache
/// * `pgno` - Page number to obtain
/// * `createFlag` - If true, create page if it does not exist already
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheFetch(
    mut pCache: *mut PCache,
    mut pgno: u32,
    mut createFlag: i32,
) -> *mut sqlite3_pcache_page {
    let mut eCreate: i32 = 0 as i32;
    let mut pRes: *mut sqlite3_pcache_page = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    // eCreate defines what to do if the page does not exist.
    // 0     Do not allocate a new page.  (createFlag==0)
    // 1     Allocate a new page if doing so is inexpensive.
    //       (createFlag==1 AND bPurgeable AND pDirty)
    // 2     Allocate a new page even it doing so is difficult.
    //       (createFlag==1 AND !(bPurgeable AND pDirty)
    eCreate = createFlag & (((unsafe { (*pCache).eCreate }) as u32) as i32);
    0 as i32;
    0 as i32;
    0 as i32;
    pRes = unsafe {
        unsafe { sqlite3Config.pcache2.xFetch }.unwrap()(unsafe { (*pCache).pCache }, pgno, eCreate)
    };
    {}
    {}
    return pRes;
}

/// If the sqlite3PcacheFetch() routine is unable to allocate a new
/// page because no clean pages are available for reuse and the cache
/// size limit has been reached, then this routine can be invoked to
/// try harder to allocate a page.  This routine might invoke the stress
/// callback to spill dirty pages to the journal.  It will then try to
/// allocate the new page and will only fail to allocate a new page on
/// an OOM error.
///
/// This routine should be invoked only after sqlite3PcacheFetch() fails.
///
/// # Arguments
///
/// * `pCache` - Obtain the page from this cache
/// * `pgno` - Page number to obtain
/// * `ppPage` - Write result here
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheFetchStress(
    mut pCache: *mut PCache,
    mut pgno: u32,
    mut ppPage: *mut *mut sqlite3_pcache_page,
) -> i32 {
    let mut pPg: *mut PgHdr = unsafe { std::mem::zeroed() };
    if (((unsafe { (*pCache).eCreate }) as u32) as i32) == (2 as i32) {
        return 0 as i32;
    }
    if sqlite3PcachePagecount(pCache) > unsafe { (*pCache).szSpill } {
        // Find a dirty page to write-out and recycle. First try to find a
        // page that does not require a journal-sync (one with PGHDR_NEED_SYNC
        // cleared), but if that is not possible settle for any other
        // unreferenced dirty page.
        //
        // If the LRU page in the dirty list that has a clear PGHDR_NEED_SYNC
        // flag is currently referenced, then the following may leave pSynced
        // set incorrectly (pointing to other than the LRU page with NEED_SYNC
        // cleared). This is Ok, as pSynced is just an optimization.
        pPg = unsafe { (*pCache).pSynced };
        '__slate_break_221: while pPg != std::ptr::null_mut::<PgHdr>()
            && ((unsafe { (*pPg).nRef }) != (0 as i64)
                || (((unsafe { (*pPg).flags }) as u32) as i32) & (8 as i32) != (0 as i32))
        {
            {}
            pPg = unsafe { (*pPg).pDirtyPrev };
        }
        unsafe {
            (*pCache).pSynced = pPg;
        }
        if !(pPg != std::ptr::null_mut::<PgHdr>()) {
            pPg = unsafe { (*pCache).pDirtyTail };
            '__slate_break_222: while pPg != std::ptr::null_mut::<PgHdr>()
                && (unsafe { (*pPg).nRef }) != (0 as i64)
            {
                {}
                pPg = unsafe { (*pPg).pDirtyPrev };
            }
        }
        if pPg != std::ptr::null_mut::<PgHdr>() {
            let mut rc: i32 = 0 as i32;
            {}
            rc =
                unsafe { unsafe { (*pCache).xStress }.unwrap()(unsafe { (*pCache).pStress }, pPg) };
            {}
            if rc != (0 as i32) && rc != (5 as i32) {
                return rc;
            }
        }
    }
    unsafe {
        *ppPage = unsafe {
            unsafe { sqlite3Config.pcache2.xFetch }.unwrap()(
                unsafe { (*pCache).pCache },
                pgno,
                2 as i32,
            )
        };
    }
    return if (unsafe { *ppPage }) == std::ptr::null_mut::<sqlite3_pcache_page>() {
        7 as i32
    } else {
        0 as i32
    };
}

/// This is a helper routine for sqlite3PcacheFetchFinish()
///
/// In the uncommon case where the page being fetched has not been
/// initialized, this routine is invoked to do the initialization.
/// This routine is broken out into a separate function since it
/// requires extra stack manipulation that can be avoided in the common
/// case.
///
/// # Arguments
///
/// * `pCache` - Obtain the page from this cache
/// * `pgno` - Page number obtained
/// * `pPage` - Page obtained by prior PcacheFetch() call
fn pcacheFetchFinishWithInit(
    mut pCache: *mut PCache,
    mut pgno: u32,
    mut pPage: *mut sqlite3_pcache_page,
) -> *mut PgHdr {
    let mut pPgHdr: *mut PgHdr = unsafe { std::mem::zeroed() };
    0 as i32;
    pPgHdr = (unsafe { (*pPage).pExtra }) as *mut PgHdr;
    0 as i32;
    unsafe {
        memset(
            (unsafe { std::ptr::addr_of_mut!((*pPgHdr).pDirty) }) as *mut (),
            0 as i32,
            (80 as u64).wrapping_sub(32 as u64),
        )
    };
    unsafe {
        (*pPgHdr).pPage = pPage;
    }
    unsafe {
        (*pPgHdr).pData = unsafe { (*pPage).pBuf };
    }
    unsafe {
        (*pPgHdr).pExtra = (unsafe { pPgHdr.offset((1 as i32) as isize) }) as *mut ();
    }
    unsafe {
        memset(
            unsafe { (*pPgHdr).pExtra },
            0 as i32,
            ((8 as i32) as i64) as u64,
        )
    };
    0 as i32;
    unsafe {
        (*pPgHdr).pCache = pCache;
    }
    unsafe {
        (*pPgHdr).pgno = pgno;
    }
    unsafe {
        (*pPgHdr).flags = ((1 as i32) as i16) as u16;
    }
    return sqlite3PcacheFetchFinish(pCache, pgno, pPage);
}

/// This routine converts the sqlite3_pcache_page object returned by
/// sqlite3PcacheFetch() into an initialized PgHdr object.  This routine
/// must be called after sqlite3PcacheFetch() in order to get a usable
/// result.
///
/// # Arguments
///
/// * `pCache` - Obtain the page from this cache
/// * `pgno` - Page number obtained
/// * `pPage` - Page obtained by prior PcacheFetch() call
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheFetchFinish(
    mut pCache: *mut PCache,
    mut pgno: u32,
    mut pPage: *mut sqlite3_pcache_page,
) -> *mut PgHdr {
    let mut pPgHdr: *mut PgHdr = unsafe { std::mem::zeroed() };
    0 as i32;
    pPgHdr = (unsafe { (*pPage).pExtra }) as *mut PgHdr;
    if !((unsafe { (*pPgHdr).pPage }) != std::ptr::null_mut::<sqlite3_pcache_page>()) {
        return pcacheFetchFinishWithInit(pCache, pgno, pPage);
    }
    let __v233: *mut PCache = pCache;
    let __v234: i64 = unsafe { (*__v233).nRefSum };
    let __v235: i64 = __v234 + ((1 as i32) as i64);
    unsafe {
        (*__v233).nRefSum = __v235;
    }
    let __v236: *mut PgHdr = pPgHdr;
    let __v237: i64 = unsafe { (*__v236).nRef };
    let __v238: i64 = __v237 + ((1 as i32) as i64);
    unsafe {
        (*__v236).nRef = __v238;
    }
    0 as i32;
    return pPgHdr;
}

/// Decrement the reference count on a page. If the page is clean and the
/// reference count drops to 0, then it is made eligible for recycling.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheRelease(mut p: *mut PgHdr) {
    0 as i32;
    let __v239: *mut PCache = unsafe { (*p).pCache };
    let __v240: i64 = unsafe { (*__v239).nRefSum };
    let __v241: i64 = __v240 - ((1 as i32) as i64);
    unsafe {
        (*__v239).nRefSum = __v241;
    }
    let __v242: *mut PgHdr = p;
    let __v243: i64 = unsafe { (*__v242).nRef };
    let __v244: i64 = __v243 - ((1 as i32) as i64);
    unsafe {
        (*__v242).nRef = __v244;
    }
    if __v244 == ((0 as i32) as i64) {
        if (((unsafe { (*p).flags }) as u32) as i32) & (1 as i32) != (0 as i32) {
            pcacheUnpin(p);
        } else {
            pcacheManageDirtyList(p, ((3 as i32) as i8) as u8);
            0 as i32;
        }
    }
}

/// Increase the reference count of a supplied page by 1.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheRef(mut p: *mut PgHdr) {
    0 as i32;
    0 as i32;
    let __v273: *mut PgHdr = p;
    let __v274: i64 = unsafe { (*__v273).nRef };
    let __v275: i64 = __v274 + ((1 as i32) as i64);
    unsafe {
        (*__v273).nRef = __v275;
    }
    let __v276: *mut PCache = unsafe { (*p).pCache };
    let __v277: i64 = unsafe { (*__v276).nRefSum };
    let __v278: i64 = __v277 + ((1 as i32) as i64);
    unsafe {
        (*__v276).nRefSum = __v278;
    }
}

/// Drop a page from the cache. There must be exactly one reference to the
/// page. This function deletes that reference, so after it returns the
/// page pointed to by p is invalid.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheDrop(mut p: *mut PgHdr) {
    0 as i32;
    0 as i32;
    if (((unsafe { (*p).flags }) as u32) as i32) & (2 as i32) != (0 as i32) {
        pcacheManageDirtyList(p, ((1 as i32) as i8) as u8);
    }
    let __v245: *mut PCache = unsafe { (*p).pCache };
    let __v246: i64 = unsafe { (*__v245).nRefSum };
    let __v247: i64 = __v246 - ((1 as i32) as i64);
    unsafe {
        (*__v245).nRefSum = __v247;
    }
    unsafe {
        unsafe { sqlite3Config.pcache2.xUnpin }.unwrap()(
            unsafe { (*unsafe { (*p).pCache }).pCache },
            unsafe { (*p).pPage },
            1 as i32,
        )
    };
}

/// Make sure the page is marked as dirty. If it isn't dirty already,
/// make it so.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheMakeDirty(mut p: *mut PgHdr) {
    0 as i32;
    0 as i32;
    if (((unsafe { (*p).flags }) as u32) as i32) & ((1 as i32) | (16 as i32)) != (0 as i32) {
        let __v248: *mut PgHdr = p;
        let __v249: u16 = unsafe { (*__v248).flags };
        let __v250: u16 = ((((__v249 as u32) as i32) & !(16 as i32)) as i16) as u16;
        unsafe {
            (*__v248).flags = __v250;
        }
        if (((unsafe { (*p).flags }) as u32) as i32) & (1 as i32) != (0 as i32) {
            let __v251: *mut PgHdr = p;
            let __v252: u16 = unsafe { (*__v251).flags };
            let __v253: u16 =
                ((((__v252 as u32) as i32) ^ ((2 as i32) | (1 as i32))) as i16) as u16;
            unsafe {
                (*__v251).flags = __v253;
            }
            {}
            0 as i32;
            pcacheManageDirtyList(p, ((2 as i32) as i8) as u8);
            0 as i32;
        }
        0 as i32;
    }
}

/// Make sure the page is marked as clean. If it isn't clean already,
/// make it so.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheMakeClean(mut p: *mut PgHdr) {
    0 as i32;
    0 as i32;
    0 as i32;
    pcacheManageDirtyList(p, ((1 as i32) as i8) as u8);
    let __v254: *mut PgHdr = p;
    let __v255: u16 = unsafe { (*__v254).flags };
    let __v256: u16 =
        ((((__v255 as u32) as i32) & !((2 as i32) | (8 as i32) | (4 as i32))) as i16) as u16;
    unsafe {
        (*__v254).flags = __v256;
    }
    let __v257: *mut PgHdr = p;
    let __v258: u16 = unsafe { (*__v257).flags };
    let __v259: u16 = ((((__v258 as u32) as i32) | (1 as i32)) as i16) as u16;
    unsafe {
        (*__v257).flags = __v259;
    }
    {}
    0 as i32;
    if (unsafe { (*p).nRef }) == ((0 as i32) as i64) {
        pcacheUnpin(p);
    }
}

/// Make every page in the cache clean.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheCleanAll(mut pCache: *mut PCache) {
    let mut p: *mut PgHdr = unsafe { std::mem::zeroed() };
    {}
    '__slate_break_223: loop {
        let __v260: *mut PgHdr = unsafe { (*pCache).pDirty };
        p = __v260;
        if !(__v260 != std::ptr::null_mut::<PgHdr>()) {
            break;
        }
        sqlite3PcacheMakeClean(p);
    }
}

/// Clear the PGHDR_NEED_SYNC and PGHDR_WRITEABLE flag from all dirty pages.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheClearWritable(mut pCache: *mut PCache) {
    let mut p: *mut PgHdr = unsafe { std::mem::zeroed() };
    {}
    p = unsafe { (*pCache).pDirty };
    '__slate_break_224: while p != std::ptr::null_mut::<PgHdr>() {
        let __v261: *mut PgHdr = p;
        let __v262: u16 = unsafe { (*__v261).flags };
        let __v263: u16 = ((((__v262 as u32) as i32) & !((8 as i32) | (4 as i32))) as i16) as u16;
        unsafe {
            (*__v261).flags = __v263;
        }
        p = unsafe { (*p).pDirtyNext };
    }
    unsafe {
        (*pCache).pSynced = unsafe { (*pCache).pDirtyTail };
    }
}

/// Clear the PGHDR_NEED_SYNC flag from all dirty pages.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheClearSyncFlags(mut pCache: *mut PCache) {
    let mut p: *mut PgHdr = unsafe { std::mem::zeroed() };
    p = unsafe { (*pCache).pDirty };
    '__slate_break_225: while p != std::ptr::null_mut::<PgHdr>() {
        let __v270: *mut PgHdr = p;
        let __v271: u16 = unsafe { (*__v270).flags };
        let __v272: u16 = ((((__v271 as u32) as i32) & !(8 as i32)) as i16) as u16;
        unsafe {
            (*__v270).flags = __v272;
        }
        p = unsafe { (*p).pDirtyNext };
    }
    unsafe {
        (*pCache).pSynced = unsafe { (*pCache).pDirtyTail };
    }
}

/// Change the page number of page p to newPgno.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheMove(mut p: *mut PgHdr, mut newPgno: u32) {
    let mut pCache: *mut PCache = unsafe { (*p).pCache };
    let mut pOther: *mut sqlite3_pcache_page = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    {}
    pOther = unsafe {
        unsafe { sqlite3Config.pcache2.xFetch }.unwrap()(
            unsafe { (*pCache).pCache },
            newPgno,
            0 as i32,
        )
    };
    if pOther != std::ptr::null_mut::<sqlite3_pcache_page>() {
        let mut pXPage: *mut PgHdr = (unsafe { (*pOther).pExtra }) as *mut PgHdr;
        0 as i32;
        let __v264: *mut PgHdr = pXPage;
        let __v265: i64 = unsafe { (*__v264).nRef };
        let __v266: i64 = __v265 + ((1 as i32) as i64);
        unsafe {
            (*__v264).nRef = __v266;
        }
        let __v267: *mut PCache = pCache;
        let __v268: i64 = unsafe { (*__v267).nRefSum };
        let __v269: i64 = __v268 + ((1 as i32) as i64);
        unsafe {
            (*__v267).nRefSum = __v269;
        }
        sqlite3PcacheDrop(pXPage);
    }
    unsafe {
        unsafe { sqlite3Config.pcache2.xRekey }.unwrap()(
            unsafe { (*pCache).pCache },
            unsafe { (*p).pPage },
            unsafe { (*p).pgno },
            newPgno,
        )
    };
    unsafe {
        (*p).pgno = newPgno;
    }
    if (((unsafe { (*p).flags }) as u32) as i32) & (2 as i32) != (0 as i32)
        && (((unsafe { (*p).flags }) as u32) as i32) & (8 as i32) != (0 as i32)
    {
        pcacheManageDirtyList(p, ((3 as i32) as i8) as u8);
        0 as i32;
    }
}

/// Drop every cache entry whose page number is greater than "pgno". The
/// caller must ensure that there are no outstanding references to any pages
/// other than page 1 with a page number greater than pgno.
///
/// If there is a reference to page 1 and the pgno parameter passed to this
/// function is 0, then the data area associated with page 1 is zeroed, but
/// the page object is not dropped.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheTruncate(mut pCache: *mut PCache, mut pgno: u32) {
    if (unsafe { (*pCache).pCache }) != std::ptr::null_mut::<sqlite3_pcache>() {
        let mut p: *mut PgHdr = unsafe { std::mem::zeroed() };
        let mut pNext: *mut PgHdr = unsafe { std::mem::zeroed() };
        {}
        p = unsafe { (*pCache).pDirty };
        '__slate_break_226: while p != std::ptr::null_mut::<PgHdr>() {
            pNext = unsafe { (*p).pDirtyNext };
            // This routine never gets call with a positive pgno except right
            // after sqlite3PcacheCleanAll().  So if there are dirty pages,
            // it must be that pgno==0.
            0 as i32;
            if (unsafe { (*p).pgno }) > pgno {
                0 as i32;
                sqlite3PcacheMakeClean(p);
            }
            p = pNext;
        }
        if pgno == ((0 as i32) as u32) && (unsafe { (*pCache).nRefSum }) != (0 as i64) {
            let mut pPage1: *mut sqlite3_pcache_page = unsafe { std::mem::zeroed() };
            pPage1 = unsafe {
                unsafe { sqlite3Config.pcache2.xFetch }.unwrap()(
                    unsafe { (*pCache).pCache },
                    (1 as i32) as u32,
                    0 as i32,
                )
            };
            if pPage1 != std::ptr::null_mut::<sqlite3_pcache_page>() {
                // Page 1 is always available in cache, because
                // pCache->nRefSum>0
                unsafe {
                    memset(
                        unsafe { (*pPage1).pBuf },
                        0 as i32,
                        ((unsafe { (*pCache).szPage }) as i64) as u64,
                    )
                };
                pgno = (1 as i32) as u32;
            }
        }
        unsafe {
            unsafe { sqlite3Config.pcache2.xTruncate }.unwrap()(
                unsafe { (*pCache).pCache },
                pgno.wrapping_add((1 as i32) as u32),
            )
        };
    }
}

/// Close a cache.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheClose(mut pCache: *mut PCache) {
    0 as i32;
    {}
    unsafe { unsafe { sqlite3Config.pcache2.xDestroy }.unwrap()(unsafe { (*pCache).pCache }) };
}

/// Discard the contents of the cache.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheClear(mut pCache: *mut PCache) {
    sqlite3PcacheTruncate(pCache, (0 as i32) as u32);
}

/// Merge two lists of pages connected by pDirty and in pgno order.
/// Do not bother fixing the pDirtyPrev pointers.
fn pcacheMergeDirtyList(mut pA: *mut PgHdr, mut pB: *mut PgHdr) -> *mut PgHdr {
    let mut result: PgHdr = unsafe { std::mem::zeroed() };
    let mut pTail: *mut PgHdr = unsafe { std::mem::zeroed() };
    pTail = std::ptr::addr_of_mut!(result);
    0 as i32;
    '__slate_break_227: loop {
        if (unsafe { (*pA).pgno }) < unsafe { (*pB).pgno } {
            unsafe {
                (*pTail).pDirty = pA;
            }
            pTail = pA;
            pA = unsafe { (*pA).pDirty };
            if pA == std::ptr::null_mut::<PgHdr>() {
                unsafe {
                    (*pTail).pDirty = pB;
                }
                break '__slate_break_227;
            }
        } else {
            unsafe {
                (*pTail).pDirty = pB;
            }
            pTail = pB;
            pB = unsafe { (*pB).pDirty };
            if pB == std::ptr::null_mut::<PgHdr>() {
                unsafe {
                    (*pTail).pDirty = pA;
                }
                break '__slate_break_227;
            }
        }
    }
    return result.pDirty;
}

// Sort the list of pages in ascending order by pgno.  Pages are
// connected by pDirty pointers.  The pDirtyPrev pointers are
// corrupted by this sort.
//
// Since there cannot be more than 2^31 distinct pages in a database,
// there cannot be more than 31 buckets required by the merge sorter.
// One extra bucket is added to catch overflow in case something
// ever changes to make the previous sentence incorrect.
fn pcacheSortDirtyList(mut pIn: *mut PgHdr) -> *mut PgHdr {
    let mut a: __SlateAlign16<[*mut PgHdr; 32]> = __SlateAlign16([0 as *mut PgHdr; 32]);
    let mut p: *mut PgHdr = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    unsafe {
        memset(
            (a.0.as_mut_ptr() as *mut *mut PgHdr) as *mut (),
            0 as i32,
            256 as u64,
        )
    };
    '__slate_break_228: while pIn != std::ptr::null_mut::<PgHdr>() {
        p = pIn;
        pIn = unsafe { (*p).pDirty };
        unsafe {
            (*p).pDirty = std::ptr::null_mut::<PgHdr>();
        }
        i = 0 as i32;
        '__slate_break_229: loop {
            if !(i < (32 as i32) - (1 as i32)) {
                break;
            }
            if (unsafe { *unsafe { (a.0.as_mut_ptr() as *mut *mut PgHdr).offset(i as isize) } })
                == std::ptr::null_mut::<PgHdr>()
            {
                unsafe {
                    *unsafe { (a.0.as_mut_ptr() as *mut *mut PgHdr).offset(i as isize) } = p;
                }
                break '__slate_break_229;
            } else {
                p = pcacheMergeDirtyList(
                    unsafe { *unsafe { (a.0.as_mut_ptr() as *mut *mut PgHdr).offset(i as isize) } },
                    p,
                );
                unsafe {
                    *unsafe { (a.0.as_mut_ptr() as *mut *mut PgHdr).offset(i as isize) } =
                        std::ptr::null_mut::<PgHdr>();
                }
            }
            let __v281: i32 = i;
            let __v282: i32 = __v281 + (1 as i32);
            i = __v282;
        }
        if i == (32 as i32) - (1 as i32) {
            // To get here, there need to be 2^(N_SORT_BUCKET) elements in
            // the input list.  But that is impossible.
            unsafe {
                *unsafe { (a.0.as_mut_ptr() as *mut *mut PgHdr).offset(i as isize) } =
                    pcacheMergeDirtyList(
                        unsafe {
                            *unsafe { (a.0.as_mut_ptr() as *mut *mut PgHdr).offset(i as isize) }
                        },
                        p,
                    );
            }
        }
    }
    p = unsafe { *unsafe { (a.0.as_mut_ptr() as *mut *mut PgHdr).offset((0 as i32) as isize) } };
    i = 1 as i32;
    '__slate_break_230: loop {
        if !(i < (32 as i32)) {
            break;
        }
        if (unsafe { *unsafe { (a.0.as_mut_ptr() as *mut *mut PgHdr).offset(i as isize) } })
            == std::ptr::null_mut::<PgHdr>()
        {
        } else {
            let __v285: *mut PgHdr;
            if p != std::ptr::null_mut::<PgHdr>() {
                __v285 = pcacheMergeDirtyList(p, unsafe {
                    *unsafe { (a.0.as_mut_ptr() as *mut *mut PgHdr).offset(i as isize) }
                });
            } else {
                __v285 =
                    unsafe { *unsafe { (a.0.as_mut_ptr() as *mut *mut PgHdr).offset(i as isize) } };
            }
            p = __v285;
        }
        let __v283: i32 = i;
        let __v284: i32 = __v283 + (1 as i32);
        i = __v284;
    }
    return p;
}

/// Return a list of all dirty pages in the cache, sorted by page number.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheDirtyList(mut pCache: *mut PCache) -> *mut PgHdr {
    let mut p: *mut PgHdr = unsafe { std::mem::zeroed() };
    p = unsafe { (*pCache).pDirty };
    '__slate_break_231: while p != std::ptr::null_mut::<PgHdr>() {
        unsafe {
            (*p).pDirty = unsafe { (*p).pDirtyNext };
        }
        p = unsafe { (*p).pDirtyNext };
    }
    return pcacheSortDirtyList(unsafe { (*pCache).pDirty });
}

/// Return the total number of references to all pages held by the cache.
///
/// This is not the total number of pages referenced, but the sum of the
/// reference count for all pages.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheRefCount(mut pCache: *mut PCache) -> i64 {
    return unsafe { (*pCache).nRefSum };
}

/// Return the number of references to the page supplied as an argument.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcachePageRefcount(mut p: *mut PgHdr) -> i64 {
    return unsafe { (*p).nRef };
}

/// Return the total number of pages in the cache.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcachePagecount(mut pCache: *mut PCache) -> i32 {
    0 as i32;
    return unsafe {
        unsafe { sqlite3Config.pcache2.xPagecount }.unwrap()(unsafe { (*pCache).pCache })
    };
}

/// Set the suggested cache-size value.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheSetCachesize(mut pCache: *mut PCache, mut mxPage: i32) {
    0 as i32;
    unsafe {
        (*pCache).szCache = mxPage;
    }
    unsafe {
        unsafe { sqlite3Config.pcache2.xCachesize }.unwrap()(
            unsafe { (*pCache).pCache },
            numberOfCachePages(pCache),
        )
    };
}

/// Set the suggested cache-spill value.  Make no changes if if the
/// argument is zero.  Return the effective cache-spill size, which will
/// be the larger of the szSpill and szCache.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheSetSpillsize(mut p: *mut PCache, mut mxPage: i32) -> i32 {
    let mut res: i32 = 0 as i32;
    0 as i32;
    if mxPage != (0 as i32) {
        if mxPage < (0 as i32) {
            mxPage = ((-(1024 as i32) as i64) * (mxPage as i64)
                / (((unsafe { (*p).szPage }) + unsafe { (*p).szExtra }) as i64))
                as i32;
        }
        unsafe {
            (*p).szSpill = mxPage;
        }
    }
    res = numberOfCachePages(p);
    if res < unsafe { (*p).szSpill } {
        res = unsafe { (*p).szSpill };
    }
    return res;
}

/// Free up as much memory as possible from the page cache.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PcacheShrink(mut pCache: *mut PCache) {
    0 as i32;
    unsafe { unsafe { sqlite3Config.pcache2.xShrink }.unwrap()(unsafe { (*pCache).pCache }) };
}

/// Return the size of the header added by this middleware layer
/// in the page-cache hierarchy.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3HeaderSizePcache() -> i32 {
    return (((80 as u64).wrapping_add(((7 as i32) as i64) as u64) & ((!(7 as i32) as i64) as u64))
        as u32) as i32;
}

/// Return the number of dirty pages currently in the cache, as a percentage
/// of the configured cache size.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PCachePercentDirty(mut pCache: *mut PCache) -> i32 {
    let mut pDirty: *mut PgHdr = unsafe { std::mem::zeroed() };
    let mut nDirty: i32 = 0 as i32;
    let mut nCache: i32 = numberOfCachePages(pCache);
    pDirty = unsafe { (*pCache).pDirty };
    '__slate_break_232: while pDirty != std::ptr::null_mut::<PgHdr>() {
        let __v279: i32 = nDirty;
        let __v280: i32 = __v279 + (1 as i32);
        nDirty = __v280;
        pDirty = unsafe { (*pDirty).pDirtyNext };
    }
    return if nCache != (0 as i32) {
        ((nDirty as i64) * ((100 as i32) as i64) / (nCache as i64)) as i32
    } else {
        0 as i32
    };
}

/// Return true if there are one or more dirty pages in the cache. Else false.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PCacheIsDirty(mut pCache: *mut PCache) -> i32 {
    return ((unsafe { (*pCache).pDirty }) != std::ptr::null_mut::<PgHdr>()) as i32;
}
