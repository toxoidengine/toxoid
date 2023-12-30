use rand::Rng;
use toxoid_ffi::emscripten::*;
use toxoid_api::*;

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
pub fn load_image(filename: &str) {
    unsafe {
        println!("Loading image: {}", filename);
        
        let mut attr: emscripten_fetch_attr_t = std::mem::zeroed();
        emscripten_fetch_attr_init(&mut attr);
        attr.attributes = EMSCRIPTEN_FETCH_LOAD_TO_MEMORY;
        attr.onsuccess = Some(download_succeeded);
        attr.onerror = Some(download_failed);
        
        let filename_cstring = std::ffi::CString::new(filename).unwrap();
        emscripten_fetch(&attr, filename_cstring.as_ptr());

        let mut entity = Entity::new();
        entity.add::<Sprite>();
        entity.add::<Loadable>();
        entity.add::<Position>();
        entity.add::<Size>();
        
        use core::ffi::CStr;
        let mut sprite = entity.get::<Sprite>();
        sprite.set_filename(Pointer::new(filename_cstring.as_ptr() as *mut core::ffi::c_void));
        let raw_ptr = sprite.get_filename().ptr;
        let c_str: &CStr = CStr::from_ptr(raw_ptr as *const i8);
        let str_slice: &str = c_str.to_str().unwrap();

        println!("{}", str_slice);
    }
}

// pub fn create_image() {
//     let mut width: i32 = 0;
//         let mut height: i32 = 0;
//         let mut channels: i32 = 0;
        
//         let image_data = unsafe {
//             // Converts from PNG format to RGBA8 format
//             stbi_load_from_memory(data as *const u8, data.len() as c_int, &mut width, &mut height, &mut channels, 0)
//         };

//         // Create sg_image from data
//         let mut image_desc = sg_image_desc {
//             _start_canary: 0,
//             type_: sg_image_type_SG_IMAGETYPE_2D,
//             render_target: false,
//             width: 100, // You need to provide the width
//             height: 100, // You need to provide the height
//             num_slices: 1,
//             num_mipmaps: 1,
//             usage: sg_usage_SG_USAGE_IMMUTABLE,
//             pixel_format: sg_pixel_format_SG_PIXELFORMAT_RGBA8,
//             sample_count: 1,
//             data: sg_image_data {
//                 subimage: [[sg_range { ptr: image_data as *const c_void, size: (width * height * 4) as usize }; 16]; 6],
//             },
//             label: ptr::null(),
//             gl_textures: [0; 2usize],
//             gl_texture_target: 0,
//             mtl_textures: [ptr::null(); 2usize],
//             d3d11_texture: ptr::null(),
//             d3d11_shader_resource_view: ptr::null(),
//             wgpu_texture: ptr::null(),
//             wgpu_texture_view: ptr::null(),
//             _end_canary: 0,
//         };

//         let image = sg_make_image(&mut image_desc);
//         // emscripten_fetch_close(fetch);
//         image
// }

extern "C" fn download_succeeded(result: *mut emscripten_fetch_t) {
    unsafe {
        // println!("Successfully loaded {}", CStr::from_ptr((*result).url).to_str().unwrap());
        let data = (*result).data as *mut u8;
        let size = (*result).totalBytes as usize;

        // Create sg_image from data
        // let mut image_desc = sg_image_desc {
        //     _start_canary: 0,
        //     type_: sg_image_type_SG_IMAGETYPE_2D,
        //     render_target: false,
        //     width: 0, // You need to provide the width
        //     height: 0, // You need to provide the height
        //     num_slices: 1,
        //     num_mipmaps: 1,
        //     usage: sg_usage_SG_USAGE_IMMUTABLE,
        //     pixel_format: sg_pixel_format_SG_PIXELFORMAT_RGBA8,
        //     sample_count: 1,
        //     data: sg_image_data {
        //         subimage: [[sg_range { ptr: data as *const c_void, size }; 16]; 6],
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

        // let image = sg_make_image(&mut image_desc);
        // emscripten_fetch_close(fetch);
        // image
    }
}

extern "C" fn download_failed(fetch: *mut emscripten_fetch_t) {
    unsafe {
        println!("Fail!");
        // eprintln!("Failed to load image: {}", CStr::from_ptr((*fetch).url).to_str().unwrap());
        // emscripten_fetch_close(fetch);
    }
}