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
//! This is the implementation of generic hash-tables used in SQLite.
//! We've modified it slightly to serve as a standalone hash table
//! implementation for the full-text indexing module.
unsafe extern "C" {
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64) -> i32;
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3_malloc64(__v137: u64) -> *mut ();
    fn sqlite3_free(__v138: *mut ());
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Hash {
    keyClass: i8,
    copyKey: i8,
    count: i32,
    first: *mut Fts3HashElem,
    htsize: i32,
    ht: *mut _fts3ht,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3HashElem {
    next: *mut Fts3HashElem,
    prev: *mut Fts3HashElem,
    data: *mut (),
    pKey: *mut (),
    nKey: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _fts3ht {
    count: i32,
    chain: *mut Fts3HashElem,
}

// The code in this file is only compiled if:
//
//     * The FTS3 module is being built as an extension
//       (in which case SQLITE_CORE is not defined), or
//
//     * The FTS3 module is being built into the core of
//       SQLite (in which case SQLITE_ENABLE_FTS3 is defined).
/// Malloc and Free functions
fn fts3HashMalloc(mut n: i64) -> *mut () {
    let mut p: *mut () = unsafe { sqlite3_malloc64(n as u64) };
    if p != std::ptr::null_mut::<()>() {
        unsafe { memset(p, 0 as i32, n as u64) };
    }
    return p;
}

fn fts3HashFree(mut p: *mut ()) {
    unsafe { sqlite3_free(p) };
}

/// Turn bulk memory into a hash table object by initializing the
/// fields of the Hash structure.
///
/// "pNew" is a pointer to the hash table that is to be initialized.
/// keyClass is one of the constants
/// FTS3_HASH_BINARY or FTS3_HASH_STRING.  The value of keyClass
/// determines what kind of key the hash table will use.  "copyKey" is
/// true if the hash table should make its own private copy of keys and
/// false if it should just use the supplied pointer.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3HashInit(
    mut pNew: *mut Fts3Hash,
    mut keyClass: i8,
    mut copyKey: i8,
) {
    0 as i32;
    0 as i32;
    unsafe {
        (*pNew).keyClass = keyClass;
    }
    unsafe {
        (*pNew).copyKey = copyKey;
    }
    unsafe {
        (*pNew).first = std::ptr::null_mut::<Fts3HashElem>();
    }
    unsafe {
        (*pNew).count = 0 as i32;
    }
    unsafe {
        (*pNew).htsize = 0 as i32;
    }
    unsafe {
        (*pNew).ht = std::ptr::null_mut::<_fts3ht>();
    }
}

/// Remove all entries from a hash table.  Reclaim all memory.
/// Call this routine to delete a hash table or to reset a hash table
/// to the empty state.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3HashClear(mut pH: *mut Fts3Hash) {
    let mut elem: *mut Fts3HashElem = unsafe { std::mem::zeroed() }; // For looping over all elements of the table
    0 as i32;
    elem = unsafe { (*pH).first };
    unsafe {
        (*pH).first = std::ptr::null_mut::<Fts3HashElem>();
    }
    fts3HashFree((unsafe { (*pH).ht }) as *mut ());
    unsafe {
        (*pH).ht = std::ptr::null_mut::<_fts3ht>();
    }
    unsafe {
        (*pH).htsize = 0 as i32;
    }
    '__slate_break_153: while elem != std::ptr::null_mut::<Fts3HashElem>() {
        let mut next_elem: *mut Fts3HashElem = unsafe { (*elem).next };
        if (unsafe { (*pH).copyKey }) != (0 as i8)
            && (unsafe { (*elem).pKey }) != std::ptr::null_mut::<()>()
        {
            fts3HashFree(unsafe { (*elem).pKey });
        }
        fts3HashFree(elem as *mut ());
        elem = next_elem;
    }
    unsafe {
        (*pH).count = 0 as i32;
    }
}

/// Hash and comparison functions when the mode is FTS3_HASH_STRING
#[unsafe(link_section = ".text.slate_distinct.fts3_hash.fts3StrHash")]
extern "C-unwind" fn fts3StrHash(mut pKey: *const (), mut nKey: i32) -> i32 {
    let mut z: *const i8 = pKey as *const i8;
    let mut h: u32 = (0 as i32) as u32;
    if nKey <= (0 as i32) {
        nKey = ((unsafe { strlen(z) }) as u32) as i32;
    }
    '__slate_break_154: while nKey > (0 as i32) {
        let __v164: *const i8 = z;
        let __v165: *const i8 = unsafe { __v164.offset((1 as i32) as isize) };
        z = __v165;
        h = h << (3 as i32) ^ h ^ (((unsafe { *__v164 }) as i32) as u32);
        let __v166: i32 = nKey;
        let __v167: i32 = __v166 - (1 as i32);
        nKey = __v167;
    }
    return (h & ((2147483647 as i32) as u32)) as i32;
}

#[unsafe(link_section = ".text.slate_distinct.fts3_hash.fts3StrCompare")]
extern "C-unwind" fn fts3StrCompare(
    mut pKey1: *const (),
    mut n1: i32,
    mut pKey2: *const (),
    mut n2: i32,
) -> i32 {
    if n1 != n2 {
        return 1 as i32;
    }
    return unsafe { strncmp(pKey1 as *const i8, pKey2 as *const i8, (n1 as i64) as u64) };
}

/// Hash and comparison functions when the mode is FTS3_HASH_BINARY
#[unsafe(link_section = ".text.slate_distinct.fts3_hash.fts3BinHash")]
extern "C-unwind" fn fts3BinHash(mut pKey: *const (), mut nKey: i32) -> i32 {
    let mut h: i32 = 0 as i32;
    let mut z: *const i8 = pKey as *const i8;
    '__slate_break_155: loop {
        let __v168: i32 = nKey;
        let __v169: i32 = __v168 - (1 as i32);
        nKey = __v169;
        if !(__v168 > (0 as i32)) {
            break;
        }
        let __v170: *const i8 = z;
        let __v171: *const i8 = unsafe { __v170.offset((1 as i32) as isize) };
        z = __v171;
        h = h << (3 as i32) ^ h ^ ((unsafe { *__v170 }) as i32);
    }
    return h & (2147483647 as i32);
}

#[unsafe(link_section = ".text.slate_distinct.fts3_hash.fts3BinCompare")]
extern "C-unwind" fn fts3BinCompare(
    mut pKey1: *const (),
    mut n1: i32,
    mut pKey2: *const (),
    mut n2: i32,
) -> i32 {
    if n1 != n2 {
        return 1 as i32;
    }
    return unsafe { memcmp(pKey1, pKey2, (n1 as i64) as u64) };
}

/// Return a pointer to the appropriate hash function given the key class.
///
/// The C syntax in this function definition may be unfamilar to some
/// programmers, so we provide the following additional explanation:
///
/// The name of the function is "ftsHashFunction".  The function takes a
/// single parameter "keyClass".  The return value of ftsHashFunction()
/// is a pointer to another function.  Specifically, the return value
/// of ftsHashFunction() is a pointer to a function that takes two parameters
/// with types "const void*" and "int" and returns an "int".
fn ftsHashFunction(
    mut keyClass: i32,
) -> Option<unsafe extern "C-unwind" fn(*const (), i32) -> i32> {
    if keyClass == (1 as i32) {
        return Some(fts3StrHash);
    } else {
        0 as i32;
        return Some(fts3BinHash);
    }
    return unsafe { std::mem::zeroed() };
}

/// Return a pointer to the appropriate hash function given the key class.
///
/// For help in interpreted the obscure C code in the function definition,
/// see the header comment on the previous function.
fn ftsCompareFunction(
    mut keyClass: i32,
) -> Option<unsafe extern "C-unwind" fn(*const (), i32, *const (), i32) -> i32> {
    if keyClass == (1 as i32) {
        return Some(fts3StrCompare);
    } else {
        0 as i32;
        return Some(fts3BinCompare);
    }
    return unsafe { std::mem::zeroed() };
}

/// Link an element into the hash table
///
/// # Arguments
///
/// * `pH` - The complete hash table
/// * `pEntry` - The entry into which pNew is inserted
/// * `pNew` - The element to be inserted
fn fts3HashInsertElement(
    mut pH: *mut Fts3Hash,
    mut pEntry: *mut _fts3ht,
    mut pNew: *mut Fts3HashElem,
) {
    let mut pHead: *mut Fts3HashElem = unsafe { std::mem::zeroed() }; // First element already in pEntry
    pHead = unsafe { (*pEntry).chain };
    if pHead != std::ptr::null_mut::<Fts3HashElem>() {
        unsafe {
            (*pNew).next = pHead;
        }
        unsafe {
            (*pNew).prev = unsafe { (*pHead).prev };
        }
        if (unsafe { (*pHead).prev }) != std::ptr::null_mut::<Fts3HashElem>() {
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
        if (unsafe { (*pH).first }) != std::ptr::null_mut::<Fts3HashElem>() {
            unsafe {
                (*unsafe { (*pH).first }).prev = pNew;
            }
        }
        unsafe {
            (*pNew).prev = std::ptr::null_mut::<Fts3HashElem>();
        }
        unsafe {
            (*pH).first = pNew;
        }
    }
    let __v172: *mut _fts3ht = pEntry;
    let __v173: i32 = unsafe { (*__v172).count };
    let __v174: i32 = __v173 + (1 as i32);
    unsafe {
        (*__v172).count = __v174;
    }
    unsafe {
        (*pEntry).chain = pNew;
    }
}

/// Resize the hash table so that it contains "new_size" buckets.
/// "new_size" must be a power of 2.  The hash table might fail
/// to resize if sqliteMalloc() fails.
///
/// Return non-zero if a memory allocation error occurs.
fn fts3Rehash(mut pH: *mut Fts3Hash, mut new_size: i32) -> i32 {
    let mut new_ht: *mut _fts3ht = unsafe { std::mem::zeroed() }; // The new hash table
    let mut elem: *mut Fts3HashElem = unsafe { std::mem::zeroed() };
    let mut next_elem: *mut Fts3HashElem = unsafe { std::mem::zeroed() }; // For looping over existing elements
    let mut xHash: Option<unsafe extern "C-unwind" fn(*const (), i32) -> i32> =
        unsafe { std::mem::zeroed() }; // The hash function
    0 as i32;
    new_ht =
        fts3HashMalloc(((new_size as i64) as u64).wrapping_mul(16 as u64) as i64) as *mut _fts3ht;
    if new_ht == std::ptr::null_mut::<_fts3ht>() {
        return 1 as i32;
    }
    fts3HashFree((unsafe { (*pH).ht }) as *mut ());
    unsafe {
        (*pH).ht = new_ht;
    }
    unsafe {
        (*pH).htsize = new_size;
    }
    xHash = ftsHashFunction((unsafe { (*pH).keyClass }) as i32);
    elem = unsafe { (*pH).first };
    unsafe {
        (*pH).first = std::ptr::null_mut::<Fts3HashElem>();
    }
    '__slate_break_156: while elem != std::ptr::null_mut::<Fts3HashElem>() {
        let mut h: i32 = (unsafe {
            xHash.unwrap()((unsafe { (*elem).pKey }) as *const (), unsafe {
                (*elem).nKey
            })
        }) & new_size - (1 as i32);
        next_elem = unsafe { (*elem).next };
        fts3HashInsertElement(pH, unsafe { new_ht.offset(h as isize) }, elem);
        elem = next_elem;
    }
    return 0 as i32;
}

/// This function (for internal use only) locates an element in an
/// hash table that matches the given key.  The hash for this key has
/// already been computed and is passed as the 4th parameter.
///
/// # Arguments
///
/// * `pH` - The pH to be searched
/// * `pKey` - The key we are searching for
/// * `h` - The hash for this key.
fn fts3FindElementByHash(
    mut pH: *const Fts3Hash,
    mut pKey: *const (),
    mut nKey: i32,
    mut h: i32,
) -> *mut Fts3HashElem {
    let mut elem: *mut Fts3HashElem = unsafe { std::mem::zeroed() }; // Used to loop thru the element list
    let mut count: i32 = 0 as i32; // Number of elements left to test
    let mut xCompare: Option<unsafe extern "C-unwind" fn(*const (), i32, *const (), i32) -> i32> =
        unsafe { std::mem::zeroed() }; // comparison function
    if (unsafe { (*pH).ht }) != std::ptr::null_mut::<_fts3ht>() {
        let mut pEntry: *mut _fts3ht = unsafe { unsafe { (*pH).ht }.offset(h as isize) };
        elem = unsafe { (*pEntry).chain };
        count = unsafe { (*pEntry).count };
        xCompare = ftsCompareFunction((unsafe { (*pH).keyClass }) as i32);
        '__slate_break_157: loop {
            let __v175: i32 = count;
            let __v176: i32 = __v175 - (1 as i32);
            count = __v176;
            if !(__v175 != (0 as i32) && elem != std::ptr::null_mut::<Fts3HashElem>()) {
                break;
            }
            if (unsafe {
                xCompare.unwrap()(
                    (unsafe { (*elem).pKey }) as *const (),
                    unsafe { (*elem).nKey },
                    pKey,
                    nKey,
                )
            }) == (0 as i32)
            {
                return elem;
            }
            elem = unsafe { (*elem).next };
        }
    }
    return std::ptr::null_mut::<Fts3HashElem>();
}

/// Remove a single entry from the hash table given a pointer to that
/// element and a hash on the element's key.
///
/// # Arguments
///
/// * `pH` - The pH containing "elem"
/// * `elem` - The element to be removed from the pH
/// * `h` - Hash value for the element
fn fts3RemoveElementByHash(mut pH: *mut Fts3Hash, mut elem: *mut Fts3HashElem, mut h: i32) {
    let mut pEntry: *mut _fts3ht = unsafe { std::mem::zeroed() };
    if (unsafe { (*elem).prev }) != std::ptr::null_mut::<Fts3HashElem>() {
        unsafe {
            (*unsafe { (*elem).prev }).next = unsafe { (*elem).next };
        }
    } else {
        unsafe {
            (*pH).first = unsafe { (*elem).next };
        }
    }
    if (unsafe { (*elem).next }) != std::ptr::null_mut::<Fts3HashElem>() {
        unsafe {
            (*unsafe { (*elem).next }).prev = unsafe { (*elem).prev };
        }
    }
    pEntry = unsafe { unsafe { (*pH).ht }.offset(h as isize) };
    if (unsafe { (*pEntry).chain }) == elem {
        unsafe {
            (*pEntry).chain = unsafe { (*elem).next };
        }
    }
    let __v177: *mut _fts3ht = pEntry;
    let __v178: i32 = unsafe { (*__v177).count };
    let __v179: i32 = __v178 - (1 as i32);
    unsafe {
        (*__v177).count = __v179;
    }
    if (unsafe { (*pEntry).count }) <= (0 as i32) {
        unsafe {
            (*pEntry).chain = std::ptr::null_mut::<Fts3HashElem>();
        }
    }
    if (unsafe { (*pH).copyKey }) != (0 as i8)
        && (unsafe { (*elem).pKey }) != std::ptr::null_mut::<()>()
    {
        fts3HashFree(unsafe { (*elem).pKey });
    }
    fts3HashFree(elem as *mut ());
    let __v180: *mut Fts3Hash = pH;
    let __v181: i32 = unsafe { (*__v180).count };
    let __v182: i32 = __v181 - (1 as i32);
    unsafe {
        (*__v180).count = __v182;
    }
    if (unsafe { (*pH).count }) <= (0 as i32) {
        0 as i32;
        0 as i32;
        sqlite3Fts3HashClear(pH);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3HashFindElem(
    mut pH: *const Fts3Hash,
    mut pKey: *const (),
    mut nKey: i32,
) -> *mut Fts3HashElem {
    let mut h: i32 = 0 as i32; // A hash on key
    let mut xHash: Option<unsafe extern "C-unwind" fn(*const (), i32) -> i32> =
        unsafe { std::mem::zeroed() }; // The hash function
    if pH == std::ptr::null::<Fts3Hash>()
        || (unsafe { (*pH).ht }) == std::ptr::null_mut::<_fts3ht>()
    {
        return std::ptr::null_mut::<Fts3HashElem>();
    }
    xHash = ftsHashFunction((unsafe { (*pH).keyClass }) as i32);
    0 as i32;
    h = unsafe { xHash.unwrap()(pKey, nKey) };
    0 as i32;
    return fts3FindElementByHash(pH, pKey, nKey, h & (unsafe { (*pH).htsize }) - (1 as i32));
}

/// Attempt to locate an element of the hash table pH with a key
/// that matches pKey,nKey.  Return the data for this element if it is
/// found, or NULL if there is no match.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3HashFind(
    mut pH: *const Fts3Hash,
    mut pKey: *const (),
    mut nKey: i32,
) -> *mut () {
    let mut pElem: *mut Fts3HashElem = unsafe { std::mem::zeroed() }; // The element that matches key (if any)
    pElem = sqlite3Fts3HashFindElem(pH, pKey, nKey);
    return if pElem != std::ptr::null_mut::<Fts3HashElem>() {
        unsafe { (*pElem).data }
    } else {
        std::ptr::null_mut::<()>()
    };
}

/// Insert an element into the hash table pH.  The key is pKey,nKey
/// and the data is "data".
///
/// If no element exists with a matching key, then a new
/// element is created.  A copy of the key is made if the copyKey
/// flag is set.  NULL is returned.
///
/// If another element already exists with the same key, then the
/// new data replaces the old data and the old data is returned.
/// The key is not copied in this instance.  If a malloc fails, then
/// the new data is returned and the hash table is unchanged.
///
/// If the "data" parameter to this function is NULL, then the
/// element corresponding to "key" is removed from the hash table.
///
/// # Arguments
///
/// * `pH` - The hash table to insert into
/// * `pKey` - The key
/// * `nKey` - Number of bytes in the key
/// * `data` - The data
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3HashInsert(
    mut pH: *mut Fts3Hash,
    mut pKey: *const (),
    mut nKey: i32,
    mut data: *mut (),
) -> *mut () {
    let mut hraw: i32 = 0 as i32; // Raw hash value of the key
    let mut h: i32 = 0 as i32; // the hash of the key modulo hash table size
    let mut elem: *mut Fts3HashElem = unsafe { std::mem::zeroed() }; // Used to loop thru the element list
    let mut new_elem: *mut Fts3HashElem = unsafe { std::mem::zeroed() }; // New element added to the pH
    let mut xHash: Option<unsafe extern "C-unwind" fn(*const (), i32) -> i32> =
        unsafe { std::mem::zeroed() }; // The hash function
    0 as i32;
    xHash = ftsHashFunction((unsafe { (*pH).keyClass }) as i32);
    0 as i32;
    hraw = unsafe { xHash.unwrap()(pKey, nKey) };
    0 as i32;
    h = hraw & (unsafe { (*pH).htsize }) - (1 as i32);
    elem = fts3FindElementByHash(pH as *const Fts3Hash, pKey, nKey, h);
    if elem != std::ptr::null_mut::<Fts3HashElem>() {
        let mut old_data: *mut () = unsafe { (*elem).data };
        if data == std::ptr::null_mut::<()>() {
            fts3RemoveElementByHash(pH, elem, h);
        } else {
            unsafe {
                (*elem).data = data;
            }
        }
        return old_data;
    }
    if data == std::ptr::null_mut::<()>() {
        return std::ptr::null_mut::<()>();
    }
    let __v158: bool;
    if (unsafe { (*pH).htsize }) == (0 as i32) {
        __v158 = fts3Rehash(pH, 8 as i32) != (0 as i32);
    } else {
        __v158 = false as bool;
    }
    let __v159: bool;
    if __v158 {
        __v159 = true as bool;
    } else {
        let __v160: bool;
        if (unsafe { (*pH).count }) >= unsafe { (*pH).htsize } {
            __v160 = fts3Rehash(pH, (unsafe { (*pH).htsize }) * (2 as i32)) != (0 as i32);
        } else {
            __v160 = false as bool;
        }
        __v159 = __v160;
    }
    if __v159 {
        unsafe {
            (*pH).count = 0 as i32;
        }
        return data;
    }
    0 as i32;
    new_elem = fts3HashMalloc((40 as u64) as i64) as *mut Fts3HashElem;
    if new_elem == std::ptr::null_mut::<Fts3HashElem>() {
        return data;
    }
    if (unsafe { (*pH).copyKey }) != (0 as i8) && pKey != std::ptr::null::<()>() {
        unsafe {
            (*new_elem).pKey = fts3HashMalloc(nKey as i64);
        }
        if (unsafe { (*new_elem).pKey }) == std::ptr::null_mut::<()>() {
            fts3HashFree(new_elem as *mut ());
            return data;
        }
        unsafe { memcpy(unsafe { (*new_elem).pKey }, pKey, (nKey as i64) as u64) };
    } else {
        unsafe {
            (*new_elem).pKey = pKey as *mut ();
        }
    }
    unsafe {
        (*new_elem).nKey = nKey;
    }
    let __v161: *mut Fts3Hash = pH;
    let __v162: i32 = unsafe { (*__v161).count };
    let __v163: i32 = __v162 + (1 as i32);
    unsafe {
        (*__v161).count = __v163;
    }
    0 as i32;
    0 as i32;
    h = hraw & (unsafe { (*pH).htsize }) - (1 as i32);
    fts3HashInsertElement(
        pH,
        unsafe { unsafe { (*pH).ht }.offset(h as isize) },
        new_elem,
    );
    unsafe {
        (*new_elem).data = data;
    }
    return std::ptr::null_mut::<()>();
}
