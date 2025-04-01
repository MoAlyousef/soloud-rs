use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

pub fn build(out_dir: &Path, target_triple: &str) {
    println!("cargo:rerun-if-env-changed=ANDROID_SDK_ROOT");
    println!("cargo:rerun-if-env-changed=ANDROID_NDK_ROOT");

    let sdk =
        PathBuf::from(env::var("ANDROID_SDK_ROOT").expect("ANDROID_SDK_ROOT needs to be set!"));
    let mut ndk: Option<PathBuf> = None;
    if let Ok(root) = env::var("ANDROID_NDK_ROOT") {
        ndk = Some(PathBuf::from(root));
    }
    // fallback to NDK_HOME
    if ndk.is_none() {
        ndk = Some(PathBuf::from(
            env::var("NDK_HOME").expect("ANDROID_NDK_ROOT or NDK_HOME need to be set!"),
        ));
    }

    let ndk = ndk.expect("ANDROID_NDK_ROOT or NDK_HOME need to be set!");

    let cmake_build_dir = out_dir.join("cmake_build").to_str().unwrap().to_string();
    let mut cmd = vec![];
    cmd.push(format!("-B{}", cmake_build_dir));
    cmd.push("-DCMAKE_EXPORT_COMPILE_COMMANDS=ON".to_string());
    cmd.push("-DCMAKE_BUILD_TYPE=Release".to_string());
    cmd.push(format!(
        "-DCMAKE_INSTALL_PREFIX={}",
        out_dir.to_str().unwrap()
    ));
    cmd.push("-GNinja".to_string());
    cmd.push("-DCMAKE_SYSTEM_NAME=Android".to_string());
    cmd.push("-DCMAKE_SYSTEM_VERSION=21".to_string());
    cmd.push("-DANDROID_PLATFORM=android-21".to_string());
    cmd.push(format!("-DCMAKE_ANDROID_NDK={}", &ndk.to_str().unwrap()));
    cmd.push(format!("-DANDROID_NDK={}", &ndk.to_str().unwrap()));
    cmd.push(format!(
        "-DCMAKE_MAKE_PROGRAM={}",
        find_ninja(&sdk)
            .expect("Couldn't find NDK ninja!")
            .to_str()
            .unwrap()
    ));
    cmd.push(format!(
        "-DCMAKE_TOOLCHAIN_FILE={}",
        ndk.join("build")
            .join("cmake")
            .join("android.toolchain.cmake")
            .to_str()
            .unwrap()
    ));

    if cfg!(feature = "miniaudio") {
        cmd.push("-DWITH_MINIAUDIO=ON".to_string());
    } else if cfg!(feature = "alsa") {
        cmd.push("-DWITH_ALSA=ON".to_string());
    } else if cfg!(feature = "sdl2-static") {
        cmd.push("-DWITH_SDL2_STATIC=ON".to_string());
    } else if cfg!(feature = "sdl2") {
        cmd.push("-DWITH_SDL2=ON".to_string());
    } else if cfg!(feature = "openal") {
        cmd.push("-DWITH_OPENAL=ON".to_string());
    } else if cfg!(feature = "portaudio") {
        cmd.push("-DWITH_PORTAUDIO=ON".to_string());
    } else if cfg!(feature = "xaudio2") {
        cmd.push("-DWITH_XAUDIO2=ON".to_string());
    } else if cfg!(feature = "winmm") {
        cmd.push("-DWITH_WINMM=ON".to_string());
    } else if cfg!(feature = "wasapi") {
        cmd.push("-DWITH_WASAPI=ON".to_string());
    } else if cfg!(feature = "oss") {
        cmd.push("-DWITH_OSS=ON".to_string());
    } else if cfg!(feature = "opensles") {
        cmd.push("-DWITH_OPENSLES=ON".to_string());
    } else if cfg!(feature = "coreaudio") {
        cmd.push("-DWITH_COREAUDIO=ON".to_string());
    } else if cfg!(feature = "jack") {
        cmd.push("-DWITH_JACK=ON".to_string());
    } else if cfg!(feature = "nosound") {
        cmd.push("-DWITH_NOSOUND=ON".to_string());
    } else if cfg!(feature = "null") {
        cmd.push("-DWITH_NULL=ON".to_string());
    } else {
        panic!("Unsupported backend!");
    }

    match target_triple {
        "i686-linux-android" => {
            cmd.push("-DANDROID_ABI=x86".to_string());
            cmd.push("-DCMAKE_ANDROID_ARCH_ABI=x86".to_string());
        }
        "aarch64-linux-android" => {
            cmd.push("-DANDROID_ABI=arm64-v8a".to_string());
            cmd.push("-DCMAKE_ANDROID_ARCH_ABI=arm64-v8a".to_string());
        }
        "armv7-linux-androideabi" => {
            cmd.push("-DANDROID_ABI=armeabi-v7a".to_string());
            cmd.push("-DCMAKE_ANDROID_ARCH_ABI=armeabi-v7a".to_string());
        }
        "x86_64-linux-android" => {
            cmd.push("-DANDROID_ABI=x86_64".to_string());
            cmd.push("-DCMAKE_ANDROID_ARCH_ABI=x86_64".to_string());
        }
        _ => panic!("Unknown android triple"),
    }

    Command::new("cmake")
        .args(&cmd)
        .current_dir("sys")
        .status()
        .expect("CMake is needed for android builds!");

    Command::new("cmake")
        .args(["--build", &cmake_build_dir, "--target", "install"])
        .current_dir("sys")
        .status()
        .expect("CMake is needed for android builds!");
}

fn find_ninja(sdk_path: &Path) -> Option<PathBuf> {
    let cmk = sdk_path.join("cmake");
    for subdir in fs::read_dir(cmk).unwrap() {
        let subdir = subdir
            .unwrap() // Shouldn't fail!
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        if subdir.starts_with("3.") {
            return Some(
                sdk_path
                    .join("cmake")
                    .join(subdir)
                    .join("bin")
                    .join("ninja"),
            );
        }
    }
    None
}
