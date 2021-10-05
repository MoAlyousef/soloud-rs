#![allow(unused_variables)]

use std::{env, path::PathBuf, process::Command};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut dst = cmake::Config::new("sys");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=CXX");
    println!("cargo:rerun-if-env-changed=CFLAGS");
    println!("cargo:rerun-if-env-changed=CXXFLAGS");
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

    if cfg!(feature = "use-ninja") || has_program("ninja") {
        dst.generator("Ninja");
    }

    if cfg!(feature = "miniaudio") {
        dst.define("WITH_MINIAUDIO", "ON");
    } else if cfg!(feature = "alsa") {
        dst.define("WITH_ALSA", "ON");
        println!("cargo:rustc-link-lib=dylib=asound");
    } else if cfg!(feature = "sdl2-static") {
        dst.define("WITH_SDL2_STATIC", "ON");
        println!("cargo:rustc-link-lib=static=SDL2");
    } else if cfg!(feature = "sdl2") {
        dst.define("WITH_SDL2", "ON");
        println!("cargo:rustc-link-lib=dylib=SDL2");
    } else if cfg!(feature = "openal") {
        dst.define("WITH_OPENAL", "ON");
        println!("cargo:rustc-link-lib=openal");
    } else if cfg!(feature = "portaudio") {
        dst.define("WITH_PORTAUDIO", "ON");
        println!("cargo:rustc-link-lib=portaudio");
    } else if cfg!(feature = "xaudio2") {
        dst.define("WITH_XAUDIO2", "ON");
        println!("cargo:rustc-link-lib=xaudio2");
    } else if cfg!(feature = "winmm") {
        dst.define("WITH_WINMM", "ON");
        println!("cargo:rustc-link-lib=winmm");
    } else if cfg!(feature = "wasapi") {
        dst.define("WITH_WASAPI", "ON");
        println!("cargo:rustc-link-lib=win-wasapi");
    } else if cfg!(feature = "oss") {
        dst.define("WITH_OSS", "ON");
        println!("cargo:rustc-link-lib=oss");
    } else if cfg!(feature = "opensles") {
        dst.define("WITH_OPENSLES", "ON");
        println!("cargo:rustc-link-lib=OpenSLES");
    } else if cfg!(feature = "coreaudio") {
        dst.define("WITH_COREAUDIO", "ON");
    } else if cfg!(feature = "jack") {
        dst.define("WITH_JACK", "ON");
        println!("cargo:rustc-link-lib=jack");
    } else if cfg!(feature = "nosound") {
        dst.define("WITH_NOSOUND", "ON");
    } else if cfg!(feature = "null") {
        dst.define("WITH_NULL", "ON");
    } else {
        panic!("Unsupported backend!");
    }

    if let Ok(toolchain) = env::var("SOLOUD_TOOLCHAIN") {
        dst.define("CMAKE_TOOLCHAIN_FILE", &toolchain);
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

    println!("cargo:rustc-link-lib=static=soloud");
}

fn has_program(prog: &str) -> bool {
    match std::process::Command::new(prog).arg("--version").output() {
        Ok(out) => !out.stdout.is_empty(),
        _ => false,
    }
}