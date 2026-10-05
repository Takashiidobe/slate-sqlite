unsafe extern "C" {
    fn sqlite3_free(__v63: *mut ());
    fn sqlite3MallocZero(__v64: u64) -> *mut ();
    fn pthread_mutex_init(__mutex: *mut __SlateRecord10, __mutexattr: *const __SlateRecord8)
    -> i32;
    fn pthread_mutex_destroy(__mutex: *mut __SlateRecord10) -> i32;
    fn pthread_mutex_trylock(__mutex: *mut __SlateRecord10) -> i32;
    fn pthread_mutex_lock(__mutex: *mut __SlateRecord10) -> i32;
    fn pthread_mutex_unlock(__mutex: *mut __SlateRecord10) -> i32;
    fn pthread_mutexattr_init(__attr: *mut __SlateRecord8) -> i32;
    fn pthread_mutexattr_destroy(__attr: *mut __SlateRecord8) -> i32;
    fn pthread_mutexattr_settype(__attr: *mut __SlateRecord8, __kind: i32) -> i32;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_mutex {
    mutex: __SlateRecord10,
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
struct __pthread_internal_list {
    __prev: *mut __pthread_internal_list,
    __next: *mut __pthread_internal_list,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __pthread_mutex_s {
    __lock: i32,
    __count: u32,
    __owner: i32,
    __nusers: u32,
    __kind: i32,
    __spins: i16,
    __glibc_reserved: i16,
    __list: __pthread_internal_list,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord8 {
    __size: [i8; 4],
    __align: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord10 {
    __data: __pthread_mutex_s,
    __size: [i8; 40],
    __align: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union staticMutex {
    m: sqlite3_mutex,
    aSpacer: [i8; 128],
}

#[repr(C, align(128))]
struct __SlateAlign128<T>(T);

static mut aMutex: __SlateAlign128<[staticMutex; 12]> = __SlateAlign128([
    {
        let mut __t0: staticMutex = unsafe { std::mem::zeroed() };
        __t0.m = sqlite3_mutex {
            mutex: {
                let mut __t1: __SlateRecord10 = unsafe { std::mem::zeroed() };
                __t1.__data = __pthread_mutex_s {
                    __lock: 0 as i32,
                    __count: (0 as i32) as u32,
                    __owner: 0 as i32,
                    __nusers: (0 as i32) as u32,
                    __kind: 0 as i32,
                    __spins: (0 as i32) as i16,
                    __glibc_reserved: (0 as i32) as i16,
                    __list: __pthread_internal_list {
                        __prev: std::ptr::null_mut::<__pthread_internal_list>(),
                        __next: std::ptr::null_mut::<__pthread_internal_list>(),
                    },
                };
                __t1
            },
        };
        __t0
    },
    {
        let mut __t2: staticMutex = unsafe { std::mem::zeroed() };
        __t2.m = sqlite3_mutex {
            mutex: {
                let mut __t3: __SlateRecord10 = unsafe { std::mem::zeroed() };
                __t3.__data = __pthread_mutex_s {
                    __lock: 0 as i32,
                    __count: (0 as i32) as u32,
                    __owner: 0 as i32,
                    __nusers: (0 as i32) as u32,
                    __kind: 0 as i32,
                    __spins: (0 as i32) as i16,
                    __glibc_reserved: (0 as i32) as i16,
                    __list: __pthread_internal_list {
                        __prev: std::ptr::null_mut::<__pthread_internal_list>(),
                        __next: std::ptr::null_mut::<__pthread_internal_list>(),
                    },
                };
                __t3
            },
        };
        __t2
    },
    {
        let mut __t4: staticMutex = unsafe { std::mem::zeroed() };
        __t4.m = sqlite3_mutex {
            mutex: {
                let mut __t5: __SlateRecord10 = unsafe { std::mem::zeroed() };
                __t5.__data = __pthread_mutex_s {
                    __lock: 0 as i32,
                    __count: (0 as i32) as u32,
                    __owner: 0 as i32,
                    __nusers: (0 as i32) as u32,
                    __kind: 0 as i32,
                    __spins: (0 as i32) as i16,
                    __glibc_reserved: (0 as i32) as i16,
                    __list: __pthread_internal_list {
                        __prev: std::ptr::null_mut::<__pthread_internal_list>(),
                        __next: std::ptr::null_mut::<__pthread_internal_list>(),
                    },
                };
                __t5
            },
        };
        __t4
    },
    {
        let mut __t6: staticMutex = unsafe { std::mem::zeroed() };
        __t6.m = sqlite3_mutex {
            mutex: {
                let mut __t7: __SlateRecord10 = unsafe { std::mem::zeroed() };
                __t7.__data = __pthread_mutex_s {
                    __lock: 0 as i32,
                    __count: (0 as i32) as u32,
                    __owner: 0 as i32,
                    __nusers: (0 as i32) as u32,
                    __kind: 0 as i32,
                    __spins: (0 as i32) as i16,
                    __glibc_reserved: (0 as i32) as i16,
                    __list: __pthread_internal_list {
                        __prev: std::ptr::null_mut::<__pthread_internal_list>(),
                        __next: std::ptr::null_mut::<__pthread_internal_list>(),
                    },
                };
                __t7
            },
        };
        __t6
    },
    {
        let mut __t8: staticMutex = unsafe { std::mem::zeroed() };
        __t8.m = sqlite3_mutex {
            mutex: {
                let mut __t9: __SlateRecord10 = unsafe { std::mem::zeroed() };
                __t9.__data = __pthread_mutex_s {
                    __lock: 0 as i32,
                    __count: (0 as i32) as u32,
                    __owner: 0 as i32,
                    __nusers: (0 as i32) as u32,
                    __kind: 0 as i32,
                    __spins: (0 as i32) as i16,
                    __glibc_reserved: (0 as i32) as i16,
                    __list: __pthread_internal_list {
                        __prev: std::ptr::null_mut::<__pthread_internal_list>(),
                        __next: std::ptr::null_mut::<__pthread_internal_list>(),
                    },
                };
                __t9
            },
        };
        __t8
    },
    {
        let mut __t10: staticMutex = unsafe { std::mem::zeroed() };
        __t10.m = sqlite3_mutex {
            mutex: {
                let mut __t11: __SlateRecord10 = unsafe { std::mem::zeroed() };
                __t11.__data = __pthread_mutex_s {
                    __lock: 0 as i32,
                    __count: (0 as i32) as u32,
                    __owner: 0 as i32,
                    __nusers: (0 as i32) as u32,
                    __kind: 0 as i32,
                    __spins: (0 as i32) as i16,
                    __glibc_reserved: (0 as i32) as i16,
                    __list: __pthread_internal_list {
                        __prev: std::ptr::null_mut::<__pthread_internal_list>(),
                        __next: std::ptr::null_mut::<__pthread_internal_list>(),
                    },
                };
                __t11
            },
        };
        __t10
    },
    {
        let mut __t12: staticMutex = unsafe { std::mem::zeroed() };
        __t12.m = sqlite3_mutex {
            mutex: {
                let mut __t13: __SlateRecord10 = unsafe { std::mem::zeroed() };
                __t13.__data = __pthread_mutex_s {
                    __lock: 0 as i32,
                    __count: (0 as i32) as u32,
                    __owner: 0 as i32,
                    __nusers: (0 as i32) as u32,
                    __kind: 0 as i32,
                    __spins: (0 as i32) as i16,
                    __glibc_reserved: (0 as i32) as i16,
                    __list: __pthread_internal_list {
                        __prev: std::ptr::null_mut::<__pthread_internal_list>(),
                        __next: std::ptr::null_mut::<__pthread_internal_list>(),
                    },
                };
                __t13
            },
        };
        __t12
    },
    {
        let mut __t14: staticMutex = unsafe { std::mem::zeroed() };
        __t14.m = sqlite3_mutex {
            mutex: {
                let mut __t15: __SlateRecord10 = unsafe { std::mem::zeroed() };
                __t15.__data = __pthread_mutex_s {
                    __lock: 0 as i32,
                    __count: (0 as i32) as u32,
                    __owner: 0 as i32,
                    __nusers: (0 as i32) as u32,
                    __kind: 0 as i32,
                    __spins: (0 as i32) as i16,
                    __glibc_reserved: (0 as i32) as i16,
                    __list: __pthread_internal_list {
                        __prev: std::ptr::null_mut::<__pthread_internal_list>(),
                        __next: std::ptr::null_mut::<__pthread_internal_list>(),
                    },
                };
                __t15
            },
        };
        __t14
    },
    {
        let mut __t16: staticMutex = unsafe { std::mem::zeroed() };
        __t16.m = sqlite3_mutex {
            mutex: {
                let mut __t17: __SlateRecord10 = unsafe { std::mem::zeroed() };
                __t17.__data = __pthread_mutex_s {
                    __lock: 0 as i32,
                    __count: (0 as i32) as u32,
                    __owner: 0 as i32,
                    __nusers: (0 as i32) as u32,
                    __kind: 0 as i32,
                    __spins: (0 as i32) as i16,
                    __glibc_reserved: (0 as i32) as i16,
                    __list: __pthread_internal_list {
                        __prev: std::ptr::null_mut::<__pthread_internal_list>(),
                        __next: std::ptr::null_mut::<__pthread_internal_list>(),
                    },
                };
                __t17
            },
        };
        __t16
    },
    {
        let mut __t18: staticMutex = unsafe { std::mem::zeroed() };
        __t18.m = sqlite3_mutex {
            mutex: {
                let mut __t19: __SlateRecord10 = unsafe { std::mem::zeroed() };
                __t19.__data = __pthread_mutex_s {
                    __lock: 0 as i32,
                    __count: (0 as i32) as u32,
                    __owner: 0 as i32,
                    __nusers: (0 as i32) as u32,
                    __kind: 0 as i32,
                    __spins: (0 as i32) as i16,
                    __glibc_reserved: (0 as i32) as i16,
                    __list: __pthread_internal_list {
                        __prev: std::ptr::null_mut::<__pthread_internal_list>(),
                        __next: std::ptr::null_mut::<__pthread_internal_list>(),
                    },
                };
                __t19
            },
        };
        __t18
    },
    {
        let mut __t20: staticMutex = unsafe { std::mem::zeroed() };
        __t20.m = sqlite3_mutex {
            mutex: {
                let mut __t21: __SlateRecord10 = unsafe { std::mem::zeroed() };
                __t21.__data = __pthread_mutex_s {
                    __lock: 0 as i32,
                    __count: (0 as i32) as u32,
                    __owner: 0 as i32,
                    __nusers: (0 as i32) as u32,
                    __kind: 0 as i32,
                    __spins: (0 as i32) as i16,
                    __glibc_reserved: (0 as i32) as i16,
                    __list: __pthread_internal_list {
                        __prev: std::ptr::null_mut::<__pthread_internal_list>(),
                        __next: std::ptr::null_mut::<__pthread_internal_list>(),
                    },
                };
                __t21
            },
        };
        __t20
    },
    {
        let mut __t22: staticMutex = unsafe { std::mem::zeroed() };
        __t22.m = sqlite3_mutex {
            mutex: {
                let mut __t23: __SlateRecord10 = unsafe { std::mem::zeroed() };
                __t23.__data = __pthread_mutex_s {
                    __lock: 0 as i32,
                    __count: (0 as i32) as u32,
                    __owner: 0 as i32,
                    __nusers: (0 as i32) as u32,
                    __kind: 0 as i32,
                    __spins: (0 as i32) as i16,
                    __glibc_reserved: (0 as i32) as i16,
                    __list: __pthread_internal_list {
                        __prev: std::ptr::null_mut::<__pthread_internal_list>(),
                        __next: std::ptr::null_mut::<__pthread_internal_list>(),
                    },
                };
                __t23
            },
        };
        __t22
    },
]);

static mut sMutex: sqlite3_mutex_methods = sqlite3_mutex_methods {
    xMutexInit: Some(pthreadMutexInit),
    xMutexEnd: Some(pthreadMutexEnd),
    xMutexAlloc: Some(pthreadMutexAlloc),
    xMutexFree: Some(pthreadMutexFree),
    xMutexEnter: Some(pthreadMutexEnter),
    xMutexTry: Some(pthreadMutexTry),
    xMutexLeave: Some(pthreadMutexLeave),
    xMutexHeld: None,
    xMutexNotheld: None,
};

// /* SQLITE_MUTEX_PTHREADS */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DefaultMutex() -> *const sqlite3_mutex_methods {
    return unsafe { std::ptr::addr_of!(sMutex) };
}

// /*
// ** 2007 August 28
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// ** This file contains the C functions that implement mutexes for pthreads
// */
// /*
// ** The code in this file is only used if we are compiling threadsafe
// ** under unix with pthreads.
// **
// ** Note that this implementation requires a version of pthreads that
// ** supports recursive mutexes.
// */
// /*
// ** The sqlite3_mutex.id, sqlite3_mutex.nRef, and sqlite3_mutex.owner fields
// ** are necessary under two conditions:  (1) Debug builds and (2) using
// ** home-grown mutexes.  Encapsulate these conditions into a single #define.
// */
// /*
// ** Each SQLite mutex is an instance of the following structure.
// **
// */
// /* Mutex controlling the lock */
// /*
// ** The sqlite3_mutex_held() and sqlite3_mutex_notheld() routine are
// ** intended for use only inside assert() statements.  On some platforms,
// ** there might be race conditions that can cause these routines to
// ** deliver incorrect results.  In particular, if pthread_equal() is
// ** not an atomic operation, then these routines might delivery
// ** incorrect results.  On most platforms, pthread_equal() is a
// ** comparison of two integers and is therefore atomic.  But we are
// ** told that HPUX is not such a platform.  If so, then these routines
// ** will not always work correctly on HPUX.
// **
// ** On those platforms where pthread_equal() is not atomic, SQLite
// ** should be compiled without -DSQLITE_DEBUG and with -DNDEBUG to
// ** make sure no assert() statements are evaluated and hence these
// ** routines are never called.
// */
// /*
// ** Try to provide a memory barrier operation, needed for initialization
// ** and also for the implementation of xShmBarrier in the VFS in cases
// ** where SQLite is compiled without mutexes.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MemoryBarrier() {
    std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
}

// /*
// ** Initialize and deinitialize the mutex subsystem.
// */
#[unsafe(link_section = ".text.slate_distinct.mutex_unix.pthreadMutexInit")]
extern "C-unwind" fn pthreadMutexInit() -> i32 {
    return 0 as i32;
}

#[unsafe(link_section = ".text.slate_distinct.mutex_unix.pthreadMutexEnd")]
extern "C-unwind" fn pthreadMutexEnd() -> i32 {
    return 0 as i32;
}

// /*
// ** The sqlite3_mutex_alloc() routine allocates a new
// ** mutex and returns a pointer to it.  If it returns NULL
// ** that means that a mutex could not be allocated.  SQLite
// ** will unwind its stack and return an error.  The argument
// ** to sqlite3_mutex_alloc() is one of these integer constants:
// **
// ** <ul>
// ** <li>  SQLITE_MUTEX_FAST
// ** <li>  SQLITE_MUTEX_RECURSIVE
// ** <li>  SQLITE_MUTEX_STATIC_MAIN
// ** <li>  SQLITE_MUTEX_STATIC_MEM
// ** <li>  SQLITE_MUTEX_STATIC_OPEN
// ** <li>  SQLITE_MUTEX_STATIC_PRNG
// ** <li>  SQLITE_MUTEX_STATIC_LRU
// ** <li>  SQLITE_MUTEX_STATIC_PMEM
// ** <li>  SQLITE_MUTEX_STATIC_APP1
// ** <li>  SQLITE_MUTEX_STATIC_APP2
// ** <li>  SQLITE_MUTEX_STATIC_APP3
// ** <li>  SQLITE_MUTEX_STATIC_VFS1
// ** <li>  SQLITE_MUTEX_STATIC_VFS2
// ** <li>  SQLITE_MUTEX_STATIC_VFS3
// ** </ul>
// **
// ** The first two constants cause sqlite3_mutex_alloc() to create
// ** a new mutex.  The new mutex is recursive when SQLITE_MUTEX_RECURSIVE
// ** is used but not necessarily so when SQLITE_MUTEX_FAST is used.
// ** The mutex implementation does not need to make a distinction
// ** between SQLITE_MUTEX_RECURSIVE and SQLITE_MUTEX_FAST if it does
// ** not want to.  But SQLite will only request a recursive mutex in
// ** cases where it really needs one.  If a faster non-recursive mutex
// ** implementation is available on the host platform, the mutex subsystem
// ** might return such a mutex in response to SQLITE_MUTEX_FAST.
// **
// ** The other allowed parameters to sqlite3_mutex_alloc() each return
// ** a pointer to a static preexisting mutex.  Six static mutexes are
// ** used by the current version of SQLite.  Future versions of SQLite
// ** may add additional static mutexes.  Static mutexes are for internal
// ** use by SQLite only.  Applications that use SQLite mutexes should
// ** use only the dynamic mutexes returned by SQLITE_MUTEX_FAST or
// ** SQLITE_MUTEX_RECURSIVE.
// **
// ** Note that if one of the dynamic mutex parameters (SQLITE_MUTEX_FAST
// ** or SQLITE_MUTEX_RECURSIVE) is used then sqlite3_mutex_alloc()
// ** returns a different mutex on every call.  But for the static
// ** mutex types, the same mutex is returned on every call that has
// ** the same type number.
// */
#[unsafe(link_section = ".text.slate_distinct.mutex_unix.pthreadMutexAlloc")]
extern "C-unwind" fn pthreadMutexAlloc(mut iType: i32) -> *mut sqlite3_mutex {
    // /* Static mutexes - those with IDs of 2 or more...
    //   **
    //   ** The ALIGN128 macro attempts to force 128-byte alignment on mutexes,
    //   ** so that adjacent mutex objects are always on different cache lines
    //   ** in the CPU.  This is CPU-dependent, of course, but 128-byte alignment
    //   ** seems to work well for all contemporary processors.  Experiments show
    //   ** that 64 works just as well most of the time, but the internet says
    //   ** that 128-byte alignment works better.
    //   */
    let mut p: *mut sqlite3_mutex = unsafe { std::mem::zeroed() };
    '__slate_break_75: {
        match iType {
            1 => {
                p = (unsafe { sqlite3MallocZero(40 as u64) }) as *mut sqlite3_mutex;
                if p != std::ptr::null_mut::<sqlite3_mutex>() {
                    // /* Use a recursive mutex if it is available */
                    let mut recursiveAttr: __SlateRecord8 = unsafe { std::mem::zeroed() };
                    unsafe { pthread_mutexattr_init(std::ptr::addr_of_mut!(recursiveAttr)) };
                    unsafe {
                        pthread_mutexattr_settype(std::ptr::addr_of_mut!(recursiveAttr), 1 as i32)
                    };
                    unsafe {
                        pthread_mutex_init(
                            unsafe { std::ptr::addr_of_mut!((*p).mutex) },
                            std::ptr::addr_of_mut!(recursiveAttr) as *const __SlateRecord8,
                        )
                    };
                    unsafe { pthread_mutexattr_destroy(std::ptr::addr_of_mut!(recursiveAttr)) };
                }
            }
            0 => {
                p = (unsafe { sqlite3MallocZero(40 as u64) }) as *mut sqlite3_mutex;
                if p != std::ptr::null_mut::<sqlite3_mutex>() {
                    unsafe {
                        pthread_mutex_init(
                            unsafe { std::ptr::addr_of_mut!((*p).mutex) },
                            std::ptr::null::<__SlateRecord8>(),
                        )
                    };
                }
            }
            _ => {
                p = unsafe {
                    std::ptr::addr_of_mut!(
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!(aMutex.0) as *mut staticMutex }
                                .offset((iType - (2 as i32)) as isize)
                        })
                        .m
                    )
                };
            }
        }
    }
    return p;
}

// /*
// ** This routine deallocates a previously
// ** allocated mutex.  SQLite is careful to deallocate every
// ** mutex that it allocates.
// */
#[unsafe(link_section = ".text.slate_distinct.mutex_unix.pthreadMutexFree")]
extern "C-unwind" fn pthreadMutexFree(mut p: *mut sqlite3_mutex) {
    0 as i32;
    unsafe { pthread_mutex_destroy(unsafe { std::ptr::addr_of_mut!((*p).mutex) }) };
    unsafe { sqlite3_free(p as *mut ()) };
}

// /*
// ** The sqlite3_mutex_enter() and sqlite3_mutex_try() routines attempt
// ** to enter a mutex.  If another thread is already within the mutex,
// ** sqlite3_mutex_enter() will block and sqlite3_mutex_try() will return
// ** SQLITE_BUSY.  The sqlite3_mutex_try() interface returns SQLITE_OK
// ** upon successful entry.  Mutexes created using SQLITE_MUTEX_RECURSIVE can
// ** be entered multiple times by the same thread.  In such cases the,
// ** mutex must be exited an equal number of times before another thread
// ** can enter.  If the same thread tries to enter any other kind of mutex
// ** more than once, the behavior is undefined.
// */
#[unsafe(link_section = ".text.slate_distinct.mutex_unix.pthreadMutexEnter")]
extern "C-unwind" fn pthreadMutexEnter(mut p: *mut sqlite3_mutex) {
    0 as i32;
    // /* Use the built-in recursive mutexes if they are available.
    //   */
    unsafe { pthread_mutex_lock(unsafe { std::ptr::addr_of_mut!((*p).mutex) }) };
}

#[unsafe(link_section = ".text.slate_distinct.mutex_unix.pthreadMutexTry")]
extern "C-unwind" fn pthreadMutexTry(mut p: *mut sqlite3_mutex) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    // /* Use the built-in recursive mutexes if they are available.
    //   */
    if (unsafe { pthread_mutex_trylock(unsafe { std::ptr::addr_of_mut!((*p).mutex) }) })
        == (0 as i32)
    {
        rc = 0 as i32;
    } else {
        rc = 5 as i32;
    }
    return rc;
}

// /*
// ** The sqlite3_mutex_leave() routine exits a mutex that was
// ** previously entered by the same thread.  The behavior
// ** is undefined if the mutex is not currently entered or
// ** is not currently allocated.  SQLite will never do either.
// */
#[unsafe(link_section = ".text.slate_distinct.mutex_unix.pthreadMutexLeave")]
extern "C-unwind" fn pthreadMutexLeave(mut p: *mut sqlite3_mutex) {
    0 as i32;
    0 as i32;
    unsafe { pthread_mutex_unlock(unsafe { std::ptr::addr_of_mut!((*p).mutex) }) };
}
