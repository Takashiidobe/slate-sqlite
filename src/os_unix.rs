//! 2004 May 22
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
//! This file contains the VFS implementation for unix-like operating systems
//! include Linux, MacOSX, *BSD, QNX, VxWorks, AIX, HPUX, and others.
//!
//! There are actually several different VFS implementations in this file.
//! The differences are in the way that file locking is done.  The default
//! implementation uses Posix Advisory Locks.  Alternative implementations
//! use flock(), dot-files, various proprietary locking schemas, or simply
//! skip locking all together.
//!
//! This source file is organized into divisions where the logic for various
//! subfunctions is contained within the appropriate division.  PLEASE
//! KEEP THE STRUCTURE OF THIS FILE INTACT.  New code should be placed
//! in the correct division and should be clearly labelled.
//!
//! The layout of divisions is as follows:
//!
//!   *  General-purpose declarations and utility functions.
//!   *  Unique file ID logic used by VxWorks.
//!   *  Various locking primitive implementations (all except proxy locking):
//!      + for Posix Advisory Locks
//!      + for no-op locks
//!      + for dot-file locks
//!      + for flock() locking
//!      + for named semaphore locks (VxWorks only)
//!      + for AFP filesystem locks (MacOSX only)
//!   *  sqlite3_file methods not associated with locking.
//!   *  Definitions of sqlite3_io_methods objects for all locking
//!      methods plus "finder" functions for each locking method.
//!   *  sqlite3_vfs method implementations.
//!   *  Locking primitives for the proxy uber-locking-method. (MacOSX only)
//!   *  Definitions of sqlite3_vfs objects for all locking methods
//!      plus implementations of sqlite3_os_init() and sqlite3_os_end().
unsafe extern "C" {
    static mut sqlite3_temp_directory: *mut i8;
    static mut sqlite3Config: Sqlite3Config;
    static mut sqlite3PendingByte: i32;
    fn sqlite3_mprintf(__v1048: *const i8, ...) -> *mut i8;
    fn sqlite3_snprintf(__v1049: i32, __v1050: *mut i8, __v1051: *const i8, ...) -> *mut i8;
    fn sqlite3_malloc64(__v1052: u64) -> *mut ();
    fn sqlite3_realloc64(__v1053: *mut (), __v1054: u64) -> *mut ();
    fn sqlite3_free(__v1055: *mut ());
    fn sqlite3_randomness(N: i32, P: *mut ());
    fn sqlite3_uri_parameter(z: *const i8, zParam: *const i8) -> *const i8;
    fn sqlite3_uri_boolean(z: *const i8, zParam: *const i8, bDefault: i32) -> i32;
    fn sqlite3_vfs_register(__v1063: *mut sqlite3_vfs, makeDflt: i32) -> i32;
    fn sqlite3_mutex_alloc(__v1065: i32) -> *mut sqlite3_mutex;
    fn sqlite3_mutex_free(__v1066: *mut sqlite3_mutex);
    fn sqlite3_mutex_enter(__v1067: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(__v1068: *mut sqlite3_mutex);
    fn sqlite3_log(iErrCode: i32, zFormat: *const i8, ...);
    fn getenv(__name: *const i8) -> *mut i8;
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3CantopenError(__v1084: i32) -> i32;
    fn sqlite3Strlen30(__v1085: *const i8) -> i32;
    fn sqlite3MutexAlloc(__v1086: i32) -> *mut sqlite3_mutex;
    fn sqlite3MemoryBarrier();
    fn stat(__file: *const i8, __buf: *mut stat) -> i32;
    fn fstat(__fd: i32, __buf: *mut stat) -> i32;
    fn lstat(__file: *const i8, __buf: *mut stat) -> i32;
    fn fchmod(__fd: i32, __mode: u32) -> i32;
    fn mkdir(__path: *const i8, __mode: u32) -> i32;
    fn fcntl(__fd: i32, __cmd: i32, ...) -> i32;
    fn open(__file: *const i8, __oflag: i32, ...) -> i32;
    fn access(__name: *const i8, __type: i32) -> i32;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut (), __nbytes: u64) -> i64;
    fn write(__fd: i32, __buf: *const (), __n: u64) -> i64;
    fn pread64(__fd: i32, __buf: *mut (), __nbytes: u64, __offset: i64) -> i64;
    fn pwrite64(__fd: i32, __buf: *const (), __n: u64, __offset: i64) -> i64;
    fn fchown(__fd: i32, __owner: u32, __group: u32) -> i32;
    fn getcwd(__buf: *mut i8, __size: u64) -> *mut i8;
    fn sysconf(__name: i32) -> i64;
    fn getpid() -> i32;
    fn geteuid() -> u32;
    fn readlink(__path: *const i8, __buf: *mut i8, __len: u64) -> i64;
    fn unlink(__name: *const i8) -> i32;
    fn rmdir(__path: *const i8) -> i32;
    fn ftruncate(__fd: i32, __length: i64) -> i32;
    fn fdatasync(__fildes: i32) -> i32;
    fn time(__timer: *mut i64) -> i64;
    fn nanosleep(__requested_time: *const timespec, __remaining: *mut timespec) -> i32;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut ()) -> i32;
    fn __errno_location() -> *mut i32;
    fn mmap(
        __addr: *mut (),
        __len: u64,
        __prot: i32,
        __flags: i32,
        __fd: i32,
        __offset: i64,
    ) -> *mut ();
    fn munmap(__addr: *mut (), __len: u64) -> i32;
    fn mremap(__addr: *mut (), __old_len: u64, __new_len: u64, __flags: i32, ...) -> *mut ();
    fn utime(__file: *const i8, __file_times: *const utimbuf) -> i32;
    fn dlopen(__file: *const i8, __mode: i32) -> *mut ();
    fn dlclose(__handle: *mut ()) -> i32;
    fn dlsym(__handle: *mut (), __name: *const i8) -> *mut ();
    fn dlerror() -> *mut i8;
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
struct timeval {
    tv_sec: i64,
    tv_usec: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct timespec {
    tv_sec: i64,
    tv_nsec: i64,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct stat {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: u32,
    st_uid: u32,
    st_gid: u32,
    __pad0: i32,
    st_rdev: u64,
    st_size: i64,
    st_blksize: i64,
    st_blocks: i64,
    st_atim: timespec,
    st_mtim: timespec,
    st_ctim: timespec,
    __glibc_reserved: [i64; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct flock {
    l_type: i16,
    l_whence: i16,
    l_start: i64,
    l_len: i64,
    l_pid: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct utimbuf {
    actime: i64,
    modtime: i64,
}

// There are various methods for file locking used for concurrency
// control:
//
//   1. POSIX locking (the default),
//   2. No locking,
//   3. Dot-file locking,
//   4. flock() locking,
//   5. AFP locking (OSX only),
//   6. Named POSIX semaphores (VXWorks only),
//   7. proxy locking. (OSX only)
//
// Styles 4, 5, and 7 are only available of SQLITE_ENABLE_LOCKING_STYLE
// is defined to 1.  The SQLITE_ENABLE_LOCKING_STYLE also enables automatic
// selection of the appropriate locking style based on the filesystem
// where the database is located.
// Use pread() and pwrite() if they are available
// standard include files.
// amalgamator: keep
// amalgamator: keep
// amalgamator: keep
// amalgamator: keep
// Try to determine if gethostuuid() is available based on standard
// macros.  This might sometimes compute the wrong value for some
// obscure platforms.  For those cases, simply compile with one of
// the following:
//
//    -DHAVE_GETHOSTUUID=0
//    -DHAVE_GETHOSTUUID=1
//
// None if this matters except when building on Apple products with
// -DSQLITE_ENABLE_LOCKING_STYLE.
// Allowed values of unixFile.fsFlags
// If we are to be thread-safe, include the pthreads header.
// Default permissions when creating a new file
// Default permissions when creating auto proxy dir
// Maximum supported path-length.
// Maximum supported symbolic links
// Remove and stub certain info for WASI (WebAssembly System
// Interface) builds.
// Always cast the getpid() return type for compatibility with
// kernel modules in VxWorks.
// Only set the lastErrno if the error code is a real error and not
// a normal expected return code of SQLITE_BUSY or SQLITE_OK
// Connection shared memory
// Shared memory instance
// An i-node
// An unused file descriptor
/// Sometimes, after a file handle is closed by SQLite, the file descriptor
/// cannot be closed immediately. In these cases, instances of the following
/// structure are used to store the file descriptor while waiting for an
/// opportunity to either close or reuse it.
#[repr(C)]
#[derive(Clone, Copy)]
struct UnixUnusedFd {
    /// File descriptor to close
    fd: i32,
    /// Flags this file descriptor was opened with
    flags: i32,
    /// Next unused file descriptor on same file
    pNext: *mut UnixUnusedFd,
}

/// The unixFile structure is subclass of sqlite3_file specific to the unix
/// VFS implementations.
#[repr(C)]
#[derive(Clone, Copy)]
struct unixFile {
    /// Always the first entry
    pMethod: *const sqlite3_io_methods,
    /// The VFS that created this unixFile
    pVfs: *mut sqlite3_vfs,
    /// Info about locks on this inode
    pInode: *mut unixInodeInfo,
    /// The file descriptor
    h: i32,
    /// The type of lock held on this fd
    eFileLock: u8,
    /// Behavioral bits.  UNIXFILE_* flags
    ctrlFlags: u16,
    /// The unix errno from last I/O error
    lastErrno: i32,
    /// Locking style specific state
    lockingContext: *mut (),
    /// Pre-allocated UnixUnusedFd
    pPreallocatedUnused: *mut UnixUnusedFd,
    /// Name of the file
    zPath: *const i8,
    /// Shared memory segment information
    pShm: *mut unixShm,
    /// Configured by FCNTL_CHUNK_SIZE
    szChunk: i32,
    /// Number of outstanding xFetch refs
    nFetchOut: i32,
    /// Usable size of mapping at pMapRegion
    mmapSize: i64,
    /// Actual size of mapping at pMapRegion
    mmapSizeActual: i64,
    /// Configured FCNTL_MMAP_SIZE value
    mmapSizeMax: i64,
    /// Memory mapped region
    pMapRegion: *mut (),
    /// Device sector size
    sectorSize: i32,
    /// Precomputed device characteristics
    deviceCharacteristics: i32,
}

/// This variable holds the process id (pid) from when the xRandomness()
/// method was called.  If xOpen() is called from a different process id,
/// indicating that a fork() has occurred, the PRNG will be reset.
static mut randomnessPid: i32 = 0 as i32;

// Allowed values for the unixFile.ctrlFlags bitmask:
// Connections from one process only
// Connection is read only
// Persistent WAL mode
// Directory sync needed
// SQLITE_IOCAP_POWERSAFE_OVERWRITE
// Delete on close
// Filename might have query parameters
// Do no file locking
// Include code that is common to all os_*.c files
// Define various macros that are missing from some systems.
// The threadid macro resolves to the thread-id or to 0.  Used for
// testing and debugging only.
// HAVE_MREMAP defaults to true on Linux and false everywhere else.
// Explicitly call the 64-bit version of lseek() on Android. Otherwise, lseek()
// is the 32-bit version, even if _FILE_OFFSET_BITS=64 is defined.
// Linux-specific IOCTL magic numbers used for controlling F2FS
/// Different Unix systems declare open() in different ways.  Same use
/// open(const char*,int,mode_t).  Others use open(const char*,int,...).
/// The difference is important when using a pointer to the function.
///
/// The safest way to deal with the problem is to always use this wrapper
/// which always has the same well-defined interface.
#[unsafe(link_section = ".text.slate_distinct.os_unix.posixOpen")]
extern "C-unwind" fn posixOpen(mut zFile: *const i8, mut flags: i32, mut mode: i32) -> i32 {
    return unsafe { open(zFile, flags, mode) };
}

/// Many system calls are accessed through pointer-to-functions so that
/// they may be overridden at runtime to facilitate fault injection during
/// testing and sandboxing.  The following array holds the names and pointers
/// to all overrideable system calls.
#[repr(C)]
#[derive(Clone, Copy)]
struct unix_syscall {
    /// Name of the system call
    zName: *const i8,
    /// Current value of the system call
    pCurrent: Option<unsafe extern "C-unwind" fn()>,
    /// Default value
    pDefault: Option<unsafe extern "C-unwind" fn()>,
}

/// The DJGPP compiler environment looks mostly like Unix, but it
/// lacks the fcntl() system call.  So redefine fcntl() to be something
/// that always succeeds.  This means that locking does not occur under
/// DJGPP.  But it is DOS - what did you expect?
/// End of the overrideable system calls
static mut aSyscall: __SlateAlign16<[unix_syscall; 29]> = __SlateAlign16([
    unix_syscall {
        zName: (b"open\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*const i8, i32, i32) -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(Some(posixOpen))
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"close\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(i32) -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i32) -> i32>>(
                    close as *const (),
                )
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"access\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*const i8, i32) -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*const i8, i32) -> i32>,
                >(access as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"getcwd\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut i8, u64) -> *mut i8>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut i8, u64) -> *mut i8>,
                >(getcwd as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"stat\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*const i8, *mut stat) -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*const i8, *mut stat) -> i32>,
                >(stat as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"fstat\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(i32, *mut stat) -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(i32, *mut stat) -> i32>,
                >(fstat as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"ftruncate\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(i32, i64) -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i32, i64) -> i32>>(
                    ftruncate as *const (),
                )
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"fcntl\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(i32, i32, ...) -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(i32, i32, ...) -> i32>,
                >(fcntl as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"read\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(i32, *mut (), u64) -> i64>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(i32, *mut (), u64) -> i64>,
                >(read as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"pread\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: None,
        pDefault: None,
    },
    unix_syscall {
        zName: (b"pread64\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(i32, *mut (), u64, i64) -> i64>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(i32, *mut (), u64, i64) -> i64>,
                >(pread64 as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"write\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(i32, *const (), u64) -> i64>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(i32, *const (), u64) -> i64>,
                >(write as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"pwrite\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: None,
        pDefault: None,
    },
    unix_syscall {
        zName: (b"pwrite64\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(i32, *const (), u64, i64) -> i64>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(i32, *const (), u64, i64) -> i64>,
                >(pwrite64 as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"fchmod\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(i32, u32) -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i32, u32) -> i32>>(
                    fchmod as *const (),
                )
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"fallocate\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: None,
        pDefault: None,
    },
    unix_syscall {
        zName: (b"unlink\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*const i8) -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*const i8) -> i32>,
                >(unlink as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"openDirectory\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*const i8, *mut i32) -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(Some(openDirectory))
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"mkdir\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*const i8, u32) -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*const i8, u32) -> i32>,
                >(mkdir as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"rmdir\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*const i8) -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*const i8) -> i32>,
                >(rmdir as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"fchown\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(i32, u32, u32) -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(i32, u32, u32) -> i32>,
                >(fchown as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"geteuid\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn() -> u32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> u32>>(
                    geteuid as *const (),
                )
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"mmap\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut (), u64, i32, i32, i32, i64) -> *mut ()>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<
                        unsafe extern "C-unwind" fn(*mut (), u64, i32, i32, i32, i64) -> *mut (),
                    >,
                >(mmap as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"munmap\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut (), u64) -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut (), u64) -> i32>,
                >(munmap as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"mremap\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut (), u64, u64, i32, ...) -> *mut ()>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut (), u64, u64, i32, ...) -> *mut ()>,
                >(mremap as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"getpagesize\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn() -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(Some(unixGetpagesize))
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"readlink\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*const i8, *mut i8, u64) -> i64>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*const i8, *mut i8, u64) -> i64>,
                >(readlink as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"lstat\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*const i8, *mut stat) -> i32>,
                Option<unsafe extern "C-unwind" fn()>,
            >(unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*const i8, *mut stat) -> i32>,
                >(lstat as *const ())
            })
        },
        pDefault: None,
    },
    unix_syscall {
        zName: (b"ioctl\0".as_ptr() as *mut i8) as *const i8,
        pCurrent: None,
        pDefault: None,
    },
]);

/// On some systems, calls to fchown() will trigger a message in a security
/// log if they come from non-root processes.  So avoid calling fchown() if
/// we are not running as root.
fn robustFchown(mut fd: i32, mut uid: u32, mut gid: u32) -> i32 {
    let __v1287: i32;
    if (unsafe {
        unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn() -> u32>,
            >(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                        .offset((21 as i32) as isize)
                })
                .pCurrent
            })
        }
        .unwrap()()
    }) != (0 as u32)
    {
        __v1287 = 0 as i32;
    } else {
        __v1287 = unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(i32, u32, u32) -> i32>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((20 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(fd, uid, gid)
        };
    }
    return __v1287;
}

/// This is the xSetSystemCall() method of sqlite3_vfs for all of the
/// "unix" VFSes.  Return SQLITE_OK upon successfully updating the
/// system call pointer, or SQLITE_NOTFOUND if there is no configurable
/// system call named zName.
///
/// # Arguments
///
/// * `pNotUsed` - The VFS pointer.  Not used
/// * `zName` - Name of system call to override
/// * `pNewFunc` - Pointer to new system call value
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixSetSystemCall")]
extern "C-unwind" fn unixSetSystemCall(
    mut pNotUsed: *mut sqlite3_vfs,
    mut zName: *const i8,
    mut pNewFunc: Option<unsafe extern "C-unwind" fn()>,
) -> i32 {
    let mut i: u32 = 0 as u32;
    let mut rc: i32 = 12 as i32;
    pNotUsed;
    if zName == std::ptr::null::<i8>() {
        // If no zName is given, restore all system calls to their default
        // settings and return NULL
        rc = 0 as i32;
        i = (0 as i32) as u32;
        '__slate_break_1182: while (i as u64) < (696 as u64) / (24 as u64) {
            if (unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                        .offset(i as isize)
                })
                .pDefault
            }) != None
            {
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset(i as isize)
                    })
                    .pCurrent = unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset(i as isize)
                        })
                        .pDefault
                    };
                }
            }
            let __v1288: u32 = i;
            let __v1289: u32 = __v1288.wrapping_add((1 as i32) as u32);
            i = __v1289;
        }
    } else {
        // If zName is specified, operate on only the one system call
        // specified.
        i = (0 as i32) as u32;
        '__slate_break_1183: while (i as u64) < (696 as u64) / (24 as u64) {
            if (unsafe {
                strcmp(zName, unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset(i as isize)
                    })
                    .zName
                })
            }) == (0 as i32)
            {
                if (unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset(i as isize)
                    })
                    .pDefault
                }) == None
                {
                    unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset(i as isize)
                        })
                        .pDefault = unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                    .offset(i as isize)
                            })
                            .pCurrent
                        };
                    }
                }
                rc = 0 as i32;
                if pNewFunc == None {
                    pNewFunc = unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset(i as isize)
                        })
                        .pDefault
                    };
                }
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset(i as isize)
                    })
                    .pCurrent = pNewFunc;
                }
                break '__slate_break_1183;
            }
            let __v1290: u32 = i;
            let __v1291: u32 = __v1290.wrapping_add((1 as i32) as u32);
            i = __v1291;
        }
    }
    return rc;
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

/// Return the value of a system call.  Return NULL if zName is not a
/// recognized system call name.  NULL is also returned if the system call
/// is currently undefined.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixGetSystemCall")]
extern "C-unwind" fn unixGetSystemCall(
    mut pNotUsed: *mut sqlite3_vfs,
    mut zName: *const i8,
) -> Option<unsafe extern "C-unwind" fn()> {
    let mut i: u32 = 0 as u32;
    pNotUsed;
    i = (0 as i32) as u32;
    '__slate_break_1184: while (i as u64) < (696 as u64) / (24 as u64) {
        if (unsafe {
            strcmp(zName, unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                        .offset(i as isize)
                })
                .zName
            })
        }) == (0 as i32)
        {
            return unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                        .offset(i as isize)
                })
                .pCurrent
            };
        }
        let __v1292: u32 = i;
        let __v1293: u32 = __v1292.wrapping_add((1 as i32) as u32);
        i = __v1293;
    }
    return None;
}

/// Return the name of the first system call after zName.  If zName==NULL
/// then return the name of the first system call.  Return NULL if zName
/// is the last system call or if zName is not the name of a valid
/// system call.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixNextSystemCall")]
extern "C-unwind" fn unixNextSystemCall(
    mut p: *mut sqlite3_vfs,
    mut zName: *const i8,
) -> *const i8 {
    let mut i: i32 = -(1 as i32);
    p;
    if zName != std::ptr::null::<i8>() {
        i = 0 as i32;
        '__slate_break_1185: loop {
            if !(i < ((((696 as u64) / (24 as u64)) as u32) as i32) - (1 as i32)) {
                break;
            }
            if (unsafe {
                strcmp(zName, unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset(i as isize)
                    })
                    .zName
                })
            }) == (0 as i32)
            {
                break '__slate_break_1185;
            }
            let __v1294: i32 = i;
            let __v1295: i32 = __v1294 + (1 as i32);
            i = __v1295;
        }
    }
    let __v1296: i32 = i;
    let __v1297: i32 = __v1296 + (1 as i32);
    i = __v1297;
    '__slate_break_1186: loop {
        if !(i < ((((696 as u64) / (24 as u64)) as u32) as i32)) {
            break;
        }
        if (unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                    .offset(i as isize)
            })
            .pCurrent
        }) != None
        {
            return unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                        .offset(i as isize)
                })
                .zName
            };
        }
        let __v1298: i32 = i;
        let __v1299: i32 = __v1298 + (1 as i32);
        i = __v1299;
    }
    return std::ptr::null::<i8>();
}

// Do not accept any file descriptor less than this value, in order to avoid
// opening database file using file descriptors that are commonly used for
// standard input, output, and error.
/// Invoke open().  Do so multiple times, until it either succeeds or
/// fails for some reason other than EINTR.
///
/// If the file creation mode "m" is 0 then set it to the default for
/// SQLite.  The default is SQLITE_DEFAULT_FILE_PERMISSIONS (normally
/// 0644) as modified by the system umask.  If m is not 0, then
/// make the file creation mode be exactly m ignoring the umask.
///
/// The m parameter will be non-zero only when creating -wal, -journal,
/// and -shm files.  We want those files to have *exactly* the same
/// permissions as their original database, unadulterated by the umask.
/// In that way, if a database file is -rw-rw-rw or -rw-rw-r-, and a
/// transaction crashes and leaves behind hot journals, then any
/// process that is able to write to the database will also be able to
/// recover the hot journals.
fn robust_open(mut z: *const i8, mut f: i32, mut m: u32) -> i32 {
    let mut fd: i32 = 0 as i32;
    let mut m2: u32 = if m != (0 as u32) {
        m
    } else {
        (420 as i32) as u32
    };
    '__slate_break_1187: while (1 as i32) != (0 as i32) {
        '__slate_continue_1187: {
            fd = unsafe {
                unsafe {
                    std::mem::transmute::<
                        Option<unsafe extern "C-unwind" fn()>,
                        Option<unsafe extern "C-unwind" fn(*const i8, i32, i32) -> i32>,
                    >(unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset((0 as i32) as isize)
                        })
                        .pCurrent
                    })
                }
                .unwrap()(z, f | (524288 as i32), m2 as i32)
            };
            if fd < (0 as i32) {
                if (unsafe { *unsafe { __errno_location() } }) == (4 as i32) {
                    break '__slate_continue_1187;
                }
                break '__slate_break_1187;
            }
            if fd >= (3 as i32) {
                break '__slate_break_1187;
            }
            if f & ((128 as i32) | (64 as i32)) == (128 as i32) | (64 as i32) {
                unsafe {
                    unsafe {
                        std::mem::transmute::<
                            Option<unsafe extern "C-unwind" fn()>,
                            Option<unsafe extern "C-unwind" fn(*const i8) -> i32>,
                        >(unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                    .offset((16 as i32) as isize)
                            })
                            .pCurrent
                        })
                    }
                    .unwrap()(z)
                };
            }
            unsafe {
                unsafe {
                    std::mem::transmute::<
                        Option<unsafe extern "C-unwind" fn()>,
                        Option<unsafe extern "C-unwind" fn(i32) -> i32>,
                    >(unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset((1 as i32) as isize)
                        })
                        .pCurrent
                    })
                }
                .unwrap()(fd)
            };
            unsafe {
                sqlite3_log(
                    28 as i32,
                    (b"attempt to open \"%s\" as file descriptor %d\0".as_ptr() as *mut i8)
                        as *const i8,
                    z,
                    fd,
                )
            };
            fd = -(1 as i32);
            if (unsafe {
                unsafe {
                    std::mem::transmute::<
                        Option<unsafe extern "C-unwind" fn()>,
                        Option<unsafe extern "C-unwind" fn(*const i8, i32, i32) -> i32>,
                    >(unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset((0 as i32) as isize)
                        })
                        .pCurrent
                    })
                }
                .unwrap()(
                    (b"/dev/null\0".as_ptr() as *mut i8) as *const i8,
                    0 as i32,
                    m as i32,
                )
            }) < (0 as i32)
            {
                break '__slate_break_1187;
            }
        }
    }
    if fd >= (0 as i32) {
        if m != ((0 as i32) as u32) {
            let mut statbuf: stat = unsafe { std::mem::zeroed() };
            if (unsafe {
                unsafe {
                    std::mem::transmute::<
                        Option<unsafe extern "C-unwind" fn()>,
                        Option<unsafe extern "C-unwind" fn(i32, *mut stat) -> i32>,
                    >(unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset((5 as i32) as isize)
                        })
                        .pCurrent
                    })
                }
                .unwrap()(fd, std::ptr::addr_of_mut!(statbuf))
            }) == (0 as i32)
                && statbuf.st_size == ((0 as i32) as i64)
                && statbuf.st_mode & ((511 as i32) as u32) != m
            {
                unsafe {
                    unsafe {
                        std::mem::transmute::<
                            Option<unsafe extern "C-unwind" fn()>,
                            Option<unsafe extern "C-unwind" fn(i32, u32) -> i32>,
                        >(unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                    .offset((14 as i32) as isize)
                            })
                            .pCurrent
                        })
                    }
                    .unwrap()(fd, m)
                };
            }
        }
    }
    return fd;
}

/// Helper functions to obtain and relinquish the global mutex. The
/// global mutex is used to protect the unixInodeInfo objects used by
/// this file, all of which may be shared by multiple threads.
///
/// Function unixMutexHeld() is used to assert() that the global mutex
/// is held when required. This function is only used as part of assert()
/// statements. e.g.
///
///   unixEnterMutex()
///     assert( unixMutexHeld() );
///   unixEnterLeave()
///
/// To prevent deadlock, the global unixBigLock must must be acquired
/// before the unixInodeInfo.pLockMutex mutex, if both are held.  It is
/// OK to get the pLockMutex without holding unixBigLock first, but if
/// that happens, the unixBigLock mutex must not be acquired until after
/// pLockMutex is released.
///
///      OK:     enter(unixBigLock),  enter(pLockInfo)
///      OK:     enter(unixBigLock)
///      OK:     enter(pLockInfo)
///   ERROR:     enter(pLockInfo), enter(unixBigLock)
static mut unixBigLock: *mut sqlite3_mutex = std::ptr::null_mut::<sqlite3_mutex>();

fn unixEnterMutex() {
    0 as i32; // Not a recursive mutex
    unsafe { sqlite3_mutex_enter(unsafe { unixBigLock }) };
}

fn unixLeaveMutex() {
    0 as i32;
    unsafe { sqlite3_mutex_leave(unsafe { unixBigLock }) };
}

/// Retry ftruncate() calls that fail due to EINTR
///
/// All calls to ftruncate() within this file should be made through
/// this wrapper.  On the Android platform, bypassing the logic below
/// could lead to a corrupt database.
fn robust_ftruncate(mut h: i32, mut sz: i64) -> i32 {
    let mut rc: i32 = 0 as i32;
    '__slate_break_1190: loop {
        rc = unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(i32, i64) -> i32>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((6 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(h, sz)
        };
        if !(rc < (0 as i32) && (unsafe { *unsafe { __errno_location() } }) == (4 as i32)) {
            break;
        }
    }
    return rc;
}

/// This routine translates a standard POSIX errno code into something
/// useful to the clients of the sqlite3 functions.  Specifically, it is
/// intended to translate a variety of "try again" errors into SQLITE_BUSY
/// and a variety of "please close the file descriptor NOW" errors into
/// SQLITE_IOERR
///
/// Errors during initialization of locks, or file system support for locks,
/// should handle ENOLCK, ENOTSUP, EOPNOTSUPP separately.
fn sqliteErrorFromPosixError(mut posixError: i32, mut sqliteIOErr: i32) -> i32 {
    0 as i32;
    match posixError {
        13 | 11 | 110 | 16 | 4 | 37 => {
            return 5 as i32;
            // random NFS retry error, unless during file system support
            // introspection, in which it actually means what it says
        }
        1 => {
            return 3 as i32;
        }
        _ => {
            return sqliteIOErr;
        }
    }
    return unsafe { std::mem::zeroed() };
}

// End of Unique File ID Utility Used By VxWorks ****************
// Posix Advisory Locking ****************************
//
// POSIX advisory locks are broken by design.  ANSI STD 1003.1 (1996)
// section 6.5.2.2 lines 483 through 490 specify that when a process
// sets or clears a lock, that operation overrides any prior locks set
// by the same process.  It does not explicitly say so, but this implies
// that it overrides locks set by the same process using a different
// file descriptor.  Consider this test case:
//
//       int fd1 = open("./file1", O_RDWR|O_CREAT, 0644);
//       int fd2 = open("./file2", O_RDWR|O_CREAT, 0644);
//
// Suppose ./file1 and ./file2 are really the same file (because
// one is a hard or symbolic link to the other) then if you set
// an exclusive lock on fd1, then try to get an exclusive lock
// on fd2, it works.  I would have expected the second lock to
// fail since there was already a lock on the file due to fd1.
// But not so.  Since both locks came from the same process, the
// second overrides the first, even though they were on different
// file descriptors opened on different file names.
//
// This means that we cannot use POSIX locks to synchronize file access
// among competing threads of the same process.  POSIX locks will work fine
// to synchronize access for threads in separate processes, but not
// threads within the same process.
//
// To work around the problem, SQLite has to manage file locks internally
// on its own.  Whenever a new database is opened, we have to find the
// specific inode of the database file (the inode is determined by the
// st_dev and st_ino fields of the stat structure that fstat() fills in)
// and check for locks already existing on that inode.  When locks are
// created or removed, we have to look at our own internal record of the
// locks to see if another thread has previously set a lock on that same
// inode.
//
// (Aside: The use of inode numbers as unique IDs does not work on VxWorks.
// For VxWorks, we have to use the alternative unique ID system based on
// canonical filename and implemented in the previous division.)
//
// The sqlite3_file structure for POSIX is no longer just an integer file
// descriptor.  It is now a structure that holds the integer file
// descriptor and a pointer to a structure that describes the internal
// locks on the corresponding inode.  There is one locking structure
// per inode, so if the same inode is opened twice, both unixFile structures
// point to the same locking structure.  The locking structure keeps
// a reference count (so we will know when to delete it) and a "cnt"
// field that tells us its internal lock status.  cnt==0 means the
// file is unlocked.  cnt==-1 means the file has an exclusive lock.
// cnt>0 means there are cnt shared locks on the file.
//
// Any attempt to lock or unlock a file first checks the locking
// structure.  The fcntl() system call is only invoked to set a
// POSIX lock if the internal lock structure transitions between
// a locked and an unlocked state.
//
// But wait:  there are yet more problems with POSIX advisory locks.
//
// If you close a file descriptor that points to a file that has locks,
// all locks on that file that are owned by the current process are
// released.  To work around this problem, each unixInodeInfo object
// maintains a count of the number of pending locks on the inode.
// When an attempt is made to close an unixFile, if there are
// other unixFile open on the same inode that are holding locks, the call
// to close() the file descriptor is deferred until all of the locks clear.
// The unixInodeInfo structure keeps a list of file descriptors that need to
// be closed and that list is walked (and cleared) when the last lock
// clears.
//
// Yet another problem:  LinuxThreads do not play well with posix locks.
//
// Many older versions of linux use the LinuxThreads library which is
// not posix compliant.  Under LinuxThreads, a lock created by thread
// A cannot be modified or overridden by a different thread B.
// Only thread A can modify the lock.  Locking behavior is correct
// if the application uses the newer Native Posix Thread Library (NPTL)
// on linux - with NPTL a lock created by thread A can override locks
// in thread B.  But there is no way to know at compile-time which
// threading library is being used.  So there is no way to know at
// compile-time whether or not thread A can override locks on thread B.
// One has to do a run-time check to discover the behavior of the
// current process.
//
// SQLite used to support LinuxThreads.  But support for LinuxThreads
// was dropped beginning with version 3.7.0.  SQLite will still work with
// LinuxThreads provided that (1) there is no more than one connection
// per database file in the same process and (2) database connections
// do not move across threads.
// Begin Unique File ID Utility Used By VxWorks ***************
//
// On most versions of unix, we can get a unique ID for a file by concatenating
// the device number and the inode number.  But this does not work on VxWorks.
// On VxWorks, a unique file id must be based on the canonical filename.
//
// A pointer to an instance of the following structure can be used as a
// unique file ID in VxWorks.  Each instance of this structure contains
// a copy of the canonical filename.  There is also a reference count.
// The structure is reclaimed when the number of pointers to it drops to
// zero.
//
// There are never very many files open at one time and lookups are not
// a performance-critical path, so it is sufficient to put these
// structures on a linked list.
/// An instance of the following structure serves as the key used
/// to locate a particular unixInodeInfo object.
#[repr(C)]
#[derive(Clone, Copy)]
struct unixFileId {
    /// Device number
    dev: u64,
    /// We are told that some versions of Android contain a bug that
    /// sizes ino_t at only 32-bits instead of 64-bits. (See
    /// https://android-review.googlesource.com/#/c/115351/3/dist/sqlite3.c)
    /// To work around this, always allocate 64-bits for the inode number.
    /// On small machines that only have 32-bit inodes, this wastes 4 bytes,
    /// but that should not be a big deal.
    ///
    /// WAS:  ino_t ino;
    /// Inode number
    ino: u64,
}

/// An instance of the following structure is allocated for each open
/// inode.
///
/// A single inode can have multiple file descriptors, so each unixFile
/// structure contains a pointer to an instance of this object and this
/// object keeps a count of the number of unixFile pointing to it.
///
/// Mutex rules:
///
///  (1) Only the pLockMutex mutex must be held in order to read or write
///      any of the locking fields:
///          nShared, nLock, eFileLock, bProcessLock, pUnused
///
///  (2) When nRef>0, then the following fields are unchanging and can
///      be read (but not written) without holding any mutex:
///          fileId, pLockMutex
///
///  (3) With the exceptions above, all the fields may only be read
///      or written while holding the global unixBigLock mutex.
///
/// Deadlock prevention:  The global unixBigLock mutex may not
/// be acquired while holding the pLockMutex mutex.  If both unixBigLock
/// and pLockMutex are needed, then unixBigLock must be acquired first.
#[repr(C)]
#[derive(Clone, Copy)]
struct unixInodeInfo {
    /// The lookup key
    fileId: unixFileId,
    /// Hold this mutex for...
    pLockMutex: *mut sqlite3_mutex,
    /// Number of SHARED locks held
    nShared: i32,
    /// Number of outstanding file locks
    nLock: i32,
    /// One of SHARED_LOCK, RESERVED_LOCK etc.
    eFileLock: u8,
    /// An exclusive process lock is held
    bProcessLock: u8,
    /// Unused file descriptors to close
    pUnused: *mut UnixUnusedFd,
    /// Number of pointers to this structure
    nRef: i32,
    /// Shared memory associated with this inode
    pShmNode: *mut unixShmNode,
    /// List of all unixInodeInfo objects
    pNext: *mut unixInodeInfo,
    ///    .... doubly linked
    pPrev: *mut unixInodeInfo,
}

/// A lists of all unixInodeInfo objects.
///
/// Must hold unixBigLock in order to read or write this variable.
/// All unixInodeInfo objects
static mut inodeList: *mut unixInodeInfo = std::ptr::null_mut::<unixInodeInfo>();

// This function - unixLogErrorAtLine(), is only ever called via the macro
// unixLogError().
//
// It is invoked after an error occurs in an OS function and errno has been
// set. It logs a message using sqlite3_log() containing the current value of
// errno and, if possible, the human-readable equivalent from strerror() or
// strerror_r().
//
// The first argument passed to the macro should be the error code that
// will be returned to SQLite (e.g. SQLITE_IOERR_DELETE, SQLITE_CANTOPEN).
// The two subsequent arguments should be the name of the OS function that
// failed (e.g. "unlink", "open") and the associated file-system path,
// if any.
/// # Arguments
///
/// * `errcode` - SQLite error code
/// * `zFunc` - Name of OS function that failed
/// * `zPath` - File path associated with error
/// * `iLine` - Source line number where error occurred
fn unixLogErrorAtLine(
    mut errcode: i32,
    mut zFunc: *const i8,
    mut zPath: *const i8,
    mut iLine: i32,
) -> i32 {
    let mut zErr: *mut i8 = unsafe { std::mem::zeroed() }; // Message from strerror() or equivalent
    let mut iErrno: i32 = unsafe { *unsafe { __errno_location() } }; // Saved syscall error number
    // If this is not a threadsafe build (SQLITE_THREADSAFE==0), then use
    // the strerror() function to obtain the human-readable error message
    // equivalent to errno. Otherwise, use strerror_r().
    // This is a threadsafe build, but strerror_r() is not available.
    zErr = b"\0".as_ptr() as *mut i8;
    if zPath == std::ptr::null::<i8>() {
        zPath = (b"\0".as_ptr() as *mut i8) as *const i8;
    }
    unsafe {
        sqlite3_log(
            errcode,
            (b"os_unix.c:%d: (%d) %s(%s) - %s\0".as_ptr() as *mut i8) as *const i8,
            iLine,
            iErrno,
            zFunc,
            zPath,
            zErr,
        )
    };
    return errcode;
}

/// Close a file descriptor.
///
/// We assume that close() almost always works, since it is only in a
/// very sick application or on a very sick platform that it might fail.
/// If it does fail, simply leak the file descriptor, but do log the
/// error.
///
/// Note that it is not safe to retry close() after EINTR since the
/// file descriptor might have already been reused by another thread.
/// So we don't even try to recover from an EINTR.  Just log the error
/// and move on.
fn robust_close(mut pFile: *mut unixFile, mut h: i32, mut lineno: i32) {
    if (unsafe {
        unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(i32) -> i32>,
            >(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                        .offset((1 as i32) as isize)
                })
                .pCurrent
            })
        }
        .unwrap()(h)
    }) != (0 as i32)
    {
        unixLogErrorAtLine(
            (10 as i32) | (16 as i32) << (8 as i32),
            (b"close\0".as_ptr() as *mut i8) as *const i8,
            if pFile != std::ptr::null_mut::<unixFile>() {
                unsafe { (*pFile).zPath }
            } else {
                std::ptr::null::<i8>()
            },
            lineno,
        );
    }
}

/// Set the pFile->lastErrno.  Do this in a subroutine as that provides
/// a convenient place to set a breakpoint.
fn storeLastErrno(mut pFile: *mut unixFile, mut error: i32) {
    unsafe {
        (*pFile).lastErrno = error;
    }
}

/// Close all file descriptors accumulated in the unixInodeInfo->pUnused list.
fn closePendingFds(mut pFile: *mut unixFile) {
    let mut pInode: *mut unixInodeInfo = unsafe { (*pFile).pInode };
    let mut p: *mut UnixUnusedFd = unsafe { std::mem::zeroed() };
    let mut pNext: *mut UnixUnusedFd = unsafe { std::mem::zeroed() };
    0 as i32;
    p = unsafe { (*pInode).pUnused };
    '__slate_break_1196: while p != std::ptr::null_mut::<UnixUnusedFd>() {
        pNext = unsafe { (*p).pNext };
        robust_close(pFile, unsafe { (*p).fd }, 1478 as i32);
        unsafe { sqlite3_free(p as *mut ()) };
        p = pNext;
    }
    unsafe {
        (*pInode).pUnused = std::ptr::null_mut::<UnixUnusedFd>();
    }
}

/// Release a unixInodeInfo structure previously allocated by findInodeInfo().
///
/// The global mutex must be held when this routine is called, but the mutex
/// on the inode being deleted must NOT be held.
fn releaseInodeInfo(mut pFile: *mut unixFile) {
    let mut pInode: *mut unixInodeInfo = unsafe { (*pFile).pInode };
    0 as i32;
    0 as i32;
    if pInode != std::ptr::null_mut::<unixInodeInfo>() {
        let __v1300: *mut unixInodeInfo = pInode;
        let __v1301: i32 = unsafe { (*__v1300).nRef };
        let __v1302: i32 = __v1301 - (1 as i32);
        unsafe {
            (*__v1300).nRef = __v1302;
        }
        if (unsafe { (*pInode).nRef }) == (0 as i32) {
            0 as i32;
            unsafe { sqlite3_mutex_enter(unsafe { (*pInode).pLockMutex }) };
            closePendingFds(pFile);
            unsafe { sqlite3_mutex_leave(unsafe { (*pInode).pLockMutex }) };
            if (unsafe { (*pInode).pPrev }) != std::ptr::null_mut::<unixInodeInfo>() {
                0 as i32;
                unsafe {
                    (*unsafe { (*pInode).pPrev }).pNext = unsafe { (*pInode).pNext };
                }
            } else {
                0 as i32;
                unsafe {
                    inodeList = unsafe { (*pInode).pNext };
                }
            }
            if (unsafe { (*pInode).pNext }) != std::ptr::null_mut::<unixInodeInfo>() {
                0 as i32;
                unsafe {
                    (*unsafe { (*pInode).pNext }).pPrev = unsafe { (*pInode).pPrev };
                }
            }
            unsafe { sqlite3_mutex_free(unsafe { (*pInode).pLockMutex }) };
            unsafe { sqlite3_free(pInode as *mut ()) };
        }
    }
}

/// Given a file descriptor, locate the unixInodeInfo object that
/// describes that file descriptor.  Create a new one if necessary.  The
/// return value might be uninitialized if an error occurs.
///
/// The global mutex must held when calling this routine.
///
/// Return an appropriate error code.
///
/// # Arguments
///
/// * `pFile` - Unix file with file desc used in the key
/// * `ppInode` - Return the unixInodeInfo object here
fn findInodeInfo(mut pFile: *mut unixFile, mut ppInode: *mut *mut unixInodeInfo) -> i32 {
    let mut rc: i32 = 0 as i32; // System call return code
    let mut fd: i32 = 0 as i32; // The file descriptor for pFile
    let mut fileId: unixFileId = unsafe { std::mem::zeroed() }; // Lookup key for the unixInodeInfo
    let mut statbuf: stat = unsafe { std::mem::zeroed() }; // Low-level file information
    let mut pInode: *mut unixInodeInfo = std::ptr::null_mut::<unixInodeInfo>(); // Candidate unixInodeInfo object
    0 as i32;
    // Get low-level information about the file that we can used to
    // create a unique name for the file.
    fd = unsafe { (*pFile).h };
    rc = unsafe {
        unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(i32, *mut stat) -> i32>,
            >(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                        .offset((5 as i32) as isize)
                })
                .pCurrent
            })
        }
        .unwrap()(fd, std::ptr::addr_of_mut!(statbuf))
    };
    if rc != (0 as i32) {
        storeLastErrno(pFile, unsafe { *unsafe { __errno_location() } });
        return 10 as i32;
    }
    unsafe {
        memset(
            std::ptr::addr_of_mut!(fileId) as *mut (),
            0 as i32,
            16 as u64,
        )
    };
    fileId.dev = statbuf.st_dev;
    fileId.ino = statbuf.st_ino;
    0 as i32;
    pInode = unsafe { inodeList };
    '__slate_break_1197: while pInode != std::ptr::null_mut::<unixInodeInfo>()
        && (unsafe {
            memcmp(
                std::ptr::addr_of_mut!(fileId) as *const (),
                (unsafe { std::ptr::addr_of_mut!((*pInode).fileId) }) as *const (),
                16 as u64,
            )
        }) != (0 as i32)
    {
        pInode = unsafe { (*pInode).pNext };
    }
    if pInode == std::ptr::null_mut::<unixInodeInfo>() {
        pInode = (unsafe { sqlite3_malloc64(80 as u64) }) as *mut unixInodeInfo;
        if pInode == std::ptr::null_mut::<unixInodeInfo>() {
            return 7 as i32;
        }
        unsafe { memset(pInode as *mut (), 0 as i32, 80 as u64) };
        unsafe {
            memcpy(
                (unsafe { std::ptr::addr_of_mut!((*pInode).fileId) }) as *mut (),
                std::ptr::addr_of_mut!(fileId) as *const (),
                16 as u64,
            )
        };
        if (unsafe { sqlite3Config.bCoreMutex }) != (0 as u8) {
            unsafe {
                (*pInode).pLockMutex = unsafe { sqlite3_mutex_alloc(0 as i32) };
            }
            if (unsafe { (*pInode).pLockMutex }) == std::ptr::null_mut::<sqlite3_mutex>() {
                unsafe { sqlite3_free(pInode as *mut ()) };
                return 7 as i32;
            }
        }
        unsafe {
            (*pInode).nRef = 1 as i32;
        }
        0 as i32;
        unsafe {
            (*pInode).pNext = unsafe { inodeList };
        }
        unsafe {
            (*pInode).pPrev = std::ptr::null_mut::<unixInodeInfo>();
        }
        if (unsafe { inodeList }) != std::ptr::null_mut::<unixInodeInfo>() {
            unsafe {
                (*unsafe { inodeList }).pPrev = pInode;
            }
        }
        unsafe {
            inodeList = pInode;
        }
    } else {
        let __v1303: *mut unixInodeInfo = pInode;
        let __v1304: i32 = unsafe { (*__v1303).nRef };
        let __v1305: i32 = __v1304 + (1 as i32);
        unsafe {
            (*__v1303).nRef = __v1305;
        }
    }
    unsafe {
        *ppInode = pInode;
    }
    return 0 as i32;
}

/// Return TRUE if pFile has been renamed or unlinked since it was first opened.
fn fileHasMoved(mut pFile: *mut unixFile) -> i32 {
    let mut buf: stat = unsafe { std::mem::zeroed() };
    let __v1306: bool;
    if (unsafe { (*pFile).pInode }) != std::ptr::null_mut::<unixInodeInfo>() {
        __v1306 = (unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(*const i8, *mut stat) -> i32>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((4 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(unsafe { (*pFile).zPath }, std::ptr::addr_of_mut!(buf))
        }) != (0 as i32)
            || buf.st_ino != unsafe { (*unsafe { (*pFile).pInode }).fileId.ino };
    } else {
        __v1306 = false as bool;
    }
    return __v1306 as i32;
}

/// Check a unixFile that is a database.  Verify the following:
///
/// (1) There is exactly one hard link on the file
/// (2) The file is not a symbolic link
/// (3) The file has not been renamed or unlinked
///
/// Issue sqlite3_log(SQLITE_WARNING,...) messages if anything is not right.
fn verifyDbFile(mut pFile: *mut unixFile) {
    let mut buf: stat = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    // These verifications occurs for the main database only
    if (((unsafe { (*pFile).ctrlFlags }) as u32) as i32) & (128 as i32) != (0 as i32) {
        return;
    }
    rc = unsafe {
        unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(i32, *mut stat) -> i32>,
            >(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                        .offset((5 as i32) as isize)
                })
                .pCurrent
            })
        }
        .unwrap()(unsafe { (*pFile).h }, std::ptr::addr_of_mut!(buf))
    };
    if rc != (0 as i32) {
        unsafe {
            sqlite3_log(
                28 as i32,
                (b"cannot fstat db file %s\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pFile).zPath },
            )
        };
        return;
    }
    if buf.st_nlink == (((0 as i32) as i64) as u64) {
        unsafe {
            sqlite3_log(
                28 as i32,
                (b"file unlinked while open: %s\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pFile).zPath },
            )
        };
        return;
    }
    if buf.st_nlink > (((1 as i32) as i64) as u64) {
        unsafe {
            sqlite3_log(
                28 as i32,
                (b"multiple links to file: %s\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pFile).zPath },
            )
        };
        return;
    }
    if fileHasMoved(pFile) != (0 as i32) {
        unsafe {
            sqlite3_log(
                28 as i32,
                (b"file renamed while open: %s\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pFile).zPath },
            )
        };
        return;
    }
}

/// This routine checks if there is a RESERVED lock held on the specified
/// file by this or any other process. If such a lock is held, set *pResOut
/// to a non-zero value otherwise *pResOut is set to zero.  The return value
/// is set to SQLITE_OK unless an I/O error occurs during lock checking.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixCheckReservedLock")]
extern "C-unwind" fn unixCheckReservedLock(
    mut id: *mut sqlite3_file,
    mut pResOut: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut reserved: i32 = 0 as i32;
    let mut pFile: *mut unixFile = id as *mut unixFile;
    {}
    0 as i32;
    0 as i32;
    unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*pFile).pInode }).pLockMutex }) };
    // Check if a thread in this process holds such a lock
    if (((unsafe { (*unsafe { (*pFile).pInode }).eFileLock }) as u32) as i32) > (1 as i32) {
        reserved = 1 as i32;
    }
    // Otherwise see if some other process holds it.
    if !(reserved != (0 as i32))
        && !((unsafe { (*unsafe { (*pFile).pInode }).bProcessLock }) != (0 as u8))
    {
        let mut lock: flock = unsafe { std::mem::zeroed() };
        lock.l_whence = (0 as i32) as i16;
        lock.l_start = ((unsafe { sqlite3PendingByte }) + (1 as i32)) as i64;
        lock.l_len = (1 as i32) as i64;
        lock.l_type = (1 as i32) as i16;
        if (unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(i32, i32, ...) -> i32>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((7 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(
                unsafe { (*pFile).h },
                5 as i32,
                std::ptr::addr_of_mut!(lock),
            )
        }) != (0 as i32)
        {
            rc = (10 as i32) | (14 as i32) << (8 as i32);
            storeLastErrno(pFile, unsafe { *unsafe { __errno_location() } });
        } else {
            if (lock.l_type as i32) != (2 as i32) {
                reserved = 1 as i32;
            }
        }
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*pFile).pInode }).pLockMutex }) };
    {}
    unsafe {
        *pResOut = reserved;
    }
    return rc;
}

// Set a posix-advisory-lock.
//
// There are two versions of this routine.  If compiled with
// SQLITE_ENABLE_SETLK_TIMEOUT then the routine has an extra parameter
// which is a pointer to a unixFile.  If the unixFile->iBusyTimeout
// value is set, then it is the number of milliseconds to wait before
// failing the lock.  The iBusyTimeout value is always reset back to
// zero on each call.
//
// If SQLITE_ENABLE_SETLK_TIMEOUT is not defined, then do a non-blocking
// attempt to set the lock.
/// Attempt to set a system-lock on the file pFile.  The lock is
/// described by pLock.
///
/// If the pFile was opened read/write from unix-excl, then the only lock
/// ever obtained is an exclusive lock, and it is obtained exactly once
/// the first time any lock is attempted.  All subsequent system locking
/// operations become no-ops.  Locking operations still happen internally,
/// in order to coordinate access between separate database connections
/// within this process, but all of that is handled in memory and the
/// operating system does not participate.
///
/// This function is a pass-through to fcntl(F_SETLK) if pFile is using
/// any VFS other than "unix-excl" or if pFile is opened on "unix-excl"
/// and is read-only.
///
/// Zero is returned if the call completes successfully, or -1 if a call
/// to fcntl() fails. In this case, errno is set appropriately (by fcntl()).
fn unixFileLock(mut pFile: *mut unixFile, mut pLock: *mut flock) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pInode: *mut unixInodeInfo = unsafe { (*pFile).pInode };
    0 as i32;
    0 as i32;
    if (((unsafe { (*pFile).ctrlFlags }) as u32) as i32) & ((1 as i32) | (2 as i32)) == (1 as i32) {
        if (((unsafe { (*pInode).bProcessLock }) as u32) as i32) == (0 as i32) {
            let mut lock: flock = unsafe { std::mem::zeroed() };
            // assert( pInode->nLock==0 ); <-- Not true if unix-excl READONLY used
            lock.l_whence = (0 as i32) as i16;
            lock.l_start = ((unsafe { sqlite3PendingByte }) + (2 as i32)) as i64;
            lock.l_len = (510 as i32) as i64;
            lock.l_type = (1 as i32) as i16;
            rc = unsafe {
                unsafe {
                    std::mem::transmute::<
                        Option<unsafe extern "C-unwind" fn()>,
                        Option<unsafe extern "C-unwind" fn(i32, i32, ...) -> i32>,
                    >(unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset((7 as i32) as isize)
                        })
                        .pCurrent
                    })
                }
                .unwrap()(
                    unsafe { (*pFile).h },
                    6 as i32,
                    std::ptr::addr_of_mut!(lock),
                )
            };
            if rc < (0 as i32) {
                return rc;
            }
            unsafe {
                (*pInode).bProcessLock = ((1 as i32) as i8) as u8;
            }
            let __v1307: *mut unixInodeInfo = pInode;
            let __v1308: i32 = unsafe { (*__v1307).nLock };
            let __v1309: i32 = __v1308 + (1 as i32);
            unsafe {
                (*__v1307).nLock = __v1309;
            }
        } else {
            rc = 0 as i32;
        }
    } else {
        rc = unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(i32, i32, ...) -> i32>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((7 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(unsafe { (*pFile).h }, 6 as i32, pLock)
        };
    }
    return rc;
}

/// Lock the file with the lock specified by parameter eFileLock - one
/// of the following:
///
///     (1) SHARED_LOCK
///     (2) RESERVED_LOCK
///     (3) PENDING_LOCK
///     (4) EXCLUSIVE_LOCK
///
/// Sometimes when requesting one lock state, additional lock states
/// are inserted in between.  The locking might fail on one of the later
/// transitions leaving the lock state different from what it started but
/// still short of its goal.  The following chart shows the allowed
/// transitions and the inserted intermediate states:
///
///    UNLOCKED -> SHARED
///    SHARED -> RESERVED
///    SHARED -> EXCLUSIVE
///    RESERVED -> (PENDING) -> EXCLUSIVE
///    PENDING -> EXCLUSIVE
///
/// This routine will only increase a lock.  Use the sqlite3OsUnlock()
/// routine to lower a locking level.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixLock")]
extern "C-unwind" fn unixLock(mut id: *mut sqlite3_file, mut eFileLock: i32) -> i32 {
    let mut __slate_storage_1318: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1318: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1318) as *mut i32;
    let mut __slate_storage_1317: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1317: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1317) as *mut i32;
    let mut __slate_storage_1316: std::mem::MaybeUninit<*mut unixInodeInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1316: *mut *mut unixInodeInfo =
        std::ptr::addr_of_mut!(__slate_storage_1316) as *mut *mut unixInodeInfo;
    let mut __slate_storage_1315: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1315: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1315) as *mut i32;
    let mut __slate_storage_1314: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1314: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1314) as *mut i32;
    let mut __slate_storage_1313: std::mem::MaybeUninit<*mut unixInodeInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1313: *mut *mut unixInodeInfo =
        std::ptr::addr_of_mut!(__slate_storage_1313) as *mut *mut unixInodeInfo;
    let mut __slate_storage_1312: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1312: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1312) as *mut i32;
    let mut __slate_storage_1311: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1311: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1311) as *mut i32;
    let mut __slate_storage_1310: std::mem::MaybeUninit<*mut unixInodeInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1310: *mut *mut unixInodeInfo =
        std::ptr::addr_of_mut!(__slate_storage_1310) as *mut *mut unixInodeInfo;
    let mut __slate_storage_600: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_600: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_600) as *mut i32;
    let mut __slate_storage_599: std::mem::MaybeUninit<flock> = std::mem::MaybeUninit::uninit();
    let __slate_slot_599: *mut flock = std::ptr::addr_of_mut!(__slate_storage_599) as *mut flock;
    let mut __slate_storage_598: std::mem::MaybeUninit<*mut unixInodeInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_598: *mut *mut unixInodeInfo =
        std::ptr::addr_of_mut!(__slate_storage_598) as *mut *mut unixInodeInfo;
    let mut __slate_storage_597: std::mem::MaybeUninit<*mut unixFile> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_597: *mut *mut unixFile =
        std::ptr::addr_of_mut!(__slate_storage_597) as *mut *mut unixFile;
    // The following describes the implementation of the various locks and
    // lock transitions in terms of the POSIX advisory shared and exclusive
    // lock primitives (called read-locks and write-locks below, to avoid
    // confusion with SQLite lock names). The algorithms are complicated
    // slightly in order to be compatible with Windows95 systems simultaneously
    // accessing the same database file, in case that is ever required.
    //
    // Symbols defined in os.h identify the 'pending byte' and the 'reserved
    // byte', each single bytes at well known offsets, and the 'shared byte
    // range', a range of 510 bytes at a well known offset.
    //
    // To obtain a SHARED lock, a read-lock is obtained on the 'pending
    // byte'.  If this is successful, 'shared byte range' is read-locked
    // and the lock on the 'pending byte' released.  (Legacy note:  When
    // SQLite was first developed, Windows95 systems were still very common,
    // and Windows95 lacks a shared-lock capability.  So on Windows95, a
    // single randomly selected by from the 'shared byte range' is locked.
    // Windows95 is now pretty much extinct, but this work-around for the
    // lack of shared-locks on Windows95 lives on, for backwards
    // compatibility.)
    //
    // A process may only obtain a RESERVED lock after it has a SHARED lock.
    // A RESERVED lock is implemented by grabbing a write-lock on the
    // 'reserved byte'.
    //
    // An EXCLUSIVE lock may only be requested after either a SHARED or
    // RESERVED lock is held. An EXCLUSIVE lock is implemented by obtaining
    // a write-lock on the entire 'shared byte range'. Since all other locks
    // require a read-lock on one of the bytes within this range, this ensures
    // that no other locks are held on the database.
    //
    // If a process that holds a RESERVED lock requests an EXCLUSIVE, then
    // a PENDING lock is obtained first. A PENDING lock is implemented by
    // obtaining a write-lock on the 'pending byte'. This ensures that no new
    // SHARED locks can be obtained, but existing SHARED locks are allowed to
    // persist. If the call to this function fails to obtain the EXCLUSIVE
    // lock in this case, it holds the PENDING lock instead. The client may
    // then re-attempt the EXCLUSIVE lock later on, after existing SHARED
    // locks have cleared.
    let mut __slate_storage_596: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_596: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_596) as *mut i32;
    unsafe {
        std::ptr::write(__slate_slot_596, 0 as i32);
        std::ptr::write(__slate_slot_597, id as *mut unixFile);
        std::ptr::write(__slate_slot_600, 0 as i32);
        0 as i32;
        {}
        // If there is already a lock of this type or more restrictive on the
        // unixFile, do nothing. Don't use the end_lock: exit path, as
        // unixEnterMutex() hasn't been called yet.
        if (((unsafe { (*(*__slate_slot_597)).eFileLock }) as u32) as i32) >= eFileLock {
            {}
            return 0 as i32;
        } else {
            '__join_0: {
                // Make sure the locking sequence is correct.
                // (1) We never move from unlocked to anything higher than shared lock.
                // (2) SQLite never explicitly requests a pending lock.
                // (3) A shared lock is always held when a reserve lock is requested.
                0 as i32;
                0 as i32;
                0 as i32;
                // This mutex is needed because pFile->pInode is shared across threads
                *__slate_slot_598 = unsafe { (*(*__slate_slot_597)).pInode };
                unsafe { sqlite3_mutex_enter(unsafe { (*(*__slate_slot_598)).pLockMutex }) };
                // If some thread using this PID has a lock via a different unixFile*
                // handle that precludes the requested lock, return BUSY.
                if (((unsafe { (*(*__slate_slot_597)).eFileLock }) as u32) as i32)
                    != (((unsafe { (*(*__slate_slot_598)).eFileLock }) as u32) as i32)
                    && ((((unsafe { (*(*__slate_slot_598)).eFileLock }) as u32) as i32)
                        >= (3 as i32)
                        || eFileLock > (1 as i32))
                {
                    *__slate_slot_596 = 5 as i32;
                } else {
                    // If a SHARED lock is requested, and some thread using this PID already
                    // has a SHARED or RESERVED lock, then increment reference counts and
                    // return SQLITE_OK.
                    if eFileLock == (1 as i32)
                        && ((((unsafe { (*(*__slate_slot_598)).eFileLock }) as u32) as i32)
                            == (1 as i32)
                            || (((unsafe { (*(*__slate_slot_598)).eFileLock }) as u32) as i32)
                                == (2 as i32))
                    {
                        0 as i32;
                        0 as i32;
                        0 as i32;
                        unsafe {
                            (*(*__slate_slot_597)).eFileLock = ((1 as i32) as i8) as u8;
                        }
                        std::ptr::write(__slate_slot_1310, *__slate_slot_598);
                        std::ptr::write(__slate_slot_1311, unsafe {
                            (*(*__slate_slot_1310)).nShared
                        });
                        std::ptr::write(__slate_slot_1312, *__slate_slot_1311 + (1 as i32));
                        unsafe {
                            (*(*__slate_slot_1310)).nShared = *__slate_slot_1312;
                        }
                        std::ptr::write(__slate_slot_1313, *__slate_slot_598);
                        std::ptr::write(__slate_slot_1314, unsafe {
                            (*(*__slate_slot_1313)).nLock
                        });
                        std::ptr::write(__slate_slot_1315, *__slate_slot_1314 + (1 as i32));
                        unsafe {
                            (*(*__slate_slot_1313)).nLock = *__slate_slot_1315;
                        }
                    } else {
                        // A PENDING lock is needed before acquiring a SHARED lock and before
                        // acquiring an EXCLUSIVE lock.  For the SHARED lock, the PENDING will
                        // be released.
                        (*__slate_slot_599).l_len = 1 as i64;
                        (*__slate_slot_599).l_whence = (0 as i32) as i16;
                        if eFileLock == (1 as i32)
                            || eFileLock == (4 as i32)
                                && (((unsafe { (*(*__slate_slot_597)).eFileLock }) as u32) as i32)
                                    == (2 as i32)
                        {
                            (*__slate_slot_599).l_type = (if eFileLock == (1 as i32) {
                                0 as i32
                            } else {
                                1 as i32
                            }) as i16;
                            (*__slate_slot_599).l_start = (unsafe { sqlite3PendingByte }) as i64;
                            if unixFileLock(
                                *__slate_slot_597,
                                std::ptr::addr_of_mut!(*__slate_slot_599),
                            ) != (0 as i32)
                            {
                                *__slate_slot_600 = unsafe { *unsafe { __errno_location() } };
                                *__slate_slot_596 = sqliteErrorFromPosixError(
                                    *__slate_slot_600,
                                    (10 as i32) | (15 as i32) << (8 as i32),
                                );
                                if *__slate_slot_596 != (5 as i32) {
                                    storeLastErrno(*__slate_slot_597, *__slate_slot_600);
                                    break '__join_0;
                                } else {
                                    break '__join_0;
                                }
                            } else {
                                if eFileLock == (4 as i32) {
                                    unsafe {
                                        (*(*__slate_slot_597)).eFileLock = ((3 as i32) as i8) as u8;
                                    }
                                    unsafe {
                                        (*(*__slate_slot_598)).eFileLock = ((3 as i32) as i8) as u8;
                                    }
                                }
                            }
                        }
                        // If control gets to this point, then actually go ahead and make
                        // operating system calls for the specified lock.
                        if eFileLock == (1 as i32) {
                            0 as i32;
                            0 as i32;
                            0 as i32;
                            // Now get the read-lock
                            (*__slate_slot_599).l_start =
                                ((unsafe { sqlite3PendingByte }) + (2 as i32)) as i64;
                            (*__slate_slot_599).l_len = (510 as i32) as i64;
                            if unixFileLock(
                                *__slate_slot_597,
                                std::ptr::addr_of_mut!(*__slate_slot_599),
                            ) != (0 as i32)
                            {
                                *__slate_slot_600 = unsafe { *unsafe { __errno_location() } };
                                *__slate_slot_596 = sqliteErrorFromPosixError(
                                    *__slate_slot_600,
                                    (10 as i32) | (15 as i32) << (8 as i32),
                                );
                            }
                            // Drop the temporary PENDING lock
                            (*__slate_slot_599).l_start = (unsafe { sqlite3PendingByte }) as i64;
                            (*__slate_slot_599).l_len = 1 as i64;
                            (*__slate_slot_599).l_type = (2 as i32) as i16;
                            if unixFileLock(
                                *__slate_slot_597,
                                std::ptr::addr_of_mut!(*__slate_slot_599),
                            ) != (0 as i32)
                                && *__slate_slot_596 == (0 as i32)
                            {
                                // This could happen with a network mount
                                *__slate_slot_600 = unsafe { *unsafe { __errno_location() } };
                                *__slate_slot_596 = (10 as i32) | (8 as i32) << (8 as i32);
                            }
                            if *__slate_slot_596 != (0 as i32) {
                                if *__slate_slot_596 != (5 as i32) {
                                    storeLastErrno(*__slate_slot_597, *__slate_slot_600);
                                    break '__join_0;
                                } else {
                                    break '__join_0;
                                }
                            } else {
                                unsafe {
                                    (*(*__slate_slot_597)).eFileLock = ((1 as i32) as i8) as u8;
                                }
                                std::ptr::write(__slate_slot_1316, *__slate_slot_598);
                                std::ptr::write(__slate_slot_1317, unsafe {
                                    (*(*__slate_slot_1316)).nLock
                                });
                                std::ptr::write(__slate_slot_1318, *__slate_slot_1317 + (1 as i32));
                                unsafe {
                                    (*(*__slate_slot_1316)).nLock = *__slate_slot_1318;
                                }
                                unsafe {
                                    (*(*__slate_slot_598)).nShared = 1 as i32;
                                }
                            }
                        } else {
                            if eFileLock == (4 as i32)
                                && (unsafe { (*(*__slate_slot_598)).nShared }) > (1 as i32)
                            {
                                // We are trying for an exclusive lock but another thread in this
                                // same process is still holding a shared lock.
                                *__slate_slot_596 = 5 as i32;
                            } else {
                                if unixIsSharingShmNode(*__slate_slot_597) != (0 as i32) {
                                    // We are in WAL mode and attempting to delete the SHM and WAL
                                    // files due to closing the connection or changing out of WAL mode,
                                    // but another process still holds locks on the SHM file, thus
                                    // indicating that database locks have been broken, perhaps due
                                    // to a rogue close(open(dbFile)) or similar.
                                    *__slate_slot_596 = 5 as i32;
                                } else {
                                    // The request was for a RESERVED or EXCLUSIVE lock.  It is
                                    // assumed that there is a SHARED or greater lock on the file
                                    // already.
                                    0 as i32;
                                    (*__slate_slot_599).l_type = (1 as i32) as i16;
                                    0 as i32;
                                    if eFileLock == (2 as i32) {
                                        (*__slate_slot_599).l_start =
                                            ((unsafe { sqlite3PendingByte }) + (1 as i32)) as i64;
                                        (*__slate_slot_599).l_len = 1 as i64;
                                    } else {
                                        (*__slate_slot_599).l_start =
                                            ((unsafe { sqlite3PendingByte }) + (2 as i32)) as i64;
                                        (*__slate_slot_599).l_len = (510 as i32) as i64;
                                    }
                                    if unixFileLock(
                                        *__slate_slot_597,
                                        std::ptr::addr_of_mut!(*__slate_slot_599),
                                    ) != (0 as i32)
                                    {
                                        *__slate_slot_600 =
                                            unsafe { *unsafe { __errno_location() } };
                                        *__slate_slot_596 = sqliteErrorFromPosixError(
                                            *__slate_slot_600,
                                            (10 as i32) | (15 as i32) << (8 as i32),
                                        );
                                        if *__slate_slot_596 != (5 as i32) {
                                            storeLastErrno(*__slate_slot_597, *__slate_slot_600);
                                        }
                                    }
                                }
                            }
                        }
                        if *__slate_slot_596 == (0 as i32) {
                            unsafe {
                                (*(*__slate_slot_597)).eFileLock = (eFileLock as i8) as u8;
                            }
                            unsafe {
                                (*(*__slate_slot_598)).eFileLock = (eFileLock as i8) as u8;
                            }
                        }
                    }
                }
            }
            unsafe { sqlite3_mutex_leave(unsafe { (*(*__slate_slot_598)).pLockMutex }) };
            {}
            return *__slate_slot_596;
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Add the file descriptor used by file handle pFile to the corresponding
/// pUnused list.
fn setPendingFd(mut pFile: *mut unixFile) {
    let mut pInode: *mut unixInodeInfo = unsafe { (*pFile).pInode };
    let mut p: *mut UnixUnusedFd = unsafe { (*pFile).pPreallocatedUnused };
    0 as i32;
    unsafe {
        (*p).pNext = unsafe { (*pInode).pUnused };
    }
    unsafe {
        (*pInode).pUnused = p;
    }
    unsafe {
        (*pFile).h = -(1 as i32);
    }
    unsafe {
        (*pFile).pPreallocatedUnused = std::ptr::null_mut::<UnixUnusedFd>();
    }
}

/// Lower the locking level on file descriptor pFile to eFileLock.  eFileLock
/// must be either NO_LOCK or SHARED_LOCK.
///
/// If the locking level of the file descriptor is already at or below
/// the requested locking level, this routine is a no-op.
///
/// If handleNFSUnlock is true, then on downgrading an EXCLUSIVE_LOCK to SHARED
/// the byte range is divided into 2 parts and the first part is unlocked then
/// set to a read lock, then the other part is simply unlocked.  This works
/// around a bug in BSD NFS lockd (also seen on MacOSX 10.3+) that fails to
/// remove the write lock on a region when a read lock is set.
fn posixUnlock(mut id: *mut sqlite3_file, mut eFileLock: i32, mut handleNFSUnlock: i32) -> i32 {
    let mut __slate_storage_1324: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1324: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1324) as *mut i32;
    let mut __slate_storage_1323: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1323: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1323) as *mut i32;
    // Decrement the count of locks against this same file.  When the
    // count reaches zero, close any other file descriptors whose close
    // was deferred because of outstanding locks.
    let mut __slate_storage_1322: std::mem::MaybeUninit<*mut unixInodeInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1322: *mut *mut unixInodeInfo =
        std::ptr::addr_of_mut!(__slate_storage_1322) as *mut *mut unixInodeInfo;
    let mut __slate_storage_1321: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1321: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1321) as *mut i32;
    let mut __slate_storage_1320: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1320: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1320) as *mut i32;
    // Decrement the shared lock counter.  Release the lock using an
    // OS call only when all threads in this same process have released
    // the lock.
    let mut __slate_storage_1319: std::mem::MaybeUninit<*mut unixInodeInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1319: *mut *mut unixInodeInfo =
        std::ptr::addr_of_mut!(__slate_storage_1319) as *mut *mut unixInodeInfo;
    let mut __slate_storage_613: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_613: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_613) as *mut i32;
    let mut __slate_storage_612: std::mem::MaybeUninit<flock> = std::mem::MaybeUninit::uninit();
    let __slate_slot_612: *mut flock = std::ptr::addr_of_mut!(__slate_storage_612) as *mut flock;
    let mut __slate_storage_611: std::mem::MaybeUninit<*mut unixInodeInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_611: *mut *mut unixInodeInfo =
        std::ptr::addr_of_mut!(__slate_storage_611) as *mut *mut unixInodeInfo;
    let mut __slate_storage_610: std::mem::MaybeUninit<*mut unixFile> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_610: *mut *mut unixFile =
        std::ptr::addr_of_mut!(__slate_storage_610) as *mut *mut unixFile;
    unsafe {
        std::ptr::write(__slate_slot_610, id as *mut unixFile);
        std::ptr::write(__slate_slot_613, 0 as i32);
        0 as i32;
        {}
        0 as i32;
        if (((unsafe { (*(*__slate_slot_610)).eFileLock }) as u32) as i32) <= eFileLock {
            return 0 as i32;
        } else {
            '__join_2: {
                *__slate_slot_611 = unsafe { (*(*__slate_slot_610)).pInode };
                unsafe { sqlite3_mutex_enter(unsafe { (*(*__slate_slot_611)).pLockMutex }) };
                0 as i32;
                if (((unsafe { (*(*__slate_slot_610)).eFileLock }) as u32) as i32) > (1 as i32) {
                    0 as i32;
                    // downgrading to a shared lock on NFS involves clearing the write lock
                    // before establishing the readlock - to avoid a race condition we downgrade
                    // the lock in 2 blocks, so that part of the range will be covered by a
                    // write lock until the rest is covered by a read lock:
                    //  1:   [WWWWW]
                    //  2:   [....W]
                    //  3:   [RRRRW]
                    //  4:   [RRRR.]
                    if eFileLock == (1 as i32) {
                        handleNFSUnlock;
                        0 as i32;
                        (*__slate_slot_612).l_type = (0 as i32) as i16;
                        (*__slate_slot_612).l_whence = (0 as i32) as i16;
                        (*__slate_slot_612).l_start =
                            ((unsafe { sqlite3PendingByte }) + (2 as i32)) as i64;
                        (*__slate_slot_612).l_len = (510 as i32) as i64;
                        if unixFileLock(
                            *__slate_slot_610,
                            std::ptr::addr_of_mut!(*__slate_slot_612),
                        ) != (0 as i32)
                        {
                            // In theory, the call to unixFileLock() cannot fail because another
                            // process is holding an incompatible lock. If it does, this
                            // indicates that the other process is not following the locking
                            // protocol. If this happens, return SQLITE_IOERR_RDLOCK. Returning
                            // SQLITE_BUSY would confuse the upper layer (in practice it causes
                            // an assert to fail).
                            *__slate_slot_613 = (10 as i32) | (9 as i32) << (8 as i32);
                            storeLastErrno(*__slate_slot_610, unsafe {
                                *unsafe { __errno_location() }
                            });
                            break '__join_2;
                        }
                    }
                    (*__slate_slot_612).l_type = (2 as i32) as i16;
                    (*__slate_slot_612).l_whence = (0 as i32) as i16;
                    (*__slate_slot_612).l_start = (unsafe { sqlite3PendingByte }) as i64;
                    (*__slate_slot_612).l_len = 2 as i64;
                    0 as i32;
                    if unixFileLock(*__slate_slot_610, std::ptr::addr_of_mut!(*__slate_slot_612))
                        == (0 as i32)
                    {
                        unsafe {
                            (*(*__slate_slot_611)).eFileLock = ((1 as i32) as i8) as u8;
                        }
                    } else {
                        *__slate_slot_613 = (10 as i32) | (8 as i32) << (8 as i32);
                        storeLastErrno(*__slate_slot_610, unsafe {
                            *unsafe { __errno_location() }
                        });
                        break '__join_2;
                    }
                }
                if eFileLock == (0 as i32) {
                    std::ptr::write(__slate_slot_1319, *__slate_slot_611);
                    std::ptr::write(__slate_slot_1320, unsafe {
                        (*(*__slate_slot_1319)).nShared
                    });
                    std::ptr::write(__slate_slot_1321, *__slate_slot_1320 - (1 as i32));
                    unsafe {
                        (*(*__slate_slot_1319)).nShared = *__slate_slot_1321;
                    }
                    if (unsafe { (*(*__slate_slot_611)).nShared }) == (0 as i32) {
                        (*__slate_slot_612).l_type = (2 as i32) as i16;
                        (*__slate_slot_612).l_whence = (0 as i32) as i16;
                        (*__slate_slot_612).l_len = 0 as i64;
                        (*__slate_slot_612).l_start = 0 as i64;
                        if unixFileLock(
                            *__slate_slot_610,
                            std::ptr::addr_of_mut!(*__slate_slot_612),
                        ) == (0 as i32)
                        {
                            unsafe {
                                (*(*__slate_slot_611)).eFileLock = ((0 as i32) as i8) as u8;
                            }
                        } else {
                            *__slate_slot_613 = (10 as i32) | (8 as i32) << (8 as i32);
                            storeLastErrno(*__slate_slot_610, unsafe {
                                *unsafe { __errno_location() }
                            });
                            unsafe {
                                (*(*__slate_slot_611)).eFileLock = ((0 as i32) as i8) as u8;
                            }
                            unsafe {
                                (*(*__slate_slot_610)).eFileLock = ((0 as i32) as i8) as u8;
                            }
                        }
                    }
                    std::ptr::write(__slate_slot_1322, *__slate_slot_611);
                    std::ptr::write(__slate_slot_1323, unsafe { (*(*__slate_slot_1322)).nLock });
                    std::ptr::write(__slate_slot_1324, *__slate_slot_1323 - (1 as i32));
                    unsafe {
                        (*(*__slate_slot_1322)).nLock = *__slate_slot_1324;
                    }
                    0 as i32;
                    if (unsafe { (*(*__slate_slot_611)).nLock }) == (0 as i32) {
                        closePendingFds(*__slate_slot_610);
                    }
                }
            }
            unsafe { sqlite3_mutex_leave(unsafe { (*(*__slate_slot_611)).pLockMutex }) };
            if *__slate_slot_613 == (0 as i32) {
                unsafe {
                    (*(*__slate_slot_610)).eFileLock = (eFileLock as i8) as u8;
                }
            }
            return *__slate_slot_613;
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Lower the locking level on file descriptor pFile to eFileLock.  eFileLock
/// must be either NO_LOCK or SHARED_LOCK.
///
/// If the locking level of the file descriptor is already at or below
/// the requested locking level, this routine is a no-op.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixUnlock")]
extern "C-unwind" fn unixUnlock(mut id: *mut sqlite3_file, mut eFileLock: i32) -> i32 {
    0 as i32;
    return posixUnlock(id, eFileLock, 0 as i32);
}

/// This function performs the parts of the "close file" operation
/// common to all locking schemes. It closes the directory and file
/// handles, if they are valid, and sets all fields of the unixFile
/// structure to 0.
///
/// It is *not* necessary to hold the mutex when this routine is called,
/// even on VxWorks.  A mutex will be acquired on VxWorks by the
/// vxworksReleaseFileId() routine.
fn closeUnixFile(mut id: *mut sqlite3_file) -> i32 {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    unixUnmapfile(pFile);
    if (unsafe { (*pFile).h }) >= (0 as i32) {
        robust_close(pFile, unsafe { (*pFile).h }, 2312 as i32);
        unsafe {
            (*pFile).h = -(1 as i32);
        }
    }
    {}
    {}
    unsafe { sqlite3_free((unsafe { (*pFile).pPreallocatedUnused }) as *mut ()) };
    unsafe { memset(pFile as *mut (), 0 as i32, 120 as u64) };
    return 0 as i32;
}

/// Close a file.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixClose")]
extern "C-unwind" fn unixClose(mut id: *mut sqlite3_file) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let mut pInode: *mut unixInodeInfo = unsafe { (*pFile).pInode };
    0 as i32;
    verifyDbFile(pFile);
    unixUnlock(id, 0 as i32);
    0 as i32;
    unixEnterMutex();
    // unixFile.pInode is always valid here. Otherwise, a different close
    // routine (e.g. nolockClose()) would be called instead.
    0 as i32;
    unsafe { sqlite3_mutex_enter(unsafe { (*pInode).pLockMutex }) };
    if (unsafe { (*pInode).nLock }) != (0 as i32) {
        // If there are outstanding locks, do not actually close the file just
        // yet because that would clear those locks.  Instead, add the file
        // descriptor to pInode->pUnused list.  It will be automatically closed
        // when the last lock is cleared.
        setPendingFd(pFile);
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*pInode).pLockMutex }) };
    releaseInodeInfo(pFile);
    0 as i32;
    rc = closeUnixFile(id);
    unixLeaveMutex();
    return rc;
}

// End of the posix advisory lock implementation *****************
// No-op Locking **********************************
//
// Of the various locking implementations available, this is by far the
// simplest:  locking is ignored.  No attempt is made to lock the database
// file for reading or writing.
//
// This locking mode is appropriate for use on read-only databases
// (ex: databases that are burned into CD-ROM, for example.)  It can
// also be used if the application employs some external mechanism to
// prevent simultaneous access of the same database by two or more
// database connections.  But there is a serious risk of database
// corruption if this locking mode is used in situations where multiple
// database connections are accessing the same database file at the same
// time and one or more of those connections are writing.
#[unsafe(link_section = ".text.slate_distinct.os_unix.nolockCheckReservedLock")]
extern "C-unwind" fn nolockCheckReservedLock(
    mut NotUsed: *mut sqlite3_file,
    mut pResOut: *mut i32,
) -> i32 {
    NotUsed;
    unsafe {
        *pResOut = 0 as i32;
    }
    return 0 as i32;
}

#[unsafe(link_section = ".text.slate_distinct.os_unix.nolockLock")]
extern "C-unwind" fn nolockLock(mut NotUsed: *mut sqlite3_file, mut NotUsed2: i32) -> i32 {
    NotUsed;
    NotUsed2;
    return 0 as i32;
}

#[unsafe(link_section = ".text.slate_distinct.os_unix.nolockUnlock")]
extern "C-unwind" fn nolockUnlock(mut NotUsed: *mut sqlite3_file, mut NotUsed2: i32) -> i32 {
    NotUsed;
    NotUsed2;
    return 0 as i32;
}

/// Close the file.
#[unsafe(link_section = ".text.slate_distinct.os_unix.nolockClose")]
extern "C-unwind" fn nolockClose(mut id: *mut sqlite3_file) -> i32 {
    return closeUnixFile(id);
}

// End of the no-op lock implementation *********************
// Begin dot-file Locking ******************************
//
// The dotfile locking implementation uses the existence of separate lock
// files (really a directory) to control access to the database.  This works
// on just about every filesystem imaginable.  But there are serious downsides:
//
//    (1)  There is zero concurrency.  A single reader blocks all other
//         connections from reading or writing the database.
//
//    (2)  An application crash or power loss can leave stale lock files
//         sitting around that need to be cleared manually.
//
// Nevertheless, a dotlock is an appropriate locking mode for use if no
// other locking strategy is available.
//
// Dotfile locking works by creating a subdirectory in the same directory as
// the database and with the same name but with a ".lock" extension added.
// The existence of a lock directory implies an EXCLUSIVE lock.  All other
// lock types (SHARED, RESERVED, PENDING) are mapped into EXCLUSIVE.
// The file suffix added to the data base filename in order to create the
// lock directory.
/// This routine checks if there is a RESERVED lock held on the specified
/// file by this or any other process. If the caller holds a SHARED
/// or greater lock when it is called, then it is assumed that no other
/// client may hold RESERVED. Or, if the caller holds no lock, then it
/// is assumed another client holds RESERVED if the lock-file exists.
#[unsafe(link_section = ".text.slate_distinct.os_unix.dotlockCheckReservedLock")]
extern "C-unwind" fn dotlockCheckReservedLock(
    mut id: *mut sqlite3_file,
    mut pResOut: *mut i32,
) -> i32 {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    {}
    if (((unsafe { (*pFile).eFileLock }) as u32) as i32) >= (1 as i32) {
        unsafe {
            *pResOut = 0 as i32;
        }
    } else {
        unsafe {
            *pResOut = ((unsafe {
                unsafe {
                    std::mem::transmute::<
                        Option<unsafe extern "C-unwind" fn()>,
                        Option<unsafe extern "C-unwind" fn(*const i8, i32) -> i32>,
                    >(unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset((2 as i32) as isize)
                        })
                        .pCurrent
                    })
                }
                .unwrap()(
                    (unsafe { (*pFile).lockingContext }) as *const i8, 0 as i32
                )
            }) == (0 as i32)) as i32;
        }
    }
    {}
    return 0 as i32;
}

/// Lock the file with the lock specified by parameter eFileLock - one
/// of the following:
///
///     (1) SHARED_LOCK
///     (2) RESERVED_LOCK
///     (3) PENDING_LOCK
///     (4) EXCLUSIVE_LOCK
///
/// Sometimes when requesting one lock state, additional lock states
/// are inserted in between.  The locking might fail on one of the later
/// transitions leaving the lock state different from what it started but
/// still short of its goal.  The following chart shows the allowed
/// transitions and the inserted intermediate states:
///
///    UNLOCKED -> SHARED
///    SHARED -> RESERVED
///    SHARED -> (PENDING) -> EXCLUSIVE
///    RESERVED -> (PENDING) -> EXCLUSIVE
///    PENDING -> EXCLUSIVE
///
/// This routine will only increase a lock.  Use the sqlite3OsUnlock()
/// routine to lower a locking level.
///
/// With dotfile locking, we really only support state (4): EXCLUSIVE.
/// But we track the other locking levels internally.
#[unsafe(link_section = ".text.slate_distinct.os_unix.dotlockLock")]
extern "C-unwind" fn dotlockLock(mut id: *mut sqlite3_file, mut eFileLock: i32) -> i32 {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let mut zLockFile: *mut i8 = (unsafe { (*pFile).lockingContext }) as *mut i8;
    let mut rc: i32 = 0 as i32;
    // If we have any lock, then the lock file already exists.  All we have
    // to do is adjust our internal record of the lock level.
    if (((unsafe { (*pFile).eFileLock }) as u32) as i32) > (0 as i32) {
        unsafe {
            (*pFile).eFileLock = (eFileLock as i8) as u8;
        }
        // Always update the timestamp on the old file
        unsafe { utime(zLockFile as *const i8, std::ptr::null::<utimbuf>()) };
        return 0 as i32;
    }
    // grab an exclusive lock
    rc = unsafe {
        unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(*const i8, u32) -> i32>,
            >(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                        .offset((18 as i32) as isize)
                })
                .pCurrent
            })
        }
        .unwrap()(zLockFile as *const i8, (511 as i32) as u32)
    };
    if rc < (0 as i32) {
        // failed to open/create the lock directory
        let mut tErrno: i32 = unsafe { *unsafe { __errno_location() } };
        if (17 as i32) == tErrno {
            rc = 5 as i32;
        } else {
            rc = sqliteErrorFromPosixError(tErrno, (10 as i32) | (15 as i32) << (8 as i32));
            if rc != (5 as i32) {
                storeLastErrno(pFile, tErrno);
            }
        }
        return rc;
    }
    // got it, set the type and return ok
    unsafe {
        (*pFile).eFileLock = (eFileLock as i8) as u8;
    }
    return rc;
}

/// Lower the locking level on file descriptor pFile to eFileLock.  eFileLock
/// must be either NO_LOCK or SHARED_LOCK.
///
/// If the locking level of the file descriptor is already at or below
/// the requested locking level, this routine is a no-op.
///
/// When the locking level reaches NO_LOCK, delete the lock file.
#[unsafe(link_section = ".text.slate_distinct.os_unix.dotlockUnlock")]
extern "C-unwind" fn dotlockUnlock(mut id: *mut sqlite3_file, mut eFileLock: i32) -> i32 {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let mut zLockFile: *mut i8 = (unsafe { (*pFile).lockingContext }) as *mut i8;
    let mut rc: i32 = 0 as i32;
    0 as i32;
    {}
    0 as i32;
    // no-op if possible
    if (((unsafe { (*pFile).eFileLock }) as u32) as i32) == eFileLock {
        return 0 as i32;
    }
    // To downgrade to shared, simply update our internal notion of the
    // lock state.  No need to mess with the file on disk.
    if eFileLock == (1 as i32) {
        unsafe {
            (*pFile).eFileLock = ((1 as i32) as i8) as u8;
        }
        return 0 as i32;
    }
    // To fully unlock the database, delete the lock file
    0 as i32;
    rc = unsafe {
        unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(*const i8) -> i32>,
            >(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                        .offset((19 as i32) as isize)
                })
                .pCurrent
            })
        }
        .unwrap()(zLockFile as *const i8)
    };
    if rc < (0 as i32) {
        let mut tErrno: i32 = unsafe { *unsafe { __errno_location() } };
        if tErrno == (2 as i32) {
            rc = 0 as i32;
        } else {
            rc = (10 as i32) | (8 as i32) << (8 as i32);
            storeLastErrno(pFile, tErrno);
        }
        return rc;
    }
    unsafe {
        (*pFile).eFileLock = ((0 as i32) as i8) as u8;
    }
    return 0 as i32;
}

/// Close a file.  Make sure the lock has been released before closing.
#[unsafe(link_section = ".text.slate_distinct.os_unix.dotlockClose")]
extern "C-unwind" fn dotlockClose(mut id: *mut sqlite3_file) -> i32 {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    0 as i32;
    dotlockUnlock(id, 0 as i32);
    unsafe { sqlite3_free(unsafe { (*pFile).lockingContext }) };
    return closeUnixFile(id);
}

// End of the dot-file lock implementation *******************
// Begin flock Locking ********************************
//
// Use the flock() system call to do file locking.
//
// flock() locking is like dot-file locking in that the various
// fine-grain locking levels supported by SQLite are collapsed into
// a single exclusive lock.  In other words, SHARED, RESERVED, and
// PENDING locks are the same thing as an EXCLUSIVE lock.  SQLite
// still works when you do this, but concurrency is reduced since
// only a single process can be reading the database at a time.
//
// Omit this section if SQLITE_ENABLE_LOCKING_STYLE is turned off
// End of the flock lock implementation *********************
// Begin Named Semaphore Locking ************************
//
// Named semaphore locking is only supported on VxWorks.
//
// Semaphore locking is like dot-lock and flock in that it really only
// supports EXCLUSIVE locking.  Only a single process can read or write
// the database file at a time.  This reduces potential concurrency, but
// makes the lock implementation much easier.
// Named semaphore locking is only available on VxWorks.
//
// End of the named semaphore lock implementation ****************
// Begin AFP Locking *********************************
//
// AFP is the Apple Filing Protocol.  AFP is a network filesystem found
// on Apple Macintosh computers - both OS9 and OSX.
//
// Third-party implementations of AFP are available.  But this code here
// only works on OSX.
// The code above is the AFP lock implementation.  The code is specific
// to MacOSX and does not work on other unix platforms.  No alternative
// is available.  If you don't compile for a mac, then the "unix-afp"
// VFS is not available.
//
// End of the AFP lock implementation **********************
// Begin NFS Locking
// The code above is the NFS lock implementation.  The code is specific
// to MacOSX and does not work on other unix platforms.  No alternative
// is available.
//
// End of the NFS lock implementation **********************
// Non-locking sqlite3_file methods *****************************
//
// The next division contains implementations for all methods of the
// sqlite3_file object other than the locking methods.  The locking
// methods were defined in divisions above (one locking method per
// division).  Those methods that are common to all locking modes
// are gather together into this division.
/// Seek to the offset passed as the second argument, then read cnt
/// bytes into pBuf. Return the number of bytes actually read.
///
/// To avoid stomping the errno value on a failed read the lastErrno value
/// is set before returning.
fn seekAndRead(mut id: *mut unixFile, mut offset: i64, mut pBuf: *mut (), mut cnt: i32) -> i32 {
    let mut got: i32 = 0 as i32;
    let mut prior: i32 = 0 as i32;
    {}
    0 as i32;
    0 as i32;
    '__slate_break_1208: loop {
        got = (unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(i32, *mut (), u64, i64) -> i64>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((10 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(unsafe { (*id).h }, pBuf, (cnt as i64) as u64, offset)
        }) as i32;
        {}
        if got == cnt {
            break '__slate_break_1208;
        }
        if got < (0 as i32) {
            if (unsafe { *unsafe { __errno_location() } }) == (4 as i32) {
                got = 1 as i32;
            } else {
                prior = 0 as i32;
                storeLastErrno(id, unsafe { *unsafe { __errno_location() } });
                break '__slate_break_1208;
            }
        } else {
            if got > (0 as i32) {
                let __v1325: i32 = cnt;
                let __v1326: i32 = __v1325 - got;
                cnt = __v1326;
                let __v1327: i64 = offset;
                let __v1328: i64 = __v1327 + (got as i64);
                offset = __v1328;
                let __v1329: i32 = prior;
                let __v1330: i32 = __v1329 + got;
                prior = __v1330;
                pBuf = (unsafe { (pBuf as *mut i8).offset(got as isize) }) as *mut ();
            }
        }
        if !(got > (0 as i32)) {
            break;
        }
    }
    {}
    {}
    return got + prior;
}

/// Read data from a file into a buffer.  Return SQLITE_OK if all
/// bytes were read successfully and SQLITE_IOERR if anything goes
/// wrong.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixRead")]
extern "C-unwind" fn unixRead(
    mut id: *mut sqlite3_file,
    mut pBuf: *mut (),
    mut amt: i32,
    mut offset: i64,
) -> i32 {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let mut got: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    // If this is a database file (not a journal, super-journal or temp
    // file), the bytes in the locking range should never be read or written.
    // Deal with as much of this read request as possible by transferring
    // data from the memory mapping using memcpy().
    if offset < unsafe { (*pFile).mmapSize } {
        if offset + (amt as i64) <= unsafe { (*pFile).mmapSize } {
            unsafe {
                memcpy(
                    pBuf,
                    (unsafe {
                        ((unsafe { (*pFile).pMapRegion }) as *mut u8).offset(offset as isize)
                    }) as *const (),
                    (amt as i64) as u64,
                )
            };
            return 0 as i32;
        } else {
            let mut nCopy: i32 = ((unsafe { (*pFile).mmapSize }) - offset) as i32;
            unsafe {
                memcpy(
                    pBuf,
                    (unsafe {
                        ((unsafe { (*pFile).pMapRegion }) as *mut u8).offset(offset as isize)
                    }) as *const (),
                    (nCopy as i64) as u64,
                )
            };
            pBuf = (unsafe { (pBuf as *mut u8).offset(nCopy as isize) }) as *mut ();
            let __v1331: i32 = amt;
            let __v1332: i32 = __v1331 - nCopy;
            amt = __v1332;
            let __v1333: i64 = offset;
            let __v1334: i64 = __v1333 + (nCopy as i64);
            offset = __v1334;
        }
    }
    got = seekAndRead(pFile, offset, pBuf, amt);
    if got == amt {
        return 0 as i32;
    } else {
        if got < (0 as i32) {
            // pFile->lastErrno has been set by seekAndRead().
            // Usually we return SQLITE_IOERR_READ here, though for some
            // kinds of errors we return SQLITE_IOERR_CORRUPTFS.  The
            // SQLITE_IOERR_CORRUPTFS will be converted into SQLITE_CORRUPT
            // prior to returning to the application by the sqlite3ApiExit()
            // routine.
            match unsafe { (*pFile).lastErrno } {
                34 | 5 | 6 => {
                    return (10 as i32) | (33 as i32) << (8 as i32);
                }
                _ => {}
            }
            return (10 as i32) | (1 as i32) << (8 as i32);
        } else {
            storeLastErrno(pFile, 0 as i32); // not a system error
            // Unread parts of the buffer must be zero-filled
            unsafe {
                memset(
                    (unsafe { (pBuf as *mut i8).offset(got as isize) }) as *mut (),
                    0 as i32,
                    ((amt - got) as i64) as u64,
                )
            };
            return (10 as i32) | (2 as i32) << (8 as i32);
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Attempt to seek the file-descriptor passed as the first argument to
/// absolute offset iOff, then attempt to write nBuf bytes of data from
/// pBuf to it. If an error occurs, return -1 and set *piErrno. Otherwise,
/// return the actual number of bytes written (which may be less than
/// nBuf).
///
/// # Arguments
///
/// * `fd` - File descriptor to write to
/// * `iOff` - File offset to begin writing at
/// * `pBuf` - Copy data from this buffer to the file
/// * `nBuf` - Size of buffer pBuf in bytes
/// * `piErrno` - OUT: Error number if error occurs
fn seekAndWriteFd(
    mut fd: i32,
    mut iOff: i64,
    mut pBuf: *const (),
    mut nBuf: i32,
    mut piErrno: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Value returned by system call
    0 as i32;
    0 as i32;
    0 as i32;
    let __v1335: i32 = nBuf;
    let __v1336: i32 = __v1335 & (131071 as i32);
    nBuf = __v1336;
    {}
    '__slate_break_1210: loop {
        rc = (unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(i32, *const (), u64, i64) -> i64>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((13 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(fd, pBuf, (nBuf as i64) as u64, iOff)
        }) as i32;
        if !(rc < (0 as i32) && (unsafe { *unsafe { __errno_location() } }) == (4 as i32)) {
            break;
        }
    }
    {}
    {}
    if rc < (0 as i32) {
        unsafe {
            *piErrno = unsafe { *unsafe { __errno_location() } };
        }
    }
    return rc;
}

/// Seek to the offset in id->offset then read cnt bytes into pBuf.
/// Return the number of bytes actually read.  Update the offset.
///
/// To avoid stomping the errno value on a failed write the lastErrno value
/// is set before returning.
fn seekAndWrite(mut id: *mut unixFile, mut offset: i64, mut pBuf: *const (), mut cnt: i32) -> i32 {
    return seekAndWriteFd(unsafe { (*id).h }, offset, pBuf, cnt, unsafe {
        std::ptr::addr_of_mut!((*id).lastErrno)
    });
}

/// Write data from a buffer into a file.  Return SQLITE_OK on success
/// or some other error code on failure.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixWrite")]
extern "C-unwind" fn unixWrite(
    mut id: *mut sqlite3_file,
    mut pBuf: *const (),
    mut amt: i32,
    mut offset: i64,
) -> i32 {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let mut wrote: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    // If this is a database file (not a journal, super-journal or temp
    // file), the bytes in the locking range should never be read or written.
    '__slate_break_1211: loop {
        let __v1337: i32 = seekAndWrite(pFile, offset, pBuf, amt);
        wrote = __v1337;
        if !(__v1337 < amt && wrote > (0 as i32)) {
            break;
        }
        let __v1338: i32 = amt;
        let __v1339: i32 = __v1338 - wrote;
        amt = __v1339;
        let __v1340: i64 = offset;
        let __v1341: i64 = __v1340 + (wrote as i64);
        offset = __v1341;
        pBuf = (unsafe { (pBuf as *mut i8).offset(wrote as isize) }) as *const ();
    }
    {}
    {}
    if amt > wrote {
        if wrote < (0 as i32) && (unsafe { (*pFile).lastErrno }) != (28 as i32) {
            // lastErrno set by seekAndWrite
            return (10 as i32) | (3 as i32) << (8 as i32);
        } else {
            storeLastErrno(pFile, 0 as i32); // not a system error
            return 13 as i32;
        }
    }
    return 0 as i32;
}

// We do not trust systems to provide a working fdatasync().  Some do.
// Others do no.  To be safe, we will stick with the (slightly slower)
// fsync(). If you know that your system does support fdatasync() correctly,
// then simply compile with -Dfdatasync=fdatasync or -DHAVE_FDATASYNC
// Define HAVE_FULLFSYNC to 0 or 1 depending on whether or not
// the F_FULLFSYNC macro is defined.  F_FULLFSYNC is currently
// only available on Mac OS X.  But that could change.
/// The fsync() system call does not work as advertised on many
/// unix systems.  The following procedure is an attempt to make
/// it work better.
///
/// The SQLITE_NO_SYNC macro disables all fsync()s.  This is useful
/// for testing when we want to run through the test suite quickly.
/// You are strongly advised *not* to deploy with SQLITE_NO_SYNC
/// enabled, however, since with SQLITE_NO_SYNC enabled, an OS crash
/// or power failure will likely corrupt the database file.
///
/// SQLite sets the dataOnly flag if the size of the file is unchanged.
/// The idea behind dataOnly is that it should only write the file content
/// to disk, not the inode.  We only set dataOnly if the file size is
/// unchanged since the file size is part of the inode.  However,
/// Ted Ts'o tells us that fdatasync() will also write the inode if the
/// file size has changed.  The only real difference between fdatasync()
/// and fsync(), Ted tells us, is that fdatasync() will not flush the
/// inode if the mtime or owner or other inode attributes have changed.
/// We only care about the file size, not the other file attributes, so
/// as far as SQLite is concerned, an fdatasync() is always adequate.
/// So, we always use fdatasync() if it is available, regardless of
/// the value of the dataOnly flag.
fn full_fsync(mut fd: i32, mut fullSync: i32, mut dataOnly: i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    // The following "ifdef/elif/else/" block has the same structure as
    // the one below. It is replicated here solely to avoid cluttering
    // up the real code with the UNUSED_PARAMETER() macros.
    fullSync;
    dataOnly;
    // Record the number of times that we do a normal fsync() and
    // FULLSYNC.  This is used during testing to verify that this procedure
    // gets called with the correct arguments.
    // If we compiled with the SQLITE_NO_SYNC flag, then syncing is a
    // no-op.  But go ahead and call fstat() to validate the file
    // descriptor as we need a method to provoke a failure during
    // coverage testing.
    rc = unsafe { fdatasync(fd) };
    if (0 as i32) != (0 as i32) && rc != -(1 as i32) {
        rc = 0 as i32;
    }
    return rc;
}

/// Open a file descriptor to the directory containing file zFilename.
/// If successful, *pFd is set to the opened file descriptor and
/// SQLITE_OK is returned. If an error occurs, either SQLITE_NOMEM
/// or SQLITE_CANTOPEN is returned and *pFd is set to an undefined
/// value.
///
/// The directory file descriptor is used for only one thing - to
/// fsync() a directory to make sure file creation and deletion events
/// are flushed to disk.  Such fsyncs are not needed on newer
/// journaling filesystems, but are required on older filesystems.
///
/// This routine can be overridden using the xSetSysCall interface.
/// The ability to override this routine was added in support of the
/// chromium sandbox.  Opening a directory is a security risk (we are
/// told) so making it overrideable allows the chromium sandbox to
/// replace this routine with a harmless no-op.  To make this routine
/// a no-op, replace it with a stub that returns SQLITE_OK but leaves
/// *pFd set to a negative number.
///
/// If SQLITE_OK is returned, the caller is responsible for closing
/// the file descriptor *pFd using close().
#[unsafe(link_section = ".text.slate_distinct.os_unix.openDirectory")]
extern "C-unwind" fn openDirectory(mut zFilename: *const i8, mut pFd: *mut i32) -> i32 {
    let mut ii: i32 = 0 as i32;
    let mut fd: i32 = -(1 as i32);
    let mut zDirname: __SlateAlign16<[i8; 513]> = __SlateAlign16([0 as i8; 513]);
    unsafe {
        sqlite3_snprintf(
            512 as i32,
            zDirname.0.as_mut_ptr() as *mut i8,
            (b"%s\0".as_ptr() as *mut i8) as *const i8,
            zFilename,
        )
    };
    ii = ((unsafe { strlen((zDirname.0.as_mut_ptr() as *mut i8) as *const i8) }) as u32) as i32;
    '__slate_break_1213: loop {
        if !(ii > (0 as i32)
            && ((unsafe { *unsafe { (zDirname.0.as_mut_ptr() as *mut i8).offset(ii as isize) } })
                as i32)
                != (47 as i32))
        {
            break;
        }
        {}
        let __v1285: i32 = ii;
        let __v1286: i32 = __v1285 - (1 as i32);
        ii = __v1286;
    }
    if ii > (0 as i32) {
        unsafe {
            *unsafe { (zDirname.0.as_mut_ptr() as *mut i8).offset(ii as isize) } = (0 as i32) as i8;
        }
    } else {
        if ((unsafe {
            *unsafe { (zDirname.0.as_mut_ptr() as *mut i8).offset((0 as i32) as isize) }
        }) as i32)
            != (47 as i32)
        {
            unsafe {
                *unsafe { (zDirname.0.as_mut_ptr() as *mut i8).offset((0 as i32) as isize) } =
                    (46 as i32) as i8;
            }
        }
        unsafe {
            *unsafe { (zDirname.0.as_mut_ptr() as *mut i8).offset((1 as i32) as isize) } =
                (0 as i32) as i8;
        }
    }
    fd = robust_open(
        (zDirname.0.as_mut_ptr() as *mut i8) as *const i8,
        (0 as i32) | (0 as i32),
        (0 as i32) as u32,
    );
    if fd >= (0 as i32) {
        {}
    }
    unsafe {
        *pFd = fd;
    }
    if fd >= (0 as i32) {
        return 0 as i32;
    }
    return unixLogErrorAtLine(
        unsafe { sqlite3CantopenError(3893 as i32) },
        (b"openDirectory\0".as_ptr() as *mut i8) as *const i8,
        (zDirname.0.as_mut_ptr() as *mut i8) as *const i8,
        3893 as i32,
    );
}

/// Make sure all writes to a particular file are committed to disk.
///
/// If dataOnly==0 then both the file itself and its metadata (file
/// size, access time, etc) are synced.  If dataOnly!=0 then only the
/// file data is synced.
///
/// Under Unix, also make sure that the directory entry for the file
/// has been created by fsync-ing the directory that contains the file.
/// If we do not do this and we encounter a power failure, the directory
/// entry for the journal might not exist after we reboot.  The next
/// SQLite to access the file will not know that the journal exists (because
/// the directory entry for the journal was never created) and the transaction
/// will not roll back - possibly leading to database corruption.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixSync")]
extern "C-unwind" fn unixSync(mut id: *mut sqlite3_file, mut flags: i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let mut isDataOnly: i32 = flags & (16 as i32);
    let mut isFullsync: i32 = (flags & (15 as i32) == (3 as i32)) as i32;
    // Check that one of SQLITE_SYNC_NORMAL or FULL was passed
    0 as i32;
    // Unix cannot, but some systems may return SQLITE_FULL from here. This
    // line is to test that doing so does not cause any problems.
    {}
    0 as i32;
    {}
    rc = full_fsync(unsafe { (*pFile).h }, isFullsync, isDataOnly);
    {}
    if rc != (0 as i32) {
        storeLastErrno(pFile, unsafe { *unsafe { __errno_location() } });
        return unixLogErrorAtLine(
            (10 as i32) | (4 as i32) << (8 as i32),
            (b"full_fsync\0".as_ptr() as *mut i8) as *const i8,
            unsafe { (*pFile).zPath },
            3934 as i32,
        );
    }
    // Also fsync the directory containing the file if the DIRSYNC flag
    // is set.  This is a one-time occurrence.  Many systems (examples: AIX)
    // are unable to fsync a directory, so ignore errors on the fsync.
    if (((unsafe { (*pFile).ctrlFlags }) as u32) as i32) & (8 as i32) != (0 as i32) {
        let mut dirfd: i32 = 0 as i32;
        {}
        rc = unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(*const i8, *mut i32) -> i32>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((17 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(unsafe { (*pFile).zPath }, std::ptr::addr_of_mut!(dirfd))
        };
        if rc == (0 as i32) {
            full_fsync(dirfd, 0 as i32, 0 as i32);
            robust_close(pFile, dirfd, 3948 as i32);
        } else {
            0 as i32;
            rc = 0 as i32;
        }
        let __v1342: *mut unixFile = pFile;
        let __v1343: u16 = unsafe { (*__v1342).ctrlFlags };
        let __v1344: u16 = ((((__v1343 as u32) as i32) & !(8 as i32)) as i16) as u16;
        unsafe {
            (*__v1342).ctrlFlags = __v1344;
        }
    }
    return rc;
}

/// Truncate an open file to a specified size
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixTruncate")]
extern "C-unwind" fn unixTruncate(mut id: *mut sqlite3_file, mut nByte: i64) -> i32 {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let mut rc: i32 = 0 as i32;
    0 as i32;
    {}
    // If the user has configured a chunk-size for this file, truncate the
    // file so that it consists of an integer number of chunks (i.e. the
    // actual file size after the operation may be larger than the requested
    // size).
    if (unsafe { (*pFile).szChunk }) > (0 as i32) {
        nByte = (nByte + ((unsafe { (*pFile).szChunk }) as i64) - ((1 as i32) as i64))
            / ((unsafe { (*pFile).szChunk }) as i64)
            * ((unsafe { (*pFile).szChunk }) as i64);
    }
    rc = robust_ftruncate(unsafe { (*pFile).h }, nByte);
    if rc != (0 as i32) {
        storeLastErrno(pFile, unsafe { *unsafe { __errno_location() } });
        return unixLogErrorAtLine(
            (10 as i32) | (6 as i32) << (8 as i32),
            (b"ftruncate\0".as_ptr() as *mut i8) as *const i8,
            unsafe { (*pFile).zPath },
            3979 as i32,
        );
    } else {
        // If the file was just truncated to a size smaller than the currently
        // mapped region, reduce the effective mapping size as well. SQLite will
        // use read() and write() to access data beyond this point from now on.
        if nByte < unsafe { (*pFile).mmapSize } {
            unsafe {
                (*pFile).mmapSize = nByte;
            }
        }
        return 0 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

/// Determine the current size of a file in bytes
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixFileSize")]
extern "C-unwind" fn unixFileSize(mut id: *mut sqlite3_file, mut pSize: *mut i64) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut buf: stat = unsafe { std::mem::zeroed() };
    0 as i32;
    rc = unsafe {
        unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(i32, *mut stat) -> i32>,
            >(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                        .offset((5 as i32) as isize)
                })
                .pCurrent
            })
        }
        .unwrap()(
            unsafe { (*(id as *mut unixFile)).h },
            std::ptr::addr_of_mut!(buf),
        )
    };
    {}
    if rc != (0 as i32) {
        storeLastErrno(id as *mut unixFile, unsafe {
            *unsafe { __errno_location() }
        });
        return (10 as i32) | (7 as i32) << (8 as i32);
    }
    unsafe {
        *pSize = buf.st_size;
    }
    // When opening a zero-size database, the findInodeInfo() procedure
    // writes a single byte into that file in order to work around a bug
    // in the OS-X msdos filesystem.  In order to avoid problems with upper
    // layers, we need to report this file size as zero even though it is
    // really 1.   Ticket #3260.
    if (unsafe { *pSize }) == ((1 as i32) as i64) {
        unsafe {
            *pSize = (0 as i32) as i64;
        }
    }
    return 0 as i32;
}

/// This function is called to handle the SQLITE_FCNTL_SIZE_HINT
/// file-control operation.  Enlarge the database to nBytes in size
/// (rounded up to the next chunk-size).  If the database is already
/// nBytes or larger, this routine is a no-op.
fn fcntlSizeHint(mut pFile: *mut unixFile, mut nByte: i64) -> i32 {
    if (unsafe { (*pFile).szChunk }) > (0 as i32) {
        let mut nSize: i64 = 0 as i64; // Required file size
        let mut buf: stat = unsafe { std::mem::zeroed() }; // Used to hold return values of fstat()
        if (unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(i32, *mut stat) -> i32>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((5 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(unsafe { (*pFile).h }, std::ptr::addr_of_mut!(buf))
        }) != (0 as i32)
        {
            return (10 as i32) | (7 as i32) << (8 as i32);
        }
        nSize = (nByte + ((unsafe { (*pFile).szChunk }) as i64) - ((1 as i32) as i64))
            / ((unsafe { (*pFile).szChunk }) as i64)
            * ((unsafe { (*pFile).szChunk }) as i64);
        if nSize > buf.st_size {
            // If the OS does not have posix_fallocate(), fake it. Write a
            // single byte to the last byte in each block that falls entirely
            // within the extended region. Then, if required, a single byte
            // at offset (nSize-1), to set the size of the file correctly.
            // This is a similar technique to that used by glibc on systems
            // that do not have a real fallocate() call.
            let mut nBlk: i32 = buf.st_blksize as i32; // File-system block size
            let mut nWrite: i32 = 0 as i32; // Number of bytes written by seekAndWrite
            let mut iWrite: i64 = 0 as i64; // Next offset to write to
            iWrite =
                buf.st_size / (nBlk as i64) * (nBlk as i64) + (nBlk as i64) - ((1 as i32) as i64);
            0 as i32;
            0 as i32;
            '__slate_break_1217: loop {
                if !(iWrite < nSize + (nBlk as i64) - ((1 as i32) as i64)) {
                    break;
                }
                // no-op
                if iWrite >= nSize {
                    iWrite = nSize - ((1 as i32) as i64);
                }
                nWrite = seekAndWrite(
                    pFile,
                    iWrite,
                    (b"\0".as_ptr() as *mut i8) as *const (),
                    1 as i32,
                );
                if nWrite != (1 as i32) {
                    return (10 as i32) | (3 as i32) << (8 as i32);
                }
                let __v1345: i64 = iWrite;
                let __v1346: i64 = __v1345 + (nBlk as i64);
                iWrite = __v1346;
            }
        }
    }
    if (unsafe { (*pFile).mmapSizeMax }) > ((0 as i32) as i64)
        && nByte > unsafe { (*pFile).mmapSize }
    {
        let mut rc: i32 = 0 as i32;
        if (unsafe { (*pFile).szChunk }) <= (0 as i32) {
            if robust_ftruncate(unsafe { (*pFile).h }, nByte) != (0 as i32) {
                storeLastErrno(pFile, unsafe { *unsafe { __errno_location() } });
                return unixLogErrorAtLine(
                    (10 as i32) | (6 as i32) << (8 as i32),
                    (b"ftruncate\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*pFile).zPath },
                    4100 as i32,
                );
            }
        }
        rc = unixMapfile(pFile, nByte);
        return rc;
    }
    return 0 as i32;
}

/// If *pArg is initially negative then this is a query.  Set *pArg to
/// 1 or 0 depending on whether or not bit mask of pFile->ctrlFlags is set.
///
/// If *pArg is 0 or 1, then clear or set the mask bit of pFile->ctrlFlags.
fn unixModeBit(mut pFile: *mut unixFile, mut mask: u8, mut pArg: *mut i32) {
    if (unsafe { *pArg }) < (0 as i32) {
        unsafe {
            *pArg = ((((unsafe { (*pFile).ctrlFlags }) as u32) as i32) & ((mask as u32) as i32)
                != (0 as i32)) as i32;
        }
    } else {
        if (unsafe { *pArg }) == (0 as i32) {
            let __v1347: *mut unixFile = pFile;
            let __v1348: u16 = unsafe { (*__v1347).ctrlFlags };
            let __v1349: u16 =
                ((((__v1348 as u32) as i32) & !((mask as u32) as i32)) as i16) as u16;
            unsafe {
                (*__v1347).ctrlFlags = __v1349;
            }
        } else {
            let __v1350: *mut unixFile = pFile;
            let __v1351: u16 = unsafe { (*__v1350).ctrlFlags };
            let __v1352: u16 = ((((__v1351 as u32) as i32) | ((mask as u32) as i32)) as i16) as u16;
            unsafe {
                (*__v1350).ctrlFlags = __v1352;
            }
        }
    }
}

/// Information and control of an open file handle.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixFileControl")]
extern "C-unwind" fn unixFileControl(
    mut id: *mut sqlite3_file,
    mut op: i32,
    mut pArg: *mut (),
) -> i32 {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    match op {
        43 => {
            unsafe {
                unsafe {
                    std::mem::transmute::<
                        Option<unsafe extern "C-unwind" fn()>,
                        Option<unsafe extern "C-unwind" fn(i32) -> i32>,
                    >(unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset((1 as i32) as isize)
                        })
                        .pCurrent
                    })
                }
                .unwrap()(unsafe { (*pFile).h })
            };
            unsafe {
                (*pFile).h = -(1 as i32);
            }
            return 0 as i32;
        }
        1 => {
            unsafe {
                *(pArg as *mut i32) = ((unsafe { (*pFile).eFileLock }) as u32) as i32;
            }
            return 0 as i32;
        }
        4 => {
            unsafe {
                *(pArg as *mut i32) = unsafe { (*pFile).lastErrno };
            }
            return 0 as i32;
        }
        6 => {
            unsafe {
                (*pFile).szChunk = unsafe { *(pArg as *mut i32) };
            }
            return 0 as i32;
        }
        5 => {
            let mut rc: i32 = 0 as i32;
            {}
            rc = fcntlSizeHint(pFile, unsafe { *(pArg as *mut i64) });
            {}
            return rc;
        }
        10 => {
            unixModeBit(pFile, ((4 as i32) as i8) as u8, pArg as *mut i32);
            return 0 as i32;
        }
        13 => {
            unixModeBit(pFile, ((16 as i32) as i8) as u8, pArg as *mut i32);
            return 0 as i32;
        }
        12 => {
            unsafe {
                *(pArg as *mut *mut i8) = unsafe {
                    sqlite3_mprintf((b"%s\0".as_ptr() as *mut i8) as *const i8, unsafe {
                        (*unsafe { (*pFile).pVfs }).zName
                    })
                };
            }
            return 0 as i32;
        }
        16 => {
            let mut zTFile: *mut i8 = (unsafe {
                sqlite3_malloc64(
                    ((unsafe { (*unsafe { (*pFile).pVfs }).mxPathname }) as i64) as u64,
                )
            }) as *mut i8;
            if zTFile != std::ptr::null_mut::<i8>() {
                unixGetTempname(unsafe { (*unsafe { (*pFile).pVfs }).mxPathname }, zTFile);
                unsafe {
                    *(pArg as *mut *mut i8) = zTFile;
                }
            }
            return 0 as i32;
        }
        20 => {
            unsafe {
                *(pArg as *mut i32) = fileHasMoved(pFile);
            }
            return 0 as i32;
        }
        18 => {
            let mut newLimit: i64 = unsafe { *(pArg as *mut i64) };
            let mut rc: i32 = 0 as i32;
            if newLimit > unsafe { sqlite3Config.mxMmap } {
                newLimit = unsafe { sqlite3Config.mxMmap };
            }
            // The value of newLimit may be eventually cast to (size_t) and passed
            // to mmap(). Restrict its value to 2GB if (size_t) is not at least a
            // 64-bit type.
            if newLimit > ((0 as i32) as i64) && (8 as u64) < (((8 as i32) as i64) as u64) {
                newLimit = newLimit & ((2147483647 as i32) as i64);
            }
            unsafe {
                *(pArg as *mut i64) = unsafe { (*pFile).mmapSizeMax };
            }
            if newLimit >= ((0 as i32) as i64)
                && newLimit != unsafe { (*pFile).mmapSizeMax }
                && (unsafe { (*pFile).nFetchOut }) == (0 as i32)
            {
                unsafe {
                    (*pFile).mmapSizeMax = newLimit;
                }
                if (unsafe { (*pFile).mmapSize }) > ((0 as i32) as i64) {
                    unixUnmapfile(pFile);
                    rc = unixMapfile(pFile, -(1 as i32) as i64);
                }
            }
            return rc;
        }
        40 => {
            return unixFcntlExternalReader(id as *mut unixFile, pArg as *mut i32);
        }
        _ => {}
    }
    return 12 as i32;
}

/// If pFd->sectorSize is non-zero when this function is called, it is a
/// no-op. Otherwise, the values of pFd->sectorSize and
/// pFd->deviceCharacteristics are set according to the file-system
/// characteristics.
///
/// There are two versions of this function. One for QNX and one for all
/// other systems.
fn setDeviceCharacteristics(mut pFd: *mut unixFile) {
    0 as i32;
    if (unsafe { (*pFd).sectorSize }) == (0 as i32) {
        // Set the POWERSAFE_OVERWRITE flag if requested.
        if (((unsafe { (*pFd).ctrlFlags }) as u32) as i32) & (16 as i32) != (0 as i32) {
            let __v1356: *mut unixFile = pFd;
            let __v1357: i32 = unsafe { (*__v1356).deviceCharacteristics };
            let __v1358: i32 = __v1357 | (4096 as i32);
            unsafe {
                (*__v1356).deviceCharacteristics = __v1358;
            }
        }
        let __v1359: *mut unixFile = pFd;
        let __v1360: i32 = unsafe { (*__v1359).deviceCharacteristics };
        let __v1361: i32 = __v1360 | (32768 as i32);
        unsafe {
            (*__v1359).deviceCharacteristics = __v1361;
        }
        unsafe {
            (*pFd).sectorSize = 4096 as i32;
        }
    }
}

/// Return the sector size in bytes of the underlying block device for
/// the specified file. This is almost always 512 bytes, but may be
/// larger for some devices.
///
/// SQLite code assumes this function cannot fail. It also assumes that
/// if two files are created in the same file-system directory (i.e.
/// a database and its journal file) that the sector size will be the
/// same for both.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixSectorSize")]
extern "C-unwind" fn unixSectorSize(mut id: *mut sqlite3_file) -> i32 {
    let mut pFd: *mut unixFile = id as *mut unixFile;
    setDeviceCharacteristics(pFd);
    return unsafe { (*pFd).sectorSize };
}

/// Return the device characteristics for the file.
///
/// This VFS is set up to return SQLITE_IOCAP_POWERSAFE_OVERWRITE by default.
/// However, that choice is controversial since technically the underlying
/// file system does not always provide powersafe overwrites.  (In other
/// words, after a power-loss event, parts of the file that were never
/// written might end up being altered.)  However, non-PSOW behavior is very,
/// very rare.  And asserting PSOW makes a large reduction in the amount
/// of required I/O for journaling, since a lot of padding is eliminated.
///  Hence, while POWERSAFE_OVERWRITE is on by default, there is a file-control
/// available to turn it off and URI query parameter available to turn it off.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixDeviceCharacteristics")]
extern "C-unwind" fn unixDeviceCharacteristics(mut id: *mut sqlite3_file) -> i32 {
    let mut pFd: *mut unixFile = id as *mut unixFile;
    setDeviceCharacteristics(pFd);
    return unsafe { (*pFd).deviceCharacteristics };
}

/// Return the system page size.
///
/// This function should not be called directly by other code in this file.
/// Instead, it should be called via macro osGetpagesize().
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixGetpagesize")]
extern "C-unwind" fn unixGetpagesize() -> i32 {
    return (unsafe { sysconf(30 as i32) }) as i32;
}

/// Object used to represent an shared memory buffer.
///
/// When multiple threads all reference the same wal-index, each thread
/// has its own unixShm object, but they all point to a single instance
/// of this unixShmNode object.  In other words, each wal-index is opened
/// only once per process.
///
/// Each unixShmNode object is connected to a single unixInodeInfo object.
/// We could coalesce this object into unixInodeInfo, but that would mean
/// every open file that does not use shared memory (in other words, most
/// open files) would have to carry around this extra information.  So
/// the unixInodeInfo object contains a pointer to this unixShmNode object
/// and the unixShmNode object is created only when needed.
///
/// unixMutexHeld() must be true when creating or destroying
/// this object or while reading or writing the following fields:
///
///      nRef
///
/// The following fields are read-only after the object is created:
///
///      hShm
///      zFilename
///
/// Either unixShmNode.pShmMutex must be held or unixShmNode.nRef==0 and
/// unixMutexHeld() is true when reading or writing any other field
/// in this structure.
///
/// aLock[SQLITE_SHM_NLOCK]:
///   This array records the various locks held by clients on each of the
///   SQLITE_SHM_NLOCK slots. If the aLock[] entry is set to 0, then no
///   locks are held by the process on this slot. If it is set to -1, then
///   some client holds an EXCLUSIVE lock on the locking slot. If the aLock[]
///   value is set to a positive value, then it is the number of shared
///   locks currently held on the slot.
///
/// aMutex[SQLITE_SHM_NLOCK]:
///   Normally, when SQLITE_ENABLE_SETLK_TIMEOUT is not defined, mutex
///   pShmMutex is used to protect the aLock[] array and the right to
///   call fcntl() on unixShmNode.hShm to obtain or release locks.
///
///   If SQLITE_ENABLE_SETLK_TIMEOUT is defined though, we use an array
///   of mutexes - one for each locking slot. To read or write locking
///   slot aLock[iSlot], the caller must hold the corresponding mutex
///   aMutex[iSlot]. Similarly, to call fcntl() to obtain or release a
///   lock corresponding to slot iSlot, mutex aMutex[iSlot] must be held.
#[repr(C)]
#[derive(Clone, Copy)]
struct unixShmNode {
    /// unixInodeInfo that owns this SHM node
    pInode: *mut unixInodeInfo,
    /// Mutex to access this object
    pShmMutex: *mut sqlite3_mutex,
    /// Name of the mmapped file
    zFilename: *mut i8,
    /// Open file descriptor
    hShm: i32,
    /// Size of shared-memory regions
    szRegion: i32,
    /// Size of array apRegion
    nRegion: u16,
    /// True if read-only
    isReadonly: u8,
    /// True if no DMS lock held
    isUnlocked: u8,
    /// Array of mapped shared-memory regions
    apRegion: *mut *mut i8,
    /// Number of unixShm objects pointing to this
    nRef: i32,
    /// All unixShm objects pointing to this
    pFirst: *mut unixShm,
    /// # shared locks on slot, -1==excl lock
    aLock: [i32; 8],
}

/// Structure used internally by this VFS to record the state of an
/// open shared memory connection.
///
/// The following fields are initialized when this object is created and
/// are read-only thereafter:
///
///    unixShm.pShmNode
///    unixShm.id
///
/// All other fields are read/write.  The unixShm.pShmNode->pShmMutex must
/// be held while accessing any read/write fields.
#[repr(C)]
#[derive(Clone, Copy)]
struct unixShm {
    /// The underlying unixShmNode object
    pShmNode: *mut unixShmNode,
    /// Next unixShm with the same unixShmNode
    pNext: *mut unixShm,
    /// True if holding the unixShmNode->pShmMutex
    hasMutex: u8,
    /// Id of this connection within its unixShmNode
    id: u8,
    /// Mask of shared locks held
    sharedMask: u16,
    /// Mask of exclusive locks held
    exclMask: u16,
}

// Constants used for locking
// first lock byte
// deadman switch
/// Use F_GETLK to check whether or not there are any readers with open
/// wal-mode transactions in other processes on database file pFile. If
/// no error occurs, return SQLITE_OK and set (*piOut) to 1 if there are
/// such transactions, or 0 otherwise. If an error occurs, return an
/// SQLite error code. The final value of *piOut is undefined in this
/// case.
fn unixFcntlExternalReader(mut pFile: *mut unixFile, mut piOut: *mut i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    unsafe {
        *piOut = 0 as i32;
    }
    if (unsafe { (*pFile).pShm }) != std::ptr::null_mut::<unixShm>() {
        let mut pShmNode: *mut unixShmNode = unsafe { (*unsafe { (*pFile).pShm }).pShmNode };
        let mut f: flock = unsafe { std::mem::zeroed() };
        unsafe { memset(std::ptr::addr_of_mut!(f) as *mut (), 0 as i32, 32 as u64) };
        f.l_type = (1 as i32) as i16;
        f.l_whence = (0 as i32) as i16;
        f.l_start = (((22 as i32) + (8 as i32)) * (4 as i32) + (3 as i32)) as i64;
        f.l_len = ((8 as i32) - (3 as i32)) as i64;
        unsafe { sqlite3_mutex_enter(unsafe { (*pShmNode).pShmMutex }) };
        if (unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(i32, i32, ...) -> i32>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((7 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(
                unsafe { (*pShmNode).hShm },
                5 as i32,
                std::ptr::addr_of_mut!(f),
            )
        }) < (0 as i32)
        {
            rc = (10 as i32) | (15 as i32) << (8 as i32);
        } else {
            unsafe {
                *piOut = ((f.l_type as i32) != (2 as i32)) as i32;
            }
        }
        unsafe { sqlite3_mutex_leave(unsafe { (*pShmNode).pShmMutex }) };
    }
    return rc;
}

/// If pFile has a -shm file open and it is sharing that file with some
/// other connection, either in the same process or in a separate process,
/// then return true.  Return false if either pFile does not have a -shm
/// file open or if it is the only connection to that -shm file across the
/// entire system.
///
/// This routine is not required for correct operation.  It can always return
/// false and SQLite will continue to operate according to spec.  However,
/// when this routine does its job, it adds extra robustness in cases
/// where database file locks have been erroneously deleted in a WAL-mode
/// database by doing close(open(DATABASE_PATHNAME)) or similar.
///
/// With false negatives, SQLite still operates to spec, though with less
/// robustness.  With false positives, the last database connection on a
/// WAL-mode database will fail to unlink the -wal and -shm files, which
/// is annoying but harmless.  False positives will also prevent a database
/// connection from running "PRAGMA journal_mode=DELETE" in order to take
/// the database out of WAL mode, which is perhaps more serious, but is
/// still not a disaster.
fn unixIsSharingShmNode(mut pFile: *mut unixFile) -> i32 {
    let mut pShmNode: *mut unixShmNode = unsafe { std::mem::zeroed() };
    let mut lock: flock = unsafe { std::mem::zeroed() };
    if (unsafe { (*pFile).pShm }) == std::ptr::null_mut::<unixShm>() {
        return 0 as i32;
    }
    if (((unsafe { (*pFile).ctrlFlags }) as u32) as i32) & (1 as i32) != (0 as i32) {
        return 0 as i32;
    }
    pShmNode = unsafe { (*unsafe { (*pFile).pShm }).pShmNode };
    0 as i32;
    unsafe { memset(std::ptr::addr_of_mut!(lock) as *mut (), 0 as i32, 32 as u64) };
    lock.l_whence = (0 as i32) as i16;
    lock.l_start = (((22 as i32) + (8 as i32)) * (4 as i32) + (8 as i32)) as i64;
    lock.l_len = (1 as i32) as i64;
    lock.l_type = (1 as i32) as i16;
    unsafe {
        unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(i32, i32, ...) -> i32>,
            >(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                        .offset((7 as i32) as isize)
                })
                .pCurrent
            })
        }
        .unwrap()(
            unsafe { (*pShmNode).hShm },
            5 as i32,
            std::ptr::addr_of_mut!(lock),
        )
    };
    return ((lock.l_type as i32) != (2 as i32)) as i32;
}

/// Apply posix advisory locks for all bytes from ofst through ofst+n-1.
///
/// Locks block if the mask is exactly UNIX_SHM_C and are non-blocking
/// otherwise.
///
/// # Arguments
///
/// * `pFile` - Open connection to the WAL file
/// * `lockType` - F_UNLCK, F_RDLCK, or F_WRLCK
/// * `ofst` - First byte of the locking range
/// * `n` - Number of bytes to lock
fn unixShmSystemLock(
    mut pFile: *mut unixFile,
    mut lockType: i32,
    mut ofst: i32,
    mut n: i32,
) -> i32 {
    let mut pShmNode: *mut unixShmNode = unsafe { std::mem::zeroed() }; // Apply locks to this open shared-memory segment
    let mut f: flock = unsafe { std::mem::zeroed() }; // The posix advisory locking structure
    let mut rc: i32 = 0 as i32; // Result code form fcntl()
    pShmNode = unsafe { (*unsafe { (*pFile).pInode }).pShmNode };
    // Assert that the parameters are within expected range and that the
    // correct mutex or mutexes are held.
    0 as i32;
    0 as i32;
    if ofst == ((22 as i32) + (8 as i32)) * (4 as i32) + (8 as i32) {
        0 as i32;
        0 as i32;
    } else {
        0 as i32;
        0 as i32;
    }
    // Shared locks never span more than one byte
    0 as i32;
    // Locks are within range
    0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe { (*pShmNode).hShm }) >= (0 as i32) {
        let mut res: i32 = 0 as i32;
        // Initialize the locking parameters
        f.l_type = lockType as i16;
        f.l_whence = (0 as i32) as i16;
        f.l_start = ofst as i64;
        f.l_len = n as i64;
        res = unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(i32, i32, ...) -> i32>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((7 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(
                unsafe { (*pShmNode).hShm },
                6 as i32,
                std::ptr::addr_of_mut!(f),
            )
        };
        if res == -(1 as i32) {
            rc = 5 as i32;
        }
    }
    // Do debug tracing
    return rc;
}

/// Return the minimum number of 32KB shm regions that should be mapped at
/// a time, assuming that each mapping must be an integer multiple of the
/// current system page-size.
///
/// Usually, this is 1. The exception seems to be systems that are configured
/// to use 64KB pages - in this case each mapping must cover at least two
/// shm regions.
fn unixShmRegionPerMap() -> i32 {
    let mut shmsz: i32 = (32 as i32) * (1024 as i32); // SHM region size
    let mut pgsz_777: i32 = unsafe {
        unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn() -> i32>,
            >(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                        .offset((25 as i32) as isize)
                })
                .pCurrent
            })
        }
        .unwrap()()
    }; // System page size
    0 as i32; // Page size must be a power of 2
    if pgsz_777 < shmsz {
        return 1 as i32;
    }
    return pgsz_777 / shmsz;
}

/// Purge the unixShmNodeList list of all entries with unixShmNode.nRef==0.
///
/// This is not a VFS shared-memory method; it is a utility function called
/// by VFS shared-memory methods.
fn unixShmPurge(mut pFd: *mut unixFile) {
    let mut p: *mut unixShmNode = unsafe { (*unsafe { (*pFd).pInode }).pShmNode };
    0 as i32;
    if p != std::ptr::null_mut::<unixShmNode>() && (unsafe { (*p).nRef }) == (0 as i32) {
        let mut nShmPerMap: i32 = unixShmRegionPerMap();
        let mut i: i32 = 0 as i32;
        0 as i32;
        unsafe { sqlite3_mutex_free(unsafe { (*p).pShmMutex }) };
        i = 0 as i32;
        '__slate_break_1226: loop {
            if !(i < (((unsafe { (*p).nRegion }) as u32) as i32)) {
                break;
            }
            if (unsafe { (*p).hShm }) >= (0 as i32) {
                unsafe {
                    unsafe {
                        std::mem::transmute::<
                            Option<unsafe extern "C-unwind" fn()>,
                            Option<unsafe extern "C-unwind" fn(*mut (), u64) -> i32>,
                        >(unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                    .offset((23 as i32) as isize)
                            })
                            .pCurrent
                        })
                    }
                    .unwrap()(
                        (unsafe { *unsafe { unsafe { (*p).apRegion }.offset(i as isize) } })
                            as *mut (),
                        ((unsafe { (*p).szRegion }) as i64) as u64,
                    )
                };
            } else {
                unsafe {
                    sqlite3_free(
                        (unsafe { *unsafe { unsafe { (*p).apRegion }.offset(i as isize) } })
                            as *mut (),
                    )
                };
            }
            let __v1362: i32 = i;
            let __v1363: i32 = __v1362 + nShmPerMap;
            i = __v1363;
        }
        unsafe { sqlite3_free((unsafe { (*p).apRegion }) as *mut ()) };
        if (unsafe { (*p).hShm }) >= (0 as i32) {
            robust_close(pFd, unsafe { (*p).hShm }, 4833 as i32);
            unsafe {
                (*p).hShm = -(1 as i32);
            }
        }
        unsafe {
            (*unsafe { (*p).pInode }).pShmNode = std::ptr::null_mut::<unixShmNode>();
        }
        unsafe { sqlite3_free(p as *mut ()) };
    }
}

/// The DMS lock has not yet been taken on shm file pShmNode. Attempt to
/// take it now. Return SQLITE_OK if successful, or an SQLite error
/// code otherwise.
///
/// If the DMS cannot be locked because this is a readonly_shm=1
/// connection and no other process already holds a lock, return
/// SQLITE_READONLY_CANTINIT and set pShmNode->isUnlocked=1.
fn unixLockSharedMemory(mut pDbFd: *mut unixFile, mut pShmNode: *mut unixShmNode) -> i32 {
    let mut lock: flock = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    // Use F_GETLK to determine the locks other processes are holding
    // on the DMS byte. If it indicates that another process is holding
    // a SHARED lock, then this process may also take a SHARED lock
    // and proceed with opening the *-shm file.
    //
    // Or, if no other process is holding any lock, then this process
    // is the first to open it. In this case take an EXCLUSIVE lock on the
    // DMS byte and truncate the *-shm file to zero bytes in size. Then
    // downgrade to a SHARED lock on the DMS byte.
    //
    // If another process is holding an EXCLUSIVE lock on the DMS byte,
    // return SQLITE_BUSY to the caller (it will try again). An earlier
    // version of this code attempted the SHARED lock at this point. But
    // this introduced a subtle race condition: if the process holding
    // EXCLUSIVE failed just before truncating the *-shm file, then this
    // process might open and use the *-shm file without truncating it.
    // And if the *-shm file has been corrupted by a power failure or
    // system crash, the database itself may also become corrupt.
    lock.l_whence = (0 as i32) as i16;
    lock.l_start = (((22 as i32) + (8 as i32)) * (4 as i32) + (8 as i32)) as i64;
    lock.l_len = (1 as i32) as i64;
    lock.l_type = (1 as i32) as i16;
    if (unsafe {
        unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(i32, i32, ...) -> i32>,
            >(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                        .offset((7 as i32) as isize)
                })
                .pCurrent
            })
        }
        .unwrap()(
            unsafe { (*pShmNode).hShm },
            5 as i32,
            std::ptr::addr_of_mut!(lock),
        )
    }) != (0 as i32)
    {
        rc = (10 as i32) | (15 as i32) << (8 as i32);
    } else {
        if (lock.l_type as i32) == (2 as i32) {
            if (unsafe { (*pShmNode).isReadonly }) != (0 as u8) {
                unsafe {
                    (*pShmNode).isUnlocked = ((1 as i32) as i8) as u8;
                }
                rc = (8 as i32) | (5 as i32) << (8 as i32);
            } else {
                rc = unixShmSystemLock(
                    pDbFd,
                    1 as i32,
                    ((22 as i32) + (8 as i32)) * (4 as i32) + (8 as i32),
                    1 as i32,
                );
                // The first connection to attach must truncate the -shm file.  We
                // truncate to 3 bytes (an arbitrary small number, less than the
                // -shm header size) rather than 0 as a system debugging aid, to
                // help detect if a -shm file truncation is legitimate or is the work
                // or a rogue process.
                let __v1364: bool;
                if rc == (0 as i32) {
                    __v1364 = robust_ftruncate(unsafe { (*pShmNode).hShm }, (3 as i32) as i64)
                        != (0 as i32);
                } else {
                    __v1364 = false as bool;
                }
                if __v1364 {
                    rc = unixLogErrorAtLine(
                        (10 as i32) | (18 as i32) << (8 as i32),
                        (b"ftruncate\0".as_ptr() as *mut i8) as *const i8,
                        (unsafe { (*pShmNode).zFilename }) as *const i8,
                        4903 as i32,
                    );
                }
            }
        } else {
            if (lock.l_type as i32) == (1 as i32) {
                rc = 5 as i32;
            }
        }
    }
    if rc == (0 as i32) {
        0 as i32;
        rc = unixShmSystemLock(
            pDbFd,
            0 as i32,
            ((22 as i32) + (8 as i32)) * (4 as i32) + (8 as i32),
            1 as i32,
        );
    }
    return rc;
}

/// Open a shared-memory area associated with open database file pDbFd.
/// This particular implementation uses mmapped files.
///
/// The file used to implement shared-memory is in the same directory
/// as the open database file and has the same name as the open database
/// file with the "-shm" suffix added.  For example, if the database file
/// is "/home/user1/config.db" then the file that is created and mmapped
/// for shared memory will be called "/home/user1/config.db-shm".
///
/// Another approach to is to use files in /dev/shm or /dev/tmp or an
/// some other tmpfs mount. But if a file in a different directory
/// from the database file is used, then differing access permissions
/// or a chroot() might cause two different processes on the same
/// database to end up using different files for shared memory -
/// meaning that their memory would not really be shared - resulting
/// in database corruption.  Nevertheless, this tmpfs file usage
/// can be enabled at compile-time using -DSQLITE_SHM_DIRECTORY="/dev/shm"
/// or the equivalent.  The use of the SQLITE_SHM_DIRECTORY compile-time
/// option results in an incompatible build of SQLite;  builds of SQLite
/// that with differing SQLITE_SHM_DIRECTORY settings attempt to use the
/// same database file at the same time, database corruption will likely
/// result. The SQLITE_SHM_DIRECTORY compile-time option is considered
/// "unsupported" and may go away in a future SQLite release.
///
/// When opening a new shared-memory file, if no other instances of that
/// file are currently open, in this process or in other processes, then
/// the file must be truncated to zero length or have its header cleared.
///
/// If the original database file (pDbFd) is using the "unix-excl" VFS
/// that means that an exclusive lock is held on the database file and
/// that no other processes are able to read or write the database.  In
/// that case, we do not really need shared memory.  No shared memory
/// file is created.  The shared memory will be simulated with heap memory.
fn unixOpenSharedMemory(mut pDbFd: *mut unixFile) -> i32 {
    let mut __slate_storage_1368: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1368: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1368) as *mut i32;
    let mut __slate_storage_1367: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1367: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1367) as *mut i32;
    let mut __slate_storage_1366: std::mem::MaybeUninit<*mut unixShmNode> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1366: *mut *mut unixShmNode =
        std::ptr::addr_of_mut!(__slate_storage_1366) as *mut *mut unixShmNode;
    let mut __slate_storage_1365: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1365: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1365) as *mut *mut i8;
    let mut __slate_storage_798: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_798: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_798) as *mut *const i8; // fstat() info for database file
    let mut __slate_storage_797: std::mem::MaybeUninit<stat> = std::mem::MaybeUninit::uninit();
    let __slate_slot_797: *mut stat = std::ptr::addr_of_mut!(__slate_storage_797) as *mut stat; // Size of the SHM filename in bytes
    let mut __slate_storage_796: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_796: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_796) as *mut i32; // Name of the file used for SHM
    let mut __slate_storage_795: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_795: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_795) as *mut *mut i8; // The inode of fd
    let mut __slate_storage_794: std::mem::MaybeUninit<*mut unixInodeInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_794: *mut *mut unixInodeInfo =
        std::ptr::addr_of_mut!(__slate_storage_794) as *mut *mut unixInodeInfo; // Result code
    let mut __slate_storage_793: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_793: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_793) as *mut i32; // The underlying mmapped file
    let mut __slate_storage_792: std::mem::MaybeUninit<*mut unixShmNode> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_792: *mut *mut unixShmNode =
        std::ptr::addr_of_mut!(__slate_storage_792) as *mut *mut unixShmNode; // The connection to be opened
    let mut __slate_storage_791: std::mem::MaybeUninit<*mut unixShm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_791: *mut *mut unixShm =
        std::ptr::addr_of_mut!(__slate_storage_791) as *mut *mut unixShm;
    unsafe {
        std::ptr::write(__slate_slot_791, std::ptr::null_mut::<unixShm>());
        std::ptr::write(__slate_slot_793, 0 as i32);
        // Allocate space for the new unixShm object.
        *__slate_slot_791 = (unsafe { sqlite3_malloc64(24 as u64) }) as *mut unixShm;
        if *__slate_slot_791 == std::ptr::null_mut::<unixShm>() {
            return 7 as i32;
        } else {
            '__join_1: {
                unsafe { memset(*__slate_slot_791 as *mut (), 0 as i32, 24 as u64) };
                0 as i32;
                // Check to see if a unixShmNode object already exists. Reuse an existing
                // one if present. Create a new one if necessary.
                0 as i32;
                unixEnterMutex();
                *__slate_slot_794 = unsafe { (*pDbFd).pInode };
                *__slate_slot_792 = unsafe { (*(*__slate_slot_794)).pShmNode };
                if *__slate_slot_792 == std::ptr::null_mut::<unixShmNode>() {
                    '__join_0: {
                        std::ptr::write(__slate_slot_798, unsafe { (*pDbFd).zPath });
                        // Call fstat() to figure out the permissions on the database file. If
                        // a new *-shm file is created, an attempt will be made to create it
                        // with the same permissions.
                        if (unsafe {
                            unsafe {
                                std::mem::transmute::<
                                    Option<unsafe extern "C-unwind" fn()>,
                                    Option<unsafe extern "C-unwind" fn(i32, *mut stat) -> i32>,
                                >(unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall
                                        }
                                        .offset((5 as i32) as isize)
                                    })
                                    .pCurrent
                                })
                            }
                            .unwrap()(
                                unsafe { (*pDbFd).h },
                                std::ptr::addr_of_mut!(*__slate_slot_797),
                            )
                        }) != (0 as i32)
                        {
                            *__slate_slot_793 = (10 as i32) | (7 as i32) << (8 as i32);
                        } else {
                            *__slate_slot_796 = (6 as i32)
                                + (((unsafe { strlen(*__slate_slot_798) }) as u32) as i32);
                            *__slate_slot_792 = (unsafe {
                                sqlite3_malloc64(
                                    (96 as u64).wrapping_add((*__slate_slot_796 as i64) as u64),
                                )
                            }) as *mut unixShmNode;
                            if *__slate_slot_792 == std::ptr::null_mut::<unixShmNode>() {
                                *__slate_slot_793 = 7 as i32;
                            } else {
                                unsafe {
                                    memset(
                                        *__slate_slot_792 as *mut (),
                                        0 as i32,
                                        (96 as u64).wrapping_add((*__slate_slot_796 as i64) as u64),
                                    )
                                };
                                std::ptr::write(
                                    __slate_slot_1365,
                                    (unsafe { (*__slate_slot_792).offset((1 as i32) as isize) })
                                        as *mut i8,
                                );
                                unsafe {
                                    (*(*__slate_slot_792)).zFilename = *__slate_slot_1365;
                                }
                                *__slate_slot_795 = *__slate_slot_1365;
                                unsafe {
                                    sqlite3_snprintf(
                                        *__slate_slot_796,
                                        *__slate_slot_795,
                                        (b"%s-shm\0".as_ptr() as *mut i8) as *const i8,
                                        *__slate_slot_798,
                                    )
                                };
                                {}
                                unsafe {
                                    (*(*__slate_slot_792)).hShm = -(1 as i32);
                                }
                                unsafe {
                                    (*unsafe { (*pDbFd).pInode }).pShmNode = *__slate_slot_792;
                                }
                                unsafe {
                                    (*(*__slate_slot_792)).pInode = unsafe { (*pDbFd).pInode };
                                }
                                if (unsafe { sqlite3Config.bCoreMutex }) != (0 as u8) {
                                    unsafe {
                                        (*(*__slate_slot_792)).pShmMutex =
                                            unsafe { sqlite3_mutex_alloc(0 as i32) };
                                    }
                                    if (unsafe { (*(*__slate_slot_792)).pShmMutex })
                                        == std::ptr::null_mut::<sqlite3_mutex>()
                                    {
                                        *__slate_slot_793 = 7 as i32;
                                        break '__join_0;
                                    }
                                }
                                if (((unsafe { (*(*__slate_slot_794)).bProcessLock }) as u32)
                                    as i32)
                                    == (0 as i32)
                                {
                                    if (0 as i32)
                                        == unsafe {
                                            sqlite3_uri_boolean(
                                                unsafe { (*pDbFd).zPath },
                                                (b"readonly_shm\0".as_ptr() as *mut i8)
                                                    as *const i8,
                                                0 as i32,
                                            )
                                        }
                                    {
                                        unsafe {
                                            (*(*__slate_slot_792)).hShm = robust_open(
                                                *__slate_slot_795 as *const i8,
                                                (2 as i32) | (64 as i32) | (131072 as i32),
                                                (*__slate_slot_797).st_mode & ((511 as i32) as u32),
                                            );
                                        }
                                    }
                                    if (unsafe { (*(*__slate_slot_792)).hShm }) < (0 as i32) {
                                        unsafe {
                                            (*(*__slate_slot_792)).hShm = robust_open(
                                                *__slate_slot_795 as *const i8,
                                                (0 as i32) | (131072 as i32),
                                                (*__slate_slot_797).st_mode & ((511 as i32) as u32),
                                            );
                                        }
                                        if (unsafe { (*(*__slate_slot_792)).hShm }) < (0 as i32) {
                                            *__slate_slot_793 = unixLogErrorAtLine(
                                                unsafe { sqlite3CantopenError(5040 as i32) },
                                                (b"open\0".as_ptr() as *mut i8) as *const i8,
                                                *__slate_slot_795 as *const i8,
                                                5040 as i32,
                                            );
                                            break '__join_0;
                                        } else {
                                            unsafe {
                                                (*(*__slate_slot_792)).isReadonly =
                                                    ((1 as i32) as i8) as u8;
                                            }
                                        }
                                    }
                                    // If this process is running as root, make sure that the SHM file
                                    // is owned by the same user that owns the original database.  Otherwise,
                                    // the original owner will not be able to connect.
                                    robustFchown(
                                        unsafe { (*(*__slate_slot_792)).hShm },
                                        (*__slate_slot_797).st_uid,
                                        (*__slate_slot_797).st_gid,
                                    );
                                    *__slate_slot_793 =
                                        unixLockSharedMemory(pDbFd, *__slate_slot_792);
                                    if *__slate_slot_793 != (0 as i32)
                                        && *__slate_slot_793
                                            != (8 as i32) | (5 as i32) << (8 as i32)
                                    {
                                    } else {
                                        break '__join_1;
                                    }
                                } else {
                                    break '__join_1;
                                }
                            }
                        }
                    }
                    unixShmPurge(pDbFd); // This call frees pShmNode if required
                    unsafe { sqlite3_free(*__slate_slot_791 as *mut ()) };
                    unixLeaveMutex();
                    return *__slate_slot_793;
                }
            }
            // Make the new connection a child of the unixShmNode
            unsafe {
                (*(*__slate_slot_791)).pShmNode = *__slate_slot_792;
            }
            std::ptr::write(__slate_slot_1366, *__slate_slot_792);
            std::ptr::write(__slate_slot_1367, unsafe { (*(*__slate_slot_1366)).nRef });
            std::ptr::write(__slate_slot_1368, *__slate_slot_1367 + (1 as i32));
            unsafe {
                (*(*__slate_slot_1366)).nRef = *__slate_slot_1368;
            }
            unsafe {
                (*pDbFd).pShm = *__slate_slot_791;
            }
            unixLeaveMutex();
            // The reference count on pShmNode has already been incremented under
            // the cover of the unixEnterMutex() mutex and the pointer from the
            // new (struct unixShm) object to the pShmNode has been set. All that is
            // left to do is to link the new object into the linked list starting
            // at pShmNode->pFirst. This must be done while holding the
            // pShmNode->pShmMutex.
            unsafe { sqlite3_mutex_enter(unsafe { (*(*__slate_slot_792)).pShmMutex }) };
            unsafe {
                (*(*__slate_slot_791)).pNext = unsafe { (*(*__slate_slot_792)).pFirst };
            }
            unsafe {
                (*(*__slate_slot_792)).pFirst = *__slate_slot_791;
            }
            unsafe { sqlite3_mutex_leave(unsafe { (*(*__slate_slot_792)).pShmMutex }) };
            return *__slate_slot_793;
        }
    }
    // Jump here on any error
    return unsafe { std::mem::zeroed() };
}

/// This function is called to obtain a pointer to region iRegion of the
/// shared-memory associated with the database file fd. Shared-memory regions
/// are numbered starting from zero. Each shared-memory region is szRegion
/// bytes in size.
///
/// If an error occurs, an error code is returned and *pp is set to NULL.
///
/// Otherwise, if the bExtend parameter is 0 and the requested shared-memory
/// region has not been allocated (by any client, including one running in a
/// separate process), then *pp is set to NULL and SQLITE_OK returned. If
/// bExtend is non-zero and the requested shared-memory region has not yet
/// been allocated, it is allocated by this function.
///
/// If the shared-memory region has already been allocated or is allocated by
/// this call as described above, then it is mapped into this processes
/// address space (if it is not already), *pp is set to point to the mapped
/// memory and SQLITE_OK returned.
///
/// # Arguments
///
/// * `fd` - Handle open on database file
/// * `iRegion` - Region to retrieve
/// * `szRegion` - Size of regions
/// * `bExtend` - True to extend file if necessary
/// * `pp` - OUT: Mapped memory
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixShmMap")]
extern "C-unwind" fn unixShmMap(
    mut fd: *mut sqlite3_file,
    mut iRegion: i32,
    mut szRegion: i32,
    mut bExtend: i32,
    mut pp: *mut *mut (),
) -> i32 {
    let mut __slate_storage_1375: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1375: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1375) as *mut u16;
    let mut __slate_storage_1374: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1374: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1374) as *mut u16;
    let mut __slate_storage_1373: std::mem::MaybeUninit<*mut unixShmNode> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1373: *mut *mut unixShmNode =
        std::ptr::addr_of_mut!(__slate_storage_1373) as *mut *mut unixShmNode;
    let mut __slate_storage_1372: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1372: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1372) as *mut i64;
    let mut __slate_storage_1371: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1371: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1371) as *mut i64;
    let mut __slate_storage_821: std::mem::MaybeUninit<*mut ()> = std::mem::MaybeUninit::uninit();
    let __slate_slot_821: *mut *mut () =
        std::ptr::addr_of_mut!(__slate_storage_821) as *mut *mut ();
    let mut __slate_storage_820: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_820: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_820) as *mut i64;
    let mut __slate_storage_819: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_819: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_819) as *mut i64;
    let mut __slate_storage_1370: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1370: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1370) as *mut i64;
    let mut __slate_storage_1369: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1369: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1369) as *mut i64;
    let mut __slate_storage_818: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_818: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_818) as *mut *const i8;
    let mut __slate_storage_817: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_817: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_817) as *mut i32;
    let mut __slate_storage_816: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_816: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_816) as *mut i64; // Used by fstat()
    let mut __slate_storage_814: std::mem::MaybeUninit<stat> = std::mem::MaybeUninit::uninit();
    let __slate_slot_814: *mut stat = std::ptr::addr_of_mut!(__slate_storage_814) as *mut stat; // Minimum required file size
    let mut __slate_storage_813: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_813: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_813) as *mut i64; // New apRegion[] array
    let mut __slate_storage_812: std::mem::MaybeUninit<*mut *mut i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_812: *mut *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_812) as *mut *mut *mut i8;
    let mut __slate_storage_811: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_811: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_811) as *mut i32;
    let mut __slate_storage_810: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_810: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_810) as *mut i32;
    let mut __slate_storage_809: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_809: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_809) as *mut i32;
    let mut __slate_storage_808: std::mem::MaybeUninit<*mut unixShmNode> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_808: *mut *mut unixShmNode =
        std::ptr::addr_of_mut!(__slate_storage_808) as *mut *mut unixShmNode;
    let mut __slate_storage_807: std::mem::MaybeUninit<*mut unixShm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_807: *mut *mut unixShm =
        std::ptr::addr_of_mut!(__slate_storage_807) as *mut *mut unixShm;
    let mut __slate_storage_806: std::mem::MaybeUninit<*mut unixFile> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_806: *mut *mut unixFile =
        std::ptr::addr_of_mut!(__slate_storage_806) as *mut *mut unixFile;
    unsafe {
        '__join_34: {
            std::ptr::write(__slate_slot_806, fd as *mut unixFile);
            std::ptr::write(__slate_slot_809, 0 as i32);
            std::ptr::write(__slate_slot_810, unixShmRegionPerMap());
            // If the shared-memory file has not yet been opened, open it now.
            if (unsafe { (*(*__slate_slot_806)).pShm }) == std::ptr::null_mut::<unixShm>() {
                *__slate_slot_809 = unixOpenSharedMemory(*__slate_slot_806);
                if *__slate_slot_809 != (0 as i32) {
                    return *__slate_slot_809;
                }
            }
        }
        '__join_5: {
            *__slate_slot_807 = unsafe { (*(*__slate_slot_806)).pShm };
            *__slate_slot_808 = unsafe { (*(*__slate_slot_807)).pShmNode };
            unsafe { sqlite3_mutex_enter(unsafe { (*(*__slate_slot_808)).pShmMutex }) };
            if (unsafe { (*(*__slate_slot_808)).isUnlocked }) != (0 as u8) {
                *__slate_slot_809 = unixLockSharedMemory(*__slate_slot_806, *__slate_slot_808);
                if *__slate_slot_809 != (0 as i32) {
                    break '__join_5;
                } else {
                    unsafe {
                        (*(*__slate_slot_808)).isUnlocked = ((0 as i32) as i8) as u8;
                    }
                }
            }
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            // Minimum number of regions required to be mapped.
            *__slate_slot_811 =
                (iRegion + *__slate_slot_810) / *__slate_slot_810 * *__slate_slot_810;
            if (((unsafe { (*(*__slate_slot_808)).nRegion }) as u32) as i32) < *__slate_slot_811 {
                '__join_19: {
                    std::ptr::write(
                        __slate_slot_813,
                        (*__slate_slot_811 as i64) * (szRegion as i64),
                    );
                    unsafe {
                        (*(*__slate_slot_808)).szRegion = szRegion;
                    }
                    if (unsafe { (*(*__slate_slot_808)).hShm }) >= (0 as i32) {
                        // The requested region is not mapped into this processes address space.
                        // Check to see if it has been allocated (i.e. if the wal-index file is
                        // large enough to contain the requested region).
                        if (unsafe {
                            unsafe {
                                std::mem::transmute::<
                                    Option<unsafe extern "C-unwind" fn()>,
                                    Option<unsafe extern "C-unwind" fn(i32, *mut stat) -> i32>,
                                >(unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall
                                        }
                                        .offset((5 as i32) as isize)
                                    })
                                    .pCurrent
                                })
                            }
                            .unwrap()(
                                unsafe { (*(*__slate_slot_808)).hShm },
                                std::ptr::addr_of_mut!(*__slate_slot_814),
                            )
                        }) != (0 as i32)
                        {
                            *__slate_slot_809 = (10 as i32) | (19 as i32) << (8 as i32);
                            break '__join_5;
                        } else {
                            if (*__slate_slot_814).st_size < *__slate_slot_813 {
                                // The requested memory region does not exist. If bExtend is set to
                                // false, exit early. *pp will be set to NULL and SQLITE_OK returned.
                                if !(bExtend != (0 as i32)) {
                                    break '__join_5;
                                } else {
                                    // Write to the last byte of each newly allocated or extended page
                                    0 as i32;
                                    *__slate_slot_816 =
                                        (*__slate_slot_814).st_size / ((unsafe { pgsz }) as i64);
                                    '__join_23: {
                                        loop {
                                            if *__slate_slot_816
                                                < *__slate_slot_813 / ((unsafe { pgsz }) as i64)
                                            {
                                                std::ptr::write(__slate_slot_817, 0 as i32);
                                                if seekAndWriteFd(
                                                    unsafe { (*(*__slate_slot_808)).hShm },
                                                    *__slate_slot_816 * ((unsafe { pgsz }) as i64)
                                                        + ((unsafe { pgsz }) as i64)
                                                        - ((1 as i32) as i64),
                                                    (b"\0".as_ptr() as *mut i8) as *const (),
                                                    1 as i32,
                                                    std::ptr::addr_of_mut!(*__slate_slot_817),
                                                ) != (1 as i32)
                                                {
                                                    break '__join_23;
                                                } else {
                                                    std::ptr::write(
                                                        __slate_slot_1369,
                                                        *__slate_slot_816,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1370,
                                                        *__slate_slot_1369 + ((1 as i32) as i64),
                                                    );
                                                    *__slate_slot_816 = *__slate_slot_1370;
                                                }
                                            } else {
                                                break;
                                            }
                                        }
                                        // Alternatively, if bExtend is true, extend the file. Do this by
                                        // writing a single byte to the end of each (OS) page being
                                        // allocated or extended. Technically, we need only write to the
                                        // last page in order to extend the file. But writing to all new
                                        // pages forces the OS to allocate them immediately, which reduces
                                        // the chances of SIGBUS while accessing the mapped region later on.
                                        break '__join_19;
                                    }
                                    std::ptr::write(
                                        __slate_slot_818,
                                        (unsafe { (*(*__slate_slot_808)).zFilename }) as *const i8,
                                    );
                                    *__slate_slot_809 = unixLogErrorAtLine(
                                        (10 as i32) | (19 as i32) << (8 as i32),
                                        (b"write\0".as_ptr() as *mut i8) as *const i8,
                                        *__slate_slot_818,
                                        5184 as i32,
                                    );
                                    break '__join_5;
                                }
                            }
                        }
                    }
                }
                // Map the requested memory region into this processes address space.
                *__slate_slot_812 = (unsafe {
                    sqlite3_realloc64(
                        (unsafe { (*(*__slate_slot_808)).apRegion }) as *mut (),
                        ((*__slate_slot_811 as i64) as u64).wrapping_mul(8 as u64),
                    )
                }) as *mut *mut i8;
                if !(*__slate_slot_812 != std::ptr::null_mut::<*mut i8>()) {
                    *__slate_slot_809 = (10 as i32) | (12 as i32) << (8 as i32);
                } else {
                    unsafe {
                        (*(*__slate_slot_808)).apRegion = *__slate_slot_812;
                    }
                    '__join_11: {
                        '__loop_6: loop {
                            if (((unsafe { (*(*__slate_slot_808)).nRegion }) as u32) as i32)
                                < *__slate_slot_811
                            {
                                std::ptr::write(
                                    __slate_slot_819,
                                    (szRegion as i64) * (*__slate_slot_810 as i64),
                                );
                                if (unsafe { (*(*__slate_slot_808)).hShm }) >= (0 as i32) {
                                    *__slate_slot_821 = unsafe {
                                        unsafe {
                                            std::mem::transmute::<
                                                Option<unsafe extern "C-unwind" fn()>,
                                                Option<
                                                    unsafe extern "C-unwind" fn(
                                                        *mut (),
                                                        u64,
                                                        i32,
                                                        i32,
                                                        i32,
                                                        i64,
                                                    )
                                                        -> *mut (),
                                                >,
                                            >(unsafe {
                                                (*unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of_mut!(aSyscall.0)
                                                            as *mut unix_syscall
                                                    }
                                                    .offset((22 as i32) as isize)
                                                })
                                                .pCurrent
                                            })
                                        }
                                        .unwrap()(
                                            std::ptr::null_mut::<()>(),
                                            *__slate_slot_819 as u64,
                                            if (unsafe { (*(*__slate_slot_808)).isReadonly })
                                                != (0 as u8)
                                            {
                                                1 as i32
                                            } else {
                                                (1 as i32) | (2 as i32)
                                            },
                                            1 as i32,
                                            unsafe { (*(*__slate_slot_808)).hShm },
                                            (szRegion as i64)
                                                * (((unsafe { (*(*__slate_slot_808)).nRegion })
                                                    as u64)
                                                    as i64),
                                        )
                                    };
                                    if *__slate_slot_821 == (-(1 as i32) as *mut ()) {
                                        break '__join_11;
                                    }
                                } else {
                                    *__slate_slot_821 =
                                        unsafe { sqlite3_malloc64(*__slate_slot_819 as u64) };
                                    if *__slate_slot_821 == std::ptr::null_mut::<()>() {
                                        break '__loop_6;
                                    } else {
                                        unsafe {
                                            memset(
                                                *__slate_slot_821,
                                                0 as i32,
                                                *__slate_slot_819 as u64,
                                            )
                                        };
                                    }
                                }
                                *__slate_slot_820 = (0 as i32) as i64;
                                loop {
                                    if *__slate_slot_820 < (*__slate_slot_810 as i64) {
                                        unsafe {
                                            *unsafe {
                                                unsafe { (*(*__slate_slot_808)).apRegion }.offset(
                                                    (((((unsafe { (*(*__slate_slot_808)).nRegion })
                                                        as u32)
                                                        as i32)
                                                        as i64)
                                                        + *__slate_slot_820)
                                                        as isize,
                                                )
                                            } = unsafe {
                                                (*__slate_slot_821 as *mut i8).offset(
                                                    ((szRegion as i64) * *__slate_slot_820)
                                                        as isize,
                                                )
                                            };
                                        }
                                        std::ptr::write(__slate_slot_1371, *__slate_slot_820);
                                        std::ptr::write(
                                            __slate_slot_1372,
                                            *__slate_slot_1371 + ((1 as i32) as i64),
                                        );
                                        *__slate_slot_820 = *__slate_slot_1372;
                                    } else {
                                        break;
                                    }
                                }
                                std::ptr::write(__slate_slot_1373, *__slate_slot_808);
                                std::ptr::write(__slate_slot_1374, unsafe {
                                    (*(*__slate_slot_1373)).nRegion
                                });
                                std::ptr::write(
                                    __slate_slot_1375,
                                    ((((*__slate_slot_1374 as u32) as i32) + *__slate_slot_810)
                                        as i16) as u16,
                                );
                                unsafe {
                                    (*(*__slate_slot_1373)).nRegion = *__slate_slot_1375;
                                }
                            } else {
                                break '__join_5;
                            }
                        }
                        *__slate_slot_809 = 7 as i32;
                        break '__join_5;
                    }
                    *__slate_slot_809 = unixLogErrorAtLine(
                        (10 as i32) | (21 as i32) << (8 as i32),
                        (b"mmap\0".as_ptr() as *mut i8) as *const i8,
                        (unsafe { (*(*__slate_slot_808)).zFilename }) as *const i8,
                        5211 as i32,
                    );
                }
            }
        }
        if (((unsafe { (*(*__slate_slot_808)).nRegion }) as u32) as i32) > iRegion {
            unsafe {
                *pp = (unsafe {
                    *unsafe { unsafe { (*(*__slate_slot_808)).apRegion }.offset(iRegion as isize) }
                }) as *mut ();
            }
        } else {
            unsafe {
                *pp = std::ptr::null_mut::<()>();
            }
        }
        if (unsafe { (*(*__slate_slot_808)).isReadonly }) != (0 as u8)
            && *__slate_slot_809 == (0 as i32)
        {
            *__slate_slot_809 = 8 as i32;
        }
        unsafe { sqlite3_mutex_leave(unsafe { (*(*__slate_slot_808)).pShmMutex }) };
        return *__slate_slot_809;
    }
    return unsafe { std::mem::zeroed() };
}

static mut pgsz: i32 = 4096 as i32;

/// Check that the pShmNode->aLock[] array comports with the locking bitmasks
/// held by each client. Return true if it does, or false otherwise. This
/// is to be used in an assert(). e.g.
///
///     assert( assertLockingArrayOk(pShmNode) );
/// Change the lock state for a shared-memory segment.
///
/// Note that the relationship between SHARED and EXCLUSIVE locks is a little
/// different here than in posix.  In xShmLock(), one can go from unlocked
/// to shared and back or from unlocked to exclusive and back.  But one may
/// not go from shared to exclusive or from exclusive to shared.
///
/// # Arguments
///
/// * `fd` - Database file holding the shared memory
/// * `ofst` - First lock to acquire or release
/// * `n` - Number of locks to acquire or release
/// * `flags` - What to do with the lock
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixShmLock")]
extern "C-unwind" fn unixShmLock(
    mut fd: *mut sqlite3_file,
    mut ofst: i32,
    mut n: i32,
    mut flags: i32,
) -> i32 {
    let mut pDbFd: *mut unixFile = fd as *mut unixFile; // Connection holding shared memory
    let mut p: *mut unixShm = unsafe { std::mem::zeroed() }; // The shared memory being locked
    let mut pShmNode: *mut unixShmNode = unsafe { std::mem::zeroed() }; // The underlying file iNode
    let mut rc: i32 = 0 as i32; // Result code
    let mut mask: u16 = ((((1 as i32) << ofst + n) - ((1 as i32) << ofst)) as i16) as u16; // Mask of locks to take or release
    let mut aLock: *mut i32 = unsafe { std::mem::zeroed() };
    p = unsafe { (*pDbFd).pShm };
    if p == std::ptr::null_mut::<unixShm>() {
        return (10 as i32) | (20 as i32) << (8 as i32);
    }
    pShmNode = unsafe { (*p).pShmNode };
    if pShmNode == std::ptr::null_mut::<unixShmNode>() {
        return (10 as i32) | (20 as i32) << (8 as i32);
    }
    aLock = unsafe { (*pShmNode).aLock.as_mut_ptr() as *mut i32 };
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    // Check that, if this to be a blocking lock, no locks that occur later
    // in the following list than the lock being obtained are already held:
    //
    //   1. Recovery lock (ofst==2).
    //   2. Checkpointer lock (ofst==1).
    //   3. Write lock (ofst==0).
    //   4. Read locks (ofst>=3 && ofst<SQLITE_SHM_NLOCK).
    //
    // In other words, if this is a blocking lock, none of the locks that
    // occur later in the above list than the lock being obtained may be
    // held.
    // Check if there is any work to do. There are three cases:
    //
    //    a) An unlock operation where there are locks to unlock,
    //    b) An shared lock where the requested lock is not already held
    //    c) An exclusive lock where the requested lock is not already held
    //
    // The SQLite core never requests an exclusive lock that it already holds.
    // This is assert()ed below.
    0 as i32;
    if flags & (1 as i32) != (0 as i32)
        && ((((unsafe { (*p).exclMask }) as u32) as i32)
            | (((unsafe { (*p).sharedMask }) as u32) as i32))
            & ((mask as u32) as i32)
            != (0 as i32)
        || flags == (4 as i32) | (2 as i32)
            && (0 as i32) == (((unsafe { (*p).sharedMask }) as u32) as i32) & ((mask as u32) as i32)
        || flags == (8 as i32) | (2 as i32)
    {
        // Take the required mutexes. In SETLK_TIMEOUT mode (blocking locks), if
        // this is an attempt on an exclusive lock use sqlite3_mutex_try(). If any
        // other thread is holding this mutex, then it is either holding or about
        // to hold a lock exclusive to the one being requested, and we may
        // therefore return SQLITE_BUSY to the caller.
        //
        // Doing this prevents some deadlock scenarios. For example, thread 1 may
        // be a checkpointer blocked waiting on the WRITER lock. And thread 2
        // may be a normal SQL client upgrading to a write transaction. In this
        // case thread 2 does a non-blocking request for the WRITER lock. But -
        // if it were to use sqlite3_mutex_enter() then it would effectively
        // become a (doomed) blocking request, as thread 2 would block until thread
        // 1 obtained WRITER and released the mutex. Since thread 2 already holds
        // a lock on a read-locking slot at this point, this breaks the
        // anti-deadlock rules (see above).
        unsafe { sqlite3_mutex_enter(unsafe { (*pShmNode).pShmMutex }) };
        if rc == (0 as i32) {
            if flags & (1 as i32) != (0 as i32) {
                // Case (a) - unlock.
                let mut bUnlock: i32 = 1 as i32;
                0 as i32;
                0 as i32;
                0 as i32;
                // If this is a SHARED lock being unlocked, it is possible that other
                // clients within this process are holding the same SHARED lock. In
                // this case, set bUnlock to 0 so that the posix lock is not removed
                // from the file-descriptor below.
                if flags & (4 as i32) != (0 as i32) {
                    0 as i32;
                    0 as i32;
                    if (unsafe { *unsafe { aLock.offset(ofst as isize) } }) > (1 as i32) {
                        bUnlock = 0 as i32;
                        let __v1376: *mut i32 = unsafe { aLock.offset(ofst as isize) };
                        let __v1377: i32 = unsafe { *__v1376 };
                        let __v1378: i32 = __v1377 - (1 as i32);
                        unsafe {
                            *__v1376 = __v1378;
                        }
                        let __v1379: *mut unixShm = p;
                        let __v1380: u16 = unsafe { (*__v1379).sharedMask };
                        let __v1381: u16 =
                            ((((__v1380 as u32) as i32) & !((mask as u32) as i32)) as i16) as u16;
                        unsafe {
                            (*__v1379).sharedMask = __v1381;
                        }
                    }
                }
                if bUnlock != (0 as i32) {
                    rc = unixShmSystemLock(
                        pDbFd,
                        2 as i32,
                        ofst + ((22 as i32) + (8 as i32)) * (4 as i32),
                        n,
                    );
                    if rc == (0 as i32) {
                        unsafe {
                            memset(
                                (unsafe { aLock.offset(ofst as isize) }) as *mut (),
                                0 as i32,
                                (4 as u64).wrapping_mul((n as i64) as u64),
                            )
                        };
                        let __v1382: *mut unixShm = p;
                        let __v1383: u16 = unsafe { (*__v1382).sharedMask };
                        let __v1384: u16 =
                            ((((__v1383 as u32) as i32) & !((mask as u32) as i32)) as i16) as u16;
                        unsafe {
                            (*__v1382).sharedMask = __v1384;
                        }
                        let __v1385: *mut unixShm = p;
                        let __v1386: u16 = unsafe { (*__v1385).exclMask };
                        let __v1387: u16 =
                            ((((__v1386 as u32) as i32) & !((mask as u32) as i32)) as i16) as u16;
                        unsafe {
                            (*__v1385).exclMask = __v1387;
                        }
                    }
                }
            } else {
                if flags & (4 as i32) != (0 as i32) {
                    // Case (b) - a shared lock.
                    if (unsafe { *unsafe { aLock.offset(ofst as isize) } }) < (0 as i32) {
                        // An exclusive lock is held by some other connection. BUSY.
                        rc = 5 as i32;
                    } else {
                        if (unsafe { *unsafe { aLock.offset(ofst as isize) } }) == (0 as i32) {
                            rc = unixShmSystemLock(
                                pDbFd,
                                0 as i32,
                                ofst + ((22 as i32) + (8 as i32)) * (4 as i32),
                                n,
                            );
                        }
                    }
                    // Get the local shared locks
                    if rc == (0 as i32) {
                        let __v1388: *mut unixShm = p;
                        let __v1389: u16 = unsafe { (*__v1388).sharedMask };
                        let __v1390: u16 =
                            ((((__v1389 as u32) as i32) | ((mask as u32) as i32)) as i16) as u16;
                        unsafe {
                            (*__v1388).sharedMask = __v1390;
                        }
                        let __v1391: *mut i32 = unsafe { aLock.offset(ofst as isize) };
                        let __v1392: i32 = unsafe { *__v1391 };
                        let __v1393: i32 = __v1392 + (1 as i32);
                        unsafe {
                            *__v1391 = __v1393;
                        }
                    }
                } else {
                    // Case (c) - an exclusive lock.
                    let mut ii: i32 = 0 as i32;
                    0 as i32;
                    0 as i32;
                    0 as i32;
                    // Make sure no sibling connections hold locks that will block this
                    // lock.  If any do, return SQLITE_BUSY right away.
                    ii = ofst;
                    '__slate_break_1237: loop {
                        if !(ii < ofst + n) {
                            break;
                        }
                        if (unsafe { *unsafe { aLock.offset(ii as isize) } }) != (0 as i32) {
                            rc = 5 as i32;
                            break '__slate_break_1237;
                        }
                        let __v1394: i32 = ii;
                        let __v1395: i32 = __v1394 + (1 as i32);
                        ii = __v1395;
                    }
                    // Get the exclusive locks at the system level. Then if successful
                    // also update the in-memory values.
                    if rc == (0 as i32) {
                        rc = unixShmSystemLock(
                            pDbFd,
                            1 as i32,
                            ofst + ((22 as i32) + (8 as i32)) * (4 as i32),
                            n,
                        );
                        if rc == (0 as i32) {
                            let __v1396: *mut unixShm = p;
                            let __v1397: u16 = unsafe { (*__v1396).exclMask };
                            let __v1398: u16 = ((((__v1397 as u32) as i32) | ((mask as u32) as i32))
                                as i16) as u16;
                            unsafe {
                                (*__v1396).exclMask = __v1398;
                            }
                            ii = ofst;
                            '__slate_break_1238: loop {
                                if !(ii < ofst + n) {
                                    break;
                                }
                                unsafe {
                                    *unsafe { aLock.offset(ii as isize) } = -(1 as i32);
                                }
                                let __v1399: i32 = ii;
                                let __v1400: i32 = __v1399 + (1 as i32);
                                ii = __v1400;
                            }
                        }
                    }
                }
            }
            0 as i32;
        }
        // Drop the mutexes acquired above.
        unsafe { sqlite3_mutex_leave(unsafe { (*pShmNode).pShmMutex }) };
    }
    {}
    return rc;
}

/// Implement a memory barrier or memory fence on shared memory.
///
/// All loads and stores begun before the barrier must complete before
/// any load or store begun after the barrier.
///
/// # Arguments
///
/// * `fd` - Database file holding the shared memory
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixShmBarrier")]
extern "C-unwind" fn unixShmBarrier(mut fd: *mut sqlite3_file) {
    fd;
    unsafe { sqlite3MemoryBarrier() }; // compiler-defined memory barrier
    0 as i32;
    unixEnterMutex(); // Also mutex, for redundancy
    unixLeaveMutex();
}

/// Close a connection to shared-memory.  Delete the underlying
/// storage if deleteFlag is true.
///
/// If there is no shared memory associated with the connection then this
/// routine is a harmless no-op.
///
/// # Arguments
///
/// * `fd` - The underlying database file
/// * `deleteFlag` - Delete shared-memory if true
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixShmUnmap")]
extern "C-unwind" fn unixShmUnmap(mut fd: *mut sqlite3_file, mut deleteFlag: i32) -> i32 {
    let mut p: *mut unixShm = unsafe { std::mem::zeroed() }; // The connection to be closed
    let mut pShmNode: *mut unixShmNode = unsafe { std::mem::zeroed() }; // The underlying shared-memory file
    let mut pp: *mut *mut unixShm = unsafe { std::mem::zeroed() }; // For looping over sibling connections
    let mut pDbFd: *mut unixFile = unsafe { std::mem::zeroed() }; // The underlying database file
    pDbFd = fd as *mut unixFile;
    p = unsafe { (*pDbFd).pShm };
    if p == std::ptr::null_mut::<unixShm>() {
        return 0 as i32;
    }
    pShmNode = unsafe { (*p).pShmNode };
    0 as i32;
    0 as i32;
    // Remove connection p from the set of connections associated
    // with pShmNode
    unsafe { sqlite3_mutex_enter(unsafe { (*pShmNode).pShmMutex }) };
    pp = unsafe { std::ptr::addr_of_mut!((*pShmNode).pFirst) };
    '__slate_break_1239: while (unsafe { *pp }) != p {
        pp = unsafe { std::ptr::addr_of_mut!((*unsafe { *pp }).pNext) };
    }
    unsafe {
        *pp = unsafe { (*p).pNext };
    }
    // Free the connection p
    unsafe { sqlite3_free(p as *mut ()) };
    unsafe {
        (*pDbFd).pShm = std::ptr::null_mut::<unixShm>();
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*pShmNode).pShmMutex }) };
    // If pShmNode->nRef has reached 0, then close the underlying
    // shared-memory file, too
    0 as i32;
    unixEnterMutex();
    0 as i32;
    let __v1401: *mut unixShmNode = pShmNode;
    let __v1402: i32 = unsafe { (*__v1401).nRef };
    let __v1403: i32 = __v1402 - (1 as i32);
    unsafe {
        (*__v1401).nRef = __v1403;
    }
    if (unsafe { (*pShmNode).nRef }) == (0 as i32) {
        if deleteFlag != (0 as i32) && (unsafe { (*pShmNode).hShm }) >= (0 as i32) {
            unsafe {
                unsafe {
                    std::mem::transmute::<
                        Option<unsafe extern "C-unwind" fn()>,
                        Option<unsafe extern "C-unwind" fn(*const i8) -> i32>,
                    >(unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset((16 as i32) as isize)
                        })
                        .pCurrent
                    })
                }
                .unwrap()((unsafe { (*pShmNode).zFilename }) as *const i8)
            };
        }
        unixShmPurge(pDbFd);
    }
    unixLeaveMutex();
    return 0 as i32;
}

/// If it is currently memory mapped, unmap file pFd.
fn unixUnmapfile(mut pFd: *mut unixFile) {
    0 as i32;
    if (unsafe { (*pFd).pMapRegion }) != std::ptr::null_mut::<()>() {
        unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(*mut (), u64) -> i32>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((23 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(
                unsafe { (*pFd).pMapRegion },
                (unsafe { (*pFd).mmapSizeActual }) as u64,
            )
        };
        unsafe {
            (*pFd).pMapRegion = std::ptr::null_mut::<()>();
        }
        unsafe {
            (*pFd).mmapSize = (0 as i32) as i64;
        }
        unsafe {
            (*pFd).mmapSizeActual = (0 as i32) as i64;
        }
    }
}

/// Attempt to set the size of the memory mapping maintained by file
/// descriptor pFd to nNew bytes. Any existing mapping is discarded.
///
/// If successful, this function sets the following variables:
///
///       unixFile.pMapRegion
///       unixFile.mmapSize
///       unixFile.mmapSizeActual
///
/// If unsuccessful, an error message is logged via sqlite3_log() and
/// the three variables above are zeroed. In this case SQLite should
/// continue accessing the database using the xRead() and xWrite()
/// methods.
///
/// # Arguments
///
/// * `pFd` - File descriptor object
/// * `nNew` - Required mapping size
fn unixRemapfile(mut pFd: *mut unixFile, mut nNew: i64) {
    let mut zErr: *const i8 = (b"mmap\0".as_ptr() as *mut i8) as *const i8;
    let mut h: i32 = unsafe { (*pFd).h }; // File descriptor open on db file
    let mut pOrig: *mut u8 = (unsafe { (*pFd).pMapRegion }) as *mut u8; // Pointer to current file mapping
    let mut pNew: *mut u8 = std::ptr::null_mut::<u8>(); // Location of new mapping
    let mut flags: i32 = 1 as i32; // Flags to pass to mmap()
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if pOrig != std::ptr::null_mut::<u8>() {
        let mut nReuse: i64 = unsafe { (*pFd).mmapSize };
        pNew = (unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(*mut (), u64, u64, i32, ...) -> *mut ()>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((24 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(pOrig as *mut (), nReuse as u64, nNew as u64, 1 as i32)
        }) as *mut u8;
        zErr = (b"mremap\0".as_ptr() as *mut i8) as *const i8;
        // The attempt to extend the existing mapping failed. Free it.
        if pNew == ((-(1 as i32) as *mut ()) as *mut u8) || pNew == std::ptr::null_mut::<u8>() {
            unsafe {
                unsafe {
                    std::mem::transmute::<
                        Option<unsafe extern "C-unwind" fn()>,
                        Option<unsafe extern "C-unwind" fn(*mut (), u64) -> i32>,
                    >(unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset((23 as i32) as isize)
                        })
                        .pCurrent
                    })
                }
                .unwrap()(pOrig as *mut (), nReuse as u64)
            };
        }
    }
    // If pNew is still NULL, try to create an entirely new mapping.
    if pNew == std::ptr::null_mut::<u8>() {
        pNew = (unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<
                        unsafe extern "C-unwind" fn(*mut (), u64, i32, i32, i32, i64) -> *mut (),
                    >,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((22 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(
                std::ptr::null_mut::<()>(),
                nNew as u64,
                flags,
                1 as i32,
                h,
                (0 as i32) as i64,
            )
        }) as *mut u8;
    }
    if pNew == ((-(1 as i32) as *mut ()) as *mut u8) {
        pNew = std::ptr::null_mut::<u8>();
        nNew = (0 as i32) as i64;
        unixLogErrorAtLine(0 as i32, zErr, unsafe { (*pFd).zPath }, 5640 as i32);
        // If the mmap() above failed, assume that all subsequent mmap() calls
        // will probably fail too. Fall back to using xRead/xWrite exclusively
        // in this case.
        unsafe {
            (*pFd).mmapSizeMax = (0 as i32) as i64;
        }
    }
    unsafe {
        (*pFd).pMapRegion = pNew as *mut ();
    }
    let __v1404: i64 = nNew;
    unsafe {
        (*pFd).mmapSizeActual = __v1404;
    }
    unsafe {
        (*pFd).mmapSize = __v1404;
    }
}

/// Memory map or remap the file opened by file-descriptor pFd (if the file
/// is already mapped, the existing mapping is replaced by the new). Or, if
/// there already exists a mapping for this file, and there are still
/// outstanding xFetch() references to it, this function is a no-op.
///
/// If parameter nByte is non-negative, then it is the requested size of
/// the mapping to create. Otherwise, if nByte is less than zero, then the
/// requested size is the size of the file on disk. The actual size of the
/// created mapping is either the requested size or the value configured
/// using SQLITE_FCNTL_MMAP_LIMIT, whichever is smaller.
///
/// SQLITE_OK is returned if no error occurs (even if the mapping is not
/// recreated as a result of outstanding references) or an SQLite error
/// code otherwise.
fn unixMapfile(mut pFd: *mut unixFile, mut nMap: i64) -> i32 {
    0 as i32;
    0 as i32;
    if (unsafe { (*pFd).nFetchOut }) > (0 as i32) {
        return 0 as i32;
    }
    if nMap < ((0 as i32) as i64) {
        let mut statbuf: stat = unsafe { std::mem::zeroed() }; // Low-level file information
        if (unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(i32, *mut stat) -> i32>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((5 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(unsafe { (*pFd).h }, std::ptr::addr_of_mut!(statbuf))
        }) != (0 as i32)
        {
            return (10 as i32) | (7 as i32) << (8 as i32);
        }
        nMap = statbuf.st_size;
    }
    if nMap > unsafe { (*pFd).mmapSizeMax } {
        nMap = unsafe { (*pFd).mmapSizeMax };
    }
    0 as i32;
    if nMap != unsafe { (*pFd).mmapSize } {
        unixRemapfile(pFd, nMap);
    }
    return 0 as i32;
}

/// If possible, return a pointer to a mapping of file fd starting at offset
/// iOff. The mapping must be valid for at least nAmt bytes.
///
/// If such a pointer can be obtained, store it in *pp and return SQLITE_OK.
/// Or, if one cannot but no error occurs, set *pp to 0 and return SQLITE_OK.
/// Finally, if an error does occur, return an SQLite error code. The final
/// value of *pp is undefined in this case.
///
/// If this function does return a pointer, the caller must eventually
/// release the reference by calling unixUnfetch().
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixFetch")]
extern "C-unwind" fn unixFetch(
    mut fd: *mut sqlite3_file,
    mut iOff: i64,
    mut nAmt: i32,
    mut pp: *mut *mut (),
) -> i32 {
    let mut pFd: *mut unixFile = fd as *mut unixFile; // The underlying database file
    unsafe {
        *pp = std::ptr::null_mut::<()>();
    }
    if (unsafe { (*pFd).mmapSizeMax }) > ((0 as i32) as i64) {
        // Ensure that there is always at least a 256 byte buffer of addressable
        // memory following the returned page. If the database is corrupt,
        // SQLite may overread the page slightly (in practice only a few bytes,
        // but 256 is safe, round, number).
        let mut nEofBuffer: i32 = 256 as i32;
        if (unsafe { (*pFd).pMapRegion }) == std::ptr::null_mut::<()>() {
            let mut rc: i32 = unixMapfile(pFd, -(1 as i32) as i64);
            if rc != (0 as i32) {
                return rc;
            }
        }
        if (unsafe { (*pFd).mmapSize }) >= iOff + (nAmt as i64) + (nEofBuffer as i64) {
            unsafe {
                *pp = (unsafe { ((unsafe { (*pFd).pMapRegion }) as *mut u8).offset(iOff as isize) })
                    as *mut ();
            }
            let __v1405: *mut unixFile = pFd;
            let __v1406: i32 = unsafe { (*__v1405).nFetchOut };
            let __v1407: i32 = __v1406 + (1 as i32);
            unsafe {
                (*__v1405).nFetchOut = __v1407;
            }
        }
    }
    return 0 as i32;
}

/// If the third argument is non-NULL, then this function releases a
/// reference obtained by an earlier call to unixFetch(). The second
/// argument passed to this function must be the same as the corresponding
/// argument that was passed to the unixFetch() invocation.
///
/// Or, if the third argument is NULL, then this function is being called
/// to inform the VFS layer that, according to POSIX, any existing mapping
/// may now be invalid and should be unmapped.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixUnfetch")]
extern "C-unwind" fn unixUnfetch(mut fd: *mut sqlite3_file, mut iOff: i64, mut p: *mut ()) -> i32 {
    let mut pFd: *mut unixFile = fd as *mut unixFile; // The underlying database file
    iOff;
    // If p==0 (unmap the entire file) then there must be no outstanding
    // xFetch references. Or, if p!=0 (meaning it is an xFetch reference),
    // then there must be at least one outstanding.
    0 as i32;
    // If p!=0, it must match the iOff value.
    0 as i32;
    if p != std::ptr::null_mut::<()>() {
        let __v1408: *mut unixFile = pFd;
        let __v1409: i32 = unsafe { (*__v1408).nFetchOut };
        let __v1410: i32 = __v1409 - (1 as i32);
        unsafe {
            (*__v1408).nFetchOut = __v1410;
        }
    } else {
        unixUnmapfile(pFd);
    }
    0 as i32;
    return 0 as i32;
}

// Here ends the implementation of all sqlite3_file methods.
//
// End sqlite3_file Methods *******************************
// This division contains definitions of sqlite3_io_methods objects that
// implement various file locking strategies.  It also contains definitions
// of "finder" functions.  A finder-function is used to locate the appropriate
// sqlite3_io_methods object for a particular database file.  The pAppData
// field of the sqlite3_vfs VFS objects are initialized to be pointers to
// the correct finder-function for that VFS.
//
// Most finder functions return a pointer to a fixed sqlite3_io_methods
// object.  The only interesting finder-function is autolockIoFinder, which
// looks at the filesystem type and tries to guess the best locking
// strategy from that.
//
// For finder-function F, two objects are created:
//
//    (1) The real finder-function named "FImpt()".
//
//    (2) A constant pointer to this function named just "F".
//
//
// A pointer to the F pointer is used as the pAppData value for VFS
// objects.  We have to do this instead of letting pAppData point
// directly at the finder-function since C90 rules prevent a void*
// from be cast into a function pointer.
//
//
// Each instance of this macro generates two objects:
//
//   *  A constant sqlite3_io_methods object call METHOD that has locking
//      methods CLOSE, LOCK, UNLOCK, CKRESLOCK.
//
//   *  An I/O method finder function called FINDER that returns a pointer
//      to the METHOD object in the previous bullet.
// iVersion
// xClose
// xRead
// xWrite
// xTruncate
// xSync
// xFileSize
// xLock
// xUnlock
// xCheckReservedLock
// xFileControl
// xSectorSize
// xDeviceCapabilities
// xShmMap
// xShmLock
// xShmBarrier
// xShmUnmap
// xFetch
// xUnfetch
/// Here are all of the sqlite3_io_methods objects for each of the
/// locking strategies.  Functions that return pointers to these methods
/// are also created.
static mut posixIoMethods: sqlite3_io_methods = sqlite3_io_methods {
    iVersion: 3 as i32,
    xClose: Some(unixClose),
    xRead: Some(unixRead),
    xWrite: Some(unixWrite),
    xTruncate: Some(unixTruncate),
    xSync: Some(unixSync),
    xFileSize: Some(unixFileSize),
    xLock: Some(unixLock),
    xUnlock: Some(unixUnlock),
    xCheckReservedLock: Some(unixCheckReservedLock),
    xFileControl: Some(unixFileControl),
    xSectorSize: Some(unixSectorSize),
    xDeviceCharacteristics: Some(unixDeviceCharacteristics),
    xShmMap: Some(unixShmMap),
    xShmLock: Some(unixShmLock),
    xShmBarrier: Some(unixShmBarrier),
    xShmUnmap: Some(unixShmUnmap),
    xFetch: Some(unixFetch),
    xUnfetch: Some(unixUnfetch),
};

#[unsafe(link_section = ".text.slate_distinct.os_unix.posixIoFinderImpl")]
extern "C-unwind" fn posixIoFinderImpl(
    mut z: *const i8,
    mut p: *mut unixFile,
) -> *const sqlite3_io_methods {
    z;
    p;
    return unsafe { std::ptr::addr_of!(posixIoMethods) };
}

/// Finder function name
/// sqlite3_io_methods object name
/// shared memory and mmap are enabled
/// xClose method
/// xLock method
/// xUnlock method
/// xCheckReservedLock method
/// xShmMap method
static mut posixIoFinder: Option<
    unsafe extern "C-unwind" fn(*const i8, *mut unixFile) -> *const sqlite3_io_methods,
> = Some(posixIoFinderImpl);

static mut nolockIoMethods: sqlite3_io_methods = sqlite3_io_methods {
    iVersion: 3 as i32,
    xClose: Some(nolockClose),
    xRead: Some(unixRead),
    xWrite: Some(unixWrite),
    xTruncate: Some(unixTruncate),
    xSync: Some(unixSync),
    xFileSize: Some(unixFileSize),
    xLock: Some(nolockLock),
    xUnlock: Some(nolockUnlock),
    xCheckReservedLock: Some(nolockCheckReservedLock),
    xFileControl: Some(unixFileControl),
    xSectorSize: Some(unixSectorSize),
    xDeviceCharacteristics: Some(unixDeviceCharacteristics),
    xShmMap: None,
    xShmLock: Some(unixShmLock),
    xShmBarrier: Some(unixShmBarrier),
    xShmUnmap: Some(unixShmUnmap),
    xFetch: Some(unixFetch),
    xUnfetch: Some(unixUnfetch),
};

#[unsafe(link_section = ".text.slate_distinct.os_unix.nolockIoFinderImpl")]
extern "C-unwind" fn nolockIoFinderImpl(
    mut z: *const i8,
    mut p: *mut unixFile,
) -> *const sqlite3_io_methods {
    z;
    p;
    return unsafe { std::ptr::addr_of!(nolockIoMethods) };
}

/// Finder function name
/// sqlite3_io_methods object name
/// shared memory and mmap are enabled
/// xClose method
/// xLock method
/// xUnlock method
/// xCheckReservedLock method
/// xShmMap method
static mut nolockIoFinder: Option<
    unsafe extern "C-unwind" fn(*const i8, *mut unixFile) -> *const sqlite3_io_methods,
> = Some(nolockIoFinderImpl);

static mut dotlockIoMethods: sqlite3_io_methods = sqlite3_io_methods {
    iVersion: 1 as i32,
    xClose: Some(dotlockClose),
    xRead: Some(unixRead),
    xWrite: Some(unixWrite),
    xTruncate: Some(unixTruncate),
    xSync: Some(unixSync),
    xFileSize: Some(unixFileSize),
    xLock: Some(dotlockLock),
    xUnlock: Some(dotlockUnlock),
    xCheckReservedLock: Some(dotlockCheckReservedLock),
    xFileControl: Some(unixFileControl),
    xSectorSize: Some(unixSectorSize),
    xDeviceCharacteristics: Some(unixDeviceCharacteristics),
    xShmMap: None,
    xShmLock: Some(unixShmLock),
    xShmBarrier: Some(unixShmBarrier),
    xShmUnmap: Some(unixShmUnmap),
    xFetch: Some(unixFetch),
    xUnfetch: Some(unixUnfetch),
};

#[unsafe(link_section = ".text.slate_distinct.os_unix.dotlockIoFinderImpl")]
extern "C-unwind" fn dotlockIoFinderImpl(
    mut z: *const i8,
    mut p: *mut unixFile,
) -> *const sqlite3_io_methods {
    z;
    p;
    return unsafe { std::ptr::addr_of!(dotlockIoMethods) };
}

/// Finder function name
/// sqlite3_io_methods object name
/// shared memory is disabled
/// xClose method
/// xLock method
/// xUnlock method
/// xCheckReservedLock method
/// xShmMap method
static mut dotlockIoFinder: Option<
    unsafe extern "C-unwind" fn(*const i8, *mut unixFile) -> *const sqlite3_io_methods,
> = Some(dotlockIoFinderImpl);

// sqlite3_vfs methods ****************************
//
// This division contains the implementation of methods on the
// sqlite3_vfs object.
/// The proxy locking method is a "super-method" in the sense that it
/// opens secondary file descriptors for the conch and lock files and
/// it uses proxy, dot-file, AFP, and flock() locking methods on those
/// secondary files.  For this reason, the division that implements
/// proxy locking is located much further down in the file.  But we need
/// to go ahead and define the sqlite3_io_methods and finder function
/// for proxy locking here.  So we forward declare the I/O methods.
/// nfs lockd on OSX 10.3+ doesn't clear write locks when a read lock is set
/// An abstract type for a pointer to an IO method finder function:
/// Initialize the contents of the unixFile structure pointed to by pId.
///
/// # Arguments
///
/// * `pVfs` - Pointer to vfs object
/// * `h` - Open file descriptor of file being opened
/// * `pId` - Write to the unixFile structure here
/// * `zFilename` - Name of the file being opened
/// * `ctrlFlags` - Zero or more UNIXFILE_* values
fn fillInUnixFile(
    mut pVfs: *mut sqlite3_vfs,
    mut h: i32,
    mut pId: *mut sqlite3_file,
    mut zFilename: *const i8,
    mut ctrlFlags: i32,
) -> i32 {
    let mut pLockingStyle: *const sqlite3_io_methods = unsafe { std::mem::zeroed() };
    let mut pNew: *mut unixFile = pId as *mut unixFile;
    let mut rc: i32 = 0 as i32;
    0 as i32;
    // No locking occurs in temporary files
    0 as i32;
    {}
    unsafe {
        (*pNew).h = h;
    }
    unsafe {
        (*pNew).pVfs = pVfs;
    }
    unsafe {
        (*pNew).zPath = zFilename;
    }
    unsafe {
        (*pNew).ctrlFlags = ((ctrlFlags as i8) as u8) as u16;
    }
    unsafe {
        (*pNew).mmapSizeMax = unsafe { sqlite3Config.szMmap };
    }
    if (unsafe {
        sqlite3_uri_boolean(
            if ctrlFlags & (64 as i32) != (0 as i32) {
                zFilename
            } else {
                std::ptr::null::<i8>()
            },
            (b"psow\0".as_ptr() as *mut i8) as *const i8,
            1 as i32,
        )
    }) != (0 as i32)
    {
        let __v1411: *mut unixFile = pNew;
        let __v1412: u16 = unsafe { (*__v1411).ctrlFlags };
        let __v1413: u16 = ((((__v1412 as u32) as i32) | (16 as i32)) as i16) as u16;
        unsafe {
            (*__v1411).ctrlFlags = __v1413;
        }
    }
    if (unsafe {
        strcmp(
            unsafe { (*pVfs).zName },
            (b"unix-excl\0".as_ptr() as *mut i8) as *const i8,
        )
    }) == (0 as i32)
    {
        let __v1414: *mut unixFile = pNew;
        let __v1415: u16 = unsafe { (*__v1414).ctrlFlags };
        let __v1416: u16 = ((((__v1415 as u32) as i32) | (1 as i32)) as i16) as u16;
        unsafe {
            (*__v1414).ctrlFlags = __v1416;
        }
    }
    if ctrlFlags & (128 as i32) != (0 as i32) {
        pLockingStyle = unsafe { std::ptr::addr_of!(nolockIoMethods) };
    } else {
        pLockingStyle = unsafe {
            unsafe {
                *((unsafe { (*pVfs).pAppData })
                    as *mut Option<
                        unsafe extern "C-unwind" fn(
                            *const i8,
                            *mut unixFile,
                        )
                            -> *const sqlite3_io_methods,
                    >)
            }
            .unwrap()(zFilename, pNew)
        };
    }
    if pLockingStyle == unsafe { std::ptr::addr_of!(posixIoMethods) } {
        unixEnterMutex();
        rc = findInodeInfo(pNew, unsafe { std::ptr::addr_of_mut!((*pNew).pInode) });
        if rc != (0 as i32) {
            // If an error occurred in findInodeInfo(), close the file descriptor
            // immediately, before releasing the mutex. findInodeInfo() may fail
            // in two scenarios:
            //
            //   (a) A call to fstat() failed.
            //   (b) A malloc failed.
            //
            // Scenario (b) may only occur if the process is holding no other
            // file descriptors open on the same file. If there were other file
            // descriptors on this file, then no malloc would be required by
            // findInodeInfo(). If this is the case, it is quite safe to close
            // handle h - as it is guaranteed that no posix locks will be released
            // by doing so.
            //
            // If scenario (a) caused the error then things are not so safe. The
            // implicit assumption here is that if fstat() fails, things are in
            // such bad shape that dropping a lock or two doesn't matter much.
            robust_close(pNew, h, 6148 as i32);
            h = -(1 as i32);
        }
        unixLeaveMutex();
    } else {
        if pLockingStyle == unsafe { std::ptr::addr_of!(dotlockIoMethods) } {
            // Dotfile locking uses the file path so it needs to be included in
            // the dotlockLockingContext
            let mut zLockFile: *mut i8 = unsafe { std::mem::zeroed() };
            let mut nFilename: i32 = 0 as i32;
            0 as i32;
            nFilename = (((unsafe { strlen(zFilename) }) as u32) as i32) + (6 as i32);
            zLockFile = (unsafe { sqlite3_malloc64((nFilename as i64) as u64) }) as *mut i8;
            if zLockFile == std::ptr::null_mut::<i8>() {
                rc = 7 as i32;
            } else {
                unsafe {
                    sqlite3_snprintf(
                        nFilename,
                        zLockFile,
                        (b"%s.lock\0".as_ptr() as *mut i8) as *const i8,
                        zFilename,
                    )
                };
            }
            unsafe {
                (*pNew).lockingContext = zLockFile as *mut ();
            }
        }
    }
    storeLastErrno(pNew, 0 as i32);
    if rc != (0 as i32) {
        if h >= (0 as i32) {
            robust_close(pNew, h, 6240 as i32);
        }
    } else {
        unsafe {
            (*pId).pMethods = pLockingStyle;
        }
        {}
        verifyDbFile(pNew);
    }
    return rc;
}

/// Return the name of a directory in which to put temporary files.
/// If no suitable temporary file directory can be found, return NULL.
///
/// The return value might be a string obtained from getenv() and so
/// the return value should not be used after any call to setenv() or
/// putenv() as that value might have been freed.
fn unixTempFileDir() -> *const i8 {
    let mut i: u32 = 0 as u32;
    let mut buf: stat = unsafe { std::mem::zeroed() };
    let mut zDir: *const i8 = unsafe { std::mem::zeroed() };
    i = (0 as i32) as u32;
    '__slate_break_1245: while i < ((7 as i32) as u32) {
        '__slate_break_1246: {
            match i {
                0 => {
                    zDir = (unsafe { sqlite3_temp_directory }) as *const i8;
                }
                1 => {
                    zDir =
                        (unsafe { getenv((b"SQLITE_TMPDIR\0".as_ptr() as *mut i8) as *const i8) })
                            as *const i8;
                }
                2 => {
                    zDir = (unsafe { getenv((b"TMPDIR\0".as_ptr() as *mut i8) as *const i8) })
                        as *const i8;
                }
                3 => {
                    zDir = (b"/var/tmp\0".as_ptr() as *mut i8) as *const i8;
                }
                4 => {
                    zDir = (b"/usr/tmp\0".as_ptr() as *mut i8) as *const i8;
                }
                5 => {
                    zDir = (b"/tmp\0".as_ptr() as *mut i8) as *const i8;
                }
                _ => {
                    zDir = (b".\0".as_ptr() as *mut i8) as *const i8;
                }
            }
        }
        let __v1419: bool;
        if zDir != std::ptr::null::<i8>() {
            __v1419 = (unsafe {
                unsafe {
                    std::mem::transmute::<
                        Option<unsafe extern "C-unwind" fn()>,
                        Option<unsafe extern "C-unwind" fn(*const i8, *mut stat) -> i32>,
                    >(unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset((4 as i32) as isize)
                        })
                        .pCurrent
                    })
                }
                .unwrap()(zDir, std::ptr::addr_of_mut!(buf))
            }) == (0 as i32);
        } else {
            __v1419 = false as bool;
        }
        let __v1420: bool;
        if __v1419 && buf.st_mode & ((61440 as i32) as u32) == ((16384 as i32) as u32) {
            __v1420 = (unsafe {
                unsafe {
                    std::mem::transmute::<
                        Option<unsafe extern "C-unwind" fn()>,
                        Option<unsafe extern "C-unwind" fn(*const i8, i32) -> i32>,
                    >(unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset((2 as i32) as isize)
                        })
                        .pCurrent
                    })
                }
                .unwrap()(zDir, 3 as i32)
            }) == (0 as i32);
        } else {
            __v1420 = false as bool;
        }
        if __v1420 {
            return zDir;
        }
        let __v1417: u32 = i;
        let __v1418: u32 = __v1417.wrapping_add((1 as i32) as u32);
        i = __v1418;
    }
    return std::ptr::null::<i8>();
}

/// Create a temporary file name in zBuf.  zBuf must be allocated
/// by the calling process and must be big enough to hold at least
/// pVfs->mxPathname bytes.
fn unixGetTempname(mut nBuf: i32, mut zBuf: *mut i8) -> i32 {
    let mut zDir: *const i8 = unsafe { std::mem::zeroed() };
    let mut iLimit: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    // It's odd to simulate an io-error here, but really this is just
    // using the io-error infrastructure to test that SQLite handles this
    // function failing.
    unsafe {
        *unsafe { zBuf.offset((0 as i32) as isize) } = (0 as i32) as i8;
    }
    {}
    unsafe { sqlite3_mutex_enter(unsafe { sqlite3MutexAlloc(11 as i32) }) };
    zDir = unixTempFileDir();
    if zDir == std::ptr::null::<i8>() {
        rc = (10 as i32) | (25 as i32) << (8 as i32);
    } else {
        '__slate_break_1253: loop {
            let mut r: u64 = 0 as u64;
            unsafe {
                sqlite3_randomness(
                    ((8 as u64) as u32) as i32,
                    std::ptr::addr_of_mut!(r) as *mut (),
                )
            };
            0 as i32;
            unsafe {
                *unsafe { zBuf.offset((nBuf - (2 as i32)) as isize) } = (0 as i32) as i8;
            }
            unsafe {
                sqlite3_snprintf(
                    nBuf,
                    zBuf,
                    (b"%s/etilqs_%llx%c\0".as_ptr() as *mut i8) as *const i8,
                    zDir,
                    r,
                    0 as i32,
                )
            };
            let __v1353: bool;
            if ((unsafe { *unsafe { zBuf.offset((nBuf - (2 as i32)) as isize) } }) as i32)
                != (0 as i32)
            {
                __v1353 = true as bool;
            } else {
                let __v1354: i32 = iLimit;
                let __v1355: i32 = __v1354 + (1 as i32);
                iLimit = __v1355;
                __v1353 = __v1354 > (10 as i32);
            }
            if __v1353 {
                rc = 1 as i32;
                break '__slate_break_1253;
            }
            if !((unsafe {
                unsafe {
                    std::mem::transmute::<
                        Option<unsafe extern "C-unwind" fn()>,
                        Option<unsafe extern "C-unwind" fn(*const i8, i32) -> i32>,
                    >(unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset((2 as i32) as isize)
                        })
                        .pCurrent
                    })
                }
                .unwrap()(zBuf as *const i8, 0 as i32)
            }) == (0 as i32))
            {
                break;
            }
        }
    }
    unsafe { sqlite3_mutex_leave(unsafe { sqlite3MutexAlloc(11 as i32) }) };
    return rc;
}

/// Search for an unused file descriptor that was opened on the database
/// file (not a journal or super-journal file) identified by pathname
/// zPath with SQLITE_OPEN_XXX flags matching those passed as the second
/// argument to this function.
///
/// Such a file descriptor may exist if a database connection was closed
/// but the associated file descriptor could not be closed because some
/// other file descriptor open on the same file is holding a file-lock.
/// Refer to comments in the unixClose() function and the lengthy comment
/// describing "Posix Advisory Locking" at the start of this file for
/// further details. Also, ticket #4018.
///
/// If a suitable file descriptor is found, then it is returned. If no
/// such file descriptor is located, -1 is returned.
fn findReusableFd(mut zPath: *const i8, mut flags: i32) -> *mut UnixUnusedFd {
    let mut pUnused: *mut UnixUnusedFd = std::ptr::null_mut::<UnixUnusedFd>();
    // Do not search for an unused file descriptor on vxworks. Not because
    // vxworks would not benefit from the change (it might, we're not sure),
    // but because no way to test it is currently available. It is better
    // not to risk breaking vxworks support for the sake of such an obscure
    // feature.
    let mut sStat: stat = unsafe { std::mem::zeroed() }; // Results of stat() call
    unixEnterMutex();
    // A stat() call may fail for various reasons. If this happens, it is
    // almost certain that an open() call on the same path will also fail.
    // For this reason, if an error occurs in the stat() call here, it is
    // ignored and -1 is returned. The caller will try to open a new file
    // descriptor on the same path, fail, and return an error to SQLite.
    //
    // Even if a subsequent open() call does succeed, the consequences of
    // not searching for a reusable file descriptor are not dire.
    let __v1421: bool;
    if (unsafe { inodeList }) != std::ptr::null_mut::<unixInodeInfo>() {
        __v1421 = (0 as i32)
            == unsafe {
                unsafe {
                    std::mem::transmute::<
                        Option<unsafe extern "C-unwind" fn()>,
                        Option<unsafe extern "C-unwind" fn(*const i8, *mut stat) -> i32>,
                    >(unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset((4 as i32) as isize)
                        })
                        .pCurrent
                    })
                }
                .unwrap()(zPath, std::ptr::addr_of_mut!(sStat))
            };
    } else {
        __v1421 = false as bool;
    }
    if __v1421 {
        let mut pInode: *mut unixInodeInfo = unsafe { std::mem::zeroed() };
        pInode = unsafe { inodeList };
        '__slate_break_1255: while pInode != std::ptr::null_mut::<unixInodeInfo>()
            && ((unsafe { (*pInode).fileId.dev }) != sStat.st_dev
                || (unsafe { (*pInode).fileId.ino }) != sStat.st_ino)
        {
            pInode = unsafe { (*pInode).pNext };
        }
        if pInode != std::ptr::null_mut::<unixInodeInfo>() {
            let mut pp: *mut *mut UnixUnusedFd = unsafe { std::mem::zeroed() };
            0 as i32;
            unsafe { sqlite3_mutex_enter(unsafe { (*pInode).pLockMutex }) };
            let __v1422: i32 = flags;
            let __v1423: i32 = __v1422 & ((1 as i32) | (2 as i32));
            flags = __v1423;
            pp = unsafe { std::ptr::addr_of_mut!((*pInode).pUnused) };
            '__slate_break_1256: while (unsafe { *pp }) != std::ptr::null_mut::<UnixUnusedFd>()
                && (unsafe { (*unsafe { *pp }).flags }) != flags
            {
                {}
                pp = unsafe { std::ptr::addr_of_mut!((*unsafe { *pp }).pNext) };
            }
            pUnused = unsafe { *pp };
            if pUnused != std::ptr::null_mut::<UnixUnusedFd>() {
                unsafe {
                    *pp = unsafe { (*pUnused).pNext };
                }
            }
            unsafe { sqlite3_mutex_leave(unsafe { (*pInode).pLockMutex }) };
        }
    }
    unixLeaveMutex();
    return pUnused;
}

/// Find the mode, uid and gid of file zFile.
///
/// # Arguments
///
/// * `zFile` - File name
/// * `pMode` - OUT: Permissions of zFile
/// * `pUid` - OUT: uid of zFile.
/// * `pGid` - OUT: gid of zFile.
fn getFileMode(
    mut zFile: *const i8,
    mut pMode: *mut u32,
    mut pUid: *mut u32,
    mut pGid: *mut u32,
) -> i32 {
    let mut sStat: stat = unsafe { std::mem::zeroed() }; // Output of stat() on database file
    let mut rc: i32 = 0 as i32;
    if (0 as i32)
        == unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(*const i8, *mut stat) -> i32>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((4 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(zFile, std::ptr::addr_of_mut!(sStat))
        }
    {
        unsafe {
            *pMode = sStat.st_mode & ((511 as i32) as u32);
        }
        unsafe {
            *pUid = sStat.st_uid;
        }
        unsafe {
            *pGid = sStat.st_gid;
        }
    } else {
        rc = (10 as i32) | (7 as i32) << (8 as i32);
    }
    return rc;
}

/// This function is called by unixOpen() to determine the unix permissions
/// to create new files with. If no error occurs, then SQLITE_OK is returned
/// and a value suitable for passing as the third argument to open(2) is
/// written to *pMode. If an IO error occurs, an SQLite error code is
/// returned and the value of *pMode is not modified.
///
/// In most cases, this routine sets *pMode to 0, which will become
/// an indication to robust_open() to create the file using
/// SQLITE_DEFAULT_FILE_PERMISSIONS adjusted by the umask.
/// But if the file being opened is a WAL or regular journal file, then
/// this function queries the file-system for the permissions on the
/// corresponding database file and sets *pMode to this value. Whenever
/// possible, WAL and journal files are created using the same permissions
/// as the associated database file.
///
/// If the SQLITE_ENABLE_8_3_NAMES option is enabled, then the
/// original filename is unavailable.  But 8_3_NAMES is only used for
/// FAT filesystems and permissions do not matter there, so just use
/// the default permissions.  In 8_3_NAMES mode, leave *pMode set to zero.
///
/// # Arguments
///
/// * `zPath` - Path of file (possibly) being created
/// * `flags` - Flags passed as 4th argument to xOpen()
/// * `pMode` - OUT: Permissions to open file with
/// * `pUid` - OUT: uid to set on the file
/// * `pGid` - OUT: gid to set on the file
fn findCreateFileMode(
    mut zPath: *const i8,
    mut flags: i32,
    mut pMode: *mut u32,
    mut pUid: *mut u32,
    mut pGid: *mut u32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return Code
    unsafe {
        *pMode = (0 as i32) as u32;
    }
    unsafe {
        *pUid = (0 as i32) as u32;
    }
    unsafe {
        *pGid = (0 as i32) as u32;
    }
    if flags & ((524288 as i32) | (2048 as i32)) != (0 as i32) {
        let mut zDb: __SlateAlign16<[i8; 513]> = __SlateAlign16([0 as i8; 513]); // Database file path
        let mut nDb: i32 = 0 as i32; // Number of valid bytes in zDb
        // zPath is a path to a WAL or journal file. The following block derives
        // the path to the associated database file from zPath. This block handles
        // the following naming conventions:
        //
        //   "<path to db>-journal"
        //   "<path to db>-wal"
        //   "<path to db>-journalNN"
        //   "<path to db>-walNN"
        //
        // where NN is a decimal number. The NN naming schemes are
        // used by the test_multiplex.c module.
        //
        // In normal operation, the journal file name will always contain
        // a '-' character.  However in 8+3 filename mode, or if a corrupt
        // rollback journal specifies a super-journal with a goofy name, then
        // the '-' might be missing or the '-' might be the first character in
        // the filename.  In that case, just return SQLITE_OK with *pMode==0.
        nDb = (unsafe { sqlite3Strlen30(zPath) }) - (1 as i32);
        '__slate_break_1257: while nDb > (0 as i32)
            && ((unsafe { *unsafe { zPath.offset(nDb as isize) } }) as i32) != (46 as i32)
        {
            if ((unsafe { *unsafe { zPath.offset(nDb as isize) } }) as i32) == (45 as i32) {
                unsafe {
                    memcpy(
                        (zDb.0.as_mut_ptr() as *mut i8) as *mut (),
                        zPath as *const (),
                        (nDb as i64) as u64,
                    )
                };
                unsafe {
                    *unsafe { (zDb.0.as_mut_ptr() as *mut i8).offset(nDb as isize) } =
                        (0 as i32) as i8;
                }
                rc = getFileMode(
                    (zDb.0.as_mut_ptr() as *mut i8) as *const i8,
                    pMode,
                    pUid,
                    pGid,
                );
                break '__slate_break_1257;
            }
            let __v1424: i32 = nDb;
            let __v1425: i32 = __v1424 - (1 as i32);
            nDb = __v1425;
        }
    } else {
        if flags & (8 as i32) != (0 as i32) {
            unsafe {
                *pMode = (384 as i32) as u32;
            }
        } else {
            if flags & (64 as i32) != (0 as i32) {
                // If this is a main database file and the file was opened using a URI
                // filename, check for the "modeof" parameter. If present, interpret
                // its value as a filename and try to copy the mode, uid and gid from
                // that file.
                let mut z: *const i8 = unsafe {
                    sqlite3_uri_parameter(zPath, (b"modeof\0".as_ptr() as *mut i8) as *const i8)
                };
                if z != std::ptr::null::<i8>() {
                    rc = getFileMode(z, pMode, pUid, pGid);
                }
            }
        }
    }
    return rc;
}

/// Open the file zPath.
///
/// Previously, the SQLite OS layer used three functions in place of this
/// one:
///
///     sqlite3OsOpenReadWrite();
///     sqlite3OsOpenReadOnly();
///     sqlite3OsOpenExclusive();
///
/// These calls correspond to the following combinations of flags:
///
///     ReadWrite() ->     (READWRITE | CREATE)
///     ReadOnly()  ->     (READONLY)
///     OpenExclusive() -> (READWRITE | CREATE | EXCLUSIVE)
///
/// The old OpenExclusive() accepted a boolean argument - "delFlag". If
/// true, the file was configured to be automatically deleted when the
/// file handle closed. To achieve the same effect using this new
/// interface, add the DELETEONCLOSE flag to those specified above for
/// OpenExclusive().
///
/// # Arguments
///
/// * `pVfs` - The VFS for which this is the xOpen method
/// * `zPath` - Pathname of file to be opened
/// * `pFile` - The file descriptor to be filled in
/// * `flags` - Input flags to control the opening
/// * `pOutFlags` - Output flags returned to SQLite core
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixOpen")]
extern "C-unwind" fn unixOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const i8,
    mut pFile: *mut sqlite3_file,
    mut flags: i32,
    mut pOutFlags: *mut i32,
) -> i32 {
    let mut __slate_storage_1456: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1456: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1456) as *mut i32;
    let mut __slate_storage_1455: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1455: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1455) as *mut i32;
    let mut __slate_storage_1454: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1454: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1454) as *mut i32;
    let mut __slate_storage_1453: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1453: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1453) as *mut i32;
    let mut __slate_storage_1452: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1452: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1452) as *mut i32;
    let mut __slate_storage_1451: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1451: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1451) as *mut i32;
    let mut __slate_storage_1450: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1450: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1450) as *mut i32;
    let mut __slate_storage_1449: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1449: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1449) as *mut i32;
    let mut __slate_storage_1448: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1448: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1448) as *mut i32;
    let mut __slate_storage_1447: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1447: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1447) as *mut i32;
    let mut __slate_storage_961: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_961: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_961) as *mut i32;
    let mut __slate_storage_1446: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1446: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1446) as *mut i32;
    let mut __slate_storage_1445: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1445: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1445) as *mut i32;
    let mut __slate_storage_1444: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1444: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1444) as *mut i32;
    let mut __slate_storage_1443: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1443: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1443) as *mut i32;
    let mut __slate_storage_1442: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1442: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1442) as *mut i32;
    let mut __slate_storage_1441: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1441: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1441) as *mut i32;
    let mut __slate_storage_1440: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1440: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1440) as *mut i32;
    let mut __slate_storage_1439: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1439: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1439) as *mut i32;
    // Failed to open the file for read/write access. Try read-only.
    let mut __slate_storage_960: std::mem::MaybeUninit<*mut UnixUnusedFd> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_960: *mut *mut UnixUnusedFd =
        std::ptr::addr_of_mut!(__slate_storage_960) as *mut *mut UnixUnusedFd;
    let mut __slate_storage_1438: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1438: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1438) as *mut bool; // Groupid for the file
    let mut __slate_storage_959: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_959: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_959) as *mut u32; // Userid for the file
    let mut __slate_storage_958: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_958: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_958) as *mut u32; // Permissions to create file with
    let mut __slate_storage_957: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_957: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_957) as *mut u32;
    let mut __slate_storage_1437: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1437: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1437) as *mut i32;
    let mut __slate_storage_1436: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1436: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1436) as *mut i32;
    let mut __slate_storage_1435: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1435: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1435) as *mut i32;
    let mut __slate_storage_1434: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1434: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1434) as *mut i32;
    let mut __slate_storage_1433: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1433: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1433) as *mut i32;
    let mut __slate_storage_1432: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1432: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1432) as *mut i32;
    let mut __slate_storage_1431: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1431: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1431) as *mut i32;
    let mut __slate_storage_1430: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1430: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1430) as *mut i32;
    let mut __slate_storage_1429: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1429: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1429) as *mut i32;
    let mut __slate_storage_1428: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1428: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1428) as *mut i32;
    let mut __slate_storage_956: std::mem::MaybeUninit<*mut UnixUnusedFd> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_956: *mut *mut UnixUnusedFd =
        std::ptr::addr_of_mut!(__slate_storage_956) as *mut *mut UnixUnusedFd;
    let mut __slate_storage_1427: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1427: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1427) as *mut i32;
    let mut __slate_storage_1426: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1426: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1426) as *mut bool;
    let mut __slate_storage_955: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_955: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_955) as *mut *const i8;
    // If argument zPath is a NULL pointer, this function is required to open
    // a temporary file. Use this buffer to store the file name in.
    let mut __slate_storage_954: std::mem::MaybeUninit<__SlateAlign16<[i8; 514]>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_954: *mut [i8; 514] =
        std::ptr::addr_of_mut!(__slate_storage_954) as *mut [i8; 514];
    // If creating a super- or main-file journal, this function will open
    // a file-descriptor on the directory too. The first time unixSync()
    // is called the directory file descriptor will be fsync()ed and close()d.
    let mut __slate_storage_953: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_953: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_953) as *mut i32;
    let mut __slate_storage_952: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_952: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_952) as *mut i32;
    let mut __slate_storage_951: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_951: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_951) as *mut i32;
    let mut __slate_storage_950: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_950: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_950) as *mut i32;
    let mut __slate_storage_949: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_949: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_949) as *mut i32;
    let mut __slate_storage_948: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_948: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_948) as *mut i32; // UNIXFILE_* flags
    let mut __slate_storage_947: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_947: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_947) as *mut i32; // Function Return Code
    let mut __slate_storage_946: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_946: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_946) as *mut i32; // True to omit locking primitives
    let mut __slate_storage_945: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_945: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_945) as *mut i32; // Type of file to open
    let mut __slate_storage_944: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_944: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_944) as *mut i32; // Flags to pass to open()
    let mut __slate_storage_943: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_943: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_943) as *mut i32; // File descriptor returned by open()
    let mut __slate_storage_942: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_942: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_942) as *mut i32;
    let mut __slate_storage_941: std::mem::MaybeUninit<*mut unixFile> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_941: *mut *mut unixFile =
        std::ptr::addr_of_mut!(__slate_storage_941) as *mut *mut unixFile;
    unsafe {
        '__join_60: {
            std::ptr::write(__slate_slot_941, pFile as *mut unixFile);
            std::ptr::write(__slate_slot_942, -(1 as i32));
            std::ptr::write(__slate_slot_943, 0 as i32);
            std::ptr::write(__slate_slot_944, flags & (1048320 as i32));
            std::ptr::write(__slate_slot_946, 0 as i32);
            std::ptr::write(__slate_slot_947, 0 as i32);
            std::ptr::write(__slate_slot_948, flags & (16 as i32));
            std::ptr::write(__slate_slot_949, flags & (8 as i32));
            std::ptr::write(__slate_slot_950, flags & (4 as i32));
            std::ptr::write(__slate_slot_951, flags & (1 as i32));
            std::ptr::write(__slate_slot_952, flags & (2 as i32));
            std::ptr::write(
                __slate_slot_953,
                (*__slate_slot_950 != (0 as i32)
                    && (*__slate_slot_944 == (16384 as i32)
                        || *__slate_slot_944 == (2048 as i32)
                        || *__slate_slot_944 == (524288 as i32))) as i32,
            );
            std::ptr::write(__slate_slot_955, zPath);
            // Check the following statements are true:
            //
            // (a) Exactly one of the READWRITE and READONLY flags must be set, and
            // (b) if CREATE is set, then READWRITE must also be set, and
            // (c) if EXCLUSIVE is set, then CREATE must also be set.
            // (d) if DELETEONCLOSE is set, then CREATE must also be set.
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            // The main DB, main journal, WAL file and super-journal are never
            // automatically deleted. Nor are they ever temporary files.
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            // Assert that the upper layer has set one of the "file-type" flags.
            0 as i32;
            // Detect a pid change and reset the PRNG.  There is a race condition
            // here such that two or more threads all trying to open databases at
            // the same instant might all reset the PRNG.  But multiple resets
            // are harmless.
            if (unsafe { randomnessPid }) != unsafe { getpid() } {
                unsafe {
                    randomnessPid = unsafe { getpid() };
                }
                unsafe { sqlite3_randomness(0 as i32, std::ptr::null_mut::<()>()) };
            }
        }
        '__join_2: {
            unsafe { memset(*__slate_slot_941 as *mut (), 0 as i32, 120 as u64) };
            if *__slate_slot_944 == (256 as i32) {
                *__slate_slot_956 = findReusableFd(*__slate_slot_955, flags);
                if *__slate_slot_956 != std::ptr::null_mut::<UnixUnusedFd>() {
                    *__slate_slot_942 = unsafe { (*(*__slate_slot_956)).fd };
                } else {
                    *__slate_slot_956 =
                        (unsafe { sqlite3_malloc64(16 as u64) }) as *mut UnixUnusedFd;
                    if !(*__slate_slot_956 != std::ptr::null_mut::<UnixUnusedFd>()) {
                        return 7 as i32;
                    }
                }
                unsafe {
                    (*(*__slate_slot_941)).pPreallocatedUnused = *__slate_slot_956;
                }
                // Database filenames are double-zero terminated if they are not
                // URIs with parameters.  Hence, they can always be passed into
                // sqlite3_uri_parameter().
                0 as i32;
            } else {
                if !(*__slate_slot_955 != std::ptr::null::<i8>()) {
                    // If zName is NULL, the upper layer is requesting a temp file.
                    0 as i32;
                    0 as i32;
                    0 as i32;
                    0 as i32;
                    // On systems that support O_TMPFILE, use that flag to create a more
                    // secure temporary file that cannot be accessed by other processes
                    *__slate_slot_955 = unixTempFileDir();
                    if *__slate_slot_955 != std::ptr::null::<i8>() {
                        std::ptr::write(
                            __slate_slot_1427,
                            robust_open(
                                *__slate_slot_955,
                                (2 as i32)
                                    | (64 as i32)
                                    | (128 as i32)
                                    | ((4194304 as i32) | (65536 as i32)),
                                (384 as i32) as u32,
                            ),
                        );
                        *__slate_slot_942 = *__slate_slot_1427;
                        *__slate_slot_1426 = *__slate_slot_1427 >= (0 as i32);
                    } else {
                        *__slate_slot_1426 = false as bool;
                    }
                    if *__slate_slot_1426 {
                        *__slate_slot_946 = fillInUnixFile(
                            pVfs,
                            *__slate_slot_942,
                            pFile,
                            zPath,
                            *__slate_slot_947,
                        );
                        break '__join_2;
                    } else {
                        *__slate_slot_946 = unixGetTempname(
                            unsafe { (*pVfs).mxPathname },
                            (*__slate_slot_954).as_mut_ptr() as *mut i8,
                        );
                        if *__slate_slot_946 != (0 as i32) {
                            return *__slate_slot_946;
                        } else {
                            *__slate_slot_955 =
                                ((*__slate_slot_954).as_mut_ptr() as *mut i8) as *const i8;
                            // Generated temporary filenames are always double-zero terminated
                            // for use by sqlite3_uri_parameter().
                            0 as i32;
                        }
                    }
                }
            }
            // Determine the value of the flags parameter passed to POSIX function
            // open(). These must be calculated even if open() is not called, as
            // they may be stored as part of the file handle and used by the
            // 'conch file' locking functions later on.
            if *__slate_slot_951 != (0 as i32) {
                std::ptr::write(__slate_slot_1428, *__slate_slot_943);
                std::ptr::write(__slate_slot_1429, *__slate_slot_1428 | (0 as i32));
                *__slate_slot_943 = *__slate_slot_1429;
            }
            if *__slate_slot_952 != (0 as i32) {
                std::ptr::write(__slate_slot_1430, *__slate_slot_943);
                std::ptr::write(__slate_slot_1431, *__slate_slot_1430 | (2 as i32));
                *__slate_slot_943 = *__slate_slot_1431;
            }
            if *__slate_slot_950 != (0 as i32) {
                std::ptr::write(__slate_slot_1432, *__slate_slot_943);
                std::ptr::write(__slate_slot_1433, *__slate_slot_1432 | (64 as i32));
                *__slate_slot_943 = *__slate_slot_1433;
            }
            if *__slate_slot_948 != (0 as i32) {
                std::ptr::write(__slate_slot_1434, *__slate_slot_943);
                std::ptr::write(
                    __slate_slot_1435,
                    *__slate_slot_1434 | ((128 as i32) | (131072 as i32)),
                );
                *__slate_slot_943 = *__slate_slot_1435;
            }
            std::ptr::write(__slate_slot_1436, *__slate_slot_943);
            std::ptr::write(
                __slate_slot_1437,
                *__slate_slot_1436 | ((0 as i32) | (0 as i32) | (131072 as i32)),
            );
            *__slate_slot_943 = *__slate_slot_1437;
            if *__slate_slot_942 < (0 as i32) {
                *__slate_slot_946 = findCreateFileMode(
                    *__slate_slot_955,
                    flags,
                    std::ptr::addr_of_mut!(*__slate_slot_957),
                    std::ptr::addr_of_mut!(*__slate_slot_958),
                    std::ptr::addr_of_mut!(*__slate_slot_959),
                );
                if *__slate_slot_946 != (0 as i32) {
                    0 as i32;
                    0 as i32;
                    return *__slate_slot_946;
                } else {
                    *__slate_slot_942 =
                        robust_open(*__slate_slot_955, *__slate_slot_943, *__slate_slot_957);
                    {}
                    0 as i32;
                    if *__slate_slot_942 < (0 as i32) {
                        '__join_30: {
                            if *__slate_slot_953 != (0 as i32)
                                && (unsafe { *unsafe { __errno_location() } }) == (13 as i32)
                            {
                                *__slate_slot_1438 = (unsafe {
                                    unsafe {
                                        std::mem::transmute::<
                                            Option<unsafe extern "C-unwind" fn()>,
                                            Option<
                                                unsafe extern "C-unwind" fn(*const i8, i32) -> i32,
                                            >,
                                        >(unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!(aSyscall.0)
                                                        as *mut unix_syscall
                                                }
                                                .offset((2 as i32) as isize)
                                            })
                                            .pCurrent
                                        })
                                    }
                                    .unwrap()(
                                        *__slate_slot_955, 0 as i32
                                    )
                                }) != (0 as i32);
                            } else {
                                *__slate_slot_1438 = false as bool;
                            }
                        }
                        if *__slate_slot_1438 {
                            // If unable to create a journal because the directory is not
                            // writable, change the error code to indicate that.
                            *__slate_slot_946 = (8 as i32) | (6 as i32) << (8 as i32);
                        } else {
                            if (unsafe { *unsafe { __errno_location() } }) != (21 as i32)
                                && *__slate_slot_952 != (0 as i32)
                            {
                                std::ptr::write(
                                    __slate_slot_960,
                                    std::ptr::null_mut::<UnixUnusedFd>(),
                                );
                                std::ptr::write(__slate_slot_1439, flags);
                                std::ptr::write(
                                    __slate_slot_1440,
                                    *__slate_slot_1439 & !((2 as i32) | (4 as i32)),
                                );
                                flags = *__slate_slot_1440;
                                std::ptr::write(__slate_slot_1441, *__slate_slot_943);
                                std::ptr::write(
                                    __slate_slot_1442,
                                    *__slate_slot_1441 & !((2 as i32) | (64 as i32)),
                                );
                                *__slate_slot_943 = *__slate_slot_1442;
                                std::ptr::write(__slate_slot_1443, flags);
                                std::ptr::write(__slate_slot_1444, *__slate_slot_1443 | (1 as i32));
                                flags = *__slate_slot_1444;
                                std::ptr::write(__slate_slot_1445, *__slate_slot_943);
                                std::ptr::write(__slate_slot_1446, *__slate_slot_1445 | (0 as i32));
                                *__slate_slot_943 = *__slate_slot_1446;
                                *__slate_slot_951 = 1 as i32;
                                *__slate_slot_960 = findReusableFd(*__slate_slot_955, flags);
                                if *__slate_slot_960 != std::ptr::null_mut::<UnixUnusedFd>() {
                                    *__slate_slot_942 = unsafe { (*(*__slate_slot_960)).fd };
                                    unsafe { sqlite3_free(*__slate_slot_960 as *mut ()) };
                                } else {
                                    *__slate_slot_942 = robust_open(
                                        *__slate_slot_955,
                                        *__slate_slot_943,
                                        *__slate_slot_957,
                                    );
                                }
                            }
                        }
                    }
                    if *__slate_slot_942 < (0 as i32) {
                        std::ptr::write(
                            __slate_slot_961,
                            unixLogErrorAtLine(
                                unsafe { sqlite3CantopenError(6703 as i32) },
                                (b"open\0".as_ptr() as *mut i8) as *const i8,
                                *__slate_slot_955,
                                6703 as i32,
                            ),
                        );
                        if *__slate_slot_946 == (0 as i32) {
                            *__slate_slot_946 = *__slate_slot_961;
                            break '__join_2;
                        } else {
                            break '__join_2;
                        }
                    } else {
                        // The owner of the rollback journal or WAL file should always be the
                        // same as the owner of the database file.  Try to ensure that this is
                        // the case.  The chown() system call will be a no-op if the current
                        // process lacks root privileges, be we should at least try.  Without
                        // this step, if a root process opens a database file, it can leave
                        // behinds a journal/WAL that is owned by root and hence make the
                        // database inaccessible to unprivileged processes.
                        //
                        // If openMode==0, then that means uid and gid are not set correctly
                        // (probably because SQLite is configured to use 8+3 filename mode) and
                        // in that case we do not want to attempt the chown().
                        if *__slate_slot_957 != (0 as u32)
                            && flags & ((524288 as i32) | (2048 as i32)) != (0 as i32)
                        {
                            robustFchown(*__slate_slot_942, *__slate_slot_958, *__slate_slot_959);
                        }
                    }
                }
            }
            0 as i32;
            if pOutFlags != std::ptr::null_mut::<i32>() {
                unsafe {
                    *pOutFlags = flags;
                }
            }
            if (unsafe { (*(*__slate_slot_941)).pPreallocatedUnused })
                != std::ptr::null_mut::<UnixUnusedFd>()
            {
                unsafe {
                    (*unsafe { (*(*__slate_slot_941)).pPreallocatedUnused }).fd = *__slate_slot_942;
                }
                unsafe {
                    (*unsafe { (*(*__slate_slot_941)).pPreallocatedUnused }).flags =
                        flags & ((1 as i32) | (2 as i32));
                }
            }
            if *__slate_slot_949 != (0 as i32) {
                unsafe {
                    unsafe {
                        std::mem::transmute::<
                            Option<unsafe extern "C-unwind" fn()>,
                            Option<unsafe extern "C-unwind" fn(*const i8) -> i32>,
                        >(unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                    .offset((16 as i32) as isize)
                            })
                            .pCurrent
                        })
                    }
                    .unwrap()(*__slate_slot_955)
                };
            }
            // Set up appropriate ctrlFlags
            if *__slate_slot_949 != (0 as i32) {
                std::ptr::write(__slate_slot_1447, *__slate_slot_947);
                std::ptr::write(__slate_slot_1448, *__slate_slot_1447 | (32 as i32));
                *__slate_slot_947 = *__slate_slot_1448;
            }
            if *__slate_slot_951 != (0 as i32) {
                std::ptr::write(__slate_slot_1449, *__slate_slot_947);
                std::ptr::write(__slate_slot_1450, *__slate_slot_1449 | (2 as i32));
                *__slate_slot_947 = *__slate_slot_1450;
            }
            *__slate_slot_945 = (*__slate_slot_944 != (256 as i32)) as i32;
            if *__slate_slot_945 != (0 as i32) {
                std::ptr::write(__slate_slot_1451, *__slate_slot_947);
                std::ptr::write(__slate_slot_1452, *__slate_slot_1451 | (128 as i32));
                *__slate_slot_947 = *__slate_slot_1452;
            }
            if *__slate_slot_953 != (0 as i32) {
                std::ptr::write(__slate_slot_1453, *__slate_slot_947);
                std::ptr::write(__slate_slot_1454, *__slate_slot_1453 | (8 as i32));
                *__slate_slot_947 = *__slate_slot_1454;
            }
            if flags & (64 as i32) != (0 as i32) {
                std::ptr::write(__slate_slot_1455, *__slate_slot_947);
                std::ptr::write(__slate_slot_1456, *__slate_slot_1455 | (64 as i32));
                *__slate_slot_947 = *__slate_slot_1456;
            }
            0 as i32;
            *__slate_slot_946 =
                fillInUnixFile(pVfs, *__slate_slot_942, pFile, zPath, *__slate_slot_947);
        }
        if *__slate_slot_946 != (0 as i32) {
            unsafe {
                sqlite3_free((unsafe { (*(*__slate_slot_941)).pPreallocatedUnused }) as *mut ())
            };
        }
        return *__slate_slot_946;
    }
    return unsafe { std::mem::zeroed() };
}

/// Delete the file at zPath. If the dirSync argument is true, fsync()
/// the directory after deleting the file.
///
/// # Arguments
///
/// * `NotUsed` - VFS containing this as the xDelete method
/// * `zPath` - Name of file to be deleted
/// * `dirSync` - If true, fsync() directory after deleting file
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixDelete")]
extern "C-unwind" fn unixDelete(
    mut NotUsed: *mut sqlite3_vfs,
    mut zPath: *const i8,
    mut dirSync: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    NotUsed;
    {}
    if (unsafe {
        unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(*const i8) -> i32>,
            >(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                        .offset((16 as i32) as isize)
                })
                .pCurrent
            })
        }
        .unwrap()(zPath)
    }) == -(1 as i32)
    {
        if (unsafe { *unsafe { __errno_location() } }) == (2 as i32) {
            rc = (10 as i32) | (23 as i32) << (8 as i32);
        } else {
            rc = unixLogErrorAtLine(
                (10 as i32) | (10 as i32) << (8 as i32),
                (b"unlink\0".as_ptr() as *mut i8) as *const i8,
                zPath,
                6845 as i32,
            );
        }
        return rc;
    }
    if dirSync & (1 as i32) != (0 as i32) {
        let mut fd: i32 = 0 as i32;
        rc = unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(*const i8, *mut i32) -> i32>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((17 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(zPath, std::ptr::addr_of_mut!(fd))
        };
        if rc == (0 as i32) {
            if full_fsync(fd, 0 as i32, 0 as i32) != (0 as i32) {
                rc = unixLogErrorAtLine(
                    (10 as i32) | (5 as i32) << (8 as i32),
                    (b"fsync\0".as_ptr() as *mut i8) as *const i8,
                    zPath,
                    6855 as i32,
                );
            }
            robust_close(std::ptr::null_mut::<unixFile>(), fd, 6857 as i32);
        } else {
            0 as i32;
            rc = 0 as i32;
        }
    }
    return rc;
}

/// Test the existence of or access permissions of file zPath. The
/// test performed depends on the value of flags:
///
///     SQLITE_ACCESS_EXISTS: Return 1 if the file exists
///     SQLITE_ACCESS_READWRITE: Return 1 if the file is read and writable.
///     SQLITE_ACCESS_READONLY: Return 1 if the file is readable.
///
/// Otherwise return 0.
///
/// # Arguments
///
/// * `NotUsed` - The VFS containing this xAccess method
/// * `zPath` - Path of the file to examine
/// * `flags` - What do we want to learn about the zPath file?
/// * `pResOut` - Write result boolean here
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixAccess")]
extern "C-unwind" fn unixAccess(
    mut NotUsed: *mut sqlite3_vfs,
    mut zPath: *const i8,
    mut flags: i32,
    mut pResOut: *mut i32,
) -> i32 {
    NotUsed;
    {}
    0 as i32;
    // The spec says there are three possible values for flags.  But only
    // two of them are actually used
    0 as i32;
    if flags == (0 as i32) {
        let mut buf: stat = unsafe { std::mem::zeroed() };
        unsafe {
            *pResOut = ((0 as i32)
                == unsafe {
                    unsafe {
                        std::mem::transmute::<
                            Option<unsafe extern "C-unwind" fn()>,
                            Option<unsafe extern "C-unwind" fn(*const i8, *mut stat) -> i32>,
                        >(unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                    .offset((4 as i32) as isize)
                            })
                            .pCurrent
                        })
                    }
                    .unwrap()(zPath, std::ptr::addr_of_mut!(buf))
                }
                && (!(buf.st_mode & ((61440 as i32) as u32) == ((32768 as i32) as u32))
                    || buf.st_size > ((0 as i32) as i64))) as i32;
        }
    } else {
        unsafe {
            *pResOut = ((unsafe {
                unsafe {
                    std::mem::transmute::<
                        Option<unsafe extern "C-unwind" fn()>,
                        Option<unsafe extern "C-unwind" fn(*const i8, i32) -> i32>,
                    >(unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset((2 as i32) as isize)
                        })
                        .pCurrent
                    })
                }
                .unwrap()(zPath, (2 as i32) | (4 as i32))
            }) == (0 as i32)) as i32;
        }
    }
    return 0 as i32;
}

/// A pathname under construction
#[repr(C)]
#[derive(Clone, Copy)]
struct DbPath {
    /// Non-zero following any error
    rc: i32,
    /// Number of symlinks resolved
    nSymlink: i32,
    /// Write the pathname here
    zOut: *mut i8,
    /// Bytes of space available to zOut[]
    nOut: i32,
    /// Bytes of zOut[] currently being used
    nUsed: i32,
}

/// Append a single path element to the DbPath under construction
///
/// # Arguments
///
/// * `pPath` - Path under construction, to which to append zName
/// * `zName` - Name to append to pPath.  Not zero-terminated
/// * `nName` - Number of significant bytes in zName
fn appendOnePathElement(mut pPath: *mut DbPath, mut zName: *const i8, mut nName: i32) {
    0 as i32;
    0 as i32;
    if ((unsafe { *unsafe { zName.offset((0 as i32) as isize) } }) as i32) == (46 as i32) {
        if nName == (1 as i32) {
            return;
        }
        if ((unsafe { *unsafe { zName.offset((1 as i32) as isize) } }) as i32) == (46 as i32)
            && nName == (2 as i32)
        {
            if (unsafe { (*pPath).nUsed }) > (1 as i32) {
                0 as i32;
                '__slate_break_1264: loop {
                    let __v1461: *mut DbPath = pPath;
                    let __v1462: i32 = unsafe { (*__v1461).nUsed };
                    let __v1463: i32 = __v1462 - (1 as i32);
                    unsafe {
                        (*__v1461).nUsed = __v1463;
                    }
                    if !(((unsafe { *unsafe { unsafe { (*pPath).zOut }.offset(__v1463 as isize) } })
                        as i32)
                        != (47 as i32))
                    {
                        break;
                    }
                }
            }
            return;
        }
    }
    if (unsafe { (*pPath).nUsed }) + nName + (2 as i32) >= unsafe { (*pPath).nOut } {
        unsafe {
            (*pPath).rc = 1 as i32;
        }
        return;
    }
    let __v1464: *mut DbPath = pPath;
    let __v1465: i32 = unsafe { (*__v1464).nUsed };
    let __v1466: i32 = __v1465 + (1 as i32);
    unsafe {
        (*__v1464).nUsed = __v1466;
    }
    unsafe {
        *unsafe { unsafe { (*pPath).zOut }.offset(__v1465 as isize) } = (47 as i32) as i8;
    }
    unsafe {
        memcpy(
            (unsafe { unsafe { (*pPath).zOut }.offset((unsafe { (*pPath).nUsed }) as isize) })
                as *mut (),
            zName as *const (),
            (nName as i64) as u64,
        )
    };
    let __v1467: *mut DbPath = pPath;
    let __v1468: i32 = unsafe { (*__v1467).nUsed };
    let __v1469: i32 = __v1468 + nName;
    unsafe {
        (*__v1467).nUsed = __v1469;
    }
    if (unsafe { (*pPath).rc }) == (0 as i32) {
        let mut zIn: *const i8 = unsafe { std::mem::zeroed() };
        let mut buf: stat = unsafe { std::mem::zeroed() };
        unsafe {
            *unsafe { unsafe { (*pPath).zOut }.offset((unsafe { (*pPath).nUsed }) as isize) } =
                (0 as i32) as i8;
        }
        zIn = (unsafe { (*pPath).zOut }) as *const i8;
        if (unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(*const i8, *mut stat) -> i32>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((27 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(zIn, std::ptr::addr_of_mut!(buf))
        }) != (0 as i32)
        {
            if (unsafe { *unsafe { __errno_location() } }) != (2 as i32) {
                unsafe {
                    (*pPath).rc = unixLogErrorAtLine(
                        unsafe { sqlite3CantopenError(6951 as i32) },
                        (b"lstat\0".as_ptr() as *mut i8) as *const i8,
                        zIn,
                        6951 as i32,
                    );
                }
            }
        } else {
            if buf.st_mode & ((61440 as i32) as u32) == ((40960 as i32) as u32) {
                let mut got: i64 = 0 as i64;
                let mut zLnk: __SlateAlign16<[i8; 4098]> = __SlateAlign16([0 as i8; 4098]);
                let __v1470: *mut DbPath = pPath;
                let __v1471: i32 = unsafe { (*__v1470).nSymlink };
                let __v1472: i32 = __v1471 + (1 as i32);
                unsafe {
                    (*__v1470).nSymlink = __v1472;
                }
                if __v1471 > (200 as i32) {
                    unsafe {
                        (*pPath).rc = unsafe { sqlite3CantopenError(6957 as i32) };
                    }
                    return;
                }
                got = unsafe {
                    unsafe {
                        std::mem::transmute::<
                            Option<unsafe extern "C-unwind" fn()>,
                            Option<unsafe extern "C-unwind" fn(*const i8, *mut i8, u64) -> i64>,
                        >(unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                    .offset((26 as i32) as isize)
                            })
                            .pCurrent
                        })
                    }
                    .unwrap()(
                        zIn,
                        zLnk.0.as_mut_ptr() as *mut i8,
                        (4098 as u64).wrapping_sub(((2 as i32) as i64) as u64),
                    )
                };
                if got <= ((0 as i32) as i64) || got >= ((4098 as u64) as i64) - ((2 as i32) as i64)
                {
                    unsafe {
                        (*pPath).rc = unixLogErrorAtLine(
                            unsafe { sqlite3CantopenError(6962 as i32) },
                            (b"readlink\0".as_ptr() as *mut i8) as *const i8,
                            zIn,
                            6962 as i32,
                        );
                    }
                    return;
                }
                unsafe {
                    *unsafe { (zLnk.0.as_mut_ptr() as *mut i8).offset(got as isize) } =
                        (0 as i32) as i8;
                }
                if ((unsafe {
                    *unsafe { (zLnk.0.as_mut_ptr() as *mut i8).offset((0 as i32) as isize) }
                }) as i32)
                    == (47 as i32)
                {
                    unsafe {
                        (*pPath).nUsed = 0 as i32;
                    }
                } else {
                    let __v1473: *mut DbPath = pPath;
                    let __v1474: i32 = unsafe { (*__v1473).nUsed };
                    let __v1475: i32 = __v1474 - (nName + (1 as i32));
                    unsafe {
                        (*__v1473).nUsed = __v1475;
                    }
                }
                appendAllPathElements(pPath, (zLnk.0.as_mut_ptr() as *mut i8) as *const i8);
            }
        }
    }
}

/// Append all path elements in zPath to the DbPath under construction.
///
/// # Arguments
///
/// * `pPath` - Path under construction, to which to append zName
/// * `zPath` - Path to append to pPath.  Is zero-terminated
fn appendAllPathElements(mut pPath: *mut DbPath, mut zPath: *const i8) {
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    '__slate_break_1267: loop {
        '__slate_break_1268: while (unsafe { *unsafe { zPath.offset(i as isize) } }) != (0 as i8)
            && ((unsafe { *unsafe { zPath.offset(i as isize) } }) as i32) != (47 as i32)
        {
            let __v1457: i32 = i;
            let __v1458: i32 = __v1457 + (1 as i32);
            i = __v1458;
        }
        if i > j {
            appendOnePathElement(pPath, unsafe { zPath.offset(j as isize) }, i - j);
        }
        j = i + (1 as i32);
        let __v1459: i32 = i;
        let __v1460: i32 = __v1459 + (1 as i32);
        i = __v1460;
        if !((unsafe { *unsafe { zPath.offset(__v1459 as isize) } }) != (0 as i8)) {
            break;
        }
    }
}

/// Turn a relative pathname into a full pathname. The relative path
/// is stored as a nul-terminated string in the buffer pointed to by
/// zPath.
///
/// zOut points to a buffer of at least sqlite3_vfs.mxPathname bytes
/// (in this case, MAX_PATHNAME bytes). The full-path is written to
/// this buffer before returning.
///
/// # Arguments
///
/// * `pVfs` - Pointer to vfs object
/// * `zPath` - Possibly relative input path
/// * `nOut` - Size of output buffer in bytes
/// * `zOut` - Output buffer
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixFullPathname")]
extern "C-unwind" fn unixFullPathname(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const i8,
    mut nOut: i32,
    mut zOut: *mut i8,
) -> i32 {
    let mut path: DbPath = unsafe { std::mem::zeroed() };
    pVfs;
    path.rc = 0 as i32;
    path.nUsed = 0 as i32;
    path.nSymlink = 0 as i32;
    path.nOut = nOut;
    path.zOut = zOut;
    if ((unsafe { *unsafe { zPath.offset((0 as i32) as isize) } }) as i32) != (47 as i32) {
        let mut zPwd: __SlateAlign16<[i8; 4098]> = __SlateAlign16([0 as i8; 4098]);
        if (unsafe {
            unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<unsafe extern "C-unwind" fn(*mut i8, u64) -> *mut i8>,
                >(unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                            .offset((3 as i32) as isize)
                    })
                    .pCurrent
                })
            }
            .unwrap()(
                zPwd.0.as_mut_ptr() as *mut i8,
                (4098 as u64).wrapping_sub(((2 as i32) as i64) as u64),
            )
        }) == std::ptr::null_mut::<i8>()
        {
            return unixLogErrorAtLine(
                unsafe { sqlite3CantopenError(7020 as i32) },
                (b"getcwd\0".as_ptr() as *mut i8) as *const i8,
                zPath,
                7020 as i32,
            );
        }
        appendAllPathElements(
            std::ptr::addr_of_mut!(path),
            (zPwd.0.as_mut_ptr() as *mut i8) as *const i8,
        );
    }
    appendAllPathElements(std::ptr::addr_of_mut!(path), zPath);
    unsafe {
        *unsafe { zOut.offset(path.nUsed as isize) } = (0 as i32) as i8;
    }
    if path.rc != (0 as i32) || path.nUsed < (2 as i32) {
        return unsafe { sqlite3CantopenError(7026 as i32) };
    }
    if path.nSymlink != (0 as i32) {
        return (0 as i32) | (2 as i32) << (8 as i32);
    }
    return 0 as i32;
}

// Interfaces for opening a shared library, finding entry points
// within the shared library, and closing the shared library.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixDlOpen")]
extern "C-unwind" fn unixDlOpen(
    mut NotUsed: *mut sqlite3_vfs,
    mut zFilename: *const i8,
) -> *mut () {
    NotUsed;
    return unsafe { dlopen(zFilename, (2 as i32) | (256 as i32)) };
}

/// SQLite calls this function immediately after a call to unixDlSym() or
/// unixDlOpen() fails (returns a null pointer). If a more detailed error
/// message is available, it is written to zBufOut. If no error message
/// is available, zBufOut is left unmodified and SQLite uses a default
/// error message.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixDlError")]
extern "C-unwind" fn unixDlError(
    mut NotUsed: *mut sqlite3_vfs,
    mut nBuf: i32,
    mut zBufOut: *mut i8,
) {
    let mut zErr: *const i8 = unsafe { std::mem::zeroed() };
    NotUsed;
    unixEnterMutex();
    zErr = (unsafe { dlerror() }) as *const i8;
    if zErr != std::ptr::null::<i8>() {
        unsafe {
            sqlite3_snprintf(
                nBuf,
                zBufOut,
                (b"%s\0".as_ptr() as *mut i8) as *const i8,
                zErr,
            )
        };
    }
    unixLeaveMutex();
}

#[unsafe(link_section = ".text.slate_distinct.os_unix.unixDlSym")]
extern "C-unwind" fn unixDlSym(
    mut NotUsed: *mut sqlite3_vfs,
    mut p: *mut (),
    mut zSym: *const i8,
) -> Option<unsafe extern "C-unwind" fn()> {
    // GCC with -pedantic-errors says that C90 does not allow a void* to be
    // cast into a pointer to a function.  And yet the library dlsym() routine
    // returns a void* which is really a pointer to a function.  So how do we
    // use dlsym() with -pedantic-errors?
    //
    // Variable x below is defined to be a pointer to a function taking
    // parameters void* and const char* and returning a pointer to a function.
    // We initialize x by assigning it a pointer to the dlsym() function.
    // (That assignment requires a cast.)  Then we call the function that
    // x points to.
    //
    // This work-around is unlikely to work correctly on any system where
    // you really cannot cast a function pointer into void*.  But then, on the
    // other hand, dlsym() will not work on such a system either, so we have
    // not really lost anything.
    let mut x: Option<
        unsafe extern "C-unwind" fn(*mut (), *const i8) -> Option<unsafe extern "C-unwind" fn()>,
    > = unsafe { std::mem::zeroed() };
    NotUsed;
    x = unsafe {
        std::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut (), *const i8) -> *mut ()>,
            Option<
                unsafe extern "C-unwind" fn(
                    *mut (),
                    *const i8,
                )
                    -> Option<unsafe extern "C-unwind" fn()>,
            >,
        >(unsafe {
            std::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut (), *const i8) -> *mut ()>,
            >(dlsym as *const ())
        })
    };
    return unsafe { x.unwrap()(p, zSym) };
}

#[unsafe(link_section = ".text.slate_distinct.os_unix.unixDlClose")]
extern "C-unwind" fn unixDlClose(mut NotUsed: *mut sqlite3_vfs, mut pHandle: *mut ()) {
    NotUsed;
    unsafe { dlclose(pHandle) };
}

/// Write nBuf bytes of random data to the supplied buffer zBuf.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixRandomness")]
extern "C-unwind" fn unixRandomness(
    mut NotUsed: *mut sqlite3_vfs,
    mut nBuf: i32,
    mut zBuf: *mut i8,
) -> i32 {
    NotUsed;
    0 as i32;
    // We have to initialize zBuf to prevent valgrind from reporting
    // errors.  The reports issued by valgrind are incorrect - we would
    // prefer that the randomness be increased by making use of the
    // uninitialized space in zBuf - but valgrind errors tend to worry
    // some users.  Rather than argue, it seems easier just to initialize
    // the whole array and silence valgrind, even if that means less randomness
    // in the random seed.
    //
    // When testing, initializing zBuf[] to zero is all we do.  That means
    // that we always use the same random number sequence.  This makes the
    // tests repeatable.
    unsafe { memset(zBuf as *mut (), 0 as i32, (nBuf as i64) as u64) };
    unsafe {
        randomnessPid = unsafe { getpid() };
    }
    let mut fd: i32 = 0 as i32;
    let mut got: i32 = 0 as i32;
    fd = robust_open(
        (b"/dev/urandom\0".as_ptr() as *mut i8) as *const i8,
        0 as i32,
        (0 as i32) as u32,
    );
    if fd < (0 as i32) {
        let mut t: i64 = 0 as i64;
        unsafe { time(std::ptr::addr_of_mut!(t)) };
        unsafe {
            memcpy(
                zBuf as *mut (),
                std::ptr::addr_of_mut!(t) as *const (),
                8 as u64,
            )
        };
        unsafe {
            memcpy(
                (unsafe { zBuf.offset((8 as u64) as isize) }) as *mut (),
                (unsafe { std::ptr::addr_of_mut!(randomnessPid) }) as *const (),
                4 as u64,
            )
        };
        0 as i32;
        nBuf = ((8 as u64).wrapping_add(4 as u64) as u32) as i32;
    } else {
        '__slate_break_1277: loop {
            got = (unsafe {
                unsafe {
                    std::mem::transmute::<
                        Option<unsafe extern "C-unwind" fn()>,
                        Option<unsafe extern "C-unwind" fn(i32, *mut (), u64) -> i64>,
                    >(unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aSyscall.0) as *mut unix_syscall }
                                .offset((8 as i32) as isize)
                        })
                        .pCurrent
                    })
                }
                .unwrap()(fd, zBuf as *mut (), (nBuf as i64) as u64)
            }) as i32;
            if !(got < (0 as i32) && (unsafe { *unsafe { __errno_location() } }) == (4 as i32)) {
                break;
            }
        }
        robust_close(std::ptr::null_mut::<unixFile>(), fd, 7127 as i32);
    }
    return nBuf;
}

/// Sleep for a little while.  Return the amount of time slept.
/// The argument is the number of microseconds we want to sleep.
/// The return value is the number of microseconds of sleep actually
/// requested from the underlying operating system, a number which
/// might be greater than or equal to the argument, but not less
/// than the argument.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixSleep")]
extern "C-unwind" fn unixSleep(mut NotUsed: *mut sqlite3_vfs, mut microseconds: i32) -> i32 {
    let mut sp: timespec = unsafe { std::mem::zeroed() };
    sp.tv_sec = (microseconds / (1000000 as i32)) as i64;
    sp.tv_nsec = (microseconds % (1000000 as i32) * (1000 as i32)) as i64;
    // Almost all modern unix systems support nanosleep().  But if you are
    // compiling for one of the rare exceptions, you can use
    // -DHAVE_NANOSLEEP=0 (perhaps in conjunction with -DHAVE_USLEEP if
    // usleep() is available) in order to bypass the use of nanosleep()
    unsafe {
        nanosleep(
            std::ptr::addr_of_mut!(sp) as *const timespec,
            std::ptr::null_mut::<timespec>(),
        )
    };
    NotUsed;
    return microseconds;
}

/// The following variable, if set to a non-zero value, is interpreted as
/// the number of seconds since 1970 and is used to set the result of
/// sqlite3OsCurrentTime() during testing.
/// Find the current time (in Universal Coordinated Time).  Write into *piNow
/// the current time and date as a Julian Day number times 86_400_000.  In
/// other words, write into *piNow the number of milliseconds since the Julian
/// epoch of noon in Greenwich on November 24, 4714 B.C according to the
/// proleptic Gregorian calendar.
///
/// On success, return SQLITE_OK.  Return SQLITE_ERROR if the time and date
/// cannot be found.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixCurrentTimeInt64")]
extern "C-unwind" fn unixCurrentTimeInt64(
    mut NotUsed: *mut sqlite3_vfs,
    mut piNow: *mut i64,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut sNow: timeval = unsafe { std::mem::zeroed() };
    unsafe { gettimeofday(std::ptr::addr_of_mut!(sNow), std::ptr::null_mut::<()>()) }; // Cannot fail given valid arguments
    unsafe {
        *piNow = (unsafe { unixEpoch })
            + ((1000 as i32) as i64) * sNow.tv_sec
            + sNow.tv_usec / ((1000 as i32) as i64);
    }
    NotUsed;
    return rc;
}

static mut unixEpoch: i64 = ((24405875 as i32) as i64) * ((8640000 as i32) as i64);

/// Find the current time (in Universal Coordinated Time).  Write the
/// current time and date as a Julian Day number into *prNow and
/// return 0.  Return 1 if the time and date cannot be found.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixCurrentTime")]
extern "C-unwind" fn unixCurrentTime(mut NotUsed: *mut sqlite3_vfs, mut prNow: *mut f64) -> i32 {
    let mut i: i64 = (0 as i32) as i64;
    let mut rc: i32 = 0 as i32;
    NotUsed;
    rc = unixCurrentTimeInt64(
        std::ptr::null_mut::<sqlite3_vfs>(),
        std::ptr::addr_of_mut!(i),
    );
    unsafe {
        *prNow = (i as f64) / 86400000.0f64;
    }
    return rc;
}

/// The xGetLastError() method is designed to return a better
/// low-level error message when operating-system problems come up
/// during SQLite operation.  Only the integer return code is currently
/// used.
#[unsafe(link_section = ".text.slate_distinct.os_unix.unixGetLastError")]
extern "C-unwind" fn unixGetLastError(
    mut NotUsed: *mut sqlite3_vfs,
    mut NotUsed2: i32,
    mut NotUsed3: *mut i8,
) -> i32 {
    NotUsed;
    NotUsed2;
    NotUsed3;
    return unsafe { *unsafe { __errno_location() } };
}

// End of sqlite3_vfs methods ***************************
// Begin Proxy Locking ********************************
//
// Proxy locking is a "uber-locking-method" in this sense:  It uses the
// other locking methods on secondary lock files.  Proxy locking is a
// meta-layer over top of the primitive locking implemented above.  For
// this reason, the division that implements of proxy locking is deferred
// until late in the file (here) after all of the other I/O methods have
// been defined - so that the primitive locking methods are available
// as services to help with the implementation of proxy locking.
//
//
//
// The default locking schemes in SQLite use byte-range locks on the
// database file to coordinate safe, concurrent access by multiple readers
// and writers [http://sqlite.org/lockingv3.html].  The five file locking
// states (UNLOCKED, PENDING, SHARED, RESERVED, EXCLUSIVE) are implemented
// as POSIX read & write locks over fixed set of locations (via fsctl),
// on AFP and SMB only exclusive byte-range locks are available via fsctl
// with _IOWR('z', 23, struct ByteRangeLockPB2) to track the same 5 states.
// To simulate a F_RDLCK on the shared range, on AFP a randomly selected
// address in the shared range is taken for a SHARED lock, the entire
// shared range is taken for an EXCLUSIVE lock):
//
//      PENDING_BYTE        0x40000000
//      RESERVED_BYTE       0x40000001
//      SHARED_RANGE        0x40000002 -> 0x40000200
//
// This works well on the local file system, but shows a nearly 100x
// slowdown in read performance on AFP because the AFP client disables
// the read cache when byte-range locks are present.  Enabling the read
// cache exposes a cache coherency problem that is present on all OS X
// supported network file systems.  NFS and AFP both observe the
// close-to-open semantics for ensuring cache coherency
// [http://nfs.sourceforge.net/#faq_a8], which does not effectively
// address the requirements for concurrent database access by multiple
// readers and writers
// [http://www.nabble.com/SQLite-on-NFS-cache-coherency-td15655701.html].
//
// To address the performance and cache coherency issues, proxy file locking
// changes the way database access is controlled by limiting access to a
// single host at a time and moving file locks off of the database file
// and onto a proxy file on the local file system.
//
//
// Using proxy locks
//
// C APIs
//
//  sqlite3_file_control(db, dbname, SQLITE_FCNTL_SET_LOCKPROXYFILE,
//                       <proxy_path> | ":auto:");
//  sqlite3_file_control(db, dbname, SQLITE_FCNTL_GET_LOCKPROXYFILE,
//                       &<proxy_path>);
//
//
// SQL pragmas
//
//  PRAGMA [database.]lock_proxy_file=<proxy_path> | :auto:
//  PRAGMA [database.]lock_proxy_file
//
// Specifying ":auto:" means that if there is a conch file with a matching
// host ID in it, the proxy path in the conch file will be used, otherwise
// a proxy path based on the user's temp dir
// (via confstr(_CS_DARWIN_USER_TEMP_DIR,...)) will be used and the
// actual proxy file name is generated from the name and path of the
// database file.  For example:
//
//       For database path "/Users/me/foo.db"
//       The lock path will be "<tmpdir>/sqliteplocks/_Users_me_foo.db:auto:")
//
// Once a lock proxy is configured for a database connection, it can not
// be removed, however it may be switched to a different proxy path via
// the above APIs (assuming the conch file is not being held by another
// connection or process).
//
//
// How proxy locking works
//
// Proxy file locking relies primarily on two new supporting files:
//
//   *  conch file to limit access to the database file to a single host
//      at a time
//
//   *  proxy file to act as a proxy for the advisory locks normally
//      taken on the database
//
// The conch file - to use a proxy file, sqlite must first "hold the conch"
// by taking an sqlite-style shared lock on the conch file, reading the
// contents and comparing the host's unique host ID (see below) and lock
// proxy path against the values stored in the conch.  The conch file is
// stored in the same directory as the database file and the file name
// is patterned after the database file name as ".<databasename>-conch".
// If the conch file does not exist, or its contents do not match the
// host ID and/or proxy path, then the lock is escalated to an exclusive
// lock and the conch file contents is updated with the host ID and proxy
// path and the lock is downgraded to a shared lock again.  If the conch
// is held by another process (with a shared lock), the exclusive lock
// will fail and SQLITE_BUSY is returned.
//
// The proxy file - a single-byte file used for all advisory file locks
// normally taken on the database file.   This allows for safe sharing
// of the database file for multiple readers and writers on the same
// host (the conch ensures that they all use the same local lock file).
//
// Requesting the lock proxy does not immediately take the conch, it is
// only taken when the first request to lock database file is made.
// This matches the semantics of the traditional locking behavior, where
// opening a connection to a database file does not take a lock on it.
// The shared lock and an open file descriptor are maintained until
// the connection to the database is closed.
//
// The proxy file and the lock file are never deleted so they only need
// to be created the first time they are used.
//
// Configuration options
//
//  SQLITE_PREFER_PROXY_LOCKING
//
//       Database files accessed on non-local file systems are
//       automatically configured for proxy locking, lock files are
//       named automatically using the same logic as
//       PRAGMA lock_proxy_file=":auto:"
//
//  SQLITE_PROXY_DEBUG
//
//       Enables the logging of error messages during host id file
//       retrieval and creation
//
//  LOCKPROXYDIR
//
//       Overrides the default directory used for lock proxy files that
//       are named automatically via the ":auto:" setting
//
//  SQLITE_DEFAULT_PROXYDIR_PERMISSIONS
//
//       Permissions to use when creating a directory for storing the
//       lock proxy files, only used when LOCKPROXYDIR is not set.
//
//
// As mentioned above, when compiled with SQLITE_PREFER_PROXY_LOCKING,
// setting the environment variable SQLITE_FORCE_PROXY_LOCKING to 1 will
// force proxy locking to be used for every database file opened, and 0
// will force automatic proxy locking to be disabled for all database
// files (explicitly calling the SQLITE_FCNTL_SET_LOCKPROXYFILE pragma or
// sqlite_file_control API is not affected by SQLITE_FORCE_PROXY_LOCKING).
// Proxy locking is only available on MacOSX
// The proxy locking style is intended for use with AFP filesystems.
// And since AFP is only supported on MacOSX, the proxy locking is also
// restricted to MacOSX.
//
//
// End of the proxy lock implementation **********************
/// Initialize the operating system interface.
///
/// This routine registers all VFS implementations for unix-like operating
/// systems.  This routine, and the sqlite3_os_end() routine that follows,
/// should be the only routines in this file that are visible from other
/// files.
///
/// This routine is called once during SQLite initialization and by a
/// single thread.  The memory allocation and mutex subsystems have not
/// necessarily been initialized when this routine is called, and so they
/// should not be used.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3_os_init() -> i32 {
    // The following macro defines an initializer for an sqlite3_vfs object.
    // The name of the VFS is NAME.  The pAppData is a pointer to a pointer
    // to the "finder" function.  (pAppData is a pointer to a pointer because
    // silly C90 rules prohibit a void* from being cast to a function pointer
    // and so we have to go through the intermediate pointer to avoid problems
    // when compiling with -pedantic-errors on GCC.)
    //
    // The FINDER parameter to this macro is the name of the pointer to the
    // finder-function.  The finder-function returns a pointer to the
    // sqlite_io_methods object that implements the desired locking
    // behaviors.  See the division above that contains the IOMETHODS
    // macro for addition information on finder-functions.
    //
    // Most finders simply return a pointer to a fixed sqlite3_io_methods
    // object.  But the "autolockIoFinder" available on MacOSX does a little
    // more than that; it looks at the filesystem type that hosts the
    // database file and tries to choose an locking method appropriate for
    // that filesystem time.
    // iVersion
    // szOsFile
    // mxPathname
    // pNext
    // zName
    // pAppData
    // xOpen
    // xDelete
    // xAccess
    // xFullPathname
    // xDlOpen
    // xDlError
    // xDlSym
    // xDlClose
    // xRandomness
    // xSleep
    // xCurrentTime
    // xGetLastError
    // xCurrentTimeInt64
    // xSetSystemCall
    // xGetSystemCall
    // xNextSystemCall
    // All default VFSes for unix are contained in the following array.
    //
    // Note that the sqlite3_vfs.pNext field of the VFS object is modified
    // by the SQLite core when the VFS is registered.  So the following
    // array cannot be const.
    let mut i: u32 = 0 as u32; // Loop counter
    // Double-check that the aSyscall[] array has been constructed
    // correctly.  See ticket [bb3a86e890c8e96ab]
    0 as i32;
    // Register all VFSes defined in the aVfs[] array
    i = (0 as i32) as u32;
    '__slate_break_1282: while (i as u64) < (672 as u64) / (168 as u64) {
        unsafe {
            sqlite3_vfs_register(
                unsafe {
                    unsafe { std::ptr::addr_of_mut!(aVfs.0) as *mut sqlite3_vfs }.offset(i as isize)
                },
                (i == ((0 as i32) as u32)) as i32,
            )
        };
        let __v1283: u32 = i;
        let __v1284: u32 = __v1283.wrapping_add((1 as i32) as u32);
        i = __v1284;
    }
    unsafe {
        unixBigLock = unsafe { sqlite3MutexAlloc(11 as i32) };
    }
    // Validate lock assumptions
    0 as i32; // Number of available locks
    0 as i32; // Start of locking area
    // Locks:
    // WRITE       UNIX_SHM_BASE      120
    // CKPT        UNIX_SHM_BASE+1    121
    // RECOVER     UNIX_SHM_BASE+2    122
    // READ-0      UNIX_SHM_BASE+3    123
    // READ-1      UNIX_SHM_BASE+4    124
    // READ-2      UNIX_SHM_BASE+5    125
    // READ-3      UNIX_SHM_BASE+6    126
    // READ-4      UNIX_SHM_BASE+7    127
    // DMS         UNIX_SHM_BASE+8    128
    0 as i32; // Byte offset of the deadman-switch
    return 0 as i32;
}

static mut aVfs: __SlateAlign16<[sqlite3_vfs; 4]> = __SlateAlign16([
    sqlite3_vfs {
        iVersion: 3 as i32,
        szOsFile: ((120 as u64) as u32) as i32,
        mxPathname: 512 as i32,
        pNext: std::ptr::null_mut::<sqlite3_vfs>(),
        zName: (b"unix\0".as_ptr() as *mut i8) as *const i8,
        pAppData: (unsafe { std::ptr::addr_of!(posixIoFinder) }) as *mut (),
        xOpen: Some(unixOpen),
        xDelete: Some(unixDelete),
        xAccess: Some(unixAccess),
        xFullPathname: Some(unixFullPathname),
        xDlOpen: Some(unixDlOpen),
        xDlError: Some(unixDlError),
        xDlSym: Some(unixDlSym),
        xDlClose: Some(unixDlClose),
        xRandomness: Some(unixRandomness),
        xSleep: Some(unixSleep),
        xCurrentTime: Some(unixCurrentTime),
        xGetLastError: Some(unixGetLastError),
        xCurrentTimeInt64: Some(unixCurrentTimeInt64),
        xSetSystemCall: Some(unixSetSystemCall),
        xGetSystemCall: Some(unixGetSystemCall),
        xNextSystemCall: Some(unixNextSystemCall),
    },
    sqlite3_vfs {
        iVersion: 3 as i32,
        szOsFile: ((120 as u64) as u32) as i32,
        mxPathname: 512 as i32,
        pNext: std::ptr::null_mut::<sqlite3_vfs>(),
        zName: (b"unix-none\0".as_ptr() as *mut i8) as *const i8,
        pAppData: (unsafe { std::ptr::addr_of!(nolockIoFinder) }) as *mut (),
        xOpen: Some(unixOpen),
        xDelete: Some(unixDelete),
        xAccess: Some(unixAccess),
        xFullPathname: Some(unixFullPathname),
        xDlOpen: Some(unixDlOpen),
        xDlError: Some(unixDlError),
        xDlSym: Some(unixDlSym),
        xDlClose: Some(unixDlClose),
        xRandomness: Some(unixRandomness),
        xSleep: Some(unixSleep),
        xCurrentTime: Some(unixCurrentTime),
        xGetLastError: Some(unixGetLastError),
        xCurrentTimeInt64: Some(unixCurrentTimeInt64),
        xSetSystemCall: Some(unixSetSystemCall),
        xGetSystemCall: Some(unixGetSystemCall),
        xNextSystemCall: Some(unixNextSystemCall),
    },
    sqlite3_vfs {
        iVersion: 3 as i32,
        szOsFile: ((120 as u64) as u32) as i32,
        mxPathname: 512 as i32,
        pNext: std::ptr::null_mut::<sqlite3_vfs>(),
        zName: (b"unix-dotfile\0".as_ptr() as *mut i8) as *const i8,
        pAppData: (unsafe { std::ptr::addr_of!(dotlockIoFinder) }) as *mut (),
        xOpen: Some(unixOpen),
        xDelete: Some(unixDelete),
        xAccess: Some(unixAccess),
        xFullPathname: Some(unixFullPathname),
        xDlOpen: Some(unixDlOpen),
        xDlError: Some(unixDlError),
        xDlSym: Some(unixDlSym),
        xDlClose: Some(unixDlClose),
        xRandomness: Some(unixRandomness),
        xSleep: Some(unixSleep),
        xCurrentTime: Some(unixCurrentTime),
        xGetLastError: Some(unixGetLastError),
        xCurrentTimeInt64: Some(unixCurrentTimeInt64),
        xSetSystemCall: Some(unixSetSystemCall),
        xGetSystemCall: Some(unixGetSystemCall),
        xNextSystemCall: Some(unixNextSystemCall),
    },
    sqlite3_vfs {
        iVersion: 3 as i32,
        szOsFile: ((120 as u64) as u32) as i32,
        mxPathname: 512 as i32,
        pNext: std::ptr::null_mut::<sqlite3_vfs>(),
        zName: (b"unix-excl\0".as_ptr() as *mut i8) as *const i8,
        pAppData: (unsafe { std::ptr::addr_of!(posixIoFinder) }) as *mut (),
        xOpen: Some(unixOpen),
        xDelete: Some(unixDelete),
        xAccess: Some(unixAccess),
        xFullPathname: Some(unixFullPathname),
        xDlOpen: Some(unixDlOpen),
        xDlError: Some(unixDlError),
        xDlSym: Some(unixDlSym),
        xDlClose: Some(unixDlClose),
        xRandomness: Some(unixRandomness),
        xSleep: Some(unixSleep),
        xCurrentTime: Some(unixCurrentTime),
        xGetLastError: Some(unixGetLastError),
        xCurrentTimeInt64: Some(unixCurrentTimeInt64),
        xSetSystemCall: Some(unixSetSystemCall),
        xGetSystemCall: Some(unixGetSystemCall),
        xNextSystemCall: Some(unixNextSystemCall),
    },
]);

/// Shutdown the operating system interface.
///
/// Some operating systems might need to do some cleanup in this routine,
/// to release dynamically allocated objects.  But not on unix.
/// This routine is a no-op for unix.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3_os_end() -> i32 {
    unsafe {
        unixBigLock = std::ptr::null_mut::<sqlite3_mutex>();
    }
    return 0 as i32;
}
