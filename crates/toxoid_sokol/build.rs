extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let sokol_headers_path = Path::new(&manifest_dir).join("lib");
    let target = env::var("TARGET").unwrap();
    let sokol_gp_wrapper = sokol_headers_path.join("sokol_gp_wrapper.c");
    let sokol_gp_wrapper = sokol_gp_wrapper.to_str().unwrap();
    
    // Rebuild on build.rs change
    println!("cargo:rerun-if-changed=build.rs");
    println!("{}", format!("cargo:rerun-if-changed={}", sokol_gp_wrapper));


    if !target.contains("emscripten") {
        let _bindings = bindgen::Builder::default()
            .header(sokol_headers_path.join("sokol_app.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol_gfx.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol_glue.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol_log.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol_time.h").to_str().unwrap())
            .header(sokol_headers_path.join("stb_image.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol_gp.h").to_str().unwrap())
            .generate()
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        // let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        // let out_path = PathBuf::from("./src");
        // bindings
        //     .write_to_file(out_path.join("bindings.rs"))
        //     .expect("Couldn't write bindings!");
    }
    
    let mut build = cc::Build::new();
    // TODO: Match on target_os, for now target GLES3 for Emscripten
    build.define("SOKOL_GLES3", None);
    build.file(sokol_gp_wrapper);
    build.compile("toxoid_sokol");
}