use std::{env, path::Path};

fn main() {
    // Disable msvcrt.lib
    let dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let dir = Path::new(&dir).join("libs");
    println!("cargo:rustc-link-search=native={}", dir.display());
    println!();
}
