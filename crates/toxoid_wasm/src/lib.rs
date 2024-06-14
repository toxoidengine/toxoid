use toxoid_api::{c_void, c_char};

#[cfg(feature = "wasmtime")]
use wasmtime::*;
#[cfg(feature = "wasmi")]
use wasmi::*;

// Macro for creating the engine with wasmtime
#[cfg(feature = "wasmtime")]
macro_rules! create_engine {
    () => {
        wasmtime::Engine::default()
    };
}

// Macro for creating the engine with wasmi
#[cfg(feature = "wasmi")]
macro_rules! create_engine {
    () => {
        wasmi::Engine::default()
    };
}

// Macro for compiling modules with wasmtime
#[cfg(feature = "wasmtime")]
macro_rules! compile_module {
    ($engine:expr, $wasm:expr) => {
        wasmtime::Module::new(&$engine, &$wasm)
        .expect("Failed to create module")
    };
}

// Macro for compiling modules with wasmi
#[cfg(feature = "wasmi")]
macro_rules! compile_module {
    ($engine:expr, $wasm:expr) => {{
        wasmi::Module::new(&$engine, &$wasm[..])
        .expect("Failed to create module")
    }};
}

// Macro for creating a store with wasmtime
#[cfg(feature = "wasmtime")]
macro_rules! create_store {
    ($engine:expr, $host_state:expr) => {
        wasmtime::Store::new(&$engine, $host_state)
    };
}

// Macro for creating a store with wasmi
#[cfg(feature = "wasmi")]
macro_rules! create_store {
    ($engine:expr, $host_state:expr) => {{
        wasmi::Store::new(&$engine, $host_state)
    }};
}


#[cfg(feature = "wasmtime")]
macro_rules! create_linker {
    ($engine:expr, $host_state_type:ty) => {{
        wasmtime::Linker::new(&$engine)
    }};
}

#[cfg(feature = "wasmi")]
macro_rules! create_linker {
    ($engine:expr, $host_state_type:ty) => {{
        wasmi::Linker::<$host_state_type>::new(&$engine)
    }};
}

#[cfg(feature = "wasmtime")]
macro_rules! link_function {
    ($linker:expr, $store:expr, $name:expr, $func:expr) => {{
        linker
        .func_wrap("env", $name, $func)
        .expect("Failed to wrap function");
    }};
}

#[cfg(feature = "wasmi")]
macro_rules! link_function {
    ($linker:expr, $store:expr, $name:expr, $func:expr) => {{
        $linker
        .define("env", $name, wasmi::Func::wrap(&mut $store, $func))
        .expect("Failed to define function");
    }};
}

#[cfg(feature = "wasmtime")]
macro_rules! instantiate_module {
    ($linker:expr, $store:expr, $module:expr) => {
        $linker
        .instantiate(&mut $store, &$module)
        .expect("Failed to instantiate module")
    };
}

#[cfg(feature = "wasmi")]
macro_rules! instantiate_module {
    ($linker:expr, $store:expr, $module:expr) => {
        $linker
        .instantiate(&mut $store, &$module)
        .expect("Failed to instantiate module")
        .start(&mut $store)
        .expect("Failed to start instance")
    };
}

#[cfg(feature = "wasmtime")]
macro_rules! call_function {
    ($instance:expr, $store:expr, $func_name:expr) => {
        $instance
        .get_typed_func::<(), ()>(&mut $store, $func_name)
        .expect("Failed to get function")
        .call(&mut $store, ())
        .expect("Failed to call function");
    };
}

// Macro for instantiation and function calling with wasmi
#[cfg(feature = "wasmi")]
macro_rules! call_function {
    ($instance:expr, $store:expr, $func_name:expr) => {
        $instance
        .get_typed_func::<(), ()>(&$store, $func_name)
        .expect("Failed to get function")
        .call(&mut $store, ())
        .expect("Failed to call function");
    };
}

#[cfg(any(feature = "wasmi", feature = "wasmtime"))]
fn get_wasm_string(mut caller: Caller<'_, u32>, v: i32, v_len: i32) -> String {
    let memory = caller
    .get_export("memory")
    .and_then(|extern_| extern_.into_memory())
    .ok_or_else(|| panic!("failed to find host memory"))
    .expect("failed to find host memory");
    let mut buffer = vec![0; v_len as usize];
    memory
    .read(&mut caller, v as usize, &mut buffer)
    .expect("failed to read memory");
    String::from_utf8_lossy(&buffer)
    .to_string()
}

#[cfg(any(feature = "wasmi", feature = "wasmtime"))]
pub fn wasm_init() {
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))] {
        let mut path = std::env::current_exe()
        .expect("Failed to get current executable path");
        path.pop(); // Remove the executable name
        path.push("low.wasm");
        
        let wasm = std::fs::read(path)
        .expect("Failed to read WASM file");
        // Setup WASM runtime
        let engine = create_engine!();
        let module = compile_module!(engine, wasm);
        let mut store = create_store!(engine, 0);  
        let mut linker = create_linker!(engine, u32);
        
        #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
        // Bind functions
        unsafe {
            link_function!(linker, store, "toxoid_print_i32", |_caller: Caller<'_, u32>, v: i32| {
                toxoid_api::toxoid_print_i32(v)
            });
            link_function!(linker, store, "toxoid_print_u64", |_caller: Caller<'_, u32>, v: u64 | {
                toxoid_api::toxoid_print_u64(v);
            });
            link_function!(linker, store, "toxoid_print_f32", |_caller: Caller<'_, u32>, v: f32| {
                toxoid_api::toxoid_print_f32(v)
            });
            link_function!(linker, store, "toxoid_print_f64", |_caller: Caller<'_, u32>, v: f64| {
                toxoid_api::toxoid_print_f64(v)
            });
            link_function!(linker, store, "toxoid_print_string", |caller: Caller<'_, u32>, v: i32, v_len: i32| {
                let wasm_string = get_wasm_string(caller, v, v_len);
                toxoid_api::toxoid_print_string(wasm_string.as_ptr() as *const i8, wasm_string.len() as usize);
            });
            link_function!(linker, store, "toxoid_register_tag", |_caller: Caller<'_, u32>, name: i32, name_len: u32| -> u64 {
                toxoid_api::toxoid_register_tag(name as *const i8, name_len as usize)
            });
            link_function!(linker, store, "toxoid_register_component_ecs", |_caller: Caller<'_, u32>, name: i32, member_names: i32, member_types: i32| -> i32 {
                // toxoid_api::toxoid_register_component_ecs(
                //     name as *const i8 as *const str, 
                //     member_names as *const [&str], 
                //     member_types as *const [u8]
                // ) as i32
                0
            });
            link_function!(linker, store, "toxoid_entity_create", |_caller: Caller<'_, u32>| {
                toxoid_api::toxoid_entity_create()
            });
            link_function!(linker, store, "toxoid_entity_set_name", |caller: Caller<'_, u32>, entity_id: u64, name: i32, name_len: i32| {
                let wasm_string = get_wasm_string(caller, name, name_len);
                toxoid_api::toxoid_entity_set_name(entity_id, wasm_string.as_ptr() as *const i8);
            });
            link_function!(linker, store, "toxoid_entity_add_component", |_caller: Caller<'_, u32>, entity: u64, component: u64| {
                toxoid_api::toxoid_entity_add_component(entity, component);
            });
            link_function!(linker, store, "toxoid_entity_add_tag", |_caller: Caller<'_, u32>, entity: u64, tag: u64| {
                toxoid_api::toxoid_entity_add_tag(entity, tag);
            });
            link_function!(linker, store, "toxoid_entity_get_component", |_caller: Caller<'_, u32>, entity: u64, component: u64| -> i32 {
                toxoid_api::toxoid_entity_get_component(entity, component) as i32
            });
            link_function!(linker, store, "toxoid_entity_child_of", |_caller: Caller<'_, u32>, entity: u64, parent: u64| {
                toxoid_api::toxoid_entity_child_of(entity, parent);
            });
            link_function!(linker, store, "toxoid_entity_children", |_caller: Caller<'_, u32>, parent: u64| -> i32 {
                toxoid_api::toxoid_entity_children(parent) as i32
            });
            link_function!(linker, store, "toxoid_child_entities", |_caller: Caller<'_, u32>, iter: i32| -> i32 {
                toxoid_api::toxoid_child_entities(iter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_term_next", |_caller: Caller<'_, u32>, iter: i32| -> i32 {
                toxoid_api::toxoid_term_next(iter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_query_create", |_caller: Caller<'_, u32>| -> i32 {
                toxoid_api::toxoid_query_create() as i32
            });
            link_function!(linker, store, "toxoid_query_with", |_caller: Caller<'_, u32>, query_desc: i32, filter_index: i32, ids: i32, components_count: i32| -> i32 {
                toxoid_api::toxoid_query_with(query_desc as *mut c_void, filter_index as u8, ids as *mut u64, components_count) as i32
            });
            link_function!(linker, store, "toxoid_query_without", |_caller: Caller<'_, u32>, query_desc: i32, filter_index: i32, ids: i32, components_count: i32| -> i32 {
                toxoid_api::toxoid_query_without(query_desc as *mut c_void, filter_index as u8, ids as *mut u64, components_count) as i32
            });
            link_function!(linker, store, "toxoid_query_with_or", |_caller: Caller<'_, u32>, query_desc: i32, filter_index: i32, ids: i32, components_count: i32| -> i32 {
                toxoid_api::toxoid_query_with_or(query_desc as *mut c_void, filter_index as u8, ids as *mut u64, components_count) as i32
            });
            link_function!(linker, store, "toxoid_query_order_by", |_caller: Caller<'_, u32>, query_desc: i32, component_id: u64, callback: i32| {
                // toxoid_api::toxoid_query_order_by(query_desc as *mut c_void, component_id, callback as *mut c_void)
            });
            link_function!(linker, store, "toxoid_query_build", |_caller: Caller<'_, u32>, query_desc: i32| -> i32 {
                toxoid_api::toxoid_query_build(query_desc as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_query_iter", |_caller: Caller<'_, u32>, query: i32| -> i32 {
                toxoid_api::toxoid_query_iter(query as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_query_next", |_caller: Caller<'_, u32>, iter: i32| -> i32 {
                toxoid_api::toxoid_query_next(iter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_query_count", |_caller: Caller<'_, u32>, iter: i32| -> i32 {
                toxoid_api::toxoid_query_count(iter as *mut c_void)
            });
            link_function!(linker, store, "toxoid_query_field", |_caller: Caller<'_, u32>, iter: i32, term_index: i32, count: u32, index: u32| -> i32 {
                toxoid_api::toxoid_query_field(iter as *mut c_void, term_index, count, index) as i32
            });
            link_function!(linker, store, "toxoid_query_entity_list", |_caller: Caller<'_, u32>, iter: i32| -> i32 {
                toxoid_api::toxoid_query_entity_list(iter as *mut c_void).as_ptr() as i32
            });
            link_function!(linker, store, "toxoid_query_field_size", |_caller: Caller<'_, u32>, iter: i32, term_index: i32| -> i32 {
                toxoid_api::toxoid_query_field_size(iter as *mut c_void, term_index) as i32
            });
            link_function!(linker, store, "toxoid_query_field_list", |caller: Caller<'_, u32>, iter: i32, term_index: i32, count: u32| -> i32 {
                // toxoid_api::toxoid_query_field_list(iter as *mut c_void, term_index, count) as *mut _ as i32
                unimplemented!()
            });
            link_function!(linker, store, "toxoid_filter_create", |caller: Caller<'_, u32>| -> i32 {
                toxoid_api::toxoid_filter_create() as i32
            });
            link_function!(linker, store, "toxoid_filter_with", |caller: Caller<'_, u32>, filter: i32, filter_index: i32, ids: i32, components_count: i32| -> i32 {
                toxoid_api::toxoid_filter_with(filter as *mut c_void, filter_index as u8, ids as *mut u64, components_count) as i32
            });
            link_function!(linker, store, "toxoid_filter_without", |caller: Caller<'_, u32>, filter: i32, filter_index: i32, ids: i32, components_count: i32| -> i32 {
                toxoid_api::toxoid_filter_without(filter as *mut c_void, filter_index as u8, ids as *mut u64, components_count) as i32
            });
            link_function!(linker, store, "toxoid_filter_with_or", |caller: Caller<'_, u32>, filter: i32, filter_index: i32, ids: i32, components_count: i32| -> i32 {
                toxoid_api::toxoid_filter_with_or(filter as *mut c_void, filter_index as u8, ids as *mut u64, components_count) as i32
            });
            link_function!(linker, store, "toxoid_filter_build", |caller: Caller<'_, u32>, filter: i32| -> i32 {
                toxoid_api::toxoid_filter_build(filter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_filter_term_count", |caller: Caller<'_, u32>, filter: i32| -> i32 {
                toxoid_api::toxoid_filter_term_count(filter as *mut c_void)
            });
            link_function!(linker, store, "toxoid_filter_term_size", |caller: Caller<'_, u32>, filter: i32, term_index: i32| -> i32 {
                toxoid_api::toxoid_filter_term_size(filter as *mut c_void, term_index) as i32
            });
            link_function!(linker, store, "toxoid_filter_term_list", |caller: Caller<'_, u32>, filter: i32, term_index: i32, count: u32| -> i32 {
                toxoid_api::toxoid_filter_term_list(filter as *mut c_void, term_index, count).as_ptr() as i32
            });
            link_function!(linker, store, "toxoid_filter_term_iter", |caller: Caller<'_, u32>, filter: i32, term_index: i32| -> i32 {
                toxoid_api::toxoid_filter_term_iter(filter as *mut c_void, term_index) as i32
            });
            link_function!(linker, store, "toxoid_filter_term_next", |caller: Caller<'_, u32>, iter: i32| -> i32 {
                toxoid_api::toxoid_filter_term_next(iter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_filter_term_entity", |caller: Caller<'_, u32>, iter: i32, count: u32, index: u32| -> u64 {
                toxoid_api::toxoid_filter_term_entity(iter as *mut c_void, count, index) as u64
            });
            link_function!(linker, store, "toxoid_filter_term_entity_list", |caller: Caller<'_, u32>, iter: i32| -> i32 {
                toxoid_api::toxoid_filter_term_entity_list(iter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_iter_count", |caller: Caller<'_, u32>, iter: i32| -> i32 {
                toxoid_api::toxoid_iter_count(iter as *mut c_void)
            });
            link_function!(linker, store, "toxoid_component_cache_insert", |caller: Caller<'_, u32>, type_id: u64, component_id: u64| {
                toxoid_api::toxoid_component_cache_insert(type_id, component_id) 
            });
            link_function!(linker, store, "toxoid_component_cache_get", |caller: Caller<'_, u32>, type_id: u64| -> u64 {
                toxoid_api::toxoid_component_cache_get(type_id)
            });
            link_function!(linker, store, "toxoid_network_entity_cache_insert", |caller: Caller<'_, u32>, network_id: u64, entity_id: u64| {
                toxoid_api::toxoid_network_entity_cache_insert(network_id, entity_id);
            });
            link_function!(linker, store, "toxoid_network_entity_cache_get", |caller: Caller<'_, u32>, network_id: u64| -> u64 {
                toxoid_api::toxoid_network_entity_cache_get(network_id)
            });
            link_function!(linker, store, "toxoid_network_entity_cache_remove", |caller: Caller<'_, u32>, network_id: u64| {
                toxoid_api::toxoid_network_entity_cache_remove(network_id);
            });
            link_function!(linker, store, "toxoid_component_get_member_u8", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32| -> u32 {
                toxoid_api::toxoid_component_get_member_u8(component_ptr as *mut c_void, offset) as u32
            });
            link_function!(linker, store, "toxoid_component_get_member_u16", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32| -> u32 {
                toxoid_api::toxoid_component_get_member_u16(component_ptr as *mut c_void, offset) as u32
            });
            link_function!(linker, store, "toxoid_component_get_member_u32", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32| -> u32 {
                toxoid_api::toxoid_component_get_member_u32(component_ptr as *mut c_void, offset)
            });
            link_function!(linker, store, "toxoid_component_get_member_u64", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32| -> u64 {
                toxoid_api::toxoid_component_get_member_u64(component_ptr as *mut c_void, offset).into()
            });
            link_function!(linker, store, "toxoid_component_get_member_i8", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_i8(component_ptr as *mut c_void, offset as u8) as i32
            });
            link_function!(linker, store, "toxoid_component_get_member_i16", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_i16(component_ptr as *mut c_void, offset as u8) as i32
            });
            link_function!(linker, store, "toxoid_component_get_member_i32", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_i32(component_ptr as *mut c_void, offset)
            });
            link_function!(linker, store, "toxoid_component_get_member_i64", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32| -> i64 {
                toxoid_api::toxoid_component_get_member_i64(component_ptr as *mut c_void, offset)
            });
            link_function!(linker, store, "toxoid_component_get_member_f32", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32| -> f32 {
                toxoid_api::toxoid_component_get_member_f32(component_ptr as *mut c_void, offset)
            });
            link_function!(linker, store, "toxoid_component_get_member_f64", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32| -> f64 {
                toxoid_api::toxoid_component_get_member_f64(component_ptr as *mut c_void, offset)
            });
            link_function!(linker, store, "toxoid_component_get_member_bool", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_bool(component_ptr as *mut c_void, offset) as i32
            });
            link_function!(linker, store, "toxoid_component_get_member_u32array", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_u32array(component_ptr as *mut c_void, offset) as i32
            });
            link_function!(linker, store, "toxoid_component_get_member_f32array", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_f32array(component_ptr as *mut c_void, offset) as i32
            });
            link_function!(linker, store, "toxoid_component_get_member_string", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, len: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_string(component_ptr as *mut c_void, offset, len) as i32
            });
            link_function!(linker, store, "toxoid_component_set_member_u8", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, value: u32| {
                toxoid_api::toxoid_component_set_member_u8(component_ptr as *mut c_void, offset, value as u8)
            });
            link_function!(linker, store, "toxoid_component_set_member_u16", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, value: u32| {
                toxoid_api::toxoid_component_set_member_u16(component_ptr as *mut c_void, offset, value as u16);
            });
            link_function!(linker, store, "toxoid_component_set_member_u32", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, value: u32| {
                toxoid_api::toxoid_component_set_member_u32(component_ptr as *mut c_void, offset, value);
            });
            link_function!(linker, store, "toxoid_component_set_member_u64", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, value: u64| {
                toxoid_api::toxoid_component_set_member_u64(component_ptr as *mut c_void, offset, toxoid_api::split_u64(value));
            });
            link_function!(linker, store, "toxoid_component_set_member_i8", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, value: i32| {
                toxoid_api::toxoid_component_set_member_i8(component_ptr as *mut c_void, offset, value as i8);
            });
            link_function!(linker, store, "toxoid_component_set_member_i16", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, value: u32| {
                toxoid_api::toxoid_component_set_member_i16(component_ptr as *mut c_void, offset, value as i16);
            });
            link_function!(linker, store, "toxoid_component_set_member_i32", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, value: i32| {
                toxoid_api::toxoid_component_set_member_i32(component_ptr as *mut c_void, offset, value);
            });
            link_function!(linker, store, "toxoid_component_set_member_i64", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, value: i64| {
                toxoid_api::toxoid_component_set_member_i64(component_ptr as *mut c_void, offset, value);
            });
            link_function!(linker, store, "toxoid_component_set_member_f32", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, value: f32| {
                toxoid_api::toxoid_component_set_member_f32(component_ptr as *mut c_void, offset, value);
            });
            link_function!(linker, store, "toxoid_component_set_member_f64", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, value: f64| {
                toxoid_api::toxoid_component_set_member_f64(component_ptr as *mut c_void, offset, value);
            });
            link_function!(linker, store, "toxoid_component_set_member_bool", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, value: i32| {
                if value == 0 {
                    toxoid_api::toxoid_component_set_member_bool(component_ptr as *mut c_void, offset, false);
                } else {
                    toxoid_api::toxoid_component_set_member_bool(component_ptr as *mut c_void, offset, true);
                }
            });
            link_function!(linker, store, "toxoid_component_set_member_u32array", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, value: i32| {
                toxoid_api::toxoid_component_set_member_u32array(component_ptr as *mut c_void, offset, value as *mut u32);
            });
            link_function!(linker, store, "toxoid_component_set_member_f32array", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, value: i32| {
                toxoid_api::toxoid_component_set_member_f32array(component_ptr as *mut c_void, offset, value as *mut f32);
            });
            link_function!(linker, store, "toxoid_component_set_member_string", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, len: u32, value: i32| {
                toxoid_api::toxoid_component_set_member_string(component_ptr as *mut c_void, offset, len, value as *mut c_char);
            });
            link_function!(linker, store, "toxoid_component_set_member_ptr", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32, value: i32| {
                toxoid_api::toxoid_component_set_member_ptr(component_ptr as *mut c_void, offset, value as *mut c_void);
            });
            link_function!(linker, store, "toxoid_component_get_member_ptr", |_caller: Caller<'_, u32>, component_ptr: i32, offset: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_ptr(component_ptr as *mut c_void, offset) as i32
            });
            link_function!(linker, store, "toxoid_progress", |_caller: Caller<'_, u32>, delta_time: f32| -> i32 {
                toxoid_api::toxoid_progress(delta_time) as i32
            });
            link_function!(linker, store, "toxoid_filter_children_init", |_caller: Caller<'_, u32>, parent: u64| -> i32 {
                toxoid_api::toxoid_filter_children_init(parent) as i32
            });
            link_function!(linker, store, "toxoid_filter_iter", |_caller: Caller<'_, u32>, filter: i32| -> i32 {
                toxoid_api::toxoid_filter_iter(filter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_filter_next", |_caller: Caller<'_, u32>, iter: i32| -> i32 {
                toxoid_api::toxoid_filter_next(iter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_iter_entities", |_caller: Caller<'_, u32>, iter: i32| -> i32 {
                // toxoid_api::toxoid_iter_entities(iter as *mut c_void) as *mut _ as i32
                unimplemented!()
            });
            link_function!(linker, store, "toxoid_delete_entity", |_caller: Caller<'_, u32>, entity: u64| {
                toxoid_api::toxoid_delete_entity(entity);
            });
            link_function!(linker, store, "toxoid_entity_remove_component", |_caller: Caller<'_, u32>, entity: u64, component: u64| {
                toxoid_api::toxoid_entity_remove_component(entity, component);
            });
            link_function!(linker, store, "toxoid_is_valid", |_caller: Caller<'_, u32>, entity: u64| -> i32 {
                toxoid_api::toxoid_is_valid(entity) as i32
            });
            link_function!(linker, store, "toxoid_entity_has_component", |_caller: Caller<'_, u32>, entity: u64, component: u64| -> i32 {
                toxoid_api::toxoid_entity_has_component(entity, component) as i32
            });
            link_function!(linker, store, "toxoid_singleton_get", |_caller: Caller<'_, u32>, component: u64| -> i32 {
                toxoid_api::toxoid_singleton_get(component) as i32
            });
            link_function!(linker, store, "toxoid_singleton_add", |_caller: Caller<'_, u32>, component: u64| {
                toxoid_api::toxoid_singleton_add(component);
            });
            link_function!(linker, store, "toxoid_singleton_remove", |_caller: Caller<'_, u32>, component: u64| {
                toxoid_api::toxoid_singleton_remove(component);
            });
            link_function!(linker, store, "toxoid_systems_init", |_caller: Caller<'_, u32>, system_name: i32, ids: i32, callback: i32| -> u64 {
                // toxoid_api::toxoid_systems_init(system_name as *const i8, ids as *const u64, callback as *mut c_void) as u64
                unimplemented!()
            });
            link_function!(linker, store, "toxoid_prefab_create", |_caller: Caller<'_, u32>| -> i32 {
                toxoid_api::toxoid_prefab_create() as i32
            });
            link_function!(linker, store, "toxoid_prefab_instance", |_caller: Caller<'_, u32>, prefab_high: u32, prefab_low: u32| -> i32 {
                // toxoid_api::toxoid_prefab_instance(prefab_high, prefab_low) as i32
                0
            });
            link_function!(linker, store, "toxoid_system_create", |_caller: Caller<'_, u32>, callback_closure: i32| -> i32 {
                // toxoid_api::toxoid_system_create(callback_closure as *mut c_void) as i32
                unimplemented!()
            });
            link_function!(linker, store, "toxoid_system_build", |_caller: Caller<'_, u32>, system_desc: i32| -> i32 {
                toxoid_api::toxoid_system_build(system_desc as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_query_from_system_desc", |_caller: Caller<'_, u32>, query_desc: i32| -> i32 {
                toxoid_api::toxoid_query_from_system_desc(query_desc as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_network_send", |_caller: Caller<'_, u32>, network_messages: i32| {
                toxoid_api::toxoid_network_send(network_messages as *mut c_void);
            });
            link_function!(linker, store, "toxoid_net_send_components", |_caller: Caller<'_, u32>, entity_id: u64, components: i32, event: i32| {
                // toxoid_api::toxoid_net_send_components(entity_id, components as *mut c_void, event as *const i8);
                unimplemented!()
            });
            link_function!(linker, store, "toxoid_component_lookup", |_caller: Caller<'_, u32>, name: i32| -> i32 {
                toxoid_api::toxoid_component_lookup(name as *mut i8) as i32
            });
            link_function!(linker, store, "toxoid_net_add_event", |_caller: Caller<'_, u32>, event_name: i32, callback: i32| {
                // toxoid_api::toxoid_net_add_event(event_name as *const i8, callback as *mut c_void);
                unimplemented!()
            });
            link_function!(linker, store, "toxoid_deserialize_entity_sync", |_caller: Caller<'_, u32>, entity_id: u64, components_serialized: i32| {
                // toxoid_api::toxoid_deserialize_entity_sync(entity_id, components_serialized as *mut c_void);
                unimplemented!()
            });
            link_function!(linker, store, "toxoid_make_c_string", |_caller: Caller<'_, u32>, string: i32| -> i32 {
                // toxoid_api::toxoid_make_c_string(string as *const i8) as i32
                unimplemented!()
            });
            link_function!(linker, store, "toxoid_get_timestamp", |_caller: Caller<'_, u32>| -> i32 {
                // toxoid_api::toxoid_get_timestamp() as i32
                unimplemented!()
            });
            link_function!(linker, store, "gen_uuid", |_caller: Caller<'_, u32>| -> i32 {
                toxoid_api::gen_uuid() as i32
            });
                
        }
        // Instantiate WASM module
        let instance = instantiate_module!(linker, store, module);
        call_function!(instance, store, "app_main");
    }
}