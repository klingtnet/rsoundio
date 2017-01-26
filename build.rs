extern crate pkg_config;
use std::process::Command;
use std::path::PathBuf;
use std::env;
use std::fs;

macro_rules! sio {
    ($expr:expr) => { format!("libsoundio-{}", $expr) }
}

macro_rules! err_exists {
    ($expr:expr, $msg:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                match err.kind() {
                    ::std::io::ErrorKind::AlreadyExists => (),
                    _ => panic!(format!("{}: {}", $msg, err)),
                }
            }
        }
    }
}

fn lib_available(name: &str) -> bool {
    match pkg_config::find_library(name) {
        Ok(_) => true,
        Err(_) => {
            let res = Command::new("ldconfig").arg("--print-cache").output().unwrap();
            if res.status.success() {
                String::from_utf8(res.stdout).unwrap().contains(name)
            } else {
                false
            }
        }
    }
}

fn windows(_: String) {
    // TODO: Use precompiled binary!
    unimplemented!();
}

fn osx(_: String) {
    // TODO: I down't own any apple hardware to test this.
    unimplemented!();
}

fn linux(target: String) {
    if !lib_available("soundio") {
        build(target);
    }
}

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        windows(target)
    } else if target.contains("apple") {
        osx(target)
    } else {
        // assume the rest is linux
        linux(target)
    }
}

fn sio_url(ext: &'static str) -> String {
    match ext {
        "tar.gz" | "zip" => format!("http://libsound.io/release/{}.{}", sio!("1.1.0"), ext),
        _ => panic!(format!("No release for format: {}", ext)),
    }
}

fn build(target: String) {
    let host = env::var("HOST").unwrap();
    let dst_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let lib_dir = dst_dir.join("lib");
    let include_dir = dst_dir.join("include");
    err_exists!(fs::create_dir(&lib_dir), &lib_dir.display());
    err_exists!(fs::create_dir(&include_dir), &include_dir.display());

    // set cargo flags
    println!("cargo:rustc-link-lib=dylib={}", "soundio"); // -l
    println!("cargo:rustc-link-search=native={}", &lib_dir.display()); // -L
    println!("cargo:include={}", &include_dir.display());
    println!("cargo:root={}", &dst_dir.display());

    // download and extract libsoundio source
    Command::new("curl")
        .current_dir(&dst_dir)
        .args(&["--location", "--remote-name"])
        .arg(sio_url("tar.gz"))
        .output()
        .unwrap();
    Command::new("tar")
        .current_dir(&dst_dir)
        .arg("-xvzf")
        .arg(format!("{}.{}", sio!("1.1.0"), "tar.gz"))
        .output()
        .unwrap();

    // create build dir
    let soundio_root = dst_dir.join(sio!("1.1.0"));
    let build_dir = soundio_root.join("build");
    err_exists!(fs::create_dir(&build_dir), &build_dir.display());

    // TODO:set build type to release for env var PROFILE={release,bench}
    // run cmake
    Command::new("cmake")
        .current_dir(&build_dir)
        .arg("-DCMAKE_BUILD_TYPE=Debug")
        .arg("-DCMAKE_INSTALL_LIBDIR:PATH=lib")
        .arg(format!("-DCMAKE_INSTALL_PREFIX:PATH={}", &dst_dir.display()))
        .arg("-DBUILD_EXAMPLE_PROGRAMS:BOOL=OFF")
        .arg("-DBUILD_TESTS:BOOL=OFF")
        .arg("-DBUILD_STATIC_LIBS:BOOL=OFF")
        .arg("-DBUILD_SHARED_LIBS:BOOL=ON")
        .arg("-DENABLE_JACK:BOOL=OFF")
        .arg("-DENABLE_PULSEAUDIO:BOOL=OFF")
        .arg("-DCMAKE_POSITION_INDEPENDENT_CODE:BOOL=ON")
        .arg("..")
        .output()
        .unwrap();

    // make install
    Command::new("make")
        .current_dir(&build_dir)
        .arg("install")
        .output()
        .unwrap();

    // remove builddir
    fs::remove_dir_all(build_dir).unwrap();
}
