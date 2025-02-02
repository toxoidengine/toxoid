mod render;
mod fetch;
use toxoid_api::*;

// Trampoline closure from Rust using C callback and binding_ctx field to call a Rust closure
#[no_mangle]
pub unsafe extern "C" fn query_trampoline(iter: *mut toxoid_host::ecs_iter_t) {
    let handle = (*iter).callback_ctx as u64;
    let is_guest = (*iter).ctx != std::ptr::null_mut();
    if is_guest {
        // If target is not emscripten
        #[cfg(not(target_os = "emscripten"))]
        {
            // Get a lock on the store and get a mutable reference to the actual Store
            let mut store_guard = unsafe { toxoid_runtime::STORE.lock().unwrap() };
            let store = &mut *store_guard;

            let iter = Box::into_raw(Box::new(toxoid_host::Iter::new(iter as u64)));
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

#[components(Color, Size, Position)]
pub fn test_system(iter: &Iter) {
    for (color, size, pos) in components {
        println!("Color: {:?}, Size: {:?}, Position: {:?}", color.get_a(), size.get_width(), pos.get_y());  
        // println!("Color: {:?}, Size: {:?}, Position: {:?}", color, size, pos);
    }
}

pub fn init() {    
    unsafe {
        toxoid_host::QUERY_TRAMPOLINE = Some(query_trampoline);
    }
    render::init();
    fetch::init();

    System::dsl("Color, Size, Position", None, test_system).build();


    // System::dsl("", None, |_iter| {
    //     Query::dsl_each("Position", |query| {
    //         while query.next() {
    //             let positions = query.components::<Position>(0);
    //             positions.iter().for_each(|position| {
    //                 println!("Position X: {:?}, Y: {:?}", position.get_x(), position.get_y());
    //             });
    //         }
    //     });
    // })
    //     .build();
}