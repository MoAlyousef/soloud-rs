use std::{env, path::PathBuf};

mod android;
mod link;
mod source;

fn main() {
    println!("cargo:rerun-if-changed=build/android.rs");
    println!("cargo:rerun-if-changed=build/link.rs");
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-changed=build/source.rs");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_triple = env::var("TARGET").unwrap();

    source::build(&manifest_dir, &target_triple, &out_dir);
    link::link(&out_dir);
}
