// TODO: Make this file more crossplatform generic and less dependent on Emscripten (must fix sokol-fetch)
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
    callback: Box<dyn FnMut(&mut Entity)>
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

#[cfg(feature = "fetch")]
pub unsafe extern "C" fn worldmap_load_callback(result: *const sfetch_response_t) {
    let data = unsafe { (*result).data.ptr as *const u8 };
    let size = unsafe { (*result).data.size };
    let data_string = unsafe { std::str::from_utf8(core::slice::from_raw_parts(data, size)).unwrap() };
    let world = toxoid_tiled::parse_world(data_string);
    let world_ptr = Box::into_raw(Box::new(world));
    // Get user data
    let user_data: Box<FetchUserData> = Box::from_raw((*result).user_data as *mut FetchUserData);

    // Grab entity from user data
    let mut entity: Box<Entity> = Box::from_raw(user_data.entity);

    // Add TiledWorldComponent to entity
    entity.add::<TiledWorldComponent>();
    let mut world_component = entity.get::<TiledWorldComponent>();
    world_component.set_world(Pointer { ptr: world_ptr as *mut c_void });

    // Make loadable
    entity.add::<Loadable>();
    entity.add::<Position>();
    
    // Get user data
    let mut user_data: Box<FetchUserData> = Box::from_raw((*result).user_data as *mut FetchUserData);
    (user_data.callback)(&mut *user_data.entity);
}

#[cfg(feature = "fetch")]
pub extern "C" fn load_worldmap(filename: &str, callback: impl FnMut(&mut Entity) + 'static) -> *mut Entity {
    let entity = Entity::new();
    let entity_boxed = Box::into_raw(Box::new(entity));
    let user_data = Box::into_raw(Box::new(FetchUserData {
        entity: entity_boxed,
        callback: Box::new(callback)
    })) as *mut c_void;
    let size = core::mem::size_of::<FetchUserData>();
    fetch(filename, worldmap_load_callback, user_data, size);
    entity_boxed
}

#[cfg(feature = "fetch")]
#[no_mangle]
pub extern "C" fn toxoid_engine_load_worldmap(filename: &str,  callback: extern "C" fn(*mut Entity)) -> *mut Entity {
    load_worldmap(filename, move |entity: &mut Entity| {
        callback(entity)
    })
}

#[cfg(feature = "fetch")]
pub unsafe extern "C" fn cell_load_callback(result: *const sfetch_response_t) {
    let data = unsafe { (*result).data.ptr as *const u8 };
    let size = unsafe { (*result).data.size };
    let data_string = unsafe { std::str::from_utf8(core::slice::from_raw_parts(data, size)).unwrap() };
    let cell = toxoid_tiled::parse_cell(data_string);
    let cell_ptr = Box::into_raw(Box::new(cell));
    // Get user data
    let user_data: Box<FetchUserData> = Box::from_raw((*result).user_data as *mut FetchUserData);

    // Grab entity from user data
    let mut entity: Box<Entity> = Box::from_raw(user_data.entity);

    // Add TiledWorldComponent to entity
    entity.add::<TiledCellComponent>();
    let mut cell_component = entity.get::<TiledCellComponent>();
    cell_component.set_cell(Pointer { ptr: cell_ptr as *mut c_void });

    // Add Loadable
    entity.add::<Loadable>();
    
    // Get user data
    let mut user_data: Box<FetchUserData> = Box::from_raw((*result).user_data as *mut FetchUserData);
    (user_data.callback)(&mut *user_data.entity);
}

#[cfg(feature = "fetch")]
pub fn load_cell(filename: &str, callback: impl FnMut(&mut Entity) + 'static) -> *mut Entity {
    let entity = Entity::new();
    let entity_boxed = Box::into_raw(Box::new(entity));
    let user_data = Box::into_raw(Box::new(FetchUserData {
        entity: entity_boxed,
        callback: Box::new(callback)
    })) as *mut c_void;
    let size = core::mem::size_of::<FetchUserData>();
    
    fetch(filename, cell_load_callback, user_data, size);
    entity_boxed
} 

#[cfg(all(feature = "fetch", feature = "render"))]
#[no_mangle]
pub extern "C" fn toxoid_engine_load_sprite(filename: &str,  callback: extern "C" fn(*mut Entity)) -> *mut Entity {
    load_sprite(filename, move |entity: &mut Entity| {
        callback(entity)
    })
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
        let mut user_data: Box<FetchUserData> = Box::from_raw((*result).user_data as *mut FetchUserData);

        // Grab entity from user data
        let entity: Box<Entity> = Box::from_raw(user_data.entity);

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

#[cfg(all(feature = "fetch", feature = "render"))]
pub fn load_sprite(filename: &str, callback: impl FnMut(&mut Entity) + 'static) -> *mut Entity {
    // println!("Loading image: {}", filename);
    // Create entity and pass it to fetch
    let mut entity = Entity::new();
    entity.add::<Sprite>();
    entity.add::<Position>();
    entity.add::<Size>();

    let entity_boxed = Box::into_raw(Box::new(entity));
    let user_data = Box::into_raw(Box::new(FetchUserData {
        entity: entity_boxed,
        callback: Box::new(callback)
    })) as *mut c_void;
    let size = core::mem::size_of::<FetchUserData>();
    fetch(filename, sprite_load_callback, user_data, size);
    entity_boxed
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
#[cfg(all(feature = "fetch", feature = "render"))]
pub unsafe extern "C" fn image_load_callback(result: *const sfetch_response_t) {
    // Check for errors
    if (*result).failed {
        eprintln!("Failed to load image: {}", CStr::from_ptr((*result).path).to_str().unwrap());
        return;
    } else {
        // println!("Successfully loaded {}", CStr::from_ptr((*result).path).to_str().unwrap());
    }

    // Get image data
    let img_info = *((*result).user_data as *mut sspine_image_info);
    let data = (*result).data.ptr as *const u8;
    let size = (*result).data.size as usize;

    // Initialize sokol-gfx image object
    SokolRenderer2D::init_image(img_info.sgimage, data, size);
    // Initialize sokol-gfx sampler object
    SokolRenderer2D::init_sampler(
        img_info.sgsampler,
        img_info.min_filter,
        img_info.mag_filter,
        img_info.mipmap_filter,
        img_info.wrap_u,
        img_info.wrap_v,
        &img_info.filename.cstr as *const _ as *const i8
    );
}

// this function is called when both the spine atlas and skeleton file has been loaded,
// first an atlas object is created from the loaded atlas data, and then a skeleton
// object (which requires an atlas object as dependency), then a spine instance object.
// Finally any images required by the atlas object are loaded
#[cfg(all(feature = "fetch", feature = "render", feature = "spine"))]
pub extern "C" fn animation_load_callback(result: *const sfetch_response_t) {
    unsafe {
        // Get user data
        let user_data: Box<FetchUserData> = Box::from_raw((*result).user_data as *mut FetchUserData);

        // Grab entity from user data
        let mut entity: Box<Entity> = Box::from_raw(user_data.entity);

        // Get animation data
        let data = (*result).data.ptr as *const u8;
        let size = (*result).data.size;
        
        // Get url
        let url = CStr::from_ptr((*result).path).to_str().unwrap();
        if url.contains("atlas") {
            // println!("Successfully loaded atlas {}", url);
            let mut atlas = (*entity).get::<Atlas>();
            atlas.set_loaded(true);
            atlas.set_data_size(size as i32);
            atlas.set_data(Pointer::new(data as *mut c_void));
        } else if url.contains("json") {
            // println!("Successfully loaded skeleton {}", url);
            let mut skeleton = (*entity).get::<Skeleton>();
            skeleton.set_loaded(true);
            skeleton.set_data_size(size as i32);
            skeleton.set_data(Pointer::new(data as *mut c_void));
        }
        
        if (*entity).get::<Atlas>().get_loaded() && (*entity).get::<Skeleton>().get_loaded() {
            // Create spine atlas object from loaded atlas data.
            let mut atlas_desc: sspine_atlas_desc = core::mem::MaybeUninit::zeroed().assume_init();
            let mut atlas = (*entity).get::<Atlas>();
            let data_ptr = atlas.get_data().ptr;
            atlas_desc.data = sspine_range {
                ptr: data_ptr,
                size: size as usize
            };
            let spine_atlas = sspine_make_atlas(&atlas_desc);
            atlas.set_atlas(Pointer::new(Box::into_raw(Box::new(spine_atlas)) as *mut c_void));

            // Next create a spine skeleton object, skeleton data files can be either
            // text (JSON) or binary (in our case, 'raptor-pro.skel' is a binary skeleton file).
            // In case of JSON data, make sure that the data is 0-terminated!
            let mut skeleton_desc: sspine_skeleton_desc = core::mem::MaybeUninit::zeroed().assume_init();
            let mut skeleton = (*entity).get::<Skeleton>();
            let data_ptr = skeleton.get_data().ptr;
            let size = skeleton.get_data_size() as usize;
            let data_ptr = core::slice::from_raw_parts(data_ptr, size + 1).as_ptr() as *const i8;
            let data_ptr = std::ffi::CString::from_raw(data_ptr as *mut i8);
            let data_ptr = data_ptr.as_ptr();
            skeleton_desc.atlas = spine_atlas;
            skeleton_desc.json_data = data_ptr as *const i8;
            skeleton_desc.prescale = 2.0;
            skeleton_desc.anim_default_mix = 0.2;

            let spine_skeleton = sspine_make_skeleton(&skeleton_desc);
            skeleton.set_skeleton(Pointer::new(Box::into_raw(Box::new(spine_skeleton)) as *mut c_void));

            let mut spine_instance_desc: sspine_instance_desc = core::mem::MaybeUninit::zeroed().assume_init();
            spine_instance_desc.skeleton = spine_skeleton;

            // create a spine instance object, that's the thing that's actually rendered
            let instance = sspine_make_instance(&spine_instance_desc);

            // Store Spine instance in singleton for now
            (*entity).add::<SpineInstance>();
            let mut instance_component = (*entity).get::<SpineInstance>();
            let instance_ptr = Box::new(instance);
            let instance_ptr = Box::into_raw(instance_ptr) as *mut c_void;
            instance_component.set_instance(Pointer::new(instance_ptr));
            instance_component.set_instantiated(true);
            
            // configure a simple animation sequence
            sspine_add_animation(instance, sspine_anim_by_name(spine_skeleton, make_c_string("idle_down")), 0, true, 0.);
            
            let atlas_images_num = sspine_num_images(spine_atlas);

            // load all atlas images
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
                let img_ptr = Box::new(img_info);
                let img_ptr = Box::into_raw(img_ptr) as *mut c_void;

                fetch(file_path, image_load_callback, img_ptr as *mut c_void, std::mem::size_of::<sspine_image_info>());
            }
        }
    }
}

#[cfg(all(feature = "fetch", feature = "render", feature = "spine"))]
pub fn load_animation(atlas_filename: &str, skeleton_filename: &str, callback: impl FnMut(&mut Entity) + 'static) -> *mut Entity {
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
        atlas.set_filename(StringPtr::new(atlas_filename));
        let mut skeleton = entity.get::<Skeleton>();
        skeleton.set_filename(StringPtr::new(skeleton_filename));

        let entity_boxed = Box::into_raw(Box::new(entity));
        let user_data = Box::into_raw(Box::new(FetchUserData {
            entity: entity_boxed,
            callback: Box::new(callback)
        })) as *mut c_void;
        let size = core::mem::size_of::<FetchUserData>();

        fetch(skeleton_filename, animation_load_callback, user_data, size);
        fetch(atlas_filename, animation_load_callback, user_data, size);

        entity_boxed
    }
}
