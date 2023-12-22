// #![allow(non_camel_case_types)]
// #![allow(improper_ctypes)]
// extern crate toxoid_api_macro;

mod components;
mod entities;
mod systems;

#[no_mangle]
pub unsafe extern "C" fn app_main() {
    // Initialize snake components
    components::init();
    // Initialize snake entities
    entities::init();
    // Initialize snake systems
    systems::init();
}
