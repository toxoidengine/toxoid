use std::fs;
use std::path::Path;

fn main() {
    let src_path = Path::new("src/bindings.rs");
    let mut content = fs::read_to_string(&src_path)
        .expect("Failed to read bindings.rs");

    // Add #[macro_export] to __export_toxoid_component_world_impl if not present
    if !content.contains("#[macro_export] macro_rules! __export_toxoid_component_world_impl") {
        content = content.replace(
            "#[doc(hidden)]\nmacro_rules! __export_toxoid_component_world_impl {",
            "#[doc(hidden)]\n#[macro_export]\nmacro_rules! __export_toxoid_component_world_impl {"
        );
    }

    // Add #[macro_export] to __export_world_toxoid_component_world_cabi if not present
    if !content.contains("#[macro_export] macro_rules! __export_world_toxoid_component_world_cabi") {
        content = content.replace(
            "macro_rules! __export_world_toxoid_component_world_cabi {",
            "#[macro_export]\nmacro_rules! __export_world_toxoid_component_world_cabi {"
        );
    }

    // Add #[macro_export] to __export_toxoid_component_component_callbacks_cabi if not present
    if !content.contains("#[macro_export] macro_rules! __export_toxoid_component_component_callbacks_cabi") {
        content = content.replace(
            "macro_rules! __export_toxoid_component_component_callbacks_cabi {",
            "#[macro_export]\nmacro_rules! __export_toxoid_component_component_callbacks_cabi {"
        );
    }

    // Change `pub(crate)` to `pub` for __export_toxoid_component_world_impl re-export
    if content.contains("pub(crate) use __export_toxoid_component_world_impl as export;") {
        content = content.replace(
            "pub(crate) use __export_toxoid_component_world_impl as export;",
            "pub use __export_toxoid_component_world_impl as export;"
        );
    }

    // Change `pub(crate)` to `pub` for __export_world_toxoid_component_world_cabi re-export
    if content.contains("pub(crate) use __export_world_toxoid_component_world_cabi;") {
        content = content.replace(
            "pub(crate) use __export_world_toxoid_component_world_cabi;",
            "pub use __export_world_toxoid_component_world_cabi;"
        );
    }

    // Change `pub(crate)` to `pub` for __export_toxoid_component_component_callbacks_cabi re-export
    if content.contains("pub(crate) use __export_toxoid_component_component_callbacks_cabi;") {
        content = content.replace(
            "pub(crate) use __export_toxoid_component_component_callbacks_cabi;",
            "pub use __export_toxoid_component_component_callbacks_cabi;"
        );
    }

    // Write the modified content back to the file
    fs::write(&src_path, content)
        .expect("Failed to write to bindings.rs");
}