extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
use std::path::Path;

fn main() {
    // Get the current directory so we can find the sokol headers
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // Get the path to the sokol headers
    let sokol_headers_path = Path::new(&manifest_dir).join("lib");
    
    // Root Sokol wrapper
    let sokol_gp_wrapper = sokol_headers_path.join("sokol_gp_wrapper.c");
    let sokol_gp_wrapper = sokol_gp_wrapper.to_str().unwrap();

    // ImGui
    let sokol_imgui_path = sokol_headers_path.join("cimgui").join("imgui"); 
    let lib_imgui_tables = sokol_imgui_path.join("imgui_tables.cpp");
    let lib_imgui_widgets = sokol_imgui_path.join("imgui_widgets.cpp");
    let lib_imgui_draw = sokol_imgui_path.join("imgui_draw.cpp");
    let lib_imgui = sokol_imgui_path.join("imgui.cpp");
    let lib_c_imgui = sokol_headers_path.join("cimgui").join("cimgui.cpp");
    let lib_c_spine = sokol_headers_path
        .join("spine-runtimes")
        .join("spine-c")
        .join("spine-c")
        .join("src")
        .join("spine");
    let spine_files: Vec<PathBuf> = std::fs::read_dir(lib_c_spine)
    .expect("Unable to list spine files")
    .filter_map(|entry| {
        entry.ok().and_then(|e| {
            let path = e.path();
            if path.is_file() { Some(path) } else { None }
        })
    })
    .collect();
    
    // Rebuild on build.rs change
    println!("cargo:rerun-if-changed=build.rs");
    println!("{}", format!("cargo:rerun-if-changed={}", sokol_gp_wrapper));

    // Check if we are building for Emscripten
    let target = env::var("TARGET").unwrap();
    if !target.contains("emscripten") {
        let _bindings = bindgen::Builder::default()
            .clang_arg("-x")
            .clang_arg("c++")
            .clang_arg("-std=c++11")
            // CIMGUI_DEFINE_ENUMS_AND_STRUCTS
            .clang_arg("-DCIMGUI_DEFINE_ENUMS_AND_STRUCTS=0")
            // IMGUI_DISABLE_OBSOLETE_FUNCTIONS
            .clang_arg("-DIMGUI_DISABLE_OBSOLETE_FUNCTIONS=1")
            .clang_arg("-Wno-unused-parameter")
            .clang_arg("-Wno-missing-field-initializers")

            .header(sokol_headers_path.join("sokol_app.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol_gfx.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol_glue.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol_log.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol_time.h").to_str().unwrap())
            .header(sokol_headers_path.join("stb_image.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol_gp.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol_imgui.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol_spine.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol_fetch.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol_audio.h").to_str().unwrap())
            .header(sokol_headers_path.join("cimgui").join("cimgui.h").to_str().unwrap())
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
    // Define variables
    build
        .define("SOKOL_GLES3", None)
        .define("IMPL", None)
        .define("IMGUI_DISABLE_OBSOLETE_FUNCTIONS", None)
        // Flags
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-missing-field-initializers")
        // Build files
        .include("lib/spine-runtimes/spine-c/spine-c/include")
        .file(lib_imgui_draw)
        .file(lib_imgui_tables)
        .file(lib_imgui_widgets)
        .file(lib_imgui)
        .file(lib_c_imgui)
        .file(sokol_gp_wrapper)
        .files(spine_files)
        .compile("toxoid_sokol");

    println!("Hello world!");
}

