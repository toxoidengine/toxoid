use std::env;
use std::path::PathBuf;

fn main() {
    // Get env variable for target in build.rs
    let target = std::env::var("TARGET").unwrap();
    // Check if target contains windows
    if target.contains("windows") {
        if target.contains("msvc") {
            println!("cargo:rustc-link-arg=/FORCE");
        } else if target.contains("gnu") {
            println!("cargo:rustc-link-arg=-Wl,--unresolved-symbols=ignore-all");
        }
    } 
}