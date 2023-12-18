extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
use std::path::Path;

fn main() {
    // Rebuild on build.rs change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./lib/sokol_gp_wrapper.c");

    // let emsdk_path = "C:\\Users\\troye\\dev\\bin\\emsdk\\upstream\\emscripten"; // Replace with your actual emsdk path
    // let current_path = env::var("PATH").unwrap_or_default();
    // let new_path = format!("{};{}", emsdk_path, current_path);
    // env::set_var("PATH", &new_path);

    // let target = env::var("TARGET").unwrap();
    // if target.contains("emscripten") {}

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    // let bindings = bindgen::Builder::default()
    //     // The input header we would like to generate
    //     // bindings for.
    //     .header("./lib/sokol_app.h")
    //     .header("./lib/sokol_gfx.h")
    //     .header("./lib/sokol_glue.h")
    //     .header("./lib/sokol_log.h")
    //     .header("./lib/sokol_time.h")
    //     .header("./lib/stb_image.h")
    //     .header("./lib/sokol_gp.h")
    //     // .clang_arg("-target")
    //     // .clang_arg("wasm32-unknown-emscripten")
    //     .generate()
    //     // Unwrap the Result and panic on failure.
    //     .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // let out_path = PathBuf::from("./src");
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");

    let mut build = cc::Build::new();
    // TODO: Match on target_os, for now target GLES3 for Emscripten
    build.define("SOKOL_GLES3", None);
    build.file("./lib/sokol_gp_wrapper.c");
    build.compile("toxoid_sokol");
}