use toxoid_sokol::SokolRenderer2D;
use toxoid_render::Renderer2D;
use toxoid_api::*;
use crate::entities;

// Trampoline closure from Rust using C callback and binding_ctx field to call a Rust closure
#[no_mangle]
pub unsafe extern "C" fn query_trampoline(iter: *mut toxoid_host::ecs_iter_t) {
    let handle = (*iter).callback_ctx as i64;
    let is_guest = (*iter).ctx != std::ptr::null_mut();
    if is_guest {
        // If target is not emscripten
        #[cfg(not(target_os = "emscripten"))]
        {
            // Get a lock on the store and get a mutable reference to the actual Store
            let mut store_guard = unsafe { toxoid_runtime::STORE.lock().unwrap() };
            let store = &mut *store_guard;

            let iter = Box::into_raw(Box::new(toxoid_host::Iter::new(iter as i64)));
            let iter_resource_id = store
                .data_mut()
                .table.push::<toxoid_runtime::IterProxy>(toxoid_runtime::IterProxy { ptr: iter })
                .unwrap();
            
            unsafe {
                let component_world_result = toxoid_runtime::TOXOID_COMPONENT_WORLD.as_ref();
                if let Some(component_world) = component_world_result {
                    component_world
                        .toxoid_component_component_callbacks()
                        .call_run(store, iter_resource_id, handle)
                        .unwrap_or_else(|e| {
                            println!("Error calling run: {:?}", e);
                        });
                }
            }
        }
    } else {
        let callback = unsafe {
            toxoid_api::CALLBACKS[handle as usize]
                .as_ref()
        };
        let iter = toxoid_api::Iter {
            iter: toxoid_api::ToxoidIter {
                ptr: iter as *mut core::ffi::c_void
            }
        };
        callback(&iter);
    }
}

pub fn init() {    
    unsafe {
        toxoid_host::QUERY_TRAMPOLINE = Some(query_trampoline);
    }
    
    // Rendering System
    System::dsl("Rect, Position, Size, Color, Renderable", None, |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let pos = entity.get::<Position>();
            let size = entity.get::<Size>();
            let color = entity.get::<Color>();
            SokolRenderer2D::draw_filled_rect(&pos, &size, &color);
        });
    })
        .build();

    Observer::dsl("FetchRequest, Loading", vec![Event::OnSet], |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let fetch_request = entity.get::<FetchRequest>();
            let path = fetch_request.get_path();
            println!("Fetching asset: {:?}", path);
        });
    })
        .build();

    toxoid_api::fetch("assets/test.png");
}