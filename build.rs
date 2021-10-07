//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
extern crate bindgen;
extern crate cc;
fn main() {

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    println!("cargo:rerun-if-changed=c_libs/sh2");
    cc::Build::new()
        .flag("-mthumb")
        .flag("-mcpu=cortex-m4")
        .flag("-mfpu=fpv4-sp-d16")
        .flag("-mfloat-abi=hard")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-Os")
        .file("c_libs/sh2/sh2.c")
        .file("c_libs/sh2/shtp.c")
        .file("c_libs/sh2/sh2_util.c")
        .file("c_libs/sh2/sh2_SensorValue.c")
        .compile("libsh2.a");

    let _bindings = bindgen::Builder::default()
        //--use-core instead std
        .use_core()
        //--ctypes-prefix=cty
        .ctypes_prefix("cty")
        // The path to arm-none-eabi libc headers
        .clang_arg("-I/Users/matvei/libs/arm/gcc-arm-none-eabi-10-2020-q4-major/include")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let b_path = PathBuf::from("src/bindings");
    _bindings
        .write_to_file(b_path.join("shtp.rs"))
        .expect("Couldn't write bindings!");
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");
}
