use toxoid_api::*;

pub struct ToxoidWasmComponent;

impl CallbacksGuest for ToxoidWasmComponent {
    fn run(iter: ToxoidIter, handle: u64) {
        run_callback(iter, handle);
    }
}

impl WorldGuest for ToxoidWasmComponent {
    fn init() {
        crate::init();
    }
}

bindings::export!(ToxoidWasmComponent with_types_in bindings);