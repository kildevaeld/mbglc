use bindgen;
use cc;
use cmake;
use pkg_config::{probe_library, Config};
use std::env;
use std::path::{Path, PathBuf};

pub struct Info {
    libs: Vec<PathBuf>,
    include: Vec<PathBuf>,
}

fn build_local() -> PathBuf {
    let root = Path::new("../../");

    let dst = cmake::Config::new(root).build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    println!("cargo:rustc-link-lib=dylib=mbglc");

    // Info {
    //     libs: vec![dst.join("lib")],
    //     include: vec![dst.join("include")],
    // }
    dst.join("include")
}

// fn main() {
//     println!("cargo:rerun-if-changed=build.rs");
//     env::set_var("CC", "clang");
//     env::set_var("CXX", "clang++");

//     // let info = if let Ok(library) = probe_library("mbglc") {
//     //     Info {
//     //         libs: library.link_paths,
//     //         include: library.include_paths,
//     //     }
//     // } else {
//     //     build_local()
//     // };
//     let info = build_local();

//     let mut bindings = bindgen::Builder::default()
//         .header("./wrapper.h")
//         // Tell cargo to invalidate the built crate whenever any of the
//         // included header files changed.
//         .parse_callbacks(Box::new(bindgen::CargoCallbacks));
//     // Finish the builder and generate the bindings.

//     for i in &info.include {
//         bindings = bindings.clang_arg(format!("-I{}", i.to_string_lossy()));
//     }

//     let bindings = bindings
//         .generate()
//         // Unwrap the Result and panic on failure.
//         .expect("Unable to generate bindings");

//     // // The bindgen::Builder is the main entry point
//     // // to bindgen, and lets you build up options for
//     // // the resulting bindings.
//     // let bindings = bindgen::Builder::default()
//     //     // The input header we would like to generate
//     //     // bindings for.
//     //     .header("./wrapper.h")
//     //     // Tell cargo to invalidate the built crate whenever any of the
//     //     // included header files changed.
//     //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
//     //     // Finish the builder and generate the bindings.
//     //     .generate()
//     //     // Unwrap the Result and panic on failure.
//     //     .expect("Unable to generate bindings");

//     // Write the bindings to the $OUT_DIR/bindings.rs file.
//     let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//     bindings
//         .write_to_file(out_path.join("bindings.rs"))
//         .expect("Couldn't write bindings!");
//     // println!(
//     //     "cargo:rustc-link-search=native={}",
//     //     dst.join("lib").display()
//     // );
//     // println!("cargo:rustc-link-lib=dylib=mbglc");

//     let mut compiler = cc::Build::new();
//     compiler.file("./wrapper.c");

//     for i in info.include {
//         compiler.include(i);
//     }

//     compiler.compiler("libwrapper");
// }

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.c");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=../../src/mbglc.cpp");

    env::set_var("CC", "clang");
    env::set_var("CXX", "clang++");

    let include_path = if let Ok(lib) = probe_library("mbglc") {
        Path::new(lib.include_paths.first().unwrap()).to_path_buf()
    } else {
        build_local()
    };

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_arg(format!("-I{}", include_path.to_str().unwrap()))
        .header("./wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("wrapper.c")
        .include(include_path)
        .compile("libwrapper");
}
