extern crate bindgen;
extern crate cc;

use std::env::var;
use std::path::{PathBuf, Path};

fn main() {
    // Rebuild on build.rs change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=lib/sokol_wrapper.c");
    // Get the current directory so we can find the sokol headers
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    // Get the path to the sokol headers
    let sokol_headers_path = Path::new(&manifest_dir).join("lib");

    // ImGui
    // Check if IMGUI feature is enabled
    let imgui_files = if var("CARGO_FEATURE_IMGUI").is_ok() {
        let sokol_imgui_path = sokol_headers_path.join("cimgui").join("imgui"); 
        
        let lib_imgui_draw = sokol_imgui_path.join("imgui_draw.cpp");
        let lib_imgui_tables = sokol_imgui_path.join("imgui_tables.cpp");
        let lib_imgui_widgets = sokol_imgui_path.join("imgui_widgets.cpp");
        let lib_imgui = sokol_imgui_path.join("imgui.cpp");
        let lib_c_imgui = sokol_headers_path.join("cimgui").join("cimgui.cpp");

        let imgui_files = vec![
            lib_imgui_draw,
            lib_imgui_tables,
            lib_imgui_widgets,
            lib_imgui,
            lib_c_imgui
        ];
        imgui_files
    } else {
        eprintln!("Skipping imgui files");
        Vec::new()
    };
    // Spine
    // Check if spine feature is enabled
    let spine_files = if var("CARGO_FEATURE_SPINE").is_ok() {
        eprintln!("Building spine files");
        let lib_c_spine = sokol_headers_path
            .join("spine-runtimes")
            .join("spine-c")
            .join("spine-c")
            .join("src")
            .join("spine");

        // Add Json.c to the list of spine files
        let mut spine_files: Vec<PathBuf> = std::fs::read_dir(lib_c_spine.clone())
            .expect("Unable to list spine files")
            .filter_map(|entry| {
                println!("toxoid_sokol build.rs spine file entry: {:?}", entry);
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path.is_file() { Some(path) } else { None }
                })
            })
            .collect();
        // Add Json.c explicitly
        spine_files.push(lib_c_spine.join("Json.c"));
        spine_files
    } else {
        eprintln!("Skipping spine files");
        Vec::new()
    };
    
    // Check what target we are building for since cfg
    // does not work for build.rs
    let target = var("TARGET").unwrap();
    if !target.contains("emscripten") {
        let bindings = bindgen::Builder::default()
            .clang_arg("-x")
            .clang_arg("c++")
            .clang_arg("-std=c++11")  // Enable C++11 features
            .clang_arg("-DCIMGUI_DEFINE_ENUMS_AND_STRUCTS=0")
            .clang_arg("-DIMGUI_DISABLE_OBSOLETE_FUNCTIONS=1")
            .header(sokol_headers_path.join("sokol").join("sokol_app.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol").join("sokol_log.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol").join("sokol_time.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol").join("sokol_gfx.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol").join("sokol_glue.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol").join("sokol_fetch.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol").join("util").join("sokol_spine.h").to_str().unwrap())
            .header(sokol_headers_path.join("cimgui").join("cimgui.h").to_str().unwrap())
            .header(sokol_headers_path.join("sokol").join("util").join("sokol_imgui.h").to_str().unwrap())
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
    }

    let mut build = cc::Build::new();
    // Flags
    build
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-missing-field-initializers")
        .cpp(true)  // Enable C++ compilation globally
        .flag_if_supported("-std=c++11");  // Enable C++11 features like constexpr

    // For Metal on macOS, we need to compile as Objective-C/C++
    if target.contains("darwin") {
        build
            .define("SOKOL_METAL", None)
            .flag("-x")
            .flag("objective-c++");  // Use objective-c++ instead of objective-c
    } else if target.contains("windows") {
        build.define("SOKOL_D3D11", None);
    } else if target.contains("linux") {
        build.define("SOKOL_GLCORE33", None);
    } else if target.contains("android") {
        build.define("SOKOL_GLES3", None);
    } else if target.contains("emscripten") {
        build.define("SOKOL_GLES3", None);
    } else if target.contains("ios") {
        build
            .define("SOKOL_METAL", None)
            .flag("-x")
            .flag("objective-c++");  // Use objective-c++ instead of objective-c
    }
    // If fetch feature is enabled, define TOXOID_FETCH
    if var("CARGO_FEATURE_FETCH").is_ok() {
        build.define("TOXOID_FETCH", None);
    }
    // If audio feature is enabled, define TOXOID_AUDIO
    if var("CARGO_FEATURE_AUDIO").is_ok() {
        build.define("TOXOID_AUDIO", None);
    }
    // If imgui feature is enabled, add imgui files and include paths
    if var("CARGO_FEATURE_IMGUI").is_ok() {
        build
            .define("TOXOID_IMGUI", None)
            .define("IMGUI_DISABLE_OBSOLETE_FUNCTIONS", None)
            .include("lib/cimgui")  // Add include path for imgui headers
            .include("lib/cimgui/imgui")  // Add include path for imgui headers
            .files(imgui_files);
    }
    // If spine feature is enabled, add spine files
    if var("CARGO_FEATURE_SPINE").is_ok() {
        build
            .define("TOXOID_SPINE", None)
            .include("lib/spine-runtimes/spine-c/spine-c/include")
            .files(spine_files);
    }
    if var("CARGO_FEATURE_STB").is_ok() {
        build.define("TOXOID_STB", None);
        build.define("STBI_NO_STDIO", None);
    }
    build
        .file(sokol_headers_path.join("sokol_wrapper.c"))
        .compile("toxoid_sokol");
}
