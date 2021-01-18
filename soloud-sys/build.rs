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

    if cfg!(feature = "miniaudio") {
        dst.define("WITH_MINIAUDIO", "ON");
    }

    if cfg!(feature = "alsa") {
        dst.define("WITH_ALSA", "ON");
    }

    if cfg!(feature = "sdl2-static") {
        dst.define("WITH_SDL2_STATIC", "ON");
    }

    if cfg!(feature = "sdl2") {
        dst.define("WITH_SDL2", "ON");
    }

    if cfg!(feature = "openal") {
        dst.define("WITH_OPENAL", "ON");
    }

    if cfg!(feature = "portaudio") {
        dst.define("WITH_PORTAUDIO", "ON");
    }

    if cfg!(feature = "xaudio2") {
        dst.define("WITH_XAUDIO2", "ON");
    }

    if cfg!(feature = "winmm") {
        dst.define("WITH_WINMM", "ON");
    }

    if cfg!(feature = "wasapi") {
        dst.define("WITH_WASAPI", "ON");
    }

    if cfg!(feature = "oss") {
        dst.define("WITH_OSS", "ON");
    }

    if cfg!(feature = "opensles") {
        dst.define("WITH_OPENSLES", "ON");
    }

    if cfg!(feature = "coreaudio") {
        dst.define("WITH_COREAUDIO", "ON");
    }

    if cfg!(feature = "jack") {
        dst.define("WITH_JACK", "ON");
    }

    if cfg!(feature = "nosound") {
        dst.define("WITH_NOSOUND", "ON");
    }

    if cfg!(feature = "null") {
        dst.define("WITH_NULL", "ON");
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
    
    if cfg!(feature = "alsa") {
        println!("cargo:rustc-link-lib=dylib=asound");
    }

    if cfg!(feature = "sdl2-static") {
        println!("cargo:rustc-link-lib=static=SDL2");
    }

    if cfg!(feature = "sdl2") {
        println!("cargo:rustc-link-lib=dylib=SDL2");
    }

    if cfg!(feature = "openal") {
        println!("cargo:rustc-link-lib=openal");
    }

    if cfg!(feature = "portaudio") {
        println!("cargo:rustc-link-lib=portaudio");
    }

    if cfg!(feature = "xaudio2") {
        println!("cargo:rustc-link-lib=xaudio2");
    }

    if cfg!(feature = "winmm") {
        println!("cargo:rustc-link-lib=winmm");
    }

    if cfg!(feature = "wasapi") {
        println!("cargo:rustc-link-lib=win-wasapi");
    }

    if cfg!(feature = "oss") {
        println!("cargo:rustc-link-lib=oss");
    }

    if cfg!(feature = "opensles") {
        println!("cargo:rustc-link-lib=OpenSLES");
    }

    if cfg!(feature = "jack") {
        println!("cargo:rustc-link-lib=jack");
    }
}
