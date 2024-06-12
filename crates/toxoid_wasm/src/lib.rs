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
    ($engine:expr, $wat:expr) => {
        wasmtime::Module::new(&$engine, $wat)
            .expect("Failed to create module")
    };
}

// Macro for compiling modules with wasmi
#[cfg(feature = "wasmi")]
macro_rules! compile_module {
    ($engine:expr, $wat:expr) => {{
        let wasm = wat::parse_str($wat)
            .expect("Failed to parse WAT");
        wasmi::Module::new(&$engine, &wasm[..])
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
            .func_wrap("host", $name, $func)
            .expect("Failed to wrap function");
    }};
}

#[cfg(feature = "wasmi")]
macro_rules! link_function {
    ($linker:expr, $store:expr, $name:expr, $func:expr) => {{
        $linker
            .define("host", $name, wasmi::Func::wrap(&mut $store, $func))
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
pub fn wasm_init() {
    // Use WAT for initial testing
    let wat = r#"
        (module
            (import "host" "toxoid_print_i32" (func $toxoid_print_i32 (param i32)))
            (func (export "app_main")
                (call $toxoid_print_i32 (i32.const 777))
            )
        )
    "#;

    // Setup WASM runtime
    let engine = create_engine!();
    let module = compile_module!(engine, wat);
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
    }
    
    // Instantiate WASM module
    let instance = instantiate_module!(linker, store, module);
    call_function!(instance, store, "app_main");
}