use std::env;
use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Put the linker script somewhere the linker can find it
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
}
