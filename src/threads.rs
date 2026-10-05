unsafe extern "C" {
    fn sqlite3_free(__v33: *mut ());
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3Malloc(__v37: u64) -> *mut ();
    fn sqlite3FaultSim(__v38: i32) -> i32;
    fn pthread_create(
        __newthread: *mut u64,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C-unwind" fn(*mut ()) -> *mut ()>,
        __arg: *mut (),
    ) -> i32;
    fn pthread_join(__th: u64, __thread_return: *mut *mut ()) -> i32;
}

#[repr(C)]
#[derive(Clone, Copy)]
union pthread_attr_t {
    __size: [i8; 56],
    __align: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SQLiteThread {
    tid: u64,
    done: i32,
    pOut: *mut (),
    xTask: Option<unsafe extern "C-unwind" fn(*mut ()) -> *mut ()>,
    pIn: *mut (),
}

// /*
// ** 2012 July 21
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
// ** This file presents a simple cross-platform threading interface for
// ** use internally by SQLite.
// **
// ** A "thread" can be created using sqlite3ThreadCreate().  This thread
// ** runs independently of its creator until it is joined using
// ** sqlite3ThreadJoin(), at which point it terminates.
// **
// ** Threads do not have to be real.  It could be that the work of the
// ** "thread" is done by the main thread at either the sqlite3ThreadCreate()
// ** or sqlite3ThreadJoin() call.  This is, in fact, what happens in
// ** single threaded systems.  Nothing in SQLite requires multiple threads.
// ** This interface exists so that applications that want to take advantage
// ** of multiple cores can do so, while also allowing applications to stay
// ** single-threaded if desired.
// */
// /********************************* Unix Pthreads ****************************/
// /* Prevent the single-thread code below */
// /* A running thread */
// /* Thread ID */
// /* Set to true when thread finishes */
// /* Result returned by the thread */
// /* The thread routine */
// /* Argument to the thread */
// /* Create a new thread */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ThreadCreate(
    mut ppThread: *mut *mut SQLiteThread,
    mut xTask: Option<unsafe extern "C-unwind" fn(*mut ()) -> *mut ()>,
    mut pIn: *mut (),
) -> i32 {
    let mut p: *mut SQLiteThread = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    // /* This routine is never used in single-threaded mode */
    0 as i32;
    unsafe {
        *ppThread = std::ptr::null_mut::<SQLiteThread>();
    }
    p = (unsafe { sqlite3Malloc(40 as u64) }) as *mut SQLiteThread;
    if p == std::ptr::null_mut::<SQLiteThread>() {
        return 7 as i32;
    }
    unsafe { memset(p as *mut (), 0 as i32, 40 as u64) };
    unsafe {
        (*p).xTask = xTask;
    }
    unsafe {
        (*p).pIn = pIn;
    }
    // /* If the SQLITE_TESTCTRL_FAULT_INSTALL callback is registered to a
    //   ** function that returns SQLITE_ERROR when passed the argument 200, that
    //   ** forces worker threads to run sequentially and deterministically
    //   ** for testing purposes. */
    if (unsafe { sqlite3FaultSim(200 as i32) }) != (0 as i32) {
        rc = 1 as i32;
    } else {
        rc = unsafe {
            pthread_create(
                unsafe { std::ptr::addr_of_mut!((*p).tid) },
                std::ptr::null::<pthread_attr_t>(),
                xTask,
                pIn,
            )
        };
    }
    if rc != (0 as i32) {
        unsafe {
            (*p).done = 1 as i32;
        }
        unsafe {
            (*p).pOut = unsafe { xTask.unwrap()(pIn) };
        }
    }
    unsafe {
        *ppThread = p;
    }
    return 0 as i32;
}

// /* OUT: Write the thread object here */
// /* Routine to run in a separate thread */
// /* Argument passed into xTask() */
// /* Get the results of the thread */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ThreadJoin(mut p: *mut SQLiteThread, mut ppOut: *mut *mut ()) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    if p == std::ptr::null_mut::<SQLiteThread>() {
        return 7 as i32;
    }
    if (unsafe { (*p).done }) != (0 as i32) {
        unsafe {
            *ppOut = unsafe { (*p).pOut };
        }
        rc = 0 as i32;
    } else {
        rc = if (unsafe { pthread_join(unsafe { (*p).tid }, ppOut) }) != (0 as i32) {
            1 as i32
        } else {
            0 as i32
        };
    }
    unsafe { sqlite3_free(p as *mut ()) };
    return rc;
}

// /* SQLITE_OS_UNIX && defined(SQLITE_MUTEX_PTHREADS) */
// /******************************** End Unix Pthreads *************************/
// /********************************* Win32 Threads ****************************/
// /******************************** End Win32 Threads *************************/
// /********************************* Single-Threaded **************************/
// /****************************** End Single-Threaded *************************/
// /* SQLITE_MAX_WORKER_THREADS>0 */
