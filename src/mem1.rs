unsafe extern "C" {
    fn sqlite3_config(__v37: i32, ...) -> i32;
    fn sqlite3_log(iErrCode: i32, zFormat: *const i8, ...);
    fn malloc(__size: u64) -> *mut ();
    fn realloc(__ptr: *mut (), __size: u64) -> *mut ();
    fn free(__ptr: *mut ());
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

static mut defaultMethods: sqlite3_mem_methods = sqlite3_mem_methods {
    xMalloc: Some(sqlite3MemMalloc),
    xFree: Some(sqlite3MemFree),
    xRealloc: Some(sqlite3MemRealloc),
    xSize: Some(sqlite3MemSize),
    xRoundup: Some(sqlite3MemRoundup),
    xInit: Some(sqlite3MemInit),
    xShutdown: Some(sqlite3MemShutdown),
    pAppData: std::ptr::null_mut::<()>(),
};

// /*
// ** This routine is the only routine in this file with external linkage.
// **
// ** Populate the low-level memory allocation function pointers in
// ** sqlite3GlobalConfig.m with pointers to the routines in this file.
// */
// /* SQLITE_SYSTEM_MALLOC */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MemSetDefault() {
    unsafe { sqlite3_config(4 as i32, unsafe { std::ptr::addr_of!(defaultMethods) }) };
}

// /*
// ** 2007 August 14
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// **
// ** This file contains low-level memory allocation drivers for when
// ** SQLite will use the standard C-library malloc/realloc/free interface
// ** to obtain the memory it needs.
// **
// ** This file contains implementations of the low-level memory allocation
// ** routines specified in the sqlite3_mem_methods object.  The content of
// ** this file is only used if SQLITE_SYSTEM_MALLOC is defined.  The
// ** SQLITE_SYSTEM_MALLOC macro is defined automatically if neither the
// ** SQLITE_MEMDEBUG nor the SQLITE_WIN32_MALLOC macros are defined.  The
// ** default configuration is to use memory allocation routines in this
// ** file.
// **
// ** C-preprocessor macro summary:
// **
// **    HAVE_MALLOC_USABLE_SIZE     The configure script sets this symbol if
// **                                the malloc_usable_size() interface exists
// **                                on the target platform.  Or, this symbol
// **                                can be set manually, if desired.
// **                                If an equivalent interface exists by
// **                                a different name, using a separate -D
// **                                option to rename it.
// **
// **    SQLITE_WITHOUT_ZONEMALLOC   Some older macs lack support for the zone
// **                                memory allocator.  Set this symbol to enable
// **                                building on older macs.
// **
// **    SQLITE_WITHOUT_MSIZE        Set this symbol to disable the use of
// **                                _msize() on windows systems.  This might
// **                                be necessary when compiling for Delphi,
// **                                for example.
// */
// /*
// ** This version of the memory allocator is the default.  It is
// ** used when no other memory allocator is specified using compile-time
// ** macros.
// */
// /*
// ** Use standard C library malloc and free on non-Apple systems.
// ** Also used by Apple systems if SQLITE_WITHOUT_ZONEMALLOC is defined.
// */
// /*
// ** The malloc.h header file is needed for malloc_usable_size() function
// ** on some systems (e.g. Linux).
// */
// /*
// ** Include the malloc.h header file, if necessary.  Also set define macro
// ** SQLITE_MALLOCSIZE to the appropriate function name, which is _msize()
// ** for MSVC and malloc_usable_size() for most other systems (e.g. Linux).
// ** The memory size function can always be overridden manually by defining
// ** the macro SQLITE_MALLOCSIZE to the desired function name.
// */
// /* __APPLE__ or not __APPLE__ */
// /*
// ** Like malloc(), but remember the size of the allocation
// ** so that we can find it later using sqlite3MemSize().
// **
// ** For this low-level routine, we are guaranteed that nByte>0 because
// ** cases of nByte<=0 will be intercepted and dealt with by higher level
// ** routines.
// */
#[unsafe(link_section = ".text.slate_distinct.mem1.sqlite3MemMalloc")]
extern "C-unwind" fn sqlite3MemMalloc(mut nByte: i32) -> *mut () {
    let mut p: *mut i64 = unsafe { std::mem::zeroed() };
    0 as i32;
    {}
    p = (unsafe { malloc(((nByte + (8 as i32)) as i64) as u64) }) as *mut i64;
    if p != std::ptr::null_mut::<i64>() {
        unsafe {
            *unsafe { p.offset((0 as i32) as isize) } = nByte as i64;
        }
        let __v46: *mut i64 = p;
        let __v47: *mut i64 = unsafe { __v46.offset((1 as i32) as isize) };
        p = __v47;
    } else {
        {}
        unsafe {
            sqlite3_log(
                7 as i32,
                (b"failed to allocate %u bytes of memory\0".as_ptr() as *mut i8) as *const i8,
                nByte,
            )
        };
    }
    return p as *mut ();
}

// /*
// ** Like free() but works for allocations obtained from sqlite3MemMalloc()
// ** or sqlite3MemRealloc().
// **
// ** For this low-level routine, we already know that pPrior!=0 since
// ** cases where pPrior==0 will have been intercepted and dealt with
// ** by higher-level routines.
// */
#[unsafe(link_section = ".text.slate_distinct.mem1.sqlite3MemFree")]
extern "C-unwind" fn sqlite3MemFree(mut pPrior: *mut ()) {
    let mut p: *mut i64 = pPrior as *mut i64;
    0 as i32;
    let __v48: *mut i64 = p;
    let __v49: *mut i64 = unsafe { __v48.offset(-((1 as i32) as isize)) };
    p = __v49;
    unsafe { free(p as *mut ()) };
}

// /*
// ** Report the allocated size of a prior return from xMalloc()
// ** or xRealloc().
// */
#[unsafe(link_section = ".text.slate_distinct.mem1.sqlite3MemSize")]
extern "C-unwind" fn sqlite3MemSize(mut pPrior: *mut ()) -> i32 {
    let mut p: *mut i64 = unsafe { std::mem::zeroed() };
    0 as i32;
    p = pPrior as *mut i64;
    let __v50: *mut i64 = p;
    let __v51: *mut i64 = unsafe { __v50.offset(-((1 as i32) as isize)) };
    p = __v51;
    return (unsafe { *unsafe { p.offset((0 as i32) as isize) } }) as i32;
}

// /*
// ** Like realloc().  Resize an allocation previously obtained from
// ** sqlite3MemMalloc().
// **
// ** For this low-level interface, we know that pPrior!=0.  Cases where
// ** pPrior==0 while have been intercepted by higher-level routine and
// ** redirected to xMalloc.  Similarly, we know that nByte>0 because
// ** cases where nByte<=0 will have been intercepted by higher-level
// ** routines and redirected to xFree.
// */
#[unsafe(link_section = ".text.slate_distinct.mem1.sqlite3MemRealloc")]
extern "C-unwind" fn sqlite3MemRealloc(mut pPrior: *mut (), mut nByte: i32) -> *mut () {
    let mut p: *mut i64 = pPrior as *mut i64;
    0 as i32;
    // /* EV: R-46199-30249 */
    0 as i32;
    let __v52: *mut i64 = p;
    let __v53: *mut i64 = unsafe { __v52.offset(-((1 as i32) as isize)) };
    p = __v53;
    p = (unsafe { realloc(p as *mut (), ((nByte + (8 as i32)) as i64) as u64) }) as *mut i64;
    if p != std::ptr::null_mut::<i64>() {
        unsafe {
            *unsafe { p.offset((0 as i32) as isize) } = nByte as i64;
        }
        let __v54: *mut i64 = p;
        let __v55: *mut i64 = unsafe { __v54.offset((1 as i32) as isize) };
        p = __v55;
    } else {
        {}
        unsafe {
            sqlite3_log(
                7 as i32,
                (b"failed memory resize %u to %u bytes\0".as_ptr() as *mut i8) as *const i8,
                sqlite3MemSize(pPrior),
                nByte,
            )
        };
    }
    return p as *mut ();
}

// /*
// ** Round up a request size to the next valid allocation size.
// */
#[unsafe(link_section = ".text.slate_distinct.mem1.sqlite3MemRoundup")]
extern "C-unwind" fn sqlite3MemRoundup(mut n: i32) -> i32 {
    return n + (7 as i32) & !(7 as i32);
}

// /*
// ** Initialize this module.
// */
#[unsafe(link_section = ".text.slate_distinct.mem1.sqlite3MemInit")]
extern "C-unwind" fn sqlite3MemInit(mut NotUsed: *mut ()) -> i32 {
    NotUsed;
    return 0 as i32;
}

// /*
// ** Deinitialize this module.
// */
#[unsafe(link_section = ".text.slate_distinct.mem1.sqlite3MemShutdown")]
extern "C-unwind" fn sqlite3MemShutdown(mut NotUsed: *mut ()) {
    NotUsed;
    return;
}
