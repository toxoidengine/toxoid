use core::ffi::c_void;

use rand::Rng;
use toxoid_ffi::emscripten::*;
use toxoid_api::*;
use toxoid_sokol::bindings::sspine_atlas_desc;

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
    let random_x = rng.gen_range(0..range_max_x) * entity_width; // divided by entity height 
    let random_y = rng.gen_range(0..range_max_y) * entity_height; 
    (random_x, random_y)
}


#[cfg(target_os = "emscripten")]
pub fn fetch(filename: &str, user_data: *mut core::ffi::c_void, success_cb: extern "C" fn(*mut emscripten_fetch_t), error_cb: extern "C" fn(*mut emscripten_fetch_t)) {
    // Create fetch attributes
    let mut attr: emscripten_fetch_attr_t = unsafe { core::mem::MaybeUninit::zeroed().assume_init()  };
    unsafe { emscripten_fetch_attr_init(&mut attr) };
    attr.attributes = EMSCRIPTEN_FETCH_LOAD_TO_MEMORY;
    // Callbacks
    attr.onsuccess = Some(success_cb);
    attr.onerror = Some(error_cb);
    attr.userData = user_data;

    // Convert filename to CString
    let filename_cstring = std::ffi::CString::new(filename).unwrap();
            
    // Fetch file
    unsafe { emscripten_fetch(&attr, filename_cstring.as_ptr()) };
}

#[cfg(not(target_os = "emscripten"))]
pub fn fetch() {
    // ssfetch_send(&mut sfetch_request);
}

#[cfg(target_os = "emscripten")]
pub fn load_image(filename: &str) -> &mut Entity {
    println!("Loading image: {}", filename);

    // Create entity and pass it to fetch
    let mut entity = Entity::new();
    entity.add::<Loadable>();
    entity.add::<Sprite>();
    entity.add::<Position>();
    entity.add::<Size>();

    let entity_box = Box::new(entity);
    let entity_box = Box::leak(entity_box);
    let entity_raw = entity_box as *mut _ as *mut core::ffi::c_void;
    fetch(filename, entity_raw, image_load_success, image_load_fail);
    
    entity_box
}

// this function is called when both the spine atlas and skeleton file has been loaded,
// first an atlas object is created from the loaded atlas data, and then a skeleton
// object (which requires an atlas object as dependency), then a spine instance object.
// Finally any images required by the atlas object are loaded
// const sfetch_response_t* respons
pub extern "C" fn animation_load_success(result: *mut emscripten_fetch_t) {
    // From Cstring (*result).url
    use core::ffi::CStr;
    unsafe {
        let url = CStr::from_ptr((*result).url).to_str().unwrap();
        let entity = (*result).userData as *mut Entity;
        if url.contains("atlas") {
            println!("Successfully loaded atlas {}", url);
            let mut atlas = (*entity).get::<Atlas>();
            atlas.set_loaded(true);
            atlas.set_data_size((*result).totalBytes);
            atlas.set_atlas(Pointer::new((*result).data as *mut core::ffi::c_void));
        } else if url.contains("json") {
            println!("Successfully loaded skeleton {}", url);
            let mut skeleton = (*entity).get::<Skeleton>();
            skeleton.set_loaded(true);
            skeleton.set_data_size((*result).totalBytes);
            skeleton.set_skeleton(Pointer::new((*result).data as *mut core::ffi::c_void));
        }
        
        if (*entity).get::<Atlas>().get_loaded() && (*entity).get::<Skeleton>().get_loaded() {

            // Create spine atlas object from loaded atlas data.
            let mut atlas_desc: sspine_atlas_desc = core::mem::MaybeUninit::zeroed().assume_init();
            let atlas = (*entity).get::<Atlas>();
            let ptr = atlas.get_atlas().ptr;
            atlas_desc.data = toxoid_sokol::bindings::sspine_range {
                ptr,
                size: (*result).totalBytes as usize
            };
            let spine_atlas = toxoid_sokol::bindings::sspine_make_atlas(&atlas_desc);

            // Next create a spine skeleton object, skeleton data files can be either
            // text (JSON) or binary (in our case, 'raptor-pro.skel' is a binary skeleton file).
            // In case of JSON data, make sure that the data is 0-terminated!
            let mut skeleton_desc: toxoid_sokol::bindings::sspine_skeleton_desc = core::mem::MaybeUninit::zeroed().assume_init();
            let skeleton = (*entity).get::<Skeleton>();
            let ptr = skeleton.get_skeleton().ptr;
            let size = skeleton.get_data_size() as usize;
            skeleton_desc.atlas = spine_atlas;
            skeleton_desc.json_data = ptr as *const i8;
            skeleton_desc.prescale = 0.5;
            skeleton_desc.anim_default_mix = 0.2;

            let spine_skeleton = toxoid_sokol::bindings::sspine_make_skeleton(&skeleton_desc);

            let mut spine_instance_desc: toxoid_sokol::bindings::sspine_instance_desc = core::mem::MaybeUninit::zeroed().assume_init();
            spine_instance_desc.skeleton = spine_skeleton;

            // create a spine instance object, that's the thing that's actually rendered
            let instance = toxoid_sokol::bindings::sspine_make_instance(&spine_instance_desc);

            // Since the spine instance doesn't move, its position can be set once,
            // the coordinate units depends on the sspine_layer_transform struct
            // that's passed to the sspine_draw_layer() during rendering (in our
            // case it's simply framebuffer pixels, with the origin in the
            // center)
            toxoid_sokol::bindings::sspine_set_position(instance, toxoid_sokol::bindings::sspine_vec2 { x: -100., y:200. });

            // configure a simple animation sequence
            let anim_c_string = std::ffi::CString::new("idle_down_weapon").unwrap();
            let anim_c_string = anim_c_string.as_ptr();
            toxoid_sokol::bindings::sspine_add_animation(instance, toxoid_sokol::bindings::sspine_anim_by_name(spine_skeleton, anim_c_string), 0, true, 0.);
            
            let atlas_images_num = toxoid_sokol::bindings::sspine_num_images(spine_atlas);
            for img_index in 0..atlas_images_num {
                let img = toxoid_sokol::bindings::sspine_image_by_index(spine_atlas, img_index);
                let img_info = toxoid_sokol::bindings::sspine_get_image_info(img);
                println!("Image info filename: {:?}", img_info);

                let filename_c_str = core::ffi::CStr::from_ptr(img_info.filename.cstr.as_ptr());
                println!("Image info filename: {:?}", filename_c_str);
                // We'll store the sspine_image handle in the fetch request's user data
                // blob, because we need the image info again later in the fetch callback
                // in order to initialize the sokol-gfx image with the right parameters.
                //      
                // Also important to note: all image fetch requests load their data into the same
                // buffer. This is fine because sokol-fetch has been configured
                // with num_lanes=1, this will cause all requests on the same
                // channel to be serialized (not run in parallel). That way
                // the same buffer can be reused even if there are multiple atlas images.
                // The downside is that loading multiple images would take longer.
                // core::
                let file_path = format!("assets/{}", filename_c_str.to_str().unwrap());
                let file_path = file_path.as_str();
                fetch(file_path, (*result).userData, images_load_success, images_load_fail);
            }
        }
        emscripten_fetch_close(result);
    }
}

pub extern "C" fn images_load_success(result: *mut emscripten_fetch_t) {
    println!("Images load success");
}

pub extern "C" fn images_load_fail(result: *mut emscripten_fetch_t) {
    println!("Failed images load");
}

pub extern "C" fn animation_load_failed(result: *mut emscripten_fetch_t) {
    println!("Failed animation load");
}

#[cfg(target_os = "emscripten")]
pub fn load_animation(atlas_filename: &str, skeleton_filename: &str) -> &'static mut Entity {
    unsafe {
        // Create entity and pass it to fetch
        let mut entity = Entity::new();
        entity.add::<Loadable>();
        entity.add::<Position>();
        entity.add::<Size>();
        entity.add::<BoneAnimation>();
        entity.add::<Skeleton>();
        entity.add::<Atlas>();
        entity.add::<Images>();

        let mut atlas = entity.get::<Atlas>();
        atlas.set_atlas_filename(StringPtr::new(atlas_filename));
        let mut skeleton = entity.get::<Skeleton>();
        skeleton.set_skeleton_filename(StringPtr::new(skeleton_filename));

        let entity_box = Box::new(entity);
        let entity_box = Box::leak(entity_box);
        let entity_raw = entity_box as *mut _ as *mut core::ffi::c_void;

        entity_box
    }
}

pub extern "C" fn image_load_success(result: *mut emscripten_fetch_t) {
    unsafe {
        use core::ffi::CStr;
        println!("Successfully loaded {}", CStr::from_ptr((*result).url).to_str().unwrap());

        // Grab entity passed into fetch attributes
        let entity_raw = (*result).userData;
        let entity_box: Box<Entity> = Box::from_raw(entity_raw as *mut Entity);

        // Get fetch data and size
        let data = (*result).data as *mut u8;
        let size = (*result).totalBytes as usize;
        
        // Create image
        // create_image(data, size, entity_box);

        // Close fetch
        emscripten_fetch_close(result);
    }
}

pub extern "C" fn image_load_fail(fetch: *mut emscripten_fetch_t) {
    unsafe {
        println!("Fail!");
        // eprintln!("Failed to load image: {}", CStr::from_ptr((*fetch).url).to_str().unwrap());
        // emscripten_fetch_close(fetch);
    }
}

pub fn create_image(data: *mut u8, size: usize, mut entity: Box<Entity>) {
    // use toxoid_sokol::{SokolRenderer2D, SokolSprite};
    // use toxoid_sokol::sokol::{app as sapp, gfx as sg};
    // use toxoid_sokol::bindings::*;
    // use core::ffi::{c_void, c_int};
    // use core::ptr;

    // let mut width: i32 = 0;
    // let mut height: i32 = 0;
    // let mut channels: i32 = 0;
    
    // let image_data = unsafe {
    //     // Converts from PNG format to RGBA8 format
    //     stbi_load_from_memory(data as *const u8, size as c_int, &mut width, &mut height, &mut channels, 0)
    // };

    // // Create sg_image from data
    // let mut image_desc = sg_image_desc {
    //     _start_canary: 0,
    //     type_: sg_image_type_SG_IMAGETYPE_2D,
    //     render_target: false,
    //     width: 100, // You need to provide the width
    //     height: 100, // You need to provide the height
    //     num_slices: 1,
    //     num_mipmaps: 1,
    //     usage: sg_usage_SG_USAGE_IMMUTABLE,
    //     pixel_format: sg_pixel_format_SG_PIXELFORMAT_RGBA8,
    //     sample_count: 1,
    //     data: sg_image_data {
    //         subimage: [[sg_range { ptr: image_data as *const c_void, size: (width * height * 4) as usize }; 16]; 6],
    //     },
    //     label: ptr::null(),
    //     gl_textures: [0; 2usize],
    //     gl_texture_target: 0,
    //     mtl_textures: [ptr::null(); 2usize],
    //     d3d11_texture: ptr::null(),
    //     d3d11_shader_resource_view: ptr::null(),
    //     wgpu_texture: ptr::null(),
    //     wgpu_texture_view: ptr::null(),
    //     _end_canary: 0,
    // };

    // let image = unsafe { sg_make_image(&mut image_desc) };
    // let sprite_box = Box::new(SokolSprite {
    //         width: width as u32,
    //         height: height as u32,
    //         image: sg::Image { id: image.id },
    // });
    // let sprite_box = Box::leak(sprite_box);

    // let mut sprite = entity.get::<Sprite>();
    // sprite.set_sprite(Pointer::new(sprite_box as *mut _ as *mut c_void));

    // entity.add::<Renderable>();
}
