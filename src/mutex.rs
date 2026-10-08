//! 2007 August 14
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This file contains the C functions that implement mutexes.
//!
//! This file contains code that is common across all mutex implementations.
unsafe extern "C" {
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_initialize() -> i32;
    fn sqlite3DefaultMutex() -> *const sqlite3_mutex_methods;
    fn sqlite3NoopMutex() -> *const sqlite3_mutex_methods;
    fn sqlite3MemoryBarrier();
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

/// Initialize the mutex system.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MutexInit() -> i32 {
    let mut rc: i32 = 0 as i32;
    if !((unsafe { sqlite3Config.mutex.xMutexAlloc }) != None) {
        // If the xMutexAlloc method has not been set, then the user did not
        // install a mutex implementation via sqlite3_config() prior to
        // sqlite3_initialize() being called. This block copies pointers to
        // the default implementation into the sqlite3GlobalConfig structure.
        let mut pFrom: *const sqlite3_mutex_methods = unsafe { std::mem::zeroed() };
        let mut pTo: *mut sqlite3_mutex_methods =
            unsafe { std::ptr::addr_of_mut!(sqlite3Config.mutex) };
        if (unsafe { sqlite3Config.bCoreMutex }) != (0 as u8) {
            pFrom = unsafe { sqlite3DefaultMutex() };
        } else {
            pFrom = unsafe { sqlite3NoopMutex() };
        }
        unsafe {
            (*pTo).xMutexInit = unsafe { (*pFrom).xMutexInit };
        }
        unsafe {
            (*pTo).xMutexEnd = unsafe { (*pFrom).xMutexEnd };
        }
        unsafe {
            (*pTo).xMutexFree = unsafe { (*pFrom).xMutexFree };
        }
        unsafe {
            (*pTo).xMutexEnter = unsafe { (*pFrom).xMutexEnter };
        }
        unsafe {
            (*pTo).xMutexTry = unsafe { (*pFrom).xMutexTry };
        }
        unsafe {
            (*pTo).xMutexLeave = unsafe { (*pFrom).xMutexLeave };
        }
        unsafe {
            (*pTo).xMutexHeld = unsafe { (*pFrom).xMutexHeld };
        }
        unsafe {
            (*pTo).xMutexNotheld = unsafe { (*pFrom).xMutexNotheld };
        }
        unsafe { sqlite3MemoryBarrier() };
        unsafe {
            (*pTo).xMutexAlloc = unsafe { (*pFrom).xMutexAlloc };
        }
    }
    0 as i32;
    rc = unsafe { unsafe { sqlite3Config.mutex.xMutexInit }.unwrap()() };
    unsafe { sqlite3MemoryBarrier() };
    return rc;
}

/// Shutdown the mutex system. This call frees resources allocated by
/// sqlite3MutexInit().
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MutexEnd() -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { sqlite3Config.mutex.xMutexEnd }) != None {
        rc = unsafe { unsafe { sqlite3Config.mutex.xMutexEnd }.unwrap()() };
    }
    return rc;
}

/// Retrieve a pointer to a static mutex or allocate a new dynamic one.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.mutex.sqlite3_mutex_alloc")]
extern "C-unwind" fn sqlite3_mutex_alloc(mut id: i32) -> *mut sqlite3_mutex {
    let __v61: bool;
    if id <= (1 as i32) {
        __v61 = (unsafe { sqlite3_initialize() }) != (0 as i32);
    } else {
        __v61 = false as bool;
    }
    if __v61 {
        return std::ptr::null_mut::<sqlite3_mutex>();
    }
    let __v62: bool;
    if id > (1 as i32) {
        __v62 = sqlite3MutexInit() != (0 as i32);
    } else {
        __v62 = false as bool;
    }
    if __v62 {
        return std::ptr::null_mut::<sqlite3_mutex>();
    }
    0 as i32;
    return unsafe { unsafe { sqlite3Config.mutex.xMutexAlloc }.unwrap()(id) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MutexAlloc(mut id: i32) -> *mut sqlite3_mutex {
    if !((unsafe { sqlite3Config.bCoreMutex }) != (0 as u8)) {
        return std::ptr::null_mut::<sqlite3_mutex>();
    }
    0 as i32;
    0 as i32;
    return unsafe { unsafe { sqlite3Config.mutex.xMutexAlloc }.unwrap()(id) };
}

/// Free a dynamic mutex.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.mutex.sqlite3_mutex_free")]
extern "C-unwind" fn sqlite3_mutex_free(mut p: *mut sqlite3_mutex) {
    if p != std::ptr::null_mut::<sqlite3_mutex>() {
        0 as i32;
        unsafe { unsafe { sqlite3Config.mutex.xMutexFree }.unwrap()(p) };
    }
}

/// Obtain the mutex p. If some other thread already has the mutex, block
/// until it can be obtained.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.mutex.sqlite3_mutex_enter")]
extern "C-unwind" fn sqlite3_mutex_enter(mut p: *mut sqlite3_mutex) {
    if p != std::ptr::null_mut::<sqlite3_mutex>() {
        0 as i32;
        unsafe { unsafe { sqlite3Config.mutex.xMutexEnter }.unwrap()(p) };
    }
}

/// Obtain the mutex p. If successful, return SQLITE_OK. Otherwise, if another
/// thread holds the mutex and it cannot be obtained, return SQLITE_BUSY.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.mutex.sqlite3_mutex_try")]
extern "C-unwind" fn sqlite3_mutex_try(mut p: *mut sqlite3_mutex) -> i32 {
    let mut rc: i32 = 0 as i32;
    if p != std::ptr::null_mut::<sqlite3_mutex>() {
        0 as i32;
        return unsafe { unsafe { sqlite3Config.mutex.xMutexTry }.unwrap()(p) };
    }
    return rc;
}

/// The sqlite3_mutex_leave() routine exits a mutex that was previously
/// entered by the same thread.  The behavior is undefined if the mutex
/// is not currently entered. If a NULL pointer is passed as an argument
/// this function is a no-op.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.mutex.sqlite3_mutex_leave")]
extern "C-unwind" fn sqlite3_mutex_leave(mut p: *mut sqlite3_mutex) {
    if p != std::ptr::null_mut::<sqlite3_mutex>() {
        0 as i32;
        unsafe { unsafe { sqlite3Config.mutex.xMutexLeave }.unwrap()(p) };
    }
}
