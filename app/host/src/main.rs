#[cfg(feature = "static-linking")]
use guest::bindings::Guest;

fn main() {
    #[cfg(feature = "wasm-linking")]
    toxoid_bootstrap::init();

    #[cfg(feature = "static-linking")]
    println!("{}", guest::ToxoidWasmComponent::init());
}
