fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let target = std::env::var("TARGET").unwrap();
    if target.contains("emscripten") {
        println!("cargo:rustc-link-arg=-sEXPORT_ES6=1");
        println!("cargo:rustc-link-arg=-sMODULARIZE=1");
        println!("cargo:rustc-link-arg=-sSTACK_SIZE=1mb");
        println!("cargo:rustc-link-arg=-sFETCH=1");
        println!("cargo:rustc-link-arg=-sMIN_WEBGL_VERSION=2");
        println!("cargo:rustc-link-arg=-sMAX_WEBGL_VERSION=2");
        println!("cargo:rustc-link-arg=-sFULL_ES3=1");
        println!("cargo:rustc-link-arg=-sALLOW_MEMORY_GROWTH=1");
        println!("cargo:rustc-link-arg=-sFORCE_FILESYSTEM=1");
        println!("cargo:rustc-link-arg=-g");
        println!("cargo:rustc-link-arg=-O0");
    }
}
