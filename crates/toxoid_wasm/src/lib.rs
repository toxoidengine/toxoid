use toxoid_api::{c_void, c_char};
use toxoid_api::serde::Serialize;

#[cfg(feature = "wasmtime")]
use wasmtime::*;
#[cfg(feature = "wasmi")]
use wasmi::*;
use std::sync::Mutex;

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
        let store = &mut *STORE.lock().unwrap();
        $linker
        .define("env", $name, wasmi::Func::wrap(store, $func))
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
    ($linker:expr, $module:expr) => {{
        let mut store = &mut *STORE.lock().unwrap();
        $linker
            .instantiate(&mut store, &$module)
            .expect("Failed to instantiate module")
            .start(&mut store)
            .expect("Failed to start instance")
    }};
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
    ($instance:expr, $func_name:expr) => {
        let mut store = &mut *STORE.lock().unwrap();
        $instance
        .get_typed_func::<(), ()>(&mut store, $func_name)
        .expect("Failed to get function")
        .call(&mut store, ())
        .expect("Failed to call function");
    };
}

#[cfg(any(feature = "wasmi", feature = "wasmtime"))]
fn get_wasm_string(v: i32, v_len: i32) -> String {
    unsafe {
        let memory = {
            let store = &mut *STORE.lock().unwrap();
            INSTANCE
                .lock()
                .unwrap()
                .unwrap()
                .get_export(store, "memory")
                .and_then(|extern_| extern_.into_memory())
                .expect("failed to find host memory")
        };   
        
        let store = &mut *STORE.lock().unwrap();
        let mut buffer = vec![0; v_len as usize];
        memory
            .read(store.as_context_mut(), v as usize, &mut buffer)
            .expect("failed to read memory");
        String::from_utf8(buffer).expect("failed to convert to UTF-8")
    }
}

#[cfg(any(feature = "wasmi", feature = "wasmtime"))]
fn get_wasm_func<Params, Results>(callback: i32) -> TypedFunc<Params, Results> 
where
    Params: WasmParams,
    Results: WasmResults, 
{
    let table = unsafe {
        let store = &mut *STORE.lock().unwrap();
        INSTANCE
            .lock()
            .unwrap()
            .unwrap()
            .get_export(store, "table")
            .and_then(|export| export.into_table())
            .expect("expected table export")
    };

    // Retrieve the callback function from the table using the index
    {
        let store = &mut *STORE.lock().unwrap();
        let func = table
            .get(store.as_context_mut(), callback as u32)
            .unwrap();
        let func = func
            .funcref()
            .unwrap()
            .func()
            .unwrap();
        // Convert to a proper function pointer
        func
            .typed::<Params, Results>(store.as_context())
            .expect("function type mismatch")
            .clone()
    }
}

// OnceCell global store
static ENGINE: once_cell::sync::Lazy<wasmi::Engine> = once_cell::sync::Lazy::new(|| {
    create_engine!()
});
static STORE: once_cell::sync::Lazy<Mutex<wasmi::Store<u32>>> = once_cell::sync::Lazy::new(|| {
    Mutex::new(create_store!(ENGINE, 0))
});
static mut INSTANCE: once_cell::sync::Lazy<Mutex<Option<wasmi::Instance>>> = once_cell::sync::Lazy::new(|| {
   Mutex::new(None)
});

#[cfg(any(feature = "wasmi", feature = "wasmtime"))]
pub fn wasm_init() {
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))] {
        let mut path = std::env::current_exe()
            .expect("Failed to get current executable path");
        path.pop(); // Remove the executable name
        path.push("toxoid_wasm_test.wasm");
        
        let wasm = std::fs::read(path)
            .expect("Failed to read WASM file");
        // Set up WASM module for parsing
        let mut wasm_parsed = walrus::ModuleConfig::new().parse(&wasm).unwrap();
        // Assuming the table is the first table in the module
        let table_id = wasm_parsed.tables.iter().next().unwrap().id();
        // Export the table
        wasm_parsed.exports.add("table", walrus::ExportItem::Table(table_id));
        let wasm = wasm_parsed.emit_wasm();

        // Setup WASM runtime
        let module = compile_module!(ENGINE, wasm);
        let mut linker = create_linker!(ENGINE, u32);
        
        #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
        // Bind functions
        unsafe {
            link_function!(linker, store, "toxoid_print_i32", |v: i32| {
                toxoid_api::toxoid_print_i32(v)
            });
            link_function!(linker, store, "toxoid_print_u64", |v: u64 | {
                toxoid_api::toxoid_print_u64(v);
            });
            link_function!(linker, store, "toxoid_print_f32", |v: f32| {
                toxoid_api::toxoid_print_f32(v)
            });
            link_function!(linker, store, "toxoid_print_f64", |v: f64| {
                toxoid_api::toxoid_print_f64(v)
            });
            link_function!(linker, store, "toxoid_print_string", |v: i32, v_len: i32| {
                std::thread::spawn(move || {
                    let wasm_string = get_wasm_string(v, v_len);
                    toxoid_api::toxoid_print_string(wasm_string.as_ptr() as *const i8, wasm_string.len() as usize);
                });
            });
            link_function!(linker, store, "toxoid_register_tag", |name: i32, name_len: u32| -> u64 {
                toxoid_api::toxoid_register_tag(name as *const i8, name_len as usize)
            });
            link_function!(linker, store, "toxoid_register_component_ecs", |name: i32, member_names: i32, member_types: i32| -> i32 {
                // toxoid_api::toxoid_register_component_ecs(
                //     name as *const i8 as *const str, 
                //     member_names as *const [&str], 
                //     member_types as *const [u8]
                // ) as i32
                0
            });
            link_function!(linker, store, "toxoid_entity_create", || {
                // toxoid_api::toxoid_entity_create()
            });
            link_function!(linker, store, "toxoid_entity_set_name", |entity_id: u64, name: i32, name_len: i32| {
                // let wasm_string = get_wasm_string(&caller, name, name_len);
                // toxoid_api::toxoid_entity_set_name(entity_id, wasm_string.as_str());
            });
            link_function!(linker, store, "toxoid_entity_add_component", |entity: u64, component: u64| {
                toxoid_api::toxoid_entity_add_component(entity, component);
            });
            link_function!(linker, store, "toxoid_entity_add_tag", |entity: u64, tag: u64| {
                toxoid_api::toxoid_entity_add_tag(entity, tag);
            });
            link_function!(linker, store, "toxoid_entity_get_component", |entity: u64, component: u64| -> i32 {
                toxoid_api::toxoid_entity_get_component(entity, component) as i32
            });
            link_function!(linker, store, "toxoid_entity_child_of", |entity: u64, parent: u64| {
                toxoid_api::toxoid_entity_child_of(entity, parent);
            });
            link_function!(linker, store, "toxoid_entity_children", |parent: u64| -> i32 {
                toxoid_api::toxoid_entity_children(parent) as i32
            });
            link_function!(linker, store, "toxoid_child_entities", |iter: i32| -> i32 {
                toxoid_api::toxoid_child_entities(iter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_term_next", |iter: i32| -> i32 {
                toxoid_api::toxoid_term_next(iter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_query_create", || -> i32 {
                toxoid_api::toxoid_query_create() as i32
            });
            link_function!(linker, store, "toxoid_query_with", |query_desc: i32, filter_index: i32, ids: i32, components_count: i32| -> i32 {
                toxoid_api::toxoid_query_with(query_desc as *mut c_void, filter_index as u8, ids as *mut u64, components_count) as i32
            });
            link_function!(linker, store, "toxoid_query_without", |query_desc: i32, filter_index: i32, ids: i32, components_count: i32| -> i32 {
                toxoid_api::toxoid_query_without(query_desc as *mut c_void, filter_index as u8, ids as *mut u64, components_count) as i32
            });
            link_function!(linker, store, "toxoid_query_with_or", |query_desc: i32, filter_index: i32, ids: i32, components_count: i32| -> i32 {
                toxoid_api::toxoid_query_with_or(query_desc as *mut c_void, filter_index as u8, ids as *mut u64, components_count) as i32
            });
            link_function!(linker, store, "toxoid_query_order_by", |query_desc: i32, component_id: u64, callback: i32| {
                // toxoid_api::toxoid_query_order_by(query_desc as *mut c_void, component_id, callback as *mut c_void)
            });
            link_function!(linker, store, "toxoid_query_build", |query_desc: i32| -> i32 {
                toxoid_api::toxoid_query_build(query_desc as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_query_iter", |query: i32| -> i32 {
                toxoid_api::toxoid_query_iter(query as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_query_next", |iter: i32| -> i32 {
                toxoid_api::toxoid_query_next(iter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_query_count", |iter: i32| -> i32 {
                toxoid_api::toxoid_query_count(iter as *mut c_void)
            });
            link_function!(linker, store, "toxoid_query_field", |iter: i32, term_index: i32, count: u32, index: u32| -> i32 {
                toxoid_api::toxoid_query_field(iter as *mut c_void, term_index, count, index) as i32
            });
            link_function!(linker, store, "toxoid_query_entity_list", |iter: i32, iter_len: i32|  {
                // toxoid_api::toxoid_query_entity_list(iter as *mut c_void).as_ptr() as i32
                unimplemented!()
            });
            link_function!(linker, store, "toxoid_query_field_size", |iter: i32, term_index: i32| -> i32 {
                toxoid_api::toxoid_query_field_size(iter as *mut c_void, term_index) as i32
            });
            link_function!(linker, store, "toxoid_query_field_list", |iter: i32, term_index: i32, count: u32| -> i32 {
                // toxoid_api::toxoid_query_field_list(iter as *mut c_void, term_index, count) as *mut _ as i32
                unimplemented!()
            });
            link_function!(linker, store, "toxoid_filter_create", || -> i32 {
                toxoid_api::toxoid_filter_create() as i32
            });
            link_function!(linker, store, "toxoid_filter_with", |filter: i32, filter_index: i32, ids: i32, components_count: i32| -> i32 {
                toxoid_api::toxoid_filter_with(filter as *mut c_void, filter_index as u8, ids as *mut u64, components_count) as i32
            });
            link_function!(linker, store, "toxoid_filter_without", |filter: i32, filter_index: i32, ids: i32, components_count: i32| -> i32 {
                toxoid_api::toxoid_filter_without(filter as *mut c_void, filter_index as u8, ids as *mut u64, components_count) as i32
            });
            link_function!(linker, store, "toxoid_filter_with_or", |filter: i32, filter_index: i32, ids: i32, components_count: i32| -> i32 {
                toxoid_api::toxoid_filter_with_or(filter as *mut c_void, filter_index as u8, ids as *mut u64, components_count) as i32
            });
            link_function!(linker, store, "toxoid_filter_build", |filter: i32| -> i32 {
                toxoid_api::toxoid_filter_build(filter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_filter_term_count", |filter: i32| -> i32 {
                toxoid_api::toxoid_filter_term_count(filter as *mut c_void)
            });
            link_function!(linker, store, "toxoid_filter_term_size", |filter: i32, term_index: i32| -> i32 {
                toxoid_api::toxoid_filter_term_size(filter as *mut c_void, term_index) as i32
            });
            link_function!(linker, store, "toxoid_filter_term_list", |filter: i32, term_index: i32, count: u32| -> i32 {
                toxoid_api::toxoid_filter_term_list(filter as *mut c_void, term_index, count).as_ptr() as i32
            });
            link_function!(linker, store, "toxoid_filter_term_iter", |filter: i32, term_index: i32| -> i32 {
                toxoid_api::toxoid_filter_term_iter(filter as *mut c_void, term_index) as i32
            });
            link_function!(linker, store, "toxoid_filter_term_next", |iter: i32| -> i32 {
                toxoid_api::toxoid_filter_term_next(iter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_filter_term_entity", |iter: i32, count: u32, index: u32| -> u64 {
                toxoid_api::toxoid_filter_term_entity(iter as *mut c_void, count, index) as u64
            });
            link_function!(linker, store, "toxoid_filter_term_entity_list", |iter: i32| -> i32 {
                toxoid_api::toxoid_filter_term_entity_list(iter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_iter_count", |iter: i32| -> i32 {
                toxoid_api::toxoid_iter_count(iter as *mut c_void)
            });
            link_function!(linker, store, "toxoid_component_cache_insert", |type_id: u64, component_id: u64| {
                toxoid_api::toxoid_component_cache_insert(type_id, component_id) 
            });
            link_function!(linker, store, "toxoid_component_cache_get", |type_id: u64| -> u64 {
                toxoid_api::toxoid_component_cache_get(type_id)
            });
            link_function!(linker, store, "toxoid_network_entity_cache_insert", |network_id: u64, entity_id: u64| {
                toxoid_api::toxoid_network_entity_cache_insert(network_id, entity_id);
            });
            link_function!(linker, store, "toxoid_network_entity_cache_get", |network_id: u64| -> u64 {
                toxoid_api::toxoid_network_entity_cache_get(network_id)
            });
            link_function!(linker, store, "toxoid_network_entity_cache_remove", |network_id: u64| {
                toxoid_api::toxoid_network_entity_cache_remove(network_id);
            });
            link_function!(linker, store, "toxoid_component_get_member_u8", |component_ptr: i32, offset: u32| -> u32 {
                toxoid_api::toxoid_component_get_member_u8(component_ptr as *mut c_void, offset) as u32
            });
            link_function!(linker, store, "toxoid_component_get_member_u16", |component_ptr: i32, offset: u32| -> u32 {
                toxoid_api::toxoid_component_get_member_u16(component_ptr as *mut c_void, offset) as u32
            });
            link_function!(linker, store, "toxoid_component_get_member_u32", |component_ptr: i32, offset: u32| -> u32 {
                toxoid_api::toxoid_component_get_member_u32(component_ptr as *mut c_void, offset)
            });
            link_function!(linker, store, "toxoid_component_get_member_u64", |component_ptr: i32, offset: u32| -> u64 {
                toxoid_api::toxoid_component_get_member_u64(component_ptr as *mut c_void, offset).into()
            });
            link_function!(linker, store, "toxoid_component_get_member_i8", |component_ptr: i32, offset: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_i8(component_ptr as *mut c_void, offset as u8) as i32
            });
            link_function!(linker, store, "toxoid_component_get_member_i16", |component_ptr: i32, offset: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_i16(component_ptr as *mut c_void, offset as u8) as i32
            });
            link_function!(linker, store, "toxoid_component_get_member_i32", |component_ptr: i32, offset: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_i32(component_ptr as *mut c_void, offset)
            });
            link_function!(linker, store, "toxoid_component_get_member_i64", |component_ptr: i32, offset: u32| -> i64 {
                toxoid_api::toxoid_component_get_member_i64(component_ptr as *mut c_void, offset)
            });
            link_function!(linker, store, "toxoid_component_get_member_f32", |component_ptr: i32, offset: u32| -> f32 {
                toxoid_api::toxoid_component_get_member_f32(component_ptr as *mut c_void, offset)
            });
            link_function!(linker, store, "toxoid_component_get_member_f64", |component_ptr: i32, offset: u32| -> f64 {
                toxoid_api::toxoid_component_get_member_f64(component_ptr as *mut c_void, offset)
            });
            link_function!(linker, store, "toxoid_component_get_member_bool", |component_ptr: i32, offset: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_bool(component_ptr as *mut c_void, offset) as i32
            });
            link_function!(linker, store, "toxoid_component_get_member_u32array", |component_ptr: i32, offset: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_u32array(component_ptr as *mut c_void, offset) as i32
            });
            link_function!(linker, store, "toxoid_component_get_member_f32array", |component_ptr: i32, offset: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_f32array(component_ptr as *mut c_void, offset) as i32
            });
            link_function!(linker, store, "toxoid_component_get_member_string", |component_ptr: i32, offset: u32, len: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_string(component_ptr as *mut c_void, offset, len) as i32
            });
            link_function!(linker, store, "toxoid_component_set_member_u8", |component_ptr: i32, offset: u32, value: u32| {
                toxoid_api::toxoid_component_set_member_u8(component_ptr as *mut c_void, offset, value as u8)
            });
            link_function!(linker, store, "toxoid_component_set_member_u16", |component_ptr: i32, offset: u32, value: u32| {
                toxoid_api::toxoid_component_set_member_u16(component_ptr as *mut c_void, offset, value as u16);
            });
            link_function!(linker, store, "toxoid_component_set_member_u32", |component_ptr: i32, offset: u32, value: u32| {
                toxoid_api::toxoid_component_set_member_u32(component_ptr as *mut c_void, offset, value);
            });
            link_function!(linker, store, "toxoid_component_set_member_u64", |component_ptr: i32, offset: u32, value: u64| {
                toxoid_api::toxoid_component_set_member_u64(component_ptr as *mut c_void, offset, toxoid_api::split_u64(value));
            });
            link_function!(linker, store, "toxoid_component_set_member_i8", |component_ptr: i32, offset: u32, value: i32| {
                toxoid_api::toxoid_component_set_member_i8(component_ptr as *mut c_void, offset, value as i8);
            });
            link_function!(linker, store, "toxoid_component_set_member_i16", |component_ptr: i32, offset: u32, value: u32| {
                toxoid_api::toxoid_component_set_member_i16(component_ptr as *mut c_void, offset, value as i16);
            });
            link_function!(linker, store, "toxoid_component_set_member_i32", |component_ptr: i32, offset: u32, value: i32| {
                toxoid_api::toxoid_component_set_member_i32(component_ptr as *mut c_void, offset, value);
            });
            link_function!(linker, store, "toxoid_component_set_member_i64", |component_ptr: i32, offset: u32, value: i64| {
                toxoid_api::toxoid_component_set_member_i64(component_ptr as *mut c_void, offset, value);
            });
            link_function!(linker, store, "toxoid_component_set_member_f32", |component_ptr: i32, offset: u32, value: f32| {
                toxoid_api::toxoid_component_set_member_f32(component_ptr as *mut c_void, offset, value);
            });
            link_function!(linker, store, "toxoid_component_set_member_f64", |component_ptr: i32, offset: u32, value: f64| {
                toxoid_api::toxoid_component_set_member_f64(component_ptr as *mut c_void, offset, value);
            });
            link_function!(linker, store, "toxoid_component_set_member_bool", |component_ptr: i32, offset: u32, value: i32| {
                if value == 0 {
                    toxoid_api::toxoid_component_set_member_bool(component_ptr as *mut c_void, offset, false);
                } else {
                    toxoid_api::toxoid_component_set_member_bool(component_ptr as *mut c_void, offset, true);
                }
            });
            link_function!(linker, store, "toxoid_component_set_member_u32array", |component_ptr: i32, offset: u32, value: i32| {
                toxoid_api::toxoid_component_set_member_u32array(component_ptr as *mut c_void, offset, value as *mut u32);
            });
            link_function!(linker, store, "toxoid_component_set_member_f32array", |component_ptr: i32, offset: u32, value: i32| {
                toxoid_api::toxoid_component_set_member_f32array(component_ptr as *mut c_void, offset, value as *mut f32);
            });
            link_function!(linker, store, "toxoid_component_set_member_string", |component_ptr: i32, offset: u32, len: u32, value: i32, value_len: i32| {
                // let wasm_string = get_wasm_string(&caller, value, value_len);
                // toxoid_api::toxoid_component_set_member_string(component_ptr as *mut c_void, offset, len, wasm_string.as_str());
            });
            link_function!(linker, store, "toxoid_component_set_member_ptr", |component_ptr: i32, offset: u32, value: i32| {
                toxoid_api::toxoid_component_set_member_ptr(component_ptr as *mut c_void, offset, value as *mut c_void);
            });
            link_function!(linker, store, "toxoid_component_get_member_ptr", |component_ptr: i32, offset: u32| -> i32 {
                toxoid_api::toxoid_component_get_member_ptr(component_ptr as *mut c_void, offset) as i32
            });
            link_function!(linker, store, "toxoid_progress", |delta_time: f32| -> i32 {
                toxoid_api::toxoid_progress(delta_time) as i32
            });
            link_function!(linker, store, "toxoid_filter_children_init", |parent: u64| -> i32 {
                toxoid_api::toxoid_filter_children_init(parent) as i32
            });
            link_function!(linker, store, "toxoid_filter_iter", |filter: i32| -> i32 {
                toxoid_api::toxoid_filter_iter(filter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_filter_next", |iter: i32| -> i32 {
                toxoid_api::toxoid_filter_next(iter as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_iter_entities", |iter: i32| -> i32 {
                // toxoid_api::toxoid_iter_entities(iter as *mut c_void) as *mut _ as i32
                unimplemented!()
            });
            link_function!(linker, store, "toxoid_delete_entity", |entity: u64| {
                toxoid_api::toxoid_delete_entity(entity);
            });
            link_function!(linker, store, "toxoid_entity_remove_component", |entity: u64, component: u64| {
                toxoid_api::toxoid_entity_remove_component(entity, component);
            });
            link_function!(linker, store, "toxoid_is_valid", |entity: u64| -> i32 {
                toxoid_api::toxoid_is_valid(entity) as i32
            });
            link_function!(linker, store, "toxoid_entity_has_component", |entity: u64, component: u64| -> i32 {
                toxoid_api::toxoid_entity_has_component(entity, component) as i32
            });
            link_function!(linker, store, "toxoid_singleton_get", |component: u64| -> i32 {
                toxoid_api::toxoid_singleton_get(component) as i32
            });
            link_function!(linker, store, "toxoid_singleton_add", |component: u64| {
                toxoid_api::toxoid_singleton_add(component);
            });
            link_function!(linker, store, "toxoid_singleton_remove", |component: u64| {
                toxoid_api::toxoid_singleton_remove(component);
            });
            link_function!(linker, store, "toxoid_prefab_create", || -> i32 {
                toxoid_api::toxoid_prefab_create() as i32
            });
            link_function!(linker, store, "toxoid_prefab_instance", |prefab_high: u32, prefab_low: u32| -> i32 {
                // toxoid_api::toxoid_prefab_instance(prefab_high, prefab_low) as i32
                0
            });
            link_function!(linker, store, "toxoid_system_create", |callback_closure: i32| -> i32 {
                // toxoid_api::toxoid_system_create(callback_closure as *mut c_void) as i32
                unimplemented!()
            });
            link_function!(linker, store, "toxoid_system_build", |system_desc: i32| -> u64 {
                toxoid_api::toxoid_system_build(system_desc as *mut c_void) 
            });
            link_function!(linker, store, "toxoid_query_from_system_desc", |query_desc: i32| -> i32 {
                toxoid_api::toxoid_query_from_system_desc(query_desc as *mut c_void) as i32
            });
            link_function!(linker, store, "toxoid_network_send", |network_messages: i32| {
                toxoid_api::toxoid_network_send(network_messages as *mut c_void);
            });
            link_function!(linker, store, "toxoid_net_send_components", |entity_id: u64, components: i32, components_len: i32, event: i32, even_len: i32| {
                // `toxoid_api::toxoid_net_send_components(entity_id, components as *mut c_void, event as *const i8);
                unimplemented!()
            });
            link_function!(linker, store, "toxoid_component_lookup", |name: i32| -> u64 {
                toxoid_api::toxoid_component_lookup(name as *mut i8)
            });
            link_function!(linker, store, "toxoid_net_add_event", |mut event_name: i32, event_name_len: i32, callback: i32| {
                std::thread::spawn(move || {
                    let closure: Box<dyn FnMut(&toxoid_api::net::MessageEntity) + Send + Sync> = Box::new(move |message: &toxoid_api::net::MessageEntity| {
                        println!("Hello?");
                        let wasm_func = get_wasm_func::<(i32,), ()>(callback);
                        let store = &mut *STORE.lock().unwrap();

                        // Serialize MessagEntity using flexbuffers
                        let mut s = toxoid_serialize::flexbuffers::FlexbufferSerializer::new();
                        message.serialize(&mut s).unwrap();
                        println!("Serialized message: {:?}", s.view());
                        // Write to wasmtime memory
                        {
                            let wasm_memory = INSTANCE
                                .lock()
                                .unwrap()
                                .unwrap()
                                .get_export(&store, "memory")
                                .and_then(|extern_| extern_.into_memory())
                                .expect("failed to find host memory");
                            wasm_memory
                                .write(store.as_context_mut(), 0, &s.view())
                                .unwrap();
                        }
                        let msg_ptr = message as *const _ as *const c_void as i32;
                        wasm_func
                            .call(
                                store.as_context_mut(), 
                                (msg_ptr,)
                            )
                            .expect("failed to call WASM function")
                    });
                    let wasm_string = get_wasm_string(event_name, event_name_len);
                    toxoid_net::toxoid_net_add_event(wasm_string.as_str(), closure);
                    toxoid_net::toxoid_net_run_event(wasm_string, &toxoid_serialize::NetworkMessageEntity::default());
                });
            });
            link_function!(linker, store, "toxoid_deserialize_entity_sync", |entity_id: u64, components_serialized: i32, components_serialized_len: i32| {
                // toxoid_api::toxoid_deserialize_entity_sync(entity_id, components_serialized as *mut c_void);
                unimplemented!()
            });
            link_function!(linker, store, "toxoid_make_c_string", |v: i32, v_len: i32| -> i32 {
                // toxoid_api::toxoid_make_c_string(string as *const i8) as i32
                unimplemented!()
            });
            link_function!(linker, store, "toxoid_get_timestamp", || -> i32 {
                // toxoid_api::toxoid_get_timestamp() as i32
                unimplemented!()
            });
            link_function!(linker, store, "gen_uuid", || -> i32 {
                toxoid_api::gen_uuid() as i32
            });
                
        }
        // Instantiate WASM module
        let instance = instantiate_module!(linker, module);
        unsafe { *INSTANCE = Mutex::new(Some(instance)); }
        call_function!(instance, "app_main");
    }
}