// use toxoid_api_macro::Component;
use crate::*;
use core::ffi::c_void;
use core::alloc::{GlobalAlloc, Layout};
use core::any::TypeId;

#[repr(u8)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    String,
    Array,
    U32Array,
    F32Array,
}

pub trait ComponentTuple {
    fn get_type_ids() -> &'static [TypeId];
}

pub trait IsComponent {
    fn register() -> i32;
    fn set_ptr(&mut self, ptr: *mut c_void);
    fn get_ptr(&self) -> *mut c_void;
}

#[repr(C)]
pub struct Query {
    query: *mut c_void,
    iter: *mut c_void,
    indexes: &'static [TypeId],
}

impl Query {
    pub fn new<T: ComponentTuple + 'static>() -> Self {
        unsafe {
            let type_ids = T::get_type_ids();
            let layout = Layout::array::<i32>(type_ids.len()).unwrap();
            let ids_ptr = ALLOCATOR.alloc(layout) as *mut i32;
            type_ids
                .iter()
                .enumerate()
                .for_each(|(i, type_id)| {
                    let id = toxoid_component_cache_get(*type_id);
                    ids_ptr.add(i).write(id);
                });

            Query {
                query: toxoid_query_create(ids_ptr, type_ids.len() as i32),
                iter: core::ptr::null_mut(),  
                indexes: type_ids
            }
        }
    }

    pub fn iter(&mut self) -> &mut Query {
        self.iter = unsafe { toxoid_query_iter(self.query) };
        self
    }

    pub fn count(&self) -> i32 {
        unsafe { toxoid_iter_count(self.iter) }
    }

    pub fn next(&self) -> bool {
        unsafe { toxoid_query_next(self.iter) }
    }

    pub fn entities(&self) -> &[Entity] {
        unsafe { toxoid_query_entity_list(self.iter) }
    }

    pub fn field<T: Default + IsComponent + 'static>(&self) -> &'static [T] {
        unsafe {
            let count = toxoid_iter_count(self.iter);
            // let component_id = toxoid_component_cache_get(core::any::TypeId::of::<T>());
            // Get index of component type in query
            let type_id = TypeId::of::<T>();
            let mut term_index = 0;
            self.indexes.iter().enumerate().for_each(|(i, &x)| {
                if x == type_id {
                    // + 1 because of 1-based indexing for term index
                    term_index = i + 1;
                }
            });
            // Get slice of pointers to components
            let field_slice = toxoid_query_field_list(self.iter, term_index as i32, count as u32);
            // Call allocator to create a slice of component structs
            // TODO, look into making this Layout::array::<T>(count as usize).unwrap();
            // Without Emscripten crashing
            let layout = Layout::new::<T>();
            let components_ptr = ALLOCATOR.alloc(layout) as *mut T;
            field_slice
                .iter()
                .enumerate()
                .for_each(|(i, &component_ptr)| { 
                    let mut component = T::default();
                    component.set_ptr(component_ptr as *mut c_void);
                    components_ptr.add(i).write(component);
                });
            // Convert from pointer to slice
            let components = core::slice::from_raw_parts(components_ptr, count as usize);
            // Make sure slice is not freed at the end of this function
            core::mem::forget(components);
            // Return slice of components
            components
        }
    }

    pub fn each_mut<T: ComponentTuple>(&mut self, mut _func: impl FnMut(T)) {
        // Ensure we start from the beginning
        self.iter();
        while self.next() {
            let _type_ids = T::get_type_ids();
            // let components: T = T::get_type_ids()
            //     .iter()
            //     .map(|&id| {
            //         unsafe {
            //             let comp_ptr = toxoid_query_field_list(self.iter, id as i32, 1);
            //             let mut comp = <id's type>::default();
            //             comp.set_ptr(*comp_ptr as *mut c_void);
            //             comp
            //         }
            //     })
            
            // for i in 0...10 {
            //     func(components);
            // }
        }
    }
}

#[repr(C)]
pub struct System {
    pub query: Query,
    pub update_fn: fn(&mut Query)
}

pub trait SystemTrait {
    fn update(&mut self);
}

impl System {
    pub fn new<T: ComponentTuple + 'static>(update_fn: fn(&mut Query)) -> Self {
        // |self_f| {
        //     self_f.query.iter();
        //     while self_f.query.next() {
                
        //     }
        // }
        System {
            query: Query::new::<T>(),
            update_fn
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Entity {
    id: ecs_id_t,
}

impl Entity {
    pub fn new() -> Self {
        unsafe {
            Entity {
                id: toxoid_entity_create(),
            }
        }
    }

    pub fn add<T: IsComponent + 'static>(&mut self) {
        unsafe {
            let component_id = toxoid_component_cache_get(core::any::TypeId::of::<T>());
            toxoid_entity_add_component(self.id as u32, component_id as u32);
        }
    }

    pub fn add_id(&mut self, component: ecs_id_t) {
        unsafe {
            toxoid_entity_add_component(self.id as u32, component as u32);
        }
    }

    pub fn add_tag(&mut self, tag: ecs_entity_t) {
        unsafe {
            toxoid_entity_add_tag(self.id as u32, tag as u32);
        }
    }

    pub fn get_id(&self) -> ecs_entity_t {
        self.id
    }

    pub fn get<T: Default + IsComponent + 'static>(&self) -> T {
        unsafe {
            let mut component = T::default();
            let component_id = toxoid_component_cache_get(core::any::TypeId::of::<T>());
            let ptr = toxoid_entity_get_component(self.id as u32, component_id as u32);
            component.set_ptr(ptr);
            component
        }
    }

    pub fn child_of(&mut self, parent: Entity) {
        unsafe {
            toxoid_entity_child_of(self.id as u32, parent.get_id() as u32);
        }
    }

    pub fn add_child(&mut self, child: Entity) {
        unsafe {
            toxoid_entity_child_of(child.get_id() as u32, self.id as u32);
        }
    }
    
    pub fn children(&self) -> &[Entity] {
        unsafe {
            let iter = toxoid_entity_children(self.id as u32);
            toxoid_term_next(iter);

            let count = toxoid_iter_count(iter);
            let children = toxoid_child_entities(iter);
            let children_slice = core::slice::from_raw_parts(children, count as usize);

            let layout = Layout::array::<Entity>(count as usize).unwrap();
            let entities_ptr = ALLOCATOR.alloc(layout) as *mut Entity;
            children_slice
                .iter()
                .enumerate()
                .for_each(|(i, &entity_ptr)| { 
                    entities_ptr.add(i).write(Entity { id: entity_ptr as i32 });
                });
            let entities = core::slice::from_raw_parts(entities_ptr, count as usize);
            core::mem::forget(entities);
            entities
        }
    }
}

pub fn register_tag(name: &str) -> ecs_entity_t {
    unsafe { toxoid_register_tag(name.as_bytes().as_ptr() as *const i8, name.len()) }
}

pub fn register_component(name: &str, member_names: &[&str], member_types: &[u8]) -> ecs_entity_t {
    unsafe {
        let mut c_member_names: [*const c_char; MAX_ELEMENTS] = [core::ptr::null(); MAX_ELEMENTS];
        let mut c_member_names_len: [u8; MAX_ELEMENTS] = [0; MAX_ELEMENTS];
        for (i, &s) in member_names.iter().enumerate() {
            c_member_names[i] = s.as_ptr() as *const c_char;
            c_member_names_len[i] = s.len() as u8;
        }

        // TODO: Since this is Rust, it can probably just be the u8 value
        // instead of a pointer to the value.
        let mut c_member_types: [*const u8; MAX_ELEMENTS] = [core::ptr::null(); MAX_ELEMENTS];
        for (i, &t) in member_types.iter().enumerate() {
            c_member_types[i] = &t as *const u8;
        }

        toxoid_register_component(
            name.as_bytes().as_ptr() as *const c_char,
            name.len() as u8,
            c_member_names.as_ptr(),
            member_names.len() as u32,
            c_member_names_len.as_ptr(),
            c_member_types.as_ptr(),
            c_member_types.len() as u32,
        )
    }
}


macro_rules! count_args {
    ($($args:ident),*) => { <[()]>::len(&[$(count_args!(@substitute $args)),*]) };
    (@substitute $_t:tt) => { () };
}

macro_rules! impl_component_tuple {
    ($($name:ident)+) => {
        impl<$($name: Default + IsComponent + 'static),+> ComponentTuple for ($($name,)+) {
            #[allow(unused_assignments)]
            fn get_type_ids() -> &'static [TypeId] {
                unsafe {
                    let count = count_args!($($name),+);
                    let layout = Layout::array::<TypeId>(count).unwrap();
                    let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut TypeId;
                    let mut i = 0;
                    $(
                        type_ids_ptr.add(i).write(TypeId::of::<$name>());
                        i += 1;
                    )+
                    let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
                    core::mem::forget(type_ids);
                    type_ids
                }
            }
        }
    }
}

impl_component_tuple! { Component1 }
impl_component_tuple! { Component1 Component2 }
impl_component_tuple! { Component1 Component2 Component3 }
impl_component_tuple! { Component1 Component2 Component3 Component4 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 Component6 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 Component6 Component7 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 Component6 Component7 Component8 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 Component6 Component7 Component8 Component9 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 Component6 Component7 Component8 Component9 Component10 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 Component6 Component7 Component8 Component9 Component10 Component11 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 Component6 Component7 Component8 Component9 Component10 Component11 Component12 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 Component6 Component7 Component8 Component9 Component10 Component11 Component12 Component13 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 Component6 Component7 Component8 Component9 Component10 Component11 Component12 Component13 Component14 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 Component6 Component7 Component8 Component9 Component10 Component11 Component12 Component13 Component14 Component15 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 Component6 Component7 Component8 Component9 Component10 Component11 Component12 Component13 Component14 Component15 Component16 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 Component6 Component7 Component8 Component9 Component10 Component11 Component12 Component13 Component14 Component15 Component16 Component17 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 Component6 Component7 Component8 Component9 Component10 Component11 Component12 Component13 Component14 Component15 Component16 Component17 Component18 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 Component6 Component7 Component8 Component9 Component10 Component11 Component12 Component13 Component14 Component15 Component16 Component17 Component18 Component19 }
impl_component_tuple! { Component1 Component2 Component3 Component4 Component5 Component6 Component7 Component8 Component9 Component10 Component11 Component12 Component13 Component14 Component15 Component16 Component17 Component18 Component19 Component20 }


