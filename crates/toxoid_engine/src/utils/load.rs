// TODO: Make this file more crossplatform generic and less dependent on Emscripten (must fix sokol-fetch)
use toxoid_api::*;
use toxoid_render::Renderer2D;
#[cfg(any(feature = "fetch", feature = "audio", feature = "render"))]
use toxoid_sokol::bindings::*;
#[cfg(feature = "render")]
use toxoid_sokol::SokolRenderer2D;
use core::ffi::CStr;
use core::ffi::c_void;
use std::mem::MaybeUninit;

struct FetchUserData {
    entity: *mut Entity,
    callback: fn(&mut Entity)
}

#[cfg(feature = "fetch")]
pub fn fetch(filename: &str, callback: unsafe extern "C" fn(*const sfetch_response_t), user_data: *mut c_void, user_data_size: usize) {
    // Create fetch description
    let mut sfetch_request: sfetch_request_t = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
    let filename = std::ffi::CString::new(filename).unwrap();
    sfetch_request.path = filename.as_ptr();
    sfetch_request.channel = 0;
    // PNG buffer - 4.00 KB
    // TODO: Figure out from server what the size of the file is
    // 123 KB
    let file_size = 124_0000;
    // let file_size = 4_096;
    let buffer = Box::into_raw(
        vec![0u8; file_size]
            .into_boxed_slice()
    );
    sfetch_request.buffer = sfetch_range_t {
        ptr: buffer as *const core::ffi::c_void,
        size: file_size
    };
    sfetch_request.callback = Some(callback);
    sfetch_request.user_data = sfetch_range_t {
        ptr: user_data,
        size: user_data_size
    };
    unsafe { sfetch_send(&sfetch_request) };
}

#[cfg(all(feature = "fetch", feature = "render"))]
pub fn load_sprite(filename: &str, callback: fn(&mut Entity)) -> *mut Entity {
    // println!("Loading image: {}", filename);
    // Create entity and pass it to fetch
    let mut entity = Entity::new();
    entity.add::<Sprite>();
    entity.add::<Position>();
    entity.add::<Size>();

    let entity_boxed = Box::into_raw(Box::new(entity));
    let size = core::mem::size_of::<Entity>();
    let user_data = Box::into_raw(Box::new(FetchUserData {
        entity: entity_boxed,
        callback
    })) as *mut c_void;
    fetch(filename, sprite_load_callback, user_data, size);
    entity_boxed
}

#[cfg(all(feature = "fetch", feature = "render"))]
pub unsafe extern "C" fn sprite_load_callback(result: *const sfetch_response_t) {
    unsafe {
        if (*result).failed {
            eprintln!("Failed to load image: {}", CStr::from_ptr((*result).path).to_str().unwrap());
            return;
        }

        // println!("Successfully loaded {}", CStr::from_ptr((*result).url).to_str().unwrap());

        // Get user data
        let user_data: Box<FetchUserData> = Box::from_raw((*result).user_data as *mut FetchUserData);

        // Grab entity from user data
        let mut entity: Box<Entity> = Box::from_raw(user_data.entity);

        // Get image data
        let data = (*result).data.ptr as *const u8;
        let size = (*result).data.size;

        // Create sprite
        let sprite = SokolRenderer2D::create_sprite(data, size);

        // Set sprite size
        let mut sprite_size = entity.get::<Size>();
        sprite_size.set_width(sprite.width());
        sprite_size.set_height(sprite.height());
        
        // Set sprite object
        let mut sprite_component = entity.get::<Sprite>();
        sprite_component.set_sprite(Pointer { 
            ptr: Box::into_raw(sprite) as *mut c_void 
        });

        // Flag as renderable for draw_sprite_system
        // entity.add::<Renderable>();

        // Call load_sprite callback
        (user_data.callback)(&mut *user_data.entity);
    }
}

// this function is called when both the spine atlas and skeleton file has been loaded,
// first an atlas object is created from the loaded atlas data, and then a skeleton
// object (which requires an atlas object as dependency), then a spine instance object.
// Finally any images required by the atlas object are loaded
#[cfg(all(feature = "fetch", feature = "render", feature = "spine"))]
pub extern "C" fn animation_load_success(result: *const sfetch_response_t) {
    // unsafe {
    //     let url = CStr::from_ptr((*result).url).to_str().unwrap();
    //     let entity = (*result).userData as *mut Entity;
    //     if url.contains("atlas") {
    //         // println!("Successfully loaded atlas {}", url);
    //         let mut atlas = (*entity).get::<Atlas>();
    //         atlas.set_loaded(true);
    //         atlas.set_data_size((*result).totalBytes);
    //         atlas.set_atlas(Pointer::new((*result).data as *mut c_void));
    //     } else if url.contains("json") {
    //         // println!("Successfully loaded skeleton {}", url);
    //         let mut skeleton = (*entity).get::<Skeleton>();
    //         skeleton.set_loaded(true);
    //         skeleton.set_data_size((*result).totalBytes);
    //         skeleton.set_skeleton(Pointer::new((*result).data as *mut c_void));
    //     }
        
    //     if (*entity).get::<Atlas>().get_loaded() && (*entity).get::<Skeleton>().get_loaded() {

    //         // Create spine atlas object from loaded atlas data.
    //         let mut atlas_desc: sspine_atlas_desc = core::mem::MaybeUninit::zeroed().assume_init();
    //         let atlas = (*entity).get::<Atlas>();
    //         let ptr = atlas.get_atlas().ptr;
    //         atlas_desc.data = sspine_range {
    //             ptr,
    //             size: (*result).totalBytes as usize
    //         };
    //         let spine_atlas = sspine_make_atlas(&atlas_desc);

    //         // Next create a spine skeleton object, skeleton data files can be either
    //         // text (JSON) or binary (in our case, 'raptor-pro.skel' is a binary skeleton file).
    //         // In case of JSON data, make sure that the data is 0-terminated!
    //         let mut skeleton_desc: sspine_skeleton_desc = core::mem::MaybeUninit::zeroed().assume_init();
    //         let skeleton = (*entity).get::<Skeleton>();
    //         let ptr = skeleton.get_skeleton().ptr;
    //         let size = skeleton.get_data_size() as usize;
    //         let ptr = core::slice::from_raw_parts(ptr, size + 1).as_ptr() as *const i8;
    //         let ptr = std::ffi::CString::from_raw(ptr as *mut i8);
    //         let ptr = ptr.as_ptr();
    //         skeleton_desc.atlas = spine_atlas;
    //         skeleton_desc.json_data = ptr as *const i8;
    //         skeleton_desc.prescale = 5.0;
    //         skeleton_desc.anim_default_mix = 0.2;

    //         let spine_skeleton = sspine_make_skeleton(&skeleton_desc);

    //         let mut spine_instance_desc: sspine_instance_desc = core::mem::MaybeUninit::zeroed().assume_init();
    //         spine_instance_desc.skeleton = spine_skeleton;

    //         // create a spine instance object, that's the thing that's actually rendered
    //         let instance = sspine_make_instance(&spine_instance_desc);

    //         // Store Spine instance in singleton for now
    //         (*entity).add::<SpineInstance>();
    //         let mut instance_singleton = (*entity).get::<SpineInstance>();
    //         let instance_ptr = Box::new(instance);
    //         let instance_ptr = Box::into_raw(instance_ptr) as *mut c_void;
    //         instance_singleton.set_instance(Pointer::new(instance_ptr));
    //         instance_singleton.set_instantiated(true);

    //         // Since the spine instance doesn't move, its position can be set once,
    //         // the coordinate units depends on the sspine_layer_transform struct
    //         // that's passed to the sspine_draw_layer() during rendering (in our
    //         // case it's simply framebuffer pixels, with the origin in the
    //         // center)
    //         sspine_set_position(instance, sspine_vec2 { x: 0., y: 0. });

    //         // configure a simple animation sequence
    //         let anim_c_string = std::ffi::CString::new("idle_down_weapon").unwrap();
    //         let anim_c_string = anim_c_string.as_ptr();
    //         sspine_add_animation(instance, sspine_anim_by_name(spine_skeleton, anim_c_string), 0, true, 0.);
            
    //         let atlas_images_num = sspine_num_images(spine_atlas);
    //         for img_index in 0..atlas_images_num {
    //             let img = sspine_image_by_index(spine_atlas, img_index);
    //             let img_info = sspine_get_image_info(img);
    //             let filename_c_str = core::ffi::CStr::from_ptr(img_info.filename.cstr.as_ptr());
    //             // We'll store the sspine_image handle in the fetch request's user data
    //             // blob, because we need the image info again later in the fetch callback
    //             // in order to initialize the sokol-gfx image with the right parameters.
    //             //      
    //             // Also important to note: all image fetch requests load their data into the same
    //             // buffer. This is fine because sokol-fetch has been configured
    //             // with num_lanes=1, this will cause all requests on the same
    //             // channel to be serialized (not run in parallel). That way
    //             // the same buffer can be reused even if there are multiple atlas images.
    //             // The downside is that loading multiple images would take longer.
    //             let file_path = format!("assets/{}", filename_c_str.to_str().unwrap());
    //             let file_path = file_path.as_str();
    //             let img_ptr = Box::new(img);
    //             let img_ptr = Box::into_raw(img_ptr) as *mut c_void;
               
    //             fetch(file_path, img_ptr as *mut c_void, images_load_success, images_load_fail);
    //         }
    //     }
    //     // emscripten_fetch_close(result);
    // }
}

#[cfg(all(feature = "fetch", feature = "render", feature = "spine"))]
pub fn load_animation(atlas_filename: &str, skeleton_filename: &str) -> &'static mut Entity {
    unimplemented!();
    // unsafe {
    //     // Create entity and pass it to fetch
    //     let mut entity = Entity::new();
    //     entity.add::<Loadable>();
    //     entity.add::<Position>();
    //     entity.add::<Size>();
    //     entity.add::<BoneAnimation>();
    //     entity.add::<Skeleton>();
    //     entity.add::<Atlas>();
    //     entity.add::<Images>();

    //     let mut atlas = entity.get::<Atlas>();
    //     atlas.set_atlas_filename(StringPtr::new(atlas_filename));
    //     let mut skeleton = entity.get::<Skeleton>();
    //     skeleton.set_skeleton_filename(StringPtr::new(skeleton_filename));

    //     let entity_box = Box::new(entity);
    //     let entity_box = Box::leak(entity_box);
    //     let entity_raw = entity_box as *mut _ as *mut c_void;

    //     entity_box
    // }
}
