extern crate cmake;
extern crate pkg_config;
use std::process::Command;
use std::path::PathBuf;
use std::env;
use std::fs;

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
    } else {
        // TODO: remove else branch
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

const LIBSOUNDIO_TAR: &'static str = "http://libsound.io/release/libsoundio-1.1.0.tar.gz";
const LIBSOUNDIO_WIN: &'static str = "http://libsound.io/release/libsoundio-1.1.0.zip";

fn build(target: String) {
    let host = env::var("HOST").unwrap();
    let dst_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dst_root = format!("{}", &dst_dir.display());

    // set cargo flags
    println!("cargo:rustc-link-search={}/lib", &dst_root);
    println!("cargo:rustc-link-lib=static={}", "soundio");
    println!("cargo:include={}/include", &dst_root);
    println!("cargo:root={}", &dst_root);

    // download and extract libsoundio source
    Command::new("curl")
        .current_dir(&dst_dir)
        .args(&["--location", "--remote-name"])
        .arg(LIBSOUNDIO_TAR)
        .output()
        .unwrap();
    Command::new("tar")
        .current_dir(&dst_dir)
        .arg("-xvzf")
        .arg("libsoundio-1.1.0.tar.gz")
        .output()
        .unwrap();

    // create build dir
    let soundio_root = dst_dir.join("libsoundio-1.1.0");
    let build_dir = soundio_root.join("build");
    fs::create_dir(&build_dir).unwrap();

    let cc = env::var("CMAKE_C_COMPILER").unwrap();
    let cxx = env::var("CMAKE_CXX_COMPILER").unwrap();

    // run cmake
    cmake::Config::new(&soundio_root)
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("CMAKE_INSTALL_PREFIX", format!("{}", &dst_root))
        .define("BUILD_EXAMPLE_PROGRAMS", "OFF")
        .define("BUILD_TESTS", "OFF")
        .define("BUILD_STATIC_LIBS", "ON")
        .define("ENABLE_JACK", "OFF")
        .define("CMAKE_C_COMPILER", cc)
        .define("CMAKE_CXX_COMPILER", cxx)
        .build();

    // make install
    Command::new("make")
        .current_dir(&build_dir)
        .arg("install")
        .output()
        .unwrap();

    // remove builddir
    fs::remove_dir_all(soundio_root).unwrap();
}
