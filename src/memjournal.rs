unsafe extern "C" {
    fn sqlite3_malloc(__v149: i32) -> *mut ();
    fn sqlite3_free(__v150: *mut ());
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3OsClose(__v157: *mut sqlite3_file);
    fn sqlite3OsWrite(__v158: *mut sqlite3_file, __v159: *const (), amt: i32, offset: i64) -> i32;
    fn sqlite3OsOpen(
        __v162: *mut sqlite3_vfs,
        __v163: *const i8,
        __v164: *mut sqlite3_file,
        __v165: i32,
        __v166: *mut i32,
    ) -> i32;
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
// ** 2008 October 7
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
// ** This file contains code use to implement an in-memory rollback journal.
// ** The in-memory rollback journal is used to journal transactions for
// ** ":memory:" databases and when the journal_mode=MEMORY pragma is used.
// **
// ** Update:  The in-memory journal is also used to temporarily cache
// ** smaller journals that are not critical for power-loss recovery.
// ** For example, statement journals that are not too big will be held
// ** entirely in memory, thus reducing the number of file I/O calls, and
// ** more importantly, reducing temporary file creation events.  If these
// ** journals become too large for memory, they are spilled to disk.  But
// ** in the common case, they are usually small and no file I/O needs to
// ** occur.
// */
// /* Forward references to internal structures */
#[repr(C)]
#[derive(Clone, Copy)]
struct MemJournal {
    // /*
    // ** The rollback journal is composed of a linked list of these structures.
    // **
    // ** The zChunk array is always at least 8 bytes in size - usually much more.
    // ** Its actual size is stored in the MemJournal.nChunkSize variable.
    // */
    // /*
    // ** By default, allocate this many bytes of memory for each FileChunk object.
    // */
    // /*
    // ** For chunk size nChunkSize, return the number of bytes that should
    // ** be allocated for each FileChunk structure.
    // */
    // /*
    // ** An instance of this object serves as a cursor into the rollback journal.
    // ** The cursor can be either for reading or writing.
    // */
    // /*
    // ** This structure is a subclass of sqlite3_file. Each open memory-journal
    // ** is an instance of this class.
    // */
    // /* Next chunk in the journal */
    // /* Content of this chunk */
    // /* Offset from the beginning of the file */
    // /* Specific chunk into which cursor points */
    pMethod: *const sqlite3_io_methods,
    // /* Parent class. MUST BE FIRST */
    nChunkSize: i32,
    // /* In-memory chunk-size */
    nSpill: i32,
    // /* Bytes of data before flushing */
    pFirst: *mut FileChunk,
    // /* Head of in-memory chunk-list */
    endpoint: FilePoint,
    // /* Pointer to the end of the file */
    readpoint: FilePoint,
    // /* Pointer to the end of the last xRead() */
    flags: i32,
    // /* xOpen flags */
    pVfs: *mut sqlite3_vfs,
    // /* The "real" underlying VFS */
    zJournal: *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct FilePoint {
    iOffset: i64,
    pChunk: *mut FileChunk,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct FileChunk {
    pNext: *mut FileChunk,
    zChunk: [u8; 8],
}

// /*
// ** Table of methods for MemJournal sqlite3_file object.
// */
static mut MemJournalMethods: sqlite3_io_methods = sqlite3_io_methods {
    iVersion: 1 as i32,
    xClose: Some(memjrnlClose),
    xRead: Some(memjrnlRead),
    xWrite: Some(memjrnlWrite),
    xTruncate: Some(memjrnlTruncate),
    xSync: Some(memjrnlSync),
    xFileSize: Some(memjrnlFileSize),
    xLock: None,
    xUnlock: None,
    xCheckReservedLock: None,
    xFileControl: None,
    xSectorSize: None,
    xDeviceCharacteristics: None,
    xShmMap: None,
    xShmLock: None,
    xShmBarrier: None,
    xShmUnmap: None,
    xFetch: None,
    xUnfetch: None,
};

// /* iVersion */
// /* xClose */
// /* xRead */
// /* xWrite */
// /* xTruncate */
// /* xSync */
// /* xFileSize */
// /* xLock */
// /* xUnlock */
// /* xCheckReservedLock */
// /* xFileControl */
// /* xSectorSize */
// /* xDeviceCharacteristics */
// /* xShmMap */
// /* xShmLock */
// /* xShmBarrier */
// /* xShmUnmap */
// /* xFetch */
// /* xUnfetch */
// /*
// ** Open a journal file.
// **
// ** The behaviour of the journal file depends on the value of parameter
// ** nSpill. If nSpill is 0, then the journal file is always create and
// ** accessed using the underlying VFS. If nSpill is less than zero, then
// ** all content is always stored in main-memory. Finally, if nSpill is a
// ** positive value, then the journal file is initially created in-memory
// ** but may be flushed to disk later on. In this case the journal file is
// ** flushed to disk either when it grows larger than nSpill bytes in size,
// ** or when sqlite3JournalCreate() is called.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3JournalOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const i8,
    mut pJfd: *mut sqlite3_file,
    mut flags: i32,
    mut nSpill: i32,
) -> i32 {
    let mut p: *mut MemJournal = pJfd as *mut MemJournal;
    0 as i32;
    // /* Zero the file-handle object. If nSpill was passed zero, initialize
    //   ** it using the sqlite3OsOpen() function of the underlying VFS. In this
    //   ** case none of the code in this module is executed as a result of calls
    //   ** made on the journal file-handle.  */
    unsafe { memset(p as *mut (), 0 as i32, 80 as u64) };
    if nSpill == (0 as i32) {
        return unsafe { sqlite3OsOpen(pVfs, zName, pJfd, flags, std::ptr::null_mut::<i32>()) };
    }
    if nSpill > (0 as i32) {
        unsafe {
            (*p).nChunkSize = nSpill;
        }
    } else {
        unsafe {
            (*p).nChunkSize = (((((8 as i32) + (1024 as i32)) as i64) as u64)
                .wrapping_sub(16 as u64) as u32) as i32;
        }
        0 as i32;
    }
    unsafe {
        (*pJfd).pMethods = unsafe { std::ptr::addr_of!(MemJournalMethods) };
    }
    unsafe {
        (*p).nSpill = nSpill;
    }
    unsafe {
        (*p).flags = flags;
    }
    unsafe {
        (*p).zJournal = zName;
    }
    unsafe {
        (*p).pVfs = pVfs;
    }
    return 0 as i32;
}

// /*
// ** Return the number of bytes required to store a JournalFile that uses vfs
// ** pVfs to create the underlying on-disk files.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3JournalSize(mut pVfs: *mut sqlite3_vfs) -> i32 {
    return if (unsafe { (*pVfs).szOsFile }) > (((80 as u64) as u32) as i32) {
        unsafe { (*pVfs).szOsFile }
    } else {
        ((80 as u64) as u32) as i32
    };
}

// /*
// ** The file-handle passed as the only argument is open on a journal file.
// ** Return true if this "journal file" is currently stored in heap memory,
// ** or false otherwise.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3JournalIsInMemory(mut p: *mut sqlite3_file) -> i32 {
    return ((unsafe { (*p).pMethods }) == unsafe { std::ptr::addr_of!(MemJournalMethods) }) as i32;
}

// /* The VFS to use for actual file I/O */
// /* Name of the journal file */
// /* Preallocated, blank file handle */
// /* Opening flags */
// /* Bytes buffered before opening the file */
// /*
// ** Open an in-memory journal file.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MemJournalOpen(mut pJfd: *mut sqlite3_file) {
    sqlite3JournalOpen(
        std::ptr::null_mut::<sqlite3_vfs>(),
        std::ptr::null::<i8>(),
        pJfd,
        0 as i32,
        -(1 as i32),
    );
}

// /* Name of the journal file */
// /*
// ** Read data from the in-memory journal file.  This is the implementation
// ** of the sqlite3_vfs.xRead method.
// */
#[unsafe(link_section = ".text.slate_distinct.memjournal.memjrnlRead")]
extern "C-unwind" fn memjrnlRead(
    mut pJfd: *mut sqlite3_file,
    mut zBuf: *mut (),
    mut iAmt: i32,
    mut iOfst: i64,
) -> i32 {
    let mut p: *mut MemJournal = pJfd as *mut MemJournal;
    let mut zOut: *mut u8 = zBuf as *mut u8;
    let mut nRead: i32 = iAmt;
    let mut iChunkOffset: i32 = 0 as i32;
    let mut pChunk: *mut FileChunk = unsafe { std::mem::zeroed() };
    if (iAmt as i64) + iOfst > unsafe { (*p).endpoint.iOffset } {
        return (10 as i32) | (2 as i32) << (8 as i32);
    }
    0 as i32;
    if (unsafe { (*p).readpoint.iOffset }) != iOfst || iOfst == ((0 as i32) as i64) {
        let mut iOff: i64 = (0 as i32) as i64;
        pChunk = unsafe { (*p).pFirst };
        '__slate_break_175: while pChunk != std::ptr::null_mut::<FileChunk>()
            && iOff + ((unsafe { (*p).nChunkSize }) as i64) <= iOfst
        {
            let __v183: i64 = iOff;
            let __v184: i64 = __v183 + ((unsafe { (*p).nChunkSize }) as i64);
            iOff = __v184;
            pChunk = unsafe { (*pChunk).pNext };
        }
    } else {
        pChunk = unsafe { (*p).readpoint.pChunk };
        0 as i32;
    }
    iChunkOffset = (iOfst % ((unsafe { (*p).nChunkSize }) as i64)) as i32;
    '__slate_break_176: loop {
        let mut iSpace: i32 = (unsafe { (*p).nChunkSize }) - iChunkOffset;
        let mut nCopy: i32 = if nRead < (unsafe { (*p).nChunkSize }) - iChunkOffset {
            nRead
        } else {
            (unsafe { (*p).nChunkSize }) - iChunkOffset
        };
        unsafe {
            memcpy(
                zOut as *mut (),
                (unsafe {
                    unsafe { (*pChunk).zChunk.as_mut_ptr() as *mut u8 }
                        .offset(iChunkOffset as isize)
                }) as *const (),
                (nCopy as i64) as u64,
            )
        };
        let __v185: *mut u8 = zOut;
        let __v186: *mut u8 = unsafe { __v185.offset(nCopy as isize) };
        zOut = __v186;
        let __v187: i32 = nRead;
        let __v188: i32 = __v187 - iSpace;
        nRead = __v188;
        iChunkOffset = 0 as i32;
        let __v189: bool;
        if nRead >= (0 as i32) {
            let __v190: *mut FileChunk = unsafe { (*pChunk).pNext };
            pChunk = __v190;
            __v189 = __v190 != std::ptr::null_mut::<FileChunk>();
        } else {
            __v189 = false as bool;
        }
        if !(__v189 && nRead > (0 as i32)) {
            break;
        }
    }
    unsafe {
        (*p).readpoint.iOffset = if pChunk != std::ptr::null_mut::<FileChunk>() {
            iOfst + (iAmt as i64)
        } else {
            (0 as i32) as i64
        };
    }
    unsafe {
        (*p).readpoint.pChunk = pChunk;
    }
    return 0 as i32;
}

// /* The journal file from which to read */
// /* Put the results here */
// /* Number of bytes to read */
// /* Begin reading at this offset */
// /*
// ** Free the list of FileChunk structures headed at MemJournal.pFirst.
// */
fn memjrnlFreeChunks(mut pFirst: *mut FileChunk) {
    let mut pIter: *mut FileChunk = unsafe { std::mem::zeroed() };
    let mut pNext: *mut FileChunk = unsafe { std::mem::zeroed() };
    pIter = pFirst;
    '__slate_break_177: while pIter != std::ptr::null_mut::<FileChunk>() {
        pNext = unsafe { (*pIter).pNext };
        unsafe { sqlite3_free(pIter as *mut ()) };
        pIter = pNext;
    }
}

// /*
// ** Flush the contents of memory to a real file on disk.
// */
fn memjrnlCreateFile(mut p: *mut MemJournal) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pReal: *mut sqlite3_file = p as *mut sqlite3_file;
    let mut copy: MemJournal = unsafe { *p };
    unsafe { memset(p as *mut (), 0 as i32, 80 as u64) };
    rc = unsafe {
        sqlite3OsOpen(
            copy.pVfs,
            copy.zJournal,
            pReal,
            copy.flags,
            std::ptr::null_mut::<i32>(),
        )
    };
    if rc == (0 as i32) {
        let mut nChunk: i32 = copy.nChunkSize;
        let mut iOff: i64 = (0 as i32) as i64;
        let mut pIter: *mut FileChunk = unsafe { std::mem::zeroed() };
        pIter = copy.pFirst;
        '__slate_break_178: while pIter != std::ptr::null_mut::<FileChunk>() {
            if iOff + (nChunk as i64) > copy.endpoint.iOffset {
                nChunk = (copy.endpoint.iOffset - iOff) as i32;
            }
            rc = unsafe {
                sqlite3OsWrite(
                    pReal,
                    (unsafe { (*pIter).zChunk.as_mut_ptr() as *mut u8 }) as *const (),
                    nChunk,
                    iOff,
                )
            };
            if rc != (0 as i32) {
                break '__slate_break_178;
            }
            let __v191: i64 = iOff;
            let __v192: i64 = __v191 + (nChunk as i64);
            iOff = __v192;
            pIter = unsafe { (*pIter).pNext };
        }
        if rc == (0 as i32) {
            // /* No error has occurred. Free the in-memory buffers. */
            memjrnlFreeChunks(copy.pFirst);
        }
    }
    if rc != (0 as i32) {
        // /* If an error occurred while creating or writing to the file, restore
        //     ** the original before returning. This way, SQLite uses the in-memory
        //     ** journal data to roll back changes made to the internal page-cache
        //     ** before this function was called.  */
        unsafe { sqlite3OsClose(pReal) };
        unsafe {
            *p = copy;
        }
    }
    return rc;
}

// /* The journal file into which to write */
// /* Take data to be written from here */
// /* Number of bytes to write */
// /* Begin writing at this offset into the file */
// /*
// ** Truncate the in-memory file.
// */
#[unsafe(link_section = ".text.slate_distinct.memjournal.memjrnlTruncate")]
extern "C-unwind" fn memjrnlTruncate(mut pJfd: *mut sqlite3_file, mut size: i64) -> i32 {
    let mut p: *mut MemJournal = pJfd as *mut MemJournal;
    0 as i32;
    if size < unsafe { (*p).endpoint.iOffset } {
        let mut pIter: *mut FileChunk = std::ptr::null_mut::<FileChunk>();
        if size == ((0 as i32) as i64) {
            memjrnlFreeChunks(unsafe { (*p).pFirst });
            unsafe {
                (*p).pFirst = std::ptr::null_mut::<FileChunk>();
            }
        } else {
            let mut iOff: i64 = (unsafe { (*p).nChunkSize }) as i64;
            pIter = unsafe { (*p).pFirst };
            '__slate_break_182: while pIter != std::ptr::null_mut::<FileChunk>() && iOff < size {
                let __v193: i64 = iOff;
                let __v194: i64 = __v193 + ((unsafe { (*p).nChunkSize }) as i64);
                iOff = __v194;
                pIter = unsafe { (*pIter).pNext };
            }
            if pIter != std::ptr::null_mut::<FileChunk>() {
                memjrnlFreeChunks(unsafe { (*pIter).pNext });
                unsafe {
                    (*pIter).pNext = std::ptr::null_mut::<FileChunk>();
                }
            }
        }
        unsafe {
            (*p).endpoint.pChunk = pIter;
        }
        unsafe {
            (*p).endpoint.iOffset = size;
        }
        unsafe {
            (*p).readpoint.pChunk = std::ptr::null_mut::<FileChunk>();
        }
        unsafe {
            (*p).readpoint.iOffset = (0 as i32) as i64;
        }
    }
    return 0 as i32;
}

// /* Forward reference */
// /*
// ** Write data to the file.
// */
#[unsafe(link_section = ".text.slate_distinct.memjournal.memjrnlWrite")]
extern "C-unwind" fn memjrnlWrite(
    mut pJfd: *mut sqlite3_file,
    mut zBuf: *const (),
    mut iAmt: i32,
    mut iOfst: i64,
) -> i32 {
    let mut p: *mut MemJournal = pJfd as *mut MemJournal;
    let mut nWrite: i32 = iAmt;
    let mut zWrite: *mut u8 = zBuf as *mut u8;
    // /* If the file should be created now, create it and write the new data
    //   ** into the file on disk. */
    if (unsafe { (*p).nSpill }) > (0 as i32)
        && (iAmt as i64) + iOfst > ((unsafe { (*p).nSpill }) as i64)
    {
        let mut rc: i32 = memjrnlCreateFile(p);
        if rc == (0 as i32) {
            rc = unsafe { sqlite3OsWrite(pJfd, zBuf, iAmt, iOfst) };
        }
        return rc;
    } else {
        // /* An in-memory journal file should only ever be appended to. Random
        //     ** access writes are not required. The only exception to this is when
        //     ** the in-memory journal is being used by a connection using the
        //     ** atomic-write optimization. In this case the first 28 bytes of the
        //     ** journal file may be written as part of committing the transaction. */
        0 as i32;
        if iOfst > ((0 as i32) as i64) && iOfst != unsafe { (*p).endpoint.iOffset } {
            memjrnlTruncate(pJfd, iOfst);
        }
        if iOfst == ((0 as i32) as i64)
            && (unsafe { (*p).pFirst }) != std::ptr::null_mut::<FileChunk>()
        {
            0 as i32;
            unsafe {
                memcpy(
                    (unsafe { (*unsafe { (*p).pFirst }).zChunk.as_mut_ptr() as *mut u8 })
                        as *mut (),
                    zBuf,
                    (iAmt as i64) as u64,
                )
            };
        } else {
            '__slate_break_181: while nWrite > (0 as i32) {
                let mut pChunk: *mut FileChunk = unsafe { (*p).endpoint.pChunk };
                let mut iChunkOffset: i32 = ((unsafe { (*p).endpoint.iOffset })
                    % ((unsafe { (*p).nChunkSize }) as i64))
                    as i32;
                let mut iSpace: i32 = if nWrite < (unsafe { (*p).nChunkSize }) - iChunkOffset {
                    nWrite
                } else {
                    (unsafe { (*p).nChunkSize }) - iChunkOffset
                };
                0 as i32;
                if iChunkOffset == (0 as i32) {
                    // /* New chunk is required to extend the file. */
                    let mut pNew: *mut FileChunk = (unsafe {
                        sqlite3_malloc(
                            ((16 as u64).wrapping_add(
                                (((unsafe { (*p).nChunkSize }) - (8 as i32)) as i64) as u64,
                            ) as u32) as i32,
                        )
                    }) as *mut FileChunk;
                    if !(pNew != std::ptr::null_mut::<FileChunk>()) {
                        return (10 as i32) | (12 as i32) << (8 as i32);
                    }
                    unsafe {
                        (*pNew).pNext = std::ptr::null_mut::<FileChunk>();
                    }
                    if pChunk != std::ptr::null_mut::<FileChunk>() {
                        0 as i32;
                        unsafe {
                            (*pChunk).pNext = pNew;
                        }
                    } else {
                        0 as i32;
                        unsafe {
                            (*p).pFirst = pNew;
                        }
                    }
                    let __v195: *mut FileChunk = pNew;
                    unsafe {
                        (*p).endpoint.pChunk = __v195;
                    }
                    pChunk = __v195;
                }
                0 as i32;
                unsafe {
                    memcpy(
                        (unsafe {
                            unsafe { (*pChunk).zChunk.as_mut_ptr() as *mut u8 }
                                .offset(iChunkOffset as isize)
                        }) as *mut (),
                        zWrite as *const (),
                        (iSpace as i64) as u64,
                    )
                };
                let __v196: *mut u8 = zWrite;
                let __v197: *mut u8 = unsafe { __v196.offset(iSpace as isize) };
                zWrite = __v197;
                let __v198: i32 = nWrite;
                let __v199: i32 = __v198 - iSpace;
                nWrite = __v199;
                let __v200: *mut MemJournal = p;
                let __v201: i64 = unsafe { (*__v200).endpoint.iOffset };
                let __v202: i64 = __v201 + (iSpace as i64);
                unsafe {
                    (*__v200).endpoint.iOffset = __v202;
                }
            }
        }
    }
    // /* If the contents of this write should be stored in memory */
    return 0 as i32;
}

// /*
// ** Close the file.
// */
#[unsafe(link_section = ".text.slate_distinct.memjournal.memjrnlClose")]
extern "C-unwind" fn memjrnlClose(mut pJfd: *mut sqlite3_file) -> i32 {
    let mut p: *mut MemJournal = pJfd as *mut MemJournal;
    memjrnlFreeChunks(unsafe { (*p).pFirst });
    return 0 as i32;
}

// /*
// ** Sync the file.
// **
// ** If the real file has been created, call its xSync method. Otherwise,
// ** syncing an in-memory journal is a no-op.
// */
#[unsafe(link_section = ".text.slate_distinct.memjournal.memjrnlSync")]
extern "C-unwind" fn memjrnlSync(mut pJfd: *mut sqlite3_file, mut flags: i32) -> i32 {
    pJfd;
    flags;
    return 0 as i32;
}

// /*
// ** Query the size of the file in bytes.
// */
#[unsafe(link_section = ".text.slate_distinct.memjournal.memjrnlFileSize")]
extern "C-unwind" fn memjrnlFileSize(mut pJfd: *mut sqlite3_file, mut pSize: *mut i64) -> i32 {
    let mut p: *mut MemJournal = pJfd as *mut MemJournal;
    unsafe {
        *pSize = unsafe { (*p).endpoint.iOffset };
    }
    return 0 as i32;
}
