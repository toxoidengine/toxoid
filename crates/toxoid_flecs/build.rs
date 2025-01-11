// use std::path::{PathBuf, Path};
use std::path::{PathBuf, Path};
fn main() {
    // Tell cargo to invalidate the built crate whenever the sources change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=flecs.h");
    println!("cargo:rerun-if-changed=flecs.c");

    let target = std::env::var("TARGET").unwrap();
    if target.contains("emscripten") {
        // Get rid of the warning about unused command line arguments from emcc
        std::env::set_var("CFLAGS", "-Wno-unused-command-line-argument");
    };

    // Check of target is aarch64
    let is_aarch64 = target.contains("aarch64");
    let bindings = bindgen::Builder::default()
        .clang_arg("-DFLECS_CUSTOM_BUILD")
        .clang_arg("-DFLECS_PERF_TRACE")
        .clang_arg("-DFLECS_MODULE")
        .clang_arg("-DFLECS_SCRIPT")
        .clang_arg("-DFLECS_SNAPSHOT")
        .clang_arg("-DFLECS_STATS")
        .clang_arg("-DFLECS_METRICS")
        .clang_arg("-DFLECS_ALERTS")
        .clang_arg("-DFLECS_SYSTEM")
        .clang_arg("-DFLECS_PIPELINE")
        .clang_arg("-DFLECS_TIMER")
        .clang_arg("-DFLECS_META")
        .clang_arg("-DFLECS_META_C")
        .clang_arg("-DFLECS_UNITS")
        .clang_arg("-DFLECS_JSON")
        .clang_arg("-DFLECS_DOC")
        .clang_arg("-DFLECS_LOG")
        .clang_arg("-DFLECS_APP")
        .clang_arg("-DFLECS_OS_API_IMPL")
        .clang_arg("-DFLECS_HTTP")
        .clang_arg("-DFLECS_REST")
        .clang_arg("-DFLECS_JOURNAL")
        .header(Path::new("flecs.h").to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("./src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
    // Compile Flecs
    let mut build = cc::Build::new();
    build
        .define("FLECS_CUSTOM_BUILD", None)
        .define("FLECS_PERF_TRACE", None)
        .define("FLECS_MODULE", None)
        .define("FLECS_SCRIPT", None)
        .define("FLECS_SNAPSHOT", None)
        .define("FLECS_STATS", None)
        .define("FLECS_METRICS", None)
        .define("FLECS_ALERTS", None)
        .define("FLECS_SYSTEM", None)
        .define("FLECS_PIPELINE", None)
        .define("FLECS_TIMER", None)
        .define("FLECS_META", None)
        .define("FLECS_META_C", None)
        .define("FLECS_UNITS", None)
        .define("FLECS_JSON", None)
        .define("FLECS_DOC", None)
        .define("FLECS_LOG", None)
        .define("FLECS_APP", None)
        .define("FLECS_OS_API_IMPL", None)
        // .define("FLECS_HTTP", None)
        // .define("FLECS_REST", None)
        .define("FLECS_JOURNAL", None)
        .define("NDEBUG", None);
    // Has a backtrace error otherwise. 
    if is_aarch64 {
        build.define("__wasm32__", None);
    }
    build
        .include("flecs.h")
        .file("flecs.c")
        .compile("flecs_core");
}
