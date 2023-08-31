use std::{env, path::Path};

pub fn build(target_triple: &str, out_dir: &Path) {
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=CXX");
    println!("cargo:rerun-if-env-changed=CFLAGS");
    println!("cargo:rerun-if-env-changed=CXXFLAGS");
    println!("cargo:rerun-if-changed=sys/CMakeLists.txt");
    println!("cargo:rerun-if-changed=sys/soloud_new.cpp");
    println!("cargo:rerun-if-changed=sys/soloud_derives.h");
    println!("cargo:rerun-if-changed=sys/soloud_derives.cpp");

    if target_triple.contains("android") {
        crate::android::build(out_dir, target_triple);
    } else {
        let mut dst = cmake::Config::new("sys");

        if cfg!(feature = "use-ninja") || has_program("ninja") {
            dst.generator("Ninja");
        }
    
        if cfg!(feature = "miniaudio") {
            dst.define("WITH_MINIAUDIO", "ON");
        } else if cfg!(feature = "alsa") {
            dst.define("WITH_ALSA", "ON");
        } else if cfg!(feature = "sdl2-static") {
            dst.define("WITH_SDL2_STATIC", "ON");
        } else if cfg!(feature = "sdl2") {
            dst.define("WITH_SDL2", "ON");
        } else if cfg!(feature = "openal") {
            dst.define("WITH_OPENAL", "ON");
        } else if cfg!(feature = "portaudio") {
            dst.define("WITH_PORTAUDIO", "ON");
        } else if cfg!(feature = "xaudio2") {
            dst.define("WITH_XAUDIO2", "ON");
        } else if cfg!(feature = "winmm") {
            dst.define("WITH_WINMM", "ON");
        } else if cfg!(feature = "wasapi") {
            dst.define("WITH_WASAPI", "ON");
        } else if cfg!(feature = "oss") {
            dst.define("WITH_OSS", "ON");
        } else if cfg!(feature = "opensles") {
            dst.define("WITH_OPENSLES", "ON");
        } else if cfg!(feature = "coreaudio") {
            dst.define("WITH_COREAUDIO", "ON");
        } else if cfg!(feature = "jack") {
            dst.define("WITH_JACK", "ON");
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
            .profile("Debug")
            .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
            .build();
    }
}

fn has_program(prog: &str) -> bool {
    match std::process::Command::new(prog).arg("--version").output() {
        Ok(out) => !out.stdout.is_empty(),
        _ => false,
    }
}