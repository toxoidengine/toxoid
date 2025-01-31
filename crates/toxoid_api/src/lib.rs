#![allow(warnings)]
pub mod components;
pub use components::*;

// Native
#[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
pub use toxoid_host::{
    Component as ToxoidComponent,
    ComponentType as ToxoidComponentType,
    Entity as ToxoidEntity,
    Query as ToxoidQuery,
    System as ToxoidSystem,
    Callback as ToxoidCallback,
    Observer as ToxoidObserver,
    Iter as ToxoidIter,
    bindings::exports::toxoid::engine::ecs::{
        GuestComponent,
        GuestComponentType,
        GuestEntity,
        GuestQuery,
        GuestSystem,
        GuestObserver,
        GuestCallback,
        GuestIter,
    },
    ToxoidApi
};
#[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
pub use toxoid_host::bindings::exports::toxoid::engine::ecs::{
    EntityDesc,
    ComponentDesc,
    QueryDesc,
    SystemDesc,
    ObserverDesc,
    MemberType,
    Relationship,
    Event,
    Guest as WorldGuest,
};
// WASM
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
pub use toxoid_guest::bindings::{
    toxoid_component::component::ecs::{
        Component as ToxoidComponent,
        ComponentType as ToxoidComponentType,
        Entity as ToxoidEntity,
        Query as ToxoidQuery,
        System as ToxoidSystem,
        Observer as ToxoidObserver,
        Callback as ToxoidCallback,
        Iter as ToxoidIter,
        EntityDesc,
        ComponentDesc,
        QueryDesc,
        SystemDesc,
        ObserverDesc,
        MemberType,
        Relationship,
        Event,
        self as ToxoidApi
    },
    self,
    exports::toxoid_component::component::callbacks::Guest as CallbacksGuest,
    Guest as WorldGuest,
};
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
pub use toxoid_guest;
// Both (Native + WASM)
pub use toxoid_api_macro::component;

pub type ecs_entity_t = u64;

pub struct ToxoidWasmComponent;

pub trait ComponentType {
    fn get_name() -> &'static str;
    fn get_id() -> ecs_entity_t;
    fn register() -> ecs_entity_t;
}

pub trait Component {
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    fn set_component(&mut self, ptr: toxoid_guest::bindings::toxoid_component::component::ecs::Component);
    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
    fn set_component(&mut self, ptr: ToxoidComponent);
    fn set_entity_added(&mut self, entity_id: ecs_entity_t);
    fn set_component_type(&mut self, component_type_id: ecs_entity_t);
}

pub struct Entity {
    entity: ToxoidEntity
}

impl Entity {
    pub fn new(desc: Option<EntityDesc>) -> Self {
        let desc = desc.unwrap_or(EntityDesc { name: None });
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        let entity = ToxoidEntity::new(desc);
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        let entity = ToxoidEntity::new(&desc);
        Self { entity }
    }

    pub fn named(name: &str) -> Self {
        let desc = EntityDesc { name: Some(name.to_string()) };
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        let entity = ToxoidEntity::new(desc);
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        let entity = ToxoidEntity::new(&desc);
        Self { entity }
    }

    pub fn get_id(&self) -> u64 {
        self.entity.get_id()
    }

    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
    pub fn from_id(id: u64) -> Self {
        Self { entity: unsafe { *Box::from_raw(ToxoidEntity::from_id(id) as usize as *mut ToxoidEntity) } }
    }

    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub fn from_id(id: u64) -> Self {
        Self { entity: ToxoidEntity::from_id(id) }
    }

    pub fn get_name(&self) -> String {
        self.entity.get_name()
    }

    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
    pub fn set_name(&mut self, name: String) {
        self.entity.set_name(name);
    }

    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub fn set_name(&mut self, name: String) {
        self.entity.set_name(name.as_str());
    }

    pub fn get<T: Component + ComponentType + Default + 'static>(&mut self) -> T {
        let mut component = T::default();
        let component_ptr = self.entity.get(T::get_id());
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        let toxoid_component = ToxoidComponent::new(component_ptr, self.entity.get_id(), T::get_id());
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        let toxoid_component = component_ptr;
        component.set_component(toxoid_component);
        component.set_entity_added(self.entity.get_id());
        component.set_component_type(T::get_id());
        component
    }

    pub fn add<T: Component + ComponentType + 'static>(&mut self) -> &Self {
        let component_id = T::get_id();
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        self.entity.add(component_id);
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        self.entity.add(component_id);
        self
    }

    pub fn add_relationship(&mut self, relationship: Relationship, target: Entity) {
        self.entity.add_relationship(relationship, target.get_id());
    }

    pub fn remove_relationship(&mut self, relationship: Relationship, target: Entity) {
        self.entity.remove_relationship(relationship, target.get_id());
    }

    pub fn remove<T: Component + ComponentType + 'static>(&mut self) {  
        self.entity.remove(T::get_id());
    }

    pub fn parent_of(&mut self, target: Entity) {
        self.entity.parent_of(target.get_id());
    }

    pub fn child_of(&mut self, target: Entity) {
        self.entity.child_of(target.get_id());
    }

    pub fn parent_of_id(&mut self, target: ecs_entity_t) {
        self.entity.parent_of(target);
    }

    pub fn child_of_id(&mut self, target: ecs_entity_t) {
        self.entity.child_of(target);
    }

    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
    pub fn parent(&self) -> Entity {        
        Self { entity: unsafe { *Box::from_raw(ToxoidEntity::from_id(self.entity.parent()) as usize as *mut ToxoidEntity) } }
    }

    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub fn parent(&self) -> Entity {
        Self { entity: ToxoidEntity::from_id(self.entity.parent().get_id()) }
    }

    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
    pub fn children(&self) -> Vec<Entity> {
        self.entity.children().iter().map(|child| Entity { entity: ToxoidEntity { id: *child } }).collect()
    }

    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub fn children(&self) -> Vec<Entity> {
        return self
            .entity
            .children()
            .iter()
            .map(|child| Entity { entity: ToxoidEntity::from_id(child.get_id()) })
            .collect();
    }

    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
    pub fn relationships(&self) -> Vec<Entity> {
        self
            .entity
            .relationships()
            .iter()
            .map(|relationship| Entity { entity: ToxoidEntity { id: *relationship } })
            .collect()
    }

    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub fn relationships(&self) -> Vec<Entity> {
        return self.entity.relationships().iter().map(|relationship| Entity { entity: ToxoidEntity::from_id(relationship.get_id()) }).collect();
    }
}

pub struct Query {
    query: ToxoidQuery
}

impl Query {
    pub fn new(desc: Option<QueryDesc>) -> Self {
        let desc = desc.unwrap_or(QueryDesc { expr: "".to_string() });
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        let query = ToxoidQuery::new(desc);
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        let query = ToxoidQuery::new(&desc);
        Self { query }
    }

    pub fn dsl(dsl: &str) -> Self {
        let desc = QueryDesc { expr: dsl.to_string() };
        Self::new(Some(desc))
    }

    pub fn build(&mut self) {
        self.query.build();
    }

    pub fn iter(&mut self) {
        self.query.iter();
    }

    pub fn next(&mut self) {
        self.query.next();
    }

    pub fn count(&self) -> i32 {
        self.query.count()
    }

    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
    pub fn entities(&self) -> Vec<Entity> {
        unimplemented!()
    }

    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub fn entities(&self) -> Vec<Entity> {
        self
            .query
            .entities()
            .iter()
            .map(|entity| Entity { 
                entity: ToxoidEntity::from_id(entity.get_id()) 
            })
            .collect()
    }
}

pub struct System {
    system: ToxoidSystem
}

impl System {
    // #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
    // pub fn new(desc: Option<SystemDesc>) -> Self {
    //     unimplemented!()
    // }

    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub fn new(desc: Option<SystemDesc>, callback_fn: fn(&Iter)) -> Self {
        // Register the callback in the guest environment
        let callback = Callback::new(callback_fn);
        // Create the Toxoid callback with the registered callback handle
        let callback = ToxoidCallback::new(callback.cb_handle());
        let query_desc = desc.as_ref().unwrap().query_desc.clone();
        let desc = desc.unwrap_or(SystemDesc { 
            name: None, 
            callback, 
            query_desc, 
            is_guest: true, 
            tick_rate: None
        });
        Self { system: ToxoidSystem::new(desc) }
    }

    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
    pub fn new(desc: Option<SystemDesc>, callback_fn: fn(&Iter)) -> Self {
        // Register the callback in the guest environment
        let callback = Callback::new(callback_fn);
        // Create the Toxoid callback with the registered callback handle
        let callback = ToxoidCallback::new(callback.cb_handle());
        let query_desc = desc.as_ref().unwrap().query_desc.clone();
        let desc = desc.unwrap_or(SystemDesc { 
            name: None, 
            callback: callback.cb_handle(), 
            query_desc, 
            is_guest: false, 
            tick_rate: None 
        });
        Self { system: ToxoidSystem::new(desc) }
    }

    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub fn dsl(dsl: &str, tick_rate: Option<i32>, callback_fn: fn(&Iter)) -> Self {
        // Register the callback in the guest environment
        let callback = Callback::new(callback_fn);
        // Create the Toxoid callback with the registered callback handle
        let callback = ToxoidCallback::new(callback.cb_handle());
        let desc = SystemDesc { 
            // TODO: Only do this in dev mode with hot reload enabled
            // TODO: Map something other than the query string to the system name
            // so that multiple systems can have the same query on
            // hot reload
            name: Some(dsl.to_string()), 
            callback, 
            is_guest: true, 
            query_desc: QueryDesc { expr: dsl.to_string() },
            tick_rate
        };
        Self { system: ToxoidSystem::new(desc) }
    }

    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
    pub fn dsl(dsl: &str, tick_rate: Option<i32>, callback_fn: fn(&Iter)) -> Self {
        // Register the callback in the guest environment
        let callback = Callback::new(callback_fn);
        let desc = SystemDesc { 
            name: None, 
            callback: callback.cb_handle(), 
            query_desc: QueryDesc { expr: dsl.to_string() }, 
            is_guest: false,
            tick_rate
        };
        Self { system: ToxoidSystem::new(desc) }
    }

    pub fn build(&mut self) {
        self.system.build();
    }
}

pub struct Observer {
    observer: ToxoidObserver
}

impl Observer {
    // Not wasm
    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
    pub fn new(desc: Option<ObserverDesc>, callback_fn: fn(&Iter)) -> Self {
        let callback = Callback::new(callback_fn);
        let desc = desc.unwrap_or(ObserverDesc { 
            name: None, 
            query_desc: QueryDesc { expr: "".to_string() }, 
            events: vec![], 
            callback: callback.cb_handle(), 
            is_guest: false 
        });
        Self { observer: ToxoidObserver::new(desc) }
    }

    // WASM
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub fn new(desc: Option<ObserverDesc>, callback_fn: fn(&Iter)) -> Self {
        // Register the callback in the guest environment
        let callback = Callback::new(callback_fn);
        // Create the Toxoid callback with the registered callback handle
        let callback = ToxoidCallback::new(callback.cb_handle());
        let query_desc = desc.as_ref().unwrap().query_desc.clone();
        let events = desc.as_ref().unwrap().events.clone();
        let desc = desc.unwrap_or(ObserverDesc { 
            name: None, 
            callback, 
            query_desc, 
            events, 
            is_guest: true 
        });
        Self { observer: ToxoidObserver::new(desc) }
    }

    // Not wasm
    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
    pub fn dsl(dsl: &str, events: Vec<Event>, callback_fn: fn(&Iter)) -> Self {
        // TODO: Make this work in WASM without using toxoid_host::get_event
        let callback = Callback::new(callback_fn);
        let desc = ObserverDesc { 
            name: None, 
            query_desc: QueryDesc { expr: dsl.to_string() }, 
            events, 
            callback: callback.cb_handle(), 
            is_guest: false
        };
        Self { observer: ToxoidObserver::new(desc) }
    }

    // WASM
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub fn dsl(dsl: &str, events: Vec<Event>, callback_fn: fn(&Iter)) -> Self {
        // Register the callback in the guest environment
        let callback = Callback::new(callback_fn);
        // Create the Toxoid callback with the registered callback handle
        let callback = ToxoidCallback::new(callback.cb_handle());
        let desc = ObserverDesc { 
            name: None, 
            callback, 
            is_guest: true, 
            query_desc: QueryDesc { expr: dsl.to_string() },
            events
        };
        Self { observer: ToxoidObserver::new(desc) }
    }

    pub fn build(&mut self) {
        self.observer.build();
    }

    // Not wasm
    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]      
    pub fn callback(&self) -> u64 {
        self.observer.callback()
    }

    // WASM
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    pub fn callback(&self) -> i64 {
        unimplemented!()
    }
}

pub struct Callback {
    callback: ToxoidCallback,
}

pub static mut CALLBACKS: once_cell::sync::Lazy<Vec<Box<dyn Fn(&Iter)>>> = once_cell::sync::Lazy::new(|| Vec::new());

impl Callback {
    pub fn new(callback_fn: fn(&Iter)) -> Self {
        let handle = unsafe { CALLBACKS.push(Box::new(callback_fn)); CALLBACKS.len() - 1 };
        Self { callback: ToxoidCallback::new(handle as u64) }   
    }

    pub fn run(&self, iter: &Iter) {
        let callback = unsafe { CALLBACKS[self.callback.cb_handle() as usize].as_ref() };
        callback(iter);
    }

    pub fn cb_handle(&self) -> u64 {
        self.callback.cb_handle()
    }
}

pub struct Iter {
    pub iter: ToxoidIter
}

impl Iter {
    pub fn new(iter: ToxoidIter) -> Self {
        Self { iter }
    }

    pub fn next(&mut self) {
        self
            .iter
            .next();
    }

    pub fn count(&self) -> i32 {
        self
            .iter
            .count()
    }

    pub fn entities(&self) -> Vec<Entity> {
        self.iter
            .entities()
            .iter()
            .map(|entity| {
                #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
                // In native mode, we get a u64 ID directly
                return Entity {
                    entity: ToxoidEntity { id: *entity }
                };
                #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
                // In WASM mode, we're working with the guest component object
                return Entity {
                    entity: ToxoidEntity::from_id(entity.get_id())
                };
            })
            .collect()
    }
}

pub struct World;

impl World {
    pub fn add_singleton<T: Component + ComponentType + 'static>() {
        ToxoidApi::add_singleton(T::get_id())
    }

    pub fn get_singleton<T: Component + ComponentType + Default + 'static>() -> T {
        let mut component = T::default();
        let component_id = T::get_id();
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        let component_ptr = ToxoidApi::get_singleton(component_id);
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        let component_resource = ToxoidApi::get_singleton(component_id);
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        let toxoid_component = ToxoidComponent::new(component_ptr, component_id, component_id);
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        component.set_component(toxoid_component);
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        component.set_component(component_resource);
        component
    }

    pub fn remove_singleton<T: Component + ComponentType + 'static>() {
        let component_id = T::get_id();
        ToxoidApi::remove_singleton(component_id);
    }

    pub fn get_entity(entity_id: u64) -> Entity {
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        return Entity { entity: ToxoidEntity { id: entity_id } };
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        return Entity { entity: ToxoidEntity::from_id(entity_id) };
    }

    pub fn remove_entity(entity_id: u64) {
        ToxoidApi::remove_entity(entity_id);
    }

    pub fn has_entity_named(name: String) -> bool {
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        return ToxoidApi::has_entity_named(name);
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        return ToxoidApi::has_entity_named(name.as_str());
    }
}

pub fn run_callback(iter: ToxoidIter, handle: u64) {
    let iter = Iter::new(iter);
    let callback = unsafe { CALLBACKS[handle as usize].as_ref() };
    callback(&iter);
}

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
pub fn register_component(component_name: &str, member_names: Vec<String>, member_types: Vec<u8>) -> ecs_entity_t {
    let component_type = ToxoidComponentType::new(&ComponentDesc {
        name: component_name.to_string(),
        member_names,
        member_types,
    });
    component_type.get_id()
}

#[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
pub fn register_component(component_name: &str, member_names: Vec<String>, member_types: Vec<u8>) -> ecs_entity_t {
    let component_type = ToxoidComponentType::new(ComponentDesc {
        name: component_name.to_string(),
        member_names,
        member_types,
    });
    component_type.get_id()
}

pub fn get_component_id(component_name: &str) -> ecs_entity_t {
    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
    return ToxoidApi::get_component_id(component_name);
    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
    return ToxoidApi::get_component_id(component_name.to_string());
}

// Fetch assets / resources from the asset server or local file system
pub fn fetch(path: &str) {
    let mut entity = Entity::new(None);
    entity.add::<FetchRequest>();
    let mut fetch_request = entity.get::<FetchRequest>();
    fetch_request.set_path(path.to_string());
    entity.add::<Loading>();
}