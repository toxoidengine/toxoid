#![allow(unused_must_use)]
bindgen!({
    world: "toxoid-component-world",
    path: "../toxoid_guest/wit",
    with: {
        // Specify that our host resource is going to point to the `ComponentProxy`
        "toxoid-component:component/ecs/component-type": ComponentTypeProxy,
        "toxoid-component:component/ecs/component": ComponentProxy,
        "toxoid-component:component/ecs/entity": EntityProxy,
        "toxoid-component:component/ecs/query": QueryProxy,
        "toxoid-component:component/ecs/system": SystemProxy,
        "toxoid-component:component/ecs/callback": CallbackProxy,
        "toxoid-component:component/ecs/iter": IterProxy,
        "toxoid-component:component/ecs/observer": ObserverProxy,
    },
});

use std::collections::HashMap;
use toxoid_api::GuestObserver;
use toxoid_host::bindings::exports::toxoid::engine::ecs::{Guest, GuestCallback, GuestComponent, GuestComponentType, GuestEntity, GuestIter, GuestQuery, GuestSystem};
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
}

impl toxoid_component::component::ecs::HostIter for StoreState {
    fn new(&mut self, ptr: i64) -> Resource<IterProxy> {
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

    fn drop(&mut self, _iter: Resource<IterProxy>) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

impl toxoid_component::component::ecs::HostCallback for StoreState {
    fn new(&mut self, handle: i64) -> Resource<CallbackProxy> {
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

    fn cb_handle(&mut self, _callback: Resource<toxoid_component::component::ecs::Callback>) -> i64 {
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
    fn new(&mut self, desc: toxoid_component::component::ecs::EntityDesc) -> Resource<EntityProxy> {
        let entity = toxoid_host::Entity::new(toxoid_host::bindings::exports::toxoid::engine::ecs::EntityDesc {
            name: desc.name,
        });
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

    fn remove(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, component: toxoid_component::component::ecs::EcsEntityT) -> () {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        entity.remove(component);
        Box::into_raw(entity);
    }

    fn add_relationship(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, relationship: toxoid_component::component::ecs::EcsEntityT, target: toxoid_component::component::ecs::EcsEntityT) -> () {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        entity.add_relationship(relationship, target);
        Box::into_raw(entity);
    }

    fn remove_relationship(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, relationship: toxoid_component::component::ecs::EcsEntityT, target: toxoid_component::component::ecs::EcsEntityT) -> () {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        entity.remove_relationship(relationship, target);
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
    fn new(&mut self, component_ptr: i64, entity_id: u64, component_type_id: u64) -> Resource<ComponentProxy> {
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

    fn expr(&mut self, query: Resource<toxoid_component::component::ecs::Query>, expr: String) -> () {
        let query_proxy = self.table.get(&query).unwrap() as &QueryProxy;
        let query = unsafe { Box::from_raw(query_proxy.ptr) };
        query.expr(expr);
        Box::into_raw(query);
    }

    fn build(&mut self, query: Resource<toxoid_component::component::ecs::Query>) -> () {
        let query_proxy = self.table.get(&query).unwrap() as &QueryProxy;
        let query = unsafe { Box::from_raw(query_proxy.ptr) };
        query.build();
        Box::into_raw(query);
    }

    fn iter(&mut self, query: Resource<toxoid_component::component::ecs::Query>) -> () {
        let query_proxy = self.table.get(&query).unwrap() as &QueryProxy;
        let query = unsafe { Box::from_raw(query_proxy.ptr) };
        query.iter();
        Box::into_raw(query);
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
        query.count()
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

    fn drop(&mut self, _query: Resource<toxoid_component::component::ecs::Query>) -> Result<(), wasmtime::Error> {
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