#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

/// Automatically generated.  Do not edit
///
/// See the tool/mkopcodec.tcl script for details.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OpcodeName(mut i: i32) -> *const i8 {
    //   0
    //   1
    //   2
    //   3
    //   4
    //   5
    //   6
    //   7
    //   8
    //   9
    //  10
    //  11
    //  12
    //  13
    //  14
    //  15
    //  16
    //  17
    //  18
    //  19
    //  20
    //  21
    //  22
    //  23
    //  24
    //  25
    //  26
    //  27
    //  28
    //  29
    //  30
    //  31
    //  32
    //  33
    //  34
    //  35
    //  36
    //  37
    //  38
    //  39
    //  40
    //  41
    //  42
    //  43
    //  44
    //  45
    //  46
    //  47
    //  48
    //  49
    //  50
    //  51
    //  52
    //  53
    //  54
    //  55
    //  56
    //  57
    //  58
    //  59
    //  60
    //  61
    //  62
    //  63
    //  64
    //  65
    //  66
    //  67
    //  68
    //  69
    //  70
    //  71
    //  72
    //  73
    //  74
    //  75
    //  76
    //  77
    //  78
    //  79
    //  80
    //  81
    //  82
    //  83
    //  84
    //  85
    //  86
    //  87
    //  88
    //  89
    //  90
    //  91
    //  92
    //  93
    //  94
    //  95
    //  96
    //  97
    //  98
    //  99
    // 100
    // 101
    // 102
    // 103
    // 104
    // 105
    // 106
    // 107
    // 108
    // 109
    // 110
    // 111
    // 112
    // 113
    // 114
    // 115
    // 116
    // 117
    // 118
    // 119
    // 120
    // 121
    // 122
    // 123
    // 124
    // 125
    // 126
    // 127
    // 128
    // 129
    // 130
    // 131
    // 132
    // 133
    // 134
    // 135
    // 136
    // 137
    // 138
    // 139
    // 140
    // 141
    // 142
    // 143
    // 144
    // 145
    // 146
    // 147
    // 148
    // 149
    // 150
    // 151
    // 152
    // 153
    // 154
    // 155
    // 156
    // 157
    // 158
    // 159
    // 160
    // 161
    // 162
    // 163
    // 164
    // 165
    // 166
    // 167
    // 168
    // 169
    // 170
    // 171
    // 172
    // 173
    // 174
    // 175
    // 176
    // 177
    // 178
    // 179
    // 180
    // 181
    // 182
    // 183
    // 184
    // 185
    // 186
    // 187
    // 188
    // 189
    // 190
    // 191
    return unsafe {
        *unsafe { unsafe { std::ptr::addr_of!(azName.0) as *const *const i8 }.offset(i as isize) }
    };
}

static mut azName: __SlateAlign16<[*const i8; 192]> = __SlateAlign16([
    (b"Savepoint\0\0".as_ptr() as *mut i8) as *const i8,
    (b"AutoCommit\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Transaction\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Checkpoint\0\0".as_ptr() as *mut i8) as *const i8,
    (b"JournalMode\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Vacuum\0\0".as_ptr() as *mut i8) as *const i8,
    (b"VFilter\0iplan=r[P3] zplan='P4'\0".as_ptr() as *mut i8) as *const i8,
    (b"VUpdate\0data=r[P3@P2]\0".as_ptr() as *mut i8) as *const i8,
    (b"Init\0Start at P2\0".as_ptr() as *mut i8) as *const i8,
    (b"Goto\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Gosub\0\0".as_ptr() as *mut i8) as *const i8,
    (b"InitCoroutine\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Yield\0\0".as_ptr() as *mut i8) as *const i8,
    (b"MustBeInt\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Jump\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Once\0\0".as_ptr() as *mut i8) as *const i8,
    (b"If\0\0".as_ptr() as *mut i8) as *const i8,
    (b"IfNot\0\0".as_ptr() as *mut i8) as *const i8,
    (b"IsType\0if typeof(P1.P3) in P5 goto P2\0".as_ptr() as *mut i8) as *const i8,
    (b"Not\0r[P2]= !r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"IfNullRow\0if P1.nullRow then r[P3]=NULL, goto P2\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekLT\0key=r[P3@P4]\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekLE\0key=r[P3@P4]\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekGE\0key=r[P3@P4]\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekGT\0key=r[P3@P4]\0".as_ptr() as *mut i8) as *const i8,
    (b"IfNotOpen\0if( !csr[P1] ) goto P2\0".as_ptr() as *mut i8) as *const i8,
    (b"IfNoHope\0key=r[P3@P4]\0".as_ptr() as *mut i8) as *const i8,
    (b"NoConflict\0key=r[P3@P4]\0".as_ptr() as *mut i8) as *const i8,
    (b"NotFound\0key=r[P3@P4]\0".as_ptr() as *mut i8) as *const i8,
    (b"Found\0key=r[P3@P4]\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekRowid\0intkey=r[P3]\0".as_ptr() as *mut i8) as *const i8,
    (b"NotExists\0intkey=r[P3]\0".as_ptr() as *mut i8) as *const i8,
    (b"Last\0\0".as_ptr() as *mut i8) as *const i8,
    (b"IfSizeBetween\0\0".as_ptr() as *mut i8) as *const i8,
    (b"SorterSort\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Sort\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Rewind\0\0".as_ptr() as *mut i8) as *const i8,
    (b"IfEmpty\0if( empty(P1) ) goto P2\0".as_ptr() as *mut i8) as *const i8,
    (b"SorterNext\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Prev\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Next\0\0".as_ptr() as *mut i8) as *const i8,
    (b"IdxLE\0key=r[P3@P4]\0".as_ptr() as *mut i8) as *const i8,
    (b"IdxGT\0key=r[P3@P4]\0".as_ptr() as *mut i8) as *const i8,
    (b"Or\0r[P3]=(r[P1] || r[P2])\0".as_ptr() as *mut i8) as *const i8,
    (b"And\0r[P3]=(r[P1] && r[P2])\0".as_ptr() as *mut i8) as *const i8,
    (b"IdxLT\0key=r[P3@P4]\0".as_ptr() as *mut i8) as *const i8,
    (b"IdxGE\0key=r[P3@P4]\0".as_ptr() as *mut i8) as *const i8,
    (b"IFindKey\0\0".as_ptr() as *mut i8) as *const i8,
    (b"RowSetRead\0r[P3]=rowset(P1)\0".as_ptr() as *mut i8) as *const i8,
    (b"RowSetTest\0if r[P3] in rowset(P1) goto P2\0".as_ptr() as *mut i8) as *const i8,
    (b"Program\0\0".as_ptr() as *mut i8) as *const i8,
    (b"IsNull\0if r[P1]==NULL goto P2\0".as_ptr() as *mut i8) as *const i8,
    (b"NotNull\0if r[P1]!=NULL goto P2\0".as_ptr() as *mut i8) as *const i8,
    (b"Ne\0IF r[P3]!=r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"Eq\0IF r[P3]==r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"Gt\0IF r[P3]>r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"Le\0IF r[P3]<=r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"Lt\0IF r[P3]<r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"Ge\0IF r[P3]>=r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"ElseEq\0\0".as_ptr() as *mut i8) as *const i8,
    (b"FkIfZero\0if fkctr[P1]==0 goto P2\0".as_ptr() as *mut i8) as *const i8,
    (b"IfPos\0if r[P1]>0 then r[P1]-=P3, goto P2\0".as_ptr() as *mut i8) as *const i8,
    (b"IfNotZero\0if r[P1]!=0 then r[P1]--, goto P2\0".as_ptr() as *mut i8) as *const i8,
    (b"DecrJumpZero\0if (--r[P1])==0 goto P2\0".as_ptr() as *mut i8) as *const i8,
    (b"IncrVacuum\0\0".as_ptr() as *mut i8) as *const i8,
    (b"VNext\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Filter\0if key(P3@P4) not in filter(P1) goto P2\0".as_ptr() as *mut i8) as *const i8,
    (b"PureFunc\0r[P3]=func(r[P2@NP])\0".as_ptr() as *mut i8) as *const i8,
    (b"Function\0r[P3]=func(r[P2@NP])\0".as_ptr() as *mut i8) as *const i8,
    (b"Return\0\0".as_ptr() as *mut i8) as *const i8,
    (b"EndCoroutine\0\0".as_ptr() as *mut i8) as *const i8,
    (b"HaltIfNull\0if r[P3]=null halt\0".as_ptr() as *mut i8) as *const i8,
    (b"Halt\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Integer\0r[P2]=P1\0".as_ptr() as *mut i8) as *const i8,
    (b"Int64\0r[P2]=PINT13\0".as_ptr() as *mut i8) as *const i8,
    (b"String\0r[P2]='P4' (len=P1)\0".as_ptr() as *mut i8) as *const i8,
    (b"BeginSubrtn\0r[P2]=NULL\0".as_ptr() as *mut i8) as *const i8,
    (b"Null\0r[P2..P3]=NULL\0".as_ptr() as *mut i8) as *const i8,
    (b"SoftNull\0r[P1]=NULL\0".as_ptr() as *mut i8) as *const i8,
    (b"Blob\0r[P2]=P4 (len=P1)\0".as_ptr() as *mut i8) as *const i8,
    (b"Variable\0r[P2]=parameter(P1)\0".as_ptr() as *mut i8) as *const i8,
    (b"Move\0r[P2@P3]=r[P1@P3]\0".as_ptr() as *mut i8) as *const i8,
    (b"Copy\0r[P2@P3+1]=r[P1@P3+1]\0".as_ptr() as *mut i8) as *const i8,
    (b"SCopy\0r[P2]=r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"IntCopy\0r[P2]=r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"FkCheck\0\0".as_ptr() as *mut i8) as *const i8,
    (b"ResultRow\0output=r[P1@P2]\0".as_ptr() as *mut i8) as *const i8,
    (b"CollSeq\0\0".as_ptr() as *mut i8) as *const i8,
    (b"AddImm\0r[P1]=r[P1]+P2\0".as_ptr() as *mut i8) as *const i8,
    (b"RealAffinity\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Cast\0affinity(r[P1])\0".as_ptr() as *mut i8) as *const i8,
    (b"Permutation\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Compare\0r[P1@P3] <-> r[P2@P3]\0".as_ptr() as *mut i8) as *const i8,
    (b"IsTrue\0r[P2] = coalesce(r[P1]==TRUE,P3) ^ P4\0".as_ptr() as *mut i8) as *const i8,
    (b"ZeroOrNull\0r[P2] = 0 OR NULL\0".as_ptr() as *mut i8) as *const i8,
    (b"Offset\0r[P3] = sqlite_offset(P1)\0".as_ptr() as *mut i8) as *const i8,
    (b"Column\0r[P3]=PX cursor P1 column P2\0".as_ptr() as *mut i8) as *const i8,
    (b"TypeCheck\0typecheck(r[P1@P2])\0".as_ptr() as *mut i8) as *const i8,
    (b"Affinity\0affinity(r[P1@P2])\0".as_ptr() as *mut i8) as *const i8,
    (b"MakeRecord\0r[P3]=mkrec(r[P1@P2])\0".as_ptr() as *mut i8) as *const i8,
    (b"Count\0r[P2]=count()\0".as_ptr() as *mut i8) as *const i8,
    (b"ReadCookie\0\0".as_ptr() as *mut i8) as *const i8,
    (b"SetCookie\0\0".as_ptr() as *mut i8) as *const i8,
    (b"BitAnd\0r[P3]=r[P1]&r[P2]\0".as_ptr() as *mut i8) as *const i8,
    (b"BitOr\0r[P3]=r[P1]|r[P2]\0".as_ptr() as *mut i8) as *const i8,
    (b"ShiftLeft\0r[P3]=r[P2]<<r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"ShiftRight\0r[P3]=r[P2]>>r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"Add\0r[P3]=r[P1]+r[P2]\0".as_ptr() as *mut i8) as *const i8,
    (b"Subtract\0r[P3]=r[P2]-r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"Multiply\0r[P3]=r[P1]*r[P2]\0".as_ptr() as *mut i8) as *const i8,
    (b"Divide\0r[P3]=r[P2]/r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"Remainder\0r[P3]=r[P2]%r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"Concat\0r[P3]=r[P2]+r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"ReopenIdx\0root=P2 iDb=P3\0".as_ptr() as *mut i8) as *const i8,
    (b"OpenRead\0root=P2 iDb=P3\0".as_ptr() as *mut i8) as *const i8,
    (b"BitNot\0r[P2]= ~r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"OpenWrite\0root=P2 iDb=P3\0".as_ptr() as *mut i8) as *const i8,
    (b"OpenDup\0\0".as_ptr() as *mut i8) as *const i8,
    (b"String8\0r[P2]='P4'\0".as_ptr() as *mut i8) as *const i8,
    (b"OpenAutoindex\0nColumn=P2\0".as_ptr() as *mut i8) as *const i8,
    (b"OpenEphemeral\0nColumn=P2\0".as_ptr() as *mut i8) as *const i8,
    (b"SorterOpen\0\0".as_ptr() as *mut i8) as *const i8,
    (b"SequenceTest\0if( cursor[P1].ctr++ ) pc = P2\0".as_ptr() as *mut i8) as *const i8,
    (b"OpenPseudo\0P3 columns in r[P2]\0".as_ptr() as *mut i8) as *const i8,
    (b"Close\0\0".as_ptr() as *mut i8) as *const i8,
    (b"ColumnsUsed\0Cursor P1 uses columns PHEX23\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekScan\0Scan-ahead up to P1 rows\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekHit\0set P2<=seekHit<=P3\0".as_ptr() as *mut i8) as *const i8,
    (b"Sequence\0r[P2]=cursor[P1].ctr++\0".as_ptr() as *mut i8) as *const i8,
    (b"NewRowid\0r[P2]=rowid\0".as_ptr() as *mut i8) as *const i8,
    (b"Insert\0intkey=r[P3] data=r[P2]\0".as_ptr() as *mut i8) as *const i8,
    (b"RowCell\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Delete\0\0".as_ptr() as *mut i8) as *const i8,
    (b"ResetCount\0\0".as_ptr() as *mut i8) as *const i8,
    (b"SorterCompare\0if key(P1)!=trim(r[P3],P4) goto P2\0".as_ptr() as *mut i8) as *const i8,
    (b"SorterData\0r[P2]=data\0".as_ptr() as *mut i8) as *const i8,
    (b"RowData\0r[P2]=data\0".as_ptr() as *mut i8) as *const i8,
    (b"Rowid\0r[P2]=PX rowid of P1\0".as_ptr() as *mut i8) as *const i8,
    (b"NullRow\0\0".as_ptr() as *mut i8) as *const i8,
    (b"SeekEnd\0\0".as_ptr() as *mut i8) as *const i8,
    (b"IdxInsert\0key=r[P2]\0".as_ptr() as *mut i8) as *const i8,
    (b"SorterInsert\0key=r[P2]\0".as_ptr() as *mut i8) as *const i8,
    (b"IdxDelete\0key=r[P2@P5]\0".as_ptr() as *mut i8) as *const i8,
    (b"DeferredSeek\0Move P3 to P1.rowid if needed\0".as_ptr() as *mut i8) as *const i8,
    (b"IdxRowid\0r[P2]=rowid\0".as_ptr() as *mut i8) as *const i8,
    (b"FinishSeek\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Destroy\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Clear\0\0".as_ptr() as *mut i8) as *const i8,
    (b"ResetSorter\0\0".as_ptr() as *mut i8) as *const i8,
    (b"CreateBtree\0r[P2]=root iDb=P1 flags=P3\0".as_ptr() as *mut i8) as *const i8,
    (b"SqlExec\0\0".as_ptr() as *mut i8) as *const i8,
    (b"ParseSchema\0\0".as_ptr() as *mut i8) as *const i8,
    (b"LoadAnalysis\0\0".as_ptr() as *mut i8) as *const i8,
    (b"DropTable\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Real\0r[P2]=PDBL13\0".as_ptr() as *mut i8) as *const i8,
    (b"DropIndex\0\0".as_ptr() as *mut i8) as *const i8,
    (b"DropTrigger\0\0".as_ptr() as *mut i8) as *const i8,
    (b"IntegrityCk\0\0".as_ptr() as *mut i8) as *const i8,
    (b"RowSetAdd\0rowset(P1)=r[P2]\0".as_ptr() as *mut i8) as *const i8,
    (b"Param\0\0".as_ptr() as *mut i8) as *const i8,
    (b"FkCounter\0fkctr[P1]+=P2\0".as_ptr() as *mut i8) as *const i8,
    (b"MemMax\0r[P1]=max(r[P1],r[P2])\0".as_ptr() as *mut i8) as *const i8,
    (b"OffsetLimit\0if r[P1]>0 then r[P2]=r[P1]+max(0,r[P3]) else r[P2]=(-1)\0".as_ptr() as *mut i8)
        as *const i8,
    (b"AggInverse\0accum=r[P3] inverse(r[P2@P5])\0".as_ptr() as *mut i8) as *const i8,
    (b"AggStep\0accum=r[P3] step(r[P2@P5])\0".as_ptr() as *mut i8) as *const i8,
    (b"AggStep1\0accum=r[P3] step(r[P2@P5])\0".as_ptr() as *mut i8) as *const i8,
    (b"AggValue\0r[P3]=value N=P2\0".as_ptr() as *mut i8) as *const i8,
    (b"AggFinal\0accum=r[P1] N=P2\0".as_ptr() as *mut i8) as *const i8,
    (b"Expire\0\0".as_ptr() as *mut i8) as *const i8,
    (b"CursorLock\0\0".as_ptr() as *mut i8) as *const i8,
    (b"CursorUnlock\0\0".as_ptr() as *mut i8) as *const i8,
    (b"TableLock\0iDb=P1 root=P2 write=P3\0".as_ptr() as *mut i8) as *const i8,
    (b"VBegin\0\0".as_ptr() as *mut i8) as *const i8,
    (b"VCreate\0\0".as_ptr() as *mut i8) as *const i8,
    (b"VDestroy\0\0".as_ptr() as *mut i8) as *const i8,
    (b"VOpen\0\0".as_ptr() as *mut i8) as *const i8,
    (b"VCheck\0\0".as_ptr() as *mut i8) as *const i8,
    (b"VInitIn\0r[P2]=ValueList(P1,P3)\0".as_ptr() as *mut i8) as *const i8,
    (b"VColumn\0r[P3]=vcolumn(P2)\0".as_ptr() as *mut i8) as *const i8,
    (b"VRename\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Pagecount\0\0".as_ptr() as *mut i8) as *const i8,
    (b"MaxPgcnt\0\0".as_ptr() as *mut i8) as *const i8,
    (b"ClrSubtype\0r[P1].subtype = 0\0".as_ptr() as *mut i8) as *const i8,
    (b"GetSubtype\0r[P2] = r[P1].subtype\0".as_ptr() as *mut i8) as *const i8,
    (b"SetSubtype\0r[P2].subtype = r[P1]\0".as_ptr() as *mut i8) as *const i8,
    (b"FilterAdd\0filter(P1) += key(P3@P4)\0".as_ptr() as *mut i8) as *const i8,
    (b"Trace\0\0".as_ptr() as *mut i8) as *const i8,
    (b"CursorHint\0\0".as_ptr() as *mut i8) as *const i8,
    (b"ReleaseReg\0release r[P1@P2] mask P3\0".as_ptr() as *mut i8) as *const i8,
    (b"Noop\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Explain\0\0".as_ptr() as *mut i8) as *const i8,
    (b"Abortable\0\0".as_ptr() as *mut i8) as *const i8,
]);
