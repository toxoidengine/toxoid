use toxoid_api::*;
use toxoid_sokol::bindings::*;

#[no_mangle]
pub extern "C" fn fetch_callback(response: *const sfetch_response_t) {
    println!("Fetch callback called");
}

fn sokol_fetch(path: &str) {
    // Create fetch description
    let mut sfetch_request: sfetch_request_t = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
    let path = std::ffi::CString::new(path).unwrap();
    sfetch_request.path = path.as_ptr();
    sfetch_request.channel = 0;
    // TODO: Figure out from server what the size of the file is
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
    let user_data = std::ptr::null_mut();
    let user_data_size = 0;
    sfetch_request.user_data = sfetch_range_t {
        ptr: user_data,
        size: user_data_size
    };
    unsafe { sfetch_send(&sfetch_request) };
}

// Fetch Observers
pub fn init() {
    Observer::dsl("FetchRequest, Loading", vec![Event::OnSet], |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let fetch_request = entity.get::<FetchRequest>();
            let path = fetch_request.get_path();
            println!("Fetching asset: {:?}", path);
            sokol_fetch(&path);
        });
    })
        .build();
}

