#![allow(unused_variables)]

use std::{
    env,
    path::PathBuf,
    process::Command,
};


fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut dst = cmake::Config::new("sys");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=sys/CMakeLists.txt");
    println!("cargo:rerun-if-changed=sys/soloud_new.cpp");
    println!("cargo:rerun-if-changed=sys/soloud_derives.h");
    println!("cargo:rerun-if-changed=sys/soloud_derives.cpp");

    Command::new("git")
        .args(&["submodule", "update", "--init"])
        .current_dir(manifest_dir.clone())
        .status()
        .expect("Git is needed to retrieve the soloud source files!");

    Command::new("git")
        .args(&["checkout", "master"])
        .current_dir(manifest_dir.join("sys").join("soloud"))
        .status()
        .expect("Git is needed to retrieve the soloud source files!");

    Command::new("git")
        .args(&["apply", "../soloud.patch"])
        .current_dir(manifest_dir.join("sys").join("soloud"))
        .status()
        .expect("Git is needed to retrieve the soloud source files!");

    if cfg!(feature = "use-ninja") {
        dst.generator("Ninja");
    }

    if let Ok(toolchain) = env::var("SOLOUD_TOOLCHAIN") {
        dst.define("CMAKE_TOOLCHAIN_FILE", &toolchain);
    }

    let _dst = dst
        .profile("Release")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
        .build();

    Command::new("git")
        .args(&["reset", "--hard", "origin/master"])
        .current_dir(manifest_dir.join("sys").join("soloud"))
        .status()
        .expect("Git is needed to retrieve the fltk source files!");

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("build").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib").join("Release").display()
    );

    println!("cargo:rustc-link-lib=static=soloud");
}
