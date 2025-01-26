use toxoid_api::*;
use toxoid_render::Renderer2D;
use toxoid_sokol::{bindings::*, SokolRenderer2D}; 

enum DataType {
    Raw,
    Image,
    Spine,
    Worldmap,
    Cell,
    Audio,
    Font
}

#[no_mangle]
pub extern "C" fn fetch_callback(response: *const sfetch_response_t) {
    let response = unsafe { *response };
    // Get user data / entity 
    let entity_id = unsafe { *(response.user_data as *mut u64) };
    // println!("User data: {:?}", unsafe { *user_data });
    // println!("Data: {:?}", response.data.ptr);
    // println!("Data size: {:?}", response.data.size);
    // println!("Failed: {:?}", response.failed);
    let mut entity = Entity::from_id(entity_id);
    let mut fetch_request = entity.get::<FetchRequest>();
    let data = unsafe { std::slice::from_raw_parts(response.data.ptr as *const u8, response.data.size) };
    // Convert byte slice into Vec<u64>
    // TODO: Make this a Vec<u8> instead after we implement into toxoid_api_macro
    let data = data.to_vec();
    let data = data.into_iter().map(|x| x as u64).collect::<Vec<u64>>();
    fetch_request.set_data(data.clone());
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
    // 123 KB
    let file_size = 124_0000;
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

// Fetch Observers
pub fn init() {
    Observer::dsl("FetchRequest, Loading", vec![Event::OnAdd], |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let fetch_request = entity.get::<FetchRequest>();
            let path = fetch_request.get_path();
            println!("Fetching asset: {:?}", path);
            sokol_fetch(&path, entity);
        });
    })
        .build();

    Observer::dsl("FetchRequest, Loaded", vec![Event::OnAdd], |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let fetch_request = entity.get::<FetchRequest>();
            let data = fetch_request.get_data();
            let size = data.len() as usize;
            let data = data.into_iter().map(|x| x as u8).collect::<Vec<u8>>();
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
        });
    })
        .build();
}

