# SQLite translated with Slate

SQLite translated from C to Rust with [Slate](https://github.com/takashiidobe/slate).
The repository contains the generated SQLite core and command-line shell, plus
local Rust support crates.

## Build

This translation targets x86_64 Linux with glibc and requires Rust nightly,
zlib, GNU readline, and ncurses development libraries. On Debian or Ubuntu:

```sh
sudo apt-get install build-essential zlib1g-dev libreadline-dev libncurses-dev
rustup toolchain install nightly
cargo +nightly build --release --locked
```

The build produces `target/release/slate-sqlite` (the SQLite shell) and
`target/release/libsqlite_generated.so` (the SQLite core with its C ABI).

```sh
cargo r --release -- :memory: 'SELECT sqlite_version();'
cargo r --release -- example.db
```

## Validation

The translated version passed upstream `select1.test` and `main.test`
using `testfixture`, as well as `mptest` and `threadtest3`. These test harnesses
are not included in this repository.

The Rust sources are generated translation output, intended as an example of
using Slate to translate a C project to Rust.
