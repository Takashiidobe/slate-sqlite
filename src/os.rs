unsafe extern "C" {
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_initialize() -> i32;
    fn sqlite3_os_init() -> i32;
    fn sqlite3_malloc(__v270: i32) -> *mut ();
    fn sqlite3_free(__v271: *mut ());
    fn sqlite3_mutex_enter(__v276: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(__v277: *mut sqlite3_mutex);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn sqlite3MallocZero(__v374: u64) -> *mut ();
    fn sqlite3MutexAlloc(__v375: i32) -> *mut sqlite3_mutex;
    fn sqlite3RealToI64(__v376: f64) -> i64;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_file {
    pMethods: *const sqlite3_io_methods,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_io_methods {
    iVersion: i32,
    xClose: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file) -> i32>,
    xRead: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, *mut (), i32, i64) -> i32>,
    xWrite: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, *const (), i32, i64) -> i32>,
    xTruncate: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i64) -> i32>,
    xSync: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i32) -> i32>,
    xFileSize: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, *mut i64) -> i32>,
    xLock: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i32) -> i32>,
    xUnlock: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i32) -> i32>,
    xCheckReservedLock: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, *mut i32) -> i32>,
    xFileControl: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i32, *mut ()) -> i32>,
    xSectorSize: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file) -> i32>,
    xDeviceCharacteristics: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file) -> i32>,
    xShmMap:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i32, i32, i32, *mut *mut ()) -> i32>,
    xShmLock: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i32, i32, i32) -> i32>,
    xShmBarrier: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file)>,
    xShmUnmap: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i32) -> i32>,
    xFetch: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i64, i32, *mut *mut ()) -> i32>,
    xUnfetch: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i64, *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_mutex {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_vfs {
    iVersion: i32,
    szOsFile: i32,
    mxPathname: i32,
    pNext: *mut sqlite3_vfs,
    zName: *const i8,
    pAppData: *mut (),
    xOpen: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vfs,
            *const i8,
            *mut sqlite3_file,
            i32,
            *mut i32,
        ) -> i32,
    >,
    xDelete: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *const i8, i32) -> i32>,
    xAccess: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *const i8, i32, *mut i32) -> i32>,
    xFullPathname:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *const i8, i32, *mut i8) -> i32>,
    xDlOpen: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *const i8) -> *mut ()>,
    xDlError: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, i32, *mut i8)>,
    xDlSym: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vfs,
            *mut (),
            *const i8,
        ) -> Option<unsafe extern "C-unwind" fn()>,
    >,
    xDlClose: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *mut ())>,
    xRandomness: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, i32, *mut i8) -> i32>,
    xSleep: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, i32) -> i32>,
    xCurrentTime: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *mut f64) -> i32>,
    xGetLastError: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, i32, *mut i8) -> i32>,
    xCurrentTimeInt64: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *mut i64) -> i32>,
    xSetSystemCall: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vfs,
            *const i8,
            Option<unsafe extern "C-unwind" fn()>,
        ) -> i32,
    >,
    xGetSystemCall: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vfs,
            *const i8,
        ) -> Option<unsafe extern "C-unwind" fn()>,
    >,
    xNextSystemCall: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *const i8) -> *const i8>,
}

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

// /*
// ** The list of all registered VFS implementations.
// */
static mut vfsList: *mut sqlite3_vfs = std::ptr::null_mut::<sqlite3_vfs>();

// /*
// ** Locate a VFS by name.  If no name is given, simply return the
// ** first VFS on the list.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.os.sqlite3_vfs_find")]
extern "C-unwind" fn sqlite3_vfs_find(mut zVfs: *const i8) -> *mut sqlite3_vfs {
    let mut pVfs: *mut sqlite3_vfs = std::ptr::null_mut::<sqlite3_vfs>();
    let mut mutex: *mut sqlite3_mutex = unsafe { std::mem::zeroed() };
    let mut rc: i32 = unsafe { sqlite3_initialize() };
    if rc != (0 as i32) {
        return std::ptr::null_mut::<sqlite3_vfs>();
    }
    mutex = unsafe { sqlite3MutexAlloc(2 as i32) };
    unsafe { sqlite3_mutex_enter(mutex) };
    pVfs = unsafe { vfsList };
    '__slate_break_377: while pVfs != std::ptr::null_mut::<sqlite3_vfs>() {
        if zVfs == std::ptr::null::<i8>() {
            break '__slate_break_377;
        }
        if (unsafe { strcmp(zVfs, unsafe { (*pVfs).zName }) }) == (0 as i32) {
            break '__slate_break_377;
        }
        pVfs = unsafe { (*pVfs).pNext };
    }
    unsafe { sqlite3_mutex_leave(mutex) };
    return pVfs;
}

// /*
// ** Register a VFS with the system.  It is harmless to register the same
// ** VFS multiple times.  The new VFS becomes the default if makeDflt is
// ** true.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.os.sqlite3_vfs_register")]
extern "C-unwind" fn sqlite3_vfs_register(mut pVfs: *mut sqlite3_vfs, mut makeDflt: i32) -> i32 {
    let mut mutex: *mut sqlite3_mutex = unsafe { std::mem::zeroed() };
    let mut rc: i32 = unsafe { sqlite3_initialize() };
    if rc != (0 as i32) {
        return rc;
    }
    mutex = unsafe { sqlite3MutexAlloc(2 as i32) };
    unsafe { sqlite3_mutex_enter(mutex) };
    vfsUnlink(pVfs);
    if makeDflt != (0 as i32) || (unsafe { vfsList }) == std::ptr::null_mut::<sqlite3_vfs>() {
        unsafe {
            (*pVfs).pNext = unsafe { vfsList };
        }
        unsafe {
            vfsList = pVfs;
        }
    } else {
        unsafe {
            (*pVfs).pNext = unsafe { (*unsafe { vfsList }).pNext };
        }
        unsafe {
            (*unsafe { vfsList }).pNext = pVfs;
        }
    }
    0 as i32;
    unsafe { sqlite3_mutex_leave(mutex) };
    return 0 as i32;
}

// /*
// ** Unregister a VFS so that it is no longer accessible.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.os.sqlite3_vfs_unregister")]
extern "C-unwind" fn sqlite3_vfs_unregister(mut pVfs: *mut sqlite3_vfs) -> i32 {
    let mut mutex: *mut sqlite3_mutex = unsafe { std::mem::zeroed() };
    let mut rc: i32 = unsafe { sqlite3_initialize() };
    if rc != (0 as i32) {
        return rc;
    }
    mutex = unsafe { sqlite3MutexAlloc(2 as i32) };
    unsafe { sqlite3_mutex_enter(mutex) };
    vfsUnlink(pVfs);
    unsafe { sqlite3_mutex_leave(mutex) };
    return 0 as i32;
}

// /*
// ** This function is a wrapper around the OS specific implementation of
// ** sqlite3_os_init(). The purpose of the wrapper is to provide the
// ** ability to simulate a malloc failure, so that the handling of an
// ** error in sqlite3_os_init() by the upper layers can be tested.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsInit() -> i32 {
    let mut p: *mut () = unsafe { sqlite3_malloc(10 as i32) };
    if p == std::ptr::null_mut::<()>() {
        return 7 as i32;
    }
    unsafe { sqlite3_free(p) };
    return unsafe { sqlite3_os_init() };
}

// /*
// ** 2005 November 29
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// ******************************************************************************
// **
// ** This file contains OS interface code that is common to all
// ** architectures.
// */
// /*
// ** If we compile with the SQLITE_TEST macro set, then the following block
// ** of code will give us the ability to simulate a disk I/O error.  This
// ** is used for testing the I/O recovery logic.
// */
// /*
// ** When testing, also keep a count of the number of open files.
// */
// /*
// ** The default SQLite sqlite3_vfs implementations do not allocate
// ** memory (actually, os_unix.c allocates a small amount of memory
// ** from within OsOpen()), but some third-party implementations may.
// ** So we test the effects of a malloc() failing and the sqlite3OsXXX()
// ** function returning SQLITE_IOERR_NOMEM using the DO_OS_MALLOC_TEST macro.
// **
// ** The following functions are instrumented for malloc() failure
// ** testing:
// **
// **     sqlite3OsRead()
// **     sqlite3OsWrite()
// **     sqlite3OsSync()
// **     sqlite3OsFileSize()
// **     sqlite3OsLock()
// **     sqlite3OsCheckReservedLock()
// **     sqlite3OsFileControl()
// **     sqlite3OsShmMap()
// **     sqlite3OsOpen()
// **     sqlite3OsDelete()
// **     sqlite3OsAccess()
// **     sqlite3OsFullPathname()
// **
// */
// /*
// ** The following routines are convenience wrappers around methods
// ** of the sqlite3_file object.  This is mostly just syntactic sugar. All
// ** of this would be completely automatic if SQLite were coded using
// ** C++ instead of plain old C.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsClose(mut pId: *mut sqlite3_file) {
    if (unsafe { (*pId).pMethods }) != std::ptr::null::<sqlite3_io_methods>() {
        unsafe { unsafe { (*unsafe { (*pId).pMethods }).xClose }.unwrap()(pId) };
        unsafe {
            (*pId).pMethods = std::ptr::null::<sqlite3_io_methods>();
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsRead(
    mut id: *mut sqlite3_file,
    mut pBuf: *mut (),
    mut amt: i32,
    mut offset: i64,
) -> i32 {
    {}
    return unsafe {
        unsafe { (*unsafe { (*id).pMethods }).xRead }.unwrap()(id, pBuf, amt, offset)
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsWrite(
    mut id: *mut sqlite3_file,
    mut pBuf: *const (),
    mut amt: i32,
    mut offset: i64,
) -> i32 {
    {}
    return unsafe {
        unsafe { (*unsafe { (*id).pMethods }).xWrite }.unwrap()(id, pBuf, amt, offset)
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsTruncate(mut id: *mut sqlite3_file, mut size: i64) -> i32 {
    return unsafe { unsafe { (*unsafe { (*id).pMethods }).xTruncate }.unwrap()(id, size) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsSync(mut id: *mut sqlite3_file, mut flags: i32) -> i32 {
    {}
    let __v379: i32;
    if flags != (0 as i32) {
        __v379 = unsafe { unsafe { (*unsafe { (*id).pMethods }).xSync }.unwrap()(id, flags) };
    } else {
        __v379 = 0 as i32;
    }
    return __v379;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsFileSize(mut id: *mut sqlite3_file, mut pSize: *mut i64) -> i32 {
    {}
    return unsafe { unsafe { (*unsafe { (*id).pMethods }).xFileSize }.unwrap()(id, pSize) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsLock(mut id: *mut sqlite3_file, mut lockType: i32) -> i32 {
    {}
    0 as i32;
    return unsafe { unsafe { (*unsafe { (*id).pMethods }).xLock }.unwrap()(id, lockType) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsUnlock(mut id: *mut sqlite3_file, mut lockType: i32) -> i32 {
    0 as i32;
    return unsafe { unsafe { (*unsafe { (*id).pMethods }).xUnlock }.unwrap()(id, lockType) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsCheckReservedLock(
    mut id: *mut sqlite3_file,
    mut pResOut: *mut i32,
) -> i32 {
    {}
    return unsafe {
        unsafe { (*unsafe { (*id).pMethods }).xCheckReservedLock }.unwrap()(id, pResOut)
    };
}

// /*
// ** Use sqlite3OsFileControl() when we are doing something that might fail
// ** and we need to know about the failures.  Use sqlite3OsFileControlHint()
// ** when simply tossing information over the wall to the VFS and we do not
// ** really care if the VFS receives and understands the information since it
// ** is only a hint and can be safely ignored.  The sqlite3OsFileControlHint()
// ** routine has no return value since the return value would be meaningless.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsFileControl(
    mut id: *mut sqlite3_file,
    mut op: i32,
    mut pArg: *mut (),
) -> i32 {
    if (unsafe { (*id).pMethods }) == std::ptr::null::<sqlite3_io_methods>() {
        return 12 as i32;
    }
    return unsafe { unsafe { (*unsafe { (*id).pMethods }).xFileControl }.unwrap()(id, op, pArg) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsFileControlHint(
    mut id: *mut sqlite3_file,
    mut op: i32,
    mut pArg: *mut (),
) {
    if (unsafe { (*id).pMethods }) != std::ptr::null::<sqlite3_io_methods>() {
        unsafe { unsafe { (*unsafe { (*id).pMethods }).xFileControl }.unwrap()(id, op, pArg) };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsSectorSize(mut id: *mut sqlite3_file) -> i32 {
    let mut xSectorSize: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file) -> i32> =
        unsafe { (*unsafe { (*id).pMethods }).xSectorSize };
    let __v380: i32;
    if xSectorSize != None {
        __v380 = unsafe { xSectorSize.unwrap()(id) };
    } else {
        __v380 = 4096 as i32;
    }
    return __v380;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsDeviceCharacteristics(mut id: *mut sqlite3_file) -> i32 {
    if (unsafe { (*id).pMethods }) == std::ptr::null::<sqlite3_io_methods>() {
        return 0 as i32;
    }
    return unsafe { unsafe { (*unsafe { (*id).pMethods }).xDeviceCharacteristics }.unwrap()(id) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsShmMap(
    mut id: *mut sqlite3_file,
    mut iPage: i32,
    mut pgsz: i32,
    mut bExtend: i32,
    mut pp: *mut *mut (),
) -> i32 {
    {}
    return unsafe {
        unsafe { (*unsafe { (*id).pMethods }).xShmMap }.unwrap()(id, iPage, pgsz, bExtend, pp)
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsShmLock(
    mut id: *mut sqlite3_file,
    mut offset: i32,
    mut n: i32,
    mut flags: i32,
) -> i32 {
    return unsafe {
        unsafe { (*unsafe { (*id).pMethods }).xShmLock }.unwrap()(id, offset, n, flags)
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsShmBarrier(mut id: *mut sqlite3_file) {
    unsafe { unsafe { (*unsafe { (*id).pMethods }).xShmBarrier }.unwrap()(id) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsShmUnmap(mut id: *mut sqlite3_file, mut deleteFlag: i32) -> i32 {
    return unsafe { unsafe { (*unsafe { (*id).pMethods }).xShmUnmap }.unwrap()(id, deleteFlag) };
}

// /* Database file handle */
// /* True to extend file if necessary */
// /* OUT: Pointer to mapping */
// /* SQLITE_OMIT_WAL */
// /* The real implementation of xFetch and xUnfetch */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsFetch(
    mut id: *mut sqlite3_file,
    mut iOff: i64,
    mut iAmt: i32,
    mut pp: *mut *mut (),
) -> i32 {
    {}
    return unsafe { unsafe { (*unsafe { (*id).pMethods }).xFetch }.unwrap()(id, iOff, iAmt, pp) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsUnfetch(
    mut id: *mut sqlite3_file,
    mut iOff: i64,
    mut p: *mut (),
) -> i32 {
    return unsafe { unsafe { (*unsafe { (*id).pMethods }).xUnfetch }.unwrap()(id, iOff, p) };
}

// /*
// ** The next group of routines are convenience wrappers around the
// ** VFS methods.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const i8,
    mut pFile: *mut sqlite3_file,
    mut flags: i32,
    mut pFlagsOut: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    {}
    // /* 0x87f7f is a mask of SQLITE_OPEN_ flags that are valid to be passed
    //   ** down into the VFS layer.  Some SQLITE_OPEN_ flags (for example,
    //   ** SQLITE_OPEN_FULLMUTEX or SQLITE_OPEN_SHAREDCACHE) are blocked before
    //   ** reaching the VFS. */
    0 as i32;
    rc = unsafe {
        unsafe { (*pVfs).xOpen }.unwrap()(pVfs, zPath, pFile, flags & (17334143 as i32), pFlagsOut)
    };
    0 as i32;
    return rc;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsDelete(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const i8,
    mut dirSync: i32,
) -> i32 {
    {}
    0 as i32;
    let __v381: i32;
    if (unsafe { (*pVfs).xDelete }) != None {
        __v381 = unsafe { unsafe { (*pVfs).xDelete }.unwrap()(pVfs, zPath, dirSync) };
    } else {
        __v381 = 0 as i32;
    }
    return __v381;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsAccess(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const i8,
    mut flags: i32,
    mut pResOut: *mut i32,
) -> i32 {
    {}
    return unsafe { unsafe { (*pVfs).xAccess }.unwrap()(pVfs, zPath, flags, pResOut) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsFullPathname(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const i8,
    mut nPathOut: i32,
    mut zPathOut: *mut i8,
) -> i32 {
    {}
    unsafe {
        *unsafe { zPathOut.offset((0 as i32) as isize) } = (0 as i32) as i8;
    }
    return unsafe { unsafe { (*pVfs).xFullPathname }.unwrap()(pVfs, zPath, nPathOut, zPathOut) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsDlOpen(mut pVfs: *mut sqlite3_vfs, mut zPath: *const i8) -> *mut () {
    0 as i32;
    // /* tag-20210611-1 */
    0 as i32;
    return unsafe { unsafe { (*pVfs).xDlOpen }.unwrap()(pVfs, zPath) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsDlError(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: i32,
    mut zBufOut: *mut i8,
) {
    unsafe { unsafe { (*pVfs).xDlError }.unwrap()(pVfs, nByte, zBufOut) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsDlSym(
    mut pVfs: *mut sqlite3_vfs,
    mut pHdle: *mut (),
    mut zSym: *const i8,
) -> Option<unsafe extern "C-unwind" fn()> {
    return unsafe { unsafe { (*pVfs).xDlSym }.unwrap()(pVfs, pHdle, zSym) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsDlClose(mut pVfs: *mut sqlite3_vfs, mut pHandle: *mut ()) {
    unsafe { unsafe { (*pVfs).xDlClose }.unwrap()(pVfs, pHandle) };
}

// /* SQLITE_OMIT_LOAD_EXTENSION */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsRandomness(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: i32,
    mut zBufOut: *mut i8,
) -> i32 {
    if (unsafe { sqlite3Config.iPrngSeed }) != (0 as u32) {
        unsafe { memset(zBufOut as *mut (), 0 as i32, (nByte as i64) as u64) };
        if nByte > (((4 as u64) as u32) as i32) {
            nByte = ((4 as u64) as u32) as i32;
        }
        unsafe {
            memcpy(
                zBufOut as *mut (),
                (unsafe { std::ptr::addr_of_mut!(sqlite3Config.iPrngSeed) }) as *const (),
                (nByte as i64) as u64,
            )
        };
        return 0 as i32;
    } else {
        return unsafe { unsafe { (*pVfs).xRandomness }.unwrap()(pVfs, nByte, zBufOut) };
    }
    return unsafe { std::mem::zeroed() };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsSleep(mut pVfs: *mut sqlite3_vfs, mut nMicro: i32) -> i32 {
    return unsafe { unsafe { (*pVfs).xSleep }.unwrap()(pVfs, nMicro) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsGetLastError(mut pVfs: *mut sqlite3_vfs) -> i32 {
    let __v382: i32;
    if (unsafe { (*pVfs).xGetLastError }) != None {
        __v382 = unsafe {
            unsafe { (*pVfs).xGetLastError }.unwrap()(pVfs, 0 as i32, std::ptr::null_mut::<i8>())
        };
    } else {
        __v382 = 0 as i32;
    }
    return __v382;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsCurrentTimeInt64(
    mut pVfs: *mut sqlite3_vfs,
    mut pTimeOut: *mut i64,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    // /* IMPLEMENTATION-OF: R-49045-42493 SQLite will use the xCurrentTimeInt64()
    //   ** method to get the current date and time if that method is available
    //   ** (if iVersion is 2 or greater and the function pointer is not NULL) and
    //   ** will fall back to xCurrentTime() if xCurrentTimeInt64() is
    //   ** unavailable.
    //   */
    if (unsafe { (*pVfs).iVersion }) >= (2 as i32) && (unsafe { (*pVfs).xCurrentTimeInt64 }) != None
    {
        rc = unsafe { unsafe { (*pVfs).xCurrentTimeInt64 }.unwrap()(pVfs, pTimeOut) };
    } else {
        let mut r: f64 = 0 as f64;
        rc = unsafe { unsafe { (*pVfs).xCurrentTime }.unwrap()(pVfs, std::ptr::addr_of_mut!(r)) };
        unsafe {
            *pTimeOut = unsafe { sqlite3RealToI64(r * 86400000.0f64) };
        }
    }
    return rc;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsOpenMalloc(
    mut pVfs: *mut sqlite3_vfs,
    mut zFile: *const i8,
    mut ppFile: *mut *mut sqlite3_file,
    mut flags: i32,
    mut pOutFlags: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pFile: *mut sqlite3_file = unsafe { std::mem::zeroed() };
    pFile = (unsafe { sqlite3MallocZero(((unsafe { (*pVfs).szOsFile }) as i64) as u64) })
        as *mut sqlite3_file;
    if pFile != std::ptr::null_mut::<sqlite3_file>() {
        rc = sqlite3OsOpen(pVfs, zFile, pFile, flags, pOutFlags);
        if rc != (0 as i32) {
            unsafe { sqlite3_free(pFile as *mut ()) };
            unsafe {
                *ppFile = std::ptr::null_mut::<sqlite3_file>();
            }
        } else {
            unsafe {
                *ppFile = pFile;
            }
        }
    } else {
        unsafe {
            *ppFile = std::ptr::null_mut::<sqlite3_file>();
        }
        rc = 7 as i32;
    }
    0 as i32;
    return rc;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OsCloseFree(mut pFile: *mut sqlite3_file) {
    0 as i32;
    sqlite3OsClose(pFile);
    unsafe { sqlite3_free(pFile as *mut ()) };
}

// /*
// ** Unlink a VFS from the linked list
// */
fn vfsUnlink(mut pVfs: *mut sqlite3_vfs) {
    0 as i32;
    if pVfs == std::ptr::null_mut::<sqlite3_vfs>() {
        // /* No-op */
    } else {
        if (unsafe { vfsList }) == pVfs {
            unsafe {
                vfsList = unsafe { (*pVfs).pNext };
            }
        } else {
            if (unsafe { vfsList }) != std::ptr::null_mut::<sqlite3_vfs>() {
                let mut p: *mut sqlite3_vfs = unsafe { vfsList };
                '__slate_break_378: while (unsafe { (*p).pNext })
                    != std::ptr::null_mut::<sqlite3_vfs>()
                    && (unsafe { (*p).pNext }) != pVfs
                {
                    p = unsafe { (*p).pNext };
                }
                if (unsafe { (*p).pNext }) == pVfs {
                    unsafe {
                        (*p).pNext = unsafe { (*pVfs).pNext };
                    }
                }
            }
        }
    }
}
