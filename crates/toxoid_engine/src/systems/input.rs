use toxoid_api::*;

// TODO: Remove system, temporary to test multiplayer
#[cfg(target_os = "emscripten")]
pub fn input_system(query: &mut Query) {
    let query = query.iter();
    let mut keyboard_input = World::get_singleton::<KeyboardInput>();
    let websocket = World::get_singleton::<WebSocket>();
    while query.next() {
        let entities = query.entities();
        entities
            .iter_mut()
            .for_each(|entity| {
                let mut pos = entity.get::<Position>();

                if keyboard_input.get_up() {
                    // pos.set_y(pos.get_y() - 3);
                    unsafe { toxoid_ffi::emscripten::emscripten_websocket_send_binary(websocket.get_socket().ptr, "UP".as_ptr() as *const core::ffi::c_void, 2) };
                }
                if keyboard_input.get_down() {
                    unsafe { toxoid_ffi::emscripten::emscripten_websocket_send_binary(websocket.get_socket().ptr, "DOWN".as_ptr() as *const core::ffi::c_void, 4) };
                    // pos.set_y(pos.get_y() + 3);
                }
                if keyboard_input.get_left() {
                    unsafe { toxoid_ffi::emscripten::emscripten_websocket_send_binary(websocket.get_socket().ptr, "LEFT".as_ptr() as *const core::ffi::c_void, 4) };
                    // pos.set_x(pos.get_x() - 3);
                }
                if keyboard_input.get_right() {
                    unsafe { toxoid_ffi::emscripten::emscripten_websocket_send_binary(websocket.get_socket().ptr, "RIGHT".as_ptr() as *const core::ffi::c_void, 5) };
                    // pos.set_x(pos.get_x() + 3);
                }
            });
    }
}