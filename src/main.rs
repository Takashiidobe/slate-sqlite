#![allow(non_camel_case_types)]
#![feature(atomic_volatile)]
#![no_main]
#[path = "alter.rs"]
mod __slate_unit_alter;
#[path = "analyze.rs"]
mod __slate_unit_analyze;
#[path = "attach.rs"]
mod __slate_unit_attach;
#[path = "auth.rs"]
mod __slate_unit_auth;
#[path = "backup.rs"]
mod __slate_unit_backup;
#[path = "bitvec.rs"]
mod __slate_unit_bitvec;
#[path = "btmutex.rs"]
mod __slate_unit_btmutex;
#[path = "btree.rs"]
mod __slate_unit_btree;
#[path = "build.rs"]
mod __slate_unit_build;
#[path = "callback.rs"]
mod __slate_unit_callback;
#[path = "carray.rs"]
mod __slate_unit_carray;
#[path = "complete.rs"]
mod __slate_unit_complete;
#[path = "ctime.rs"]
mod __slate_unit_ctime;
#[path = "date.rs"]
mod __slate_unit_date;
#[path = "dbpage.rs"]
mod __slate_unit_dbpage;
#[path = "dbstat.rs"]
mod __slate_unit_dbstat;
#[path = "delete.rs"]
mod __slate_unit_delete;
#[path = "expr.rs"]
mod __slate_unit_expr;
#[path = "fault.rs"]
mod __slate_unit_fault;
#[path = "fkey.rs"]
mod __slate_unit_fkey;
#[path = "fts3.rs"]
mod __slate_unit_fts3;
#[path = "fts3_aux.rs"]
mod __slate_unit_fts3_aux;
#[path = "fts3_expr.rs"]
mod __slate_unit_fts3_expr;
#[path = "fts3_hash.rs"]
mod __slate_unit_fts3_hash;
#[path = "fts3_icu.rs"]
mod __slate_unit_fts3_icu;
#[path = "fts3_porter.rs"]
mod __slate_unit_fts3_porter;
#[path = "fts3_snippet.rs"]
mod __slate_unit_fts3_snippet;
#[path = "fts3_tokenize_vtab.rs"]
mod __slate_unit_fts3_tokenize_vtab;
#[path = "fts3_tokenizer.rs"]
mod __slate_unit_fts3_tokenizer;
#[path = "fts3_tokenizer1.rs"]
mod __slate_unit_fts3_tokenizer1;
#[path = "fts3_unicode.rs"]
mod __slate_unit_fts3_unicode;
#[path = "fts3_unicode2.rs"]
mod __slate_unit_fts3_unicode2;
#[path = "fts3_write.rs"]
mod __slate_unit_fts3_write;
#[path = "fts5.rs"]
mod __slate_unit_fts5;
#[path = "func.rs"]
mod __slate_unit_func;
#[path = "global.rs"]
mod __slate_unit_global;
#[path = "hash.rs"]
mod __slate_unit_hash;
#[path = "icu.rs"]
mod __slate_unit_icu;
#[path = "insert.rs"]
mod __slate_unit_insert;
#[path = "json.rs"]
mod __slate_unit_json;
#[path = "legacy.rs"]
mod __slate_unit_legacy;
#[path = "loadext.rs"]
mod __slate_unit_loadext;
#[path = "__slate_unit_main.rs"]
mod __slate_unit_main;
#[path = "malloc.rs"]
mod __slate_unit_malloc;
#[path = "mem0.rs"]
mod __slate_unit_mem0;
#[path = "mem1.rs"]
mod __slate_unit_mem1;
#[path = "mem2.rs"]
mod __slate_unit_mem2;
#[path = "mem3.rs"]
mod __slate_unit_mem3;
#[path = "mem5.rs"]
mod __slate_unit_mem5;
#[path = "memdb.rs"]
mod __slate_unit_memdb;
#[path = "memjournal.rs"]
mod __slate_unit_memjournal;
#[path = "mutex.rs"]
mod __slate_unit_mutex;
#[path = "mutex_noop.rs"]
mod __slate_unit_mutex_noop;
#[path = "mutex_unix.rs"]
mod __slate_unit_mutex_unix;
#[path = "mutex_w32.rs"]
mod __slate_unit_mutex_w32;
#[path = "notify.rs"]
mod __slate_unit_notify;
#[path = "opcodes.rs"]
mod __slate_unit_opcodes;
#[path = "os.rs"]
mod __slate_unit_os;
#[path = "os_kv.rs"]
mod __slate_unit_os_kv;
#[path = "os_unix.rs"]
mod __slate_unit_os_unix;
#[path = "os_win.rs"]
mod __slate_unit_os_win;
#[path = "pager.rs"]
mod __slate_unit_pager;
#[path = "parse.rs"]
mod __slate_unit_parse;
#[path = "pcache.rs"]
mod __slate_unit_pcache;
#[path = "pcache1.rs"]
mod __slate_unit_pcache1;
#[path = "pragma.rs"]
mod __slate_unit_pragma;
#[path = "prepare.rs"]
mod __slate_unit_prepare;
#[path = "printf.rs"]
mod __slate_unit_printf;
#[path = "random.rs"]
mod __slate_unit_random;
#[path = "resolve.rs"]
mod __slate_unit_resolve;
#[path = "rowset.rs"]
mod __slate_unit_rowset;
#[path = "rtree.rs"]
mod __slate_unit_rtree;
#[path = "select.rs"]
mod __slate_unit_select;
#[path = "sqlite3rbu.rs"]
mod __slate_unit_sqlite3rbu;
#[path = "sqlite3session.rs"]
mod __slate_unit_sqlite3session;
#[path = "status.rs"]
mod __slate_unit_status;
#[path = "stmt.rs"]
mod __slate_unit_stmt;
#[path = "table.rs"]
mod __slate_unit_table;
#[path = "threads.rs"]
mod __slate_unit_threads;
#[path = "tokenize.rs"]
mod __slate_unit_tokenize;
#[path = "treeview.rs"]
mod __slate_unit_treeview;
#[path = "trigger.rs"]
mod __slate_unit_trigger;
#[path = "update.rs"]
mod __slate_unit_update;
#[path = "upsert.rs"]
mod __slate_unit_upsert;
#[path = "utf.rs"]
mod __slate_unit_utf;
#[path = "util.rs"]
mod __slate_unit_util;
#[path = "vacuum.rs"]
mod __slate_unit_vacuum;
#[path = "vdbe.rs"]
mod __slate_unit_vdbe;
#[path = "vdbeapi.rs"]
mod __slate_unit_vdbeapi;
#[path = "vdbeaux.rs"]
mod __slate_unit_vdbeaux;
#[path = "vdbeblob.rs"]
mod __slate_unit_vdbeblob;
#[path = "vdbemem.rs"]
mod __slate_unit_vdbemem;
#[path = "vdbesort.rs"]
mod __slate_unit_vdbesort;
#[path = "vdbetrace.rs"]
mod __slate_unit_vdbetrace;
#[path = "vdbevtab.rs"]
mod __slate_unit_vdbevtab;
#[path = "vtab.rs"]
mod __slate_unit_vtab;
#[path = "wal.rs"]
mod __slate_unit_wal;
#[path = "walker.rs"]
mod __slate_unit_walker;
#[path = "where.rs"]
mod __slate_unit_where;
#[path = "wherecode.rs"]
mod __slate_unit_wherecode;
#[path = "whereexpr.rs"]
mod __slate_unit_whereexpr;
#[path = "window.rs"]
mod __slate_unit_window;

mod shell;
