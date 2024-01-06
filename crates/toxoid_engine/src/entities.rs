use std::os::raw::c_void;

use toxoid_api::*;
use toxoid_ffi::emscripten::EmscriptenWebSocketCreateAttributes;

pub extern "C" fn onopen_cb(
    _event_type: *mut ::std::os::raw::c_void,
    _user_data: *mut ::std::os::raw::c_void
) {
    println!("Connection opened.");
}

pub extern "C" fn onmessage_cb(
    _event_type: ::std::os::raw::c_int,
    websocket_event: *const toxoid_ffi::emscripten::EmscriptenWebSocketMessageEvent,
    user_data: *mut ::std::os::raw::c_void,
)  {
    let data = unsafe{ (*websocket_event).data };
    let data_len = unsafe{ (*websocket_event).numBytes };
    let data = unsafe { std::slice::from_raw_parts(data, data_len as usize) };
    
    // let data = std::str::from_utf8(data).unwrap();
    // println!("Message received: {:?}", data);

    // let req = unsafe { toxoid_ffi::emscripten::emscripten_websocket_send_binary(user_data, "Hello".as_ptr() as *const c_void, 5) };
    // println!("req: {:?}", req);
    
    let network_messages = toxoid_net::deserialize(data);
    network_messages
        .messages
        .iter()
        .for_each(|entity| {
            println!("Event received: {:?}", entity.event);
            match entity.event.as_str() {
                "LocalPlayerJoin" => {
                    println!("Local player ID received: {:?}", entity.id);
                    let mut local_player = World::get_singleton::<NetworkEntity>();
                    local_player.set_id(entity.id);
                },
                "PlayerJoin" => {
                    println!("ID received: {:?}", entity.id);
                    // entity
                    //     .components
                    //     .iter()
                    //     .for_each(|component| {
                    //         println!("component: {:?}", component);
                    //     });
                    // let req = unsafe { toxoid_ffi::emscripten::emscripten_websocket_send_binary(user_data, "Hello".as_ptr() as *const c_void, 5) };
                    // println!("req: {:?}", req);
                },
                "PlayerLeave" => {
                    println!("Player ID {:?} disconnected from server.", entity.id);
                },
                "PlayerMove" => {
                    println!("Player ID {:?} moved to position {:?}.", entity.id, entity.components[0]);
                },
                _ => {}
            }
        });
}


pub fn init() {
    // Game Config
    World::add_singleton::<GameConfig>();
    let mut game_config = World::get_singleton::<GameConfig>();
    game_config.set_resolution_width(1280);
    game_config.set_resolution_height(720);

    // Keyboard Input
    World::add_singleton::<KeyboardInput>();

    // Local Player
    World::add_singleton::<NetworkEntity>();
    
    // WebSocket Networked Multiplayer Test
    let mut attributes = EmscriptenWebSocketCreateAttributes {
        url: "ws://127.0.0.1:8080\0".as_ptr() as *const i8, 
        protocol: std::ptr::null()
    };
    let ws = unsafe { toxoid_ffi::emscripten::emscripten_websocket_new(&mut attributes as *mut EmscriptenWebSocketCreateAttributes) };
    let user_data = ws as *mut ::std::os::raw::c_void;
    unsafe {
        toxoid_ffi::emscripten::emscripten_websocket_set_onopen_callback_on_thread(ws, user_data, onopen_cb,  toxoid_ffi::emscripten::EM_CALLBACK_THREAD_CONTEXT_CALLING_THREAD as *mut c_void);
        toxoid_ffi::emscripten::emscripten_websocket_set_onmessage_callback_on_thread(ws, user_data, onmessage_cb, toxoid_ffi::emscripten::EM_CALLBACK_THREAD_CONTEXT_CALLING_THREAD);
    }

    World::add_singleton::<WebSocket>();
    let mut websocket = World::get_singleton::<WebSocket>();
    websocket.set_socket(Pointer{ ptr: ws });
    
    let render_entity = crate::utils::load_image("assets/character.png");
    render_entity.
}