#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_mutex {}

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

static mut sMutex: sqlite3_mutex_methods = sqlite3_mutex_methods {
    xMutexInit: Some(noopMutexInit),
    xMutexEnd: Some(noopMutexEnd),
    xMutexAlloc: Some(noopMutexAlloc),
    xMutexFree: Some(noopMutexFree),
    xMutexEnter: Some(noopMutexEnter),
    xMutexTry: Some(noopMutexTry),
    xMutexLeave: Some(noopMutexLeave),
    xMutexHeld: None,
    xMutexNotheld: None,
};

// /* !SQLITE_DEBUG */
// /*
// ** If compiled with SQLITE_MUTEX_NOOP, then the no-op mutex implementation
// ** is used regardless of the run-time threadsafety setting.
// */
// /* !defined(SQLITE_MUTEX_OMIT) */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3NoopMutex() -> *const sqlite3_mutex_methods {
    return unsafe { std::ptr::addr_of!(sMutex) };
}

// /*
// ** 2008 October 07
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// ** This file contains the C functions that implement mutexes.
// **
// ** This implementation in this file does not provide any mutual
// ** exclusion and is thus suitable for use only in applications
// ** that use SQLite in a single thread.  The routines defined
// ** here are place-holders.  Applications can substitute working
// ** mutex routines at start-time using the
// **
// **     sqlite3_config(SQLITE_CONFIG_MUTEX,...)
// **
// ** interface.
// **
// ** If compiled with SQLITE_DEBUG, then additional logic is inserted
// ** that does error checking on mutexes to make sure they are being
// ** called correctly.
// */
// /*
// ** Stub routines for all mutex methods.
// **
// ** This routines provide no mutual exclusion or error checking.
// */
#[unsafe(link_section = ".text.slate_distinct.mutex_noop.noopMutexInit")]
extern "C-unwind" fn noopMutexInit() -> i32 {
    return 0 as i32;
}

#[unsafe(link_section = ".text.slate_distinct.mutex_noop.noopMutexEnd")]
extern "C-unwind" fn noopMutexEnd() -> i32 {
    return 0 as i32;
}

#[unsafe(link_section = ".text.slate_distinct.mutex_noop.noopMutexAlloc")]
extern "C-unwind" fn noopMutexAlloc(mut id: i32) -> *mut sqlite3_mutex {
    id;
    return (8 as i32) as *mut sqlite3_mutex;
}

#[unsafe(link_section = ".text.slate_distinct.mutex_noop.noopMutexFree")]
extern "C-unwind" fn noopMutexFree(mut p: *mut sqlite3_mutex) {
    p;
    return;
}

#[unsafe(link_section = ".text.slate_distinct.mutex_noop.noopMutexEnter")]
extern "C-unwind" fn noopMutexEnter(mut p: *mut sqlite3_mutex) {
    p;
    return;
}

#[unsafe(link_section = ".text.slate_distinct.mutex_noop.noopMutexTry")]
extern "C-unwind" fn noopMutexTry(mut p: *mut sqlite3_mutex) -> i32 {
    p;
    return 0 as i32;
}

#[unsafe(link_section = ".text.slate_distinct.mutex_noop.noopMutexLeave")]
extern "C-unwind" fn noopMutexLeave(mut p: *mut sqlite3_mutex) {
    p;
    return;
}
