#[cfg(feature = "wasmtime")]
use wasmtime::*;
#[cfg(feature = "wasmi")]
use wasmi::*;

// Macro for creating the engine with wasmtime
#[cfg(feature = "wasmtime")]
#[macro_export]
macro_rules! create_engine_wasmtime {
    () => {
        wasmtime::Engine::default()
    };
}

// Macro for creating the engine with wasmi
#[cfg(feature = "wasmi")]
#[macro_export]
macro_rules! create_engine_wasmi {
    () => {
        wasmi::Engine::default()
    };
}

// Unified macro to call the appropriate macro based on the feature flag
#[macro_export]
macro_rules! create_engine {
    () => {
        create_engine_impl!()
    };
}

#[cfg(feature = "wasmtime")]
macro_rules! create_engine_impl {
    () => {
        create_engine_wasmtime!()
    };
}

#[cfg(feature = "wasmi")]
macro_rules! create_engine_impl {
    () => {
        create_engine_wasmi!()
    };
}

// Macro for compiling modules with wasmtime
#[cfg(feature = "wasmtime")]
#[macro_export]
macro_rules! compile_module_wasmtime {
    ($engine:expr, $wat:expr) => {
        wasmtime::Module::new(&$engine, $wat).expect("Failed to create module")
    };
}

// Macro for compiling modules with wasmi
#[cfg(feature = "wasmi")]
#[macro_export]
macro_rules! compile_module_wasmi {
    ($engine:expr, $wat:expr) => {{
        let wasm = wat::parse_str($wat).expect("Failed to parse WAT");
        wasmi::Module::new(&$engine, &wasm[..]).expect("Failed to create module")
    }};
}

// Unified macro to call the appropriate macro based on the feature flag
#[macro_export]
macro_rules! compile_module {
    ($engine:expr, $wat:expr) => {
        compile_module_impl!($engine, $wat)
    };
}

#[cfg(feature = "wasmtime")]
macro_rules! compile_module_impl {
    ($engine:expr, $wat:expr) => {
        compile_module_wasmtime!($engine, $wat)
    };
}

#[cfg(feature = "wasmi")]
macro_rules! compile_module_impl {
    ($engine:expr, $wat:expr) => {
        compile_module_wasmi!($engine, $wat)
    };
}

// Macro for creating a store with wasmtime
#[cfg(feature = "wasmtime")]
#[macro_export]
macro_rules! create_store_wasmtime {
    ($engine:expr, $host_state:expr) => {
        wasmtime::Store::new(&$engine, $host_state)
    };
}

// Macro for creating a store with wasmi
#[cfg(feature = "wasmi")]
#[macro_export]
macro_rules! create_store_wasmi {
    ($engine:expr, $host_state:expr) => {{
        let mut store = wasmi::Store::new(&$engine, $host_state);
        store
    }};
}

// Unified macro to call the appropriate macro based on the feature flag
#[macro_export]
macro_rules! create_store {
    ($engine:expr, $host_state:expr) => {
        create_store_impl!($engine, $host_state)
    };
}

#[cfg(feature = "wasmtime")]
macro_rules! create_store_impl {
    ($engine:expr, $host_state:expr) => {
        create_store_wasmtime!($engine, $host_state)
    };
}

#[cfg(feature = "wasmi")]
macro_rules! create_store_impl {
    ($engine:expr, $host_state:expr) => {
        create_store_wasmi!($engine, $host_state)
    };
}

// Macro for linking functions with wasmtime
#[cfg(feature = "wasmtime")]
#[macro_export]
macro_rules! link_function_wasmtime {
    ($engine:expr, $store:expr, $func:expr) => {{
        // TODO: Extract the host state type from the closure in a macro
        let mut linker: wasmtime::Linker<u32> = wasmtime::Linker::new(&$engine);
        linker.func_wrap("host", "host_func", $func).expect("Failed to wrap function");
        linker
    }};
}

// Macro for linking functions with wasmi
#[cfg(feature = "wasmi")]
#[macro_export]
macro_rules! link_function_wasmi {
    ($engine:expr, $store:expr, $func:expr) => {{
        // TODO: Extract the host state type from the closure in a macro
        let mut linker: wasmi::Linker<u32> = wasmi::Linker::new(&$engine);
        let host_hello = wasmi::Func::wrap(&mut $store, $func);
        linker.define("host", "hello", host_hello).expect("Failed to define function");
        linker
    }};
}


// Unified macro to call the appropriate macro based on the feature flag
#[macro_export]
macro_rules! link_function {
    ($engine:expr, $store:expr, $func:expr) => {{
        link_function_impl!($engine, $store, $func)
    }};
}


#[cfg(feature = "wasmtime")]
macro_rules! link_function_impl {
    ($engine:expr, $store:expr,  $func:expr) => {
        link_function_wasmtime!($engine, $store, $func)
    };
}

#[cfg(feature = "wasmi")]
macro_rules! link_function_impl {
    ($engine:expr, $store:expr, $func:expr) => {
        link_function_wasmi!($engine, $store, $func)
    };
}

// Macro for instantiation and function calling with wasmtime
#[cfg(feature = "wasmtime")]
#[macro_export]
macro_rules! instantiate_and_call_wasmtime {
    ($linker:expr, $store:expr, $module:expr) => {
        let instance = $linker.instantiate(&mut $store, &$module).expect("Failed to instantiate module");
        let hello = instance.get_typed_func::<(), ()>(&mut $store, "hello").expect("Failed to get function");
        hello.call(&mut $store, ()).expect("Failed to call function");
    };
}

// Macro for instantiation and function calling with wasmi
#[cfg(feature = "wasmi")]
#[macro_export]
macro_rules! instantiate_and_call_wasmi {
    ($linker:expr, $store:expr, $module:expr) => {
        let instance = $linker.instantiate(&mut $store, &$module).expect("Failed to instantiate module").start(&mut $store).expect("Failed to start instance");
        let hello = instance.get_typed_func::<(), ()>(&$store, "hello").expect("Failed to get function");
        hello.call(&mut $store, ()).expect("Failed to call function");
    };
}

// Unified macro to call the appropriate macro based on the feature flag
#[macro_export]
macro_rules! instantiate_and_call {
    ($linker:expr, $store:expr, $module:expr) => {
        instantiate_and_call_impl!($linker, $store, $module)
    };
}

#[cfg(feature = "wasmtime")]
macro_rules! instantiate_and_call_impl {
    ($linker:expr, $store:expr, $module:expr) => {
        instantiate_and_call_wasmtime!($linker, $store, $module)
    };
}

#[cfg(feature = "wasmi")]
macro_rules! instantiate_and_call_impl {
    ($linker:expr, $store:expr, $module:expr) => {
        instantiate_and_call_wasmi!($linker, $store, $module)
    };
}

#[cfg(any(feature = "wasmi", feature = "wasmtime"))]
pub fn wasm_init() {
    let wat = r#"
    (module
        (import "host" "hello" (func $host_hello (param i32) (param i64)))
        (func (export "hello")
            (call $host_hello (i32.const 3) (i64.const 3))
        )
    )
    "#;
    let engine = create_engine!();
    let module = compile_module!(engine, wat);
    #[allow(unused_mut)]
    let mut store = create_store!(engine, 0);  
    let linker = link_function!(engine, store, |caller: Caller<'_, u32>, param1: i32, param2: i64| {
        // Your function logic here
        println!("Integer: {}, Double: {}", param1, param2);
        println!("Host data: {}", caller.data());
    });
    instantiate_and_call!(linker, store, module);
}