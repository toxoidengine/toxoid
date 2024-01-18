use std::os::raw::c_void;

use toxoid_api::*;
use toxoid_ffi::emscripten::EmscriptenWebSocketCreateAttributes;
use toxoid_ffi::utils::{split_u64, combine_u32};

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
            // println!("Event received: {:?}", entity.event);
            match entity.event.as_str() {
                "LocalPlayerJoin" => {
                    println!("Local player ID received: {:?}", entity.id);
                    // Set local player ID
                    let mut local_player = World::get_singleton::<NetworkEntity>();
                    local_player.set_id(entity.id);

                    // Create entity
                    let render_entity = crate::utils::load_image("assets/character.png");
                    render_entity.add::<Local>();

                    // Add to network entity cache
                    unsafe { toxoid_ffi::ecs::toxoid_network_entity_cache_insert(split_u64(entity.id), split_u64(render_entity.get_id())) };
                },
                "PlayerJoin" => {
                    println!("ID received: {:?}", entity.id);
                    // Create entity
                    let player_animation_entity = crate::utils::load_animation("assets/player_spine.atlas", "assets/player_spine.json");
                    let mut position = player_animation_entity.get::<Position>();
                    position.set_x(100);
                    position.set_y(100);
                    player_animation_entity.add::<Remote>();
                    
                    // Update position
                    let deserialized_component = entity.components[0].clone();
                    let mut position = player_animation_entity.get::<Position>();
                    position.set_x(deserialized_component.x);
                    position.set_y(deserialized_component.y);
                    
                    // Add to network entity cache
                    unsafe { toxoid_ffi::ecs::toxoid_network_entity_cache_insert(split_u64(entity.id), split_u64(player_animation_entity.get_id())) };
                },
                "PlayerLeave" => {
                    println!("Player ID {:?} disconnected from server.", entity.id);
                },
                "PlayerMove" => {
                    // println!("Player ID {:?} moved to position {:?}.", entity.id, entity.components[0]);
                    // Get entity from network entity cache
                    let entity_id = unsafe { toxoid_ffi::ecs::toxoid_network_entity_cache_get(split_u64(entity.id)) };
                    let entity_id = combine_u32(entity_id);
                    if entity_id == 0 {
                        return;
                    }

                    // Create entity object
                    let render_entity = Entity {
                        id: entity_id,
                        children: &mut []
                    };
                    
                    // Update position
                    let mut position = render_entity.get::<Position>();
                    position.set_x(entity.components[0].x);
                    position.set_y(entity.components[0].y);
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
    
    // #[repr(C)]
    // struct Buffers {
    //     atlas: [u8; 4 * 1024],
    //     skeleton: [u8; 128 * 1024],
    //     image: [u8; 512 * 1024]
    // }
    // let buffers = Buffers {
    //     atlas: [0; 4 * 1024],
    //     skeleton: [0; 128 * 1024],
    //     image: [0; 512 * 1024]
    // };
    // unsafe {
    //     let mut sfetch_request: toxoid_sokol::bindings::sfetch_request_t = core::mem::MaybeUninit::zeroed().assume_init();
    //     // sfetch_request.path = "assets/character.png\0".as_ptr() as *const i8;
    //     sfetch_request.path = "assets/character.png\0".as_ptr() as *const i8;
    //     sfetch_request.channel = 0;
    //     sfetch_request.buffer = toxoid_sokol::bindings::sfetch_range_t {
    //         ptr:  &buffers.image as *const u8 as *const c_void,
    //         size: 512 * 1024 
    //     };
    //     sfetch_request.callback = Some(atlas_data_loaded);
    //     toxoid_sokol::bindings::sfetch_send(&sfetch_request);
    // }

    let player_animation_entity = crate::utils::load_animation("assets/player_spine.atlas", "assets/player_spine.json");
    let mut position = player_animation_entity.get::<Position>();
    // position.set_x(100);
    // position.set_y(100);
    player_animation_entity.add::<Local>();
}