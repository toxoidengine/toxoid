use std::env;
use std::path::PathBuf;

fn main() {
    let workspace_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).parent().unwrap().parent().unwrap().parent().unwrap().to_path_buf();
    println!("cargo:rustc-env=WORKSPACE_DIR={}", workspace_dir.display());
}