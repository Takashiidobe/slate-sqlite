#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

static mut azName: __SlateAlign16<[*const i8; 192]> = __SlateAlign16([
    (b"Savepoint\0".as_ptr() as *mut i8) as *const i8,
    (b"AutoCommit\0".as_ptr() as *mut i8) as *const i8,
    (b"Transaction\0".as_ptr() as *mut i8) as *const i8,
    (b"Checkpoint\0".as_ptr() as *mut i8) as *const i8,
    (b"JournalMode\0".as_ptr() as *mut i8) as *const i8,
    (b"Vacuum\0".as_ptr() as *mut i8) as *const i8,
    (b"VFilter\0".as_ptr() as *mut i8) as *const i8,
    (b"VUpdate\0".as_ptr() as *mut i8) as *const i8,
    (b"Init\0".as_ptr() as *mut i8) as *const i8,
    (b"Goto\0".as_ptr() as *mut i8) as *const i8,
    (b"Gosub\0".as_ptr() as *mut i8) as *const i8,
    (b"InitCoroutine\0".as_ptr() as *mut i8) as *const i8,
    (b"Yield\0".as_ptr() as *mut i8) as *const i8,
    (b"MustBeInt\0".as_ptr() as *mut i8) as *const i8,
    (b"Jump\0".as_ptr() as *mut i8) as *const i8,
    (b"Once\0".as_ptr() as *mut i8) as *const i8,
    (b"If\0".as_ptr() as *mut i8) as *const i8,
    (b"IfNot\0".as_ptr() as *mut i8) as *const i8,
    (b"IsType\0".as_ptr() as *mut i8) as *const i8,
    (b"Not\0".as_ptr() as *mut i8) as *const i8,
    (b"IfNullRow\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekLT\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekLE\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekGE\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekGT\0".as_ptr() as *mut i8) as *const i8,
    (b"IfNotOpen\0".as_ptr() as *mut i8) as *const i8,
    (b"IfNoHope\0".as_ptr() as *mut i8) as *const i8,
    (b"NoConflict\0".as_ptr() as *mut i8) as *const i8,
    (b"NotFound\0".as_ptr() as *mut i8) as *const i8,
    (b"Found\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekRowid\0".as_ptr() as *mut i8) as *const i8,
    (b"NotExists\0".as_ptr() as *mut i8) as *const i8,
    (b"Last\0".as_ptr() as *mut i8) as *const i8,
    (b"IfSizeBetween\0".as_ptr() as *mut i8) as *const i8,
    (b"SorterSort\0".as_ptr() as *mut i8) as *const i8,
    (b"Sort\0".as_ptr() as *mut i8) as *const i8,
    (b"Rewind\0".as_ptr() as *mut i8) as *const i8,
    (b"IfEmpty\0".as_ptr() as *mut i8) as *const i8,
    (b"SorterNext\0".as_ptr() as *mut i8) as *const i8,
    (b"Prev\0".as_ptr() as *mut i8) as *const i8,
    (b"Next\0".as_ptr() as *mut i8) as *const i8,
    (b"IdxLE\0".as_ptr() as *mut i8) as *const i8,
    (b"IdxGT\0".as_ptr() as *mut i8) as *const i8,
    (b"Or\0".as_ptr() as *mut i8) as *const i8,
    (b"And\0".as_ptr() as *mut i8) as *const i8,
    (b"IdxLT\0".as_ptr() as *mut i8) as *const i8,
    (b"IdxGE\0".as_ptr() as *mut i8) as *const i8,
    (b"IFindKey\0".as_ptr() as *mut i8) as *const i8,
    (b"RowSetRead\0".as_ptr() as *mut i8) as *const i8,
    (b"RowSetTest\0".as_ptr() as *mut i8) as *const i8,
    (b"Program\0".as_ptr() as *mut i8) as *const i8,
    (b"IsNull\0".as_ptr() as *mut i8) as *const i8,
    (b"NotNull\0".as_ptr() as *mut i8) as *const i8,
    (b"Ne\0".as_ptr() as *mut i8) as *const i8,
    (b"Eq\0".as_ptr() as *mut i8) as *const i8,
    (b"Gt\0".as_ptr() as *mut i8) as *const i8,
    (b"Le\0".as_ptr() as *mut i8) as *const i8,
    (b"Lt\0".as_ptr() as *mut i8) as *const i8,
    (b"Ge\0".as_ptr() as *mut i8) as *const i8,
    (b"ElseEq\0".as_ptr() as *mut i8) as *const i8,
    (b"FkIfZero\0".as_ptr() as *mut i8) as *const i8,
    (b"IfPos\0".as_ptr() as *mut i8) as *const i8,
    (b"IfNotZero\0".as_ptr() as *mut i8) as *const i8,
    (b"DecrJumpZero\0".as_ptr() as *mut i8) as *const i8,
    (b"IncrVacuum\0".as_ptr() as *mut i8) as *const i8,
    (b"VNext\0".as_ptr() as *mut i8) as *const i8,
    (b"Filter\0".as_ptr() as *mut i8) as *const i8,
    (b"PureFunc\0".as_ptr() as *mut i8) as *const i8,
    (b"Function\0".as_ptr() as *mut i8) as *const i8,
    (b"Return\0".as_ptr() as *mut i8) as *const i8,
    (b"EndCoroutine\0".as_ptr() as *mut i8) as *const i8,
    (b"HaltIfNull\0".as_ptr() as *mut i8) as *const i8,
    (b"Halt\0".as_ptr() as *mut i8) as *const i8,
    (b"Integer\0".as_ptr() as *mut i8) as *const i8,
    (b"Int64\0".as_ptr() as *mut i8) as *const i8,
    (b"String\0".as_ptr() as *mut i8) as *const i8,
    (b"BeginSubrtn\0".as_ptr() as *mut i8) as *const i8,
    (b"Null\0".as_ptr() as *mut i8) as *const i8,
    (b"SoftNull\0".as_ptr() as *mut i8) as *const i8,
    (b"Blob\0".as_ptr() as *mut i8) as *const i8,
    (b"Variable\0".as_ptr() as *mut i8) as *const i8,
    (b"Move\0".as_ptr() as *mut i8) as *const i8,
    (b"Copy\0".as_ptr() as *mut i8) as *const i8,
    (b"SCopy\0".as_ptr() as *mut i8) as *const i8,
    (b"IntCopy\0".as_ptr() as *mut i8) as *const i8,
    (b"FkCheck\0".as_ptr() as *mut i8) as *const i8,
    (b"ResultRow\0".as_ptr() as *mut i8) as *const i8,
    (b"CollSeq\0".as_ptr() as *mut i8) as *const i8,
    (b"AddImm\0".as_ptr() as *mut i8) as *const i8,
    (b"RealAffinity\0".as_ptr() as *mut i8) as *const i8,
    (b"Cast\0".as_ptr() as *mut i8) as *const i8,
    (b"Permutation\0".as_ptr() as *mut i8) as *const i8,
    (b"Compare\0".as_ptr() as *mut i8) as *const i8,
    (b"IsTrue\0".as_ptr() as *mut i8) as *const i8,
    (b"ZeroOrNull\0".as_ptr() as *mut i8) as *const i8,
    (b"Offset\0".as_ptr() as *mut i8) as *const i8,
    (b"Column\0".as_ptr() as *mut i8) as *const i8,
    (b"TypeCheck\0".as_ptr() as *mut i8) as *const i8,
    (b"Affinity\0".as_ptr() as *mut i8) as *const i8,
    (b"MakeRecord\0".as_ptr() as *mut i8) as *const i8,
    (b"Count\0".as_ptr() as *mut i8) as *const i8,
    (b"ReadCookie\0".as_ptr() as *mut i8) as *const i8,
    (b"SetCookie\0".as_ptr() as *mut i8) as *const i8,
    (b"BitAnd\0".as_ptr() as *mut i8) as *const i8,
    (b"BitOr\0".as_ptr() as *mut i8) as *const i8,
    (b"ShiftLeft\0".as_ptr() as *mut i8) as *const i8,
    (b"ShiftRight\0".as_ptr() as *mut i8) as *const i8,
    (b"Add\0".as_ptr() as *mut i8) as *const i8,
    (b"Subtract\0".as_ptr() as *mut i8) as *const i8,
    (b"Multiply\0".as_ptr() as *mut i8) as *const i8,
    (b"Divide\0".as_ptr() as *mut i8) as *const i8,
    (b"Remainder\0".as_ptr() as *mut i8) as *const i8,
    (b"Concat\0".as_ptr() as *mut i8) as *const i8,
    (b"ReopenIdx\0".as_ptr() as *mut i8) as *const i8,
    (b"OpenRead\0".as_ptr() as *mut i8) as *const i8,
    (b"BitNot\0".as_ptr() as *mut i8) as *const i8,
    (b"OpenWrite\0".as_ptr() as *mut i8) as *const i8,
    (b"OpenDup\0".as_ptr() as *mut i8) as *const i8,
    (b"String8\0".as_ptr() as *mut i8) as *const i8,
    (b"OpenAutoindex\0".as_ptr() as *mut i8) as *const i8,
    (b"OpenEphemeral\0".as_ptr() as *mut i8) as *const i8,
    (b"SorterOpen\0".as_ptr() as *mut i8) as *const i8,
    (b"SequenceTest\0".as_ptr() as *mut i8) as *const i8,
    (b"OpenPseudo\0".as_ptr() as *mut i8) as *const i8,
    (b"Close\0".as_ptr() as *mut i8) as *const i8,
    (b"ColumnsUsed\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekScan\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekHit\0".as_ptr() as *mut i8) as *const i8,
    (b"Sequence\0".as_ptr() as *mut i8) as *const i8,
    (b"NewRowid\0".as_ptr() as *mut i8) as *const i8,
    (b"Insert\0".as_ptr() as *mut i8) as *const i8,
    (b"RowCell\0".as_ptr() as *mut i8) as *const i8,
    (b"Delete\0".as_ptr() as *mut i8) as *const i8,
    (b"ResetCount\0".as_ptr() as *mut i8) as *const i8,
    (b"SorterCompare\0".as_ptr() as *mut i8) as *const i8,
    (b"SorterData\0".as_ptr() as *mut i8) as *const i8,
    (b"RowData\0".as_ptr() as *mut i8) as *const i8,
    (b"Rowid\0".as_ptr() as *mut i8) as *const i8,
    (b"NullRow\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekEnd\0".as_ptr() as *mut i8) as *const i8,
    (b"IdxInsert\0".as_ptr() as *mut i8) as *const i8,
    (b"SorterInsert\0".as_ptr() as *mut i8) as *const i8,
    (b"IdxDelete\0".as_ptr() as *mut i8) as *const i8,
    (b"DeferredSeek\0".as_ptr() as *mut i8) as *const i8,
    (b"IdxRowid\0".as_ptr() as *mut i8) as *const i8,
    (b"FinishSeek\0".as_ptr() as *mut i8) as *const i8,
    (b"Destroy\0".as_ptr() as *mut i8) as *const i8,
    (b"Clear\0".as_ptr() as *mut i8) as *const i8,
    (b"ResetSorter\0".as_ptr() as *mut i8) as *const i8,
    (b"CreateBtree\0".as_ptr() as *mut i8) as *const i8,
    (b"SqlExec\0".as_ptr() as *mut i8) as *const i8,
    (b"ParseSchema\0".as_ptr() as *mut i8) as *const i8,
    (b"LoadAnalysis\0".as_ptr() as *mut i8) as *const i8,
    (b"DropTable\0".as_ptr() as *mut i8) as *const i8,
    (b"Real\0".as_ptr() as *mut i8) as *const i8,
    (b"DropIndex\0".as_ptr() as *mut i8) as *const i8,
    (b"DropTrigger\0".as_ptr() as *mut i8) as *const i8,
    (b"IntegrityCk\0".as_ptr() as *mut i8) as *const i8,
    (b"RowSetAdd\0".as_ptr() as *mut i8) as *const i8,
    (b"Param\0".as_ptr() as *mut i8) as *const i8,
    (b"FkCounter\0".as_ptr() as *mut i8) as *const i8,
    (b"MemMax\0".as_ptr() as *mut i8) as *const i8,
    (b"OffsetLimit\0".as_ptr() as *mut i8) as *const i8,
    (b"AggInverse\0".as_ptr() as *mut i8) as *const i8,
    (b"AggStep\0".as_ptr() as *mut i8) as *const i8,
    (b"AggStep1\0".as_ptr() as *mut i8) as *const i8,
    (b"AggValue\0".as_ptr() as *mut i8) as *const i8,
    (b"AggFinal\0".as_ptr() as *mut i8) as *const i8,
    (b"Expire\0".as_ptr() as *mut i8) as *const i8,
    (b"CursorLock\0".as_ptr() as *mut i8) as *const i8,
    (b"CursorUnlock\0".as_ptr() as *mut i8) as *const i8,
    (b"TableLock\0".as_ptr() as *mut i8) as *const i8,
    (b"VBegin\0".as_ptr() as *mut i8) as *const i8,
    (b"VCreate\0".as_ptr() as *mut i8) as *const i8,
    (b"VDestroy\0".as_ptr() as *mut i8) as *const i8,
    (b"VOpen\0".as_ptr() as *mut i8) as *const i8,
    (b"VCheck\0".as_ptr() as *mut i8) as *const i8,
    (b"VInitIn\0".as_ptr() as *mut i8) as *const i8,
    (b"VColumn\0".as_ptr() as *mut i8) as *const i8,
    (b"VRename\0".as_ptr() as *mut i8) as *const i8,
    (b"Pagecount\0".as_ptr() as *mut i8) as *const i8,
    (b"MaxPgcnt\0".as_ptr() as *mut i8) as *const i8,
    (b"ClrSubtype\0".as_ptr() as *mut i8) as *const i8,
    (b"GetSubtype\0".as_ptr() as *mut i8) as *const i8,
    (b"SetSubtype\0".as_ptr() as *mut i8) as *const i8,
    (b"FilterAdd\0".as_ptr() as *mut i8) as *const i8,
    (b"Trace\0".as_ptr() as *mut i8) as *const i8,
    (b"CursorHint\0".as_ptr() as *mut i8) as *const i8,
    (b"ReleaseReg\0".as_ptr() as *mut i8) as *const i8,
    (b"Noop\0".as_ptr() as *mut i8) as *const i8,
    (b"Explain\0".as_ptr() as *mut i8) as *const i8,
    (b"Abortable\0".as_ptr() as *mut i8) as *const i8,
]);

// /* Automatically generated.  Do not edit */
// /* See the tool/mkopcodec.tcl script for details. */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OpcodeName(mut i: i32) -> *const i8 {
    // /*   0 */
    // /*   1 */
    // /*   2 */
    // /*   3 */
    // /*   4 */
    // /*   5 */
    // /*   6 */
    // /*   7 */
    // /*   8 */
    // /*   9 */
    // /*  10 */
    // /*  11 */
    // /*  12 */
    // /*  13 */
    // /*  14 */
    // /*  15 */
    // /*  16 */
    // /*  17 */
    // /*  18 */
    // /*  19 */
    // /*  20 */
    // /*  21 */
    // /*  22 */
    // /*  23 */
    // /*  24 */
    // /*  25 */
    // /*  26 */
    // /*  27 */
    // /*  28 */
    // /*  29 */
    // /*  30 */
    // /*  31 */
    // /*  32 */
    // /*  33 */
    // /*  34 */
    // /*  35 */
    // /*  36 */
    // /*  37 */
    // /*  38 */
    // /*  39 */
    // /*  40 */
    // /*  41 */
    // /*  42 */
    // /*  43 */
    // /*  44 */
    // /*  45 */
    // /*  46 */
    // /*  47 */
    // /*  48 */
    // /*  49 */
    // /*  50 */
    // /*  51 */
    // /*  52 */
    // /*  53 */
    // /*  54 */
    // /*  55 */
    // /*  56 */
    // /*  57 */
    // /*  58 */
    // /*  59 */
    // /*  60 */
    // /*  61 */
    // /*  62 */
    // /*  63 */
    // /*  64 */
    // /*  65 */
    // /*  66 */
    // /*  67 */
    // /*  68 */
    // /*  69 */
    // /*  70 */
    // /*  71 */
    // /*  72 */
    // /*  73 */
    // /*  74 */
    // /*  75 */
    // /*  76 */
    // /*  77 */
    // /*  78 */
    // /*  79 */
    // /*  80 */
    // /*  81 */
    // /*  82 */
    // /*  83 */
    // /*  84 */
    // /*  85 */
    // /*  86 */
    // /*  87 */
    // /*  88 */
    // /*  89 */
    // /*  90 */
    // /*  91 */
    // /*  92 */
    // /*  93 */
    // /*  94 */
    // /*  95 */
    // /*  96 */
    // /*  97 */
    // /*  98 */
    // /*  99 */
    // /* 100 */
    // /* 101 */
    // /* 102 */
    // /* 103 */
    // /* 104 */
    // /* 105 */
    // /* 106 */
    // /* 107 */
    // /* 108 */
    // /* 109 */
    // /* 110 */
    // /* 111 */
    // /* 112 */
    // /* 113 */
    // /* 114 */
    // /* 115 */
    // /* 116 */
    // /* 117 */
    // /* 118 */
    // /* 119 */
    // /* 120 */
    // /* 121 */
    // /* 122 */
    // /* 123 */
    // /* 124 */
    // /* 125 */
    // /* 126 */
    // /* 127 */
    // /* 128 */
    // /* 129 */
    // /* 130 */
    // /* 131 */
    // /* 132 */
    // /* 133 */
    // /* 134 */
    // /* 135 */
    // /* 136 */
    // /* 137 */
    // /* 138 */
    // /* 139 */
    // /* 140 */
    // /* 141 */
    // /* 142 */
    // /* 143 */
    // /* 144 */
    // /* 145 */
    // /* 146 */
    // /* 147 */
    // /* 148 */
    // /* 149 */
    // /* 150 */
    // /* 151 */
    // /* 152 */
    // /* 153 */
    // /* 154 */
    // /* 155 */
    // /* 156 */
    // /* 157 */
    // /* 158 */
    // /* 159 */
    // /* 160 */
    // /* 161 */
    // /* 162 */
    // /* 163 */
    // /* 164 */
    // /* 165 */
    // /* 166 */
    // /* 167 */
    // /* 168 */
    // /* 169 */
    // /* 170 */
    // /* 171 */
    // /* 172 */
    // /* 173 */
    // /* 174 */
    // /* 175 */
    // /* 176 */
    // /* 177 */
    // /* 178 */
    // /* 179 */
    // /* 180 */
    // /* 181 */
    // /* 182 */
    // /* 183 */
    // /* 184 */
    // /* 185 */
    // /* 186 */
    // /* 187 */
    // /* 188 */
    // /* 189 */
    // /* 190 */
    // /* 191 */
    return unsafe {
        *unsafe { unsafe { std::ptr::addr_of!(azName.0) as *const *const i8 }.offset(i as isize) }
    };
}
