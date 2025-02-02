#![allow(unused_must_use)]
bindgen!({
    world: "toxoid-component-world",
    path: "../toxoid_guest/wit",
    with: {
        // Specify that our host resource is going to point to the `ComponentTypeProxy`, `ComponentProxy`, etc.
        "toxoid-component:component/ecs/component-type": ComponentTypeProxy,
        "toxoid-component:component/ecs/component": ComponentProxy,
        "toxoid-component:component/ecs/entity": EntityProxy,
        "toxoid-component:component/ecs/query": QueryProxy,
        "toxoid-component:component/ecs/system": SystemProxy,
        "toxoid-component:component/ecs/callback": CallbackProxy,
        "toxoid-component:component/ecs/iter": IterProxy,
        "toxoid-component:component/ecs/observer": ObserverProxy,
        "toxoid-component:component/ecs/pipeline": PipelineProxy,
        "toxoid-component:component/ecs/phase": PhaseProxy,
    },
});

use std::collections::HashMap;
use toxoid_api::{EcsEntityT, GuestObserver};
use toxoid_component::component::ecs::PointerT;
use toxoid_host::bindings::exports::toxoid::engine::ecs::{Guest, GuestCallback, GuestComponent, GuestComponentType, GuestEntity, GuestIter, GuestPhase, GuestPipeline, GuestQuery, GuestSystem};
use toxoid_host::ToxoidApi;
use wasmtime::component::{bindgen, Component, Linker, Resource, ResourceTable};
use wasmtime::{Config, Engine, OptLevel, Result, Store};
use wasmtime_wasi::{WasiCtx, WasiView, WasiCtxBuilder};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct ComponentTypeProxy {
    ptr: *mut toxoid_host::ComponentType
}
unsafe impl Send for ComponentTypeProxy {}
pub struct ComponentProxy {
    ptr: *mut toxoid_host::Component
}
unsafe impl Send for ComponentProxy {}
pub struct EntityProxy {
    ptr: *mut toxoid_host::Entity
}
unsafe impl Send for EntityProxy {}
pub struct QueryProxy {
    ptr: *mut toxoid_host::Query
}
unsafe impl Send for QueryProxy {}
pub struct SystemProxy {
    ptr: *mut toxoid_host::System
}
unsafe impl Send for SystemProxy {}
pub struct CallbackProxy {
    ptr: *mut toxoid_host::Callback
}
unsafe impl Send for CallbackProxy {}
pub struct IterProxy {
    pub ptr: *mut toxoid_host::Iter
}
unsafe impl Send for IterProxy {}
pub struct ObserverProxy {
    pub ptr: *mut toxoid_host::Observer
}
unsafe impl Send for ObserverProxy {}
pub struct PipelineProxy {
    pub ptr: *mut toxoid_host::Pipeline
}
unsafe impl Send for PipelineProxy {}
pub struct PhaseProxy {
    pub ptr: *mut toxoid_host::Phase
}
unsafe impl Send for PhaseProxy {}

// StoreState is the state of the WASM store.
pub struct StoreState {
    pub ctx: WasiCtx,
    pub table: ResourceTable,
}

// A trait which provides access to internal WASI state.
// For a Store<T> this trait will be implemented for the T. This also corresponds to the T in Linker<T>.
impl WasiView for StoreState {
    fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
    fn table(&mut self) -> &mut ResourceTable { &mut self.table }
}

static mut SINGLETON_MAP: Lazy<HashMap<toxoid_component::component::ecs::EcsEntityT, u32>> = Lazy::new(|| HashMap::new());

impl toxoid_component::component::ecs::Host for StoreState {
    fn add_singleton(&mut self, component: toxoid_component::component::ecs::EcsEntityT) {
        ToxoidApi::add_singleton(component);
        // TODO: Check if !ptr.is_null()
        let component_ptr = ToxoidApi::get_singleton(component);
        let host_component = toxoid_host::Component::new(component_ptr, component, component);
        let boxed_component_ptr = Box::into_raw(Box::new(host_component));
        let resource = self.table.push::<ComponentProxy>(ComponentProxy {
            ptr: boxed_component_ptr 
        })
            .expect("Failed to push component to table");
        unsafe {
            SINGLETON_MAP.insert(component, resource.rep());
        }
    }

    fn get_singleton(&mut self, component: toxoid_component::component::ecs::EcsEntityT) -> Resource<ComponentProxy> {
        unsafe {
            if SINGLETON_MAP.contains_key(&component) {    
                let resource_rep = SINGLETON_MAP.get(&component).unwrap().clone();
                let resource = Resource::<ComponentProxy>::new_own(resource_rep);
                resource
            } else {
                panic!("Failed to get singleton, component ID: {:?}", component);
                // let ptr = ToxoidApi::get_singleton(component) as *mut toxoid_host::Component;
                // let resource = self.table.push::<ComponentProxy>(ComponentProxy { ptr: ptr }).expect("Failed to push component to table");
                // resource
            }
        }
    }

    fn remove_singleton(&mut self, component: toxoid_component::component::ecs::EcsEntityT) {
        ToxoidApi::remove_singleton(component);
        unsafe {
            SINGLETON_MAP.remove(&component);
        }
    }

    fn add_entity(&mut self, entity: toxoid_component::component::ecs::EcsEntityT) {
        ToxoidApi::add_entity(entity);
    }

    fn remove_entity(&mut self, entity: toxoid_component::component::ecs::EcsEntityT) {
        // TODO: Clean up entity resources for WASM
        // let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        // drop(unsafe { Box::from_raw(entity_proxy.ptr) });
        // self.table.delete::<EntityProxy>(entity).unwrap();
        ToxoidApi::remove_entity(entity);
    }

    // fn get_entity_named(&mut self, name: String) -> Resource<EntityProxy> {
    //     let entity = ToxoidApi::get_entity_named(name);
    //     self.table.push::<EntityProxy>(EntityProxy { ptr: entity }).unwrap()
    // }

    fn has_entity_named(&mut self, name: String) -> bool {
        ToxoidApi::has_entity_named(name)
    }

    fn get_component_id(&mut self, component_name: String) -> toxoid_component::component::ecs::EcsEntityT {
        ToxoidApi::get_component_id(component_name)
    }
}

impl toxoid_component::component::ecs::HostIter for StoreState {
    fn new(&mut self, ptr: u64) -> Resource<IterProxy> {
        self.table.push::<IterProxy>(IterProxy { ptr: ptr as *mut toxoid_host::Iter }).unwrap()
    }   

    fn next(&mut self, iter: Resource<IterProxy>) -> bool {
        let iter_proxy = self.table.get(&iter).unwrap() as &IterProxy;
        let iter = unsafe { Box::from_raw(iter_proxy.ptr) };
        let result = iter.next();
        Box::into_raw(iter);
        result
    }

    fn count(&mut self, iter: Resource<IterProxy>) -> i32 {
        let iter_proxy = self.table.get(&iter).unwrap() as &IterProxy;
        let iter = unsafe { Box::from_raw(iter_proxy.ptr) };
        let result = iter.count();
        Box::into_raw(iter);
        result
    }

    fn entities(&mut self, iter: Resource<IterProxy>) -> Vec<Resource<EntityProxy>>{
        let iter_proxy = self.table.get(&iter).unwrap() as &IterProxy;
        let iter = unsafe { Box::from_raw(iter_proxy.ptr) };
        let entity_ids = iter.entities();
        let ids = entity_ids.iter().map(|entity_id| {
            // Create entity
            let entity = toxoid_host::Entity::from_id(*entity_id) as *mut toxoid_host::Entity;

            // Push component to resource table
            let id = self
                .table
                .push::<EntityProxy>(EntityProxy {
                    ptr: entity
                })
                .expect("Failed to push component to table");
            id
        }).collect();
        Box::into_raw(iter);
        ids
    }

    fn components(&mut self, iter: Resource<IterProxy>, index: i8) -> Vec<PointerT> {
        let iter_proxy = self.table.get(&iter).unwrap() as &IterProxy;
        let iter = unsafe { Box::from_raw(iter_proxy.ptr) };
        let result = iter.components(index);
        Box::into_raw(iter);
        result
    }

    fn drop(&mut self, _iter: Resource<IterProxy>) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

impl toxoid_component::component::ecs::HostCallback for StoreState {
    fn new(&mut self, handle: u64) -> Resource<CallbackProxy> {
        let callback = <toxoid_host::Callback as toxoid_host::bindings::exports::toxoid::engine::ecs::GuestCallback>::new(handle);
        let boxed_callback = Box::new(callback);
        let boxed_callback_ptr = Box::into_raw(boxed_callback);
        self.table.push::<CallbackProxy>(CallbackProxy { ptr: boxed_callback_ptr }).unwrap()
    }

    fn run(&mut self, _callback: wasmtime::component::Resource<CallbackProxy>, iter: wasmtime::component::Resource<IterProxy>) -> () {
        let callback_proxy = self.table.get(&_callback).unwrap() as &CallbackProxy;
        let callback = unsafe { Box::from_raw(callback_proxy.ptr) };
        let callbacks = unsafe { TOXOID_COMPONENT_WORLD.as_mut().unwrap() };
        
        // Get a lock on the store and get a mutable reference to the actual Store
        let mut store_guard = unsafe { STORE.lock().unwrap() };
        let store = &mut *store_guard;
        
        callbacks.interface1.call_run(store, iter, callback.handle).unwrap();
    }

    fn cb_handle(&mut self, _callback: Resource<toxoid_component::component::ecs::Callback>) -> u64 {
        let callback_proxy = self.table.get(&_callback).unwrap() as &CallbackProxy;
        let callback = unsafe { Box::from_raw(callback_proxy.ptr) };
        callback.handle
    }

    fn drop(&mut self, _callback: Resource<toxoid_component::component::ecs::Callback>) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

impl toxoid_component::component::ecs::HostSystem for StoreState {
    fn new(&mut self, desc: toxoid_component::component::ecs::SystemDesc) -> Resource<SystemProxy> {
        let callback_proxy = self.table.get(&desc.callback).unwrap() as &CallbackProxy;
        let callback = unsafe { Box::from_raw(callback_proxy.ptr) };
        let query_desc = toxoid_host::bindings::exports::toxoid::engine::ecs::QueryDesc {
            expr: desc.query_desc.expr,
        };
        let query = toxoid_host::Query::new(query_desc.clone());
        // TODO: Remove this
        let _query_ptr = Box::into_raw(Box::new(query));
        let system = <toxoid_host::System as toxoid_host::bindings::exports::toxoid::engine::ecs::GuestSystem>::new(toxoid_host::bindings::exports::toxoid::engine::ecs::SystemDesc {
            name: desc.name,
            query_desc,
            callback: callback.cb_handle(),
            is_guest: true,
            tick_rate: desc.tick_rate
        });
        let id = self
            .table
            .push::<SystemProxy>(SystemProxy {
                ptr: Box::into_raw(Box::new(system))
            })
            .unwrap();
        Box::into_raw(callback);
        id
    }
    
    fn callback(&mut self, system: Resource<toxoid_component::component::ecs::System>) -> Resource<CallbackProxy> {
        let system_proxy = self.table.get(&system).unwrap() as &SystemProxy;
        let system = unsafe { Box::from_raw(system_proxy.ptr) };
        let callback_handle = system.callback();
        let callback = <toxoid_host::Callback as toxoid_host::bindings::exports::toxoid::engine::ecs::GuestCallback>::new(callback_handle);
        self.table.push::<CallbackProxy>(CallbackProxy { ptr: Box::into_raw(Box::new(callback)) }).unwrap()
    }

    fn build(&mut self, system: Resource<toxoid_component::component::ecs::System>) -> () {
        let system_proxy = self.table.get(&system).unwrap() as &SystemProxy;
        let mut system = unsafe { Box::from_raw(system_proxy.ptr) };
        system.as_mut().build();
        Box::into_raw(system);
    }

    fn disable(&mut self, system: Resource<toxoid_component::component::ecs::System>) -> () {
        let system_proxy = self.table.get(&system).unwrap() as &SystemProxy;
        let mut system = unsafe { Box::from_raw(system_proxy.ptr) };
        system.as_mut().disable();
        Box::into_raw(system);
    }

    fn enable(&mut self, system: Resource<toxoid_component::component::ecs::System>) -> () {
        let system_proxy = self.table.get(&system).unwrap() as &SystemProxy;
        let mut system = unsafe { Box::from_raw(system_proxy.ptr) };
        system.as_mut().enable();
        Box::into_raw(system);
    }

    fn drop(&mut self, _system: Resource<toxoid_component::component::ecs::System>) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

impl toxoid_component::component::ecs::HostObserver for StoreState {
    fn new(&mut self, desc: toxoid_component::component::ecs::ObserverDesc) -> Resource<ObserverProxy> {
        let callback_proxy = self.table.get(&desc.callback).unwrap() as &CallbackProxy;
        let callback = unsafe { Box::from_raw(callback_proxy.ptr) };
        let query_desc = toxoid_host::bindings::exports::toxoid::engine::ecs::QueryDesc {
            expr: desc.query_desc.expr,
        };
        let observer = <toxoid_host::Observer as toxoid_host::bindings::exports::toxoid::engine::ecs::GuestObserver>::new(toxoid_host::bindings::exports::toxoid::engine::ecs::ObserverDesc {
            name: desc.name,
            query_desc,
            events: desc.events.iter().map(|event| match event {
                toxoid_component::component::ecs::Event::OnSet => toxoid_api::Event::OnSet,
                toxoid_component::component::ecs::Event::OnAdd => toxoid_api::Event::OnAdd,
                toxoid_component::component::ecs::Event::OnRemove => toxoid_api::Event::OnRemove,
                toxoid_component::component::ecs::Event::OnDelete => toxoid_api::Event::OnDelete,
                toxoid_component::component::ecs::Event::OnDeleteTarget => toxoid_api::Event::OnDeleteTarget,
                toxoid_component::component::ecs::Event::OnTableCreate => toxoid_api::Event::OnTableCreate,
                toxoid_component::component::ecs::Event::OnTableDelete => toxoid_api::Event::OnTableDelete,
                toxoid_component::component::ecs::Event::OnTableEmpty => toxoid_api::Event::OnTableEmpty,
                toxoid_component::component::ecs::Event::OnTableFill => toxoid_api::Event::OnTableFill
            })
                .collect::<Vec<toxoid_api::Event>>(),
            callback: callback.cb_handle(),
            is_guest: true
        });
        let id = self
            .table
            .push::<ObserverProxy>(ObserverProxy {
                ptr: Box::into_raw(Box::new(observer))
            })
            .unwrap();
        Box::into_raw(callback);
        id
    }

    fn callback(&mut self, _observer: Resource<ObserverProxy>) -> Resource<CallbackProxy> {
        let observer_proxy = self.table.get(&_observer).unwrap() as &ObserverProxy;
        let observer = unsafe { Box::from_raw(observer_proxy.ptr) };
        let callback_handle = observer.callback();
        let callback = <toxoid_host::Callback as toxoid_host::bindings::exports::toxoid::engine::ecs::GuestCallback>::new(callback_handle);
        self.table.push::<CallbackProxy>(CallbackProxy { ptr: Box::into_raw(Box::new(callback)) }).unwrap()
    }

    fn build(&mut self, _observer: Resource<ObserverProxy>) -> () {
        let observer_proxy = self.table.get(&_observer).unwrap() as &ObserverProxy;
        let mut observer = unsafe { Box::from_raw(observer_proxy.ptr) };
        observer.as_mut().build();
        Box::into_raw(observer);
     }

    fn drop(&mut self, _observer: Resource<ObserverProxy>) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

impl toxoid_component::component::ecs::HostEntity for StoreState {
    fn new(&mut self, desc: toxoid_component::component::ecs::EntityDesc, inherits: Option<toxoid_component::component::ecs::EcsEntityT>) -> Resource<EntityProxy> {
        let entity = toxoid_host::Entity::new(toxoid_host::bindings::exports::toxoid::engine::ecs::EntityDesc {
            name: desc.name,
            add: desc.add,
            prefab: desc.prefab
        }, inherits);
        // Create boxed component
        let boxed_entity = Box::new(entity);
        let box_ptr = Box::into_raw(boxed_entity);
        // Push entity to resource table
        let id = self
            .table
            .push::<EntityProxy>(EntityProxy {
                ptr: box_ptr
            })
            .unwrap();
        id
    }

    fn get_id(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>) -> u64 {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        let id = entity.get_id();
        Box::into_raw(entity);
        id
    }

    fn get_name(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>) -> String {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        let name = entity.get_name();
        Box::into_raw(entity);
        name
    }

    fn set_name(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, name: String) -> () {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        entity.set_name(name);
        Box::into_raw(entity);
    }

    fn get(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, component: toxoid_component::component::ecs::EcsEntityT) -> Resource<ComponentProxy> {
        // Safely retrieve the entity proxy
        let entity_proxy = self.table.get(&entity).expect("Entity not found in table") as &EntityProxy;

        // Get entity
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        let entity_id = entity.get_id();
        
        // Retrieve the component
        let component_ptr = entity.get(component);
        Box::into_raw(entity);

        // Create component
        let component = toxoid_host::Component::new(component_ptr, entity_id, component);

        // Create boxed component
        let boxed_component = Box::new(component);
        let boxed_component_ptr = Box::into_raw(boxed_component);

        // Push component to resource table
        let id = self
            .table
            .push::<ComponentProxy>(ComponentProxy { 
                ptr: boxed_component_ptr
            })
            .expect("Failed to push component to table");
        id
    }

    fn from_id(&mut self, id: u64) -> Resource<EntityProxy> {
        let entity = toxoid_host::Entity::from_id(id) as *mut toxoid_host::Entity; 
        self.table.push::<EntityProxy>(EntityProxy { ptr: entity }).unwrap()
    }

    fn add(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, component: toxoid_component::component::ecs::EcsEntityT) -> () {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        entity.add(component);
        Box::into_raw(entity);
    }

    fn has(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, component: toxoid_component::component::ecs::EcsEntityT) -> bool {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        let has = entity.has(component);
        Box::into_raw(entity);
        has
    }

    fn remove(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, component: toxoid_component::component::ecs::EcsEntityT) -> () {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        entity.remove(component);
        Box::into_raw(entity);
    }

    fn add_relationship(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, relationship: toxoid_component::component::ecs::Relationship, target: toxoid_component::component::ecs::EcsEntityT) -> () {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        let relationship = match relationship {
            toxoid_component::component::ecs::Relationship::IsA => toxoid_api::Relationship::IsA,
            toxoid_component::component::ecs::Relationship::ChildOf => toxoid_api::Relationship::ChildOf,
            toxoid_component::component::ecs::Relationship::Custom(entity) => toxoid_api::Relationship::Custom(entity)
        };
        entity.add_relationship(relationship, target);
        Box::into_raw(entity);
    }

    fn remove_relationship(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, relationship: toxoid_component::component::ecs::Relationship, target: toxoid_component::component::ecs::EcsEntityT) -> () {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        let relationship = match relationship {
            toxoid_component::component::ecs::Relationship::IsA => toxoid_api::Relationship::IsA,
            toxoid_component::component::ecs::Relationship::ChildOf => toxoid_api::Relationship::ChildOf,
            toxoid_component::component::ecs::Relationship::Custom(entity) => toxoid_api::Relationship::Custom(entity)
        };
        entity.remove_relationship(relationship, target);
        Box::into_raw(entity);
    }

    fn parent_of(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, target: toxoid_component::component::ecs::EcsEntityT) -> () {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        entity.parent_of(target);
        Box::into_raw(entity);
    }

    fn child_of(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, target: toxoid_component::component::ecs::EcsEntityT) -> () {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        entity.child_of(target);
        Box::into_raw(entity);
    }

    fn parent(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>) -> Resource<EntityProxy> {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        let parent = entity.parent();
        Box::into_raw(entity);
        self.from_id(parent)
    }

    fn children(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>) -> Vec<Resource<EntityProxy>> {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        let children = entity.children();
        Box::into_raw(entity);
        children.iter().map(|child| self.from_id(*child)).collect()
    }

    fn relationships(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>) -> Vec<Resource<EntityProxy>> {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        let relationships = entity.relationships();
        Box::into_raw(entity);
        relationships.iter().map(|relationship| self.from_id(*relationship)).collect()
    }

    fn disable(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>) -> () {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        entity.disable();
        Box::into_raw(entity);
    }

    fn enable(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>) -> () {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        entity.enable();
        Box::into_raw(entity);
    }

    fn drop(&mut self, _entity: Resource<toxoid_component::component::ecs::Entity>) -> Result<(), wasmtime::Error> {
        // let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        // drop(unsafe { Box::from_raw(entity_proxy.ptr) });
        // self.table.delete::<EntityProxy>(entity).unwrap();
        Ok(())
    }
}

impl toxoid_component::component::ecs::HostComponentType for StoreState {
    fn new(&mut self, desc: toxoid_component::component::ecs::ComponentDesc) -> Resource<ComponentTypeProxy> {
        // Create component
        let component = toxoid_host::ComponentType::new(toxoid_host::bindings::exports::toxoid::engine::ecs::ComponentDesc {
            name: desc.name,
            member_names: desc.member_names,
            member_types: desc.member_types,
        });
        // Create boxed component
        let boxed_component = Box::new(component);
        let boxed_component_ptr = Box::into_raw(boxed_component);
        // Push component to resource table
        let id = self
            .table
            .push::<ComponentTypeProxy>(ComponentTypeProxy { 
                ptr: boxed_component_ptr as *mut toxoid_host::ComponentType
            })
            .unwrap();
        id
    }

    fn get_id(&mut self, component: Resource<toxoid_component::component::ecs::ComponentType>) -> u64 {    
        // Get component from resource table
        let component_proxy = self.table.get(&component).unwrap() as &ComponentTypeProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let id = component.get_id();
        Box::into_raw(component);
        id
    }

    fn drop(&mut self, _component: Resource<toxoid_component::component::ecs::ComponentType>) -> Result<(), wasmtime::Error> {
        // let component_proxy = self.table.get(&component).unwrap() as &ComponentTypeProxy;
        // drop(unsafe { Box::from_raw(component_proxy.ptr) });
        // self.table.delete::<ComponentTypeProxy>(component).unwrap();
        Ok(())
    }
}

impl toxoid_component::component::ecs::HostComponent for StoreState {
    fn new(&mut self, component_ptr: u64, entity_id: u64, component_type_id: u64) -> Resource<ComponentProxy> {
        let component = toxoid_host::Component::new(component_ptr, entity_id, component_type_id);
        let boxed_component = Box::new(component);
        let boxed_component_ptr = Box::into_raw(boxed_component);
        self.table.push::<ComponentProxy>(ComponentProxy { ptr: boxed_component_ptr }).unwrap()
    }

    fn set_member_u8(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: u8) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_u8(offset, value);
        Box::into_raw(component);
    }

    fn get_member_u8(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> u8 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_u8(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_u16(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: u16) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_u16(offset, value);
        Box::into_raw(component);
    }

    fn get_member_u16(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> u16 {  
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_u16(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_u32(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: u32) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_u32(offset, value);
        Box::into_raw(component);
    }

    fn get_member_u32(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> u32 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_u32(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_u64(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: u64) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_u64(offset, value);
        Box::into_raw(component);
    }

    fn get_member_u64(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> u64 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_u64(offset);
        Box::into_raw(component);
        value
    }

    fn get_member_i8(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> i8 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_i8(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_i8(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: i8) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_i8(offset, value);
        Box::into_raw(component);
    }

    fn get_member_i16(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> i16 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_i16(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_i16(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: i16) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_i16(offset, value);
        Box::into_raw(component);
    }

    fn get_member_i32(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> i32 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_i32(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_i32(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: i32) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_i32(offset, value);
        Box::into_raw(component);
    }

    fn get_member_i64(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> i64 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_i64(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_i64(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: i64) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_i64(offset, value);
        Box::into_raw(component);
    }

    fn get_member_f32(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> f32 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_f32(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_f32(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: f32) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_f32(offset, value);
        Box::into_raw(component);
    }

    fn get_member_f64(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> f64 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_f64(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_f64(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: f64) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_f64(offset, value);
        Box::into_raw(component);
    }

    fn get_member_bool(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> bool {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_bool(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_bool(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: bool) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_bool(offset, value);
        Box::into_raw(component);
    }

    fn get_member_string(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> String {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_string(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_string(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: String) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_string(offset, value);
        Box::into_raw(component);
    }

    fn get_member_u32list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> Vec<u32> {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_u32list(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_u8list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: Vec<u8>) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_u8list(offset, value);
        Box::into_raw(component);
    }

    fn get_member_u8list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> Vec<u8> {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_u8list(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_u16list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: Vec<u16>) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_u16list(offset, value);
        Box::into_raw(component);
    }

    fn get_member_u16list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> Vec<u16> {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_u16list(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_u32list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: Vec<u32>) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_u32list(offset, value);
        Box::into_raw(component);
    }

    fn get_member_u64list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> Vec<u64> {   
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_u64list(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_u64list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: Vec<u64>) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_u64list(offset, value);
        Box::into_raw(component);
    }

    fn set_member_i8list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: Vec<i8>) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_i8list(offset, value);
        Box::into_raw(component);
    }

    fn get_member_i8list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> Vec<i8> {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_i8list(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_i16list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: Vec<i16>) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_i16list(offset, value);
        Box::into_raw(component);
    }

    fn get_member_i16list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> Vec<i16> {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_i16list(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_i32list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: Vec<i32>) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_i32list(offset, value);
        Box::into_raw(component);
    }

    fn get_member_i32list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> Vec<i32> {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_i32list(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_i64list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: Vec<i64>) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_i64list(offset, value);
        Box::into_raw(component);
    }

    fn get_member_i64list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> Vec<i64> {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_i64list(offset);
        Box::into_raw(component);
        value
    }

    fn get_member_f32list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> Vec<f32> {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_f32list(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_f32list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: Vec<f32>) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_f32list(offset, value);
        Box::into_raw(component);
    }

    fn set_member_f64list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: Vec<f64>) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_f64list(offset, value);
        Box::into_raw(component);
    }

    fn get_member_f64list(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> Vec<f64> {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_f64list(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_pointer(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: u64) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_pointer(offset, value);
        Box::into_raw(component);
    }

    fn get_member_pointer(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> u64 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_pointer(offset);
        Box::into_raw(component);
        value
    }

    fn drop(&mut self, _component: Resource<toxoid_component::component::ecs::Component>) -> Result<(), wasmtime::Error> {
        // let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        // drop(unsafe { Box::from_raw(component_proxy.ptr) });
        // self.table.delete::<ComponentProxy>(component).unwrap();
        Ok(())
    }
}

impl toxoid_component::component::ecs::HostQuery for StoreState {
    fn new(&mut self, query_desc: toxoid_component::component::ecs::QueryDesc) -> Resource<toxoid_component::component::ecs::Query> {
        let query = toxoid_host::Query::new(toxoid_host::bindings::exports::toxoid::engine::ecs::QueryDesc { expr: query_desc.expr });
        let ptr = Box::into_raw(Box::new(query));
        let query_resource = self.table.push(QueryProxy { ptr }).unwrap();
        query_resource
    }

    fn build(&mut self, query: Resource<toxoid_component::component::ecs::Query>) -> () {
        let query_proxy = self.table.get(&query).unwrap() as &QueryProxy;
        let query = unsafe { Box::from_raw(query_proxy.ptr) };
        query.build();
        Box::into_raw(query);
    }

    fn iter(&mut self, query: Resource<toxoid_component::component::ecs::Query>) -> Resource<IterProxy> {
        let query_proxy = self.table.get(&query).unwrap() as &QueryProxy;
        let query = unsafe { Box::from_raw(query_proxy.ptr) };
        let iter = query.iter();
        Box::into_raw(query);
        self.table.push::<IterProxy>(IterProxy { ptr: iter as *mut toxoid_host::Iter }).unwrap()
    }

    fn next(&mut self, query: Resource<toxoid_component::component::ecs::Query>) -> bool {
        let query_proxy = self.table.get(&query).unwrap() as &QueryProxy;
        let query = unsafe { Box::from_raw(query_proxy.ptr) };
        let result = query.next();
        Box::into_raw(query);
        result
    }

    fn count(&mut self, query: Resource<toxoid_component::component::ecs::Query>) -> i32 {
        let query_proxy = self.table.get(&query).unwrap() as &QueryProxy;
        let query = unsafe { Box::from_raw(query_proxy.ptr) };
        let count = query.count();
        Box::into_raw(query);
        count
    }

    fn entities(&mut self, query: Resource<toxoid_component::component::ecs::Query>) -> Vec<Resource<EntityProxy>> {
        let query_proxy = self.table.get(&query).unwrap() as &QueryProxy;
        let query = unsafe { Box::from_raw(query_proxy.ptr) };
        let entity_ids = query.entities();
        let ids = entity_ids.iter().map(|entity_id| {
            // Create entity
            let entity = toxoid_host::Entity::from_id(*entity_id) as *mut toxoid_host::Entity;

            // Push component to resource table
            let id = self
                .table
                .push::<EntityProxy>(EntityProxy {
                    ptr: entity
                })
                .expect("Failed to push component to table");
            id
        }).collect();
        Box::into_raw(query);
        ids
    }

    fn components(&mut self, query: Resource<toxoid_component::component::ecs::Query>, index: i8) -> Vec<PointerT> {
        let query_proxy = self.table.get(&query).unwrap() as &QueryProxy;
        let query = unsafe { Box::from_raw(query_proxy.ptr) };
        let components = query.components(index);
        Box::into_raw(query);
        components
    }

    fn drop(&mut self, _query: Resource<toxoid_component::component::ecs::Query>) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

impl toxoid_component::component::ecs::HostPhase for StoreState {
    fn new(&mut self, name: String) -> Resource<toxoid_component::component::ecs::Phase> {
        let phase = toxoid_host::Phase::new(name);
        let ptr = Box::into_raw(Box::new(phase));
        let phase_resource = self.table.push(PhaseProxy { ptr }).unwrap();
        phase_resource
    }

    fn depends_on(&mut self, phase: Resource<toxoid_component::component::ecs::Phase>, dependency: toxoid_component::component::ecs::Phases) -> () {
        let phase_proxy = self.table.get(&phase).unwrap() as &PhaseProxy;
        let phase = unsafe { Box::from_raw(phase_proxy.ptr) };
        let dependency = match dependency {
            toxoid_component::component::ecs::Phases::OnStart => toxoid_host::bindings::exports::toxoid::engine::ecs::Phases::OnStart,
            toxoid_component::component::ecs::Phases::OnLoad => toxoid_host::bindings::exports::toxoid::engine::ecs::Phases::OnLoad,
            toxoid_component::component::ecs::Phases::PostLoad => toxoid_host::bindings::exports::toxoid::engine::ecs::Phases::PostLoad,
            toxoid_component::component::ecs::Phases::PreUpdate => toxoid_host::bindings::exports::toxoid::engine::ecs::Phases::PreUpdate,
            toxoid_component::component::ecs::Phases::OnUpdate => toxoid_host::bindings::exports::toxoid::engine::ecs::Phases::OnUpdate,
            toxoid_component::component::ecs::Phases::OnValidate => toxoid_host::bindings::exports::toxoid::engine::ecs::Phases::OnValidate,
            toxoid_component::component::ecs::Phases::PostUpdate => toxoid_host::bindings::exports::toxoid::engine::ecs::Phases::PostUpdate,
            toxoid_component::component::ecs::Phases::PreStore => toxoid_host::bindings::exports::toxoid::engine::ecs::Phases::PreStore,
            toxoid_component::component::ecs::Phases::OnStore => toxoid_host::bindings::exports::toxoid::engine::ecs::Phases::OnStore,
            toxoid_component::component::ecs::Phases::Custom(entity) => toxoid_host::bindings::exports::toxoid::engine::ecs::Phases::Custom(entity),
        };
        phase.depends_on(dependency);
        Box::into_raw(phase);
    }

    fn get_id(&mut self, phase: Resource<toxoid_component::component::ecs::Phase>) -> u64 {
        let phase_proxy = self.table.get(&phase).unwrap() as &PhaseProxy;
        let phase = unsafe { Box::from_raw(phase_proxy.ptr) };
        let id = phase.get_id();
        Box::into_raw(phase);
        id
    }

    fn drop(&mut self, _phase: Resource<toxoid_component::component::ecs::Phase>) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

impl toxoid_component::component::ecs::HostPipeline for StoreState {
    fn new(&mut self, desc: toxoid_component::component::ecs::PipelineDesc) -> Resource<PipelineProxy> {
        let query_desc = toxoid_host::bindings::exports::toxoid::engine::ecs::QueryDesc { expr: desc.query_desc.expr };
        let pipeline = toxoid_host::Pipeline::new(toxoid_host::bindings::exports::toxoid::engine::ecs::PipelineDesc { 
            name: desc.name,
            query_desc: query_desc,
            phases: desc.phases
        });
        let ptr = Box::into_raw(Box::new(pipeline));
        let pipeline_resource = self.table.push(PipelineProxy { ptr }).unwrap();
        pipeline_resource
    }

    fn build(&mut self, pipeline: Resource<toxoid_component::component::ecs::Pipeline>) -> () {
        let pipeline_proxy = self.table.get(&pipeline).unwrap() as &PipelineProxy;
        let pipeline = unsafe { Box::from_raw(pipeline_proxy.ptr) };
        pipeline.build();
        Box::into_raw(pipeline);
    }

    fn add_phase(&mut self, pipeline: Resource<toxoid_component::component::ecs::Pipeline>, phase: EcsEntityT) -> () {
        let pipeline_proxy = self.table.get(&pipeline).unwrap() as &PipelineProxy;
        let pipeline = unsafe { Box::from_raw(pipeline_proxy.ptr) };
        pipeline.add_phase(phase);
        Box::into_raw(pipeline);
    }

    fn get_id(&mut self, pipeline: Resource<toxoid_component::component::ecs::Pipeline>) -> u64 {
        let pipeline_proxy = self.table.get(&pipeline).unwrap() as &PipelineProxy;
        let pipeline = unsafe { Box::from_raw(pipeline_proxy.ptr) };
        pipeline.get_id()
    }

    fn disable(&mut self, pipeline: Resource<toxoid_component::component::ecs::Pipeline>) -> () {
        let pipeline_proxy = self.table.get(&pipeline).unwrap() as &PipelineProxy;
        let pipeline = unsafe { Box::from_raw(pipeline_proxy.ptr) };
        pipeline.disable();
        Box::into_raw(pipeline);
    }

    fn enable(&mut self, pipeline: Resource<toxoid_component::component::ecs::Pipeline>) -> () {
        let pipeline_proxy = self.table.get(&pipeline).unwrap() as &PipelineProxy;
        let pipeline = unsafe { Box::from_raw(pipeline_proxy.ptr) };
        pipeline.enable();
        Box::into_raw(pipeline);
    }

    fn drop(&mut self, _pipeline: Resource<toxoid_component::component::ecs::Pipeline>) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

// Instantiate the WASM engine
pub static ENGINE: Lazy<Engine> = Lazy::new(|| { 
    // TODO: Base on debug flag
    Engine::new(
        Config::new()
            .debug_info(true)
            .cranelift_opt_level(OptLevel::None),   
    )
        .unwrap()
});

// Create WASM Component Linker
static LINKER: Lazy<Linker<StoreState>> = Lazy::new(|| {
    let engine = &*ENGINE; // Ensure ENGINE is initialized
    let mut linker = Linker::<StoreState>::new(engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap();
    ToxoidComponentWorld::add_to_linker(&mut linker, |store_state| store_state).unwrap();
    linker
});

fn new_store() -> Store<StoreState> {
    let engine = &*ENGINE; // Ensure ENGINE is initialized
    Store::new(
        engine,
        StoreState {
            ctx: WasiCtxBuilder::new()
                    // .inherit_stdin()
                    // .inherit_stdout()
                    // .inherit_stderr()
                    .inherit_stdio()
                    .inherit_env()
                    .inherit_args()
                    // .inherit_network()
                    .build(),
            table: ResourceTable::new(),
        }
    )
}

// Create WASM Store
pub static mut STORE: Lazy<Mutex<Store<StoreState>>> = Lazy::new(|| Mutex::new(new_store()));
// Crate lazy initialized option
pub static mut TOXOID_COMPONENT_WORLD: Option<ToxoidComponentWorld> = None;

// TODO: Add the ability to load multiple WASM components
// instead of overwriting a single guest WASM component
pub fn load_wasm_component(filename: &str) -> Result<()> {
    let engine = &*ENGINE;
    let linker = &*LINKER;

    // Acquire lock before manipulating store
    let mut store = unsafe { STORE.lock().unwrap() };

    unsafe {
        if let Some(world) = TOXOID_COMPONENT_WORLD.take() {
            drop(world);
        }
        
        SINGLETON_MAP.clear();
    }

    // Load the component from disk
    let bytes = std::fs::read(filename)?;
    let component = Component::new(&engine, bytes)?;
    
    unsafe { 
        TOXOID_COMPONENT_WORLD = Some(ToxoidComponentWorld::instantiate(&mut *store, &component, &linker)?);
        TOXOID_COMPONENT_WORLD.as_mut().unwrap().call_init(&mut *store)?;
    };

    Ok(())
}