//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    println!("cargo:rustc-link-search={}", out.display());
    println!(
        "cargo:rustc-link-search={}/lib",
        env::current_dir().unwrap().to_string_lossy()
    );
}
