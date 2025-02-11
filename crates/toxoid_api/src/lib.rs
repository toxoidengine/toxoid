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
    Phase as ToxoidPhase,
    Pipeline as ToxoidPipeline,
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
        GuestPhase,
        GuestPipeline,
        EntityDesc,
        ComponentDesc,
        QueryDesc,
        SystemDesc,
        ObserverDesc,
        SortingDesc,
        MemberType,
        Relationship,
        Event,
        Guest as WorldGuest,
        EcsEntityT,
        PointerT
    },
    ToxoidApi
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
        Phase as ToxoidPhase,
        Pipeline as ToxoidPipeline,
        EntityDesc,
        ComponentDesc,
        QueryDesc,
        SystemDesc,
        ObserverDesc,
        SortingDesc,
        MemberType,
        Relationship,
        Event,
        EcsEntityT,
        PointerT,
        self as ToxoidApi
    },
    self,
    exports::toxoid_component::component::callbacks::Guest as CallbacksGuest,
    Guest as WorldGuest,
};
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
pub use toxoid_guest;
// Both (Native + WASM)
pub use toxoid_api_macro::{component, components};

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

pub struct Query {
    query: ToxoidQuery
}

pub struct System {
    system: ToxoidSystem
}

pub struct Observer {
    observer: ToxoidObserver
}

pub struct Callback {
    callback: ToxoidCallback,
}

pub struct Iter {
    pub iter: ToxoidIter
}

pub struct Phase {
    phase: ToxoidPhase
}

pub struct Pipeline {
    pipeline: ToxoidPipeline
}

pub struct World;

pub static mut CALLBACKS: once_cell::sync::Lazy<Vec<Box<dyn Fn(&Iter)>>> = once_cell::sync::Lazy::new(|| Vec::new());

impl Entity {
    pub fn new(desc: Option<EntityDesc>) -> Self {
        let desc = desc.unwrap_or(EntityDesc { name: None, add: None, prefab: false });
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        let entity = ToxoidEntity::new(desc, None);
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        let entity = ToxoidEntity::new(&desc, None);
        Self { entity }
    }

    pub fn named(name: &str) -> Self {
        let desc = EntityDesc { name: Some(name.to_string()), add: None, prefab: false };
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        let entity = ToxoidEntity::new(desc, None);
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        let entity = ToxoidEntity::new(&desc, None);
        Self { entity }
    }

    pub fn from_prefab(desc: Option<EntityDesc>, prefab: Entity) -> Self {
        let desc = desc.unwrap_or(EntityDesc { name: None, add: None, prefab: false });
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        let entity = ToxoidEntity::new(desc, Some(prefab.get_id()));
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        let entity = ToxoidEntity::new(&desc, Some(prefab.get_id()));
        Self { entity }
    }

    pub fn from_prefab_id(desc: Option<EntityDesc>, prefab: EcsEntityT) -> Self {
        let desc = desc.unwrap_or(EntityDesc { name: None, add: None, prefab: false });
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        let entity = ToxoidEntity::new(desc, Some(prefab));
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        let entity = ToxoidEntity::new(&desc, Some(prefab));
        Self { entity }
    }

    pub fn prefab() -> Self {
        let desc = EntityDesc { name: None, add: Some(vec![]), prefab: true };
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        let entity = ToxoidEntity::new(desc, None);
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        let entity = ToxoidEntity::new(&desc, None);
        Self { entity }
    }

    pub fn prefab_named(name: &str) -> Self {
        let desc = EntityDesc { name: Some(name.to_string()), add: Some(vec![]), prefab: true };
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        let entity = ToxoidEntity::new(desc, None);
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        let entity = ToxoidEntity::new(&desc, None);
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

    pub fn has<T: Component + ComponentType + 'static>(&self) -> bool {
        self.entity.has(T::get_id())
    }

    pub fn remove<T: Component + ComponentType + 'static>(&mut self) {  
        self.entity.remove(T::get_id());
    }

    pub fn add_relationship(&mut self, relationship: Relationship, target: Entity) {
        self.entity.add_relationship(relationship, target.get_id());
    }

    pub fn remove_relationship(&mut self, relationship: Relationship, target: Entity) {
        self.entity.remove_relationship(relationship, target.get_id());
    }
    
    pub fn is_a(&self, target: Entity) {
        self.entity.add_relationship(Relationship::IsA, target.get_id());
    }

    pub fn is_a_id(&mut self, target: ecs_entity_t) {
        self.entity.add_relationship(Relationship::IsA, target);
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

    pub fn disable(&mut self) {
        self.entity.disable();
    }

    pub fn enable(&mut self) {
        self.entity.enable();
    }
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

    pub fn dsl_each<F>(dsl: &str, mut f: F) -> Self 
    where 
        F: FnMut(&mut Query)
    {
        let mut query = Self::new(Some(QueryDesc { expr: dsl.to_string() }));
        query.build();
        let mut iter = query.iter();
        while query.next() {
            f(&mut query);
        }
        query
    }

    pub fn build(&mut self) {
        self.query.build();
    }

    // TODO: Use query trampoline instead of C functions directly and use callback resource to make this work on WASM.
    // pub fn order_by(&mut self, id: EcsEntityT, callback: unsafe fn(u64, *const std::ffi::c_void, u64, *const std::ffi::c_void) -> i32) {
    //     let sorting = SortingDesc { id, callback: unsafe { std::mem::transmute(callback) } };
    //     self.query.order_by(sorting);
    // }

    pub fn iter(&mut self) -> Iter {
        #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
        let iter = ToxoidIter::new(self.query.iter());
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        let iter = self.query.iter();
        Iter { iter }
    }

    pub fn next(&mut self) -> bool {
        self.query.next()
    }

    pub fn count(&self) -> i32 {
        self.query.count()
    }

    pub fn entities(&self) -> Vec<Entity> {
        self
            .query
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

    // pub fn field(&self, index: i8) -> Vec<u64> {
    //     self.query.field(index)
    // }

    pub fn components<T: Component + ComponentType + Default + 'static>(&self, index: i8) -> Vec<T> {
        let field_raw_ptrs = self.query.components(index);
        let components = field_raw_ptrs
            .iter()
            .map(|component_ptr| {
                let mut component = T::default();
                let toxoid_component = ToxoidComponent::new(*component_ptr, 0, T::get_id());
                component.set_component(toxoid_component);
                component.set_component_type(T::get_id());
                component
            })
            .collect();
        components
    }
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

    pub fn get_id(&self) -> ecs_entity_t {
        self.system.get_id()
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
            // TODO: Only do this in dev mode with hot reload enabled
            // TODO: Map something other than the query string to the system name
            // so that multiple systems can have the same query on
            // hot reload
            name: Some(dsl.to_string()), 
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

    // TODO: Use query trampoline instead of C functions directlyand use callback resource to make this work on WASM.
    #[cfg(not(target_os = "emscripten"))]
    pub fn order_by(&mut self, id: EcsEntityT, callback: unsafe extern "C" fn(u64, *const std::ffi::c_void, u64, *const std::ffi::c_void) -> i32) {
        let sorting = SortingDesc { id, callback: unsafe { std::mem::transmute(callback) } };
        self.system.order_by(sorting);
    }

    #[cfg(target_os = "emscripten")]
    pub fn order_by(&mut self, id: EcsEntityT, callback: unsafe extern "C" fn(u64, *const std::ffi::c_void, u64, *const std::ffi::c_void) -> i32) {
        let callback_ptr: u32 = unsafe { std::mem::transmute(callback as u32) };
        let sorting = SortingDesc { id, callback: callback_ptr as u64 };
        self.system.order_by(sorting);
    }

    pub fn disable(&mut self) {
        self.system.disable();
    }

    pub fn enable(&mut self) {
        self.system.enable();
    }
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
        unimplemented!("Callback for observer not implemented on WASM");
    }
}

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

impl Iter {
    pub fn new(iter: ToxoidIter) -> Self {
        Self { iter }
    }

    pub fn next(&mut self) -> bool {
        self
            .iter
            .next()
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

    pub fn components<T: Component + ComponentType + Default + 'static>(&self, index: i8) -> Vec<T> {
        let field_raw_ptrs = self.iter.components(index);
        let components = field_raw_ptrs
            .iter()
            .map(|component_ptr| {
                let mut component = T::default();
                #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))] {
                    let toxoid_component = ToxoidComponent::from_ptr_host(*component_ptr);
                    component.set_component(toxoid_component);
                    component.set_component_type(T::get_id());
                    component
                }
                #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))] {
                    let toxoid_component = ToxoidComponent::from_ptr(*component_ptr);
                    component.set_component(toxoid_component);
                    component.set_component_type(T::get_id());
                    component
                }
            })
            .collect();
        components
    }
}

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

#[repr(u8)]
pub enum DataType {
    Raw,
    Image,
    Sprite,
    BoneAnimationAtlas,
    BoneAnimationSkeleton,
    Worldmap,
    Cell,
    Tileset,
    Audio,
    Font
}

#[repr(u32)]
pub enum ZDepth {
    UndergroundLayer,
    BottomLayer,
    BelowPlayer,
    SameAsPlayer,
    AbovePlayer,
    DarknessLayer,
    LightLayer,
    SkyLayer,
    UILayer
}

#[repr(u8)]
pub enum DirectionEnum {
    Up,
    Down,
    Left,
    Right
}

// Fetch assets / resources from the asset server or local file system
pub fn fetch(path: &str, data_type: DataType, user_data: Option<u64>) {
    let mut entity = Entity::new(None);
    entity.add::<FetchRequest>();
    let mut fetch_request = entity.get::<FetchRequest>();
    fetch_request.set_path(path.to_string());
    fetch_request.set_data_type(data_type as u8);
    if let Some(user_data) = user_data {
        fetch_request.set_user_data(user_data);
    }
    entity.add::<Loading>();
    // println!("Fetch request data type: {}", fetch_request.get_data_type());
    // println!("Entity ID origin: {}", entity.get_id());
}

pub fn load_sprite(path: &str) -> Entity {
    let mut entity = Entity::new(None);
    entity.add::<Sprite>();
    entity.add::<Position>();
    entity.add::<Size>();
    fetch(path, DataType::Sprite, Some(entity.get_id()));
    entity
}

pub fn load_animation(atlas_filename: &str, skeleton_filename: &str) -> Entity {
    let mut entity = Entity::new(None);
    entity.add::<Loading>();
    entity.add::<Skeleton>();
    entity.add::<BoneAnimation>();
    entity.add::<Atlas>();
    entity.add::<Images>();
    entity.add::<Position>();
    entity.add::<Size>();

    let mut atlas = entity.get::<Atlas>();
    atlas.set_filename(atlas_filename.to_string());
    let mut skeleton = entity.get::<Skeleton>();
    skeleton.set_filename(skeleton_filename.to_string());
    fetch(atlas_filename, DataType::BoneAnimationAtlas, Some(entity.get_id()));
    fetch(skeleton_filename, DataType::BoneAnimationSkeleton, Some(entity.get_id()));
    entity
}

pub fn load_worldmap(path: &str) -> Entity {
    let mut entity = Entity::new(None);
    entity.add::<TiledWorld>();
    entity.add::<Size>();
    fetch(path, DataType::Worldmap, Some(entity.get_id()));
    entity
}

pub fn load_cell(path: &str) -> Entity {
    // let cell_index = path.split("cell_")
    //     .nth(1)
    //     .and_then(|s| s.split(".").next())
    //     .and_then(|s| s.parse::<u32>().ok())
    //     .unwrap_or(0);
    let mut entity = Entity::new(None);
    entity.add::<TiledCell>();
    entity.add::<Size>();
    fetch(path, DataType::Cell, Some(entity.get_id()));
    entity
}

pub fn load_tileset(path: &str) -> Entity {
    let mut entity = Entity::new(None);
    entity.add::<Tileset>();
    entity.add::<Sprite>();
    entity.add::<Size>();
    fetch(path, DataType::Tileset, Some(entity.get_id()));
    entity
}