//! 2008 November 05
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
//! This file implements the default page cache implementation (the
//! sqlite3_pcache interface). It also contains part of the implementation
//! of the SQLITE_CONFIG_PAGECACHE and sqlite3_release_memory() features.
//! If the default page cache implementation is overridden, then neither of
//! these two features are available.
//!
//! A Page cache line looks like this:
//!
//!  |  database page content   |  PgHdr1  |  MemPage  |  PgHdr  |
//!
//! The database page content is up front (so that buffer overreads tend to
//! flow harmlessly into the PgHdr1, MemPage, and PgHdr extensions).   MemPage
//! is the extension added by the btree.c module containing information such
//! as the database page number and how that database page is used.  PgHdr
//! is added by the pcache.c layer and contains information used to keep track
//! of which pages are "dirty".  PgHdr1 is an extension added by this
//! module (pcache1.c).  The PgHdr1 header is a subclass of sqlite3_pcache_page.
//! PgHdr1 contains information needed to look up a page by its page number.
//! The superclass sqlite3_pcache_page.pBuf points to the start of the
//! database page content and sqlite3_pcache_page.pExtra points to PgHdr.
//!
//! The size of the extension (MemPage+PgHdr+PgHdr1) can be determined at
//! runtime using sqlite3_config(SQLITE_CONFIG_PCACHE_HDRSZ, &size).  The
//! sizes of the extensions sum to 272 bytes on x64 for 3.8.10, but this
//! size can vary according to architecture, compile-time options, and
//! SQLite library version number.
//!
//! Historical note:  It used to be that if the SQLITE_PCACHE_SEPARATE_HEADER
//! was defined, then the page content would be held in a separate memory
//! allocation from the PgHdr1.  This was intended to avoid clownshoe memory
//! allocations.  However, the btree layer needs a small (16-byte) overrun
//! area after the page content buffer.  The header serves as that overrun
//! area.  Therefore SQLITE_PCACHE_SEPARATE_HEADER was discontinued to avoid
//! any possibility of a memory error.
//!
//! This module tracks pointers to PgHdr1 objects.  Only pcache.c communicates
//! with this module.  Information is passed back and forth as PgHdr1 pointers.
//!
//! The pcache.c and pager.c modules deal pointers to PgHdr objects.
//! The btree.c module deals with pointers to MemPage objects.
//!
//! SOURCE OF PAGE CACHE MEMORY:
//!
//! Memory for a page might come from any of three sources:
//!
//!    (1)  The general-purpose memory allocator - sqlite3Malloc()
//!    (2)  Global page-cache memory provided using sqlite3_config() with
//!         SQLITE_CONFIG_PAGECACHE.
//!    (3)  PCache-local bulk allocation.
//!
//! The third case is a chunk of heap memory (defaulting to 100 pages worth)
//! that is allocated when the page cache is created.  The size of the local
//! bulk allocation can be adjusted using
//!
//!     sqlite3_config(SQLITE_CONFIG_PAGECACHE, (void*)0, 0, N).
//!
//! If N is positive, then N pages worth of memory are allocated using a single
//! sqlite3Malloc() call and that memory is used for the first N pages allocated.
//! Or if N is negative, then -1024*N bytes of memory are allocated and used
//! for as many pages as can be accommodated.
//!
//! Only one of (2) or (3) can be used.  Once the memory available to (2) or
//! (3) is exhausted, subsequent allocations fail over to the general-purpose
//! memory allocator (1).
//!
//! Earlier versions of SQLite used only methods (1) and (2).  But experiments
//! show that method (3) with N==100 provides about a 5% performance boost for
//! common workloads.
unsafe extern "C" {
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_config(__v207: i32, ...) -> i32;
    fn sqlite3_free(__v208: *mut ());
    fn sqlite3_mutex_enter(__v209: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(__v210: *mut sqlite3_mutex);
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3Malloc(__v217: u64) -> *mut ();
    fn sqlite3MallocZero(__v218: u64) -> *mut ();
    fn sqlite3MallocSize(__v219: *const ()) -> i32;
    fn sqlite3HeapNearlyFull() -> i32;
    fn sqlite3MutexAlloc(__v222: i32) -> *mut sqlite3_mutex;
    fn sqlite3StatusUp(__v223: i32, __v224: i32);
    fn sqlite3StatusDown(__v225: i32, __v226: i32);
    fn sqlite3StatusHighwater(__v227: i32, __v228: i32);
    fn sqlite3BeginBenignMalloc();
    fn sqlite3EndBenignMalloc();
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

/// Each cache entry is represented by an instance of the following
/// structure. A buffer of PgHdr1.pCache->szPage bytes is allocated
/// directly before this structure and is used to cache the page content.
///
/// When reading a corrupt database file, it is possible that SQLite might
/// read a few bytes (no more than 16 bytes) past the end of the page buffer.
/// It will only read past the end of the page buffer, never write.  This
/// object is positioned immediately after the page buffer to serve as an
/// overrun area, so that overreads are harmless.
///
/// Variables isBulkLocal and isAnchor were once type "u8". That works,
/// but causes a 2-byte gap in the structure for most architectures (since
/// pointers must be either 4 or 8-byte aligned). As this structure is located
/// in memory directly after the associated page data, if the database is
/// corrupt, code at the b-tree layer may overread the page buffer and
/// read part of this structure before the corruption is detected. This
/// can cause a valgrind error if the uninitialized gap is accessed. Using u16
/// ensures there is no such gap, and therefore no bytes of uninitialized
/// memory in the structure.
///
/// The pLruNext and pLruPrev pointers form a double-linked circular list
/// of all pages that are unpinned.  The PGroup.lru element (which should be
/// the only element on the list with PgHdr1.isAnchor set to 1) forms the
/// beginning and the end of the list.
#[repr(C)]
#[derive(Clone, Copy)]
struct PgHdr1 {
    /// Base class. Must be first. pBuf & pExtra
    page: sqlite3_pcache_page,
    /// Key value (page number)
    iKey: u32,
    /// This page from bulk local storage
    isBulkLocal: u16,
    /// This is the PGroup.lru element
    isAnchor: u16,
    /// Next in hash table chain
    pNext: *mut PgHdr1,
    /// Cache that currently owns this page
    pCache: *mut PCache1,
    /// Next in circular LRU list of unpinned pages
    pLruNext: *mut PgHdr1,
    /// Previous in LRU list of unpinned pages
    ///
    /// NB: pLruPrev is only valid if pLruNext!=0
    pLruPrev: *mut PgHdr1,
}

// A page is pinned if it is not on the LRU list.  To be "pinned" means
// that the page is in active use and must not be deallocated.
/// Each page cache (or PCache) belongs to a PGroup.  A PGroup is a set
/// of one or more PCaches that are able to recycle each other's unpinned
/// pages when they are under memory pressure.  A PGroup is an instance of
/// the following object.
///
/// This page cache implementation works in one of two modes:
///
///   (1)  Every PCache is the sole member of its own PGroup.  There is
///        one PGroup per PCache.
///
///   (2)  There is a single global PGroup that all PCaches are a member
///        of.
///
/// Mode 1 uses more memory (since PCache instances are not able to rob
/// unused pages from other PCaches) but it also operates without a mutex,
/// and is therefore often faster.  Mode 2 requires a mutex in order to be
/// threadsafe, but recycles pages more efficiently.
///
/// For mode (1), PGroup.mutex is NULL.  For mode (2) there is only a single
/// PGroup which is the pcache1.grp global variable and its mutex is
/// SQLITE_MUTEX_STATIC_LRU.
#[repr(C)]
#[derive(Clone, Copy)]
struct PGroup {
    /// MUTEX_STATIC_LRU or NULL
    mutex: *mut sqlite3_mutex,
    /// Sum of nMax for purgeable caches
    nMaxPage: u32,
    /// Sum of nMin for purgeable caches
    nMinPage: u32,
    /// nMaxpage + 10 - nMinPage
    mxPinned: u32,
    /// Number of purgeable pages allocated
    nPurgeable: u32,
    /// The beginning and end of the LRU list
    lru: PgHdr1,
}

/// Each page cache is an instance of the following object.  Every
/// open database file (including each in-memory database and each
/// temporary or transient database) has a single page cache which
/// is an instance of this object.
///
/// Pointers to structures of this type are cast and returned as
/// opaque sqlite3_pcache* handles.
#[repr(C)]
#[derive(Clone, Copy)]
struct PCache1 {
    /// Cache configuration parameters. Page size (szPage) and the purgeable
    /// flag (bPurgeable) and the pnPurgeable pointer are all set when the
    /// cache is created and are never changed thereafter. nMax may be
    /// modified at any time by a call to the pcache1Cachesize() method.
    /// The PGroup mutex must be held when accessing nMax.
    /// PGroup this cache belongs to
    pGroup: *mut PGroup,
    /// Pointer to pGroup->nPurgeable
    pnPurgeable: *mut u32,
    /// Size of database content section
    szPage: i32,
    /// sizeof(MemPage)+sizeof(PgHdr)
    szExtra: i32,
    /// Total size of one pcache line
    szAlloc: i32,
    /// True if cache is purgeable
    bPurgeable: i32,
    /// Minimum number of pages reserved
    nMin: u32,
    /// Configured "cache_size" value
    nMax: u32,
    /// nMax*9/10
    n90pct: u32,
    /// Largest key seen since xTruncate()
    iMaxKey: u32,
    /// pnPurgeable points here when not used
    nPurgeableDummy: u32,
    /// Hash table of all pages. The following variables may only be accessed
    /// when the accessor is holding the PGroup mutex.
    /// Number of pages in the LRU list
    nRecyclable: u32,
    /// Total number of pages in apHash
    nPage: u32,
    /// Number of slots in apHash[]
    nHash: u32,
    /// Hash table for fast lookup by key
    apHash: *mut *mut PgHdr1,
    /// List of unused pcache-local pages
    pFree: *mut PgHdr1,
    /// Bulk memory used by pcache-local
    pBulk: *mut (),
}

/// Free slots in the allocator used to divide up the global page cache
/// buffer provided using the SQLITE_CONFIG_PAGECACHE mechanism.
#[repr(C)]
#[derive(Clone, Copy)]
struct PgFreeslot {
    /// Next free slot
    pNext: *mut PgFreeslot,
}

/// Global data used by this cache.
#[repr(C)]
#[derive(Clone, Copy)]
struct PCacheGlobal {
    /// The global PGroup for mode (2)
    grp: PGroup,
    /// Variables related to SQLITE_CONFIG_PAGECACHE settings.  The
    /// szSlot, nSlot, pStart, pEnd, nReserve, and isInit values are all
    /// fixed at sqlite3_initialize() time and do not require mutex protection.
    /// The nFreeSlot and pFree values do require mutex protection.
    /// True if initialized
    isInit: i32,
    /// Use a new PGroup for each PCache
    separateCache: i32,
    /// Initial bulk allocation size
    nInitPage: i32,
    /// Size of each free slot
    szSlot: i32,
    /// The number of pcache slots
    nSlot: i32,
    /// Try to keep nFreeSlot above this
    nReserve: i32,
    pStart: *mut (),
    /// Bounds of global page cache memory
    pEnd: *mut (),
    /// Above requires no mutex.  Use mutex below for variable that follow.
    /// Mutex for accessing the following:
    mutex: *mut sqlite3_mutex,
    /// Free page blocks
    pFree: *mut PgFreeslot,
    /// Number of unused pcache slots
    nFreeSlot: i32,
    /// True if low on PAGECACHE memory
    bUnderPressure: i32,
}

static mut pcache1_g: PCacheGlobal = unsafe { std::mem::zeroed() };

// All code in this file should access the global structure above via the
// alias "pcache1". This ensures that the WSD emulation is used when
// compiling for systems that do not support real WSD.
// Macros to enter and leave the PCache LRU mutex.
// Page Allocation/SQLITE_CONFIG_PCACHE Related Functions
/// This function is called during initialization if a static buffer is
/// supplied to use for the page-cache by passing the SQLITE_CONFIG_PAGECACHE
/// verb to sqlite3_config(). Parameter pBuf points to an allocation large
/// enough to contain 'n' buffers of 'sz' bytes each.
///
/// This routine is called from sqlite3_initialize() and so it is guaranteed
/// to be serialized already.  There is no need for further mutexing.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PCacheBufferSetup(mut pBuf: *mut (), mut sz: i32, mut n: i32) {
    if (unsafe { pcache1_g.isInit }) != (0 as i32) {
        let mut p: *mut PgFreeslot = unsafe { std::mem::zeroed() };
        if pBuf == std::ptr::null_mut::<()>() {
            n = 0 as i32;
            sz = 0 as i32;
        }
        if n == (0 as i32) {
            sz = 0 as i32;
        }
        sz = sz & !(7 as i32);
        unsafe {
            pcache1_g.szSlot = sz;
        }
        let __v240: i32 = n;
        unsafe {
            pcache1_g.nFreeSlot = __v240;
        }
        unsafe {
            pcache1_g.nSlot = __v240;
        }
        unsafe {
            pcache1_g.nReserve = if n > (90 as i32) {
                10 as i32
            } else {
                n / (10 as i32) + (1 as i32)
            };
        }
        unsafe {
            pcache1_g.pStart = pBuf;
        }
        unsafe {
            pcache1_g.pFree = std::ptr::null_mut::<PgFreeslot>();
        }
        unsafe {
            std::sync::atomic::AtomicI32::from_ptr(
                (unsafe { std::ptr::addr_of_mut!(pcache1_g.bUnderPressure) }) as *mut i32,
            )
            .store(0 as i32, std::sync::atomic::Ordering::Relaxed)
        };
        '__slate_break_229: loop {
            let __v241: i32 = n;
            let __v242: i32 = __v241 - (1 as i32);
            n = __v242;
            if !(__v241 != (0 as i32)) {
                break;
            }
            p = pBuf as *mut PgFreeslot;
            unsafe {
                (*p).pNext = unsafe { pcache1_g.pFree };
            }
            unsafe {
                pcache1_g.pFree = p;
            }
            pBuf = (unsafe { (pBuf as *mut i8).offset(sz as isize) }) as *mut ();
        }
        unsafe {
            pcache1_g.pEnd = pBuf;
        }
    }
}

/// Try to initialize the pCache->pFree and pCache->pBulk fields.  Return
/// true if pCache->pFree ends up containing one or more free pages.
fn pcache1InitBulk(mut pCache: *mut PCache1) -> i32 {
    let mut szBulk: i64 = 0 as i64;
    let mut zBulk: *mut i8 = unsafe { std::mem::zeroed() };
    if (unsafe { pcache1_g.nInitPage }) == (0 as i32) {
        return 0 as i32;
    }
    // Do not bother with a bulk allocation if the cache size very small
    if (unsafe { (*pCache).nMax }) < ((3 as i32) as u32) {
        return 0 as i32;
    }
    unsafe { sqlite3BeginBenignMalloc() };
    if (unsafe { pcache1_g.nInitPage }) > (0 as i32) {
        szBulk =
            ((unsafe { (*pCache).szAlloc }) as i64) * ((unsafe { pcache1_g.nInitPage }) as i64);
    } else {
        szBulk = (-(1024 as i32) as i64) * ((unsafe { pcache1_g.nInitPage }) as i64);
    }
    if szBulk
        > ((unsafe { (*pCache).szAlloc }) as i64) * (((unsafe { (*pCache).nMax }) as u64) as i64)
    {
        szBulk =
            ((unsafe { (*pCache).szAlloc }) as i64) * (((unsafe { (*pCache).nMax }) as u64) as i64);
    }
    if szBulk >= ((unsafe { (*pCache).szAlloc }) as i64) {
        let __v243: *mut () = unsafe { sqlite3Malloc(szBulk as u64) };
        unsafe {
            (*pCache).pBulk = __v243;
        }
        zBulk = __v243 as *mut i8;
        unsafe { sqlite3EndBenignMalloc() };
        if zBulk != std::ptr::null_mut::<i8>() {
            let mut nBulk: i32 =
                (unsafe { sqlite3MallocSize(zBulk as *const ()) }) / unsafe { (*pCache).szAlloc };
            '__slate_break_230: loop {
                let mut pX: *mut PgHdr1 =
                    (unsafe { zBulk.offset((unsafe { (*pCache).szPage }) as isize) })
                        as *mut PgHdr1;
                unsafe {
                    (*pX).page.pBuf = zBulk as *mut ();
                }
                unsafe {
                    (*pX).page.pExtra = (unsafe {
                        (pX as *mut u8).offset(
                            ((56 as u64).wrapping_add(((7 as i32) as i64) as u64)
                                & ((!(7 as i32) as i64) as u64))
                                as isize,
                        )
                    }) as *mut ();
                }
                0 as i32;
                unsafe {
                    (*pX).isBulkLocal = ((1 as i32) as i16) as u16;
                }
                unsafe {
                    (*pX).isAnchor = ((0 as i32) as i16) as u16;
                }
                unsafe {
                    (*pX).pNext = unsafe { (*pCache).pFree };
                }
                unsafe {
                    (*pX).pLruPrev = std::ptr::null_mut::<PgHdr1>();
                }
                // Initializing this saves a valgrind error
                unsafe {
                    (*pCache).pFree = pX;
                }
                let __v244: *mut i8 = zBulk;
                let __v245: *mut i8 =
                    unsafe { __v244.offset((unsafe { (*pCache).szAlloc }) as isize) };
                zBulk = __v245;
                let __v246: i32 = nBulk;
                let __v247: i32 = __v246 - (1 as i32);
                nBulk = __v247;
                if !(__v247 != (0 as i32)) {
                    break;
                }
            }
        }
    }
    return ((unsafe { (*pCache).pFree }) != std::ptr::null_mut::<PgHdr1>()) as i32;
}

/// Malloc function used within this file to allocate space from the buffer
/// configured using sqlite3_config(SQLITE_CONFIG_PAGECACHE) option. If no
/// such buffer exists or there is no space left in it, this function falls
/// back to sqlite3Malloc().
///
/// Multiple threads can run this routine at the same time.  Global variables
/// in pcache1 need to be protected via mutex.
fn pcache1Alloc(mut nByte: i32) -> *mut () {
    let mut p: *mut () = std::ptr::null_mut::<()>();
    0 as i32;
    if nByte <= unsafe { pcache1_g.szSlot } {
        unsafe { sqlite3_mutex_enter(unsafe { pcache1_g.mutex }) };
        p = ((unsafe { pcache1_g.pFree }) as *mut PgHdr1) as *mut ();
        if p != std::ptr::null_mut::<()>() {
            unsafe {
                pcache1_g.pFree = unsafe { (*unsafe { pcache1_g.pFree }).pNext };
            }
            let __v248: i32 = unsafe { pcache1_g.nFreeSlot };
            let __v249: i32 = __v248 - (1 as i32);
            unsafe {
                pcache1_g.nFreeSlot = __v249;
            }
            unsafe {
                std::sync::atomic::AtomicI32::from_ptr(
                    (unsafe { std::ptr::addr_of_mut!(pcache1_g.bUnderPressure) }) as *mut i32,
                )
                .store(
                    ((unsafe { pcache1_g.nFreeSlot }) < unsafe { pcache1_g.nReserve }) as i32,
                    std::sync::atomic::Ordering::Relaxed,
                )
            };
            0 as i32;
            unsafe { sqlite3StatusHighwater(7 as i32, nByte) };
            unsafe { sqlite3StatusUp(1 as i32, 1 as i32) };
        }
        unsafe { sqlite3_mutex_leave(unsafe { pcache1_g.mutex }) };
    }
    if p == std::ptr::null_mut::<()>() {
        // Memory is not available in the SQLITE_CONFIG_PAGECACHE pool.  Get
        // it from sqlite3Malloc instead.
        p = unsafe { sqlite3Malloc((nByte as i64) as u64) };
        if p != std::ptr::null_mut::<()>() {
            let mut sz: i32 = unsafe { sqlite3MallocSize(p as *const ()) };
            unsafe { sqlite3_mutex_enter(unsafe { pcache1_g.mutex }) };
            unsafe { sqlite3StatusHighwater(7 as i32, nByte) };
            unsafe { sqlite3StatusUp(2 as i32, sz) };
            unsafe { sqlite3_mutex_leave(unsafe { pcache1_g.mutex }) };
        }
        {}
    }
    return p;
}

/// Free an allocated buffer obtained from pcache1Alloc().
fn pcache1Free(mut p: *mut ()) {
    if p == std::ptr::null_mut::<()>() {
        return;
    }
    if (p as u64) >= ((unsafe { pcache1_g.pStart }) as u64)
        && (p as u64) < ((unsafe { pcache1_g.pEnd }) as u64)
    {
        let mut pSlot: *mut PgFreeslot = unsafe { std::mem::zeroed() };
        unsafe { sqlite3_mutex_enter(unsafe { pcache1_g.mutex }) };
        unsafe { sqlite3StatusDown(1 as i32, 1 as i32) };
        pSlot = p as *mut PgFreeslot;
        unsafe {
            (*pSlot).pNext = unsafe { pcache1_g.pFree };
        }
        unsafe {
            pcache1_g.pFree = pSlot;
        }
        let __v250: i32 = unsafe { pcache1_g.nFreeSlot };
        let __v251: i32 = __v250 + (1 as i32);
        unsafe {
            pcache1_g.nFreeSlot = __v251;
        }
        unsafe {
            std::sync::atomic::AtomicI32::from_ptr(
                (unsafe { std::ptr::addr_of_mut!(pcache1_g.bUnderPressure) }) as *mut i32,
            )
            .store(
                ((unsafe { pcache1_g.nFreeSlot }) < unsafe { pcache1_g.nReserve }) as i32,
                std::sync::atomic::Ordering::Relaxed,
            )
        };
        0 as i32;
        unsafe { sqlite3_mutex_leave(unsafe { pcache1_g.mutex }) };
    } else {
        0 as i32;
        {}
        let mut nFreed: i32 = 0 as i32;
        nFreed = unsafe { sqlite3MallocSize(p as *const ()) };
        unsafe { sqlite3_mutex_enter(unsafe { pcache1_g.mutex }) };
        unsafe { sqlite3StatusDown(2 as i32, nFreed) };
        unsafe { sqlite3_mutex_leave(unsafe { pcache1_g.mutex }) };
        unsafe { sqlite3_free(p) };
    }
}

/// Allocate a new page object initially associated with cache pCache.
fn pcache1AllocPage(mut pCache: *mut PCache1, mut benignMalloc: i32) -> *mut PgHdr1 {
    let mut p: *mut PgHdr1 = std::ptr::null_mut::<PgHdr1>();
    let mut pPg: *mut () = unsafe { std::mem::zeroed() };
    0 as i32;
    let __v252: bool;
    if (unsafe { (*pCache).pFree }) != std::ptr::null_mut::<PgHdr1>() {
        __v252 = true as bool;
    } else {
        let __v253: bool;
        if (unsafe { (*pCache).nPage }) == ((0 as i32) as u32) {
            __v253 = pcache1InitBulk(pCache) != (0 as i32);
        } else {
            __v253 = false as bool;
        }
        __v252 = __v253;
    }
    if __v252 {
        0 as i32;
        p = unsafe { (*pCache).pFree };
        unsafe {
            (*pCache).pFree = unsafe { (*p).pNext };
        }
        unsafe {
            (*p).pNext = std::ptr::null_mut::<PgHdr1>();
        }
    } else {
        if benignMalloc != (0 as i32) {
            unsafe { sqlite3BeginBenignMalloc() };
        }
        pPg = pcache1Alloc(unsafe { (*pCache).szAlloc });
        if benignMalloc != (0 as i32) {
            unsafe { sqlite3EndBenignMalloc() };
        }
        if pPg == std::ptr::null_mut::<()>() {
            return std::ptr::null_mut::<PgHdr1>();
        }
        p = (unsafe { (pPg as *mut u8).offset((unsafe { (*pCache).szPage }) as isize) })
            as *mut PgHdr1;
        unsafe {
            (*p).page.pBuf = pPg;
        }
        unsafe {
            (*p).page.pExtra = (unsafe {
                (p as *mut u8).offset(
                    ((56 as u64).wrapping_add(((7 as i32) as i64) as u64)
                        & ((!(7 as i32) as i64) as u64)) as isize,
                )
            }) as *mut ();
        }
        0 as i32;
        unsafe {
            (*p).isBulkLocal = ((0 as i32) as i16) as u16;
        }
        unsafe {
            (*p).isAnchor = ((0 as i32) as i16) as u16;
        }
        unsafe {
            (*p).pLruPrev = std::ptr::null_mut::<PgHdr1>();
        }
        // Initializing this saves a valgrind error
    }
    let __v254: *mut u32 = unsafe { (*pCache).pnPurgeable };
    let __v255: u32 = unsafe { *__v254 };
    let __v256: u32 = __v255.wrapping_add((1 as i32) as u32);
    unsafe {
        *__v254 = __v256;
    }
    return p;
}

/// Free a page object allocated by pcache1AllocPage().
fn pcache1FreePage(mut p: *mut PgHdr1) {
    let mut pCache: *mut PCache1 = unsafe { std::mem::zeroed() };
    0 as i32;
    pCache = unsafe { (*p).pCache };
    0 as i32;
    if (unsafe { (*p).isBulkLocal }) != (0 as u16) {
        unsafe {
            (*p).pNext = unsafe { (*pCache).pFree };
        }
        unsafe {
            (*pCache).pFree = p;
        }
    } else {
        pcache1Free(unsafe { (*p).page.pBuf });
    }
    let __v257: *mut u32 = unsafe { (*pCache).pnPurgeable };
    let __v258: u32 = unsafe { *__v257 };
    let __v259: u32 = __v258.wrapping_sub((1 as i32) as u32);
    unsafe {
        *__v257 = __v259;
    }
}

/// Malloc function used by SQLite to obtain space from the buffer configured
/// using sqlite3_config(SQLITE_CONFIG_PAGECACHE) option. If no such buffer
/// exists, this function falls back to sqlite3Malloc().
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PageMalloc(mut sz: i32) -> *mut () {
    0 as i32; // These allocations are never very large
    return pcache1Alloc(sz);
}

/// Free an allocated buffer obtained from sqlite3PageMalloc().
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PageFree(mut p: *mut ()) {
    pcache1Free(p);
}

/// Return true if it desirable to avoid allocating a new page cache
/// entry.
///
/// If memory was allocated specifically to the page cache using
/// SQLITE_CONFIG_PAGECACHE but that memory has all been used, then
/// it is desirable to avoid allocating a new page cache entry because
/// presumably SQLITE_CONFIG_PAGECACHE was suppose to be sufficient
/// for all page cache needs and we should not need to spill the
/// allocation onto the heap.
///
/// Or, the heap is used for all page cache memory but the heap is
/// under memory pressure, then again it is desirable to avoid
/// allocating a new page cache entry in order to avoid stressing
/// the heap even further.
fn pcache1UnderMemoryPressure(mut pCache: *mut PCache1) -> i32 {
    if (unsafe { pcache1_g.nSlot }) != (0 as i32)
        && (unsafe { (*pCache).szPage }) + unsafe { (*pCache).szExtra }
            <= unsafe { pcache1_g.szSlot }
    {
        return unsafe {
            std::sync::atomic::AtomicI32::from_ptr(
                (unsafe { std::ptr::addr_of_mut!(pcache1_g.bUnderPressure) }) as *mut i32,
            )
            .load(std::sync::atomic::Ordering::Relaxed)
        };
    } else {
        return unsafe { sqlite3HeapNearlyFull() };
    }
    return unsafe { std::mem::zeroed() };
}

// General Implementation Functions
/// This function is used to resize the hash table used by the cache passed
/// as the first argument.
///
/// The PCache mutex must be held when this function is called.
fn pcache1ResizeHash(mut p: *mut PCache1) {
    let mut apNew: *mut *mut PgHdr1 = unsafe { std::mem::zeroed() };
    let mut nNew: u64 = 0 as u64;
    let mut i: u32 = 0 as u32;
    0 as i32;
    nNew = (((2 as i32) as i64) as u64).wrapping_mul((unsafe { (*p).nHash }) as u64);
    if nNew < (((256 as i32) as i64) as u64) {
        nNew = ((256 as i32) as i64) as u64;
    }
    0 as i32; // nNew is always a power of two
    0 as i32;
    if (unsafe { (*p).nHash }) != (0 as u32) {
        unsafe { sqlite3BeginBenignMalloc() };
    }
    apNew = (unsafe { sqlite3MallocZero((8 as u64).wrapping_mul(nNew)) }) as *mut *mut PgHdr1;
    if (unsafe { (*p).nHash }) != (0 as u32) {
        unsafe { sqlite3EndBenignMalloc() };
    }
    0 as i32;
    if apNew != std::ptr::null_mut::<*mut PgHdr1>() {
        i = (0 as i32) as u32;
        '__slate_break_231: while i < unsafe { (*p).nHash } {
            let mut pPage: *mut PgHdr1 = unsafe { std::mem::zeroed() };
            let mut pNext: *mut PgHdr1 =
                unsafe { *unsafe { unsafe { (*p).apHash }.offset(i as isize) } };
            '__slate_break_232: loop {
                let __v262: *mut PgHdr1 = pNext;
                pPage = __v262;
                if !(__v262 != std::ptr::null_mut::<PgHdr1>()) {
                    break;
                }
                let mut h: u32 = (((unsafe { (*pPage).iKey }) as u64)
                    & nNew.wrapping_sub(((1 as i32) as i64) as u64))
                    as u32;
                pNext = unsafe { (*pPage).pNext };
                unsafe {
                    (*pPage).pNext = unsafe { *unsafe { apNew.offset(h as isize) } };
                }
                unsafe {
                    *unsafe { apNew.offset(h as isize) } = pPage;
                }
            }
            let __v260: u32 = i;
            let __v261: u32 = __v260.wrapping_add((1 as i32) as u32);
            i = __v261;
        }
        unsafe { sqlite3_free((unsafe { (*p).apHash }) as *mut ()) };
        unsafe {
            (*p).apHash = apNew;
        }
        unsafe {
            (*p).nHash = nNew as u32;
        }
    }
}

/// This function is used internally to remove the page pPage from the
/// PGroup LRU list, if is part of it. If pPage is not part of the PGroup
/// LRU list, then this function is a no-op.
///
/// The PGroup mutex must be held when this function is called.
fn pcache1PinPage(mut pPage: *mut PgHdr1) -> *mut PgHdr1 {
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe {
        (*unsafe { (*pPage).pLruPrev }).pLruNext = unsafe { (*pPage).pLruNext };
    }
    unsafe {
        (*unsafe { (*pPage).pLruNext }).pLruPrev = unsafe { (*pPage).pLruPrev };
    }
    unsafe {
        (*pPage).pLruNext = std::ptr::null_mut::<PgHdr1>();
    }
    // pPage->pLruPrev = 0;
    // No need to clear pLruPrev as it is never accessed if pLruNext is 0
    0 as i32;
    0 as i32;
    let __v263: *mut PCache1 = unsafe { (*pPage).pCache };
    let __v264: u32 = unsafe { (*__v263).nRecyclable };
    let __v265: u32 = __v264.wrapping_sub((1 as i32) as u32);
    unsafe {
        (*__v263).nRecyclable = __v265;
    }
    return pPage;
}

/// Remove the page supplied as an argument from the hash table
/// (PCache1.apHash structure) that it is currently stored in.
/// Also free the page if freePage is true.
///
/// The PGroup mutex must be held when this function is called.
fn pcache1RemoveFromHash(mut pPage: *mut PgHdr1, mut freeFlag: i32) {
    let mut h: u32 = 0 as u32;
    let mut pCache: *mut PCache1 = unsafe { (*pPage).pCache };
    let mut pp: *mut *mut PgHdr1 = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    h = (unsafe { (*pPage).iKey }) & unsafe { (*pCache).nHash }.wrapping_sub((1 as i32) as u32);
    pp = unsafe { unsafe { (*pCache).apHash }.offset(h as isize) };
    '__slate_break_233: while (unsafe { *pp }) != pPage {
        {}
        pp = unsafe { std::ptr::addr_of_mut!((*unsafe { *pp }).pNext) };
    }
    unsafe {
        *pp = unsafe { (*unsafe { *pp }).pNext };
    }
    let __v266: *mut PCache1 = pCache;
    let __v267: u32 = unsafe { (*__v266).nPage };
    let __v268: u32 = __v267.wrapping_sub((1 as i32) as u32);
    unsafe {
        (*__v266).nPage = __v268;
    }
    if freeFlag != (0 as i32) {
        pcache1FreePage(pPage);
    }
}

/// If there are currently more than nMaxPage pages allocated, try
/// to recycle pages to reduce the number allocated to nMaxPage.
fn pcache1EnforceMaxPage(mut pCache: *mut PCache1) {
    let mut pGroup: *mut PGroup = unsafe { (*pCache).pGroup };
    let mut p: *mut PgHdr1 = unsafe { std::mem::zeroed() };
    0 as i32;
    '__slate_break_234: loop {
        let __v269: bool;
        if (unsafe { (*pGroup).nPurgeable }) > unsafe { (*pGroup).nMaxPage } {
            let __v270: *mut PgHdr1 = unsafe { (*pGroup).lru.pLruPrev };
            p = __v270;
            __v269 = (((unsafe { (*__v270).isAnchor }) as u32) as i32) == (0 as i32);
        } else {
            __v269 = false as bool;
        }
        if !__v269 {
            break;
        }
        0 as i32;
        0 as i32;
        pcache1PinPage(p);
        pcache1RemoveFromHash(p, 1 as i32);
    }
    if (unsafe { (*pCache).nPage }) == ((0 as i32) as u32)
        && (unsafe { (*pCache).pBulk }) != std::ptr::null_mut::<()>()
    {
        unsafe { sqlite3_free(unsafe { (*pCache).pBulk }) };
        unsafe {
            (*pCache).pFree = std::ptr::null_mut::<PgHdr1>();
        }
        unsafe {
            (*pCache).pBulk = std::ptr::null_mut::<PgHdr1>() as *mut ();
        }
    }
}

/// Discard all pages from cache pCache with a page number (key value)
/// greater than or equal to iLimit. Any pinned pages that meet this
/// criteria are unpinned before they are discarded.
///
/// The PCache mutex must be held when this function is called.
///
/// # Arguments
///
/// * `pCache` - The cache to truncate
/// * `iLimit` - Drop pages with this pgno or larger
fn pcache1TruncateUnsafe(mut pCache: *mut PCache1, mut iLimit: u32) {
    // To assert pCache->nPage is correct
    let mut h: u32 = 0 as u32;
    let mut iStop: u32 = 0 as u32;
    0 as i32;
    0 as i32;
    0 as i32;
    if unsafe { (*pCache).iMaxKey }.wrapping_sub(iLimit) < unsafe { (*pCache).nHash } {
        // If we are just shaving the last few pages off the end of the
        // cache, then there is no point in scanning the entire hash table.
        // Only scan those hash slots that might contain pages that need to
        // be removed.
        h = iLimit & unsafe { (*pCache).nHash }.wrapping_sub((1 as i32) as u32);
        iStop = (unsafe { (*pCache).iMaxKey })
            & unsafe { (*pCache).nHash }.wrapping_sub((1 as i32) as u32); // Disable the pCache->nPage validity check
    } else {
        // This is the general case where many pages are being removed.
        // It is necessary to scan the entire hash table
        h = (unsafe { (*pCache).nHash }) / ((2 as i32) as u32);
        iStop = h.wrapping_sub((1 as i32) as u32);
    }
    '__slate_break_235: loop {
        let mut pp: *mut *mut PgHdr1 = unsafe { std::mem::zeroed() };
        let mut pPage: *mut PgHdr1 = unsafe { std::mem::zeroed() };
        0 as i32;
        pp = unsafe { unsafe { (*pCache).apHash }.offset(h as isize) };
        '__slate_break_236: loop {
            let __v271: *mut PgHdr1 = unsafe { *pp };
            pPage = __v271;
            if !(__v271 != std::ptr::null_mut::<PgHdr1>()) {
                break;
            }
            if (unsafe { (*pPage).iKey }) >= iLimit {
                let __v272: *mut PCache1 = pCache;
                let __v273: u32 = unsafe { (*__v272).nPage };
                let __v274: u32 = __v273.wrapping_sub((1 as i32) as u32);
                unsafe {
                    (*__v272).nPage = __v274;
                }
                unsafe {
                    *pp = unsafe { (*pPage).pNext };
                }
                if (unsafe { (*pPage).pLruNext }) != std::ptr::null_mut::<PgHdr1>() {
                    pcache1PinPage(pPage);
                }
                pcache1FreePage(pPage);
            } else {
                pp = unsafe { std::ptr::addr_of_mut!((*pPage).pNext) };
            }
        }
        if h == iStop {
            break '__slate_break_235;
        }
        h = h.wrapping_add((1 as i32) as u32)
            & unsafe { (*pCache).nHash }.wrapping_sub((1 as i32) as u32);
    }
    0 as i32;
}

// sqlite3_pcache Methods
/// Implementation of the sqlite3_pcache.xInit method.
#[unsafe(link_section = ".text.slate_distinct.pcache1.pcache1Init")]
extern "C-unwind" fn pcache1Init(mut NotUsed: *mut ()) -> i32 {
    NotUsed;
    0 as i32;
    unsafe {
        memset(
            (unsafe { std::ptr::addr_of_mut!(pcache1_g) }) as *mut (),
            0 as i32,
            144 as u64,
        )
    };
    // The pcache1.separateCache variable is true if each PCache has its own
    // private PGroup (mode-1).  pcache1.separateCache is false if the single
    // PGroup in pcache1.grp is used for all page caches (mode-2).
    //
    //   *  Always use a unified cache (mode-2) if ENABLE_MEMORY_MANAGEMENT
    //
    //   *  Use a unified cache in single-threaded applications that have
    //      configured a start-time buffer for use as page-cache memory using
    //      sqlite3_config(SQLITE_CONFIG_PAGECACHE, pBuf, sz, N) with non-NULL
    //      pBuf argument.
    //
    //   *  Otherwise use separate caches (mode-1)
    unsafe {
        pcache1_g.separateCache = ((unsafe { sqlite3Config.pPage }) == std::ptr::null_mut::<()>()
            || (((unsafe { sqlite3Config.bCoreMutex }) as u32) as i32) > (0 as i32))
            as i32;
    }
    if (unsafe { sqlite3Config.bCoreMutex }) != (0 as u8) {
        unsafe {
            pcache1_g.grp.mutex = unsafe { sqlite3MutexAlloc(6 as i32) };
        }
        unsafe {
            pcache1_g.mutex = unsafe { sqlite3MutexAlloc(7 as i32) };
        }
    }
    if (unsafe { pcache1_g.separateCache }) != (0 as i32)
        && (unsafe { sqlite3Config.nPage }) != (0 as i32)
        && (unsafe { sqlite3Config.pPage }) == std::ptr::null_mut::<()>()
    {
        unsafe {
            pcache1_g.nInitPage = unsafe { sqlite3Config.nPage };
        }
    } else {
        unsafe {
            pcache1_g.nInitPage = 0 as i32;
        }
    }
    unsafe {
        pcache1_g.grp.mxPinned = (10 as i32) as u32;
    }
    unsafe {
        pcache1_g.isInit = 1 as i32;
    }
    return 0 as i32;
}

/// Implementation of the sqlite3_pcache.xShutdown method.
/// Note that the static mutex allocated in xInit does
/// not need to be freed.
#[unsafe(link_section = ".text.slate_distinct.pcache1.pcache1Shutdown")]
extern "C-unwind" fn pcache1Shutdown(mut NotUsed: *mut ()) {
    NotUsed;
    0 as i32;
    unsafe {
        memset(
            (unsafe { std::ptr::addr_of_mut!(pcache1_g) }) as *mut (),
            0 as i32,
            144 as u64,
        )
    };
}

/// Implementation of the sqlite3_pcache.xCreate method.
///
/// Allocate a new cache.
#[unsafe(link_section = ".text.slate_distinct.pcache1.pcache1Create")]
extern "C-unwind" fn pcache1Create(
    mut szPage: i32,
    mut szExtra: i32,
    mut bPurgeable: i32,
) -> *mut sqlite3_pcache {
    let mut pCache: *mut PCache1 = unsafe { std::mem::zeroed() }; // The newly created page cache
    let mut pGroup: *mut PGroup = unsafe { std::mem::zeroed() }; // The group the new page cache will belong to
    let mut sz: i64 = 0 as i64; // Bytes of memory required to allocate the new cache
    0 as i32;
    0 as i32;
    sz = (88 as u64).wrapping_add(
        (80 as u64).wrapping_mul(((unsafe { pcache1_g.separateCache }) as i64) as u64),
    ) as i64;
    pCache = (unsafe { sqlite3MallocZero(sz as u64) }) as *mut PCache1;
    if pCache != std::ptr::null_mut::<PCache1>() {
        if (unsafe { pcache1_g.separateCache }) != (0 as i32) {
            pGroup = (unsafe { pCache.offset((1 as i32) as isize) }) as *mut PGroup;
            unsafe {
                (*pGroup).mxPinned = (10 as i32) as u32;
            }
        } else {
            pGroup = unsafe { std::ptr::addr_of_mut!(pcache1_g.grp) };
        }
        0 as i32;
        if (((unsafe { (*pGroup).lru.isAnchor }) as u32) as i32) == (0 as i32) {
            unsafe {
                (*pGroup).lru.isAnchor = ((1 as i32) as i16) as u16;
            }
            let __v281: *mut PgHdr1 = unsafe { std::ptr::addr_of_mut!((*pGroup).lru) };
            unsafe {
                (*pGroup).lru.pLruNext = __v281;
            }
            unsafe {
                (*pGroup).lru.pLruPrev = __v281;
            }
        }
        unsafe {
            (*pCache).pGroup = pGroup;
        }
        unsafe {
            (*pCache).szPage = szPage;
        }
        unsafe {
            (*pCache).szExtra = szExtra;
        }
        unsafe {
            (*pCache).szAlloc = ((((szPage + szExtra) as i64) as u64).wrapping_add(
                (56 as u64).wrapping_add(((7 as i32) as i64) as u64)
                    & ((!(7 as i32) as i64) as u64),
            ) as u32) as i32;
        }
        unsafe {
            (*pCache).bPurgeable = if bPurgeable != (0 as i32) {
                1 as i32
            } else {
                0 as i32
            };
        }
        pcache1ResizeHash(pCache);
        if bPurgeable != (0 as i32) {
            unsafe {
                (*pCache).nMin = (10 as i32) as u32;
            }
            let __v282: *mut PGroup = pGroup;
            let __v283: u32 = unsafe { (*__v282).nMinPage };
            let __v284: u32 = __v283.wrapping_add(unsafe { (*pCache).nMin });
            unsafe {
                (*__v282).nMinPage = __v284;
            }
            unsafe {
                (*pGroup).mxPinned = unsafe { (*pGroup).nMaxPage }
                    .wrapping_add((10 as i32) as u32)
                    .wrapping_sub(unsafe { (*pGroup).nMinPage });
            }
            unsafe {
                (*pCache).pnPurgeable = unsafe { std::ptr::addr_of_mut!((*pGroup).nPurgeable) };
            }
        } else {
            unsafe {
                (*pCache).pnPurgeable =
                    unsafe { std::ptr::addr_of_mut!((*pCache).nPurgeableDummy) };
            }
        }
        0 as i32;
        if (unsafe { (*pCache).nHash }) == ((0 as i32) as u32) {
            pcache1Destroy(pCache as *mut sqlite3_pcache);
            pCache = std::ptr::null_mut::<PCache1>();
        }
    }
    return pCache as *mut sqlite3_pcache;
}

/// Implementation of the sqlite3_pcache.xCachesize method.
///
/// Configure the cache_size limit for a cache.
#[unsafe(link_section = ".text.slate_distinct.pcache1.pcache1Cachesize")]
extern "C-unwind" fn pcache1Cachesize(mut p: *mut sqlite3_pcache, mut nMax: i32) {
    let mut pCache: *mut PCache1 = p as *mut PCache1;
    let mut n: u32 = 0 as u32;
    0 as i32;
    if (unsafe { (*pCache).bPurgeable }) != (0 as i32) {
        let mut pGroup: *mut PGroup = unsafe { (*pCache).pGroup };
        0 as i32;
        n = nMax as u32;
        if n > ((2147418112 as i32) as u32)
            .wrapping_sub(unsafe { (*pGroup).nMaxPage })
            .wrapping_add(unsafe { (*pCache).nMax })
        {
            n = ((2147418112 as i32) as u32)
                .wrapping_sub(unsafe { (*pGroup).nMaxPage })
                .wrapping_add(unsafe { (*pCache).nMax });
        }
        let __v285: *mut PGroup = pGroup;
        let __v286: u32 = unsafe { (*__v285).nMaxPage };
        let __v287: u32 = __v286.wrapping_add(n.wrapping_sub(unsafe { (*pCache).nMax }));
        unsafe {
            (*__v285).nMaxPage = __v287;
        }
        unsafe {
            (*pGroup).mxPinned = unsafe { (*pGroup).nMaxPage }
                .wrapping_add((10 as i32) as u32)
                .wrapping_sub(unsafe { (*pGroup).nMinPage });
        }
        unsafe {
            (*pCache).nMax = n;
        }
        unsafe {
            (*pCache).n90pct =
                unsafe { (*pCache).nMax }.wrapping_mul((9 as i32) as u32) / ((10 as i32) as u32);
        }
        pcache1EnforceMaxPage(pCache);
        0 as i32;
    }
}

/// Implementation of the sqlite3_pcache.xShrink method.
///
/// Free up as much memory as possible.
#[unsafe(link_section = ".text.slate_distinct.pcache1.pcache1Shrink")]
extern "C-unwind" fn pcache1Shrink(mut p: *mut sqlite3_pcache) {
    let mut pCache: *mut PCache1 = p as *mut PCache1;
    if (unsafe { (*pCache).bPurgeable }) != (0 as i32) {
        let mut pGroup: *mut PGroup = unsafe { (*pCache).pGroup };
        let mut savedMaxPage: u32 = 0 as u32;
        0 as i32;
        savedMaxPage = unsafe { (*pGroup).nMaxPage };
        unsafe {
            (*pGroup).nMaxPage = (0 as i32) as u32;
        }
        pcache1EnforceMaxPage(pCache);
        unsafe {
            (*pGroup).nMaxPage = savedMaxPage;
        }
        0 as i32;
    }
}

/// Implementation of the sqlite3_pcache.xPagecount method.
#[unsafe(link_section = ".text.slate_distinct.pcache1.pcache1Pagecount")]
extern "C-unwind" fn pcache1Pagecount(mut p: *mut sqlite3_pcache) -> i32 {
    let mut n: i32 = 0 as i32;
    let mut pCache: *mut PCache1 = p as *mut PCache1;
    0 as i32;
    n = (unsafe { (*pCache).nPage }) as i32;
    0 as i32;
    return n;
}

/// Implement steps 3, 4, and 5 of the pcache1Fetch() algorithm described
/// in the header of the pcache1Fetch() procedure.
///
/// This steps are broken out into a separate procedure because they are
/// usually not needed, and by avoiding the stack initialization required
/// for these steps, the main pcache1Fetch() procedure can run faster.
fn pcache1FetchStage2(mut pCache: *mut PCache1, mut iKey: u32, mut createFlag: i32) -> *mut PgHdr1 {
    let mut nPinned: u32 = 0 as u32;
    let mut pGroup: *mut PGroup = unsafe { (*pCache).pGroup };
    let mut pPage: *mut PgHdr1 = std::ptr::null_mut::<PgHdr1>();
    // Step 3: Abort if createFlag is 1 but the cache is nearly full
    0 as i32;
    nPinned = unsafe { (*pCache).nPage }.wrapping_sub(unsafe { (*pCache).nRecyclable });
    0 as i32;
    0 as i32;
    let __v288: bool;
    if createFlag == (1 as i32) {
        let __v289: bool;
        if nPinned >= unsafe { (*pGroup).mxPinned } || nPinned >= unsafe { (*pCache).n90pct } {
            __v289 = true as bool;
        } else {
            __v289 = pcache1UnderMemoryPressure(pCache) != (0 as i32)
                && (unsafe { (*pCache).nRecyclable }) < nPinned;
        }
        __v288 = __v289;
    } else {
        __v288 = false as bool;
    }
    if __v288 {
        return std::ptr::null_mut::<PgHdr1>();
    }
    if (unsafe { (*pCache).nPage }) >= unsafe { (*pCache).nHash } {
        pcache1ResizeHash(pCache);
    }
    0 as i32;
    0 as i32;
    // Step 4. Try to recycle a page.
    let __v290: bool;
    if (unsafe { (*pCache).bPurgeable }) != (0 as i32)
        && !((unsafe { (*unsafe { (*pGroup).lru.pLruPrev }).isAnchor }) != (0 as u16))
    {
        let __v291: bool;
        if unsafe { (*pCache).nPage }.wrapping_add((1 as i32) as u32) >= unsafe { (*pCache).nMax } {
            __v291 = true as bool;
        } else {
            __v291 = pcache1UnderMemoryPressure(pCache) != (0 as i32);
        }
        __v290 = __v291;
    } else {
        __v290 = false as bool;
    }
    if __v290 {
        let mut pOther: *mut PCache1 = unsafe { std::mem::zeroed() };
        pPage = unsafe { (*pGroup).lru.pLruPrev };
        0 as i32;
        pcache1RemoveFromHash(pPage, 0 as i32);
        pcache1PinPage(pPage);
        pOther = unsafe { (*pPage).pCache };
        if (unsafe { (*pOther).szAlloc }) != unsafe { (*pCache).szAlloc } {
            pcache1FreePage(pPage);
            pPage = std::ptr::null_mut::<PgHdr1>();
        } else {
            let __v292: *mut PGroup = pGroup;
            let __v293: u32 = unsafe { (*__v292).nPurgeable };
            let __v294: u32 = __v293.wrapping_sub(
                ((unsafe { (*pOther).bPurgeable }) - unsafe { (*pCache).bPurgeable }) as u32,
            );
            unsafe {
                (*__v292).nPurgeable = __v294;
            }
        }
    }
    // Step 5. If a usable page buffer has still not been found,
    // attempt to allocate a new one.
    if !(pPage != std::ptr::null_mut::<PgHdr1>()) {
        pPage = pcache1AllocPage(pCache, (createFlag == (1 as i32)) as i32);
    }
    if pPage != std::ptr::null_mut::<PgHdr1>() {
        let mut h: u32 = iKey & unsafe { (*pCache).nHash }.wrapping_sub((1 as i32) as u32);
        let __v295: *mut PCache1 = pCache;
        let __v296: u32 = unsafe { (*__v295).nPage };
        let __v297: u32 = __v296.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v295).nPage = __v297;
        }
        unsafe {
            (*pPage).iKey = iKey;
        }
        unsafe {
            (*pPage).pNext = unsafe { *unsafe { unsafe { (*pCache).apHash }.offset(h as isize) } };
        }
        unsafe {
            (*pPage).pCache = pCache;
        }
        unsafe {
            (*pPage).pLruNext = std::ptr::null_mut::<PgHdr1>();
        }
        // pPage->pLruPrev = 0;
        // No need to clear pLruPrev since it is not accessed when pLruNext==0
        unsafe {
            *((unsafe { (*pPage).page.pExtra }) as *mut *mut ()) = std::ptr::null_mut::<()>();
        }
        unsafe {
            *unsafe { unsafe { (*pCache).apHash }.offset(h as isize) } = pPage;
        }
        if iKey > unsafe { (*pCache).iMaxKey } {
            unsafe {
                (*pCache).iMaxKey = iKey;
            }
        }
    }
    return pPage;
}

/// Implementation of the sqlite3_pcache.xFetch method.
///
/// Fetch a page by key value.
///
/// Whether or not a new page may be allocated by this function depends on
/// the value of the createFlag argument.  0 means do not allocate a new
/// page.  1 means allocate a new page if space is easily available.  2
/// means to try really hard to allocate a new page.
///
/// For a non-purgeable cache (a cache used as the storage for an in-memory
/// database) there is really no difference between createFlag 1 and 2.  So
/// the calling function (pcache.c) will never have a createFlag of 1 on
/// a non-purgeable cache.
///
/// There are three different approaches to obtaining space for a page,
/// depending on the value of parameter createFlag (which may be 0, 1 or 2).
///
///   1. Regardless of the value of createFlag, the cache is searched for a
///      copy of the requested page. If one is found, it is returned.
///
///   2. If createFlag==0 and the page is not already in the cache, NULL is
///      returned.
///
///   3. If createFlag is 1, and the page is not already in the cache, then
///      return NULL (do not allocate a new page) if any of the following
///      conditions are true:
///
///       (a) the number of pages pinned by the cache is greater than
///           PCache1.nMax, or
///
///       (b) the number of pages pinned by the cache is greater than
///           the sum of nMax for all purgeable caches, less the sum of
///           nMin for all other purgeable caches, or
///
///   4. If none of the first three conditions apply and the cache is marked
///      as purgeable, and if one of the following is true:
///
///       (a) The number of pages allocated for the cache is already
///           PCache1.nMax, or
///
///       (b) The number of pages allocated for all purgeable caches is
///           already equal to or greater than the sum of nMax for all
///           purgeable caches,
///
///       (c) The system is under memory pressure and wants to avoid
///           unnecessary pages cache entry allocations
///
///      then attempt to recycle a page from the LRU list. If it is the right
///      size, return the recycled buffer. Otherwise, free the buffer and
///      proceed to step 5.
///
///   5. Otherwise, allocate and return a new page buffer.
///
/// There are two versions of this routine.  pcache1FetchWithMutex() is
/// the general case.  pcache1FetchNoMutex() is a faster implementation for
/// the common case where pGroup->mutex is NULL.  The pcache1Fetch() wrapper
/// invokes the appropriate routine.
fn pcache1FetchNoMutex(
    mut p: *mut sqlite3_pcache,
    mut iKey: u32,
    mut createFlag: i32,
) -> *mut PgHdr1 {
    let mut pCache: *mut PCache1 = p as *mut PCache1;
    let mut pPage: *mut PgHdr1 = std::ptr::null_mut::<PgHdr1>();
    // Step 1: Search the hash table for an existing entry.  nHash is always
    // a power of two when the cache is in use (see pcache1ResizeHash()), so
    // the modulo reduces to a mask, avoiding a hardware divide.
    0 as i32;
    pPage = unsafe {
        *unsafe {
            unsafe { (*pCache).apHash }
                .offset((iKey & unsafe { (*pCache).nHash }.wrapping_sub(1 as u32)) as isize)
        }
    };
    '__slate_break_238: while pPage != std::ptr::null_mut::<PgHdr1>()
        && (unsafe { (*pPage).iKey }) != iKey
    {
        pPage = unsafe { (*pPage).pNext };
    }
    // Step 2: If the page was found in the hash table, then return it.
    // If the page was not in the hash table and createFlag is 0, abort.
    // Otherwise (page not in hash and createFlag!=0) continue with
    // subsequent steps to try to create the page.
    if pPage != std::ptr::null_mut::<PgHdr1>() {
        if (unsafe { (*pPage).pLruNext }) != std::ptr::null_mut::<PgHdr1>() {
            return pcache1PinPage(pPage);
        } else {
            return pPage;
        }
    } else {
        if createFlag != (0 as i32) {
            // Steps 3, 4, and 5 implemented by this subroutine
            return pcache1FetchStage2(pCache, iKey, createFlag);
        } else {
            return std::ptr::null_mut::<PgHdr1>();
        }
    }
    return unsafe { std::mem::zeroed() };
}

#[unsafe(link_section = ".text.slate_distinct.pcache1.pcache1Fetch")]
extern "C-unwind" fn pcache1Fetch(
    mut p: *mut sqlite3_pcache,
    mut iKey: u32,
    mut createFlag: i32,
) -> *mut sqlite3_pcache_page {
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    return pcache1FetchNoMutex(p, iKey, createFlag) as *mut sqlite3_pcache_page;
    return unsafe { std::mem::zeroed() };
}

/// Implementation of the sqlite3_pcache.xUnpin method.
///
/// Mark a page as unpinned (eligible for asynchronous recycling).
#[unsafe(link_section = ".text.slate_distinct.pcache1.pcache1Unpin")]
extern "C-unwind" fn pcache1Unpin(
    mut p: *mut sqlite3_pcache,
    mut pPg: *mut sqlite3_pcache_page,
    mut reuseUnlikely: i32,
) {
    let mut pCache: *mut PCache1 = p as *mut PCache1;
    let mut pPage: *mut PgHdr1 = pPg as *mut PgHdr1;
    let mut pGroup: *mut PGroup = unsafe { (*pCache).pGroup };
    0 as i32;
    0 as i32;
    // It is an error to call this function if the page is already
    // part of the PGroup LRU list.
    0 as i32;
    0 as i32;
    if reuseUnlikely != (0 as i32)
        || (unsafe { (*pGroup).nPurgeable }) > unsafe { (*pGroup).nMaxPage }
    {
        pcache1RemoveFromHash(pPage, 1 as i32);
    } else {
        // Add the page to the PGroup LRU list.
        let mut ppFirst: *mut *mut PgHdr1 =
            unsafe { std::ptr::addr_of_mut!((*pGroup).lru.pLruNext) };
        unsafe {
            (*pPage).pLruPrev = unsafe { std::ptr::addr_of_mut!((*pGroup).lru) };
        }
        let __v298: *mut PgHdr1 = unsafe { *ppFirst };
        unsafe {
            (*pPage).pLruNext = __v298;
        }
        unsafe {
            (*__v298).pLruPrev = pPage;
        }
        unsafe {
            *ppFirst = pPage;
        }
        let __v299: *mut PCache1 = pCache;
        let __v300: u32 = unsafe { (*__v299).nRecyclable };
        let __v301: u32 = __v300.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v299).nRecyclable = __v301;
        }
    }
    0 as i32;
}

/// Implementation of the sqlite3_pcache.xRekey method.
#[unsafe(link_section = ".text.slate_distinct.pcache1.pcache1Rekey")]
extern "C-unwind" fn pcache1Rekey(
    mut p: *mut sqlite3_pcache,
    mut pPg: *mut sqlite3_pcache_page,
    mut iOld: u32,
    mut iNew: u32,
) {
    let mut pCache: *mut PCache1 = p as *mut PCache1;
    let mut pPage: *mut PgHdr1 = pPg as *mut PgHdr1;
    let mut pp: *mut *mut PgHdr1 = unsafe { std::mem::zeroed() };
    let mut hOld: u32 = 0 as u32;
    let mut hNew: u32 = 0 as u32;
    0 as i32;
    0 as i32;
    0 as i32; // The page number really is changing
    0 as i32;
    0 as i32; // pPg really is iOld
    0 as i32;
    hOld = iOld & unsafe { (*pCache).nHash }.wrapping_sub((1 as i32) as u32);
    pp = unsafe { unsafe { (*pCache).apHash }.offset(hOld as isize) };
    '__slate_break_239: while (unsafe { *pp }) != pPage {
        pp = unsafe { std::ptr::addr_of_mut!((*unsafe { *pp }).pNext) };
    }
    unsafe {
        *pp = unsafe { (*pPage).pNext };
    }
    0 as i32; // iNew not in cache
    hNew = iNew & unsafe { (*pCache).nHash }.wrapping_sub((1 as i32) as u32);
    unsafe {
        (*pPage).iKey = iNew;
    }
    unsafe {
        (*pPage).pNext = unsafe { *unsafe { unsafe { (*pCache).apHash }.offset(hNew as isize) } };
    }
    unsafe {
        *unsafe { unsafe { (*pCache).apHash }.offset(hNew as isize) } = pPage;
    }
    if iNew > unsafe { (*pCache).iMaxKey } {
        unsafe {
            (*pCache).iMaxKey = iNew;
        }
    }
    0 as i32;
}

/// Implementation of the sqlite3_pcache.xTruncate method.
///
/// Discard all unpinned pages in the cache with a page number equal to
/// or greater than parameter iLimit. Any pinned pages with a page number
/// equal to or greater than iLimit are implicitly unpinned.
#[unsafe(link_section = ".text.slate_distinct.pcache1.pcache1Truncate")]
extern "C-unwind" fn pcache1Truncate(mut p: *mut sqlite3_pcache, mut iLimit: u32) {
    let mut pCache: *mut PCache1 = p as *mut PCache1;
    0 as i32;
    if iLimit <= unsafe { (*pCache).iMaxKey } {
        pcache1TruncateUnsafe(pCache, iLimit);
        unsafe {
            (*pCache).iMaxKey = iLimit.wrapping_sub((1 as i32) as u32);
        }
    }
    0 as i32;
}

/// Implementation of the sqlite3_pcache.xDestroy method.
///
/// Destroy a cache allocated using pcache1Create().
#[unsafe(link_section = ".text.slate_distinct.pcache1.pcache1Destroy")]
extern "C-unwind" fn pcache1Destroy(mut p: *mut sqlite3_pcache) {
    let mut pCache: *mut PCache1 = p as *mut PCache1;
    let mut pGroup: *mut PGroup = unsafe { (*pCache).pGroup };
    0 as i32;
    0 as i32;
    if (unsafe { (*pCache).nPage }) != (0 as u32) {
        pcache1TruncateUnsafe(pCache, (0 as i32) as u32);
    }
    0 as i32;
    let __v275: *mut PGroup = pGroup;
    let __v276: u32 = unsafe { (*__v275).nMaxPage };
    let __v277: u32 = __v276.wrapping_sub(unsafe { (*pCache).nMax });
    unsafe {
        (*__v275).nMaxPage = __v277;
    }
    0 as i32;
    let __v278: *mut PGroup = pGroup;
    let __v279: u32 = unsafe { (*__v278).nMinPage };
    let __v280: u32 = __v279.wrapping_sub(unsafe { (*pCache).nMin });
    unsafe {
        (*__v278).nMinPage = __v280;
    }
    unsafe {
        (*pGroup).mxPinned = unsafe { (*pGroup).nMaxPage }
            .wrapping_add((10 as i32) as u32)
            .wrapping_sub(unsafe { (*pGroup).nMinPage });
    }
    pcache1EnforceMaxPage(pCache);
    0 as i32;
    unsafe { sqlite3_free(unsafe { (*pCache).pBulk }) };
    unsafe { sqlite3_free((unsafe { (*pCache).apHash }) as *mut ()) };
    unsafe { sqlite3_free(pCache as *mut ()) };
}

/// This function is called during initialization (sqlite3_initialize()) to
/// install the default pluggable cache module, assuming the user has not
/// already provided an alternative.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PCacheSetDefault() {
    // iVersion
    // pArg
    // xInit
    // xShutdown
    // xCreate
    // xCachesize
    // xPagecount
    // xFetch
    // xUnpin
    // xRekey
    // xTruncate
    // xDestroy
    // xShrink
    unsafe { sqlite3_config(18 as i32, unsafe { std::ptr::addr_of!(defaultMethods) }) };
}

static mut defaultMethods: sqlite3_pcache_methods2 = sqlite3_pcache_methods2 {
    iVersion: 1 as i32,
    pArg: std::ptr::null_mut::<()>(),
    xInit: Some(pcache1Init),
    xShutdown: Some(pcache1Shutdown),
    xCreate: Some(pcache1Create),
    xCachesize: Some(pcache1Cachesize),
    xPagecount: Some(pcache1Pagecount),
    xFetch: Some(pcache1Fetch),
    xUnpin: Some(pcache1Unpin),
    xRekey: Some(pcache1Rekey),
    xTruncate: Some(pcache1Truncate),
    xDestroy: Some(pcache1Destroy),
    xShrink: Some(pcache1Shrink),
};

/// Return the size of the header on each page of this PCACHE implementation.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3HeaderSizePcache1() -> i32 {
    return (((56 as u64).wrapping_add(((7 as i32) as i64) as u64) & ((!(7 as i32) as i64) as u64))
        as u32) as i32;
}

/// Return the global mutex used by this PCACHE implementation.  The
/// sqlite3_status() routine needs access to this mutex.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Pcache1Mutex() -> *mut sqlite3_mutex {
    return unsafe { pcache1_g.mutex };
}
