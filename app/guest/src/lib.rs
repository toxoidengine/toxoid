mod components;
mod entities;
mod systems;
use toxoid_api::*;

pub struct ToxoidWasmComponent;

impl CallbacksGuest for ToxoidWasmComponent {
    fn run(iter: ToxoidIter, handle: i64) {
        run_callback(iter, handle);
    }
}

impl WorldGuest for ToxoidWasmComponent {
    fn init() {
        components::init();
        entities::init();
        systems::init();
    }
}

#[cfg(target_arch = "wasm32")]
bindings::export!(ToxoidWasmComponent with_types_in bindings);