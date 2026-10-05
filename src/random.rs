unsafe extern "C" {
    fn sqlite3_initialize() -> i32;
    fn sqlite3_vfs_find(zVfsName: *const i8) -> *mut sqlite3_vfs;
    fn sqlite3_mutex_enter(__v98: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(__v99: *mut sqlite3_mutex);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3OsRandomness(__v106: *mut sqlite3_vfs, __v107: i32, __v108: *mut i8) -> i32;
    fn sqlite3MutexAlloc(__v109: i32) -> *mut sqlite3_mutex;
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

// /*
// ** 2001 September 15
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// ** This file contains code to implement a pseudo-random number
// ** generator (PRNG) for SQLite.
// **
// ** Random numbers are used by some of the database backends in order
// ** to generate random integer keys for tables or random filenames.
// */
// /* All threads share a single random number generator.
// ** This structure is the current state of the generator.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3PrngType {
    s: [u32; 16],
    // /* 64 bytes of chacha20 state */
    out: [u8; 64],
    // /* Output bytes */
    n: u8,
    // /* Output bytes remaining */
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

static mut sqlite3Prng: sqlite3PrngType = unsafe { std::mem::zeroed() };

static mut chacha20_init: __SlateAlign16<[u32; 4]> = __SlateAlign16([
    (1634760805 as i32) as u32,
    (857760878 as i32) as u32,
    (2036477234 as i32) as u32,
    (1797285236 as i32) as u32,
]);

// /*
// ** For testing purposes, we sometimes want to preserve the state of
// ** PRNG and restore the PRNG to its saved state at a later time, or
// ** to reset the PRNG to its initial state.  These routines accomplish
// ** those tasks.
// **
// ** The sqlite3_test_control() interface calls these routines to
// ** control the PRNG.
// */
static mut sqlite3SavedPrng: sqlite3PrngType = unsafe { std::mem::zeroed() };

// /*
// ** Return N random bytes.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.random.sqlite3_randomness")]
extern "C-unwind" fn sqlite3_randomness(mut N: i32, mut pBuf: *mut ()) {
    let mut zBuf: *mut u8 = pBuf as *mut u8;
    // /* The "wsdPrng" macro will resolve to the pseudo-random number generator
    //   ** state vector.  If writable static data is unsupported on the target,
    //   ** we have to locate the state vector at run-time.  In the more common
    //   ** case where writable static data is supported, wsdPrng can refer directly
    //   ** to the "sqlite3Prng" state vector declared above.
    //   */
    let mut mutex: *mut sqlite3_mutex = unsafe { std::mem::zeroed() };
    if (unsafe { sqlite3_initialize() }) != (0 as i32) {
        return;
    }
    mutex = unsafe { sqlite3MutexAlloc(5 as i32) };
    unsafe { sqlite3_mutex_enter(mutex) };
    if N <= (0 as i32) || pBuf == std::ptr::null_mut::<()>() {
        unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!(sqlite3Prng.s) as *mut u32 }
                    .offset((0 as i32) as isize)
            } = (0 as i32) as u32;
        }
        unsafe { sqlite3_mutex_leave(mutex) };
        return;
    }
    // /* Initialize the state of the random number generator once,
    //   ** the first time this routine is called.
    //   */
    if (unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of_mut!(sqlite3Prng.s) as *mut u32 }.offset((0 as i32) as isize)
        }
    }) == ((0 as i32) as u32)
    {
        let mut pVfs: *mut sqlite3_vfs = unsafe { sqlite3_vfs_find(std::ptr::null::<i8>()) };
        unsafe {
            memcpy(
                (unsafe {
                    unsafe { std::ptr::addr_of_mut!(sqlite3Prng.s) as *mut u32 }
                        .offset((0 as i32) as isize)
                }) as *mut (),
                (unsafe { std::ptr::addr_of!(chacha20_init.0) as *const u32 }) as *const (),
                ((16 as i32) as i64) as u64,
            )
        };
        if pVfs == std::ptr::null_mut::<sqlite3_vfs>() {
            unsafe {
                memset(
                    (unsafe {
                        unsafe { std::ptr::addr_of_mut!(sqlite3Prng.s) as *mut u32 }
                            .offset((4 as i32) as isize)
                    }) as *mut (),
                    0 as i32,
                    ((44 as i32) as i64) as u64,
                )
            };
        } else {
            unsafe {
                sqlite3OsRandomness(
                    pVfs,
                    44 as i32,
                    (unsafe {
                        unsafe { std::ptr::addr_of_mut!(sqlite3Prng.s) as *mut u32 }
                            .offset((4 as i32) as isize)
                    }) as *mut i8,
                )
            };
        }
        unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!(sqlite3Prng.s) as *mut u32 }
                    .offset((15 as i32) as isize)
            } = unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!(sqlite3Prng.s) as *mut u32 }
                        .offset((12 as i32) as isize)
                }
            };
        }
        unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!(sqlite3Prng.s) as *mut u32 }
                    .offset((12 as i32) as isize)
            } = (0 as i32) as u32;
        }
        unsafe {
            sqlite3Prng.n = ((0 as i32) as i8) as u8;
        }
    }
    0 as i32;
    // /* exit by break */
    '__slate_break_112: while (1 as i32) != (0 as i32) {
        if N <= (((unsafe { sqlite3Prng.n }) as u32) as i32) {
            unsafe {
                memcpy(
                    zBuf as *mut (),
                    (unsafe {
                        unsafe { std::ptr::addr_of_mut!(sqlite3Prng.out) as *mut u8 }
                            .offset(((((unsafe { sqlite3Prng.n }) as u32) as i32) - N) as isize)
                    }) as *const (),
                    (N as i64) as u64,
                )
            };
            let __v113: u8 = unsafe { sqlite3Prng.n };
            let __v114: u8 = ((((__v113 as u32) as i32) - N) as i8) as u8;
            unsafe {
                sqlite3Prng.n = __v114;
            }
            break '__slate_break_112;
        }
        if (((unsafe { sqlite3Prng.n }) as u32) as i32) > (0 as i32) {
            unsafe {
                memcpy(
                    zBuf as *mut (),
                    (unsafe { std::ptr::addr_of_mut!(sqlite3Prng.out) as *mut u8 }) as *const (),
                    (unsafe { sqlite3Prng.n }) as u64,
                )
            };
            let __v115: i32 = N;
            let __v116: i32 = __v115 - (((unsafe { sqlite3Prng.n }) as u32) as i32);
            N = __v116;
            let __v117: *mut u8 = zBuf;
            let __v118: *mut u8 =
                unsafe { __v117.offset((((unsafe { sqlite3Prng.n }) as u32) as i32) as isize) };
            zBuf = __v118;
        }
        let __v119: *mut u32 = unsafe {
            unsafe { std::ptr::addr_of_mut!(sqlite3Prng.s) as *mut u32 }
                .offset((12 as i32) as isize)
        };
        let __v120: u32 = unsafe { *__v119 };
        let __v121: u32 = __v120.wrapping_add((1 as i32) as u32);
        unsafe {
            *__v119 = __v121;
        }
        chacha_block(
            (unsafe { std::ptr::addr_of_mut!(sqlite3Prng.out) as *mut u8 }) as *mut u32,
            (unsafe { std::ptr::addr_of_mut!(sqlite3Prng.s) as *mut u32 }) as *const u32,
        );
        unsafe {
            sqlite3Prng.n = ((64 as i32) as i8) as u8;
        }
    }
    unsafe { sqlite3_mutex_leave(mutex) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PrngSaveState() {
    unsafe {
        memcpy(
            (unsafe { std::ptr::addr_of_mut!(sqlite3SavedPrng) }) as *mut (),
            (unsafe { std::ptr::addr_of_mut!(sqlite3Prng) }) as *const (),
            132 as u64,
        )
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PrngRestoreState() {
    unsafe {
        memcpy(
            (unsafe { std::ptr::addr_of_mut!(sqlite3Prng) }) as *mut (),
            (unsafe { std::ptr::addr_of_mut!(sqlite3SavedPrng) }) as *const (),
            132 as u64,
        )
    };
}

// /* The RFC-7539 ChaCha20 block function
// */
// /* SQLITE_UNTESTABLE */
fn chacha_block(mut out: *mut u32, mut r#in: *const u32) {
    let mut i: i32 = 0 as i32;
    let mut x: __SlateAlign16<[u32; 16]> = __SlateAlign16([0 as u32; 16]);
    unsafe {
        memcpy(
            (x.0.as_mut_ptr() as *mut u32) as *mut (),
            r#in as *const (),
            ((64 as i32) as i64) as u64,
        )
    };
    i = 0 as i32;
    '__slate_break_110: loop {
        if !(i < (10 as i32)) {
            break;
        }
        let __v124: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((0 as i32) as isize) };
        let __v125: u32 = unsafe { *__v124 };
        let __v126: u32 = __v125.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) }
        });
        unsafe {
            *__v124 = __v126;
        }
        let __v127: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) };
        let __v128: u32 = unsafe { *__v127 };
        let __v129: u32 = __v128
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((0 as i32) as isize) } };
        unsafe {
            *__v127 = __v129;
        }
        let __v130: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) } })
                << (16 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) }
                }) >> (32 as i32) - (16 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) } = __v130;
        }
        let __v131: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((8 as i32) as isize) };
        let __v132: u32 = unsafe { *__v131 };
        let __v133: u32 = __v132.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) }
        });
        unsafe {
            *__v131 = __v133;
        }
        let __v134: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) };
        let __v135: u32 = unsafe { *__v134 };
        let __v136: u32 = __v135
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((8 as i32) as isize) } };
        unsafe {
            *__v134 = __v136;
        }
        let __v137: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) } })
                << (12 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) }
                }) >> (32 as i32) - (12 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) } = __v137;
        }
        let __v138: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((0 as i32) as isize) };
        let __v139: u32 = unsafe { *__v138 };
        let __v140: u32 = __v139.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) }
        });
        unsafe {
            *__v138 = __v140;
        }
        let __v141: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) };
        let __v142: u32 = unsafe { *__v141 };
        let __v143: u32 = __v142
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((0 as i32) as isize) } };
        unsafe {
            *__v141 = __v143;
        }
        let __v144: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) } })
                << (8 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) }
                }) >> (32 as i32) - (8 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) } = __v144;
        }
        let __v145: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((8 as i32) as isize) };
        let __v146: u32 = unsafe { *__v145 };
        let __v147: u32 = __v146.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) }
        });
        unsafe {
            *__v145 = __v147;
        }
        let __v148: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) };
        let __v149: u32 = unsafe { *__v148 };
        let __v150: u32 = __v149
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((8 as i32) as isize) } };
        unsafe {
            *__v148 = __v150;
        }
        let __v151: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) } })
                << (7 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) }
                }) >> (32 as i32) - (7 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) } = __v151;
        }
        let __v152: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((1 as i32) as isize) };
        let __v153: u32 = unsafe { *__v152 };
        let __v154: u32 = __v153.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) }
        });
        unsafe {
            *__v152 = __v154;
        }
        let __v155: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) };
        let __v156: u32 = unsafe { *__v155 };
        let __v157: u32 = __v156
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((1 as i32) as isize) } };
        unsafe {
            *__v155 = __v157;
        }
        let __v158: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) } })
                << (16 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) }
                }) >> (32 as i32) - (16 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) } = __v158;
        }
        let __v159: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((9 as i32) as isize) };
        let __v160: u32 = unsafe { *__v159 };
        let __v161: u32 = __v160.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) }
        });
        unsafe {
            *__v159 = __v161;
        }
        let __v162: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) };
        let __v163: u32 = unsafe { *__v162 };
        let __v164: u32 = __v163
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((9 as i32) as isize) } };
        unsafe {
            *__v162 = __v164;
        }
        let __v165: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) } })
                << (12 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) }
                }) >> (32 as i32) - (12 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) } = __v165;
        }
        let __v166: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((1 as i32) as isize) };
        let __v167: u32 = unsafe { *__v166 };
        let __v168: u32 = __v167.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) }
        });
        unsafe {
            *__v166 = __v168;
        }
        let __v169: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) };
        let __v170: u32 = unsafe { *__v169 };
        let __v171: u32 = __v170
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((1 as i32) as isize) } };
        unsafe {
            *__v169 = __v171;
        }
        let __v172: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) } })
                << (8 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) }
                }) >> (32 as i32) - (8 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) } = __v172;
        }
        let __v173: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((9 as i32) as isize) };
        let __v174: u32 = unsafe { *__v173 };
        let __v175: u32 = __v174.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) }
        });
        unsafe {
            *__v173 = __v175;
        }
        let __v176: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) };
        let __v177: u32 = unsafe { *__v176 };
        let __v178: u32 = __v177
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((9 as i32) as isize) } };
        unsafe {
            *__v176 = __v178;
        }
        let __v179: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) } })
                << (7 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) }
                }) >> (32 as i32) - (7 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) } = __v179;
        }
        let __v180: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((2 as i32) as isize) };
        let __v181: u32 = unsafe { *__v180 };
        let __v182: u32 = __v181.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) }
        });
        unsafe {
            *__v180 = __v182;
        }
        let __v183: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) };
        let __v184: u32 = unsafe { *__v183 };
        let __v185: u32 = __v184
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((2 as i32) as isize) } };
        unsafe {
            *__v183 = __v185;
        }
        let __v186: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) } })
                << (16 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) }
                }) >> (32 as i32) - (16 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) } = __v186;
        }
        let __v187: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((10 as i32) as isize) };
        let __v188: u32 = unsafe { *__v187 };
        let __v189: u32 = __v188.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) }
        });
        unsafe {
            *__v187 = __v189;
        }
        let __v190: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) };
        let __v191: u32 = unsafe { *__v190 };
        let __v192: u32 = __v191
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((10 as i32) as isize) } };
        unsafe {
            *__v190 = __v192;
        }
        let __v193: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) } })
                << (12 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) }
                }) >> (32 as i32) - (12 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) } = __v193;
        }
        let __v194: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((2 as i32) as isize) };
        let __v195: u32 = unsafe { *__v194 };
        let __v196: u32 = __v195.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) }
        });
        unsafe {
            *__v194 = __v196;
        }
        let __v197: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) };
        let __v198: u32 = unsafe { *__v197 };
        let __v199: u32 = __v198
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((2 as i32) as isize) } };
        unsafe {
            *__v197 = __v199;
        }
        let __v200: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) } })
                << (8 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) }
                }) >> (32 as i32) - (8 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) } = __v200;
        }
        let __v201: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((10 as i32) as isize) };
        let __v202: u32 = unsafe { *__v201 };
        let __v203: u32 = __v202.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) }
        });
        unsafe {
            *__v201 = __v203;
        }
        let __v204: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) };
        let __v205: u32 = unsafe { *__v204 };
        let __v206: u32 = __v205
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((10 as i32) as isize) } };
        unsafe {
            *__v204 = __v206;
        }
        let __v207: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) } })
                << (7 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) }
                }) >> (32 as i32) - (7 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) } = __v207;
        }
        let __v208: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((3 as i32) as isize) };
        let __v209: u32 = unsafe { *__v208 };
        let __v210: u32 = __v209.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) }
        });
        unsafe {
            *__v208 = __v210;
        }
        let __v211: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) };
        let __v212: u32 = unsafe { *__v211 };
        let __v213: u32 = __v212
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((3 as i32) as isize) } };
        unsafe {
            *__v211 = __v213;
        }
        let __v214: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) } })
                << (16 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) }
                }) >> (32 as i32) - (16 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) } = __v214;
        }
        let __v215: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((11 as i32) as isize) };
        let __v216: u32 = unsafe { *__v215 };
        let __v217: u32 = __v216.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) }
        });
        unsafe {
            *__v215 = __v217;
        }
        let __v218: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) };
        let __v219: u32 = unsafe { *__v218 };
        let __v220: u32 = __v219
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((11 as i32) as isize) } };
        unsafe {
            *__v218 = __v220;
        }
        let __v221: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) } })
                << (12 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) }
                }) >> (32 as i32) - (12 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) } = __v221;
        }
        let __v222: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((3 as i32) as isize) };
        let __v223: u32 = unsafe { *__v222 };
        let __v224: u32 = __v223.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) }
        });
        unsafe {
            *__v222 = __v224;
        }
        let __v225: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) };
        let __v226: u32 = unsafe { *__v225 };
        let __v227: u32 = __v226
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((3 as i32) as isize) } };
        unsafe {
            *__v225 = __v227;
        }
        let __v228: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) } })
                << (8 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) }
                }) >> (32 as i32) - (8 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) } = __v228;
        }
        let __v229: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((11 as i32) as isize) };
        let __v230: u32 = unsafe { *__v229 };
        let __v231: u32 = __v230.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) }
        });
        unsafe {
            *__v229 = __v231;
        }
        let __v232: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) };
        let __v233: u32 = unsafe { *__v232 };
        let __v234: u32 = __v233
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((11 as i32) as isize) } };
        unsafe {
            *__v232 = __v234;
        }
        let __v235: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) } })
                << (7 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) }
                }) >> (32 as i32) - (7 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) } = __v235;
        }
        let __v236: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((0 as i32) as isize) };
        let __v237: u32 = unsafe { *__v236 };
        let __v238: u32 = __v237.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) }
        });
        unsafe {
            *__v236 = __v238;
        }
        let __v239: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) };
        let __v240: u32 = unsafe { *__v239 };
        let __v241: u32 = __v240
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((0 as i32) as isize) } };
        unsafe {
            *__v239 = __v241;
        }
        let __v242: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) } })
                << (16 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) }
                }) >> (32 as i32) - (16 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) } = __v242;
        }
        let __v243: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((10 as i32) as isize) };
        let __v244: u32 = unsafe { *__v243 };
        let __v245: u32 = __v244.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) }
        });
        unsafe {
            *__v243 = __v245;
        }
        let __v246: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) };
        let __v247: u32 = unsafe { *__v246 };
        let __v248: u32 = __v247
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((10 as i32) as isize) } };
        unsafe {
            *__v246 = __v248;
        }
        let __v249: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) } })
                << (12 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) }
                }) >> (32 as i32) - (12 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) } = __v249;
        }
        let __v250: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((0 as i32) as isize) };
        let __v251: u32 = unsafe { *__v250 };
        let __v252: u32 = __v251.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) }
        });
        unsafe {
            *__v250 = __v252;
        }
        let __v253: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) };
        let __v254: u32 = unsafe { *__v253 };
        let __v255: u32 = __v254
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((0 as i32) as isize) } };
        unsafe {
            *__v253 = __v255;
        }
        let __v256: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) } })
                << (8 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) }
                }) >> (32 as i32) - (8 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) } = __v256;
        }
        let __v257: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((10 as i32) as isize) };
        let __v258: u32 = unsafe { *__v257 };
        let __v259: u32 = __v258.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((15 as i32) as isize) }
        });
        unsafe {
            *__v257 = __v259;
        }
        let __v260: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) };
        let __v261: u32 = unsafe { *__v260 };
        let __v262: u32 = __v261
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((10 as i32) as isize) } };
        unsafe {
            *__v260 = __v262;
        }
        let __v263: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) } })
                << (7 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) }
                }) >> (32 as i32) - (7 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((5 as i32) as isize) } = __v263;
        }
        let __v264: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((1 as i32) as isize) };
        let __v265: u32 = unsafe { *__v264 };
        let __v266: u32 = __v265.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) }
        });
        unsafe {
            *__v264 = __v266;
        }
        let __v267: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) };
        let __v268: u32 = unsafe { *__v267 };
        let __v269: u32 = __v268
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((1 as i32) as isize) } };
        unsafe {
            *__v267 = __v269;
        }
        let __v270: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) } })
                << (16 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) }
                }) >> (32 as i32) - (16 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) } = __v270;
        }
        let __v271: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((11 as i32) as isize) };
        let __v272: u32 = unsafe { *__v271 };
        let __v273: u32 = __v272.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) }
        });
        unsafe {
            *__v271 = __v273;
        }
        let __v274: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) };
        let __v275: u32 = unsafe { *__v274 };
        let __v276: u32 = __v275
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((11 as i32) as isize) } };
        unsafe {
            *__v274 = __v276;
        }
        let __v277: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) } })
                << (12 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) }
                }) >> (32 as i32) - (12 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) } = __v277;
        }
        let __v278: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((1 as i32) as isize) };
        let __v279: u32 = unsafe { *__v278 };
        let __v280: u32 = __v279.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) }
        });
        unsafe {
            *__v278 = __v280;
        }
        let __v281: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) };
        let __v282: u32 = unsafe { *__v281 };
        let __v283: u32 = __v282
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((1 as i32) as isize) } };
        unsafe {
            *__v281 = __v283;
        }
        let __v284: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) } })
                << (8 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) }
                }) >> (32 as i32) - (8 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) } = __v284;
        }
        let __v285: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((11 as i32) as isize) };
        let __v286: u32 = unsafe { *__v285 };
        let __v287: u32 = __v286.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((12 as i32) as isize) }
        });
        unsafe {
            *__v285 = __v287;
        }
        let __v288: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) };
        let __v289: u32 = unsafe { *__v288 };
        let __v290: u32 = __v289
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((11 as i32) as isize) } };
        unsafe {
            *__v288 = __v290;
        }
        let __v291: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) } })
                << (7 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) }
                }) >> (32 as i32) - (7 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((6 as i32) as isize) } = __v291;
        }
        let __v292: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((2 as i32) as isize) };
        let __v293: u32 = unsafe { *__v292 };
        let __v294: u32 = __v293.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) }
        });
        unsafe {
            *__v292 = __v294;
        }
        let __v295: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) };
        let __v296: u32 = unsafe { *__v295 };
        let __v297: u32 = __v296
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((2 as i32) as isize) } };
        unsafe {
            *__v295 = __v297;
        }
        let __v298: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) } })
                << (16 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) }
                }) >> (32 as i32) - (16 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) } = __v298;
        }
        let __v299: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((8 as i32) as isize) };
        let __v300: u32 = unsafe { *__v299 };
        let __v301: u32 = __v300.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) }
        });
        unsafe {
            *__v299 = __v301;
        }
        let __v302: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) };
        let __v303: u32 = unsafe { *__v302 };
        let __v304: u32 = __v303
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((8 as i32) as isize) } };
        unsafe {
            *__v302 = __v304;
        }
        let __v305: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) } })
                << (12 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) }
                }) >> (32 as i32) - (12 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) } = __v305;
        }
        let __v306: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((2 as i32) as isize) };
        let __v307: u32 = unsafe { *__v306 };
        let __v308: u32 = __v307.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) }
        });
        unsafe {
            *__v306 = __v308;
        }
        let __v309: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) };
        let __v310: u32 = unsafe { *__v309 };
        let __v311: u32 = __v310
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((2 as i32) as isize) } };
        unsafe {
            *__v309 = __v311;
        }
        let __v312: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) } })
                << (8 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) }
                }) >> (32 as i32) - (8 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) } = __v312;
        }
        let __v313: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((8 as i32) as isize) };
        let __v314: u32 = unsafe { *__v313 };
        let __v315: u32 = __v314.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((13 as i32) as isize) }
        });
        unsafe {
            *__v313 = __v315;
        }
        let __v316: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) };
        let __v317: u32 = unsafe { *__v316 };
        let __v318: u32 = __v317
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((8 as i32) as isize) } };
        unsafe {
            *__v316 = __v318;
        }
        let __v319: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) } })
                << (7 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) }
                }) >> (32 as i32) - (7 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((7 as i32) as isize) } = __v319;
        }
        let __v320: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((3 as i32) as isize) };
        let __v321: u32 = unsafe { *__v320 };
        let __v322: u32 = __v321.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) }
        });
        unsafe {
            *__v320 = __v322;
        }
        let __v323: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) };
        let __v324: u32 = unsafe { *__v323 };
        let __v325: u32 = __v324
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((3 as i32) as isize) } };
        unsafe {
            *__v323 = __v325;
        }
        let __v326: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) } })
                << (16 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) }
                }) >> (32 as i32) - (16 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) } = __v326;
        }
        let __v327: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((9 as i32) as isize) };
        let __v328: u32 = unsafe { *__v327 };
        let __v329: u32 = __v328.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) }
        });
        unsafe {
            *__v327 = __v329;
        }
        let __v330: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) };
        let __v331: u32 = unsafe { *__v330 };
        let __v332: u32 = __v331
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((9 as i32) as isize) } };
        unsafe {
            *__v330 = __v332;
        }
        let __v333: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) } })
                << (12 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) }
                }) >> (32 as i32) - (12 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) } = __v333;
        }
        let __v334: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((3 as i32) as isize) };
        let __v335: u32 = unsafe { *__v334 };
        let __v336: u32 = __v335.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) }
        });
        unsafe {
            *__v334 = __v336;
        }
        let __v337: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) };
        let __v338: u32 = unsafe { *__v337 };
        let __v339: u32 = __v338
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((3 as i32) as isize) } };
        unsafe {
            *__v337 = __v339;
        }
        let __v340: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) } })
                << (8 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) }
                }) >> (32 as i32) - (8 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) } = __v340;
        }
        let __v341: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((9 as i32) as isize) };
        let __v342: u32 = unsafe { *__v341 };
        let __v343: u32 = __v342.wrapping_add(unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((14 as i32) as isize) }
        });
        unsafe {
            *__v341 = __v343;
        }
        let __v344: *mut u32 =
            unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) };
        let __v345: u32 = unsafe { *__v344 };
        let __v346: u32 = __v345
            ^ unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((9 as i32) as isize) } };
        unsafe {
            *__v344 = __v346;
        }
        let __v347: u32 =
            (unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) } })
                << (7 as i32)
                | (unsafe {
                    *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) }
                }) >> (32 as i32) - (7 as i32);
        unsafe {
            *unsafe { (x.0.as_mut_ptr() as *mut u32).offset((4 as i32) as isize) } = __v347;
        }
        let __v122: i32 = i;
        let __v123: i32 = __v122 + (1 as i32);
        i = __v123;
    }
    i = 0 as i32;
    '__slate_break_111: loop {
        if !(i < (16 as i32)) {
            break;
        }
        unsafe {
            *unsafe { out.offset(i as isize) } =
                unsafe { *unsafe { (x.0.as_mut_ptr() as *mut u32).offset(i as isize) } }
                    .wrapping_add(unsafe { *unsafe { r#in.offset(i as isize) } });
        }
        let __v348: i32 = i;
        let __v349: i32 = __v348 + (1 as i32);
        i = __v349;
    }
}
