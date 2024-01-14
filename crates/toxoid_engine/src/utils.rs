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
pub fn load_image(filename: &str) -> &mut Entity {
    unsafe {
        println!("Loading image: {}", filename);
        
        // Create fetch attributes
        let mut attr: emscripten_fetch_attr_t = core::mem::zeroed();
        emscripten_fetch_attr_init(&mut attr);
        attr.attributes = EMSCRIPTEN_FETCH_LOAD_TO_MEMORY;
        // Callbacks
        attr.onsuccess = Some(download_succeeded);
        attr.onerror = Some(download_failed);
        
        // Convert filename to CString
        let filename_cstring = std::ffi::CString::new(filename).unwrap();

        // Create entity and pass it to fetch
        let mut entity = Entity::new();
        entity.add::<Loadable>();
        entity.add::<Sprite>();
        entity.add::<Position>();
        entity.add::<Size>();

        let entity_box = Box::new(entity);
        let entity_box = Box::leak(entity_box);
        let entity_raw = entity_box as *mut _ as *mut core::ffi::c_void;
        attr.userData = entity_raw;

        // Fetch file
        emscripten_fetch(&attr, filename_cstring.as_ptr());

        entity_box
    }
}


// const sfetch_response_t* respons
pub extern "C" fn atlas_data_loaded(result: *mut emscripten_fetch_t) {
     println!("Atlas data loaded.");
     /*sspine_make_atlas(&(sspine_atlas_desc){
        .data = state.load_status.atlas.data,
    }); */
    unsafe {
        let mut atlas: sspine_atlas_desc = core::mem::MaybeUninit::zeroed().assume_init();
        atlas.data = toxoid_sokol::bindings::sspine_range {
            ptr: (*result).data as *const core::ffi::c_void,
            size: (*result).totalBytes as usize
        };
        let spine_atlas = toxoid_sokol::bindings::sspine_make_atlas(&atlas);
        println!("Spine atlas: {:?}", spine_atlas);

        let atlas_num = toxoid_sokol::bindings::sspine_num_images(spine_atlas);
        println!("Atlas num: {}", atlas_num);
    }
 }

pub extern "C" fn atlas_data_failed(result: *mut emscripten_fetch_t) {}
 

#[cfg(target_os = "emscripten")]
pub fn load_atlas(filename: &str) -> &mut Entity {
    unsafe {
        println!("Loading image: {}", filename);
        
        // Create fetch attributes
        let mut attr: emscripten_fetch_attr_t = core::mem::zeroed();
        emscripten_fetch_attr_init(&mut attr);
        attr.attributes = EMSCRIPTEN_FETCH_LOAD_TO_MEMORY;

        // Callbacks
        attr.onsuccess = Some(atlas_data_loaded);
        attr.onerror = Some(atlas_data_failed);
        
        // Convert filename to CString
        let filename_cstring = std::ffi::CString::new(filename).unwrap();

        // Create entity and pass it to fetch
        let mut entity = Entity::new();
        entity.add::<Loadable>();
        entity.add::<Sprite>();
        entity.add::<Position>();
        entity.add::<Size>();

        let entity_box = Box::new(entity);
        let entity_box = Box::leak(entity_box);
        let entity_raw = entity_box as *mut _ as *mut core::ffi::c_void;
        attr.userData = entity_raw;

        // Fetch file
        emscripten_fetch(&attr, filename_cstring.as_ptr());

        entity_box
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

pub extern "C" fn download_succeeded(result: *mut emscripten_fetch_t) {
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
        create_image(data, size, entity_box);

        // Close fetch
        emscripten_fetch_close(result);
    }
}

pub extern "C" fn download_failed(fetch: *mut emscripten_fetch_t) {
    unsafe {
        println!("Fail!");
        // eprintln!("Failed to load image: {}", CStr::from_ptr((*fetch).url).to_str().unwrap());
        // emscripten_fetch_close(fetch);
    }
}