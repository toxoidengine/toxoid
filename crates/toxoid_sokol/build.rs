extern crate bindgen;
extern crate cc;

use std::env::var;
use std::path::{PathBuf, Path};

fn main() {
    // Rebuild on build.rs change
    println!("cargo:rerun-if-changed=build.rs");
    // Get the current directory so we can find the sokol headers
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    // Get the path to the sokol headers
    let sokol_headers_path = Path::new(&manifest_dir).join("lib");
    // Check what target we are building for since cfg
    // does not work for build.rs
    let target = var("TARGET").unwrap();
    let bindings = bindgen::Builder::default()
        .header(sokol_headers_path.join("sokol").join("sokol_app.h").to_str().unwrap())
        .header(sokol_headers_path.join("sokol").join("sokol_log.h").to_str().unwrap())
        .header(sokol_headers_path.join("sokol").join("sokol_time.h").to_str().unwrap())
        .header(sokol_headers_path.join("sokol").join("sokol_gfx.h").to_str().unwrap())
        .header(sokol_headers_path.join("sokol").join("sokol_glue.h").to_str().unwrap())
        .header(sokol_headers_path.join("sokol_gp").join("sokol_gp.h").to_str().unwrap())
        .header(sokol_headers_path.join("stb_image.h").to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(var("OUT_DIR").unwrap());
    let out_path = PathBuf::from("./src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let mut build = cc::Build::new();
    // Flags
    build
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-missing-field-initializers");

    // For Metal on macOS, we need to compile as Objective-C
    if target.contains("darwin") {
        build
            .define("SOKOL_METAL", None)
            .flag("-x")
            .flag("objective-c");
    } else if target.contains("windows") {
        build.define("SOKOL_D3D11", None);
    } else if target.contains("linux") {
        build.define("SOKOL_GLCORE33", None);
    } else if target.contains("android") {
        build.define("SOKOL_GLES3", None);
    } else if target.contains("ios") {
        build
            .define("SOKOL_METAL", None)
            .flag("-x")
            .flag("objective-c");
    }
    // If fetch feature is enabled, define TOXOID_FETCH
    if var("CARGO_FEATURE_FETCH").is_ok() {
        build.define("TOXOID_FETCH", None);
    }
    // If audio feature is enabled, define TOXOID_AUDIO
    if var("CARGO_FEATURE_AUDIO").is_ok() {
        build.define("TOXOID_AUDIO", None);
    }
    // // If imgui feature is enabled, add imgui files
    // if var("CARGO_FEATURE_IMGUI").is_ok() {
    //     build
    //         .define("TOXOID_IMGUI", None)
    //         .define("IMGUI_DISABLE_OBSOLETE_FUNCTIONS", None)
    //         .files(imgui_files);
    // }
    // // If spine feature is enabled, add spine files
    // if var("CARGO_FEATURE_SPINE").is_ok() {
    //     build
    //         .define("TOXOID_SPINE", None)
    //         .include("lib/spine-runtimes/spine-c/spine-c/include")
    //         .files(spine_files);
    // }
    if var("CARGO_FEATURE_STB").is_ok() {
        build.define("TOXOID_STB", None);
    }
    build
        .file(sokol_headers_path.join("sokol_wrapper.c"))
        .compile("toxoid_sokol");
}
