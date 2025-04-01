use std::path::Path;

pub fn link(out_dir: &Path) {
    if cfg!(feature = "alsa") {
        println!("cargo:rustc-link-lib=dylib=asound");
    } else if cfg!(feature = "sdl2-static") {
        println!("cargo:rustc-link-lib=static=SDL2");
    } else if cfg!(feature = "sdl2") {
        println!("cargo:rustc-link-lib=dylib=SDL2");
    } else if cfg!(feature = "openal") {
        println!("cargo:rustc-link-lib=openal");
    } else if cfg!(feature = "portaudio") {
        println!("cargo:rustc-link-lib=portaudio");
    } else if cfg!(feature = "xaudio2") {
        println!("cargo:rustc-link-lib=xaudio2");
    } else if cfg!(feature = "winmm") {
        println!("cargo:rustc-link-lib=winmm");
    } else if cfg!(feature = "wasapi") {
        println!("cargo:rustc-link-lib=win-wasapi");
    } else if cfg!(feature = "oss") {
        println!("cargo:rustc-link-lib=oss");
    } else if cfg!(feature = "opensles") {
        println!("cargo:rustc-link-lib=OpenSLES");
    } else if cfg!(feature = "jack") {
        println!("cargo:rustc-link-lib=jack");
    } else {
        // nothing to link
    }

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
