use toxoid_api::*;
use toxoid_net::{NetworkMessageEntity, NetworkMessages};
use toxoid_net::serialize;

// TODO: Remove system, temporary to test multiplayer
#[cfg(target_os = "emscripten")]
pub fn input_system(iter: &mut Iter) {
    let keyboard_input = World::get_singleton::<KeyboardInput>();
    let websocket = World::get_singleton::<WebSocket>();
    let entities = iter.entities();
    entities
        .iter_mut()
        .for_each(|entity| {
            let mut pos = entity.get::<Position>();

            if keyboard_input.get_up() {
                let network_messages = NetworkMessages {
                    messages: vec![
                        NetworkMessageEntity {
                            id: 0,
                            event: "TestingUp".to_string(),
                            components: vec![],
                        }
                    ],
                };
                let serialized = serialize(network_messages).unwrap();
                unsafe { toxoid_ffi::emscripten::emscripten_websocket_send_binary(websocket.get_socket().ptr, serialized.as_ptr() as *const core::ffi::c_void, serialized.len() as i32) };
            }
            if keyboard_input.get_down() {
                let network_messages = NetworkMessages {
                    messages: vec![
                        NetworkMessageEntity {
                            id: 0,
                            event: "TestingDown".to_string(),
                            components: vec![],
                        }
                    ],
                };
                let serialized = serialize(network_messages).unwrap();
                unsafe { toxoid_ffi::emscripten::emscripten_websocket_send_binary(websocket.get_socket().ptr, serialized.as_ptr() as *const core::ffi::c_void, serialized.len() as i32) };
            }
            if keyboard_input.get_left() {
                let network_messages = NetworkMessages {
                    messages: vec![
                        NetworkMessageEntity {
                            id: 0,
                            event: "TestingLeft".to_string(),
                            components: vec![],
                        }
                    ],
                };
                let serialized = serialize(network_messages).unwrap();
                unsafe { toxoid_ffi::emscripten::emscripten_websocket_send_binary(websocket.get_socket().ptr, serialized.as_ptr() as *const core::ffi::c_void, serialized.len() as i32) };
            }
            if keyboard_input.get_right() {
                let network_messages = NetworkMessages {
                    messages: vec![
                        NetworkMessageEntity {
                            id: 0,
                            event: "TestingRight".to_string(),
                            components: vec![],
                        }
                    ],
                };
                let serialized = serialize(network_messages).unwrap();
                unsafe { toxoid_ffi::emscripten::emscripten_websocket_send_binary(websocket.get_socket().ptr, serialized.as_ptr() as *const core::ffi::c_void, serialized.len() as i32) };
            }
        });
}