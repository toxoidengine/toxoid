fn main() {
    println!(
        "cargo:rustc-env=WORKSPACE_ROOT={}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );

    // Set env var WORKSPACE_ROOT to the root of the workspace
    let workspace_root =
        std::env::var("WORKSPACE_ROOT").expect("Failed to retrieve workspace root directory");
}
