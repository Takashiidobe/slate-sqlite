fn main() {
    for lib in ["z", "readline", "ncurses", "m", "dl"] {
        println!("cargo:rustc-link-lib={lib}");
        println!("cargo:rustc-link-arg-bin=slate-sqlite=-l{lib}");
    }
}
