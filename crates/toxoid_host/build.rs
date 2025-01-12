use std::fs;
use std::path::Path;

fn main() {
    let src_path = Path::new("src/bindings.rs");
    let mut content = fs::read_to_string(&src_path)
        .expect("Failed to read bindings.rs");
    
    // Add cfg attribute to wit_bindgen_rt calls
    content = content.replace(
        "wit_bindgen_rt::", 
        "#[cfg(not(target_os = \"emscripten\"))]\nwit_bindgen_rt::"
    );

    // Write the modified content back to the file
    fs::write(&src_path, content)
        .expect("Failed to write to bindings.rs");
}