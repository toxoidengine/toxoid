// use toxoid_api_macro::Component;
use crate::*;
use core::ffi::c_void;
use core::alloc::{GlobalAlloc, Layout};
use serde::{Serialize, Deserialize};
use bindings::*;

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

pub const MAX_ELEMENTS: usize = 100;

pub fn default_ptr() -> *mut c_void {
    core::ptr::null_mut()
}

pub fn default_string() -> *mut i8 {
    core::ptr::null_mut()
}

#[derive(Serialize, Deserialize)]
pub struct Pointer {
    #[serde(skip, default = "default_ptr")]
    pub ptr: *mut c_void
}

impl Pointer {
    pub fn new(ptr: *mut c_void) -> Self {
        Self {
            ptr
        }
    }
}

impl Default for Pointer {
    fn default() -> Self {
        Self {
            ptr: core::ptr::null_mut()
        }
    }
}

impl Clone for Pointer {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr
        }
    }
}

impl Copy for Pointer {}

impl PartialEq for Pointer {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

#[derive(Serialize, Deserialize)]
pub struct StringPtr {
    #[serde(skip, default = "default_string")]
    pub ptr: *mut c_char
}

impl StringPtr {
    pub fn new(rust_str: &str) -> Self {
        Self {
            ptr: unsafe { toxoid_make_c_string(rust_str) }
        }
    }
}

impl Default for StringPtr {
    fn default() -> Self {
        Self {
            ptr: core::ptr::null_mut()
        }
    }
}

impl Clone for StringPtr {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr
        }
    }
}

impl Copy for StringPtr {}

impl PartialEq for StringPtr {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

pub struct U32Array {
    pub ptr: *mut u32,
    pub len: u32,
}

pub struct F32Array {
    pub ptr: *mut f32,
    pub len: u32,
}

impl U32Array {
    pub fn new(ptr: *mut u32, len: u32) -> Self {
        Self {
            ptr,
            len,
        }
    }

    pub fn from_slice(slice: &[u32]) -> Self {
        let layout = Layout::array::<u32>(slice.len()).unwrap();
        let ptr = unsafe { ALLOCATOR.alloc(layout) as *mut u32 };
        unsafe {
            ptr.copy_from_nonoverlapping(slice.as_ptr(), slice.len());
        }
        Self {
            ptr,
            len: slice.len() as u32,
        }
    }

    pub fn from_raw(ptr: *mut u32, len: u32) -> Self {
        Self {
            ptr,
            len,
        }
    }

    pub fn from_array(array: [u32; MAX_ELEMENTS]) -> Self {
        let layout = Layout::array::<u32>(MAX_ELEMENTS).unwrap();
        let ptr = unsafe { ALLOCATOR.alloc(layout) as *mut u32 };
        unsafe {
            ptr.copy_from_nonoverlapping(array.as_ptr(), array.len());
        }
        Self {
            ptr,
            len: array.len() as u32,
        }
    }

    pub fn create_array(_len: u32) -> *mut u32 {
        let mut arr = [0u32; MAX_ELEMENTS];
        arr.as_mut_ptr()
    }

    pub fn from_array_any() -> Self {
        let mut arr = [0u32; MAX_ELEMENTS];
        let ptr = arr.as_mut_ptr();
        Self {
            ptr,
            len: MAX_ELEMENTS as u32,
        }
    }

    pub fn create(len: u32) -> Self {
        let layout = Layout::array::<u32>(len as usize).unwrap();
        let ptr = unsafe { ALLOCATOR.alloc(layout) as *mut u32 };
        Self {
            ptr,
            len,
        }
    }

    pub fn create_raw(len: u32) -> *mut u32 {
        unsafe { ALLOCATOR.alloc(Layout::array::<u32>(len as usize).unwrap()) as *mut u32 }
    }
}

pub fn create_raw(len: u32) -> *mut u32 {
    unsafe { ALLOCATOR.alloc(Layout::array::<u32>(len as usize).unwrap()) as *mut u32 }
}

impl Clone for U32Array {
    fn clone(&self) -> Self {
        let slice = unsafe { std::slice::from_raw_parts(self.ptr, self.len as usize) };
        Self::from_slice(slice)
    }
}

impl Copy for U32Array {}

impl PartialEq for U32Array {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        unsafe {
            let self_slice = std::slice::from_raw_parts(self.ptr, self.len as usize);
            let other_slice = std::slice::from_raw_parts(other.ptr, other.len as usize);
            self_slice == other_slice
        }
    }
}

impl Default for U32Array {
    fn default() -> Self {
        Self {
            ptr: core::ptr::null_mut(),
            len: 0,
        }
    }
}


impl F32Array {
    pub fn new(ptr: *mut f32, len: u32) -> Self {
        Self {
            ptr,
            len,
        }
    }

    pub fn from_slice(slice: &[f32]) -> Self {
        let layout = Layout::array::<f32>(slice.len()).unwrap();
        let ptr = unsafe { ALLOCATOR.alloc(layout) as *mut f32 };
        unsafe {
            ptr.copy_from_nonoverlapping(slice.as_ptr(), slice.len());
        }
        Self {
            ptr,
            len: slice.len() as u32,
        }
    }

    pub fn from_raw(ptr: *mut f32, len: u32) -> Self {
        Self {
            ptr,
            len,
        }
    }
}

impl Clone for F32Array {
    fn clone(&self) -> Self {
        let slice = unsafe { std::slice::from_raw_parts(self.ptr, self.len as usize) };
        Self::from_slice(slice)
    }
}

impl Copy for F32Array {}

impl PartialEq for F32Array {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        unsafe {
            let self_slice = std::slice::from_raw_parts(self.ptr, self.len as usize);
            let other_slice = std::slice::from_raw_parts(other.ptr, other.len as usize);
            self_slice == other_slice
        }
    }
}

pub trait ComponentTuple {
    fn get_type_ids() -> &'static [u64];
}

pub trait Component {
    fn get_id(&self) -> u64;
    fn set_ptr(&mut self, ptr: *mut c_void);
    fn get_ptr(&self) -> *mut c_void;
    fn set_singleton(&mut self, singleton: bool);
    fn get_singleton(&self) -> bool;
}

pub trait ComponentType {
    fn register() -> ecs_entity_t;
    fn get_name() -> &'static str;
    fn get_hash() -> u64;
}

#[repr(C)]
pub struct Filter {
    pub filter: *mut c_void,
    pub filter_desc: *mut c_void,
    pub filter_index: u8,
    iter: *mut c_void,
    entities: &'static mut [Entity],
}

impl Filter {
    pub fn new() -> Filter {
        Filter {
            filter: core::ptr::null_mut(),
            filter_desc: unsafe { toxoid_filter_create() },
            filter_index: 0,
            iter: core::ptr::null_mut(),
            entities: &mut [],
        }
    }

    pub fn with<T: ComponentTuple + 'static>(mut self) -> Self {
        let type_ids = T::get_type_ids();
        let layout = Layout::array::<u64>(type_ids.len()).unwrap();
        let ids_ptr = unsafe { ALLOCATOR.alloc(layout) as *mut ecs_entity_t };
        type_ids
            .iter()
            .enumerate()
            .for_each(|(i, type_id)| {
                let type_id = split_u64(*type_id);
                let id = unsafe { toxoid_component_cache_get(type_id) };
                let id = combine_u32(id);
                unsafe { ids_ptr.add(i).write(id) };
            });
        self.filter_index = unsafe { toxoid_filter_with(self.filter_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
        self
    }

    pub fn without<T: ComponentTuple + 'static>(mut self) -> Self {
        let type_ids = T::get_type_ids();
        let layout = Layout::array::<u64>(type_ids.len()).unwrap();
        let ids_ptr = unsafe { ALLOCATOR.alloc(layout) as *mut ecs_entity_t };
        type_ids
            .iter()
            .enumerate()
            .for_each(|(i, type_id)| {
                let type_id = split_u64(*type_id);
                let id = unsafe { toxoid_component_cache_get(type_id) };
                let id = combine_u32(id);
                unsafe { ids_ptr.add(i).write(id) };
            });
        
        self.filter_index = unsafe { toxoid_filter_without(self.filter_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
        self
    }

    pub fn with_or<T: ComponentTuple + 'static>(mut self) -> Self {
        let type_ids = T::get_type_ids();
        let layout = Layout::array::<u64>(type_ids.len()).unwrap();
        let ids_ptr = unsafe { ALLOCATOR.alloc(layout) as *mut ecs_entity_t };
        type_ids
            .iter()
            .enumerate()
            .for_each(|(i, type_id)| {
                let type_id = split_u64(*type_id);
                let id = unsafe { toxoid_component_cache_get(type_id) };
                let id = combine_u32(id);
                unsafe { ids_ptr.add(i).write(id) };
            });

        self.filter_index = unsafe { toxoid_filter_with_or(self.filter_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
        self
    }

    pub fn iter(&mut self) -> *mut c_void {
        unsafe { toxoid_filter_iter(self.filter) }
    }

    pub fn build(mut self) -> Self {
        self.filter = unsafe { toxoid_filter_build(self.filter_desc) };
        self
    }
}

pub struct Iter {
    iter: *mut c_void,
    entities: &'static mut [Entity]
}

impl Iter {
    pub fn new() -> Iter {
        Iter {
            iter: core::ptr::null_mut(),
            entities: &mut []
        }
    }

    pub fn from(iter: *mut c_void) -> Iter {
        Iter {
            iter,
            entities: &mut []
        }
    }

    pub fn count(&self) -> usize {
        unsafe { toxoid_iter_count(self.iter) as usize }
    }

    pub fn next(&self) -> bool {
        unsafe { toxoid_query_next(self.iter) }
    }

    pub fn filter_next(&self) -> bool {
        unsafe { toxoid_filter_next(self.iter) }
    }

    // TODO: FREE MEMORY
    pub fn entities(&mut self) -> &mut [Entity] {
        self.entities = unsafe { toxoid_query_entity_list(self.iter) };
        self.entities
    }

    // TODO: FREE MEMORY
    pub fn field<T: Default + Component + ComponentType + 'static>(&self, term_index: i32) -> &'static [T] {
        unsafe {
            // Get count of components
            let count = toxoid_iter_count(self.iter);
            // Get slice of pointers to components
            let field_slice = toxoid_query_field_list(self.iter, term_index, count as u32) as *mut *const T;
            let field_slice = core::slice::from_raw_parts(field_slice, count as usize);
            // Correctly calculate the layout for an array of T
            let layout = Layout::array::<T>(count as usize).expect("Failed to create layout");
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
            // Return slice of components
            components
        }
    }

    // TODO: FREE MEMORY
    pub fn field_mut<T: Default + Component + ComponentType + 'static>(&self, term_index: i32) -> &'static mut [T] {
        unsafe {
            // Get count of components
            let count = toxoid_iter_count(self.iter);
            // Get slice of pointers to components
            let field_slice = toxoid_query_field_list(self.iter, term_index, count as u32) as *mut *mut T;
            let field_slice = core::slice::from_raw_parts(field_slice, count as usize);
            // Correctly calculate the layout for an array of T
            let layout = Layout::array::<T>(count as usize).expect("Failed to create layout");
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
            let components = core::slice::from_raw_parts_mut(components_ptr, count as usize);
            // Return slice of components
            components
        }
    }

    // pub fn each_mut<T: ComponentTuple>(&mut self, mut _func: impl FnMut(T)) {
    //     // Ensure we start from the beginning
    //     self.iter();
    //     while self.next() {
    //         let _type_ids = T::get_type_ids();
    //         // let components: T = T::get_type_ids()
    //         //     .iter()
    //         //     .map(|&id| {
    //         //         unsafe {
    //         //             let comp_ptr = toxoid_query_field_list(self.iter, id as i32, 1);
    //         //             let mut comp = <id's type>::default();
    //         //             comp.set_ptr(*comp_ptr as *mut c_void);
    //         //             comp
    //         //         }
    //         //     })
            
    //         // for i in 0...10 {
    //         //     func(components);
    //         // }
    //     }
    // }
    
    // #[cfg(target_arch="wasm32")]
    // pub fn drop(&self) {
    //     unsafe {
    //         ALLOCATOR.dealloc(self.iter as *mut u8, core::alloc::Layout::new::<c_void>()); 
    //         ALLOCATOR.dealloc(self.entities.as_ptr() as *mut u8,core::alloc::Layout::array::<Entity>(self.entities.len()).unwrap());
    //     }   
    // }
}

// Not wasm targets
#[cfg(not(target_arch="wasm32"))]
impl Drop for Iter {
    fn drop(&mut self) {
        // unsafe {
            // ALLOCATOR.dealloc(self.iter as *mut u8, core::alloc::Layout::new::<c_void>()); 
            // ALLOCATOR.dealloc(self.entities.as_ptr() as *mut u8,core::alloc::Layout::array::<Entity>(self.entities.len()).unwrap());
        // }
    }
}

#[repr(C)]
pub struct Query {
    query: *mut c_void,
    query_desc: *mut c_void,
    filter_index: u8
}

// TODO: Figure out why this causes undefined symbols
// on method calls in Emscripten dynamically linked no_std side module
// Aborted(Assertion failed: undefined symbol '__THREW__'. perhaps a side module was not linked in? if this global was expected to arrive from a system library, try to build the MAIN_MODULE with EMCC_FORCE_STDLIBS=1 in the environment)
// impl Drop for Query {
//     fn drop(&mut self) {
//         unsafe {
//             ALLOCATOR.dealloc(self.iter as *mut u8, core::alloc::Layout::new::<c_void>()); 
//             ALLOCATOR.dealloc(self.entities.as_ptr() as *mut u8,core::alloc::Layout::array::<Entity>(self.entities.len()).unwrap());
//             ALLOCATOR.dealloc(self.indexes.as_ptr() as *mut u8, core::alloc::Layout::array::<Entity>(self.indexes.len()).unwrap()); 
//         }
//     }
// }

impl Query {
    // pub fn new<T: ComponentTuple + 'static>() -> Query  {
    //     unsafe {
    //         let type_ids = T::get_type_ids();
    //         let layout = Layout::array::<u64>(type_ids.len()).unwrap();
    //         let ids_ptr = ALLOCATOR.alloc(layout) as *mut ecs_entity_t;
    //         type_ids
    //             .iter()
    //             .enumerate()
    //             .for_each(|(i, type_id)| {
    //                 let id = toxoid_component_cache_get(*type_id);
    //                 let id = combine_u32(id);
    //                 ids_ptr.add(i).write(id);
    //             });
            
    //         Query {
    //             query: toxoid_query_create(ids_ptr, type_ids.len() as i32),
    //             indexes: type_ids,
    //             iter: core::ptr::null_mut(),  
    //             entities: &mut [],
    //         }
    //     }
    // }

    pub fn new() -> Query  {
        Query {
            query: core::ptr::null_mut(),
            query_desc: unsafe { toxoid_query_create() },
            filter_index: 0,
        }
    }

    pub fn iter(&mut self) -> *mut c_void {
        unsafe { toxoid_query_iter(self.query) }
    }

    pub fn with<T: ComponentTuple + 'static>(&mut self) -> &mut Query {
        let type_ids = T::get_type_ids();
        let layout = Layout::array::<u64>(type_ids.len()).unwrap();
        let ids_ptr = unsafe { ALLOCATOR.alloc(layout) as *mut ecs_entity_t };
        type_ids
            .iter()
            .enumerate()
            .for_each(|(i, type_id)| {
                let type_id = split_u64(*type_id);
                let id = unsafe { toxoid_component_cache_get(type_id) };
                let id = combine_u32(id);
                unsafe { ids_ptr.add(i).write(id) };
            });
        self.filter_index = unsafe { toxoid_query_with(self.query_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
        self
    }

    pub fn without<T: ComponentTuple + 'static>(&mut self) -> &mut Query {
        let type_ids = T::get_type_ids();
        let layout = Layout::array::<u64>(type_ids.len()).unwrap();
        let ids_ptr = unsafe { ALLOCATOR.alloc(layout) as *mut ecs_entity_t };
        type_ids
            .iter()
            .enumerate()
            .for_each(|(i, type_id)| {
                let type_id = split_u64(*type_id);
                let id = unsafe { toxoid_component_cache_get(type_id) };
                let id = combine_u32(id);
                unsafe { ids_ptr.add(i).write(id) };
            });
        
        self.filter_index = unsafe { toxoid_query_without(self.query_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
        self
    }

    pub fn with_or<T: ComponentTuple + 'static>(&mut self) -> &mut Query {
        let type_ids = T::get_type_ids();
        let layout = Layout::array::<u64>(type_ids.len()).unwrap();
        let ids_ptr = unsafe { ALLOCATOR.alloc(layout) as *mut ecs_entity_t };
        type_ids
            .iter()
            .enumerate()
            .for_each(|(i, type_id)| {
                let type_id = split_u64(*type_id);
                let id = unsafe { toxoid_component_cache_get(type_id) };
                let id = combine_u32(id);
                unsafe { ids_ptr.add(i).write(id) };
            });
        
        self.filter_index = unsafe { toxoid_query_with_or(self.query_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
        self
    }

    pub fn order_by<T: Default + Component + ComponentType + 'static>(&mut self, callback: extern "C" fn(ecs_entity_t, *const c_void, ecs_entity_t, *const c_void) -> i32) -> &mut Self {
        unsafe {
            let type_hash = split_u64(T::get_hash());
            let component_id_split = toxoid_component_cache_get(type_hash);
            let component_id = combine_u32(component_id_split);
            toxoid_query_order_by(self.query_desc, component_id, callback);
            self
        }
    }

    pub fn build(&mut self) -> &mut Query {
        self.query = unsafe { toxoid_query_build(self.query_desc) };
        self
    }
}

#[allow(improper_ctypes_definitions)]
#[repr(C)]
pub struct System {
    system_desc: *mut c_void,
    query_desc: *mut c_void,
    filter_index: u8,
}

pub trait SystemTrait {
    fn update(&mut self);
}

impl System {
    pub fn new(callback: fn(&mut Iter)) -> Self {        
        let system_desc = unsafe { toxoid_system_create(callback) };
        let query_desc = unsafe { toxoid_query_from_system_desc(system_desc) };
        System {
            system_desc,
            query_desc,
            filter_index: 0
        }
    }

    pub fn with<T: ComponentTuple + 'static>(&mut self) -> &mut Self {
        let type_ids = T::get_type_ids();
        let layout = Layout::array::<u64>(type_ids.len()).unwrap();
        let ids_ptr = unsafe { ALLOCATOR.alloc(layout) as *mut ecs_entity_t };
        type_ids
            .iter()
            .enumerate()
            .for_each(|(i, type_id)| {
                let type_id = split_u64(*type_id);
                let id = unsafe { toxoid_component_cache_get(type_id) };
                let id = combine_u32(id);
                unsafe { ids_ptr.add(i).write(id) };
            });
        self.filter_index = unsafe { toxoid_query_with(self.query_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
        self
    }

    pub fn without<T: ComponentTuple + 'static>(&mut self) -> &mut Self {
        let type_ids = T::get_type_ids();
        let layout = Layout::array::<u64>(type_ids.len()).unwrap();
        let ids_ptr = unsafe { ALLOCATOR.alloc(layout) as *mut ecs_entity_t };
        type_ids
            .iter()
            .enumerate()
            .for_each(|(i, type_id)| {
                let type_id = split_u64(*type_id);
                let id = unsafe { toxoid_component_cache_get(type_id) };
                let id = combine_u32(id);
                unsafe { ids_ptr.add(i).write(id) };
            });
        
        self.filter_index = unsafe { toxoid_query_without(self.query_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
        self
    }

    pub fn with_or<T: ComponentTuple + 'static>(&mut self) -> &mut Self {
        let type_ids = T::get_type_ids();
        let layout = Layout::array::<u64>(type_ids.len()).unwrap();
        let ids_ptr = unsafe { ALLOCATOR.alloc(layout) as *mut ecs_entity_t };
        type_ids
            .iter()
            .enumerate()
            .for_each(|(i, type_id)| {
                let type_id = split_u64(*type_id);
                let id = unsafe { toxoid_component_cache_get(type_id) };
                let id = combine_u32(id);
                unsafe { ids_ptr.add(i).write(id) };
            });
        
        self.filter_index = unsafe { toxoid_query_with_or(self.query_desc, self.filter_index, ids_ptr, type_ids.len() as i32) };
        self
    }

    pub fn order_by<T: Default + Component + ComponentType + 'static>(&mut self, callback: extern "C" fn(ecs_entity_t, *const c_void, ecs_entity_t, *const c_void) -> i32) -> &mut Self {
        unsafe {
            let type_hash = split_u64(T::get_hash());
            let component_id_split = toxoid_component_cache_get(type_hash);
            let component_id = combine_u32(component_id_split);
            toxoid_query_order_by(self.query_desc, component_id, callback);
            self
        }
    }

    pub fn build(&mut self) -> &mut Self {
        unsafe { toxoid_system_build(self.system_desc) };
        self
    }
}

#[repr(C)]
pub struct World {}

impl World {
    pub fn add_singleton<T: Component + ComponentType + 'static>() {
        unsafe {
            let type_hash = split_u64(T::get_hash());
            let component_id_split = toxoid_component_cache_get(type_hash);
            let component_id = combine_u32(component_id_split);
            toxoid_singleton_add(component_id);
        }
    }

    pub fn get_singleton<T: Default + Component + ComponentType + 'static>() -> T {
        unsafe {
            let mut component = T::default();
            let type_hash = split_u64(T::get_hash());
            let component_id_split = toxoid_component_cache_get(type_hash);
            let component_id = combine_u32(component_id_split);
            let ptr = toxoid_singleton_get(component_id);
            component.set_ptr(ptr);
            component.set_singleton(true);
            component
        }
    }

    pub fn remove_singleton<T: Component + ComponentType + 'static>() {
        unsafe {
            let type_hash = split_u64(T::get_hash());
            let component_id_split = toxoid_component_cache_get(type_hash);
            let component_id = combine_u32(component_id_split);
            toxoid_singleton_remove(component_id);
        }
    }

    pub fn delete_entity(entity: Entity) {
        unsafe {
            toxoid_delete_entity(entity.get_id());
        }
    }
    
    pub fn delete_entity_mut(entity: &mut Entity) {
        unsafe {
            let id = entity.get_id();
            toxoid_delete_entity(id);
        }
    }
}

#[repr(C)]
pub struct Prefab {
    entity: Entity
}

impl Prefab {
    pub fn new() -> Prefab {
        let entity = Entity {
            id: unsafe { combine_u32(toxoid_prefab_create()) },
            children: &mut []
        };
        Prefab {
            entity
        }
    }

    pub fn add<T: Component + ComponentType + 'static>(&mut self) {
        unsafe {
            let type_hash = split_u64(T::get_hash());
            let component_id_split = toxoid_component_cache_get(type_hash);
            let component_id = combine_u32(component_id_split);
            toxoid_entity_add_component(self.entity.id, component_id);
        }
    }

    pub fn remove<T: Component + ComponentType + 'static>(&mut self) {
        unsafe {
            let type_hash = split_u64(T::get_hash());
            let component_id_split = toxoid_component_cache_get(type_hash);
            let component_id = combine_u32(component_id_split);
            toxoid_entity_remove_component(self.entity.id, component_id);
        }
    }

    pub fn add_id(&mut self, component: ecs_id_t) {
        unsafe {
            toxoid_entity_add_component(self.entity.id, component);
        }
    }

    pub fn add_tag(&mut self, tag: ecs_entity_t) {
        unsafe {
            toxoid_entity_add_tag(self.entity.id, tag);
        }
    }

    pub fn get_id(&self) -> ecs_entity_t {
        self.entity.id
    }

    pub fn get<T: Default + Component + ComponentType + 'static>(&self) -> T {
        unsafe {
            let mut component = T::default();
            let type_hash = split_u64(T::get_hash());
            let component_id_split = toxoid_component_cache_get(type_hash);
            let component_id = combine_u32(component_id_split);
            let ptr = toxoid_entity_get_component(self.entity.id, component_id);
            component.set_ptr(ptr);
            component
        }
    }

    pub fn has<T: Component + ComponentType + 'static>(&self) -> bool {
        unsafe {
            let type_hash = split_u64(T::get_hash());
            let component_id_split = toxoid_component_cache_get(type_hash);
            let component_id = combine_u32(component_id_split);
            toxoid_entity_has_component(self.entity.id, component_id)
        }
    }

    pub fn child_of(&mut self, parent: Entity) {
        unsafe {
            toxoid_entity_child_of(self.entity.id, parent.get_id());
        }
    }

    pub fn parent_of(&mut self, child: Entity) {
        unsafe {
            toxoid_entity_child_of(child.get_id(), self.entity.id);
        }
    }
}
 
#[repr(C)]
#[derive(Debug)]
pub struct Entity {
    pub id: ecs_id_t,
    // For deallocating all children returend from entities.children() on drop
    pub children: &'static mut [Entity],
    // For deallocating all components grabbed from entity.get::<T>() on drop
    // components: *mut Component
}

// #[repr(C)]
// #[derive(Debug)]
// pub struct Entity {
//     ptr: *mut Entity
// }

// TODO: Figure out why this causes undefined symbols
// on method calls in Emscripten dynamically linked no_std side module
// Aborted(Assertion failed: undefined symbol '__THREW__'. perhaps a side module was not linked in? if this global was expected to arrive from a system library, try to build the MAIN_MODULE with EMCC_FORCE_STDLIBS=1 in the environment)
// impl Drop for Entity {
//     fn drop(&mut self) {
//         unsafe {
//             ALLOCATOR.dealloc(self.children.as_ptr() as *mut u8, Layout::array::<Entity>(self.children.len() as usize).unwrap());
//         }
//     }
// }


impl Entity {
    pub fn new() -> Entity {
        let entity = unsafe { toxoid_entity_create() };
        #[cfg(target_arch="wasm32")]
        let entity = combine_u32(entity);
        unsafe { toxoid_entity_set_name(entity, make_c_string("")); }
        Entity {
            id: entity,
            children: &mut []
        }
    }

    pub fn set_name(&self, name: &str) {
        unsafe {
           toxoid_entity_set_name(self.id, make_c_string(name));
        }
    }

    pub fn from_id(id: ecs_id_t) -> Entity {
        Entity {
            id,
            children: &mut []
        }
    }

    pub fn from_prefab(prefab: Prefab) -> Entity {
        let prefab_split = split_u64(prefab.entity.id);
        let entity_split = unsafe { toxoid_prefab_instance(prefab_split.high, prefab_split.low) };
        let entity = combine_u32(entity_split);
        Entity {
            id: entity,
            children: &mut []
        }
    }

    pub fn add<T: Component + ComponentType + 'static>(&mut self) {
        unsafe {
            let type_hash = split_u64(T::get_hash());
            let component_id_split = toxoid_component_cache_get(type_hash);
            let component_id = combine_u32(component_id_split);
            toxoid_entity_add_component(self.id, component_id);
        }
    }

    pub fn remove<T: Component + ComponentType + 'static>(&mut self) {
        unsafe {
            let type_hash = split_u64(T::get_hash());
            let component_id_split = toxoid_component_cache_get(type_hash);
            let component_id = combine_u32(component_id_split);
            toxoid_entity_remove_component(self.id, component_id);
        }
    }

    pub fn add_by_id(&mut self, component_id: ecs_entity_t) {
        unsafe {
            toxoid_entity_add_component(self.id, component_id);
        }
    }

    pub fn remove_by_id(&mut self, component_id: ecs_entity_t) {
        unsafe {
            toxoid_entity_remove_component(self.id, component_id);
        }
    }

    pub fn add_tag(&mut self, tag: ecs_entity_t) {
        unsafe {
            toxoid_entity_add_tag(self.id, tag);
        }
    }

    pub fn get_id(&self) -> ecs_entity_t {
        self.id
    }

    // TODO: FREE MEMORY
    pub fn get<T: Default + Component + ComponentType + 'static>(&self) -> T {
        unsafe {
            let mut component = T::default();
            let type_hash = split_u64(T::get_hash());
            let component_id_split = toxoid_component_cache_get(type_hash);
            let component_id = combine_u32(component_id_split);
            let ptr = toxoid_entity_get_component(self.id, component_id);
            component.set_ptr(ptr);
            component
        }
    }

    pub fn has<T: Component + ComponentType + 'static>(&self) -> bool {
        unsafe {
            let type_hash = split_u64(T::get_hash());
            let component_id_split = toxoid_component_cache_get(type_hash);
            let component_id = combine_u32(component_id_split);
            toxoid_entity_has_component(self.id, component_id)
        }
    }

    pub fn child_of(&mut self, parent: Entity) {
        unsafe {
            toxoid_entity_child_of(self.id, parent.get_id());
        }
    }

    pub fn parent_of(&mut self, child: Entity) {
        unsafe {
            toxoid_entity_child_of(child.get_id(), self.id);
        }
    }

    pub fn child_of_by_id(&mut self, parent: ecs_entity_t) {
        unsafe {
            toxoid_entity_child_of(self.id, parent);
        }
    }

    pub fn parent_of_by_id(&mut self, child: ecs_entity_t) {
        unsafe {
            toxoid_entity_child_of(child, self.id);
        }
    }

    pub fn parents(&mut self) -> Entity {
        unimplemented!()
    }

    pub fn children(&mut self) -> &mut [Entity]  {
        unsafe {
            let filter = toxoid_filter_children_init(self.get_id());
            let it = toxoid_filter_iter(filter);
            toxoid_filter_next(it);
            let entities = toxoid_iter_entities(it);
            let count = toxoid_iter_count(it);
            let layout = Layout::array::<Entity>(count as usize).unwrap();
            let entities_ptr = ALLOCATOR.alloc(layout) as *mut Entity;
            entities
                .iter()
                .enumerate()
                .for_each(|(i, entity_id)| {
                    entities_ptr.add(i).write(Entity { 
                        id: *entity_id, 
                        children: &mut []
                    });
                });
            let entities = core::slice::from_raw_parts_mut(entities_ptr, count as usize);
            // Assign to self so we can drop it later
            // self.children = entities;
            entities
        }
    }

    pub fn children_each(&mut self, mut cb: impl FnMut(Entity)) {
        unsafe {
            let filter = toxoid_filter_children_init(self.get_id());
            let it = toxoid_filter_iter(filter);
            while toxoid_filter_next(it) {
                let entities = toxoid_iter_entities(it);
                entities
                    .iter()
                    .for_each(|entity_id| {
                        let e = Entity { 
                            id: *entity_id, 
                            children: &mut []
                        };
                        cb(e);
                    });
			}
        }
    }

    pub fn drop(&mut self) {
        unsafe {
            ALLOCATOR.dealloc(self.children.as_ptr() as *mut u8, Layout::array::<Entity>(self.children.len() as usize).unwrap());
        }
    }
}

pub fn delete_entity(entity: Entity) {
    unsafe {
        toxoid_delete_entity(entity.get_id());
    }
}

pub fn delete_entity_mut(entity: &mut Entity) {
    unsafe {
        toxoid_delete_entity(entity.get_id());
    }
}

pub fn is_valid(entity: Entity) -> bool {
    unsafe {
        toxoid_is_valid(entity.get_id())
    }
}

pub fn is_valid_mut(entity: &mut Entity) -> bool {
    unsafe {
        toxoid_is_valid(entity.get_id())
    }
}

pub fn register_tag(name: &str) -> ecs_entity_t {
    unsafe { toxoid_register_tag(name.as_bytes().as_ptr() as *const i8, name.len()) }
}

pub fn cache_component_ecs(type_id: SplitU64, component_id: SplitU64) {
    unsafe {
        toxoid_component_cache_insert(type_id, component_id);
    }
}

// Used to count the number of arguments passed to it.
#[macro_export]
macro_rules! count_args {
    ($($args:ident),*) => { <[()]>::len(&[$(count_args!(@substitute $args)),*]) };
    (@substitute $_t:tt) => { () };
}

// Convenience macro to convert a slice of `Component` objects into a slice of their IDs, ie: pass in &[keyboard_input] instead of &[keyboard_input.get_id()]
// #[macro_export]
// macro_rules! component_ids {
//     ($($components:expr),*) => {{
//         &[$(combine_u32($components.get_id())),*]
//     }}
// }
// macro_rules! component_ids {
//     // Match a single component without a comma
//     ($component:expr) => {{
//         unsafe {
//             let layout = std::alloc::Layout::array::<u64>(1).unwrap();
//             let ids_ptr = std::alloc::alloc(layout) as *mut u64;
//             *ids_ptr = combine_u32($component.get_id());
//             ids_ptr as *const u32

//             let layout = Layout::array::<u64>(1).unwrap();
//             let entities_ptr = ALLOCATOR.alloc(layout) as *mut u64;
//             entities
//                 .iter()
//                 .enumerate()
//                 .for_each(|(i, entity_id)| {
//                     entities_ptr.add(i).write(Entity { 
//                         id: *entity_id, 
//                         children: &mut []
//                     });
//                 });
//         }
//     }};
//     // Match multiple components with commas
//     ($($components:expr),+ $(,)?) => {{
//         unsafe {
//             let count = count_args!($($components),*);
//             let layout = std::alloc::Layout::array::<u64>(count).unwrap();
//             let ids_ptr = std::alloc::alloc(layout) as *mut u64;
//             let mut temp_ptr = ids_ptr;
//             $(
//                 std::ptr::write(temp_ptr, $components.get_id());
//                 temp_ptr = temp_ptr.add(1);
//             )*
//             ids_ptr as *const u32
//         }
//     }};
// }

// Used to implement the ComponentTuple trait for tuples of different lengths, where each element of the tuple is a component.
#[macro_export]
macro_rules! impl_component_tuple {
    ($($name:ident)+) => {
        impl<$($name: Default + Component + ComponentType + 'static),+> ComponentTuple for ($($name,)+) {
            #[allow(unused_assignments)]
            fn get_type_ids() -> &'static [u64] { // Change return type to u64
                unsafe {
                    let count = count_args!($($name),+);
                    let layout = Layout::array::<u64>(count).unwrap(); // Change layout to u64
                    let type_ids_ptr = ALLOCATOR.alloc(layout) as *mut u64; // Change pointer type to u64
                    let mut i = 0;
                    $(
                        type_ids_ptr.add(i).write($name::get_hash()); // Use get_hash() instead of TypeId::of
                        i += 1;
                    )+
                    let type_ids = core::slice::from_raw_parts(type_ids_ptr, count as usize);
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


