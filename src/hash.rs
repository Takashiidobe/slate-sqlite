//! 2001 September 22
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This is the implementation of generic hash-tables
//! used in SQLite.
unsafe extern "C" {
    fn sqlite3_free(__v65: *mut ());
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3StrICmp(__v76: *const i8, __v77: *const i8) -> i32;
    fn sqlite3Malloc(__v78: u64) -> *mut ();
    fn sqlite3MallocSize(__v79: *const ()) -> i32;
    fn sqlite3BeginBenignMalloc();
    fn sqlite3EndBenignMalloc();
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Hash {
    htsize: u32,
    count: u32,
    first: *mut HashElem,
    ht: *mut _ht,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct HashElem {
    next: *mut HashElem,
    prev: *mut HashElem,
    data: *mut (),
    pKey: *const i8,
    h: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _ht {
    count: u32,
    chain: *mut HashElem,
}

/// Turn bulk memory into a hash table object by initializing the
/// fields of the Hash structure.
///
/// "pNew" is a pointer to the hash table that is to be initialized.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3HashInit(mut pNew: *mut Hash) {
    0 as i32;
    unsafe {
        (*pNew).first = std::ptr::null_mut::<HashElem>();
    }
    unsafe {
        (*pNew).count = (0 as i32) as u32;
    }
    unsafe {
        (*pNew).htsize = (0 as i32) as u32;
    }
    unsafe {
        (*pNew).ht = std::ptr::null_mut::<_ht>();
    }
}

/// Remove all entries from a hash table.  Reclaim all memory.
/// Call this routine to delete a hash table or to reset a hash table
/// to the empty state.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3HashClear(mut pH: *mut Hash) {
    let mut elem: *mut HashElem = unsafe { std::mem::zeroed() }; // For looping over all elements of the table
    0 as i32;
    elem = unsafe { (*pH).first };
    unsafe {
        (*pH).first = std::ptr::null_mut::<HashElem>();
    }
    unsafe { sqlite3_free((unsafe { (*pH).ht }) as *mut ()) };
    unsafe {
        (*pH).ht = std::ptr::null_mut::<_ht>();
    }
    unsafe {
        (*pH).htsize = (0 as i32) as u32;
    }
    '__slate_break_80: while elem != std::ptr::null_mut::<HashElem>() {
        let mut next_elem: *mut HashElem = unsafe { (*elem).next };
        unsafe { sqlite3_free(elem as *mut ()) };
        elem = next_elem;
    }
    unsafe {
        (*pH).count = (0 as i32) as u32;
    }
}

/// The hashing function.
fn strHash(mut z: *const i8) -> u32 {
    let mut h: u32 = (0 as i32) as u32;
    '__slate_break_81: while (unsafe { *unsafe { z.offset((0 as i32) as isize) } }) != (0 as i8) {
        // Knuth multiplicative hashing.  (Sorting & Searching, p. 510).
        // 0x9e3779b1 is 2654435761 which is the closest prime number to
        // (2**32)*golden_ratio, where golden_ratio = (sqrt(5) - 1)/2.
        //
        // Only bits 0xdf for ASCII and bits 0xbf for EBCDIC each octet are
        // hashed since the omitted bits determine the upper/lower case difference.
        let __v87: u32 = h;
        let __v88: *const i8 = z;
        let __v89: *const i8 = unsafe { __v88.offset((1 as i32) as isize) };
        z = __v89;
        let __v90: u32 = __v87
            .wrapping_add(((223 as i32) & ((((unsafe { *__v88 }) as u8) as u32) as i32)) as u32);
        h = __v90;
        let __v91: u32 = h;
        let __v92: u32 = __v91.wrapping_mul(2654435761 as u32);
        h = __v92;
    }
    return h;
}

/// Link pNew element into the hash table pH.  If pEntry!=0 then also
/// insert pNew into the pEntry hash bucket.
///
/// # Arguments
///
/// * `pH` - The complete hash table
/// * `pEntry` - The entry into which pNew is inserted
/// * `pNew` - The element to be inserted
fn insertElement(mut pH: *mut Hash, mut pEntry: *mut _ht, mut pNew: *mut HashElem) {
    let mut pHead: *mut HashElem = unsafe { std::mem::zeroed() }; // First element already in pEntry
    if pEntry != std::ptr::null_mut::<_ht>() {
        pHead = if (unsafe { (*pEntry).count }) != (0 as u32) {
            unsafe { (*pEntry).chain }
        } else {
            std::ptr::null_mut::<HashElem>()
        };
        let __v93: *mut _ht = pEntry;
        let __v94: u32 = unsafe { (*__v93).count };
        let __v95: u32 = __v94.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v93).count = __v95;
        }
        unsafe {
            (*pEntry).chain = pNew;
        }
    } else {
        pHead = std::ptr::null_mut::<HashElem>();
    }
    if pHead != std::ptr::null_mut::<HashElem>() {
        unsafe {
            (*pNew).next = pHead;
        }
        unsafe {
            (*pNew).prev = unsafe { (*pHead).prev };
        }
        if (unsafe { (*pHead).prev }) != std::ptr::null_mut::<HashElem>() {
            unsafe {
                (*unsafe { (*pHead).prev }).next = pNew;
            }
        } else {
            unsafe {
                (*pH).first = pNew;
            }
        }
        unsafe {
            (*pHead).prev = pNew;
        }
    } else {
        unsafe {
            (*pNew).next = unsafe { (*pH).first };
        }
        if (unsafe { (*pH).first }) != std::ptr::null_mut::<HashElem>() {
            unsafe {
                (*unsafe { (*pH).first }).prev = pNew;
            }
        }
        unsafe {
            (*pNew).prev = std::ptr::null_mut::<HashElem>();
        }
        unsafe {
            (*pH).first = pNew;
        }
    }
}

/// Resize the hash table so that it contains "new_size" buckets.
///
/// The hash table might fail to resize if sqlite3_malloc() fails or
/// if the new size is the same as the prior size.
/// Return TRUE if the resize occurs and false if not.
fn rehash(mut pH: *mut Hash, mut new_size: u32) -> i32 {
    let mut new_ht: *mut _ht = unsafe { std::mem::zeroed() }; // The new hash table
    let mut elem: *mut HashElem = unsafe { std::mem::zeroed() };
    let mut next_elem: *mut HashElem = unsafe { std::mem::zeroed() }; // For looping over existing elements
    if (new_size as u64).wrapping_mul(16 as u64) > (((1024 as i32) as i64) as u64) {
        new_size = ((((1024 as i32) as i64) as u64) / (16 as u64)) as u32;
    }
    if new_size == unsafe { (*pH).htsize } {
        return 0 as i32;
    }
    // The inability to allocates space for a larger hash table is
    // a performance hit but it is not a fatal error.  So mark the
    // allocation as a benign. Use sqlite3Malloc()/memset(0) instead of
    // sqlite3MallocZero() to make the allocation, as sqlite3MallocZero()
    // only zeroes the requested number of bytes whereas this module will
    // use the actual amount of space allocated for the hash table (which
    // may be larger than the requested amount).
    unsafe { sqlite3BeginBenignMalloc() };
    new_ht = (unsafe { sqlite3Malloc((new_size as u64).wrapping_mul(16 as u64)) }) as *mut _ht;
    unsafe { sqlite3EndBenignMalloc() };
    if new_ht == std::ptr::null_mut::<_ht>() {
        return 0 as i32;
    }
    unsafe { sqlite3_free((unsafe { (*pH).ht }) as *mut ()) };
    unsafe {
        (*pH).ht = new_ht;
    }
    let __v96: u32 = ((((unsafe { sqlite3MallocSize(new_ht as *const ()) }) as i64) as u64)
        / (16 as u64)) as u32;
    new_size = __v96;
    unsafe {
        (*pH).htsize = __v96;
    }
    unsafe {
        memset(
            new_ht as *mut (),
            0 as i32,
            (new_size as u64).wrapping_mul(16 as u64),
        )
    };
    elem = unsafe { (*pH).first };
    unsafe {
        (*pH).first = std::ptr::null_mut::<HashElem>();
    }
    '__slate_break_82: while elem != std::ptr::null_mut::<HashElem>() {
        next_elem = unsafe { (*elem).next };
        insertElement(
            pH,
            unsafe { new_ht.offset(((unsafe { (*elem).h }) % new_size) as isize) },
            elem,
        );
        elem = next_elem;
    }
    return 1 as i32;
}

/// This function (for internal use only) locates an element in an
/// hash table that matches the given key.  If no element is found,
/// a pointer to a static null element with HashElem.data==0 is returned.
/// If pH is not NULL, then the hash for this key is written to *pH.
///
/// # Arguments
///
/// * `pH` - The pH to be searched
/// * `pKey` - The key we are searching for
/// * `pHash` - Write the hash value here
fn findElementWithHash(
    mut pH: *const Hash,
    mut pKey: *const i8,
    mut pHash: *mut u32,
) -> *mut HashElem {
    let mut elem: *mut HashElem = unsafe { std::mem::zeroed() }; // Used to loop thru the element list
    let mut count: u32 = 0 as u32; // Number of elements left to test
    let mut h: u32 = 0 as u32; // The computed hash
    h = strHash(pKey);
    if (unsafe { (*pH).ht }) != std::ptr::null_mut::<_ht>() {
        let mut pEntry: *mut _ht = unsafe { std::mem::zeroed() };
        pEntry = unsafe { unsafe { (*pH).ht }.offset((h % unsafe { (*pH).htsize }) as isize) };
        elem = unsafe { (*pEntry).chain };
        count = unsafe { (*pEntry).count };
    } else {
        elem = unsafe { (*pH).first };
        count = unsafe { (*pH).count };
    }
    if pHash != std::ptr::null_mut::<u32>() {
        unsafe {
            *pHash = h;
        }
    }
    '__slate_break_83: while count != (0 as u32) {
        0 as i32;
        let __v97: bool;
        if h == unsafe { (*elem).h } {
            __v97 = (unsafe { sqlite3StrICmp(unsafe { (*elem).pKey }, pKey) }) == (0 as i32);
        } else {
            __v97 = false as bool;
        }
        if __v97 {
            return elem;
        }
        elem = unsafe { (*elem).next };
        let __v98: u32 = count;
        let __v99: u32 = __v98.wrapping_sub((1 as i32) as u32);
        count = __v99;
    }
    return unsafe { std::ptr::addr_of_mut!(nullElement) };
}

static mut nullElement: HashElem = HashElem {
    next: std::ptr::null_mut::<HashElem>(),
    prev: std::ptr::null_mut::<HashElem>(),
    data: std::ptr::null_mut::<()>(),
    pKey: std::ptr::null::<i8>(),
    h: (0 as i32) as u32,
};

/// Remove a single entry from the hash table given a pointer to that
/// element and a hash on the element's key.
///
/// # Arguments
///
/// * `pH` - The pH containing "elem"
/// * `elem` - The element to be removed from the pH
fn removeElement(mut pH: *mut Hash, mut elem: *mut HashElem) {
    let mut pEntry: *mut _ht = unsafe { std::mem::zeroed() };
    if (unsafe { (*elem).prev }) != std::ptr::null_mut::<HashElem>() {
        unsafe {
            (*unsafe { (*elem).prev }).next = unsafe { (*elem).next };
        }
    } else {
        unsafe {
            (*pH).first = unsafe { (*elem).next };
        }
    }
    if (unsafe { (*elem).next }) != std::ptr::null_mut::<HashElem>() {
        unsafe {
            (*unsafe { (*elem).next }).prev = unsafe { (*elem).prev };
        }
    }
    if (unsafe { (*pH).ht }) != std::ptr::null_mut::<_ht>() {
        pEntry = unsafe {
            unsafe { (*pH).ht }.offset(((unsafe { (*elem).h }) % unsafe { (*pH).htsize }) as isize)
        };
        if (unsafe { (*pEntry).chain }) == elem {
            unsafe {
                (*pEntry).chain = unsafe { (*elem).next };
            }
        }
        0 as i32;
        let __v100: *mut _ht = pEntry;
        let __v101: u32 = unsafe { (*__v100).count };
        let __v102: u32 = __v101.wrapping_sub((1 as i32) as u32);
        unsafe {
            (*__v100).count = __v102;
        }
    }
    unsafe { sqlite3_free(elem as *mut ()) };
    let __v103: *mut Hash = pH;
    let __v104: u32 = unsafe { (*__v103).count };
    let __v105: u32 = __v104.wrapping_sub((1 as i32) as u32);
    unsafe {
        (*__v103).count = __v105;
    }
    if (unsafe { (*pH).count }) == ((0 as i32) as u32) {
        0 as i32;
        0 as i32;
        sqlite3HashClear(pH);
    }
}

/// Attempt to locate an element of the hash table pH with a key
/// that matches pKey.  Return the data for this element if it is
/// found, or NULL if there is no match.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3HashFind(mut pH: *const Hash, mut pKey: *const i8) -> *mut () {
    0 as i32;
    0 as i32;
    return unsafe { (*findElementWithHash(pH, pKey, std::ptr::null_mut::<u32>())).data };
}

/// Insert an element into the hash table pH.  The key is pKey
/// and the data is "data".
///
/// If no element exists with a matching key, then a new
/// element is created and NULL is returned.
///
/// If another element already exists with the same key, then the
/// new data replaces the old data and the old data is returned.
/// The key is not copied in this instance.  If a malloc fails, then
/// the new data is returned and the hash table is unchanged.
///
/// If the "data" parameter to this function is NULL, then the
/// element corresponding to "key" is removed from the hash table.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3HashInsert(
    mut pH: *mut Hash,
    mut pKey: *const i8,
    mut data: *mut (),
) -> *mut () {
    let mut h: u32 = 0 as u32; // the hash of the key modulo hash table size
    let mut elem: *mut HashElem = unsafe { std::mem::zeroed() }; // Used to loop thru the element list
    let mut new_elem: *mut HashElem = unsafe { std::mem::zeroed() }; // New element added to the pH
    0 as i32;
    0 as i32;
    elem = findElementWithHash(pH as *const Hash, pKey, std::ptr::addr_of_mut!(h));
    if (unsafe { (*elem).data }) != std::ptr::null_mut::<()>() {
        let mut old_data: *mut () = unsafe { (*elem).data };
        if data == std::ptr::null_mut::<()>() {
            removeElement(pH, elem);
        } else {
            unsafe {
                (*elem).data = data;
            }
            unsafe {
                (*elem).pKey = pKey;
            }
        }
        return old_data;
    }
    if data == std::ptr::null_mut::<()>() {
        return std::ptr::null_mut::<()>();
    }
    new_elem = (unsafe { sqlite3Malloc(40 as u64) }) as *mut HashElem;
    if new_elem == std::ptr::null_mut::<HashElem>() {
        return data;
    }
    unsafe {
        (*new_elem).pKey = pKey;
    }
    unsafe {
        (*new_elem).h = h;
    }
    unsafe {
        (*new_elem).data = data;
    }
    let __v84: *mut Hash = pH;
    let __v85: u32 = unsafe { (*__v84).count };
    let __v86: u32 = __v85.wrapping_add((1 as i32) as u32);
    unsafe {
        (*__v84).count = __v86;
    }
    if (unsafe { (*pH).count }) >= ((5 as i32) as u32)
        && (unsafe { (*pH).count }) > ((2 as i32) as u32).wrapping_mul(unsafe { (*pH).htsize })
    {
        rehash(pH, unsafe { (*pH).count }.wrapping_mul((3 as i32) as u32));
    }
    insertElement(
        pH,
        if (unsafe { (*pH).ht }) != std::ptr::null_mut::<_ht>() {
            unsafe {
                unsafe { (*pH).ht }
                    .offset(((unsafe { (*new_elem).h }) % unsafe { (*pH).htsize }) as isize)
            }
        } else {
            std::ptr::null_mut::<_ht>()
        },
        new_elem,
    );
    return std::ptr::null_mut::<()>();
}
