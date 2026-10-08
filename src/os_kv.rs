//! 2022-09-06
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
//! This file contains an experimental VFS layer that operates on a
//! Key/Value storage engine where both keys and values must be pure
//! text.
//!
//! DEBUG AND TEST
//!
//! For testing on Unix, compile using:
//!
//!    make clean sqlite3d CFLAGS='-DSQLITE_OS_KV_OPTIONAL'
//!
//! Then start up a shell using something like:
//!
//!    ./sqlite3d 'file:dbname?vfs=kvvfs'
//!
//! Each K/V entry is stored in a separate file in the working
//! directory that has a name like "kvvfs-dbname-*".  Due to limitations
//! on the key size, the name of the database must be very short - just
//! a few characters.  If the database name is too long, the VFS will
//! malfunction and you will get SQLITE_CORRUPT errors.
