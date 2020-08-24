use std::{
    env,
    path::PathBuf,
    process::Command,
};


fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut dst = cmake::Config::new("sys");
    Command::new("git")
        .args(&["submodule", "update", "--init"])
        .current_dir(manifest_dir.clone())
        .status()
        .expect("Git is needed to retrieve the fltk source files!");

    Command::new("git")
        .args(&["checkout", "master"])
        .current_dir(manifest_dir.join("sys").join("soloud"))
        .status()
        .expect("Git is needed to retrieve the fltk source files!");

    if cfg!(feature = "use-ninja") {
        dst.generator("Ninja");
    }

    let _dst = dst
        .profile("Release")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
        .build();

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
}
