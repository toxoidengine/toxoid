use wasmi::{
    Engine, Linker, Module, Store, Memory, MemoryType, 
    Instance, Extern, Func, FuncType, ValType
};
use toxoid_wasm::*;
use std::sync::Arc;

pub struct WasmiRuntime {
    store: Store<()>,
    instance: Instance,
    memory: Memory,
}

impl WasmiRuntime {
    pub fn new(wasm_bytes: &[u8]) -> Result<Self, wasmi::Error> {
        let engine = Engine::default();
        let module = Module::new(&engine, wasm_bytes)?;
        
        let mut linker = Linker::new(&engine);
        
        // Add host functions
        Self::add_component_imports(&mut linker)?;
        Self::add_entity_imports(&mut linker)?;
        Self::add_system_imports(&mut linker)?;
        Self::add_query_imports(&mut linker)?;
        Self::add_world_imports(&mut linker)?;
        
        let mut store = Store::new(&engine, ());
        let instance = linker.instantiate(&mut store, &module)?;
        
        // Get memory export
        let memory = instance
            .get_export(&store, "memory")
            .and_then(Extern::into_memory)
            .ok_or(wasmi::Error::Instantiation("No memory export".into()))?;

        Ok(Self {
            store,
            instance,
            memory,
        })
    }

    fn add_component_imports(linker: &mut Linker<()>) -> Result<(), wasmi::Error> {
        // Component new
        linker.func_wrap(
            "env", 
            "toxoid_component_new",
            |ptr: u64, entity_id: u64, component_type_id: u64| -> u32 {
                let component = toxoid_component_new(ptr, entity_id, component_type_id);
                component as u32
            }
        )?;

        // Component get/set methods
        linker.func_wrap(
            "env",
            "toxoid_component_set_member_u8", 
            |component: u32, offset: u32, value: u8| {
                toxoid_component_set_member_u8(component as *mut _, offset, value);
            }
        )?;

        // Add other component methods...

        Ok(())
    }

    fn add_entity_imports(linker: &mut Linker<()>) -> Result<(), wasmi::Error> {
        linker.func_wrap(
            "env",
            "toxoid_entity_new",
            |desc_ptr: u32, desc_len: u32| -> u32 {
                let entity = toxoid_entity_new(desc_ptr as *const _, desc_len as usize);
                entity as u32
            }
        )?;

        // Add other entity methods...

        Ok(())
    }

    fn add_system_imports(linker: &mut Linker<()>) -> Result<(), wasmi::Error> {
        // Similar to entity imports...
        Ok(())
    }

    fn add_query_imports(linker: &mut Linker<()>) -> Result<(), wasmi::Error> {
        // Similar pattern...
        Ok(())
    }

    fn add_world_imports(linker: &mut Linker<()>) -> Result<(), wasmi::Error> {
        linker.func_wrap(
            "env",
            "toxoid_world_add_singleton",
            |component_id: u64| {
                toxoid_world_add_singleton(component_id);
            }
        )?;

        // Add other world functions...

        Ok(())
    }

    pub fn call_init(&mut self) -> Result<(), wasmi::Error> {
        if let Some(init) = self.instance.get_export(&self.store, "init") {
            if let Some(func) = init.into_func() {
                func.call(&mut self.store, &[], &mut [])?;
            }
        }
        Ok(())
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }
} 