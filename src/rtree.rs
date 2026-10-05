// /*
// ** 2001 September 15
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// ** This file contains code for implementations of the r-tree and r*-tree
// ** algorithms packaged as an SQLite virtual table module.
// */
// /*
// ** Database Format of R-Tree Tables
// ** --------------------------------
// **
// ** The data structure for a single virtual r-tree table is stored in three
// ** native SQLite tables declared as follows. In each case, the '%' character
// ** in the table name is replaced with the user-supplied name of the r-tree
// ** table.
// **
// **   CREATE TABLE %_node(nodeno INTEGER PRIMARY KEY, data BLOB)
// **   CREATE TABLE %_parent(nodeno INTEGER PRIMARY KEY, parentnode INTEGER)
// **   CREATE TABLE %_rowid(rowid INTEGER PRIMARY KEY, nodeno INTEGER, ...)
// **
// ** The data for each node of the r-tree structure is stored in the %_node
// ** table. For each node that is not the root node of the r-tree, there is
// ** an entry in the %_parent table associating the node with its parent.
// ** And for each row of data in the table, there is an entry in the %_rowid
// ** table that maps from the entries rowid to the id of the node that it
// ** is stored on.  If the r-tree contains auxiliary columns, those are stored
// ** on the end of the %_rowid table.
// **
// ** The root node of an r-tree always exists, even if the r-tree table is
// ** empty. The nodeno of the root node is always 1. All other nodes in the
// ** table must be the same size as the root node. The content of each node
// ** is formatted as follows:
// **
// **   1. If the node is the root node (node 1), then the first 2 bytes
// **      of the node contain the tree depth as a big-endian integer.
// **      For non-root nodes, the first 2 bytes are left unused.
// **
// **   2. The next 2 bytes contain the number of entries currently
// **      stored in the node.
// **
// **   3. The remainder of the node contains the node entries. Each entry
// **      consists of a single 8-byte integer followed by an even number
// **      of 4-byte coordinates. For leaf nodes the integer is the rowid
// **      of a record. For internal nodes it is the node number of a
// **      child page.
// */
