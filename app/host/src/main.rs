// #[cfg(target_os = "emscripten")]
use toxoid_ffi::*;

#[no_mangle]
pub extern "C" fn init_host() {
    #[cfg(feature = "static-linking")]
    guest::init();
}

fn main() {
    toxoid_bootstrap::init(init_host);
}