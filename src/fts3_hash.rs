// /*
// ** 2001 September 22
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// ** This is the implementation of generic hash-tables used in SQLite.
// ** We've modified it slightly to serve as a standalone hash table
// ** implementation for the full-text indexing module.
// */
// /*
// ** The code in this file is only compiled if:
// **
// **     * The FTS3 module is being built as an extension
// **       (in which case SQLITE_CORE is not defined), or
// **
// **     * The FTS3 module is being built into the core of
// **       SQLite (in which case SQLITE_ENABLE_FTS3 is defined).
// */
