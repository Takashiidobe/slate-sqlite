//! 2012-05-25
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
#[repr(C)]
#[derive(Clone, Copy)]
struct TableEntry {
    iCode: u16,
    flags: u8,
    nRange: u8,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

// DO NOT EDIT THIS MACHINE GENERATED FILE.
/// Return true if the argument corresponds to a unicode codepoint
/// classified as either a letter or a number. Otherwise false.
///
/// The results are undefined if the value passed to this function
/// is less than zero.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FtsUnicodeIsalnum(mut c: i32) -> i32 {
    // Each unsigned integer in the following array corresponds to a contiguous
    // range of unicode codepoints that are not either letters or numbers (i.e.
    // codepoints for which this function should return 0).
    //
    // The most significant 22 bits in each 32-bit value contain the first
    // codepoint in the range. The least significant 10 bits are used to store
    // the size of the range (always at least 1). In other words, the value
    // ((C<<22) + N) represents a range of N codepoints starting with codepoint
    // C. It is not possible to represent a range larger than 1023 codepoints
    // using this format.
    if (c as u32) < ((128 as i32) as u32) {
        return ((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(aAscii.0) as *const u32 }
                    .offset((c >> (5 as i32)) as isize)
            }
        }) & ((1 as i32) as u32) << (c & (31 as i32))
            == ((0 as i32) as u32)) as i32;
    } else {
        if (c as u32) < (((1 as i32) << (22 as i32)) as u32) {
            let mut key: u32 = (c as u32) << (10 as i32) | ((1023 as i32) as u32);
            let mut iRes: i32 = 0 as i32;
            let mut iHi: i32 = (((1624 as u64) / (4 as u64))
                .wrapping_sub(((1 as i32) as i64) as u64) as u32)
                as i32;
            let mut iLo: i32 = 0 as i32;
            '__slate_break_36: while iHi >= iLo {
                let mut iTest: i32 = (iHi + iLo) / (2 as i32);
                if key
                    >= unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(aEntry_2.0) as *const u32 }
                                .offset(iTest as isize)
                        }
                    }
                {
                    iRes = iTest;
                    iLo = iTest + (1 as i32);
                } else {
                    iHi = iTest - (1 as i32);
                }
            }
            0 as i32;
            0 as i32;
            return ((c as u32)
                >= ((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(aEntry_2.0) as *const u32 }
                            .offset(iRes as isize)
                    }
                }) >> (10 as i32))
                    .wrapping_add(
                        (unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(aEntry_2.0) as *const u32 }
                                    .offset(iRes as isize)
                            }
                        }) & ((1023 as i32) as u32),
                    )) as i32;
        }
    }
    return 1 as i32;
}

static mut aEntry_2: __SlateAlign16<[u32; 406]> = __SlateAlign16([
    (48 as i32) as u32,
    (59399 as i32) as u32,
    (93190 as i32) as u32,
    (125999 as i32) as u32,
    (175111 as i32) as u32,
    (184321 as i32) as u32,
    (186371 as i32) as u32,
    (191489 as i32) as u32,
    (195585 as i32) as u32,
    (220161 as i32) as u32,
    (252929 as i32) as u32,
    (722948 as i32) as u32,
    (739342 as i32) as u32,
    (758791 as i32) as u32,
    (766977 as i32) as u32,
    (769153 as i32) as u32,
    (906241 as i32) as u32,
    (915457 as i32) as u32,
    (921602 as i32) as u32,
    (924673 as i32) as u32,
    (1038337 as i32) as u32,
    (1181704 as i32) as u32,
    (1402886 as i32) as u32,
    (1451010 as i32) as u32,
    (1457153 as i32) as u32,
    (1459255 as i32) as u32,
    (1559554 as i32) as u32,
    (1572869 as i32) as u32,
    (1579030 as i32) as u32,
    (1603586 as i32) as u32,
    (1649685 as i32) as u32,
    (1681412 as i32) as u32,
    (1687553 as i32) as u32,
    (1789953 as i32) as u32,
    (1792015 as i32) as u32,
    (1809415 as i32) as u32,
    (1831938 as i32) as u32,
    (1835022 as i32) as u32,
    (1850369 as i32) as u32,
    (1852417 as i32) as u32,
    (1884187 as i32) as u32,
    (2005003 as i32) as u32,
    (2075657 as i32) as u32,
    (2086916 as i32) as u32,
    (2119684 as i32) as u32,
    (2124809 as i32) as u32,
    (2135043 as i32) as u32,
    (2139141 as i32) as u32,
    (2146319 as i32) as u32,
    (2188291 as i32) as u32,
    (2193409 as i32) as u32,
    (2330651 as i32) as u32,
    (2359300 as i32) as u32,
    (2418691 as i32) as u32,
    (2422802 as i32) as u32,
    (2442247 as i32) as u32,
    (2459652 as i32) as u32,
    (2473985 as i32) as u32,
    (2491395 as i32) as u32,
    (2551809 as i32) as u32,
    (2553863 as i32) as u32,
    (2563074 as i32) as u32,
    (2567171 as i32) as u32,
    (2579457 as i32) as u32,
    (2590722 as i32) as u32,
    (2607106 as i32) as u32,
    (2615298 as i32) as u32,
    (2622467 as i32) as u32,
    (2682881 as i32) as u32,
    (2684933 as i32) as u32,
    (2694146 as i32) as u32,
    (2698243 as i32) as u32,
    (2704385 as i32) as u32,
    (2736130 as i32) as u32,
    (2741249 as i32) as u32,
    (2753539 as i32) as u32,
    (2813953 as i32) as u32,
    (2816008 as i32) as u32,
    (2825219 as i32) as u32,
    (2829315 as i32) as u32,
    (2852866 as i32) as u32,
    (2867202 as i32) as u32,
    (2884611 as i32) as u32,
    (2945025 as i32) as u32,
    (2947079 as i32) as u32,
    (2956290 as i32) as u32,
    (2960387 as i32) as u32,
    (2971650 as i32) as u32,
    (2983938 as i32) as u32,
    (2998273 as i32) as u32,
    (3016705 as i32) as u32,
    (3078149 as i32) as u32,
    (3086339 as i32) as u32,
    (3090436 as i32) as u32,
    (3103745 as i32) as u32,
    (3132424 as i32) as u32,
    (3146755 as i32) as u32,
    (3209223 as i32) as u32,
    (3217411 as i32) as u32,
    (3221508 as i32) as u32,
    (3232770 as i32) as u32,
    (3246082 as i32) as u32,
    (3275777 as i32) as u32,
    (3278850 as i32) as u32,
    (3338241 as i32) as u32,
    (3340295 as i32) as u32,
    (3348483 as i32) as u32,
    (3352580 as i32) as u32,
    (3363842 as i32) as u32,
    (3377154 as i32) as u32,
    (3409922 as i32) as u32,
    (3471367 as i32) as u32,
    (3479555 as i32) as u32,
    (3483652 as i32) as u32,
    (3496961 as i32) as u32,
    (3508226 as i32) as u32,
    (3531777 as i32) as u32,
    (3540994 as i32) as u32,
    (3614721 as i32) as u32,
    (3619846 as i32) as u32,
    (3627009 as i32) as u32,
    (3629064 as i32) as u32,
    (3655683 as i32) as u32,
    (3720193 as i32) as u32,
    (3723271 as i32) as u32,
    (3734529 as i32) as u32,
    (3742729 as i32) as u32,
    (3762178 as i32) as u32,
    (3851265 as i32) as u32,
    (3854342 as i32) as u32,
    (3861506 as i32) as u32,
    (3874822 as i32) as u32,
    (3933215 as i32) as u32,
    (3985420 as i32) as u32,
    (4047895 as i32) as u32,
    (4076555 as i32) as u32,
    (4088868 as i32) as u32,
    (4126735 as i32) as u32,
    (4143117 as i32) as u32,
    (4238356 as i32) as u32,
    (4270086 as i32) as u32,
    (4282372 as i32) as u32,
    (4290563 as i32) as u32,
    (4294659 as i32) as u32,
    (4299783 as i32) as u32,
    (4310020 as i32) as u32,
    (4327436 as i32) as u32,
    (4340737 as i32) as u32,
    (4352006 as i32) as u32,
    (4451329 as i32) as u32,
    (5075980 as i32) as u32,
    (5128202 as i32) as u32,
    (5242881 as i32) as u32,
    (5878786 as i32) as u32,
    (5898241 as i32) as u32,
    (5925890 as i32) as u32,
    (6007811 as i32) as u32,
    (6047747 as i32) as u32,
    (6080517 as i32) as u32,
    (6113282 as i32) as u32,
    (6146050 as i32) as u32,
    (6213667 as i32) as u32,
    (6250500 as i32) as u32,
    (6255617 as i32) as u32,
    (6291471 as i32) as u32,
    (6464513 as i32) as u32,
    (6586380 as i32) as u32,
    (6602764 as i32) as u32,
    (6619137 as i32) as u32,
    (6623234 as i32) as u32,
    (6733841 as i32) as u32,
    (6758402 as i32) as u32,
    (6780962 as i32) as u32,
    (6839301 as i32) as u32,
    (6846466 as i32) as u32,
    (6902794 as i32) as u32,
    (6914077 as i32) as u32,
    (6945793 as i32) as u32,
    (6979591 as i32) as u32,
    (6987782 as i32) as u32,
    (7077893 as i32) as u32,
    (7131153 as i32) as u32,
    (7170083 as i32) as u32,
    (7208963 as i32) as u32,
    (7242765 as i32) as u32,
    (7313422 as i32) as u32,
    (7335940 as i32) as u32,
    (7376916 as i32) as u32,
    (7400453 as i32) as u32,
    (7469058 as i32) as u32,
    (7536648 as i32) as u32,
    (7553049 as i32) as u32,
    (7582721 as i32) as u32,
    (7587843 as i32) as u32,
    (7798823 as i32) as u32,
    (7860228 as i32) as u32,
    (8320001 as i32) as u32,
    (8322051 as i32) as u32,
    (8336387 as i32) as u32,
    (8352771 as i32) as u32,
    (8369155 as i32) as u32,
    (8385538 as i32) as u32,
    (8388709 as i32) as u32,
    (8497158 as i32) as u32,
    (8513541 as i32) as u32,
    (8529925 as i32) as u32,
    (8552474 as i32) as u32,
    (8601633 as i32) as u32,
    (8650754 as i32) as u32,
    (8653828 as i32) as u32,
    (8658946 as i32) as u32,
    (8671233 as i32) as u32,
    (8673283 as i32) as u32,
    (8681478 as i32) as u32,
    (8688641 as i32) as u32,
    (8690689 as i32) as u32,
    (8692737 as i32) as u32,
    (8697857 as i32) as u32,
    (8710146 as i32) as u32,
    (8716293 as i32) as u32,
    (8726532 as i32) as u32,
    (8731649 as i32) as u32,
    (8798820 as i32) as u32,
    (9437223 as i32) as u32,
    (9502731 as i32) as u32,
    (9597006 as i32) as u32,
    (9699840 as i32) as u32,
    (10224757 as i32) as u32,
    (10376121 as i32) as u32,
    (11354122 as i32) as u32,
    (11768838 as i32) as u32,
    (11779075 as i32) as u32,
    (11789316 as i32) as u32,
    (11794434 as i32) as u32,
    (11911169 as i32) as u32,
    (11926529 as i32) as u32,
    (12025935 as i32) as u32,
    (12107788 as i32) as u32,
    (12189722 as i32) as u32,
    (12217433 as i32) as u32,
    (12320982 as i32) as u32,
    (12566540 as i32) as u32,
    (12582917 as i32) as u32,
    (12591129 as i32) as u32,
    (12625927 as i32) as u32,
    (12638210 as i32) as u32,
    (12645379 as i32) as u32,
    (12739588 as i32) as u32,
    (12746753 as i32) as u32,
    (12839937 as i32) as u32,
    (12992514 as i32) as u32,
    (12998666 as i32) as u32,
    (13041700 as i32) as u32,
    (13107231 as i32) as u32,
    (13150238 as i32) as u32,
    (13189121 as i32) as u32,
    (13205536 as i32) as u32,
    (13248551 as i32) as u32,
    (13303871 as i32) as u32,
    (13369600 as i32) as u32,
    (20381760 as i32) as u32,
    (43139127 as i32) as u32,
    (43251714 as i32) as u32,
    (43529219 as i32) as u32,
    (43629584 as i32) as u32,
    (43678721 as i32) as u32,
    (43761672 as i32) as u32,
    (43778071 as i32) as u32,
    (43810818 as i32) as u32,
    (43918338 as i32) as u32,
    (44042241 as i32) as u32,
    (44046337 as i32) as u32,
    (44051457 as i32) as u32,
    (44076041 as i32) as u32,
    (44095492 as i32) as u32,
    (44158980 as i32) as u32,
    (44171266 as i32) as u32,
    (44224529 as i32) as u32,
    (44251138 as i32) as u32,
    (44269586 as i32) as u32,
    (44294147 as i32) as u32,
    (44341258 as i32) as u32,
    (44375053 as i32) as u32,
    (44399617 as i32) as u32,
    (44433412 as i32) as u32,
    (44485659 as i32) as u32,
    (44529666 as i32) as u32,
    (44606478 as i32) as u32,
    (44633089 as i32) as u32,
    (44642306 as i32) as u32,
    (44658692 as i32) as u32,
    (44686339 as i32) as u32,
    (44690433 as i32) as u32,
    (44744705 as i32) as u32,
    (44746755 as i32) as u32,
    (44751874 as i32) as u32,
    (44759042 as i32) as u32,
    (44762113 as i32) as u32,
    (44791810 as i32) as u32,
    (44805127 as i32) as u32,
    (44815362 as i32) as u32,
    (45059083 as i32) as u32,
    (56623105 as i32) as u32,
    (57539586 as i32) as u32,
    (57670658 as i32) as u32,
    (58719233 as i32) as u32,
    (65828865 as i32) as u32,
    (65840129 as i32) as u32,
    (65980432 as i32) as u32,
    (66385922 as i32) as u32,
    (66580482 as i32) as u32,
    (66584602 as i32) as u32,
    (66617351 as i32) as u32,
    (66633763 as i32) as u32,
    (66670611 as i32) as u32,
    (66691076 as i32) as u32,
    (66845697 as i32) as u32,
    (66847759 as i32) as u32,
    (66873351 as i32) as u32,
    (66907142 as i32) as u32,
    (66939915 as i32) as u32,
    (67076103 as i32) as u32,
    (67084295 as i32) as u32,
    (67101701 as i32) as u32,
    (67371011 as i32) as u32,
    (67427337 as i32) as u32,
    (67494929 as i32) as u32,
    (67518476 as i32) as u32,
    (67584046 as i32) as u32,
    (68058113 as i32) as u32,
    (68108289 as i32) as u32,
    (69295105 as i32) as u32,
    (69499905 as i32) as u32,
    (69532673 as i32) as u32,
    (69731331 as i32) as u32,
    (69735426 as i32) as u32,
    (69742596 as i32) as u32,
    (69787651 as i32) as u32,
    (69794817 as i32) as u32,
    (69812233 as i32) as u32,
    (69860353 as i32) as u32,
    (70050823 as i32) as u32,
    (71303171 as i32) as u32,
    (71360534 as i32) as u32,
    (71434243 as i32) as u32,
    (71483410 as i32) as u32,
    (71565315 as i32) as u32,
    (71605262 as i32) as u32,
    (71630852 as i32) as u32,
    (71696387 as i32) as u32,
    (71748622 as i32) as u32,
    (71767044 as i32) as u32,
    (73051149 as i32) as u32,
    (76660740 as i32) as u32,
    (96289838 as i32) as u32,
    (96353284 as i32) as u32,
    (121635062 as i32) as u32,
    (121896999 as i32) as u32,
    (121939125 as i32) as u32,
    (122159174 as i32) as u32,
    (122421335 as i32) as u32,
    (123405313 as i32) as u32,
    (123431937 as i32) as u32,
    (123464705 as i32) as u32,
    (123491329 as i32) as u32,
    (123524097 as i32) as u32,
    (123550721 as i32) as u32,
    (123583489 as i32) as u32,
    (123610113 as i32) as u32,
    (123642881 as i32) as u32,
    (123669505 as i32) as u32,
    (129744898 as i32) as u32,
    (130023468 as i32) as u32,
    (130072676 as i32) as u32,
    (130187279 as i32) as u32,
    (130204686 as i32) as u32,
    (130221071 as i32) as u32,
    (130237455 as i32) as u32,
    (130301983 as i32) as u32,
    (130334780 as i32) as u32,
    (130400299 as i32) as u32,
    (130521117 as i32) as u32,
    (130564139 as i32) as u32,
    (130613257 as i32) as u32,
    (130629634 as i32) as u32,
    (130809889 as i32) as u32,
    (130859014 as i32) as u32,
    (130866246 as i32) as u32,
    (130940948 as i32) as u32,
    (130973733 as i32) as u32,
    (131012613 as i32) as u32,
    (131039249 as i32) as u32,
    (131072063 as i32) as u32,
    (131137537 as i32) as u32,
    (131139766 as i32) as u32,
    (131326980 as i32) as u32,
    (131334206 as i32) as u32,
    (131399684 as i32) as u32,
    (131416088 as i32) as u32,
    (131591238 as i32) as u32,
    (131666955 as i32) as u32,
    (131727430 as i32) as u32,
    (131858548 as i32) as u32,
    (939525121 as i32) as u32,
    (939556960 as i32) as u32,
    (939786480 as i32) as u32,
]);

static mut aAscii: __SlateAlign16<[u32; 4]> = __SlateAlign16([
    4294967295 as u32,
    4227923967 as u32,
    4160749569 as u32,
    4160749569 as u32,
]);

/// If the argument is a codepoint corresponding to a lowercase letter
/// in the ASCII range with a diacritic added, return the codepoint
/// of the ASCII letter only. For example, if passed 235 - "LATIN
/// SMALL LETTER E WITH DIAERESIS" - return 65 ("LATIN SMALL LETTER
/// E"). The resuls of passing a codepoint that corresponds to an
/// uppercase letter are undefined.
fn remove_diacritic(mut c: i32, mut bComplex: i32) -> i32 {
    let mut aDia: __SlateAlign16<[u16; 126]> = __SlateAlign16([
        ((0 as i32) as i16) as u16,
        ((1797 as i32) as i16) as u16,
        ((1848 as i32) as i16) as u16,
        ((1859 as i32) as i16) as u16,
        ((1891 as i32) as i16) as u16,
        ((1928 as i32) as i16) as u16,
        ((1940 as i32) as i16) as u16,
        ((1995 as i32) as i16) as u16,
        ((2024 as i32) as i16) as u16,
        ((2040 as i32) as i16) as u16,
        ((2060 as i32) as i16) as u16,
        ((2110 as i32) as i16) as u16,
        ((2168 as i32) as i16) as u16,
        ((2206 as i32) as i16) as u16,
        ((2264 as i32) as i16) as u16,
        ((2286 as i32) as i16) as u16,
        ((2344 as i32) as i16) as u16,
        ((2383 as i32) as i16) as u16,
        ((2472 as i32) as i16) as u16,
        ((2488 as i32) as i16) as u16,
        ((2516 as i32) as i16) as u16,
        ((2596 as i32) as i16) as u16,
        ((2668 as i32) as i16) as u16,
        ((2732 as i32) as i16) as u16,
        ((2782 as i32) as i16) as u16,
        ((2842 as i32) as i16) as u16,
        ((2894 as i32) as i16) as u16,
        ((2954 as i32) as i16) as u16,
        ((2984 as i32) as i16) as u16,
        ((3000 as i32) as i16) as u16,
        ((3028 as i32) as i16) as u16,
        ((3336 as i32) as i16) as u16,
        ((3456 as i32) as i16) as u16,
        ((3696 as i32) as i16) as u16,
        ((3712 as i32) as i16) as u16,
        ((3728 as i32) as i16) as u16,
        ((3744 as i32) as i16) as u16,
        ((3766 as i32) as i16) as u16,
        ((3832 as i32) as i16) as u16,
        ((3896 as i32) as i16) as u16,
        ((3912 as i32) as i16) as u16,
        ((3928 as i32) as i16) as u16,
        ((3944 as i32) as i16) as u16,
        ((3968 as i32) as i16) as u16,
        ((4008 as i32) as i16) as u16,
        ((4040 as i32) as i16) as u16,
        ((4056 as i32) as i16) as u16,
        ((4106 as i32) as i16) as u16,
        ((4138 as i32) as i16) as u16,
        ((4170 as i32) as i16) as u16,
        ((4202 as i32) as i16) as u16,
        ((4234 as i32) as i16) as u16,
        ((4266 as i32) as i16) as u16,
        ((4296 as i32) as i16) as u16,
        ((4312 as i32) as i16) as u16,
        ((4344 as i32) as i16) as u16,
        ((4408 as i32) as i16) as u16,
        ((4424 as i32) as i16) as u16,
        ((4442 as i32) as i16) as u16,
        ((4472 as i32) as i16) as u16,
        ((4488 as i32) as i16) as u16,
        ((4504 as i32) as i16) as u16,
        ((6148 as i32) as i16) as u16,
        ((6198 as i32) as i16) as u16,
        ((6264 as i32) as i16) as u16,
        ((6280 as i32) as i16) as u16,
        ((6360 as i32) as i16) as u16,
        ((6429 as i32) as i16) as u16,
        ((6505 as i32) as i16) as u16,
        ((6529 as i32) as i16) as u16,
        ((61448 as i32) as i16) as u16,
        ((61468 as i32) as i16) as u16,
        ((61512 as i32) as i16) as u16,
        ((61534 as i32) as i16) as u16,
        ((61592 as i32) as i16) as u16,
        ((61610 as i32) as i16) as u16,
        ((61642 as i32) as i16) as u16,
        ((61672 as i32) as i16) as u16,
        ((61688 as i32) as i16) as u16,
        ((61704 as i32) as i16) as u16,
        ((61726 as i32) as i16) as u16,
        ((61784 as i32) as i16) as u16,
        ((61800 as i32) as i16) as u16,
        ((61816 as i32) as i16) as u16,
        ((61836 as i32) as i16) as u16,
        ((61880 as i32) as i16) as u16,
        ((61896 as i32) as i16) as u16,
        ((61914 as i32) as i16) as u16,
        ((61948 as i32) as i16) as u16,
        ((61998 as i32) as i16) as u16,
        ((62062 as i32) as i16) as u16,
        ((62122 as i32) as i16) as u16,
        ((62154 as i32) as i16) as u16,
        ((62184 as i32) as i16) as u16,
        ((62200 as i32) as i16) as u16,
        ((62218 as i32) as i16) as u16,
        ((62252 as i32) as i16) as u16,
        ((62302 as i32) as i16) as u16,
        ((62364 as i32) as i16) as u16,
        ((62410 as i32) as i16) as u16,
        ((62442 as i32) as i16) as u16,
        ((62478 as i32) as i16) as u16,
        ((62536 as i32) as i16) as u16,
        ((62554 as i32) as i16) as u16,
        ((62584 as i32) as i16) as u16,
        ((62604 as i32) as i16) as u16,
        ((62640 as i32) as i16) as u16,
        ((62648 as i32) as i16) as u16,
        ((62656 as i32) as i16) as u16,
        ((62664 as i32) as i16) as u16,
        ((62730 as i32) as i16) as u16,
        ((62766 as i32) as i16) as u16,
        ((62830 as i32) as i16) as u16,
        ((62890 as i32) as i16) as u16,
        ((62924 as i32) as i16) as u16,
        ((62974 as i32) as i16) as u16,
        ((63032 as i32) as i16) as u16,
        ((63050 as i32) as i16) as u16,
        ((63082 as i32) as i16) as u16,
        ((63118 as i32) as i16) as u16,
        ((63182 as i32) as i16) as u16,
        ((63242 as i32) as i16) as u16,
        ((63274 as i32) as i16) as u16,
        ((63310 as i32) as i16) as u16,
        ((63368 as i32) as i16) as u16,
        ((63390 as i32) as i16) as u16,
    ]);
    let mut aChar: __SlateAlign16<[u8; 126]> = __SlateAlign16([
        ((0 as i32) as i8) as u8,
        ((97 as i32) as i8) as u8,
        ((99 as i32) as i8) as u8,
        ((101 as i32) as i8) as u8,
        ((105 as i32) as i8) as u8,
        ((110 as i32) as i8) as u8,
        ((111 as i32) as i8) as u8,
        ((117 as i32) as i8) as u8,
        ((121 as i32) as i8) as u8,
        ((121 as i32) as i8) as u8,
        ((97 as i32) as i8) as u8,
        ((99 as i32) as i8) as u8,
        ((100 as i32) as i8) as u8,
        ((101 as i32) as i8) as u8,
        ((101 as i32) as i8) as u8,
        ((103 as i32) as i8) as u8,
        ((104 as i32) as i8) as u8,
        ((105 as i32) as i8) as u8,
        ((106 as i32) as i8) as u8,
        ((107 as i32) as i8) as u8,
        ((108 as i32) as i8) as u8,
        ((110 as i32) as i8) as u8,
        ((111 as i32) as i8) as u8,
        ((114 as i32) as i8) as u8,
        ((115 as i32) as i8) as u8,
        ((116 as i32) as i8) as u8,
        ((117 as i32) as i8) as u8,
        ((117 as i32) as i8) as u8,
        ((119 as i32) as i8) as u8,
        ((121 as i32) as i8) as u8,
        ((122 as i32) as i8) as u8,
        ((111 as i32) as i8) as u8,
        ((117 as i32) as i8) as u8,
        ((97 as i32) as i8) as u8,
        ((105 as i32) as i8) as u8,
        ((111 as i32) as i8) as u8,
        ((117 as i32) as i8) as u8,
        (((117 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        (((97 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((103 as i32) as i8) as u8,
        ((107 as i32) as i8) as u8,
        ((111 as i32) as i8) as u8,
        (((111 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((106 as i32) as i8) as u8,
        ((103 as i32) as i8) as u8,
        ((110 as i32) as i8) as u8,
        (((97 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((97 as i32) as i8) as u8,
        ((101 as i32) as i8) as u8,
        ((105 as i32) as i8) as u8,
        ((111 as i32) as i8) as u8,
        ((114 as i32) as i8) as u8,
        ((117 as i32) as i8) as u8,
        ((115 as i32) as i8) as u8,
        ((116 as i32) as i8) as u8,
        ((104 as i32) as i8) as u8,
        ((97 as i32) as i8) as u8,
        ((101 as i32) as i8) as u8,
        (((111 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((111 as i32) as i8) as u8,
        (((111 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((121 as i32) as i8) as u8,
        ((0 as i32) as i8) as u8,
        ((0 as i32) as i8) as u8,
        ((0 as i32) as i8) as u8,
        ((0 as i32) as i8) as u8,
        ((0 as i32) as i8) as u8,
        ((0 as i32) as i8) as u8,
        ((0 as i32) as i8) as u8,
        ((0 as i32) as i8) as u8,
        ((97 as i32) as i8) as u8,
        ((98 as i32) as i8) as u8,
        (((99 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((100 as i32) as i8) as u8,
        ((100 as i32) as i8) as u8,
        (((101 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((101 as i32) as i8) as u8,
        (((101 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((102 as i32) as i8) as u8,
        ((103 as i32) as i8) as u8,
        ((104 as i32) as i8) as u8,
        ((104 as i32) as i8) as u8,
        ((105 as i32) as i8) as u8,
        (((105 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((107 as i32) as i8) as u8,
        ((108 as i32) as i8) as u8,
        (((108 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((108 as i32) as i8) as u8,
        ((109 as i32) as i8) as u8,
        ((110 as i32) as i8) as u8,
        (((111 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((112 as i32) as i8) as u8,
        ((114 as i32) as i8) as u8,
        (((114 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((114 as i32) as i8) as u8,
        ((115 as i32) as i8) as u8,
        (((115 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((116 as i32) as i8) as u8,
        ((117 as i32) as i8) as u8,
        (((117 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((118 as i32) as i8) as u8,
        ((119 as i32) as i8) as u8,
        ((119 as i32) as i8) as u8,
        ((120 as i32) as i8) as u8,
        ((121 as i32) as i8) as u8,
        ((122 as i32) as i8) as u8,
        ((104 as i32) as i8) as u8,
        ((116 as i32) as i8) as u8,
        ((119 as i32) as i8) as u8,
        ((121 as i32) as i8) as u8,
        ((97 as i32) as i8) as u8,
        (((97 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        (((97 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        (((97 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((101 as i32) as i8) as u8,
        (((101 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        (((101 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((105 as i32) as i8) as u8,
        ((111 as i32) as i8) as u8,
        (((111 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        (((111 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        (((111 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((117 as i32) as i8) as u8,
        (((117 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        (((117 as i32) | (((((128 as i32) as i8) as u8) as u32) as i32)) as i8) as u8,
        ((121 as i32) as i8) as u8,
    ]);
    let mut key: u32 = (c as u32) << (3 as i32) | ((7 as i32) as u32);
    let mut iRes: i32 = 0 as i32;
    let mut iHi: i32 =
        (((252 as u64) / (2 as u64)).wrapping_sub(((1 as i32) as i64) as u64) as u32) as i32;
    let mut iLo: i32 = 0 as i32;
    '__slate_break_37: while iHi >= iLo {
        let mut iTest: i32 = (iHi + iLo) / (2 as i32);
        if key
            >= ((((unsafe { *unsafe { (aDia.0.as_mut_ptr() as *mut u16).offset(iTest as isize) } })
                as u32) as i32) as u32)
        {
            iRes = iTest;
            iLo = iTest + (1 as i32);
        } else {
            iHi = iTest - (1 as i32);
        }
    }
    0 as i32;
    if bComplex == (0 as i32)
        && (((unsafe { *unsafe { (aChar.0.as_mut_ptr() as *mut u8).offset(iRes as isize) } })
            as u32) as i32)
            & (128 as i32)
            != (0 as i32)
    {
        return c;
    }
    return if c
        > ((((unsafe { *unsafe { (aDia.0.as_mut_ptr() as *mut u16).offset(iRes as isize) } })
            as u32) as i32)
            >> (3 as i32))
            + ((((unsafe { *unsafe { (aDia.0.as_mut_ptr() as *mut u16).offset(iRes as isize) } })
                as u32) as i32)
                & (7 as i32))
    {
        c
    } else {
        (((unsafe { *unsafe { (aChar.0.as_mut_ptr() as *mut u8).offset(iRes as isize) } }) as u32)
            as i32)
            & (127 as i32)
    };
}

/// Return true if the argument interpreted as a unicode codepoint
/// is a diacritical modifier character.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FtsUnicodeIsdiacritic(mut c: i32) -> i32 {
    let mut mask0: u32 = (134389727 as i32) as u32;
    let mut mask1: u32 = (221688 as i32) as u32;
    if c < (768 as i32) || c > (817 as i32) {
        return 0 as i32;
    }
    return (if c < (768 as i32) + (32 as i32) {
        mask0 & ((1 as i32) as u32) << c - (768 as i32)
    } else {
        mask1 & ((1 as i32) as u32) << c - (768 as i32) - (32 as i32)
    }) as i32;
}

/// Interpret the argument as a unicode codepoint. If the codepoint
/// is an upper case character that has a lower case equivalent,
/// return the codepoint corresponding to the lower case version.
/// Otherwise, return a copy of the argument.
///
/// The results are undefined if the value passed to this function
/// is less than zero.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FtsUnicodeFold(mut c: i32, mut eRemoveDiacritic: i32) -> i32 {
    // Each entry in the following array defines a rule for folding a range
    // of codepoints to lower case. The rule applies to a range of nRange
    // codepoints starting at codepoint iCode.
    //
    // If the least significant bit in flags is clear, then the rule applies
    // to all nRange codepoints (i.e. all nRange codepoints are upper case and
    // need to be folded). Or, if it is set, then the rule only applies to
    // every second codepoint in the range, starting with codepoint C.
    //
    // The 7 most significant bits in flags are an index into the aiOff[]
    // array. If a specific codepoint C does require folding, then its lower
    // case equivalent is ((C + aiOff[flags>>1]) & 0xFFFF).
    //
    // The contents of this array are generated by parsing the CaseFolding.txt
    // file distributed as part of the "Unicode Character Database". See
    // http://www.unicode.org for details.
    let mut ret: i32 = c;
    0 as i32;
    if c < (128 as i32) {
        if c >= (65 as i32) && c <= (90 as i32) {
            ret = c + ((97 as i32) - (65 as i32));
        }
    } else {
        if c < (65536 as i32) {
            let mut p: *const TableEntry = unsafe { std::mem::zeroed() };
            let mut iHi: i32 = (((652 as u64) / (4 as u64)).wrapping_sub(((1 as i32) as i64) as u64)
                as u32) as i32;
            let mut iLo: i32 = 0 as i32;
            let mut iRes: i32 = -(1 as i32);
            0 as i32;
            '__slate_break_38: while iHi >= iLo {
                let mut iTest: i32 = (iHi + iLo) / (2 as i32);
                let mut cmp: i32 = c
                    - (((unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of!(aEntry_27.0) as *const TableEntry }
                                .offset(iTest as isize)
                        })
                        .iCode
                    }) as u32) as i32);
                if cmp >= (0 as i32) {
                    iRes = iTest;
                    iLo = iTest + (1 as i32);
                } else {
                    iHi = iTest - (1 as i32);
                }
            }
            0 as i32;
            p = unsafe {
                unsafe { std::ptr::addr_of!(aEntry_27.0) as *const TableEntry }
                    .offset(iRes as isize)
            };
            if c < (((unsafe { (*p).iCode }) as u32) as i32)
                + (((unsafe { (*p).nRange }) as u32) as i32)
                && (0 as i32)
                    == (1 as i32)
                        & (((unsafe { (*p).flags }) as u32) as i32)
                        & ((((unsafe { (*p).iCode }) as u32) as i32) ^ c)
            {
                ret = c
                    + (((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(aiOff.0) as *const u16 }.offset(
                                ((((unsafe { (*p).flags }) as u32) as i32) >> (1 as i32)) as isize,
                            )
                        }
                    }) as u32) as i32)
                    & (65535 as i32);
                0 as i32;
            }
            if eRemoveDiacritic != (0 as i32) {
                ret = remove_diacritic(ret, (eRemoveDiacritic == (2 as i32)) as i32);
            }
        } else {
            if c >= (66560 as i32) && c < (66600 as i32) {
                ret = c + (40 as i32);
            }
        }
    }
    return ret;
}

static mut aEntry_27: __SlateAlign16<[TableEntry; 163]> = __SlateAlign16([
    TableEntry {
        iCode: ((65 as i32) as i16) as u16,
        flags: ((14 as i32) as i8) as u8,
        nRange: ((26 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((181 as i32) as i16) as u16,
        flags: ((64 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((192 as i32) as i16) as u16,
        flags: ((14 as i32) as i8) as u8,
        nRange: ((23 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((216 as i32) as i16) as u16,
        flags: ((14 as i32) as i8) as u8,
        nRange: ((7 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((256 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((48 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((306 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((6 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((313 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((16 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((330 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((46 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((376 as i32) as i16) as u16,
        flags: ((116 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((377 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((6 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((383 as i32) as i16) as u16,
        flags: ((104 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((385 as i32) as i16) as u16,
        flags: ((50 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((386 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((4 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((390 as i32) as i16) as u16,
        flags: ((44 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((391 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((393 as i32) as i16) as u16,
        flags: ((42 as i32) as i8) as u8,
        nRange: ((2 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((395 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((398 as i32) as i16) as u16,
        flags: ((32 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((399 as i32) as i16) as u16,
        flags: ((38 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((400 as i32) as i16) as u16,
        flags: ((40 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((401 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((403 as i32) as i16) as u16,
        flags: ((42 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((404 as i32) as i16) as u16,
        flags: ((46 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((406 as i32) as i16) as u16,
        flags: ((52 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((407 as i32) as i16) as u16,
        flags: ((48 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((408 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((412 as i32) as i16) as u16,
        flags: ((52 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((413 as i32) as i16) as u16,
        flags: ((54 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((415 as i32) as i16) as u16,
        flags: ((56 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((416 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((6 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((422 as i32) as i16) as u16,
        flags: ((60 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((423 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((425 as i32) as i16) as u16,
        flags: ((60 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((428 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((430 as i32) as i16) as u16,
        flags: ((60 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((431 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((433 as i32) as i16) as u16,
        flags: ((58 as i32) as i8) as u8,
        nRange: ((2 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((435 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((4 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((439 as i32) as i16) as u16,
        flags: ((62 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((440 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((444 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((452 as i32) as i16) as u16,
        flags: ((2 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((453 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((455 as i32) as i16) as u16,
        flags: ((2 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((456 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((458 as i32) as i16) as u16,
        flags: ((2 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((459 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((18 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((478 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((18 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((497 as i32) as i16) as u16,
        flags: ((2 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((498 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((4 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((502 as i32) as i16) as u16,
        flags: ((122 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((503 as i32) as i16) as u16,
        flags: ((134 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((504 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((40 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((544 as i32) as i16) as u16,
        flags: ((110 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((546 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((18 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((570 as i32) as i16) as u16,
        flags: ((70 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((571 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((573 as i32) as i16) as u16,
        flags: ((108 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((574 as i32) as i16) as u16,
        flags: ((68 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((577 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((579 as i32) as i16) as u16,
        flags: ((106 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((580 as i32) as i16) as u16,
        flags: ((28 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((581 as i32) as i16) as u16,
        flags: ((30 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((582 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((10 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((837 as i32) as i16) as u16,
        flags: ((36 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((880 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((4 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((886 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((902 as i32) as i16) as u16,
        flags: ((18 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((904 as i32) as i16) as u16,
        flags: ((16 as i32) as i8) as u8,
        nRange: ((3 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((908 as i32) as i16) as u16,
        flags: ((26 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((910 as i32) as i16) as u16,
        flags: ((24 as i32) as i8) as u8,
        nRange: ((2 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((913 as i32) as i16) as u16,
        flags: ((14 as i32) as i8) as u8,
        nRange: ((17 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((931 as i32) as i16) as u16,
        flags: ((14 as i32) as i8) as u8,
        nRange: ((9 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((962 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((975 as i32) as i16) as u16,
        flags: ((4 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((976 as i32) as i16) as u16,
        flags: ((140 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((977 as i32) as i16) as u16,
        flags: ((142 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((981 as i32) as i16) as u16,
        flags: ((146 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((982 as i32) as i16) as u16,
        flags: ((144 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((984 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((24 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1008 as i32) as i16) as u16,
        flags: ((136 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1009 as i32) as i16) as u16,
        flags: ((138 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1012 as i32) as i16) as u16,
        flags: ((130 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1013 as i32) as i16) as u16,
        flags: ((128 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1015 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1017 as i32) as i16) as u16,
        flags: ((152 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1018 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1021 as i32) as i16) as u16,
        flags: ((110 as i32) as i8) as u8,
        nRange: ((3 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1024 as i32) as i16) as u16,
        flags: ((34 as i32) as i8) as u8,
        nRange: ((16 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1040 as i32) as i16) as u16,
        flags: ((14 as i32) as i8) as u8,
        nRange: ((32 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1120 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((34 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1162 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((54 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1216 as i32) as i16) as u16,
        flags: ((6 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1217 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((14 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1232 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((88 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((1329 as i32) as i16) as u16,
        flags: ((22 as i32) as i8) as u8,
        nRange: ((38 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((4256 as i32) as i16) as u16,
        flags: ((66 as i32) as i8) as u8,
        nRange: ((38 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((4295 as i32) as i16) as u16,
        flags: ((66 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((4301 as i32) as i16) as u16,
        flags: ((66 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((7680 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((150 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((7835 as i32) as i16) as u16,
        flags: ((132 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((7838 as i32) as i16) as u16,
        flags: ((96 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((7840 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((96 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((7944 as i32) as i16) as u16,
        flags: ((150 as i32) as i8) as u8,
        nRange: ((8 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((7960 as i32) as i16) as u16,
        flags: ((150 as i32) as i8) as u8,
        nRange: ((6 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((7976 as i32) as i16) as u16,
        flags: ((150 as i32) as i8) as u8,
        nRange: ((8 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((7992 as i32) as i16) as u16,
        flags: ((150 as i32) as i8) as u8,
        nRange: ((8 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8008 as i32) as i16) as u16,
        flags: ((150 as i32) as i8) as u8,
        nRange: ((6 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8025 as i32) as i16) as u16,
        flags: ((151 as i32) as i8) as u8,
        nRange: ((8 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8040 as i32) as i16) as u16,
        flags: ((150 as i32) as i8) as u8,
        nRange: ((8 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8072 as i32) as i16) as u16,
        flags: ((150 as i32) as i8) as u8,
        nRange: ((8 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8088 as i32) as i16) as u16,
        flags: ((150 as i32) as i8) as u8,
        nRange: ((8 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8104 as i32) as i16) as u16,
        flags: ((150 as i32) as i8) as u8,
        nRange: ((8 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8120 as i32) as i16) as u16,
        flags: ((150 as i32) as i8) as u8,
        nRange: ((2 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8122 as i32) as i16) as u16,
        flags: ((126 as i32) as i8) as u8,
        nRange: ((2 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8124 as i32) as i16) as u16,
        flags: ((148 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8126 as i32) as i16) as u16,
        flags: ((100 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8136 as i32) as i16) as u16,
        flags: ((124 as i32) as i8) as u8,
        nRange: ((4 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8140 as i32) as i16) as u16,
        flags: ((148 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8152 as i32) as i16) as u16,
        flags: ((150 as i32) as i8) as u8,
        nRange: ((2 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8154 as i32) as i16) as u16,
        flags: ((120 as i32) as i8) as u8,
        nRange: ((2 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8168 as i32) as i16) as u16,
        flags: ((150 as i32) as i8) as u8,
        nRange: ((2 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8170 as i32) as i16) as u16,
        flags: ((118 as i32) as i8) as u8,
        nRange: ((2 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8172 as i32) as i16) as u16,
        flags: ((152 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8184 as i32) as i16) as u16,
        flags: ((112 as i32) as i8) as u8,
        nRange: ((2 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8186 as i32) as i16) as u16,
        flags: ((114 as i32) as i8) as u8,
        nRange: ((2 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8188 as i32) as i16) as u16,
        flags: ((148 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8486 as i32) as i16) as u16,
        flags: ((98 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8490 as i32) as i16) as u16,
        flags: ((92 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8491 as i32) as i16) as u16,
        flags: ((94 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8498 as i32) as i16) as u16,
        flags: ((12 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8544 as i32) as i16) as u16,
        flags: ((8 as i32) as i8) as u8,
        nRange: ((16 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((8579 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((9398 as i32) as i16) as u16,
        flags: ((10 as i32) as i8) as u8,
        nRange: ((26 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11264 as i32) as i16) as u16,
        flags: ((22 as i32) as i8) as u8,
        nRange: ((47 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11360 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11362 as i32) as i16) as u16,
        flags: ((88 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11363 as i32) as i16) as u16,
        flags: ((102 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11364 as i32) as i16) as u16,
        flags: ((90 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11367 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((6 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11373 as i32) as i16) as u16,
        flags: ((84 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11374 as i32) as i16) as u16,
        flags: ((86 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11375 as i32) as i16) as u16,
        flags: ((80 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11376 as i32) as i16) as u16,
        flags: ((82 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11378 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11381 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11390 as i32) as i16) as u16,
        flags: ((78 as i32) as i8) as u8,
        nRange: ((2 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11392 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((100 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11499 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((4 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((11506 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((42560 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((46 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((42624 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((24 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((42786 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((14 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((42802 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((62 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((42873 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((4 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((42877 as i32) as i16) as u16,
        flags: ((76 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((42878 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((10 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((42891 as i32) as i16) as u16,
        flags: ((0 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((42893 as i32) as i16) as u16,
        flags: ((74 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((42896 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((4 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((42912 as i32) as i16) as u16,
        flags: ((1 as i32) as i8) as u8,
        nRange: ((10 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((42922 as i32) as i16) as u16,
        flags: ((72 as i32) as i8) as u8,
        nRange: ((1 as i32) as i8) as u8,
    },
    TableEntry {
        iCode: ((65313 as i32) as i16) as u16,
        flags: ((14 as i32) as i8) as u8,
        nRange: ((26 as i32) as i8) as u8,
    },
]);

static mut aiOff: __SlateAlign16<[u16; 77]> = __SlateAlign16([
    ((1 as i32) as i16) as u16,
    ((2 as i32) as i16) as u16,
    ((8 as i32) as i16) as u16,
    ((15 as i32) as i16) as u16,
    ((16 as i32) as i16) as u16,
    ((26 as i32) as i16) as u16,
    ((28 as i32) as i16) as u16,
    ((32 as i32) as i16) as u16,
    ((37 as i32) as i16) as u16,
    ((38 as i32) as i16) as u16,
    ((40 as i32) as i16) as u16,
    ((48 as i32) as i16) as u16,
    ((63 as i32) as i16) as u16,
    ((64 as i32) as i16) as u16,
    ((69 as i32) as i16) as u16,
    ((71 as i32) as i16) as u16,
    ((79 as i32) as i16) as u16,
    ((80 as i32) as i16) as u16,
    ((116 as i32) as i16) as u16,
    ((202 as i32) as i16) as u16,
    ((203 as i32) as i16) as u16,
    ((205 as i32) as i16) as u16,
    ((206 as i32) as i16) as u16,
    ((207 as i32) as i16) as u16,
    ((209 as i32) as i16) as u16,
    ((210 as i32) as i16) as u16,
    ((211 as i32) as i16) as u16,
    ((213 as i32) as i16) as u16,
    ((214 as i32) as i16) as u16,
    ((217 as i32) as i16) as u16,
    ((218 as i32) as i16) as u16,
    ((219 as i32) as i16) as u16,
    ((775 as i32) as i16) as u16,
    ((7264 as i32) as i16) as u16,
    ((10792 as i32) as i16) as u16,
    ((10795 as i32) as i16) as u16,
    ((23228 as i32) as i16) as u16,
    ((23256 as i32) as i16) as u16,
    ((30204 as i32) as i16) as u16,
    ((54721 as i32) as i16) as u16,
    ((54753 as i32) as i16) as u16,
    ((54754 as i32) as i16) as u16,
    ((54756 as i32) as i16) as u16,
    ((54787 as i32) as i16) as u16,
    ((54793 as i32) as i16) as u16,
    ((54809 as i32) as i16) as u16,
    ((57153 as i32) as i16) as u16,
    ((57274 as i32) as i16) as u16,
    ((57921 as i32) as i16) as u16,
    ((58019 as i32) as i16) as u16,
    ((58363 as i32) as i16) as u16,
    ((61722 as i32) as i16) as u16,
    ((65268 as i32) as i16) as u16,
    ((65341 as i32) as i16) as u16,
    ((65373 as i32) as i16) as u16,
    ((65406 as i32) as i16) as u16,
    ((65408 as i32) as i16) as u16,
    ((65410 as i32) as i16) as u16,
    ((65415 as i32) as i16) as u16,
    ((65424 as i32) as i16) as u16,
    ((65436 as i32) as i16) as u16,
    ((65439 as i32) as i16) as u16,
    ((65450 as i32) as i16) as u16,
    ((65462 as i32) as i16) as u16,
    ((65472 as i32) as i16) as u16,
    ((65476 as i32) as i16) as u16,
    ((65478 as i32) as i16) as u16,
    ((65480 as i32) as i16) as u16,
    ((65482 as i32) as i16) as u16,
    ((65488 as i32) as i16) as u16,
    ((65506 as i32) as i16) as u16,
    ((65511 as i32) as i16) as u16,
    ((65514 as i32) as i16) as u16,
    ((65521 as i32) as i16) as u16,
    ((65527 as i32) as i16) as u16,
    ((65528 as i32) as i16) as u16,
    ((65529 as i32) as i16) as u16,
]);
