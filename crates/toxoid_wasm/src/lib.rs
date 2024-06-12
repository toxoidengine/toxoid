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
    let mut path = std::env::current_exe()
        .expect("Failed to get current executable path");
    path.pop(); // Remove the executable name
    path.push("toxoid_wasm_test.wasm");

    println!("path: {:?}", path);
 
    let wasm = std::fs::read(path)
        .expect("Failed to read WASM file");
    // Setup WASM runtime
    let engine = create_engine!();
    let module = compile_module!(engine, wasm);
    let mut store = create_store!(engine, 0);  
    let mut linker = create_linker!(engine, u32);

    // Bind functions
    unsafe {
        link_function!(linker, store, "toxoid_print_i32", |_caller: Caller<'_, u32>, v: i32| {
            toxoid_api::toxoid_print_i32(v)
        });
        link_function!(linker, store, "toxoid_print_u64", |_caller: Caller<'_, u32>, v: u64| {
            toxoid_api::toxoid_print_u64(toxoid_api::split_u64(v))
        });
        link_function!(linker, store, "toxoid_print_f32", |_caller: Caller<'_, u32>, v: f32| {
            toxoid_api::toxoid_print_f32(v)
        });
        link_function!(linker, store, "toxoid_print_f64", |_caller: Caller<'_, u32>, v: f64| {
            toxoid_api::toxoid_print_f64(toxoid_api::split_f64(v))
        });
        link_function!(linker, store, "toxoid_print_string", |caller: Caller<'_, u32>, v: i32, v_len: i32| {
            let wasm_string = get_wasm_string(caller, v, v_len);
            toxoid_api::toxoid_print_string(wasm_string.as_ptr() as *const i8, wasm_string.len() as usize);
        });
    }
    
    // Instantiate WASM module
    let instance = instantiate_module!(linker, store, module);
    call_function!(instance, store, "app_main");
}