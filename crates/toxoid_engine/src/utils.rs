use rand::Rng;
use toxoid_ffi::emscripten::*;
use toxoid_api::*;
use toxoid_sokol::bindings::*;
use core::ffi::CStr;
use core::ffi::c_void;
use std::mem::MaybeUninit;
use std::mem::size_of;

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
pub fn fetch(filename: &str, user_data: *mut c_void, success_cb: extern "C" fn(*mut emscripten_fetch_t), error_cb: extern "C" fn(*mut emscripten_fetch_t)) {
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
    let entity_raw = entity_box as *mut _ as *mut c_void;
    fetch(filename, entity_raw, image_load_success, image_load_fail);
    
    entity_box
}

// this function is called when both the spine atlas and skeleton file has been loaded,
// first an atlas object is created from the loaded atlas data, and then a skeleton
// object (which requires an atlas object as dependency), then a spine instance object.
// Finally any images required by the atlas object are loaded
// const sfetch_response_t* respons
pub extern "C" fn animation_load_success(result: *mut emscripten_fetch_t) {
    unsafe {
        let url = CStr::from_ptr((*result).url).to_str().unwrap();
        let entity = (*result).userData as *mut Entity;
        if url.contains("atlas") {
            println!("Successfully loaded atlas {}", url);
            let mut atlas = (*entity).get::<Atlas>();
            atlas.set_loaded(true);
            atlas.set_data_size((*result).totalBytes);
            atlas.set_atlas(Pointer::new((*result).data as *mut c_void));
        } else if url.contains("json") {
            println!("Successfully loaded skeleton {}", url);
            let mut skeleton = (*entity).get::<Skeleton>();
            skeleton.set_loaded(true);
            skeleton.set_data_size((*result).totalBytes);
            skeleton.set_skeleton(Pointer::new((*result).data as *mut c_void));
        }
        
        if (*entity).get::<Atlas>().get_loaded() && (*entity).get::<Skeleton>().get_loaded() {

            // Create spine atlas object from loaded atlas data.
            let mut atlas_desc: sspine_atlas_desc = core::mem::MaybeUninit::zeroed().assume_init();
            let atlas = (*entity).get::<Atlas>();
            let ptr = atlas.get_atlas().ptr;
            atlas_desc.data = sspine_range {
                ptr,
                size: (*result).totalBytes as usize
            };
            let spine_atlas = sspine_make_atlas(&atlas_desc);

            // Next create a spine skeleton object, skeleton data files can be either
            // text (JSON) or binary (in our case, 'raptor-pro.skel' is a binary skeleton file).
            // In case of JSON data, make sure that the data is 0-terminated!
            let mut skeleton_desc: sspine_skeleton_desc = core::mem::MaybeUninit::zeroed().assume_init();
            let skeleton = (*entity).get::<Skeleton>();
            let ptr = skeleton.get_skeleton().ptr;
            let size = skeleton.get_data_size() as usize;
            let ptr = core::slice::from_raw_parts(ptr, size + 1).as_ptr() as *const i8;
            let ptr = std::ffi::CString::from_raw(ptr as *mut i8);
            let ptr = ptr.as_ptr();
            skeleton_desc.atlas = spine_atlas;
            skeleton_desc.json_data = ptr as *const i8;
            skeleton_desc.prescale = 0.5;
            skeleton_desc.anim_default_mix = 0.2;

            let spine_skeleton = sspine_make_skeleton(&skeleton_desc);

            let mut spine_instance_desc: sspine_instance_desc = core::mem::MaybeUninit::zeroed().assume_init();
            spine_instance_desc.skeleton = spine_skeleton;

            // create a spine instance object, that's the thing that's actually rendered
            let instance = sspine_make_instance(&spine_instance_desc);

            // Since the spine instance doesn't move, its position can be set once,
            // the coordinate units depends on the sspine_layer_transform struct
            // that's passed to the sspine_draw_layer() during rendering (in our
            // case it's simply framebuffer pixels, with the origin in the
            // center)
            sspine_set_position(instance, sspine_vec2 { x: -100., y:200. });

            // configure a simple animation sequence
            let anim_c_string = std::ffi::CString::new("idle_down_weapon").unwrap();
            let anim_c_string = anim_c_string.as_ptr();
            sspine_add_animation(instance, sspine_anim_by_name(spine_skeleton, anim_c_string), 0, true, 0.);
            
            let atlas_images_num = sspine_num_images(spine_atlas);
            for img_index in 0..atlas_images_num {
                let img = sspine_image_by_index(spine_atlas, img_index);
                let img_info = sspine_get_image_info(img);
                let filename_c_str = core::ffi::CStr::from_ptr(img_info.filename.cstr.as_ptr());
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
                let file_path = format!("assets/{}", filename_c_str.to_str().unwrap());
                let file_path = file_path.as_str();
                let img_ptr = Box::new(img);
                let img_ptr = Box::into_raw(img_ptr) as *mut c_void;
               
                fetch(file_path, img_ptr as *mut c_void, images_load_success, images_load_fail);
            }
        }
        // emscripten_fetch_close(result);
    }
}

// This is the image-data fetch callback. The loaded image data will be decoded
// via stb_image.h and a sokol-gfx image object will be created.
//
// What's interesting here is that we're using sokol-gfx's multi-step
// image setup. sokol-spine has already allocated an image handle
// for each atlas image in sspine_make_atlas() via sg_alloc_image().
//
// The fetch callback just needs to finish the image setup by
// calling sg_init_image(), or if loading has failed, put the
// image object into the 'failed' resource state.
//
pub extern "C" fn images_load_success(result: *mut emscripten_fetch_t) {
    println!("Images load success");
    // println!("Successfully loaded {}", CStr::from_ptr((*result).url).to_str().unwrap());
    unsafe {
        let img = *((*result).userData as *mut sspine_image);
        let img_info = sspine_get_image_info(img);
        let filename_c_str = core::ffi::CStr::from_ptr(img_info.filename.cstr.as_ptr());
        println!("Successfully loaded images {}", filename_c_str.to_str().unwrap());
        
        // // decode pixels via stb_image.h
        // const int desired_channels = 4;
        // int img_width, img_height, num_channels;
        // stbi_uc* pixels = stbi_load_from_memory(
        //     response->data.ptr,
        //     (int)response->data.size,
        //     &img_width,
        //     &img_height,
        //     &num_channels, desired_channels);
        // // sokol-spine has already allocated an image and sampler handle,
        // // just need to call sg_init_image() and sg_init_sampler() to complete setup
        // sg_init_image(img_info.sgimage, &(sg_image_desc){
        //     .width = img_width,
        //     .height = img_height,
        //     .pixel_format = SG_PIXELFORMAT_RGBA8,
        //     .label = img_info.filename.cstr,
        //     .data.subimage[0][0] = {
        //         .ptr = pixels,
        //         .size = (size_t)(img_width * img_height * 4)
        //     }
        // });
        // sg_init_sampler(img_info.sgsampler, &(sg_sampler_desc){
        //     .min_filter = img_info.min_filter,
        //     .mag_filter = img_info.mag_filter,
        //     .mipmap_filter = img_info.mipmap_filter,
        //     .wrap_u = img_info.wrap_u,
        //     .wrap_v = img_info.wrap_v,
        //     .label = img_info.filename.cstr,
        // });
        // stbi_image_free(pixels);

        // Conver th above to Rust
        let desired_channels = 4;
        let mut img_width: i32 = 0;
        let mut img_height: i32 = 0;
        let mut num_channels: i32 = 0;
        let pixels = stbi_load_from_memory(
            (*result).data as *const u8,
            (*result).totalBytes as i32,
            &mut img_width,
            &mut img_height,
            &mut num_channels,
            desired_channels
        );
        // sokol-spine has already allocated an image and sampler handle,
        // just need to call sg_init_image() and sg_init_sampler() to complete setup
        let mut image_desc = sg_image_desc {
            _start_canary: 0,
            type_: sg_image_type_SG_IMAGETYPE_2D,
            render_target: false,
            width: img_width as i32,
            height: img_height as i32,
            num_slices: 1,
            num_mipmaps: 1,
            usage: sg_usage_SG_USAGE_IMMUTABLE,
            pixel_format: sg_pixel_format_SG_PIXELFORMAT_RGBA8,
            sample_count: 1,
            data: sg_image_data {
                subimage: [[sg_range { ptr: pixels as *const c_void, size: (img_width * img_height * 4) as usize }; 16]; 6],
            },
            label: std::ptr::null(),
            gl_textures: [0; 2usize],
            gl_texture_target: 0,
            mtl_textures: [std::ptr::null(); 2usize],
            d3d11_texture: std::ptr::null(),
            d3d11_shader_resource_view: std::ptr::null(),
            wgpu_texture: std::ptr::null(),
            wgpu_texture_view: std::ptr::null(),
            _end_canary: 0,
        };
        sg_init_image(img_info.sgimage, &mut image_desc);
        let mut sampler_desc: sg_sampler_desc = MaybeUninit::zeroed().assume_init();
        sampler_desc.min_filter = img_info.min_filter;
        sampler_desc.mag_filter = img_info.mag_filter;
        sampler_desc.mipmap_filter = img_info.mipmap_filter;
        sampler_desc.wrap_u = img_info.wrap_u;
        sampler_desc.wrap_v = img_info.wrap_v;
        sampler_desc.label = &img_info.filename.cstr as *const _ as *const i8;
        sg_init_sampler(img_info.sgsampler, &mut sampler_desc);
        stbi_image_free(pixels as *mut c_void);
    }
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
        let entity_raw = entity_box as *mut _ as *mut c_void;

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
    // use *;
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
