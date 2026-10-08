//! 2001 September 15
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This file contains routines used for analyzing expressions and
//! for generating VDBE code that evaluates expressions in SQLite.
unsafe extern "C" {
    static mut sqlite3CtypeMap: [u8; 0];
    fn sqlite3_value_int64(__v1635: *mut sqlite3_value) -> i64;
    fn sqlite3_value_text(__v1636: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_type(__v1637: *mut sqlite3_value) -> i32;
    fn sqlite3_stricmp(__v1638: *const i8, __v1639: *const i8) -> i32;
    fn sqlite3_strnicmp(__v1640: *const i8, __v1641: *const i8, __v1642: i32) -> i32;
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3VdbeParser(__v1652: *mut Vdbe) -> *mut Parse;
    fn sqlite3VdbeAddOp0(__v1653: *mut Vdbe, __v1654: i32) -> i32;
    fn sqlite3VdbeAddOp1(__v1655: *mut Vdbe, __v1656: i32, __v1657: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v1658: *mut Vdbe, __v1659: i32, __v1660: i32, __v1661: i32) -> i32;
    fn sqlite3VdbeGoto(__v1662: *mut Vdbe, __v1663: i32) -> i32;
    fn sqlite3VdbeLoadString(__v1664: *mut Vdbe, __v1665: i32, __v1666: *const i8) -> i32;
    fn sqlite3VdbeAddOp3(
        __v1667: *mut Vdbe,
        __v1668: i32,
        __v1669: i32,
        __v1670: i32,
        __v1671: i32,
    ) -> i32;
    fn sqlite3VdbeAddInt64(__v1672: *mut Vdbe, __v1673: i32, __v1674: i64) -> i32;
    fn sqlite3VdbeAddDouble(__v1675: *mut Vdbe, __v1676: i32, __v1677: f64) -> i32;
    fn sqlite3VdbeAddOp4(
        __v1678: *mut Vdbe,
        __v1679: i32,
        __v1680: i32,
        __v1681: i32,
        __v1682: i32,
        zP4: *const i8,
        __v1684: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4Int(
        __v1685: *mut Vdbe,
        __v1686: i32,
        __v1687: i32,
        __v1688: i32,
        __v1689: i32,
        __v1690: i32,
    ) -> i32;
    fn sqlite3VdbeAddFunctionCall(
        __v1691: *mut Parse,
        __v1692: i32,
        __v1693: i32,
        __v1694: i32,
        __v1695: i32,
        __v1696: *const FuncDef,
        __v1697: i32,
    ) -> i32;
    fn sqlite3VdbeExplain(__v1698: *mut Parse, __v1699: u8, __v1700: *const i8, ...) -> i32;
    fn sqlite3VdbeChangeP2(__v1701: *mut Vdbe, addr: i32, P2: i32);
    fn sqlite3VdbeChangeP3(__v1704: *mut Vdbe, addr: i32, P3: i32);
    fn sqlite3VdbeChangeP5(__v1707: *mut Vdbe, P5: u16);
    fn sqlite3VdbeTypeofColumn(__v1709: *mut Vdbe, __v1710: i32);
    fn sqlite3VdbeJumpHere(__v1711: *mut Vdbe, addr: i32);
    fn sqlite3VdbeChangeToNoop(__v1713: *mut Vdbe, addr: i32) -> i32;
    fn sqlite3VdbeChangeP4(__v1715: *mut Vdbe, addr: i32, zP4: *const i8, N: i32);
    fn sqlite3VdbeSetP4KeyInfo(__v1719: *mut Parse, __v1720: *mut Index);
    fn sqlite3VdbeGetOp(__v1721: *mut Vdbe, __v1722: i32) -> *mut VdbeOp;
    fn sqlite3VdbeGetLastOp(__v1723: *mut Vdbe) -> *mut VdbeOp;
    fn sqlite3VdbeMakeLabel(__v1724: *mut Parse) -> i32;
    fn sqlite3VdbeResolveLabel(__v1725: *mut Vdbe, __v1726: i32);
    fn sqlite3VdbeCurrentAddr(__v1727: *mut Vdbe) -> i32;
    fn sqlite3VdbeDb(__v1728: *mut Vdbe) -> *mut sqlite3;
    fn sqlite3VdbeGetBoundValue(
        __v1729: *mut Vdbe,
        __v1730: i32,
        __v1731: u8,
    ) -> *mut sqlite3_value;
    fn sqlite3VdbeReprepareOnBind(__v1732: *mut Vdbe, __v1733: i32, __v1734: i32);
    fn sqlite3MemCompare(
        __v1735: *const sqlite3_value,
        __v1736: *const sqlite3_value,
        __v1737: *const CollSeq,
    ) -> i32;
    fn sqlite3VdbeComment(__v1738: *mut Vdbe, __v1739: *const i8, ...);
    fn sqlite3VdbeNoopComment(__v1740: *mut Vdbe, __v1741: *const i8, ...);
    fn sqlite3WalkExpr(__v1742: *mut Walker, __v1743: *mut Expr) -> i32;
    fn sqlite3WalkExprList(__v1744: *mut Walker, __v1745: *mut ExprList) -> i32;
    fn sqlite3WalkSelect(__v1746: *mut Walker, __v1747: *mut Select) -> i32;
    fn sqlite3SelectWalkNoop(__v1748: *mut Walker, __v1749: *mut Select) -> i32;
    fn sqlite3WalkerDepthIncrease(__v1752: *mut Walker, __v1753: *mut Select) -> i32;
    fn sqlite3WalkerDepthDecrease(__v1754: *mut Walker, __v1755: *mut Select);
    fn sqlite3WindowDelete(__v1756: *mut sqlite3, __v1757: *mut Window);
    fn sqlite3WindowLink(pSel: *mut Select, pWin: *mut Window);
    fn sqlite3WindowCompare(
        __v1760: *const Parse,
        __v1761: *const Window,
        __v1762: *const Window,
        __v1763: i32,
    ) -> i32;
    fn sqlite3WindowDup(db: *mut sqlite3, pOwner: *mut Expr, p: *mut Window) -> *mut Window;
    fn sqlite3WindowListDup(db: *mut sqlite3, p: *mut Window) -> *mut Window;
    fn sqlite3StrICmp(__v1769: *const i8, __v1770: *const i8) -> i32;
    fn sqlite3Strlen30(__v1771: *const i8) -> i32;
    fn sqlite3DbMallocZero(__v1772: *mut sqlite3, __v1773: u64) -> *mut ();
    fn sqlite3DbMallocRaw(__v1774: *mut sqlite3, __v1775: u64) -> *mut ();
    fn sqlite3DbMallocRawNN(__v1776: *mut sqlite3, __v1777: u64) -> *mut ();
    fn sqlite3DbStrDup(__v1778: *mut sqlite3, __v1779: *const i8) -> *mut i8;
    fn sqlite3DbStrNDup(__v1780: *mut sqlite3, __v1781: *const i8, __v1782: u64) -> *mut i8;
    fn sqlite3DbSpanDup(__v1783: *mut sqlite3, __v1784: *const i8, __v1785: *const i8) -> *mut i8;
    fn sqlite3DbRealloc(__v1786: *mut sqlite3, __v1787: *mut (), __v1788: u64) -> *mut ();
    fn sqlite3DbFree(__v1789: *mut sqlite3, __v1790: *mut ());
    fn sqlite3DbNNFreeNN(__v1791: *mut sqlite3, __v1792: *mut ());
    fn sqlite3DbMallocSize(__v1793: *mut sqlite3, __v1794: *const ()) -> i32;
    fn sqlite3ErrorMsg(__v1795: *mut Parse, __v1796: *const i8, ...);
    fn sqlite3Dequote(__v1797: *mut i8);
    fn sqlite3DequoteExpr(__v1798: *mut Expr);
    fn sqlite3TokenInit(__v1799: *mut Token, __v1800: *mut i8);
    fn sqlite3ColumnExpr(__v1887: *mut Table, __v1888: *mut Column) -> *mut Expr;
    fn sqlite3ColumnColl(__v1889: *mut Column) -> *const i8;
    fn sqlite3PrimaryKeyIndex(__v1890: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToIndex(__v1891: *mut Index, __v1892: i32) -> i32;
    fn sqlite3TableColumnToStorage(__v1893: *mut Table, __v1894: i16) -> i16;
    fn sqlite3ArrayAllocate(
        __v1895: *mut sqlite3,
        __v1896: *mut (),
        __v1897: i32,
        __v1898: *mut i32,
        __v1899: *mut i32,
    ) -> *mut ();
    fn sqlite3IdListDelete(__v1900: *mut sqlite3, __v1901: *mut IdList);
    fn sqlite3Select(__v1904: *mut Parse, __v1905: *mut Select, __v1906: *mut SelectDest) -> i32;
    fn sqlite3SelectNew(
        __v1907: *mut Parse,
        __v1908: *mut ExprList,
        __v1909: *mut SrcList,
        __v1910: *mut Expr,
        __v1911: *mut ExprList,
        __v1912: *mut Expr,
        __v1913: *mut ExprList,
        __v1914: u32,
        __v1915: *mut Expr,
    ) -> *mut Select;
    fn sqlite3SelectDelete(__v1916: *mut sqlite3, __v1917: *mut Select);
    fn sqlite3OpenTable(
        __v1918: *mut Parse,
        iCur: i32,
        iDb: i32,
        __v1921: *mut Table,
        __v1922: i32,
    );
    fn sqlite3GetVdbe(__v2016: *mut Parse) -> *mut Vdbe;
    fn sqlite3CodeVerifySchema(__v2017: *mut Parse, __v2018: i32);
    fn sqlite3MayAbort(__v2045: *mut Parse);
    fn sqlite3FindFunction(
        __v2060: *mut sqlite3,
        __v2061: *const i8,
        __v2062: i32,
        __v2063: u8,
        __v2064: u8,
    ) -> *mut FuncDef;
    fn sqlite3ColumnIndex(pTab: *mut Table, zCol: *const i8) -> i32;
    fn sqlite3AtoF(z: *const i8, __v2070: *mut f64) -> i32;
    fn sqlite3VListAdd(
        __v2071: *mut sqlite3,
        __v2072: *mut i32,
        __v2073: *const i8,
        __v2074: i32,
        __v2075: i32,
    ) -> *mut i32;
    fn sqlite3VListNumToName(__v2076: *mut i32, __v2077: i32) -> *const i8;
    fn sqlite3VListNameToNum(__v2078: *mut i32, __v2079: *const i8, __v2080: i32) -> i32;
    fn sqlite3Atoi64(__v2089: *const i8, __v2090: *mut i64, __v2091: i32, __v2092: u8) -> i32;
    fn sqlite3DecOrHexToI64(__v2093: *const i8, __v2094: *mut i64) -> i32;
    fn sqlite3HexToBlob(__v2095: *mut sqlite3, z: *const i8, n: i32) -> *mut ();
    fn sqlite3FindCollSeq(
        __v2098: *mut sqlite3,
        enc: u8,
        __v2100: *const i8,
        __v2101: i32,
    ) -> *mut CollSeq;
    fn sqlite3IsBinary(__v2102: *const CollSeq) -> i32;
    fn sqlite3CheckCollSeq(__v2119: *mut Parse, __v2120: *mut CollSeq) -> i32;
    fn sqlite3ValueFree(__v2121: *mut sqlite3_value);
    fn sqlite3ValueFromExpr(
        __v2122: *mut sqlite3,
        __v2123: *const Expr,
        __v2124: u8,
        __v2125: u8,
        __v2126: *mut *mut sqlite3_value,
    ) -> i32;
    fn sqlite3ColumnDefault(__v2133: *mut Vdbe, __v2134: *mut Table, __v2135: i32, __v2136: i32);
    fn sqlite3RenameTokenMap(
        __v2137: *mut Parse,
        __v2138: *const (),
        __v2139: *const Token,
    ) -> *const ();
    fn sqlite3RenameExprUnmap(__v2140: *mut Parse, __v2141: *mut Expr);
    fn sqlite3GetCollSeq(
        __v2142: *mut Parse,
        __v2143: u8,
        __v2144: *mut CollSeq,
        __v2145: *const i8,
    ) -> *mut CollSeq;
    fn sqlite3AffinityType(__v2146: *const i8, __v2147: *mut Column) -> i8;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, __v2149: *mut Schema) -> i32;
    fn sqlite3KeyInfoAlloc(__v2150: *mut sqlite3, __v2151: i32, __v2152: i32) -> *mut KeyInfo;
    fn sqlite3KeyInfoUnref(__v2153: *mut KeyInfo);
    fn sqlite3SelectDestInit(__v2154: *mut SelectDest, __v2155: i32, __v2156: i32);
    fn sqlite3RecordErrorOffsetOfExpr(__v2157: *mut sqlite3, __v2158: *const Expr);
    fn sqlite3TableLock(
        __v2161: *mut Parse,
        __v2162: i32,
        __v2163: u32,
        __v2164: u8,
        __v2165: *const i8,
    );
    fn sqlite3VtabOverloadFunction(
        __v2166: *mut sqlite3,
        __v2167: *mut FuncDef,
        nArg: i32,
        __v2169: *mut Expr,
    ) -> *mut FuncDef;
    fn sqlite3ParserAddCleanup(
        __v2170: *mut Parse,
        __v2171: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
        __v2172: *mut (),
    ) -> *mut ();
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

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_mutex {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_module {
    iVersion: i32,
    xCreate: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *mut (),
            i32,
            *const *const i8,
            *mut *mut sqlite3_vtab,
            *mut *mut i8,
        ) -> i32,
    >,
    xConnect: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *mut (),
            i32,
            *const *const i8,
            *mut *mut sqlite3_vtab,
            *mut *mut i8,
        ) -> i32,
    >,
    xBestIndex:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab, *mut sqlite3_index_info) -> i32>,
    xDisconnect: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32>,
    xDestroy: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32>,
    xOpen: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_vtab, *mut *mut sqlite3_vtab_cursor) -> i32,
    >,
    xClose: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab_cursor) -> i32>,
    xFilter: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vtab_cursor,
            i32,
            *const i8,
            i32,
            *mut *mut sqlite3_value,
        ) -> i32,
    >,
    xNext: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab_cursor) -> i32>,
    xEof: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab_cursor) -> i32>,
    xColumn: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_vtab_cursor, *mut sqlite3_context, i32) -> i32,
    >,
    xRowid: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab_cursor, *mut i64) -> i32>,
    xUpdate: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vtab,
            i32,
            *mut *mut sqlite3_value,
            *mut i64,
        ) -> i32,
    >,
    xBegin: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32>,
    xSync: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32>,
    xCommit: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32>,
    xRollback: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32>,
    xFindFunction: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vtab,
            i32,
            *const i8,
            *mut Option<
                unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
            >,
            *mut *mut (),
        ) -> i32,
    >,
    xRename: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab, *const i8) -> i32>,
    xSavepoint: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab, i32) -> i32>,
    xRelease: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab, i32) -> i32>,
    xRollbackTo: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab, i32) -> i32>,
    xShadowName: Option<unsafe extern "C-unwind" fn(*const i8) -> i32>,
    xIntegrity: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vtab,
            *const i8,
            *const i8,
            i32,
            *mut *mut i8,
        ) -> i32,
    >,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_value {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_context {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_index_info {
    nConstraint: i32,
    aConstraint: *mut sqlite3_index_constraint,
    nOrderBy: i32,
    aOrderBy: *mut sqlite3_index_orderby,
    aConstraintUsage: *mut sqlite3_index_constraint_usage,
    idxNum: i32,
    idxStr: *mut i8,
    needToFreeIdxStr: i32,
    orderByConsumed: i32,
    estimatedCost: f64,
    estimatedRows: i64,
    idxFlags: i32,
    colUsed: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_vtab {
    pModule: *const sqlite3_module,
    nRef: i32,
    zErrMsg: *mut i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_vtab_cursor {
    pVtab: *mut sqlite3_vtab,
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
struct sqlite3_index_constraint {
    iColumn: i32,
    op: u8,
    usable: u8,
    iTermOffset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_index_orderby {
    iColumn: i32,
    desc: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_index_constraint_usage {
    argvIndex: i32,
    omit: u8,
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
struct BusyHandler {
    xBusyHandler: Option<unsafe extern "C-unwind" fn(*mut (), i32) -> i32>,
    pBusyArg: *mut (),
    nBusy: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _ht {
    count: u32,
    chain: *mut HashElem,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SubrtnSig {
    selId: i32,
    bComplete: u8,
    zAff: *mut i8,
    iTable: i32,
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VdbeOp {
    opcode: u8,
    p4type: i8,
    p5: u16,
    p1: i32,
    p2: i32,
    p3: i32,
    p4: p4union,
    zComment: *mut i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SubProgram {
    aOp: *mut VdbeOp,
    nOp: i32,
    nMem: i32,
    nCsr: i32,
    aOnce: *mut u8,
    token: *mut (),
    pNext: *mut SubProgram,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Db {
    zDbSName: *mut i8,
    pBt: *mut Btree,
    safety_level: u8,
    bSyncSet: u8,
    pSchema: *mut Schema,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Schema {
    schema_cookie: i32,
    iGeneration: i32,
    tblHash: Hash,
    idxHash: Hash,
    trigHash: Hash,
    fkeyHash: Hash,
    pSeqTab: *mut Table,
    file_format: u8,
    enc: u8,
    schemaFlags: u16,
    cache_size: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Lookaside {
    bDisable: u32,
    sz: u16,
    szTrue: u16,
    bMalloced: u8,
    nSlot: u32,
    anStat: [u32; 3],
    pInit: *mut LookasideSlot,
    pFree: *mut LookasideSlot,
    pSmallInit: *mut LookasideSlot,
    pSmallFree: *mut LookasideSlot,
    pMiddle: *mut (),
    pStart: *mut (),
    pEnd: *mut (),
    pTrueEnd: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct LookasideSlot {
    pNext: *mut LookasideSlot,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3 {
    pVfs: *mut sqlite3_vfs,
    pVdbe: *mut Vdbe,
    pDfltColl: *mut CollSeq,
    mutex: *mut sqlite3_mutex,
    aDb: *mut Db,
    nDb: i32,
    mDbFlags: u32,
    flags: u64,
    lastRowid: i64,
    szMmap: i64,
    nSchemaLock: u32,
    openFlags: u32,
    errCode: i32,
    errByteOffset: i32,
    errMask: i32,
    iSysErrno: i32,
    dbOptFlags: u32,
    enc: u8,
    autoCommit: u8,
    temp_store: u8,
    mallocFailed: u8,
    bBenignMalloc: u8,
    dfltLockMode: u8,
    nextAutovac: i8,
    suppressErr: u8,
    vtabOnConflict: u8,
    isTransactionSavepoint: u8,
    mTrace: u8,
    noSharedCache: u8,
    nSqlExec: u8,
    eOpenState: u8,
    nFpDigit: u8,
    nextPagesize: i32,
    nChange: i64,
    nTotalChange: i64,
    aLimit: [i32; 15],
    nMaxSorterMmap: i32,
    init: sqlite3InitInfo,
    nVdbeActive: i32,
    nVdbeRead: i32,
    nVdbeWrite: i32,
    nVdbeExec: i32,
    nVDestroy: i32,
    nExtension: i32,
    aExtension: *mut *mut (),
    trace: __SlateRecord165,
    pTraceArg: *mut (),
    xProfile: Option<unsafe extern "C-unwind" fn(*mut (), *const i8, u64)>,
    pProfileArg: *mut (),
    pCommitArg: *mut (),
    xCommitCallback: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    pRollbackArg: *mut (),
    xRollbackCallback: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pUpdateArg: *mut (),
    xUpdateCallback: Option<unsafe extern "C-unwind" fn(*mut (), i32, *const i8, *const i8, i64)>,
    pAutovacPagesArg: *mut (),
    xAutovacDestr: Option<unsafe extern "C-unwind" fn(*mut ())>,
    xAutovacPages: Option<unsafe extern "C-unwind" fn(*mut (), *const i8, u32, u32, u32) -> u32>,
    pParse: *mut Parse,
    xWalCallback: Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, *const i8, i32) -> i32>,
    pWalArg: *mut (),
    xCollNeeded: Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, i32, *const i8)>,
    xCollNeeded16: Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, i32, *const ())>,
    pCollNeededArg: *mut (),
    pErr: *mut sqlite3_value,
    u1: __SlateRecord166,
    lookaside: Lookaside,
    xAuth: Option<
        unsafe extern "C-unwind" fn(
            *mut (),
            i32,
            *const i8,
            *const i8,
            *const i8,
            *const i8,
        ) -> i32,
    >,
    pAuthArg: *mut (),
    xProgress: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    pProgressArg: *mut (),
    nProgressOps: u32,
    nVTrans: i32,
    aModule: Hash,
    pVtabCtx: *mut VtabCtx,
    aVTrans: *mut *mut VTable,
    pDisconnect: *mut VTable,
    aFunc: Hash,
    aCollSeq: Hash,
    busyHandler: BusyHandler,
    aDbStatic: [Db; 2],
    pSavepoint: *mut Savepoint,
    nAnalysisLimit: i32,
    busyTimeout: i32,
    nSavepoint: i32,
    nStatement: i32,
    nDeferredCons: i64,
    nDeferredImmCons: i64,
    pnBytesFreed: *mut i32,
    pDbData: *mut DbClientData,
    nSpill: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct FuncDef {
    nArg: i16,
    funcFlags: u32,
    pUserData: *mut (),
    pNext: *mut FuncDef,
    xSFunc: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value)>,
    xFinalize: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    xValue: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    xInverse:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value)>,
    zName: *const i8,
    u: __SlateRecord167,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct FuncDestructor {
    nRef: i32,
    xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pUserData: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Savepoint {
    zName: *mut i8,
    nDeferredCons: i64,
    nDeferredImmCons: i64,
    pNext: *mut Savepoint,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Module {
    pModule: *const sqlite3_module,
    zName: *const i8,
    nRefModule: i32,
    pAux: *mut (),
    xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pEpoTab: *mut Table,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Column {
    zCnName: *mut i8,
    __slate_bits_0: __slate_bits::__SlateBits62U0,
    affinity: i8,
    szEst: u8,
    hName: u8,
    iDflt: u16,
    colFlags: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CollSeq {
    zName: *mut i8,
    enc: u8,
    pUser: *mut (),
    xCmp: Option<unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32>,
    xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VTable {
    db: *mut sqlite3,
    pMod: *mut Module,
    pVtab: *mut sqlite3_vtab,
    nRef: i32,
    bConstraint: u8,
    bAllSchemas: u8,
    eVtabRisk: u8,
    iSavepoint: i32,
    pNext: *mut VTable,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Table {
    zName: *mut i8,
    aCol: *mut Column,
    pIndex: *mut Index,
    zColAff: *mut i8,
    pCheck: *mut ExprList,
    tnum: u32,
    nTabRef: u32,
    tabFlags: u32,
    iPKey: i16,
    nCol: i16,
    nNVCol: i16,
    nRowLogEst: i16,
    szTabRow: i16,
    keyConf: u8,
    eTabType: u8,
    u: __SlateRecord168,
    pTrigger: *mut Trigger,
    pSchema: *mut Schema,
    aHx: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct FKey {
    pFrom: *mut Table,
    pNextFrom: *mut FKey,
    zTo: *mut i8,
    pNextTo: *mut FKey,
    pPrevTo: *mut FKey,
    nCol: i32,
    isDeferred: u8,
    aAction: [u8; 2],
    apTrigger: [*mut Trigger; 2],
    aCol: [sColMap; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct KeyInfo {
    nRef: u32,
    enc: u8,
    nKeyField: u16,
    nAllField: u16,
    db: *mut sqlite3,
    aSortFlags: *mut u8,
    aColl: [*mut CollSeq; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Index {
    zName: *mut i8,
    aiColumn: *mut i16,
    aiRowLogEst: *mut i16,
    pTable: *mut Table,
    zColAff: *mut i8,
    pNext: *mut Index,
    pSchema: *mut Schema,
    aSortOrder: *mut u8,
    azColl: *mut *const i8,
    pPartIdxWhere: *mut Expr,
    aColExpr: *mut ExprList,
    tnum: u32,
    szIdxRow: i16,
    nKeyCol: u16,
    nColumn: u16,
    onError: u8,
    __slate_bits_0: __slate_bits::__SlateBits88U0,
    colNotIdxed: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Token {
    z: *const i8,
    n: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct AggInfo {
    directMode: u8,
    useSortingIdx: u8,
    nSortingColumn: u32,
    sortingIdx: i32,
    sortingIdxPTab: i32,
    iFirstReg: i32,
    pGroupBy: *mut ExprList,
    aCol: *mut AggInfo_col,
    nColumn: i32,
    nAccumulator: i32,
    aFunc: *mut AggInfo_func,
    nFunc: i32,
    selId: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Expr {
    op: u8,
    affExpr: i8,
    op2: u8,
    flags: u32,
    u: __SlateRecord176,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord177,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord178,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord179,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList {
    nExpr: i32,
    nAlloc: i32,
    a: [ExprList_item; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct IdList {
    nId: i32,
    a: [IdList_item; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Subquery {
    pSelect: *mut Select,
    addrFillSub: i32,
    regReturn: i32,
    regResult: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SrcItem {
    zName: *mut i8,
    zAlias: *mut i8,
    pSTab: *mut Table,
    fg: __SlateRecord186,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord187,
    u2: __SlateRecord188,
    u3: __SlateRecord189,
    u4: __SlateRecord190,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct OnOrUsing {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RenameToken {}

#[repr(C)]
#[derive(Clone, Copy)]
struct SrcList {
    nSrc: i32,
    nAlloc: u32,
    a: [SrcItem; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct NameContext {
    pParse: *mut Parse,
    pSrcList: *mut SrcList,
    uNC: __SlateRecord191,
    pNext: *mut NameContext,
    nRef: i32,
    nNcErr: i32,
    ncFlags: i32,
    nNestedSelect: u32,
    pWinSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Upsert {
    pUpsertTarget: *mut ExprList,
    pUpsertTargetWhere: *mut Expr,
    pUpsertSet: *mut ExprList,
    pUpsertWhere: *mut Expr,
    pNextUpsert: *mut Upsert,
    isDoUpdate: u8,
    isDup: u8,
    pToFree: *mut (),
    pUpsertIdx: *mut Index,
    pUpsertSrc: *mut SrcList,
    regData: i32,
    iDataCur: i32,
    iIdxCur: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Select {
    op: u8,
    nSelectRow: i16,
    selFlags: u32,
    iLimit: i32,
    iOffset: i32,
    selId: u32,
    pEList: *mut ExprList,
    pSrc: *mut SrcList,
    pWhere: *mut Expr,
    pGroupBy: *mut ExprList,
    pHaving: *mut Expr,
    pOrderBy: *mut ExprList,
    pPrior: *mut Select,
    pNext: *mut Select,
    pLimit: *mut Expr,
    pWith: *mut With,
    pWin: *mut Window,
    pWinDefn: *mut Window,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SelectDest {
    eDest: u8,
    iSDParm: i32,
    iSDParm2: i32,
    iSdst: i32,
    nSdst: i32,
    zAffSdst: *mut i8,
    pOrderBy: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct AutoincInfo {
    pNext: *mut AutoincInfo,
    pTab: *mut Table,
    iDb: i32,
    regCtr: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TriggerPrg {
    pTrigger: *mut Trigger,
    pNext: *mut TriggerPrg,
    pProgram: *mut SubProgram,
    orconf: i32,
    aColmask: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct IndexedExpr {
    pExpr: *mut Expr,
    iDataCur: i32,
    iIdxCur: i32,
    iIdxCol: i32,
    bMaybeNullRow: u8,
    aff: u8,
    pIENext: *mut IndexedExpr,
    zIdxName: *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TableLock {}

#[repr(C)]
#[derive(Clone, Copy)]
struct ParseCleanup {
    pNext: *mut ParseCleanup,
    pPtr: *mut (),
    xCleanup: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Parse {
    db: *mut sqlite3,
    zErrMsg: *mut i8,
    pVdbe: *mut Vdbe,
    rc: i32,
    nQueryLoop: i16,
    nested: u8,
    nTempReg: u8,
    isMultiWrite: u8,
    disableLookaside: u8,
    prepFlags: u8,
    withinRJSubrtn: u8,
    mSubrtnSig: u8,
    eTriggerOp: u8,
    eOrconf: u8,
    __slate_bits_0: __slate_bits::__SlateBits104U0,
    nRangeReg: i32,
    iRangeReg: i32,
    nErr: i32,
    nTab: i32,
    nMem: i32,
    iSelfTab: i32,
    nNestSel: i32,
    nLabel: i32,
    nLabelAlloc: i32,
    aLabel: *mut i32,
    pConstExpr: *mut ExprList,
    pIdxEpr: *mut IndexedExpr,
    pIdxPartExpr: *mut IndexedExpr,
    writeMask: u32,
    cookieMask: u32,
    nMaxArg: i32,
    nSelect: i32,
    nProgressSteps: u32,
    nTableLock: i32,
    pToplevel: *mut Parse,
    pTriggerTab: *mut Table,
    pTriggerPrg: *mut TriggerPrg,
    pCleanup: *mut ParseCleanup,
    aTempReg: [i32; 8],
    pOuterParse: *mut Parse,
    sNameToken: Token,
    oldmask: u32,
    newmask: u32,
    u1: __SlateRecord193,
    pAinc: *mut AutoincInfo,
    aTableLock: *mut TableLock,
    sLastToken: Token,
    nVar: i16,
    aVnbmc: [u64; 2],
    iPkSortOrder: u8,
    explain: u8,
    eParseMode: u8,
    nVtabLock: i32,
    nHeight: i32,
    addrExplain: i32,
    pVList: *mut i32,
    pReprepare: *mut Vdbe,
    zTail: *const i8,
    pNewTable: *mut Table,
    pNewIndex: *mut Index,
    pNewTrigger: *mut Trigger,
    zAuthContext: *const i8,
    sArg: Token,
    apVtabLock: *mut *mut Table,
    pWith: *mut With,
    pRename: *mut RenameToken,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Trigger {
    zName: *mut i8,
    table: *mut i8,
    op: u8,
    tr_tm: u8,
    bReturning: u8,
    pWhen: *mut Expr,
    pColumns: *mut IdList,
    pSchema: *mut Schema,
    pTabSchema: *mut Schema,
    step_list: *mut TriggerStep,
    pNext: *mut Trigger,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TriggerStep {
    op: u8,
    orconf: u8,
    pTrig: *mut Trigger,
    pSelect: *mut Select,
    pSrc: *mut SrcList,
    pWhere: *mut Expr,
    pExprList: *mut ExprList,
    pIdList: *mut IdList,
    pUpsert: *mut Upsert,
    zSpan: *mut i8,
    pNext: *mut TriggerStep,
    pLast: *mut TriggerStep,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Returning {
    pParse: *mut Parse,
    pReturnEL: *mut ExprList,
    retTrig: Trigger,
    retTStep: TriggerStep,
    iRetCur: i32,
    nRetCol: i32,
    iRetReg: i32,
    zName: [i8; 40],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Walker {
    pParse: *mut Parse,
    xExprCallback: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Expr) -> i32>,
    xSelectCallback: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
    xSelectCallback2: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select)>,
    walkerDepth: i32,
    eCode: u16,
    mWFlags: u16,
    u: __SlateRecord196,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VtabCtx {}

#[repr(C)]
#[derive(Clone, Copy)]
struct DbFixer {
    pParse: *mut Parse,
    w: Walker,
    pSchema: *mut Schema,
    bTemp: u8,
    zDb: *const i8,
    zType: *const i8,
    pName: *const Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Cte {
    zName: *mut i8,
    pCols: *mut ExprList,
    pSelect: *mut Select,
    zCteErr: *const i8,
    pUse: *mut CteUse,
    eM10d: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct With {
    nCte: i32,
    bView: i32,
    pOuter: *mut With,
    a: [Cte; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Btree {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Vdbe {}

#[repr(C)]
#[derive(Clone, Copy)]
struct CteUse {
    nUse: i32,
    addrM9e: i32,
    regRtn: i32,
    iCur: i32,
    nRowEst: i16,
    eM10d: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct DbClientData {
    pNext: *mut DbClientData,
    pData: *mut (),
    xDestructor: Option<unsafe extern "C-unwind" fn(*mut ())>,
    zName: [i8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Window {
    zName: *mut i8,
    zBase: *mut i8,
    pPartition: *mut ExprList,
    pOrderBy: *mut ExprList,
    eFrmType: u8,
    eStart: u8,
    eEnd: u8,
    bImplicitFrame: u8,
    eExclude: u8,
    pStart: *mut Expr,
    pEnd: *mut Expr,
    ppThis: *mut *mut Window,
    pNextWin: *mut Window,
    pFilter: *mut Expr,
    pWFunc: *mut FuncDef,
    iEphCsr: i32,
    regAccum: i32,
    regResult: i32,
    csrApp: i32,
    regApp: i32,
    regPart: i32,
    pOwner: *mut Expr,
    nBufferCol: i32,
    iArgCol: i32,
    regOne: i32,
    regStartRowid: i32,
    regEndRowid: i32,
    bExprArgs: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union p4union {
    i: i32,
    p: *mut (),
    z: *mut i8,
    pFunc: *mut FuncDef,
    pCtx: *mut sqlite3_context,
    pColl: *mut CollSeq,
    pMem: *mut sqlite3_value,
    pVtab: *mut VTable,
    pKeyInfo: *mut KeyInfo,
    ai: *mut u32,
    pProgram: *mut SubProgram,
    pTab: *mut Table,
    pSubrtnSig: *mut SubrtnSig,
    pIdx: *mut Index,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3InitInfo {
    newTnum: u32,
    iDb: u8,
    busy: u8,
    __slate_bits_0: __slate_bits::__SlateBits164U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord165 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord166 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord167 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord168 {
    tab: __SlateRecord169,
    view: __SlateRecord170,
    vtab: __SlateRecord171,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord169 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord170 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord171 {
    nArg: i32,
    azArg: *mut *mut i8,
    p: *mut VTable,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sColMap {
    iFrom: i32,
    zCol: *mut i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct AggInfo_col {
    pTab: *mut Table,
    pCExpr: *mut Expr,
    iTable: i32,
    iColumn: i32,
    iSorterColumn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct AggInfo_func {
    pFExpr: *mut Expr,
    pFunc: *mut FuncDef,
    iDistinct: i32,
    iDistAddr: i32,
    iOBTab: i32,
    bOBPayload: u8,
    bOBUnique: u8,
    bUseSubtype: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord180,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord180 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord182,
    u: __SlateRecord183,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord182 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits182U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord183 {
    x: __SlateRecord184,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord184 {
    iOrderByCol: u16,
    iAlias: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct IdList_item {
    zName: *mut i8,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord186 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits186U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord189 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord190 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    pEList: *mut ExprList,
    pAggInfo: *mut AggInfo,
    pUpsert: *mut Upsert,
    iBaseReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord193 {
    cr: __SlateRecord194,
    d: __SlateRecord195,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord194 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord195 {
    pReturning: *mut Returning,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord196 {
    pNC: *mut NameContext,
    n: i32,
    iCur: i32,
    sz: i32,
    pSrcList: *mut SrcList,
    pCCurHint: *mut CCurHint,
    pRefSrcList: *mut RefSrcList,
    aiCol: *mut i32,
    pIdxCover: *mut IdxCover,
    pGroupBy: *mut ExprList,
    pSelect: *mut Select,
    pRewrite: *mut WindowRewrite,
    pConst: *mut WhereConst,
    pRename: *mut RenameCtx,
    pTab: *mut Table,
    pCovIdxCk: *mut CoveringIndexCheck,
    pSrcItem: *mut SrcItem,
    pFix: *mut DbFixer,
    aMem: *mut sqlite3_value,
    pCheckOnCtx: *mut CheckOnCtx,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CCurHint {}

/// Return the affinity character for a single column of a table.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TableColumnAffinity(mut pTab: *const Table, mut iCol: i32) -> i8 {
    if iCol < (0 as i32) || iCol >= ((unsafe { (*pTab).nCol }) as i32) {
        return (68 as i32) as i8;
    }
    return unsafe { (*unsafe { unsafe { (*pTab).aCol }.offset(iCol as isize) }).affinity };
}

/// Return the 'affinity' of the expression pExpr if any.
///
/// If pExpr is a column, a reference to a column via an 'AS' alias,
/// or a sub-select with a column as the return value, then the
/// affinity of that column is returned. Otherwise, 0x00 is returned,
/// indicating no affinity for the expression.
///
/// i.e. the WHERE clause expressions in the following statements all
/// have an affinity:
///
/// CREATE TABLE t1(a);
/// SELECT * FROM t1 WHERE a;
/// SELECT a AS b FROM t1 WHERE b;
/// SELECT * FROM t1 WHERE (select a from t1);
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprAffinity(mut pExpr: *const Expr) -> i8 {
    let mut op: i32 = 0 as i32;
    op = ((unsafe { (*pExpr).op }) as u32) as i32;
    '__slate_break_2212: while (1 as i32) != (0 as i32) {
        // exit-by-break
        if op == (168 as i32)
            || op == (170 as i32) && (unsafe { (*pExpr).y.pTab }) != std::ptr::null_mut::<Table>()
        {
            0 as i32;
            0 as i32;
            return sqlite3TableColumnAffinity(
                (unsafe { (*pExpr).y.pTab }) as *const Table,
                (unsafe { (*pExpr).iColumn }) as i32,
            );
        }
        if op == (139 as i32) {
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            return sqlite3ExprAffinity(
                (unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!(
                                (*unsafe { (*unsafe { (*pExpr).x.pSelect }).pEList }).a
                            ) as *mut ExprList_item
                        }
                        .offset((0 as i32) as isize)
                    })
                    .pExpr
                }) as *const Expr,
            );
        }
        if op == (36 as i32) {
            0 as i32;
            return unsafe {
                sqlite3AffinityType(
                    (unsafe { (*pExpr).u.zToken }) as *const i8,
                    std::ptr::null_mut::<Column>(),
                )
            };
        }
        if op == (178 as i32) {
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            return sqlite3ExprAffinity(
                (unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!(
                                (*unsafe {
                                    (*unsafe { (*unsafe { (*pExpr).pLeft }).x.pSelect }).pEList
                                })
                                .a
                            ) as *mut ExprList_item
                        }
                        .offset(((unsafe { (*pExpr).iColumn }) as i32) as isize)
                    })
                    .pExpr
                }) as *const Expr,
            );
        }
        if op == (177 as i32)
            || op == (172 as i32) && ((unsafe { (*pExpr).affExpr }) as i32) == (88 as i32)
        {
            0 as i32;
            return sqlite3ExprAffinity(
                (unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*pExpr).x.pList }).a)
                                as *mut ExprList_item
                        }
                        .offset((0 as i32) as isize)
                    })
                    .pExpr
                }) as *const Expr,
            );
        }
        if (unsafe { (*pExpr).flags }) & (((8192 as i32) | (262144 as i32)) as u32)
            != ((0 as i32) as u32)
        {
            0 as i32;
            pExpr = (unsafe { (*pExpr).pLeft }) as *const Expr;
            op = ((unsafe { (*pExpr).op }) as u32) as i32;
        } else {
            if op != (176 as i32) {
                break '__slate_break_2212;
            }
            op = ((unsafe { (*pExpr).op2 }) as u32) as i32;
            if op == (176 as i32) {
                break '__slate_break_2212;
            }
        }
    }
    return unsafe { (*pExpr).affExpr };
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WindowRewrite {}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereConst {}

#[repr(C)]
#[derive(Clone, Copy)]
struct RenameCtx {}

#[repr(C)]
#[derive(Clone, Copy)]
struct CoveringIndexCheck {}

#[repr(C)]
#[derive(Clone, Copy)]
struct CheckOnCtx {}

/// Make a guess at all the possible datatypes of the result that could
/// be returned by an expression.  Return a bitmask indicating the answer:
///
///     0x01         Numeric
///     0x02         Text
///     0x04         Blob
///
/// If the expression must return NULL, then 0x00 is returned.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprDataType(mut pExpr: *const Expr) -> i32 {
    '__slate_break_2213: while pExpr != std::ptr::null::<Expr>() {
        match ((unsafe { (*pExpr).op }) as u32) as i32 {
            114 | 179 | 173 => {
                pExpr = (unsafe { (*pExpr).pLeft }) as *const Expr;
            }
            122 => {
                pExpr = std::ptr::null::<Expr>();
            }
            118 => {
                return 2 as i32;
            }
            155 => {
                return 4 as i32;
            }
            112 => {
                return 6 as i32;
            }
            157 | 169 | 172 => {
                return 7 as i32;
            }
            168 | 170 | 139 | 36 | 178 | 177 => {
                let mut aff: i32 = sqlite3ExprAffinity(pExpr) as i32;
                if aff >= (67 as i32) {
                    return 5 as i32;
                }
                if aff == (66 as i32) {
                    return 6 as i32;
                }
                return 7 as i32;
            }
            158 => {
                let mut res: i32 = 0 as i32;
                let mut ii: i32 = 0 as i32;
                let mut pList: *mut ExprList = unsafe { (*pExpr).x.pList };
                0 as i32;
                0 as i32;
                ii = 1 as i32;
                '__slate_break_2215: loop {
                    if !(ii < unsafe { (*pList).nExpr }) {
                        break;
                    }
                    let __v2544: i32 = res;
                    let __v2545: i32 = __v2544
                        | sqlite3ExprDataType(
                            (unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item
                                    }
                                    .offset(ii as isize)
                                })
                                .pExpr
                            }) as *const Expr,
                        );
                    res = __v2545;
                    let __v2542: i32 = ii;
                    let __v2543: i32 = __v2542 + (2 as i32);
                    ii = __v2543;
                }
                if (unsafe { (*pList).nExpr }) % (2 as i32) != (0 as i32) {
                    let __v2546: i32 = res;
                    let __v2547: i32 = __v2546
                        | sqlite3ExprDataType(
                            (unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item
                                    }
                                    .offset(((unsafe { (*pList).nExpr }) - (1 as i32)) as isize)
                                })
                                .pExpr
                            }) as *const Expr,
                        );
                    res = __v2547;
                }
                return res;
            }
            _ => {
                return 1 as i32;
            }
        }
        // End of switch(op)
    }
    // End of while(pExpr)
    return 0 as i32;
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

mod __slate_bits {
    #[bitfields::bitfield(
        u8,
        c_names = true,
        new = false,
        from_into_bits = false,
        from_traits = false,
        default = false,
        debug = false,
        builder = false,
        bit_ops = false
    )]
    pub struct __SlateBits62U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits88U0 {
        #[bits(2)]
        pub idxType: u32,
        #[bits(1)]
        pub bUnordered: u32,
        #[bits(1)]
        pub uniqNotNull: u32,
        #[bits(1)]
        pub isResized: u32,
        #[bits(1)]
        pub isCovering: u32,
        #[bits(1)]
        pub noSkipScan: u32,
        #[bits(1)]
        pub hasStat1: u32,
        #[bits(1)]
        pub bNoQuery: u32,
        #[bits(1)]
        pub bAscKeyBug: u32,
        #[bits(1)]
        pub bHasVCol: u32,
        #[bits(1)]
        pub bHasExpr: u32,
        #[bits(4, access = na)]
        pub __slate_pad_11: u8,
    }
    #[bitfields::bitfield(
        u8,
        c_names = true,
        new = false,
        from_into_bits = false,
        from_traits = false,
        default = false,
        debug = false,
        builder = false,
        bit_ops = false
    )]
    pub struct __SlateBits164U0 {
        #[bits(1)]
        pub orphanTrigger: u32,
        #[bits(2)]
        pub imposterTable: u32,
        #[bits(1)]
        pub reopenMemdb: u32,
        #[bits(4, access = na)]
        pub __slate_pad_3: u8,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits104U0 {
        #[bits(1)]
        pub disableTriggers: u32,
        #[bits(1)]
        pub mayAbort: u32,
        #[bits(1)]
        pub hasCompound: u32,
        #[bits(1)]
        pub bReturning: u32,
        #[bits(1)]
        pub bHasExists: u32,
        #[bits(1)]
        pub colNamesSet: u32,
        #[bits(1)]
        pub bHasWith: u32,
        #[bits(1)]
        pub okConstFactor: u32,
        #[bits(1)]
        pub checkSchema: u32,
        #[bits(1)]
        pub usesAinc: u32,
        #[bits(6, access = na)]
        pub __slate_pad_10: u8,
    }
    #[bitfields::bitfield([u8; 3], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits186U0 {
        #[bits(1)]
        pub notIndexed: u32,
        #[bits(1)]
        pub isIndexedBy: u32,
        #[bits(1)]
        pub isSubquery: u32,
        #[bits(1)]
        pub isTabFunc: u32,
        #[bits(1)]
        pub isCorrelated: u32,
        #[bits(1)]
        pub isMaterialized: u32,
        #[bits(1)]
        pub viaCoroutine: u32,
        #[bits(1)]
        pub isRecursive: u32,
        #[bits(1)]
        pub fromDDL: u32,
        #[bits(1)]
        pub isCte: u32,
        #[bits(1)]
        pub notCte: u32,
        #[bits(1)]
        pub isUsing: u32,
        #[bits(1)]
        pub isOn: u32,
        #[bits(1)]
        pub isSynthUsing: u32,
        #[bits(1)]
        pub isNestedFrom: u32,
        #[bits(1)]
        pub rowidUsed: u32,
        #[bits(1)]
        pub fixedSchema: u32,
        #[bits(1)]
        pub hadSchema: u32,
        #[bits(1)]
        pub fromExists: u32,
        #[bits(5, access = na)]
        pub __slate_pad_19: u8,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits182U0 {
        #[bits(2)]
        pub eEName: u32,
        #[bits(1)]
        pub done: u32,
        #[bits(1)]
        pub reusable: u32,
        #[bits(1)]
        pub bSorterRef: u32,
        #[bits(1)]
        pub bNulls: u32,
        #[bits(1)]
        pub bUsed: u32,
        #[bits(1)]
        pub bUsingTerm: u32,
        #[bits(1)]
        pub bNoExpand: u32,
        #[bits(7, access = na)]
        pub __slate_pad_8: u8,
    }
}

/// Set the collating sequence for expression pExpr to be the collating
/// sequence named by pToken.   Return a pointer to a new Expr node that
/// implements the COLLATE operator.
///
/// If a memory allocation error occurs, that fact is recorded in pParse->db
/// and the pExpr parameter is returned unchanged.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pExpr` - Add the "COLLATE" clause to this expression
/// * `pCollName` - Name of collating sequence
/// * `dequote` - True to dequote pCollName
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprAddCollateToken(
    mut pParse: *const Parse,
    mut pExpr: *mut Expr,
    mut pCollName: *const Token,
    mut dequote: i32,
) -> *mut Expr {
    if (unsafe { (*pCollName).n }) > ((0 as i32) as u32) {
        let mut pNew: *mut Expr =
            sqlite3ExprAlloc(unsafe { (*pParse).db }, 114 as i32, pCollName, dequote);
        if pNew != std::ptr::null_mut::<Expr>() {
            unsafe {
                (*pNew).pLeft = pExpr;
            }
            let __v2551: *mut Expr = pNew;
            let __v2552: u32 = unsafe { (*__v2551).flags };
            let __v2553: u32 = __v2552 | (((512 as i32) | (8192 as i32)) as u32);
            unsafe {
                (*__v2551).flags = __v2553;
            }
            pExpr = pNew;
        }
    }
    return pExpr;
}

/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pExpr` - Add the "COLLATE" clause to this expression
/// * `zC` - The collating sequence name
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprAddCollateString(
    mut pParse: *const Parse,
    mut pExpr: *mut Expr,
    mut zC: *const i8,
) -> *mut Expr {
    let mut s: Token = unsafe { std::mem::zeroed() };
    0 as i32;
    unsafe { sqlite3TokenInit(std::ptr::addr_of_mut!(s), zC as *mut i8) };
    return sqlite3ExprAddCollateToken(
        pParse,
        pExpr,
        std::ptr::addr_of_mut!(s) as *const Token,
        0 as i32,
    );
}

/// Skip over any TK_COLLATE operators.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprSkipCollate(mut pExpr: *mut Expr) -> *mut Expr {
    '__slate_break_2216: while pExpr != std::ptr::null_mut::<Expr>()
        && (unsafe { (*pExpr).flags }) & ((8192 as i32) as u32) != ((0 as i32) as u32)
    {
        0 as i32;
        pExpr = unsafe { (*pExpr).pLeft };
    }
    return pExpr;
}

/// Skip over any TK_COLLATE operators and/or any unlikely()
/// or likelihood() or likely() functions at the root of an
/// expression.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprSkipCollateAndLikely(mut pExpr: *mut Expr) -> *mut Expr {
    '__slate_break_2217: while pExpr != std::ptr::null_mut::<Expr>()
        && (unsafe { (*pExpr).flags }) & (((8192 as i32) | (524288 as i32)) as u32)
            != ((0 as i32) as u32)
    {
        if (unsafe { (*pExpr).flags }) & ((524288 as i32) as u32) != ((0 as i32) as u32) {
            0 as i32;
            0 as i32;
            0 as i32;
            pExpr = unsafe {
                (*unsafe {
                    unsafe {
                        std::ptr::addr_of_mut!((*unsafe { (*pExpr).x.pList }).a)
                            as *mut ExprList_item
                    }
                    .offset((0 as i32) as isize)
                })
                .pExpr
            };
        } else {
            if (((unsafe { (*pExpr).op }) as u32) as i32) == (114 as i32) {
                pExpr = unsafe { (*pExpr).pLeft };
            } else {
                break '__slate_break_2217;
            }
        }
    }
    return pExpr;
}

/// Return the collation sequence for the expression pExpr. If
/// there is no defined collating sequence, return NULL.
///
/// See also: sqlite3ExprNNCollSeq()
///
/// The sqlite3ExprNNCollSeq() works the same exact that it returns the
/// default collation if pExpr has no defined collation.
///
/// The collating sequence might be determined by a COLLATE operator
/// or by the presence of a column with a defined collating sequence.
/// COLLATE operators take first precedence.  Left operands take
/// precedence over right operands.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCollSeq(
    mut pParse: *mut Parse,
    mut pExpr: *const Expr,
) -> *mut CollSeq {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pColl: *mut CollSeq = std::ptr::null_mut::<CollSeq>();
    let mut p: *const Expr = pExpr;
    '__slate_break_2218: while p != std::ptr::null::<Expr>() {
        let mut op: i32 = ((unsafe { (*p).op }) as u32) as i32;
        if op == (176 as i32) {
            op = ((unsafe { (*p).op2 }) as u32) as i32;
        }
        if op == (170 as i32) && (unsafe { (*p).y.pTab }) != std::ptr::null_mut::<Table>()
            || op == (168 as i32)
            || op == (78 as i32)
        {
            let mut j: i32 = 0 as i32;
            0 as i32;
            0 as i32;
            let __v2548: i32 = (unsafe { (*p).iColumn }) as i32;
            j = __v2548;
            if __v2548 >= (0 as i32) {
                let mut zColl: *const i8 = unsafe {
                    sqlite3ColumnColl(unsafe {
                        unsafe { (*unsafe { (*p).y.pTab }).aCol }.offset(j as isize)
                    })
                };
                pColl = unsafe { sqlite3FindCollSeq(db, unsafe { (*db).enc }, zColl, 0 as i32) };
            }
            break '__slate_break_2218;
        }
        if op == (36 as i32) || op == (173 as i32) {
            p = (unsafe { (*p).pLeft }) as *const Expr;
        } else {
            if op == (177 as i32)
                || op == (172 as i32) && ((unsafe { (*p).affExpr }) as i32) == (88 as i32)
            {
                0 as i32;
                p = (unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*p).x.pList }).a)
                                as *mut ExprList_item
                        }
                        .offset((0 as i32) as isize)
                    })
                    .pExpr
                }) as *const Expr;
            } else {
                if op == (114 as i32) {
                    0 as i32;
                    pColl = unsafe {
                        sqlite3GetCollSeq(
                            pParse,
                            unsafe { (*db).enc },
                            std::ptr::null_mut::<CollSeq>(),
                            (unsafe { (*p).u.zToken }) as *const i8,
                        )
                    };
                    break '__slate_break_2218;
                }
                if (unsafe { (*p).flags }) & ((512 as i32) as u32) != (0 as u32) {
                    if (unsafe { (*p).pLeft }) != std::ptr::null_mut::<Expr>()
                        && (unsafe { (*unsafe { (*p).pLeft }).flags }) & ((512 as i32) as u32)
                            != ((0 as i32) as u32)
                    {
                        p = (unsafe { (*p).pLeft }) as *const Expr;
                    } else {
                        let mut pNext: *mut Expr = unsafe { (*p).pRight };
                        // The Expr.x union is never used at the same time as Expr.pRight
                        0 as i32;
                        if (unsafe { (*p).flags }) & ((4096 as i32) as u32) == ((0 as i32) as u32)
                            && (unsafe { (*p).x.pList }) != std::ptr::null_mut::<ExprList>()
                            && !((unsafe { (*db).mallocFailed }) != (0 as u8))
                        {
                            let mut i: i32 = 0 as i32;
                            i = 0 as i32;
                            '__slate_break_2219: loop {
                                if !(i < unsafe { (*unsafe { (*p).x.pList }).nExpr }) {
                                    break;
                                }
                                if (unsafe {
                                    (*unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!((*unsafe { (*p).x.pList }).a)
                                                    as *mut ExprList_item
                                            }
                                            .offset(i as isize)
                                        })
                                        .pExpr
                                    })
                                    .flags
                                }) & ((512 as i32) as u32)
                                    != ((0 as i32) as u32)
                                {
                                    pNext = unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!((*unsafe { (*p).x.pList }).a)
                                                    as *mut ExprList_item
                                            }
                                            .offset(i as isize)
                                        })
                                        .pExpr
                                    };
                                    break '__slate_break_2219;
                                }
                                let __v2549: i32 = i;
                                let __v2550: i32 = __v2549 + (1 as i32);
                                i = __v2550;
                            }
                        }
                        p = pNext as *const Expr;
                    }
                } else {
                    break '__slate_break_2218;
                }
            }
        }
    }
    if (unsafe { sqlite3CheckCollSeq(pParse, pColl) }) != (0 as i32) {
        pColl = std::ptr::null_mut::<CollSeq>();
    }
    return pColl;
}

/// Return the collation sequence for the expression pExpr. If
/// there is no defined collating sequence, return a pointer to the
/// default collation sequence.
///
/// See also: sqlite3ExprCollSeq()
///
/// The sqlite3ExprCollSeq() routine works the same except that it
/// returns NULL if there is no defined collation.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprNNCollSeq(
    mut pParse: *mut Parse,
    mut pExpr: *const Expr,
) -> *mut CollSeq {
    let mut p: *mut CollSeq = sqlite3ExprCollSeq(pParse, pExpr);
    if p == std::ptr::null_mut::<CollSeq>() {
        p = unsafe { (*unsafe { (*pParse).db }).pDfltColl };
    }
    0 as i32;
    return p;
}

/// Return TRUE if the two expressions have equivalent collating sequences.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCollSeqMatch(
    mut pParse: *mut Parse,
    mut pE1: *const Expr,
    mut pE2: *const Expr,
) -> i32 {
    let mut pColl1: *mut CollSeq = sqlite3ExprNNCollSeq(pParse, pE1);
    let mut pColl2: *mut CollSeq = sqlite3ExprNNCollSeq(pParse, pE2);
    0 as i32;
    return (pColl1 == pColl2) as i32;
}

/// pExpr is an operand of a comparison operator.  aff2 is the
/// type affinity of the other operand.  This routine returns the
/// type affinity that should be used for the comparison operator.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CompareAffinity(mut pExpr: *const Expr, mut aff2: i8) -> i8 {
    let mut aff1: i8 = sqlite3ExprAffinity(pExpr);
    if (aff1 as i32) > (64 as i32) && (aff2 as i32) > (64 as i32) {
        // Both sides of the comparison are columns. If one has numeric
        // affinity, use that. Otherwise use no affinity.
        if (aff1 as i32) >= (67 as i32) || (aff2 as i32) >= (67 as i32) {
            return (67 as i32) as i8;
        } else {
            return (65 as i32) as i8;
        }
    } else {
        // One side is a column, the other is not. Use the columns affinity.
        0 as i32;
        return ((if (aff1 as i32) <= (64 as i32) {
            aff2 as i32
        } else {
            aff1 as i32
        }) | (64 as i32)) as i8;
    }
    return unsafe { std::mem::zeroed() };
}

/// pExpr is a comparison operator.  Return the type affinity that should
/// be applied to both operands prior to doing the comparison.
fn comparisonAffinity(mut pExpr: *const Expr) -> i8 {
    let mut aff: i8 = 0 as i8;
    0 as i32;
    0 as i32;
    aff = sqlite3ExprAffinity((unsafe { (*pExpr).pLeft }) as *const Expr);
    if (unsafe { (*pExpr).pRight }) != std::ptr::null_mut::<Expr>() {
        aff = sqlite3CompareAffinity((unsafe { (*pExpr).pRight }) as *const Expr, aff);
    } else {
        if (unsafe { (*pExpr).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
            aff = sqlite3CompareAffinity(
                (unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!(
                                (*unsafe { (*unsafe { (*pExpr).x.pSelect }).pEList }).a
                            ) as *mut ExprList_item
                        }
                        .offset((0 as i32) as isize)
                    })
                    .pExpr
                }) as *const Expr,
                aff,
            );
        } else {
            if (aff as i32) == (0 as i32) {
                aff = (65 as i32) as i8;
            }
        }
    }
    return aff;
}

/// pExpr is a comparison expression, eg. '=', '<', IN(...) etc.
/// idx_affinity is the affinity of an indexed column. Return true
/// if the index with affinity idx_affinity may be used to implement
/// the comparison in pExpr.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IndexAffinityOk(mut pExpr: *const Expr, mut idx_affinity: i8) -> i32 {
    let mut aff: i8 = comparisonAffinity(pExpr);
    if (aff as i32) < (66 as i32) {
        return 1 as i32;
    }
    if (aff as i32) == (66 as i32) {
        return ((idx_affinity as i32) == (66 as i32)) as i32;
    }
    return ((idx_affinity as i32) >= (67 as i32)) as i32;
}

/// Return the P5 value that should be used for a binary comparison
/// opcode (OP_Eq, OP_Ge etc.) used to compare pExpr1 and pExpr2.
///
/// # Arguments
///
/// * `pExpr1` - Left operand
/// * `pExpr2` - Right operand
/// * `jumpIfNull` - Extra flags added to P5
fn binaryCompareP5(mut pExpr1: *const Expr, mut pExpr2: *const Expr, mut jumpIfNull: i32) -> u8 {
    let mut aff: u8 = sqlite3ExprAffinity(pExpr2) as u8;
    aff = (((((sqlite3CompareAffinity(pExpr1, aff as i8) as u8) as u32) as i32)
        | ((((jumpIfNull as i8) as u8) as u32) as i32)) as i8) as u8;
    return aff;
}

/// Return a pointer to the collation sequence that should be used by
/// a binary comparison operator comparing pLeft and pRight.
///
/// If the left hand expression has a collating sequence type, then it is
/// used. Otherwise the collation sequence for the right hand expression
/// is used, or the default (BINARY) if neither expression has a collating
/// type.
///
/// Argument pRight (but not pLeft) may be a null pointer. In this case,
/// it is not considered.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BinaryCompareCollSeq(
    mut pParse: *mut Parse,
    mut pLeft: *const Expr,
    mut pRight: *const Expr,
) -> *mut CollSeq {
    let mut pColl: *mut CollSeq = unsafe { std::mem::zeroed() };
    0 as i32;
    if (unsafe { (*pLeft).flags }) & ((512 as i32) as u32) != (0 as u32) {
        pColl = sqlite3ExprCollSeq(pParse, pLeft);
    } else {
        if pRight != std::ptr::null::<Expr>()
            && (unsafe { (*pRight).flags }) & ((512 as i32) as u32) != ((0 as i32) as u32)
        {
            pColl = sqlite3ExprCollSeq(pParse, pRight);
        } else {
            pColl = sqlite3ExprCollSeq(pParse, pLeft);
            if !(pColl != std::ptr::null_mut::<CollSeq>()) {
                pColl = sqlite3ExprCollSeq(pParse, pRight);
            }
        }
    }
    return pColl;
}

/// Expression p is a comparison operator.  Return a collation sequence
/// appropriate for the comparison operator.
///
/// This is normally just a wrapper around sqlite3BinaryCompareCollSeq().
/// However, if the OP_Commuted flag is set, then the order of the operands
/// is reversed in the sqlite3BinaryCompareCollSeq() call so that the
/// correct collating sequence is found.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCompareCollSeq(
    mut pParse: *mut Parse,
    mut p: *const Expr,
) -> *mut CollSeq {
    if (unsafe { (*p).flags }) & ((1024 as i32) as u32) != ((0 as i32) as u32) {
        return sqlite3BinaryCompareCollSeq(
            pParse,
            (unsafe { (*p).pRight }) as *const Expr,
            (unsafe { (*p).pLeft }) as *const Expr,
        );
    } else {
        return sqlite3BinaryCompareCollSeq(
            pParse,
            (unsafe { (*p).pLeft }) as *const Expr,
            (unsafe { (*p).pRight }) as *const Expr,
        );
    }
    return unsafe { std::mem::zeroed() };
}

/// Generate code for a comparison operator.
///
/// # Arguments
///
/// * `pParse` - The parsing (and code generating) context
/// * `pLeft` - The left operand
/// * `pRight` - The right operand
/// * `opcode` - The comparison opcode
/// * `in2` - Register holding operands
/// * `dest` - Jump here if true.
/// * `jumpIfNull` - If true, jump if either operand is NULL
/// * `isCommuted` - The comparison has been commuted
fn codeCompare(
    mut pParse: *mut Parse,
    mut pLeft: *mut Expr,
    mut pRight: *mut Expr,
    mut opcode: i32,
    mut in1: i32,
    mut in2: i32,
    mut dest: i32,
    mut jumpIfNull: i32,
    mut isCommuted: i32,
) -> i32 {
    let mut p5: i32 = 0 as i32;
    let mut addr: i32 = 0 as i32;
    let mut p4: *mut CollSeq = unsafe { std::mem::zeroed() };
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        return 0 as i32;
    }
    if isCommuted != (0 as i32) {
        p4 = sqlite3BinaryCompareCollSeq(pParse, pRight as *const Expr, pLeft as *const Expr);
    } else {
        p4 = sqlite3BinaryCompareCollSeq(pParse, pLeft as *const Expr, pRight as *const Expr);
    }
    p5 = (binaryCompareP5(pLeft as *const Expr, pRight as *const Expr, jumpIfNull) as u32) as i32;
    addr = unsafe {
        sqlite3VdbeAddOp4(
            unsafe { (*pParse).pVdbe },
            opcode,
            in2,
            dest,
            in1,
            (p4 as *mut ()) as *const i8,
            -(2 as i32),
        )
    };
    unsafe { sqlite3VdbeChangeP5(unsafe { (*pParse).pVdbe }, (p5 as i16) as u16) };
    return addr;
}

/// Return true if expression pExpr is a vector, or false otherwise.
///
/// A vector is defined as any expression that results in two or more
/// columns of result.  Every TK_VECTOR node is an vector because the
/// parser will not generate a TK_VECTOR with fewer than two entries.
/// But a TK_SELECT might be either a vector or a scalar. It is only
/// considered a vector if it has two or more result columns.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprIsVector(mut pExpr: *const Expr) -> i32 {
    return (sqlite3ExprVectorSize(pExpr) > (1 as i32)) as i32;
}

/// If the expression passed as the only argument is of type TK_VECTOR
/// return the number of expressions in the vector. Or, if the expression
/// is a sub-select, return the number of columns in the sub-select. For
/// any other type of expression, return 1.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprVectorSize(mut pExpr: *const Expr) -> i32 {
    let mut op: u8 = unsafe { (*pExpr).op };
    if ((op as u32) as i32) == (176 as i32) {
        op = unsafe { (*pExpr).op2 };
    }
    if ((op as u32) as i32) == (177 as i32) {
        0 as i32;
        return unsafe { (*unsafe { (*pExpr).x.pList }).nExpr };
    } else {
        if ((op as u32) as i32) == (139 as i32) {
            0 as i32;
            return unsafe { (*unsafe { (*unsafe { (*pExpr).x.pSelect }).pEList }).nExpr };
        } else {
            return 1 as i32;
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Return a pointer to a subexpression of pVector that is the i-th
/// column of the vector (numbered starting with 0).  The caller must
/// ensure that i is within range.
///
/// If pVector is really a scalar (and "scalar" here includes subqueries
/// that return a single column!) then return pVector unmodified.
///
/// pVector retains ownership of the returned subexpression.
///
/// If the vector is a (SELECT ...) then the expression returned is
/// just the expression for the i-th term of the result set, and may
/// not be ready for evaluation because the table cursor has not yet
/// been positioned.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VectorFieldSubexpr(mut pVector: *mut Expr, mut i: i32) -> *mut Expr {
    0 as i32;
    if sqlite3ExprIsVector(pVector as *const Expr) != (0 as i32) {
        0 as i32;
        if (((unsafe { (*pVector).op }) as u32) as i32) == (139 as i32)
            || (((unsafe { (*pVector).op2 }) as u32) as i32) == (139 as i32)
        {
            0 as i32;
            return unsafe {
                (*unsafe {
                    unsafe {
                        std::ptr::addr_of_mut!(
                            (*unsafe { (*unsafe { (*pVector).x.pSelect }).pEList }).a
                        ) as *mut ExprList_item
                    }
                    .offset(i as isize)
                })
                .pExpr
            };
        } else {
            0 as i32;
            return unsafe {
                (*unsafe {
                    unsafe {
                        std::ptr::addr_of_mut!((*unsafe { (*pVector).x.pList }).a)
                            as *mut ExprList_item
                    }
                    .offset(i as isize)
                })
                .pExpr
            };
        }
    }
    return pVector;
}

/// Compute and return a new Expr object which when passed to
/// sqlite3ExprCode() will generate all necessary code to compute
/// the iField-th column of the vector expression pVector.
///
/// It is ok for pVector to be a scalar (as long as iField==0).
/// In that case, this routine works like sqlite3ExprDup().
///
/// The caller owns the returned Expr object and is responsible for
/// ensuring that the returned value eventually gets freed.
///
/// The caller retains ownership of pVector.  If pVector is a TK_SELECT,
/// then the returned object will reference pVector and so pVector must remain
/// valid for the life of the returned object.  If pVector is a TK_VECTOR
/// or a scalar expression, then it can be deleted as soon as this routine
/// returns.
///
/// A trick to cause a TK_SELECT pVector to be deleted together with
/// the returned Expr object is to attach the pVector to the pRight field
/// of the returned TK_SELECT_COLUMN Expr object.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pVector` - The vector.  List of expressions or a sub-SELECT
/// * `iField` - Which column of the vector to return
/// * `nField` - Total number of columns in the vector
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprForVectorField(
    mut pParse: *mut Parse,
    mut pVector: *mut Expr,
    mut iField: i32,
    mut nField: i32,
) -> *mut Expr {
    let mut pRet: *mut Expr = unsafe { std::mem::zeroed() };
    if (((unsafe { (*pVector).op }) as u32) as i32) == (139 as i32) {
        0 as i32;
        // The TK_SELECT_COLUMN Expr node:
        //
        // pLeft:           pVector containing TK_SELECT.  Not deleted.
        // pRight:          not used.  But recursively deleted.
        // iColumn:         Index of a column in pVector
        // iTable:          0 or the number of columns on the LHS of an assignment
        // pLeft->iTable:   First in an array of register holding result, or 0
        //                  if the result is not yet computed.
        //
        // sqlite3ExprDelete() specifically skips the recursive delete of
        // pLeft on TK_SELECT_COLUMN nodes.  But pRight is followed, so pVector
        // can be attached to pRight to cause this node to take ownership of
        // pVector.  Typically there will be multiple TK_SELECT_COLUMN nodes
        // with the same pLeft pointer to the pVector, but only one of them
        // will own the pVector.
        pRet = sqlite3PExpr(
            pParse,
            178 as i32,
            std::ptr::null_mut::<Expr>(),
            std::ptr::null_mut::<Expr>(),
        );
        if pRet != std::ptr::null_mut::<Expr>() {
            let __v2619: *mut Expr = pRet;
            let __v2620: u32 = unsafe { (*__v2619).flags };
            let __v2621: u32 = __v2620 | ((131072 as i32) as u32);
            unsafe {
                (*__v2619).flags = __v2621;
            }
            unsafe {
                (*pRet).iTable = nField;
            }
            unsafe {
                (*pRet).iColumn = iField as i16;
            }
            unsafe {
                (*pRet).pLeft = pVector;
            }
        }
    } else {
        if (((unsafe { (*pVector).op }) as u32) as i32) == (177 as i32) {
            let mut ppVector: *mut *mut Expr = unsafe { std::mem::zeroed() };
            0 as i32;
            ppVector = unsafe {
                std::ptr::addr_of_mut!(
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*pVector).x.pList }).a)
                                as *mut ExprList_item
                        }
                        .offset(iField as isize)
                    })
                    .pExpr
                )
            };
            pVector = unsafe { *ppVector };
            if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
                // This must be a vector UPDATE inside a trigger
                unsafe {
                    *ppVector = std::ptr::null_mut::<Expr>();
                }
                return pVector;
            }
        }
        pRet = sqlite3ExprDup(unsafe { (*pParse).db }, pVector as *const Expr, 0 as i32);
    }
    return pRet;
}

/// If expression pExpr is of type TK_SELECT, generate code to evaluate
/// it. Return the register in which the result is stored (or, if the
/// sub-select returns more than one column, the first in an array
/// of registers in which the result is stored).
///
/// If pExpr is not a TK_SELECT expression, return 0.
fn exprCodeSubselect(mut pParse: *mut Parse, mut pExpr: *mut Expr) -> i32 {
    let mut reg: i32 = 0 as i32;
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (139 as i32) {
        reg = sqlite3CodeSubselect(pParse, pExpr);
    }
    return reg;
}

/// Argument pVector points to a vector expression - either a TK_VECTOR
/// or TK_SELECT that returns more than one column. This function returns
/// the register number of a register that contains the value of
/// element iField of the vector.
///
/// If pVector is a TK_SELECT expression, then code for it must have
/// already been generated using the exprCodeSubselect() routine. In this
/// case parameter regSelect should be the first in an array of registers
/// containing the results of the sub-select.
///
/// If pVector is of type TK_VECTOR, then code for the requested field
/// is generated. In this case (*pRegFree) may be set to the number of
/// a temporary register to be freed by the caller before returning.
///
/// Before returning, output parameter (*ppExpr) is set to point to the
/// Expr object corresponding to element iElem of the vector.
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `pVector` - Vector to extract element from
/// * `iField` - Field to extract from pVector
/// * `regSelect` - First in array of registers
/// * `pTmp` - Temporary space
/// * `ppExpr` - OUT: Expression element
/// * `pRegFree` - OUT: Temp register to free
fn exprVectorRegister(
    mut pParse: *mut Parse,
    mut pVector: *mut Expr,
    mut iField: i32,
    mut regSelect: i32,
    mut pTmp: *mut Expr,
    mut ppExpr: *mut *mut Expr,
    mut pRegFree: *mut i32,
) -> i32 {
    let mut op: u8 = unsafe { (*pVector).op };
    0 as i32;
    if ((op as u32) as i32) == (176 as i32) {
        unsafe {
            *ppExpr = sqlite3VectorFieldSubexpr(pVector, iField);
        }
        return (unsafe { (*pVector).iTable }) + iField;
    }
    if ((op as u32) as i32) == (139 as i32) {
        0 as i32;
        // Use the temporary expression node to wrap expression iField of the
        // sub-select in a TK_SELECT_COLUMN node. This causes the caller to
        // use the affinity of the expression in any comparison, but not the
        // collation sequence.
        unsafe { memset(pTmp as *mut (), 0 as i32, 72 as u64) };
        unsafe {
            (*pTmp).op = ((178 as i32) as i8) as u8;
        }
        unsafe {
            (*pTmp).pLeft = pVector;
        }
        unsafe {
            (*pTmp).iColumn = iField as i16;
        }
        unsafe {
            (*pTmp).iTable =
                unsafe { (*unsafe { (*unsafe { (*pVector).x.pSelect }).pEList }).nExpr };
        }
        unsafe {
            *ppExpr = pTmp;
        }
        return regSelect + iField;
    }
    if ((op as u32) as i32) == (177 as i32) {
        0 as i32;
        unsafe {
            *ppExpr = unsafe {
                (*unsafe {
                    unsafe {
                        std::ptr::addr_of_mut!((*unsafe { (*pVector).x.pList }).a)
                            as *mut ExprList_item
                    }
                    .offset(iField as isize)
                })
                .pExpr
            };
        }
        return sqlite3ExprCodeTemp(pParse, unsafe { *ppExpr }, pRegFree);
    }
    return 0 as i32;
}

/// Expression pExpr is a comparison between two vector values. Compute
/// the result of the comparison (1, 0, or NULL) and write that
/// result into register dest.
///
/// The caller must satisfy the following preconditions:
///
///    if pExpr->op==TK_IS:      op==TK_EQ and p5==SQLITE_NULLEQ
///    if pExpr->op==TK_ISNOT:   op==TK_NE and p5==SQLITE_NULLEQ
///    otherwise:                op==pExpr->op and p5==0
///
/// # Arguments
///
/// * `pParse` - Code generator context
/// * `pExpr` - The comparison operation
/// * `dest` - Write results into this register
/// * `op` - Comparison operator
/// * `p5` - SQLITE_NULLEQ or zero
fn codeVectorCompare(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut dest: i32,
    mut op: u8,
    mut p5: u8,
) {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut pLeft: *mut Expr = unsafe { (*pExpr).pLeft };
    let mut pRight: *mut Expr = unsafe { (*pExpr).pRight };
    let mut nLeft: i32 = sqlite3ExprVectorSize(pLeft as *const Expr);
    let mut i: i32 = 0 as i32;
    let mut regLeft: i32 = 0 as i32;
    let mut regRight: i32 = 0 as i32;
    let mut opx: u8 = op;
    let mut addrCmp: i32 = 0 as i32;
    let mut addrDone: i32 = unsafe { sqlite3VdbeMakeLabel(pParse) };
    let mut isCommuted: i32 =
        ((unsafe { (*pExpr).flags }) & ((1024 as i32) as u32) != ((0 as i32) as u32)) as i32;
    0 as i32;
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        return;
    }
    if nLeft != sqlite3ExprVectorSize(pRight as *const Expr) {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"row value misused\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        return;
    }
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if ((op as u32) as i32) == (56 as i32) {
        opx = ((57 as i32) as i8) as u8;
    }
    if ((op as u32) as i32) == (58 as i32) {
        opx = ((55 as i32) as i8) as u8;
    }
    if ((op as u32) as i32) == (53 as i32) {
        opx = ((54 as i32) as i8) as u8;
    }
    regLeft = exprCodeSubselect(pParse, pLeft);
    regRight = exprCodeSubselect(pParse, pRight);
    unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 1 as i32, dest) };
    i = 0 as i32;
    '__slate_break_2221: loop {
        if !((1 as i32) != (0 as i32)) {
            break;
        }
        // Loop exits by "break"
        let mut regFree1: i32 = 0 as i32;
        let mut regFree2: i32 = 0 as i32;
        let mut pL: *mut Expr = std::ptr::null_mut::<Expr>();
        let mut pR: *mut Expr = std::ptr::null_mut::<Expr>();
        let mut tmp1: Expr = unsafe { std::mem::zeroed() };
        let mut tmp2: Expr = unsafe { std::mem::zeroed() };
        let mut r1: i32 = 0 as i32;
        let mut r2: i32 = 0 as i32;
        0 as i32;
        if addrCmp != (0 as i32) {
            unsafe { sqlite3VdbeJumpHere(v, addrCmp) };
        }
        r1 = exprVectorRegister(
            pParse,
            pLeft,
            i,
            regLeft,
            std::ptr::addr_of_mut!(tmp1),
            std::ptr::addr_of_mut!(pL),
            std::ptr::addr_of_mut!(regFree1),
        );
        r2 = exprVectorRegister(
            pParse,
            pRight,
            i,
            regRight,
            std::ptr::addr_of_mut!(tmp2),
            std::ptr::addr_of_mut!(pR),
            std::ptr::addr_of_mut!(regFree2),
        );
        addrCmp = unsafe { sqlite3VdbeCurrentAddr(v) };
        codeCompare(
            pParse,
            pL,
            pR,
            (opx as u32) as i32,
            r1,
            r2,
            addrDone,
            (p5 as u32) as i32,
            isCommuted,
        );
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        sqlite3ReleaseTempReg(pParse, regFree1);
        sqlite3ReleaseTempReg(pParse, regFree2);
        if (((opx as u32) as i32) == (57 as i32) || ((opx as u32) as i32) == (55 as i32))
            && i < nLeft - (1 as i32)
        {
            addrCmp = unsafe { sqlite3VdbeAddOp0(v, 59 as i32) };
            {}
            {}
            {}
            {}
        }
        if ((p5 as u32) as i32) == (128 as i32) {
            unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, dest) };
        } else {
            unsafe { sqlite3VdbeAddOp3(v, 94 as i32, r1, dest, r2) };
        }
        if i == nLeft - (1 as i32) {
            break '__slate_break_2221;
        }
        if ((opx as u32) as i32) == (54 as i32) {
            unsafe { sqlite3VdbeAddOp2(v, 52 as i32, dest, addrDone) };
            {}
        } else {
            0 as i32;
            unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 0 as i32, addrDone) };
            if i == nLeft - (2 as i32) {
                opx = op;
            }
        }
        let __v2630: i32 = i;
        let __v2631: i32 = __v2630 + (1 as i32);
        i = __v2631;
    }
    unsafe { sqlite3VdbeJumpHere(v, addrCmp) };
    unsafe { sqlite3VdbeResolveLabel(v, addrDone) };
    if ((op as u32) as i32) == (53 as i32) {
        unsafe { sqlite3VdbeAddOp2(v, 19 as i32, dest, dest) };
    }
}

/// Check that argument nHeight is less than or equal to the maximum
/// expression depth allowed. If it is not, leave an error message in
/// pParse.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCheckHeight(mut pParse: *mut Parse, mut nHeight: i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut mxHeight: i32 = unsafe {
        *unsafe {
            unsafe { (*unsafe { (*pParse).db }).aLimit.as_mut_ptr() as *mut i32 }
                .offset((3 as i32) as isize)
        }
    };
    if nHeight > mxHeight {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"Expression tree is too large (maximum depth %d)\0".as_ptr() as *mut i8)
                    as *const i8,
                mxHeight,
            )
        };
        rc = 1 as i32;
    }
    return rc;
}

/// The following three functions, heightOfExpr(), heightOfExprList()
/// and heightOfSelect(), are used to determine the maximum height
/// of any expression tree referenced by the structure passed as the
/// first argument.
///
/// If this maximum height is greater than the current value pointed
/// to by pnHeight, the second parameter, then set *pnHeight to that
/// value.
fn heightOfExpr(mut p: *const Expr, mut pnHeight: *mut i32) {
    if p != std::ptr::null::<Expr>() {
        if (unsafe { (*p).nHeight }) > unsafe { *pnHeight } {
            unsafe {
                *pnHeight = unsafe { (*p).nHeight };
            }
        }
    }
}

fn heightOfExprList(mut p: *const ExprList, mut pnHeight: *mut i32) {
    if p != std::ptr::null::<ExprList>() {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_2223: loop {
            if !(i < unsafe { (*p).nExpr }) {
                break;
            }
            heightOfExpr(
                (unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of!((*p).a) as *const ExprList_item }
                            .offset(i as isize)
                    })
                    .pExpr
                }) as *const Expr,
                pnHeight,
            );
            let __v2632: i32 = i;
            let __v2633: i32 = __v2632 + (1 as i32);
            i = __v2633;
        }
    }
}

fn heightOfSelect(mut pSelect: *const Select, mut pnHeight: *mut i32) {
    let mut p: *const Select = unsafe { std::mem::zeroed() };
    p = pSelect;
    '__slate_break_2224: while p != std::ptr::null::<Select>() {
        heightOfExpr((unsafe { (*p).pWhere }) as *const Expr, pnHeight);
        heightOfExpr((unsafe { (*p).pHaving }) as *const Expr, pnHeight);
        heightOfExpr((unsafe { (*p).pLimit }) as *const Expr, pnHeight);
        heightOfExprList((unsafe { (*p).pEList }) as *const ExprList, pnHeight);
        heightOfExprList((unsafe { (*p).pGroupBy }) as *const ExprList, pnHeight);
        heightOfExprList((unsafe { (*p).pOrderBy }) as *const ExprList, pnHeight);
        p = (unsafe { (*p).pPrior }) as *const Select;
    }
}

/// Set the Expr.nHeight variable in the structure passed as an
/// argument. An expression with no children, Expr.pList or
/// Expr.pSelect member has a height of 1. Any other expression
/// has a height equal to the maximum height of any other
/// referenced Expr plus one.
///
/// Also propagate EP_Propagate flags up from Expr.x.pList to Expr.flags,
/// if appropriate.
fn exprSetHeight(mut p: *mut Expr) {
    let mut nHeight: i32 = if (unsafe { (*p).pLeft }) != std::ptr::null_mut::<Expr>() {
        unsafe { (*unsafe { (*p).pLeft }).nHeight }
    } else {
        0 as i32
    };
    if (unsafe { (*p).pRight }) != std::ptr::null_mut::<Expr>()
        && (unsafe { (*unsafe { (*p).pRight }).nHeight }) > nHeight
    {
        nHeight = unsafe { (*unsafe { (*p).pRight }).nHeight };
    }
    if (unsafe { (*p).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
        heightOfSelect(
            (unsafe { (*p).x.pSelect }) as *const Select,
            std::ptr::addr_of_mut!(nHeight),
        );
    } else {
        if (unsafe { (*p).x.pList }) != std::ptr::null_mut::<ExprList>() {
            heightOfExprList(
                (unsafe { (*p).x.pList }) as *const ExprList,
                std::ptr::addr_of_mut!(nHeight),
            );
            let __v2634: *mut Expr = p;
            let __v2635: u32 = unsafe { (*__v2634).flags };
            let __v2636: u32 = __v2635
                | (((512 as i32) | (4194304 as i32) | (8 as i32)) as u32)
                    & sqlite3ExprListFlags((unsafe { (*p).x.pList }) as *const ExprList);
            unsafe {
                (*__v2634).flags = __v2636;
            }
        }
    }
    unsafe {
        (*p).nHeight = nHeight + (1 as i32);
    }
}

/// Set the Expr.nHeight variable using the exprSetHeight() function. If
/// the height is greater than the maximum allowed expression depth,
/// leave an error in pParse.
///
/// Also propagate all EP_Propagate flags from the Expr.x.pList into
/// Expr.flags.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprSetHeightAndFlags(mut pParse: *mut Parse, mut p: *mut Expr) {
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        return;
    }
    exprSetHeight(p);
    sqlite3ExprCheckHeight(pParse, unsafe { (*p).nHeight });
}

/// Return the maximum height of any expression tree referenced
/// by the select statement passed as an argument.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SelectExprHeight(mut p: *const Select) -> i32 {
    let mut nHeight: i32 = 0 as i32;
    heightOfSelect(p, std::ptr::addr_of_mut!(nHeight));
    return nHeight;
}

/// Set the error offset for an Expr node, if possible.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprSetErrorOffset(mut pExpr: *mut Expr, mut iOfst: i32) {
    if pExpr == std::ptr::null_mut::<Expr>() {
        return;
    }
    if (unsafe { (*pExpr).flags }) & (((2 as i32) | (1 as i32)) as u32) != ((0 as i32) as u32) {
        return;
    }
    unsafe {
        (*pExpr).w.iOfst = iOfst;
    }
}

/// This routine is the core allocator for Expr nodes.
///
/// Construct a new expression node and return a pointer to it.  Memory
/// for this node and for the pToken argument is a single allocation
/// obtained from sqlite3DbMalloc().  The calling function
/// is responsible for making sure the node eventually gets freed.
///
/// If dequote is true, then the token (if it exists) is dequoted.
/// If dequote is false, no dequoting is performed.  The deQuote
/// parameter is ignored if pToken is NULL or if the token does not
/// appear to be quoted.  If the quotes were of the form "..." (double-quotes)
/// then the EP_DblQuoted flag is set on the expression node.
///
/// Special case (tag-20240227-a):  If op==TK_INTEGER and pToken points to
/// a string that can be translated into a 32-bit integer, then the token is
/// not stored in u.zToken.  Instead, the integer values is written
/// into u.iValue and the EP_IntValue flag is set. No extra storage
/// is allocated to hold the integer text and the dequote flag is ignored.
/// See also tag-20240227-b.
///
/// # Arguments
///
/// * `db` - Handle for sqlite3DbMallocRawNN()
/// * `op` - Expression opcode
/// * `pToken` - Token argument.  Might be NULL
/// * `dequote` - True to dequote
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprAlloc(
    mut db: *mut sqlite3,
    mut op: i32,
    mut pToken: *const Token,
    mut dequote: i32,
) -> *mut Expr {
    let mut pNew: *mut Expr = unsafe { std::mem::zeroed() };
    let mut nExtra: i32 = (if pToken != std::ptr::null::<Token>() {
        unsafe { (*pToken).n }.wrapping_add((1 as i32) as u32)
    } else {
        (0 as i32) as u32
    }) as i32;
    0 as i32;
    pNew = (unsafe { sqlite3DbMallocRawNN(db, (72 as u64).wrapping_add((nExtra as i64) as u64)) })
        as *mut Expr;
    if pNew != std::ptr::null_mut::<Expr>() {
        unsafe { memset(pNew as *mut (), 0 as i32, 72 as u64) };
        unsafe {
            (*pNew).op = (op as i8) as u8;
        }
        unsafe {
            (*pNew).iAgg = -(1 as i32) as i16;
        }
        if nExtra != (0 as i32) {
            0 as i32;
            unsafe {
                (*pNew).u.zToken = (unsafe { pNew.offset((1 as i32) as isize) }) as *mut i8;
            }
            0 as i32;
            if (unsafe { (*pToken).n }) != (0 as u32) {
                unsafe {
                    memcpy(
                        (unsafe { (*pNew).u.zToken }) as *mut (),
                        (unsafe { (*pToken).z }) as *const (),
                        (unsafe { (*pToken).n }) as u64,
                    )
                };
            }
            unsafe {
                *unsafe { unsafe { (*pNew).u.zToken }.offset((unsafe { (*pToken).n }) as isize) } =
                    (0 as i32) as i8;
            }
            if dequote != (0 as i32)
                && (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                            ((((unsafe {
                                *unsafe { unsafe { (*pNew).u.zToken }.offset((0 as i32) as isize) }
                            }) as u8) as u32) as i32) as isize,
                        )
                    }
                }) as u32) as i32)
                    & (128 as i32)
                    != (0 as i32)
            {
                unsafe { sqlite3DequoteExpr(pNew) };
            }
        }
        unsafe {
            (*pNew).nHeight = 1 as i32;
        }
    }
    return pNew;
}

/// Allocate a new expression node from a zero-terminated token that has
/// already been dequoted.
///
/// # Arguments
///
/// * `db` - Handle for sqlite3DbMallocZero() (may be null)
/// * `op` - Expression opcode
/// * `zToken` - Token argument.  Might be NULL
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Expr(
    mut db: *mut sqlite3,
    mut op: i32,
    mut zToken: *const i8,
) -> *mut Expr {
    let mut x: Token = unsafe { std::mem::zeroed() };
    x.z = zToken;
    x.n = (unsafe { sqlite3Strlen30(zToken) }) as u32;
    return sqlite3ExprAlloc(db, op, std::ptr::addr_of_mut!(x) as *const Token, 0 as i32);
}

/// Allocate an expression for a 32-bit signed integer literal.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprInt32(mut db: *mut sqlite3, mut iVal: i32) -> *mut Expr {
    let mut pNew: *mut Expr = (unsafe { sqlite3DbMallocRawNN(db, 72 as u64) }) as *mut Expr;
    if pNew != std::ptr::null_mut::<Expr>() {
        unsafe { memset(pNew as *mut (), 0 as i32, 72 as u64) };
        unsafe {
            (*pNew).op = ((156 as i32) as i8) as u8;
        }
        unsafe {
            (*pNew).iAgg = -(1 as i32) as i16;
        }
        unsafe {
            (*pNew).flags = ((2048 as i32)
                | (8388608 as i32)
                | if iVal != (0 as i32) {
                    268435456 as i32
                } else {
                    536870912 as i32
                }) as u32;
        }
        unsafe {
            (*pNew).u.iValue = iVal;
        }
        unsafe {
            (*pNew).nHeight = 1 as i32;
        }
    }
    return pNew;
}

/// Attach subtrees pLeft and pRight to the Expr node pRoot.
///
/// If pRoot==NULL that means that a memory allocation error has occurred.
/// In that case, delete the subtrees pLeft and pRight.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprAttachSubtrees(
    mut db: *mut sqlite3,
    mut pRoot: *mut Expr,
    mut pLeft: *mut Expr,
    mut pRight: *mut Expr,
) {
    if pRoot == std::ptr::null_mut::<Expr>() {
        0 as i32;
        sqlite3ExprDelete(db, pLeft);
        sqlite3ExprDelete(db, pRight);
    } else {
        0 as i32;
        0 as i32;
        if pRight != std::ptr::null_mut::<Expr>() {
            unsafe {
                (*pRoot).pRight = pRight;
            }
            let __v2383: *mut Expr = pRoot;
            let __v2384: u32 = unsafe { (*__v2383).flags };
            let __v2385: u32 = __v2384
                | (((512 as i32) | (4194304 as i32) | (8 as i32)) as u32)
                    & unsafe { (*pRight).flags };
            unsafe {
                (*__v2383).flags = __v2385;
            }
            unsafe {
                (*pRoot).nHeight = (unsafe { (*pRight).nHeight }) + (1 as i32);
            }
        } else {
            unsafe {
                (*pRoot).nHeight = 1 as i32;
            }
        }
        if pLeft != std::ptr::null_mut::<Expr>() {
            unsafe {
                (*pRoot).pLeft = pLeft;
            }
            let __v2386: *mut Expr = pRoot;
            let __v2387: u32 = unsafe { (*__v2386).flags };
            let __v2388: u32 = __v2387
                | (((512 as i32) | (4194304 as i32) | (8 as i32)) as u32)
                    & unsafe { (*pLeft).flags };
            unsafe {
                (*__v2386).flags = __v2388;
            }
            if (unsafe { (*pLeft).nHeight }) >= unsafe { (*pRoot).nHeight } {
                unsafe {
                    (*pRoot).nHeight = (unsafe { (*pLeft).nHeight }) + (1 as i32);
                }
            }
        }
    }
}

/// Allocate an Expr node which joins as many as two subtrees.
///
/// One or both of the subtrees can be NULL.  Return a pointer to the new
/// Expr node.  Or, if an OOM error occurs, set pParse->db->mallocFailed,
/// free the subtrees and return NULL.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `op` - Expression opcode
/// * `pLeft` - Left operand
/// * `pRight` - Right operand
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PExpr(
    mut pParse: *mut Parse,
    mut op: i32,
    mut pLeft: *mut Expr,
    mut pRight: *mut Expr,
) -> *mut Expr {
    let mut p: *mut Expr = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3DbMallocRawNN(unsafe { (*pParse).db }, 72 as u64) }) as *mut Expr;
    if p != std::ptr::null_mut::<Expr>() {
        unsafe { memset(p as *mut (), 0 as i32, 72 as u64) };
        unsafe {
            (*p).op = ((op & (255 as i32)) as i8) as u8;
        }
        unsafe {
            (*p).iAgg = -(1 as i32) as i16;
        }
        sqlite3ExprAttachSubtrees(unsafe { (*pParse).db }, p, pLeft, pRight);
        sqlite3ExprCheckHeight(pParse, unsafe { (*p).nHeight });
    } else {
        sqlite3ExprDelete(unsafe { (*pParse).db }, pLeft);
        sqlite3ExprDelete(unsafe { (*pParse).db }, pRight);
    }
    return p;
}

/// Add pSelect to the Expr.x.pSelect field.  Or, if pExpr is NULL (due
/// do a memory allocation failure) then delete the pSelect object.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PExprAddSelect(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut pSelect: *mut Select,
) {
    if pExpr != std::ptr::null_mut::<Expr>() {
        unsafe {
            (*pExpr).x.pSelect = pSelect;
        }
        let __v2389: *mut Expr = pExpr;
        let __v2390: u32 = unsafe { (*__v2389).flags };
        let __v2391: u32 = __v2390 | (((4096 as i32) | (4194304 as i32)) as u32);
        unsafe {
            (*__v2389).flags = __v2391;
        }
        sqlite3ExprSetHeightAndFlags(pParse, pExpr);
    } else {
        0 as i32;
        unsafe { sqlite3SelectDelete(unsafe { (*pParse).db }, pSelect) };
    }
}

/// Expression list pEList is a list of vector values. This function
/// converts the contents of pEList to a VALUES(...) Select statement
/// returning 1 row for each element of the list. For example, the
/// expression list:
///
///   ( (1,2), (3,4) (5,6) )
///
/// is translated to the equivalent of:
///
///   VALUES(1,2), (3,4), (5,6)
///
/// Each of the vector values in pEList must contain exactly nElem terms.
/// If a list element that is not a vector or does not contain nElem terms,
/// an error message is left in pParse.
///
/// This is used as part of processing IN(...) expressions with a list
/// of vectors on the RHS. e.g. "... IN ((1,2), (3,4), (5,6))".
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprListToValues(
    mut pParse: *mut Parse,
    mut nElem: i32,
    mut pEList: *mut ExprList,
) -> *mut Select {
    let mut ii: i32 = 0 as i32;
    let mut pRet: *mut Select = std::ptr::null_mut::<Select>();
    0 as i32;
    ii = 0 as i32;
    '__slate_break_2225: loop {
        if !(ii < unsafe { (*pEList).nExpr }) {
            break;
        }
        let mut pSel: *mut Select = unsafe { std::mem::zeroed() };
        let mut pExpr: *mut Expr = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                    .offset(ii as isize)
            })
            .pExpr
        };
        let mut nExprElem: i32 = 0 as i32;
        if (((unsafe { (*pExpr).op }) as u32) as i32) == (177 as i32) {
            0 as i32;
            nExprElem = unsafe { (*unsafe { (*pExpr).x.pList }).nExpr };
        } else {
            nExprElem = 1 as i32;
        }
        if nExprElem != nElem {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"IN(...) element has %d term%s - expected %d\0".as_ptr() as *mut i8)
                        as *const i8,
                    nExprElem,
                    if nExprElem > (1 as i32) {
                        b"s\0".as_ptr() as *mut i8
                    } else {
                        b"\0".as_ptr() as *mut i8
                    },
                    nElem,
                )
            };
            break '__slate_break_2225;
        }
        0 as i32;
        pSel = unsafe {
            sqlite3SelectNew(
                pParse,
                unsafe { (*pExpr).x.pList },
                std::ptr::null_mut::<SrcList>(),
                std::ptr::null_mut::<Expr>(),
                std::ptr::null_mut::<ExprList>(),
                std::ptr::null_mut::<Expr>(),
                std::ptr::null_mut::<ExprList>(),
                (512 as i32) as u32,
                std::ptr::null_mut::<Expr>(),
            )
        };
        unsafe {
            (*pExpr).x.pList = std::ptr::null_mut::<ExprList>();
        }
        if pSel != std::ptr::null_mut::<Select>() {
            if pRet != std::ptr::null_mut::<Select>() {
                unsafe {
                    (*pSel).op = ((136 as i32) as i8) as u8;
                }
                unsafe {
                    (*pSel).pPrior = pRet;
                }
            }
            pRet = pSel;
        }
        let __v2417: i32 = ii;
        let __v2418: i32 = __v2417 + (1 as i32);
        ii = __v2418;
    }
    if pRet != std::ptr::null_mut::<Select>()
        && (unsafe { (*pRet).pPrior }) != std::ptr::null_mut::<Select>()
    {
        let __v2419: *mut Select = pRet;
        let __v2420: u32 = unsafe { (*__v2419).selFlags };
        let __v2421: u32 = __v2420 | ((1024 as i32) as u32);
        unsafe {
            (*__v2419).selFlags = __v2421;
        }
    }
    sqlite3ExprListDelete(unsafe { (*pParse).db }, pEList);
    return pRet;
}

/// Join two expressions using an AND operator.  If either expression is
/// NULL, then just return the other expression.
///
/// If one side or the other of the AND is known to be false, and neither side
/// is part of an ON clause, then instead of returning an AND expression,
/// just return a constant expression with a value of false.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprAnd(
    mut pParse: *mut Parse,
    mut pLeft: *mut Expr,
    mut pRight: *mut Expr,
) -> *mut Expr {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    if pLeft == std::ptr::null_mut::<Expr>() {
        return pRight;
    } else {
        if pRight == std::ptr::null_mut::<Expr>() {
            return pLeft;
        } else {
            let mut f: u32 = (unsafe { (*pLeft).flags }) | unsafe { (*pRight).flags };
            if f & (((1 as i32) | (2 as i32) | (536870912 as i32) | (8 as i32)) as u32)
                == ((536870912 as i32) as u32)
                && !((((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32))
            {
                sqlite3ExprDeferredDelete(pParse, pLeft);
                sqlite3ExprDeferredDelete(pParse, pRight);
                return sqlite3ExprInt32(db, 0 as i32);
            } else {
                return sqlite3PExpr(pParse, 44 as i32, pLeft, pRight);
            }
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Construct a new expression node for a function with multiple
/// arguments.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pList` - Argument list
/// * `pToken` - Name of the function
/// * `eDistinct` - SF_Distinct or SF_ALL or 0
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprFunction(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut pToken: *const Token,
    mut eDistinct: i32,
) -> *mut Expr {
    let mut pNew: *mut Expr = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    0 as i32;
    pNew = sqlite3ExprAlloc(db, 172 as i32, pToken, 1 as i32);
    if pNew == std::ptr::null_mut::<Expr>() {
        sqlite3ExprListDelete(db, pList); // Avoid memory leak when malloc fails
        return std::ptr::null_mut::<Expr>();
    }
    0 as i32;
    unsafe {
        (*pNew).w.iOfst = ((unsafe {
            unsafe { (*pToken).z }.offset_from((unsafe { (*pParse).zTail }) as *const i8)
        }) as i64) as i32;
    }
    if pList != std::ptr::null_mut::<ExprList>()
        && (unsafe { (*pList).nExpr })
            > unsafe {
                *unsafe {
                    unsafe { (*unsafe { (*pParse).db }).aLimit.as_mut_ptr() as *mut i32 }
                        .offset((6 as i32) as isize)
                }
            }
        && !((unsafe { (*pParse).nested }) != (0 as u8))
    {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"too many arguments on function %T\0".as_ptr() as *mut i8) as *const i8,
                pToken,
            )
        };
    }
    unsafe {
        (*pNew).x.pList = pList;
    }
    let __v2392: *mut Expr = pNew;
    let __v2393: u32 = unsafe { (*__v2392).flags };
    let __v2394: u32 = __v2393 | ((8 as i32) as u32);
    unsafe {
        (*__v2392).flags = __v2394;
    }
    0 as i32;
    sqlite3ExprSetHeightAndFlags(pParse, pNew);
    if eDistinct == (1 as i32) {
        let __v2395: *mut Expr = pNew;
        let __v2396: u32 = unsafe { (*__v2395).flags };
        let __v2397: u32 = __v2396 | ((4 as i32) as u32);
        unsafe {
            (*__v2395).flags = __v2397;
        }
    }
    return pNew;
}

/// Report an error when attempting to use an ORDER BY clause within
/// the arguments of a non-aggregate function.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprOrderByAggregateError(mut pParse: *mut Parse, mut p: *mut Expr) {
    unsafe {
        sqlite3ErrorMsg(
            pParse,
            (b"ORDER BY may not be used with non-aggregate %#T()\0".as_ptr() as *mut i8)
                as *const i8,
            p,
        )
    };
}

/// Attach an ORDER BY clause to a function call.
///
///     functionname( arguments ORDER BY sortlist )
///     \_____________________/          \______/
///             pExpr                    pOrderBy
///
/// The ORDER BY clause is inserted into a new Expr node of type TK_ORDER
/// and added to the Expr.pLeft field of the parent TK_FUNCTION node.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pExpr` - The function call to which ORDER BY is to be added
/// * `pOrderBy` - The ORDER BY clause to add
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprAddFunctionOrderBy(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut pOrderBy: *mut ExprList,
) {
    let mut pOB: *mut Expr = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    if pOrderBy == std::ptr::null_mut::<ExprList>() {
        0 as i32;
        return;
    }
    if pExpr == std::ptr::null_mut::<Expr>() {
        0 as i32;
        sqlite3ExprListDelete(db, pOrderBy);
        return;
    }
    0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe { (*pExpr).x.pList }) == std::ptr::null_mut::<ExprList>()
        || (unsafe { (*unsafe { (*pExpr).x.pList }).nExpr }) == (0 as i32)
    {
        // Ignore ORDER BY on zero-argument aggregates
        unsafe {
            sqlite3ParserAddCleanup(
                pParse,
                Some(sqlite3ExprListDeleteGeneric),
                pOrderBy as *mut (),
            )
        };
        return;
    }
    if (unsafe { (*pExpr).flags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32)
        && (((unsafe { (*unsafe { (*pExpr).y.pWin }).eFrmType }) as u32) as i32) != (167 as i32)
    {
        sqlite3ExprOrderByAggregateError(pParse, pExpr);
        sqlite3ExprListDelete(db, pOrderBy);
        return;
    }
    if (unsafe { (*pOrderBy).nExpr })
        > unsafe {
            *unsafe { unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((2 as i32) as isize) }
        }
    {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"too many terms in ORDER BY clause\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        sqlite3ExprListDelete(db, pOrderBy);
        return;
    }
    pOB = sqlite3ExprAlloc(db, 146 as i32, std::ptr::null::<Token>(), 0 as i32);
    if pOB == std::ptr::null_mut::<Expr>() {
        sqlite3ExprListDelete(db, pOrderBy);
        return;
    }
    unsafe {
        (*pOB).x.pList = pOrderBy;
    }
    0 as i32;
    unsafe {
        (*pExpr).pLeft = pOB;
    }
    let __v2398: *mut Expr = pOB;
    let __v2399: u32 = unsafe { (*__v2398).flags };
    let __v2400: u32 = __v2399 | ((131072 as i32) as u32);
    unsafe {
        (*__v2398).flags = __v2400;
    }
}

/// Check to see if a function is usable according to current access
/// rules:
///
///    SQLITE_FUNC_DIRECT    -     Only usable from top-level SQL
///
///    SQLITE_FUNC_UNSAFE    -     Usable if TRUSTED_SCHEMA or from
///                                top-level SQL
///
/// If the function is not usable, create an error.
///
/// # Arguments
///
/// * `pParse` - Parsing and code generating context
/// * `pExpr` - The function invocation
/// * `pDef` - The function being invoked
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprFunctionUsable(
    mut pParse: *mut Parse,
    mut pExpr: *const Expr,
    mut pDef: *const FuncDef,
) {
    0 as i32;
    0 as i32;
    if (unsafe { (*pExpr).flags }) & ((1073741824 as i32) as u32) != ((0 as i32) as u32)
        || (((unsafe { (*pParse).prepFlags }) as u32) as i32) & (32 as i32) != (0 as i32)
    {
        if (unsafe { (*pDef).funcFlags }) & ((524288 as i32) as u32) != ((0 as i32) as u32)
            || (unsafe { (*unsafe { (*pParse).db }).flags }) & (((128 as i32) as i64) as u64)
                == (((0 as i32) as i64) as u64)
        {
            // Functions prohibited in triggers and views if:
            // (1) tagged with SQLITE_DIRECTONLY
            // (2) not tagged with SQLITE_INNOCUOUS (which means it
            //     is tagged with SQLITE_FUNC_UNSAFE) and
            //     SQLITE_DBCONFIG_TRUSTED_SCHEMA is off (meaning
            //     that the schema is possibly tainted).
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"unsafe use of %#T()\0".as_ptr() as *mut i8) as *const i8,
                    pExpr,
                )
            };
        }
    }
}

/// Assign a variable number to an expression that encodes a wildcard
/// in the original SQL statement.
///
/// Wildcards consisting of a single "?" are assigned the next sequential
/// variable number.
///
/// Wildcards of the form "?nnn" are assigned the number "nnn".  We make
/// sure "nnn" is not too big to avoid a denial of service attack when
/// the SQL statement comes from an external source.
///
/// Wildcards of the form ":aaa", "@aaa", or "$aaa" are assigned the same number
/// as the previous instance of the same wildcard.  Or if this is the first
/// instance of the wildcard, the next sequential variable number is
/// assigned.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprAssignVarNumber(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut n: u32,
) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut z: *const i8 = unsafe { std::mem::zeroed() };
    let mut x: i16 = 0 as i16;
    if pExpr == std::ptr::null_mut::<Expr>() {
        return;
    }
    0 as i32;
    z = (unsafe { (*pExpr).u.zToken }) as *const i8;
    0 as i32;
    0 as i32;
    0 as i32;
    if ((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as i32) == (0 as i32) {
        // Wildcard of the form "?".  Assign the next variable number
        0 as i32;
        let __v2401: *mut Parse = pParse;
        let __v2402: i16 = unsafe { (*__v2401).nVar };
        let __v2403: i16 = ((__v2402 as i32) + (1 as i32)) as i16;
        unsafe {
            (*__v2401).nVar = __v2403;
        }
        x = __v2403;
    } else {
        let mut doAdd: i32 = 0 as i32;
        if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (63 as i32) {
            // Wildcard of the form "?nnn".  Convert "nnn" to an integer and
            // use it as the variable number
            let mut i: i64 = 0 as i64;
            let mut bOk: i32 = 0 as i32;
            if n == ((2 as i32) as u32) {
                i = (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as i32) - (48 as i32))
                    as i64; // The common case of ?N for a single digit N
                bOk = 1 as i32;
            } else {
                bOk = ((0 as i32)
                    == unsafe {
                        sqlite3Atoi64(
                            unsafe { z.offset((1 as i32) as isize) },
                            std::ptr::addr_of_mut!(i),
                            n.wrapping_sub((1 as i32) as u32) as i32,
                            ((1 as i32) as i8) as u8,
                        )
                    }) as i32;
            }
            {}
            {}
            {}
            {}
            if bOk == (0 as i32)
                || i < ((1 as i32) as i64)
                || i > ((unsafe {
                    *unsafe {
                        unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((9 as i32) as isize)
                    }
                }) as i64)
            {
                unsafe {
                    sqlite3ErrorMsg(
                        pParse,
                        (b"variable number must be between ?1 and ?%d\0".as_ptr() as *mut i8)
                            as *const i8,
                        unsafe {
                            *unsafe {
                                unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }
                                    .offset((9 as i32) as isize)
                            }
                        },
                    )
                };
                unsafe {
                    sqlite3RecordErrorOffsetOfExpr(unsafe { (*pParse).db }, pExpr as *const Expr)
                };
                return;
            }
            x = i as i16;
            if (x as i32) > ((unsafe { (*pParse).nVar }) as i32) {
                unsafe {
                    (*pParse).nVar = (x as i32) as i16;
                }
                doAdd = 1 as i32;
            } else {
                if (x as i32) < (2 as i32) * (64 as i32) {
                    doAdd = ((unsafe {
                        *unsafe {
                            unsafe { (*pParse).aVnbmc.as_mut_ptr() as *mut u64 }
                                .offset(((x as i32) >> (6 as i32)) as isize)
                        }
                    }) & (((1 as i32) as i64) as u64) << ((x as i32) & (63 as i32))
                        == (((0 as i32) as i64) as u64)) as i32;
                } else {
                    if (unsafe { sqlite3VListNumToName(unsafe { (*pParse).pVList }, x as i32) })
                        == std::ptr::null::<i8>()
                    {
                        doAdd = 1 as i32;
                    }
                }
            }
        } else {
            // Wildcards like ":aaa", "$aaa" or "@aaa".  Reuse the same variable
            // number as the prior appearance of the same name, or if the name
            // has never appeared before, reuse the same variable number
            x = (unsafe { sqlite3VListNameToNum(unsafe { (*pParse).pVList }, z, n as i32) }) as i16;
            if (x as i32) == (0 as i32) {
                let __v2404: *mut Parse = pParse;
                let __v2405: i16 = unsafe { (*__v2404).nVar };
                let __v2406: i16 = ((__v2405 as i32) + (1 as i32)) as i16;
                unsafe {
                    (*__v2404).nVar = __v2406;
                }
                x = __v2406;
                doAdd = 1 as i32;
            }
        }
        if doAdd != (0 as i32) {
            unsafe {
                (*pParse).pVList = unsafe {
                    sqlite3VListAdd(db, unsafe { (*pParse).pVList }, z, n as i32, x as i32)
                };
            }
            if (unsafe { (*pParse).pVList }) != std::ptr::null_mut::<i32>()
                && (x as i32) < (2 as i32) * (64 as i32)
            {
                let __v2407: *mut u64 = unsafe {
                    unsafe { (*pParse).aVnbmc.as_mut_ptr() as *mut u64 }
                        .offset(((x as i32) >> (6 as i32)) as isize)
                };
                let __v2408: u64 = unsafe { *__v2407 };
                let __v2409: u64 =
                    __v2408 | (((1 as i32) as i64) as u64) << ((x as i32) & (63 as i32));
                unsafe {
                    *__v2407 = __v2409;
                }
            }
        }
    }
    unsafe {
        (*pExpr).iColumn = x;
    }
    if (x as i32)
        > unsafe {
            *unsafe { unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((9 as i32) as isize) }
        }
    {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"too many SQL variables\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        unsafe { sqlite3RecordErrorOffsetOfExpr(unsafe { (*pParse).db }, pExpr as *const Expr) };
    }
}

/// Recursively delete an expression tree.
fn sqlite3ExprDeleteNN(mut db: *mut sqlite3, mut p: *mut Expr) {
    let mut __slate_storage_783: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_783: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_783) as *mut *mut Expr;
    unsafe {
        0 as i32;
        0 as i32;
        '__join_2: {
            loop {
                0 as i32;
                0 as i32;
                0 as i32;
                0 as i32;
                if !((unsafe { (*p).flags }) & (((65536 as i32) | (8388608 as i32)) as u32)
                    != ((0 as i32) as u32))
                {
                    // The Expr.x union is never used at the same time as Expr.pRight
                    0 as i32;
                    if (unsafe { (*p).pRight }) != std::ptr::null_mut::<Expr>() {
                        0 as i32;
                        sqlite3ExprDeleteNN(db, unsafe { (*p).pRight });
                    } else {
                        if (unsafe { (*p).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
                            0 as i32;
                            unsafe { sqlite3SelectDelete(db, unsafe { (*p).x.pSelect }) };
                        } else {
                            sqlite3ExprListDelete(db, unsafe { (*p).x.pList });
                            if (unsafe { (*p).flags }) & ((16777216 as i32) as u32)
                                != ((0 as i32) as u32)
                            {
                                unsafe { sqlite3WindowDelete(db, unsafe { (*p).y.pWin }) };
                            }
                        }
                    }
                    if (unsafe { (*p).pLeft }) != std::ptr::null_mut::<Expr>()
                        && (((unsafe { (*p).op }) as u32) as i32) != (178 as i32)
                    {
                        std::ptr::write(__slate_slot_783, unsafe { (*p).pLeft });
                        if !((unsafe { (*p).flags }) & ((134217728 as i32) as u32)
                            != ((0 as i32) as u32))
                            && !((unsafe { (*(*__slate_slot_783)).flags })
                                & ((134217728 as i32) as u32)
                                != ((0 as i32) as u32))
                        {
                            // Avoid unnecessary recursion on unary operators
                            unsafe { sqlite3DbNNFreeNN(db, p as *mut ()) };
                            p = *__slate_slot_783;
                        } else {
                            break;
                        }
                    } else {
                        break '__join_2;
                    }
                } else {
                    break '__join_2;
                }
            }
            sqlite3ExprDeleteNN(db, *__slate_slot_783);
        }
        if !((unsafe { (*p).flags }) & ((134217728 as i32) as u32) != ((0 as i32) as u32)) {
            unsafe { sqlite3DbNNFreeNN(db, p as *mut ()) };
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprDelete(mut db: *mut sqlite3, mut p: *mut Expr) {
    if p != std::ptr::null_mut::<Expr>() {
        sqlite3ExprDeleteNN(db, p);
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.expr.sqlite3ExprDeleteGeneric")]
extern "C-unwind" fn sqlite3ExprDeleteGeneric(mut db: *mut sqlite3, mut p: *mut ()) {
    if p != std::ptr::null_mut::<()>() {
        sqlite3ExprDeleteNN(db, p as *mut Expr);
    }
}

/// Clear both elements of an OnOrUsing object
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ClearOnOrUsing(mut db: *mut sqlite3, mut p: *mut OnOrUsing) {
    if p == std::ptr::null_mut::<OnOrUsing>() {
        // Nothing to clear
    } else {
        if (unsafe { (*p).pOn }) != std::ptr::null_mut::<Expr>() {
            sqlite3ExprDeleteNN(db, unsafe { (*p).pOn });
        } else {
            if (unsafe { (*p).pUsing }) != std::ptr::null_mut::<IdList>() {
                unsafe { sqlite3IdListDelete(db, unsafe { (*p).pUsing }) };
            }
        }
    }
}

/// Arrange to cause pExpr to be deleted when the pParse is deleted.
/// This is similar to sqlite3ExprDelete() except that the delete is
/// deferred until the pParse is deleted.
///
/// The pExpr might be deleted immediately on an OOM error.
///
/// Return 0 if the delete was successfully deferred.  Return non-zero
/// if the delete happened immediately because of an OOM.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprDeferredDelete(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
) -> i32 {
    return (std::ptr::null_mut::<()>()
        == unsafe {
            sqlite3ParserAddCleanup(pParse, Some(sqlite3ExprDeleteGeneric), pExpr as *mut ())
        }) as i32;
}

/// Invoke sqlite3RenameExprUnmap() and sqlite3ExprDelete() on the
/// expression.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprUnmapAndDelete(mut pParse: *mut Parse, mut p: *mut Expr) {
    if p != std::ptr::null_mut::<Expr>() {
        if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
            unsafe { sqlite3RenameExprUnmap(pParse, p) };
        }
        sqlite3ExprDeleteNN(unsafe { (*pParse).db }, p);
    }
}

/// Return the number of bytes allocated for the expression structure
/// passed as the first argument. This is always one of EXPR_FULLSIZE,
/// EXPR_REDUCEDSIZE or EXPR_TOKENONLYSIZE.
fn exprStructSize(mut p: *const Expr) -> i32 {
    if (unsafe { (*p).flags }) & ((65536 as i32) as u32) != ((0 as i32) as u32) {
        return ((16 as u64) as u32) as i32;
    }
    if (unsafe { (*p).flags }) & ((16384 as i32) as u32) != ((0 as i32) as u32) {
        return ((44 as u64) as u32) as i32;
    }
    return ((72 as u64) as u32) as i32;
}

/// The dupedExpr*Size() routines each return the number of bytes required
/// to store a copy of an expression or expression tree.  They differ in
/// how much of the tree is measured.
///
///     dupedExprStructSize()     Size of only the Expr structure
///     dupedExprNodeSize()       Size of Expr + space for token
///     dupedExprSize()           Expr + token + subtree components
///
///
///
/// The dupedExprStructSize() function returns two values OR-ed together:
/// (1) the space required for a copy of the Expr structure only and
/// (2) the EP_xxx flags that indicate what the structure size should be.
/// The return values is always one of:
///
///      EXPR_FULLSIZE
///      EXPR_REDUCEDSIZE   | EP_Reduced
///      EXPR_TOKENONLYSIZE | EP_TokenOnly
///
/// The size of the structure can be found by masking the return value
/// of this routine with 0xfff.  The flags can be found by masking the
/// return value with EP_Reduced|EP_TokenOnly.
///
/// Note that with flags==EXPRDUP_REDUCE, this routines works on full-size
/// (unreduced) Expr objects as they or originally constructed by the parser.
/// During expression analysis, extra information is computed and moved into
/// later parts of the Expr object and that extra information might get chopped
/// off if the expression is reduced.  Note also that it does not work to
/// make an EXPRDUP_REDUCE copy of a reduced expression.  It is only legal
/// to reduce a pristine expression tree from the parser.  The implementation
/// of dupedExprStructSize() contain multiple assert() statements that attempt
/// to enforce this constraint.
fn dupedExprStructSize(mut p: *const Expr, mut flags: i32) -> i32 {
    let mut nSize: i32 = 0 as i32;
    0 as i32; // Only one flag value allowed
    0 as i32;
    0 as i32;
    if (0 as i32) == flags
        || (unsafe { (*p).flags }) & ((131072 as i32) as u32) != ((0 as i32) as u32)
    {
        nSize = ((72 as u64) as u32) as i32;
    } else {
        0 as i32;
        0 as i32;
        0 as i32;
        if (unsafe { (*p).pLeft }) != std::ptr::null_mut::<Expr>()
            || (unsafe { (*p).x.pList }) != std::ptr::null_mut::<ExprList>()
        {
            nSize = (((44 as u64) | (((16384 as i32) as i64) as u64)) as u32) as i32;
        } else {
            0 as i32;
            nSize = (((16 as u64) | (((65536 as i32) as i64) as u64)) as u32) as i32;
        }
    }
    return nSize;
}

/// This function returns the space in bytes required to store the copy
/// of the Expr structure and a copy of the Expr.u.zToken string (if that
/// string is defined.)
fn dupedExprNodeSize(mut p: *const Expr, mut flags: i32) -> i32 {
    let mut nByte: i32 = dupedExprStructSize(p, flags) & (4095 as i32);
    if !((unsafe { (*p).flags }) & ((2048 as i32) as u32) != ((0 as i32) as u32))
        && (unsafe { (*p).u.zToken }) != std::ptr::null_mut::<i8>()
    {
        let __v2637: i32 = nByte;
        let __v2638: i32 = (((__v2637 as i64) as u64).wrapping_add(
            ((unsafe { strlen((unsafe { (*p).u.zToken }) as *const i8) })
                & (((1073741823 as i32) as i64) as u64))
                .wrapping_add(((1 as i32) as i64) as u64),
        ) as u32) as i32;
        nByte = __v2638;
    }
    return nByte + (7 as i32) & !(7 as i32);
}

/// Return the number of bytes required to create a duplicate of the
/// expression passed as the first argument.
///
/// The value returned includes space to create a copy of the Expr struct
/// itself and the buffer referred to by Expr.u.zToken, if any.
///
/// The return value includes space to duplicate all Expr nodes in the
/// tree formed by Expr.pLeft and Expr.pRight, but not any other
/// substructure such as Expr.x.pList, Expr.x.pSelect, and Expr.y.pWin.
fn dupedExprSize(mut p: *const Expr) -> i32 {
    let mut nByte: i32 = 0 as i32;
    0 as i32;
    nByte = dupedExprNodeSize(p, 1 as i32);
    if (unsafe { (*p).pLeft }) != std::ptr::null_mut::<Expr>() {
        let __v2639: i32 = nByte;
        let __v2640: i32 = __v2639 + dupedExprSize((unsafe { (*p).pLeft }) as *const Expr);
        nByte = __v2640;
    }
    if (unsafe { (*p).pRight }) != std::ptr::null_mut::<Expr>() {
        let __v2641: i32 = nByte;
        let __v2642: i32 = __v2641 + dupedExprSize((unsafe { (*p).pRight }) as *const Expr);
        nByte = __v2642;
    }
    0 as i32;
    return nByte;
}

/// An EdupBuf is a memory allocation used to stored multiple Expr objects
/// together with their Expr.zToken content.  This is used to help implement
/// compression while doing sqlite3ExprDup().  The top-level Expr does the
/// allocation for itself and many of its decendents, then passes an instance
/// of the structure down into exprDup() so that they decendents can have
/// access to that memory.
#[repr(C)]
#[derive(Clone, Copy)]
struct EdupBuf {
    /// Memory space available for storage
    zAlloc: *mut u8,
}

/// This function is similar to sqlite3ExprDup(), except that if pEdupBuf
/// is not NULL then it points to memory that can be used to store a copy
/// of the input Expr p together with its p->u.zToken (if any).  pEdupBuf
/// is updated with the new buffer tail prior to returning.
///
/// # Arguments
///
/// * `db` - Database connection (for memory allocation)
/// * `p` - Expr tree to be duplicated
/// * `dupFlags` - EXPRDUP_REDUCE for compression.  0 if not
/// * `pEdupBuf` - Preallocated storage space, or NULL
fn exprDup(
    mut db: *mut sqlite3,
    mut p: *const Expr,
    mut dupFlags: i32,
    mut pEdupBuf: *mut EdupBuf,
) -> *mut Expr {
    let mut pNew: *mut Expr = unsafe { std::mem::zeroed() }; // Value to return
    let mut sEdupBuf: EdupBuf = unsafe { std::mem::zeroed() }; // Memory space from which to build Expr object
    let mut staticFlag: u32 = 0 as u32; // EP_Static if space not obtained from malloc
    let mut nToken: i32 = -(1 as i32); // Space needed for p->u.zToken.  -1 means unknown
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    // Figure out where to write the new Expr structure.
    if pEdupBuf != std::ptr::null_mut::<EdupBuf>() {
        sEdupBuf.zAlloc = unsafe { (*pEdupBuf).zAlloc };
        staticFlag = (134217728 as i32) as u32;
        0 as i32;
        0 as i32;
    } else {
        let mut nAlloc: i32 = 0 as i32;
        if dupFlags != (0 as i32) {
            nAlloc = dupedExprSize(p);
        } else {
            if !((unsafe { (*p).flags }) & ((2048 as i32) as u32) != ((0 as i32) as u32))
                && (unsafe { (*p).u.zToken }) != std::ptr::null_mut::<i8>()
            {
                nToken = (((unsafe { strlen((unsafe { (*p).u.zToken }) as *const i8) })
                    & (((1073741823 as i32) as i64) as u64))
                    .wrapping_add(((1 as i32) as i64) as u64) as u32)
                    as i32;
                nAlloc = (((72 as u64)
                    .wrapping_add((nToken as i64) as u64)
                    .wrapping_add(((7 as i32) as i64) as u64)
                    & ((!(7 as i32) as i64) as u64)) as u32) as i32;
            } else {
                nToken = 0 as i32;
                nAlloc = (((72 as u64).wrapping_add(((7 as i32) as i64) as u64)
                    & ((!(7 as i32) as i64) as u64)) as u32) as i32;
            }
        }
        0 as i32;
        sEdupBuf.zAlloc = (unsafe { sqlite3DbMallocRawNN(db, (nAlloc as i64) as u64) }) as *mut u8;
        staticFlag = (0 as i32) as u32;
    }
    pNew = sEdupBuf.zAlloc as *mut Expr;
    0 as i32;
    if pNew != std::ptr::null_mut::<Expr>() {
        // Set nNewSize to the size allocated for the structure pointed to
        // by pNew. This is either EXPR_FULLSIZE, EXPR_REDUCEDSIZE or
        // EXPR_TOKENONLYSIZE. nToken is set to the number of bytes consumed
        // by the copy of the p->u.zToken string (if any).
        let mut nStructSize: u32 = dupedExprStructSize(p, dupFlags) as u32;
        let mut nNewSize: i32 = (nStructSize & ((4095 as i32) as u32)) as i32;
        if nToken < (0 as i32) {
            if !((unsafe { (*p).flags }) & ((2048 as i32) as u32) != ((0 as i32) as u32))
                && (unsafe { (*p).u.zToken }) != std::ptr::null_mut::<i8>()
            {
                nToken = (unsafe { sqlite3Strlen30((unsafe { (*p).u.zToken }) as *const i8) })
                    + (1 as i32);
            } else {
                nToken = 0 as i32;
            }
        }
        if dupFlags != (0 as i32) {
            0 as i32;
            0 as i32;
            unsafe {
                memcpy(
                    sEdupBuf.zAlloc as *mut (),
                    p as *const (),
                    (nNewSize as i64) as u64,
                )
            };
        } else {
            let mut nSize: u32 = exprStructSize(p) as u32;
            0 as i32;
            unsafe { memcpy(sEdupBuf.zAlloc as *mut (), p as *const (), nSize as u64) };
            if (nSize as u64) < (72 as u64) {
                unsafe {
                    memset(
                        (unsafe { sEdupBuf.zAlloc.offset(nSize as isize) }) as *mut (),
                        0 as i32,
                        (72 as u64).wrapping_sub(nSize as u64),
                    )
                };
            }
            nNewSize = ((72 as u64) as u32) as i32;
        }
        // Set the EP_Reduced, EP_TokenOnly, and EP_Static flags appropriately.
        let __v2643: *mut Expr = pNew;
        let __v2644: u32 = unsafe { (*__v2643).flags };
        let __v2645: u32 =
            __v2644 & (!((16384 as i32) | (65536 as i32) | (134217728 as i32)) as u32);
        unsafe {
            (*__v2643).flags = __v2645;
        }
        let __v2646: *mut Expr = pNew;
        let __v2647: u32 = unsafe { (*__v2646).flags };
        let __v2648: u32 = __v2647 | nStructSize & (((16384 as i32) | (65536 as i32)) as u32);
        unsafe {
            (*__v2646).flags = __v2648;
        }
        let __v2649: *mut Expr = pNew;
        let __v2650: u32 = unsafe { (*__v2649).flags };
        let __v2651: u32 = __v2650 | staticFlag;
        unsafe {
            (*__v2649).flags = __v2651;
        }
        {}
        if dupFlags != (0 as i32) {
            {}
        }
        // Copy the p->u.zToken string, if any.
        0 as i32;
        if nToken > (0 as i32) {
            let mut zToken: *mut i8 = unsafe { std::mem::zeroed() };
            let __v2652: *mut i8 =
                (unsafe { sEdupBuf.zAlloc.offset(nNewSize as isize) }) as *mut i8;
            unsafe {
                (*pNew).u.zToken = __v2652;
            }
            zToken = __v2652;
            unsafe {
                memcpy(
                    zToken as *mut (),
                    (unsafe { (*p).u.zToken }) as *const (),
                    (nToken as i64) as u64,
                )
            };
            let __v2653: i32 = nNewSize;
            let __v2654: i32 = __v2653 + nToken;
            nNewSize = __v2654;
        }
        let __v2655: *mut u8 = sEdupBuf.zAlloc;
        let __v2656: *mut u8 =
            unsafe { __v2655.offset((nNewSize + (7 as i32) & !(7 as i32)) as isize) };
        sEdupBuf.zAlloc = __v2656;
        if ((unsafe { (*p).flags }) | unsafe { (*pNew).flags })
            & (((65536 as i32) | (8388608 as i32)) as u32)
            == ((0 as i32) as u32)
        {
            // Fill in the pNew->x.pSelect or pNew->x.pList member.
            if (unsafe { (*p).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
                unsafe {
                    (*pNew).x.pSelect = sqlite3SelectDup(
                        db,
                        (unsafe { (*p).x.pSelect }) as *const Select,
                        dupFlags,
                    );
                }
            } else {
                unsafe {
                    (*pNew).x.pList = sqlite3ExprListDup(
                        db,
                        (unsafe { (*p).x.pList }) as *const ExprList,
                        if (((unsafe { (*p).op }) as u32) as i32) != (146 as i32) {
                            dupFlags
                        } else {
                            0 as i32
                        },
                    );
                }
            }
            if (unsafe { (*p).flags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32) {
                unsafe {
                    (*pNew).y.pWin = unsafe { sqlite3WindowDup(db, pNew, unsafe { (*p).y.pWin }) };
                }
                0 as i32;
            }
            // Fill in pNew->pLeft and pNew->pRight.
            if dupFlags != (0 as i32) {
                if (((unsafe { (*p).op }) as u32) as i32) == (178 as i32) {
                    unsafe {
                        (*pNew).pLeft = unsafe { (*p).pLeft };
                    }
                    0 as i32;
                } else {
                    let __v2657: *mut Expr;
                    if (unsafe { (*p).pLeft }) != std::ptr::null_mut::<Expr>() {
                        __v2657 = exprDup(
                            db,
                            (unsafe { (*p).pLeft }) as *const Expr,
                            1 as i32,
                            std::ptr::addr_of_mut!(sEdupBuf),
                        );
                    } else {
                        __v2657 = std::ptr::null_mut::<Expr>();
                    }
                    unsafe {
                        (*pNew).pLeft = __v2657;
                    }
                }
                let __v2658: *mut Expr;
                if (unsafe { (*p).pRight }) != std::ptr::null_mut::<Expr>() {
                    __v2658 = exprDup(
                        db,
                        (unsafe { (*p).pRight }) as *const Expr,
                        1 as i32,
                        std::ptr::addr_of_mut!(sEdupBuf),
                    );
                } else {
                    __v2658 = std::ptr::null_mut::<Expr>();
                }
                unsafe {
                    (*pNew).pRight = __v2658;
                }
            } else {
                if (((unsafe { (*p).op }) as u32) as i32) == (178 as i32) {
                    unsafe {
                        (*pNew).pLeft = unsafe { (*p).pLeft };
                    }
                    0 as i32;
                } else {
                    unsafe {
                        (*pNew).pLeft =
                            sqlite3ExprDup(db, (unsafe { (*p).pLeft }) as *const Expr, 0 as i32);
                    }
                }
                unsafe {
                    (*pNew).pRight =
                        sqlite3ExprDup(db, (unsafe { (*p).pRight }) as *const Expr, 0 as i32);
                }
            }
        }
    }
    if pEdupBuf != std::ptr::null_mut::<EdupBuf>() {
        unsafe {
            memcpy(
                pEdupBuf as *mut (),
                std::ptr::addr_of_mut!(sEdupBuf) as *const (),
                8 as u64,
            )
        };
    }
    0 as i32;
    return pNew;
}

/// Create and return a deep copy of the object passed as the second
/// argument. If an OOM condition is encountered, NULL is returned
/// and the db->mallocFailed flag set.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WithDup(mut db: *mut sqlite3, mut p: *mut With) -> *mut With {
    let mut pRet: *mut With = std::ptr::null_mut::<With>();
    if p != std::ptr::null_mut::<With>() {
        let mut nByte: i64 = (16 as u64)
            .wrapping_add((((unsafe { (*p).nCte }) as i64) as u64).wrapping_mul(48 as u64))
            as i64;
        pRet = (unsafe { sqlite3DbMallocZero(db, nByte as u64) }) as *mut With;
        if pRet != std::ptr::null_mut::<With>() {
            let mut i: i32 = 0 as i32;
            unsafe {
                (*pRet).nCte = unsafe { (*p).nCte };
            }
            i = 0 as i32;
            '__slate_break_2235: loop {
                if !(i < unsafe { (*p).nCte }) {
                    break;
                }
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pRet).a) as *mut Cte }.offset(i as isize)
                    })
                    .pSelect = sqlite3SelectDup(
                        db,
                        (unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*p).a) as *mut Cte }
                                    .offset(i as isize)
                            })
                            .pSelect
                        }) as *const Select,
                        0 as i32,
                    );
                }
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pRet).a) as *mut Cte }.offset(i as isize)
                    })
                    .pCols = sqlite3ExprListDup(
                        db,
                        (unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*p).a) as *mut Cte }
                                    .offset(i as isize)
                            })
                            .pCols
                        }) as *const ExprList,
                        0 as i32,
                    );
                }
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pRet).a) as *mut Cte }.offset(i as isize)
                    })
                    .zName = unsafe {
                        sqlite3DbStrDup(
                            db,
                            (unsafe {
                                (*unsafe {
                                    unsafe { std::ptr::addr_of_mut!((*p).a) as *mut Cte }
                                        .offset(i as isize)
                                })
                                .zName
                            }) as *const i8,
                        )
                    };
                }
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pRet).a) as *mut Cte }.offset(i as isize)
                    })
                    .eM10d = unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*p).a) as *mut Cte }.offset(i as isize)
                        })
                        .eM10d
                    };
                }
                let __v2540: i32 = i;
                let __v2541: i32 = __v2540 + (1 as i32);
                i = __v2541;
            }
        }
    }
    return pRet;
}

/// The gatherSelectWindows() procedure and its helper routine
/// gatherSelectWindowsCallback() are used to scan all the expressions
/// an a newly duplicated SELECT statement and gather all of the Window
/// objects found there, assembling them onto the linked list at Select->pWin.
#[unsafe(link_section = ".text.slate_distinct.expr.gatherSelectWindowsCallback")]
extern "C-unwind" fn gatherSelectWindowsCallback(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> i32 {
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (172 as i32)
        && (unsafe { (*pExpr).flags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32)
    {
        let mut pSelect: *mut Select = unsafe { (*pWalker).u.pSelect };
        let mut pWin: *mut Window = unsafe { (*pExpr).y.pWin };
        0 as i32;
        0 as i32;
        0 as i32;
        unsafe { sqlite3WindowLink(pSelect, pWin) };
    }
    return 0 as i32;
}

#[unsafe(link_section = ".text.slate_distinct.expr.gatherSelectWindowsSelectCallback")]
extern "C-unwind" fn gatherSelectWindowsSelectCallback(
    mut pWalker: *mut Walker,
    mut p: *mut Select,
) -> i32 {
    return if p == unsafe { (*pWalker).u.pSelect } {
        0 as i32
    } else {
        1 as i32
    };
}

fn gatherSelectWindows(mut p: *mut Select) {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    w.xExprCallback = Some(gatherSelectWindowsCallback);
    w.xSelectCallback = Some(gatherSelectWindowsSelectCallback);
    w.xSelectCallback2 = None;
    w.pParse = std::ptr::null_mut::<Parse>();
    unsafe {
        w.u.pSelect = p;
    }
    unsafe { sqlite3WalkSelect(std::ptr::addr_of_mut!(w), p) };
}

/// The following group of routines make deep copies of expressions,
/// expression lists, ID lists, and select statements.  The copies can
/// be deleted (by being passed to their respective ...Delete() routines)
/// without effecting the originals.
///
/// The expression list, ID, and source lists return by sqlite3ExprListDup(),
/// sqlite3IdListDup(), and sqlite3SrcListDup() can not be further expanded
/// by subsequent calls to sqlite*ListAppend() routines.
///
/// Any tables that the SrcList might point to are not duplicated.
///
/// The flags parameter contains a combination of the EXPRDUP_XXX flags.
/// If the EXPRDUP_REDUCE flag is set, then the structure returned is a
/// truncated version of the usual Expr structure that will be stored as
/// part of the in-memory representation of the database schema.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprDup(
    mut db: *mut sqlite3,
    mut p: *const Expr,
    mut flags: i32,
) -> *mut Expr {
    0 as i32;
    let __v2519: *mut Expr;
    if p != std::ptr::null::<Expr>() {
        __v2519 = exprDup(db, p, flags, std::ptr::null_mut::<EdupBuf>());
    } else {
        __v2519 = std::ptr::null_mut::<Expr>();
    }
    return __v2519;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprListDup(
    mut db: *mut sqlite3,
    mut p: *const ExprList,
    mut flags: i32,
) -> *mut ExprList {
    let mut pNew: *mut ExprList = unsafe { std::mem::zeroed() };
    let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
    let mut pOldItem: *const ExprList_item = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let mut pPriorSelectColOld: *mut Expr = std::ptr::null_mut::<Expr>();
    let mut pPriorSelectColNew: *mut Expr = std::ptr::null_mut::<Expr>();
    0 as i32;
    if p == std::ptr::null::<ExprList>() {
        return std::ptr::null_mut::<ExprList>();
    }
    pNew = (unsafe {
        sqlite3DbMallocRawNN(
            db,
            ((unsafe { sqlite3DbMallocSize(db, p as *const ()) }) as i64) as u64,
        )
    }) as *mut ExprList;
    if pNew == std::ptr::null_mut::<ExprList>() {
        return std::ptr::null_mut::<ExprList>();
    }
    unsafe {
        (*pNew).nExpr = unsafe { (*p).nExpr };
    }
    unsafe {
        (*pNew).nAlloc = unsafe { (*p).nAlloc };
    }
    pItem = unsafe { std::ptr::addr_of_mut!((*pNew).a) as *mut ExprList_item };
    pOldItem = unsafe { std::ptr::addr_of!((*p).a) as *const ExprList_item };
    i = 0 as i32;
    '__slate_break_2236: while i < unsafe { (*p).nExpr } {
        let mut pOldExpr: *mut Expr = unsafe { (*pOldItem).pExpr };
        let mut pNewExpr: *mut Expr = unsafe { std::mem::zeroed() };
        unsafe {
            (*pItem).pExpr = sqlite3ExprDup(db, pOldExpr as *const Expr, flags);
        }
        let __v2526: bool;
        if pOldExpr != std::ptr::null_mut::<Expr>()
            && (((unsafe { (*pOldExpr).op }) as u32) as i32) == (178 as i32)
        {
            let __v2527: *mut Expr = unsafe { (*pItem).pExpr };
            pNewExpr = __v2527;
            __v2526 = __v2527 != std::ptr::null_mut::<Expr>();
        } else {
            __v2526 = false as bool;
        }
        if __v2526 {
            if (unsafe { (*pNewExpr).pRight }) != std::ptr::null_mut::<Expr>() {
                pPriorSelectColOld = unsafe { (*pOldExpr).pRight };
                pPriorSelectColNew = unsafe { (*pNewExpr).pRight };
                unsafe {
                    (*pNewExpr).pLeft = unsafe { (*pNewExpr).pRight };
                }
            } else {
                if (unsafe { (*pOldExpr).pLeft }) != pPriorSelectColOld {
                    pPriorSelectColOld = unsafe { (*pOldExpr).pLeft };
                    pPriorSelectColNew =
                        sqlite3ExprDup(db, pPriorSelectColOld as *const Expr, flags);
                    unsafe {
                        (*pNewExpr).pRight = pPriorSelectColNew;
                    }
                }
                unsafe {
                    (*pNewExpr).pLeft = pPriorSelectColNew;
                }
            }
        }
        unsafe {
            (*pItem).zEName =
                unsafe { sqlite3DbStrDup(db, (unsafe { (*pOldItem).zEName }) as *const i8) };
        }
        unsafe {
            (*pItem).fg = unsafe { (*pOldItem).fg };
        }
        unsafe {
            (*pItem).u = unsafe { (*pOldItem).u };
        }
        let __v2520: i32 = i;
        let __v2521: i32 = __v2520 + (1 as i32);
        i = __v2521;
        let __v2522: *mut ExprList_item = pItem;
        let __v2523: *mut ExprList_item = unsafe { __v2522.offset((1 as i32) as isize) };
        pItem = __v2523;
        let __v2524: *const ExprList_item = pOldItem;
        let __v2525: *const ExprList_item = unsafe { __v2524.offset((1 as i32) as isize) };
        pOldItem = __v2525;
    }
    return pNew;
}

/// If cursors, triggers, views and subqueries are all omitted from
/// the build, then none of the following routines, except for
/// sqlite3SelectDup(), can be called. sqlite3SelectDup() is sometimes
/// called with a NULL argument.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SrcListDup(
    mut db: *mut sqlite3,
    mut p: *const SrcList,
    mut flags: i32,
) -> *mut SrcList {
    let mut pNew: *mut SrcList = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    0 as i32;
    if p == std::ptr::null::<SrcList>() {
        return std::ptr::null_mut::<SrcList>();
    }
    pNew = (unsafe {
        sqlite3DbMallocRawNN(
            db,
            (8 as u64)
                .wrapping_add((((unsafe { (*p).nSrc }) as i64) as u64).wrapping_mul(72 as u64)),
        )
    }) as *mut SrcList;
    if pNew == std::ptr::null_mut::<SrcList>() {
        return std::ptr::null_mut::<SrcList>();
    }
    let __v2528: u32 = (unsafe { (*p).nSrc }) as u32;
    unsafe {
        (*pNew).nAlloc = __v2528;
    }
    unsafe {
        (*pNew).nSrc = __v2528 as i32;
    }
    i = 0 as i32;
    '__slate_break_2237: loop {
        if !(i < unsafe { (*p).nSrc }) {
            break;
        }
        let mut pNewItem: *mut SrcItem = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pNew).a) as *mut SrcItem }.offset(i as isize)
        };
        let mut pOldItem: *const SrcItem =
            unsafe { unsafe { std::ptr::addr_of!((*p).a) as *const SrcItem }.offset(i as isize) };
        let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
        unsafe {
            (*pNewItem).fg = unsafe { (*pOldItem).fg };
        }
        if ((unsafe { (*pOldItem).fg.__slate_bits_0.__get_isSubquery() }) as i32) != (0 as i32) {
            let mut pNewSubq: *mut Subquery =
                (unsafe { sqlite3DbMallocRaw(db, 24 as u64) }) as *mut Subquery;
            if pNewSubq == std::ptr::null_mut::<Subquery>() {
                0 as i32;
                unsafe {
                    (*pNewItem)
                        .fg
                        .__slate_bits_0
                        .__set_isSubquery((0 as i32) as u32);
                }
            } else {
                unsafe {
                    memcpy(
                        pNewSubq as *mut (),
                        (unsafe { (*pOldItem).u4.pSubq }) as *const (),
                        24 as u64,
                    )
                };
                unsafe {
                    (*pNewSubq).pSelect = sqlite3SelectDup(
                        db,
                        (unsafe { (*pNewSubq).pSelect }) as *const Select,
                        flags,
                    );
                }
                if (unsafe { (*pNewSubq).pSelect }) == std::ptr::null_mut::<Select>() {
                    unsafe { sqlite3DbFree(db, pNewSubq as *mut ()) };
                    pNewSubq = std::ptr::null_mut::<Subquery>();
                    unsafe {
                        (*pNewItem)
                            .fg
                            .__slate_bits_0
                            .__set_isSubquery((0 as i32) as u32);
                    }
                }
            }
            unsafe {
                (*pNewItem).u4.pSubq = pNewSubq;
            }
        } else {
            if ((unsafe { (*pOldItem).fg.__slate_bits_0.__get_fixedSchema() }) as i32) != (0 as i32)
            {
                unsafe {
                    (*pNewItem).u4.pSchema = unsafe { (*pOldItem).u4.pSchema };
                }
            } else {
                unsafe {
                    (*pNewItem).u4.zDatabase = unsafe {
                        sqlite3DbStrDup(db, (unsafe { (*pOldItem).u4.zDatabase }) as *const i8)
                    };
                }
            }
        }
        unsafe {
            (*pNewItem).zName =
                unsafe { sqlite3DbStrDup(db, (unsafe { (*pOldItem).zName }) as *const i8) };
        }
        unsafe {
            (*pNewItem).zAlias =
                unsafe { sqlite3DbStrDup(db, (unsafe { (*pOldItem).zAlias }) as *const i8) };
        }
        unsafe {
            (*pNewItem).iCursor = unsafe { (*pOldItem).iCursor };
        }
        if ((unsafe { (*pNewItem).fg.__slate_bits_0.__get_isIndexedBy() }) as i32) != (0 as i32) {
            unsafe {
                (*pNewItem).u1.zIndexedBy = unsafe {
                    sqlite3DbStrDup(db, (unsafe { (*pOldItem).u1.zIndexedBy }) as *const i8)
                };
            }
        } else {
            if ((unsafe { (*pNewItem).fg.__slate_bits_0.__get_isTabFunc() }) as i32) != (0 as i32) {
                unsafe {
                    (*pNewItem).u1.pFuncArg = sqlite3ExprListDup(
                        db,
                        (unsafe { (*pOldItem).u1.pFuncArg }) as *const ExprList,
                        flags,
                    );
                }
            } else {
                unsafe {
                    (*pNewItem).u1.nRow = unsafe { (*pOldItem).u1.nRow };
                }
            }
        }
        unsafe {
            (*pNewItem).u2 = unsafe { (*pOldItem).u2 };
        }
        if ((unsafe { (*pNewItem).fg.__slate_bits_0.__get_isCte() }) as i32) != (0 as i32) {
            let __v2531: *mut CteUse = unsafe { (*pNewItem).u2.pCteUse };
            let __v2532: i32 = unsafe { (*__v2531).nUse };
            let __v2533: i32 = __v2532 + (1 as i32);
            unsafe {
                (*__v2531).nUse = __v2533;
            }
        }
        let __v2534: *mut Table = unsafe { (*pOldItem).pSTab };
        unsafe {
            (*pNewItem).pSTab = __v2534;
        }
        pTab = __v2534;
        if pTab != std::ptr::null_mut::<Table>() {
            let __v2535: *mut Table = pTab;
            let __v2536: u32 = unsafe { (*__v2535).nTabRef };
            let __v2537: u32 = __v2536.wrapping_add((1 as i32) as u32);
            unsafe {
                (*__v2535).nTabRef = __v2537;
            }
        }
        if ((unsafe { (*pOldItem).fg.__slate_bits_0.__get_isUsing() }) as i32) != (0 as i32) {
            0 as i32;
            unsafe {
                (*pNewItem).u3.pUsing =
                    sqlite3IdListDup(db, (unsafe { (*pOldItem).u3.pUsing }) as *const IdList);
            }
        } else {
            unsafe {
                (*pNewItem).u3.pOn =
                    sqlite3ExprDup(db, (unsafe { (*pOldItem).u3.pOn }) as *const Expr, flags);
            }
        }
        unsafe {
            (*pNewItem).colUsed = unsafe { (*pOldItem).colUsed };
        }
        let __v2529: i32 = i;
        let __v2530: i32 = __v2529 + (1 as i32);
        i = __v2530;
    }
    return pNew;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IdListDup(mut db: *mut sqlite3, mut p: *const IdList) -> *mut IdList {
    let mut pNew: *mut IdList = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    0 as i32;
    if p == std::ptr::null::<IdList>() {
        return std::ptr::null_mut::<IdList>();
    }
    pNew = (unsafe {
        sqlite3DbMallocRawNN(
            db,
            (8 as u64).wrapping_add((((unsafe { (*p).nId }) as i64) as u64).wrapping_mul(8 as u64)),
        )
    }) as *mut IdList;
    if pNew == std::ptr::null_mut::<IdList>() {
        return std::ptr::null_mut::<IdList>();
    }
    unsafe {
        (*pNew).nId = unsafe { (*p).nId };
    }
    i = 0 as i32;
    '__slate_break_2238: loop {
        if !(i < unsafe { (*p).nId }) {
            break;
        }
        let mut pNewItem: *mut IdList_item = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pNew).a) as *mut IdList_item }.offset(i as isize)
        };
        let mut pOldItem: *const IdList_item = unsafe {
            unsafe { std::ptr::addr_of!((*p).a) as *const IdList_item }.offset(i as isize)
        };
        unsafe {
            (*pNewItem).zName =
                unsafe { sqlite3DbStrDup(db, (unsafe { (*pOldItem).zName }) as *const i8) };
        }
        let __v2538: i32 = i;
        let __v2539: i32 = __v2538 + (1 as i32);
        i = __v2539;
    }
    return pNew;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SelectDup(
    mut db: *mut sqlite3,
    mut pDup: *const Select,
    mut flags: i32,
) -> *mut Select {
    let mut pRet: *mut Select = std::ptr::null_mut::<Select>();
    let mut pNext: *mut Select = std::ptr::null_mut::<Select>();
    let mut pp: *mut *mut Select = std::ptr::addr_of_mut!(pRet);
    let mut p: *const Select = unsafe { std::mem::zeroed() };
    0 as i32;
    p = pDup;
    '__slate_break_2239: while p != std::ptr::null::<Select>() {
        let mut pNew: *mut Select =
            (unsafe { sqlite3DbMallocRawNN(db, 120 as u64) }) as *mut Select;
        if pNew == std::ptr::null_mut::<Select>() {
            break '__slate_break_2239;
        }
        unsafe {
            (*pNew).pEList =
                sqlite3ExprListDup(db, (unsafe { (*p).pEList }) as *const ExprList, flags);
        }
        unsafe {
            (*pNew).pSrc = sqlite3SrcListDup(db, (unsafe { (*p).pSrc }) as *const SrcList, flags);
        }
        unsafe {
            (*pNew).pWhere = sqlite3ExprDup(db, (unsafe { (*p).pWhere }) as *const Expr, flags);
        }
        unsafe {
            (*pNew).pGroupBy =
                sqlite3ExprListDup(db, (unsafe { (*p).pGroupBy }) as *const ExprList, flags);
        }
        unsafe {
            (*pNew).pHaving = sqlite3ExprDup(db, (unsafe { (*p).pHaving }) as *const Expr, flags);
        }
        unsafe {
            (*pNew).pOrderBy =
                sqlite3ExprListDup(db, (unsafe { (*p).pOrderBy }) as *const ExprList, flags);
        }
        unsafe {
            (*pNew).op = unsafe { (*p).op };
        }
        unsafe {
            (*pNew).pNext = pNext;
        }
        unsafe {
            (*pNew).pPrior = std::ptr::null_mut::<Select>();
        }
        unsafe {
            (*pNew).pLimit = sqlite3ExprDup(db, (unsafe { (*p).pLimit }) as *const Expr, flags);
        }
        unsafe {
            (*pNew).iLimit = 0 as i32;
        }
        unsafe {
            (*pNew).iOffset = 0 as i32;
        }
        unsafe {
            (*pNew).selFlags = unsafe { (*p).selFlags };
        }
        unsafe {
            (*pNew).nSelectRow = unsafe { (*p).nSelectRow };
        }
        unsafe {
            (*pNew).pWith = sqlite3WithDup(db, unsafe { (*p).pWith });
        }
        unsafe {
            (*pNew).pWin = std::ptr::null_mut::<Window>();
        }
        unsafe {
            (*pNew).pWinDefn = unsafe { sqlite3WindowListDup(db, unsafe { (*p).pWinDefn }) };
        }
        if (unsafe { (*p).pWin }) != std::ptr::null_mut::<Window>()
            && (((unsafe { (*db).mallocFailed }) as u32) as i32) == (0 as i32)
        {
            gatherSelectWindows(pNew);
        }
        unsafe {
            (*pNew).selId = unsafe { (*p).selId };
        }
        if (unsafe { (*db).mallocFailed }) != (0 as u8) {
            // Any prior OOM might have left the Select object incomplete.
            // Delete the whole thing rather than allow an incomplete Select
            // to be used by the code generator.
            unsafe {
                (*pNew).pNext = std::ptr::null_mut::<Select>();
            }
            unsafe { sqlite3SelectDelete(db, pNew) };
            break '__slate_break_2239;
        }
        unsafe {
            *pp = pNew;
        }
        pp = unsafe { std::ptr::addr_of_mut!((*pNew).pPrior) };
        pNext = pNew;
        p = (unsafe { (*p).pPrior }) as *const Select;
    }
    return pRet;
}

/// Add a new element to the end of an expression list.  If pList is
/// initially NULL, then create a new expression list.
///
/// The pList argument must be either NULL or a pointer to an ExprList
/// obtained from a prior call to sqlite3ExprListAppend().
///
/// If a memory allocation error occurs, the entire list is freed and
/// NULL is returned.  If non-NULL is returned, then it is guaranteed
/// that the new entry was successfully appended.
static mut zeroItem: ExprList_item = {
    let mut __t0: ExprList_item = unsafe { std::mem::zeroed() };
    __t0.pExpr = std::ptr::null_mut::<Expr>();
    __t0
};

/// # Arguments
///
/// * `db` - Database handle.  Used for memory allocation
/// * `pExpr` - Expression to be appended. Might be NULL
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprListAppendNew(
    mut db: *mut sqlite3,
    mut pExpr: *mut Expr,
) -> *mut ExprList {
    let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
    let mut pList: *mut ExprList = unsafe { std::mem::zeroed() };
    pList = (unsafe {
        sqlite3DbMallocRawNN(
            db,
            (8 as u64).wrapping_add((((4 as i32) as i64) as u64).wrapping_mul(24 as u64)),
        )
    }) as *mut ExprList;
    if pList == std::ptr::null_mut::<ExprList>() {
        sqlite3ExprDelete(db, pExpr);
        return std::ptr::null_mut::<ExprList>();
    }
    unsafe {
        (*pList).nAlloc = 4 as i32;
    }
    unsafe {
        (*pList).nExpr = 1 as i32;
    }
    pItem = unsafe {
        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
            .offset((0 as i32) as isize)
    };
    unsafe {
        *pItem = unsafe { zeroItem };
    }
    unsafe {
        (*pItem).pExpr = pExpr;
    }
    return pList;
}

/// # Arguments
///
/// * `db` - Database handle.  Used for memory allocation
/// * `pList` - List to which to append. Might be NULL
/// * `pExpr` - Expression to be appended. Might be NULL
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprListAppendGrow(
    mut db: *mut sqlite3,
    mut pList: *mut ExprList,
    mut pExpr: *mut Expr,
) -> *mut ExprList {
    let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
    let mut pNew: *mut ExprList = unsafe { std::mem::zeroed() };
    let __v2659: *mut ExprList = pList;
    let __v2660: i32 = unsafe { (*__v2659).nAlloc };
    let __v2661: i32 = __v2660 * (2 as i32);
    unsafe {
        (*__v2659).nAlloc = __v2661;
    }
    pNew = (unsafe {
        sqlite3DbRealloc(
            db,
            pList as *mut (),
            (8 as u64).wrapping_add(
                (((unsafe { (*pList).nAlloc }) as i64) as u64).wrapping_mul(24 as u64),
            ),
        )
    }) as *mut ExprList;
    if pNew == std::ptr::null_mut::<ExprList>() {
        sqlite3ExprListDelete(db, pList);
        sqlite3ExprDelete(db, pExpr);
        return std::ptr::null_mut::<ExprList>();
    } else {
        pList = pNew;
    }
    let __v2662: *mut ExprList = pList;
    let __v2663: i32 = unsafe { (*__v2662).nExpr };
    let __v2664: i32 = __v2663 + (1 as i32);
    unsafe {
        (*__v2662).nExpr = __v2664;
    }
    pItem = unsafe {
        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }.offset(__v2663 as isize)
    };
    unsafe {
        *pItem = unsafe { zeroItem };
    }
    unsafe {
        (*pItem).pExpr = pExpr;
    }
    return pList;
}

/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pList` - List to which to append. Might be NULL
/// * `pExpr` - Expression to be appended. Might be NULL
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprListAppend(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut pExpr: *mut Expr,
) -> *mut ExprList {
    let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
    if pList == std::ptr::null_mut::<ExprList>() {
        return sqlite3ExprListAppendNew(unsafe { (*pParse).db }, pExpr);
    }
    if (unsafe { (*pList).nAlloc }) < (unsafe { (*pList).nExpr }) + (1 as i32) {
        return sqlite3ExprListAppendGrow(unsafe { (*pParse).db }, pList, pExpr);
    }
    let __v2410: *mut ExprList = pList;
    let __v2411: i32 = unsafe { (*__v2410).nExpr };
    let __v2412: i32 = __v2411 + (1 as i32);
    unsafe {
        (*__v2410).nExpr = __v2412;
    }
    pItem = unsafe {
        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }.offset(__v2411 as isize)
    };
    unsafe {
        *pItem = unsafe { zeroItem };
    }
    unsafe {
        (*pItem).pExpr = pExpr;
    }
    return pList;
}

/// pColumns and pExpr form a vector assignment which is part of the SET
/// clause of an UPDATE statement.  Like this:
///
///        (a,b,c) = (expr1,expr2,expr3)
/// Or:    (a,b,c) = (SELECT x,y,z FROM ....)
///
/// For each term of the vector assignment, append new entries to the
/// expression list pList.  In the case of a subquery on the RHS, append
/// TK_SELECT_COLUMN expressions.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pList` - List to which to append. Might be NULL
/// * `pColumns` - List of names of LHS of the assignment
/// * `pExpr` - Vector expression to be appended. Might be NULL
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprListAppendVector(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut pColumns: *mut IdList,
    mut pExpr: *mut Expr,
) -> *mut ExprList {
    let mut __slate_storage_902: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_902: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_902) as *mut *mut Expr;
    let mut __slate_storage_2416: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2416: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2416) as *mut i32;
    let mut __slate_storage_2415: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2415: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2415) as *mut i32;
    let mut __slate_storage_901: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_901: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_901) as *mut *mut Expr;
    let mut __slate_storage_2414: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2414: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2414) as *mut i32;
    // If the RHS is a vector, then we can immediately check to see that
    // the size of the RHS and LHS match.  But if the RHS is a SELECT,
    // wildcards ("*") in the result set of the SELECT must be expanded before
    // we can do the size check, so defer the size check until code generation.
    let mut __slate_storage_2413: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2413: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2413) as *mut bool;
    let mut __slate_storage_900: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_900: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_900) as *mut i32;
    let mut __slate_storage_899: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_899: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_899) as *mut i32;
    let mut __slate_storage_898: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_898: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_898) as *mut i32;
    let mut __slate_storage_897: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_897: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_897) as *mut *mut sqlite3;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_897, unsafe { (*pParse).db });
            std::ptr::write(
                __slate_slot_900,
                if pList != std::ptr::null_mut::<ExprList>() {
                    unsafe { (*pList).nExpr }
                } else {
                    0 as i32
                },
            );
            // pColumns can only be NULL due to an OOM but an OOM will cause an
            // exit prior to this routine being invoked
            if pColumns == std::ptr::null_mut::<IdList>() {
            } else {
                if pExpr == std::ptr::null_mut::<Expr>() {
                } else {
                    if (((unsafe { (*pExpr).op }) as u32) as i32) != (139 as i32) {
                        std::ptr::write(
                            __slate_slot_2414,
                            sqlite3ExprVectorSize(pExpr as *const Expr),
                        );
                        *__slate_slot_898 = *__slate_slot_2414;
                        *__slate_slot_2413 = (unsafe { (*pColumns).nId }) != *__slate_slot_2414;
                    } else {
                        *__slate_slot_2413 = false as bool;
                    }
                    if *__slate_slot_2413 {
                        unsafe {
                            sqlite3ErrorMsg(
                                pParse,
                                (b"%d columns assigned %d values\0".as_ptr() as *mut i8)
                                    as *const i8,
                                unsafe { (*pColumns).nId },
                                *__slate_slot_898,
                            )
                        };
                    } else {
                        *__slate_slot_899 = 0 as i32;
                        loop {
                            if *__slate_slot_899 < unsafe { (*pColumns).nId } {
                                std::ptr::write(
                                    __slate_slot_901,
                                    sqlite3ExprForVectorField(
                                        pParse,
                                        pExpr,
                                        *__slate_slot_899,
                                        unsafe { (*pColumns).nId },
                                    ),
                                );
                                0 as i32;
                                if *__slate_slot_901 == std::ptr::null_mut::<Expr>() {
                                } else {
                                    pList = sqlite3ExprListAppend(pParse, pList, *__slate_slot_901);
                                    if pList != std::ptr::null_mut::<ExprList>() {
                                        0 as i32;
                                        unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!((*pList).a)
                                                        as *mut ExprList_item
                                                }
                                                .offset(
                                                    ((unsafe { (*pList).nExpr }) - (1 as i32))
                                                        as isize,
                                                )
                                            })
                                            .zEName = unsafe {
                                                (*unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of_mut!((*pColumns).a)
                                                            as *mut IdList_item
                                                    }
                                                    .offset(*__slate_slot_899 as isize)
                                                })
                                                .zName
                                            };
                                        }
                                        unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!((*pColumns).a)
                                                        as *mut IdList_item
                                                }
                                                .offset(*__slate_slot_899 as isize)
                                            })
                                            .zName = std::ptr::null_mut::<i8>();
                                        }
                                    }
                                }
                                std::ptr::write(__slate_slot_2415, *__slate_slot_899);
                                std::ptr::write(__slate_slot_2416, *__slate_slot_2415 + (1 as i32));
                                *__slate_slot_899 = *__slate_slot_2416;
                            } else {
                                break;
                            }
                        }
                        if !((unsafe { (*(*__slate_slot_897)).mallocFailed }) != (0 as u8))
                            && (((unsafe { (*pExpr).op }) as u32) as i32) == (139 as i32)
                            && pList != std::ptr::null_mut::<ExprList>()
                        {
                            std::ptr::write(__slate_slot_902, unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item
                                    }
                                    .offset(*__slate_slot_900 as isize)
                                })
                                .pExpr
                            });
                            0 as i32;
                            0 as i32;
                            // Store the SELECT statement in pRight so it will be deleted when
                            // sqlite3ExprListDelete() is called
                            unsafe {
                                (*(*__slate_slot_902)).pRight = pExpr;
                            }
                            pExpr = std::ptr::null_mut::<Expr>();
                            // Remember the size of the LHS in iTable so that we can check that
                            // the RHS and LHS sizes match during code generation.
                            unsafe {
                                (*(*__slate_slot_902)).iTable = unsafe { (*pColumns).nId };
                            }
                        }
                    }
                }
            }
        }
        sqlite3ExprUnmapAndDelete(pParse, pExpr);
        unsafe { sqlite3IdListDelete(*__slate_slot_897, pColumns) };
        return pList;
    }
    return unsafe { std::mem::zeroed() };
}

/// Set the sort order for the last element on the given ExprList.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprListSetSortOrder(
    mut p: *mut ExprList,
    mut iSortOrder: i32,
    mut eNulls: i32,
) {
    let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
    if p == std::ptr::null_mut::<ExprList>() {
        return;
    }
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    pItem = unsafe {
        unsafe { std::ptr::addr_of_mut!((*p).a) as *mut ExprList_item }
            .offset(((unsafe { (*p).nExpr }) - (1 as i32)) as isize)
    };
    0 as i32;
    if iSortOrder == -(1 as i32) {
        iSortOrder = 0 as i32;
    }
    unsafe {
        (*pItem).fg.sortFlags = (iSortOrder as i8) as u8;
    }
    if eNulls != -(1 as i32) {
        unsafe {
            (*pItem).fg.__slate_bits_0.__set_bNulls((1 as i32) as u32);
        }
        if iSortOrder != eNulls {
            let __v2422: *mut ExprList_item = pItem;
            let __v2423: u8 = unsafe { (*__v2422).fg.sortFlags };
            let __v2424: u8 = ((((__v2423 as u32) as i32) | (2 as i32)) as i8) as u8;
            unsafe {
                (*__v2422).fg.sortFlags = __v2424;
            }
        }
    }
}

/// Set the ExprList.a[].zEName element of the most recently added item
/// on the expression list.
///
/// pList might be NULL following an OOM error.  But pName should never be
/// NULL.  If a memory allocation fails, the pParse->db->mallocFailed flag
/// is set.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pList` - List to which to add the span.
/// * `pName` - Name to be added
/// * `dequote` - True to cause the name to be dequoted
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprListSetName(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut pName: *const Token,
    mut dequote: i32,
) {
    0 as i32;
    0 as i32;
    if pList != std::ptr::null_mut::<ExprList>() {
        let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
        0 as i32;
        pItem = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                .offset(((unsafe { (*pList).nExpr }) - (1 as i32)) as isize)
        };
        0 as i32;
        0 as i32;
        unsafe {
            (*pItem).zEName = unsafe {
                sqlite3DbStrNDup(
                    unsafe { (*pParse).db },
                    unsafe { (*pName).z },
                    (unsafe { (*pName).n }) as u64,
                )
            };
        }
        if dequote != (0 as i32) {
            // If dequote==0, then pName->z does not point to part of a DDL
            // statement handled by the parser. And so no token need be added
            // to the token-map.
            unsafe { sqlite3Dequote(unsafe { (*pItem).zEName }) };
            if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
                unsafe {
                    sqlite3RenameTokenMap(pParse, (unsafe { (*pItem).zEName }) as *const (), pName)
                };
            }
        }
    }
}

/// Set the ExprList.a[].zSpan element of the most recently added item
/// on the expression list.
///
/// pList might be NULL following an OOM error.  But pSpan should never be
/// NULL.  If a memory allocation fails, the pParse->db->mallocFailed flag
/// is set.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pList` - List to which to add the span.
/// * `zStart` - Start of the span
/// * `zEnd` - End of the span
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprListSetSpan(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut zStart: *const i8,
    mut zEnd: *const i8,
) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    0 as i32;
    if pList != std::ptr::null_mut::<ExprList>() {
        let mut pItem: *mut ExprList_item = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                .offset(((unsafe { (*pList).nExpr }) - (1 as i32)) as isize)
        };
        0 as i32;
        if (unsafe { (*pItem).zEName }) == std::ptr::null_mut::<i8>() {
            unsafe {
                (*pItem).zEName = unsafe { sqlite3DbSpanDup(db, zStart, zEnd) };
            }
            unsafe {
                (*pItem).fg.__slate_bits_0.__set_eEName((1 as i32) as u32);
            }
        }
    }
}

/// If the expression list pEList contains more than iLimit elements,
/// leave an error message in pParse.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprListCheckLength(
    mut pParse: *mut Parse,
    mut pEList: *mut ExprList,
    mut zObject: *const i8,
) {
    let mut mx: i32 = unsafe {
        *unsafe {
            unsafe { (*unsafe { (*pParse).db }).aLimit.as_mut_ptr() as *mut i32 }
                .offset((2 as i32) as isize)
        }
    };
    {}
    {}
    if pEList != std::ptr::null_mut::<ExprList>() && (unsafe { (*pEList).nExpr }) > mx {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"too many columns in %s\0".as_ptr() as *mut i8) as *const i8,
                zObject,
            )
        };
    }
}

/// Delete an entire expression list.
fn exprListDeleteNN(mut db: *mut sqlite3, mut pList: *mut ExprList) {
    let mut i: i32 = unsafe { (*pList).nExpr };
    let mut pItem: *mut ExprList_item =
        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item };
    0 as i32;
    0 as i32;
    '__slate_break_2243: loop {
        sqlite3ExprDelete(db, unsafe { (*pItem).pExpr });
        if (unsafe { (*pItem).zEName }) != std::ptr::null_mut::<i8>() {
            unsafe { sqlite3DbNNFreeNN(db, (unsafe { (*pItem).zEName }) as *mut ()) };
        }
        let __v2665: *mut ExprList_item = pItem;
        let __v2666: *mut ExprList_item = unsafe { __v2665.offset((1 as i32) as isize) };
        pItem = __v2666;
        let __v2667: i32 = i;
        let __v2668: i32 = __v2667 - (1 as i32);
        i = __v2668;
        if !(__v2668 > (0 as i32)) {
            break;
        }
    }
    unsafe { sqlite3DbNNFreeNN(db, pList as *mut ()) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprListDelete(mut db: *mut sqlite3, mut pList: *mut ExprList) {
    if pList != std::ptr::null_mut::<ExprList>() {
        exprListDeleteNN(db, pList);
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.expr.sqlite3ExprListDeleteGeneric")]
extern "C-unwind" fn sqlite3ExprListDeleteGeneric(mut db: *mut sqlite3, mut pList: *mut ()) {
    if pList != std::ptr::null_mut::<()>() {
        exprListDeleteNN(db, pList as *mut ExprList);
    }
}

/// Return the bitwise-OR of all Expr.flags fields in the given
/// ExprList.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprListFlags(mut pList: *const ExprList) -> u32 {
    let mut i: i32 = 0 as i32;
    let mut m: u32 = (0 as i32) as u32;
    0 as i32;
    i = 0 as i32;
    '__slate_break_2244: loop {
        if !(i < unsafe { (*pList).nExpr }) {
            break;
        }
        let mut pExpr: *mut Expr = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of!((*pList).a) as *const ExprList_item }.offset(i as isize)
            })
            .pExpr
        };
        0 as i32;
        let __v2427: u32 = m;
        let __v2428: u32 = __v2427 | unsafe { (*pExpr).flags };
        m = __v2428;
        let __v2425: i32 = i;
        let __v2426: i32 = __v2425 + (1 as i32);
        i = __v2426;
    }
    return m;
}

/// This is a SELECT-node callback for the expression walker that
/// always "fails".  By "fail" in this case, we mean set
/// pWalker->eCode to zero and abort.
///
/// This callback is used by multiple expression walkers.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.expr.sqlite3SelectWalkFail")]
extern "C-unwind" fn sqlite3SelectWalkFail(
    mut pWalker: *mut Walker,
    mut NotUsed: *mut Select,
) -> i32 {
    NotUsed;
    unsafe {
        (*pWalker).eCode = ((0 as i32) as i16) as u16;
    }
    return 2 as i32;
}

/// Check the input string to see if it is "true" or "false" (in any case).
///
///       If the string is....           Return
///         "true"                         EP_IsTrue
///         "false"                        EP_IsFalse
///         anything else                  0
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IsTrueOrFalse(mut zIn: *const i8) -> u32 {
    if (unsafe { sqlite3StrICmp(zIn, (b"true\0".as_ptr() as *mut i8) as *const i8) }) == (0 as i32)
    {
        return (268435456 as i32) as u32;
    }
    if (unsafe { sqlite3StrICmp(zIn, (b"false\0".as_ptr() as *mut i8) as *const i8) }) == (0 as i32)
    {
        return (536870912 as i32) as u32;
    }
    return (0 as i32) as u32;
}

/// If the input expression is an ID with the name "true" or "false"
/// then convert it into an TK_TRUEFALSE term.  Return non-zero if
/// the conversion happened, and zero if the expression is unaltered.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprIdToTrueFalse(mut pExpr: *mut Expr) -> i32 {
    let mut v: u32 = 0 as u32;
    0 as i32;
    let __v2506: bool;
    if !((unsafe { (*pExpr).flags }) & (((67108864 as i32) | (2048 as i32)) as u32)
        != ((0 as i32) as u32))
    {
        let __v2507: u32 = sqlite3IsTrueOrFalse((unsafe { (*pExpr).u.zToken }) as *const i8);
        v = __v2507;
        __v2506 = __v2507 != ((0 as i32) as u32);
    } else {
        __v2506 = false as bool;
    }
    if __v2506 {
        unsafe {
            (*pExpr).op = ((171 as i32) as i8) as u8;
        }
        let __v2508: *mut Expr = pExpr;
        let __v2509: u32 = unsafe { (*__v2508).flags };
        let __v2510: u32 = __v2509 | v;
        unsafe {
            (*__v2508).flags = __v2510;
        }
        return 1 as i32;
    }
    return 0 as i32;
}

/// The argument must be a TK_TRUEFALSE Expr node.  Return 1 if it is TRUE
/// and 0 if it is FALSE.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprTruthValue(mut pExpr: *const Expr) -> i32 {
    pExpr = sqlite3ExprSkipCollateAndLikely(pExpr as *mut Expr) as *const Expr;
    0 as i32;
    0 as i32;
    0 as i32;
    return (((unsafe { *unsafe { unsafe { (*pExpr).u.zToken }.offset((4 as i32) as isize) } })
        as i32)
        == (0 as i32)) as i32;
}

/// If pExpr is an AND or OR expression, try to simplify it by eliminating
/// terms that are always true or false.  Return the simplified expression.
/// Or return the original expression if no simplification is possible.
///
/// Examples:
///
///     (x<10) AND true                =>   (x<10)
///     (x<10) AND false               =>   false
///     (x<10) AND (y=22 OR false)     =>   (x<10) AND (y=22)
///     (x<10) AND (y=22 OR true)      =>   (x<10)
///     (y=22) OR true                 =>   true
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprSimplifiedAndOr(mut pExpr: *mut Expr) -> *mut Expr {
    0 as i32;
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (44 as i32)
        || (((unsafe { (*pExpr).op }) as u32) as i32) == (43 as i32)
    {
        let mut pRight: *mut Expr = sqlite3ExprSimplifiedAndOr(unsafe { (*pExpr).pRight });
        let mut pLeft: *mut Expr = sqlite3ExprSimplifiedAndOr(unsafe { (*pExpr).pLeft });
        if (unsafe { (*pLeft).flags }) & (((1 as i32) | (268435456 as i32)) as u32)
            == ((268435456 as i32) as u32)
            || (unsafe { (*pRight).flags }) & (((1 as i32) | (536870912 as i32)) as u32)
                == ((536870912 as i32) as u32)
        {
            pExpr = if (((unsafe { (*pExpr).op }) as u32) as i32) == (44 as i32) {
                pRight
            } else {
                pLeft
            };
        } else {
            if (unsafe { (*pRight).flags }) & (((1 as i32) | (268435456 as i32)) as u32)
                == ((268435456 as i32) as u32)
                || (unsafe { (*pLeft).flags }) & (((1 as i32) | (536870912 as i32)) as u32)
                    == ((536870912 as i32) as u32)
            {
                pExpr = if (((unsafe { (*pExpr).op }) as u32) as i32) == (44 as i32) {
                    pLeft
                } else {
                    pRight
                };
            }
        }
    }
    return pExpr;
}

/// Return true if it might be advantageous to compute the right operand
/// of expression pExpr first, before the left operand.
///
/// Normally the left operand is computed before the right operand.  But if
/// the left operand contains a subquery and the right does not, then it
/// might be more efficient to compute the right operand first.
fn exprEvalRhsFirst(mut pExpr: *mut Expr) -> i32 {
    if (unsafe { (*unsafe { (*pExpr).pLeft }).flags }) & ((4194304 as i32) as u32)
        != ((0 as i32) as u32)
        && !((unsafe { (*unsafe { (*pExpr).pRight }).flags }) & ((4194304 as i32) as u32)
            != ((0 as i32) as u32))
    {
        return 1 as i32;
    } else {
        return 0 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

/// Compute the two operands of a binary operator.
///
/// If either operand contains a subquery, then the code strives to
/// compute the operand containing the subquery second.  If the other
/// operand evalutes to NULL, then a jump is made.  The address of the
/// IsNull operand that does this jump is returned.  The caller can use
/// this to optimize the computation so as to avoid doing the potentially
/// expensive subquery.
///
/// If no optimization opportunities exist, return 0.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pExpr` - The comparison expression
/// * `pR1` - OUT: Register holding the left operand
/// * `pR2` - OUT: Register holding the right operand
/// * `pFree1` - OUT: Temp register to free if not zero
/// * `pFree2` - OUT: Another temp register to free if not zero
fn exprComputeOperands(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut pR1: *mut i32,
    mut pR2: *mut i32,
    mut pFree1: *mut i32,
    mut pFree2: *mut i32,
) -> i32 {
    let mut addrIsNull: i32 = 0 as i32;
    let mut r1: i32 = 0 as i32;
    let mut r2: i32 = 0 as i32;
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    0 as i32;
    // If the left operand contains a (possibly expensive) subquery and the
    // right operand does not and the right operation might be NULL,
    // then compute the right operand first and do an IsNull jump if the
    // right operand evalutes to NULL.
    let __v2669: bool;
    if exprEvalRhsFirst(pExpr) != (0 as i32) {
        __v2669 = sqlite3ExprCanBeNull((unsafe { (*pExpr).pRight }) as *const Expr) != (0 as i32);
    } else {
        __v2669 = false as bool;
    }
    if __v2669 {
        r2 = sqlite3ExprCodeTemp(pParse, unsafe { (*pExpr).pRight }, pFree2);
        addrIsNull = unsafe { sqlite3VdbeAddOp1(v, 51 as i32, r2) };
        unsafe { sqlite3VdbeComment(v, (b"skip left operand\0".as_ptr() as *mut i8) as *const i8) };
        {}
    } else {
        r2 = 0 as i32; // Silence a false-positive uninit-var warning in MSVC
        addrIsNull = 0 as i32;
    }
    r1 = sqlite3ExprCodeTemp(pParse, unsafe { (*pExpr).pLeft }, pFree1);
    if addrIsNull == (0 as i32) {
        // If the right operand contains a subquery and the left operand does not
        // and the left operand might be NULL, then do an IsNull check
        // check on the left operand before computing the right operand.
        let __v2670: bool;
        if (unsafe { (*unsafe { (*pExpr).pRight }).flags }) & ((4194304 as i32) as u32)
            != ((0 as i32) as u32)
        {
            __v2670 =
                sqlite3ExprCanBeNull((unsafe { (*pExpr).pLeft }) as *const Expr) != (0 as i32);
        } else {
            __v2670 = false as bool;
        }
        if __v2670 {
            addrIsNull = unsafe { sqlite3VdbeAddOp1(v, 51 as i32, r1) };
            unsafe {
                sqlite3VdbeComment(
                    v,
                    (b"skip right operand\0".as_ptr() as *mut i8) as *const i8,
                )
            };
            {}
        }
        r2 = sqlite3ExprCodeTemp(pParse, unsafe { (*pExpr).pRight }, pFree2);
    }
    unsafe {
        *pR1 = r1;
    }
    unsafe {
        *pR2 = r2;
    }
    return addrIsNull;
}

/// pExpr is a TK_FUNCTION node.  Try to determine whether or not the
/// function is a constant function.  A function is constant if all of
/// the following are true:
///
///    (1)  It is a scalar function (not an aggregate or window function)
///    (2)  It has either the SQLITE_FUNC_CONSTANT or SQLITE_FUNC_SLOCHNG
///         property.
///    (3)  All of its arguments are constants
///
/// This routine sets pWalker->eCode to 0 if pExpr is not a constant.
/// It makes no changes to pWalker->eCode if pExpr is constant.  In
/// every case, it returns WRC_Abort.
///
/// Called as a service subroutine from exprNodeIsConstant().
fn exprNodeIsConstantFunction(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    let mut n: i32 = 0 as i32; // Number of arguments
    let mut pList: *mut ExprList = unsafe { std::mem::zeroed() }; // List of arguments
    let mut pDef: *mut FuncDef = unsafe { std::mem::zeroed() }; // The function
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() }; // The database
    0 as i32;
    let __v2671: bool;
    if (unsafe { (*pExpr).flags }) & ((65536 as i32) as u32) != ((0 as i32) as u32) {
        __v2671 = true as bool;
    } else {
        let __v2672: *mut ExprList = unsafe { (*pExpr).x.pList };
        pList = __v2672;
        __v2671 = __v2672 == std::ptr::null_mut::<ExprList>();
    }
    if __v2671 {
        {}
        n = 0 as i32;
    } else {
        n = unsafe { (*pList).nExpr };
        unsafe { sqlite3WalkExprList(pWalker, pList) };
        if (((unsafe { (*pWalker).eCode }) as u32) as i32) == (0 as i32) {
            return 2 as i32;
        }
    }
    db = unsafe { (*unsafe { (*pWalker).pParse }).db };
    pDef = unsafe {
        sqlite3FindFunction(
            db,
            (unsafe { (*pExpr).u.zToken }) as *const i8,
            n,
            unsafe { (*db).enc },
            ((0 as i32) as i8) as u8,
        )
    };
    if pDef == std::ptr::null_mut::<FuncDef>()
        || (unsafe { (*pDef).xFinalize }) != None
        || (unsafe { (*pDef).funcFlags }) & (((2048 as i32) | (8192 as i32)) as u32)
            == ((0 as i32) as u32)
        || (unsafe { (*pExpr).flags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32)
    {
        unsafe {
            (*pWalker).eCode = ((0 as i32) as i16) as u16;
        }
        return 2 as i32;
    }
    return 1 as i32;
}

/// These routines are Walker callbacks used to check expressions to
/// see if they are "constant" for some definition of constant.  The
/// Walker.eCode value determines the type of "constant" we are looking
/// for.
///
/// These callback routines are used to implement the following:
///
///     sqlite3ExprIsConstant()                  pWalker->eCode==1
///     sqlite3ExprIsConstantNotJoin()           pWalker->eCode==2
///     sqlite3ExprIsTableConstant()             pWalker->eCode==3
///     sqlite3ExprIsConstantOrFunction()        pWalker->eCode==4 or 5
///     sqlite3ExprListIsConstant()              pWalker->eCode==1 or 6
///
/// In all cases, the callbacks set Walker.eCode=0 and abort if the expression
/// is found to not be a constant.
///
/// The sqlite3ExprIsConstantOrFunction() is used for evaluating DEFAULT
/// expressions in a CREATE TABLE statement.  The Walker.eCode value is 5
/// when parsing an existing schema out of the sqlite_schema table and 4
/// when processing a new CREATE TABLE statement.  A bound parameter raises
/// an error for new statements, but is silently converted
/// to NULL for existing schemas.  This allows sqlite_schema tables that
/// contain a bound parameter because they were generated by older versions
/// of SQLite to be parsed by newer versions of SQLite without raising a
/// malformed schema error.
#[unsafe(link_section = ".text.slate_distinct.expr.exprNodeIsConstant")]
extern "C-unwind" fn exprNodeIsConstant(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    0 as i32;
    // If pWalker->eCode is 2 then any term of the expression that comes from
    // the ON or USING clauses of an outer join disqualifies the expression
    // from being considered constant.
    if (((unsafe { (*pWalker).eCode }) as u32) as i32) == (2 as i32)
        && (unsafe { (*pExpr).flags }) & ((1 as i32) as u32) != ((0 as i32) as u32)
    {
        unsafe {
            (*pWalker).eCode = ((0 as i32) as i16) as u16;
        }
        return 2 as i32;
    }
    let mut __t0: i64 = match ((unsafe { (*pExpr).op }) as u32) as i32 {
        172 => 0,
        60 => 1,
        168 | 169 | 170 => 2,
        179 | 176 | 142 | 72 => 3,
        45 | 46 => 4,
        157 => 5,
        _ => 6,
    };
    '__slate_break_2249: loop {
        match __t0 {
            0 => {
                // Consider functions to be constant if all their arguments are constant
                // and either pWalker->eCode==4 or 5 or the function has the
                // SQLITE_FUNC_CONST flag.
                if ((((unsafe { (*pWalker).eCode }) as u32) as i32) >= (4 as i32)
                    || (unsafe { (*pExpr).flags }) & ((1048576 as i32) as u32)
                        != ((0 as i32) as u32))
                    && !((unsafe { (*pExpr).flags }) & ((16777216 as i32) as u32)
                        != ((0 as i32) as u32))
                {
                    if (((unsafe { (*pWalker).eCode }) as u32) as i32) == (5 as i32) {
                        let __v2673: *mut Expr = pExpr;
                        let __v2674: u32 = unsafe { (*__v2673).flags };
                        let __v2675: u32 = __v2674 | ((1073741824 as i32) as u32);
                        unsafe {
                            (*__v2673).flags = __v2675;
                        }
                    }
                    return 0 as i32;
                } else {
                    if (unsafe { (*pWalker).pParse }) != std::ptr::null_mut::<Parse>() {
                        return exprNodeIsConstantFunction(pWalker, pExpr);
                    } else {
                        unsafe {
                            (*pWalker).eCode = ((0 as i32) as i16) as u16;
                        }
                        return 2 as i32;
                    }
                }
                __t0 = 1;
                continue '__slate_break_2249;
            }
            1 => {
                if sqlite3ExprIdToTrueFalse(pExpr) != (0 as i32) {
                    return 1 as i32;
                }
                // Convert "true" or "false" in a DEFAULT clause into the
                // appropriate TK_TRUEFALSE operator
                // no break
                {}
                __t0 = 2;
                continue '__slate_break_2249;
            }
            2 => {
                {}
                {}
                {}
                {}
                if (unsafe { (*pExpr).flags }) & ((32 as i32) as u32) != ((0 as i32) as u32)
                    && (((unsafe { (*pWalker).eCode }) as u32) as i32) != (2 as i32)
                {
                    return 0 as i32;
                }
                if (((unsafe { (*pWalker).eCode }) as u32) as i32) == (3 as i32)
                    && (unsafe { (*pExpr).iTable }) == unsafe { (*pWalker).u.iCur }
                {
                    return 0 as i32;
                }
                // no break
                {}
                __t0 = 3;
                continue '__slate_break_2249;
            }
            3 => {
                {}
                {}
                {}
                {}
                unsafe {
                    (*pWalker).eCode = ((0 as i32) as i16) as u16;
                }
                return 2 as i32;
            }
            4 => {
                if (((unsafe { (*pWalker).eCode }) as u32) as i32) == (6 as i32) {
                    unsafe {
                        (*pWalker).eCode = ((0 as i32) as i16) as u16;
                    }
                    return 2 as i32;
                }
                return 0 as i32;
            }
            5 => {
                if (((unsafe { (*pWalker).eCode }) as u32) as i32) == (5 as i32) {
                    // Silently convert bound parameters that appear inside of CREATE
                    // statements into a NULL when parsing the CREATE statement text out
                    // of the sqlite_schema table
                    unsafe {
                        (*pExpr).op = ((122 as i32) as i8) as u8;
                    }
                } else {
                    if (((unsafe { (*pWalker).eCode }) as u32) as i32) == (4 as i32) {
                        // A bound parameter in a CREATE statement that originates from
                        // sqlite3_prepare() causes an error
                        unsafe {
                            (*pWalker).eCode = ((0 as i32) as i16) as u16;
                        }
                        return 2 as i32;
                    }
                }
                // no break
                {}
                __t0 = 6;
                continue '__slate_break_2249;
            }
            6 => {
                {}
                // sqlite3SelectWalkFail() disallows
                {}
                // sqlite3SelectWalkFail() disallows
                return 0 as i32;
            }
            _ => {
                break '__slate_break_2249;
            }
        }
    }
    return unsafe { std::mem::zeroed() };
}

fn exprIsConst(mut pParse: *mut Parse, mut p: *mut Expr, mut initFlag: i32) -> i32 {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    w.eCode = (initFlag as i16) as u16;
    w.pParse = pParse;
    w.xExprCallback = Some(exprNodeIsConstant);
    w.xSelectCallback = Some(sqlite3SelectWalkFail);
    unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(w), p) };
    return (w.eCode as u32) as i32;
}

/// Walk an expression tree.  Return non-zero if the expression is constant
/// or return zero if the expression involves variables or function calls.
///
/// For the purposes of this function, a double-quoted string (ex: "abc")
/// is considered a variable but a single-quoted string (ex: 'abc') is
/// a constant.
///
/// The pParse parameter may be NULL.  But if it is NULL, there is no way
/// to determine if function calls are constant or not, and hence all
/// function calls will be considered to be non-constant.  If pParse is
/// not NULL, then a function call might be constant, depending on the
/// function and on its parameters.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprIsConstant(mut pParse: *mut Parse, mut p: *mut Expr) -> i32 {
    return exprIsConst(pParse, p, 1 as i32);
}

/// Return true if all expressions in pList are constant. If parameter bNoIs
/// is true, do not consider expressions that contain "IS" operators to be
/// constant. IS operators are not always considered constant as expressions
/// like "x IS TRUE" need to be transformed to TK_TRUTH expression nodes,
/// which happens at the same time as column name resolution.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprListIsConstant(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut bNoIs: i32,
) -> i32 {
    let mut iInit: i32 = if bNoIs != (0 as i32) {
        6 as i32
    } else {
        1 as i32
    };
    let mut ii: i32 = 0 as i32;
    ii = 0 as i32;
    '__slate_break_2250: loop {
        if !(ii < unsafe { (*pList).nExpr }) {
            break;
        }
        if (0 as i32)
            == exprIsConst(
                pParse,
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                            .offset(ii as isize)
                    })
                    .pExpr
                },
                iInit,
            )
        {
            return 0 as i32;
        }
        let __v2513: i32 = ii;
        let __v2514: i32 = __v2513 + (1 as i32);
        ii = __v2514;
    }
    return 1 as i32;
}

/// Walk an expression tree.  Return non-zero if
///
///   (1) the expression is constant, and
///   (2) the expression does not originate in the ON or USING clause
///       of a LEFT JOIN, and
///   (3) the expression does not contain any EP_FixedCol TK_COLUMN
///       operands created by the constant propagation optimization.
///
/// When this routine returns true, it indicates that the expression
/// can be added to the pParse->pConstExpr list and evaluated once when
/// the prepared statement starts up.  See sqlite3ExprCodeRunJustOnce().
fn sqlite3ExprIsConstantNotJoin(mut pParse: *mut Parse, mut p: *mut Expr) -> i32 {
    return exprIsConst(pParse, p, 2 as i32);
}

/// This routine examines sub-SELECT statements as an expression is being
/// walked as part of sqlite3ExprIsTableConstant().  Sub-SELECTs are considered
/// constant as long as they are uncorrelated - meaning that they do not
/// contain any terms from outer contexts.
#[unsafe(link_section = ".text.slate_distinct.expr.exprSelectWalkTableConstant")]
extern "C-unwind" fn exprSelectWalkTableConstant(
    mut pWalker: *mut Walker,
    mut pSelect: *mut Select,
) -> i32 {
    0 as i32;
    0 as i32;
    if (unsafe { (*pSelect).selFlags }) & ((536870912 as i32) as u32) != ((0 as i32) as u32) {
        unsafe {
            (*pWalker).eCode = ((0 as i32) as i16) as u16;
        }
        return 2 as i32;
    }
    return 1 as i32;
}

/// Walk an expression tree.  Return non-zero if the expression is constant
/// for any single row of the table with cursor iCur.  In other words, the
/// expression must not refer to any non-deterministic function nor any
/// table other than iCur.
///
/// Consider uncorrelated subqueries to be constants if the bAllowSubq
/// parameter is true.
fn sqlite3ExprIsTableConstant(mut p: *mut Expr, mut iCur: i32, mut bAllowSubq: i32) -> i32 {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    w.eCode = ((3 as i32) as i16) as u16;
    w.pParse = std::ptr::null_mut::<Parse>();
    w.xExprCallback = Some(exprNodeIsConstant);
    if bAllowSubq != (0 as i32) {
        w.xSelectCallback = Some(exprSelectWalkTableConstant);
    } else {
        w.xSelectCallback = Some(sqlite3SelectWalkFail);
    }
    unsafe {
        w.u.iCur = iCur;
    }
    unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(w), p) };
    return (w.eCode as u32) as i32;
}

/// Check pExpr to see if it is an constraint on the single data source
/// pSrc = &pSrcList->a[iSrc].  In other words, check to see if pExpr
/// constrains pSrc but does not depend on any other tables or data
/// sources anywhere else in the query.  Return true (non-zero) if pExpr
/// is a constraint on pSrc only.
///
/// This is an optimization.  False negatives will perhaps cause slower
/// queries, but false positives will yield incorrect answers.  So when in
/// doubt, return 0.
///
/// To be an single-source constraint, the following must be true:
///
///   (1)  pExpr cannot refer to any table other than pSrc->iCursor.
///
///   (2a) pExpr cannot use subqueries unless the bAllowSubq parameter is
///        true and the subquery is non-correlated
///
///   (2b) pExpr cannot use non-deterministic functions.
///
///   (3)  pSrc cannot be part of the left operand for a RIGHT JOIN.
///        (Is there some way to relax this constraint?)
///
///   (4)  If pSrc is the right operand of a LEFT JOIN, then...
///         (4a)  pExpr must come from an ON clause..
///         (4b)  and specifically the ON clause associated with the LEFT JOIN.
///
///   (5)  If pSrc is the right operand of a LEFT JOIN or the left
///        operand of a RIGHT JOIN, then pExpr must be from the WHERE
///        clause, not an ON clause.
///
///   (6) Either:
///
///       (6a) pExpr does not originate in an ON or USING clause, or
///
///       (6b) The ON or USING clause from which pExpr is derived is
///            not to the left of a RIGHT JOIN (or FULL JOIN).
///
///       Without this restriction, accepting pExpr as a single-table
///       constraint might move the the ON/USING filter expression
///       from the left side of a RIGHT JOIN over to the right side,
///       which leads to incorrect answers.  See also restriction (9)
///       on push-down.
///
/// # Arguments
///
/// * `pExpr` - The constraint
/// * `pSrcList` - Complete FROM clause
/// * `iSrc` - Which element of pSrcList to use
/// * `bAllowSubq` - Allow non-correlated subqueries
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprIsSingleTableConstraint(
    mut pExpr: *mut Expr,
    mut pSrcList: *const SrcList,
    mut iSrc: i32,
    mut bAllowSubq: i32,
) -> i32 {
    let mut pSrc: *const SrcItem = unsafe {
        unsafe { std::ptr::addr_of!((*pSrcList).a) as *const SrcItem }.offset(iSrc as isize)
    };
    if (((unsafe { (*pSrc).fg.jointype }) as u32) as i32) & (64 as i32) != (0 as i32) {
        return 0 as i32; // rule (3)
    }
    if (((unsafe { (*pSrc).fg.jointype }) as u32) as i32) & (8 as i32) != (0 as i32) {
        if !((unsafe { (*pExpr).flags }) & ((1 as i32) as u32) != ((0 as i32) as u32)) {
            return 0 as i32;
        }
        // rule (4a)
        if (unsafe { (*pExpr).w.iJoin }) != unsafe { (*pSrc).iCursor } {
            return 0 as i32;
        }
    // rule (4b)
    } else {
        if (unsafe { (*pExpr).flags }) & ((1 as i32) as u32) != ((0 as i32) as u32) {
            return 0 as i32;
        }
        // rule (5)
    }
    if (unsafe { (*pExpr).flags }) & (((1 as i32) | (2 as i32)) as u32) != ((0 as i32) as u32)
        && (((unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of!((*pSrcList).a) as *const SrcItem }
                    .offset((0 as i32) as isize)
            })
            .fg
            .jointype
        }) as u32) as i32)
            & (64 as i32)
            != (0 as i32)
    {
        let mut jj: i32 = 0 as i32;
        jj = 0 as i32;
        '__slate_break_2251: loop {
            if !(jj < iSrc) {
                break;
            }
            if (unsafe { (*pExpr).w.iJoin })
                == unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of!((*pSrcList).a) as *const SrcItem }
                            .offset(jj as isize)
                    })
                    .iCursor
                }
            {
                if (((unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of!((*pSrcList).a) as *const SrcItem }
                            .offset(jj as isize)
                    })
                    .fg
                    .jointype
                }) as u32) as i32)
                    & (64 as i32)
                    != (0 as i32)
                {
                    return 0 as i32; // restriction (6)
                }
                break '__slate_break_2251;
            }
            let __v2511: i32 = jj;
            let __v2512: i32 = __v2511 + (1 as i32);
            jj = __v2512;
        }
    }
    // (6a)
    // Fast pre-test of (6b)
    // Rules (1), (2a), and (2b) handled by the following:
    return sqlite3ExprIsTableConstant(pExpr, unsafe { (*pSrc).iCursor }, bAllowSubq);
}

/// sqlite3WalkExpr() callback used by sqlite3ExprIsConstantOrGroupBy().
#[unsafe(link_section = ".text.slate_distinct.expr.exprNodeIsConstantOrGroupBy")]
extern "C-unwind" fn exprNodeIsConstantOrGroupBy(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> i32 {
    let mut pGroupBy: *mut ExprList = unsafe { (*pWalker).u.pGroupBy };
    let mut i: i32 = 0 as i32;
    // Check if pExpr is identical to any GROUP BY term. If so, consider
    // it constant.
    i = 0 as i32;
    '__slate_break_2252: loop {
        if !(i < unsafe { (*pGroupBy).nExpr }) {
            break;
        }
        let mut p: *mut Expr = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pGroupBy).a) as *mut ExprList_item }
                    .offset(i as isize)
            })
            .pExpr
        };
        if sqlite3ExprCompare(
            std::ptr::null::<Parse>(),
            pExpr as *const Expr,
            p as *const Expr,
            -(1 as i32),
        ) < (2 as i32)
        {
            let mut pColl: *mut CollSeq =
                sqlite3ExprNNCollSeq(unsafe { (*pWalker).pParse }, p as *const Expr);
            if (unsafe { sqlite3IsBinary(pColl as *const CollSeq) }) != (0 as i32) {
                return 1 as i32;
            }
        }
        let __v2676: i32 = i;
        let __v2677: i32 = __v2676 + (1 as i32);
        i = __v2677;
    }
    // Check if pExpr is a sub-select. If so, consider it variable.
    if (unsafe { (*pExpr).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
        unsafe {
            (*pWalker).eCode = ((0 as i32) as i16) as u16;
        }
        return 2 as i32;
    }
    return exprNodeIsConstant(pWalker, pExpr);
}

/// Walk the expression tree passed as the first argument. Return non-zero
/// if the expression consists entirely of constants or copies of terms
/// in pGroupBy that sort with the BINARY collation sequence.
///
/// This routine is used to determine if a term of the HAVING clause can
/// be promoted into the WHERE clause.  In order for such a promotion to work,
/// the value of the HAVING clause term must be the same for all members of
/// a "group".  The requirement that the GROUP BY term must be BINARY
/// assumes that no other collating sequence will have a finer-grained
/// grouping than binary.  In other words (A=B COLLATE binary) implies
/// A=B in every other collating sequence.  The requirement that the
/// GROUP BY be BINARY is stricter than necessary.  It would also work
/// to promote HAVING clauses that use the same alternative collating
/// sequence as the GROUP BY term, but that is much harder to check,
/// alternative collating sequences are uncommon, and this is only an
/// optimization, so we take the easy way out and simply require the
/// GROUP BY to use the BINARY collating sequence.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprIsConstantOrGroupBy(
    mut pParse: *mut Parse,
    mut p: *mut Expr,
    mut pGroupBy: *mut ExprList,
) -> i32 {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    w.eCode = ((1 as i32) as i16) as u16;
    w.xExprCallback = Some(exprNodeIsConstantOrGroupBy);
    w.xSelectCallback = None;
    unsafe {
        w.u.pGroupBy = pGroupBy;
    }
    w.pParse = pParse;
    unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(w), p) };
    return (w.eCode as u32) as i32;
}

/// Walk an expression tree for the DEFAULT field of a column definition
/// in a CREATE TABLE statement.  Return non-zero if the expression is
/// acceptable for use as a DEFAULT.  That is to say, return non-zero if
/// the expression is constant or a function call with constant arguments.
/// Return and 0 if there are any variables.
///
/// isInit is true when parsing from sqlite_schema.  isInit is false when
/// processing a new CREATE TABLE statement.  When isInit is true, parameters
/// (such as ? or $abc) in the expression are converted into NULL.  When
/// isInit is false, parameters raise an error.  Parameters should not be
/// allowed in a CREATE TABLE statement, but some legacy versions of SQLite
/// allowed it, so we need to support it when reading sqlite_schema for
/// backwards compatibility.
///
/// If isInit is true, set EP_FromDDL on every TK_FUNCTION node.
///
/// For the purposes of this function, a double-quoted string (ex: "abc")
/// is considered a variable but a single-quoted string (ex: 'abc') is
/// a constant.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprIsConstantOrFunction(mut p: *mut Expr, mut isInit: u8) -> i32 {
    0 as i32;
    return exprIsConst(
        std::ptr::null_mut::<Parse>(),
        p,
        (4 as i32) + ((isInit as u32) as i32),
    );
}

/// If the expression p codes a constant integer between 0 and 0x7fffffff,
/// then return 1 and put the value of the integer in *pValue.  If the
/// expression is not an integer or if it is an integer that is out side
/// the range of 0...0x7fffffff, then return 0 and leave *pValue unchanged.
///
/// If the pParse pointer is provided, then allow the expression p to be
/// a parameter (TK_VARIABLE) that is bound to an integer between 0 and
/// 0x7fffffff.  Variables that hold anything other than integers, or that
/// hold integers outside the range of 0..0x7fffffff are not seen.
/// But if pParse is NULL, then p must be a pure integer literal between
/// 0 and 0x7fffffff.
///
/// If pParse is not NULL and expression p is a variable, then the variable
/// is marked so as to cause the statement to be reprepared each time a new
/// value is bound to it. Except, if parameter bRSI is true, then the statement
/// will only be reprepared if the rebind changes the value to or from a
/// "small integer" (either 0 or 1).  Note that if p is a variable then
/// reprepare is always enabled for that variable, regardless of its current
/// binding.  The RSI is only enabled if the current binding is a small
/// integer.  "RSI" stands for "Reprepare Small Integers".
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprIsInteger(
    mut p: *const Expr,
    mut pValue: *mut i32,
    mut pParse: *mut Parse,
    mut bRSI: i32,
) -> i32 {
    let mut iSign: i32 = 1 as i32; // Either +1 or -1.
    '__slate_break_2253: while (1 as i32) != (0 as i32) {
        // exit-by-break
        if p == std::ptr::null::<Expr>() {
            return 0 as i32;
        }
        if (unsafe { (*p).flags }) & ((2048 as i32) as u32) != ((0 as i32) as u32) {
            unsafe {
                *pValue = (unsafe { (*p).u.iValue }) * iSign;
            }
            return 1 as i32;
        }
        if (((unsafe { (*p).op }) as u32) as i32) == (173 as i32) {
            p = (unsafe { (*p).pLeft }) as *const Expr;
            pParse = std::ptr::null_mut::<Parse>();
        } else {
            if (((unsafe { (*p).op }) as u32) as i32) == (174 as i32) {
                iSign = -iSign;
                p = (unsafe { (*p).pLeft }) as *const Expr;
                pParse = std::ptr::null_mut::<Parse>();
            } else {
                if (((unsafe { (*p).op }) as u32) as i32) == (157 as i32)
                    && pParse != std::ptr::null_mut::<Parse>()
                {
                    let mut pVal: *mut sqlite3_value = unsafe { std::mem::zeroed() }; // The variable
                    let mut isSmall: i32 = 0 as i32; // Only reprepare if change to/from small integer
                    let mut rc: i32 = 0 as i32; // 1 if successful, 0 if failed
                    0 as i32;
                    if (unsafe { (*pParse).pVdbe }) == std::ptr::null_mut::<Vdbe>() {
                        break '__slate_break_2253;
                    }
                    if (unsafe { (*unsafe { (*pParse).db }).flags })
                        & (((8388608 as i32) as i64) as u64)
                        != (((0 as i32) as i64) as u64)
                    {
                        break '__slate_break_2253;
                    }
                    pVal = unsafe {
                        sqlite3VdbeGetBoundValue(
                            unsafe { (*pParse).pReprepare },
                            (unsafe { (*p).iColumn }) as i32,
                            ((65 as i32) as i8) as u8,
                        )
                    };
                    if pVal != std::ptr::null_mut::<sqlite3_value>() {
                        let mut vv: i64 = 0 as i64;
                        let __v2515: bool;
                        if (unsafe { sqlite3_value_type(pVal) }) == (1 as i32) {
                            let __v2516: i64 = unsafe { sqlite3_value_int64(pVal) };
                            vv = __v2516;
                            __v2515 = __v2516 >= ((0 as i32) as i64);
                        } else {
                            __v2515 = false as bool;
                        }
                        if __v2515 && vv <= ((2147483647 as i32) as i64) {
                            unsafe {
                                *pValue = vv as i32;
                            }
                            if bRSI != (0 as i32) {
                                isSmall = (vv <= ((1 as i32) as i64)) as i32;
                            }
                            rc = 1 as i32;
                        }
                        unsafe { sqlite3ValueFree(pVal) };
                    }
                    unsafe {
                        sqlite3VdbeReprepareOnBind(
                            unsafe { (*pParse).pVdbe },
                            (unsafe { (*p).iColumn }) as i32,
                            isSmall,
                        )
                    };
                    return rc;
                }
                break '__slate_break_2253;
            }
        }
    }
    return 0 as i32;
}

/// Return FALSE if there is no chance that the expression can be NULL.
///
/// If the expression might be NULL or if the expression is too complex
/// to tell return TRUE.
///
/// This routine is used as an optimization, to skip OP_IsNull opcodes
/// when we know that a value cannot be NULL.  Hence, a false positive
/// (returning TRUE when in fact the expression can never be NULL) might
/// be a small performance hit but is otherwise harmless.  On the other
/// hand, a false negative (returning FALSE when the result could be NULL)
/// will likely result in an incorrect answer.  So when in doubt, return
/// TRUE.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCanBeNull(mut p: *const Expr) -> i32 {
    let mut op: u8 = 0 as u8;
    0 as i32;
    '__slate_break_2254: while (((unsafe { (*p).op }) as u32) as i32) == (173 as i32)
        || (((unsafe { (*p).op }) as u32) as i32) == (174 as i32)
    {
        p = (unsafe { (*p).pLeft }) as *const Expr;
        0 as i32;
    }
    op = unsafe { (*p).op };
    if ((op as u32) as i32) == (176 as i32) {
        op = unsafe { (*p).op2 };
    }
    match (op as u32) as i32 {
        156 | 118 | 154 | 155 => {
            return 0 as i32;
        }
        168 => {
            0 as i32;
            return ((unsafe { (*p).flags }) & ((2097152 as i32) as u32) != ((0 as i32) as u32)
                || (unsafe { (*p).y.pTab }) == std::ptr::null_mut::<Table>()
                || ((unsafe { (*p).iColumn }) as i32) >= (0 as i32)
                    && (unsafe { (*unsafe { (*p).y.pTab }).aCol })
                        != std::ptr::null_mut::<Column>()
                    && ((unsafe { (*p).iColumn }) as i32)
                        < ((unsafe { (*unsafe { (*p).y.pTab }).nCol }) as i32)
                    && ((unsafe {
                        (*unsafe {
                            unsafe { (*unsafe { (*p).y.pTab }).aCol }
                                .offset(((unsafe { (*p).iColumn }) as i32) as isize)
                        })
                        .__slate_bits_0
                        .__get_notNull()
                    }) as i32)
                        == (0 as i32)) as i32; // Reference to column of index on expr
            // Possible due to prior error
        }
        _ => {
            return 1 as i32;
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Return TRUE if the given expression is a constant which would be
/// unchanged by OP_Affinity with the affinity given in the second
/// argument.
///
/// This routine is used to determine if the OP_Affinity operation
/// can be omitted.  When in doubt return FALSE.  A false negative
/// is harmless.  A false positive, however, can result in the wrong
/// answer.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprNeedsNoAffinityChange(mut p: *const Expr, mut aff: i8) -> i32 {
    let mut op: u8 = 0 as u8;
    let mut unaryMinus: i32 = 0 as i32;
    if (aff as i32) == (65 as i32) {
        return 1 as i32;
    }
    '__slate_break_2256: while (((unsafe { (*p).op }) as u32) as i32) == (173 as i32)
        || (((unsafe { (*p).op }) as u32) as i32) == (174 as i32)
    {
        if (((unsafe { (*p).op }) as u32) as i32) == (174 as i32) {
            unaryMinus = 1 as i32;
        }
        p = (unsafe { (*p).pLeft }) as *const Expr;
    }
    op = unsafe { (*p).op };
    if ((op as u32) as i32) == (176 as i32) {
        op = unsafe { (*p).op2 };
    }
    match (op as u32) as i32 {
        156 => {
            return ((aff as i32) >= (67 as i32)) as i32;
        }
        154 => {
            return ((aff as i32) >= (67 as i32)) as i32;
        }
        118 => {
            return (!(unaryMinus != (0 as i32)) && (aff as i32) == (66 as i32)) as i32;
        }
        155 => {
            return !(unaryMinus != (0 as i32)) as i32;
        }
        168 => {
            0 as i32; // p cannot be part of a CHECK constraint
            return ((aff as i32) >= (67 as i32) && ((unsafe { (*p).iColumn }) as i32) < (0 as i32))
                as i32;
        }
        _ => {
            return 0 as i32;
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Return TRUE if the given string is a row-id column name.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IsRowid(mut z: *const i8) -> i32 {
    if (unsafe { sqlite3StrICmp(z, (b"_ROWID_\0".as_ptr() as *mut i8) as *const i8) }) == (0 as i32)
    {
        return 1 as i32;
    }
    if (unsafe { sqlite3StrICmp(z, (b"ROWID\0".as_ptr() as *mut i8) as *const i8) }) == (0 as i32) {
        return 1 as i32;
    }
    if (unsafe { sqlite3StrICmp(z, (b"OID\0".as_ptr() as *mut i8) as *const i8) }) == (0 as i32) {
        return 1 as i32;
    }
    return 0 as i32;
}

/// Return a pointer to a buffer containing a usable rowid alias for table
/// pTab. An alias is usable if there is not an explicit user-defined column
/// of the same name.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RowidAlias(mut pTab: *mut Table) -> *const i8 {
    let mut azOpt: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
        (b"_ROWID_\0".as_ptr() as *mut i8) as *const i8,
        (b"ROWID\0".as_ptr() as *mut i8) as *const i8,
        (b"OID\0".as_ptr() as *mut i8) as *const i8,
    ]);
    let mut ii: i32 = 0 as i32;
    0 as i32;
    ii = 0 as i32;
    '__slate_break_2264: loop {
        if !(ii < ((((24 as u64) / (8 as u64)) as u32) as i32)) {
            break;
        }
        if (unsafe {
            sqlite3ColumnIndex(pTab, unsafe {
                *unsafe { (azOpt.0.as_mut_ptr() as *mut *const i8).offset(ii as isize) }
            })
        }) < (0 as i32)
        {
            return unsafe {
                *unsafe { (azOpt.0.as_mut_ptr() as *mut *const i8).offset(ii as isize) }
            };
        }
        let __v2517: i32 = ii;
        let __v2518: i32 = __v2517 + (1 as i32);
        ii = __v2518;
    }
    return std::ptr::null::<i8>();
}

/// pX is the RHS of an IN operator.  If pX is a SELECT statement
/// that can be simplified to a direct table access, then return
/// a pointer to the SELECT statement.  If pX is not a SELECT statement,
/// or if the SELECT statement needs to be materialized into a transient
/// table, then return NULL.
fn isCandidateForInOpt(mut pX: *const Expr) -> *mut Select {
    let mut p: *mut Select = unsafe { std::mem::zeroed() };
    let mut pSrc: *mut SrcList = unsafe { std::mem::zeroed() };
    let mut pEList: *mut ExprList = unsafe { std::mem::zeroed() };
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    if !((unsafe { (*pX).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32)) {
        return std::ptr::null_mut::<Select>();
    }
    // Not a subquery
    if (unsafe { (*pX).flags }) & ((64 as i32) as u32) != ((0 as i32) as u32) {
        return std::ptr::null_mut::<Select>();
    }
    // Correlated subq
    p = unsafe { (*pX).x.pSelect };
    if (unsafe { (*p).pPrior }) != std::ptr::null_mut::<Select>() {
        return std::ptr::null_mut::<Select>();
    }
    // Not a compound SELECT
    if (unsafe { (*p).selFlags }) & ((8 as i32) as u32) != (0 as u32) {
        return std::ptr::null_mut::<Select>(); // No GROUP BY keyword or aggregate functions
    }
    0 as i32; // Has no GROUP BY clause
    if (unsafe { (*p).pLimit }) != std::ptr::null_mut::<Expr>() {
        return std::ptr::null_mut::<Select>();
    }
    // Has no LIMIT clause
    if (unsafe { (*p).pWhere }) != std::ptr::null_mut::<Expr>() {
        return std::ptr::null_mut::<Select>();
    }
    // Has no WHERE clause
    pSrc = unsafe { (*p).pSrc };
    0 as i32;
    if (unsafe { (*pSrc).nSrc }) != (1 as i32) {
        return std::ptr::null_mut::<Select>();
    }
    // Single term in FROM clause
    if ((unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset((0 as i32) as isize)
        })
        .fg
        .__slate_bits_0
        .__get_isSubquery()
    }) as i32)
        != (0 as i32)
    {
        return std::ptr::null_mut::<Select>();
    }
    // FROM is not a subquery or view
    pTab = unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset((0 as i32) as isize)
        })
        .pSTab
    };
    0 as i32;
    0 as i32; // FROM clause is not a view
    if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32) {
        return std::ptr::null_mut::<Select>();
    }
    // FROM clause not a virtual table
    pEList = unsafe { (*p).pEList };
    0 as i32;
    // All SELECT results must be columns.
    i = 0 as i32;
    '__slate_break_2265: loop {
        if !(i < unsafe { (*pEList).nExpr }) {
            break;
        }
        let mut pRes: *mut Expr = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                    .offset(i as isize)
            })
            .pExpr
        };
        if (((unsafe { (*pRes).op }) as u32) as i32) != (168 as i32) {
            return std::ptr::null_mut::<Select>();
        }
        0 as i32; // Not a correlated subquery
        let __v2678: i32 = i;
        let __v2679: i32 = __v2678 + (1 as i32);
        i = __v2679;
    }
    return p;
}

/// Generate code that checks the left-most column of index table iCur to see if
/// it contains any NULL entries.  Cause the register at regHasNull to be set
/// to a non-NULL value if iCur contains no NULLs.  Cause register regHasNull
/// to be set to NULL if iCur contains one or more NULL values.
///
/// # Arguments
///
/// * `v` - Write new code into this statement under construction
/// * `iCur` - Cursor for the index
/// * `regHasNull` - Register in which to store hasNull flag
/// * `eSortOrder` - SQLITE_SO_ASC or SQLITE_SO_DESC
fn sqlite3SetHasNullFlag(
    mut v: *mut Vdbe,
    mut iCur: i32,
    mut regHasNull: i32,
    mut eSortOrder: i32,
) {
    let mut addr1: i32 = 0 as i32;
    let mut op: i32 = 0 as i32;
    unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, regHasNull) };
    if eSortOrder == (0 as i32) {
        op = 36 as i32;
    } else {
        op = 32 as i32;
    }
    addr1 = unsafe { sqlite3VdbeAddOp1(v, op, iCur) };
    {}
    unsafe { sqlite3VdbeAddOp3(v, 96 as i32, iCur, 0 as i32, regHasNull) };
    unsafe { sqlite3VdbeChangeP5(v, ((128 as i32) as i16) as u16) };
    unsafe {
        sqlite3VdbeComment(
            v,
            (if op == (32 as i32) {
                b"last_entry_in(%d)\0".as_ptr() as *mut i8
            } else {
                b"first_entry_in(%d)\0".as_ptr() as *mut i8
            }) as *const i8,
            iCur,
        )
    };
    unsafe { sqlite3VdbeJumpHere(v, addr1) };
}

/// The argument is an IN operator with a list (not a subquery) on the
/// right-hand side.  Return TRUE if that list is constant.
fn sqlite3InRhsIsConstant(mut pParse: *mut Parse, mut pIn: *mut Expr) -> i32 {
    let mut pLHS: *mut Expr = unsafe { std::mem::zeroed() };
    let mut res: i32 = 0 as i32;
    0 as i32;
    pLHS = unsafe { (*pIn).pLeft };
    unsafe {
        (*pIn).pLeft = std::ptr::null_mut::<Expr>();
    }
    res = sqlite3ExprIsConstant(pParse, pIn);
    unsafe {
        (*pIn).pLeft = pLHS;
    }
    return res;
}

/// This function is used by the implementation of the IN (...) operator.
/// The pX parameter is the expression on the RHS of the IN operator, which
/// might be either a list of expressions or a subquery.
///
/// The job of this routine is to find or create a b-tree object that can
/// be used either to test for membership in the RHS set or to iterate through
/// all members of the RHS set, skipping duplicates.
///
/// A cursor is opened on the b-tree object that is the RHS of the IN operator
/// and the *piTab parameter is set to the index of that cursor.
///
/// The returned value of this function indicates the b-tree type, as follows:
///
///   IN_INDEX_ROWID      - The cursor was opened on a database table.
///   IN_INDEX_INDEX_ASC  - The cursor was opened on an ascending index.
///   IN_INDEX_INDEX_DESC - The cursor was opened on a descending index.
///   IN_INDEX_EPH        - The cursor was opened on a specially created and
///                         populated ephemeral table.
///   IN_INDEX_NOOP       - No cursor was allocated.  The IN operator must be
///                         implemented as a sequence of comparisons.
///
/// An existing b-tree might be used if the RHS expression pX is a simple
/// subquery such as:
///
///     SELECT <column1>, <column2>... FROM <table>
///
/// If the RHS of the IN operator is a list or a more complex subquery, then
/// an ephemeral table might need to be generated from the RHS and then
/// pX->iTable made to point to the ephemeral table instead of an
/// existing table.  In this case, the creation and initialization of the
/// ephemeral table might be put inside of a subroutine, the EP_Subrtn flag
/// will be set on pX and the pX->y.sub fields will be set to show where
/// the subroutine is coded.
///
/// The inFlags parameter must contain, at a minimum, one of the bits
/// IN_INDEX_MEMBERSHIP or IN_INDEX_LOOP but not both.  If inFlags contains
/// IN_INDEX_MEMBERSHIP, then the generated table will be used for a fast
/// membership test.  When the IN_INDEX_LOOP bit is set, the IN index will
/// be used to loop over all values of the RHS of the IN operator.
///
/// When IN_INDEX_LOOP is used (and the b-tree will be used to iterate
/// through the set members) then the b-tree must not contain duplicates.
/// An ephemeral table will be created unless the selected columns are guaranteed
/// to be unique - either because it is an INTEGER PRIMARY KEY or due to
/// a UNIQUE constraint or index.
///
/// When IN_INDEX_MEMBERSHIP is used (and the b-tree will be used
/// for fast set membership tests) then an ephemeral table must
/// be used unless <columns> is a single INTEGER PRIMARY KEY column or an
/// index can be found with the specified <columns> as its left-most.
///
/// If the IN_INDEX_NOOP_OK and IN_INDEX_MEMBERSHIP are both set and
/// if the RHS of the IN operator is a list (not a subquery) then this
/// routine might decide that creating an ephemeral b-tree for membership
/// testing is too expensive and return IN_INDEX_NOOP.  In that case, the
/// calling routine should implement the IN operator using a sequence
/// of Eq or Ne comparison operations.
///
/// When the b-tree is being used for membership tests, the calling function
/// might need to know whether or not the RHS side of the IN operator
/// contains a NULL.  If prRhsHasNull is not a NULL pointer and
/// if there is any chance that the (...) might contain a NULL value at
/// runtime, then a register is allocated and the register number written
/// to *prRhsHasNull. If there is no chance that the (...) contains a
/// NULL value, then *prRhsHasNull is left unchanged.
///
/// If a register is allocated and its location stored in *prRhsHasNull, then
/// the value in that register will be NULL if the b-tree contains one or more
/// NULL values, and it will be some non-NULL value if the b-tree contains no
/// NULL values.
///
/// If the aiMap parameter is not NULL, it must point to an array containing
/// one element for each column returned by the SELECT statement on the RHS
/// of the IN(...) operator. The i'th entry of the array is populated with the
/// offset of the index column that matches the i'th column returned by the
/// SELECT. For example, if the expression and selected index are:
///
///   (?,?,?) IN (SELECT a, b, c FROM t1)
///   CREATE INDEX i1 ON t1(b, c, a);
///
/// then aiMap[] is populated with {2, 0, 1}.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pX` - The IN expression
/// * `inFlags` - IN_INDEX_LOOP, _MEMBERSHIP, and/or _NOOP_OK
/// * `prRhsHasNull` - Register holding NULL status.  See notes
/// * `aiMap` - Mapping from Index fields to RHS fields
/// * `piTab` - OUT: index to use
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FindInIndex(
    mut pParse: *mut Parse,
    mut pX: *mut Expr,
    mut inFlags: u32,
    mut prRhsHasNull: *mut i32,
    mut aiMap: *mut i32,
    mut piTab: *mut i32,
) -> i32 {
    let mut p: *mut Select = unsafe { std::mem::zeroed() }; // SELECT to the right of IN operator
    let mut eType: i32 = 0 as i32; // Type of RHS table. IN_INDEX_*
    let mut iTab: i32 = 0 as i32; // Cursor of the RHS table
    let mut mustBeUnique: i32 = 0 as i32; // True if RHS must be unique
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) }; // Virtual machine being coded
    0 as i32;
    mustBeUnique = (inFlags & ((4 as i32) as u32) != ((0 as i32) as u32)) as i32;
    let __v2590: *mut Parse = pParse;
    let __v2591: i32 = unsafe { (*__v2590).nTab };
    let __v2592: i32 = __v2591 + (1 as i32);
    unsafe {
        (*__v2590).nTab = __v2592;
    }
    iTab = __v2591;
    // If the RHS of this IN(...) operator is a SELECT, and if it matters
    // whether or not the SELECT result contains NULL values, check whether
    // or not NULL is actually possible (it may not be, for example, due
    // to NOT NULL constraints in the schema). If no NULL values are possible,
    // set prRhsHasNull to 0 before continuing.
    if prRhsHasNull != std::ptr::null_mut::<i32>()
        && (unsafe { (*pX).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32)
    {
        let mut i: i32 = 0 as i32;
        let mut pEList: *mut ExprList = unsafe { (*unsafe { (*pX).x.pSelect }).pEList };
        i = 0 as i32;
        '__slate_break_2268: loop {
            if !(i < unsafe { (*pEList).nExpr }) {
                break;
            }
            if sqlite3ExprCanBeNull(
                (unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                            .offset(i as isize)
                    })
                    .pExpr
                }) as *const Expr,
            ) != (0 as i32)
            {
                break '__slate_break_2268;
            }
            let __v2593: i32 = i;
            let __v2594: i32 = __v2593 + (1 as i32);
            i = __v2594;
        }
        if i == unsafe { (*pEList).nExpr } {
            prRhsHasNull = std::ptr::null_mut::<i32>();
        }
    }
    // Check to see if an existing table or index can be used to
    // satisfy the query.  This is preferable to generating a new
    // ephemeral table.
    let __v2595: bool;
    if (unsafe { (*pParse).nErr }) == (0 as i32) {
        let __v2596: *mut Select = isCandidateForInOpt(pX as *const Expr);
        p = __v2596;
        __v2595 = __v2596 != std::ptr::null_mut::<Select>();
    } else {
        __v2595 = false as bool;
    }
    if __v2595 {
        let mut db: *mut sqlite3 = unsafe { (*pParse).db }; // Database connection
        let mut pTab: *mut Table = unsafe { std::mem::zeroed() }; // Table <table>.
        let mut iDb: i32 = 0 as i32; // Database idx for pTab
        let mut pEList: *mut ExprList = unsafe { (*p).pEList };
        let mut nExpr: i32 = unsafe { (*pEList).nExpr };
        0 as i32; // Because of isCandidateForInOpt(p)
        0 as i32; // Because of isCandidateForInOpt(p)
        0 as i32; // Because of isCandidateForInOpt(p)
        pTab = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .pSTab
        };
        // Code an OP_Transaction and OP_TableLock for <table>.
        iDb = unsafe { sqlite3SchemaToIndex(db, unsafe { (*pTab).pSchema }) };
        0 as i32;
        unsafe { sqlite3CodeVerifySchema(pParse, iDb) };
        unsafe {
            sqlite3TableLock(
                pParse,
                iDb,
                unsafe { (*pTab).tnum },
                ((0 as i32) as i8) as u8,
                (unsafe { (*pTab).zName }) as *const i8,
            )
        };
        0 as i32; // sqlite3GetVdbe() has always been previously called
        if nExpr == (1 as i32)
            && ((unsafe {
                (*unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                            .offset((0 as i32) as isize)
                    })
                    .pExpr
                })
                .iColumn
            }) as i32)
                < (0 as i32)
        {
            // The "x IN (SELECT rowid FROM table)" case
            let mut iAddr: i32 = unsafe { sqlite3VdbeAddOp0(v, 15 as i32) };
            {}
            unsafe { sqlite3OpenTable(pParse, iTab, iDb, pTab, 114 as i32) };
            eType = 1 as i32;
            unsafe {
                sqlite3VdbeExplain(
                    pParse,
                    ((0 as i32) as i8) as u8,
                    (b"USING ROWID SEARCH ON TABLE %s FOR IN-OPERATOR\0".as_ptr() as *mut i8)
                        as *const i8,
                    unsafe { (*pTab).zName },
                )
            };
            unsafe { sqlite3VdbeJumpHere(v, iAddr) };
        } else {
            let mut pIdx: *mut Index = unsafe { std::mem::zeroed() }; // Iterator variable
            let mut affinity_ok: i32 = 1 as i32;
            let mut i: i32 = 0 as i32;
            // Check that the affinity that will be used to perform each
            // comparison is the same as the affinity of each column in table
            // on the RHS of the IN operator.  If it not, it is not possible to
            // use any index of the RHS table.
            i = 0 as i32;
            '__slate_break_2270: loop {
                if !(i < nExpr && affinity_ok != (0 as i32)) {
                    break;
                }
                let mut pLhs: *mut Expr = sqlite3VectorFieldSubexpr(unsafe { (*pX).pLeft }, i);
                let mut iCol: i32 = (unsafe {
                    (*unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                                .offset(i as isize)
                        })
                        .pExpr
                    })
                    .iColumn
                }) as i32;
                let mut idxaff: i8 = sqlite3TableColumnAffinity(pTab as *const Table, iCol); // RHS table
                let mut cmpaff: i8 = sqlite3CompareAffinity(pLhs as *const Expr, idxaff);
                {}
                {}
                '__slate_break_2271: {
                    match cmpaff as i32 {
                        65 => {}
                        66 => {
                            0 as i32;
                            // sqlite3CompareAffinity() only returns TEXT if one side or the
                            // other has no affinity and the other side is TEXT.  Hence,
                            // the only way for cmpaff to be TEXT is for idxaff to be TEXT
                            // and for the term on the LHS of the IN to have no affinity.
                        }
                        _ => {
                            affinity_ok = ((idxaff as i32) >= (67 as i32)) as i32;
                        }
                    }
                }
                let __v2597: i32 = i;
                let __v2598: i32 = __v2597 + (1 as i32);
                i = __v2598;
            }
            if affinity_ok != (0 as i32) {
                // Search for an existing index that will work for this IN operator
                pIdx = unsafe { (*pTab).pIndex };
                '__slate_break_2272: while pIdx != std::ptr::null_mut::<Index>()
                    && eType == (0 as i32)
                {
                    '__slate_continue_2272: {
                        let mut colUsed: u64 = 0 as u64; // Columns of the index used
                        let mut mCol: u64 = 0 as u64; // Mask for the current column
                        if (((unsafe { (*pIdx).nColumn }) as u32) as i32) < nExpr {
                        } else {
                            if (unsafe { (*pIdx).pPartIdxWhere }) != std::ptr::null_mut::<Expr>() {
                            } else {
                                // Maximum nColumn is BMS-2, not BMS-1, so that we can compute
                                // BITMASK(nExpr) without overflowing
                                {}
                                {}
                                if (((unsafe { (*pIdx).nColumn }) as u32) as i32)
                                    >= (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32)
                                        as i32)
                                        - (1 as i32)
                                {
                                } else {
                                    if mustBeUnique != (0 as i32) {
                                        if (((unsafe { (*pIdx).nKeyCol }) as u32) as i32) > nExpr
                                            || (((unsafe { (*pIdx).nColumn }) as u32) as i32)
                                                > nExpr
                                                && !((((unsafe { (*pIdx).onError }) as u32) as i32)
                                                    != (0 as i32))
                                        {
                                            break '__slate_continue_2272; // This index is not unique over the IN RHS columns
                                        }
                                    }
                                    colUsed = ((0 as i32) as i64) as u64; // Columns of index used so far
                                    i = 0 as i32;
                                    '__slate_break_2273: loop {
                                        if !(i < nExpr) {
                                            break;
                                        }
                                        let mut pLhs: *mut Expr =
                                            sqlite3VectorFieldSubexpr(unsafe { (*pX).pLeft }, i);
                                        let mut pRhs: *mut Expr = unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!((*pEList).a)
                                                        as *mut ExprList_item
                                                }
                                                .offset(i as isize)
                                            })
                                            .pExpr
                                        };
                                        let mut pReq: *mut CollSeq = sqlite3BinaryCompareCollSeq(
                                            pParse,
                                            pLhs as *const Expr,
                                            pRhs as *const Expr,
                                        );
                                        let mut j: i32 = 0 as i32;
                                        j = 0 as i32;
                                        '__slate_break_2274: loop {
                                            if !(j < nExpr) {
                                                break;
                                            }
                                            if ((unsafe {
                                                *unsafe {
                                                    unsafe { (*pIdx).aiColumn }.offset(j as isize)
                                                }
                                            })
                                                as i32)
                                                != ((unsafe { (*pRhs).iColumn }) as i32)
                                            {
                                            } else {
                                                0 as i32;
                                                let __v2603: bool;
                                                if pReq != std::ptr::null_mut::<CollSeq>() {
                                                    __v2603 = (unsafe {
                                                        sqlite3StrICmp(
                                                            (unsafe { (*pReq).zName }) as *const i8,
                                                            unsafe {
                                                                *unsafe {
                                                                    unsafe { (*pIdx).azColl }
                                                                        .offset(j as isize)
                                                                }
                                                            },
                                                        )
                                                    }) != (0 as i32);
                                                } else {
                                                    __v2603 = false as bool;
                                                }
                                                if __v2603 {
                                                } else {
                                                    break '__slate_break_2274;
                                                }
                                            }
                                            let __v2601: i32 = j;
                                            let __v2602: i32 = __v2601 + (1 as i32);
                                            j = __v2602;
                                        }
                                        if j == nExpr {
                                            break '__slate_break_2273;
                                        }
                                        mCol = (((1 as i32) as i64) as u64) << j;
                                        if mCol & colUsed != (0 as u64) {
                                            break '__slate_break_2273;
                                        }
                                        // Each column used only once
                                        let __v2604: u64 = colUsed;
                                        let __v2605: u64 = __v2604 | mCol;
                                        colUsed = __v2605;
                                        if aiMap != std::ptr::null_mut::<i32>() {
                                            unsafe {
                                                *unsafe { aiMap.offset(i as isize) } = j;
                                            }
                                        }
                                        let __v2599: i32 = i;
                                        let __v2600: i32 = __v2599 + (1 as i32);
                                        i = __v2600;
                                    }
                                    0 as i32;
                                    0 as i32;
                                    if colUsed
                                        == ((((1 as i32) as i64) as u64) << nExpr)
                                            .wrapping_sub(((1 as i32) as i64) as u64)
                                    {
                                        // If we reach this point, that means the index pIdx is usable
                                        let mut iAddr: i32 =
                                            unsafe { sqlite3VdbeAddOp0(v, 15 as i32) };
                                        {}
                                        unsafe {
                                            sqlite3VdbeExplain(
                                                pParse,
                                                ((0 as i32) as i8) as u8,
                                                (b"USING INDEX %s FOR IN-OPERATOR\0".as_ptr()
                                                    as *mut i8)
                                                    as *const i8,
                                                unsafe { (*pIdx).zName },
                                            )
                                        };
                                        unsafe {
                                            sqlite3VdbeAddOp3(
                                                v,
                                                114 as i32,
                                                iTab,
                                                (unsafe { (*pIdx).tnum }) as i32,
                                                iDb,
                                            )
                                        };
                                        unsafe { sqlite3VdbeSetP4KeyInfo(pParse, pIdx) };
                                        unsafe {
                                            sqlite3VdbeComment(
                                                v,
                                                (b"%s\0".as_ptr() as *mut i8) as *const i8,
                                                unsafe { (*pIdx).zName },
                                            )
                                        };
                                        0 as i32;
                                        eType = (3 as i32)
                                            + (((unsafe {
                                                *unsafe {
                                                    unsafe { (*pIdx).aSortOrder }
                                                        .offset((0 as i32) as isize)
                                                }
                                            })
                                                as u32)
                                                as i32);
                                        if prRhsHasNull != std::ptr::null_mut::<i32>() {
                                            let __v2606: *mut Parse = pParse;
                                            let __v2607: i32 = unsafe { (*__v2606).nMem };
                                            let __v2608: i32 = __v2607 + (1 as i32);
                                            unsafe {
                                                (*__v2606).nMem = __v2608;
                                            }
                                            unsafe {
                                                *prRhsHasNull = __v2608;
                                            }
                                            if nExpr == (1 as i32) {
                                                sqlite3SetHasNullFlag(
                                                    v,
                                                    iTab,
                                                    unsafe { *prRhsHasNull },
                                                    ((unsafe {
                                                        *unsafe {
                                                            unsafe { (*pIdx).aSortOrder }
                                                                .offset((0 as i32) as isize)
                                                        }
                                                    })
                                                        as u32)
                                                        as i32,
                                                );
                                            }
                                        }
                                        unsafe { sqlite3VdbeJumpHere(v, iAddr) };
                                    }
                                }
                            }
                        }
                    }
                    pIdx = unsafe { (*pIdx).pNext };
                }
                // End loop over indexes
            }
            // End if( affinity_ok )
        }
        // End if not an rowid index
    }
    // End attempt to optimize using an index
    // If no preexisting index is available for the IN clause
    // and IN_INDEX_NOOP is an allowed reply
    // and the RHS of the IN operator is a list, not a subquery
    // and the RHS is not constant or has two or fewer terms,
    // then it is not worth creating an ephemeral table to evaluate
    // the IN operator so return IN_INDEX_NOOP.
    let __v2609: bool;
    if eType == (0 as i32)
        && inFlags & ((1 as i32) as u32) != (0 as u32)
        && (unsafe { (*pX).flags }) & ((4096 as i32) as u32) == ((0 as i32) as u32)
    {
        __v2609 = !(sqlite3InRhsIsConstant(pParse, pX) != (0 as i32))
            || (unsafe { (*unsafe { (*pX).x.pList }).nExpr }) <= (2 as i32);
    } else {
        __v2609 = false as bool;
    }
    if __v2609 {
        let __v2610: *mut Parse = pParse;
        let __v2611: i32 = unsafe { (*__v2610).nTab };
        let __v2612: i32 = __v2611 - (1 as i32);
        unsafe {
            (*__v2610).nTab = __v2612;
        }
        // Back out the allocation of the unused cursor
        iTab = -(1 as i32); // Cursor is not allocated
        eType = 5 as i32;
    }
    if eType == (0 as i32) {
        // Could not find an existing table or index to use as the RHS b-tree.
        // We will have to generate an ephemeral table to do the job.
        let mut savedNQueryLoop: u32 = ((unsafe { (*pParse).nQueryLoop }) as i32) as u32;
        let mut rMayHaveNull: i32 = 0 as i32;
        let mut bloomOk: i32 = (inFlags & ((2 as i32) as u32) != ((0 as i32) as u32)) as i32;
        eType = 2 as i32;
        if inFlags & ((4 as i32) as u32) != (0 as u32) {
            unsafe {
                (*pParse).nQueryLoop = (0 as i32) as i16;
            }
        } else {
            if prRhsHasNull != std::ptr::null_mut::<i32>() {
                let __v2613: *mut Parse = pParse;
                let __v2614: i32 = unsafe { (*__v2613).nMem };
                let __v2615: i32 = __v2614 + (1 as i32);
                unsafe {
                    (*__v2613).nMem = __v2615;
                }
                let __v2616: i32 = __v2615;
                rMayHaveNull = __v2616;
                unsafe {
                    *prRhsHasNull = __v2616;
                }
            }
        }
        0 as i32;
        if !(bloomOk != (0 as i32))
            && (unsafe { (*pX).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32)
            && (unsafe { (*unsafe { (*pX).x.pSelect }).selFlags }) & ((32 as i32) as u32)
                != ((0 as i32) as u32)
        {
            bloomOk = 1 as i32;
        }
        sqlite3CodeRhsOfIN(pParse, pX, iTab, bloomOk);
        if rMayHaveNull != (0 as i32) {
            sqlite3SetHasNullFlag(v, iTab, rMayHaveNull, 0 as i32);
        }
        unsafe {
            (*pParse).nQueryLoop = (savedNQueryLoop as u16) as i16;
        }
    }
    if aiMap != std::ptr::null_mut::<i32>() && eType != (3 as i32) && eType != (4 as i32) {
        let mut i: i32 = 0 as i32;
        let mut n: i32 = 0 as i32;
        n = sqlite3ExprVectorSize((unsafe { (*pX).pLeft }) as *const Expr);
        i = 0 as i32;
        '__slate_break_2277: loop {
            if !(i < n) {
                break;
            }
            unsafe {
                *unsafe { aiMap.offset(i as isize) } = i;
            }
            let __v2617: i32 = i;
            let __v2618: i32 = __v2617 + (1 as i32);
            i = __v2618;
        }
    }
    unsafe {
        *piTab = iTab;
    }
    return eType;
}

/// Argument pExpr is an (?, ?...) IN(...) expression. This
/// function allocates and returns a nul-terminated string containing
/// the affinities to be used for each column of the comparison.
///
/// It is the responsibility of the caller to ensure that the returned
/// string is eventually freed using sqlite3DbFree().
fn exprINAffinity(mut pParse: *mut Parse, mut pExpr: *const Expr) -> *mut i8 {
    let mut pLeft: *mut Expr = unsafe { (*pExpr).pLeft };
    let mut nVal: i32 = sqlite3ExprVectorSize(pLeft as *const Expr);
    let mut pSelect: *mut Select =
        if (unsafe { (*pExpr).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
            unsafe { (*pExpr).x.pSelect }
        } else {
            std::ptr::null_mut::<Select>()
        };
    let mut zRet: *mut i8 = unsafe { std::mem::zeroed() };
    0 as i32;
    zRet = (unsafe {
        sqlite3DbMallocRaw(
            unsafe { (*pParse).db },
            (((1 as i32) as i64) + (nVal as i64)) as u64,
        )
    }) as *mut i8;
    if zRet != std::ptr::null_mut::<i8>() {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_2278: loop {
            if !(i < nVal) {
                break;
            }
            let mut pA: *mut Expr = sqlite3VectorFieldSubexpr(pLeft, i);
            let mut a: i8 = sqlite3ExprAffinity(pA as *const Expr);
            if pSelect != std::ptr::null_mut::<Select>() {
                unsafe {
                    *unsafe { zRet.offset(i as isize) } = sqlite3CompareAffinity(
                        (unsafe {
                            (*unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*unsafe { (*pSelect).pEList }).a)
                                        as *mut ExprList_item
                                }
                                .offset(i as isize)
                            })
                            .pExpr
                        }) as *const Expr,
                        a,
                    );
                }
            } else {
                unsafe {
                    *unsafe { zRet.offset(i as isize) } = a;
                }
            }
            let __v2680: i32 = i;
            let __v2681: i32 = __v2680 + (1 as i32);
            i = __v2681;
        }
        unsafe {
            *unsafe { zRet.offset(nVal as isize) } = (0 as i32) as i8;
        }
    }
    return zRet;
}

/// Load the Parse object passed as the first argument with an error
/// message of the form:
///
///   "sub-select returns N columns - expected M"
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SubselectError(
    mut pParse: *mut Parse,
    mut nActual: i32,
    mut nExpect: i32,
) {
    if (unsafe { (*pParse).nErr }) == (0 as i32) {
        let mut zFmt: *const i8 =
            (b"sub-select returns %d columns - expected %d\0".as_ptr() as *mut i8) as *const i8;
        unsafe { sqlite3ErrorMsg(pParse, zFmt, nActual, nExpect) };
    }
}

/// Expression pExpr is a vector that has been used in a context where
/// it is not permitted. If pExpr is a sub-select vector, this routine
/// loads the Parse object with a message of the form:
///
///   "sub-select returns N columns - expected 1"
///
/// Or, if it is a regular scalar vector:
///
///   "row value misused"
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VectorErrorMsg(mut pParse: *mut Parse, mut pExpr: *mut Expr) {
    if (unsafe { (*pExpr).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
        sqlite3SubselectError(
            pParse,
            unsafe { (*unsafe { (*unsafe { (*pExpr).x.pSelect }).pEList }).nExpr },
            1 as i32,
        );
    } else {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"row value misused\0".as_ptr() as *mut i8) as *const i8,
            )
        };
    }
}

/// Scan all previously generated bytecode looking for an OP_BeginSubrtn
/// that is compatible with pExpr.  If found, add the y.sub values
/// to pExpr and return true.  If not found, return false.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pExpr` - IN operator with RHS that we want to reuse
/// * `pNewSig` - Signature for the IN operator
fn findCompatibleInRhsSubrtn(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut pNewSig: *mut SubrtnSig,
) -> i32 {
    let mut pOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
    let mut pEnd: *mut VdbeOp = unsafe { std::mem::zeroed() };
    let mut pSig: *mut SubrtnSig = unsafe { std::mem::zeroed() };
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    if pNewSig == std::ptr::null_mut::<SubrtnSig>() {
        return 0 as i32;
    }
    if (((unsafe { (*pParse).mSubrtnSig }) as u32) as i32)
        & (1 as i32) << ((unsafe { (*pNewSig).selId }) & (7 as i32))
        == (0 as i32)
    {
        return 0 as i32;
    }
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    v = unsafe { (*pParse).pVdbe };
    0 as i32;
    pOp = unsafe { sqlite3VdbeGetOp(v, 1 as i32) };
    pEnd = unsafe { sqlite3VdbeGetLastOp(v) };
    '__slate_break_2281: while pOp < pEnd {
        if ((unsafe { (*pOp).p4type }) as i32) != -(16 as i32) {
        } else {
            0 as i32;
            pSig = unsafe { (*pOp).p4.pSubrtnSig };
            0 as i32;
            if !((unsafe { (*pSig).bComplete }) != (0 as u8)) {
            } else {
                if (unsafe { (*pNewSig).selId }) != unsafe { (*pSig).selId } {
                } else {
                    if (unsafe {
                        strcmp(
                            (unsafe { (*pNewSig).zAff }) as *const i8,
                            (unsafe { (*pSig).zAff }) as *const i8,
                        )
                    }) != (0 as i32)
                    {
                    } else {
                        unsafe {
                            (*pExpr).y.sub.iAddr = unsafe { (*pSig).iAddr };
                        }
                        unsafe {
                            (*pExpr).y.sub.regReturn = unsafe { (*pSig).regReturn };
                        }
                        unsafe {
                            (*pExpr).iTable = unsafe { (*pSig).iTable };
                        }
                        let __v2684: *mut Expr = pExpr;
                        let __v2685: u32 = unsafe { (*__v2684).flags };
                        let __v2686: u32 = __v2685 | ((33554432 as i32) as u32);
                        unsafe {
                            (*__v2684).flags = __v2686;
                        }
                        return 1 as i32;
                    }
                }
            }
        }
        let __v2682: *mut VdbeOp = pOp;
        let __v2683: *mut VdbeOp = unsafe { __v2682.offset((1 as i32) as isize) };
        pOp = __v2683;
    }
    return 0 as i32;
}

/// Generate code that will construct an ephemeral table containing all terms
/// in the RHS of an IN operator.  The IN operator can be in either of two
/// forms:
///
///     x IN (4,5,11)              -- IN operator with list on right-hand side
///     x IN (SELECT a FROM b)     -- IN operator with subquery on the right
///
/// The pExpr parameter is the IN operator.  The cursor number for the
/// constructed ephemeral table is returned.  The first time the ephemeral
/// table is computed, the cursor number is also stored in pExpr->iTable,
/// however the cursor number returned might not be the same, as it might
/// have been duplicated using OP_OpenDup.
///
/// If the LHS expression ("x" in the examples) is a column value, or
/// the SELECT statement returns a column value, then the affinity of that
/// column is used to build the index keys. If both 'x' and the
/// SELECT... statement are columns, then numeric affinity is used
/// if either column has NUMERIC or INTEGER affinity. If neither
/// 'x' nor the SELECT... statement are columns, then numeric affinity
/// is used.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pExpr` - The IN operator
/// * `iTab` - Use this cursor number
/// * `allowBloom` - True to allow the use of a Bloom filter
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CodeRhsOfIN(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut iTab: i32,
    mut allowBloom: i32,
) {
    let mut addrOnce: i32 = 0 as i32; // Address of the OP_Once instruction at top
    let mut addr: i32 = 0 as i32; // Address of OP_OpenEphemeral instruction
    let mut pLeft: *mut Expr = unsafe { std::mem::zeroed() }; // the LHS of the IN operator
    let mut pKeyInfo: *mut KeyInfo = std::ptr::null_mut::<KeyInfo>(); // Key information
    let mut nVal: i32 = 0 as i32; // Size of vector pLeft
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() }; // The prepared statement under construction
    let mut pSig: *mut SubrtnSig = std::ptr::null_mut::<SubrtnSig>(); // Signature for this subroutine
    v = unsafe { (*pParse).pVdbe };
    0 as i32;
    // The evaluation of the IN must be repeated every time it
    // is encountered if any of the following is true:
    //
    //    *  The right-hand side is a correlated subquery
    //    *  The right-hand side is an expression list containing variables
    //    *  We are inside a trigger
    //
    // If all of the above are false, then we can compute the RHS just once
    // and reuse it many names.
    if !((unsafe { (*pExpr).flags }) & ((64 as i32) as u32) != ((0 as i32) as u32))
        && (unsafe { (*pParse).iSelfTab }) == (0 as i32)
    {
        // Reuse of the RHS is allowed
        //
        // Compute a signature for the RHS of the IN operator to facility
        // finding and reusing prior instances of the same IN operator.
        0 as i32;
        if (unsafe { (*pExpr).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32)
            && (unsafe { (*unsafe { (*pExpr).x.pSelect }).selFlags }) & ((2 as i32) as u32)
                == ((0 as i32) as u32)
        {
            pSig = (unsafe { sqlite3DbMallocRawNN(unsafe { (*pParse).db }, 32 as u64) })
                as *mut SubrtnSig;
            if pSig != std::ptr::null_mut::<SubrtnSig>() {
                unsafe {
                    (*pSig).selId = (unsafe { (*unsafe { (*pExpr).x.pSelect }).selId }) as i32;
                }
                unsafe {
                    (*pSig).zAff = exprINAffinity(pParse, pExpr as *const Expr);
                }
            }
        }
        // Check to see if there is a prior materialization of the RHS of
        // this IN operator.  If there is, then make use of that prior
        // materialization rather than recomputing it.
        let __v2554: bool;
        if (unsafe { (*pExpr).flags }) & ((33554432 as i32) as u32) != ((0 as i32) as u32) {
            __v2554 = true as bool;
        } else {
            __v2554 = findCompatibleInRhsSubrtn(pParse, pExpr, pSig) != (0 as i32);
        }
        if __v2554 {
            addrOnce = unsafe { sqlite3VdbeAddOp0(v, 15 as i32) };
            {}
            if (unsafe { (*pExpr).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
                unsafe {
                    sqlite3VdbeExplain(
                        pParse,
                        ((0 as i32) as i8) as u8,
                        (b"REUSE LIST SUBQUERY %d\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { (*unsafe { (*pExpr).x.pSelect }).selId },
                    )
                };
            }
            0 as i32;
            unsafe {
                sqlite3VdbeAddOp2(v, 10 as i32, unsafe { (*pExpr).y.sub.regReturn }, unsafe {
                    (*pExpr).y.sub.iAddr
                })
            };
            0 as i32;
            unsafe { sqlite3VdbeAddOp2(v, 117 as i32, iTab, unsafe { (*pExpr).iTable }) };
            unsafe { sqlite3VdbeJumpHere(v, addrOnce) };
            if pSig != std::ptr::null_mut::<SubrtnSig>() {
                unsafe {
                    sqlite3DbFree(
                        unsafe { (*pParse).db },
                        (unsafe { (*pSig).zAff }) as *mut (),
                    )
                };
                unsafe { sqlite3DbFree(unsafe { (*pParse).db }, pSig as *mut ()) };
            }
            return;
        }
        // Begin coding the subroutine
        0 as i32;
        let __v2555: *mut Expr = pExpr;
        let __v2556: u32 = unsafe { (*__v2555).flags };
        let __v2557: u32 = __v2556 | ((33554432 as i32) as u32);
        unsafe {
            (*__v2555).flags = __v2557;
        }
        0 as i32;
        let __v2558: *mut Parse = pParse;
        let __v2559: i32 = unsafe { (*__v2558).nMem };
        let __v2560: i32 = __v2559 + (1 as i32);
        unsafe {
            (*__v2558).nMem = __v2560;
        }
        unsafe {
            (*pExpr).y.sub.regReturn = __v2560;
        }
        unsafe {
            (*pExpr).y.sub.iAddr = (unsafe {
                sqlite3VdbeAddOp2(v, 76 as i32, 0 as i32, unsafe { (*pExpr).y.sub.regReturn })
            }) + (1 as i32);
        }
        if pSig != std::ptr::null_mut::<SubrtnSig>() {
            unsafe {
                (*pSig).bComplete = ((0 as i32) as i8) as u8;
            }
            unsafe {
                (*pSig).iAddr = unsafe { (*pExpr).y.sub.iAddr };
            }
            unsafe {
                (*pSig).regReturn = unsafe { (*pExpr).y.sub.regReturn };
            }
            unsafe {
                (*pSig).iTable = iTab;
            }
            unsafe {
                (*pParse).mSubrtnSig =
                    (((1 as i32) << ((unsafe { (*pSig).selId }) & (7 as i32))) as i8) as u8;
            }
            unsafe { sqlite3VdbeChangeP4(v, -(1 as i32), pSig as *const i8, -(16 as i32)) };
        }
        addrOnce = unsafe { sqlite3VdbeAddOp0(v, 15 as i32) };
        {}
    }
    // Check to see if this is a vector IN operator
    pLeft = unsafe { (*pExpr).pLeft };
    nVal = sqlite3ExprVectorSize(pLeft as *const Expr);
    // Construct the ephemeral table that will contain the content of
    // RHS of the IN operator.
    unsafe {
        (*pExpr).iTable = iTab;
    }
    addr = unsafe { sqlite3VdbeAddOp2(v, 120 as i32, unsafe { (*pExpr).iTable }, nVal) };
    if (unsafe { (*pExpr).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
        unsafe {
            sqlite3VdbeComment(
                v,
                (b"Result of SELECT %u\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*unsafe { (*pExpr).x.pSelect }).selId },
            )
        };
    } else {
        unsafe {
            sqlite3VdbeComment(
                v,
                (b"RHS of IN operator\0".as_ptr() as *mut i8) as *const i8,
            )
        };
    }
    pKeyInfo = unsafe { sqlite3KeyInfoAlloc(unsafe { (*pParse).db }, nVal, 1 as i32) };
    0 as i32;
    if pKeyInfo == std::ptr::null_mut::<KeyInfo>() {
        return;
    }
    if (unsafe { (*pExpr).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
        // Case 1:     expr IN (SELECT ...)
        //
        // Generate code to write the results of the select into the temporary
        // table allocated and opened above.
        let mut pSelect: *mut Select = unsafe { (*pExpr).x.pSelect };
        let mut pEList: *mut ExprList = unsafe { (*pSelect).pEList };
        unsafe {
            sqlite3VdbeExplain(
                pParse,
                ((1 as i32) as i8) as u8,
                (b"%sLIST SUBQUERY %d\0".as_ptr() as *mut i8) as *const i8,
                if addrOnce != (0 as i32) {
                    b"\0".as_ptr() as *mut i8
                } else {
                    b"CORRELATED \0".as_ptr() as *mut i8
                },
                unsafe { (*pSelect).selId },
            )
        };
        // If the LHS and RHS of the IN operator do not match, that
        // error will have been caught long before we reach this point.
        if (unsafe { (*pEList).nExpr }) == nVal {
            let mut pCopy: *mut Select = unsafe { std::mem::zeroed() };
            let mut dest: SelectDest = unsafe { std::mem::zeroed() };
            let mut i: i32 = 0 as i32;
            let mut rc: i32 = 0 as i32;
            let mut addrBloom: i32 = 0 as i32;
            unsafe { sqlite3SelectDestInit(std::ptr::addr_of_mut!(dest), 9 as i32, iTab) };
            dest.zAffSdst = exprINAffinity(pParse, pExpr as *const Expr);
            unsafe {
                (*pSelect).iLimit = 0 as i32;
            }
            0 as i32;
            0 as i32;
            0 as i32;
            i = 0 as i32;
            '__slate_break_2288: loop {
                if !(i < nVal) {
                    break;
                }
                let mut p: *mut Expr = sqlite3VectorFieldSubexpr(pLeft, i);
                let mut pColl: *mut CollSeq = unsafe { std::mem::zeroed() };
                let __v2563: *mut CollSeq = sqlite3BinaryCompareCollSeq(
                    pParse,
                    p as *const Expr,
                    (unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                                .offset(i as isize)
                        })
                        .pExpr
                    }) as *const Expr,
                );
                pColl = __v2563;
                unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pKeyInfo).aColl) as *mut *mut CollSeq }
                            .offset(i as isize)
                    } = __v2563;
                }
                if !((unsafe { sqlite3IsBinary(pColl as *const CollSeq) }) != (0 as i32)) {
                    allowBloom = 0 as i32; // tag-202607231411
                }
                let __v2561: i32 = i;
                let __v2562: i32 = __v2561 + (1 as i32);
                i = __v2562;
            }
            if addrOnce != (0 as i32)
                && allowBloom != (0 as i32)
                && (unsafe { (*unsafe { (*pParse).db }).dbOptFlags }) & ((524288 as i32) as u32)
                    == ((0 as i32) as u32)
            {
                let mut regBloom: i32 = 0 as i32;
                let __v2564: *mut Parse = pParse;
                let __v2565: i32 = unsafe { (*__v2564).nMem };
                let __v2566: i32 = __v2565 + (1 as i32);
                unsafe {
                    (*__v2564).nMem = __v2566;
                }
                regBloom = __v2566;
                addrBloom = unsafe { sqlite3VdbeAddOp2(v, 79 as i32, 10000 as i32, regBloom) };
                unsafe {
                    sqlite3VdbeComment(v, (b"Bloom filter\0".as_ptr() as *mut i8) as *const i8)
                };
                dest.iSDParm2 = regBloom;
                unsafe {
                    sqlite3VdbeChangeP4(v, addr, (pKeyInfo as *mut ()) as *const i8, -(9 as i32))
                };
                pKeyInfo = std::ptr::null_mut::<KeyInfo>();
            }
            {}
            pCopy = sqlite3SelectDup(unsafe { (*pParse).db }, pSelect as *const Select, 0 as i32);
            let __v2567: i32;
            if (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8) {
                __v2567 = 1 as i32;
            } else {
                __v2567 = unsafe { sqlite3Select(pParse, pCopy, std::ptr::addr_of_mut!(dest)) };
            }
            rc = __v2567;
            unsafe { sqlite3SelectDelete(unsafe { (*pParse).db }, pCopy) };
            unsafe { sqlite3DbFree(unsafe { (*pParse).db }, dest.zAffSdst as *mut ()) };
            if addrBloom != (0 as i32) {
                // Remember that location of the Bloom filter in the P3 operand
                // of the OP_Once that began this subroutine. tag-202407032019
                unsafe {
                    (*unsafe { sqlite3VdbeGetOp(v, addrOnce) }).p3 = dest.iSDParm2;
                }
                if dest.iSDParm2 == (0 as i32) {
                    // If the Bloom filter won't actually be used, keep it small
                    unsafe {
                        (*unsafe { sqlite3VdbeGetOp(v, addrBloom) }).p1 = 10 as i32;
                    }
                }
            }
            if rc != (0 as i32) {
                unsafe { sqlite3KeyInfoUnref(pKeyInfo) };
                return;
            }
        }
    } else {
        if (unsafe { (*pExpr).x.pList }) != std::ptr::null_mut::<ExprList>() {
            // Case 2:     expr IN (exprlist)
            //
            // For each expression, build an index key from the evaluation and
            // store it in the temporary table. If <expr> is a column, then use
            // that columns affinity when building index keys. If <expr> is not
            // a column, use numeric affinity.
            let mut affinity: i8 = 0 as i8; // Affinity of the LHS of the IN
            let mut i: i32 = 0 as i32;
            let mut pList: *mut ExprList = unsafe { (*pExpr).x.pList };
            let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
            let mut r1: i32 = 0 as i32;
            let mut r2: i32 = 0 as i32;
            affinity = sqlite3ExprAffinity(pLeft as *const Expr);
            if (affinity as i32) <= (64 as i32) {
                affinity = (65 as i32) as i8;
            } else {
                if (affinity as i32) == (69 as i32) {
                    affinity = (67 as i32) as i8;
                }
            }
            0 as i32;
            0 as i32;
            unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pKeyInfo).aColl) as *mut *mut CollSeq }
                        .offset((0 as i32) as isize)
                } = sqlite3ExprCollSeq(pParse, (unsafe { (*pExpr).pLeft }) as *const Expr);
            }
            // Loop through each expression in <exprlist>.
            r1 = sqlite3GetTempReg(pParse);
            r2 = sqlite3GetTempReg(pParse);
            i = unsafe { (*pList).nExpr };
            let __v2568: *mut ExprList_item =
                unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item };
            pItem = __v2568;
            '__slate_break_2290: while i > (0 as i32) {
                let mut pE2: *mut Expr = unsafe { (*pItem).pExpr };
                // If the expression is not constant then we will need to
                // disable the test that was generated above that makes sure
                // this code only executes once.  Because for a non-constant
                // expression we need to rerun this code each time.
                let __v2573: bool;
                if addrOnce != (0 as i32) {
                    __v2573 = !(sqlite3ExprIsConstant(pParse, pE2) != (0 as i32));
                } else {
                    __v2573 = false as bool;
                }
                if __v2573 {
                    unsafe { sqlite3VdbeChangeToNoop(v, addrOnce - (1 as i32)) };
                    unsafe { sqlite3VdbeChangeToNoop(v, addrOnce) };
                    let __v2574: *mut Expr = pExpr;
                    let __v2575: u32 = unsafe { (*__v2574).flags };
                    let __v2576: u32 = __v2575 & !((33554432 as i32) as u32);
                    unsafe {
                        (*__v2574).flags = __v2576;
                    }
                    addrOnce = 0 as i32;
                }
                // Evaluate the expression and insert it into the temp table
                sqlite3ExprCode(pParse, pE2, r1);
                unsafe {
                    sqlite3VdbeAddOp4(
                        v,
                        99 as i32,
                        r1,
                        1 as i32,
                        r2,
                        std::ptr::addr_of_mut!(affinity) as *const i8,
                        1 as i32,
                    )
                };
                unsafe { sqlite3VdbeAddOp4Int(v, 140 as i32, iTab, r2, r1, 1 as i32) };
                let __v2569: i32 = i;
                let __v2570: i32 = __v2569 - (1 as i32);
                i = __v2570;
                let __v2571: *mut ExprList_item = pItem;
                let __v2572: *mut ExprList_item = unsafe { __v2571.offset((1 as i32) as isize) };
                pItem = __v2572;
            }
            sqlite3ReleaseTempReg(pParse, r1);
            sqlite3ReleaseTempReg(pParse, r2);
        }
    }
    if pSig != std::ptr::null_mut::<SubrtnSig>() {
        unsafe {
            (*pSig).bComplete = ((1 as i32) as i8) as u8;
        }
    }
    if pKeyInfo != std::ptr::null_mut::<KeyInfo>() {
        unsafe { sqlite3VdbeChangeP4(v, addr, (pKeyInfo as *mut ()) as *const i8, -(9 as i32)) };
    }
    if addrOnce != (0 as i32) {
        unsafe { sqlite3VdbeAddOp1(v, 138 as i32, iTab) };
        unsafe { sqlite3VdbeJumpHere(v, addrOnce) };
        // Subroutine return
        0 as i32;
        0 as i32;
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                69 as i32,
                unsafe { (*pExpr).y.sub.regReturn },
                unsafe { (*pExpr).y.sub.iAddr },
                1 as i32,
            )
        };
        {}
        sqlite3ClearTempRegCache(pParse);
    }
}

/// Generate code for scalar subqueries used as a subquery expression
/// or EXISTS operator:
///
///     (SELECT a FROM b)          -- subquery
///     EXISTS (SELECT a FROM b)   -- EXISTS subquery
///
/// The pExpr parameter is the SELECT or EXISTS operator to be coded.
///
/// Return the register that holds the result.  For a multi-column SELECT,
/// the result is stored in a contiguous array of registers and the
/// return value is the register of the left-most result column.
/// Return 0 if an error occurs.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CodeSubselect(mut pParse: *mut Parse, mut pExpr: *mut Expr) -> i32 {
    let mut addrOnce: i32 = 0 as i32; // Address of OP_Once at top of subroutine
    let mut rReg: i32 = 0 as i32; // Register storing resulting
    let mut pSel: *mut Select = unsafe { std::mem::zeroed() }; // SELECT statement to encode
    let mut dest: SelectDest = unsafe { std::mem::zeroed() }; // How to deal with SELECT result
    let mut nReg: i32 = 0 as i32; // Registers to allocate
    let mut pLimit: *mut Expr = unsafe { std::mem::zeroed() }; // New limit expression
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    0 as i32;
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        return 0 as i32;
    }
    {}
    {}
    0 as i32;
    0 as i32;
    pSel = unsafe { (*pExpr).x.pSelect };
    // If this routine has already been coded, then invoke it as a
    // subroutine.
    if (unsafe { (*pExpr).flags }) & ((33554432 as i32) as u32) != ((0 as i32) as u32) {
        unsafe {
            sqlite3VdbeExplain(
                pParse,
                ((0 as i32) as i8) as u8,
                (b"REUSE SUBQUERY %d\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pSel).selId },
            )
        };
        0 as i32;
        unsafe {
            sqlite3VdbeAddOp2(v, 10 as i32, unsafe { (*pExpr).y.sub.regReturn }, unsafe {
                (*pExpr).y.sub.iAddr
            })
        };
        return unsafe { (*pExpr).iTable };
    }
    // Begin coding the subroutine
    0 as i32;
    0 as i32;
    let __v2577: *mut Expr = pExpr;
    let __v2578: u32 = unsafe { (*__v2577).flags };
    let __v2579: u32 = __v2578 | ((33554432 as i32) as u32);
    unsafe {
        (*__v2577).flags = __v2579;
    }
    let __v2580: *mut Parse = pParse;
    let __v2581: i32 = unsafe { (*__v2580).nMem };
    let __v2582: i32 = __v2581 + (1 as i32);
    unsafe {
        (*__v2580).nMem = __v2582;
    }
    unsafe {
        (*pExpr).y.sub.regReturn = __v2582;
    }
    unsafe {
        (*pExpr).y.sub.iAddr = (unsafe {
            sqlite3VdbeAddOp2(v, 76 as i32, 0 as i32, unsafe { (*pExpr).y.sub.regReturn })
        }) + (1 as i32);
    }
    // The evaluation of the EXISTS/SELECT must be repeated every time it
    // is encountered if any of the following is true:
    //
    //    *  The right-hand side is a correlated subquery
    //    *  The right-hand side is an expression list containing variables
    //    *  We are inside a trigger
    //
    // If all of the above are false, then we can run this code just once
    // save the results, and reuse the same result on subsequent invocations.
    if !((unsafe { (*pExpr).flags }) & ((64 as i32) as u32) != ((0 as i32) as u32)) {
        addrOnce = unsafe { sqlite3VdbeAddOp0(v, 15 as i32) };
        {}
    }
    // For a SELECT, generate code to put the values for all columns of
    // the first row into an array of registers and return the index of
    // the first register.
    //
    // If this is an EXISTS, write an integer 0 (not exists) or 1 (exists)
    // into a register and return that register number.
    //
    // In both cases, the query is augmented with "LIMIT 1".  Any
    // preexisting limit is discarded in place of the new LIMIT 1.
    unsafe {
        sqlite3VdbeExplain(
            pParse,
            ((1 as i32) as i8) as u8,
            (b"%sSCALAR SUBQUERY %d\0".as_ptr() as *mut i8) as *const i8,
            if addrOnce != (0 as i32) {
                b"\0".as_ptr() as *mut i8
            } else {
                b"CORRELATED \0".as_ptr() as *mut i8
            },
            unsafe { (*pSel).selId },
        )
    };
    {}
    nReg = if (((unsafe { (*pExpr).op }) as u32) as i32) == (139 as i32) {
        unsafe { (*unsafe { (*pSel).pEList }).nExpr }
    } else {
        1 as i32
    };
    unsafe {
        sqlite3SelectDestInit(
            std::ptr::addr_of_mut!(dest),
            0 as i32,
            (unsafe { (*pParse).nMem }) + (1 as i32),
        )
    };
    let __v2583: *mut Parse = pParse;
    let __v2584: i32 = unsafe { (*__v2583).nMem };
    let __v2585: i32 = __v2584 + nReg;
    unsafe {
        (*__v2583).nMem = __v2585;
    }
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (139 as i32) {
        dest.eDest = ((8 as i32) as i8) as u8;
        if (unsafe { (*pSel).selFlags }) & ((1 as i32) as u32) != (0 as u32)
            && (unsafe { (*pSel).pLimit }) != std::ptr::null_mut::<Expr>()
            && (unsafe { (*unsafe { (*pSel).pLimit }).pRight }) != std::ptr::null_mut::<Expr>()
        {
            // If there is both a DISTINCT and an OFFSET clause, then allocate
            // a separate dest.iSdst array for sqlite3Select() and other
            // routines to populate. In this case results will be copied over
            // into the dest.iSDParm array only after OFFSET processing. This
            // ensures that in the case where OFFSET excludes all rows, the
            // dest.iSDParm array is not left populated with the contents of the
            // last row visited - it should be all NULLs if all rows were
            // excluded by OFFSET.
            dest.iSdst = (unsafe { (*pParse).nMem }) + (1 as i32);
            let __v2586: *mut Parse = pParse;
            let __v2587: i32 = unsafe { (*__v2586).nMem };
            let __v2588: i32 = __v2587 + nReg;
            unsafe {
                (*__v2586).nMem = __v2588;
            }
        } else {
            dest.iSdst = dest.iSDParm;
        }
        dest.nSdst = nReg;
        unsafe {
            sqlite3VdbeAddOp3(v, 77 as i32, 0 as i32, dest.iSDParm, unsafe {
                (*pParse).nMem
            })
        };
        unsafe {
            sqlite3VdbeComment(
                v,
                (b"Init subquery result\0".as_ptr() as *mut i8) as *const i8,
            )
        };
    } else {
        dest.eDest = ((1 as i32) as i8) as u8;
        unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, dest.iSDParm) };
        unsafe {
            sqlite3VdbeComment(
                v,
                (b"Init EXISTS result\0".as_ptr() as *mut i8) as *const i8,
            )
        };
    }
    if (unsafe { (*pSel).pLimit }) != std::ptr::null_mut::<Expr>() {
        // The subquery already has a limit.  If the pre-existing limit X is
        // not already integer value 1 or 0, then make the new limit X<>0 so that
        // the new limit is either 1 or 0
        let mut pLeft: *mut Expr = unsafe { (*unsafe { (*pSel).pLimit }).pLeft };
        if (((unsafe { (*pLeft).flags }) & ((2048 as i32) as u32) != ((0 as i32) as u32)) as i32)
            == (0 as i32)
            || (unsafe { (*pLeft).u.iValue }) != (1 as i32)
                && (unsafe { (*pLeft).u.iValue }) != (0 as i32)
        {
            let mut db: *mut sqlite3 = unsafe { (*pParse).db };
            pLimit = sqlite3ExprInt32(db, 0 as i32);
            if pLimit != std::ptr::null_mut::<Expr>() {
                unsafe {
                    (*pLimit).affExpr = (67 as i32) as i8;
                }
                pLimit = sqlite3PExpr(
                    pParse,
                    53 as i32,
                    sqlite3ExprDup(db, pLeft as *const Expr, 0 as i32),
                    pLimit,
                );
            }
            sqlite3ExprDeferredDelete(pParse, pLeft);
            unsafe {
                (*unsafe { (*pSel).pLimit }).pLeft = pLimit;
            }
        }
    } else {
        // If there is no pre-existing limit add a limit of 1
        pLimit = sqlite3ExprInt32(unsafe { (*pParse).db }, 1 as i32);
        unsafe {
            (*pSel).pLimit = sqlite3PExpr(pParse, 149 as i32, pLimit, std::ptr::null_mut::<Expr>());
        }
    }
    unsafe {
        (*pSel).iLimit = 0 as i32;
    }
    if (unsafe { sqlite3Select(pParse, pSel, std::ptr::addr_of_mut!(dest)) }) != (0 as i32) {
        unsafe {
            (*pExpr).op2 = unsafe { (*pExpr).op };
        }
        unsafe {
            (*pExpr).op = ((182 as i32) as i8) as u8;
        }
        return 0 as i32;
    }
    let __v2589: i32 = dest.iSDParm;
    rReg = __v2589;
    unsafe {
        (*pExpr).iTable = __v2589;
    }
    {}
    if addrOnce != (0 as i32) {
        unsafe { sqlite3VdbeJumpHere(v, addrOnce) };
    }
    {}
    // Subroutine return
    0 as i32;
    0 as i32;
    unsafe {
        sqlite3VdbeAddOp3(
            v,
            69 as i32,
            unsafe { (*pExpr).y.sub.regReturn },
            unsafe { (*pExpr).y.sub.iAddr },
            1 as i32,
        )
    };
    {}
    sqlite3ClearTempRegCache(pParse);
    return rReg;
}

/// Expr pIn is an IN(...) expression. This function checks that the
/// sub-select on the RHS of the IN() operator has the same number of
/// columns as the vector on the LHS. Or, if the RHS of the IN() is not
/// a sub-query, that the LHS is a vector of size 1.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCheckIN(mut pParse: *mut Parse, mut pIn: *mut Expr) -> i32 {
    let mut nVector: i32 = sqlite3ExprVectorSize((unsafe { (*pIn).pLeft }) as *const Expr);
    if (unsafe { (*pIn).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32)
        && !((unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8))
    {
        if nVector != unsafe { (*unsafe { (*unsafe { (*pIn).x.pSelect }).pEList }).nExpr } {
            sqlite3SubselectError(
                pParse,
                unsafe { (*unsafe { (*unsafe { (*pIn).x.pSelect }).pEList }).nExpr },
                nVector,
            );
            return 1 as i32;
        }
    } else {
        if nVector != (1 as i32) {
            sqlite3VectorErrorMsg(pParse, unsafe { (*pIn).pLeft });
            return 1 as i32;
        }
    }
    return 0 as i32;
}

/// Generate code for an IN expression.
///
///      x IN (SELECT ...)
///      x IN (value, value, ...)
///
/// The left-hand side (LHS) is a scalar or vector expression.  The
/// right-hand side (RHS) is an array of zero or more scalar values, or a
/// subquery.  If the RHS is a subquery, the number of result columns must
/// match the number of columns in the vector on the LHS.  If the RHS is
/// a list of values, the LHS must be a scalar.
///
/// The IN operator is true if the LHS value is contained within the RHS.
/// The result is false if the LHS is definitely not in the RHS.  The
/// result is NULL if the presence of the LHS in the RHS cannot be
/// determined due to NULLs.
///
/// This routine generates code that jumps to destIfFalse if the LHS is not
/// contained within the RHS.  If due to NULLs we cannot determine if the LHS
/// is contained in the RHS then jump to destIfNull.  If the LHS is contained
/// within the RHS then fall through.
///
/// See the separate in-operator.md documentation file in the canonical
/// SQLite source tree for additional information.
///
/// # Arguments
///
/// * `pParse` - Parsing and code generating context
/// * `pExpr` - The IN expression
/// * `destIfFalse` - Jump here if LHS is not contained in the RHS
/// * `destIfNull` - Jump here if the results are unknown due to NULLs
fn sqlite3ExprCodeIN(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut destIfFalse: i32,
    mut destIfNull: i32,
) {
    let mut __slate_storage_2699: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2699: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2699) as *mut i32;
    let mut __slate_storage_2698: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2698: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2698) as *mut i32;
    let mut __slate_storage_1194: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1194: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1194) as *mut *mut Expr;
    let mut __slate_storage_1193: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1193: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1193) as *mut i32;
    let mut __slate_storage_1192: std::mem::MaybeUninit<*mut CollSeq> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1192: *mut *mut CollSeq =
        std::ptr::addr_of_mut!(__slate_storage_1192) as *mut *mut CollSeq;
    let mut __slate_storage_1191: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1191: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1191) as *mut *mut Expr;
    let mut __slate_storage_1190: std::mem::MaybeUninit<*const VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1190: *mut *const VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_1190) as *mut *const VdbeOp;
    let mut __slate_storage_2697: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2697: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2697) as *mut i32;
    let mut __slate_storage_2696: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2696: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2696) as *mut i32;
    let mut __slate_storage_1189: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1189: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1189) as *mut *mut Expr;
    let mut __slate_storage_2695: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2695: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2695) as *mut i32;
    let mut __slate_storage_2694: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2694: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2694) as *mut i32;
    let mut __slate_storage_2693: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2693: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2693) as *mut i32;
    // Need to reorder the LHS fields according to aiMap
    let mut __slate_storage_1188: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1188: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1188) as *mut i32;
    let mut __slate_storage_2692: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2692: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2692) as *mut i32;
    let mut __slate_storage_2691: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2691: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2691) as *mut i32;
    // The OP_Affinity below may change the value. In this case, create a
    // copy of rLhs to run OP_Affinity on, in case the original register
    // is used again (e.g. if it is TK_AGG_COLUMN).
    let mut __slate_storage_1187: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1187: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1187) as *mut i32;
    let mut __slate_storage_2690: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2690: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2690) as *mut bool;
    let mut __slate_storage_1186: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1186: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_1186) as *mut i8;
    let mut __slate_storage_2688: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2688: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2688) as *mut i32;
    let mut __slate_storage_2687: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2687: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2687) as *mut i32;
    let mut __slate_storage_1184: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1184: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1184) as *mut i32;
    let mut __slate_storage_1185: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1185: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1185) as *mut i32;
    let mut __slate_storage_2689: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2689: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2689) as *mut bool;
    let mut __slate_storage_1183: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1183: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1183) as *mut i32;
    let mut __slate_storage_1182: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1182: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1182) as *mut i32;
    let mut __slate_storage_1181: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1181: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1181) as *mut i32;
    let mut __slate_storage_1180: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1180: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1180) as *mut i32;
    let mut __slate_storage_1179: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1179: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1179) as *mut i32;
    let mut __slate_storage_1178: std::mem::MaybeUninit<*mut CollSeq> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1178: *mut *mut CollSeq =
        std::ptr::addr_of_mut!(__slate_storage_1178) as *mut *mut CollSeq;
    let mut __slate_storage_1177: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1177: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_1177) as *mut *mut ExprList;
    let mut __slate_storage_1176: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1176: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1176) as *mut u8; // Index to use
    let mut __slate_storage_1175: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1175: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1175) as *mut i32; // Top of the step-6 loop
    let mut __slate_storage_1174: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1174: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1174) as *mut i32; // Jump here if a comparison is not true in step 6
    let mut __slate_storage_1173: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1173: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1173) as *mut i32; // Address of opcode that determines the IN is true
    let mut __slate_storage_1172: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1172: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1172) as *mut i32; // Start of code for Step 6
    let mut __slate_storage_1171: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1171: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1171) as *mut i32; // Where to jump when NULLs seen in step 2
    let mut __slate_storage_1170: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1170: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1170) as *mut i32; // loop counter
    let mut __slate_storage_1169: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1169: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1169) as *mut i32; // The LHS of the IN operator
    let mut __slate_storage_1168: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1168: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1168) as *mut *mut Expr; // Dummy parameter to exprCodeVector()
    let mut __slate_storage_1167: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1167: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1167) as *mut i32; // Size of vectors for this IN operator
    let mut __slate_storage_1166: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1166: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1166) as *mut i32; // Affinity string for comparisons
    let mut __slate_storage_1165: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1165: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1165) as *mut *mut i8; // Map from vector field to index column
    let mut __slate_storage_1164: std::mem::MaybeUninit<*mut i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1164: *mut *mut i32 =
        std::ptr::addr_of_mut!(__slate_storage_1164) as *mut *mut i32; // Statement under construction
    let mut __slate_storage_1163: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1163: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1163) as *mut *mut Vdbe; // Register(s) holding the LHS values
    let mut __slate_storage_1162: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1162: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1162) as *mut i32; // Type of the RHS
    let mut __slate_storage_1161: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1161: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1161) as *mut i32; // Register that is true if RHS contains NULL values
    let mut __slate_storage_1160: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1160: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1160) as *mut i32;
    unsafe {
        std::ptr::write(__slate_slot_1160, 0 as i32);
        std::ptr::write(__slate_slot_1164, std::ptr::null_mut::<i32>());
        std::ptr::write(__slate_slot_1165, std::ptr::null_mut::<i8>());
        std::ptr::write(__slate_slot_1171, 0 as i32);
        std::ptr::write(__slate_slot_1175, 0 as i32);
        std::ptr::write(
            __slate_slot_1176,
            (((unsafe { (*pParse).__slate_bits_0.__get_okConstFactor() }) as i32) as i8) as u8,
        );
        0 as i32;
        *__slate_slot_1168 = unsafe { (*pExpr).pLeft };
        if sqlite3ExprCheckIN(pParse, pExpr) != (0 as i32) {
            return;
        } else {
            '__join_0: {
                *__slate_slot_1165 = exprINAffinity(pParse, pExpr as *const Expr);
                *__slate_slot_1166 =
                    sqlite3ExprVectorSize((unsafe { (*pExpr).pLeft }) as *const Expr);
                *__slate_slot_1164 = (unsafe {
                    sqlite3DbMallocZero(
                        unsafe { (*pParse).db },
                        ((*__slate_slot_1166 as i64) as u64).wrapping_mul(4 as u64),
                    )
                }) as *mut i32;
                if (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8) {
                } else {
                    '__join_1: {
                        // Attempt to compute the RHS. After this step, if anything other than
                        // IN_INDEX_NOOP is returned, the table opened with cursor iTab
                        // contains the values that make up the RHS. If IN_INDEX_NOOP is returned,
                        // the RHS has not yet been coded.
                        *__slate_slot_1163 = unsafe { (*pParse).pVdbe };
                        0 as i32; // OOM detected prior to this routine
                        unsafe {
                            sqlite3VdbeNoopComment(
                                *__slate_slot_1163,
                                (b"begin IN expr\0".as_ptr() as *mut i8) as *const i8,
                            )
                        };
                        *__slate_slot_1161 = sqlite3FindInIndex(
                            pParse,
                            pExpr,
                            ((2 as i32) | (1 as i32)) as u32,
                            if destIfFalse == destIfNull {
                                std::ptr::null_mut::<i32>()
                            } else {
                                std::ptr::addr_of_mut!(*__slate_slot_1160)
                            },
                            *__slate_slot_1164,
                            std::ptr::addr_of_mut!(*__slate_slot_1175),
                        );
                        0 as i32;
                        // Code the LHS, the <expr> from "<expr> IN (...)". If the LHS is a
                        // vector, then it is stored in an array of nVector registers starting
                        // at r1.
                        //
                        // sqlite3FindInIndex() might have reordered the fields of the LHS vector
                        // so that the fields are in the same order as an existing index.   The
                        // aiMap[] array contains a mapping from the original LHS field order to
                        // the field order that matches the RHS index.
                        //
                        // Avoid factoring the LHS of the IN(...) expression out of the loop,
                        // even if it is constant, as OP_Affinity may be used on the register
                        // by code generated below.
                        0 as i32;
                        unsafe {
                            (*pParse)
                                .__slate_bits_0
                                .__set_okConstFactor((0 as i32) as u32);
                        }
                        *__slate_slot_1162 = exprCodeVector(
                            pParse,
                            *__slate_slot_1168,
                            std::ptr::addr_of_mut!(*__slate_slot_1167),
                        );
                        unsafe {
                            (*pParse)
                                .__slate_bits_0
                                .__set_okConstFactor(*__slate_slot_1176 as u32);
                        }
                        // If sqlite3FindInIndex() did not find or create an index that is
                        // suitable for evaluating the IN operator, then evaluate using a
                        // sequence of comparisons.
                        //
                        // This is step (1) in the in-operator.md optimized algorithm.
                        if *__slate_slot_1161 == (5 as i32) {
                            std::ptr::write(__slate_slot_1179, unsafe {
                                sqlite3VdbeMakeLabel(pParse)
                            });
                            std::ptr::write(__slate_slot_1182, 0 as i32);
                            0 as i32;
                            0 as i32;
                            *__slate_slot_1177 = unsafe { (*pExpr).x.pList };
                            *__slate_slot_1178 = sqlite3ExprCollSeq(
                                pParse,
                                (unsafe { (*pExpr).pLeft }) as *const Expr,
                            );
                            if destIfNull != destIfFalse {
                                *__slate_slot_1182 = sqlite3GetTempReg(pParse);
                                unsafe {
                                    sqlite3VdbeAddOp3(
                                        *__slate_slot_1163,
                                        103 as i32,
                                        *__slate_slot_1162,
                                        *__slate_slot_1162,
                                        *__slate_slot_1182,
                                    )
                                };
                            }
                            *__slate_slot_1183 = 0 as i32;
                            loop {
                                if *__slate_slot_1183 < unsafe { (*(*__slate_slot_1177)).nExpr } {
                                    *__slate_slot_1180 = sqlite3ExprCodeTemp(
                                        pParse,
                                        unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!(
                                                        (*(*__slate_slot_1177)).a
                                                    )
                                                        as *mut ExprList_item
                                                }
                                                .offset(*__slate_slot_1183 as isize)
                                            })
                                            .pExpr
                                        },
                                        std::ptr::addr_of_mut!(*__slate_slot_1181),
                                    );
                                    if *__slate_slot_1182 != (0 as i32) {
                                        *__slate_slot_2689 = sqlite3ExprCanBeNull(
                                            (unsafe {
                                                (*unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of_mut!(
                                                            (*(*__slate_slot_1177)).a
                                                        )
                                                            as *mut ExprList_item
                                                    }
                                                    .offset(*__slate_slot_1183 as isize)
                                                })
                                                .pExpr
                                            })
                                                as *const Expr,
                                        ) != (0 as i32);
                                    } else {
                                        *__slate_slot_2689 = false as bool;
                                    }
                                    if *__slate_slot_2689 {
                                        unsafe {
                                            sqlite3VdbeAddOp3(
                                                *__slate_slot_1163,
                                                103 as i32,
                                                *__slate_slot_1182,
                                                *__slate_slot_1180,
                                                *__slate_slot_1182,
                                            )
                                        };
                                    }
                                    sqlite3ReleaseTempReg(pParse, *__slate_slot_1181);
                                    if *__slate_slot_1183
                                        < (unsafe { (*(*__slate_slot_1177)).nExpr }) - (1 as i32)
                                        || destIfNull != destIfFalse
                                    {
                                        std::ptr::write(
                                            __slate_slot_1184,
                                            if *__slate_slot_1162 != *__slate_slot_1180 {
                                                54 as i32
                                            } else {
                                                52 as i32
                                            },
                                        );
                                        unsafe {
                                            sqlite3VdbeAddOp4(
                                                *__slate_slot_1163,
                                                *__slate_slot_1184,
                                                *__slate_slot_1162,
                                                *__slate_slot_1179,
                                                *__slate_slot_1180,
                                                (*__slate_slot_1178 as *mut ()) as *const i8,
                                                -(2 as i32),
                                            )
                                        };
                                        {}
                                        {}
                                        {}
                                        {}
                                        unsafe {
                                            sqlite3VdbeChangeP5(
                                                *__slate_slot_1163,
                                                ((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_1165)
                                                            .offset((0 as i32) as isize)
                                                    }
                                                })
                                                    as i16)
                                                    as u16,
                                            )
                                        };
                                    } else {
                                        std::ptr::write(
                                            __slate_slot_1185,
                                            if *__slate_slot_1162 != *__slate_slot_1180 {
                                                53 as i32
                                            } else {
                                                51 as i32
                                            },
                                        );
                                        0 as i32;
                                        unsafe {
                                            sqlite3VdbeAddOp4(
                                                *__slate_slot_1163,
                                                *__slate_slot_1185,
                                                *__slate_slot_1162,
                                                destIfFalse,
                                                *__slate_slot_1180,
                                                (*__slate_slot_1178 as *mut ()) as *const i8,
                                                -(2 as i32),
                                            )
                                        };
                                        {}
                                        {}
                                        unsafe {
                                            sqlite3VdbeChangeP5(
                                                *__slate_slot_1163,
                                                ((((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_1165)
                                                            .offset((0 as i32) as isize)
                                                    }
                                                })
                                                    as i32)
                                                    | (16 as i32))
                                                    as i16)
                                                    as u16,
                                            )
                                        };
                                    }
                                    std::ptr::write(__slate_slot_2687, *__slate_slot_1183);
                                    std::ptr::write(
                                        __slate_slot_2688,
                                        *__slate_slot_2687 + (1 as i32),
                                    );
                                    *__slate_slot_1183 = *__slate_slot_2688;
                                } else {
                                    break;
                                }
                            }
                            if *__slate_slot_1182 != (0 as i32) {
                                unsafe {
                                    sqlite3VdbeAddOp2(
                                        *__slate_slot_1163,
                                        51 as i32,
                                        *__slate_slot_1182,
                                        destIfNull,
                                    )
                                };
                                {}
                                unsafe { sqlite3VdbeGoto(*__slate_slot_1163, destIfFalse) };
                            }
                            unsafe {
                                sqlite3VdbeResolveLabel(*__slate_slot_1163, *__slate_slot_1179)
                            };
                            sqlite3ReleaseTempReg(pParse, *__slate_slot_1182);
                        } else {
                            if *__slate_slot_1161 != (1 as i32) {
                                // If this IN operator will use an index, then the order of columns in the
                                // vector might be different from the order in the index.  In that case,
                                // we need to reorder the LHS values to be in index order.  Run Affinity
                                // before reordering the columns, so that the affinity is correct.
                                if *__slate_slot_1166 == (1 as i32) {
                                    std::ptr::write(__slate_slot_1186, unsafe {
                                        *unsafe { (*__slate_slot_1165).offset((0 as i32) as isize) }
                                    });
                                    if (*__slate_slot_1186 as i32) >= (66 as i32) {
                                        *__slate_slot_2690 = (*__slate_slot_1186 as i32)
                                            != (sqlite3ExprAffinity(
                                                *__slate_slot_1168 as *const Expr,
                                            )
                                                as i32);
                                    } else {
                                        *__slate_slot_2690 = false as bool;
                                    }
                                    if *__slate_slot_2690 {
                                        std::ptr::write(
                                            __slate_slot_1187,
                                            sqlite3GetTempReg(pParse),
                                        );
                                        unsafe {
                                            sqlite3VdbeAddOp3(
                                                *__slate_slot_1163,
                                                82 as i32,
                                                *__slate_slot_1162,
                                                *__slate_slot_1187,
                                                0 as i32,
                                            )
                                        };
                                        *__slate_slot_1162 = *__slate_slot_1187;
                                        unsafe {
                                            sqlite3VdbeAddOp4(
                                                *__slate_slot_1163,
                                                98 as i32,
                                                *__slate_slot_1162,
                                                1 as i32,
                                                0 as i32,
                                                *__slate_slot_1165 as *const i8,
                                                1 as i32,
                                            )
                                        };
                                    }
                                } else {
                                    unsafe {
                                        sqlite3VdbeAddOp4(
                                            *__slate_slot_1163,
                                            98 as i32,
                                            *__slate_slot_1162,
                                            *__slate_slot_1166,
                                            0 as i32,
                                            *__slate_slot_1165 as *const i8,
                                            *__slate_slot_1166,
                                        )
                                    };
                                }
                                *__slate_slot_1169 = 0 as i32;
                                loop {
                                    if *__slate_slot_1169 < *__slate_slot_1166
                                        && (unsafe {
                                            *unsafe {
                                                (*__slate_slot_1164)
                                                    .offset(*__slate_slot_1169 as isize)
                                            }
                                        }) == *__slate_slot_1169
                                    {
                                        std::ptr::write(__slate_slot_2691, *__slate_slot_1169);
                                        std::ptr::write(
                                            __slate_slot_2692,
                                            *__slate_slot_2691 + (1 as i32),
                                        );
                                        *__slate_slot_1169 = *__slate_slot_2692;
                                    } else {
                                        break;
                                    }
                                }
                                // Are LHS fields reordered?
                                if *__slate_slot_1169 != *__slate_slot_1166 {
                                    std::ptr::write(__slate_slot_1188, *__slate_slot_1162);
                                    *__slate_slot_1162 =
                                        sqlite3GetTempRange(pParse, *__slate_slot_1166);
                                    *__slate_slot_1169 = 0 as i32;
                                    loop {
                                        if *__slate_slot_1169 < *__slate_slot_1166 {
                                            {}
                                            unsafe {
                                                sqlite3VdbeAddOp3(
                                                    *__slate_slot_1163,
                                                    82 as i32,
                                                    *__slate_slot_1188 + *__slate_slot_1169,
                                                    *__slate_slot_1162
                                                        + unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_1164).offset(
                                                                    *__slate_slot_1169 as isize,
                                                                )
                                                            }
                                                        },
                                                    0 as i32,
                                                )
                                            };
                                            std::ptr::write(__slate_slot_2693, *__slate_slot_1169);
                                            std::ptr::write(
                                                __slate_slot_2694,
                                                *__slate_slot_2693 + (1 as i32),
                                            );
                                            *__slate_slot_1169 = *__slate_slot_2694;
                                        } else {
                                            break;
                                        }
                                    }
                                    sqlite3ReleaseTempReg(pParse, *__slate_slot_1188);
                                }
                            }
                            // Step 2: Check to see if the LHS contains any NULL columns.  If the
                            // LHS does contain NULLs then the result must be either FALSE or NULL.
                            // We will then skip the binary search of the RHS.
                            if destIfNull == destIfFalse {
                                *__slate_slot_1170 = destIfFalse;
                            } else {
                                std::ptr::write(__slate_slot_2695, unsafe {
                                    sqlite3VdbeMakeLabel(pParse)
                                });
                                *__slate_slot_1171 = *__slate_slot_2695;
                                *__slate_slot_1170 = *__slate_slot_2695;
                            }
                            *__slate_slot_1169 = 0 as i32;
                            loop {
                                if *__slate_slot_1169 < *__slate_slot_1166 {
                                    std::ptr::write(
                                        __slate_slot_1189,
                                        sqlite3VectorFieldSubexpr(
                                            unsafe { (*pExpr).pLeft },
                                            *__slate_slot_1169,
                                        ),
                                    );
                                    if (unsafe { (*pParse).nErr }) != (0 as i32) {
                                        break '__join_0;
                                    } else {
                                        if sqlite3ExprCanBeNull(*__slate_slot_1189 as *const Expr)
                                            != (0 as i32)
                                        {
                                            {}
                                            unsafe {
                                                sqlite3VdbeAddOp2(
                                                    *__slate_slot_1163,
                                                    51 as i32,
                                                    *__slate_slot_1162
                                                        + unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_1164).offset(
                                                                    *__slate_slot_1169 as isize,
                                                                )
                                                            }
                                                        },
                                                    *__slate_slot_1170,
                                                )
                                            };
                                            {}
                                        }
                                        std::ptr::write(__slate_slot_2696, *__slate_slot_1169);
                                        std::ptr::write(
                                            __slate_slot_2697,
                                            *__slate_slot_2696 + (1 as i32),
                                        );
                                        *__slate_slot_1169 = *__slate_slot_2697;
                                    }
                                } else {
                                    break;
                                }
                            }
                            // Step 3.  The LHS is now known to be non-NULL.  Do the binary search
                            // of the RHS using the LHS as a probe.  If found, the result is
                            // true.
                            if *__slate_slot_1161 == (1 as i32) {
                                // In this case, the RHS is the ROWID of table b-tree and so we also
                                // know that the RHS is non-NULL.  Hence, we combine steps 3 and 4
                                // into a single opcode.
                                0 as i32;
                                unsafe {
                                    sqlite3VdbeAddOp3(
                                        *__slate_slot_1163,
                                        30 as i32,
                                        *__slate_slot_1175,
                                        destIfFalse,
                                        *__slate_slot_1162,
                                    )
                                };
                                {}
                                *__slate_slot_1172 =
                                    unsafe { sqlite3VdbeAddOp0(*__slate_slot_1163, 9 as i32) }; // Return True
                            } else {
                                if destIfFalse == destIfNull {
                                    // Combine Step 3 and Step 5 into a single opcode
                                    if (unsafe { (*pExpr).flags }) & ((33554432 as i32) as u32)
                                        != ((0 as i32) as u32)
                                    {
                                        std::ptr::write(
                                            __slate_slot_1190,
                                            (unsafe {
                                                sqlite3VdbeGetOp(*__slate_slot_1163, unsafe {
                                                    (*pExpr).y.sub.iAddr
                                                })
                                            })
                                                as *const VdbeOp,
                                        );
                                        0 as i32;
                                        if (unsafe { (*(*__slate_slot_1190)).p3 }) > (0 as i32) {
                                            // tag-202407032019
                                            0 as i32;
                                            unsafe {
                                                sqlite3VdbeAddOp4Int(
                                                    *__slate_slot_1163,
                                                    66 as i32,
                                                    unsafe { (*(*__slate_slot_1190)).p3 },
                                                    destIfFalse,
                                                    *__slate_slot_1162,
                                                    *__slate_slot_1166,
                                                )
                                            };
                                            {}
                                        }
                                    }
                                    unsafe {
                                        sqlite3VdbeAddOp4Int(
                                            *__slate_slot_1163,
                                            28 as i32,
                                            *__slate_slot_1175,
                                            destIfFalse,
                                            *__slate_slot_1162,
                                            *__slate_slot_1166,
                                        )
                                    };
                                    {}
                                    break '__join_1;
                                } else {
                                    // Ordinary Step 3, for the case where FALSE and NULL are distinct
                                    *__slate_slot_1172 = unsafe {
                                        sqlite3VdbeAddOp4Int(
                                            *__slate_slot_1163,
                                            29 as i32,
                                            *__slate_slot_1175,
                                            0 as i32,
                                            *__slate_slot_1162,
                                            *__slate_slot_1166,
                                        )
                                    };
                                    {}
                                }
                            }
                            // Step 4.  If the RHS is known to be non-NULL and we did not find
                            // an match on the search above, then the result must be FALSE.
                            if *__slate_slot_1160 != (0 as i32) && *__slate_slot_1166 == (1 as i32)
                            {
                                unsafe {
                                    sqlite3VdbeAddOp2(
                                        *__slate_slot_1163,
                                        52 as i32,
                                        *__slate_slot_1160,
                                        destIfFalse,
                                    )
                                };
                                {}
                            }
                            // Step 5.  If we do not care about the difference between NULL and
                            // FALSE, then just return false.
                            if destIfFalse == destIfNull {
                                unsafe { sqlite3VdbeGoto(*__slate_slot_1163, destIfFalse) };
                            }
                            // Step 6: Loop through rows of the RHS.  Compare each row to the LHS.
                            // If any comparison is NULL, then the result is NULL.  If all
                            // comparisons are FALSE then the final result is FALSE.
                            //
                            // For a scalar LHS, it is sufficient to check just the first row
                            // of the RHS.
                            if *__slate_slot_1171 != (0 as i32) {
                                unsafe {
                                    sqlite3VdbeResolveLabel(*__slate_slot_1163, *__slate_slot_1171)
                                };
                            }
                            if *__slate_slot_1161 == (4 as i32) {
                                *__slate_slot_1174 = unsafe {
                                    sqlite3VdbeAddOp2(
                                        *__slate_slot_1163,
                                        32 as i32,
                                        *__slate_slot_1175,
                                        destIfFalse,
                                    )
                                };
                            } else {
                                {}
                                {}
                                *__slate_slot_1174 = unsafe {
                                    sqlite3VdbeAddOp2(
                                        *__slate_slot_1163,
                                        36 as i32,
                                        *__slate_slot_1175,
                                        destIfFalse,
                                    )
                                };
                            }
                            {}
                            if *__slate_slot_1166 > (1 as i32) {
                                *__slate_slot_1173 = unsafe { sqlite3VdbeMakeLabel(pParse) };
                            } else {
                                // For nVector==1, combine steps 6 and 7 by immediately returning
                                // FALSE if the first comparison is not NULL
                                *__slate_slot_1173 = destIfFalse;
                            }
                            *__slate_slot_1169 = 0 as i32;
                            loop {
                                if *__slate_slot_1169 < *__slate_slot_1166 {
                                    std::ptr::write(__slate_slot_1193, sqlite3GetTempReg(pParse));
                                    *__slate_slot_1191 = sqlite3VectorFieldSubexpr(
                                        *__slate_slot_1168,
                                        *__slate_slot_1169,
                                    );
                                    if (unsafe { (*pExpr).flags }) & ((4096 as i32) as u32)
                                        != ((0 as i32) as u32)
                                    {
                                        std::ptr::write(__slate_slot_1194, unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!(
                                                        (*unsafe {
                                                            (*unsafe { (*pExpr).x.pSelect }).pEList
                                                        })
                                                        .a
                                                    )
                                                        as *mut ExprList_item
                                                }
                                                .offset(*__slate_slot_1169 as isize)
                                            })
                                            .pExpr
                                        });
                                        *__slate_slot_1192 = sqlite3BinaryCompareCollSeq(
                                            pParse,
                                            *__slate_slot_1191 as *const Expr,
                                            *__slate_slot_1194 as *const Expr,
                                        );
                                    } else {
                                        // If the RHS of the IN(...) expression are scalar expressions, do
                                        // not consider their collation sequences. The documentation says
                                        // "The collating sequence used for expressions of the form "x IN (y, z,
                                        // ...)" is the collating sequence of x.".
                                        *__slate_slot_1192 = sqlite3ExprCollSeq(
                                            pParse,
                                            *__slate_slot_1191 as *const Expr,
                                        );
                                    }
                                    {}
                                    unsafe {
                                        sqlite3VdbeAddOp3(
                                            *__slate_slot_1163,
                                            96 as i32,
                                            *__slate_slot_1175,
                                            unsafe {
                                                *unsafe {
                                                    (*__slate_slot_1164)
                                                        .offset(*__slate_slot_1169 as isize)
                                                }
                                            },
                                            *__slate_slot_1193,
                                        )
                                    };
                                    unsafe {
                                        sqlite3VdbeAddOp4(
                                            *__slate_slot_1163,
                                            53 as i32,
                                            *__slate_slot_1162
                                                + unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_1164)
                                                            .offset(*__slate_slot_1169 as isize)
                                                    }
                                                },
                                            *__slate_slot_1173,
                                            *__slate_slot_1193,
                                            (*__slate_slot_1192 as *mut ()) as *const i8,
                                            -(2 as i32),
                                        )
                                    };
                                    {}
                                    sqlite3ReleaseTempReg(pParse, *__slate_slot_1193);
                                    std::ptr::write(__slate_slot_2698, *__slate_slot_1169);
                                    std::ptr::write(
                                        __slate_slot_2699,
                                        *__slate_slot_2698 + (1 as i32),
                                    );
                                    *__slate_slot_1169 = *__slate_slot_2699;
                                } else {
                                    break;
                                }
                            }
                            unsafe {
                                sqlite3VdbeAddOp2(
                                    *__slate_slot_1163,
                                    9 as i32,
                                    0 as i32,
                                    destIfNull,
                                )
                            };
                            if *__slate_slot_1166 > (1 as i32) {
                                unsafe {
                                    sqlite3VdbeResolveLabel(*__slate_slot_1163, *__slate_slot_1173)
                                };
                                unsafe {
                                    sqlite3VdbeAddOp2(
                                        *__slate_slot_1163,
                                        40 as i32,
                                        *__slate_slot_1175,
                                        *__slate_slot_1174 + (1 as i32),
                                    )
                                };
                                {}
                                // Step 7:  If we reach this point, we know that the result must
                                // be false.
                                unsafe {
                                    sqlite3VdbeAddOp2(
                                        *__slate_slot_1163,
                                        9 as i32,
                                        0 as i32,
                                        destIfFalse,
                                    )
                                };
                            }
                            // Jumps here in order to return true.
                            unsafe { sqlite3VdbeJumpHere(*__slate_slot_1163, *__slate_slot_1172) };
                        }
                    }
                    unsafe {
                        sqlite3VdbeComment(
                            *__slate_slot_1163,
                            (b"end IN expr\0".as_ptr() as *mut i8) as *const i8,
                        )
                    };
                }
            }
            unsafe { sqlite3DbFree(unsafe { (*pParse).db }, *__slate_slot_1164 as *mut ()) };
            unsafe { sqlite3DbFree(unsafe { (*pParse).db }, *__slate_slot_1165 as *mut ()) };
        }
    }
}

/// Generate an instruction that will put the floating point
/// value described by z[0..n-1] into register iMem.
///
/// The z[] string will probably not be zero-terminated.  But the
/// z[n] character is guaranteed to be something that does not look
/// like the continuation of the number.
fn codeReal(mut v: *mut Vdbe, mut z: *const i8, mut negateFlag: i32, mut iMem: i32) {
    if z != std::ptr::null::<i8>() {
        let mut value: f64 = 0 as f64;
        unsafe { sqlite3AtoF(z, std::ptr::addr_of_mut!(value)) };
        0 as i32; // The new AtoF never returns NaN
        if negateFlag != (0 as i32) {
            value = -value;
        }
        unsafe { sqlite3VdbeAddDouble(v, iMem, value) };
    }
}

/// Generate an instruction that will put the integer describe by
/// text z[0..n-1] into register iMem.
///
/// Expr.u.zToken is always UTF8 and zero-terminated.
fn codeInteger(mut pParse: *mut Parse, mut pExpr: *mut Expr, mut negFlag: i32, mut iMem: i32) {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    if (unsafe { (*pExpr).flags }) & ((2048 as i32) as u32) != (0 as u32) {
        let mut i: i32 = unsafe { (*pExpr).u.iValue };
        0 as i32;
        if negFlag != (0 as i32) {
            i = -i;
        }
        unsafe { sqlite3VdbeAddOp2(v, 73 as i32, i, iMem) };
    } else {
        let mut c: i32 = 0 as i32;
        let mut value: i64 = 0 as i64;
        let mut z: *const i8 = (unsafe { (*pExpr).u.zToken }) as *const i8;
        0 as i32;
        c = unsafe { sqlite3DecOrHexToI64(z, std::ptr::addr_of_mut!(value)) };
        if c == (3 as i32) && !(negFlag != (0 as i32))
            || c == (2 as i32)
            || negFlag != (0 as i32)
                && value
                    == (-(1 as i32) as i64)
                        - ((((4294967295 as u32) as u64) as i64)
                            | ((2147483647 as i32) as i64) << (32 as i32))
        {
            if (unsafe {
                sqlite3_strnicmp(z, (b"0x\0".as_ptr() as *mut i8) as *const i8, 2 as i32)
            }) == (0 as i32)
            {
                unsafe {
                    sqlite3ErrorMsg(
                        pParse,
                        (b"hex literal too big: %s%#T\0".as_ptr() as *mut i8) as *const i8,
                        if negFlag != (0 as i32) {
                            b"-\0".as_ptr() as *mut i8
                        } else {
                            b"\0".as_ptr() as *mut i8
                        },
                        pExpr,
                    )
                };
            } else {
                codeReal(v, z, negFlag, iMem);
            }
        } else {
            if negFlag != (0 as i32) {
                value = if c == (3 as i32) {
                    (-(1 as i32) as i64)
                        - ((((4294967295 as u32) as u64) as i64)
                            | ((2147483647 as i32) as i64) << (32 as i32))
                } else {
                    -value
                };
            }
            unsafe { sqlite3VdbeAddInt64(v, iMem, value) };
        }
    }
}

/// Generate code that will load into register regOut a value that is
/// appropriate for the iIdxCol-th column of index pIdx.
///
/// # Arguments
///
/// * `pParse` - The parsing context
/// * `pIdx` - The index whose column is to be loaded
/// * `iTabCur` - Cursor pointing to a table row
/// * `iIdxCol` - The column of the index to be loaded
/// * `regOut` - Store the index column value in this register
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCodeLoadIndexColumn(
    mut pParse: *mut Parse,
    mut pIdx: *mut Index,
    mut iTabCur: i32,
    mut iIdxCol: i32,
    mut regOut: i32,
) {
    let mut iTabCol: i16 =
        unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(iIdxCol as isize) } };
    if (iTabCol as i32) == -(2 as i32) {
        0 as i32;
        0 as i32;
        unsafe {
            (*pParse).iSelfTab = iTabCur + (1 as i32);
        }
        sqlite3ExprCodeCopy(
            pParse,
            unsafe {
                (*unsafe {
                    unsafe {
                        std::ptr::addr_of_mut!((*unsafe { (*pIdx).aColExpr }).a)
                            as *mut ExprList_item
                    }
                    .offset(iIdxCol as isize)
                })
                .pExpr
            },
            regOut,
        );
        unsafe {
            (*pParse).iSelfTab = 0 as i32;
        }
    } else {
        sqlite3ExprCodeGetColumnOfTable(
            unsafe { (*pParse).pVdbe },
            unsafe { (*pIdx).pTable },
            iTabCur,
            iTabCol as i32,
            regOut,
        );
    }
}

/// Generate code that will compute the value of generated column pCol
/// and store the result in register regOut
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pTab` - Table containing the generated column
/// * `pCol` - The generated column
/// * `regOut` - Put the result in this register
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCodeGeneratedColumn(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pCol: *mut Column,
    mut regOut: i32,
) {
    let mut iAddr: i32 = 0 as i32;
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut nErr: i32 = unsafe { (*pParse).nErr };
    0 as i32;
    0 as i32;
    if (unsafe { (*pParse).iSelfTab }) > (0 as i32) {
        iAddr = unsafe {
            sqlite3VdbeAddOp3(
                v,
                20 as i32,
                (unsafe { (*pParse).iSelfTab }) - (1 as i32),
                0 as i32,
                regOut,
            )
        };
    } else {
        iAddr = 0 as i32;
    }
    sqlite3ExprCodeCopy(pParse, unsafe { sqlite3ColumnExpr(pTab, pCol) }, regOut);
    if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (32 as i32) != (0 as i32)
        && (unsafe { (*pTab).tabFlags }) & ((65536 as i32) as u32) != ((0 as i32) as u32)
    {
        let mut p3: i32 = (2 as i32)
            + (((unsafe { pCol.offset_from((unsafe { (*pTab).aCol }) as *mut Column) }) as i64)
                as i32);
        unsafe {
            sqlite3VdbeAddOp4(
                v,
                97 as i32,
                regOut,
                1 as i32,
                p3,
                (pTab as *mut i8) as *const i8,
                -(5 as i32),
            )
        };
    } else {
        if ((unsafe { (*pCol).affinity }) as i32) >= (66 as i32) {
            unsafe {
                sqlite3VdbeAddOp4(
                    v,
                    98 as i32,
                    regOut,
                    1 as i32,
                    0 as i32,
                    (unsafe { std::ptr::addr_of_mut!((*pCol).affinity) }) as *const i8,
                    1 as i32,
                )
            };
        }
    }
    if iAddr != (0 as i32) {
        unsafe { sqlite3VdbeJumpHere(v, iAddr) };
    }
    if (unsafe { (*pParse).nErr }) > nErr {
        unsafe {
            (*unsafe { (*pParse).db }).errByteOffset = -(1 as i32);
        }
    }
}

/// Generate code to extract the value of the iCol-th column of a table.
///
/// # Arguments
///
/// * `v` - Parsing context
/// * `pTab` - The table containing the value
/// * `iTabCur` - The table cursor.  Or the PK cursor for WITHOUT ROWID
/// * `iCol` - Index of the column to extract
/// * `regOut` - Extract the value into this register
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCodeGetColumnOfTable(
    mut v: *mut Vdbe,
    mut pTab: *mut Table,
    mut iTabCur: i32,
    mut iCol: i32,
    mut regOut: i32,
) {
    let mut pCol: *mut Column = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    if iCol < (0 as i32) || iCol == ((unsafe { (*pTab).iPKey }) as i32) {
        unsafe { sqlite3VdbeAddOp2(v, 137 as i32, iTabCur, regOut) };
        unsafe {
            sqlite3VdbeComment(
                v,
                (b"%s.rowid\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pTab).zName },
            )
        };
    } else {
        let mut op: i32 = 0 as i32;
        let mut x: i32 = 0 as i32;
        if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32) {
            op = 178 as i32;
            x = iCol;
        } else {
            let __v2429: *mut Column = unsafe { unsafe { (*pTab).aCol }.offset(iCol as isize) };
            pCol = __v2429;
            if (((unsafe { (*__v2429).colFlags }) as u32) as i32) & (32 as i32) != (0 as i32) {
                let mut pParse: *mut Parse = unsafe { sqlite3VdbeParser(v) };
                if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (256 as i32) != (0 as i32) {
                    unsafe {
                        sqlite3ErrorMsg(
                            pParse,
                            (b"generated column loop on \"%s\"\0".as_ptr() as *mut i8) as *const i8,
                            unsafe { (*pCol).zCnName },
                        )
                    };
                } else {
                    let mut savedSelfTab: i32 = unsafe { (*pParse).iSelfTab };
                    let __v2430: *mut Column = pCol;
                    let __v2431: u16 = unsafe { (*__v2430).colFlags };
                    let __v2432: u16 = ((((__v2431 as u32) as i32) | (256 as i32)) as i16) as u16;
                    unsafe {
                        (*__v2430).colFlags = __v2432;
                    }
                    unsafe {
                        (*pParse).iSelfTab = iTabCur + (1 as i32);
                    }
                    sqlite3ExprCodeGeneratedColumn(pParse, pTab, pCol, regOut);
                    unsafe {
                        (*pParse).iSelfTab = savedSelfTab;
                    }
                    let __v2433: *mut Column = pCol;
                    let __v2434: u16 = unsafe { (*__v2433).colFlags };
                    let __v2435: u16 = ((((__v2434 as u32) as i32) & !(256 as i32)) as i16) as u16;
                    unsafe {
                        (*__v2433).colFlags = __v2435;
                    }
                }
                return;
            } else {
                if !((unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32)) {
                    {}
                    x = unsafe {
                        sqlite3TableColumnToIndex(unsafe { sqlite3PrimaryKeyIndex(pTab) }, iCol)
                    };
                    op = 96 as i32;
                } else {
                    x = (unsafe { sqlite3TableColumnToStorage(pTab, iCol as i16) }) as i32;
                    {}
                    op = 96 as i32;
                }
            }
        }
        unsafe { sqlite3VdbeAddOp3(v, op, iTabCur, x, regOut) };
        unsafe { sqlite3ColumnDefault(v, pTab, iCol, regOut) };
    }
}

/// Generate code that will extract the iColumn-th column from
/// table pTab and store the column value in register iReg.
///
/// There must be an open cursor to pTab in iTable when this routine
/// is called.  If iColumn<0 then code is generated that extracts the rowid.
///
/// # Arguments
///
/// * `pParse` - Parsing and code generating context
/// * `pTab` - Description of the table we are reading from
/// * `iColumn` - Index of the table column
/// * `iTable` - The cursor pointing to the table
/// * `iReg` - Store results here
/// * `p5` - P5 value for OP_Column + FLAGS
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCodeGetColumn(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut iColumn: i32,
    mut iTable: i32,
    mut iReg: i32,
    mut p5: u8,
) -> i32 {
    0 as i32;
    0 as i32;
    0 as i32;
    sqlite3ExprCodeGetColumnOfTable(unsafe { (*pParse).pVdbe }, pTab, iTable, iColumn, iReg);
    if p5 != (0 as u8) {
        let mut pOp: *mut VdbeOp = unsafe { sqlite3VdbeGetLastOp(unsafe { (*pParse).pVdbe }) };
        if (((unsafe { (*pOp).opcode }) as u32) as i32) == (96 as i32) {
            unsafe {
                (*pOp).p5 = p5 as u16;
            }
        }
        if (((unsafe { (*pOp).opcode }) as u32) as i32) == (178 as i32) {
            unsafe {
                (*pOp).p5 = ((((p5 as u32) as i32) & (1 as i32)) as i16) as u16;
            }
        }
    }
    return iReg;
}

/// Generate code to move content from registers iFrom...iFrom+nReg-1
/// over to iTo..iTo+nReg-1.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCodeMove(
    mut pParse: *mut Parse,
    mut iFrom: i32,
    mut iTo: i32,
    mut nReg: i32,
) {
    unsafe { sqlite3VdbeAddOp3(unsafe { (*pParse).pVdbe }, 81 as i32, iFrom, iTo, nReg) };
}

/// Convert a scalar expression node to a TK_REGISTER referencing
/// register iReg.  The caller must ensure that iReg already contains
/// the correct value for the expression.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprToRegister(mut pExpr: *mut Expr, mut iReg: i32) {
    let mut p: *mut Expr = sqlite3ExprSkipCollateAndLikely(pExpr);
    if p == std::ptr::null_mut::<Expr>() {
        return;
    }
    if (((unsafe { (*p).op }) as u32) as i32) == (176 as i32) {
        0 as i32;
    } else {
        unsafe {
            (*p).op2 = unsafe { (*p).op };
        }
        unsafe {
            (*p).op = ((176 as i32) as i8) as u8;
        }
        unsafe {
            (*p).iTable = iReg;
        }
        let __v2436: *mut Expr = p;
        let __v2437: u32 = unsafe { (*__v2436).flags };
        let __v2438: u32 = __v2437 & !((8192 as i32) as u32);
        unsafe {
            (*__v2436).flags = __v2438;
        }
    }
}

/// Evaluate an expression (either a vector or a scalar expression) and store
/// the result in contiguous temporary registers.  Return the index of
/// the first register used to store the result.
///
/// If the returned result register is a temporary scalar, then also write
/// that register number into *piFreeable.  If the returned result register
/// is not a temporary or if the expression is a vector set *piFreeable
/// to 0.
fn exprCodeVector(mut pParse: *mut Parse, mut p: *mut Expr, mut piFreeable: *mut i32) -> i32 {
    let mut iResult: i32 = 0 as i32;
    let mut nResult: i32 = sqlite3ExprVectorSize(p as *const Expr);
    if nResult == (1 as i32) {
        iResult = sqlite3ExprCodeTemp(pParse, p, piFreeable);
    } else {
        unsafe {
            *piFreeable = 0 as i32;
        }
        if (((unsafe { (*p).op }) as u32) as i32) == (139 as i32) {
            iResult = sqlite3CodeSubselect(pParse, p);
        } else {
            let mut i: i32 = 0 as i32;
            iResult = (unsafe { (*pParse).nMem }) + (1 as i32);
            let __v2625: *mut Parse = pParse;
            let __v2626: i32 = unsafe { (*__v2625).nMem };
            let __v2627: i32 = __v2626 + nResult;
            unsafe {
                (*__v2625).nMem = __v2627;
            }
            0 as i32;
            i = 0 as i32;
            '__slate_break_2310: loop {
                if !(i < nResult) {
                    break;
                }
                sqlite3ExprCodeFactorable(
                    pParse,
                    unsafe {
                        (*unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!((*unsafe { (*p).x.pList }).a)
                                    as *mut ExprList_item
                            }
                            .offset(i as isize)
                        })
                        .pExpr
                    },
                    i + iResult,
                );
                let __v2628: i32 = i;
                let __v2629: i32 = __v2628 + (1 as i32);
                i = __v2629;
            }
        }
    }
    return iResult;
}

/// If the last opcode is a OP_Copy, then set the do-not-merge flag (p5)
/// so that a subsequent copy will not be merged into this one.
fn setDoNotMergeFlagOnCopy(mut v: *mut Vdbe) {
    if (((unsafe { (*unsafe { sqlite3VdbeGetLastOp(v) }).opcode }) as u32) as i32) == (82 as i32) {
        unsafe { sqlite3VdbeChangeP5(v, ((1 as i32) as i16) as u16) }; // Tag trailing OP_Copy as not mergeable
    }
}

/// Generate code to implement special SQL functions that are implemented
/// in-line rather than by using the usual callbacks.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pFarg` - List of function arguments
/// * `iFuncId` - Function ID.  One of the INTFUNC_... values
/// * `target` - Store function result in this register
fn exprCodeInlineFunction(
    mut pParse: *mut Parse,
    mut pFarg: *mut ExprList,
    mut iFuncId: i32,
    mut target: i32,
) -> i32 {
    let mut nFarg: i32 = 0 as i32;
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    0 as i32;
    0 as i32;
    nFarg = unsafe { (*pFarg).nExpr };
    0 as i32; // All in-line functions have at least one argument
    '__slate_break_2311: {
        match iFuncId {
            0 => {
                // Attempt a direct implementation of the built-in COALESCE() and
                // IFNULL() functions.  This avoids unnecessary evaluation of
                // arguments past the first non-NULL argument.
                let mut endCoalesce: i32 = unsafe { sqlite3VdbeMakeLabel(pParse) };
                let mut i: i32 = 0 as i32;
                0 as i32;
                sqlite3ExprCode(
                    pParse,
                    unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pFarg).a) as *mut ExprList_item }
                                .offset((0 as i32) as isize)
                        })
                        .pExpr
                    },
                    target,
                );
                i = 1 as i32;
                '__slate_break_2312: loop {
                    if !(i < nFarg) {
                        break;
                    }
                    unsafe { sqlite3VdbeAddOp2(v, 52 as i32, target, endCoalesce) };
                    {}
                    sqlite3ExprCode(
                        pParse,
                        unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*pFarg).a) as *mut ExprList_item }
                                    .offset(i as isize)
                            })
                            .pExpr
                        },
                        target,
                    );
                    let __v2700: i32 = i;
                    let __v2701: i32 = __v2700 + (1 as i32);
                    i = __v2701;
                }
                setDoNotMergeFlagOnCopy(v);
                unsafe { sqlite3VdbeResolveLabel(v, endCoalesce) };
            }
            5 => {
                let mut caseExpr: Expr = unsafe { std::mem::zeroed() };
                unsafe {
                    memset(
                        std::ptr::addr_of_mut!(caseExpr) as *mut (),
                        0 as i32,
                        72 as u64,
                    )
                };
                caseExpr.op = ((158 as i32) as i8) as u8;
                unsafe {
                    caseExpr.x.pList = pFarg;
                }
                return sqlite3ExprCodeTarget(pParse, std::ptr::addr_of_mut!(caseExpr), target);
            }
            6 => {
                let mut pArg: *mut Expr = unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pFarg).a) as *mut ExprList_item }
                            .offset((0 as i32) as isize)
                    })
                    .pExpr
                };
                if (((unsafe { (*pArg).op }) as u32) as i32) == (168 as i32)
                    && (unsafe { (*pArg).iTable }) >= (0 as i32)
                {
                    unsafe {
                        sqlite3VdbeAddOp3(
                            v,
                            95 as i32,
                            unsafe { (*pArg).iTable },
                            (unsafe { (*pArg).iColumn }) as i32,
                            target,
                        )
                    };
                } else {
                    unsafe { sqlite3VdbeAddOp2(v, 77 as i32, 0 as i32, target) };
                }
            }
            3 => {
                // Compare two expressions using sqlite3ExprCompare()
                0 as i32;
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        73 as i32,
                        sqlite3ExprCompare(
                            std::ptr::null::<Parse>(),
                            (unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pFarg).a) as *mut ExprList_item
                                    }
                                    .offset((0 as i32) as isize)
                                })
                                .pExpr
                            }) as *const Expr,
                            (unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pFarg).a) as *mut ExprList_item
                                    }
                                    .offset((1 as i32) as isize)
                                })
                                .pExpr
                            }) as *const Expr,
                            -(1 as i32),
                        ),
                        target,
                    )
                };
            }
            2 => {
                // Compare two expressions using sqlite3ExprImpliesExpr()
                0 as i32;
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        73 as i32,
                        sqlite3ExprImpliesExpr(
                            pParse as *const Parse,
                            (unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pFarg).a) as *mut ExprList_item
                                    }
                                    .offset((0 as i32) as isize)
                                })
                                .pExpr
                            }) as *const Expr,
                            (unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pFarg).a) as *mut ExprList_item
                                    }
                                    .offset((1 as i32) as isize)
                                })
                                .pExpr
                            }) as *const Expr,
                            -(1 as i32),
                        ),
                        target,
                    )
                };
            }
            1 => {
                // Result of sqlite3ExprImpliesNonNullRow()
                let mut pA1: *mut Expr = unsafe { std::mem::zeroed() };
                0 as i32;
                pA1 = unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pFarg).a) as *mut ExprList_item }
                            .offset((1 as i32) as isize)
                    })
                    .pExpr
                };
                if (((unsafe { (*pA1).op }) as u32) as i32) == (168 as i32) {
                    unsafe {
                        sqlite3VdbeAddOp2(
                            v,
                            73 as i32,
                            sqlite3ExprImpliesNonNullRow(
                                unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pFarg).a) as *mut ExprList_item
                                        }
                                        .offset((0 as i32) as isize)
                                    })
                                    .pExpr
                                },
                                unsafe { (*pA1).iTable },
                                1 as i32,
                            ),
                            target,
                        )
                    };
                } else {
                    unsafe { sqlite3VdbeAddOp2(v, 77 as i32, 0 as i32, target) };
                }
            }
            4 => {
                // The AFFINITY() function evaluates to a string that describes
                // the type affinity of the argument.  This is used for testing of
                // the SQLite type logic.
                let mut azAff: __SlateAlign16<[*const i8; 6]> = __SlateAlign16([
                    (b"blob\0".as_ptr() as *mut i8) as *const i8,
                    (b"text\0".as_ptr() as *mut i8) as *const i8,
                    (b"numeric\0".as_ptr() as *mut i8) as *const i8,
                    (b"integer\0".as_ptr() as *mut i8) as *const i8,
                    (b"real\0".as_ptr() as *mut i8) as *const i8,
                    (b"flexnum\0".as_ptr() as *mut i8) as *const i8,
                ]);
                let mut aff: i8 = 0 as i8;
                0 as i32;
                aff = sqlite3ExprAffinity(
                    (unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pFarg).a) as *mut ExprList_item }
                                .offset((0 as i32) as isize)
                        })
                        .pExpr
                    }) as *const Expr,
                );
                0 as i32;
                unsafe {
                    sqlite3VdbeLoadString(
                        v,
                        target,
                        if (aff as i32) <= (64 as i32) {
                            (b"none\0".as_ptr() as *mut i8) as *const i8
                        } else {
                            unsafe {
                                *unsafe {
                                    (azAff.0.as_mut_ptr() as *mut *const i8)
                                        .offset(((aff as i32) - (65 as i32)) as isize)
                                }
                            }
                        },
                    )
                };
            }
            _ => {
                // The UNLIKELY() function is a no-op.  The result is the value
                // of the first argument.
                0 as i32;
                target = sqlite3ExprCodeTarget(
                    pParse,
                    unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pFarg).a) as *mut ExprList_item }
                                .offset((0 as i32) as isize)
                        })
                        .pExpr
                    },
                    target,
                );
                break '__slate_break_2311;
                // Test-only SQL functions that are only usable if enabled
                // via SQLITE_TESTCTRL_INTERNAL_FUNCTIONS
            }
        }
    }
    return target;
}

/// Expression Node callback for sqlite3ExprCanReturnSubtype().  If
/// pExpr is able to return a subtype, set pWalker->eCode and abort
/// the search.  If pExpr can never return a subtype, prune search.
///
/// The only expressions that can return a subtype are:
///
///    1.  A function
///    2.  The no-op "+" operator
///    3.  A CASE...END expression
///    4.  A CAST() expression
///    5.  A "expr COLLATE colseq" expression.
///
/// For any other kind of expression, prune the search.
///
/// For case 1, the expression can yield a subtype if the function has
/// the SQLITE_RESULT_SUBTYPE property.  Functions can also return
/// a subtype (via sqlite3_result_value()) if any of the arguments can
/// return a subtype.
///
/// In all cases 1 through 5, the expression might also return a subtype
/// if any operand can return a subtype.
#[unsafe(link_section = ".text.slate_distinct.expr.exprNodeCanReturnSubtype")]
extern "C-unwind" fn exprNodeCanReturnSubtype(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> i32 {
    let mut n: i32 = 0 as i32;
    let mut pDef: *mut FuncDef = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (158 as i32)
        || (((unsafe { (*pExpr).op }) as u32) as i32) == (173 as i32)
        || (((unsafe { (*pExpr).op }) as u32) as i32) == (114 as i32)
        || (((unsafe { (*pExpr).op }) as u32) as i32) == (36 as i32)
    {
        return 0 as i32;
    }
    if (((unsafe { (*pExpr).op }) as u32) as i32) != (172 as i32) {
        return 1 as i32;
    }
    0 as i32;
    db = unsafe { (*unsafe { (*pWalker).pParse }).db };
    n = if (unsafe { (*pExpr).x.pList }) != std::ptr::null_mut::<ExprList>() {
        unsafe { (*unsafe { (*pExpr).x.pList }).nExpr }
    } else {
        0 as i32
    };
    pDef = unsafe {
        sqlite3FindFunction(
            db,
            (unsafe { (*pExpr).u.zToken }) as *const i8,
            n,
            unsafe { (*db).enc },
            ((0 as i32) as i8) as u8,
        )
    };
    if pDef == std::ptr::null_mut::<FuncDef>()
        || (unsafe { (*pDef).funcFlags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32)
    {
        unsafe {
            (*pWalker).eCode = ((1 as i32) as i16) as u16;
        }
        return 2 as i32;
    }
    return 0 as i32;
}

/// Return TRUE if expression pExpr is able to return a subtype.
///
/// A TRUE return does not guarantee that a subtype will be returned.
/// It only indicates that a subtype return is possible.  False positives
/// are acceptable as they only disable an optimization.  False negatives,
/// on the other hand, can lead to incorrect answers.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCanReturnSubtype(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
) -> i32 {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    unsafe { memset(std::ptr::addr_of_mut!(w) as *mut (), 0 as i32, 48 as u64) };
    w.pParse = pParse;
    w.xExprCallback = Some(exprNodeCanReturnSubtype);
    unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(w), pExpr) };
    return (w.eCode as u32) as i32;
}

/// Check to see if pExpr is one of the indexed expressions on pParse->pIdxEpr.
/// If it is, then resolve the expression by reading from the index and
/// return the register into which the value has been read.  If pExpr is
/// not an indexed expression, then return negative.
///
/// # Arguments
///
/// * `pParse` - The parsing context
/// * `pExpr` - The expression to potentially bypass
/// * `target` - Where to store the result of the expression
fn sqlite3IndexedExprLookup(mut pParse: *mut Parse, mut pExpr: *mut Expr, mut target: i32) -> i32 {
    let mut p: *mut IndexedExpr = unsafe { std::mem::zeroed() };
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    p = unsafe { (*pParse).pIdxEpr };
    '__slate_break_2320: while p != std::ptr::null_mut::<IndexedExpr>() {
        '__slate_continue_2320: {
            let mut exprAff: u8 = 0 as u8;
            let mut iDataCur: i32 = unsafe { (*p).iDataCur };
            if iDataCur < (0 as i32) {
            } else {
                if (unsafe { (*pParse).iSelfTab }) != (0 as i32) {
                    if (unsafe { (*p).iDataCur }) != (unsafe { (*pParse).iSelfTab }) - (1 as i32) {
                        break '__slate_continue_2320;
                    }
                    iDataCur = -(1 as i32);
                }
                if sqlite3ExprCompare(
                    std::ptr::null::<Parse>(),
                    pExpr as *const Expr,
                    (unsafe { (*p).pExpr }) as *const Expr,
                    iDataCur,
                ) != (0 as i32)
                {
                } else {
                    0 as i32;
                    exprAff = sqlite3ExprAffinity(pExpr as *const Expr) as u8;
                    if ((exprAff as u32) as i32) <= (65 as i32)
                        && (((unsafe { (*p).aff }) as u32) as i32) != (65 as i32)
                        || ((exprAff as u32) as i32) == (66 as i32)
                            && (((unsafe { (*p).aff }) as u32) as i32) != (66 as i32)
                        || ((exprAff as u32) as i32) >= (67 as i32)
                            && (((unsafe { (*p).aff }) as u32) as i32) != (67 as i32)
                    {
                        // Affinity mismatch on a generated column
                    } else {
                        // Functions that might set a subtype should not be replaced by the
                        // value taken from an expression index if they are themselves an
                        // argument to another scalar function or aggregate.
                        // https://sqlite.org/forum/forumpost/68d284c86b082c3e
                        let __v2702: bool;
                        if (unsafe { (*pExpr).flags }) & (2147483648 as u32) != ((0 as i32) as u32)
                        {
                            __v2702 = sqlite3ExprCanReturnSubtype(pParse, pExpr) != (0 as i32);
                        } else {
                            __v2702 = false as bool;
                        }
                        if __v2702 {
                        } else {
                            v = unsafe { (*pParse).pVdbe };
                            0 as i32;
                            if (unsafe { (*p).bMaybeNullRow }) != (0 as u8) {
                                // If the index is on a NULL row due to an outer join, then we
                                // cannot extract the value from the index.  The value must be
                                // computed using the original expression.
                                let mut addr: i32 = unsafe { sqlite3VdbeCurrentAddr(v) };
                                unsafe {
                                    sqlite3VdbeAddOp3(
                                        v,
                                        20 as i32,
                                        unsafe { (*p).iIdxCur },
                                        addr + (3 as i32),
                                        target,
                                    )
                                };
                                {}
                                unsafe {
                                    sqlite3VdbeAddOp3(
                                        v,
                                        96 as i32,
                                        unsafe { (*p).iIdxCur },
                                        unsafe { (*p).iIdxCol },
                                        target,
                                    )
                                };
                                unsafe {
                                    sqlite3VdbeComment(
                                        v,
                                        (b"%s expr-column %d\0".as_ptr() as *mut i8) as *const i8,
                                        unsafe { (*p).zIdxName },
                                        unsafe { (*p).iIdxCol },
                                    )
                                };
                                unsafe { sqlite3VdbeGoto(v, 0 as i32) };
                                p = unsafe { (*pParse).pIdxEpr };
                                unsafe {
                                    (*pParse).pIdxEpr = std::ptr::null_mut::<IndexedExpr>();
                                }
                                sqlite3ExprCode(pParse, pExpr, target);
                                unsafe {
                                    (*pParse).pIdxEpr = p;
                                }
                                unsafe { sqlite3VdbeJumpHere(v, addr + (2 as i32)) };
                            } else {
                                unsafe {
                                    sqlite3VdbeAddOp3(
                                        v,
                                        96 as i32,
                                        unsafe { (*p).iIdxCur },
                                        unsafe { (*p).iIdxCol },
                                        target,
                                    )
                                };
                                unsafe {
                                    sqlite3VdbeComment(
                                        v,
                                        (b"%s expr-column %d\0".as_ptr() as *mut i8) as *const i8,
                                        unsafe { (*p).zIdxName },
                                        unsafe { (*p).iIdxCol },
                                    )
                                };
                            }
                            return target;
                        }
                    }
                }
            }
        }
        p = unsafe { (*p).pIENext };
    }
    return -(1 as i32); // Not found
}

/// Expression pExpr is guaranteed to be a TK_COLUMN or equivalent. This
/// function checks the Parse.pIdxPartExpr list to see if this column
/// can be replaced with a constant value. If so, it generates code to
/// put the constant value in a register (ideally, but not necessarily,
/// register iTarget) and returns the register number.
///
/// Or, if the TK_COLUMN cannot be replaced by a constant, zero is
/// returned.
fn exprPartidxExprLookup(mut pParse: *mut Parse, mut pExpr: *mut Expr, mut iTarget: i32) -> i32 {
    let mut p: *mut IndexedExpr = unsafe { std::mem::zeroed() };
    p = unsafe { (*pParse).pIdxPartExpr };
    '__slate_break_2323: while p != std::ptr::null_mut::<IndexedExpr>() {
        if ((unsafe { (*pExpr).iColumn }) as i32) == unsafe { (*p).iIdxCol }
            && (unsafe { (*pExpr).iTable }) == unsafe { (*p).iDataCur }
        {
            let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
            let mut addr: i32 = 0 as i32;
            let mut ret: i32 = 0 as i32;
            if (unsafe { (*p).bMaybeNullRow }) != (0 as u8) {
                addr = unsafe { sqlite3VdbeAddOp1(v, 20 as i32, unsafe { (*p).iIdxCur }) };
            }
            ret = sqlite3ExprCodeTarget(pParse, unsafe { (*p).pExpr }, iTarget);
            unsafe {
                sqlite3VdbeAddOp4(
                    unsafe { (*pParse).pVdbe },
                    98 as i32,
                    ret,
                    1 as i32,
                    0 as i32,
                    (unsafe { std::ptr::addr_of_mut!((*p).aff) }) as *const i8,
                    1 as i32,
                )
            };
            if addr != (0 as i32) {
                unsafe { sqlite3VdbeJumpHere(v, addr) };
                unsafe { sqlite3VdbeChangeP3(v, addr, ret) };
            }
            return ret;
        }
        p = unsafe { (*p).pIENext };
    }
    return 0 as i32;
}

/// Generate code that evaluates an AND or OR operator leaving a
/// boolean result in a register.  pExpr is the AND/OR expression.
/// Store the result in the "target" register.  Use short-circuit
/// evaluation to avoid computing both operands, if possible.
///
/// The code generated might require the use of a temporary register.
/// If it does, then write the number of that temporary register
/// into *pTmpReg.  If not, leave *pTmpReg unchanged.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pExpr` - AND or OR expression to be coded
/// * `target` - Put result in this register, guaranteed
/// * `pTmpReg` - Write a temporary register here
fn exprCodeTargetAndOr(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut target: i32,
    mut pTmpReg: *mut i32,
) -> i32 {
    let mut op: i32 = 0 as i32; // The opcode.  TK_AND or TK_OR
    let mut skipOp: i32 = 0 as i32; // Opcode for the branch that skips one operand
    let mut addrSkip: i32 = 0 as i32; // Branch instruction that skips one of the operands
    let mut regSS: i32 = 0 as i32; // Register holding computed operand when other omitted
    let mut r1: i32 = 0 as i32;
    let mut r2: i32 = 0 as i32; // Registers for left and right operands, respectively
    let mut pAlt: *mut Expr = unsafe { std::mem::zeroed() }; // Alternative, simplified expression
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() }; // statement being coded
    0 as i32;
    op = ((unsafe { (*pExpr).op }) as u32) as i32;
    0 as i32;
    0 as i32;
    {}
    0 as i32;
    {}
    0 as i32;
    v = unsafe { (*pParse).pVdbe };
    pAlt = sqlite3ExprSimplifiedAndOr(pExpr);
    if pAlt != pExpr {
        r1 = sqlite3ExprCodeTarget(pParse, pAlt, target);
        unsafe { sqlite3VdbeAddOp3(v, 44 as i32, r1, r1, target) };
        return target;
    }
    skipOp = if op == (44 as i32) {
        17 as i32
    } else {
        16 as i32
    };
    if exprEvalRhsFirst(pExpr) != (0 as i32) {
        // Compute the right operand first.  Skip the computation of the left
        // operand if the right operand fully determines the result
        let __v2703: i32 = sqlite3ExprCodeTarget(pParse, unsafe { (*pExpr).pRight }, target);
        regSS = __v2703;
        r2 = __v2703;
        addrSkip = unsafe { sqlite3VdbeAddOp1(v, skipOp, r2) };
        unsafe { sqlite3VdbeComment(v, (b"skip left operand\0".as_ptr() as *mut i8) as *const i8) };
        {}
        r1 = sqlite3ExprCodeTemp(pParse, unsafe { (*pExpr).pLeft }, pTmpReg);
    } else {
        // Compute the left operand first
        r1 = sqlite3ExprCodeTarget(pParse, unsafe { (*pExpr).pLeft }, target);
        if (unsafe { (*unsafe { (*pExpr).pRight }).flags }) & ((4194304 as i32) as u32)
            != ((0 as i32) as u32)
        {
            // Skip over the computation of the right operand if the right
            // operand is a subquery and the left operand completely determines
            // the result
            regSS = r1;
            addrSkip = unsafe { sqlite3VdbeAddOp1(v, skipOp, r1) };
            unsafe {
                sqlite3VdbeComment(
                    v,
                    (b"skip right operand\0".as_ptr() as *mut i8) as *const i8,
                )
            };
            {}
        } else {
            regSS = 0 as i32;
            addrSkip = 0 as i32;
        }
        r2 = sqlite3ExprCodeTemp(pParse, unsafe { (*pExpr).pRight }, pTmpReg);
    }
    unsafe { sqlite3VdbeAddOp3(v, op, r2, r1, target) };
    {}
    if addrSkip != (0 as i32) {
        unsafe {
            sqlite3VdbeAddOp2(
                v,
                9 as i32,
                0 as i32,
                (unsafe { sqlite3VdbeCurrentAddr(v) }) + (2 as i32),
            )
        };
        unsafe { sqlite3VdbeJumpHere(v, addrSkip) };
        unsafe { sqlite3VdbeAddOp3(v, 43 as i32, regSS, regSS, target) };
        unsafe {
            sqlite3VdbeComment(
                v,
                (b"short-circut value\0".as_ptr() as *mut i8) as *const i8,
            )
        };
    }
    return target;
}

/// Generate code into the current Vdbe to evaluate the given
/// expression.  Attempt to store the results in register "target".
/// Return the register where results are stored.
///
/// With this routine, there is no guarantee that results will
/// be stored in target.  The result might be stored in some other
/// register if it is convenient to do so.  The calling function
/// must check the return code and move the results to the desired
/// register.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCodeTarget(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut target: i32,
) -> i32 {
    let mut __slate_storage_2474: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2474: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_2474) as *mut *mut Expr;
    let mut __slate_storage_1376: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1376: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1376) as *mut *mut sqlite3;
    let mut __slate_storage_1375: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1375: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1375) as *mut *mut Expr; // X==Ei (form A) or just Ei (form B)
    let mut __slate_storage_1374: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1374: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1374) as *mut *mut Expr; // The X expression
    let mut __slate_storage_1373: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1373: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1373) as *mut *mut Expr; // The X==Ei expression
    let mut __slate_storage_1372: std::mem::MaybeUninit<Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1372: *mut Expr = std::ptr::addr_of_mut!(__slate_storage_1372) as *mut Expr; // Array of WHEN terms
    let mut __slate_storage_1371: std::mem::MaybeUninit<*mut ExprList_item> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1371: *mut *mut ExprList_item =
        std::ptr::addr_of_mut!(__slate_storage_1371) as *mut *mut ExprList_item; // List of WHEN terms
    let mut __slate_storage_1370: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1370: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_1370) as *mut *mut ExprList; // Loop counter
    let mut __slate_storage_1369: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1369: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1369) as *mut i32; // 2x number of WHEN terms
    let mut __slate_storage_1368: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1368: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1368) as *mut i32; // GOTO label for next WHEN clause
    let mut __slate_storage_1367: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1367: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1367) as *mut i32; // GOTO label for end of CASE stmt
    let mut __slate_storage_1366: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1366: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1366) as *mut i32;
    let mut __slate_storage_1365: std::mem::MaybeUninit<*mut AggInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1365: *mut *mut AggInfo =
        std::ptr::addr_of_mut!(__slate_storage_1365) as *mut *mut AggInfo;
    let mut __slate_storage_1364: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1364: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1364) as *mut u8;
    let mut __slate_storage_1363: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1363: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1363) as *mut i32;
    let mut __slate_storage_1362: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1362: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1362) as *mut i32;
    let mut __slate_storage_1361: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1361: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1361) as *mut i32;
    // If the opcode is TK_TRIGGER, then the expression is a reference
    // to a column in the new.* or old.* pseudo-tables available to
    // trigger programs. In this case Expr.iTable is set to 1 for the
    // new.* pseudo-table, or 0 for the old.* pseudo-table. Expr.iColumn
    // is set to the column of the pseudo-table to read, or to -1 to
    // read the rowid field.
    //
    // The expression is implemented using an OP_Param opcode. The p1
    // parameter is set to 0 for an old.rowid reference, or to (i+1)
    // to reference another column of the old.* pseudo-table, where
    // i is the index of the column. For a new.rowid reference, p1 is
    // set to (n+1), where n is the number of columns in each pseudo-table.
    // For a reference to any other column in the new.* pseudo-table, p1
    // is set to (n+2+i), where n and i are as defined previously. For
    // example, if the table on which triggers are being fired is
    // declared as:
    //
    //   CREATE TABLE t1(a, b);
    //
    // Then p1 is interpreted as follows:
    //
    //   p1==0   ->    old.rowid     p1==3   ->    new.rowid
    //   p1==1   ->    old.a         p1==4   ->    new.a
    //   p1==2   ->    old.b         p1==5   ->    new.b
    let mut __slate_storage_1360: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1360: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_1360) as *mut *mut Table;
    let mut __slate_storage_1359: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1359: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1359) as *mut i32;
    let mut __slate_storage_1358: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1358: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1358) as *mut i32;
    let mut __slate_storage_1357: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1357: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1357) as *mut *mut Expr;
    let mut __slate_storage_1356: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1356: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1356) as *mut i32;
    let mut __slate_storage_2473: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2473: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2473) as *mut i32;
    let mut __slate_storage_2472: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2472: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2472) as *mut bool;
    let mut __slate_storage_1355: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1355: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1355) as *mut i32;
    let mut __slate_storage_1354: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1354: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1354) as *mut u8;
    let mut __slate_storage_2471: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2471: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2471) as *mut i32;
    let mut __slate_storage_2470: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2470: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2470) as *mut i32;
    let mut __slate_storage_2469: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2469: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2469) as *mut *mut Parse;
    let mut __slate_storage_2465: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2465: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2465) as *mut i32;
    let mut __slate_storage_2464: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2464: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2464) as *mut i32;
    let mut __slate_storage_2468: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2468: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2468) as *mut u32;
    let mut __slate_storage_2467: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2467: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2467) as *mut u32;
    let mut __slate_storage_2466: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2466: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2466) as *mut bool;
    let mut __slate_storage_2463: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2463: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2463) as *mut bool; // A collating sequence
    let mut __slate_storage_1353: std::mem::MaybeUninit<*mut CollSeq> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1353: *mut *mut CollSeq =
        std::ptr::addr_of_mut!(__slate_storage_1353) as *mut *mut CollSeq; // The text encoding used by this database
    let mut __slate_storage_1352: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1352: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1352) as *mut u8; // The database connection
    let mut __slate_storage_1351: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1351: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1351) as *mut *mut sqlite3; // Loop counter
    let mut __slate_storage_1350: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1350: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1350) as *mut i32; // Mask of function arguments that are constant
    let mut __slate_storage_1349: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1349: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1349) as *mut u32; // The function name
    let mut __slate_storage_1348: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1348: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1348) as *mut *const i8; // The function definition object
    let mut __slate_storage_1347: std::mem::MaybeUninit<*mut FuncDef> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1347: *mut *mut FuncDef =
        std::ptr::addr_of_mut!(__slate_storage_1347) as *mut *mut FuncDef; // Number of function arguments
    let mut __slate_storage_1346: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1346: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1346) as *mut i32; // List of function arguments
    let mut __slate_storage_1345: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1345: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_1345) as *mut *mut ExprList;
    let mut __slate_storage_1344: std::mem::MaybeUninit<*mut AggInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1344: *mut *mut AggInfo =
        std::ptr::addr_of_mut!(__slate_storage_1344) as *mut *mut AggInfo;
    let mut __slate_storage_1343: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1343: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1343) as *mut i32; // IS TRUE or IS FALSE
    let mut __slate_storage_1342: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1342: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1342) as *mut i32; // IS TRUE or IS NOT TRUE
    let mut __slate_storage_1341: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1341: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1341) as *mut i32;
    let mut __slate_storage_1340: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1340: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1340) as *mut *mut Expr;
    let mut __slate_storage_1339: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1339: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1339) as *mut i32;
    let mut __slate_storage_1338: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1338: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1338) as *mut i32;
    let mut __slate_storage_1337: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1337: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1337) as *mut *mut Expr;
    let mut __slate_storage_1336: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1336: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1336) as *mut *mut i8;
    let mut __slate_storage_1335: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1335: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1335) as *mut *const i8;
    let mut __slate_storage_1334: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1334: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1334) as *mut i32;
    let mut __slate_storage_2460: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2460: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_2460) as *mut u16;
    let mut __slate_storage_2459: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2459: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_2459) as *mut u16;
    let mut __slate_storage_2458: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2458: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_2458) as *mut *mut Column;
    let mut __slate_storage_2457: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2457: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_2457) as *mut u16;
    let mut __slate_storage_2456: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2456: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_2456) as *mut u16;
    let mut __slate_storage_2455: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2455: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_2455) as *mut *mut Column;
    let mut __slate_storage_1333: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1333: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1333) as *mut i32;
    let mut __slate_storage_1332: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1332: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1332) as *mut i32;
    let mut __slate_storage_1331: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1331: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_1331) as *mut *mut Table;
    // Other columns in the same row for CHECK constraints or
    // generated columns or for inserting into partial index.
    // The row is unpacked into registers beginning at
    // 0-(pParse->iSelfTab).  The rowid (if any) is in a register
    // immediately prior to the first column.
    let mut __slate_storage_1330: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1330: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_1330) as *mut *mut Column;
    let mut __slate_storage_2462: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2462: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2462) as *mut i32;
    let mut __slate_storage_2461: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2461: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2461) as *mut bool;
    // This COLUMN expression is really a constant due to WHERE clause
    // constraints, and that constant is coded by the pExpr->pLeft
    // expression.  However, make sure the constant has the correct
    // datatype by applying the Affinity of the table column to the
    // constant.
    let mut __slate_storage_1328: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1328: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1328) as *mut i32;
    let mut __slate_storage_1327: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1327: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1327) as *mut i32;
    let mut __slate_storage_1326: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1326: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1326) as *mut i32;
    let mut __slate_storage_1325: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1325: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_1325) as *mut *mut Table;
    let mut __slate_storage_1324: std::mem::MaybeUninit<*mut AggInfo_col> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1324: *mut *mut AggInfo_col =
        std::ptr::addr_of_mut!(__slate_storage_1324) as *mut *mut AggInfo_col;
    let mut __slate_storage_1323: std::mem::MaybeUninit<*mut AggInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1323: *mut *mut AggInfo =
        std::ptr::addr_of_mut!(__slate_storage_1323) as *mut *mut AggInfo;
    let mut __slate_storage_2454: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2454: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2454) as *mut i32;
    let mut __slate_storage_2453: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2453: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2453) as *mut bool;
    let mut __slate_storage_1322: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1322: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1322) as *mut i32; // Temporary expression node
    let mut __slate_storage_1321: std::mem::MaybeUninit<Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1321: *mut Expr = std::ptr::addr_of_mut!(__slate_storage_1321) as *mut Expr; // Various register numbers
    let mut __slate_storage_1320: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1320: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1320) as *mut i32;
    let mut __slate_storage_1319: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1319: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1319) as *mut i32; // If non-zero free this temporary register
    let mut __slate_storage_1318: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1318: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1318) as *mut i32; // If non-zero free this temporary register
    let mut __slate_storage_1317: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1317: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1317) as *mut i32; // Results stored in register inReg
    let mut __slate_storage_1316: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1316: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1316) as *mut i32; // The opcode being coded
    let mut __slate_storage_1315: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1315: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1315) as *mut i32; // The VM under construction
    let mut __slate_storage_1314: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1314: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1314) as *mut *mut Vdbe;
    unsafe {
        std::ptr::write(__slate_slot_1314, unsafe { (*pParse).pVdbe });
        std::ptr::write(__slate_slot_1316, target);
        std::ptr::write(__slate_slot_1317, 0 as i32);
        std::ptr::write(__slate_slot_1318, 0 as i32);
        std::ptr::write(__slate_slot_1322, 0 as i32);
        0 as i32;
        0 as i32;
        '__join_0: {
            '__join_158: {
                '__join_173: {
                    '__join_123: {
                        '__join_124: {
                            '__join_113: {
                                '__join_112: {
                                    '__join_107: {
                                        '__join_102: {
                                            '__join_101: {
                                                '__join_100: {
                                                    '__join_99: {
                                                        '__join_96: {
                                                            '__join_49: {
                                                                '__join_41: {
                                                                    '__join_30: {
                                                                        '__join_28: {
                                                                            '__join_27: {
                                                                                '__join_21: {
                                                                                    '__join_7: {
                                                                                        '__loop_182: loop {
                                                                                            if pExpr == std::ptr::null_mut::<Expr>() {
*__slate_slot_1315 = 122 as i32;
} else {
if (unsafe { (*pParse).pIdxEpr }) != std::ptr::null_mut::<IndexedExpr>() && !((unsafe { (*pExpr).flags }) & ((8388608 as i32) as u32) != ((0 as i32) as u32)) {
std::ptr::write(__slate_slot_2454, sqlite3IndexedExprLookup(pParse, pExpr, target));
*__slate_slot_1319 = *__slate_slot_2454;
*__slate_slot_2453 = *__slate_slot_2454 >= (0 as i32);
} else {
*__slate_slot_2453 = false as bool;
}
if *__slate_slot_2453 {
break '__loop_182;
} else {
0 as i32;
*__slate_slot_1315 = ((unsafe { (*pExpr).op }) as u32) as i32;
}
}
                                                                                            0 as i32;
                                                                                            let __t0: i32 = *__slate_slot_1315;
                                                                                            if __t0 == (170 as i32) {
break '__join_173;
} else {
if __t0 == (168 as i32) {
break '__join_158;
} else {
if __t0 == (156 as i32) {
codeInteger(pParse, pExpr, 0 as i32, target);
return target;
} else {
if __t0 == (171 as i32) {
unsafe { sqlite3VdbeAddOp2(*__slate_slot_1314, 73 as i32, sqlite3ExprTruthValue(pExpr as *const Expr), target) };
return target;
} else {
if __t0 == (154 as i32) {
0 as i32;
codeReal(*__slate_slot_1314, (unsafe { (*pExpr).u.zToken }) as *const i8, 0 as i32, target);
return target;
} else {
if __t0 == (118 as i32) {
0 as i32;
unsafe { sqlite3VdbeLoadString(*__slate_slot_1314, target, (unsafe { (*pExpr).u.zToken }) as *const i8) };
return target;
} else {
if __t0 == (83 as i32) {
// Set a range of registers to NULL.  pExpr->y.nReg registers starting
// with target
unsafe { sqlite3VdbeAddOp3(*__slate_slot_1314, 77 as i32, 0 as i32, target, target + unsafe { (*pExpr).y.nReg } - (1 as i32)) };
return target;
} else {
if __t0 == (155 as i32) {
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_1335 = (unsafe { unsafe { (*pExpr).u.zToken }.offset((2 as i32) as isize) }) as *const i8;
*__slate_slot_1334 = (unsafe { sqlite3Strlen30(*__slate_slot_1335) }) - (1 as i32);
0 as i32;
*__slate_slot_1336 = (unsafe { sqlite3HexToBlob(unsafe { sqlite3VdbeDb(*__slate_slot_1314) }, *__slate_slot_1335, *__slate_slot_1334) }) as *mut i8;
unsafe { sqlite3VdbeAddOp4(*__slate_slot_1314, 79 as i32, *__slate_slot_1334 / (2 as i32), target, 0 as i32, *__slate_slot_1336 as *const i8, -(7 as i32)) };
return target;
} else {
if __t0 == (157 as i32) {
0 as i32;
0 as i32;
0 as i32;
unsafe { sqlite3VdbeAddOp2(*__slate_slot_1314, 80 as i32, (unsafe { (*pExpr).iColumn }) as i32, target) };
return target;
} else {
if __t0 == (176 as i32) {
return unsafe { (*pExpr).iTable };
} else {
if __t0 == (36 as i32) {
// Expressions of the form:   CAST(pLeft AS token)
sqlite3ExprCode(pParse, unsafe { (*pExpr).pLeft }, target);
0 as i32;
0 as i32;
unsafe { sqlite3VdbeAddOp2(*__slate_slot_1314, 90 as i32, target, (unsafe { sqlite3AffinityType((unsafe { (*pExpr).u.zToken }) as *const i8, std::ptr::null_mut::<Column>()) }) as i32) };
return *__slate_slot_1316;
} else {
if __t0 == (45 as i32) {
break '__join_124;
} else {
if __t0 == (46 as i32) {
break '__join_124;
} else {
if __t0 == (57 as i32) {
break '__join_123;
} else {
if __t0 == (56 as i32) {
break '__join_123;
} else {
if __t0 == (55 as i32) {
break '__join_123;
} else {
if __t0 == (58 as i32) {
break '__join_123;
} else {
if __t0 == (53 as i32) {
break '__join_123;
} else {
if __t0 == (54 as i32) {
break '__join_123;
} else {
if __t0 == (44 as i32) {
break '__join_113;
} else {
if __t0 == (43 as i32) {
break '__join_113;
} else {
if __t0 == (107 as i32) {
break '__join_112;
} else {
if __t0 == (109 as i32) {
break '__join_112;
} else {
if __t0 == (108 as i32) {
break '__join_112;
} else {
if __t0 == (111 as i32) {
break '__join_112;
} else {
if __t0 == (103 as i32) {
break '__join_112;
} else {
if __t0 == (104 as i32) {
break '__join_112;
} else {
if __t0 == (110 as i32) {
break '__join_112;
} else {
if __t0 == (105 as i32) {
break '__join_112;
} else {
if __t0 == (106 as i32) {
break '__join_112;
} else {
if __t0 == (112 as i32) {
break '__join_112;
} else {
if __t0 == (174 as i32) {
break '__join_107;
} else {
if __t0 == (115 as i32) {
break '__join_102;
} else {
if __t0 == (19 as i32) {
break '__join_102;
} else {
if __t0 == (175 as i32) {
break '__join_101;
} else {
if __t0 == (51 as i32) {
break '__join_100;
} else {
if __t0 == (52 as i32) {
break '__join_100;
} else {
if __t0 == (169 as i32) {
break '__join_99;
} else {
if __t0 == (172 as i32) {
break '__join_96;
} else {
if __t0 == (20 as i32) {
break '__join_49;
} else {
if __t0 == (139 as i32) {
break '__join_49;
} else {
if __t0 == (178 as i32) {
break '__join_41;
} else {
if __t0 == (50 as i32) {
std::ptr::write(__slate_slot_1358, unsafe { sqlite3VdbeMakeLabel(pParse) });
std::ptr::write(__slate_slot_1359, unsafe { sqlite3VdbeMakeLabel(pParse) });
unsafe { sqlite3VdbeAddOp2(*__slate_slot_1314, 77 as i32, 0 as i32, target) };
sqlite3ExprCodeIN(pParse, pExpr, *__slate_slot_1358, *__slate_slot_1359);
unsafe { sqlite3VdbeAddOp2(*__slate_slot_1314, 73 as i32, 1 as i32, target) };
unsafe { sqlite3VdbeResolveLabel(*__slate_slot_1314, *__slate_slot_1358) };
unsafe { sqlite3VdbeAddOp2(*__slate_slot_1314, 88 as i32, target, 0 as i32) };
unsafe { sqlite3VdbeResolveLabel(*__slate_slot_1314, *__slate_slot_1359) };
return target;
} else {
if __t0 == (49 as i32) {
exprCodeBetween(pParse, pExpr, target, None, 0 as i32);
return target;
} else {
if __t0 == (114 as i32) {
if !((unsafe { (*pExpr).flags }) & ((512 as i32) as u32) != ((0 as i32) as u32)) {
// A TK_COLLATE Expr node without the EP_Collate tag is a so-called
// "SOFT-COLLATE" that is added to constraints that are pushed down
// from outer queries into sub-queries by the WHERE-clause push-down
// optimization. Clear subtypes as subtypes may not cross a subquery
// boundary.
0 as i32;
sqlite3ExprCode(pParse, unsafe { (*pExpr).pLeft }, target);
unsafe { sqlite3VdbeAddOp1(*__slate_slot_1314, 182 as i32, target) };
return target;
} else {
pExpr = unsafe { (*pExpr).pLeft };
continue '__loop_182;
}
} else {
if __t0 == (181 as i32) {
} else {
if __t0 == (173 as i32) {
} else {
if __t0 == (78 as i32) {
break '__join_30;
} else {
if __t0 == (177 as i32) {
break '__join_28;
} else {
if __t0 == (179 as i32) {
break '__join_27;
} else {
if __t0 == (158 as i32) {
break '__join_21;
} else {
if __t0 == (72 as i32) {
break '__join_7;
} else {
// Make NULL the default case so that if a bug causes an illegal
// Expr node to be passed into this function, it will be handled
// sanely and not crash.  But keep the assert() to bring the problem
// to the attention of the developers.
0 as i32;
unsafe { sqlite3VdbeAddOp2(*__slate_slot_1314, 77 as i32, 0 as i32, target) };
return target;
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
                                                                                            pExpr = unsafe {
                                                                                                (*pExpr).pLeft
                                                                                            };
                                                                                        }
                                                                                        return *__slate_slot_1319;
                                                                                    }
                                                                                    0 as i32;
                                                                                    if !((unsafe { (*pParse).pTriggerTab }) != std::ptr::null_mut::<Table>()) && !((unsafe { (*pParse).nested }) != (0 as u8)) {
unsafe { sqlite3ErrorMsg(pParse, (b"RAISE() may only be used within a trigger-program\0".as_ptr() as *mut i8) as *const i8) };
return 0 as i32;
} else {
if ((unsafe { (*pExpr).affExpr }) as i32) == (2 as i32) {
unsafe { sqlite3MayAbort(pParse) };
}
0 as i32;
if ((unsafe { (*pExpr).affExpr }) as i32) == (4 as i32) {
unsafe { sqlite3VdbeAddOp2(*__slate_slot_1314, 72 as i32, 0 as i32, 4 as i32) };
{
}
break '__join_0;
} else {
*__slate_slot_1319 = sqlite3ExprCodeTemp(pParse, unsafe { (*pExpr).pLeft }, std::ptr::addr_of_mut!(*__slate_slot_1317));
unsafe { sqlite3VdbeAddOp3(*__slate_slot_1314, 72 as i32, if (unsafe { (*pParse).pTriggerTab }) != std::ptr::null_mut::<Table>() { (19 as i32) | (7 as i32) << (8 as i32) } else { 1 as i32 }, (unsafe { (*pExpr).affExpr }) as i32, *__slate_slot_1319) };
break '__join_0;
}
}
                                                                                }
                                                                                std::ptr::write(__slate_slot_1374, std::ptr::null_mut::<Expr>());
                                                                                std::ptr::write(__slate_slot_1375, std::ptr::null_mut::<Expr>());
                                                                                std::ptr::write(__slate_slot_1376, unsafe { (*pParse).db });
                                                                                0 as i32;
                                                                                0 as i32;
                                                                                *__slate_slot_1370 = unsafe { (*pExpr).x.pList };
                                                                                *__slate_slot_1371 = unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1370)).a) as *mut ExprList_item };
                                                                                *__slate_slot_1368 = unsafe { (*(*__slate_slot_1370)).nExpr };
                                                                                *__slate_slot_1366 = unsafe { sqlite3VdbeMakeLabel(pParse) };
                                                                                std::ptr::write(__slate_slot_2474, unsafe { (*pExpr).pLeft });
                                                                                *__slate_slot_1373 = *__slate_slot_2474;
                                                                                if *__slate_slot_2474 != std::ptr::null_mut::<Expr>() {
*__slate_slot_1375 = sqlite3ExprDup(*__slate_slot_1376, *__slate_slot_1373 as *const Expr, 0 as i32);
if (unsafe { (*(*__slate_slot_1376)).mallocFailed }) != (0 as u8) {
sqlite3ExprDelete(*__slate_slot_1376, *__slate_slot_1375);
break '__join_0;
} else {
{
}
sqlite3ExprToRegister(*__slate_slot_1375, exprCodeVector(pParse, *__slate_slot_1375, std::ptr::addr_of_mut!(*__slate_slot_1317)));
{
}
unsafe { memset(std::ptr::addr_of_mut!(*__slate_slot_1372) as *mut (), 0 as i32, 72 as u64) };
(*__slate_slot_1372).op = ((54 as i32) as i8) as u8;
(*__slate_slot_1372).pLeft = *__slate_slot_1375;
*__slate_slot_1374 = std::ptr::addr_of_mut!(*__slate_slot_1372);
// Ticket b351d95f9cd5ef17e9d9dbae18f5ca8611190001:
// The value in regFree1 might get SCopy-ed into the file result.
// So make sure that the regFree1 register is not reused for other
// purposes and possibly overwritten.
*__slate_slot_1317 = 0 as i32;
}
}
                                                                                *__slate_slot_1369 = 0 as i32;
                                                                                loop {
                                                                                    if *__slate_slot_1369 < *__slate_slot_1368 - (1 as i32) {
if *__slate_slot_1373 != std::ptr::null_mut::<Expr>() {
0 as i32;
(*__slate_slot_1372).pRight = unsafe { (*unsafe { (*__slate_slot_1371).offset(*__slate_slot_1369 as isize) }).pExpr };
} else {
*__slate_slot_1374 = unsafe { (*unsafe { (*__slate_slot_1371).offset(*__slate_slot_1369 as isize) }).pExpr };
}
*__slate_slot_1367 = unsafe { sqlite3VdbeMakeLabel(pParse) };
{
}
sqlite3ExprIfFalse(pParse, *__slate_slot_1374, *__slate_slot_1367, 16 as i32);
{
}
sqlite3ExprCode(pParse, unsafe { (*unsafe { (*__slate_slot_1371).offset((*__slate_slot_1369 + (1 as i32)) as isize) }).pExpr }, target);
unsafe { sqlite3VdbeGoto(*__slate_slot_1314, *__slate_slot_1366) };
unsafe { sqlite3VdbeResolveLabel(*__slate_slot_1314, *__slate_slot_1367) };
*__slate_slot_1369 = *__slate_slot_1369 + (2 as i32);
} else {
break;
}
                                                                                }
                                                                                if *__slate_slot_1368 & (1 as i32) != (0 as i32) {
sqlite3ExprCode(pParse, unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1370)).a) as *mut ExprList_item }.offset((*__slate_slot_1368 - (1 as i32)) as isize) }).pExpr }, target);
} else {
unsafe { sqlite3VdbeAddOp2(*__slate_slot_1314, 77 as i32, 0 as i32, target) };
}
                                                                                sqlite3ExprDelete(*__slate_slot_1376, *__slate_slot_1375);
                                                                                setDoNotMergeFlagOnCopy(*__slate_slot_1314);
                                                                                unsafe {
                                                                                    sqlite3VdbeResolveLabel(*__slate_slot_1314, *__slate_slot_1366)
                                                                                };
                                                                                break '__join_0;
                                                                            }
                                                                            std::ptr::write(
                                                                                __slate_slot_1364,
                                                                                (((unsafe {
                                                                                    (*pParse).__slate_bits_0.__get_okConstFactor()
                                                                                })
                                                                                    as i32)
                                                                                    as i8)
                                                                                    as u8,
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_1365,
                                                                                unsafe {
                                                                                    (*pExpr)
                                                                                        .pAggInfo
                                                                                },
                                                                            );
                                                                            if *__slate_slot_1365 != std::ptr::null_mut::<AggInfo>() {
0 as i32;
if !((unsafe { (*(*__slate_slot_1365)).directMode }) != (0 as u8)) {
*__slate_slot_1316 = (unsafe { (*(*__slate_slot_1365)).iFirstReg }) + ((unsafe { (*pExpr).iAgg }) as i32);
break '__join_0;
} else {
if (unsafe { (*unsafe { (*pExpr).pAggInfo }).useSortingIdx }) != (0 as u8) {
unsafe { sqlite3VdbeAddOp3(*__slate_slot_1314, 96 as i32, unsafe { (*(*__slate_slot_1365)).sortingIdxPTab }, unsafe { (*unsafe { unsafe { (*(*__slate_slot_1365)).aCol }.offset(((unsafe { (*pExpr).iAgg }) as i32) as isize) }).iSorterColumn }, target) };
*__slate_slot_1316 = target;
break '__join_0;
}
}
}
                                                                            *__slate_slot_1363 = unsafe {
                                                                                sqlite3VdbeAddOp3(*__slate_slot_1314, 20 as i32, unsafe { (*pExpr).iTable }, 0 as i32, target)
                                                                            };
                                                                            // The OP_IfNullRow opcode above can overwrite the result register with
                                                                            // NULL.  So we have to ensure that the result register is not a value
                                                                            // that is suppose to be a constant.  Two defenses are needed:
                                                                            //   (1)  Temporarily disable factoring of constant expressions
                                                                            //   (2)  Make sure the computed value really is stored in register
                                                                            //        "target" and not someplace else.
                                                                            unsafe {
                                                                                (*pParse).__slate_bits_0.__set_okConstFactor((0 as i32) as u32);
                                                                            }
                                                                            // note (1) above
                                                                            sqlite3ExprCode(
                                                                                pParse,
                                                                                unsafe {
                                                                                    (*pExpr).pLeft
                                                                                },
                                                                                target,
                                                                            );
                                                                            0 as i32;
                                                                            unsafe {
                                                                                (*pParse).__slate_bits_0.__set_okConstFactor(*__slate_slot_1364 as u32);
                                                                            }
                                                                            unsafe {
                                                                                sqlite3VdbeJumpHere(*__slate_slot_1314, *__slate_slot_1363)
                                                                            };
                                                                            break '__join_0;
                                                                        }
                                                                        unsafe {
                                                                            sqlite3ErrorMsg(pParse, (b"row value misused\0".as_ptr() as *mut i8) as *const i8)
                                                                        };
                                                                        break '__join_0;
                                                                    }
                                                                    0 as i32;
                                                                    *__slate_slot_1360 =
                                                                        unsafe { (*pExpr).y.pTab };
                                                                    *__slate_slot_1361 = (unsafe {
                                                                        (*pExpr).iColumn
                                                                    })
                                                                        as i32;
                                                                    *__slate_slot_1362 = (unsafe {
                                                                        (*pExpr).iTable
                                                                    }) * (((unsafe {
                                                                        (*(*__slate_slot_1360)).nCol
                                                                    })
                                                                        as i32)
                                                                        + (1 as i32))
                                                                        + (1 as i32)
                                                                        + ((unsafe {
                                                                            sqlite3TableColumnToStorage(*__slate_slot_1360, *__slate_slot_1361 as i16)
                                                                        })
                                                                            as i32);
                                                                    0 as i32;
                                                                    0 as i32;
                                                                    0 as i32;
                                                                    0 as i32;
                                                                    unsafe {
                                                                        sqlite3VdbeAddOp2(
                                                                            *__slate_slot_1314,
                                                                            159 as i32,
                                                                            *__slate_slot_1362,
                                                                            target,
                                                                        )
                                                                    };
                                                                    unsafe {
                                                                        sqlite3VdbeComment(
                                                                            *__slate_slot_1314,
                                                                            (b"r[%d]=%s.%s\0"
                                                                                .as_ptr()
                                                                                as *mut i8)
                                                                                as *const i8,
                                                                            target,
                                                                            if (unsafe {
                                                                                (*pExpr).iTable
                                                                            }) != (0 as i32)
                                                                            {
                                                                                b"new\0".as_ptr()
                                                                                    as *mut i8
                                                                            } else {
                                                                                b"old\0".as_ptr()
                                                                                    as *mut i8
                                                                            },
                                                                            if ((unsafe {
                                                                                (*pExpr).iColumn
                                                                            })
                                                                                as i32)
                                                                                < (0 as i32)
                                                                            {
                                                                                b"rowid\0".as_ptr()
                                                                                    as *mut i8
                                                                            } else {
                                                                                unsafe {
                                                                                    (*unsafe { unsafe { (*unsafe { (*pExpr).y.pTab }).aCol }.offset(*__slate_slot_1361 as isize) }).zCnName
                                                                                }
                                                                            },
                                                                        )
                                                                    };
                                                                    // If the column has REAL affinity, it may currently be stored as an
                                                                    // integer. Use OP_RealAffinity to make sure it is really real.
                                                                    //
                                                                    // EVIDENCE-OF: R-60985-57662 SQLite will convert the value back to
                                                                    // floating point when extracting it from the record.
                                                                    if *__slate_slot_1361
                                                                        >= (0 as i32)
                                                                        && ((unsafe {
                                                                            (*unsafe { unsafe { (*(*__slate_slot_1360)).aCol }.offset(*__slate_slot_1361 as isize) }).affinity
                                                                        })
                                                                            as i32)
                                                                            == (69 as i32)
                                                                    {
                                                                        unsafe {
                                                                            sqlite3VdbeAddOp1(
                                                                                *__slate_slot_1314,
                                                                                89 as i32,
                                                                                target,
                                                                            )
                                                                        };
                                                                        break '__join_0;
                                                                    } else {
                                                                        break '__join_0;
                                                                    }
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_1357,
                                                                    unsafe { (*pExpr).pLeft },
                                                                );
                                                                if (unsafe {
                                                                    (*(*__slate_slot_1357)).iTable
                                                                }) == (0 as i32)
                                                                    || (((unsafe {
                                                                        (*pParse).withinRJSubrtn
                                                                    })
                                                                        as u32)
                                                                        as i32)
                                                                        > (((unsafe {
                                                                            (*(*__slate_slot_1357))
                                                                                .op2
                                                                        })
                                                                            as u32)
                                                                            as i32)
                                                                {
                                                                    unsafe {
                                                                        (*(*__slate_slot_1357))
                                                                            .iTable =
                                                                            sqlite3CodeSubselect(
                                                                                pParse,
                                                                                *__slate_slot_1357,
                                                                            );
                                                                    }
                                                                    unsafe {
                                                                        (*(*__slate_slot_1357))
                                                                            .op2 = unsafe {
                                                                            (*pParse).withinRJSubrtn
                                                                        };
                                                                    }
                                                                }
                                                                0 as i32;
                                                                *__slate_slot_1356 =
                                                                    sqlite3ExprVectorSize(
                                                                        *__slate_slot_1357
                                                                            as *const Expr,
                                                                    );
                                                                if (unsafe { (*pExpr).iTable })
                                                                    != *__slate_slot_1356
                                                                {
                                                                    unsafe {
                                                                        sqlite3ErrorMsg(pParse, (b"%d columns assigned %d values\0".as_ptr() as *mut i8) as *const i8, unsafe { (*pExpr).iTable }, *__slate_slot_1356)
                                                                    };
                                                                }
                                                                return (unsafe {
                                                                    (*(*__slate_slot_1357)).iTable
                                                                }) + ((unsafe {
                                                                    (*pExpr).iColumn
                                                                })
                                                                    as i32);
                                                            }
                                                            {}
                                                            {}
                                                            if (unsafe {
                                                                (*unsafe { (*pParse).db })
                                                                    .mallocFailed
                                                            }) != (0 as u8)
                                                            {
                                                                return 0 as i32;
                                                            } else {
                                                                if *__slate_slot_1315
                                                                    == (139 as i32)
                                                                    && (unsafe { (*pExpr).flags })
                                                                        & ((4096 as i32) as u32)
                                                                        != ((0 as i32) as u32)
                                                                {
                                                                    std::ptr::write(
                                                                        __slate_slot_2473,
                                                                        unsafe {
                                                                            (*unsafe {
                                                                                (*unsafe {
                                                                                    (*pExpr)
                                                                                        .x
                                                                                        .pSelect
                                                                                })
                                                                                .pEList
                                                                            })
                                                                            .nExpr
                                                                        },
                                                                    );
                                                                    *__slate_slot_1355 =
                                                                        *__slate_slot_2473;
                                                                    *__slate_slot_2472 =
                                                                        *__slate_slot_2473
                                                                            != (1 as i32);
                                                                } else {
                                                                    *__slate_slot_2472 =
                                                                        false as bool;
                                                                }
                                                                if *__slate_slot_2472 {
                                                                    sqlite3SubselectError(
                                                                        pParse,
                                                                        *__slate_slot_1355,
                                                                        1 as i32,
                                                                    );
                                                                    break '__join_0;
                                                                } else {
                                                                    return sqlite3CodeSubselect(
                                                                        pParse, pExpr,
                                                                    );
                                                                }
                                                            }
                                                        }
                                                        std::ptr::write(
                                                            __slate_slot_1349,
                                                            (0 as i32) as u32,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1351,
                                                            unsafe { (*pParse).db },
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1352,
                                                            unsafe { (*(*__slate_slot_1351)).enc },
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1353,
                                                            std::ptr::null_mut::<CollSeq>(),
                                                        );
                                                        if (unsafe { (*pExpr).flags })
                                                            & ((16777216 as i32) as u32)
                                                            != ((0 as i32) as u32)
                                                        {
                                                            return unsafe {
                                                                (*unsafe { (*pExpr).y.pWin })
                                                                    .regResult
                                                            };
                                                        } else {
                                                            if ((unsafe {
                                                                (*pParse)
                                                                    .__slate_bits_0
                                                                    .__get_okConstFactor()
                                                            })
                                                                as i32)
                                                                != (0 as i32)
                                                            {
                                                                *__slate_slot_2463 =
                                                                    sqlite3ExprIsConstantNotJoin(
                                                                        pParse, pExpr,
                                                                    ) != (0 as i32);
                                                            } else {
                                                                *__slate_slot_2463 = false as bool;
                                                            }
                                                            if *__slate_slot_2463 {
                                                                // SQL functions can be expensive. So try to avoid running them
                                                                // multiple times if we know they always give the same result
                                                                return sqlite3ExprCodeRunJustOnce(
                                                                    pParse,
                                                                    pExpr,
                                                                    -(1 as i32),
                                                                );
                                                            } else {
                                                                0 as i32;
                                                                0 as i32;
                                                                *__slate_slot_1345 =
                                                                    unsafe { (*pExpr).x.pList };
                                                                *__slate_slot_1346 =
                                                                    if *__slate_slot_1345
                                                                        != std::ptr::null_mut::<
                                                                            ExprList,
                                                                        >(
                                                                        )
                                                                    {
                                                                        unsafe {
                                                                            (*(*__slate_slot_1345))
                                                                                .nExpr
                                                                        }
                                                                    } else {
                                                                        0 as i32
                                                                    };
                                                                0 as i32;
                                                                *__slate_slot_1348 =
                                                                    (unsafe { (*pExpr).u.zToken })
                                                                        as *const i8;
                                                                *__slate_slot_1347 = unsafe {
                                                                    sqlite3FindFunction(
                                                                        *__slate_slot_1351,
                                                                        *__slate_slot_1348,
                                                                        *__slate_slot_1346,
                                                                        *__slate_slot_1352,
                                                                        ((0 as i32) as i8) as u8,
                                                                    )
                                                                };
                                                                if *__slate_slot_1347
                                                                    == std::ptr::null_mut::<FuncDef>(
                                                                    )
                                                                    && (unsafe {
                                                                        (*pParse).explain
                                                                    }) != (0 as u8)
                                                                {
                                                                    *__slate_slot_1347 = unsafe {
                                                                        sqlite3FindFunction(
                                                                            *__slate_slot_1351,
                                                                            (b"unknown\0".as_ptr()
                                                                                as *mut i8)
                                                                                as *const i8,
                                                                            *__slate_slot_1346,
                                                                            *__slate_slot_1352,
                                                                            ((0 as i32) as i8)
                                                                                as u8,
                                                                        )
                                                                    };
                                                                }
                                                                if *__slate_slot_1347
                                                                    == std::ptr::null_mut::<FuncDef>(
                                                                    )
                                                                    || (unsafe {
                                                                        (*(*__slate_slot_1347))
                                                                            .xFinalize
                                                                    }) != None
                                                                    || (unsafe {
                                                                        (*(*__slate_slot_1347))
                                                                            .funcFlags
                                                                    }) & ((262144 as i32) as u32)
                                                                        != ((0 as i32) as u32)
                                                                        && !((unsafe {
                                                                            (*pParse).nested
                                                                        }) != (0 as u8))
                                                                        && (unsafe {
                                                                            (*(*__slate_slot_1351))
                                                                                .mDbFlags
                                                                        }) & ((32 as i32) as u32)
                                                                            == ((0 as i32) as u32)
                                                                {
                                                                    unsafe {
                                                                        sqlite3ErrorMsg(pParse, (b"unknown function: %#T()\0".as_ptr() as *mut i8) as *const i8, pExpr)
                                                                    };
                                                                    break '__join_0;
                                                                } else {
                                                                    if (unsafe {
                                                                        (*(*__slate_slot_1347))
                                                                            .funcFlags
                                                                    }) & ((4194304 as i32)
                                                                        as u32)
                                                                        != ((0 as i32) as u32)
                                                                        && *__slate_slot_1345
                                                                            != std::ptr::null_mut::<
                                                                                ExprList,
                                                                            >(
                                                                            )
                                                                    {
                                                                        0 as i32;
                                                                        0 as i32;
                                                                        return exprCodeInlineFunction(pParse, *__slate_slot_1345, ((unsafe { (*(*__slate_slot_1347)).pUserData }) as i64) as i32, target);
                                                                    } else {
                                                                        if (unsafe {
                                                                            (*(*__slate_slot_1347))
                                                                                .funcFlags
                                                                        }) & (((524288 as i32)
                                                                            | (2097152 as i32))
                                                                            as u32)
                                                                            != (0 as u32)
                                                                        {
                                                                            sqlite3ExprFunctionUsable(pParse, pExpr as *const Expr, *__slate_slot_1347 as *const FuncDef);
                                                                        }
                                                                        *__slate_slot_1350 =
                                                                            0 as i32;
                                                                        loop {
                                                                            if *__slate_slot_1350
                                                                                < *__slate_slot_1346
                                                                            {
                                                                                if *__slate_slot_1350 < (32 as i32) {
*__slate_slot_2466 = sqlite3ExprIsConstant(pParse, unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1345)).a) as *mut ExprList_item }.offset(*__slate_slot_1350 as isize) }).pExpr }) != (0 as i32);
} else {
*__slate_slot_2466 = false as bool;
}
                                                                                if *__slate_slot_2466 {
{
}
std::ptr::write(__slate_slot_2467, *__slate_slot_1349);
std::ptr::write(__slate_slot_2468, *__slate_slot_2467 | ((1 as i32) as u32) << *__slate_slot_1350);
*__slate_slot_1349 = *__slate_slot_2468;
}
                                                                                if (unsafe { (*(*__slate_slot_1347)).funcFlags }) & ((32 as i32) as u32) != ((0 as i32) as u32) && !(*__slate_slot_1353 != std::ptr::null_mut::<CollSeq>()) {
*__slate_slot_1353 = sqlite3ExprCollSeq(pParse, (unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1345)).a) as *mut ExprList_item }.offset(*__slate_slot_1350 as isize) }).pExpr }) as *const Expr);
}
                                                                                std::ptr::write(__slate_slot_2464, *__slate_slot_1350);
                                                                                std::ptr::write(__slate_slot_2465, *__slate_slot_2464 + (1 as i32));
                                                                                *__slate_slot_1350 = *__slate_slot_2465;
                                                                            } else {
                                                                                break;
                                                                            }
                                                                        }
                                                                        if *__slate_slot_1345
                                                                            != std::ptr::null_mut::<
                                                                                ExprList,
                                                                            >(
                                                                            )
                                                                        {
                                                                            if *__slate_slot_1349
                                                                                != (0 as u32)
                                                                            {
                                                                                *__slate_slot_1319 = (unsafe { (*pParse).nMem }) + (1 as i32);
                                                                                std::ptr::write(__slate_slot_2469, pParse);
                                                                                std::ptr::write(__slate_slot_2470, unsafe { (*(*__slate_slot_2469)).nMem });
                                                                                std::ptr::write(__slate_slot_2471, *__slate_slot_2470 + *__slate_slot_1346);
                                                                                unsafe {
                                                                                    (*(*__slate_slot_2469)).nMem = *__slate_slot_2471;
                                                                                }
                                                                            } else {
                                                                                *__slate_slot_1319 = sqlite3GetTempRange(pParse, *__slate_slot_1346);
                                                                            }
                                                                            // For length() and typeof() and octet_length() functions,
                                                                            // set the P5 parameter to the OP_Column opcode to OPFLAG_LENGTHARG
                                                                            // or OPFLAG_TYPEOFARG or OPFLAG_BYTELENARG respectively, to avoid
                                                                            // unnecessary data loading.
                                                                            if (unsafe {
                                                                                (*(*__slate_slot_1347)).funcFlags
                                                                            }) & (((64 as i32)
                                                                                | (128 as i32))
                                                                                as u32)
                                                                                != ((0 as i32)
                                                                                    as u32)
                                                                            {
                                                                                0 as i32;
                                                                                0 as i32;
                                                                                *__slate_slot_1354 = unsafe { (*unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1345)).a) as *mut ExprList_item }.offset((0 as i32) as isize) }).pExpr }).op };
                                                                                if ((*__slate_slot_1354 as u32) as i32) == (168 as i32) || ((*__slate_slot_1354 as u32) as i32) == (170 as i32) {
0 as i32;
0 as i32;
0 as i32;
0 as i32;
{
}
{
}
{
}
unsafe {
(*unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1345)).a) as *mut ExprList_item }.offset((0 as i32) as isize) }).pExpr }).op2 = ((unsafe { (*(*__slate_slot_1347)).funcFlags }) & ((192 as i32) as u32)) as u8;
}
}
                                                                            }
                                                                            sqlite3ExprCodeExprList(
                                                                                pParse,
                                                                                *__slate_slot_1345,
                                                                                *__slate_slot_1319,
                                                                                0 as i32,
                                                                                ((2 as i32) as i8)
                                                                                    as u8,
                                                                            );
                                                                        } else {
                                                                            *__slate_slot_1319 =
                                                                                0 as i32;
                                                                        }
                                                                        // Possibly overload the function if the first argument is
                                                                        // a virtual table column.
                                                                        //
                                                                        // For infix functions (LIKE, GLOB, REGEXP, and MATCH) use the
                                                                        // second argument, not the first, as the argument to test to
                                                                        // see if it is a column in a virtual table.  This is done because
                                                                        // the left operand of infix functions (the operand we want to
                                                                        // control overloading) ends up as the second argument to the
                                                                        // function.  The expression "A glob B" is equivalent to
                                                                        // "glob(B,A).  We want to use the A in "A glob B" to test
                                                                        // for function overloading.  But we use the B term in "glob(B,A)".
                                                                        if *__slate_slot_1346
                                                                            >= (2 as i32)
                                                                            && (unsafe {
                                                                                (*pExpr).flags
                                                                            }) & ((256 as i32)
                                                                                as u32)
                                                                                != ((0 as i32)
                                                                                    as u32)
                                                                        {
                                                                            *__slate_slot_1347 = unsafe {
                                                                                sqlite3VtabOverloadFunction(*__slate_slot_1351, *__slate_slot_1347, *__slate_slot_1346, unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1345)).a) as *mut ExprList_item }.offset((1 as i32) as isize) }).pExpr })
                                                                            };
                                                                        } else {
                                                                            if *__slate_slot_1346
                                                                                > (0 as i32)
                                                                            {
                                                                                *__slate_slot_1347 = unsafe { sqlite3VtabOverloadFunction(*__slate_slot_1351, *__slate_slot_1347, *__slate_slot_1346, unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1345)).a) as *mut ExprList_item }.offset((0 as i32) as isize) }).pExpr }) };
                                                                            }
                                                                        }
                                                                        if (unsafe {
                                                                            (*(*__slate_slot_1347))
                                                                                .funcFlags
                                                                        }) & ((32 as i32) as u32)
                                                                            != (0 as u32)
                                                                        {
                                                                            if !(*__slate_slot_1353 != std::ptr::null_mut::<CollSeq>()) {
*__slate_slot_1353 = unsafe { (*(*__slate_slot_1351)).pDfltColl };
}
                                                                            unsafe {
                                                                                sqlite3VdbeAddOp4(*__slate_slot_1314, 87 as i32, 0 as i32, 0 as i32, 0 as i32, (*__slate_slot_1353 as *mut i8) as *const i8, -(2 as i32))
                                                                            };
                                                                        }
                                                                        unsafe {
                                                                            sqlite3VdbeAddFunctionCall(pParse, *__slate_slot_1349 as i32, *__slate_slot_1319, target, *__slate_slot_1346, *__slate_slot_1347 as *const FuncDef, ((unsafe { (*pExpr).op2 }) as u32) as i32)
                                                                        };
                                                                        if *__slate_slot_1346
                                                                            != (0 as i32)
                                                                        {
                                                                            if *__slate_slot_1349
                                                                                == ((0 as i32)
                                                                                    as u32)
                                                                            {
                                                                                sqlite3ReleaseTempRange(pParse, *__slate_slot_1319, *__slate_slot_1346);
                                                                            } else {
                                                                                {}
                                                                            }
                                                                        }
                                                                        return target;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    std::ptr::write(__slate_slot_1344, unsafe {
                                                        (*pExpr).pAggInfo
                                                    });
                                                    if *__slate_slot_1344
                                                        == std::ptr::null_mut::<AggInfo>()
                                                        || ((unsafe { (*pExpr).iAgg }) as i32)
                                                            < (0 as i32)
                                                        || ((unsafe { (*pExpr).iAgg }) as i32)
                                                            >= unsafe {
                                                                (*(*__slate_slot_1344)).nFunc
                                                            }
                                                    {
                                                        0 as i32;
                                                        unsafe {
                                                            sqlite3ErrorMsg(
                                                                pParse,
                                                                (b"misuse of aggregate: %#T()\0"
                                                                    .as_ptr()
                                                                    as *mut i8)
                                                                    as *const i8,
                                                                pExpr,
                                                            )
                                                        };
                                                        break '__join_0;
                                                    } else {
                                                        return (unsafe {
                                                            (*(*__slate_slot_1344)).iFirstReg
                                                        }) + unsafe {
                                                            (*(*__slate_slot_1344)).nColumn
                                                        } + ((unsafe { (*pExpr).iAgg })
                                                            as i32);
                                                    }
                                                }
                                                0 as i32;
                                                {}
                                                0 as i32;
                                                {}
                                                unsafe {
                                                    sqlite3VdbeAddOp2(
                                                        *__slate_slot_1314,
                                                        73 as i32,
                                                        1 as i32,
                                                        target,
                                                    )
                                                };
                                                *__slate_slot_1319 = sqlite3ExprCodeTemp(
                                                    pParse,
                                                    unsafe { (*pExpr).pLeft },
                                                    std::ptr::addr_of_mut!(*__slate_slot_1317),
                                                );
                                                {}
                                                *__slate_slot_1343 = unsafe {
                                                    sqlite3VdbeAddOp1(
                                                        *__slate_slot_1314,
                                                        *__slate_slot_1315,
                                                        *__slate_slot_1319,
                                                    )
                                                };
                                                {}
                                                {}
                                                unsafe {
                                                    sqlite3VdbeAddOp2(
                                                        *__slate_slot_1314,
                                                        73 as i32,
                                                        0 as i32,
                                                        target,
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3VdbeJumpHere(
                                                        *__slate_slot_1314,
                                                        *__slate_slot_1343,
                                                    )
                                                };
                                                break '__join_0;
                                            }
                                            *__slate_slot_1319 = sqlite3ExprCodeTemp(
                                                pParse,
                                                unsafe { (*pExpr).pLeft },
                                                std::ptr::addr_of_mut!(*__slate_slot_1317),
                                            );
                                            {}
                                            *__slate_slot_1341 = sqlite3ExprTruthValue(
                                                (unsafe { (*pExpr).pRight }) as *const Expr,
                                            );
                                            *__slate_slot_1342 =
                                                ((((unsafe { (*pExpr).op2 }) as u32) as i32)
                                                    == (45 as i32))
                                                    as i32;
                                            {}
                                            {}
                                            unsafe {
                                                sqlite3VdbeAddOp4Int(
                                                    *__slate_slot_1314,
                                                    93 as i32,
                                                    *__slate_slot_1319,
                                                    *__slate_slot_1316,
                                                    !(*__slate_slot_1341 != (0 as i32)) as i32,
                                                    *__slate_slot_1341 ^ *__slate_slot_1342,
                                                )
                                            };
                                            break '__join_0;
                                        }
                                        0 as i32;
                                        {}
                                        0 as i32;
                                        {}
                                        *__slate_slot_1319 = sqlite3ExprCodeTemp(
                                            pParse,
                                            unsafe { (*pExpr).pLeft },
                                            std::ptr::addr_of_mut!(*__slate_slot_1317),
                                        );
                                        {}
                                        unsafe {
                                            sqlite3VdbeAddOp2(
                                                *__slate_slot_1314,
                                                *__slate_slot_1315,
                                                *__slate_slot_1319,
                                                *__slate_slot_1316,
                                            )
                                        };
                                        break '__join_0;
                                    }
                                    std::ptr::write(__slate_slot_1340, unsafe { (*pExpr).pLeft });
                                    0 as i32;
                                    if (((unsafe { (*(*__slate_slot_1340)).op }) as u32) as i32)
                                        == (156 as i32)
                                    {
                                        codeInteger(pParse, *__slate_slot_1340, 1 as i32, target);
                                        return target;
                                    } else {
                                        if (((unsafe { (*(*__slate_slot_1340)).op }) as u32) as i32)
                                            == (154 as i32)
                                        {
                                            0 as i32;
                                            codeReal(
                                                *__slate_slot_1314,
                                                (unsafe { (*(*__slate_slot_1340)).u.zToken })
                                                    as *const i8,
                                                1 as i32,
                                                target,
                                            );
                                            return target;
                                        } else {
                                            (*__slate_slot_1321).op = ((156 as i32) as i8) as u8;
                                            (*__slate_slot_1321).flags =
                                                ((2048 as i32) | (65536 as i32)) as u32;
                                            unsafe {
                                                (*__slate_slot_1321).u.iValue = 0 as i32;
                                            }
                                            {}
                                            *__slate_slot_1319 = sqlite3ExprCodeTemp(
                                                pParse,
                                                std::ptr::addr_of_mut!(*__slate_slot_1321),
                                                std::ptr::addr_of_mut!(*__slate_slot_1317),
                                            );
                                            *__slate_slot_1320 = sqlite3ExprCodeTemp(
                                                pParse,
                                                unsafe { (*pExpr).pLeft },
                                                std::ptr::addr_of_mut!(*__slate_slot_1318),
                                            );
                                            unsafe {
                                                sqlite3VdbeAddOp3(
                                                    *__slate_slot_1314,
                                                    108 as i32,
                                                    *__slate_slot_1320,
                                                    *__slate_slot_1319,
                                                    target,
                                                )
                                            };
                                            {}
                                            break '__join_0;
                                        }
                                    }
                                }
                                0 as i32;
                                {}
                                0 as i32;
                                {}
                                0 as i32;
                                {}
                                0 as i32;
                                {}
                                0 as i32;
                                {}
                                0 as i32;
                                {}
                                0 as i32;
                                {}
                                0 as i32;
                                {}
                                0 as i32;
                                {}
                                if (unsafe { (*pExpr).flags }) & ((4194304 as i32) as u32)
                                    != ((0 as i32) as u32)
                                {
                                    *__slate_slot_1339 = exprComputeOperands(
                                        pParse,
                                        pExpr,
                                        std::ptr::addr_of_mut!(*__slate_slot_1319),
                                        std::ptr::addr_of_mut!(*__slate_slot_1320),
                                        std::ptr::addr_of_mut!(*__slate_slot_1317),
                                        std::ptr::addr_of_mut!(*__slate_slot_1318),
                                    );
                                } else {
                                    *__slate_slot_1319 = sqlite3ExprCodeTemp(
                                        pParse,
                                        unsafe { (*pExpr).pLeft },
                                        std::ptr::addr_of_mut!(*__slate_slot_1317),
                                    );
                                    *__slate_slot_1320 = sqlite3ExprCodeTemp(
                                        pParse,
                                        unsafe { (*pExpr).pRight },
                                        std::ptr::addr_of_mut!(*__slate_slot_1318),
                                    );
                                    *__slate_slot_1339 = 0 as i32;
                                }
                                unsafe {
                                    sqlite3VdbeAddOp3(
                                        *__slate_slot_1314,
                                        *__slate_slot_1315,
                                        *__slate_slot_1320,
                                        *__slate_slot_1319,
                                        target,
                                    )
                                };
                                {}
                                {}
                                if *__slate_slot_1339 != (0 as i32) {
                                    unsafe {
                                        sqlite3VdbeAddOp2(
                                            *__slate_slot_1314,
                                            9 as i32,
                                            0 as i32,
                                            (unsafe { sqlite3VdbeCurrentAddr(*__slate_slot_1314) })
                                                + (2 as i32),
                                        )
                                    };
                                    unsafe {
                                        sqlite3VdbeJumpHere(*__slate_slot_1314, *__slate_slot_1339)
                                    };
                                    unsafe {
                                        sqlite3VdbeAddOp2(
                                            *__slate_slot_1314,
                                            77 as i32,
                                            0 as i32,
                                            target,
                                        )
                                    };
                                    unsafe {
                                        sqlite3VdbeComment(
                                            *__slate_slot_1314,
                                            (b"short-circut value\0".as_ptr() as *mut i8)
                                                as *const i8,
                                        )
                                    };
                                    break '__join_0;
                                } else {
                                    break '__join_0;
                                }
                            }
                            *__slate_slot_1316 = exprCodeTargetAndOr(
                                pParse,
                                pExpr,
                                target,
                                std::ptr::addr_of_mut!(*__slate_slot_1317),
                            );
                            break '__join_0;
                        }
                        *__slate_slot_1315 = if *__slate_slot_1315 == (45 as i32) {
                            54 as i32
                        } else {
                            53 as i32
                        };
                        *__slate_slot_1322 = 128 as i32;
                        // no break
                        {}
                    }
                    std::ptr::write(__slate_slot_1337, unsafe { (*pExpr).pLeft });
                    std::ptr::write(__slate_slot_1338, 0 as i32);
                    if sqlite3ExprIsVector(*__slate_slot_1337 as *const Expr) != (0 as i32) {
                        codeVectorCompare(
                            pParse,
                            pExpr,
                            target,
                            (*__slate_slot_1315 as i8) as u8,
                            (*__slate_slot_1322 as i8) as u8,
                        );
                        break '__join_0;
                    } else {
                        if (unsafe { (*pExpr).flags }) & ((4194304 as i32) as u32)
                            != ((0 as i32) as u32)
                            && *__slate_slot_1322 != (128 as i32)
                        {
                            *__slate_slot_1338 = exprComputeOperands(
                                pParse,
                                pExpr,
                                std::ptr::addr_of_mut!(*__slate_slot_1319),
                                std::ptr::addr_of_mut!(*__slate_slot_1320),
                                std::ptr::addr_of_mut!(*__slate_slot_1317),
                                std::ptr::addr_of_mut!(*__slate_slot_1318),
                            );
                        } else {
                            *__slate_slot_1319 = sqlite3ExprCodeTemp(
                                pParse,
                                unsafe { (*pExpr).pLeft },
                                std::ptr::addr_of_mut!(*__slate_slot_1317),
                            );
                            *__slate_slot_1320 = sqlite3ExprCodeTemp(
                                pParse,
                                unsafe { (*pExpr).pRight },
                                std::ptr::addr_of_mut!(*__slate_slot_1318),
                            );
                        }
                        unsafe {
                            sqlite3VdbeAddOp2(
                                *__slate_slot_1314,
                                73 as i32,
                                1 as i32,
                                *__slate_slot_1316,
                            )
                        };
                        codeCompare(
                            pParse,
                            *__slate_slot_1337,
                            unsafe { (*pExpr).pRight },
                            *__slate_slot_1315,
                            *__slate_slot_1319,
                            *__slate_slot_1320,
                            (unsafe { sqlite3VdbeCurrentAddr(*__slate_slot_1314) }) + (2 as i32),
                            *__slate_slot_1322,
                            ((unsafe { (*pExpr).flags }) & ((1024 as i32) as u32)
                                != ((0 as i32) as u32)) as i32,
                        );
                        0 as i32;
                        {}
                        {}
                        0 as i32;
                        {}
                        {}
                        0 as i32;
                        {}
                        {}
                        0 as i32;
                        {}
                        {}
                        0 as i32;
                        {}
                        {}
                        0 as i32;
                        {}
                        {}
                        if *__slate_slot_1322 == (128 as i32) {
                            unsafe {
                                sqlite3VdbeAddOp2(
                                    *__slate_slot_1314,
                                    73 as i32,
                                    0 as i32,
                                    *__slate_slot_1316,
                                )
                            };
                        } else {
                            unsafe {
                                sqlite3VdbeAddOp3(
                                    *__slate_slot_1314,
                                    94 as i32,
                                    *__slate_slot_1319,
                                    *__slate_slot_1316,
                                    *__slate_slot_1320,
                                )
                            };
                            if *__slate_slot_1338 != (0 as i32) {
                                unsafe {
                                    sqlite3VdbeAddOp2(
                                        *__slate_slot_1314,
                                        9 as i32,
                                        0 as i32,
                                        (unsafe { sqlite3VdbeCurrentAddr(*__slate_slot_1314) })
                                            + (2 as i32),
                                    )
                                };
                                unsafe {
                                    sqlite3VdbeJumpHere(*__slate_slot_1314, *__slate_slot_1338)
                                };
                                unsafe {
                                    sqlite3VdbeAddOp2(
                                        *__slate_slot_1314,
                                        77 as i32,
                                        0 as i32,
                                        *__slate_slot_1316,
                                    )
                                };
                            }
                        }
                        {}
                        {}
                        break '__join_0;
                    }
                }
                std::ptr::write(__slate_slot_1323, unsafe { (*pExpr).pAggInfo });
                0 as i32;
                0 as i32;
                if ((unsafe { (*pExpr).iAgg }) as i32) >= unsafe { (*(*__slate_slot_1323)).nColumn }
                {
                    // Happens when the left table of a RIGHT JOIN is null and
                    // is using an expression index
                    unsafe { sqlite3VdbeAddOp2(*__slate_slot_1314, 77 as i32, 0 as i32, target) };
                    break '__join_0;
                } else {
                    *__slate_slot_1324 = unsafe {
                        unsafe { (*(*__slate_slot_1323)).aCol }
                            .offset(((unsafe { (*pExpr).iAgg }) as i32) as isize)
                    };
                    if !((unsafe { (*(*__slate_slot_1323)).directMode }) != (0 as u8)) {
                        return (unsafe { (*(*__slate_slot_1323)).iFirstReg })
                            + ((unsafe { (*pExpr).iAgg }) as i32);
                    } else {
                        if (unsafe { (*(*__slate_slot_1323)).useSortingIdx }) != (0 as u8) {
                            std::ptr::write(__slate_slot_1325, unsafe {
                                (*(*__slate_slot_1324)).pTab
                            });
                            unsafe {
                                sqlite3VdbeAddOp3(
                                    *__slate_slot_1314,
                                    96 as i32,
                                    unsafe { (*(*__slate_slot_1323)).sortingIdxPTab },
                                    unsafe { (*(*__slate_slot_1324)).iSorterColumn },
                                    target,
                                )
                            };
                            if *__slate_slot_1325 == std::ptr::null_mut::<Table>() {
                                // No comment added
                            } else {
                                if (unsafe { (*(*__slate_slot_1324)).iColumn }) < (0 as i32) {
                                    unsafe {
                                        sqlite3VdbeComment(
                                            *__slate_slot_1314,
                                            (b"%s.rowid\0".as_ptr() as *mut i8) as *const i8,
                                            unsafe { (*(*__slate_slot_1325)).zName },
                                        )
                                    };
                                } else {
                                    unsafe {
                                        sqlite3VdbeComment(
                                            *__slate_slot_1314,
                                            (b"%s.%s\0".as_ptr() as *mut i8) as *const i8,
                                            unsafe { (*(*__slate_slot_1325)).zName },
                                            unsafe {
                                                (*unsafe {
                                                    unsafe { (*(*__slate_slot_1325)).aCol }.offset(
                                                        (unsafe { (*(*__slate_slot_1324)).iColumn })
                                                            as isize,
                                                    )
                                                })
                                                .zCnName
                                            },
                                        )
                                    };
                                    if ((unsafe {
                                        (*unsafe {
                                            unsafe { (*(*__slate_slot_1325)).aCol }.offset(
                                                (unsafe { (*(*__slate_slot_1324)).iColumn })
                                                    as isize,
                                            )
                                        })
                                        .affinity
                                    }) as i32)
                                        == (69 as i32)
                                    {
                                        unsafe {
                                            sqlite3VdbeAddOp1(*__slate_slot_1314, 89 as i32, target)
                                        };
                                    }
                                }
                            }
                            return target;
                        } else {
                            if (unsafe { (*pExpr).y.pTab }) == std::ptr::null_mut::<Table>() {
                                // This case happens when the argument to an aggregate function
                                // is rewritten by aggregateConvertIndexedExprRefToColumn()
                                unsafe {
                                    sqlite3VdbeAddOp3(
                                        *__slate_slot_1314,
                                        96 as i32,
                                        unsafe { (*pExpr).iTable },
                                        (unsafe { (*pExpr).iColumn }) as i32,
                                        target,
                                    )
                                };
                                return target;
                            } else {
                                // Otherwise, fall thru into the TK_COLUMN case
                                //
                                // no break
                                {}
                            }
                        }
                    }
                }
            }
            std::ptr::write(__slate_slot_1326, unsafe { (*pExpr).iTable });
            if (unsafe { (*pExpr).flags }) & ((32 as i32) as u32) != ((0 as i32) as u32) {
                *__slate_slot_1327 =
                    sqlite3ExprCodeTarget(pParse, unsafe { (*pExpr).pLeft }, target);
                0 as i32;
                0 as i32;
                *__slate_slot_1328 = sqlite3TableColumnAffinity(
                    (unsafe { (*pExpr).y.pTab }) as *const Table,
                    (unsafe { (*pExpr).iColumn }) as i32,
                ) as i32;
                if *__slate_slot_1328 > (65 as i32) {
                    0 as i32;
                    0 as i32;
                    unsafe {
                        sqlite3VdbeAddOp4(
                            *__slate_slot_1314,
                            98 as i32,
                            *__slate_slot_1327,
                            1 as i32,
                            0 as i32,
                            unsafe {
                                unsafe { std::ptr::addr_of!(zAff) as *const i8 }.offset(
                                    ((*__slate_slot_1328 - (66 as i32)) * (2 as i32)) as isize,
                                )
                            },
                            -(1 as i32),
                        )
                    };
                }
                return *__slate_slot_1327;
            } else {
                if *__slate_slot_1326 < (0 as i32) {
                    if (unsafe { (*pParse).iSelfTab }) < (0 as i32) {
                        std::ptr::write(__slate_slot_1333, (unsafe { (*pExpr).iColumn }) as i32);
                        0 as i32;
                        *__slate_slot_1331 = unsafe { (*pExpr).y.pTab };
                        0 as i32;
                        0 as i32;
                        0 as i32;
                        if *__slate_slot_1333 < (0 as i32) {
                            return -(1 as i32) - unsafe { (*pParse).iSelfTab };
                        } else {
                            *__slate_slot_1330 = unsafe {
                                unsafe { (*(*__slate_slot_1331)).aCol }
                                    .offset(*__slate_slot_1333 as isize)
                            };
                            {}
                            *__slate_slot_1332 = ((unsafe {
                                sqlite3TableColumnToStorage(
                                    *__slate_slot_1331,
                                    *__slate_slot_1333 as i16,
                                )
                            }) as i32)
                                - unsafe { (*pParse).iSelfTab };
                            if (((unsafe { (*(*__slate_slot_1330)).colFlags }) as u32) as i32)
                                & (96 as i32)
                                != (0 as i32)
                            {
                                if (((unsafe { (*(*__slate_slot_1330)).colFlags }) as u32) as i32)
                                    & (256 as i32)
                                    != (0 as i32)
                                {
                                    unsafe {
                                        sqlite3ErrorMsg(
                                            pParse,
                                            (b"generated column loop on \"%s\"\0".as_ptr()
                                                as *mut i8)
                                                as *const i8,
                                            unsafe { (*(*__slate_slot_1330)).zCnName },
                                        )
                                    };
                                    return 0 as i32;
                                } else {
                                    std::ptr::write(__slate_slot_2455, *__slate_slot_1330);
                                    std::ptr::write(__slate_slot_2456, unsafe {
                                        (*(*__slate_slot_2455)).colFlags
                                    });
                                    std::ptr::write(
                                        __slate_slot_2457,
                                        ((((*__slate_slot_2456 as u32) as i32) | (256 as i32))
                                            as i16) as u16,
                                    );
                                    unsafe {
                                        (*(*__slate_slot_2455)).colFlags = *__slate_slot_2457;
                                    }
                                    if (((unsafe { (*(*__slate_slot_1330)).colFlags }) as u32)
                                        as i32)
                                        & (128 as i32)
                                        != (0 as i32)
                                    {
                                        sqlite3ExprCodeGeneratedColumn(
                                            pParse,
                                            *__slate_slot_1331,
                                            *__slate_slot_1330,
                                            *__slate_slot_1332,
                                        );
                                    }
                                    std::ptr::write(__slate_slot_2458, *__slate_slot_1330);
                                    std::ptr::write(__slate_slot_2459, unsafe {
                                        (*(*__slate_slot_2458)).colFlags
                                    });
                                    std::ptr::write(
                                        __slate_slot_2460,
                                        ((((*__slate_slot_2459 as u32) as i32)
                                            & !((256 as i32) | (128 as i32)))
                                            as i16) as u16,
                                    );
                                    unsafe {
                                        (*(*__slate_slot_2458)).colFlags = *__slate_slot_2460;
                                    }
                                    return *__slate_slot_1332;
                                }
                            } else {
                                if ((unsafe { (*(*__slate_slot_1330)).affinity }) as i32)
                                    == (69 as i32)
                                {
                                    unsafe {
                                        sqlite3VdbeAddOp2(
                                            *__slate_slot_1314,
                                            83 as i32,
                                            *__slate_slot_1332,
                                            target,
                                        )
                                    };
                                    unsafe {
                                        sqlite3VdbeAddOp1(*__slate_slot_1314, 89 as i32, target)
                                    };
                                    return target;
                                } else {
                                    return *__slate_slot_1332;
                                }
                            }
                        }
                    } else {
                        // Coding an expression that is part of an index where column names
                        // in the index refer to the table to which the index belongs
                        *__slate_slot_1326 = (unsafe { (*pParse).iSelfTab }) - (1 as i32);
                    }
                } else {
                    if (unsafe { (*pParse).pIdxPartExpr }) != std::ptr::null_mut::<IndexedExpr>() {
                        std::ptr::write(
                            __slate_slot_2462,
                            exprPartidxExprLookup(pParse, pExpr, target),
                        );
                        *__slate_slot_1319 = *__slate_slot_2462;
                        *__slate_slot_2461 = (0 as i32) != *__slate_slot_2462;
                    } else {
                        *__slate_slot_2461 = false as bool;
                    }
                    if *__slate_slot_2461 {
                        return *__slate_slot_1319;
                    }
                }
                0 as i32;
                0 as i32;
                *__slate_slot_1327 = sqlite3ExprCodeGetColumn(
                    pParse,
                    unsafe { (*pExpr).y.pTab },
                    (unsafe { (*pExpr).iColumn }) as i32,
                    *__slate_slot_1326,
                    target,
                    unsafe { (*pExpr).op2 },
                );
                return *__slate_slot_1327;
            }
        }
        sqlite3ReleaseTempReg(pParse, *__slate_slot_1317);
        sqlite3ReleaseTempReg(pParse, *__slate_slot_1318);
        return *__slate_slot_1316;
    }
    //    x BETWEEN y AND z
    //
    // This is equivalent to
    //
    //    x>=y AND x<=z
    //
    // X is stored in pExpr->pLeft.
    // Y is stored in pExpr->pList->a[0].pExpr.
    // Z is stored in pExpr->pList->a[1].pExpr.
    // 2018-04-28: Prevent deep recursion.
    // 2018-04-28: Prevent deep recursion. OSSFuzz.
    // TK_IF_NULL_ROW Expr nodes are inserted ahead of expressions
    // that derive from the right-hand table of a LEFT JOIN.  The
    // Expr.iTable value is the table number for the right-hand table.
    // The expression is only evaluated if that table is not currently
    // on a LEFT JOIN NULL row.
    // Form A:
    //   CASE x WHEN e1 THEN r1 WHEN e2 THEN r2 ... WHEN eN THEN rN ELSE y END
    //
    // Form B:
    //   CASE WHEN e1 THEN r1 WHEN e2 THEN r2 ... WHEN eN THEN rN ELSE y END
    //
    // Form A is can be transformed into the equivalent form B as follows:
    //   CASE WHEN x=e1 THEN r1 WHEN x=e2 THEN r2 ...
    //        WHEN x=eN THEN rN ELSE y END
    //
    // X (if it exists) is in pExpr->pLeft.
    // Y is in the last element of pExpr->x.pList if pExpr->x.pList->nExpr is
    // odd.  The Y is also optional.  If the number of elements in x.pList
    // is even, then Y is omitted and the "otherwise" result is NULL.
    // Ei is in pExpr->pList->a[i*2] and Ri is pExpr->pList->a[i*2+1].
    //
    // The result of the expression is the Ri for the first matching Ei,
    // or if there is no matching Ei, the ELSE term Y, or if there is
    // no ELSE term, NULL.
    return unsafe { std::mem::zeroed() };
}

static mut zAff: [i8; 10] = [
    66 as i8, 0 as i8, 67 as i8, 0 as i8, 68 as i8, 0 as i8, 69 as i8, 0 as i8, 70 as i8, 0 as i8,
];

/// Generate code that will evaluate expression pExpr just one time
/// per prepared statement execution.
///
/// If the expression uses functions (that might throw an exception) then
/// guard them with an OP_Once opcode to ensure that the code is only executed
/// once. If no functions are involved, then factor the code out and put it at
/// the end of the prepared statement in the initialization section.
///
/// If regDest>0 then the result is always stored in that register and the
/// result is not reusable.  If regDest<0 then this routine is free to
/// store the value wherever it wants.  The register where the expression
/// is stored is returned.  When regDest<0, two identical expressions might
/// code to the same register, if they do not contain function calls and hence
/// are factored out into the initialization section at the end of the
/// prepared statement.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pExpr` - The expression to code when the VDBE initializes
/// * `regDest` - Store the value in this register
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCodeRunJustOnce(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut regDest: i32,
) -> i32 {
    let mut p: *mut ExprList = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    p = unsafe { (*pParse).pConstExpr };
    if regDest < (0 as i32) && p != std::ptr::null_mut::<ExprList>() {
        let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
        let mut i: i32 = 0 as i32;
        pItem = unsafe { std::ptr::addr_of_mut!((*p).a) as *mut ExprList_item };
        let __v2440: i32 = unsafe { (*p).nExpr };
        i = __v2440;
        '__slate_break_2344: loop {
            if !(i > (0 as i32)) {
                break;
            }
            let __v2445: bool;
            if ((unsafe { (*pItem).fg.__slate_bits_0.__get_reusable() }) as i32) != (0 as i32) {
                __v2445 = sqlite3ExprCompare(
                    std::ptr::null::<Parse>(),
                    (unsafe { (*pItem).pExpr }) as *const Expr,
                    pExpr as *const Expr,
                    -(1 as i32),
                ) == (0 as i32);
            } else {
                __v2445 = false as bool;
            }
            if __v2445 {
                return unsafe { (*pItem).u.iConstExprReg };
            }
            let __v2441: *mut ExprList_item = pItem;
            let __v2442: *mut ExprList_item = unsafe { __v2441.offset((1 as i32) as isize) };
            pItem = __v2442;
            let __v2443: i32 = i;
            let __v2444: i32 = __v2443 - (1 as i32);
            i = __v2444;
        }
    }
    pExpr = sqlite3ExprDup(unsafe { (*pParse).db }, pExpr as *const Expr, 0 as i32);
    if pExpr != std::ptr::null_mut::<Expr>()
        && (unsafe { (*pExpr).flags }) & ((8 as i32) as u32) != ((0 as i32) as u32)
    {
        let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
        let mut addr: i32 = 0 as i32;
        0 as i32;
        addr = unsafe { sqlite3VdbeAddOp0(v, 15 as i32) };
        {}
        unsafe {
            (*pParse)
                .__slate_bits_0
                .__set_okConstFactor((0 as i32) as u32);
        }
        if !((unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8)) {
            if regDest < (0 as i32) {
                let __v2446: *mut Parse = pParse;
                let __v2447: i32 = unsafe { (*__v2446).nMem };
                let __v2448: i32 = __v2447 + (1 as i32);
                unsafe {
                    (*__v2446).nMem = __v2448;
                }
                regDest = __v2448;
            }
            sqlite3ExprCode(pParse, pExpr, regDest);
        }
        unsafe {
            (*pParse)
                .__slate_bits_0
                .__set_okConstFactor((1 as i32) as u32);
        }
        sqlite3ExprDelete(unsafe { (*pParse).db }, pExpr);
        unsafe { sqlite3VdbeJumpHere(v, addr) };
    } else {
        p = sqlite3ExprListAppend(pParse, p, pExpr);
        if p != std::ptr::null_mut::<ExprList>() {
            let mut pItem: *mut ExprList_item = unsafe {
                unsafe { std::ptr::addr_of_mut!((*p).a) as *mut ExprList_item }
                    .offset(((unsafe { (*p).nExpr }) - (1 as i32)) as isize)
            };
            unsafe {
                (*pItem)
                    .fg
                    .__slate_bits_0
                    .__set_reusable((regDest < (0 as i32)) as u32);
            }
            if regDest < (0 as i32) {
                let __v2449: *mut Parse = pParse;
                let __v2450: i32 = unsafe { (*__v2449).nMem };
                let __v2451: i32 = __v2450 + (1 as i32);
                unsafe {
                    (*__v2449).nMem = __v2451;
                }
                regDest = __v2451;
            }
            unsafe {
                (*pItem).u.iConstExprReg = regDest;
            }
        }
        unsafe {
            (*pParse).pConstExpr = p;
        }
    }
    return regDest;
}

/// Make arrangements to invoke OP_Null on a range of registers
/// during initialization.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `iReg` - First register to set to NULL
/// * `nReg` - Number of sequential registers to NULL out
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprNullRegisterRange(
    mut pParse: *mut Parse,
    mut iReg: i32,
    mut nReg: i32,
) {
    let mut okConstFactor: u8 =
        (((unsafe { (*pParse).__slate_bits_0.__get_okConstFactor() }) as i32) as i8) as u8;
    let mut t: Expr = unsafe { std::mem::zeroed() };
    unsafe { memset(std::ptr::addr_of_mut!(t) as *mut (), 0 as i32, 72 as u64) };
    t.op = ((83 as i32) as i8) as u8;
    unsafe {
        t.y.nReg = nReg;
    }
    unsafe {
        (*pParse)
            .__slate_bits_0
            .__set_okConstFactor((1 as i32) as u32);
    }
    sqlite3ExprCodeRunJustOnce(pParse, std::ptr::addr_of_mut!(t), iReg);
    unsafe {
        (*pParse)
            .__slate_bits_0
            .__set_okConstFactor(okConstFactor as u32);
    }
}

/// Generate code to evaluate an expression and store the results
/// into a register.  Return the register number where the results
/// are stored.
///
/// If the register is a temporary register that can be deallocated,
/// then write its number into *pReg.  If the result register is not
/// a temporary, then set *pReg to zero.
///
/// If pExpr is a constant, then this routine might generate this
/// code to fill the register in the initialization section of the
/// VDBE program, in order to factor it out of the evaluation loop.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCodeTemp(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut pReg: *mut i32,
) -> i32 {
    let mut r2: i32 = 0 as i32;
    pExpr = sqlite3ExprSkipCollateAndLikely(pExpr);
    let __v2452: bool;
    if ((unsafe { (*pParse).__slate_bits_0.__get_okConstFactor() }) as i32) != (0 as i32)
        && pExpr != std::ptr::null_mut::<Expr>()
        && (((unsafe { (*pExpr).op }) as u32) as i32) != (176 as i32)
    {
        __v2452 = sqlite3ExprIsConstantNotJoin(pParse, pExpr) != (0 as i32);
    } else {
        __v2452 = false as bool;
    }
    if __v2452 {
        unsafe {
            *pReg = 0 as i32;
        }
        r2 = sqlite3ExprCodeRunJustOnce(pParse, pExpr, -(1 as i32));
    } else {
        let mut r1: i32 = sqlite3GetTempReg(pParse);
        r2 = sqlite3ExprCodeTarget(pParse, pExpr, r1);
        if r2 == r1 {
            unsafe {
                *pReg = r1;
            }
        } else {
            sqlite3ReleaseTempReg(pParse, r1);
            unsafe {
                *pReg = 0 as i32;
            }
        }
    }
    return r2;
}

/// Generate code that will evaluate expression pExpr and store the
/// results in register target.  The results are guaranteed to appear
/// in register target.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCode(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut target: i32,
) {
    let mut inReg: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe { (*pParse).pVdbe }) == std::ptr::null_mut::<Vdbe>() {
        return;
    }
    inReg = sqlite3ExprCodeTarget(pParse, pExpr, target);
    if inReg != target {
        let mut op: u8 = 0 as u8;
        let mut pX: *mut Expr = sqlite3ExprSkipCollateAndLikely(pExpr);
        {}
        if pX != std::ptr::null_mut::<Expr>()
            && ((unsafe { (*pX).flags }) & ((4194304 as i32) as u32) != ((0 as i32) as u32)
                || (((unsafe { (*pX).op }) as u32) as i32) == (176 as i32))
        {
            op = ((82 as i32) as i8) as u8;
        } else {
            op = ((83 as i32) as i8) as u8;
        }
        unsafe {
            sqlite3VdbeAddOp2(
                unsafe { (*pParse).pVdbe },
                (op as u32) as i32,
                inReg,
                target,
            )
        };
    }
}

/// Make a transient copy of expression pExpr and then code it using
/// sqlite3ExprCode().  This routine works just like sqlite3ExprCode()
/// except that the input expression is guaranteed to be unchanged.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCodeCopy(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut target: i32,
) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    pExpr = sqlite3ExprDup(db, pExpr as *const Expr, 0 as i32);
    if !((unsafe { (*db).mallocFailed }) != (0 as u8)) {
        sqlite3ExprCode(pParse, pExpr, target);
    }
    sqlite3ExprDelete(db, pExpr);
}

/// Generate code that will evaluate expression pExpr and store the
/// results in register target.  The results are guaranteed to appear
/// in register target.  If the expression is constant, then this routine
/// might choose to code the expression at initialization time.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCodeFactorable(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut target: i32,
) {
    let __v2439: bool;
    if ((unsafe { (*pParse).__slate_bits_0.__get_okConstFactor() }) as i32) != (0 as i32) {
        __v2439 = sqlite3ExprIsConstantNotJoin(pParse, pExpr) != (0 as i32);
    } else {
        __v2439 = false as bool;
    }
    if __v2439 {
        sqlite3ExprCodeRunJustOnce(pParse, pExpr, target);
    } else {
        sqlite3ExprCodeCopy(pParse, pExpr, target);
    }
}

/// Generate code that pushes the value of every element of the given
/// expression list into a sequence of registers beginning at target.
///
/// Return the number of elements evaluated.  The number returned will
/// usually be pList->nExpr but might be reduced if SQLITE_ECEL_OMITREF
/// is defined.
///
/// The SQLITE_ECEL_DUP flag prevents the arguments from being
/// filled using OP_SCopy.  OP_Copy must be used instead.
///
/// The SQLITE_ECEL_FACTOR argument allows constant arguments to be
/// factored out into initialization code.
///
/// The SQLITE_ECEL_REF flag means that expressions in the list with
/// ExprList.a[].u.x.iOrderByCol>0 have already been evaluated and stored
/// in registers at srcReg, and so the value can be copied from there.
/// If SQLITE_ECEL_OMITREF is also set, then the values with u.x.iOrderByCol>0
/// are simply omitted rather than being copied from srcReg.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pList` - The expression list to be coded
/// * `target` - Where to write results
/// * `srcReg` - Source registers if SQLITE_ECEL_REF
/// * `flags` - SQLITE_ECEL_* flags
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCodeExprList(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut target: i32,
    mut srcReg: i32,
    mut flags: u8,
) -> i32 {
    let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut n: i32 = 0 as i32;
    let mut copyOp: u8 = ((if ((flags as u32) as i32) & (1 as i32) != (0 as i32) {
        82 as i32
    } else {
        83 as i32
    }) as i8) as u8;
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    0 as i32;
    0 as i32;
    0 as i32; // Never gets this far otherwise
    n = unsafe { (*pList).nExpr };
    if !(((unsafe { (*pParse).__slate_bits_0.__get_okConstFactor() }) as i32) != (0 as i32)) {
        let __v2475: u8 = flags;
        let __v2476: u8 = ((((__v2475 as u32) as i32) & !(2 as i32)) as i8) as u8;
        flags = __v2476;
    }
    pItem = unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item };
    i = 0 as i32;
    '__slate_break_2345: while i < n {
        let mut pExpr: *mut Expr = unsafe { (*pItem).pExpr };
        let __v2481: bool;
        if ((flags as u32) as i32) & (4 as i32) != (0 as i32) {
            let __v2482: i32 = ((unsafe { (*pItem).u.x.iOrderByCol }) as u32) as i32;
            j = __v2482;
            __v2481 = __v2482 > (0 as i32);
        } else {
            __v2481 = false as bool;
        }
        if __v2481 {
            if ((flags as u32) as i32) & (8 as i32) != (0 as i32) {
                let __v2483: i32 = i;
                let __v2484: i32 = __v2483 - (1 as i32);
                i = __v2484;
                let __v2485: i32 = n;
                let __v2486: i32 = __v2485 - (1 as i32);
                n = __v2486;
            } else {
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        (copyOp as u32) as i32,
                        j + srcReg - (1 as i32),
                        target + i,
                    )
                };
            }
        } else {
            let __v2487: bool;
            if ((flags as u32) as i32) & (2 as i32) != (0 as i32) {
                __v2487 = sqlite3ExprIsConstantNotJoin(pParse, pExpr) != (0 as i32);
            } else {
                __v2487 = false as bool;
            }
            if __v2487 {
                sqlite3ExprCodeRunJustOnce(pParse, pExpr, target + i);
            } else {
                let mut inReg: i32 = sqlite3ExprCodeTarget(pParse, pExpr, target + i);
                if inReg != target + i {
                    let mut pOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
                    let __v2488: bool;
                    if ((copyOp as u32) as i32) == (82 as i32) {
                        let __v2489: *mut VdbeOp = unsafe { sqlite3VdbeGetLastOp(v) };
                        pOp = __v2489;
                        __v2488 = (((unsafe { (*__v2489).opcode }) as u32) as i32) == (82 as i32);
                    } else {
                        __v2488 = false as bool;
                    }
                    if __v2488
                        && (unsafe { (*pOp).p1 }) + unsafe { (*pOp).p3 } + (1 as i32) == inReg
                        && (unsafe { (*pOp).p2 }) + unsafe { (*pOp).p3 } + (1 as i32) == target + i
                        && (((unsafe { (*pOp).p5 }) as u32) as i32) == (0 as i32)
                    {
                        let __v2490: *mut VdbeOp = pOp;
                        let __v2491: i32 = unsafe { (*__v2490).p3 };
                        let __v2492: i32 = __v2491 + (1 as i32);
                        unsafe {
                            (*__v2490).p3 = __v2492;
                        }
                    } else {
                        unsafe { sqlite3VdbeAddOp2(v, (copyOp as u32) as i32, inReg, target + i) };
                    }
                    // The do-not-merge flag must be clear
                }
            }
        }
        let __v2477: i32 = i;
        let __v2478: i32 = __v2477 + (1 as i32);
        i = __v2478;
        let __v2479: *mut ExprList_item = pItem;
        let __v2480: *mut ExprList_item = unsafe { __v2479.offset((1 as i32) as isize) };
        pItem = __v2480;
    }
    return n;
}

/// Generate code for a BETWEEN operator.
///
///    x BETWEEN y AND z
///
/// The above is equivalent to
///
///    x>=y AND x<=z
///
/// Code it as such, taking care to do the common subexpression
/// elimination of x.
///
/// The xJumpIf parameter determines details:
///
///    NULL:                   Store the boolean result in reg[dest]
///    sqlite3ExprIfTrue:      Jump to dest if true
///    sqlite3ExprIfFalse:     Jump to dest if false
///
/// The jumpIfNull parameter is ignored if xJumpIf is NULL.
///
/// # Arguments
///
/// * `pParse` - Parsing and code generating context
/// * `pExpr` - The BETWEEN expression
/// * `dest` - Jump destination or storage location
/// * `xJump` - Action to take
/// * `jumpIfNull` - Take the jump if the BETWEEN is NULL
fn exprCodeBetween(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut dest: i32,
    mut xJump: Option<unsafe extern "C-unwind" fn(*mut Parse, *mut Expr, i32, i32)>,
    mut jumpIfNull: i32,
) {
    let mut exprAnd: Expr = unsafe { std::mem::zeroed() }; // The AND operator in  x>=y AND x<=z
    let mut compLeft: Expr = unsafe { std::mem::zeroed() }; // The  x>=y  term
    let mut compRight: Expr = unsafe { std::mem::zeroed() }; // The  x<=z  term
    let mut regFree1: i32 = 0 as i32; // Temporary use register
    let mut pDel: *mut Expr = std::ptr::null_mut::<Expr>();
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    unsafe {
        memset(
            std::ptr::addr_of_mut!(compLeft) as *mut (),
            0 as i32,
            72 as u64,
        )
    };
    unsafe {
        memset(
            std::ptr::addr_of_mut!(compRight) as *mut (),
            0 as i32,
            72 as u64,
        )
    };
    unsafe {
        memset(
            std::ptr::addr_of_mut!(exprAnd) as *mut (),
            0 as i32,
            72 as u64,
        )
    };
    0 as i32;
    pDel = sqlite3ExprDup(db, (unsafe { (*pExpr).pLeft }) as *const Expr, 0 as i32);
    if (((unsafe { (*db).mallocFailed }) as u32) as i32) == (0 as i32) {
        exprAnd.op = ((44 as i32) as i8) as u8;
        exprAnd.pLeft = std::ptr::addr_of_mut!(compLeft);
        exprAnd.pRight = std::ptr::addr_of_mut!(compRight);
        compLeft.op = ((58 as i32) as i8) as u8;
        compLeft.pLeft = pDel;
        compLeft.pRight = unsafe {
            (*unsafe {
                unsafe {
                    std::ptr::addr_of_mut!((*unsafe { (*pExpr).x.pList }).a) as *mut ExprList_item
                }
                .offset((0 as i32) as isize)
            })
            .pExpr
        };
        compRight.op = ((56 as i32) as i8) as u8;
        compRight.pLeft = pDel;
        compRight.pRight = unsafe {
            (*unsafe {
                unsafe {
                    std::ptr::addr_of_mut!((*unsafe { (*pExpr).x.pList }).a) as *mut ExprList_item
                }
                .offset((1 as i32) as isize)
            })
            .pExpr
        };
        sqlite3ExprToRegister(
            pDel,
            exprCodeVector(pParse, pDel, std::ptr::addr_of_mut!(regFree1)),
        );
        if xJump != None {
            unsafe { xJump.unwrap()(pParse, std::ptr::addr_of_mut!(exprAnd), dest, jumpIfNull) };
        } else {
            // Mark the expression is being from the ON or USING clause of a join
            // so that the sqlite3ExprCodeTarget() routine will not attempt to move
            // it into the Parse.pConstExpr list.  We should use a new bit for this,
            // for clarity, but we are out of bits in the Expr.flags field so we
            // have to reuse the EP_OuterON bit.  Bummer.
            let __v2622: *mut Expr = pDel;
            let __v2623: u32 = unsafe { (*__v2622).flags };
            let __v2624: u32 = __v2623 | ((1 as i32) as u32);
            unsafe {
                (*__v2622).flags = __v2624;
            }
            sqlite3ExprCodeTarget(pParse, std::ptr::addr_of_mut!(exprAnd), dest);
        }
        sqlite3ReleaseTempReg(pParse, regFree1);
    }
    sqlite3ExprDelete(db, pDel);
    // Ensure adequate test coverage
    {}
    {}
    {}
    {}
    {}
    {}
    {}
    {}
    {}
}

/// Generate code for a boolean expression such that a jump is made
/// to the label "dest" if the expression is true but execution
/// continues straight thru if the expression is false.
///
/// If the expression evaluates to NULL (neither true nor false), then
/// take the jump if the jumpIfNull flag is SQLITE_JUMPIFNULL.
///
/// This code depends on the fact that certain token values (ex: TK_EQ)
/// are the same as opcode values (ex: OP_Eq) that implement the corresponding
/// operation.  Special comments in vdbe.c and the mkopcodeh.awk script in
/// the make process cause these values to align.  Assert()s in the code
/// below verify that the numbers are aligned correctly.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.expr.sqlite3ExprIfTrue")]
extern "C-unwind" fn sqlite3ExprIfTrue(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut dest: i32,
    mut jumpIfNull: i32,
) {
    let mut __slate_storage_1453: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1453: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1453) as *mut i32;
    let mut __slate_storage_1452: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1452: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1452) as *mut i32;
    let mut __slate_storage_1451: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1451: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1451) as *mut i32; // IS TRUE or IS NOT TRUE
    let mut __slate_storage_1450: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1450: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1450) as *mut i32; // IS NOT TRUE or IS NOT FALSE
    let mut __slate_storage_1449: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1449: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1449) as *mut i32;
    let mut __slate_storage_1448: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1448: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1448) as *mut i32;
    let mut __slate_storage_1447: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1447: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1447) as *mut *mut Expr;
    let mut __slate_storage_1446: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1446: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1446) as *mut *mut Expr;
    let mut __slate_storage_1445: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1445: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1445) as *mut *mut Expr;
    let mut __slate_storage_1444: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1444: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1444) as *mut i32;
    let mut __slate_storage_1443: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1443: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1443) as *mut i32;
    let mut __slate_storage_1442: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1442: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1442) as *mut i32;
    let mut __slate_storage_1441: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1441: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1441) as *mut i32;
    let mut __slate_storage_1440: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1440: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1440) as *mut i32;
    let mut __slate_storage_1439: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1439: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1439) as *mut *mut Vdbe;
    unsafe {
        std::ptr::write(__slate_slot_1439, unsafe { (*pParse).pVdbe });
        std::ptr::write(__slate_slot_1440, 0 as i32);
        std::ptr::write(__slate_slot_1441, 0 as i32);
        std::ptr::write(__slate_slot_1442, 0 as i32);
        0 as i32;
        if *__slate_slot_1439 == std::ptr::null_mut::<Vdbe>() {
            return;
        } else {
            // Existence of VDBE checked by caller
            if pExpr == std::ptr::null_mut::<Expr>() {
                return;
            } else {
                '__join_0: {
                    '__join_31: {
                        '__join_5: {
                            '__join_18: {
                                '__join_19: {
                                    // No way this can happen
                                    0 as i32;
                                    *__slate_slot_1440 = ((unsafe { (*pExpr).op }) as u32) as i32;
                                    let __t0: i32 = *__slate_slot_1440;
                                    if __t0 == (44 as i32) {
                                        break '__join_31;
                                    } else {
                                        if __t0 == (43 as i32) {
                                            break '__join_31;
                                        } else {
                                            if __t0 == (19 as i32) {
                                                {}
                                                sqlite3ExprIfFalse(
                                                    pParse,
                                                    unsafe { (*pExpr).pLeft },
                                                    dest,
                                                    jumpIfNull,
                                                );
                                                break '__join_0;
                                            } else {
                                                if __t0 == (175 as i32) {
                                                    {}
                                                    *__slate_slot_1449 =
                                                        ((((unsafe { (*pExpr).op2 }) as u32)
                                                            as i32)
                                                            == (46 as i32))
                                                            as i32;
                                                    *__slate_slot_1450 = sqlite3ExprTruthValue(
                                                        (unsafe { (*pExpr).pRight }) as *const Expr,
                                                    );
                                                    {}
                                                    {}
                                                    if *__slate_slot_1450 ^ *__slate_slot_1449
                                                        != (0 as i32)
                                                    {
                                                        sqlite3ExprIfTrue(
                                                            pParse,
                                                            unsafe { (*pExpr).pLeft },
                                                            dest,
                                                            if *__slate_slot_1449 != (0 as i32) {
                                                                16 as i32
                                                            } else {
                                                                0 as i32
                                                            },
                                                        );
                                                        break '__join_0;
                                                    } else {
                                                        sqlite3ExprIfFalse(
                                                            pParse,
                                                            unsafe { (*pExpr).pLeft },
                                                            dest,
                                                            if *__slate_slot_1449 != (0 as i32) {
                                                                16 as i32
                                                            } else {
                                                                0 as i32
                                                            },
                                                        );
                                                        break '__join_0;
                                                    }
                                                } else {
                                                    if __t0 == (45 as i32) {
                                                        break '__join_19;
                                                    } else {
                                                        if __t0 == (46 as i32) {
                                                            break '__join_19;
                                                        } else {
                                                            if __t0 == (57 as i32) {
                                                                break '__join_18;
                                                            } else {
                                                                if __t0 == (56 as i32) {
                                                                    break '__join_18;
                                                                } else {
                                                                    if __t0 == (55 as i32) {
                                                                        break '__join_18;
                                                                    } else {
                                                                        if __t0 == (58 as i32) {
                                                                            break '__join_18;
                                                                        } else {
                                                                            if __t0 == (53 as i32) {
                                                                                break '__join_18;
                                                                            } else {
                                                                                if __t0
                                                                                    == (54 as i32)
                                                                                {
                                                                                    break '__join_18;
                                                                                } else {
                                                                                    if __t0
                                                                                        == (51
                                                                                            as i32)
                                                                                    {
                                                                                    } else {
                                                                                        if __t0 == (52 as i32) {
} else {
if __t0 == (49 as i32) {
{
}
exprCodeBetween(pParse, pExpr, dest, Some(sqlite3ExprIfTrue), jumpIfNull);
break '__join_0;
} else {
if __t0 == (50 as i32) {
std::ptr::write(__slate_slot_1452, unsafe { sqlite3VdbeMakeLabel(pParse) });
std::ptr::write(__slate_slot_1453, if jumpIfNull != (0 as i32) { dest } else { *__slate_slot_1452 });
sqlite3ExprCodeIN(pParse, pExpr, *__slate_slot_1452, *__slate_slot_1453);
unsafe { sqlite3VdbeGoto(*__slate_slot_1439, dest) };
unsafe { sqlite3VdbeResolveLabel(*__slate_slot_1439, *__slate_slot_1452) };
break '__join_0;
} else {
break '__join_5;
}
}
}
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    0 as i32;
                                    {}
                                    0 as i32;
                                    {}
                                    *__slate_slot_1443 = sqlite3ExprCodeTemp(
                                        pParse,
                                        unsafe { (*pExpr).pLeft },
                                        std::ptr::addr_of_mut!(*__slate_slot_1441),
                                    );
                                    0 as i32;
                                    if *__slate_slot_1441 != (0 as i32) {
                                        unsafe {
                                            sqlite3VdbeTypeofColumn(
                                                *__slate_slot_1439,
                                                *__slate_slot_1443,
                                            )
                                        };
                                    }
                                    unsafe {
                                        sqlite3VdbeAddOp2(
                                            *__slate_slot_1439,
                                            *__slate_slot_1440,
                                            *__slate_slot_1443,
                                            dest,
                                        )
                                    };
                                    {}
                                    {}
                                    break '__join_0;
                                }
                                {}
                                {}
                                *__slate_slot_1440 = if *__slate_slot_1440 == (45 as i32) {
                                    54 as i32
                                } else {
                                    53 as i32
                                };
                                jumpIfNull = 128 as i32;
                                // no break
                                {}
                            }
                            if sqlite3ExprIsVector((unsafe { (*pExpr).pLeft }) as *const Expr)
                                != (0 as i32)
                            {
                            } else {
                                if (unsafe { (*pExpr).flags }) & ((4194304 as i32) as u32)
                                    != ((0 as i32) as u32)
                                    && jumpIfNull != (128 as i32)
                                {
                                    *__slate_slot_1451 = exprComputeOperands(
                                        pParse,
                                        pExpr,
                                        std::ptr::addr_of_mut!(*__slate_slot_1443),
                                        std::ptr::addr_of_mut!(*__slate_slot_1444),
                                        std::ptr::addr_of_mut!(*__slate_slot_1441),
                                        std::ptr::addr_of_mut!(*__slate_slot_1442),
                                    );
                                } else {
                                    *__slate_slot_1443 = sqlite3ExprCodeTemp(
                                        pParse,
                                        unsafe { (*pExpr).pLeft },
                                        std::ptr::addr_of_mut!(*__slate_slot_1441),
                                    );
                                    *__slate_slot_1444 = sqlite3ExprCodeTemp(
                                        pParse,
                                        unsafe { (*pExpr).pRight },
                                        std::ptr::addr_of_mut!(*__slate_slot_1442),
                                    );
                                    *__slate_slot_1451 = 0 as i32;
                                }
                                codeCompare(
                                    pParse,
                                    unsafe { (*pExpr).pLeft },
                                    unsafe { (*pExpr).pRight },
                                    *__slate_slot_1440,
                                    *__slate_slot_1443,
                                    *__slate_slot_1444,
                                    dest,
                                    jumpIfNull,
                                    ((unsafe { (*pExpr).flags }) & ((1024 as i32) as u32)
                                        != ((0 as i32) as u32))
                                        as i32,
                                );
                                0 as i32;
                                {}
                                {}
                                0 as i32;
                                {}
                                {}
                                0 as i32;
                                {}
                                {}
                                0 as i32;
                                {}
                                {}
                                0 as i32;
                                {}
                                {}
                                {}
                                0 as i32;
                                {}
                                {}
                                {}
                                {}
                                {}
                                if *__slate_slot_1451 != (0 as i32) {
                                    if jumpIfNull != (0 as i32) {
                                        unsafe {
                                            sqlite3VdbeChangeP2(
                                                *__slate_slot_1439,
                                                *__slate_slot_1451,
                                                dest,
                                            )
                                        };
                                        break '__join_0;
                                    } else {
                                        unsafe {
                                            sqlite3VdbeJumpHere(
                                                *__slate_slot_1439,
                                                *__slate_slot_1451,
                                            )
                                        };
                                        break '__join_0;
                                    }
                                } else {
                                    break '__join_0;
                                }
                            }
                        }
                        if (unsafe { (*pExpr).flags }) & (((1 as i32) | (268435456 as i32)) as u32)
                            == ((268435456 as i32) as u32)
                        {
                            unsafe { sqlite3VdbeGoto(*__slate_slot_1439, dest) };
                            break '__join_0;
                        } else {
                            if (unsafe { (*pExpr).flags })
                                & (((1 as i32) | (536870912 as i32)) as u32)
                                == ((536870912 as i32) as u32)
                            {
                                // No-op
                                break '__join_0;
                            } else {
                                *__slate_slot_1443 = sqlite3ExprCodeTemp(
                                    pParse,
                                    pExpr,
                                    std::ptr::addr_of_mut!(*__slate_slot_1441),
                                );
                                unsafe {
                                    sqlite3VdbeAddOp3(
                                        *__slate_slot_1439,
                                        16 as i32,
                                        *__slate_slot_1443,
                                        dest,
                                        (jumpIfNull != (0 as i32)) as i32,
                                    )
                                };
                                {}
                                {}
                                {}
                                break '__join_0;
                            }
                        }
                    }
                    std::ptr::write(__slate_slot_1445, sqlite3ExprSimplifiedAndOr(pExpr));
                    if *__slate_slot_1445 != pExpr {
                        sqlite3ExprIfTrue(pParse, *__slate_slot_1445, dest, jumpIfNull);
                    } else {
                        if exprEvalRhsFirst(pExpr) != (0 as i32) {
                            *__slate_slot_1446 = unsafe { (*pExpr).pRight };
                            *__slate_slot_1447 = unsafe { (*pExpr).pLeft };
                        } else {
                            *__slate_slot_1446 = unsafe { (*pExpr).pLeft };
                            *__slate_slot_1447 = unsafe { (*pExpr).pRight };
                        }
                        if *__slate_slot_1440 == (44 as i32) {
                            std::ptr::write(__slate_slot_1448, unsafe {
                                sqlite3VdbeMakeLabel(pParse)
                            });
                            {}
                            sqlite3ExprIfFalse(
                                pParse,
                                *__slate_slot_1446,
                                *__slate_slot_1448,
                                jumpIfNull ^ (16 as i32),
                            );
                            sqlite3ExprIfTrue(pParse, *__slate_slot_1447, dest, jumpIfNull);
                            unsafe {
                                sqlite3VdbeResolveLabel(*__slate_slot_1439, *__slate_slot_1448)
                            };
                        } else {
                            {}
                            sqlite3ExprIfTrue(pParse, *__slate_slot_1446, dest, jumpIfNull);
                            sqlite3ExprIfTrue(pParse, *__slate_slot_1447, dest, jumpIfNull);
                        }
                    }
                }
                sqlite3ReleaseTempReg(pParse, *__slate_slot_1441);
                sqlite3ReleaseTempReg(pParse, *__slate_slot_1442);
            }
        }
    }
}

/// Generate code for a boolean expression such that a jump is made
/// to the label "dest" if the expression is false but execution
/// continues straight thru if the expression is true.
///
/// If the expression evaluates to NULL (neither true nor false) then
/// jump if jumpIfNull is SQLITE_JUMPIFNULL or fall through if jumpIfNull
/// is 0.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.expr.sqlite3ExprIfFalse")]
extern "C-unwind" fn sqlite3ExprIfFalse(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut dest: i32,
    mut jumpIfNull: i32,
) {
    let mut __slate_storage_1472: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1472: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1472) as *mut i32;
    let mut __slate_storage_1471: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1471: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1471) as *mut i32; // IS TRUE or IS NOT TRUE
    let mut __slate_storage_1470: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1470: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1470) as *mut i32; // IS NOT TRUE or IS NOT FALSE
    let mut __slate_storage_1469: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1469: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1469) as *mut i32;
    let mut __slate_storage_1468: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1468: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1468) as *mut i32;
    let mut __slate_storage_1467: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1467: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1467) as *mut *mut Expr;
    let mut __slate_storage_1466: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1466: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1466) as *mut *mut Expr;
    let mut __slate_storage_1465: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1465: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1465) as *mut *mut Expr;
    let mut __slate_storage_1464: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1464: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1464) as *mut i32;
    let mut __slate_storage_1463: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1463: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1463) as *mut i32;
    let mut __slate_storage_1462: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1462: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1462) as *mut i32;
    let mut __slate_storage_1461: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1461: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1461) as *mut i32;
    let mut __slate_storage_1460: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1460: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1460) as *mut i32;
    let mut __slate_storage_1459: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1459: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1459) as *mut *mut Vdbe;
    unsafe {
        std::ptr::write(__slate_slot_1459, unsafe { (*pParse).pVdbe });
        std::ptr::write(__slate_slot_1460, 0 as i32);
        std::ptr::write(__slate_slot_1461, 0 as i32);
        std::ptr::write(__slate_slot_1462, 0 as i32);
        0 as i32;
        if *__slate_slot_1459 == std::ptr::null_mut::<Vdbe>() {
            return;
        } else {
            // Existence of VDBE checked by caller
            if pExpr == std::ptr::null_mut::<Expr>() {
                return;
            } else {
                '__join_0: {
                    '__join_33: {
                        '__join_5: {
                            '__join_20: {
                                '__join_21: {
                                    0 as i32;
                                    // The value of pExpr->op and op are related as follows:
                                    //
                                    //       pExpr->op            op
                                    //       ---------          ----------
                                    //       TK_ISNULL          OP_NotNull
                                    //       TK_NOTNULL         OP_IsNull
                                    //       TK_NE              OP_Eq
                                    //       TK_EQ              OP_Ne
                                    //       TK_GT              OP_Le
                                    //       TK_LE              OP_Gt
                                    //       TK_GE              OP_Lt
                                    //       TK_LT              OP_Ge
                                    //
                                    // For other values of pExpr->op, op is undefined and unused.
                                    // The value of TK_ and OP_ constants are arranged such that we
                                    // can compute the mapping above using the following expression.
                                    // Assert()s verify that the computation is correct.
                                    *__slate_slot_1460 = ((((unsafe { (*pExpr).op }) as u32)
                                        as i32)
                                        + ((51 as i32) & (1 as i32))
                                        ^ (1 as i32))
                                        - ((51 as i32) & (1 as i32));
                                    // Verify correct alignment of TK_ and OP_ constants
                                    0 as i32;
                                    0 as i32;
                                    0 as i32;
                                    0 as i32;
                                    0 as i32;
                                    0 as i32;
                                    0 as i32;
                                    0 as i32;
                                    let __t0: i32 = ((unsafe { (*pExpr).op }) as u32) as i32;
                                    if __t0 == (44 as i32) {
                                        break '__join_33;
                                    } else {
                                        if __t0 == (43 as i32) {
                                            break '__join_33;
                                        } else {
                                            if __t0 == (19 as i32) {
                                                {}
                                                sqlite3ExprIfTrue(
                                                    pParse,
                                                    unsafe { (*pExpr).pLeft },
                                                    dest,
                                                    jumpIfNull,
                                                );
                                                break '__join_0;
                                            } else {
                                                if __t0 == (175 as i32) {
                                                    {}
                                                    *__slate_slot_1469 =
                                                        ((((unsafe { (*pExpr).op2 }) as u32)
                                                            as i32)
                                                            == (46 as i32))
                                                            as i32;
                                                    *__slate_slot_1470 = sqlite3ExprTruthValue(
                                                        (unsafe { (*pExpr).pRight }) as *const Expr,
                                                    );
                                                    {}
                                                    {}
                                                    if *__slate_slot_1470 ^ *__slate_slot_1469
                                                        != (0 as i32)
                                                    {
                                                        // IS TRUE and IS NOT FALSE
                                                        sqlite3ExprIfFalse(
                                                            pParse,
                                                            unsafe { (*pExpr).pLeft },
                                                            dest,
                                                            if *__slate_slot_1469 != (0 as i32) {
                                                                0 as i32
                                                            } else {
                                                                16 as i32
                                                            },
                                                        );
                                                        break '__join_0;
                                                    } else {
                                                        // IS FALSE and IS NOT TRUE
                                                        sqlite3ExprIfTrue(
                                                            pParse,
                                                            unsafe { (*pExpr).pLeft },
                                                            dest,
                                                            if *__slate_slot_1469 != (0 as i32) {
                                                                0 as i32
                                                            } else {
                                                                16 as i32
                                                            },
                                                        );
                                                        break '__join_0;
                                                    }
                                                } else {
                                                    if __t0 == (45 as i32) {
                                                        break '__join_21;
                                                    } else {
                                                        if __t0 == (46 as i32) {
                                                            break '__join_21;
                                                        } else {
                                                            if __t0 == (57 as i32) {
                                                                break '__join_20;
                                                            } else {
                                                                if __t0 == (56 as i32) {
                                                                    break '__join_20;
                                                                } else {
                                                                    if __t0 == (55 as i32) {
                                                                        break '__join_20;
                                                                    } else {
                                                                        if __t0 == (58 as i32) {
                                                                            break '__join_20;
                                                                        } else {
                                                                            if __t0 == (53 as i32) {
                                                                                break '__join_20;
                                                                            } else {
                                                                                if __t0
                                                                                    == (54 as i32)
                                                                                {
                                                                                    break '__join_20;
                                                                                } else {
                                                                                    if __t0
                                                                                        == (51
                                                                                            as i32)
                                                                                    {
                                                                                    } else {
                                                                                        if __t0 == (52 as i32) {
} else {
if __t0 == (49 as i32) {
{
}
exprCodeBetween(pParse, pExpr, dest, Some(sqlite3ExprIfFalse), jumpIfNull);
break '__join_0;
} else {
if __t0 == (50 as i32) {
if jumpIfNull != (0 as i32) {
sqlite3ExprCodeIN(pParse, pExpr, dest, dest);
break '__join_0;
} else {
std::ptr::write(__slate_slot_1472, unsafe { sqlite3VdbeMakeLabel(pParse) });
sqlite3ExprCodeIN(pParse, pExpr, dest, *__slate_slot_1472);
unsafe { sqlite3VdbeResolveLabel(*__slate_slot_1459, *__slate_slot_1472) };
break '__join_0;
}
} else {
break '__join_5;
}
}
}
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    *__slate_slot_1463 = sqlite3ExprCodeTemp(
                                        pParse,
                                        unsafe { (*pExpr).pLeft },
                                        std::ptr::addr_of_mut!(*__slate_slot_1461),
                                    );
                                    0 as i32;
                                    if *__slate_slot_1461 != (0 as i32) {
                                        unsafe {
                                            sqlite3VdbeTypeofColumn(
                                                *__slate_slot_1459,
                                                *__slate_slot_1463,
                                            )
                                        };
                                    }
                                    unsafe {
                                        sqlite3VdbeAddOp2(
                                            *__slate_slot_1459,
                                            *__slate_slot_1460,
                                            *__slate_slot_1463,
                                            dest,
                                        )
                                    };
                                    {}
                                    {}
                                    {}
                                    {}
                                    break '__join_0;
                                }
                                {}
                                {}
                                *__slate_slot_1460 =
                                    if (((unsafe { (*pExpr).op }) as u32) as i32) == (45 as i32) {
                                        53 as i32
                                    } else {
                                        54 as i32
                                    };
                                jumpIfNull = 128 as i32;
                                // no break
                                {}
                            }
                            if sqlite3ExprIsVector((unsafe { (*pExpr).pLeft }) as *const Expr)
                                != (0 as i32)
                            {
                            } else {
                                if (unsafe { (*pExpr).flags }) & ((4194304 as i32) as u32)
                                    != ((0 as i32) as u32)
                                    && jumpIfNull != (128 as i32)
                                {
                                    *__slate_slot_1471 = exprComputeOperands(
                                        pParse,
                                        pExpr,
                                        std::ptr::addr_of_mut!(*__slate_slot_1463),
                                        std::ptr::addr_of_mut!(*__slate_slot_1464),
                                        std::ptr::addr_of_mut!(*__slate_slot_1461),
                                        std::ptr::addr_of_mut!(*__slate_slot_1462),
                                    );
                                } else {
                                    *__slate_slot_1463 = sqlite3ExprCodeTemp(
                                        pParse,
                                        unsafe { (*pExpr).pLeft },
                                        std::ptr::addr_of_mut!(*__slate_slot_1461),
                                    );
                                    *__slate_slot_1464 = sqlite3ExprCodeTemp(
                                        pParse,
                                        unsafe { (*pExpr).pRight },
                                        std::ptr::addr_of_mut!(*__slate_slot_1462),
                                    );
                                    *__slate_slot_1471 = 0 as i32;
                                }
                                codeCompare(
                                    pParse,
                                    unsafe { (*pExpr).pLeft },
                                    unsafe { (*pExpr).pRight },
                                    *__slate_slot_1460,
                                    *__slate_slot_1463,
                                    *__slate_slot_1464,
                                    dest,
                                    jumpIfNull,
                                    ((unsafe { (*pExpr).flags }) & ((1024 as i32) as u32)
                                        != ((0 as i32) as u32))
                                        as i32,
                                );
                                0 as i32;
                                {}
                                {}
                                0 as i32;
                                {}
                                {}
                                0 as i32;
                                {}
                                {}
                                0 as i32;
                                {}
                                {}
                                0 as i32;
                                {}
                                {}
                                {}
                                0 as i32;
                                {}
                                {}
                                {}
                                {}
                                {}
                                if *__slate_slot_1471 != (0 as i32) {
                                    if jumpIfNull != (0 as i32) {
                                        unsafe {
                                            sqlite3VdbeChangeP2(
                                                *__slate_slot_1459,
                                                *__slate_slot_1471,
                                                dest,
                                            )
                                        };
                                        break '__join_0;
                                    } else {
                                        unsafe {
                                            sqlite3VdbeJumpHere(
                                                *__slate_slot_1459,
                                                *__slate_slot_1471,
                                            )
                                        };
                                        break '__join_0;
                                    }
                                } else {
                                    break '__join_0;
                                }
                            }
                        }
                        if (unsafe { (*pExpr).flags }) & (((1 as i32) | (536870912 as i32)) as u32)
                            == ((536870912 as i32) as u32)
                        {
                            unsafe { sqlite3VdbeGoto(*__slate_slot_1459, dest) };
                            break '__join_0;
                        } else {
                            if (unsafe { (*pExpr).flags })
                                & (((1 as i32) | (268435456 as i32)) as u32)
                                == ((268435456 as i32) as u32)
                            {
                                // no-op
                                break '__join_0;
                            } else {
                                *__slate_slot_1463 = sqlite3ExprCodeTemp(
                                    pParse,
                                    pExpr,
                                    std::ptr::addr_of_mut!(*__slate_slot_1461),
                                );
                                unsafe {
                                    sqlite3VdbeAddOp3(
                                        *__slate_slot_1459,
                                        17 as i32,
                                        *__slate_slot_1463,
                                        dest,
                                        (jumpIfNull != (0 as i32)) as i32,
                                    )
                                };
                                {}
                                {}
                                {}
                                break '__join_0;
                            }
                        }
                    }
                    std::ptr::write(__slate_slot_1465, sqlite3ExprSimplifiedAndOr(pExpr));
                    if *__slate_slot_1465 != pExpr {
                        sqlite3ExprIfFalse(pParse, *__slate_slot_1465, dest, jumpIfNull);
                    } else {
                        if exprEvalRhsFirst(pExpr) != (0 as i32) {
                            *__slate_slot_1466 = unsafe { (*pExpr).pRight };
                            *__slate_slot_1467 = unsafe { (*pExpr).pLeft };
                        } else {
                            *__slate_slot_1466 = unsafe { (*pExpr).pLeft };
                            *__slate_slot_1467 = unsafe { (*pExpr).pRight };
                        }
                        if (((unsafe { (*pExpr).op }) as u32) as i32) == (44 as i32) {
                            {}
                            sqlite3ExprIfFalse(pParse, *__slate_slot_1466, dest, jumpIfNull);
                            sqlite3ExprIfFalse(pParse, *__slate_slot_1467, dest, jumpIfNull);
                        } else {
                            std::ptr::write(__slate_slot_1468, unsafe {
                                sqlite3VdbeMakeLabel(pParse)
                            });
                            {}
                            sqlite3ExprIfTrue(
                                pParse,
                                *__slate_slot_1466,
                                *__slate_slot_1468,
                                jumpIfNull ^ (16 as i32),
                            );
                            sqlite3ExprIfFalse(pParse, *__slate_slot_1467, dest, jumpIfNull);
                            unsafe {
                                sqlite3VdbeResolveLabel(*__slate_slot_1459, *__slate_slot_1468)
                            };
                        }
                    }
                }
                sqlite3ReleaseTempReg(pParse, *__slate_slot_1461);
                sqlite3ReleaseTempReg(pParse, *__slate_slot_1462);
            }
        }
    }
}

/// Like sqlite3ExprIfFalse() except that a copy is made of pExpr before
/// code generation, and that copy is deleted after code generation. This
/// ensures that the original pExpr is unchanged.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprIfFalseDup(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut dest: i32,
    mut jumpIfNull: i32,
) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pCopy: *mut Expr = sqlite3ExprDup(db, pExpr as *const Expr, 0 as i32);
    if (((unsafe { (*db).mallocFailed }) as u32) as i32) == (0 as i32) {
        sqlite3ExprIfFalse(pParse, pCopy, dest, jumpIfNull);
    }
    sqlite3ExprDelete(db, pCopy);
}

/// Expression pVar is guaranteed to be an SQL variable. pExpr may be any
/// type of expression.
///
/// If pExpr is a simple SQL value - an integer, real, string, blob
/// or NULL value - then the VDBE currently being prepared is configured
/// to re-prepare each time a new value is bound to variable pVar.
///
/// Additionally, if pExpr is a simple SQL value and the value is the
/// same as that currently bound to variable pVar, non-zero is returned.
/// Otherwise, if the values are not the same or if pExpr is not a simple
/// SQL value, zero is returned.
///
/// If the SQLITE_EnableQPSG flag is set on the database connection, then
/// this routine always returns false.
fn exprCompareVariable(
    mut pParse: *const Parse,
    mut pVar: *const Expr,
    mut pExpr: *const Expr,
) -> i32 {
    let mut res: i32 = 2 as i32;
    let mut iVar: i32 = 0 as i32;
    let mut pL: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    let mut pR: *mut sqlite3_value = std::ptr::null_mut::<sqlite3_value>();
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (157 as i32)
        && ((unsafe { (*pVar).iColumn }) as i32) == ((unsafe { (*pExpr).iColumn }) as i32)
    {
        return 0 as i32;
    }
    if (unsafe { (*unsafe { (*pParse).db }).flags }) & (((8388608 as i32) as i64) as u64)
        != (((0 as i32) as i64) as u64)
    {
        return 2 as i32;
    }
    unsafe {
        sqlite3ValueFromExpr(
            unsafe { (*pParse).db },
            pExpr,
            ((1 as i32) as i8) as u8,
            ((65 as i32) as i8) as u8,
            std::ptr::addr_of_mut!(pR),
        )
    };
    if pR != std::ptr::null_mut::<sqlite3_value>() {
        iVar = (unsafe { (*pVar).iColumn }) as i32;
        unsafe { sqlite3VdbeReprepareOnBind(unsafe { (*pParse).pVdbe }, iVar, 0 as i32) };
        pL = unsafe {
            sqlite3VdbeGetBoundValue(
                unsafe { (*pParse).pReprepare },
                iVar,
                ((65 as i32) as i8) as u8,
            )
        };
        if pL != std::ptr::null_mut::<sqlite3_value>() {
            if (unsafe { sqlite3_value_type(pL) }) == (3 as i32) {
                unsafe { sqlite3_value_text(pL) }; // Make sure the encoding is UTF-8
            }
            res = if (unsafe {
                sqlite3MemCompare(
                    pL as *const sqlite3_value,
                    pR as *const sqlite3_value,
                    std::ptr::null::<CollSeq>(),
                )
            }) != (0 as i32)
            {
                2 as i32
            } else {
                0 as i32
            };
        }
        unsafe { sqlite3ValueFree(pR) };
        unsafe { sqlite3ValueFree(pL) };
    }
    return res;
}

/// Do a deep comparison of two expression trees.  Return 0 if the two
/// expressions are completely identical.  Return 1 if they differ only
/// by a COLLATE operator at the top level.  Return 2 if there are differences
/// other than the top-level COLLATE operator.
///
/// If any subelement of pB has Expr.iTable==(-1) then it is allowed
/// to compare equal to an equivalent element in pA with Expr.iTable==iTab.
///
/// The pA side might be using TK_REGISTER.  If that is the case and pB is
/// not using TK_REGISTER but is otherwise equivalent, then still return 0.
///
/// Sometimes this routine will return 2 even if the two expressions
/// really are equivalent.  If we cannot prove that the expressions are
/// identical, we return 2 just to be safe.  So if this routine
/// returns 2, then you do not really know for certain if the two
/// expressions are the same.  But if you get a 0 or 1 return, then you
/// can be sure the expressions are the same.  In the places where
/// this routine is used, it does not hurt to get an extra 2 - that
/// just might result in some slightly slower code.  But returning
/// an incorrect 0 or 1 could lead to a malfunction.
///
/// If pParse is not NULL and SQLITE_EnableQPSG is off then TK_VARIABLE
/// terms in pA with bindings in pParse->pReprepare can be matched against
/// literals in pB.  The pParse->pVdbe->expmask bitmask is updated for
/// each variable referenced.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCompare(
    mut pParse: *const Parse,
    mut pA: *const Expr,
    mut pB: *const Expr,
    mut iTab: i32,
) -> i32 {
    let mut combinedFlags: u32 = 0 as u32;
    if pA == std::ptr::null::<Expr>() || pB == std::ptr::null::<Expr>() {
        return if pB == pA { 0 as i32 } else { 2 as i32 };
    }
    if pParse != std::ptr::null::<Parse>()
        && (((unsafe { (*pA).op }) as u32) as i32) == (157 as i32)
    {
        return exprCompareVariable(pParse, pA, pB);
    }
    combinedFlags = (unsafe { (*pA).flags }) | unsafe { (*pB).flags };
    if combinedFlags & ((2048 as i32) as u32) != (0 as u32) {
        if (unsafe { (*pA).flags }) & unsafe { (*pB).flags } & ((2048 as i32) as u32)
            != ((0 as i32) as u32)
            && (unsafe { (*pA).u.iValue }) == unsafe { (*pB).u.iValue }
        {
            return 0 as i32;
        }
        return 2 as i32;
    }
    if (((unsafe { (*pA).op }) as u32) as i32) != (((unsafe { (*pB).op }) as u32) as i32)
        || (((unsafe { (*pA).op }) as u32) as i32) == (72 as i32)
    {
        let __v2493: bool;
        if (((unsafe { (*pA).op }) as u32) as i32) == (114 as i32) {
            __v2493 = sqlite3ExprCompare(pParse, (unsafe { (*pA).pLeft }) as *const Expr, pB, iTab)
                < (2 as i32);
        } else {
            __v2493 = false as bool;
        }
        if __v2493 {
            return 1 as i32;
        }
        let __v2494: bool;
        if (((unsafe { (*pB).op }) as u32) as i32) == (114 as i32) {
            __v2494 = sqlite3ExprCompare(pParse, pA, (unsafe { (*pB).pLeft }) as *const Expr, iTab)
                < (2 as i32);
        } else {
            __v2494 = false as bool;
        }
        if __v2494 {
            return 1 as i32;
        }
        if (((unsafe { (*pA).op }) as u32) as i32) == (170 as i32)
            && (((unsafe { (*pB).op }) as u32) as i32) == (168 as i32)
            && (unsafe { (*pB).iTable }) < (0 as i32)
            && (unsafe { (*pA).iTable }) == iTab
        {
            // fall through
        } else {
            return 2 as i32;
        }
    }
    0 as i32;
    0 as i32;
    if (unsafe { (*pA).u.zToken }) != std::ptr::null_mut::<i8>() {
        if (((unsafe { (*pA).op }) as u32) as i32) == (172 as i32)
            || (((unsafe { (*pA).op }) as u32) as i32) == (169 as i32)
        {
            if (unsafe {
                sqlite3StrICmp(
                    (unsafe { (*pA).u.zToken }) as *const i8,
                    (unsafe { (*pB).u.zToken }) as *const i8,
                )
            }) != (0 as i32)
            {
                return 2 as i32;
            }
            0 as i32;
            if (((unsafe { (*pA).flags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32))
                as i32)
                != (((unsafe { (*pB).flags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32))
                    as i32)
            {
                return 2 as i32;
            }
            if (unsafe { (*pA).flags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32) {
                if (unsafe {
                    sqlite3WindowCompare(
                        pParse,
                        (unsafe { (*pA).y.pWin }) as *const Window,
                        (unsafe { (*pB).y.pWin }) as *const Window,
                        1 as i32,
                    )
                }) != (0 as i32)
                {
                    return 2 as i32;
                }
            }
        } else {
            if (((unsafe { (*pA).op }) as u32) as i32) == (122 as i32) {
                return 0 as i32;
            } else {
                if (((unsafe { (*pA).op }) as u32) as i32) == (114 as i32) {
                    if (unsafe {
                        sqlite3_stricmp(
                            (unsafe { (*pA).u.zToken }) as *const i8,
                            (unsafe { (*pB).u.zToken }) as *const i8,
                        )
                    }) != (0 as i32)
                    {
                        return 2 as i32;
                    }
                } else {
                    if (unsafe { (*pB).u.zToken }) != std::ptr::null_mut::<i8>()
                        && (((unsafe { (*pA).op }) as u32) as i32) != (168 as i32)
                        && (((unsafe { (*pA).op }) as u32) as i32) != (170 as i32)
                        && (unsafe {
                            strcmp(
                                (unsafe { (*pA).u.zToken }) as *const i8,
                                (unsafe { (*pB).u.zToken }) as *const i8,
                            )
                        }) != (0 as i32)
                    {
                        return 2 as i32;
                    }
                }
            }
        }
    }
    if (unsafe { (*pA).flags }) & (((4 as i32) | (1024 as i32)) as u32)
        != (unsafe { (*pB).flags }) & (((4 as i32) | (1024 as i32)) as u32)
    {
        return 2 as i32;
    }
    if combinedFlags & ((65536 as i32) as u32) == ((0 as i32) as u32) {
        if combinedFlags & ((4096 as i32) as u32) != (0 as u32) {
            return 2 as i32;
        }
        let __v2495: bool;
        if combinedFlags & ((32 as i32) as u32) == ((0 as i32) as u32) {
            __v2495 = sqlite3ExprCompare(
                pParse,
                (unsafe { (*pA).pLeft }) as *const Expr,
                (unsafe { (*pB).pLeft }) as *const Expr,
                iTab,
            ) != (0 as i32);
        } else {
            __v2495 = false as bool;
        }
        if __v2495 {
            return 2 as i32;
        }
        if sqlite3ExprCompare(
            pParse,
            (unsafe { (*pA).pRight }) as *const Expr,
            (unsafe { (*pB).pRight }) as *const Expr,
            iTab,
        ) != (0 as i32)
        {
            return 2 as i32;
        }
        if sqlite3ExprListCompare(
            (unsafe { (*pA).x.pList }) as *const ExprList,
            (unsafe { (*pB).x.pList }) as *const ExprList,
            iTab,
        ) != (0 as i32)
        {
            return 2 as i32;
        }
        if (((unsafe { (*pA).op }) as u32) as i32) != (118 as i32)
            && (((unsafe { (*pA).op }) as u32) as i32) != (171 as i32)
            && combinedFlags & ((16384 as i32) as u32) == ((0 as i32) as u32)
        {
            if ((unsafe { (*pA).iColumn }) as i32) != ((unsafe { (*pB).iColumn }) as i32) {
                return 2 as i32;
            }
            if (((unsafe { (*pA).op2 }) as u32) as i32) != (((unsafe { (*pB).op2 }) as u32) as i32)
                && (((unsafe { (*pA).op }) as u32) as i32) == (175 as i32)
            {
                return 2 as i32;
            }
            if (((unsafe { (*pA).op }) as u32) as i32) != (50 as i32)
                && (unsafe { (*pA).iTable }) != unsafe { (*pB).iTable }
                && (unsafe { (*pA).iTable }) != iTab
            {
                return 2 as i32;
            }
        }
    }
    return 0 as i32;
}

/// Compare two ExprList objects.  Return 0 if they are identical, 1
/// if they are certainly different, or 2 if it is not possible to
/// determine if they are identical or not.
///
/// If any subelement of pB has Expr.iTable==(-1) then it is allowed
/// to compare equal to an equivalent element in pA with Expr.iTable==iTab.
///
/// This routine might return non-zero for equivalent ExprLists.  The
/// only consequence will be disabled optimizations.  But this routine
/// must never return 0 if the two ExprList objects are different, or
/// a malfunction will result.
///
/// Two NULL pointers are considered to be the same.  But a NULL pointer
/// always differs from a non-NULL pointer.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprListCompare(
    mut pA: *const ExprList,
    mut pB: *const ExprList,
    mut iTab: i32,
) -> i32 {
    let mut i: i32 = 0 as i32;
    if pA == std::ptr::null::<ExprList>() && pB == std::ptr::null::<ExprList>() {
        return 0 as i32;
    }
    if pA == std::ptr::null::<ExprList>() || pB == std::ptr::null::<ExprList>() {
        return 1 as i32;
    }
    if (unsafe { (*pA).nExpr }) != unsafe { (*pB).nExpr } {
        return 1 as i32;
    }
    i = 0 as i32;
    '__slate_break_2348: loop {
        if !(i < unsafe { (*pA).nExpr }) {
            break;
        }
        let mut res: i32 = 0 as i32;
        let mut pExprA: *mut Expr = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of!((*pA).a) as *const ExprList_item }.offset(i as isize)
            })
            .pExpr
        };
        let mut pExprB: *mut Expr = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of!((*pB).a) as *const ExprList_item }.offset(i as isize)
            })
            .pExpr
        };
        if (((unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of!((*pA).a) as *const ExprList_item }.offset(i as isize)
            })
            .fg
            .sortFlags
        }) as u32) as i32)
            != (((unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of!((*pB).a) as *const ExprList_item }
                        .offset(i as isize)
                })
                .fg
                .sortFlags
            }) as u32) as i32)
        {
            return 1 as i32;
        }
        let __v2498: i32 = sqlite3ExprCompare(
            std::ptr::null::<Parse>(),
            pExprA as *const Expr,
            pExprB as *const Expr,
            iTab,
        );
        res = __v2498;
        if __v2498 != (0 as i32) {
            return res;
        }
        let __v2496: i32 = i;
        let __v2497: i32 = __v2496 + (1 as i32);
        i = __v2497;
    }
    return 0 as i32;
}

/// Like sqlite3ExprCompare() except COLLATE operators at the top-level
/// are ignored.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCompareSkip(
    mut pA: *mut Expr,
    mut pB: *mut Expr,
    mut iTab: i32,
) -> i32 {
    return sqlite3ExprCompare(
        std::ptr::null::<Parse>(),
        sqlite3ExprSkipCollate(pA) as *const Expr,
        sqlite3ExprSkipCollate(pB) as *const Expr,
        iTab,
    );
}

/// Return non-zero if Expr p can only be true if pNN is not NULL.
///
/// Or if seenNot is true, return non-zero if Expr p can only be
/// non-NULL if pNN is not NULL
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `p` - The expression to be checked
/// * `pNN` - The expression that is NOT NULL
/// * `iTab` - Table being evaluated
/// * `seenNot` - Return true only if p can be any non-NULL value
fn exprImpliesNotNull(
    mut pParse: *const Parse,
    mut p: *const Expr,
    mut pNN: *const Expr,
    mut iTab: i32,
    mut seenNot: i32,
) -> i32 {
    0 as i32;
    0 as i32;
    if sqlite3ExprCompare(pParse, p, pNN, iTab) == (0 as i32) {
        return ((((unsafe { (*pNN).op }) as u32) as i32) != (122 as i32)) as i32;
    }
    // no break
    // no break
    match ((unsafe { (*p).op }) as u32) as i32 {
        50 => {
            if seenNot != (0 as i32)
                && (unsafe { (*p).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32)
            {
                return 0 as i32;
            }
            0 as i32;
            return exprImpliesNotNull(
                pParse,
                (unsafe { (*p).pLeft }) as *const Expr,
                pNN,
                iTab,
                1 as i32,
            );
        }
        49 => {
            let mut pList: *mut ExprList = unsafe { std::mem::zeroed() };
            0 as i32;
            pList = unsafe { (*p).x.pList };
            0 as i32;
            0 as i32;
            if seenNot != (0 as i32) {
                return 0 as i32;
            }
            let __v2704: bool;
            if exprImpliesNotNull(
                pParse,
                (unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                            .offset((0 as i32) as isize)
                    })
                    .pExpr
                }) as *const Expr,
                pNN,
                iTab,
                1 as i32,
            ) != (0 as i32)
            {
                __v2704 = true as bool;
            } else {
                __v2704 = exprImpliesNotNull(
                    pParse,
                    (unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                                .offset((1 as i32) as isize)
                        })
                        .pExpr
                    }) as *const Expr,
                    pNN,
                    iTab,
                    1 as i32,
                ) != (0 as i32);
            }
            if __v2704 {
                return 1 as i32;
            }
            return exprImpliesNotNull(
                pParse,
                (unsafe { (*p).pLeft }) as *const Expr,
                pNN,
                iTab,
                1 as i32,
            );
        }
        54 | 53 | 57 | 56 | 55 | 58 | 107 | 108 | 104 | 105 | 106 | 112 => {
            seenNot = 1 as i32;
            // no break
            {}
            if exprImpliesNotNull(
                pParse,
                (unsafe { (*p).pRight }) as *const Expr,
                pNN,
                iTab,
                seenNot,
            ) != (0 as i32)
            {
                return 1 as i32;
            }
            // no break
            {}
            return exprImpliesNotNull(
                pParse,
                (unsafe { (*p).pLeft }) as *const Expr,
                pNN,
                iTab,
                seenNot,
            );
        }
        109 | 111 | 103 | 110 => {
            if exprImpliesNotNull(
                pParse,
                (unsafe { (*p).pRight }) as *const Expr,
                pNN,
                iTab,
                seenNot,
            ) != (0 as i32)
            {
                return 1 as i32;
            }
            // no break
            {}
            return exprImpliesNotNull(
                pParse,
                (unsafe { (*p).pLeft }) as *const Expr,
                pNN,
                iTab,
                seenNot,
            );
        }
        181 | 114 | 173 | 174 => {
            return exprImpliesNotNull(
                pParse,
                (unsafe { (*p).pLeft }) as *const Expr,
                pNN,
                iTab,
                seenNot,
            );
        }
        175 => {
            if seenNot != (0 as i32) {
                return 0 as i32;
            }
            if (((unsafe { (*p).op2 }) as u32) as i32) != (45 as i32) {
                return 0 as i32;
            }
            return exprImpliesNotNull(
                pParse,
                (unsafe { (*p).pLeft }) as *const Expr,
                pNN,
                iTab,
                1 as i32,
            );
        }
        115 | 19 => {
            return exprImpliesNotNull(
                pParse,
                (unsafe { (*p).pLeft }) as *const Expr,
                pNN,
                iTab,
                1 as i32,
            );
        }
        _ => {}
    }
    return 0 as i32;
}

/// Return true if the boolean value of the expression is always either
/// FALSE or NULL.
fn sqlite3ExprIsNotTrue(mut pExpr: *mut Expr) -> i32 {
    let mut v: i32 = 0 as i32;
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (122 as i32) {
        return 1 as i32;
    }
    let __v2705: bool;
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (171 as i32) {
        __v2705 = sqlite3ExprTruthValue(pExpr as *const Expr) == (0 as i32);
    } else {
        __v2705 = false as bool;
    }
    if __v2705 {
        return 1 as i32;
    }
    v = 1 as i32;
    if sqlite3ExprIsInteger(
        pExpr as *const Expr,
        std::ptr::addr_of_mut!(v),
        std::ptr::null_mut::<Parse>(),
        0 as i32,
    ) != (0 as i32)
        && v == (0 as i32)
    {
        return 1 as i32;
    }
    return 0 as i32;
}

/// Return true if the expression is one of the following:
///
///    CASE WHEN x THEN y END
///    CASE WHEN x THEN y ELSE NULL END
///    CASE WHEN x THEN y ELSE false END
///    iif(x,y)
///    iif(x,y,NULL)
///    iif(x,y,false)
fn sqlite3ExprIsIIF(mut db: *mut sqlite3, mut pExpr: *const Expr) -> i32 {
    let mut pList: *mut ExprList = unsafe { std::mem::zeroed() };
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (172 as i32) {
        let mut z: *const i8 = (unsafe { (*pExpr).u.zToken }) as *const i8;
        let mut pDef: *mut FuncDef = unsafe { std::mem::zeroed() };
        if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) != (105 as i32)
            && ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) != (73 as i32)
        {
            return 0 as i32;
        }
        if (unsafe { (*pExpr).x.pList }) == std::ptr::null_mut::<ExprList>() {
            return 0 as i32;
        }
        pDef = unsafe {
            sqlite3FindFunction(
                db,
                z,
                unsafe { (*unsafe { (*pExpr).x.pList }).nExpr },
                unsafe { (*db).enc },
                ((0 as i32) as i8) as u8,
            )
        };
        if pDef == std::ptr::null_mut::<FuncDef>() {
            return 0 as i32;
        }
        if (unsafe { (*pDef).funcFlags }) & ((4194304 as i32) as u32) == ((0 as i32) as u32) {
            return 0 as i32;
        }
        if (((unsafe { (*pDef).pUserData }) as i64) as i32) != (5 as i32) {
            return 0 as i32;
        }
    } else {
        if (((unsafe { (*pExpr).op }) as u32) as i32) == (158 as i32) {
            if (unsafe { (*pExpr).pLeft }) != std::ptr::null_mut::<Expr>() {
                return 0 as i32;
            }
        } else {
            return 0 as i32;
        }
    }
    pList = unsafe { (*pExpr).x.pList };
    0 as i32;
    if (unsafe { (*pList).nExpr }) == (2 as i32) {
        return 1 as i32;
    }
    let __v2706: bool;
    if (unsafe { (*pList).nExpr }) == (3 as i32) {
        __v2706 = sqlite3ExprIsNotTrue(unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                    .offset((2 as i32) as isize)
            })
            .pExpr
        }) != (0 as i32);
    } else {
        __v2706 = false as bool;
    }
    if __v2706 {
        return 1 as i32;
    }
    return 0 as i32;
}

/// Return true if we can prove the pE2 will always be true if pE1 is
/// true.  Return false if we cannot complete the proof or if pE2 might
/// be false.  Examples:
///
///     pE1: x==5        pE2: x==5             Result: true
///     pE1: x>0         pE2: x==5             Result: false
///     pE1: x=21        pE2: x=21 OR y=43     Result: true
///     pE1: x!=123      pE2: x IS NOT NULL    Result: true
///     pE1: x!=?1       pE2: x IS NOT NULL    Result: true
///     pE1: x IS NULL   pE2: x IS NOT NULL    Result: false
///     pE1: x IS ?2     pE2: x IS NOT NULL    Result: false
///     pE1: iif(x,y)    pE2: x                Result: true
///     PE1: iif(x,y,0)  pE2: x                Result: true
///
/// When comparing TK_COLUMN nodes between pE1 and pE2, if pE2 has
/// Expr.iTable<0 then assume a table number given by iTab.
///
/// If pParse is not NULL, then the values of bound variables in pE1 are
/// compared against literal values in pE2 and pParse->pVdbe->expmask is
/// modified to record which bound variables are referenced.  If pParse
/// is NULL, then false will be returned if pE1 contains any bound variables.
///
/// When in doubt, return false.  Returning true might give a performance
/// improvement.  Returning false might cause a performance reduction, but
/// it will always give the correct answer and is hence always safe.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprImpliesExpr(
    mut pParse: *const Parse,
    mut pE1: *const Expr,
    mut pE2: *const Expr,
    mut iTab: i32,
) -> i32 {
    if sqlite3ExprCompare(pParse, pE1, pE2, iTab) == (0 as i32) {
        return 1 as i32;
    }
    let __v2499: bool;
    if (((unsafe { (*pE2).op }) as u32) as i32) == (43 as i32) {
        let __v2500: bool;
        if sqlite3ExprImpliesExpr(pParse, pE1, (unsafe { (*pE2).pLeft }) as *const Expr, iTab)
            != (0 as i32)
        {
            __v2500 = true as bool;
        } else {
            __v2500 = sqlite3ExprImpliesExpr(
                pParse,
                pE1,
                (unsafe { (*pE2).pRight }) as *const Expr,
                iTab,
            ) != (0 as i32);
        }
        __v2499 = __v2500;
    } else {
        __v2499 = false as bool;
    }
    if __v2499 {
        return 1 as i32;
    }
    let __v2501: bool;
    if (((unsafe { (*pE2).op }) as u32) as i32) == (52 as i32) {
        __v2501 = exprImpliesNotNull(
            pParse,
            pE1,
            (unsafe { (*pE2).pLeft }) as *const Expr,
            iTab,
            0 as i32,
        ) != (0 as i32);
    } else {
        __v2501 = false as bool;
    }
    if __v2501 {
        return 1 as i32;
    }
    if sqlite3ExprIsIIF(unsafe { (*pParse).db }, pE1) != (0 as i32) {
        return sqlite3ExprImpliesExpr(
            pParse,
            (unsafe {
                (*unsafe {
                    unsafe {
                        std::ptr::addr_of_mut!((*unsafe { (*pE1).x.pList }).a) as *mut ExprList_item
                    }
                    .offset((0 as i32) as isize)
                })
                .pExpr
            }) as *const Expr,
            pE2,
            iTab,
        );
    }
    return 0 as i32;
}

/// This is a helper function to impliesNotNullRow().  In this routine,
/// set pWalker->eCode to one only if *both* of the input expressions
/// separately have the implies-not-null-row property.
fn bothImplyNotNullRow(mut pWalker: *mut Walker, mut pE1: *mut Expr, mut pE2: *mut Expr) {
    if (((unsafe { (*pWalker).eCode }) as u32) as i32) == (0 as i32) {
        unsafe { sqlite3WalkExpr(pWalker, pE1) };
        if (unsafe { (*pWalker).eCode }) != (0 as u16) {
            unsafe {
                (*pWalker).eCode = ((0 as i32) as i16) as u16;
            }
            unsafe { sqlite3WalkExpr(pWalker, pE2) };
        }
    }
}

/// This is the Expr node callback for sqlite3ExprImpliesNonNullRow().
/// If the expression node requires that the table at pWalker->iCur
/// have one or more non-NULL column, then set pWalker->eCode to 1 and abort.
///
/// pWalker->mWFlags is non-zero if this inquiry is being undertaking on
/// behalf of a RIGHT JOIN (or FULL JOIN).  That makes a difference when
/// evaluating terms in the ON clause of an inner join.
///
/// This routine controls an optimization.  False positives (setting
/// pWalker->eCode to 1 when it should not be) are deadly, but false-negatives
/// (never setting pWalker->eCode) is a harmless missed optimization.
#[unsafe(link_section = ".text.slate_distinct.expr.impliesNotNullRow")]
extern "C-unwind" fn impliesNotNullRow(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    {}
    {}
    if (unsafe { (*pExpr).flags }) & ((1 as i32) as u32) != ((0 as i32) as u32) {
        return 1 as i32;
    }
    if (unsafe { (*pExpr).flags }) & ((2 as i32) as u32) != ((0 as i32) as u32)
        && (unsafe { (*pWalker).mWFlags }) != (0 as u16)
    {
        // If iCur is used in an inner-join ON clause to the left of a
        // RIGHT JOIN, that does *not* mean that the table must be non-null.
        // But it is difficult to check for that condition precisely.
        // To keep things simple, any use of iCur from any inner-join is
        // ignored while attempting to simplify a RIGHT JOIN.
        return 1 as i32;
    }
    // Both sides of an AND or OR must separately imply non-null-row.
    // Consider these cases:
    //    1.  NOT (x AND y)
    //    2.  x OR y
    // If only one of x or y is non-null-row, then the overall expression
    // can be true if the other arm is false (case 1) or true (case 2).
    // Beware of "x NOT IN ()" and "x NOT IN (SELECT 1 WHERE false)",
    // both of which can be true.  But apart from these cases, if
    // the left-hand side of the IN is NULL then the IN itself will be
    // NULL.
    // In "x NOT BETWEEN y AND z" either x must be non-null-row or else
    // both y and z must be non-null row
    // Virtual tables are allowed to use constraints like x=NULL.  So
    // a term of the form x=y does not prove that y is not null if x
    // is the column of a virtual table
    // The y.pTab=0 assignment in wherecode.c always happens after the
    // impliesNotNullRow() test
    // no break
    match ((unsafe { (*pExpr).op }) as u32) as i32 {
        46 | 51 | 52 | 45 | 177 | 172 | 175 | 158 => {
            {}
            {}
            {}
            {}
            {}
            {}
            {}
            {}
            return 1 as i32;
        }
        168 => {
            if (unsafe { (*pWalker).u.iCur }) == unsafe { (*pExpr).iTable } {
                unsafe {
                    (*pWalker).eCode = ((1 as i32) as i16) as u16;
                }
                return 2 as i32;
            }
            return 1 as i32;
        }
        43 | 44 => {
            {}
            // Both sides of an AND or OR must separately imply non-null-row.
            // Consider these cases:
            //    1.  NOT (x AND y)
            //    2.  x OR y
            // If only one of x or y is non-null-row, then the overall expression
            // can be true if the other arm is false (case 1) or true (case 2).
            {}
            bothImplyNotNullRow(pWalker, unsafe { (*pExpr).pLeft }, unsafe {
                (*pExpr).pRight
            });
            return 1 as i32;
        }
        50 => {
            if (unsafe { (*pExpr).flags }) & ((4096 as i32) as u32) == ((0 as i32) as u32)
                && (unsafe { (*unsafe { (*pExpr).x.pList }).nExpr }) > (0 as i32)
            {
                unsafe { sqlite3WalkExpr(pWalker, unsafe { (*pExpr).pLeft }) };
            }
            // Beware of "x NOT IN ()" and "x NOT IN (SELECT 1 WHERE false)",
            // both of which can be true.  But apart from these cases, if
            // the left-hand side of the IN is NULL then the IN itself will be
            // NULL.
            return 1 as i32;
        }
        49 => {
            0 as i32;
            // In "x NOT BETWEEN y AND z" either x must be non-null-row or else
            // both y and z must be non-null row
            0 as i32;
            unsafe { sqlite3WalkExpr(pWalker, unsafe { (*pExpr).pLeft }) };
            bothImplyNotNullRow(
                pWalker,
                unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*pExpr).x.pList }).a)
                                as *mut ExprList_item
                        }
                        .offset((0 as i32) as isize)
                    })
                    .pExpr
                },
                unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*pExpr).x.pList }).a)
                                as *mut ExprList_item
                        }
                        .offset((1 as i32) as isize)
                    })
                    .pExpr
                },
            );
            return 1 as i32;
            // Virtual tables are allowed to use constraints like x=NULL.  So
            // a term of the form x=y does not prove that y is not null if x
            // is the column of a virtual table
        }
        54 | 53 | 57 | 56 | 55 | 58 => {
            let mut pLeft: *mut Expr = unsafe { (*pExpr).pLeft };
            let mut pRight: *mut Expr = unsafe { (*pExpr).pRight };
            {}
            {}
            {}
            {}
            {}
            {}
            // The y.pTab=0 assignment in wherecode.c always happens after the
            // impliesNotNullRow() test
            0 as i32;
            0 as i32;
            if (((unsafe { (*pLeft).op }) as u32) as i32) == (168 as i32)
                && (unsafe { (*pLeft).y.pTab }) != std::ptr::null_mut::<Table>()
                && (((unsafe { (*unsafe { (*pLeft).y.pTab }).eTabType }) as u32) as i32)
                    == (1 as i32)
                || (((unsafe { (*pRight).op }) as u32) as i32) == (168 as i32)
                    && (unsafe { (*pRight).y.pTab }) != std::ptr::null_mut::<Table>()
                    && (((unsafe { (*unsafe { (*pRight).y.pTab }).eTabType }) as u32) as i32)
                        == (1 as i32)
            {
                return 1 as i32;
            }
            // no break
            {}
            return 0 as i32;
        }
        _ => {
            return 0 as i32;
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Return true (non-zero) if expression p can only be true if at least
/// one column of table iTab is non-null.  In other words, return true
/// if expression p will always be NULL or false if every column of iTab
/// is NULL.
///
/// False negatives are acceptable.  In other words, it is ok to return
/// zero even if expression p will never be true of every column of iTab
/// is NULL.  A false negative is merely a missed optimization opportunity.
///
/// False positives are not allowed, however.  A false positive may result
/// in an incorrect answer.
///
/// Terms of p that are marked with EP_OuterON (and hence that come from
/// the ON or USING clauses of OUTER JOINS) are excluded from the analysis.
///
/// This routine is used to check if a LEFT JOIN can be converted into
/// an ordinary JOIN.  The p argument is the WHERE clause.  If the WHERE
/// clause requires that some column of the right table of the LEFT JOIN
/// be non-NULL, then the LEFT JOIN can be safely converted into an
/// ordinary join.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprImpliesNonNullRow(
    mut p: *mut Expr,
    mut iTab: i32,
    mut isRJ: i32,
) -> i32 {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    p = sqlite3ExprSkipCollateAndLikely(p);
    if p == std::ptr::null_mut::<Expr>() {
        return 0 as i32;
    }
    if (((unsafe { (*p).op }) as u32) as i32) == (52 as i32) {
        p = unsafe { (*p).pLeft };
    } else {
        '__slate_break_2351: while (((unsafe { (*p).op }) as u32) as i32) == (44 as i32) {
            if sqlite3ExprImpliesNonNullRow(unsafe { (*p).pLeft }, iTab, isRJ) != (0 as i32) {
                return 1 as i32;
            }
            p = unsafe { (*p).pRight };
        }
    }
    w.xExprCallback = Some(impliesNotNullRow);
    w.xSelectCallback = None;
    w.xSelectCallback2 = None;
    w.eCode = ((0 as i32) as i16) as u16;
    w.mWFlags = (isRJ != (0 as i32)) as u16;
    unsafe {
        w.u.iCur = iTab;
    }
    unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(w), p) };
    return (w.eCode as u32) as i32;
}

/// An instance of the following structure is used by the tree walker
/// to determine if an expression can be evaluated by reference to the
/// index only, without having to do a search for the corresponding
/// table entry.  The IdxCover.pIdx field is the index.  IdxCover.iCur
/// is the cursor for the table.
#[repr(C)]
#[derive(Clone, Copy)]
struct IdxCover {
    /// The index to be tested for coverage
    pIdx: *mut Index,
    /// Cursor number for the table corresponding to the index
    iCur: i32,
}

/// Check to see if there are references to columns in table
/// pWalker->u.pIdxCover->iCur can be satisfied using the index
/// pWalker->u.pIdxCover->pIdx.
#[unsafe(link_section = ".text.slate_distinct.expr.exprIdxCover")]
extern "C-unwind" fn exprIdxCover(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    let __v2707: bool;
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (168 as i32)
        && (unsafe { (*pExpr).iTable }) == unsafe { (*unsafe { (*pWalker).u.pIdxCover }).iCur }
    {
        __v2707 = (unsafe {
            sqlite3TableColumnToIndex(
                unsafe { (*unsafe { (*pWalker).u.pIdxCover }).pIdx },
                (unsafe { (*pExpr).iColumn }) as i32,
            )
        }) < (0 as i32);
    } else {
        __v2707 = false as bool;
    }
    if __v2707 {
        unsafe {
            (*pWalker).eCode = ((1 as i32) as i16) as u16;
        }
        return 2 as i32;
    }
    return 0 as i32;
}

/// Determine if an index pIdx on table with cursor iCur contains will
/// the expression pExpr.  Return true if the index does cover the
/// expression and false if the pExpr expression references table columns
/// that are not found in the index pIdx.
///
/// An index covering an expression means that the expression can be
/// evaluated using only the index and without having to lookup the
/// corresponding table entry.
///
/// # Arguments
///
/// * `pExpr` - The index to be tested
/// * `iCur` - The cursor number for the corresponding table
/// * `pIdx` - The index that might be used for coverage
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprCoveredByIndex(
    mut pExpr: *mut Expr,
    mut iCur: i32,
    mut pIdx: *mut Index,
) -> i32 {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    let mut xcov: IdxCover = unsafe { std::mem::zeroed() };
    unsafe { memset(std::ptr::addr_of_mut!(w) as *mut (), 0 as i32, 48 as u64) };
    xcov.iCur = iCur;
    xcov.pIdx = pIdx;
    w.xExprCallback = Some(exprIdxCover);
    unsafe {
        w.u.pIdxCover = std::ptr::addr_of_mut!(xcov);
    }
    unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(w), pExpr) };
    return !(w.eCode != (0 as u16)) as i32;
}

/// Structure used to pass information throughout the Walker in order to
/// implement sqlite3ReferencesSrcList().
#[repr(C)]
#[derive(Clone, Copy)]
struct RefSrcList {
    /// Database connection used for sqlite3DbRealloc()
    db: *mut sqlite3,
    /// Looking for references to these tables
    pRef: *mut SrcList,
    /// Number of tables to exclude from the search
    nExclude: i64,
    /// Cursor IDs for tables to exclude from the search
    aiExclude: *mut i32,
}

/// Walker SELECT callbacks for sqlite3ReferencesSrcList().
///
/// When entering a new subquery on the pExpr argument, add all FROM clause
/// entries for that subquery to the exclude list.
///
/// When leaving the subquery, remove those entries from the exclude list.
#[unsafe(link_section = ".text.slate_distinct.expr.selectRefEnter")]
extern "C-unwind" fn selectRefEnter(mut pWalker: *mut Walker, mut pSelect: *mut Select) -> i32 {
    let mut p: *mut RefSrcList = unsafe { (*pWalker).u.pRefSrcList };
    let mut pSrc: *mut SrcList = unsafe { (*pSelect).pSrc };
    let mut i: i64 = 0 as i64;
    let mut j: i64 = 0 as i64;
    let mut piNew: *mut i32 = unsafe { std::mem::zeroed() };
    if (unsafe { (*pSrc).nSrc }) == (0 as i32) {
        return 0 as i32;
    }
    j = unsafe { (*p).nExclude };
    let __v2708: *mut RefSrcList = p;
    let __v2709: i64 = unsafe { (*__v2708).nExclude };
    let __v2710: i64 = __v2709 + ((unsafe { (*pSrc).nSrc }) as i64);
    unsafe {
        (*__v2708).nExclude = __v2710;
    }
    piNew = (unsafe {
        sqlite3DbRealloc(
            unsafe { (*p).db },
            (unsafe { (*p).aiExclude }) as *mut (),
            ((unsafe { (*p).nExclude }) as u64).wrapping_mul(4 as u64),
        )
    }) as *mut i32;
    if piNew == std::ptr::null_mut::<i32>() {
        unsafe {
            (*p).nExclude = (0 as i32) as i64;
        }
        return 2 as i32;
    } else {
        unsafe {
            (*p).aiExclude = piNew;
        }
    }
    i = (0 as i32) as i64;
    '__slate_break_2352: loop {
        if !(i < ((unsafe { (*pSrc).nSrc }) as i64)) {
            break;
        }
        unsafe {
            *unsafe { unsafe { (*p).aiExclude }.offset(j as isize) } = unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset(i as isize)
                })
                .iCursor
            };
        }
        let __v2711: i64 = i;
        let __v2712: i64 = __v2711 + ((1 as i32) as i64);
        i = __v2712;
        let __v2713: i64 = j;
        let __v2714: i64 = __v2713 + ((1 as i32) as i64);
        j = __v2714;
    }
    return 0 as i32;
}

#[unsafe(link_section = ".text.slate_distinct.expr.selectRefLeave")]
extern "C-unwind" fn selectRefLeave(mut pWalker: *mut Walker, mut pSelect: *mut Select) {
    let mut p: *mut RefSrcList = unsafe { (*pWalker).u.pRefSrcList };
    let mut pSrc: *mut SrcList = unsafe { (*pSelect).pSrc };
    if (unsafe { (*p).nExclude }) != (0 as i64) {
        0 as i32;
        let __v2715: *mut RefSrcList = p;
        let __v2716: i64 = unsafe { (*__v2715).nExclude };
        let __v2717: i64 = __v2716 - ((unsafe { (*pSrc).nSrc }) as i64);
        unsafe {
            (*__v2715).nExclude = __v2717;
        }
    }
}

/// This is the Walker EXPR callback for sqlite3ReferencesSrcList().
///
/// Set the 0x01 bit of pWalker->eCode if there is a reference to any
/// of the tables shown in RefSrcList.pRef.
///
/// Set the 0x02 bit of pWalker->eCode if there is a reference to a
/// table is in neither RefSrcList.pRef nor RefSrcList.aiExclude.
#[unsafe(link_section = ".text.slate_distinct.expr.exprRefToSrcList")]
extern "C-unwind" fn exprRefToSrcList(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (168 as i32)
        || (((unsafe { (*pExpr).op }) as u32) as i32) == (170 as i32)
    {
        let mut i: i32 = 0 as i32;
        let mut p: *mut RefSrcList = unsafe { (*pWalker).u.pRefSrcList };
        let mut pSrc: *mut SrcList = unsafe { (*p).pRef };
        let mut nSrc: i32 = if pSrc != std::ptr::null_mut::<SrcList>() {
            unsafe { (*pSrc).nSrc }
        } else {
            0 as i32
        };
        i = 0 as i32;
        '__slate_break_2353: loop {
            if !(i < nSrc) {
                break;
            }
            if (unsafe { (*pExpr).iTable })
                == unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                            .offset(i as isize)
                    })
                    .iCursor
                }
            {
                let __v2720: *mut Walker = pWalker;
                let __v2721: u16 = unsafe { (*__v2720).eCode };
                let __v2722: u16 = ((((__v2721 as u32) as i32) | (1 as i32)) as i16) as u16;
                unsafe {
                    (*__v2720).eCode = __v2722;
                }
                return 0 as i32;
            }
            let __v2718: i32 = i;
            let __v2719: i32 = __v2718 + (1 as i32);
            i = __v2719;
        }
        i = 0 as i32;
        '__slate_break_2354: loop {
            if !((i as i64) < unsafe { (*p).nExclude }
                && (unsafe { *unsafe { unsafe { (*p).aiExclude }.offset(i as isize) } })
                    != unsafe { (*pExpr).iTable })
            {
                break;
            }
            let __v2723: i32 = i;
            let __v2724: i32 = __v2723 + (1 as i32);
            i = __v2724;
        }
        if (i as i64) >= unsafe { (*p).nExclude } {
            let __v2725: *mut Walker = pWalker;
            let __v2726: u16 = unsafe { (*__v2725).eCode };
            let __v2727: u16 = ((((__v2726 as u32) as i32) | (2 as i32)) as i16) as u16;
            unsafe {
                (*__v2725).eCode = __v2727;
            }
        }
    }
    return 0 as i32;
}

/// Check to see if pExpr references any tables in pSrcList.
/// Possible return values:
///
///    1         pExpr does references a table in pSrcList.
///
///    0         pExpr references some table that is not defined in either
///              pSrcList or in subqueries of pExpr itself.
///
///   -1         pExpr only references no tables at all, or it only
///              references tables defined in subqueries of pExpr itself.
///
/// As currently used, pExpr is always an aggregate function call.  That
/// fact is exploited for efficiency.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ReferencesSrcList(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut pSrcList: *mut SrcList,
) -> i32 {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    let mut x: RefSrcList = unsafe { std::mem::zeroed() };
    0 as i32;
    unsafe { memset(std::ptr::addr_of_mut!(w) as *mut (), 0 as i32, 48 as u64) };
    unsafe { memset(std::ptr::addr_of_mut!(x) as *mut (), 0 as i32, 32 as u64) };
    w.xExprCallback = Some(exprRefToSrcList);
    w.xSelectCallback = Some(selectRefEnter);
    w.xSelectCallback2 = Some(selectRefLeave);
    unsafe {
        w.u.pRefSrcList = std::ptr::addr_of_mut!(x);
    }
    x.db = unsafe { (*pParse).db };
    x.pRef = pSrcList;
    0 as i32;
    0 as i32;
    unsafe { sqlite3WalkExprList(std::ptr::addr_of_mut!(w), unsafe { (*pExpr).x.pList }) };
    if (unsafe { (*pExpr).pLeft }) != std::ptr::null_mut::<Expr>() {
        0 as i32;
        0 as i32;
        0 as i32;
        unsafe {
            sqlite3WalkExprList(std::ptr::addr_of_mut!(w), unsafe {
                (*unsafe { (*pExpr).pLeft }).x.pList
            })
        };
    }
    if (unsafe { (*pExpr).flags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32) {
        unsafe {
            sqlite3WalkExpr(std::ptr::addr_of_mut!(w), unsafe {
                (*unsafe { (*pExpr).y.pWin }).pFilter
            })
        };
    }
    if x.aiExclude != std::ptr::null_mut::<i32>() {
        unsafe { sqlite3DbNNFreeNN(unsafe { (*pParse).db }, x.aiExclude as *mut ()) };
    }
    if ((w.eCode as u32) as i32) & (1 as i32) != (0 as i32) {
        return 1 as i32;
    } else {
        if w.eCode != (0 as u16) {
            return 0 as i32;
        } else {
            return -(1 as i32);
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// This is a Walker expression node callback.
///
/// For Expr nodes that contain pAggInfo pointers, make sure the AggInfo
/// object that is referenced does not refer directly to the Expr.  If
/// it does, make a copy.  This is done because the pExpr argument is
/// subject to change.
///
/// The copy is scheduled for deletion using the sqlite3ExprDeferredDelete()
/// which builds on the sqlite3ParserAddCleanup() mechanism.
#[unsafe(link_section = ".text.slate_distinct.expr.agginfoPersistExprCb")]
extern "C-unwind" fn agginfoPersistExprCb(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    if !((unsafe { (*pExpr).flags }) & (((65536 as i32) | (16384 as i32)) as u32)
        != ((0 as i32) as u32))
        && (unsafe { (*pExpr).pAggInfo }) != std::ptr::null_mut::<AggInfo>()
    {
        let mut pAggInfo: *mut AggInfo = unsafe { (*pExpr).pAggInfo };
        let mut iAgg: i32 = (unsafe { (*pExpr).iAgg }) as i32;
        let mut pParse: *mut Parse = unsafe { (*pWalker).pParse };
        let mut db: *mut sqlite3 = unsafe { (*pParse).db };
        0 as i32;
        if (((unsafe { (*pExpr).op }) as u32) as i32) != (169 as i32) {
            if iAgg < unsafe { (*pAggInfo).nColumn }
                && (unsafe {
                    (*unsafe { unsafe { (*pAggInfo).aCol }.offset(iAgg as isize) }).pCExpr
                }) == pExpr
            {
                pExpr = sqlite3ExprDup(db, pExpr as *const Expr, 0 as i32);
                let __v2728: bool;
                if pExpr != std::ptr::null_mut::<Expr>() {
                    __v2728 = !(sqlite3ExprDeferredDelete(pParse, pExpr) != (0 as i32));
                } else {
                    __v2728 = false as bool;
                }
                if __v2728 {
                    unsafe {
                        (*unsafe { unsafe { (*pAggInfo).aCol }.offset(iAgg as isize) }).pCExpr =
                            pExpr;
                    }
                }
            }
        } else {
            0 as i32;
            if iAgg < unsafe { (*pAggInfo).nFunc }
                && (unsafe {
                    (*unsafe { unsafe { (*pAggInfo).aFunc }.offset(iAgg as isize) }).pFExpr
                }) == pExpr
            {
                pExpr = sqlite3ExprDup(db, pExpr as *const Expr, 0 as i32);
                let __v2729: bool;
                if pExpr != std::ptr::null_mut::<Expr>() {
                    __v2729 = !(sqlite3ExprDeferredDelete(pParse, pExpr) != (0 as i32));
                } else {
                    __v2729 = false as bool;
                }
                if __v2729 {
                    unsafe {
                        (*unsafe { unsafe { (*pAggInfo).aFunc }.offset(iAgg as isize) }).pFExpr =
                            pExpr;
                    }
                }
            }
        }
    }
    return 0 as i32;
}

/// Initialize a Walker object so that will persist AggInfo entries referenced
/// by the tree that is walked.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AggInfoPersistWalkerInit(
    mut pWalker: *mut Walker,
    mut pParse: *mut Parse,
) {
    unsafe { memset(pWalker as *mut (), 0 as i32, 48 as u64) };
    unsafe {
        (*pWalker).pParse = pParse;
    }
    unsafe {
        (*pWalker).xExprCallback = Some(agginfoPersistExprCb);
    }
    unsafe {
        (*pWalker).xSelectCallback = unsafe {
            std::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
            >(sqlite3SelectWalkNoop as *const ())
        };
    }
}

/// Add a new element to the pAggInfo->aCol[] array.  Return the index of
/// the new element.  Return a negative number if malloc fails.
fn addAggInfoColumn(mut db: *mut sqlite3, mut pInfo: *mut AggInfo) -> i32 {
    let mut i: i32 = 0 as i32;
    unsafe {
        (*pInfo).aCol = (unsafe {
            sqlite3ArrayAllocate(
                db,
                (unsafe { (*pInfo).aCol }) as *mut (),
                ((32 as u64) as u32) as i32,
                unsafe { std::ptr::addr_of_mut!((*pInfo).nColumn) },
                std::ptr::addr_of_mut!(i),
            )
        }) as *mut AggInfo_col;
    }
    return i;
}

/// Add a new element to the pAggInfo->aFunc[] array.  Return the index of
/// the new element.  Return a negative number if malloc fails.
fn addAggInfoFunc(mut db: *mut sqlite3, mut pInfo: *mut AggInfo) -> i32 {
    let mut i: i32 = 0 as i32;
    unsafe {
        (*pInfo).aFunc = (unsafe {
            sqlite3ArrayAllocate(
                db,
                (unsafe { (*pInfo).aFunc }) as *mut (),
                ((32 as u64) as u32) as i32,
                unsafe { std::ptr::addr_of_mut!((*pInfo).nFunc) },
                std::ptr::addr_of_mut!(i),
            )
        }) as *mut AggInfo_func;
    }
    return i;
}

/// Search the AggInfo object for an aCol[] entry that has iTable and iColumn.
/// Return the index in aCol[] of the entry that describes that column.
///
/// If no prior entry is found, create a new one and return -1.  The
/// new column will have an index of pAggInfo->nColumn-1.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pAggInfo` - The AggInfo object to search and/or modify
/// * `pExpr` - Expr describing the column to find or insert
fn findOrCreateAggInfoColumn(
    mut pParse: *mut Parse,
    mut pAggInfo: *mut AggInfo,
    mut pExpr: *mut Expr,
) {
    let mut __slate_storage_2740: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2740: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2740) as *mut u32;
    let mut __slate_storage_2739: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2739: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2739) as *mut u32;
    let mut __slate_storage_2738: std::mem::MaybeUninit<*mut AggInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2738: *mut *mut AggInfo =
        std::ptr::addr_of_mut!(__slate_storage_2738) as *mut *mut AggInfo;
    let mut __slate_storage_2737: std::mem::MaybeUninit<*mut ExprList_item> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2737: *mut *mut ExprList_item =
        std::ptr::addr_of_mut!(__slate_storage_2737) as *mut *mut ExprList_item;
    let mut __slate_storage_2736: std::mem::MaybeUninit<*mut ExprList_item> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2736: *mut *mut ExprList_item =
        std::ptr::addr_of_mut!(__slate_storage_2736) as *mut *mut ExprList_item;
    let mut __slate_storage_2735: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2735: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2735) as *mut i32;
    let mut __slate_storage_2734: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2734: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2734) as *mut i32;
    let mut __slate_storage_1597: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1597: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1597) as *mut *mut Expr;
    let mut __slate_storage_1596: std::mem::MaybeUninit<*mut ExprList_item> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1596: *mut *mut ExprList_item =
        std::ptr::addr_of_mut!(__slate_storage_1596) as *mut *mut ExprList_item;
    let mut __slate_storage_1595: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1595: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_1595) as *mut *mut ExprList;
    let mut __slate_storage_1594: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1594: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1594) as *mut i32;
    let mut __slate_storage_1593: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1593: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1593) as *mut i32;
    let mut __slate_storage_2733: std::mem::MaybeUninit<*mut AggInfo_col> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2733: *mut *mut AggInfo_col =
        std::ptr::addr_of_mut!(__slate_storage_2733) as *mut *mut AggInfo_col;
    let mut __slate_storage_2732: std::mem::MaybeUninit<*mut AggInfo_col> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2732: *mut *mut AggInfo_col =
        std::ptr::addr_of_mut!(__slate_storage_2732) as *mut *mut AggInfo_col;
    let mut __slate_storage_2731: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2731: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2731) as *mut i32;
    let mut __slate_storage_2730: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2730: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2730) as *mut i32;
    let mut __slate_storage_1592: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1592: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1592) as *mut i32;
    let mut __slate_storage_1591: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1591: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1591) as *mut i32;
    let mut __slate_storage_1590: std::mem::MaybeUninit<*mut AggInfo_col> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1590: *mut *mut AggInfo_col =
        std::ptr::addr_of_mut!(__slate_storage_1590) as *mut *mut AggInfo_col;
    unsafe {
        std::ptr::write(__slate_slot_1592, unsafe {
            *unsafe {
                unsafe { (*unsafe { (*pParse).db }).aLimit.as_mut_ptr() as *mut i32 }
                    .offset((2 as i32) as isize)
            }
        });
        0 as i32;
        0 as i32;
        *__slate_slot_1590 = unsafe { (*pAggInfo).aCol };
        *__slate_slot_1591 = 0 as i32;
        '__join_2: {
            loop {
                if *__slate_slot_1591 < unsafe { (*pAggInfo).nColumn } {
                    if (unsafe { (*(*__slate_slot_1590)).pCExpr }) == pExpr {
                        return;
                    } else {
                        if (unsafe { (*(*__slate_slot_1590)).iTable }) == unsafe { (*pExpr).iTable }
                            && (unsafe { (*(*__slate_slot_1590)).iColumn })
                                == ((unsafe { (*pExpr).iColumn }) as i32)
                            && (((unsafe { (*pExpr).op }) as u32) as i32) != (179 as i32)
                        {
                            break '__join_2;
                        } else {
                            std::ptr::write(__slate_slot_2730, *__slate_slot_1591);
                            std::ptr::write(__slate_slot_2731, *__slate_slot_2730 + (1 as i32));
                            *__slate_slot_1591 = *__slate_slot_2731;
                            std::ptr::write(__slate_slot_2732, *__slate_slot_1590);
                            std::ptr::write(__slate_slot_2733, unsafe {
                                (*__slate_slot_2732).offset((1 as i32) as isize)
                            });
                            *__slate_slot_1590 = *__slate_slot_2733;
                        }
                    }
                } else {
                    break;
                }
            }
            *__slate_slot_1591 = addAggInfoColumn(unsafe { (*pParse).db }, pAggInfo);
            if *__slate_slot_1591 < (0 as i32) {
                // OOM on resize
                0 as i32;
                return;
            } else {
                if *__slate_slot_1591 > *__slate_slot_1592 {
                    unsafe {
                        sqlite3ErrorMsg(
                            pParse,
                            (b"more than %d aggregate terms\0".as_ptr() as *mut i8) as *const i8,
                            *__slate_slot_1592,
                        )
                    };
                    *__slate_slot_1591 = *__slate_slot_1592;
                }
                '__join_4: {
                    *__slate_slot_1590 =
                        unsafe { unsafe { (*pAggInfo).aCol }.offset(*__slate_slot_1591 as isize) };
                    0 as i32;
                    unsafe {
                        (*(*__slate_slot_1590)).pTab = unsafe { (*pExpr).y.pTab };
                    }
                    unsafe {
                        (*(*__slate_slot_1590)).iTable = unsafe { (*pExpr).iTable };
                    }
                    unsafe {
                        (*(*__slate_slot_1590)).iColumn = (unsafe { (*pExpr).iColumn }) as i32;
                    }
                    unsafe {
                        (*(*__slate_slot_1590)).iSorterColumn = -(1 as i32);
                    }
                    unsafe {
                        (*(*__slate_slot_1590)).pCExpr = pExpr;
                    }
                    if (unsafe { (*pAggInfo).pGroupBy }) != std::ptr::null_mut::<ExprList>()
                        && (((unsafe { (*pExpr).op }) as u32) as i32) != (179 as i32)
                    {
                        std::ptr::write(__slate_slot_1595, unsafe { (*pAggInfo).pGroupBy });
                        std::ptr::write(__slate_slot_1596, unsafe {
                            std::ptr::addr_of_mut!((*(*__slate_slot_1595)).a) as *mut ExprList_item
                        });
                        *__slate_slot_1594 = unsafe { (*(*__slate_slot_1595)).nExpr };
                        *__slate_slot_1593 = 0 as i32;
                        loop {
                            if *__slate_slot_1593 < *__slate_slot_1594 {
                                std::ptr::write(__slate_slot_1597, unsafe {
                                    (*(*__slate_slot_1596)).pExpr
                                });
                                if (((unsafe { (*(*__slate_slot_1597)).op }) as u32) as i32)
                                    == (168 as i32)
                                    && (unsafe { (*(*__slate_slot_1597)).iTable })
                                        == unsafe { (*pExpr).iTable }
                                    && ((unsafe { (*(*__slate_slot_1597)).iColumn }) as i32)
                                        == ((unsafe { (*pExpr).iColumn }) as i32)
                                {
                                    break;
                                } else {
                                    std::ptr::write(__slate_slot_2734, *__slate_slot_1593);
                                    std::ptr::write(
                                        __slate_slot_2735,
                                        *__slate_slot_2734 + (1 as i32),
                                    );
                                    *__slate_slot_1593 = *__slate_slot_2735;
                                    std::ptr::write(__slate_slot_2736, *__slate_slot_1596);
                                    std::ptr::write(__slate_slot_2737, unsafe {
                                        (*__slate_slot_2736).offset((1 as i32) as isize)
                                    });
                                    *__slate_slot_1596 = *__slate_slot_2737;
                                }
                            } else {
                                break '__join_4;
                            }
                        }
                        unsafe {
                            (*(*__slate_slot_1590)).iSorterColumn = *__slate_slot_1593;
                        }
                    }
                }
                if (unsafe { (*(*__slate_slot_1590)).iSorterColumn }) < (0 as i32) {
                    std::ptr::write(__slate_slot_2738, pAggInfo);
                    std::ptr::write(__slate_slot_2739, unsafe {
                        (*(*__slate_slot_2738)).nSortingColumn
                    });
                    std::ptr::write(
                        __slate_slot_2740,
                        (*__slate_slot_2739).wrapping_add((1 as i32) as u32),
                    );
                    unsafe {
                        (*(*__slate_slot_2738)).nSortingColumn = *__slate_slot_2740;
                    }
                    unsafe {
                        (*(*__slate_slot_1590)).iSorterColumn = *__slate_slot_2739 as i32;
                    }
                }
            }
        }
        {}
        0 as i32;
        unsafe {
            (*pExpr).pAggInfo = pAggInfo;
        }
        if (((unsafe { (*pExpr).op }) as u32) as i32) == (168 as i32) {
            unsafe {
                (*pExpr).op = ((170 as i32) as i8) as u8;
            }
        }
        0 as i32;
        unsafe {
            (*pExpr).iAgg = *__slate_slot_1591 as i16;
        }
    }
}

/// This is the xExprCallback for a tree walker.  It is used to
/// implement sqlite3ExprAnalyzeAggregates().  See sqlite3ExprAnalyzeAggregates
/// for additional information.
#[unsafe(link_section = ".text.slate_distinct.expr.analyzeAggregate")]
extern "C-unwind" fn analyzeAggregate(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut pNC: *mut NameContext = unsafe { (*pWalker).u.pNC };
    let mut pParse: *mut Parse = unsafe { (*pNC).pParse };
    let mut pSrcList: *mut SrcList = unsafe { (*pNC).pSrcList };
    let mut pAggInfo: *mut AggInfo = unsafe { (*pNC).uNC.pAggInfo };
    0 as i32;
    0 as i32;
    match ((unsafe { (*pExpr).op }) as u32) as i32 {
        179 | 170 | 168 => {
            {}
            {}
            {}
            // Check to see if the column is in one of the tables in the FROM
            // clause of the aggregate query
            if pSrcList != std::ptr::null_mut::<SrcList>() {
                let mut pItem: *mut SrcItem =
                    unsafe { std::ptr::addr_of_mut!((*pSrcList).a) as *mut SrcItem };
                i = 0 as i32;
                '__slate_break_2361: while i < unsafe { (*pSrcList).nSrc } {
                    0 as i32;
                    if (unsafe { (*pExpr).iTable }) == unsafe { (*pItem).iCursor } {
                        findOrCreateAggInfoColumn(pParse, pAggInfo, pExpr);
                        break '__slate_break_2361;
                    }
                    // endif pExpr->iTable==pItem->iCursor
                    let __v2743: i32 = i;
                    let __v2744: i32 = __v2743 + (1 as i32);
                    i = __v2744;
                    let __v2745: *mut SrcItem = pItem;
                    let __v2746: *mut SrcItem = unsafe { __v2745.offset((1 as i32) as isize) };
                    pItem = __v2746;
                }
                // end loop over pSrcList
            }
            return 0 as i32;
        }
        169 => {
            if (unsafe { (*pNC).ncFlags }) & (131072 as i32) == (0 as i32)
                && (unsafe { (*pWalker).walkerDepth })
                    == (((unsafe { (*pExpr).op2 }) as u32) as i32)
                && (unsafe { (*pExpr).pAggInfo }) == std::ptr::null_mut::<AggInfo>()
            {
                // Check to see if pExpr is a duplicate of another aggregate
                // function that is already in the pAggInfo structure
                let mut pItem: *mut AggInfo_func = unsafe { (*pAggInfo).aFunc };
                let mut mxTerm: i32 = unsafe {
                    *unsafe {
                        unsafe { (*unsafe { (*pParse).db }).aLimit.as_mut_ptr() as *mut i32 }
                            .offset((2 as i32) as isize)
                    }
                };
                0 as i32;
                i = 0 as i32;
                '__slate_break_2362: while i < unsafe { (*pAggInfo).nFunc } {
                    if (unsafe { (*pItem).pFExpr }) == pExpr {
                        break '__slate_break_2362;
                    }
                    if sqlite3ExprCompare(
                        std::ptr::null::<Parse>(),
                        (unsafe { (*pItem).pFExpr }) as *const Expr,
                        pExpr as *const Expr,
                        -(1 as i32),
                    ) == (0 as i32)
                    {
                        break '__slate_break_2362;
                    }
                    let __v2747: i32 = i;
                    let __v2748: i32 = __v2747 + (1 as i32);
                    i = __v2748;
                    let __v2749: *mut AggInfo_func = pItem;
                    let __v2750: *mut AggInfo_func = unsafe { __v2749.offset((1 as i32) as isize) };
                    pItem = __v2750;
                }
                if i > mxTerm {
                    unsafe {
                        sqlite3ErrorMsg(
                            pParse,
                            (b"more than %d aggregate terms\0".as_ptr() as *mut i8) as *const i8,
                            mxTerm,
                        )
                    };
                    i = mxTerm;
                    0 as i32;
                } else {
                    if i >= unsafe { (*pAggInfo).nFunc } {
                        // pExpr is original.  Make a new entry in pAggInfo->aFunc[]
                        let mut enc: u8 = unsafe { (*unsafe { (*pParse).db }).enc };
                        i = addAggInfoFunc(unsafe { (*pParse).db }, pAggInfo);
                        if i >= (0 as i32) {
                            let mut nArg: i32 = 0 as i32;
                            0 as i32;
                            pItem = unsafe { unsafe { (*pAggInfo).aFunc }.offset(i as isize) };
                            unsafe {
                                (*pItem).pFExpr = pExpr;
                            }
                            0 as i32;
                            nArg = if (unsafe { (*pExpr).x.pList })
                                != std::ptr::null_mut::<ExprList>()
                            {
                                unsafe { (*unsafe { (*pExpr).x.pList }).nExpr }
                            } else {
                                0 as i32
                            };
                            unsafe {
                                (*pItem).pFunc = unsafe {
                                    sqlite3FindFunction(
                                        unsafe { (*pParse).db },
                                        (unsafe { (*pExpr).u.zToken }) as *const i8,
                                        nArg,
                                        enc,
                                        ((0 as i32) as i8) as u8,
                                    )
                                };
                            }
                            0 as i32;
                            if (unsafe { (*pExpr).pLeft }) != std::ptr::null_mut::<Expr>()
                                && (unsafe { (*unsafe { (*pItem).pFunc }).funcFlags })
                                    & ((32 as i32) as u32)
                                    == ((0 as i32) as u32)
                            {
                                // The NEEDCOLL test above causes any ORDER BY clause on
                                // aggregate min() or max() to be ignored.
                                let mut pOBList: *mut ExprList = unsafe { std::mem::zeroed() };
                                0 as i32;
                                0 as i32;
                                0 as i32;
                                let __v2751: *mut Parse = pParse;
                                let __v2752: i32 = unsafe { (*__v2751).nTab };
                                let __v2753: i32 = __v2752 + (1 as i32);
                                unsafe {
                                    (*__v2751).nTab = __v2753;
                                }
                                unsafe {
                                    (*pItem).iOBTab = __v2752;
                                }
                                pOBList = unsafe { (*unsafe { (*pExpr).pLeft }).x.pList };
                                0 as i32;
                                0 as i32;
                                let __v2754: bool;
                                if (unsafe { (*pOBList).nExpr }) == (1 as i32) && nArg == (1 as i32)
                                {
                                    __v2754 = sqlite3ExprCompare(
                                        std::ptr::null::<Parse>(),
                                        (unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!((*pOBList).a)
                                                        as *mut ExprList_item
                                                }
                                                .offset((0 as i32) as isize)
                                            })
                                            .pExpr
                                        }) as *const Expr,
                                        (unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!(
                                                        (*unsafe { (*pExpr).x.pList }).a
                                                    )
                                                        as *mut ExprList_item
                                                }
                                                .offset((0 as i32) as isize)
                                            })
                                            .pExpr
                                        }) as *const Expr,
                                        0 as i32,
                                    ) == (0 as i32);
                                } else {
                                    __v2754 = false as bool;
                                }
                                if __v2754 {
                                    unsafe {
                                        (*pItem).bOBPayload = ((0 as i32) as i8) as u8;
                                    }
                                    unsafe {
                                        (*pItem).bOBUnique = ((unsafe { (*pExpr).flags })
                                            & ((4 as i32) as u32)
                                            != ((0 as i32) as u32))
                                            as u8;
                                    }
                                } else {
                                    unsafe {
                                        (*pItem).bOBPayload = ((1 as i32) as i8) as u8;
                                    }
                                }
                                unsafe {
                                    (*pItem).bUseSubtype =
                                        ((unsafe { (*unsafe { (*pItem).pFunc }).funcFlags })
                                            & ((1048576 as i32) as u32)
                                            != ((0 as i32) as u32))
                                            as u8;
                                }
                            } else {
                                unsafe {
                                    (*pItem).iOBTab = -(1 as i32);
                                }
                            }
                            if (unsafe { (*pExpr).flags }) & ((4 as i32) as u32)
                                != ((0 as i32) as u32)
                                && !((unsafe { (*pItem).bOBUnique }) != (0 as u8))
                            {
                                let __v2755: *mut Parse = pParse;
                                let __v2756: i32 = unsafe { (*__v2755).nTab };
                                let __v2757: i32 = __v2756 + (1 as i32);
                                unsafe {
                                    (*__v2755).nTab = __v2757;
                                }
                                unsafe {
                                    (*pItem).iDistinct = __v2756;
                                }
                            } else {
                                unsafe {
                                    (*pItem).iDistinct = -(1 as i32);
                                }
                            }
                        }
                    }
                }
                // Make pExpr point to the appropriate pAggInfo->aFunc[] entry
                0 as i32;
                {}
                0 as i32;
                unsafe {
                    (*pExpr).iAgg = i as i16;
                }
                unsafe {
                    (*pExpr).pAggInfo = pAggInfo;
                }
                return 1 as i32;
            } else {
                return 0 as i32;
            }
        }
        _ => {
            let mut pIEpr: *mut IndexedExpr = unsafe { std::mem::zeroed() };
            let mut tmp: Expr = unsafe { std::mem::zeroed() };
            0 as i32;
            if (unsafe { (*pNC).ncFlags }) & (131072 as i32) == (0 as i32) {
            } else {
                if (unsafe { (*pParse).pIdxEpr }) == std::ptr::null_mut::<IndexedExpr>() {
                } else {
                    pIEpr = unsafe { (*pParse).pIdxEpr };
                    '__slate_break_2359: while pIEpr != std::ptr::null_mut::<IndexedExpr>() {
                        let mut iDataCur: i32 = unsafe { (*pIEpr).iDataCur };
                        if iDataCur < (0 as i32) {
                        } else {
                            if sqlite3ExprCompare(
                                std::ptr::null::<Parse>(),
                                pExpr as *const Expr,
                                (unsafe { (*pIEpr).pExpr }) as *const Expr,
                                iDataCur,
                            ) == (0 as i32)
                            {
                                break '__slate_break_2359;
                            }
                        }
                        pIEpr = unsafe { (*pIEpr).pIENext };
                    }
                    if pIEpr == std::ptr::null_mut::<IndexedExpr>() {
                    } else {
                        if !((unsafe { (*pExpr).flags })
                            & (((16777216 as i32) | (33554432 as i32)) as u32)
                            == ((0 as i32) as u32))
                        {
                        } else {
                            i = 0 as i32;
                            '__slate_break_2360: loop {
                                if !(i < unsafe { (*pSrcList).nSrc }) {
                                    break;
                                }
                                if (unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pSrcList).a) as *mut SrcItem
                                        }
                                        .offset(i as isize)
                                    })
                                    .iCursor
                                }) == unsafe { (*pIEpr).iDataCur }
                                {
                                    {}
                                    break '__slate_break_2360;
                                }
                                let __v2741: i32 = i;
                                let __v2742: i32 = __v2741 + (1 as i32);
                                i = __v2742;
                            }
                            if i >= unsafe { (*pSrcList).nSrc } {
                            } else {
                                if (unsafe { (*pExpr).pAggInfo }) != std::ptr::null_mut::<AggInfo>()
                                {
                                } else {
                                    // Resolved by outer context
                                    if (unsafe { (*pParse).nErr }) != (0 as i32) {
                                        return 2 as i32;
                                    }
                                    // If we reach this point, it means that expression pExpr can be
                                    // translated into a reference to an index column as described by
                                    // pIEpr.
                                    unsafe {
                                        memset(
                                            std::ptr::addr_of_mut!(tmp) as *mut (),
                                            0 as i32,
                                            72 as u64,
                                        )
                                    };
                                    tmp.op = ((170 as i32) as i8) as u8;
                                    tmp.iTable = unsafe { (*pIEpr).iIdxCur };
                                    tmp.iColumn = (unsafe { (*pIEpr).iIdxCol }) as i16;
                                    findOrCreateAggInfoColumn(
                                        pParse,
                                        pAggInfo,
                                        std::ptr::addr_of_mut!(tmp),
                                    );
                                    if (unsafe { (*pParse).nErr }) != (0 as i32) {
                                        return 2 as i32;
                                    }
                                    0 as i32;
                                    0 as i32;
                                    unsafe {
                                        (*unsafe {
                                            unsafe { (*pAggInfo).aCol }
                                                .offset((tmp.iAgg as i32) as isize)
                                        })
                                        .pCExpr = pExpr;
                                    }
                                    unsafe {
                                        (*pExpr).pAggInfo = pAggInfo;
                                    }
                                    unsafe {
                                        (*pExpr).iAgg = tmp.iAgg;
                                    }
                                    return 1 as i32;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return 0 as i32;
}

/// Analyze the pExpr expression looking for aggregate functions and
/// for variables that need to be added to AggInfo object that pNC->pAggInfo
/// points to.  Additional entries are made on the AggInfo object as
/// necessary.
///
/// This routine should only be called after the expression has been
/// analyzed by sqlite3ResolveExprNames().
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprAnalyzeAggregates(mut pNC: *mut NameContext, mut pExpr: *mut Expr) {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    w.xExprCallback = Some(analyzeAggregate);
    w.xSelectCallback = unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
        >(sqlite3WalkerDepthIncrease as *const ())
    };
    w.xSelectCallback2 = unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select)>,
        >(sqlite3WalkerDepthDecrease as *const ())
    };
    w.walkerDepth = 0 as i32;
    unsafe {
        w.u.pNC = pNC;
    }
    w.pParse = std::ptr::null_mut::<Parse>();
    0 as i32;
    unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(w), pExpr) };
}

/// Call sqlite3ExprAnalyzeAggregates() for every expression in an
/// expression list.  Return the number of errors.
///
/// If an error is found, the analysis is cut short.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprAnalyzeAggList(
    mut pNC: *mut NameContext,
    mut pList: *mut ExprList,
) {
    let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    if pList != std::ptr::null_mut::<ExprList>() {
        pItem = unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item };
        i = 0 as i32;
        '__slate_break_2364: while i < unsafe { (*pList).nExpr } {
            sqlite3ExprAnalyzeAggregates(pNC, unsafe { (*pItem).pExpr });
            let __v2502: i32 = i;
            let __v2503: i32 = __v2502 + (1 as i32);
            i = __v2503;
            let __v2504: *mut ExprList_item = pItem;
            let __v2505: *mut ExprList_item = unsafe { __v2504.offset((1 as i32) as isize) };
            pItem = __v2505;
        }
    }
}

/// Allocate a single new register for use to hold some intermediate result.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GetTempReg(mut pParse: *mut Parse) -> i32 {
    if (((unsafe { (*pParse).nTempReg }) as u32) as i32) == (0 as i32) {
        let __v2365: *mut Parse = pParse;
        let __v2366: i32 = unsafe { (*__v2365).nMem };
        let __v2367: i32 = __v2366 + (1 as i32);
        unsafe {
            (*__v2365).nMem = __v2367;
        }
        return __v2367;
    }
    let __v2368: *mut Parse = pParse;
    let __v2369: u8 = unsafe { (*__v2368).nTempReg };
    let __v2370: u8 = ((((__v2369 as u32) as i32) - (1 as i32)) as i8) as u8;
    unsafe {
        (*__v2368).nTempReg = __v2370;
    }
    return unsafe {
        *unsafe {
            unsafe { (*pParse).aTempReg.as_mut_ptr() as *mut i32 }
                .offset(((__v2370 as u32) as i32) as isize)
        }
    };
}

/// Deallocate a register, making available for reuse for some other
/// purpose.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ReleaseTempReg(mut pParse: *mut Parse, mut iReg: i32) {
    if iReg != (0 as i32) {
        {}
        if (((unsafe { (*pParse).nTempReg }) as u32) as i32)
            < ((((32 as u64) / (4 as u64)) as u32) as i32)
        {
            let __v2371: *mut Parse = pParse;
            let __v2372: u8 = unsafe { (*__v2371).nTempReg };
            let __v2373: u8 = ((((__v2372 as u32) as i32) + (1 as i32)) as i8) as u8;
            unsafe {
                (*__v2371).nTempReg = __v2373;
            }
            unsafe {
                *unsafe {
                    unsafe { (*pParse).aTempReg.as_mut_ptr() as *mut i32 }
                        .offset(((__v2372 as u32) as i32) as isize)
                } = iReg;
            }
        }
    }
}

/// Allocate or deallocate a block of nReg consecutive registers.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GetTempRange(mut pParse: *mut Parse, mut nReg: i32) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut n: i32 = 0 as i32;
    if nReg == (1 as i32) {
        return sqlite3GetTempReg(pParse);
    }
    i = unsafe { (*pParse).iRangeReg };
    n = unsafe { (*pParse).nRangeReg };
    if nReg <= n {
        let __v2374: *mut Parse = pParse;
        let __v2375: i32 = unsafe { (*__v2374).iRangeReg };
        let __v2376: i32 = __v2375 + nReg;
        unsafe {
            (*__v2374).iRangeReg = __v2376;
        }
        let __v2377: *mut Parse = pParse;
        let __v2378: i32 = unsafe { (*__v2377).nRangeReg };
        let __v2379: i32 = __v2378 - nReg;
        unsafe {
            (*__v2377).nRangeReg = __v2379;
        }
    } else {
        i = (unsafe { (*pParse).nMem }) + (1 as i32);
        let __v2380: *mut Parse = pParse;
        let __v2381: i32 = unsafe { (*__v2380).nMem };
        let __v2382: i32 = __v2381 + nReg;
        unsafe {
            (*__v2380).nMem = __v2382;
        }
    }
    return i;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ReleaseTempRange(mut pParse: *mut Parse, mut iReg: i32, mut nReg: i32) {
    if nReg == (1 as i32) {
        sqlite3ReleaseTempReg(pParse, iReg);
        return;
    }
    {}
    if nReg > unsafe { (*pParse).nRangeReg } {
        unsafe {
            (*pParse).nRangeReg = nReg;
        }
        unsafe {
            (*pParse).iRangeReg = iReg;
        }
    }
}

/// Mark all temporary registers as being unavailable for reuse.
///
/// Always invoke this procedure after coding a subroutine or co-routine
/// that might be invoked from other parts of the code, to ensure that
/// the sub/co-routine does not use registers in common with the code that
/// invokes the sub/co-routine.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ClearTempRegCache(mut pParse: *mut Parse) {
    unsafe {
        (*pParse).nTempReg = ((0 as i32) as i8) as u8;
    }
    unsafe {
        (*pParse).nRangeReg = 0 as i32;
    }
}

/// Make sure sufficient registers have been allocated so that
/// iReg is a valid register number.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TouchRegister(mut pParse: *mut Parse, mut iReg: i32) {
    if (unsafe { (*pParse).nMem }) < iReg {
        unsafe {
            (*pParse).nMem = iReg;
        }
    }
}

// Validate that no temporary register falls within the range of
// iFirst..iLast, inclusive.  This routine is only call from within assert()
// statements.
