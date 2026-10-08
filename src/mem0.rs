//! 2008 October 28
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//!
//! This file contains a no-op memory allocation drivers for use when
//! SQLITE_ZERO_MALLOC is defined.  The allocation drivers implemented
//! here always fail.  SQLite will not operate with these drivers.  These
//! are merely placeholders.  Real drivers must be substituted using
//! sqlite3_config() before SQLite will operate.
// This version of the memory allocator is the default.  It is
// used when no other memory allocator is specified using compile-time
// macros.
