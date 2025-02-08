use toxoid_api::*;
use toxoid_render::Renderer2D;
use toxoid_sokol::{bindings::*, SokolRenderer2D}; 

#[no_mangle]
pub extern "C" fn fetch_callback(response: *const sfetch_response_t) {
    let response = unsafe { *response };
    // Get user data / entity 
    // println!("User data: {:?}", unsafe { *user_data });
    // println!("Data: {:?}", response.data.ptr);
    // println!("Data size: {:?}", response.data.size);
    // println!("Failed: {:?}", response.failed);
    let entity_id = unsafe { *(response.user_data as *mut u64) };
    let mut entity = Entity::from_id(entity_id);
    let mut fetch_request = entity.get::<FetchRequest>();
    let data = unsafe { std::slice::from_raw_parts(response.data.ptr as *const u8, response.data.size) };
    let data = data.to_vec();
    fetch_request.set_data(data);
    entity.remove::<Loading>();
    entity.add::<Loaded>();
}

fn sokol_fetch(path: &str, entity: &mut Entity) {
    // Create fetch description
    let mut sfetch_request: sfetch_request_t = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
    let path = std::ffi::CString::new(path).unwrap();
    sfetch_request.path = path.as_ptr();
    sfetch_request.channel = 0;
    // TODO: Figure out from server or filesystem what the size of the file is
    // 1 MB to be safe, or determine actual size
    let file_size = 1024 * 1024;
    // PNG buffer - 4.00 KB
    // let file_size = 4_096;
    let buffer = Box::into_raw(
        vec![0u8; file_size]
            .into_boxed_slice()
    );
    sfetch_request.buffer = sfetch_range_t {
        ptr: buffer as *const core::ffi::c_void,
        size: file_size
    };
    sfetch_request.callback = Some(fetch_callback);
    // Store entity in the user data / ctx of request so that
    // we can associate the entity with the fetch request / response
    let entity_id = Box::into_raw(Box::new(entity.get_id()));
    let ptr = entity_id as *mut core::ffi::c_void;
    let size = core::mem::size_of::<u64>();
    sfetch_request.user_data = sfetch_range_t {
        ptr,
        size
    };
    unsafe { sfetch_send(&sfetch_request) };
}

fn c_string(rust_str: &str) -> *const i8 {
    let c_string = std::ffi::CString::new(rust_str).expect("CString::new failed");
    let c_ptr = c_string.as_ptr();
    std::mem::forget(c_string); // Prevent CString from being deallocated
    c_ptr
}

// #[cfg(feature = "spine")]
pub fn bone_animation_loaded(entity: &mut Entity) {
    // Create spine atlas object from loaded atlas data.
    let mut atlas_desc: sspine_atlas_desc = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
    let mut atlas = entity.get::<Atlas>();
    let data = atlas.get_data();
    let size = data.len() as usize;
    
    // Convert Vec into Box to keep the data alive
    let data_box = data.into_boxed_slice();
    let data_ptr = Box::into_raw(data_box);
    
    atlas_desc.data = sspine_range {
        ptr: data_ptr as *const std::ffi::c_void,
        size
    };
    
    let spine_atlas = unsafe { sspine_make_atlas(&atlas_desc) };
    atlas.set_atlas(Box::into_raw(Box::new(spine_atlas)) as u64);

    // Next create a spine skeleton object, skeleton data files can be either
    // text (JSON) or binary (in our case, 'raptor-pro.skel' is a binary skeleton file).
    // In case of JSON data, make sure that the data is 0-terminated!
    let mut skeleton_desc: sspine_skeleton_desc = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
    let mut skeleton = entity.get::<Skeleton>();
    let data = skeleton.get_data();
    skeleton_desc.atlas = spine_atlas;
    
    // Convert Vec into Box to keep the data alive
    let data_box = data.into_boxed_slice();
    let data_ptr = Box::into_raw(data_box);
    
    skeleton_desc.json_data = data_ptr as *const i8;
    skeleton_desc.prescale = 5.0;
    skeleton_desc.anim_default_mix = 0.2;

    let spine_skeleton = unsafe { sspine_make_skeleton(&skeleton_desc) };
    skeleton.set_skeleton(Box::into_raw(Box::new(spine_skeleton)) as u64);

    let mut spine_instance_desc: sspine_instance_desc = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
    spine_instance_desc.skeleton = spine_skeleton;

    // create a spine instance object, that's the thing that's actually rendered
    let instance = unsafe { sspine_make_instance(&spine_instance_desc) };
    entity.add::<SpineInstance>();
    let mut instance_component = entity.get::<SpineInstance>();
    let mut ctx_desc: sspine_context_desc = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
    ctx_desc.color_format = 0;
    ctx_desc.depth_format = 0;
    ctx_desc.sample_count = 1;
    let ctx = unsafe { sspine_make_context(&ctx_desc) };
    instance_component.set_instance(Box::into_raw(Box::new(instance)) as u64);
    instance_component.set_instantiated(true);
    instance_component.set_ctx(Box::into_raw(Box::new(ctx)) as u64);

    // configure a simple animation sequence

    unsafe { sspine_add_animation(instance, sspine_anim_by_name(spine_skeleton, c_string("idle_down")), 0, true, 0.) };
    unsafe { sspine_set_animation(instance, sspine_anim_by_name(spine_skeleton, c_string("idle_down")), 0, true) };

    let atlas_images_num = unsafe { sspine_num_images(spine_atlas) };

    // load all atlas images
    for img_index in 0..atlas_images_num {
        let img = unsafe { sspine_image_by_index(spine_atlas, img_index) };
        let img_info = unsafe { sspine_get_image_info(img) };
        let filename_c_str = unsafe { core::ffi::CStr::from_ptr(img_info.filename.cstr.as_ptr()) };
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
        let file_path = format!("assets/animations/{}", filename_c_str.to_str().unwrap());
        let file_path = file_path.as_str();
        let mut image_entity = Entity::new(None);
        image_entity.child_of_id(entity.get_id());
        image_entity.add::<Image>();
        let mut image = image_entity.get::<Image>();
        image.set_info(Box::into_raw(Box::new(img_info)) as u64);
        fetch(file_path, DataType::Image, Some(image_entity.get_id()));
    }

    entity.add::<Blittable>();
}

// Fetch Observers
pub fn init() {
    Observer::dsl("FetchRequest, Loading", vec![Event::OnAdd], |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let fetch_request = entity.get::<FetchRequest>();
            let path = fetch_request.get_path();
            sokol_fetch(&path, entity);
        });
    })
        .build();

    Observer::dsl("FetchRequest, Loaded", vec![Event::OnAdd], |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let fetch_request = entity.get::<FetchRequest>();
            let data = fetch_request.get_data();
            let data_type = fetch_request.get_data_type();
            let size = data.len() as usize;
            match data_type as u8 {
                d if d == DataType::Image as u8 => {
                    // Get image data
                    let mut image_entity = Entity::from_id(fetch_request.get_user_data());
                    let image = image_entity.get::<Image>();
                    let img_info = image.get_info();
                    let img_info = unsafe { Box::from_raw(img_info as *mut sspine_image_info) };
                    // let img_info = unsafe { &*(img_info as *const sspine_image_info) };
                    // let filename = unsafe { core::ffi::CStr::from_ptr(img_info.filename.cstr.as_ptr()) };
                    let data_ptr = data.as_ptr() as *const u8;
                    // Initialize sokol-gfx image object
                    SokolRenderer2D::init_image(img_info.sgimage, data_ptr, size);
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
                },
                d if d == DataType::Sprite as u8 => {
                    let data = data.as_slice().as_ptr();
                    let sokol_sprite = SokolRenderer2D::create_sprite(data, size);
                    let mut entity = Entity::new(None);
                    entity.add::<Size>();
                    entity.add::<Sprite>();
                    let mut size = entity.get::<Size>();
                    size.set_width(sokol_sprite.width());
                    size.set_height(sokol_sprite.height());
                    let mut sprite = entity.get::<Sprite>();
                    sprite.set_sprite(Box::into_raw(sokol_sprite) as *mut () as u64);
                }
                d if d == DataType::BoneAnimationAtlas as u8 => {
                    let mut animation_entity = Entity::from_id(fetch_request.get_user_data());
                    let mut atlas = animation_entity.get::<Atlas>();
                    atlas.set_data(data);
                    atlas.set_loaded(true);
                    if animation_entity.get::<Skeleton>().get_loaded() {
                        bone_animation_loaded(&mut animation_entity);
                    }
                },
                d if d == DataType::BoneAnimationSkeleton as u8 => {
                    let mut animation_entity = Entity::from_id(fetch_request.get_user_data());
                    let mut skeleton = animation_entity.get::<Skeleton>();
                    skeleton.set_data(data);
                    skeleton.set_loaded(true);
                    if animation_entity.get::<Atlas>().get_loaded() {
                        bone_animation_loaded(&mut animation_entity);
                    }
                },
                _ => {
                    unimplemented!();
                }
            }

        });
    })
        .build();
}

