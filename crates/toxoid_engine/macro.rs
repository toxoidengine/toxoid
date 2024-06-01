    Blocking waiting for file lock on build directory
warning: unused import: `Result`
   --> crates\toxoid_api_macro\src\lib.rs:650:18
    |
650 | use syn::{Token, Result};
    |                  ^^^^^^
    |
    = note: `#[warn(unused_imports)]` on by default
warning: unused import: `std::ffi::c_void`
 --> crates\toxoid_render\src\lib.rs:2:5
  |
2 | use std::ffi::c_void;
  |     ^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default
warning: unused imports: `PathBuf`, `Path`
 --> libs\flecs-polyglot\rust\flecs_core\build.rs:1:17
  |
1 | use std::path::{PathBuf, Path};
  |                 ^^^^^^^  ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default
warning: toxoid_sokol@0.1.0: clang++: warning: treating 'c-header' input as 'c++-header' when in C++ mode, this behavior is deprecated [-Wdeprecated]
warning: toxoid_sokol@0.1.0: C:\Users\troye\dev\toxoid\toxoid\crates\toxoid_sokol\lib\spine-runtimes\spine-c\spine-c\src\spine\Json.c:125:7: warning: variable 'n' set but not used [-Wunused-but-set-variable]
warning: toxoid_sokol@0.1.0:   125 |                 int n = 0;
warning: toxoid_sokol@0.1.0:       |                     ^
warning: toxoid_sokol@0.1.0: 1 warning generated.
warning: unused import: `std::collections::HashMap`
  --> libs\flecs-polyglot\rust\flecs_core\src\lib.rs:16:5
   |
16 | use std::collections::HashMap;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default
warning: unused import: `std::ffi::CString`
 --> crates\toxoid_ffi\src\emscripten.rs:2:5
  |
2 | use std::ffi::CString;
  |     ^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default
warning: unused import: `std::ptr`
 --> crates\toxoid_ffi\src\emscripten.rs:3:5
  |
3 | use std::ptr;
  |     ^^^^^^^^
warning: unused imports: `SystemTime`, `UNIX_EPOCH`
 --> crates\toxoid_ffi\src\utils.rs:1:17
  |
1 | use std::time::{SystemTime, UNIX_EPOCH};
  |                 ^^^^^^^^^^  ^^^^^^^^^^
warning: unused variable: `component_struct_ptr`
   --> crates\toxoid_ffi\src\ecs.rs:903:17
    |
903 |             let component_struct_ptr = ecs_get_mut_id(world, FLECS_IDEcsStructID_, component_id);
    |                 ^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_component_struct_ptr`
    |
    = note: `#[warn(unused_variables)]` on by default
warning: unused variable: `keys`
   --> crates\toxoid_ffi\src\ecs.rs:909:17
    |
909 |             let keys: Vec<&str> = component_map.iter_keys().collect();
    |                 ^^^^ help: if this is intentional, prefix it with an underscore: `_keys`
warning: unused variable: `component_struct_ptr`
   --> crates\toxoid_ffi\src\ecs.rs:977:17
    |
977 |             let component_struct_ptr = ecs_get_mut_id(world, FLECS_IDEcsStructID_, component_id);
    |                 ^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_component_struct_ptr`
warning: unused variable: `keys`
   --> crates\toxoid_ffi\src\ecs.rs:983:17
    |
983 |             let keys: Vec<&str> = component_map.iter_keys().collect();
    |                 ^^^^ help: if this is intentional, prefix it with an underscore: `_keys`
warning: unused variable: `component_struct_ptr`
    --> crates\toxoid_ffi\src\ecs.rs:1049:9
     |
1049 |     let component_struct_ptr = ecs_get_mut_id(world, FLECS_IDEcsStructID_, component_id);
     |         ^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_component_struct_ptr`
warning: unused variable: `component_serialized`
    --> crates\toxoid_ffi\src\ecs.rs:1133:13
     |
1133 |         let component_serialized = builder.start_map();
     |             ^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_component_serialized`
warning: call to `.clone()` on a reference in this situation does nothing
   --> crates\toxoid_ffi\src\ecs.rs:975:82
    |
975 |             let component_name = std::ffi::CString::new(component_serialized.name.clone()).unwrap();
    |                                                                                  ^^^^^^^^ help: remove this redundant call
    |
    = note: the type `str` does not implement `Clone`, so calling `clone` on `&str` copies the reference, which does not do anything and can be removed
    = note: `#[warn(noop_method_call)]` on by default
warning: call to `.clone()` on a reference in this situation does nothing
   --> crates\toxoid_ffi\src\ecs.rs:980:59
    |
980 |             let component_data = component_serialized.data.clone();
    |                                                           ^^^^^^^^ help: remove this redundant call
    |
    = note: the type `[u8]` does not implement `Clone`, so calling `clone` on `&[u8]` copies the reference, which does not do anything and can be removed
warning: unused import: `core::result::Result`
 --> crates\toxoid_net\src\lib.rs:1:5
  |
1 | use core::result::Result;
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default
warning: unused variable: `name`
  --> crates\toxoid_net\src\lib.rs:14:39
   |
14 | pub extern "C" fn toxoid_network_send(name: *const u8, name_len: usize, data: *const u8, data_len: usize) {
   |                                       ^^^^ help: if this is intentional, prefix it with an underscore: `_name`
   |
   = note: `#[warn(unused_variables)]` on by default
warning: unused variable: `name_len`
  --> crates\toxoid_net\src\lib.rs:14:56
   |
14 | pub extern "C" fn toxoid_network_send(name: *const u8, name_len: usize, data: *const u8, data_len: usize) {
   |                                                        ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_name_len`
warning: unused variable: `data`
  --> crates\toxoid_net\src\lib.rs:14:73
   |
14 | pub extern "C" fn toxoid_network_send(name: *const u8, name_len: usize, data: *const u8, data_len: usize) {
   |                                                                         ^^^^ help: if this is intentional, prefix it with an underscore: `_data`
warning: unused variable: `data_len`
  --> crates\toxoid_net\src\lib.rs:14:90
   |
14 | pub extern "C" fn toxoid_network_send(name: *const u8, name_len: usize, data: *const u8, data_len: usize) {
   |                                                                                          ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_data_len`
warning: unused variable: `result`
   --> crates\toxoid_net\src\lib.rs:125:13
    |
125 |         let result = toxoid_ffi::emscripten::emscripten_websocket_send_binary(
    |             ^^^^^^ help: if this is intentional, prefix it with an underscore: `_result`
warning: `extern` fn uses type `str`, which is not FFI-safe
   --> crates\toxoid_net\src\lib.rs:158:17
    |
158 |     event_name: &'static str,
    |                 ^^^^^^^^^^^^ not FFI-safe
    |
    = help: consider using `*const u8` and a length instead
    = note: string slices have no C equivalent
    = note: `#[warn(improper_ctypes_definitions)]` on by default
warning: `extern` fn uses type `std::string::String`, which is not FFI-safe
   --> crates\toxoid_net\src\lib.rs:167:17
    |
167 |     event_name: String,
    |                 ^^^^^^ not FFI-safe
    |
    = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
    = note: this struct has unspecified layout
    Checking toxoid_engine v0.1.0 (C:\Users\troye\dev\toxoid\toxoid\crates\toxoid_engine)
warning: unused import: `toxoid_api::*`
 --> crates\toxoid_engine\src\prefabs\mod.rs:3:5
  |
3 | use toxoid_api::*;
  |     ^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default
warning: unused import: `toxoid_sokol::SokolRenderer2D`
 --> crates\toxoid_engine\src\prefabs\mod.rs:7:5
  |
7 | use toxoid_sokol::SokolRenderer2D;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
warning: unused import: `toxoid_sokol::SokolSprite`
 --> crates\toxoid_engine\src\prefabs\mod.rs:9:5
  |
9 | use toxoid_sokol::SokolSprite;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
warning: unused import: `toxoid_api::*`
 --> crates\toxoid_engine\src\systems\input.rs:1:5
  |
1 | use toxoid_api::*;
  |     ^^^^^^^^^^^^^
warning: unused import: `bindings::*`
  --> crates\toxoid_engine\src\systems\render.rs:72:13
   |
72 |         use bindings::*;
   |             ^^^^^^^^^^^
warning: unused import: `toxoid_api_macro::*`
   --> crates\toxoid_engine\src\systems\render.rs:214:5
    |
214 | use toxoid_api_macro::*;
    |     ^^^^^^^^^^^^^^^^^^^
warning: unused import: `toxoid_serialize::NetworkMessageEntity`
 --> crates\toxoid_engine\src\systems\network.rs:4:5
  |
4 | use toxoid_serialize::NetworkMessageEntity;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
warning: unused imports: `SpineInstance`, `World`
 --> crates\toxoid_engine\src\update.rs:1:18
  |
1 | use toxoid_api::{World, SpineInstance};
  |                  ^^^^^  ^^^^^^^^^^^^^
warning: unused imports: `toxoid_set_keydown_callback`, `toxoid_set_keyup_callback`
 --> crates\toxoid_engine\src\utils\input.rs:4:63
  |
4 | use toxoid_ffi::emscripten::{EmBool, EmscriptenKeyboardEvent, toxoid_set_keydown_callback, toxoid_set_keyup_callback};
  |                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^
warning: ambiguous glob re-exports
 --> crates\toxoid_engine\src\lib.rs:6:9
  |
6 | pub use systems::*;
  |         ^^^^^^^^^^ the name `input` in the type namespace is first re-exported here
7 | pub use update::*;
8 | pub use utils::*;
  |         -------- but the name `input` in the type namespace is also re-exported here
  |
  = note: `#[warn(ambiguous_glob_reexports)]` on by default
warning: ambiguous glob re-exports
 --> crates\toxoid_engine\src\lib.rs:6:9
  |
6 | pub use systems::*;
  |         ^^^^^^^^^^ the name `network` in the type namespace is first re-exported here
7 | pub use update::*;
8 | pub use utils::*;
  |         -------- but the name `network` in the type namespace is also re-exported here
    Finished dev [optimized + debuginfo] target(s) in 0.56s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod prefabs {
    use toxoid_api::*;
    use toxoid_api::*;
    #[cfg(feature = "render")]
    use toxoid_render::Renderer2D;
    #[cfg(feature = "render")]
    use toxoid_sokol::SokolRenderer2D;
    #[cfg(feature = "render")]
    use toxoid_sokol::SokolSprite;
    pub fn init() {
        let mut player_entity = Entity::new();
        player_entity.add::<Local>();
        player_entity.add::<Player>();
        player_entity.add::<Networked>();
        #[cfg(feature = "render")]
        {
            let player_animation_entity = crate::utils::load::load_animation(
                "assets/player.atlas",
                "assets/player.json",
                |entity| {},
            );
            unsafe {
                let mut position = (*player_animation_entity).get::<Position>();
                position.set_x(100);
                position.set_y(100);
                (*player_animation_entity).add::<Local>();
            }
        }
        #[cfg(feature = "render")]
        crate::utils::load::load_worldmap(
            "assets/world_1.world",
            |world_entity: &mut Entity| {},
        );
    }
}
pub mod systems {
    pub mod input {
        use toxoid_api::*;
    }
    pub mod render {
        use toxoid_api::*;
        #[cfg(feature = "render")]
        use toxoid_sokol::render_2d::*;
        #[cfg(feature = "render")]
        use toxoid_render::Renderer2D;
        #[cfg(feature = "render")]
        use toxoid_sokol::bindings::*;
        #[cfg(feature = "render")]
        pub fn render_rect_system(iter: &mut Iter) {
            let positions = iter.field::<Position>(5);
            let sizes = iter.field::<Size>(4);
            let colors = iter.field::<Color>(3);
            for i in 0..iter.count() {
                let pos = positions.get(i).unwrap();
                let size = sizes.get(i).unwrap();
                let color = colors.get(i).unwrap();
                SokolRenderer2D::draw_filled_rect(pos, size, color);
            }
        }
        #[cfg(feature = "render")]
        pub fn render_sprite_system(iter: &mut Iter) {
            let positions = iter.field::<Position>(4);
            let sizes = iter.field::<Size>(3);
            let sprites = iter.field::<Sprite>(1);
            for i in 0..iter.count() {
                let pos = positions.get(i).unwrap();
                let size = sizes.get(i).unwrap();
                let sprite = sprites.get(i).unwrap();
                let sprite_ptr = sprite.get_sprite();
                let sprite_box = unsafe {
                    Box::from_raw(sprite_ptr.ptr as *mut SokolSprite)
                };
                let sprite_trait_object: &Box<dyn toxoid_render::Sprite> = Box::leak(
                    Box::new(sprite_box as Box<dyn toxoid_render::Sprite>),
                );
                SokolRenderer2D::draw_sprite(
                    sprite_trait_object,
                    pos.get_x() as f32,
                    pos.get_y() as f32,
                );
            }
        }
        #[cfg(feature = "render")]
        pub fn render_rt_system(iter: &mut Iter) {
            let positions = iter.field::<Position>(5);
            let sizes = iter.field::<Size>(4);
            let rts = iter.field::<RenderTarget>(1);
            for i in 0..iter.count() {
                let pos = positions.get(i).unwrap();
                let size = sizes.get(i).unwrap();
                let rt = rts.get(i).unwrap();
                let rt_ptr = rt.get_render_target();
                let rt_box = unsafe {
                    Box::from_raw(rt_ptr.ptr as *mut SokolRenderTarget)
                };
                let rt_trait_object: &Box<dyn toxoid_render::RenderTarget> = Box::leak(
                    Box::new(rt_box as Box<dyn toxoid_render::RenderTarget>),
                );
                SokolRenderer2D::draw_render_target(
                    rt_trait_object,
                    pos.get_x() as f32,
                    pos.get_y() as f32,
                    size.get_width() as f32,
                    size.get_height() as f32,
                );
            }
        }
        #[cfg(all(feature = "render", feature = "spine"))]
        pub fn render_bone_animation(iter: &mut Iter) {
            let spine_instances = iter.field::<SpineInstance>(1);
            let positions = iter.field::<Position>(2);
            for i in 0..iter.count() {
                use bindings::*;
                let spine_instance: &SpineInstance = spine_instances.get(i).unwrap();
                let instantiated = spine_instance.get_instantiated();
                if instantiated {
                    unsafe {
                        let instance = spine_instance.get_instance().ptr
                            as *mut sspine_instance;
                        let delta_time = sapp_frame_duration();
                        sspine_update_instance(*instance, delta_time as f32);
                        sspine_draw_instance_in_layer(*instance, 0);
                        let pos: &Position = positions.get(i).unwrap();
                        sspine_set_position(
                            *instance,
                            sspine_vec2 {
                                x: pos.get_x() as f32,
                                y: pos.get_y() as f32,
                            },
                        );
                    }
                }
            }
        }
        pub fn load_cell_system(iter: &mut Iter) {
            {
                ::std::io::_print(format_args!("Loading cell!\n"));
            };
            let cells = iter.field::<TiledCellComponent>(1);
            let count = iter.count();
            let entities = iter.entities();
            for i in 0..count {
                let cell = cells.get(i).unwrap().get_cell().ptr
                    as *mut toxoid_tiled::TiledCell;
                unsafe {
                    (*cell)
                        .tilesets
                        .iter()
                        .for_each(|tileset| {
                            {
                                ::std::io::_print(
                                    format_args!("Loading tileset: {0}\n", tileset.name),
                                );
                            };
                        });
                }
                entities[i].remove::<Loadable>();
            }
        }
        use toxoid_api_macro::*;
        pub fn update(iter: &mut Iter) {
            {
                ::std::io::_print(
                    format_args!(
                        "Updating entities with Position and Velocity components\n",
                    ),
                );
            };
        }
        pub fn load_tilemap_system(iter: &mut Iter) {
            {
                ::std::io::_print(format_args!("Testing macro\n"));
            };
            update(iter);
            let worlds = iter.field::<TiledWorldComponent>(1);
            let count = iter.count();
            let entities = iter.entities();
            for i in 0..count {
                let world = worlds.get(i).unwrap().get_world().ptr
                    as *mut toxoid_tiled::TiledWorld;
                unsafe {
                    (*world)
                        .maps
                        .as_ref()
                        .unwrap()
                        .iter()
                        .for_each(|map| {
                            {
                                ::std::io::_print(
                                    format_args!("Loading map: {0}\n", map.file_name),
                                );
                            };
                            #[cfg(feature = "fetch")]
                            crate::utils::load::load_cell(
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("assets/{0}", map.file_name),
                                    );
                                    res
                                }
                                    .as_str(),
                                |_| {},
                            );
                        });
                }
                entities[i].remove::<Loadable>();
            }
        }
    }
    pub mod network {
        use toxoid_api::*;
        #[cfg(feature = "net")]
        use toxoid_serialize::NetworkMessageEntity;
        #[cfg(feature = "net")]
        pub fn network_event_system(iter: &mut Iter) {}
    }
    pub use load::*;
    #[cfg(feature = "render")]
    pub use render::*;
    pub use network::*;
    use toxoid_api::*;
    #[cfg(feature = "render")]
    static mut CURRENT_ANIMATION: &str = "idle_down";
    #[cfg(feature = "render")]
    pub fn animation_input_system(iter: &mut Iter) {
        let entities = iter.entities();
        entities
            .iter_mut()
            .for_each(|entity| {
                use toxoid_sokol::bindings::*;
                let spine_instance = entity.get::<SpineInstance>();
                if spine_instance.get_instantiated() {
                    let instance = spine_instance.get_instance().ptr
                        as *mut sspine_instance;
                    let spine_instance = entity.get::<Skeleton>();
                    let spine_skeleton = spine_instance.get_skeleton().ptr
                        as *mut sspine_skeleton;
                    let keyboard_input = World::get_singleton::<KeyboardInput>();
                    if !keyboard_input.get_up() && !keyboard_input.get_down()
                        && !keyboard_input.get_left() && !keyboard_input.get_right()
                    {
                        let mut direction = entity.get::<Direction>();
                        unsafe {
                            if direction.get_direction() == DirectionEnum::Up as u8
                                && CURRENT_ANIMATION != "idle_up"
                            {
                                sspine_set_animation(
                                    *instance,
                                    sspine_anim_by_name(
                                        *spine_skeleton,
                                        make_c_string("idle_up"),
                                    ),
                                    0,
                                    true,
                                );
                                CURRENT_ANIMATION = "idle_up";
                            }
                            if direction.get_direction() == DirectionEnum::Down as u8
                                && CURRENT_ANIMATION != "idle_down"
                            {
                                sspine_set_animation(
                                    *instance,
                                    sspine_anim_by_name(
                                        *spine_skeleton,
                                        make_c_string("idle_down"),
                                    ),
                                    0,
                                    true,
                                );
                                CURRENT_ANIMATION = "idle_down";
                            }
                            if direction.get_direction() == DirectionEnum::Left as u8
                                && CURRENT_ANIMATION != "idle_left"
                            {
                                sspine_set_animation(
                                    *instance,
                                    sspine_anim_by_name(
                                        *spine_skeleton,
                                        make_c_string("idle_left"),
                                    ),
                                    0,
                                    true,
                                );
                                CURRENT_ANIMATION = "idle_left";
                            }
                            if direction.get_direction() == DirectionEnum::Right as u8
                                && CURRENT_ANIMATION != "idle_right"
                            {
                                sspine_set_animation(
                                    *instance,
                                    sspine_anim_by_name(
                                        *spine_skeleton,
                                        make_c_string("idle_right"),
                                    ),
                                    0,
                                    true,
                                );
                                CURRENT_ANIMATION = "idle_right";
                            }
                        }
                    }
                    unsafe {
                        if keyboard_input.get_up() && CURRENT_ANIMATION != "walk_up" {
                            sspine_set_animation(
                                *instance,
                                sspine_anim_by_name(
                                    *spine_skeleton,
                                    make_c_string("walk_up"),
                                ),
                                0,
                                true,
                            );
                            CURRENT_ANIMATION = "walk_up";
                        }
                        if keyboard_input.get_down() && CURRENT_ANIMATION != "walk_down"
                        {
                            sspine_set_animation(
                                *instance,
                                sspine_anim_by_name(
                                    *spine_skeleton,
                                    make_c_string("walk_down"),
                                ),
                                0,
                                true,
                            );
                            CURRENT_ANIMATION = "walk_down";
                        }
                        if keyboard_input.get_left() && CURRENT_ANIMATION != "walk_left"
                        {
                            sspine_set_animation(
                                *instance,
                                sspine_anim_by_name(
                                    *spine_skeleton,
                                    make_c_string("walk_left"),
                                ),
                                0,
                                true,
                            );
                            CURRENT_ANIMATION = "walk_left";
                        }
                        if keyboard_input.get_right()
                            && CURRENT_ANIMATION != "walk_right"
                        {
                            sspine_set_animation(
                                *instance,
                                sspine_anim_by_name(
                                    *spine_skeleton,
                                    make_c_string("walk_right"),
                                ),
                                0,
                                true,
                            );
                            CURRENT_ANIMATION = "walk_right";
                        }
                    }
                }
            });
    }
    pub fn init() {
        #[cfg(feature = "render")]
        {
            System::new(render_rect_system)
                .with::<(Rect, Renderable, Color, Size, Position)>()
                .build();
            System::new(render_sprite_system)
                .with::<(Sprite, Renderable, Size, Position)>()
                .build();
            System::new(render_rt_system)
                .with::<(RenderTarget, Renderable, Size, Position)>()
                .build();
            System::new(render_bone_animation)
                .with::<(SpineInstance, Position, BoneAnimation)>()
                .build();
            System::new(animation_input_system)
                .with::<(SpineInstance, Position)>()
                .build();
        }
        #[cfg(feature = "render")]
        System::new(load_cell_system).with::<(TiledCellComponent, Loadable)>().build();
        #[cfg(feature = "render")]
        System::new(load_tilemap_system)
            .with::<(TiledWorldComponent, Loadable)>()
            .build();
    }
}
pub mod update {
    use toxoid_api::{World, SpineInstance};
    #[cfg(feature = "render")]
    use toxoid_render::Renderer2D;
    use toxoid_api::toxoid_progress;
    pub extern "C" fn game_loop(_parg: *mut std::ffi::c_void) {
        #[cfg(feature = "fetch")] unsafe { toxoid_sokol::bindings::sfetch_dowork() };
        #[cfg(feature = "render")] toxoid_sokol::SokolRenderer2D::begin();
        unsafe { toxoid_progress(1.0) };
        #[cfg(feature = "render")]
        {
            let renderer_2d = &mut *toxoid_sokol::RENDERER_2D.lock().unwrap();
            renderer_2d.end();
        }
    }
    #[cfg(feature = "render")]
    pub extern "C" fn render_loop() {}
}
pub mod utils {
    pub mod rand {
        use rand::Rng;
        #[no_mangle]
        pub extern "C" fn gen_rng_range(min: i32, max: i32) -> i32 {
            let mut rng = rand::thread_rng();
            rng.gen_range(min..max)
        }
        #[no_mangle]
        pub extern "C" fn gen_rng_grid_pos() -> (i32, i32) {
            let mut rng = rand::thread_rng();
            let window_width = 1250;
            let window_height = 700;
            let entity_width = 50;
            let entity_height = 50;
            let range_max_x = window_width / entity_width;
            let range_max_y = window_height / entity_height;
            let random_x = rng.gen_range(0..range_max_x) * entity_width;
            let random_y = rng.gen_range(0..range_max_y) * entity_height;
            (random_x, random_y)
        }
    }
    pub mod network {
        use toxoid_api::*;
        use core::ffi::c_void;
        use std::collections::HashMap;
        #[cfg(target_os = "emscripten")]
        pub fn init() {
            let mut attributes = toxoid_ffi::emscripten::EmscriptenWebSocketCreateAttributes {
                url: "ws://127.0.0.1:8080\0".as_ptr() as *const i8,
                protocol: std::ptr::null(),
            };
            let ws = unsafe {
                toxoid_ffi::emscripten::emscripten_websocket_new(
                    &mut attributes
                        as *mut toxoid_ffi::emscripten::EmscriptenWebSocketCreateAttributes,
                )
            };
            let user_data = ws as *mut ::core::ffi::c_void;
            unsafe {
                toxoid_ffi::emscripten::emscripten_websocket_set_onopen_callback_on_thread(
                    ws,
                    user_data,
                    onopen_cb,
                    toxoid_ffi::emscripten::EM_CALLBACK_THREAD_CONTEXT_CALLING_THREAD
                        as *mut c_void,
                );
                toxoid_ffi::emscripten::emscripten_websocket_set_onmessage_callback_on_thread(
                    ws,
                    user_data,
                    onmessage_cb,
                    toxoid_ffi::emscripten::EM_CALLBACK_THREAD_CONTEXT_CALLING_THREAD,
                );
                toxoid_ffi::emscripten::emscripten_websocket_set_onerror_callback_on_thread(
                    ws,
                    user_data,
                    onerror_cb,
                    toxoid_ffi::emscripten::EM_CALLBACK_THREAD_CONTEXT_CALLING_THREAD,
                );
                toxoid_ffi::emscripten::emscripten_websocket_set_onclose_callback_on_thread(
                    ws,
                    user_data,
                    onclose_cb,
                    toxoid_ffi::emscripten::EM_CALLBACK_THREAD_CONTEXT_CALLING_THREAD,
                );
                {
                    ::std::io::_print(format_args!("Hello Emscripten WebSocket!\n"));
                };
            }
            World::add_singleton::<WebSocket>();
            let mut websocket = World::get_singleton::<WebSocket>();
            websocket.set_socket(Pointer { ptr: ws });
        }
        #[cfg(target_os = "emscripten")]
        pub extern "C" fn onopen_cb(
            _event_type: *mut ::std::os::raw::c_void,
            _user_data: *mut ::std::os::raw::c_void,
        ) {
            {
                ::std::io::_print(format_args!("Connection opened.\n"));
            };
            let mut websocket = World::get_singleton::<WebSocket>();
            websocket.set_connected(true);
        }
        #[cfg(target_os = "emscripten")]
        pub extern "C" fn onmessage_cb(
            _event_type: ::std::os::raw::c_int,
            websocket_event: *const toxoid_ffi::emscripten::EmscriptenWebSocketMessageEvent,
            _user_data: *mut ::std::os::raw::c_void,
        ) {
            let data = unsafe { (*websocket_event).data };
            let data_len = unsafe { (*websocket_event).numBytes };
            let data = unsafe { std::slice::from_raw_parts(data, data_len as usize) };
            let network_messages = toxoid_serialize::deserialize(data).unwrap();
            network_messages
                .messages
                .iter()
                .for_each(|message| {
                    unsafe {
                        toxoid_net::toxoid_net_run_event(message.event.clone(), message)
                    };
                });
        }
        #[cfg(target_os = "emscripten")]
        pub extern "C" fn onerror_cb(
            _event_type: *mut ::std::os::raw::c_void,
            _user_data: *mut ::std::os::raw::c_void,
        ) {
            {
                ::std::io::_print(format_args!("Connection error.\n"));
            };
        }
        #[cfg(target_os = "emscripten")]
        pub extern "C" fn onclose_cb(
            _event_type: *mut ::std::os::raw::c_void,
            _user_data: *mut ::std::os::raw::c_void,
        ) {
            let mut websocket = World::get_singleton::<WebSocket>();
            websocket.set_connected(false);
            {
                ::std::io::_print(format_args!("Connection closed.\n"));
            };
        }
        pub fn serialize_entity(
            entity_id: ecs_entity_t,
        ) -> Vec<toxoid_serialize::NetworkMessageComponent> {
            unsafe { toxoid_ffi::ecs::toxoid_serialize_entity(entity_id) }
        }
        pub fn serialize_component(
            entity_id: ecs_entity_t,
            component_id: ecs_entity_t,
        ) -> toxoid_serialize::NetworkMessageComponent {
            unsafe {
                toxoid_ffi::ecs::toxoid_serialize_component(entity_id, component_id)
            }
        }
        pub fn deserialize_entity(
            components_serialized: &[toxoid_api::MessageComponent],
        ) -> HashMap<
            std::string::String,
            HashMap<std::string::String, toxoid_ffi::ecs::DynamicType>,
        > {
            unsafe { toxoid_ffi::ecs::toxoid_deserialize_entity(components_serialized) }
        }
        pub fn deserialize_entity_sync(
            entity_id: ecs_entity_t,
            components_serialized: &[MessageComponent],
        ) {
            unsafe {
                toxoid_ffi::ecs::toxoid_deserialize_entity_sync(
                    entity_id,
                    components_serialized,
                )
            }
        }
    }
    pub mod load {
        use toxoid_api::*;
        #[cfg(feature = "render")]
        use toxoid_render::Renderer2D;
        #[cfg(any(feature = "fetch", feature = "audio", feature = "render"))]
        use toxoid_sokol::bindings::*;
        #[cfg(feature = "render")]
        use toxoid_sokol::SokolRenderer2D;
        #[cfg(feature = "fetch")]
        use core::ffi::CStr;
        #[cfg(feature = "fetch")]
        use core::ffi::c_void;
        #[cfg(feature = "fetch")]
        struct FetchUserData {
            entity: *mut Entity,
            callback: Box<dyn FnMut(&mut Entity)>,
        }
        #[cfg(feature = "fetch")]
        pub fn fetch(
            filename: &str,
            callback: unsafe extern "C" fn(*const sfetch_response_t),
            user_data: *mut c_void,
            user_data_size: usize,
        ) {
            let mut sfetch_request: sfetch_request_t = unsafe {
                core::mem::MaybeUninit::zeroed().assume_init()
            };
            let filename = std::ffi::CString::new(filename).unwrap();
            sfetch_request.path = filename.as_ptr();
            sfetch_request.channel = 0;
            let file_size = 124_0000;
            let buffer = Box::into_raw(
                ::alloc::vec::from_elem(0u8, file_size).into_boxed_slice(),
            );
            sfetch_request
                .buffer = sfetch_range_t {
                ptr: buffer as *const core::ffi::c_void,
                size: file_size,
            };
            sfetch_request.callback = Some(callback);
            sfetch_request
                .user_data = sfetch_range_t {
                ptr: user_data,
                size: user_data_size,
            };
            unsafe { sfetch_send(&sfetch_request) };
        }
        #[cfg(feature = "fetch")]
        pub unsafe extern "C" fn worldmap_load_callback(
            result: *const sfetch_response_t,
        ) {
            let data = unsafe { (*result).data.ptr as *const u8 };
            let size = unsafe { (*result).data.size };
            let data_string = unsafe {
                std::str::from_utf8(core::slice::from_raw_parts(data, size)).unwrap()
            };
            let world = toxoid_tiled::parse_world(data_string);
            let world_ptr = Box::into_raw(Box::new(world));
            let user_data: Box<FetchUserData> = Box::from_raw(
                (*result).user_data as *mut FetchUserData,
            );
            let mut entity: Box<Entity> = Box::from_raw(user_data.entity);
            entity.add::<TiledWorldComponent>();
            let mut world_component = entity.get::<TiledWorldComponent>();
            world_component
                .set_world(Pointer {
                    ptr: world_ptr as *mut c_void,
                });
            entity.add::<Loadable>();
            let mut user_data: Box<FetchUserData> = Box::from_raw(
                (*result).user_data as *mut FetchUserData,
            );
            (user_data.callback)(&mut *user_data.entity);
        }
        #[cfg(feature = "fetch")]
        pub extern "C" fn load_worldmap(
            filename: &str,
            callback: impl FnMut(&mut Entity) + 'static,
        ) -> *mut Entity {
            let entity = Entity::new();
            let entity_boxed = Box::into_raw(Box::new(entity));
            let user_data = Box::into_raw(
                Box::new(FetchUserData {
                    entity: entity_boxed,
                    callback: Box::new(callback),
                }),
            ) as *mut c_void;
            let size = core::mem::size_of::<FetchUserData>();
            fetch(filename, worldmap_load_callback, user_data, size);
            entity_boxed
        }
        #[cfg(feature = "fetch")]
        #[no_mangle]
        pub extern "C" fn toxoid_engine_load_worldmap(
            filename: &str,
            callback: extern "C" fn(*mut Entity),
        ) -> *mut Entity {
            load_worldmap(filename, move |entity: &mut Entity| { callback(entity) })
        }
        #[cfg(feature = "fetch")]
        pub unsafe extern "C" fn cell_load_callback(result: *const sfetch_response_t) {
            let data = unsafe { (*result).data.ptr as *const u8 };
            let size = unsafe { (*result).data.size };
            let data_string = unsafe {
                std::str::from_utf8(core::slice::from_raw_parts(data, size)).unwrap()
            };
            let cell = toxoid_tiled::parse_cell(data_string);
            let cell_ptr = Box::into_raw(Box::new(cell));
            let user_data: Box<FetchUserData> = Box::from_raw(
                (*result).user_data as *mut FetchUserData,
            );
            let mut entity: Box<Entity> = Box::from_raw(user_data.entity);
            entity.add::<TiledCellComponent>();
            let mut cell_component = entity.get::<TiledCellComponent>();
            cell_component
                .set_cell(Pointer {
                    ptr: cell_ptr as *mut c_void,
                });
            entity.add::<Loadable>();
            let mut user_data: Box<FetchUserData> = Box::from_raw(
                (*result).user_data as *mut FetchUserData,
            );
            (user_data.callback)(&mut *user_data.entity);
        }
        #[cfg(feature = "fetch")]
        pub fn load_cell(
            filename: &str,
            callback: impl FnMut(&mut Entity) + 'static,
        ) -> *mut Entity {
            let entity = Entity::new();
            let entity_boxed = Box::into_raw(Box::new(entity));
            let user_data = Box::into_raw(
                Box::new(FetchUserData {
                    entity: entity_boxed,
                    callback: Box::new(callback),
                }),
            ) as *mut c_void;
            let size = core::mem::size_of::<FetchUserData>();
            fetch(filename, cell_load_callback, user_data, size);
            entity_boxed
        }
        #[cfg(all(feature = "fetch", feature = "render"))]
        #[no_mangle]
        pub extern "C" fn toxoid_engine_load_sprite(
            filename: &str,
            callback: extern "C" fn(*mut Entity),
        ) -> *mut Entity {
            load_sprite(filename, move |entity: &mut Entity| { callback(entity) })
        }
        #[cfg(all(feature = "fetch", feature = "render"))]
        pub unsafe extern "C" fn sprite_load_callback(result: *const sfetch_response_t) {
            unsafe {
                if (*result).failed {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Failed to load image: {0}\n",
                                CStr::from_ptr((*result).path).to_str().unwrap(),
                            ),
                        );
                    };
                    return;
                }
                let mut user_data: Box<FetchUserData> = Box::from_raw(
                    (*result).user_data as *mut FetchUserData,
                );
                let mut entity: Box<Entity> = Box::from_raw(user_data.entity);
                let data = (*result).data.ptr as *const u8;
                let size = (*result).data.size;
                let sprite = SokolRenderer2D::create_sprite(data, size);
                let mut sprite_size = entity.get::<Size>();
                sprite_size.set_width(sprite.width());
                sprite_size.set_height(sprite.height());
                let mut sprite_component = entity.get::<Sprite>();
                sprite_component
                    .set_sprite(Pointer {
                        ptr: Box::into_raw(sprite) as *mut c_void,
                    });
                (user_data.callback)(&mut *user_data.entity);
            }
        }
        #[cfg(all(feature = "fetch", feature = "render"))]
        pub fn load_sprite(
            filename: &str,
            callback: impl FnMut(&mut Entity) + 'static,
        ) -> *mut Entity {
            let mut entity = Entity::new();
            entity.add::<Sprite>();
            entity.add::<Position>();
            entity.add::<Size>();
            let entity_boxed = Box::into_raw(Box::new(entity));
            let user_data = Box::into_raw(
                Box::new(FetchUserData {
                    entity: entity_boxed,
                    callback: Box::new(callback),
                }),
            ) as *mut c_void;
            let size = core::mem::size_of::<FetchUserData>();
            fetch(filename, sprite_load_callback, user_data, size);
            entity_boxed
        }
        #[cfg(all(feature = "fetch", feature = "render"))]
        pub unsafe extern "C" fn image_load_callback(result: *const sfetch_response_t) {
            if (*result).failed {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "Failed to load image: {0}\n",
                            CStr::from_ptr((*result).path).to_str().unwrap(),
                        ),
                    );
                };
                return;
            } else {}
            let img_info = *((*result).user_data as *mut sspine_image_info);
            let data = (*result).data.ptr as *const u8;
            let size = (*result).data.size as usize;
            SokolRenderer2D::init_image(img_info.sgimage, data, size);
            SokolRenderer2D::init_sampler(
                img_info.sgsampler,
                img_info.min_filter,
                img_info.mag_filter,
                img_info.mipmap_filter,
                img_info.wrap_u,
                img_info.wrap_v,
                &img_info.filename.cstr as *const _ as *const i8,
            );
        }
        #[cfg(all(feature = "fetch", feature = "render", feature = "spine"))]
        pub extern "C" fn animation_load_callback(result: *const sfetch_response_t) {
            unsafe {
                let mut user_data: Box<FetchUserData> = Box::from_raw(
                    (*result).user_data as *mut FetchUserData,
                );
                let mut entity: Box<Entity> = Box::from_raw(user_data.entity);
                let data = (*result).data.ptr as *const u8;
                let size = (*result).data.size;
                let url = CStr::from_ptr((*result).path).to_str().unwrap();
                if url.contains("atlas") {
                    let mut atlas = (*entity).get::<Atlas>();
                    atlas.set_loaded(true);
                    atlas.set_data_size(size as i32);
                    atlas.set_data(Pointer::new(data as *mut c_void));
                } else if url.contains("json") {
                    let mut skeleton = (*entity).get::<Skeleton>();
                    skeleton.set_loaded(true);
                    skeleton.set_data_size(size as i32);
                    skeleton.set_data(Pointer::new(data as *mut c_void));
                }
                if (*entity).get::<Atlas>().get_loaded()
                    && (*entity).get::<Skeleton>().get_loaded()
                {
                    let mut atlas_desc: sspine_atlas_desc = core::mem::MaybeUninit::zeroed()
                        .assume_init();
                    let mut atlas = (*entity).get::<Atlas>();
                    let data_ptr = atlas.get_data().ptr;
                    atlas_desc
                        .data = sspine_range {
                        ptr: data_ptr,
                        size: size as usize,
                    };
                    let spine_atlas = sspine_make_atlas(&atlas_desc);
                    atlas
                        .set_atlas(
                            Pointer::new(
                                Box::into_raw(Box::new(spine_atlas)) as *mut c_void,
                            ),
                        );
                    let mut skeleton_desc: sspine_skeleton_desc = core::mem::MaybeUninit::zeroed()
                        .assume_init();
                    let mut skeleton = (*entity).get::<Skeleton>();
                    let data_ptr = skeleton.get_data().ptr;
                    let size = skeleton.get_data_size() as usize;
                    let data_ptr = core::slice::from_raw_parts(data_ptr, size + 1)
                        .as_ptr() as *const i8;
                    let data_ptr = std::ffi::CString::from_raw(data_ptr as *mut i8);
                    let data_ptr = data_ptr.as_ptr();
                    skeleton_desc.atlas = spine_atlas;
                    skeleton_desc.json_data = data_ptr as *const i8;
                    skeleton_desc.prescale = 2.0;
                    skeleton_desc.anim_default_mix = 0.2;
                    let spine_skeleton = sspine_make_skeleton(&skeleton_desc);
                    skeleton
                        .set_skeleton(
                            Pointer::new(
                                Box::into_raw(Box::new(spine_skeleton)) as *mut c_void,
                            ),
                        );
                    let mut spine_instance_desc: sspine_instance_desc = core::mem::MaybeUninit::zeroed()
                        .assume_init();
                    spine_instance_desc.skeleton = spine_skeleton;
                    let instance = sspine_make_instance(&spine_instance_desc);
                    (*entity).add::<SpineInstance>();
                    let mut instance_component = (*entity).get::<SpineInstance>();
                    let instance_ptr = Box::new(instance);
                    let instance_ptr = Box::into_raw(instance_ptr) as *mut c_void;
                    instance_component.set_instance(Pointer::new(instance_ptr));
                    instance_component.set_instantiated(true);
                    sspine_add_animation(
                        instance,
                        sspine_anim_by_name(spine_skeleton, make_c_string("idle_down")),
                        0,
                        true,
                        0.,
                    );
                    let atlas_images_num = sspine_num_images(spine_atlas);
                    for img_index in 0..atlas_images_num {
                        let img = sspine_image_by_index(spine_atlas, img_index);
                        let img_info = sspine_get_image_info(img);
                        let filename_c_str = core::ffi::CStr::from_ptr(
                            img_info.filename.cstr.as_ptr(),
                        );
                        let file_path = {
                            let res = ::alloc::fmt::format(
                                format_args!("assets/{0}", filename_c_str.to_str().unwrap()),
                            );
                            res
                        };
                        let file_path = file_path.as_str();
                        let img_ptr = Box::new(img_info);
                        let img_ptr = Box::into_raw(img_ptr) as *mut c_void;
                        fetch(
                            file_path,
                            image_load_callback,
                            img_ptr as *mut c_void,
                            std::mem::size_of::<sspine_image_info>(),
                        );
                    }
                }
            }
        }
        #[cfg(all(feature = "fetch", feature = "render", feature = "spine"))]
        pub fn load_animation(
            atlas_filename: &str,
            skeleton_filename: &str,
            callback: impl FnMut(&mut Entity) + 'static,
        ) -> *mut Entity {
            unsafe {
                let mut entity = Entity::new();
                entity.add::<Loadable>();
                entity.add::<Position>();
                entity.add::<Size>();
                entity.add::<BoneAnimation>();
                entity.add::<Skeleton>();
                entity.add::<Atlas>();
                entity.add::<Images>();
                let mut atlas = entity.get::<Atlas>();
                atlas.set_filename(StringPtr::new(atlas_filename));
                let mut skeleton = entity.get::<Skeleton>();
                skeleton.set_filename(StringPtr::new(skeleton_filename));
                let entity_boxed = Box::into_raw(Box::new(entity));
                let user_data = Box::into_raw(
                    Box::new(FetchUserData {
                        entity: entity_boxed,
                        callback: Box::new(callback),
                    }),
                ) as *mut c_void;
                let size = core::mem::size_of::<FetchUserData>();
                fetch(skeleton_filename, animation_load_callback, user_data, size);
                fetch(atlas_filename, animation_load_callback, user_data, size);
                entity_boxed
            }
        }
    }
    pub mod input {
        use toxoid_api::*;
        #[cfg(target_os = "emscripten")]
        use toxoid_ffi::emscripten::{
            EmBool, EmscriptenKeyboardEvent, toxoid_set_keydown_callback,
            toxoid_set_keyup_callback,
        };
        #[cfg(target_os = "emscripten")]
        unsafe extern "C" fn keydown_cb(
            _event_type: core::ffi::c_int,
            key_event: *const EmscriptenKeyboardEvent,
            _user_data: *mut core::ffi::c_void,
        ) -> EmBool {
            let key = unsafe { (*key_event).keyCode };
            let mut keyboard_input = World::get_singleton::<KeyboardInput>();
            if key == KeyCode::Up as u32 {
                keyboard_input.set_up(true);
            }
            if key == KeyCode::Down as u32 {
                keyboard_input.set_down(true);
            }
            if key == KeyCode::Left as u32 {
                keyboard_input.set_left(true);
            }
            if key == KeyCode::Right as u32 {
                keyboard_input.set_right(true);
            }
            return 0;
        }
        #[cfg(target_os = "emscripten")]
        unsafe extern "C" fn keyup_cb(
            _event_type: core::ffi::c_int,
            key_event: *const EmscriptenKeyboardEvent,
            _user_data: *mut core::ffi::c_void,
        ) -> EmBool {
            let key = unsafe { (*key_event).keyCode };
            let mut keyboard_input = World::get_singleton::<KeyboardInput>();
            if key == KeyCode::Up as u32 {
                keyboard_input.set_up(false);
            }
            if key == KeyCode::Down as u32 {
                keyboard_input.set_down(false);
            }
            if key == KeyCode::Left as u32 {
                keyboard_input.set_left(false);
            }
            if key == KeyCode::Right as u32 {
                keyboard_input.set_right(false);
            }
            return 0;
        }
        pub fn init() {
            #[cfg(target_os = "emscripten")]
            {
                let canvas_id = std::ffi::CString::new("canvas").unwrap();
                unsafe {
                    let result = toxoid_ffi::emscripten::toxoid_set_keydown_callback(
                        canvas_id.as_ptr() as *const core::ffi::c_char,
                        std::ptr::null_mut(),
                        1,
                        keydown_cb,
                    );
                    if result != 0 {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!("Error setting keydown callback"),
                            );
                        };
                    }
                    let result = toxoid_ffi::emscripten::toxoid_set_keyup_callback(
                        canvas_id.as_ptr() as *const core::ffi::c_char,
                        std::ptr::null_mut(),
                        1,
                        keyup_cb,
                    );
                    if result != 0 {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!("Error setting keyup callback"),
                            );
                        };
                    }
                }
            }
        }
    }
    pub mod futures {
        use once_cell::sync::Lazy;
        use std::sync::atomic::Ordering;
        use std::sync::atomic::AtomicU8;
        use std::collections::HashMap;
        use std::future::Future;
        use std::pin::Pin;
        use std::task::Waker;
        use std::task::{Context, Poll};
        use std::task::{RawWaker, RawWakerVTable};
        use toxoid_api::Entity;
        pub struct LoaderState {
            pub completed: bool,
            pub waker: Option<Waker>,
            pub response: Option<Result<Entity, ()>>,
        }
        pub static mut LOADER_FUTURES_STATE: Lazy<HashMap<u8, LoaderState>> = Lazy::new(||
        HashMap::new());
        pub static LOADER_FUTURE_ID: Lazy<AtomicU8> = Lazy::new(|| AtomicU8::new(0));
        pub struct FetchFuture {
            pub id: u8,
        }
        impl FetchFuture {
            pub fn new() -> Self {
                let id = LOADER_FUTURE_ID.fetch_add(1, Ordering::SeqCst);
                unsafe {
                    LOADER_FUTURES_STATE
                        .insert(
                            id,
                            LoaderState {
                                completed: false,
                                waker: None,
                                response: None,
                            },
                        )
                };
                {
                    ::std::io::_print(format_args!("Created future {0}\n", id));
                };
                FetchFuture { id }
            }
        }
        impl Future for FetchFuture {
            type Output = Result<Entity, ()>;
            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                {
                    ::std::io::_print(format_args!("Polling future {0}\n", self.id));
                };
                let shared_state = unsafe {
                    LOADER_FUTURES_STATE.get_mut(&self.id).unwrap()
                };
                if shared_state.completed {
                    Poll::Ready(shared_state.response.take().unwrap())
                } else {
                    shared_state.waker = Some(cx.waker().clone());
                    Poll::Pending
                }
            }
        }
        pub fn block_on<F: Future>(mut future: F) -> F::Output {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);
            loop {
                match unsafe { Pin::new_unchecked(&mut future) }.poll(&mut context) {
                    Poll::Ready(value) => return value,
                    Poll::Pending => continue,
                }
            }
        }
        pub fn dummy_waker() -> Waker {
            unsafe fn clone(_: *const ()) -> RawWaker {
                dummy_raw_waker()
            }
            unsafe fn wake(_: *const ()) {}
            unsafe fn wake_by_ref(_: *const ()) {}
            unsafe fn drop(_: *const ()) {}
            unsafe fn dummy_raw_waker() -> RawWaker {
                let vtable = &RawWakerVTable::new(clone, wake, wake_by_ref, drop);
                RawWaker::new(std::ptr::null(), vtable)
            }
            unsafe { Waker::from_raw(dummy_raw_waker()) }
        }
    }
}
pub use systems::*;
pub use update::*;
pub use utils::*;
pub use utils::rand::*;
pub fn init() {
    toxoid_ffi::flecs_core::init();
    toxoid_api::components::init();
    #[cfg(feature = "render")] toxoid_sokol::init(render_loop);
    prefabs::init();
    utils::network::init();
    utils::input::init();
    systems::init();
    #[cfg(target_os = "emscripten")] toxoid_ffi::emscripten::start_loop(game_loop);
}
