pub mod bindings;
pub mod ecs;
pub mod globals;
pub mod log;
pub mod components;
// pub mod collections;
pub mod utils;
pub mod net;
pub mod load;
pub mod events;
pub mod bootstrap;

pub use bindings::*;
pub use ecs::*;
pub use globals::*;
pub use log::*;
pub use components::*;
// pub use collections::*;
pub use serde;
pub use utils::*;
pub use core::ffi::c_void;
pub use net::*;
pub use load::*;

pub fn init() {
    // Initialize events
    events::init();
}
